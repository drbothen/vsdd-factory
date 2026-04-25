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
//! `on_error = "block"` semantics (per Q3 resolution in the design doc):
//! a plugin that returns `HookResult::Block` under an `on_error = block`
//! entry records a dispatcher-level block intent but does **not**
//! abort the tier — remaining plugins still fire. The summary's
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
            if matches!(outcome.on_error, OnError::Block) && plugin_requests_block(&outcome.result)
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
        // Splice this entry's per-plugin config onto the base envelope.
        // Cheap clone since `payload_value` is a small JSON tree, and
        // doing it per-plugin guarantees one entry never sees another's
        // config — even when several entries share the same wasm
        // (e.g. multiple legacy-bash-adapter registrations).
        let mut per_plugin_value = inputs.payload_value.clone();
        if let Some(map) = per_plugin_value.as_object_mut() {
            map.insert("plugin_config".to_string(), entry_clone.config_as_json());
        }
        let payload = match serde_json::to_vec(&per_plugin_value) {
            Ok(v) => v,
            Err(e) => {
                let result = PluginResult::Crashed {
                    trap_string: format!("payload serialize: {e}"),
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

/// Internal helper: either an already-resolved outcome (load failure
/// short-circuit) or a pending tokio JoinHandle. Keeps the per-plugin
/// fan-out loop uniform.
enum JoinWrap {
    Ready(PluginOutcome),
    Pending(tokio::task::JoinHandle<PluginOutcome>),
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
    let (event_type, elapsed, fuel, extra_fields) = match result {
        PluginResult::Ok {
            exit_code,
            elapsed_ms,
            fuel_consumed,
            ..
        } => (
            PLUGIN_COMPLETED,
            *elapsed_ms,
            *fuel_consumed,
            vec![("exit_code".to_string(), serde_json::Value::from(*exit_code))],
        ),
        PluginResult::Timeout {
            cause,
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
                vec![(
                    "cause".to_string(),
                    serde_json::Value::String(cause_str.to_string()),
                )],
            )
        }
        PluginResult::Crashed {
            trap_string,
            elapsed_ms,
            fuel_consumed,
        } => (
            PLUGIN_CRASHED,
            *elapsed_ms,
            *fuel_consumed,
            vec![(
                "trap".to_string(),
                serde_json::Value::String(trap_string.clone()),
            )],
        ),
    };

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

    #[test]
    fn plugin_requests_block_detects_tagged_json() {
        let r = PluginResult::Ok {
            exit_code: 2,
            stdout: r#"{"outcome":"block","reason":"policy 9"}"#.to_string(),
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
            elapsed_ms: 2,
            fuel_consumed: 5,
        };
        assert!(!plugin_requests_block(&r));
    }

    #[test]
    fn plugin_requests_block_false_for_crash() {
        let r = PluginResult::Crashed {
            trap_string: "unreachable".to_string(),
            elapsed_ms: 1,
            fuel_consumed: 0,
        };
        assert!(!plugin_requests_block(&r));
    }

    #[test]
    fn plugin_requests_block_false_for_timeout() {
        let r = PluginResult::Timeout {
            cause: TimeoutCause::Epoch,
            elapsed_ms: 5_000,
            fuel_consumed: 0,
        };
        assert!(!plugin_requests_block(&r));
    }
}
