//! AC: "`capture-pr-activity.wasm` compiled and registered"
//! AC: "Handles non-PR bash commands gracefully (no-op)"
//!
//! BC-4.03.001 structural template anchor: the plugin must provide an
//! `on_hook`-style entry point consistent with the SS-04 plugin pattern.
//!
//! These tests drive `dispatch()` — the function that the `#[hook]` macro
//! entry point delegates to. In native test targets the host FFI is absent,
//! so any call that reaches `unimplemented!()` will panic, confirming RED.
//!
//! The non-PR no-op test is the only one that should NOT call into host FFI;
//! it must return `HookResult::Continue` without panicking once implemented.

use capture_pr_activity::dispatch;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ── Helpers ──────────────────────────────────────────────────────────────────

fn payload_with_command(event_name: &str, tool_name: &str, command: &str) -> HookPayload {
    let json = format!(
        r#"{{
            "event_name": {event_name_json},
            "tool_name": {tool_name_json},
            "session_id": "sess-test",
            "dispatcher_trace_id": "trace-test",
            "tool_input": {{"command": {command_json}}},
            "tool_response": {{"stdout": "", "stderr": "", "interrupted": false}}
        }}"#,
        event_name_json = serde_json::to_string(event_name).unwrap(),
        tool_name_json = serde_json::to_string(tool_name).unwrap(),
        command_json = serde_json::to_string(command).unwrap(),
    );
    serde_json::from_str(&json).expect("fixture must parse")
}

fn payload_non_bash(tool_name: &str) -> HookPayload {
    let json = format!(
        r#"{{
            "event_name": "PostToolUse",
            "tool_name": {tool_name_json},
            "session_id": "sess-test",
            "dispatcher_trace_id": "trace-test",
            "tool_input": {{}},
            "tool_response": {{"stdout": ""}}
        }}"#,
        tool_name_json = serde_json::to_string(tool_name).unwrap(),
    );
    serde_json::from_str(&json).expect("fixture must parse")
}

// ── BC-4.03.001 structural anchor ────────────────────────────────────────────

/// BC-4.03.001 postcondition: the crate compiles and exposes a public
/// `dispatch` function (the `#[hook]` entry point's delegate). This test
/// exists as a compile-time check — if it compiles, the public surface is correct.
///
/// At run time it panics via `unimplemented!()` — this is the expected RED state.
#[test]
#[should_panic]
fn test_BC_4_03_001_dispatch_fn_exists_and_panics_in_stub() {
    let p = payload_with_command("PostToolUse", "Bash", "gh pr create --title t --body b");
    let _ = dispatch(&p);
}

/// BC-4.04.001 postcondition: non-Bash tool → Continue (no-op, no panic).
///
/// This must NOT call into host FFI at all — it returns Continue immediately
/// after checking tool_name. So it should NOT reach `unimplemented!()`.
/// If the implementer mis-routes it, the test panics.
///
/// RED expectation: panics on `unimplemented!()` because dispatch itself
/// is not yet implemented (even the early-return path).
#[test]
#[should_panic]
fn test_BC_4_04_001_non_bash_tool_dispatches_to_continue_stub_panics() {
    let p = payload_non_bash("Edit");
    let _ = dispatch(&p);
}

/// BC-4.04.001 postcondition: non-PR bash command → Continue (no-op).
///
/// Same situation: once implemented, `dispatch` returns Continue without
/// calling host FFI. RED: panics in stub.
#[test]
#[should_panic]
fn test_BC_4_04_001_non_pr_bash_command_dispatches_to_continue_stub_panics() {
    let p = payload_with_command("PostToolUse", "Bash", "echo hello");
    let _ = dispatch(&p);
}

/// BC-4.04.001 postcondition: `gh pr create` payload routes to PR create path.
/// RED: panics in stub.
#[test]
#[should_panic]
fn test_BC_4_04_001_gh_pr_create_payload_routes_to_create_path_stub_panics() {
    let p = payload_with_command(
        "PostToolUse",
        "Bash",
        "gh pr create --title \"feat: thing\" --body \"x\"",
    );
    let _ = dispatch(&p);
}

/// BC-4.04.001 postcondition: `gh pr merge` payload routes to PR merge path.
/// RED: panics in stub.
#[test]
#[should_panic]
fn test_BC_4_04_001_gh_pr_merge_payload_routes_to_merge_path_stub_panics() {
    let p = payload_with_command(
        "PostToolUse",
        "Bash",
        "gh pr merge https://github.com/owner/repo/pull/99 --squash",
    );
    let _ = dispatch(&p);
}

/// BC-4.04.001 postcondition: `gh pr close` payload routes to PR close path.
/// RED: panics in stub.
#[test]
#[should_panic]
fn test_BC_4_04_001_gh_pr_close_payload_routes_to_close_path_stub_panics() {
    let p = payload_with_command("PostToolUse", "Bash", "gh pr close 42");
    let _ = dispatch(&p);
}
