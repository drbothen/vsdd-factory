//! Edge cases from the story spec:
//!   EC-001: git commit fails (empty repo) → no-op, do not emit commit.made
//!   EC-002: non-git bash command → Continue (no-op, no subprocess call)
//!   EC-003: git log returns empty output → log warning, return Continue
//!
//! EC-002 is also covered in contract_payload_parsing.rs (pure predicate
//! tests). This file tests EC-002 at the commit_hook_logic integration level
//! plus EC-001 and EC-003.

use capture_commit_activity::{GitLogOutcome, call_git_log, commit_hook_logic};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------------

fn bash_payload(command: &str) -> HookPayload {
    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-ec".to_string(),
        dispatcher_trace_id: "trace-ec".to_string(),
        tool_input: serde_json::json!({"command": command}),
        tool_response: Some(serde_json::json!({"interrupted": false})),
        plugin_config: serde_json::Value::Null,
    }
}

// ---------------------------------------------------------------------------
// EC-001: git commit fails (empty repo / pre-commit hook rejected)
// ---------------------------------------------------------------------------

/// EC-001: when `git log` returns a non-zero exit code (e.g. empty repo),
/// `call_git_log` must return `GitLogOutcome::Failed`, not panic.
#[test]
fn test_BC_4_03_001_ec001_git_log_nonzero_exit_returns_failed() {
    let outcome = call_git_log(|| {
        Ok((
            128,
            "fatal: your current branch 'main' does not have any commits yet\n".to_string(),
        ))
    });
    match outcome {
        GitLogOutcome::Failed { exit_code, .. } => {
            assert_eq!(exit_code, 128, "must preserve exit code");
        }
        other => panic!("expected Failed for non-zero git log exit, got {other:?}"),
    }
}

/// EC-001: when git log fails, `commit_hook_logic` must NOT emit commit.made
/// and must return Continue (advisory hook never blocks).
#[test]
fn test_BC_4_03_001_ec001_failed_git_log_no_emit_returns_continue() {
    let payload = bash_payload("git commit -m 'x'");
    let emit_called = std::cell::Cell::new(false);
    let result = commit_hook_logic(
        payload,
        || Ok((128, "fatal: not a git repo\n".to_string())),
        |_| {
            emit_called.set(true);
        },
    );
    assert!(
        !emit_called.get(),
        "emit must NOT be called when git log fails (EC-001)"
    );
    assert_eq!(
        result,
        HookResult::Continue,
        "hook must still return Continue even when git log fails"
    );
}

// ---------------------------------------------------------------------------
// EC-002: non-git bash command → Continue, no subprocess
// ---------------------------------------------------------------------------

/// EC-002: `npm install` is a non-git command; no git log call, no emit,
/// returns Continue.
#[test]
fn test_BC_4_03_001_ec002_npm_command_no_subprocess_no_emit() {
    let payload = bash_payload("npm install");
    let git_log_called = std::cell::Cell::new(false);
    let emit_called = std::cell::Cell::new(false);
    let result = commit_hook_logic(
        payload,
        || {
            git_log_called.set(true);
            panic!("git log must not be called for non-git commands")
        },
        |_| {
            emit_called.set(true);
        },
    );
    assert!(!git_log_called.get());
    assert!(!emit_called.get());
    assert_eq!(result, HookResult::Continue);
}

/// EC-002: `echo 'git commit'` inside an echo should not be treated as a
/// git commit invocation. (The predicate must not match string literals.)
///
/// Note: this is a known hard case; the implementation is expected to handle
/// it via word-boundary anchoring (same as the bash hook did).
#[test]
fn test_BC_4_03_001_ec002_echo_git_commit_string_not_matched() {
    let payload = bash_payload("echo 'please run git commit now'");
    let emit_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || panic!("subprocess must not run for echo command"),
        |_| {
            emit_called.set(true);
        },
    );
    assert!(
        !emit_called.get(),
        "emit must not fire for echo containing 'git commit'"
    );
}

/// EC-002: `git push` is not a commit command.
#[test]
fn test_BC_4_03_001_ec002_git_push_not_matched() {
    let payload = bash_payload("git push origin main");
    let emit_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || panic!("subprocess must not run for git push"),
        |_| {
            emit_called.set(true);
        },
    );
    assert!(!emit_called.get(), "emit must not fire for git push");
}

// ---------------------------------------------------------------------------
// EC-003: git log returns empty output → log warning, return Continue
// ---------------------------------------------------------------------------

/// EC-003: when git log exits 0 but stdout is empty (rare race or shallow
/// clone edge), `call_git_log` must return `GitLogOutcome::EmptyOutput`.
#[test]
fn test_BC_4_03_001_ec003_git_log_empty_stdout_returns_empty_output() {
    let outcome = call_git_log(|| Ok((0, String::new())));
    match outcome {
        GitLogOutcome::EmptyOutput => {}
        other => panic!("expected EmptyOutput for empty git log stdout, got {other:?}"),
    }
}

/// EC-003: whitespace-only stdout (just `\n`) is also treated as empty.
#[test]
fn test_BC_4_03_001_ec003_git_log_whitespace_only_returns_empty_output() {
    let outcome = call_git_log(|| Ok((0, "\n".to_string())));
    match outcome {
        GitLogOutcome::EmptyOutput => {}
        other => panic!("expected EmptyOutput for whitespace-only git log stdout, got {other:?}"),
    }
}

/// EC-003: when git log is empty, `commit_hook_logic` must NOT emit and
/// must return Continue (not Error — the hook is advisory).
#[test]
fn test_BC_4_03_001_ec003_empty_git_log_no_emit_returns_continue() {
    let payload = bash_payload("git commit -m 'x'");
    let emit_called = std::cell::Cell::new(false);
    let result = commit_hook_logic(
        payload,
        || Ok((0, String::new())),
        |_| {
            emit_called.set(true);
        },
    );
    assert!(
        !emit_called.get(),
        "emit must NOT be called when git log is empty (EC-003)"
    );
    assert_eq!(
        result,
        HookResult::Continue,
        "hook must return Continue for EC-003 (empty git log — log warning, continue)"
    );
}

// ---------------------------------------------------------------------------
// VP-043 invariant: hooks-registry routing — non-Bash tool events pass through
// ---------------------------------------------------------------------------

/// VP-043: hook must never block non-Bash tool events.
/// The plugin is PostToolUse/Bash only; all other tool events must Continue.
#[test]
fn test_VP_043_non_bash_tool_always_continue() {
    for tool in &["Edit", "Write", "Read", "TodoWrite", "Glob"] {
        let payload = HookPayload {
            event_name: "PostToolUse".to_string(),
            tool_name: tool.to_string(),
            session_id: "sess-vp043".to_string(),
            dispatcher_trace_id: "trace-vp043".to_string(),
            tool_input: serde_json::json!({"command": "git commit -m 'x'"}),
            tool_response: Some(serde_json::json!({"interrupted": false})),
            plugin_config: serde_json::Value::Null,
        };
        let result = commit_hook_logic(
            payload,
            || panic!("git log must not be called for non-Bash tool {tool}"),
            |_| {},
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "VP-043: non-Bash tool {tool} must always yield Continue"
        );
    }
}
