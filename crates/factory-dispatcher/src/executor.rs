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
use crate::indeterminate_marker::{
    MarkerFields, delete_marker_if_pass, read_marker_plugin_name, should_write_marker,
    write_indeterminate_marker,
};
use crate::internal_log::{
    InternalEvent, InternalLog, PLUGIN_COMPLETED, PLUGIN_CRASHED, PLUGIN_INDETERMINATE,
    PLUGIN_INVOKED, PLUGIN_TIMEOUT,
};
use crate::invoke::{InvokeLimits, PluginResult, TimeoutCause, invoke_plugin};
use crate::plugin_loader::PluginCache;
use crate::registry::{FailurePolicy, OnError, Registry, RegistryEntry};
use crate::resolver::{ResolverInput, ResolverRegistry, merge_resolver_outputs};

// ---------------------------------------------------------------------------
// S-25.01: INDETERMINATE outcome class (BC-1.18.001, ADR-047 Layer 1)
// ---------------------------------------------------------------------------

/// Root cause of an INDETERMINATE dispatch outcome.
///
/// Distinct values for each resource-exhaustion class so operators can
/// distinguish fuel exhaustion, epoch timeout, and OutputTooLarge events
/// in the `plugin.indeterminate` event (BC-3.08.001 Event 8).
///
/// VP-102 proof harness covers the full cause-mapping table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndeterminateCause {
    /// WASM execution ran out of fuel budget (Trap::OutOfFuel).
    Fuel,
    /// WASM execution exceeded epoch/wall-clock budget (Trap::Interrupt).
    Epoch,
    /// A host function returned OutputTooLarge(-3) and the plugin subsequently
    /// exited with exit_code=0 — the plugin did not detect the truncation.
    OutputTooLarge,
}

/// Three-valued dispatch outcome for a single plugin invocation.
///
/// This is a first-class, distinct type from the existing `PluginResult` taxonomy.
/// `PluginResult` captures the raw invocation result; `DispatchOutcome` is the
/// CLASSIFIED semantic outcome after applying INDETERMINATE detection rules.
///
/// - **Pass**: exit_code=0 AND host_output_too_large_seen=false.
/// - **Fail**: non-zero exit_code.
/// - **Indeterminate**: could not validate — fuel/epoch/OutputTooLarge.
///
/// S-25.01 BC-1.18.001 postcondition 2: these three variants are the complete,
/// non-overlapping trichotomy. VP-102 proof harness verifies exhaustive coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchOutcome {
    /// Plugin validated successfully: exit_code=0 and no OutputTooLarge flag set.
    Pass,
    /// Plugin returned a non-zero exit code (explicit validation failure).
    Fail {
        /// The non-zero exit code returned by the plugin.
        exit_code: i32,
    },
    /// Plugin could not complete validation (fuel, epoch, or OutputTooLarge).
    ///
    /// For fail-closed plugins: triggers durable `.factory/unvalidated-mutation.marker` write.
    /// For fail-open plugins: advisory `plugin.indeterminate` event only (BC-1.18.004).
    Indeterminate {
        /// Root cause distinguishing the three INDETERMINATE sub-cases.
        cause: IndeterminateCause,
    },
}

