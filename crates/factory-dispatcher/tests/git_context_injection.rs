// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! git_context_injection.rs — Red Gate integration tests for S-18.04b-prereq
//!
//! Tests for `git_context` schema completeness and HOST_ABI_VERSION invariance
//! per the story's Red Gate Test Table (AC-005, AC-006 / BC-1.16.001 INV5 / ADR-029 §Decision 4).
//!
//! # Test status
//!
//! All tests in this file are GREEN (S-18.04b-prereq fully delivered).
//! `detect_git_commit_event`, `build_git_context`, and `inject_git_context_if_qualifying`
//! in `crates/factory-dispatcher/src/invoke.rs` are fully implemented per ADR-029 §Decision 1–3.
//! Tests T-1 through T-7 pass against the real implementation.
//!
//! # Test coverage
//!
//! - `test_host_abi_version_unchanged` (AC-005 / BC-1.16.001 PC5 / ADR-029 §Decision 4):
//!   Verifies HOST_ABI_VERSION = 1 (green-by-design; compile-time constant).
//!
//! - `test_git_context_schema_seven_fields_present` (AC-006 / BC-1.16.001 INV5 / ADR-032-AC021-prereq):
//!   Verifies `GitContext::to_json()` produces a JSON object with exactly seven
//!   string fields (the original four plus head_state_timestamp,
//!   head_parent_state_timestamp, state_md_in_commit — ADR-032-AC021-prereq).
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
// GREEN-BY-DESIGN: HOST_ABI_VERSION is a compile-time constant (= 1).
// S-18.04b-prereq does not introduce a new host function; ABI bump forbidden.
// ---------------------------------------------------------------------------

/// Verify `HOST_ABI_VERSION` = 1 — no new host function introduced by
/// S-18.04b-prereq; ABI bump is forbidden (ADR-029 §Decision 4 / AC-005).
///
/// # GREEN-BY-DESIGN
///
/// Pure constant comparison; zero branching, no I/O, no helpers, 2 lines.
/// BC-5.38.002 criteria satisfied.
#[test]
fn test_host_abi_version_unchanged() {
    assert_eq!(
        HOST_ABI_VERSION, 1,
        "HOST_ABI_VERSION must remain 1 — S-18.04b-prereq does not introduce \
         a new host function (ADR-029 §Decision 4 / AC-005 / BC-1.16.001 PC5)"
    );
}

// ---------------------------------------------------------------------------
// AC-006 / BC-1.16.001 INV5 / ADR-032-AC021-prereq (seven-field completeness)
// GitContext::to_json() schema contract (GREEN-BY-DESIGN — pure object
// construction; all seven string fields present and non-null).
// ---------------------------------------------------------------------------

/// Verify `GitContext::to_json()` produces an object with exactly seven string
/// fields matching the ADR-029 §Decision 2 + ADR-032-AC021-prereq schema.
///
/// `GitContext::to_json()` is GREEN-BY-DESIGN (pure object construction).
/// The schema-completeness assertion verifies all seven fields are present as strings.
///
/// ADR-032-AC021-prereq added three fields to `GitContext`:
/// `head_state_timestamp`, `head_parent_state_timestamp`, and `state_md_in_commit`.
#[test]
fn test_git_context_schema_seven_fields_present() {
    let ctx = GitContext {
        head_subject: "state: burst-02 Commit B".to_string(),
        head_sha: "a".repeat(40),
        head_parent_subject: "state: burst-01 Commit A".to_string(),
        head_parent_sha: "b".repeat(40),
        head_state_timestamp: "2026-07-20T10:00:00Z".to_string(),
        head_parent_state_timestamp: "2026-07-20T09:00:00Z".to_string(),
        state_md_in_commit: "true".to_string(),
    };

    let json_val = ctx.to_json();
    let obj = json_val
        .as_object()
        .expect("to_json() must return a JSON object");

    // All seven fields must be present.
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
        "head_state_timestamp",
        "head_parent_state_timestamp",
        "state_md_in_commit",
    ] {
        assert!(obj.contains_key(*key), "git_context must contain {key}");
    }

    // All fields must be strings (not null, not absent).
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
        "head_state_timestamp",
        "head_parent_state_timestamp",
        "state_md_in_commit",
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

    // Exact field count: exactly 7 fields (ADR-032-AC021-prereq extended from 4 to 7).
    assert_eq!(
        obj.len(),
        7,
        "git_context must have exactly 7 fields; got {}: {:?}",
        obj.len(),
        obj.keys().collect::<Vec<_>>()
    );
}

