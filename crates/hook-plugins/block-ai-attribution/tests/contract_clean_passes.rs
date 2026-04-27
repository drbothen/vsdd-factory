//! Tests verifying that non-attributed payloads return HookResult::Continue.
//!
//! BC-4.05.002 (v1.1 candidate): non-git command or attribution-free commit
//! → HookResult::Continue.
//! EC-001: Non-git command → Continue.
//! BC-2.01.002 postcondition 1: Continue.exit_code() == 0.

use block_ai_attribution::on_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn bash_payload(command: &str) -> HookPayload {
    let json = serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Bash",
        "session_id": "sess-clean",
        "dispatcher_trace_id": "trace-clean",
        "tool_input": { "command": command }
    });
    serde_json::from_value(json).expect("fixture must deserialize")
}

fn assert_continue(result: HookResult, label: &str) {
    match result {
        HookResult::Continue => {}
        other => panic!("{label}: expected Continue, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// EC-001: Non-git commands pass through
// ---------------------------------------------------------------------------

/// EC-001: `ls` is not a git command → Continue.
#[test]
fn test_BC_4_05_002_ec001_ls_returns_continue() {
    let result = on_hook_logic(bash_payload("ls -la"));
    assert_continue(result, "ls -la");
}

/// EC-001: `cargo build` is not a git command → Continue.
#[test]
fn test_BC_4_05_002_ec001_cargo_build_returns_continue() {
    let result = on_hook_logic(bash_payload("cargo build --release"));
    assert_continue(result, "cargo build");
}

/// EC-001: `git status` is not a commit command → Continue.
#[test]
fn test_BC_4_05_002_ec001_git_status_returns_continue() {
    let result = on_hook_logic(bash_payload("git status"));
    assert_continue(result, "git status");
}

/// EC-001: `git push` is not a commit command → Continue.
#[test]
fn test_BC_4_05_002_ec001_git_push_returns_continue() {
    let result = on_hook_logic(bash_payload("git push origin main"));
    assert_continue(result, "git push");
}

/// EC-001: `git log` → Continue.
#[test]
fn test_BC_4_05_002_ec001_git_log_returns_continue() {
    let result = on_hook_logic(bash_payload("git log --oneline -10"));
    assert_continue(result, "git log");
}

/// EC-001: empty command string → Continue (no crash).
#[test]
fn test_BC_4_05_002_ec001_empty_command_returns_continue() {
    let result = on_hook_logic(bash_payload(""));
    assert_continue(result, "empty command");
}

// ---------------------------------------------------------------------------
// Clean git commit → Continue
// ---------------------------------------------------------------------------

/// AC: Clean `git commit` with no attribution → Continue.
#[test]
fn test_BC_4_05_002_clean_git_commit_returns_continue() {
    let result = on_hook_logic(bash_payload("git commit -m \"fix: correct logic error\""));
    assert_continue(result, "clean git commit");
}

/// AC: Clean multi-line commit with conventional footer but no AI attribution.
#[test]
fn test_BC_4_05_002_clean_multiline_commit_returns_continue() {
    let cmd = "git commit -m \"feat: add retries\n\nSigned-off-by: Josh <josh@example.com>\"";
    let result = on_hook_logic(bash_payload(cmd));
    assert_continue(result, "clean multiline commit");
}

/// AC: `git commit --amend` with no attribution → Continue.
#[test]
fn test_BC_4_05_002_git_commit_amend_clean_returns_continue() {
    let result = on_hook_logic(bash_payload("git commit --amend --no-edit"));
    assert_continue(result, "git commit --amend clean");
}

/// BC-2.01.002 postcondition 1: Continue.exit_code() == 0.
#[test]
fn test_BC_2_01_002_continue_exit_code_zero_for_clean_payload() {
    let result = on_hook_logic(bash_payload("git commit -m \"chore: update deps\""));
    assert_eq!(result.exit_code(), 0, "Continue must map to exit code 0");
}

/// AC: tool_input with no `command` field → Continue (missing field is safe).
#[test]
fn test_BC_4_05_002_missing_command_field_returns_continue() {
    let json = serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Bash",
        "session_id": "s",
        "dispatcher_trace_id": "t",
        "tool_input": {}
    });
    let payload: HookPayload = serde_json::from_value(json).expect("deserialize");
    let result = on_hook_logic(payload);
    assert_continue(result, "missing tool_input.command");
}
