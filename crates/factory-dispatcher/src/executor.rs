//! Tiered parallel execution of matched plugins (S-1.6).
//!
//! Consumers of this module hand it the ordered tiers produced by
//! [`crate::routing::group_by_priority`] and get back a
//! [`TierExecutionSummary`] that records every plugin's outcome plus a
//! dispatcher-level exit code.
//!
//! Within a tier: plugins run concurrently via tokio tasks; each task
//! wraps the synchronous [`crate::invoke::invoke_plugin`] in
//! `spawn_blocking` so wasmtime is never blocking the runtime. Between
//! tiers: the dispatcher awaits every task before the next tier begins,
//! preserving priority ordering.
//!
//! Advisory-block semantics (per Q3 resolution + W-15 gate fix CRIT-PR59-001):
//! a plugin that writes `{"outcome":"block","reason":"..."}` to stdout
//! records a dispatcher-level block intent regardless of `on_error` setting.
//! The `on_error` field governs fail-closed semantics for crash and timeout:
//! a sync-group plugin that Crashes or times out with `on_error=Block` triggers
//! fail-closed exit 2 (ADR-019 §Decision 2). Async hooks never trigger fail-closed
//! (async block verdicts are advisory-only per ADR-019). The summary's
//! `exit_code` is 2 iff any block intent was recorded.

use std::sync::Arc;
use std::time::Instant;

use wasmtime::Engine;

use crate::host::HostContext;
use crate::internal_log::{
    InternalEvent, InternalLog, PLUGIN_COMPLETED, PLUGIN_CRASHED, PLUGIN_INVOKED, PLUGIN_TIMEOUT,
};
use crate::invoke::{InvokeLimits, PluginResult, TimeoutCause, invoke_plugin};
use crate::plugin_loader::PluginCache;
use crate::registry::{OnError, Registry, RegistryEntry};
use crate::resolver::{ResolverInput, ResolverRegistry, merge_resolver_outputs};

/// Owned per-plugin outcome with a name attached so callers don't have
/// to zip with the original tier vec. `RegistryEntry` is cloned here —
/// it's small (no WASM binary), and cloning sidesteps borrow-plumbing
/// through the tokio tasks.
#[derive(Debug, Clone)]
pub struct PluginOutcome {
    pub plugin_name: String,
    pub plugin_version: String,
    pub on_error: OnError,
    pub result: PluginResult,
}

/// Aggregated result of running every tier in order.
#[derive(Debug, Clone)]
pub struct TierExecutionSummary {
    pub per_plugin_results: Vec<PluginOutcome>,
    pub total_elapsed_ms: u64,
    pub block_intent: bool,
    pub exit_code: i32,
}

/// Inputs into a single `execute_tiers` call. Borrowing the engine and
/// cache; owning the registry + payload so the caller can construct
/// them fresh per invocation.
///
/// `payload_value` is the *base* envelope (with `dispatcher_trace_id`
/// already injected by main.rs). The executor deep-clones it per plugin
/// and splices in `plugin_config` from the registry entry before
/// serializing to bytes for invoke. Per-plugin spliced bytes mean two
/// plugins in the same tier never see each other's config — exactly
/// what the legacy-bash-adapter (S-2.1) needs to multiplex over a
/// single shared adapter wasm.
///
/// `resolver_registry` is the in-process context-resolver registry built
/// from `resolvers-registry.toml`. It is consulted for each hook entry's
/// `needs_context` list before `plugin_config` is spliced in.
/// An empty registry (no resolvers registered) is valid and produces
/// zero overhead via the `needs_context.is_empty()` short-circuit
/// (BC-1.13.001 PC3 / AC-002).
pub struct ExecutorInputs<'a> {
    pub engine: &'a Engine,
    pub cache: &'a PluginCache,
    pub registry: &'a Registry,
    pub payload_value: serde_json::Value,
    pub base_host_ctx: HostContext,
    /// Mirror of the dispatcher's internal log, used to emit plugin
    /// lifecycle events. Held in an `Arc` so per-plugin tasks can
    /// reach it without cloning the whole log.
    pub internal_log: Arc<InternalLog>,
    /// In-process context-resolver registry. Queried per hook entry
    /// for each name in `entry.needs_context`. Pass
    /// `Arc::new(ResolverRegistry::new())` when no resolvers are
    /// configured (BC-1.13.001 INV2).
    pub resolver_registry: Arc<ResolverRegistry>,
}

