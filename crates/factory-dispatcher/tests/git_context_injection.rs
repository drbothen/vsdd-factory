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

// ---------------------------------------------------------------------------
// AC-001 / BC-1.16.001 PC1 (inject_git_context_if_qualifying positive path)
// RED GATE: todo!() in inject_git_context_if_qualifying causes test to FAIL.
//
// This test covers the POSITIVE injection path: a qualifying PostToolUse Bash
// event → payload_value is mutated with git_context key containing the four-field
// schema. Complements test_inject_git_context_if_qualifying_non_qualifying_noop
// (which only tests the noop/negative path).
//
// Uses a temporary non-git directory for factory_dir so build_git_context fails
// open (all-empty GitContext) — the test focuses on WIRING (is git_context key
// present at all?) not on actual git values (that is build_git_context's job,
// covered by test_build_git_context_fail_open_on_bad_dir).
// ---------------------------------------------------------------------------

/// Verify `inject_git_context_if_qualifying` injects `git_context` into
/// `payload_value` when the event IS qualifying (PostToolUse Bash git-commit
/// targeting .factory). Verifies the positive injection path.
///
/// # RED GATE
///
/// `inject_git_context_if_qualifying` body is `todo!()` — this test FAILS (panics)
/// until the implementer fills in T-5. A no-op implementation that never injects
/// will cause the `has("git_context")` assertion to fail after the panic is resolved.
///
/// # NON-TAUTOLOGY
///
/// The test asserts `git_context` IS present in payload_value.get("git_context").
/// A noop implementation leaves it absent → assertion fails.
/// A partial implementation that only handles the noop path → same failure.
#[test]
fn test_BC_1_16_001_inject_qualifying_mutates_payload_value() {
    // Qualifying: PostToolUse, Bash, command contains "git commit" and ".factory".
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "test-inject-02",
        "tool_input": {"command": "git -C .factory commit -m \"state: burst-01 Commit A\""},
        "tool_response": {"exit_code": 0}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();
    let mut payload_value = serde_json::to_value(&payload).unwrap();

    // Use a non-git tmpdir for factory_dir so build_git_context fail-opens
    // (all-empty). Focus: is git_context key injected at all?
    let tmp = tempfile::tempdir().unwrap();

    // RED GATE: panics with todo!() until T-5 is implemented.
    inject_git_context_if_qualifying(&payload, &mut payload_value, tmp.path());

    // Post-implementation: git_context MUST be present in payload_value.
    // (fail-open from non-git dir → all-empty values, but key must be present.)
    let git_ctx = payload_value
        .get("git_context")
        .expect("git_context must be injected into payload_value for qualifying events (AC-001 / BC-1.16.001 PC1)");

    // git_context must be a JSON object (not null, not string, not array).
    assert!(
        git_ctx.is_object(),
        "git_context must be a JSON object; got: {git_ctx:?}"
    );

    // All four fields must be present as strings (four-field completeness, AC-006 / INV5).
    let obj = git_ctx.as_object().expect("git_context is an object");
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
    ] {
        let val = obj.get(*key).unwrap_or_else(|| {
            panic!(
                "git_context.{key} must be present in injected payload (AC-006 / BC-1.16.001 INV5)"
            )
        });
        assert!(
            val.is_string(),
            "git_context.{key} must be a JSON string (not null); got: {val:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-010 / BC-1.16.001 INV4 (heuristic: no .factory indicator → non-qualifying)
// EC-007 / EC-008: command with "git commit" substring but no .factory indicator
// RED GATE: todo!() in detect_git_commit_event causes test to FAIL.
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` when the command contains
/// "git commit" but has NO `.factory` indicator (EC-007 / EC-008 / AC-010).
///
/// The heuristic requires BOTH "git commit" AND a `.factory` indicator.
/// A command like `git commit -m "..."` targeting the develop branch (no .factory)
/// must NOT qualify. A command like `echo "git commit is idempotent"` also must not.
///
/// # RED GATE
///
/// `detect_git_commit_event` body is `todo!()` — this test FAILS (panics) until
/// the implementer fills in T-1.
#[test]
fn test_BC_1_16_001_detect_no_factory_indicator_non_qualifying() {
    // Command has "git commit" but no ".factory" indicator → non-qualifying (EC-007).
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "test-detect-04",
        "tool_input": {"command": "git commit -m \"feat: add feature to develop branch\""},
        "tool_response": {"exit_code": 0}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();

    // RED GATE: panics with todo!() until T-1 is implemented.
    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false when command has 'git commit' but no '.factory' indicator (EC-007 / AC-010)"
    );
}

/// Verify `detect_git_commit_event` returns `false` for an echo command that
/// contains "git commit" as a substring but has no `.factory` indicator (EC-008).
///
/// # RED GATE
///
/// `detect_git_commit_event` body is `todo!()` — this test FAILS (panics) until
/// the implementer fills in T-1.
#[test]
fn test_BC_1_16_001_detect_echo_git_commit_substring_non_qualifying() {
    // EC-008: echo command containing "git commit" as a substring, no .factory indicator.
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Bash",
        "session_id": "test-detect-05",
        "tool_input": {"command": "echo \"git commit is idempotent\""},
        "tool_response": {"exit_code": 0}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();

    // RED GATE: panics with todo!() until T-1 is implemented.
    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for echo with 'git commit' substring and no .factory indicator (EC-008 / AC-010)"
    );
}

// ---------------------------------------------------------------------------
// AC-006, AC-011 / BC-1.16.001 INV5 / EC-003 / EC-009
// (build_git_context on real initial-commit repo — empty parent fields)
// RED GATE: todo!() in build_git_context causes test to FAIL.
// ---------------------------------------------------------------------------

/// Verify `build_git_context` on a real single-commit git repository returns:
/// - `head_subject` = the commit message subject (non-empty)
/// - `head_sha` = a valid 40-char hex SHA (non-empty)
/// - `head_parent_subject` = "" (empty string, NOT null, NOT absent)
/// - `head_parent_sha` = "" (empty string, NOT null, NOT absent)
///
/// This tests the EC-003 / EC-009 initial-commit edge case. The HEAD^ non-zero
/// exit code must NOT trigger a general fail-open (which would also zero out
/// head_subject/head_sha); only the parent fields are empty.
///
/// # RED GATE
///
/// `build_git_context` body is `todo!()` — this test FAILS (panics) until the
/// implementer fills in T-2+T-3+T-4.
///
/// # NON-TAUTOLOGY
///
/// The test asserts `head_sha` is a non-empty 40-char hex string. An all-empty
/// fail-open (from a general git error) would produce `head_sha = ""`, failing
/// the regex assertion. This distinguishes the initial-commit path from fail-open.
#[test]
fn test_BC_1_16_001_build_git_context_initial_commit_empty_parent_fields() {
    use std::process::Command;

    // Create a real git repo with a single commit (no HEAD^).
    let tmp = tempfile::tempdir().unwrap();
    let repo_dir = tmp.path();

    // Initialise a minimal git repo.
    Command::new("git")
        .args(["init", "-b", "factory-artifacts"])
        .current_dir(repo_dir)
        .status()
        .or_else(|_| {
            // Older git without -b support.
            Command::new("git")
                .arg("init")
                .current_dir(repo_dir)
                .status()
        })
        .expect("git init must succeed");

    Command::new("git")
        .args(["config", "user.email", "test@vsdd-factory"])
        .current_dir(repo_dir)
        .status()
        .expect("git config user.email must succeed");

    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(repo_dir)
        .status()
        .expect("git config user.name must succeed");

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "state: burst-01 Commit A"])
        .current_dir(repo_dir)
        .status()
        .expect("git commit must succeed");

    // Get the expected HEAD SHA and subject directly via git for comparison.
    let expected_sha = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir)
            .output()
            .expect("git rev-parse HEAD must succeed")
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();

    // RED GATE: panics with todo!() until T-2+T-3+T-4 are implemented.
    let ctx = build_git_context(repo_dir);

    // HEAD fields must be populated (initial commit exists).
    assert_eq!(
        ctx.head_subject, "state: burst-01 Commit A",
        "head_subject must match the commit message on initial commit (EC-003)"
    );
    assert_eq!(
        ctx.head_sha, expected_sha,
        "head_sha must match the full 40-char hex SHA of HEAD on initial commit (EC-003)"
    );
    // Validate 40-char hex format.
    assert_eq!(ctx.head_sha.len(), 40, "head_sha must be 40 characters");
    assert!(
        ctx.head_sha.chars().all(|c| c.is_ascii_hexdigit()),
        "head_sha must be all hex digits; got: {:?}",
        ctx.head_sha
    );

    // NON-TAUTOLOGY: head_sha must NOT be empty (distinguishes from general fail-open).
    assert!(
        !ctx.head_sha.is_empty(),
        "head_sha must not be empty on initial commit — HEAD exists (EC-003 / EC-009 / AC-011)"
    );

    // Parent fields must be empty string (not null, not absent) — EC-003 / EC-009 / AC-011.
    assert_eq!(
        ctx.head_parent_subject, "",
        "head_parent_subject must be empty string (not null) on initial commit (AC-011 / EC-003)"
    );
    assert_eq!(
        ctx.head_parent_sha, "",
        "head_parent_sha must be empty string (not null) on initial commit (AC-011 / EC-003)"
    );

    // Verify to_json() also preserves "" not null for parent fields.
    let json_val = ctx.to_json();
    let obj = json_val.as_object().expect("to_json() must return object");
    assert_eq!(
        obj.get("head_parent_subject").and_then(|v| v.as_str()),
        Some(""),
        "head_parent_subject must be JSON empty string (not null) on initial commit"
    );
    assert_eq!(
        obj.get("head_parent_sha").and_then(|v| v.as_str()),
        Some(""),
        "head_parent_sha must be JSON empty string (not null) on initial commit"
    );
}
