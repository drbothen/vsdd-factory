// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! git_context_injection.rs — Red Gate integration tests for S-18.04b-prereq
//!
//! Tests for `git_context` schema completeness and HOST_ABI_VERSION invariance
//! per the story's Red Gate Test Table (AC-005, AC-006 / BC-1.16.001 INV5 / ADR-029 §Decision 4).
//!
//! # RED GATE status
//!
//! All tests in this file are RED GATE tests. They FAIL against the stub because
//! `detect_git_commit_event`, `build_git_context`, and `inject_git_context_if_qualifying`
//! in `crates/factory-dispatcher/src/invoke.rs` have `todo!()` bodies. Tests flip
//! GREEN after the implementer fills the `todo!()` bodies (S-18.04b-prereq T-1 through T-7).
//!
//! # Test coverage
//!
//! - `test_host_abi_version_unchanged` (AC-005 / BC-1.16.001 PC5 / ADR-029 §Decision 4):
//!   Verifies HOST_ABI_VERSION = 1 (green-by-design; passes against stubs).
//!
//! - `test_git_context_schema_four_fields_present` (AC-006 / BC-1.16.001 INV5):
//!   Verifies `GitContext::to_json()` produces a JSON object with exactly four
//!   string fields (head_subject, head_sha, head_parent_subject, head_parent_sha).
//!   Also verifies null is never emitted — empty string is the sentinel.
//!
//! - `test_detect_git_commit_event_qualifying` (AC-001, AC-010):
//!   Verifies detection returns true for qualifying PostToolUse Bash git-commit events.
//!
//! - `test_detect_git_commit_event_non_qualifying_push` (AC-003, AC-012):
//!   Verifies detection returns false for git-push (non-qualifying subcommand).
//!
//! - `test_detect_git_commit_event_non_qualifying_edit_tool` (AC-004, AC-008):
//!   Verifies detection returns false and does not inspect command for Edit events.
//!
//! - `test_build_git_context_fail_open_on_bad_dir` (AC-002, AC-009):
//!   Verifies `build_git_context` returns all-empty GitContext on git error.
//!
//! - `test_inject_git_context_if_qualifying_non_qualifying_noop` (AC-003):
//!   Verifies `inject_git_context_if_qualifying` does not mutate payload_value
//!   when the event is non-qualifying.

use factory_dispatcher::HOST_ABI_VERSION;
use factory_dispatcher::invoke::{
    GitContext, build_git_context, detect_git_commit_event, inject_git_context_if_qualifying,
};
use factory_dispatcher::payload::HookPayload;

// ---------------------------------------------------------------------------
// AC-005 / BC-1.16.001 PC5 / ADR-029 §Decision 4
// GREEN-BY-DESIGN: HOST_ABI_VERSION is a compile-time constant; this test
// passes against the stub (the constant is already 1 and must not change).
// ---------------------------------------------------------------------------

/// Verify `HOST_ABI_VERSION` = 1 — no new host function introduced by
/// S-18.04b-prereq; ABI bump is forbidden (ADR-029 §Decision 4 / AC-005).
///
/// # GREEN-BY-DESIGN
///
/// Pure constant comparison; zero branching, no I/O, no helpers, 2 lines.
/// BC-5.38.002 criteria satisfied. This test is GREEN even against the stub.
#[test]
fn test_host_abi_version_unchanged() {
    assert_eq!(
        HOST_ABI_VERSION, 1,
        "HOST_ABI_VERSION must remain 1 — S-18.04b-prereq does not introduce \
         a new host function (ADR-029 §Decision 4 / AC-005 / BC-1.16.001 PC5)"
    );
}

// ---------------------------------------------------------------------------
// AC-006 / BC-1.16.001 INV5 (four-field completeness)
// RED GATE: GitContext::to_json() stub test. Will pass once to_json() is
// implemented (GREEN-BY-DESIGN for GitContext::empty() + to_json()).
// ---------------------------------------------------------------------------

