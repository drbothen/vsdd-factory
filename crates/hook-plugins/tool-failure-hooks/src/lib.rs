//! tool-failure-hooks — PostToolUseFailure WASM hook plugin.
//!
//! Emits `tool.error` with 2 plugin-set fields per BC-4.08.001:
//!   - `tool_name` — name of the failing tool (from envelope); defaults to "unknown" if absent (EC-002)
//!   - `error_message` — error message text (from envelope); truncated to 1000 chars if over limit (EC-001);
//!     defaults to "" if absent (EC-003)
//!
//! 4 host-enriched fields are auto-injected by the `emit_event` host fn from `HostContext`
//! (BC-1.05.012): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//!
//! 4 construction-time fields are set by the dispatcher:
//! `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! Wire payload: exactly 10 fields (2 plugin-set + 4 host-enriched + 4 construction-time)
//! per BC-4.08.001 Postcondition 2.
//!
//! Plugin is unconditionally stateless (once key ABSENT from hooks.json.template per
//! BC-4.08.002 Invariant 1 — PostToolUseFailure fires per-failure; defensive omission
//! mirrors S-5.03 worktree pattern).
//!
//! No `exec_subprocess` call is made (BC-4.08.001 Invariant 1). No `read_file` call is made.
//! All data comes from the incoming envelope's `tool_input` fields.
//!
//! 8 RESERVED_FIELDS that plugin MUST NOT set:
//!   Host-enriched (4): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
//!   Construction-time (4): `ts`, `ts_epoch`, `schema_version`, `type`

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level tool-failure hook logic with injectable emit callback.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime — same pattern as session-end-telemetry
/// and worktree-hooks.
///
/// - `emit_fn`: called with (event_type, fields) to emit a telemetry event;
///   called exactly once per invocation
///
/// Data sources (from payload.tool_input):
///   - `tool_name`: string field; defaults to "unknown" if absent or empty (EC-002)
///   - `error_message`: string field; truncated to 1000 chars if over limit (EC-001);
///     defaults to "" if absent (EC-003)
///
/// RESERVED_FIELDS the plugin MUST NOT set (8 total):
///   Host-enriched: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
///   Construction-time: `ts`, `ts_epoch`, `schema_version`, `type`
pub fn tool_failure_hook_logic<F>(ctx: HookPayload, emit: F) -> HookResult
where
    F: Fn(&str, &[(&str, &str)]),
{
    // Resolve tool_name: absent or empty → "unknown" sentinel (EC-002)
    let tool_name_raw = ctx
        .tool_input
        .get("tool_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let tool_name = if tool_name_raw.is_empty() {
        "unknown"
    } else {
        tool_name_raw
    };

    // Resolve error_message: absent → ""; over 1000 chars → truncate to 1000 (EC-001/EC-003)
    let error_message_raw = ctx
        .tool_input
        .get("error_message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let error_message = if error_message_raw.len() > 1000 {
        &error_message_raw[..1000]
    } else {
        error_message_raw
    };

    // Emit exactly once with the 2 plugin-set fields; RESERVED_FIELDS are NOT set here.
    emit("tool.error", &[("tool_name", tool_name), ("error_message", error_message)]);

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `tool_failure_hook_logic`.
pub fn on_post_tool_use_failure(payload: HookPayload) -> HookResult {
    tool_failure_hook_logic(payload, |event_type, fields| {
        vsdd_hook_sdk::host::emit_event(event_type, fields);
    })
}