/// Run every tier and return the aggregated summary.
pub async fn execute_tiers(
    inputs: ExecutorInputs<'_>,
    tiers: Vec<Vec<&RegistryEntry>>,
) -> TierExecutionSummary {
    let started = Instant::now();
    let mut all_outcomes: Vec<PluginOutcome> = Vec::new();
    let mut block_intent = false;

    for tier in tiers {
        let tier_outcomes = execute_tier(&inputs, tier).await;
        for outcome in &tier_outcomes {
            if plugin_requests_block(&outcome.result)
                || plugin_fail_closed(&outcome.result, outcome.on_error)
            {
                block_intent = true;
            }
        }
        all_outcomes.extend(tier_outcomes);
    }

    TierExecutionSummary {
        total_elapsed_ms: started.elapsed().as_millis() as u64,
        exit_code: if block_intent { 2 } else { 0 },
        block_intent,
        per_plugin_results: all_outcomes,
    }
}

/// Execute every plugin in a single tier concurrently.
async fn execute_tier<'a>(
    inputs: &ExecutorInputs<'a>,
    tier: Vec<&'a RegistryEntry>,
) -> Vec<PluginOutcome> {
    let mut join_handles = Vec::with_capacity(tier.len());

    for entry in tier {
        let engine = inputs.engine.clone();
        let entry_clone = entry.clone();
        let limits = InvokeLimits {
            timeout_ms: entry_clone.timeout_ms(&inputs.registry.defaults),
            fuel_cap: entry_clone.fuel_cap(&inputs.registry.defaults),
        };
        let on_error = entry_clone.on_error(&inputs.registry.defaults);

        // Build the merged plugin_config from static config + resolver outputs.
        // AC-002: zero-overhead short-circuit when needs_context is empty.
        // AC-003: invoke resolver and merge outputs when needs_context is non-empty.
        let trace_id = inputs
            .payload_value
            .get("dispatcher_trace_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let plugin_config = build_plugin_config(
            &entry_clone,
            &inputs.payload_value,
            &inputs.base_host_ctx,
            &inputs.resolver_registry,
            &inputs.internal_log,
            &trace_id,
        );

        // Splice this entry's per-plugin config onto the base envelope.
        // Cheap clone since `payload_value` is a small JSON tree, and
        // doing it per-plugin guarantees one entry never sees another's
        // config — even when several entries share the same wasm
        // (e.g. multiple legacy-bash-adapter registrations).
        let mut per_plugin_value = inputs.payload_value.clone();
        if let Some(map) = per_plugin_value.as_object_mut() {
            map.insert("plugin_config".to_string(), plugin_config);
        }
        let payload = match serde_json::to_vec(&per_plugin_value) {
            Ok(v) => v,
            Err(e) => {
                let result = PluginResult::Crashed {
                    trap_string: format!("payload serialize: {e}"),
                    stderr: String::new(),
                    elapsed_ms: 0,
                    fuel_consumed: 0,
                };
                emit_lifecycle(
                    &inputs.internal_log,
                    &inputs.base_host_ctx,
                    &entry_clone,
                    &result,
                );
                join_handles.push(JoinWrap::Ready(PluginOutcome {
                    plugin_name: entry_clone.name.clone(),
                    plugin_version: inputs.base_host_ctx.plugin_version.clone(),
                    on_error,
                    result,
                }));
                continue;
            }
        };
        let internal_log = inputs.internal_log.clone();

        let mut host_ctx = inputs.base_host_ctx.clone();
        host_ctx.plugin_name = entry_clone.name.clone();
        host_ctx.capabilities = entry_clone.capabilities.clone().unwrap_or_default();

        let module = match inputs.cache.get_or_compile(&entry_clone.plugin) {
            Ok(m) => m,
            Err(e) => {
                let result = PluginResult::Crashed {
                    trap_string: format!("plugin load failed: {e}"),
                    stderr: String::new(),
                    elapsed_ms: 0,
                    fuel_consumed: 0,
                };
                emit_lifecycle(&internal_log, &inputs.base_host_ctx, &entry_clone, &result);
                let outcome = PluginOutcome {
                    plugin_name: entry_clone.name.clone(),
                    plugin_version: host_ctx.plugin_version.clone(),
                    on_error,
                    result,
                };
                join_handles.push(JoinWrap::Ready(outcome));
                continue;
            }
        };

        emit_invoked(&internal_log, &inputs.base_host_ctx, &entry_clone);
        let base_ctx_for_event = inputs.base_host_ctx.clone();

        let handle = tokio::task::spawn_blocking(move || {
            let started = Instant::now();
            let result = invoke_plugin(&engine, &module, host_ctx.clone(), &payload, limits)
                .unwrap_or_else(|e| PluginResult::Crashed {
                    trap_string: format!("invoke setup error: {e}"),
                    stderr: String::new(),
                    elapsed_ms: started.elapsed().as_millis() as u64,
                    fuel_consumed: 0,
                });
            emit_lifecycle(&internal_log, &base_ctx_for_event, &entry_clone, &result);
            PluginOutcome {
                plugin_name: entry_clone.name.clone(),
                plugin_version: host_ctx.plugin_version.clone(),
                on_error,
                result,
            }
        });
        join_handles.push(JoinWrap::Pending(handle));
    }

    let mut outcomes = Vec::with_capacity(join_handles.len());
    for wrap in join_handles {
        match wrap {
            JoinWrap::Ready(o) => outcomes.push(o),
            JoinWrap::Pending(h) => match h.await {
                Ok(outcome) => outcomes.push(outcome),
                Err(join_err) => {
                    // A tokio JoinError here means spawn_blocking itself
                    // panicked in its harness — extremely rare, but treat
                    // as a plugin crash so the dispatcher stays up.
                    outcomes.push(PluginOutcome {
                        plugin_name: "<unknown>".into(),
                        plugin_version: "".into(),
                        on_error: OnError::Continue,
                        result: PluginResult::Crashed {
                            trap_string: format!("spawn_blocking join error: {join_err}"),
                            stderr: String::new(),
                            elapsed_ms: 0,
                            fuel_consumed: 0,
                        },
                    });
                }
            },
        }
    }
    outcomes
}