/// Verify `GitContext::empty()` satisfies the seven-field schema with all-empty strings
/// (fail-open sentinel; AC-006 / AC-011 — `""` not null).
///
/// ADR-032-AC021-prereq added `head_state_timestamp`, `head_parent_state_timestamp`,
/// and `state_md_in_commit` — all three MUST also be `""` in the empty sentinel.
///
/// # GREEN-BY-DESIGN
///
/// `GitContext::empty()` is GREEN-BY-DESIGN (pure struct construction).
/// `to_json()` is also GREEN-BY-DESIGN. Both functions are pure struct construction
/// with no I/O. Self-check (BC-5.38.005 invariant 1): "Is the test for the fail-open
/// sentinel path, which is correct-by-construction?" Yes — classified GREEN-BY-DESIGN.
#[test]
fn test_git_context_empty_satisfies_seven_field_schema() {
    let ctx = GitContext::empty();

    assert_eq!(ctx.head_subject, "");
    assert_eq!(ctx.head_sha, "");
    assert_eq!(ctx.head_parent_subject, "");
    assert_eq!(ctx.head_parent_sha, "");
    assert_eq!(ctx.head_state_timestamp, "");
    assert_eq!(ctx.head_parent_state_timestamp, "");
    assert_eq!(ctx.state_md_in_commit, "");

    let json_val = ctx.to_json();
    let obj = json_val
        .as_object()
        .expect("to_json() must return a JSON object");

    // All seven fields present as empty string (not null).
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
        "head_state_timestamp",
        "head_parent_state_timestamp",
        "state_md_in_commit",
    ] {
        let val = obj.get(*key).expect("field must be present");
        assert_eq!(
            val.as_str(),
            Some(""),
            "GitContext::empty() must produce empty-string fields in JSON (not null); \
             field '{key}' was not empty"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-001, AC-010 / BC-1.16.001 PC1 (detection of qualifying events)
// detect_git_commit_event: positive path — PostToolUse Bash with "git commit"
// and ".factory" indicator returns true (heuristic per AC-010).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `true` for a qualifying PostToolUse
/// Bash event with "git commit" and ".factory" in the command (AC-001 / AC-010).
///
/// `detect_git_commit_event` checks event_name == "PostToolUse", tool_name == "Bash",
/// command contains "git" + " commit" + ".factory" (heuristic per AC-010). Returns
/// true for qualifying events; test verifies the positive detection path (T-1).
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

    let result = detect_git_commit_event(&payload);
    assert!(
        result,
        "detect_git_commit_event must return true for qualifying PostToolUse Bash git-commit event"
    );
}

// ---------------------------------------------------------------------------
// AC-003, AC-012 / BC-1.16.001 PC3 (no injection on git push)
// detect_git_commit_event: non-qualifying push — ".factory" present but no
// " commit" subcommand token → returns false (AC-012).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` for git push (non-qualifying;
/// AC-003 / AC-012 / BC-1.16.001 EC-004).
///
/// The command contains ".factory" but lacks " commit" as a git subcommand token;
/// detection correctly rejects it (T-1 non-qualifying push path).
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

    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for git-push (not git-commit)"
    );
}

// ---------------------------------------------------------------------------
// AC-004, AC-008 / BC-1.16.001 PC4 (no injection on non-Bash tool)
// detect_git_commit_event: non-Bash short-circuit — tool_name != "Bash"
// returns false without inspecting tool_input.command (AC-004 / AC-008).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` for a PostToolUse Edit event
/// (tool_name = "Edit" — must not inspect command at all; AC-004 / AC-008).
///
/// The implementation returns false immediately on non-Bash tool_name without
/// inspecting tool_input.command. Test verifies non-Bash short-circuit path (T-1).
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

    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for non-Bash tool (Edit)"
    );
}

