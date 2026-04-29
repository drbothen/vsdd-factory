//! AC-4: Emits `commit.made` event with fields: sha, branch, message,
//! author, timestamp.
//!
//! Exercises `commit_hook_logic` with a mock emit callback and verifies the
//! CommitEventFields populated by `build_commit_fields`.
//!
//! The mock `emit` captures the fields so assertions can inspect all five.

use capture_commit_activity::{CommitEventFields, build_commit_fields, commit_hook_logic};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------------

fn git_commit_payload_with_branch(branch: &str, sha: &str) -> HookPayload {
    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-emit".to_string(),
        dispatcher_trace_id: "trace-emit".to_string(),
        tool_input: serde_json::json!({"command": "git commit -m 'test: add tests'"}),
        tool_response: Some(serde_json::json!({
            "interrupted": false,
            "stdout": format!("[{branch} {sha}] test: add tests\n 1 file changed")
        })),
        plugin_config: serde_json::Value::Null,
    }
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_emit_called_with_commit_made_event_type
// ---------------------------------------------------------------------------

/// AC-4: when a git commit is detected and git log returns a SHA,
/// `commit_hook_logic` must call `emit` exactly once.
#[test]
fn test_BC_4_03_001_emit_called_with_commit_made_event_type() {
    let payload = git_commit_payload_with_branch("main", "abc1234");
    let emit_count = std::cell::Cell::new(0usize);
    let _ = commit_hook_logic(
        payload,
        || Ok((0, "abc1234def5678901234567890123456789012345\n".to_string())),
        |_fields| {
            emit_count.set(emit_count.get() + 1);
        },
    );
    assert_eq!(
        emit_count.get(),
        1,
        "emit must be called exactly once per commit"
    );
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_emit_sha_field_matches_git_log_output
// ---------------------------------------------------------------------------

/// AC-4: the `sha` field in CommitEventFields must match the value returned
/// by `git log -1 --format=%H`.
#[test]
fn test_BC_4_03_001_emit_sha_field_matches_git_log_output() {
    let expected_sha = "abc1234def5678901234567890123456789012345";
    let payload = git_commit_payload_with_branch("main", "abc1234");
    let captured: std::cell::RefCell<Option<CommitEventFields>> = std::cell::RefCell::new(None);
    let _ = commit_hook_logic(
        payload,
        || Ok((0, format!("{expected_sha}\n"))),
        |fields| {
            *captured.borrow_mut() = Some(fields.clone());
        },
    );
    let fields = captured.into_inner().expect("emit must be called");
    assert_eq!(
        fields.sha.trim(),
        expected_sha,
        "sha field must equal git log stdout"
    );
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_emit_all_five_fields_non_empty
// ---------------------------------------------------------------------------

/// AC-4: all five required fields (sha, branch, message, author, timestamp)
/// must be non-empty in the emitted event.
#[test]
fn test_BC_4_03_001_emit_all_five_fields_non_empty() {
    let payload = git_commit_payload_with_branch("feat/S-3.01", "deadbeef");
    let captured: std::cell::RefCell<Option<CommitEventFields>> = std::cell::RefCell::new(None);
    let _ = commit_hook_logic(
        payload,
        || Ok((0, "deadbeef12345678901234567890123456789012\n".to_string())),
        |fields| {
            *captured.borrow_mut() = Some(fields.clone());
        },
    );
    let fields = captured.into_inner().expect("emit must be called");
    assert!(!fields.sha.is_empty(), "sha must not be empty");
    assert!(!fields.branch.is_empty(), "branch must not be empty");
    assert!(!fields.message.is_empty(), "message must not be empty");
    assert!(!fields.author.is_empty(), "author must not be empty");
    assert!(!fields.timestamp.is_empty(), "timestamp must not be empty");
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_emit_not_called_for_non_commit_command
// ---------------------------------------------------------------------------

/// AC-5 (no-op for non-commit): emit must NOT be called when the command is
/// not a git commit.
#[test]
fn test_BC_4_03_001_emit_not_called_for_non_commit_command() {
    let payload = HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-emit-noop".to_string(),
        dispatcher_trace_id: "trace-emit-noop".to_string(),
        tool_input: serde_json::json!({"command": "cargo test"}),
        tool_response: Some(serde_json::json!({"interrupted": false})),
        plugin_config: serde_json::Value::Null,
    };
    let emit_called = std::cell::Cell::new(false);
    let _ = commit_hook_logic(
        payload,
        || panic!("git log must not be called for non-git commands"),
        |_| {
            emit_called.set(true);
        },
    );
    assert!(
        !emit_called.get(),
        "emit must not be called for non-commit commands"
    );
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_emit_result_is_continue_on_happy_path
// ---------------------------------------------------------------------------

/// AC-4 + AC-5: the plugin must return Continue after successfully emitting
/// the commit.made event.
#[test]
fn test_BC_4_03_001_emit_result_is_continue_on_happy_path() {
    let payload = git_commit_payload_with_branch("main", "abc1234");
    let result = commit_hook_logic(
        payload,
        || Ok((0, "abc1234def5678901234567890123456789012345\n".to_string())),
        |_| {},
    );
    assert_eq!(
        result,
        HookResult::Continue,
        "plugin must return Continue after emitting commit.made"
    );
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_build_commit_fields_sha_field
// ---------------------------------------------------------------------------

/// build_commit_fields: the sha field must equal the value passed in.
#[test]
fn test_BC_4_03_001_build_commit_fields_sha_field() {
    let payload = git_commit_payload_with_branch("main", "abc1234");
    let fields = build_commit_fields("abc1234def5678901234567890123456789012345", &payload);
    assert_eq!(
        fields.sha.trim(),
        "abc1234def5678901234567890123456789012345"
    );
}

// ---------------------------------------------------------------------------
// test_TV_003_canonical_commit_made_schema
// ---------------------------------------------------------------------------

/// TV-003 canonical test vector: commit.made event schema check.
///
/// Exercises the full happy path with a known sha, branch, and message.
/// All five fields must be present and non-empty.
#[test]
fn test_TV_003_canonical_commit_made_schema() {
    let payload = HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "sess-tv-003".to_string(),
        dispatcher_trace_id: "trace-tv-003".to_string(),
        tool_input: serde_json::json!({"command": "git commit -m 'feat: implement WASM port'"}),
        tool_response: Some(serde_json::json!({
            "interrupted": false,
            "stdout": "[main a1b2c3d] feat: implement WASM port\n 3 files changed"
        })),
        plugin_config: serde_json::Value::Null,
    };

    let sha = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";
    let captured: std::cell::RefCell<Option<CommitEventFields>> = std::cell::RefCell::new(None);
    let result = commit_hook_logic(
        payload,
        || Ok((0, format!("{sha}\n"))),
        |fields| {
            *captured.borrow_mut() = Some(fields.clone());
        },
    );

    let fields = captured
        .into_inner()
        .expect("emit must be called for TV-003");
    assert_eq!(fields.sha.trim(), sha, "sha field");
    assert!(!fields.branch.is_empty(), "branch field must be non-empty");
    assert!(
        !fields.message.is_empty(),
        "message field must be non-empty"
    );
    assert!(!fields.author.is_empty(), "author field must be non-empty");
    assert!(
        !fields.timestamp.is_empty(),
        "timestamp field must be non-empty"
    );
    assert_eq!(result, HookResult::Continue);
}