/// Execute a single async-group plugin as an independent tokio task.
///
/// Returns a `JoinHandle<PluginOutcome>` so the caller can collect results
/// via a channel and `tokio::select!` drain timer (BC-1.14.001 PC4 + EC-012).
///
/// # Async-group spawn pattern (BC-1.14.001 v1.9 PC4 + Invariant 3)
///
/// - Each async-group plugin MUST be spawned via `tokio::spawn` (independent task).
/// - Results MUST be collected via a channel (not all-or-nothing `execute_tiers`).
/// - `group_by_priority` MUST NOT be called on async-group plugins.
/// - The caller uses `tokio::select!` over the channel and a drain timer.
///
/// # BC traces
/// - BC-1.14.001 PC4 — per-plugin tokio::spawn spawn pattern
/// - BC-1.14.001 Invariant 3 — async group excluded from tier ordering
/// - EC-012 — partial completions: completed events MUST emit; in-flight MAY be lost
/// - BC-1.13.001 PC3/PC4/PC5 — resolver step mirrors execute_tier behavior
#[allow(clippy::too_many_arguments)]
pub fn spawn_async_plugin(
    engine: wasmtime::Engine,
    cache: Arc<crate::plugin_loader::PluginCache>,
    registry_defaults: crate::registry::RegistryDefaults,
    entry: RegistryEntry,
    payload_value: serde_json::Value,
    base_host_ctx: HostContext,
    internal_log: Arc<InternalLog>,
    resolver_registry: Arc<ResolverRegistry>,
) -> tokio::task::JoinHandle<PluginOutcome> {
    tokio::spawn(async move {
        let limits = InvokeLimits {
            timeout_ms: entry.timeout_ms(&registry_defaults),
            fuel_cap: entry.fuel_cap(&registry_defaults),
        };
        let on_error = entry.on_error(&registry_defaults);

        // Build the merged plugin_config from static config + resolver outputs.
        // AC-002: zero-overhead short-circuit when needs_context is empty.
        let trace_id = payload_value
            .get("dispatcher_trace_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let plugin_config = build_plugin_config(
            &entry,
            &payload_value,
            &base_host_ctx,
            &resolver_registry,
            &internal_log,
            &trace_id,
        );

        // Splice this entry's per-plugin config onto the base envelope.
        let mut per_plugin_value = payload_value;
        if let Some(map) = per_plugin_value.as_object_mut() {
            map.insert("plugin_config".to_string(), plugin_config);
        }
        let payload = match serde_json::to_vec(&per_plugin_value) {
            Ok(v) => v,
            Err(e) => {
                let result = PluginResult::Crashed {
                    trap_string: format!("payload serialize: {e}"),
                    stderr: String::new(),
                    elapsed_ms: 0,
                    fuel_consumed: 0,
                };
                emit_lifecycle(&internal_log, &base_host_ctx, &entry, &result);
                return PluginOutcome {
                    plugin_name: entry.name.clone(),
                    plugin_version: base_host_ctx.plugin_version.clone(),
                    on_error,
                    result,
                };
            }
        };

        let module = match cache.get_or_compile(&entry.plugin) {
            Ok(m) => m,
            Err(e) => {
                let result = PluginResult::Crashed {
                    trap_string: format!("plugin load failed: {e}"),
                    stderr: String::new(),
                    elapsed_ms: 0,
                    fuel_consumed: 0,
                };
                emit_lifecycle(&internal_log, &base_host_ctx, &entry, &result);
                return PluginOutcome {
                    plugin_name: entry.name.clone(),
                    plugin_version: base_host_ctx.plugin_version.clone(),
                    on_error,
                    result,
                };
            }
        };

        emit_invoked(&internal_log, &base_host_ctx, &entry);
        let base_ctx_for_event = base_host_ctx.clone();

        let mut host_ctx = base_host_ctx;
        host_ctx.plugin_name = entry.name.clone();
        host_ctx.capabilities = entry.capabilities.clone().unwrap_or_default();

        let result = tokio::task::spawn_blocking(move || {
            let started = std::time::Instant::now();
            invoke_plugin(&engine, &module, host_ctx, &payload, limits).unwrap_or_else(|e| {
                PluginResult::Crashed {
                    trap_string: format!("invoke setup error: {e}"),
                    stderr: String::new(),
                    elapsed_ms: started.elapsed().as_millis() as u64,
                    fuel_consumed: 0,
                }
            })
        })
        .await
        .unwrap_or_else(|join_err| PluginResult::Crashed {
            trap_string: format!("spawn_blocking join error: {join_err}"),
            stderr: String::new(),
            elapsed_ms: 0,
            fuel_consumed: 0,
        });

        emit_lifecycle(&internal_log, &base_ctx_for_event, &entry, &result);
        PluginOutcome {
            plugin_name: entry.name.clone(),
            plugin_version: base_ctx_for_event.plugin_version.clone(),
            on_error,
            result,
        }
    })
}