/// Verify `GitContext::to_json()` produces an object with exactly four string
/// fields matching the ADR-029 §Decision 2 schema.
///
/// # RED GATE
///
/// `GitContext::to_json()` is GREEN-BY-DESIGN (pure object construction, ≤8 lines),
/// but the schema-completeness assertion below requires all four fields to be
/// present as strings. This test gates the full schema contract.
///
/// The test for `GitContext::empty()` (which returns all `""`) verifies that the
/// fail-open sentinel satisfies the four-field requirement (AC-006 / AC-011).
#[test]
fn test_git_context_schema_four_fields_present() {
    let ctx = GitContext {
        head_subject: "state: burst-02 Commit B".to_string(),
        head_sha: "a".repeat(40),
        head_parent_subject: "state: burst-01 Commit A".to_string(),
        head_parent_sha: "b".repeat(40),
    };

    let json_val = ctx.to_json();
    let obj = json_val
        .as_object()
        .expect("to_json() must return a JSON object");

    // All four fields must be present.
    assert!(
        obj.contains_key("head_subject"),
        "git_context must contain head_subject"
    );
    assert!(
        obj.contains_key("head_sha"),
        "git_context must contain head_sha"
    );
    assert!(
        obj.contains_key("head_parent_subject"),
        "git_context must contain head_parent_subject"
    );
    assert!(
        obj.contains_key("head_parent_sha"),
        "git_context must contain head_parent_sha"
    );

    // All fields must be strings (not null, not absent).
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
    ] {
        let val = obj.get(*key).expect("field must be present");
        assert!(
            val.is_string(),
            "git_context.{key} must be a JSON string (not null); got: {val:?}"
        );
        assert!(
            val.as_str() != Some("null"),
            "git_context.{key} must not be the string \"null\""
        );
    }

    // Exact field count: exactly 4 fields (no extra, no missing).
    assert_eq!(
        obj.len(),
        4,
        "git_context must have exactly 4 fields; got {}: {:?}",
        obj.len(),
        obj.keys().collect::<Vec<_>>()
    );
}

/// Verify `GitContext::empty()` satisfies the four-field schema with all-empty strings
/// (fail-open sentinel; AC-006 / AC-011 — `""` not null).
///
/// # GREEN-BY-DESIGN
///
/// `GitContext::empty()` is GREEN-BY-DESIGN (pure struct construction, ≤7 lines).
/// `to_json()` is also GREEN-BY-DESIGN. This test passes against the stub because
/// both functions are real implementations (not todo!()). Self-check (BC-5.38.005
/// invariant 1): "If I include this real implementation, will the test pass trivially
/// without implementer work?" Yes — the test is for the fail-open sentinel path,
/// which is correct-by-construction. Classified GREEN-BY-DESIGN.
#[test]
fn test_git_context_empty_satisfies_four_field_schema() {
    let ctx = GitContext::empty();

    assert_eq!(ctx.head_subject, "");
    assert_eq!(ctx.head_sha, "");
    assert_eq!(ctx.head_parent_subject, "");
    assert_eq!(ctx.head_parent_sha, "");

    let json_val = ctx.to_json();
    let obj = json_val
        .as_object()
        .expect("to_json() must return a JSON object");

    // All four fields present as empty string (not null).
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
    ] {
        let val = obj.get(*key).expect("field must be present");
        assert_eq!(
            val.as_str(),
            Some(""),
            "GitContext::empty() must produce empty-string fields in JSON (not null)"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-001, AC-010 / BC-1.16.001 PC1 (detection of qualifying events)
// RED GATE: todo!() in detect_git_commit_event causes test to FAIL (panic).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `true` for a qualifying PostToolUse
/// Bash event with "git commit" and ".factory" in the command (AC-001 / AC-010).
///
/// # RED GATE
///
/// `detect_git_commit_event` body is `todo!()` — this test FAILS (panics) until
/// the implementer fills in T-1. Do NOT add `#[should_panic]` — the test must
/// remain RED against stubs and GREEN after implementation.
#[test]
fn test_detect_git_commit_event_qualifying() {
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "test-detect-01",
        "tool_input": {"command": "git -C .factory commit -m \"state: burst-01\""},
        "tool_response": {"exit_code": 0}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();

    // RED GATE: panics with todo!() until T-1 is implemented.
    let result = detect_git_commit_event(&payload);
    assert!(
        result,
        "detect_git_commit_event must return true for qualifying PostToolUse Bash git-commit event"
    );
}

// ---------------------------------------------------------------------------
// AC-003, AC-012 / BC-1.16.001 PC3 (no injection on git push)
// RED GATE: todo!() in detect_git_commit_event causes test to FAIL (panic).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` for git push (non-qualifying;
/// AC-003 / AC-012 / BC-1.16.001 EC-004).
///
/// # RED GATE
///
/// `detect_git_commit_event` body is `todo!()` — this test FAILS (panics) until
/// the implementer fills in T-1.
#[test]
fn test_detect_git_commit_event_non_qualifying_push() {
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "test-detect-02",
        "tool_input": {"command": "git -C .factory push origin factory-artifacts"},
        "tool_response": {"exit_code": 0}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();

    // RED GATE: panics with todo!() until T-1 is implemented.
    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for git-push (not git-commit)"
    );
}

// ---------------------------------------------------------------------------
// AC-004, AC-008 / BC-1.16.001 PC4 (no injection on non-Bash tool)
// RED GATE: todo!() in detect_git_commit_event causes test to FAIL (panic).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` for a PostToolUse Edit event
/// (tool_name = "Edit" — must not inspect command at all; AC-004 / AC-008).
///
/// # RED GATE
///
/// `detect_git_commit_event` body is `todo!()` — this test FAILS (panics) until
/// the implementer fills in T-1.
#[test]
fn test_detect_git_commit_event_non_qualifying_edit_tool() {
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "test-detect-03",
        "tool_input": {"file_path": ".factory/STATE.md", "old_string": "a", "new_string": "b"},
        "tool_response": {"success": true}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();

    // RED GATE: panics with todo!() until T-1 is implemented.
    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for non-Bash tool (Edit)"
    );
}

