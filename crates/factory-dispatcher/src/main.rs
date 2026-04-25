//! vsdd-factory-dispatcher — CLI entry point.
//!
//! Reads a Claude Code hook envelope from stdin, loads the registry at
//! `${CLAUDE_PLUGIN_ROOT}/hooks-registry.toml`, selects plugins that
//! match the event/tool, groups them by priority, and hands them to
//! the execution layer.
//!
//! Execution is stubbed here — S-1.5 (wasmtime) and S-1.6 (tokio
//! parallel-tier) fill in `run_tiers`. For now, the dispatcher prints
//! tier metadata and returns `Continue` for every plugin so operators
//! can smoke-test the routing path before plugins actually execute.
//!
//! Self-telemetry: S-1.7 wires the always-on internal log. Every
//! dispatcher lifecycle event and every `internal.*` event lands in
//! `<log_dir>/dispatcher-internal-YYYY-MM-DD.jsonl`. Plugin lifecycle
//! events wait for S-1.5; sink events for S-1.8.

use factory_dispatcher::internal_log::{
    DEFAULT_RETENTION_DAYS, DISPATCHER_STARTED, INTERNAL_DISPATCHER_ERROR, InternalEvent,
    InternalLog,
};
use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::registry::{OnError, Registry};
use factory_dispatcher::routing::{
    PluginOutcome, PluginResultStub, group_by_priority, match_plugins,
};
use factory_dispatcher::{HOST_ABI_VERSION, new_trace_id};
use std::path::PathBuf;

const ENV_PLUGIN_ROOT: &str = "CLAUDE_PLUGIN_ROOT";
const ENV_PROJECT_DIR: &str = "CLAUDE_PROJECT_DIR";

fn main() {
    // Dispatcher must never propagate a raw panic into Claude Code;
    // any unhandled error becomes a structured `internal.dispatcher_error`
    // event and exits 0 (non-blocking).
    let internal_log = InternalLog::new(resolve_log_dir());
    internal_log.prune_old(DEFAULT_RETENTION_DAYS);

    let code = match run(&internal_log) {
        Ok(code) => code,
        Err(err) => {
            emit_dispatcher_error(&internal_log, None, None, &err.to_string());
            0
        }
    };
    std::process::exit(code);
}

fn run(internal_log: &InternalLog) -> anyhow::Result<i32> {
    let trace_id = new_trace_id();
    let payload = HookPayload::from_reader(std::io::stdin().lock())?;

    let registry_path = resolve_registry_path()?;
    let registry = match Registry::load(&registry_path) {
        Ok(r) => r,
        Err(e) => {
            emit_dispatcher_error(
                internal_log,
                Some(trace_id.clone()),
                Some(payload.session_id.clone()),
                &format!("registry load: {e}"),
            );
            return Ok(0);
        }
    };

    // dispatcher.started fires *after* registry load so we can include
    // loaded_plugin_count. If the registry failed to load, we've
    // already emitted internal.dispatcher_error above.
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
    let tiers = group_by_priority(&registry, matched);

    eprintln!(
        "factory-dispatcher trace={} event={} tool={} host_abi={} matched_tiers={}",
        trace_id,
        payload.event_name,
        payload.tool_name,
        HOST_ABI_VERSION,
        tiers.len(),
    );

    // Execution layer stub — replaced in S-1.5 / S-1.6. For now we
    // print tier metadata so operators can verify routing end-to-end
    // without plugins actually executing.
    let mut any_block = false;
    for (i, tier) in tiers.iter().enumerate() {
        for entry in tier {
            let result = run_plugin_stub(entry.name.clone());
            eprintln!(
                "  tier={i} plugin={} outcome={:?}",
                result.plugin_name, result.outcome,
            );
            if matches!(result.outcome, PluginOutcome::Block { .. })
                && entry.on_error(&registry.defaults) == OnError::Block
            {
                any_block = true;
            }
        }
    }

    Ok(if any_block { 2 } else { 0 })
}

fn run_plugin_stub(name: String) -> PluginResultStub {
    // S-1.5 / S-1.6 will replace this with real wasmtime execution.
    PluginResultStub {
        plugin_name: name,
        outcome: PluginOutcome::Continue,
    }
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

    // Stderr mirror: even if InternalLog init succeeded, operators
    // often tail stderr during early bring-up. Keep the line-shape
    // compatible with the 0.0.1 implementation.
    eprintln!(
        r#"{{"type":"internal.dispatcher_error","message":{:?}}}"#,
        msg
    );
}