/// Internal helper: either an already-resolved outcome (load failure
/// short-circuit) or a pending tokio JoinHandle. Keeps the per-plugin
/// fan-out loop uniform.
enum JoinWrap {
    Ready(PluginOutcome),
    Pending(tokio::task::JoinHandle<PluginOutcome>),
}

/// Build the merged `plugin_config` for one hook entry.
///
/// AC-002: if `entry.needs_context` is empty, returns the static config
/// unchanged with zero resolver invocations.
///
/// AC-003: if `entry.needs_context` is non-empty, invokes each resolver
/// via `resolver_registry` and merges outputs onto the static config
/// using `merge_resolver_outputs`. Emits `resolver.not_found` and
/// `resolver.error` via `internal_log` for observability (BC-1.13.001
/// PC6 / SOUL #4 — no silent failures).
///
/// The returned `Value` is always a JSON Object ready to be inserted at
/// the `"plugin_config"` key of the per-plugin envelope.
fn build_plugin_config(
    entry: &RegistryEntry,
    payload_value: &serde_json::Value,
    base_host_ctx: &HostContext,
    resolver_registry: &ResolverRegistry,
    internal_log: &InternalLog,
    trace_id: &str,
) -> serde_json::Value {
    // AC-002: zero-overhead short-circuit (BC-1.13.001 PC3).
    if entry.needs_context.is_empty() {
        return entry.config_as_json();
    }

    // Hoist single config_as_json() call — avoids three separate allocations below.
    // Placed inside the non-empty branch so the zero-overhead short-circuit at the
    // top of this function (needs_context.is_empty() → return early) is preserved.
    let static_json = entry.config_as_json();

    // Build the ResolverInput from the current dispatch context.
    let event_type = payload_value
        .get("event_name")
        .or_else(|| payload_value.get("hook_event_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    // agent_type is reserved for future Claude Code envelope evolution per BC-4.12.002.
    // Standard PreToolUse/PostToolUse envelopes do not carry this field today; the
    // extraction defaults to None for resolver inputs. Forward-compat with potential
    // `subagent_type` or `agent_type` envelope additions.
    let agent_type = payload_value
        .get("agent_type")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let project_dir = base_host_ctx.cwd.to_str().unwrap_or("").to_string();

    let resolver_input = ResolverInput {
        event_type: event_type.clone(),
        hook_event_name: entry.name.clone(),
        agent_type,
        project_dir,
        plugin_config: static_json.clone(),
    };

    // Coerce the static config into a Map for merge_resolver_outputs (F-006).
    //
    // After Registry::parse_str + config_as_json(), plugin_config is guaranteed to be
    // Value::Object by TOML structure semantics: TOML tables always deserialize to JSON
    // objects, and Registry::parse_str rejects non-object plugin_config at load time.
    // The non-Object arm is therefore unreachable in production. The debug_assert
    // documents this invariant; any violation is a programming error, not a runtime fault.
    debug_assert!(
        matches!(static_json, serde_json::Value::Object(_)),
        "plugin_config must be a JSON object after Registry::parse_str — TOML table \
         semantics guarantee this; a non-Object value indicates a bypass of parse_str"
    );
    let static_map = match static_json {
        serde_json::Value::Object(m) => m,
        _ => serde_json::Map::new(),
    };

    let hook_name = entry.name.clone();
    let hook_name_nf = hook_name.clone();
    let hook_name_err = hook_name.clone();
    let trace_id_nf = trace_id.to_string();
    let trace_id_err = trace_id.to_string();
    // event_type is the Claude Code envelope event (e.g. "PreToolUse") emitted as
    // the event_type field in resolver.error events (HOST_ABI.md line 1097).
    let event_type_for_log = event_type.clone();

    // AC-005: emit resolver.not_found when a named resolver is absent.
    // Clone InternalLog (PathBuf wrapper) so the closure captures by value — no unsafe needed.
    //
    // Note: resolver.not_found event field table not yet documented in HOST_ABI.
    // Wire format (per implementation): { "resolver_name": String, "trace_id": String,
    // "plugin_name": String }. Documentation symmetry with resolver.error and
    // resolver.merge_collision field tables is deferred to a S-12.06 follow-up
    // HOST_ABI maintenance burst (per-story F-P7-002 deferral).
    let emit_not_found = {
        let log = internal_log.clone();
        move |missing_name: &str| {
            let ev = InternalEvent::now("resolver.not_found")
                .with_trace_id(&trace_id_nf)
                .with_plugin_name(&hook_name_nf)
                .with_field(
                    "resolver_name",
                    serde_json::Value::String(missing_name.to_string()),
                );
            log.write(&ev);
        }
    };

    // AC-007 / SOUL #4: emit resolver.error when a resolver returns Err.
    // F-P4-001A / F-P5-001: error_kind uses snake_case serde tag (HOST_ABI line 1095).
    // F-P5-002: error_detail (singular) is the Display string (HOST_ABI line 1096).
    //           event_type carries the Claude Code envelope event type (HOST_ABI line 1097).
    //           This is distinct from ResolverInput.hook_event_name (registry entry name).
    let emit_resolver_error = {
        let log = internal_log.clone();
        move |err_name: &str, err: &crate::resolver::ResolverError| {
            let err_json = serde_json::to_value(err)
                .unwrap_or_else(|_| serde_json::json!({"kind": "unknown"}));
            let error_kind = err_json
                .get("kind")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let ev = InternalEvent::now("resolver.error")
                .with_trace_id(&trace_id_err)
                .with_plugin_name(&hook_name_err)
                .with_field(
                    "resolver_name",
                    serde_json::Value::String(err_name.to_string()),
                )
                .with_field("error_kind", serde_json::Value::String(error_kind))
                .with_field("error_detail", serde_json::Value::String(format!("{err}")))
                .with_field(
                    "event_type",
                    serde_json::Value::String(event_type_for_log.clone()),
                );
            log.write(&ev);
        }
    };

    // resolve_context_for_entry returns Vec<ResolvedContext> in declaration order
    // (BC-1.13.001 PC7 / F-P5-003 Option A / F-P2-002 / F-P3-001).
    // Each ResolvedContext carries context_key (merge key), resolver_name
    // (registry name for telemetry), and output.
    let resolver_outputs = resolver_registry.resolve_context_for_entry(
        &entry.needs_context,
        &resolver_input,
        emit_not_found,
        emit_resolver_error,
    );

    // AC-007: merge_resolver_outputs is pure (BC-4.12.005 INV1, architect Path B).
    // Collisions are returned as Vec<CollisionInfo>; caller emits telemetry for each.
    // F-P5-003 / F-P2-002 / F-P3-001: ResolvedContext carries both context_key (merge key)
    // and resolver_name (registry name) so CollisionInfo.resolver_name is the registry NAME,
    // not the context_key.
    let (merged_map, collisions) = merge_resolver_outputs(static_map, &resolver_outputs);

    // F-P4-001B / F-P2-002 / F-P3-001: emit resolver_name (registry NAME, not context_key)
    // in each merge_collision event for per-resolver traceability (BC-4.12.004 wire format).
    for collision in collisions {
        let ev = InternalEvent::now("resolver.merge_collision")
            .with_trace_id(trace_id)
            .with_plugin_name(&hook_name)
            .with_field("key", serde_json::Value::String(collision.key))
            .with_field(
                "resolver_name",
                serde_json::Value::String(collision.resolver_name),
            )
            .with_field("static_value", collision.old_value)
            .with_field("resolver_value", collision.new_value);
        internal_log.write(&ev);
    }

    serde_json::Value::Object(merged_map)
}

fn plugin_requests_block(result: &PluginResult) -> bool {
    let stdout = match result {
        PluginResult::Ok { stdout, .. } => stdout.as_str(),
        _ => return false,
    };
    // Plugins serialize `HookResult::Block { reason }` as
    // `{"outcome":"block","reason":"..."}`. The simplest durable
    // detector is a substring check against the tagged serde output —
    // we're not committing to parsing the full JSON here because the
    // contract is stable (HOST_ABI.md) and a fuller parse can be
    // layered on when the internal log needs the reason.
    stdout.contains(r#""outcome":"block""#)
}

/// Fail-closed semantics for sync-group gate hooks (ADR-019 §Decision 2).
///
/// Returns `true` when a sync-group plugin Crashed or timed out AND its
/// registry entry declared `on_error = block`. In this case the dispatcher
/// must exit 2 even though the crashed plugin never emitted stdout.
///
/// **Async hooks MUST NOT call this path.** `execute_tiers` is called only
/// for sync-group plugins; async hooks go through `spawn_async_plugin` and
/// are excluded from gate decisions by the structural partition (ADR-019
/// async semantics — async verdicts are advisory-only).
///
/// # BC traces
/// - ADR-019 §Decision 2 — fail-closed semantics
/// - BC-1.14.001 Error Paths — Crashed+on_error=Block exits 2
/// - BC-7.06.001 Invariant 1 — sync gate hooks must not silently fail open
fn plugin_fail_closed(result: &PluginResult, on_error: OnError) -> bool {
    if on_error != OnError::Block {
        return false;
    }
    matches!(
        result,
        PluginResult::Crashed { .. } | PluginResult::Timeout { .. }
    )
}

fn emit_invoked(log: &InternalLog, base_ctx: &HostContext, entry: &RegistryEntry) {
    let ev = InternalEvent::now(PLUGIN_INVOKED)
        .with_trace_id(&base_ctx.dispatcher_trace_id)
        .with_session_id(&base_ctx.session_id)
        .with_plugin_name(&entry.name)
        .with_plugin_version(&base_ctx.plugin_version)
        .with_field("event", serde_json::Value::String(entry.event.clone()));
    log.write(&ev);
}

fn emit_lifecycle(
    log: &InternalLog,
    base_ctx: &HostContext,
    entry: &RegistryEntry,
    result: &PluginResult,
) {
    let (event_type, elapsed, fuel, mut extra_fields) = match result {
        PluginResult::Ok {
            exit_code,
            elapsed_ms,
            fuel_consumed,
            stderr,
            ..
        } => (
            PLUGIN_COMPLETED,
            *elapsed_ms,
            *fuel_consumed,
            vec![
                ("exit_code".to_string(), serde_json::Value::from(*exit_code)),
                (
                    "stderr".to_string(),
                    serde_json::Value::String(stderr.clone()),
                ),
            ],
        ),
        PluginResult::Timeout {
            cause,
            stderr,
            elapsed_ms,
            fuel_consumed,
        } => {
            let cause_str = match cause {
                TimeoutCause::Epoch => "epoch",
                TimeoutCause::Fuel => "fuel",
            };
            (
                PLUGIN_TIMEOUT,
                *elapsed_ms,
                *fuel_consumed,
                vec![
                    (
                        "cause".to_string(),
                        serde_json::Value::String(cause_str.to_string()),
                    ),
                    (
                        "stderr".to_string(),
                        serde_json::Value::String(stderr.clone()),
                    ),
                ],
            )
        }
        PluginResult::Crashed {
            trap_string,
            stderr,
            elapsed_ms,
            fuel_consumed,
        } => (
            PLUGIN_CRASHED,
            *elapsed_ms,
            *fuel_consumed,
            vec![
                (
                    "trap".to_string(),
                    serde_json::Value::String(trap_string.clone()),
                ),
                (
                    "stderr".to_string(),
                    serde_json::Value::String(stderr.clone()),
                ),
            ],
        ),
    };
    // Drop empty stderr from the lifecycle event payload — it's the
    // common case (well-behaved plugins write nothing to stderr) and a
    // dangling "stderr": "" field is just visual noise in the log.
    extra_fields.retain(|(k, v)| k != "stderr" || !v.as_str().map(str::is_empty).unwrap_or(false));

    let mut ev = InternalEvent::now(event_type)
        .with_trace_id(&base_ctx.dispatcher_trace_id)
        .with_session_id(&base_ctx.session_id)
        .with_plugin_name(&entry.name)
        .with_plugin_version(&base_ctx.plugin_version)
        .with_field("elapsed_ms", serde_json::Value::from(elapsed))
        .with_field("fuel_consumed", serde_json::Value::from(fuel));
    for (k, v) in extra_fields {
        ev = ev.with_field(&k, v);
    }
    log.write(&ev);
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── CRIT-PR59-001 regression tests: advisory-block gate ──────────────────

    /// Advisory block fires when on_error=Continue and stdout contains outcome:block.
    /// Regression for CRIT-PR59-001: the AND-gate `on_error==Block &&` was removed;
    /// stdout `{"outcome":"block"}` is now sufficient regardless of on_error.
    #[test]
    fn advisory_block_fires_with_on_error_continue() {
        let outcome = PluginOutcome {
            plugin_name: "test-plugin".to_string(),
            plugin_version: "0.1.0".to_string(),
            on_error: OnError::Continue,
            result: PluginResult::Ok {
                exit_code: 0,
                stdout: r#"{"outcome":"block","reason":"test"}"#.to_string(),
                stderr: String::new(),
                elapsed_ms: 1,
                fuel_consumed: 10,
            },
        };
        assert!(
            plugin_requests_block(&outcome.result),
            "stdout outcome:block must be detected regardless of on_error"
        );
        // Aggregate to verify exit_code=2 path
        let block = plugin_requests_block(&outcome.result);
        assert!(block, "block_intent must be true");
    }

    /// Regression: on_error=Continue with no stdout block → no block intent.
    #[test]
    fn advisory_block_absent_with_on_error_continue_and_no_block_stdout() {
        let outcome = PluginOutcome {
            plugin_name: "test-plugin".to_string(),
            plugin_version: "0.1.0".to_string(),
            on_error: OnError::Continue,
            result: PluginResult::Ok {
                exit_code: 0,
                stdout: "all good, no block here".to_string(),
                stderr: String::new(),
                elapsed_ms: 1,
                fuel_consumed: 5,
            },
        };
        assert!(
            !plugin_requests_block(&outcome.result),
            "no outcome:block stdout → block_intent must be false"
        );
    }

    #[test]
    fn plugin_requests_block_detects_tagged_json() {
        let r = PluginResult::Ok {
            exit_code: 2,
            stdout: r#"{"outcome":"block","reason":"policy 9"}"#.to_string(),
            stderr: String::new(),
            elapsed_ms: 3,
            fuel_consumed: 10,
        };
        assert!(plugin_requests_block(&r));
    }

    #[test]
    fn plugin_requests_block_false_for_continue() {
        let r = PluginResult::Ok {
            exit_code: 0,
            stdout: r#"{"outcome":"continue"}"#.to_string(),
            stderr: String::new(),
            elapsed_ms: 2,
            fuel_consumed: 5,
        };
        assert!(!plugin_requests_block(&r));
    }

    #[test]
    fn plugin_requests_block_false_for_crash() {
        let r = PluginResult::Crashed {
            trap_string: "unreachable".to_string(),
            stderr: "panicked at 'unreachable'".to_string(),
            elapsed_ms: 1,
            fuel_consumed: 0,
        };
        assert!(!plugin_requests_block(&r));
    }

    #[test]
    fn plugin_requests_block_false_for_timeout() {
        let r = PluginResult::Timeout {
            cause: TimeoutCause::Epoch,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: 0,
        };
        assert!(!plugin_requests_block(&r));
    }

    // ── ADR-019 §Decision 2 fail-closed tests: plugin_fail_closed ────────────

    /// Crashed + on_error=Block → fail-closed (exit 2).
    /// This is the TC-8 root cause: WASI trap doesn't set exit_code; the
    /// aggregator must detect Crashed+Block independently.
    ///
    /// ADR-019 §Decision 2, BC-1.14.001 Error Paths, BC-7.06.001 Invariant 1.
    #[test]
    fn fail_closed_crashes_with_on_error_block() {
        let r = PluginResult::Crashed {
            trap_string: "unreachable".to_string(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 0,
        };
        assert!(
            plugin_fail_closed(&r, OnError::Block),
            "Crashed + on_error=Block must trigger fail-closed"
        );
    }

    /// Crashed + on_error=Continue → NOT fail-closed (fail-open, normal advisory path).
    #[test]
    fn fail_closed_crash_with_on_error_continue_is_open() {
        let r = PluginResult::Crashed {
            trap_string: "unreachable".to_string(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 0,
        };
        assert!(
            !plugin_fail_closed(&r, OnError::Continue),
            "Crashed + on_error=Continue must NOT trigger fail-closed"
        );
    }

    /// Timeout + on_error=Block → fail-closed (exit 2).
    /// A timed-out gate hook also cannot emit stdout; fail-closed must apply.
    ///
    /// ADR-019 §Decision 2.
    #[test]
    fn fail_closed_timeout_with_on_error_block() {
        let r = PluginResult::Timeout {
            cause: TimeoutCause::Epoch,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: 0,
        };
        assert!(
            plugin_fail_closed(&r, OnError::Block),
            "Timeout + on_error=Block must trigger fail-closed"
        );
    }

    /// Timeout + on_error=Continue → NOT fail-closed.
    #[test]
    fn fail_closed_timeout_with_on_error_continue_is_open() {
        let r = PluginResult::Timeout {
            cause: TimeoutCause::Fuel,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: 1_000_000_000,
        };
        assert!(
            !plugin_fail_closed(&r, OnError::Continue),
            "Timeout + on_error=Continue must NOT trigger fail-closed"
        );
    }

    /// Ok result + on_error=Block → NOT fail-closed (advisory path handles this).
    #[test]
    fn fail_closed_ok_result_is_not_fail_closed() {
        let r = PluginResult::Ok {
            exit_code: 0,
            stdout: r#"{"outcome":"continue"}"#.to_string(),
            stderr: String::new(),
            elapsed_ms: 10,
            fuel_consumed: 100,
        };
        assert!(
            !plugin_fail_closed(&r, OnError::Block),
            "Ok result + on_error=Block must NOT trigger fail-closed (advisory path handles Ok)"
        );
    }
}
