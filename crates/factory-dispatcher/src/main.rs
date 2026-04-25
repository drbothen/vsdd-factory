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
//! by the executor; only `dispatcher.*` and `internal.dispatcher_error`
//! are emitted here.

use std::path::PathBuf;
use std::sync::Arc;

use factory_dispatcher::engine::{EpochTicker, build_engine};
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::{
    DEFAULT_RETENTION_DAYS, DISPATCHER_STARTED, INTERNAL_DISPATCHER_ERROR, InternalEvent,
    InternalLog,
};
use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::Registry;
use factory_dispatcher::routing::{group_by_priority, match_plugins};
use factory_dispatcher::{HOST_ABI_VERSION, new_trace_id};

const ENV_PLUGIN_ROOT: &str = "CLAUDE_PLUGIN_ROOT";
const ENV_PROJECT_DIR: &str = "CLAUDE_PROJECT_DIR";

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
            emit_dispatcher_error(
                &internal_log,
                Some(trace_id.clone()),
                Some(payload.session_id.clone()),
                &format!("registry load: {e}"),
            );
            return Ok(0);
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
    let tiers = group_by_priority(&registry, matched);

    eprintln!(
        "factory-dispatcher trace={} event={} tool={} host_abi={} matched_tiers={}",
        trace_id,
        payload.event_name,
        payload.tool_name,
        HOST_ABI_VERSION,
        tiers.len(),
    );

    if tiers.is_empty() {
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
    let cache = PluginCache::new(engine.clone());

    let mut base_host_ctx = HostContext::new(
        "", // executor overrides per plugin
        env!("CARGO_PKG_VERSION"),
        payload.session_id.clone(),
        trace_id.clone(),
    );
    base_host_ctx.internal_log = Some(internal_log.clone());
    base_host_ctx.cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let payload_json = serde_json::to_vec(&payload)?;

    let inputs = ExecutorInputs {
        engine: &engine,
        cache: &cache,
        registry: &registry,
        payload_json,
        base_host_ctx,
        internal_log: internal_log.clone(),
    };

    let summary = execute_tiers(inputs, tiers).await;

    eprintln!(
        "  plugins_run={} total_ms={} block_intent={} exit_code={}",
        summary.per_plugin_results.len(),
        summary.total_elapsed_ms,
        summary.block_intent,
        summary.exit_code,
    );

    Ok(summary.exit_code)
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

    eprintln!(
        r#"{{"type":"internal.dispatcher_error","message":{:?}}}"#,
        msg
    );
}
