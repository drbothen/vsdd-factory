//! AC: "Handles gh pr create, gh pr merge, gh pr close subcommands"
//!     "Handles non-PR bash commands gracefully (no-op)"
//!
//! BC-4.04.001 — detect `gh pr create/merge/close` in PostToolUse
//! `tool_input.command`; return Continue for non-gh-pr commands.
//!
//! Tests derived from v1.1 BC candidates BC-4.04.001 and the bats
//! test corpus (echoed mention, whitespace boundaries, non-Bash tool,
//! failed command).

use capture_pr_activity::{PrSubcommand, detect_pr_subcommand, extract_command};
use vsdd_hook_sdk::HookPayload;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn posttooluse_payload(command: &str) -> HookPayload {
    let json = format!(
        r#"{{
            "event_name": "PostToolUse",
            "tool_name": "Bash",
            "session_id": "sess-test",
            "dispatcher_trace_id": "trace-test",
            "tool_input": {{"command": {command_json}}},
            "tool_response": {{"stdout": "", "stderr": "", "interrupted": false}}
        }}"#,
        command_json = serde_json::to_string(command).unwrap()
    );
    serde_json::from_str(&json).expect("fixture must parse")
}

fn non_bash_payload() -> HookPayload {
    let json = r#"{
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "sess-test",
        "dispatcher_trace_id": "trace-test",
        "tool_input": {},
        "tool_response": {"stdout": ""}
    }"#;
    serde_json::from_str(json).expect("fixture must parse")
}

// ── BC-4.04.001 — subcommand detection ───────────────────────────────────────

/// BC-4.04.001 postcondition: `gh pr create` at command start detected.
#[test]
fn test_BC_4_04_001_detects_gh_pr_create_at_start() {
    let cmd = "gh pr create --title \"feat: thing\" --body \"x\"";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Create));
}

/// BC-4.04.001 postcondition: `gh pr create` after separator detected.
#[test]
fn test_BC_4_04_001_detects_gh_pr_create_after_semicolon() {
    let cmd = "git add . ; gh pr create --title t --body b";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Create));
}

/// BC-4.04.001 postcondition: `gh pr create` after pipe detected.
#[test]
fn test_BC_4_04_001_detects_gh_pr_create_after_pipe() {
    let cmd = "echo x | gh pr create --title t --body b";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Create));
}

/// BC-4.04.001 postcondition: `gh pr merge` detected.
#[test]
fn test_BC_4_04_001_detects_gh_pr_merge() {
    let cmd = "gh pr merge https://github.com/owner/repo/pull/99 --squash";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Merge));
}

/// BC-4.04.001 postcondition: `gh pr close` detected (new AC beyond bash hook).
#[test]
fn test_BC_4_04_001_detects_gh_pr_close() {
    let cmd = "gh pr close 42";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Close));
}

/// BC-4.04.001 postcondition: non-gh command → None (no-op).
#[test]
fn test_BC_4_04_001_non_gh_command_returns_none() {
    let cmd = "echo hello";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

/// BC-4.04.001 postcondition: `git commit` → None (not a PR command).
#[test]
fn test_BC_4_04_001_git_commit_returns_none() {
    let cmd = "git commit -m \"fix: stuff\"";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

/// BC-4.04.001 — echoed mention of `gh pr create` must NOT match.
/// The regex must anchor to a real invocation boundary, not a string literal.
#[test]
fn test_BC_4_04_001_echoed_mention_of_gh_pr_create_is_not_matched() {
    let cmd = "echo \"Use gh pr create to open a PR\"";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

/// BC-4.04.001 — `gh pr status` is not one of the three handled subcommands.
#[test]
fn test_BC_4_04_001_gh_pr_status_returns_none() {
    let cmd = "gh pr status";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

/// BC-4.04.001 — `gh pr view` is not one of the three handled subcommands.
#[test]
fn test_BC_4_04_001_gh_pr_view_returns_none() {
    let cmd = "gh pr view 42";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

// ── extract_command ───────────────────────────────────────────────────────────

/// BC-4.04.001 precondition: command is extractable from tool_input.
#[test]
fn test_BC_4_04_001_extracts_command_from_payload() {
    let p = posttooluse_payload("gh pr create --title t --body b");
    let cmd = extract_command(&p);
    assert!(cmd.is_some());
    assert!(cmd.unwrap().contains("gh pr create"));
}

/// BC-4.04.001 precondition: missing command field returns None (no panic).
#[test]
fn test_BC_4_04_001_missing_command_returns_none() {
    let json = r#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "s",
        "dispatcher_trace_id": "t",
        "tool_input": {}
    }"#;
    let p: HookPayload = serde_json::from_str(json).expect("fixture must parse");
    assert_eq!(extract_command(&p), None);
}

/// BC-4.04.001 precondition: non-Bash tool — command extraction still works
/// (caller is responsible for filtering by tool_name before calling dispatch).
#[test]
fn test_BC_4_04_001_non_bash_tool_extract_command_returns_none_when_no_command_field() {
    let p = non_bash_payload();
    // The non-Bash payload has no tool_input.command field.
    assert_eq!(extract_command(&p), None);
}
