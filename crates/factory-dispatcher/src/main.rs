//! vsdd-factory-dispatcher — CLI entry point.
//!
//! Reads a Claude Code hook envelope from stdin, loads the registry at
//! `${CLAUDE_PLUGIN_ROOT}/hooks-registry.toml`, selects plugins that
//! match the event/tool, groups them by priority, and hands them to the
//! execution layer (S-1.6's `execute_tiers`).
//!
//! The runtime is a `current_thread` tokio runtime — we don't need a
//! multi-threaded pool because wasmtime invocations are wrapped in
//! `spawn_blocking` inside the executor, and the dispatcher's own fan-
//! in work is trivial. Single-thread keeps startup cost low and avoids
//! surprising thread pools in a short-lived process.
//!
//! Self-telemetry: the always-on internal log (S-1.7) is constructed
//! first so any dispatcher error — including registry load failures —
//! is durably recorded. Plugin lifecycle events (`plugin.invoked`,
//! `plugin.completed`, `plugin.timeout`, `plugin.crashed`) are emitted
//! by the executor; `dispatcher.*` structured events (schema_mismatch,
//! registry_invalid) and `internal.dispatcher_error` are emitted here.
//!
//! S-15.01 T-3e: four BC-3.08.001 structured events are now emitted from
//! the dispatch path: `dispatcher.schema_mismatch`, `dispatcher.registry_invalid`,
//! `plugin.async_block_discarded`, `plugin.timeout` (async path).
//!
//! ## VSDD_SINK_FILE (test/development hook)
//!
//! When `VSDD_SINK_FILE` is set to a path, all plugin-emitted events
//! (from `host::emit_event`) are appended as JSONL to that file after
//! execution completes. This is used by bats integration tests to
//! capture and assert on emitted events without a full observability
//! sink pipeline. Best-effort: write failures are silently dropped.

use std::path::PathBuf;
use std::sync::Arc;
// Mutex is only needed in debug builds for the VSDD_SINK_FILE flush path (SEC-003).
#[cfg(debug_assertions)]
use std::sync::Mutex;

use factory_dispatcher::engine::{EpochTicker, build_engine};
use factory_dispatcher::executor::{
    ExecutorInputs, PluginOutcome, execute_tiers, spawn_async_plugin,
};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::host::emit_event::{
    emit_dispatcher_schema_mismatch, emit_plugin_async_block_discarded, emit_plugin_timeout_async,
    emit_registry_invalid_e_reg002, emit_registry_invalid_e_reg003,
};
use factory_dispatcher::internal_log::{
    DEFAULT_RETENTION_DAYS, DISPATCHER_STARTED, INTERNAL_DISPATCHER_ERROR, InternalEvent,
    InternalLog,
};
use factory_dispatcher::invoke::PluginResult;
use factory_dispatcher::partition::partition_plugins;
use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::{Registry, RegistryError};
use factory_dispatcher::resolver::ResolverRegistry;
use factory_dispatcher::routing::{group_by_priority, match_plugins};
use factory_dispatcher::{ASYNC_DRAIN_WINDOW_MS, HOST_ABI_VERSION, new_trace_id};
use factory_dispatcher::{AggregatorPluginResult, aggregate_exit_code};
use tokio::sync::mpsc;

const ENV_PLUGIN_ROOT: &str = "CLAUDE_PLUGIN_ROOT";
const ENV_PROJECT_DIR: &str = "CLAUDE_PROJECT_DIR";
// SECURITY: VSDD_SINK_FILE is debug-only; see SEC-003 (W-15 wave gate fix).
// The constant and all logic reading it are gated behind #[cfg(debug_assertions)]
// so the env var name does not appear in release binaries.
#[cfg(debug_assertions)]
const ENV_SINK_FILE: &str = "VSDD_SINK_FILE";

