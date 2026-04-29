//! worktree-hooks — WorktreeCreate / WorktreeRemove WASM hook plugin.
//!
//! Single crate handling BOTH events per BC-4.07.004 single-crate-two-entries design.
//!
//! For WorktreeCreate: emits `worktree.created` with 2 plugin-set fields per BC-4.07.001:
//!   - `worktree_path` — path to the new worktree (from envelope)
//!   - `worktree_name` — name of the worktree (from envelope; defaults to "" if absent)
//!
//! For WorktreeRemove: emits `worktree.removed` with 1 plugin-set field per BC-4.07.002:
//!   - `worktree_path` — path of the removed worktree (from envelope; defaults to "" if absent)
//!
//! 4 host-enriched fields are auto-injected by the `emit_event` host fn from `HostContext`
//! (BC-1.05.012): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//!
//! 4 construction-time fields are set by the dispatcher:
//! `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! Wire payload count:
//!   - WorktreeCreate: 10 fields (2 plugin-set + 4 host-enriched + 4 construction-time)
//!   - WorktreeRemove: 9 fields (1 plugin-set + 4 host-enriched + 4 construction-time)
//!
//! Plugin is unconditionally stateless (EC-001: once key MUST be absent from
//! hooks.json.template — worktree events can re-fire on reconnect; defensive omission).
//!
//! No `exec_subprocess` call is made (BC-4.07.001 Invariant 2 + BC-4.07.002 Invariant 2).
//! No `read_file` call is made (Option A scoping — zero-capability profile).
//! All data comes from the incoming envelope's `tool_input` fields.
//!
//! 8 RESERVED_FIELDS that plugin MUST NOT set:
//!   Host-enriched (4): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
//!   Construction-time (4): `ts`, `ts_epoch`, `schema_version`, `type`

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level worktree hook logic with injectable emit callback.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime — same pattern as session-end-telemetry.
///
/// Dispatches internally on `payload.event_name` (NOT event_type — per CRIT-003):
///   - "WorktreeCreate": emits `worktree.created` with worktree_path + worktree_name
///   - "WorktreeRemove": emits `worktree.removed` with worktree_path
///   - unknown event_name: emits nothing, returns Ok (defensive no-op)
///
/// - `emit_fn`: called with (event_type, fields) to emit a telemetry event
///
/// Data sources (from payload.tool_input):
///   - `worktree_path`: string field; defaults to "" if absent (EC-003)
///   - `worktree_name`: string field; defaults to "" if absent (WorktreeCreate only; EC-003)
///
/// RESERVED_FIELDS the plugin MUST NOT set (8 total):
///   Host-enriched: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
///   Construction-time: `ts`, `ts_epoch`, `schema_version`, `type`
pub fn worktree_hook_logic<Emit>(payload: HookPayload, emit_fn: Emit) -> HookResult
where
    Emit: Fn(&str, &[(&str, &str)]),
{
    unimplemented!("S-5.03 GREEN")
}

// ---------------------------------------------------------------------------
// Top-level entry points called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `worktree_hook_logic`.
pub fn on_worktree_event(payload: HookPayload) -> HookResult {
    worktree_hook_logic(payload, |event_type, fields| {
        vsdd_hook_sdk::host::emit_event(event_type, fields);
    })
}