// ---------------------------------------------------------------------------
// AC-002, AC-009 / BC-1.16.001 PC2 (fail-open on git error)
// build_git_context: non-git directory → emits tracing::warn! and returns
// GitContext::empty() (fail-open; BC-1.16.001 PC2 / INV3).
// ---------------------------------------------------------------------------

/// Verify `build_git_context` returns all-empty `GitContext` when the factory_dir
/// is not a git repository (fail-open; AC-002 / AC-009 / BC-1.16.001 INV3).
///
/// Uses a temporary directory that is NOT a git repo to simulate the error path.
/// The implementation emits tracing::warn! on the git failure and returns
/// GitContext::empty() (all seven fields ""). Test verifies the fail-open path (T-2).
#[test]
fn test_build_git_context_fail_open_on_bad_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let non_git_dir = tmp.path();

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
// inject_git_context_if_qualifying: non-qualifying event (Edit) → payload_value
// is not mutated; "git_context" key is absent (AC-003 / AC-004).
// ---------------------------------------------------------------------------

/// Verify `inject_git_context_if_qualifying` does not mutate `payload_value` when
/// the event is non-qualifying (Edit tool; AC-003 / AC-004 / BC-1.16.001 PC3/PC4).
///
/// The implementation returns immediately without mutation when
/// detect_git_commit_event returns false. Test verifies the no-op path (T-5).
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

    inject_git_context_if_qualifying(&payload, &mut payload_value, tmp.path());

    // Payload_value must be unchanged for non-qualifying events.
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
// inject_git_context_if_qualifying: qualifying PostToolUse Bash event →
// payload_value is mutated with "git_context" key (seven-field JSON object).
// Complements the no-op test above (which covers non-qualifying events).
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
/// The implementation detects the qualifying event, builds git_context via
/// build_git_context (fail-open from non-git tmpdir → all-empty fields), and
/// inserts "git_context" as a JSON object into payload_value (T-5 positive path).
///
/// # NON-TAUTOLOGY
///
/// The test asserts `git_context` IS present in payload_value.get("git_context").
/// A noop implementation would leave it absent → assertion fails.
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

    inject_git_context_if_qualifying(&payload, &mut payload_value, tmp.path());

    // git_context MUST be present in payload_value.
    // (fail-open from non-git dir → all-empty values, but key must be present.)
    let git_ctx = payload_value
        .get("git_context")
        .expect("git_context must be injected into payload_value for qualifying events (AC-001 / BC-1.16.001 PC1)");

    // git_context must be a JSON object (not null, not string, not array).
    assert!(
        git_ctx.is_object(),
        "git_context must be a JSON object; got: {git_ctx:?}"
    );

    // All seven fields must be present as strings (seven-field completeness, AC-006 / INV5).
    let obj = git_ctx.as_object().expect("git_context is an object");
    for key in &[
        "head_subject",
        "head_sha",
        "head_parent_subject",
        "head_parent_sha",
        "head_state_timestamp",
        "head_parent_state_timestamp",
        "state_md_in_commit",
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
// → detect_git_commit_event returns false (heuristic requires both signals).
// ---------------------------------------------------------------------------

/// Verify `detect_git_commit_event` returns `false` when the command contains
/// "git commit" but has NO `.factory` indicator (EC-007 / EC-008 / AC-010).
///
/// The heuristic requires BOTH "git commit" AND a `.factory` indicator.
/// A command like `git commit -m "..."` targeting the develop branch (no .factory)
/// does not qualify. The implementation correctly rejects it (T-1 heuristic path).
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

    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false when command has 'git commit' but no '.factory' indicator (EC-007 / AC-010)"
    );
}

/// Verify `detect_git_commit_event` returns `false` for an echo command that
/// contains "git commit" as a substring but has no `.factory` indicator (EC-008).
///
/// The implementation requires both " commit" as a git subcommand token AND
/// ".factory" as a worktree indicator. An echo with neither is correctly rejected
/// (T-1 heuristic non-qualifying path).
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

    let result = detect_git_commit_event(&payload);
    assert!(
        !result,
        "detect_git_commit_event must return false for echo with 'git commit' substring and no .factory indicator (EC-008 / AC-010)"
    );
}