// VSDD_ASYNC_DRAIN_WINDOW_MS: debug-only override for the async drain window.
// Used by bats integration tests (VP-079 S1/S4) to account for WASM cold-start
// time in debug builds. Release builds always use ASYNC_DRAIN_WINDOW_MS (DI-019).
// SEC-003: compiled out in release builds so the env var name does not appear in
// production binaries.
#[cfg(debug_assertions)]
const ENV_ASYNC_DRAIN_WINDOW_MS: &str = "VSDD_ASYNC_DRAIN_WINDOW_MS";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let internal_log = Arc::new(InternalLog::new(resolve_log_dir()));
    internal_log.prune_old(DEFAULT_RETENTION_DAYS);

    let code = match run(internal_log.clone()).await {
        Ok(code) => code,
        Err(err) => {
            emit_dispatcher_error(&internal_log, None, None, &err.to_string());
            0
        }
    };
    std::process::exit(code);
}

async fn run(internal_log: Arc<InternalLog>) -> anyhow::Result<i32> {
    let trace_id = new_trace_id();
    let payload = HookPayload::from_reader(std::io::stdin().lock())?;

    let registry_path = resolve_registry_path()?;
    let registry = match Registry::load(&registry_path) {
        Ok(r) => r,
        Err(e) => {
            // S-15.01 T-3e/T-3f: fail-closed registry errors must exit 2 and emit structured
            // events per BC-1.14.001 EC-006 (schema mismatch) and EC-008 (async+block conflict).
            // These are the explicit exceptions to BC-1.08.001 fail-open policy (ADR-019 §Decision 2).
            //
            // Build a minimal HostContext so we can use the structured emit fns.
            // Events accumulate in the context's event queue and are flushed to
            // VSDD_SINK_FILE below (debug builds only, SEC-003).
            let err_ctx = {
                let mut ctx = HostContext::new(
                    "dispatcher",
                    env!("CARGO_PKG_VERSION"),
                    payload.session_id.clone(),
                    trace_id.clone(),
                );
                ctx.internal_log = Some(internal_log.clone());
                ctx
            };
            // Always emit generic error to internal log for durability.
            emit_dispatcher_error(
                &internal_log,
                Some(trace_id.clone()),
                Some(payload.session_id.clone()),
                &format!("registry load: {e}"),
            );
            // Emit structured event + fail-closed (exit 2) for schema/invariant errors.
            // Other registry errors (file not found, parse error) remain fail-open (exit 0)
            // per BC-1.08.001 (only schema-version mismatch and invariant violation are
            // the named fail-closed exceptions per ADR-019 §Decision 2 and BC-1.14.001 EC-006/EC-008).
            let exit_code = match &e {
                RegistryError::SchemaVersion { got, expected } => {
                    // BC-1.14.001 EC-006 + BC-3.08.001 Event 2.
                    // Emit dispatcher.schema_mismatch with found_version/expected_version/error_code.
                    emit_dispatcher_schema_mismatch(&err_ctx, *got, *expected);
                    eprintln!(
                        "factory-dispatcher: E-REG-001 schema_version={got} expected={expected}; exiting 2 (fail-closed per ADR-019 §Decision 2)"
                    );
                    2
                }
                RegistryError::AsyncBlockConflict { name } => {
                    // BC-1.14.001 EC-008 + BC-3.08.001 Event 3.
                    // Emit dispatcher.registry_invalid with offending_plugin/violation/error_code.
                    // E-REG-002 is intra-entry; offending_event/tool absence is enforced by type system.
                    emit_registry_invalid_e_reg002(&err_ctx, name, "async_block_conflict");
                    eprintln!(
                        "factory-dispatcher: E-REG-002 on_error=block AND async=true for '{name}'; exiting 2 (fail-closed per ADR-019 §Decision 2)"
                    );
                    2
                }
                RegistryError::DuplicateEntry { name, event, tool } => {
                    // BC-7.06.001 Invariant 7 + BC-3.08.001 Event 3 (E-REG-003).
                    // Emit dispatcher.registry_invalid with full wire payload per BC-7.06.001 v1.8:
                    // offending_plugin, violation, error_code, offending_event, offending_tool.
                    // F-P8-001 / F-P14-001 Path B: fail-closed; dispatcher refuses to start on
                    // duplicate (name, event, tool) tuple.
                    eprintln!(
                        "[E-REG-003] Duplicate hook registration: name={name}, event={event}, tool={tool:?} \
                         (BC-7.06.001 v1.8 Invariant 7). Each (name, event, tool) tuple must be unique \
                         across all [[hooks]] entries; dispatcher refuses to start."
                    );
                    emit_registry_invalid_e_reg003(
                        &err_ctx,
                        name.as_str(),
                        "duplicate_hook_registration",
                        event.as_str(),  // offending_event — required for E-REG-003
                        tool.as_deref(), // offending_tool — None means wildcard/"all tools"
                    );
                    2
                }
                // Other errors: file not found, parse failures, regex errors.
                // These are operational errors (misconfiguration / missing file), not
                // semantic invariant violations. Fail-open per BC-1.08.001.
                _ => 0,
            };
            // Flush structured events to VSDD_SINK_FILE (debug builds / bats harness only).
            // VP-079 S2/S3 verify these events appear in the sink.
            // SEC-003: VSDD_SINK_FILE is debug-only; only reject path traversal sequences.
            // Absolute paths are allowed — bats tests use mktemp which produces absolute paths.
            #[cfg(debug_assertions)]
            if let Ok(sink_path) = std::env::var(ENV_SINK_FILE)
                && !sink_path.is_empty()
                && !sink_path.contains("..")
            {
                flush_sink_file(&sink_path, &err_ctx.events);
            }
            return Ok(exit_code);
        }
    };

    internal_log.write(
        &InternalEvent::now(DISPATCHER_STARTED)
            .with_trace_id(trace_id.clone())
            .with_session_id(payload.session_id.clone())
            .with_field("dispatcher_version", env!("CARGO_PKG_VERSION"))
            .with_field("host_abi_version", HOST_ABI_VERSION as i64)
            .with_field(
                "platform",
                format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            )
            .with_field("pid", std::process::id() as i64)
            .with_field("registry_path", registry_path.display().to_string())
            .with_field("loaded_plugin_count", registry.hooks.len() as i64),
    );

    let matched = match_plugins(&registry, &payload);

    // S-15.01 T-3c: partition matched plugins into sync_group (gates user)
    // and async_group (fire-and-forget, verdict never reaches Claude Code).
    // BC-1.14.001 postconditions 1, 5, 6 — partition then await sync, spawn async.
    let matched_owned: Vec<_> = matched.into_iter().cloned().collect();
    let partition = partition_plugins(&matched_owned);
    let sync_tiers = group_by_priority(&registry, partition.sync_group.iter().collect());

    eprintln!(
        "factory-dispatcher trace={} event={} tool={} host_abi={} sync_plugins={} async_plugins={}",
        trace_id,
        payload.event_name,
        payload.tool_name,
        HOST_ABI_VERSION,
        partition.sync_group.len(),
        partition.async_group.len(),
    );

    if sync_tiers.is_empty() && partition.async_group.is_empty() {
        return Ok(0);
    }

    // Execution layer. Build a shared engine + epoch ticker + module
    // cache per invocation. Keeping the engine short-lived adds a bit
    // of cold-start cost but sidesteps any global state concerns for
    // the short-lived dispatcher process. S-1.5's `PluginCache` still
    // amortizes per-plugin compile cost within a single invocation.
    let engine = match build_engine() {
        Ok(e) => e,
        Err(e) => {
            emit_dispatcher_error(
                &internal_log,
                Some(trace_id.clone()),
                Some(payload.session_id.clone()),
                &format!("engine build: {e}"),
            );
            return Ok(0);
        }
    };
    let _ticker = EpochTicker::start(engine.clone());
    let cache = Arc::new(PluginCache::new(engine.clone()));

    let mut base_host_ctx = HostContext::new(
        "", // executor overrides per plugin
        env!("CARGO_PKG_VERSION"),
        payload.session_id.clone(),
        trace_id.clone(),
    );
    base_host_ctx.internal_log = Some(internal_log.clone());
    // Plugin subprocess cwd should be the project root, not the
    // dispatcher's process cwd. v0.79.x bash hooks were spawned by
    // Claude Code from the project dir, and many of them (including
    // every hook that calls bin/emit-event) walk `.factory/logs/`
    // relative to cwd. Falling back to the dispatcher's cwd produces
    // log writes in surprising places.
    base_host_ctx.cwd = std::env::var(ENV_PROJECT_DIR)
        .map(PathBuf::from)
        .ok()
        .filter(|p| !p.as_os_str().is_empty())
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    base_host_ctx.plugin_root = std::env::var(ENV_PLUGIN_ROOT)
        .map(PathBuf::from)
        .unwrap_or_default();
    // Project the dispatcher's whole process env into the host context's
    // env_view. The host's exec_subprocess + env host functions look up
    // names against ctx.env_view (not std::env::var) so per-plugin env
    // allow-lists can be enforced without a syscall per call. The
    // capability gate is the registry's `env_allow` field, applied at
    // call time inside the host fn — env_view is the source pool.
    base_host_ctx.env_view = std::env::vars().collect();

    // The dispatcher's `HookPayload` is the Claude-Code-facing shape;
    // the plugin-facing shape (mirrored in `vsdd_hook_sdk::HookPayload`)
    // includes a `dispatcher_trace_id` field the dispatcher just
    // assigned. Inject it into the serialized JSON before handing off
    // — plugins built against the SDK reject payloads missing this
    // field with a hard error. The executor splices in `plugin_config`
    // per plugin from the registry entry; we do not inject it here.
    let mut payload_value = serde_json::to_value(&payload)?;
    if let Some(map) = payload_value.as_object_mut() {
        map.insert(
            "dispatcher_trace_id".to_string(),
            serde_json::Value::String(trace_id.clone()),
        );
    }

    // Clone the event queue Arc before moving base_host_ctx into
    // ExecutorInputs. All plugin contexts share this Arc (every clone
    // of HostContext shares the same Mutex<Vec<_>>), so draining it
    // after execute_tiers completes yields all plugin-emitted events.
    // In release builds the VSDD_SINK_FILE path is compiled out (SEC-003);
    // allow(unused_variables) silences the resulting lint.
    #[allow(unused_variables)]
    let event_queue = Arc::clone(&base_host_ctx.events);

    // S-12.03: Build the resolver registry. Currently zero resolvers are
    // registered (WASM-backed resolvers are wired in S-12.04). The empty
    // registry is valid — hooks with empty needs_context skip it with zero
    // overhead (BC-1.13.001 PC3 / AC-002).
    let resolver_registry = Arc::new(ResolverRegistry::new());

    // S-15.01 T-3c: build executor inputs and run sync_group first.
    // Sync group awaits all completions; verdict gates Claude Code.
    // BC-1.14.001 postconditions 2-3.
    let inputs = ExecutorInputs {
        engine: &engine,
        cache: &cache,
        registry: &registry,
        payload_value: payload_value.clone(),
        base_host_ctx: base_host_ctx.clone(),
        internal_log: internal_log.clone(),
        resolver_registry: resolver_registry.clone(),
    };

    let summary = execute_tiers(inputs, sync_tiers).await;

    // S-15.01 F5-T-A: async_group dispatch via tokio::spawn per-plugin + tokio::select! drain.
    //
    // BC-1.14.001 v1.9 PC4 + Invariant 3 (F-P1-006 + F-P1-010):
    //   - Each async plugin is spawned as an INDEPENDENT tokio task (NOT via execute_tiers).
    //   - group_by_priority MUST NOT be called on async-group plugins (Invariant 3).
    //   - Results are collected via an unbounded channel + tokio::select! drain timer.
    //   - Completed plugins' terminal events MUST emit (EC-012).
    //   - In-flight plugins when drain timer fires are abandoned (EC-011).
    //
    // In debug builds, VSDD_ASYNC_DRAIN_WINDOW_MS env var can override the window
    // to account for WASM cold-start time in bats integration tests (VP-079 S1/S4).
    // Release builds always use ASYNC_DRAIN_WINDOW_MS (DI-019). SEC-003.
    if !partition.async_group.is_empty() {
        #[cfg(debug_assertions)]
        let effective_drain_window = std::env::var(ENV_ASYNC_DRAIN_WINDOW_MS)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(std::time::Duration::from_millis)
            .unwrap_or(ASYNC_DRAIN_WINDOW_MS);
        #[cfg(not(debug_assertions))]
        let effective_drain_window = ASYNC_DRAIN_WINDOW_MS;

        // Spawn each async plugin as an independent task with a results channel.
        // BC-1.14.001 v1.9 PC4: tokio::spawn per-plugin, NOT execute_tiers.
        // Invariant 3: MUST NOT call group_by_priority on async-group plugins.
        let (tx, mut rx) = mpsc::unbounded_channel::<PluginOutcome>();
        // For each async plugin: spawn an independent task and wire its result to the channel.
        // Each spawn_async_plugin call returns a JoinHandle; we wrap it in a forwarding task
        // so the channel gets the PluginOutcome once the plugin completes.
        let _async_handles: Vec<_> = partition
            .async_group
            .into_iter()
            .map(|entry| {
                let tx_for_task = tx.clone();
                let handle = spawn_async_plugin(
                    engine.clone(),
                    cache.clone(),
                    registry.defaults.clone(),
                    entry,
                    payload_value.clone(),
                    base_host_ctx.clone(),
                    internal_log.clone(),
                    resolver_registry.clone(),
                );
                // Forward the JoinHandle result to the drain channel.
                // EC-012: completed results MUST reach the channel so they can be emitted.
                tokio::spawn(async move {
                    match handle.await {
                        Ok(outcome) => {
                            let _ = tx_for_task.send(outcome);
                        }
                        Err(_join_err) => {} // spawn_blocking panic — lifecycle event already emitted
                    }
                })
            })
            .collect();
        // Drop the original sender so rx terminates when all forwarding tasks finish.
        drop(tx);

        // Drain timer: bound the wait at effective_drain_window (DI-019).
        let drain_timer = tokio::time::sleep(effective_drain_window);
        tokio::pin!(drain_timer);

        // Collect partial results until timer fires OR all tasks complete.
        // EC-012: events for plugins that completed before the timer MUST emit;
        // in-flight plugins (still running when timer fires) are abandoned.
        let mut partial_outcomes: Vec<PluginOutcome> = Vec::new();
        loop {
            tokio::select! {
                // Bias toward draining the channel before checking the timer.
                biased;
                maybe_outcome = rx.recv() => {
                    match maybe_outcome {
                        Some(outcome) => partial_outcomes.push(outcome),
                        None => break, // all senders dropped: all tasks finished
                    }
                }
                _ = &mut drain_timer => {
                    // EC-011: in-flight tasks are abandoned. Drain timer fired.
                    break;
                }
            }
        }

        // T-3e: emit diagnostic events for async plugin results that completed
        // within the drain window. These are observability-only; they NEVER change
        // the dispatcher exit code (BC-1.14.001 PC5 + Invariant 3).
        //
        // Block detection: plugins express block intent via stdout JSON
        // `{"outcome":"block","reason":"..."}` (HOST_ABI.md) or WASI exit code 2.
        // Invariant 4 (BC-1.14.001) guarantees no async plugin has on_error=block,
        // so block_intent is structurally false — event emitted for diagnostic visibility only.
        for outcome in &partial_outcomes {
            match &outcome.result {
                PluginResult::Ok {
                    exit_code, stdout, ..
                } => {
                    let has_block_json = stdout.contains(r#""outcome":"block""#);
                    let has_exit_2 = *exit_code == 2;
                    if has_block_json || has_exit_2 {
                        // BC-3.08.001 Event 1: async plugin returned block verdict (discarded).
                        emit_plugin_async_block_discarded(&base_host_ctx, &outcome.plugin_name, 2);
                    }
                }
                PluginResult::Timeout { .. } => {
                    // BC-3.08.001 Event 4: async plugin timed out.
                    let timeout_ms = registry
                        .hooks
                        .iter()
                        .find(|e| e.name == outcome.plugin_name)
                        .and_then(|e| e.timeout_ms)
                        .unwrap_or(registry.defaults.timeout_ms);
                    emit_plugin_timeout_async(&base_host_ctx, &outcome.plugin_name, timeout_ms);
                }
                _ => {} // Crash or non-block result — no structured event emitted
            }
        }
    }

    // Relay any non-empty plugin stderr to the dispatcher's process stderr so
    // user-visible hook messages (e.g. WAVE GATE REMINDER from
    // warn-pending-wave-gate) reach the terminal. The WASI sandbox captures
    // plugin stderr into MemoryOutputPipe; without this relay the output
    // would only appear in the internal log, invisible to the user.
    //
    // NOTE: stderr-relay is deliberately scoped to sync_group only.
    // Per BC-1.14.001 v1.9 Invariant 4, async-group plugins are telemetry — their stderr is
    // captured in InternalLog/HostContext events but is NOT relayed to the dispatcher's
    // stderr. This is intentional: async plugins should never produce user-facing output.
    // `partial_outcomes` (collected from async tasks via the channel) is iterated for
    // diagnostic event emission only, not stderr propagation.
    for outcome in &summary.per_plugin_results {
        if let PluginResult::Ok { stderr, .. } = &outcome.result
            && !stderr.is_empty()
        {
            eprint!("{stderr}");
        }
    }

    // Compute the dispatcher exit code from sync_group results only.
    // VP-077 H5/H6: aggregate_exit_code is the pure, Kani-provable computation.
    // It checks exit_code==2 && on_error==Block (WASI-exit-code path).
    // The existing summary.exit_code also handles advisory blocks (stdout JSON path).
    // Final exit code is the OR of both: either signal triggers exit 2.
    // Async group results are structurally excluded (not passed to aggregate_exit_code).
    let sync_agg_results: Vec<AggregatorPluginResult> = summary
        .per_plugin_results
        .iter()
        .map(|o| {
            let exit_code = match &o.result {
                PluginResult::Ok { exit_code, .. } => *exit_code as u8,
                _ => 0u8,
            };
            AggregatorPluginResult {
                exit_code,
                on_error: o.on_error,
            }
        })
        .collect();
    let aggregate_code = aggregate_exit_code(&sync_agg_results) as i32;
    // Combine: advisory-block (stdout JSON, summary.exit_code) OR WASI-block (exit_code==2+Block).
    let final_exit_code = if summary.exit_code == 2 || aggregate_code == 2 {
        2
    } else {
        0
    };

    eprintln!(
        "  plugins_run={} total_ms={} block_intent={} exit_code={}",
        summary.per_plugin_results.len(),
        summary.total_elapsed_ms,
        summary.block_intent,
        final_exit_code,
    );

    // SECURITY: VSDD_SINK_FILE is debug-only; see SEC-003 (W-15 wave gate fix).
    // VSDD_SINK_FILE: drain plugin events and append as JSONL for
    // bats integration tests (S-8.08 AC-005). Best-effort — any I/O
    // error is silently dropped so the dispatcher always exits 0 on
    // non-block dispatches regardless of sink write outcome.
    #[cfg(debug_assertions)]
    if let Ok(sink_path) = std::env::var(ENV_SINK_FILE)
        && !sink_path.is_empty()
    {
        // Reject path traversal sequences (SEC-003). Absolute paths are allowed —
        // bats integration tests use mktemp which produces absolute paths.
        // VSDD_SINK_FILE is debug-only (compiled out in release builds per SEC-003).
        if sink_path.contains("..") {
            eprintln!("VSDD_SINK_FILE: rejected path traversal in: {sink_path}");
        } else {
            flush_sink_file(&sink_path, &event_queue);
        }
    }

    Ok(final_exit_code)
}

fn resolve_registry_path() -> anyhow::Result<PathBuf> {
    let plugin_root = std::env::var(ENV_PLUGIN_ROOT)
        .map_err(|_| anyhow::anyhow!("${ENV_PLUGIN_ROOT} is not set"))?;
    Ok(PathBuf::from(plugin_root).join("hooks-registry.toml"))
}

/// Resolve the internal log directory.
///
/// TODO(S-2.6): v0.79.x has full git-worktree-aware resolution so the
/// log always lands on the main worktree even when the dispatcher is
/// invoked from a subdir. For v1.0-beta.1 we keep it simple: prefer
/// `${CLAUDE_PROJECT_DIR}/.factory/logs`, fall back to `./.factory/logs`
/// relative to the cwd. S-2.6 will replace this with the full
/// resolution used by the existing `emit-event` bash bin.
fn resolve_log_dir() -> PathBuf {
    match std::env::var(ENV_PROJECT_DIR) {
        Ok(root) if !root.is_empty() => PathBuf::from(root).join(".factory").join("logs"),
        _ => PathBuf::from(".factory").join("logs"),
    }
}

/// Write plugin-emitted events as JSONL to the `VSDD_SINK_FILE` path.
///
/// Only called when `VSDD_SINK_FILE` is set (bats test harness). Best-
/// effort: any I/O or serialization error is silently swallowed.
///
/// ## Event filtering (S-15.01 T-3e update)
///
/// All events EXCEPT `internal.*` are written to the sink. This allows:
/// - `dispatcher.schema_mismatch` (BC-3.08.001 Event 2, VP-079 S2)
/// - `dispatcher.registry_invalid` (BC-3.08.001 Event 3, VP-079 S3)
/// - `plugin.async_block_discarded` (BC-3.08.001 Event 1, VP-079 S1)
/// - `plugin.timeout` with execution_group=async (BC-3.08.001 Event 4, VP-079 S4)
/// - All other plugin-domain events (e.g. `agent.start`, VP-028 fan-out)
///
/// `internal.*` events (dispatcher lifecycle diagnostics: `internal.dispatcher_error`,
/// `internal.capability_denied`, etc.) are excluded — these are internal log
/// events and should not appear in the observable events-*.jsonl stream.
///
/// Used by S-8.08 AC-005 + VP-079 bats integration tests.
/// SECURITY: debug-only; see SEC-003 (W-15 wave gate fix).
#[cfg(debug_assertions)]
fn flush_sink_file(sink_path: &str, event_queue: &Arc<Mutex<Vec<InternalEvent>>>) {
    use std::io::Write;

    let events = {
        match event_queue.lock() {
            Ok(mut guard) => std::mem::take(&mut *guard),
            Err(_) => return,
        }
    };

    // Exclude only internal.* lifecycle noise — all observable events pass through.
    // internal.* events are dispatcher-private diagnostics (dispatcher_error,
    // capability_denied, plugin_invoked, plugin_completed, plugin_timeout lifecycle
    // events emitted by the executor's internal log path). Everything else —
    // including dispatcher.* and plugin.* domain events per BC-3.08.001 — is observable.
    let domain_events: Vec<_> = events
        .iter()
        .filter(|ev| !ev.type_.starts_with("internal."))
        .collect();

    if domain_events.is_empty() {
        return;
    }

    // Open (or create) the sink file for appending.
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(sink_path);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => return, // best-effort: silently drop
    };

    for ev in domain_events {
        if let Ok(line) = serde_json::to_string(ev) {
            let _ = file.write_all(line.as_bytes());
            let _ = file.write_all(b"\n");
        }
    }
}

/// Emit `internal.dispatcher_error` via the internal log, mirroring to
/// stderr as a last-resort fallback. The stderr line is the same shape
/// the previous implementation used so downstream log scrapers don't
/// need to change.
fn emit_dispatcher_error(
    log: &InternalLog,
    trace_id: Option<String>,
    session_id: Option<String>,
    msg: &str,
) {
    let mut event = InternalEvent::now(INTERNAL_DISPATCHER_ERROR).with_field("message", msg);
    if let Some(t) = trace_id {
        event = event.with_trace_id(t);
    }
    if let Some(s) = session_id {
        event = event.with_session_id(s);
    }
    log.write(&event);

    eprintln!(
        r#"{{"type":"internal.dispatcher_error","message":{:?}}}"#,
        msg
    );
}
