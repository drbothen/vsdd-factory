//! session-start-telemetry — SessionStart WASM hook plugin.
//!
//! Emits `session.started` with 6 plugin-set fields per BC-4.04.001:
//!   - `factory_version` — compile-time `env!("CARGO_PKG_VERSION")`
//!   - `plugin_count`    — count of WASM plugins loaded in the dispatcher's PluginCache
//!   - `activated_platform` — read from `.claude/settings.local.json` via `read_file` host fn
//!   - `factory_health`  — result of `factory-health --brief` via `exec_subprocess`
//!   - `tool_deps`       — whitelist-filtered tool versions; JSON-serialized, ≤ 512 bytes
//!   - `timestamp`       — ISO-8601 UTC with millisecond precision and `Z` suffix
//!
//! 4 host-enriched fields are auto-injected by the `emit_event` host fn from `HostContext`
//! (BC-1.05.012): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//!
//! 4 construction-time fields are set by `InternalEvent::now()`:
//! `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! Reads `vsdd-factory.activated_platform` from `.claude/settings.local.json` via the
//! canonical `read_file` host fn. Invokes `factory-health --brief` via `exec_subprocess`.
//!
//! Plugin is unconditionally stateless (idempotency enforced at Layer 1 by Claude Code's
//! `once: true` directive in hooks.json.template per BC-4.04.003 + BC-4.04.004).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Top-level hook logic — called by the WASI entry point in `main.rs`.
///
/// Stub: body deferred to S-5.01 GREEN phase implementation.
/// All behavior is contractually specified by BC-4.04.001..005 + VP-065.
pub fn session_start_hook_logic(payload: HookPayload) -> HookResult {
    // TODO(S-5.01 GREEN): implement full session.started emission per BC-4.04.001..005
    //   1. Read activated_platform from .claude/settings.local.json via read_file host fn
    //      (BC-4.04.001 PC-2; fail-open → "unknown" per BC-4.04.001 EC-002)
    //   2. Invoke factory-health --brief via exec_subprocess, timeout=5000ms
    //      (BC-4.04.002; map exit code to "healthy"/"warnings"/"errors"/"unknown")
    //   3. Detect tool_deps for whitelist ["git","jq","yq","rustc","cargo"]
    //      (BC-4.04.001 PC-2; defense-in-depth eviction per BC-4.04.001 EC-003)
    //   4. Emit session.started with 6 plugin-set fields via emit_event host fn
    //      (BC-4.04.001 PC-1; plugin is unconditionally stateless per BC-4.04.003)
    let _ = payload;
    todo!(
        "S-5.01 GREEN phase: implement session.started emission per BC-4.04.001..005 + VP-065 harness contracts"
    )
}
