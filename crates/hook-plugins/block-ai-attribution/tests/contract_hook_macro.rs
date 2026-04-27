//! Tests verifying the #[hook] / on_hook_logic signature contract.
//!
//! BC-2.01.002: HookResult exit codes Continue=0 / Block=2 / Error=1.
//! VP-038: SDK HookResult Exit Codes Are Stable.
//!
//! These tests validate:
//!   - on_hook_logic accepts HookPayload and returns HookResult.
//!   - Block result carries a non-empty reason string.
//!   - Continue result maps to exit code 0 (BC-2.01.002 postcondition 1).
//!   - Block result maps to exit code 2 (BC-2.01.002 postcondition 2).

use block_ai_attribution::on_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_payload(command: &str) -> HookPayload {
    let json = serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Bash",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_input": { "command": command }
    });
    serde_json::from_value(json).expect("fixture must deserialize")
}

fn make_non_bash_payload() -> HookPayload {
    let json = serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Edit",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_input": {}
    });
    serde_json::from_value(json).expect("fixture must deserialize")
}

// ---------------------------------------------------------------------------
// BC-2.01.002 — exit code contract
// ---------------------------------------------------------------------------

/// BC-2.01.002 postcondition 1: Continue maps to exit code 0.
/// VP-038: SDK HookResult exit codes are stable.
#[test]
fn test_BC_2_01_002_continue_exit_code_is_zero() {
    let payload = make_payload("echo hello");
    let result = on_hook_logic(payload);
    assert_eq!(
        result.exit_code(),
        0,
        "non-attributed, non-git command must return Continue (exit 0)"
    );
}

/// BC-2.01.002 postcondition 2: Block maps to exit code 2.
/// VP-038: SDK HookResult exit codes are stable.
#[test]
fn test_BC_2_01_002_block_exit_code_is_two() {
    let command =
        r#"git commit -m "fix: something\n\nCo-Authored-By: Claude <noreply@anthropic.com>""#;
    let payload = make_payload(command);
    let result = on_hook_logic(payload);
    assert_eq!(
        result.exit_code(),
        2,
        "git commit with AI attribution must return Block (exit 2)"
    );
}

/// AC: Returns HookResult::Block with a non-empty reason string.
#[test]
fn test_BC_2_01_002_block_reason_is_non_empty() {
    let command =
        r#"git commit -m "fix: something\n\nCo-Authored-By: Claude <noreply@anthropic.com>""#;
    let payload = make_payload(command);
    let result = on_hook_logic(payload);
    match result {
        HookResult::Block { reason } => {
            assert!(!reason.is_empty(), "Block reason must be non-empty");
        }
        other => panic!("Expected Block, got {other:?}"),
    }
}

/// AC: Block message is human-readable and includes the matched pattern.
#[test]
fn test_BC_2_01_002_block_reason_includes_matched_pattern() {
    let command = r#"git commit -m "fix\n\nCo-Authored-By: Claude <noreply@anthropic.com>""#;
    let payload = make_payload(command);
    let result = on_hook_logic(payload);
    match result {
        HookResult::Block { reason } => {
            // Reason must mention what was matched so the user can act on it.
            let reason_lower = reason.to_lowercase();
            let mentions_pattern = reason_lower.contains("co-authored-by")
                || reason_lower.contains("claude")
                || reason_lower.contains("attribution")
                || reason_lower.contains("anthropic");
            assert!(
                mentions_pattern,
                "Block reason must reference the matched pattern; got: {reason:?}"
            );
        }
        other => panic!("Expected Block, got {other:?}"),
    }
}

/// AC: Non-git tool (e.g. Edit) always returns Continue regardless of tool_input.
#[test]
fn test_BC_2_01_002_non_bash_tool_returns_continue() {
    let payload = make_non_bash_payload();
    let result = on_hook_logic(payload);
    assert_eq!(result.exit_code(), 0, "non-Bash tool must return Continue");
}
