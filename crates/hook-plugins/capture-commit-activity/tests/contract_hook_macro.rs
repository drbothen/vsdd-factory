//! AC-1: capture-commit-activity.wasm replaces the legacy bash hook.
//!
//! Verifies that `commit_hook_logic` is exported with the correct signature
//! and that the crate compiles against the vsdd-hook-sdk interface. The
//! macro entry path is exercised via `commit_hook_logic`; wasmtime is not
//! required for shape-level coverage.
//!
//! BC-4.03.001 postcondition 1: on_hook replaces the stub (wasm compiles,
//! SDK types are used throughout).

use capture_commit_activity::commit_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------------

fn git_commit_payload() -> HookPayload {
    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-hook-macro".to_string(),
        dispatcher_trace_id: "trace-hook-macro".to_string(),
        tool_input: serde_json::json!({"command": "git commit -m 'initial commit'"}),
        tool_response: Some(serde_json::json!({"interrupted": false, "stdout": "", "stderr": ""})),
        plugin_config: serde_json::Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
    }
}

fn git_log_ok() -> impl FnOnce() -> Result<(i32, String), String> {
    || Ok((0, "abc1234def5678901234567890123456789012345\n".to_string()))
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_hook_macro_entry_point_has_correct_signature
// ---------------------------------------------------------------------------

/// BC-4.03.001 postcondition 1: the plugin exposes an on_hook-compatible entry
/// point via commit_hook_logic (the testable pure-logic surface the macro wires).
///
/// Verifies the function signature compiles and panics on unimplemented!() —
/// proving the stub is a real implementation target, not a tautology.
#[test]
fn test_BC_4_03_001_hook_macro_entry_point_has_correct_signature() {
    let payload = git_commit_payload();
    let _result = commit_hook_logic(payload, git_log_ok(), |_fields| {});
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_hook_result_is_sdk_type
// ---------------------------------------------------------------------------

/// Verifies that `HookResult` from the SDK is the return type —
/// the production entry point must return Continue on the happy path,
/// not a bare integer.
///
/// This test exercises the type surface at the call site; it panics with
/// unimplemented!() confirming the stub is not returning a default value.
#[test]
fn test_BC_4_03_001_hook_result_is_sdk_type() {
    let payload = git_commit_payload();
    let result = commit_hook_logic(payload, git_log_ok(), |_| {});
    // Unreachable until implemented, but ensures type inference works
    assert_eq!(result, HookResult::Continue);
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_commit_event_fields_has_five_required_fields
// ---------------------------------------------------------------------------

/// AC: emit commit.made event with fields: sha, branch, message, author, timestamp.
///
/// Verifies that `build_commit_fields` produces all five required fields.
/// Calls the production function — panics with unimplemented!() until the
/// implementer fills it in.
#[test]
fn test_BC_4_03_001_commit_event_fields_has_five_required_fields() {
    use capture_commit_activity::build_commit_fields;
    let payload = git_commit_payload();
    let fields = build_commit_fields("abc1234def5678901234567890123456789012345", &payload);
    // Unreachable until implemented; when it is, all five must be non-empty.
    assert!(!fields.sha.is_empty());
    assert!(!fields.branch.is_empty());
    assert!(!fields.message.is_empty());
    assert!(!fields.author.is_empty());
    assert!(!fields.timestamp.is_empty());
}