// ---------------------------------------------------------------------------
// AC-002, AC-009 / BC-1.16.001 PC2 (fail-open on git error)
// RED GATE: todo!() in build_git_context causes test to FAIL (panic).
// ---------------------------------------------------------------------------

/// Verify `build_git_context` returns all-empty `GitContext` when the factory_dir
/// is not a git repository (fail-open; AC-002 / AC-009 / BC-1.16.001 INV3).
///
/// Uses a temporary directory that is NOT a git repo to simulate the error path.
///
/// # RED GATE
///
/// `build_git_context` body is `todo!()` — this test FAILS (panics) until the
/// implementer fills in T-2+T-3+T-4.
#[test]
fn test_build_git_context_fail_open_on_bad_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let non_git_dir = tmp.path();

    // RED GATE: panics with todo!() until T-2 is implemented.
    let ctx = build_git_context(non_git_dir);

    // Post-implementation: must be all-empty GitContext.
    assert_eq!(
        ctx,
        GitContext::empty(),
        "build_git_context must fail-open (return all-empty GitContext) when factory_dir is not a git repo"
    );
}

// ---------------------------------------------------------------------------
// AC-003 / BC-1.16.001 PC3 (no mutation on non-qualifying events)
// RED GATE: todo!() in inject_git_context_if_qualifying causes test to FAIL.
// ---------------------------------------------------------------------------

/// Verify `inject_git_context_if_qualifying` does not mutate `payload_value` when
/// the event is non-qualifying (Edit tool; AC-003 / AC-004 / BC-1.16.001 PC3/PC4).
///
/// # RED GATE
///
/// `inject_git_context_if_qualifying` body is `todo!()` — this test FAILS (panics)
/// until the implementer fills in T-5.
#[test]
fn test_inject_git_context_if_qualifying_non_qualifying_noop() {
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "test-inject-01",
        "tool_input": {"file_path": ".factory/STATE.md"},
        "tool_response": {"success": true}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();
    let mut payload_value = serde_json::to_value(&payload).unwrap();
    let payload_before = payload_value.clone();

    let tmp = tempfile::tempdir().unwrap();

    // RED GATE: panics with todo!() until T-5 is implemented.
    inject_git_context_if_qualifying(&payload, &mut payload_value, tmp.path());

    // Post-implementation: payload_value must be unchanged for non-qualifying events.
    assert_eq!(
        payload_value, payload_before,
        "inject_git_context_if_qualifying must not mutate payload_value for non-qualifying events"
    );
    assert!(
        payload_value.get("git_context").is_none(),
        "git_context must not be injected for non-qualifying Edit events (AC-003)"
    );
}
