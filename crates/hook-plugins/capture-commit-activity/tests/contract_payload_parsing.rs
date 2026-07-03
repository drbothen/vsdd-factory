//! AC-2: Parses `git commit` invocations from PostToolUse/Bash `tool_input.command`.
//! EC-002: Non-git bash command → Continue (no-op, no subprocess call).
//!
//! Exercises `is_git_commit_command` (pure predicate) and `commit_hook_logic`
//! (integration of the predicate with the payload routing).

use capture_commit_activity::{commit_hook_logic, is_git_commit_command};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------------

fn make_payload(tool_name: &str, command: &str) -> HookPayload {
    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: tool_name.to_string(),
        session_id: "sess-parse".to_string(),
        dispatcher_trace_id: "trace-parse".to_string(),
        tool_input: serde_json::json!({"command": command}),
        tool_response: Some(serde_json::json!({"interrupted": false})),
        plugin_config: serde_json::Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
    }
}

// ---------------------------------------------------------------------------
// is_git_commit_command — unit tests for the pure predicate
// ---------------------------------------------------------------------------

/// AC-2 happy path: bare `git commit -m '…'` is detected.
#[test]
fn test_BC_4_03_001_parse_detects_bare_git_commit() {
    let result = is_git_commit_command("git commit -m 'fix: typo'");
    assert!(
        result,
        "bare git commit must be detected as a commit command"
    );
}

/// AC-2: `git commit --amend` is also a git commit invocation.
#[test]
fn test_BC_4_03_001_parse_detects_git_commit_amend() {
    let result = is_git_commit_command("git commit --amend --no-edit");
    assert!(
        result,
        "git commit --amend must be detected as a commit command"
    );
}

/// AC-2: compound command `echo hi && git commit -m 'x'` contains git commit.
#[test]
fn test_BC_4_03_001_parse_detects_git_commit_in_compound_command() {
    let result = is_git_commit_command("echo hi && git commit -m 'x'");
    assert!(
        result,
        "compound command containing git commit must be detected"
    );
}

/// EC-002: `git status` is NOT a git commit invocation.
#[test]
fn test_BC_4_03_001_parse_rejects_git_status() {
    let result = is_git_commit_command("git status");
    assert!(!result, "git status must not match git commit");
}

/// EC-002: `git commit-tree` must NOT match (not a commit subcommand).
#[test]
fn test_BC_4_03_001_parse_rejects_git_commit_tree() {
    let result = is_git_commit_command("git commit-tree abc123");
    assert!(!result, "git commit-tree must not match git commit");
}

/// EC-002: empty string must not match.
#[test]
fn test_BC_4_03_001_parse_rejects_empty_command() {
    let result = is_git_commit_command("");
    assert!(!result, "empty command must not match");
}

/// EC-002: `ls -la` (non-git command) must not match.
#[test]
fn test_BC_4_03_001_parse_rejects_non_git_command() {
    let result = is_git_commit_command("ls -la");
    assert!(!result, "non-git command must not match");
}

// ---------------------------------------------------------------------------
// commit_hook_logic — payload routing
// ---------------------------------------------------------------------------

/// AC-2: non-Bash tool_name → Continue without calling git log.
///
/// The hook only fires on PostToolUse/Bash. Other tool types must be ignored.
#[test]
fn test_BC_4_03_001_parse_non_bash_tool_returns_continue() {
    let payload = make_payload("Edit", "git commit -m 'x'");
    let git_log_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || {
            git_log_called.set(true);
            Ok((0, "abc1234\n".to_string()))
        },
        |_| {},
    );
    assert!(
        !git_log_called.get(),
        "git log must not be called for non-Bash tool"
    );
}

/// EC-002: Bash tool but non-git command → Continue, no subprocess call.
#[test]
fn test_BC_4_03_001_parse_non_git_command_returns_continue_no_subprocess() {
    let payload = make_payload("Bash", "cargo build --release");
    let git_log_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || {
            git_log_called.set(true);
            Ok((0, "abc1234\n".to_string()))
        },
        |_| {},
    );
    assert!(
        !git_log_called.get(),
        "git log must not be called for non-git commands"
    );
}

/// AC-2: `git commit` command → triggers git log subprocess (no-op here, just
/// verifies the routing reaches the subprocess step).
#[test]
fn test_BC_4_03_001_parse_git_commit_command_triggers_subprocess() {
    let payload = make_payload("Bash", "git commit -m 'feat: add logging'");
    let git_log_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || {
            git_log_called.set(true);
            Ok((0, "deadbeef1234567890123456789012345678abcd\n".to_string()))
        },
        |_| {},
    );
    // Unreachable until implemented, but proves the routing check happens.
    assert!(
        git_log_called.get(),
        "git log must be called for git commit commands"
    );
}

/// TV-001 canonical test vector: PostToolUse payload with `git commit -m 'initial commit'`.
/// Expected: subprocess called, emit called, result is Continue.
#[test]
fn test_TV_001_canonical_git_commit_payload_routes_to_emit() {
    let payload = HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-tv-001".to_string(),
        dispatcher_trace_id: "trace-tv-001".to_string(),
        tool_input: serde_json::json!({"command": "git commit -m 'initial commit'"}),
        tool_response: Some(serde_json::json!({
            "interrupted": false,
            "stdout": "[main abc1234] initial commit\n 1 file changed"
        })),
        plugin_config: serde_json::Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
    };

    let emit_called = std::cell::Cell::new(false);
    let result = commit_hook_logic(
        payload,
        || Ok((0, "abc1234def5678901234567890123456789012345\n".to_string())),
        |_fields| {
            emit_called.set(true);
        },
    );
    assert!(
        emit_called.get(),
        "emit must be called for a valid git commit"
    );
    assert_eq!(result, HookResult::Continue);
}