// ---------------------------------------------------------------------------
// AC-006, AC-011 / BC-1.16.001 INV5 / EC-003 / EC-009
// build_git_context on real single-commit repo: HEAD fields populated,
// HEAD^ parent fields are empty string (not null; initial commit has no parent).
// ---------------------------------------------------------------------------

/// Verify `build_git_context` on a real single-commit git repository returns:
/// - `head_subject` = the commit message subject (non-empty)
/// - `head_sha` = a valid 40-char hex SHA (non-empty)
/// - `head_parent_subject` = "" (empty string, NOT null, NOT absent)
/// - `head_parent_sha` = "" (empty string, NOT null, NOT absent)
///
/// This tests the EC-003 / EC-009 initial-commit edge case. The implementation
/// treats HEAD^ non-zero exit as a non-error (initial commit has no parent) —
/// only the parent fields are empty; HEAD fields are still populated (T-2/T-3/T-4).
///
/// # NON-TAUTOLOGY
///
/// The test asserts `head_sha` is a non-empty 40-char hex string. An all-empty
/// fail-open (from a general git error) would produce `head_sha = ""`, failing
/// the assertion. This distinguishes the initial-commit path from the fail-open path.
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

// ---------------------------------------------------------------------------
// SEC-002 / CWE-345 (dispatcher-authoritative git_context eviction)
// inject_git_context_if_qualifying: non-qualifying event with a caller-supplied
// "git_context" key → key is REMOVED from payload_value before return.
// git_context is dispatcher-authoritative (BC-1.16.001 INV1): a caller cannot
// spoof it by pre-populating the JSON payload with a forged key.
// ---------------------------------------------------------------------------

/// Verify `inject_git_context_if_qualifying` evicts any pre-existing "git_context"
/// key from `payload_value` when the event is non-qualifying (SEC-002 / CWE-345 /
/// BC-1.16.001 INV1).
///
/// A non-qualifying event (Edit tool) whose payload already contains a caller-supplied
/// `git_context` key must have that key removed before the payload reaches plugins.
/// The dispatcher is the sole author of git_context — caller-supplied values are not
/// trusted and must never pass through.
///
/// # NON-TAUTOLOGY
///
/// The test pre-populates payload_value with a forged "git_context" key before calling
/// `inject_git_context_if_qualifying`. After the call, it asserts the key is ABSENT.
/// An implementation that only returns early without evicting would leave the key
/// present, failing this assertion.
#[test]
fn test_SEC_002_non_qualifying_event_evicts_caller_supplied_git_context() {
    // Non-qualifying payload (Edit tool, not Bash).
    let payload_bytes = br#"{
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "test-sec-002",
        "tool_input": {"file_path": ".factory/STATE.md"},
        "tool_response": {"success": true}
    }"#;
    let payload = HookPayload::from_bytes(payload_bytes).unwrap();
    let mut payload_value = serde_json::to_value(&payload).unwrap();

    // Pre-populate a forged caller-supplied "git_context" key to simulate an
    // attacker-controlled payload that attempts to spoof dispatcher-authoritative context.
    payload_value
        .as_object_mut()
        .expect("payload_value must be an object")
        .insert(
            "git_context".to_string(),
            serde_json::json!({
                "head_subject": "forged subject",
                "head_sha": "forged_sha",
                "head_parent_subject": "forged parent",
                "head_parent_sha": "forged_parent_sha"
            }),
        );

    // Confirm the key is present before the call (test setup sanity check).
    assert!(
        payload_value.get("git_context").is_some(),
        "test setup: git_context must be present in payload_value before injection"
    );

    let tmp = tempfile::tempdir().unwrap();

    inject_git_context_if_qualifying(&payload, &mut payload_value, tmp.path());

    // After inject_git_context_if_qualifying on a non-qualifying event, the
    // caller-supplied "git_context" key must be ABSENT (SEC-002 / CWE-345 / BC-1.16.001 INV1).
    assert!(
        payload_value.get("git_context").is_none(),
        "inject_git_context_if_qualifying must evict caller-supplied git_context on \
         non-qualifying events (SEC-002 / CWE-345 / BC-1.16.001 INV1 — dispatcher-authoritative)"
    );
}
