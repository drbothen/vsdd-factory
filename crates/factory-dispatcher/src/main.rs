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

use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::registry::{OnError, Registry};
use factory_dispatcher::routing::{
    PluginOutcome, PluginResultStub, group_by_priority, match_plugins,
};
use factory_dispatcher::{HOST_ABI_VERSION, new_trace_id};
use std::path::PathBuf;

const ENV_PLUGIN_ROOT: &str = "CLAUDE_PLUGIN_ROOT";

fn main() {
    // Dispatcher must never propagate a raw panic into Claude Code;
    // any unhandled error becomes a structured `internal.dispatcher_error`
    // event and exits 0 (non-blocking).
    let code = match run() {
        Ok(code) => code,
        Err(err) => {
            emit_dispatcher_error(&err.to_string());
            0
        }
    };
    std::process::exit(code);
}

fn run() -> anyhow::Result<i32> {
    let trace_id = new_trace_id();
    let payload = HookPayload::from_reader(std::io::stdin().lock())?;

    let registry_path = resolve_registry_path()?;
    let registry = match Registry::load(&registry_path) {
        Ok(r) => r,
        Err(e) => {
            emit_dispatcher_error(&format!("registry load: {e}"));
            return Ok(0);
        }
    };

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

fn emit_dispatcher_error(msg: &str) {
    // The proper path is S-1.7's internal log writer. Until then we
    // surface errors on stderr (which Claude Code captures) so the
    // dispatcher stays debuggable.
    eprintln!(
        r#"{{"type":"internal.dispatcher_error","message":{:?}}}"#,
        msg
    );
}