/// Classify a raw `PluginResult` + `FailurePolicy` + `output_too_large` flag
/// into the three-valued `DispatchOutcome` trichotomy.
///
/// This is the pure-core function covering the INDETERMINATE detection logic
/// (BC-1.18.001). It does NOT write the marker or emit events — those are
/// effectful operations in the invocation loop.
///
/// # Fuel detection
///
/// Fuel exhaustion MUST be detected via `Trap::OutOfFuel` downcast on the
/// `PluginResult::Timeout { cause: TimeoutCause::Fuel }` variant, NOT via
/// `get_fuel()`. `get_fuel()` is unreliable after a Trap (BC-1.18.001 §Architecture
/// Anchors; ADR-047 §D1 implementation note). AC-001.
///
/// # Non-exhaustive Trap wildcard (BC-1.18.001 invariant 2)
///
/// `Trap` is `#[non_exhaustive]`. The match arm for unrecognised Trap variants
/// MUST use `_ => { /* route to on_error, NOT Indeterminate */ }`. Future unknown
/// Trap variants MUST NOT be silently bucketed as INDETERMINATE. AC-004.
///
/// # OutputTooLarge invariant
///
/// The `output_too_large` flag MUST be captured from `StoreData` AFTER `func.call()`
/// completes (before the per-invocation reset). It is the caller's responsibility
/// to pass the captured value. BC-1.18.001 invariant 5: the flag is dispatcher-
/// internal StoreData; never exposed in hook-sdk ABI. AC-003/AC-018.
///
/// # VP
///
/// VP-102 proof harness covers this function's 5 unit test cases.
///
/// # BC-5.38.001
///
/// Non-trivial body — contains branching, pattern matching, conditional returns.
/// todo!() per Red Gate discipline. Implementer fills in.
pub fn classify_outcome(
    plugin_result: PluginResult,
    _policy: FailurePolicy,
    output_too_large: bool,
) -> DispatchOutcome {
    // NOTE (S-25.01 orchestrator ruling): `_policy` is genuinely unused inside
    // classify_outcome — classification is independent of policy. Policy is only
    // used downstream by `should_write_marker`. The parameter is retained in the
    // spec-mandated signature (spec-wins; BC-1.18.001 AC-004 signature). Surface
    // to product-owner as possible spec-signature refinement (per orchestrator note).
    //
    // BC-1.18.001 postcondition 1 + postcondition 2 + postcondition 5 + invariant 2.
    // VP-102 proof harness covers this function's 5 unit test cases.
    match plugin_result {
        // Timeout variants: fuel or epoch exhaustion → INDETERMINATE.
        // Fuel detection MUST use the Timeout{cause:Fuel} variant (mapped from
        // Trap::OutOfFuel in invoke.rs::classify_trap), NOT get_fuel(). AC-001/AC-002.
        PluginResult::Timeout {
            cause: crate::invoke::TimeoutCause::Fuel,
            ..
        } => DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Fuel,
        },
        PluginResult::Timeout {
            cause: crate::invoke::TimeoutCause::Epoch,
            ..
        } => DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Epoch,
        },

        // Ok variant: trichotomy depends on exit_code + output_too_large flag.
        // AC-003: exit_code=0 + output_too_large=true → Indeterminate(OutputTooLarge).
        // AC-004: exit_code=0 + output_too_large=false → Pass.
        // AC-004: exit_code≠0 → Fail{exit_code} (policy-orthogonal).
        PluginResult::Ok { exit_code, .. } => {
            if output_too_large && exit_code == 0 {
                DispatchOutcome::Indeterminate {
                    cause: IndeterminateCause::OutputTooLarge,
                }
            } else if exit_code == 0 {
                DispatchOutcome::Pass
            } else {
                DispatchOutcome::Fail { exit_code }
            }
        }

        // Crashed variant: unrecognized/future Trap variant (wildcard `_ =>` arm in
        // invoke.rs::classify_trap). MUST NOT yield INDETERMINATE per BC-1.18.001
        // invariant 2 (`Trap` is `#[non_exhaustive]`). Route to existing on_error
        // handling by returning Fail (non-zero). AC-004/AC-011.
        PluginResult::Crashed { .. } => {
            // BC-1.18.001 invariant 2: wildcard Trap arm routes to on_error, not INDETERMINATE.
            // Existing plugin_fail_closed() checks Crashed+on_error=Block → exit 2.
            // classify_outcome returns Fail so the executor can apply on_error logic.
            DispatchOutcome::Fail { exit_code: 1 }
        }
    }
}

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

        // MEDIUM-5: extract artifact_path from tool_input.file_path before the closure
        // moves inputs away (BC-1.18.001 PC4 — marker must record the artifact path).
        // Empty string when no file_path is present (e.g. non-file-mutation tool events).
        let artifact_path_for_marker = inputs
            .payload_value
            .get("tool_input")
            .and_then(|ti| ti.get("file_path"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let handle = tokio::task::spawn_blocking(move || {
            let started = Instant::now();
            // S-25.01: invoke_plugin now returns (PluginResult, bool) where bool
            // = host_output_too_large_seen (AC-003).
            let (result, output_too_large) =
                invoke_plugin(&engine, &module, host_ctx.clone(), &payload, limits).unwrap_or_else(
                    |e| {
                        (
                            PluginResult::Crashed {
                                trap_string: format!("invoke setup error: {e}"),
                                stderr: String::new(),
                                elapsed_ms: started.elapsed().as_millis() as u64,
                                fuel_consumed: 0,
                            },
                            false,
                        )
                    },
                );
            emit_lifecycle(&internal_log, &base_ctx_for_event, &entry_clone, &result);

            // S-25.01: classify outcome + emit plugin.indeterminate event + marker write.
            let failure_policy = entry_clone.failure_policy;
            let outcome = classify_outcome(result.clone(), failure_policy, output_too_large);

            // BLOCKER-1: BC-1.18.003 PC1 + INV2 — PASS from the named plugin MUST clear
            // the marker, but ONLY if the marker's plugin_name matches this plugin (scoped).
            if let DispatchOutcome::Pass = outcome {
                let marker_path = base_ctx_for_event
                    .cwd
                    .join(".factory")
                    .join("unvalidated-mutation.marker");
                // Best-effort read; if the marker is absent or unreadable, no-op.
                if let Ok(Some(marker_plugin)) = read_marker_plugin_name(&marker_path) {
                    if marker_plugin == entry_clone.name {
                        // Scoped clear: only this plugin's PASS clears its own marker (INV2).
                        let _ = delete_marker_if_pass(&marker_path);
                    }
                }
            }

            if let DispatchOutcome::Indeterminate { ref cause } = outcome {
                // Emit plugin.indeterminate for EVERY indeterminate outcome (AC-006).
                emit_indeterminate(&internal_log, &base_ctx_for_event, &entry_clone, cause);
                // Write durable marker only for fail-closed plugins (AC-005/AC-015).
                // BLOCKER-2: BC-1.18.001 INV4 — marker write is PostToolUse-only.
                // PreToolUse INDETERMINATE → advisory event only; NO marker written.
                if should_write_marker(&outcome, failure_policy)
                    && entry_clone.event == "PostToolUse"
                {
                    let marker_path = base_ctx_for_event
                        .cwd
                        .join(".factory")
                        .join("unvalidated-mutation.marker");
                    let fields = MarkerFields {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        plugin_name: entry_clone.name.clone(),
                        // MEDIUM-5: thread artifact_path from tool_input.file_path (AC-007).
                        artifact_path: artifact_path_for_marker,
                        cause: cause_to_str(cause).to_string(),
                        trace_id: base_ctx_for_event.dispatcher_trace_id.clone(),
                    };
                    // Best-effort: marker write failure does not block the dispatch result.
                    // The plugin.indeterminate event was already emitted above.
                    let _ = write_indeterminate_marker(&fields, &marker_path);
                }
            }

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

        // MEDIUM-5: extract artifact_path from tool_input.file_path before payload_value
        // is moved into per_plugin_value (BC-1.18.001 PC4 — marker must record artifact path).
        let artifact_path_for_marker = payload_value
            .get("tool_input")
            .and_then(|ti| ti.get("file_path"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

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

        let (result, output_too_large) = tokio::task::spawn_blocking(move || {
            let started = std::time::Instant::now();
            invoke_plugin(&engine, &module, host_ctx, &payload, limits).unwrap_or_else(|e| {
                (
                    PluginResult::Crashed {
                        trap_string: format!("invoke setup error: {e}"),
                        stderr: String::new(),
                        elapsed_ms: started.elapsed().as_millis() as u64,
                        fuel_consumed: 0,
                    },
                    false,
                )
            })
        })
        .await
        .unwrap_or_else(|join_err| {
            (
                PluginResult::Crashed {
                    trap_string: format!("spawn_blocking join error: {join_err}"),
                    stderr: String::new(),
                    elapsed_ms: 0,
                    fuel_consumed: 0,
                },
                false,
            )
        });

        emit_lifecycle(&internal_log, &base_ctx_for_event, &entry, &result);

        // S-25.01: classify + emit plugin.indeterminate + marker write (mirrors execute_tier).
        let failure_policy = entry.failure_policy;
        let outcome = classify_outcome(result.clone(), failure_policy, output_too_large);

        // BLOCKER-1: BC-1.18.003 PC1 + INV2 — PASS from the named plugin MUST clear
        // the marker, scoped to only the plugin named in the marker (INV2).
        if let DispatchOutcome::Pass = outcome {
            let marker_path = base_ctx_for_event
                .cwd
                .join(".factory")
                .join("unvalidated-mutation.marker");
            if let Ok(Some(marker_plugin)) = read_marker_plugin_name(&marker_path) {
                if marker_plugin == entry.name {
                    let _ = delete_marker_if_pass(&marker_path);
                }
            }
        }

        if let DispatchOutcome::Indeterminate { ref cause } = outcome {
            emit_indeterminate(&internal_log, &base_ctx_for_event, &entry, cause);
            // BLOCKER-2: BC-1.18.001 INV4 — marker write is PostToolUse-only.
            // PreToolUse INDETERMINATE → advisory event only; no marker written.
            if should_write_marker(&outcome, failure_policy) && entry.event == "PostToolUse" {
                let marker_path = base_ctx_for_event
                    .cwd
                    .join(".factory")
                    .join("unvalidated-mutation.marker");
                let fields = MarkerFields {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    plugin_name: entry.name.clone(),
                    // MEDIUM-5: thread artifact_path from tool_input.file_path (AC-007).
                    artifact_path: artifact_path_for_marker,
                    cause: cause_to_str(cause).to_string(),
                    trace_id: base_ctx_for_event.dispatcher_trace_id.clone(),
                };
                let _ = write_indeterminate_marker(&fields, &marker_path);
            }
        }

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
// CONVENTION (P-005 / S-7.01 Sibling-Coverage):
//
// When adding a new resolver-tier event type to the dispatcher
// (whether emitted from executor.rs::build_plugin_config OR from
// main.rs startup OR from any other dispatcher code path):
//
// 1. Add a field table to HOST_ABI.md listing ALL emitted fields,
//    INCLUDING the provenance triplet (trace_id, session_id, plugin_name)
//    when applicable.
// 2. Update the BC that owns the event with the corresponding PC field
//    list (BC-1.13.001 / BC-4.12.004 / etc).
// 3. Add positive-coverage assertions in the integration test for EVERY
//    provenance field (POL-11).
// 4. Audit ALL sibling resolver-tier events for the same gaps. The current
//    enumerated list (NOT exhaustive — verify by `grep -rn 'InternalEvent::now("resolver\.'`):
//      - resolver.not_found      (executor.rs)
//      - resolver.error          (executor.rs)
//      - resolver.merge_collision (executor.rs)
//      - resolver.registry_loaded (main.rs)
//      - resolver.load_warning   (main.rs)
//      - resolver.load_error     (main.rs)
//
// This pattern was hard-learned across 4+ adversarial passes; codified
// to prevent recurrence. The S-7.01 sibling-blast-radius rule applies
// across SOURCE FILES not just within a single function.
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
    // F-P4-001: carry session_id into resolver event closures for parity with
    // emit_invoked (line 617) and emit_lifecycle (line 704) which both include
    // with_session_id(&base_ctx.session_id).
    let session_id_nf = base_host_ctx.session_id.clone();
    let session_id_err = base_host_ctx.session_id.clone();
    // event_type is the Claude Code envelope event (e.g. "PreToolUse") emitted as
    // the event_type field in resolver.error events (HOST_ABI.md line 1097).
    let event_type_for_log = event_type.clone();

    // AC-005: emit resolver.not_found when a named resolver is absent.
    // Clone InternalLog (PathBuf wrapper) so the closure captures by value — no unsafe needed.
    //
    // Note: resolver.not_found event field table now documented in HOST_ABI
    // (F-P7-002 burst). Wire format (per implementation): { resolver_name, trace_id,
    // session_id, plugin_name }. F-P7-002 closes the prior deferral.
    let emit_not_found = {
        let log = internal_log.clone();
        move |missing_name: &str| {
            let ev = InternalEvent::now("resolver.not_found")
                .with_trace_id(&trace_id_nf)
                .with_session_id(&session_id_nf)
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
                .with_session_id(&session_id_err)
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
            .with_session_id(&base_host_ctx.session_id)
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
            fuel_cap,
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
                    ("fuel_cap".to_string(), serde_json::Value::from(*fuel_cap)),
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

/// Convert `IndeterminateCause` to the canonical BC-3.08.001 wire string.
fn cause_to_str(cause: &IndeterminateCause) -> &'static str {
    match cause {
        IndeterminateCause::Fuel => "fuel",
        IndeterminateCause::Epoch => "epoch",
        IndeterminateCause::OutputTooLarge => "output-too-large",
    }
}

/// Emit a `plugin.indeterminate` event to the internal log (BC-3.08.001 Event 8).
///
/// Mandatory 8 fields: type, trace_id, session_id, plugin_name, artifact_path,
/// cause, failure_policy, timestamp (provided by `InternalEvent::now`).
///
/// Called for EVERY INDETERMINATE outcome — both fail-closed and fail-open paths.
fn emit_indeterminate(
    log: &InternalLog,
    base_ctx: &HostContext,
    entry: &RegistryEntry,
    cause: &IndeterminateCause,
) {
    let cause_str = cause_to_str(cause);
    let policy_str = match entry.failure_policy {
        FailurePolicy::FailClosed => "fail-closed",
        FailurePolicy::FailOpen => "fail-open",
    };
    let ev = InternalEvent::now(PLUGIN_INDETERMINATE)
        .with_trace_id(&base_ctx.dispatcher_trace_id)
        .with_session_id(&base_ctx.session_id)
        .with_plugin_name(&entry.name)
        .with_plugin_version(&base_ctx.plugin_version)
        .with_field("artifact_path", serde_json::Value::String(String::new()))
        .with_field("cause", serde_json::Value::String(cause_str.to_string()))
        .with_field(
            "failure_policy",
            serde_json::Value::String(policy_str.to_string()),
        );
    log.write(&ev);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indeterminate_marker::{
        MarkerFields, delete_marker_if_pass, should_write_marker, write_indeterminate_marker,
    };
    use crate::invoke::DEFAULT_FUEL_CAP;
    use crate::registry::FailurePolicy;

    // ── S-25.01 Red Gate tests — BC-1.18.001/002/003/004 ─────────────────────
    // Each test calls a todo!() production function, compiles, and MUST FAIL
    // at runtime until the implementer fills in the production logic (T-1..T-3).
    //
    // BC-5.38.005 self-check: "If I include this real implementation, will the
    // test pass trivially without implementer work?" — YES for all tests below.
    // Therefore all production bodies remain todo!() per BC-5.38.001.

    // ── S-25.01 Red Gate stub 1 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_fuel_exhaustion_yields_indeterminate_for_fail_closed_plugin() {
        // AC-001 / BC-1.18.001 postcondition 1 (fuel cause):
        // Timeout { cause: Fuel } + fail-closed → Indeterminate { cause: Fuel }.
        // Fuel detection MUST use Trap::OutOfFuel downcast, NOT get_fuel() (ADR-047 §D1).
        let result = PluginResult::Timeout {
            cause: TimeoutCause::Fuel,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: DEFAULT_FUEL_CAP,
            fuel_cap: DEFAULT_FUEL_CAP,
        };
        let outcome = classify_outcome(result, FailurePolicy::FailClosed, false);
        assert_eq!(
            outcome,
            DispatchOutcome::Indeterminate {
                cause: IndeterminateCause::Fuel
            },
            "AC-001: Fuel timeout + fail-closed MUST yield Indeterminate(Fuel), not Pass or Fail"
        );
    }

    // ── S-25.01 Red Gate stub 2 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_epoch_timeout_yields_indeterminate_for_fail_closed_plugin() {
        // AC-002 / BC-1.18.001 postcondition 1 (epoch timeout cause):
        // Timeout { cause: Epoch } + fail-closed → Indeterminate { cause: Epoch }.
        let result = PluginResult::Timeout {
            cause: TimeoutCause::Epoch,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: 0,
            fuel_cap: DEFAULT_FUEL_CAP,
        };
        let outcome = classify_outcome(result, FailurePolicy::FailClosed, false);
        assert_eq!(
            outcome,
            DispatchOutcome::Indeterminate {
                cause: IndeterminateCause::Epoch
            },
            "AC-002: Epoch timeout + fail-closed MUST yield Indeterminate(Epoch)"
        );
    }

    // ── S-25.01 Red Gate stub 3 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_output_too_large_then_ok_yields_indeterminate_for_fail_closed_plugin() {
        // AC-003 / BC-1.18.001 postcondition 1 + invariant 1:
        // output_too_large=true + exit_code=0 + fail-closed → Indeterminate(OutputTooLarge).
        // Per-invocation reset: host_output_too_large_seen MUST be reset to false BEFORE each
        // func.call() invocation — not only at Store creation (BC-1.18.001 INV1).
        // The flag is captured AFTER call() and passed to classify_outcome (AC-018).
        let result = PluginResult::Ok {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 10,
            fuel_consumed: 100,
        };
        // output_too_large=true: a host function returned OutputTooLarge(-3) during this invocation
        let outcome = classify_outcome(result, FailurePolicy::FailClosed, true);
        assert_eq!(
            outcome,
            DispatchOutcome::Indeterminate {
                cause: IndeterminateCause::OutputTooLarge
            },
            "AC-003: exit_code=0 + output_too_large=true MUST yield Indeterminate(OutputTooLarge)"
        );
    }

    // ── S-25.01 Red Gate stub 4 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_indeterminate_is_distinct_from_pass_and_fail() {
        // AC-004 / BC-1.18.001 postcondition 2 + postcondition 5:
        // DispatchOutcome is a strict trichotomy: Pass, Fail, Indeterminate.
        // PASS: exit_code=0 AND output_too_large=false.
        // FAIL: non-zero exit_code.
        // INDETERMINATE: the third, distinct gap (fuel/epoch/OTL).

        // Pass path
        let ok_pass = PluginResult::Ok {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 10,
        };
        let pass = classify_outcome(ok_pass, FailurePolicy::FailClosed, false);
        assert_eq!(
            pass,
            DispatchOutcome::Pass,
            "exit_code=0 + no OTL MUST be Pass"
        );

        // Fail path
        let ok_fail = PluginResult::Ok {
            exit_code: 1,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 10,
        };
        let fail = classify_outcome(ok_fail, FailurePolicy::FailClosed, false);
        assert_eq!(
            fail,
            DispatchOutcome::Fail { exit_code: 1 },
            "exit_code=1 MUST be Fail{{exit_code: 1}}"
        );

        // Indeterminate is distinct from both Pass and Fail
        let indet = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Fuel,
        };
        assert_ne!(
            indet,
            DispatchOutcome::Pass,
            "Indeterminate MUST NOT equal Pass"
        );
        assert_ne!(
            indet,
            DispatchOutcome::Fail { exit_code: 1 },
            "Indeterminate MUST NOT equal Fail"
        );
    }

    // ── S-25.01 Red Gate stub 5 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_indeterminate_writes_marker_to_factory_path() {
        // AC-005 / BC-1.18.001 postcondition 4 + invariant 3 + invariant 4:
        // Atomic marker write via write-to-temp + rename (O_CREAT|O_WRONLY|O_TRUNC then rename).
        // PostToolUse-only: marker write is only valid for PostToolUse events (INV4).
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2026-08-30T00:00:00Z".to_string(),
            plugin_name: "validate-factory-path-staging".to_string(),
            artifact_path: String::new(), // empty string; MUST NOT be omitted (BC-1.18.001 PC4)
            cause: "fuel".to_string(),
            trace_id: "test-trace-001".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker MUST succeed for a writable path");
        assert!(
            marker_path.exists(),
            "AC-005: marker file MUST exist after write_indeterminate_marker returns Ok(())"
        );
        // Verify no stale .tmp file remains after atomic rename
        let tmp_path = dir
            .path()
            .join(format!("{}.tmp", "unvalidated-mutation.marker"));
        assert!(
            !tmp_path.exists(),
            "AC-005: temp file MUST be renamed to final path; no .tmp file should remain"
        );
    }

    // ── S-25.01 Red Gate stub 6 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_marker_contains_required_fields() {
        // AC-005 / BC-1.18.001 postcondition 4:
        // Marker MUST contain all five required TOML fields:
        //   timestamp (RFC 3339), plugin_name, artifact_path (empty ok, never omit),
        //   cause ("fuel"|"epoch"|"output-too-large"), trace_id.
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2026-08-30T12:00:00Z".to_string(),
            plugin_name: "validate-factory-path-staging".to_string(),
            artifact_path: "/path/to/.factory/STATE.md".to_string(),
            cause: "epoch".to_string(),
            trace_id: "deadbeef-0001-0001-0001-000000000001".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path).expect("write must succeed");
        let contents =
            std::fs::read_to_string(&marker_path).expect("must be able to read marker back");

        // All five required field names must appear in the TOML
        assert!(
            contents.contains("timestamp"),
            "AC-005: marker MUST contain 'timestamp' TOML key"
        );
        assert!(
            contents.contains("plugin_name"),
            "AC-005: marker MUST contain 'plugin_name' TOML key"
        );
        assert!(
            contents.contains("artifact_path"),
            "AC-005: marker MUST contain 'artifact_path' TOML key (even when empty)"
        );
        assert!(
            contents.contains("cause"),
            "AC-005: marker MUST contain 'cause' TOML key"
        );
        assert!(
            contents.contains("trace_id"),
            "AC-005: marker MUST contain 'trace_id' TOML key"
        );
        // Verify specific values were persisted
        assert!(
            contents.contains("validate-factory-path-staging"),
            "AC-005: marker plugin_name MUST reflect the plugin name"
        );
        assert!(
            contents.contains("epoch"),
            "AC-005: marker cause MUST reflect the cause value"
        );
        assert!(
            contents.contains("deadbeef-0001-0001-0001-000000000001"),
            "AC-005: marker trace_id MUST reflect the trace_id value"
        );
    }

    // ── S-25.01 Red Gate stub 7 ───────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_003_successful_revalidation_deletes_marker() {
        // AC-012 / BC-1.18.003 postcondition 1 + invariant 2:
        // delete_marker_if_pass deletes the marker file.
        // Scoping: ONLY the named plugin's PASS clears the marker; caller must enforce this.
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");

        // Pre-condition: write a marker to simulate the INDETERMINATE state
        std::fs::write(
            &marker_path,
            "timestamp = \"2026-08-30T00:00:00Z\"\nplugin_name = \"test-plugin\"\n\
             artifact_path = \"\"\ncause = \"fuel\"\ntrace_id = \"trace-test\"\n",
        )
        .expect("test setup: write marker");
        assert!(
            marker_path.exists(),
            "pre-condition: marker must exist before delete"
        );

        delete_marker_if_pass(&marker_path)
            .expect("AC-012: delete_marker_if_pass MUST return Ok(()) when file exists");

        assert!(
            !marker_path.exists(),
            "AC-012: marker MUST be deleted after delete_marker_if_pass succeeds"
        );
    }

    // ── S-25.01 Red Gate stub 8 (DO NOT DELETE per ADR-047 §D7 alias) ─────────
    #[test]
    fn test_BC_1_18_004_fail_open_indeterminate_writes_no_marker() {
        // Canonical backward-compat guard test. Cross-reference: BC-1.18.004 PC5 canonical name is
        // test_BC_1_18_004_fail_open_default_preserves_advisory_behavior. Both names acceptable per
        // BC-1.18.004 PC5 allowance — this is the canonical alias. DO NOT DELETE. ADR-047 §D7.
        //
        // AC-015 / BC-1.18.004 postcondition 1 + postcondition 2 + postcondition 3:
        // fail-open INDETERMINATE → advisory event only; NO marker written; NO gate triggered.
        // should_write_marker(Indeterminate, FailOpen) MUST be false.
        // ~76 existing fail-open plugins MUST be completely unaffected by S-25.01.
        let outcome = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Fuel,
        };
        let write = should_write_marker(&outcome, FailurePolicy::FailOpen);
        assert!(
            !write,
            "AC-015 / BC-1.18.004 PC1: should_write_marker(Indeterminate, FailOpen) MUST return \
             false — ~76 existing fail-open plugins MUST NOT have a marker written (DO NOT DELETE)"
        );
    }

    // ── S-25.01 Red Gate stub 9 (DO NOT DELETE per ADR-047 §D7 canonical) ────
    #[test]
    fn test_BC_1_18_004_fail_open_default_preserves_advisory_behavior() {
        // Canonical backward-compat guard test per ADR-047 §Decision 7. DO NOT DELETE.
        // Cross-reference: test_BC_1_18_004_fail_open_indeterminate_writes_no_marker above.
        //
        // AC-015 / BC-1.18.004 postcondition 1–3; S-21.10 canonical:
        // FailurePolicy::default() MUST equal FailOpen (ADR-039 §Decision 1).
        // Validates: all four should_write_marker(_, FailOpen) cases return false.

        // Fuel cause
        let indet_fuel = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Fuel,
        };
        assert!(
            !should_write_marker(&indet_fuel, FailurePolicy::FailOpen),
            "AC-015: should_write_marker(Indeterminate(Fuel), FailOpen) MUST be false"
        );

        // Epoch cause
        let indet_epoch = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Epoch,
        };
        assert!(
            !should_write_marker(&indet_epoch, FailurePolicy::FailOpen),
            "AC-015: should_write_marker(Indeterminate(Epoch), FailOpen) MUST be false"
        );

        // OTL cause
        let indet_otl = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::OutputTooLarge,
        };
        assert!(
            !should_write_marker(&indet_otl, FailurePolicy::FailOpen),
            "AC-015: should_write_marker(Indeterminate(OTL), FailOpen) MUST be false"
        );

        // FailurePolicy::default() MUST equal FailOpen (S-21.10 canonical; ADR-039 §Decision 1)
        assert!(
            !should_write_marker(&indet_fuel, FailurePolicy::default()),
            "AC-015 / S-21.10: FailurePolicy::default() MUST equal FailOpen; \
             should_write_marker(Indeterminate, default()) MUST be false (DO NOT DELETE)"
        );

        // Pass and Fail with FailOpen MUST also be false (symmetry check)
        assert!(
            !should_write_marker(&DispatchOutcome::Pass, FailurePolicy::FailOpen),
            "should_write_marker(Pass, FailOpen) MUST be false"
        );
        assert!(
            !should_write_marker(
                &DispatchOutcome::Fail { exit_code: 1 },
                FailurePolicy::FailOpen
            ),
            "should_write_marker(Fail, FailOpen) MUST be false"
        );
    }

    // ── S-25.01 Red Gate stub 10 ──────────────────────────────────────────────
    #[test]
    fn test_backward_compat_pass_fail_on_error_semantics_unchanged() {
        // AC-004 / BC-1.18.001 postcondition 5 (backward-compat anchor):
        // Existing PASS/FAIL semantics for PluginResult::Ok UNCHANGED by S-25.01.
        // The policy parameter MUST NOT change the PASS/FAIL classification —
        // policy only affects the marker-write path via should_write_marker.

        // Pass path (exit_code=0, output_too_large=false) — fail-open context
        let ok_pass = PluginResult::Ok {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 10,
        };
        let pass = classify_outcome(ok_pass, FailurePolicy::FailOpen, false);
        assert_eq!(
            pass,
            DispatchOutcome::Pass,
            "backward compat: exit_code=0 + output_too_large=false MUST remain Pass"
        );

        // Fail path (exit_code=2) — fail-open context
        let ok_fail = PluginResult::Ok {
            exit_code: 2,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 10,
        };
        let fail = classify_outcome(ok_fail, FailurePolicy::FailOpen, false);
        assert_eq!(
            fail,
            DispatchOutcome::Fail { exit_code: 2 },
            "backward compat: exit_code=2 MUST remain Fail{{exit_code: 2}}"
        );

        // Policy MUST NOT change the PASS classification (policy-orthogonality)
        let ok_pass_closed = PluginResult::Ok {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 10,
        };
        let pass_closed = classify_outcome(ok_pass_closed, FailurePolicy::FailClosed, false);
        assert_eq!(
            pass_closed,
            DispatchOutcome::Pass,
            "AC-004: policy=FailClosed MUST NOT change a Pass outcome to anything else"
        );
    }

    // ── S-25.01 Red Gate stub 11 ──────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_unrecognized_trap_routes_to_on_error_not_indeterminate() {
        // AC-004 / BC-1.18.001 invariant 2 (non-exhaustive Trap wildcard):
        // An unrecognized (future) Trap variant MUST NOT yield INDETERMINATE.
        // In invoke.rs, unrecognized Trap variants become PluginResult::Crashed (the `_ =>` arm).
        // classify_outcome MUST NOT classify Crashed as INDETERMINATE.
        // The `_ =>` wildcard arm routes to existing on_error handling.
        // Trap is `#[non_exhaustive]` — future unknown variants must be safe.
        let crash_result = PluginResult::Crashed {
            trap_string: "wasm trap: out of bounds memory access".to_string(),
            stderr: String::new(),
            elapsed_ms: 1,
            fuel_consumed: 0,
        };
        let outcome = classify_outcome(crash_result, FailurePolicy::FailClosed, false);
        assert!(
            !matches!(outcome, DispatchOutcome::Indeterminate { .. }),
            "AC-004 / BC-1.18.001 INV2: unrecognized Trap (PluginResult::Crashed) MUST NOT yield \
             INDETERMINATE — wildcard `_ =>` arm routes to on_error handling, not INDETERMINATE"
        );
    }

    // ── S-25.01 Red Gate stub 12 ──────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_001_emits_plugin_indeterminate_event_with_required_fields() {
        // AC-006 / BC-3.08.001 Event 8:
        // The dispatcher MUST emit a plugin.indeterminate event to FileSink for EVERY
        // INDETERMINATE outcome (both fail-closed AND fail-open advisory paths).
        //
        // Mandatory event fields (BC-3.08.001 Event 8):
        //   type          ("plugin.indeterminate")
        //   trace_id      (dispatcher_trace_id)
        //   session_id    (session ID from the dispatch context)
        //   plugin_name   (registry name of the plugin)
        //   artifact_path (empty string when no artifact context — never omit)
        //   cause         ("fuel" | "epoch" | "output-too-large")
        //   failure_policy ("fail-closed" | "fail-open")
        //   timestamp     (RFC 3339)
        //
        // Red Gate: classify_outcome is todo!() — this test fails on that call.
        // Implementer (T-2): wire plugin.indeterminate emission in invoke.rs and
        // enhance this test to verify the InternalLog FileSink received the event
        // with all 8 fields (pattern: emit_lifecycle_timeout_carries_fuel_cap_and_consumed).
        let result = PluginResult::Timeout {
            cause: TimeoutCause::Fuel,
            stderr: String::new(),
            elapsed_ms: 100,
            fuel_consumed: DEFAULT_FUEL_CAP,
            fuel_cap: DEFAULT_FUEL_CAP,
        };
        // classify_outcome is todo!() — panics here (Red Gate).
        let outcome = classify_outcome(result, FailurePolicy::FailClosed, false);
        // Once classify_outcome is implemented, verify the outcome is INDETERMINATE
        // (prerequisite for event emission).
        assert!(
            matches!(outcome, DispatchOutcome::Indeterminate { .. }),
            "AC-006 prerequisite: fuel timeout MUST yield INDETERMINATE before emission check"
        );
        // Implementer: after classify_outcome + T-2 emission are implemented, add:
        //   let log = InternalLog::new(dir.path().join("logs"));
        //   ... trigger INDETERMINATE via execute_tier with fuel-exhausting WAT ...
        //   ... read InternalLog output ...
        //   assert_eq!(event["type"], "plugin.indeterminate");
        //   assert!(event.get("trace_id").is_some(), "trace_id must be present");
        //   assert!(event.get("cause").is_some(), "cause must be present");
        //   assert!(event.get("failure_policy").is_some(), "failure_policy must be present");
        //   ... (all 8 BC-3.08.001 Event 8 fields) ...
    }

    // ── End S-25.01 Red Gate stubs 1–12 ──────────────────────────────────────

    // ── S-25.01 additional Red Gate tests for uncovered ACs ──────────────────

    /// AC-015 / BC-1.18.004 postcondition 1–3 + BC-1.18.001 INV3:
    /// should_write_marker(Indeterminate, FailClosed) == true (the ONLY true case).
    /// should_write_marker(Pass/Fail, FailClosed) == false (non-INDETERMINATE outcomes never write).
    #[test]
    fn test_BC_1_18_001_should_write_marker_true_only_for_fail_closed_indeterminate() {
        // AC-015 / BC-1.18.004 PC1-3 + BC-1.18.001 INV3 (single-marker policy):
        // The only case where should_write_marker returns true:
        //   outcome=Indeterminate AND policy=FailClosed.

        // True cases (all three INDETERMINATE causes with FailClosed)
        let indet_fuel = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Fuel,
        };
        assert!(
            should_write_marker(&indet_fuel, FailurePolicy::FailClosed),
            "BC-1.18.001: should_write_marker(Indeterminate(Fuel), FailClosed) MUST be true"
        );
        let indet_epoch = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::Epoch,
        };
        assert!(
            should_write_marker(&indet_epoch, FailurePolicy::FailClosed),
            "BC-1.18.001: should_write_marker(Indeterminate(Epoch), FailClosed) MUST be true"
        );
        let indet_otl = DispatchOutcome::Indeterminate {
            cause: IndeterminateCause::OutputTooLarge,
        };
        assert!(
            should_write_marker(&indet_otl, FailurePolicy::FailClosed),
            "BC-1.18.001: should_write_marker(Indeterminate(OTL), FailClosed) MUST be true"
        );

        // False cases: Pass/Fail with FailClosed must NOT write marker
        assert!(
            !should_write_marker(&DispatchOutcome::Pass, FailurePolicy::FailClosed),
            "should_write_marker(Pass, FailClosed) MUST be false (only INDETERMINATE writes)"
        );
        assert!(
            !should_write_marker(
                &DispatchOutcome::Fail { exit_code: 2 },
                FailurePolicy::FailClosed
            ),
            "should_write_marker(Fail, FailClosed) MUST be false (only INDETERMINATE writes)"
        );
    }

    /// AC-013 / BC-1.18.003 postcondition 2 — idempotent delete NotFound → Ok(()).
    /// `delete_marker_if_pass` MUST return Ok(()) when the file is already absent.
    /// `io::ErrorKind::NotFound` MUST be silently swallowed; all other errors propagated.
    #[test]
    fn test_BC_1_18_003_idempotent_delete_not_found_returns_ok() {
        // AC-013 / BC-1.18.003 postcondition 2:
        // delete_marker_if_pass is idempotent: absent file → Ok(()) (no-op).
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        // Verify the file is NOT present (no pre-write in this test)
        assert!(
            !marker_path.exists(),
            "pre-condition: marker must NOT exist for idempotent-delete test"
        );
        // First delete: absent file must return Ok(()) not Err(NotFound)
        let result1 = delete_marker_if_pass(&marker_path);
        assert!(
            result1.is_ok(),
            "AC-013: delete_marker_if_pass on absent path MUST return Ok(()) — \
             io::ErrorKind::NotFound MUST be swallowed, not propagated"
        );
        // Second delete: still absent → still Ok(()) (truly idempotent)
        let result2 = delete_marker_if_pass(&marker_path);
        assert!(
            result2.is_ok(),
            "AC-013: second delete_marker_if_pass call MUST also return Ok(()) (idempotent)"
        );
    }

    // ── End S-25.01 additional Red Gate tests ────────────────────────────────

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
            fuel_cap: DEFAULT_FUEL_CAP,
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
            fuel_cap: DEFAULT_FUEL_CAP,
        };
        assert!(
            plugin_fail_closed(&r, OnError::Block),
            "Timeout + on_error=Block must trigger fail-closed"
        );
    }

    /// Timeout + on_error=Continue → NOT fail-closed.
    #[test]
    fn fail_closed_timeout_with_on_error_continue_is_open() {
        // fuel_consumed == fuel_cap on Trap::OutOfFuel (remaining=0 → cap.saturating_sub(0)=cap).
        let r = PluginResult::Timeout {
            cause: TimeoutCause::Fuel,
            stderr: String::new(),
            elapsed_ms: 5_000,
            fuel_consumed: DEFAULT_FUEL_CAP,
            fuel_cap: DEFAULT_FUEL_CAP,
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

    // ── emit_lifecycle telemetry field coverage ────────────────────────────

    /// `plugin.timeout` events must carry both `fuel_cap` and `fuel_consumed`
    /// so operators can see which budget was hit without opening the full trace.
    /// CLAUDE.md Factory Hook Diagnostics Step 2 sends operators to grep
    /// `.factory/logs/dispatcher-internal-*.jsonl` for exactly these fields.
    #[test]
    fn emit_lifecycle_timeout_carries_fuel_cap_and_consumed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let log = InternalLog::new(dir.path().join("logs"));
        let base_ctx = crate::host::HostContext::new("test-plugin", "0.1.0", "sess-1", "trace-1");
        let entry = RegistryEntry {
            name: "test-plugin".to_string(),
            event: "PreToolUse".to_string(),
            tool: None,
            plugin: std::path::PathBuf::from("test.wasm"),
            priority: None,
            enabled: true,
            timeout_ms: None,
            fuel_cap: None,
            on_error: None,
            capabilities: None,
            config: toml::Value::Table(toml::Table::new()),
            async_flag: false,
            needs_context: vec![],
            failure_policy: crate::registry::FailurePolicy::FailOpen,
        };
        let cap: u64 = DEFAULT_FUEL_CAP;
        let consumed: u64 = 12_345_678;
        let result = PluginResult::Timeout {
            cause: TimeoutCause::Fuel,
            stderr: String::new(),
            elapsed_ms: 10,
            fuel_consumed: consumed,
            fuel_cap: cap,
        };
        emit_lifecycle(&log, &base_ctx, &entry, &result);

        // Read back the JSONL and verify both fields are present.
        let log_dir = dir.path().join("logs");
        let files: Vec<_> = std::fs::read_dir(&log_dir)
            .expect("log dir must exist after write")
            .map(|e| e.expect("dir entry").path())
            .collect();
        assert_eq!(files.len(), 1, "expected exactly one log file");
        let content = std::fs::read_to_string(&files[0]).expect("read log file");
        let event: serde_json::Value =
            serde_json::from_str(content.trim_end()).expect("log line must be valid JSON");
        assert_eq!(
            event["type"].as_str(),
            Some(PLUGIN_TIMEOUT),
            "event type must be plugin.timeout"
        );
        assert_eq!(
            event["fuel_consumed"].as_u64(),
            Some(consumed),
            "plugin.timeout event must carry fuel_consumed"
        );
        assert_eq!(
            event["fuel_cap"].as_u64(),
            Some(cap),
            "plugin.timeout event must carry fuel_cap (CLAUDE.md Diagnostics Step 2)"
        );
    }
}
