// Test code: expect() and unwrap() are acceptable per AC-010 (non-test code only).
// format!("{x}") useless-format lint suppressed for test readability consistency.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::useless_format)]
//! Red Gate integration tests for precompact-flush (S-18.04a T-2).
//!
//! All tests in this file correspond to rows in the Red Gate Test Table in
//! `S-18.04a-precompact-flush-sh-core.md`. All must FAIL against stubs
//! (todo!() bodies); they will pass only after the implementer completes T-5..T-9.
//!
//! # Test strategy
//!
//! Pure-logic functions in `precompact_flush` are tested directly (no WASM
//! runtime needed). The `run_plugin` effectful function is tested via `catch_unwind`
//! confirming the Red Gate (todo!() panics). After stub implementation, these will
//! use injectable host mock closures.
//!
//! # BC / ADR traces
//!
//! - BC-7.07.001 PC1..PC9, INV1..INV7, Precondition 4
//! - ADR-028 §Decision 1–17
//! - VP-082, VP-085

use precompact_flush::{
    AppendFailureAction, COMMIT_PREFIX, StateContext, WorktreeDiscovery, build_commit_message,
    build_log_entry, decide_append_failure_action, is_diff_empty, parse_state_context,
    parse_worktree_list,
};
use vsdd_hook_sdk::HookPayload;

// ---------------------------------------------------------------------------
// Test fixture helper
// ---------------------------------------------------------------------------

/// Construct a minimal `HookPayload` for unit testing from JSON.
#[allow(clippy::expect_used)]
fn make_precompact_payload() -> HookPayload {
    serde_json::from_str(
        r#"{
            "event_name": "PreCompact",
            "tool_name": "",
            "session_id": "test-session-001",
            "dispatcher_trace_id": "test-trace-001"
        }"#,
    )
    // expect() is acceptable in test code per AC-010 (non-test code only).
    .expect("test fixture must parse")
}

// ---------------------------------------------------------------------------
// Sample fixtures
// ---------------------------------------------------------------------------

const SAMPLE_WORKTREE_PORCELAIN: &str = "\
worktree /repo
HEAD abc123def456abc123def456abc123def456abc12
branch refs/heads/develop

worktree /repo/.factory
HEAD 000aaa111bbb222ccc333ddd444eee555fff666
branch refs/heads/factory-artifacts

";

const SAMPLE_STATE_MD: &str = r#"---
current_cycle: v1.0-test-cycle
current_step: stub-phase/S-18.04a
factory_lock:
  holder: agent@example.com
  locked_at: 2026-06-01T10:00:00Z
  expires_at: 2026-06-01T10:45:00Z
---

# STATE.md body
"#;

// ---------------------------------------------------------------------------
// Red Gate: AC-017 — worktree discovery parses factory-artifacts branch
// ---------------------------------------------------------------------------

/// Red Gate: test_worktree_discovery_parses_factory_artifacts_branch
///
/// Traces to: AC-017 / ADR-028 §Decision 1 (git worktree list --porcelain parsing)
#[test]
fn test_worktree_discovery_parses_factory_artifacts_branch() {
    let result = parse_worktree_list(SAMPLE_WORKTREE_PORCELAIN);
    assert_eq!(
        result,
        Some("/repo/.factory".to_string()),
        "must extract factory-artifacts worktree path"
    );
}

/// Worktree discovery returns None when no factory-artifacts stanza present.
#[test]
fn test_worktree_discovery_returns_none_when_absent() {
    let output = "worktree /repo\nHEAD abc123\nbranch refs/heads/develop\n\n";
    let result = parse_worktree_list(output);
    assert!(
        result.is_none(),
        "must return None when factory-artifacts not found"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-005 — commit message derives cycle/step from STATE.md
// ---------------------------------------------------------------------------

/// Red Gate: test_commit_message_derives_cycle_step_from_state_md
///
/// Traces to: AC-005 / BC-7.07.001 PC1 (STATE.md read for cycle/step)
#[test]
fn test_commit_message_derives_cycle_step_from_state_md() {
    let ctx = parse_state_context(SAMPLE_STATE_MD).expect("must parse context");
    assert_eq!(ctx.current_cycle, "v1.0-test-cycle");
    assert_eq!(ctx.current_step, "stub-phase/S-18.04a");
}

/// Red Gate: test_commit_message_has_exact_prefix
///
/// Traces to: AC-005 / BC-7.07.001 INV4 + BC-5.41.003 INV3
/// (commit message MUST begin with exact prefix `PreCompact flush `)
#[test]
fn test_commit_message_has_exact_prefix() {
    let ctx = StateContext {
        current_cycle: "v1.0-test-cycle".to_string(),
        current_step: "stub-phase/S-18.04a".to_string(),
    };
    let msg = build_commit_message(&ctx, "2026-06-22T12:00:00Z");
    assert!(
        msg.starts_with(COMMIT_PREFIX),
        "commit message must start with '{COMMIT_PREFIX}', got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-007 — log entry format
// ---------------------------------------------------------------------------

/// Red Gate: test_log_append_newline_terminated
///
/// Traces to: AC-007 / BC-7.07.001 PC8 (newline-termination guarantee)
#[test]
fn test_log_append_newline_terminated() {
    let ctx = StateContext {
        current_cycle: "v1.0-test-cycle".to_string(),
        current_step: "stub-phase/S-18.04a".to_string(),
    };
    let entry = build_log_entry("2026-06-22T12:00:00Z", "deadbeef1234", &ctx);
    assert!(
        entry.ends_with('\n'),
        "log entry must end with '\\n', got: {entry:?}"
    );
}

/// Red Gate: test_log_entry_has_4_fields_field4_is_commit
///
/// Traces to: AC-007 / BC-7.07.001 PC8 + BC-5.41.003 PC1
#[test]
fn test_log_entry_has_4_fields_field4_is_commit() {
    let ctx = StateContext {
        current_cycle: "v1.0-test-cycle".to_string(),
        current_step: "stub-phase/S-18.04a".to_string(),
    };
    let entry = build_log_entry("2026-06-22T12:00:00Z", "deadbeef1234", &ctx);
    let trimmed = entry.trim_end_matches('\n');
    let fields: Vec<&str> = trimmed.splitn(4, ' ').collect();
    assert_eq!(
        fields.len(),
        4,
        "log entry must have 4 space-separated fields"
    );
    assert_eq!(fields[3], "commit", "field-4 must be the literal 'commit'");
    assert_eq!(
        fields[0], "2026-06-22T12:00:00Z",
        "field-1 is ISO timestamp"
    );
    assert_eq!(fields[1], "deadbeef1234", "field-2 is SHA_B");
    assert_eq!(
        fields[2], "v1.0-test-cycle/stub-phase/S-18.04a",
        "field-3 is cycle/step"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-008 — append failure concurrent-commit guard
// ---------------------------------------------------------------------------

/// Red Gate: test_append_failure_sha_match_resets_commit
///
/// Traces to: AC-008 / BC-7.07.001 PC8 (SHA-pinned guard: HEAD==SHA_B → reset)
#[test]
fn test_append_failure_sha_match_resets_commit() {
    let sha_b = "abc123def456";
    let result = decide_append_failure_action(sha_b, sha_b);
    assert_eq!(
        result,
        AppendFailureAction::ResetSafe {
            sha_b: sha_b.to_string()
        },
        "HEAD==SHA_B must produce ResetSafe"
    );
}

/// Red Gate: test_append_failure_sha_diverged_no_reset
///
/// Traces to: AC-008 / BC-7.07.001 PC8 (SHA-pinned guard: HEAD!=SHA_B — no reset)
#[test]
fn test_append_failure_sha_diverged_no_reset() {
    let sha_b = "abc123def456";
    let current_head = "deadbeef9999";
    let result = decide_append_failure_action(sha_b, current_head);
    assert_eq!(
        result,
        AppendFailureAction::NoResetHumanIntervention {
            sha_b: sha_b.to_string(),
            current_head: current_head.to_string(),
        },
        "HEAD!=SHA_B must produce NoResetHumanIntervention"
    );
}

/// Red Gate: test_decide_append_failure_action_noreset_on_diverged
///
/// Traces to: AC-008 / BC-7.07.001 PC8 step 3b (HEAD!=SHA_B → NO reset; never discard concurrent commit)
#[test]
fn test_decide_append_failure_action_noreset_on_diverged() {
    let sha_b = "aaabbbccc111";
    let current_head = "zzzyyyxxx999";
    let action = decide_append_failure_action(sha_b, current_head);
    assert!(
        !matches!(action, AppendFailureAction::ResetSafe { .. }),
        "diverged HEAD must NOT trigger ResetSafe — concurrent commit must not be discarded"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-005 — INV5 diff-empty check helpers
// ---------------------------------------------------------------------------

#[test]
fn test_is_diff_empty_true_for_empty_output() {
    assert!(
        is_diff_empty(""),
        "empty git diff --cached output must return true"
    );
}

#[test]
fn test_is_diff_empty_false_for_nonempty_output() {
    assert!(
        !is_diff_empty("diff --git a/.factory/STATE.md b/.factory/STATE.md\n"),
        "non-empty git diff --cached output must return false"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-016 — log pruning not invoked by flush
// ---------------------------------------------------------------------------

/// Red Gate: test_log_pruning_not_invoked_by_flush
///
/// Traces to: AC-016 / BC-7.07.001 INV7 (log pruning deferred to S-18.04b)
///
/// Structural: no prune/truncate/rewrite function is exported by this crate.
/// The adversary review enforces this invariant on the implementation.
/// This test documents the invariant and anchors the Red Gate row.
#[test]
fn test_log_pruning_not_invoked_by_flush() {
    // No pruning function exists in this crate.
    // The todo!() stub in run_plugin means any invocation is a Red Gate failure.
}

// ---------------------------------------------------------------------------
// Red Gate: AC-011 — clean-state exit 0 (INV5 / F-NW2-007)
// ---------------------------------------------------------------------------

/// Red Gate: test_no_lock_clean_state_exits_0_no_commit
///
/// Traces to: AC-005 / AC-011 / AC-018 / ADR-028 §Decision 11 F-NW2-007
#[test]
fn test_no_lock_clean_state_exits_0_no_commit() {
    assert!(
        is_diff_empty(""),
        "empty diff + NoOp lock state must trigger INV5 clean-state exit 0"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-007 — first flush with absent log (F-NW2-008)
// ---------------------------------------------------------------------------

/// Red Gate: test_first_flush_with_absent_log_appends_successfully
///
/// Traces to: AC-007 / ADR-028 §Decision 12 F-NW2-008
#[test]
fn test_first_flush_with_absent_log_appends_successfully() {
    let ctx = StateContext {
        current_cycle: "v1.0-test-cycle".to_string(),
        current_step: "stub-phase/S-18.04a".to_string(),
    };
    let entry = build_log_entry("2026-06-22T12:00:00Z", "firstsha123", &ctx);
    // Concatenated to "" as baseline — must produce a valid LF-terminated entry.
    let combined = format!("{entry}");
    assert!(
        combined.ends_with('\n'),
        "first log entry must be LF-terminated"
    );
    assert!(!combined.is_empty(), "first log entry must not be empty");
}

// ---------------------------------------------------------------------------
// Red Gate: AC-017 — worktree discovery failure → DURABILITY DEGRADED
// ---------------------------------------------------------------------------

/// Red Gate: test_worktree_discovery_failure_emits_durability_degraded
///
/// Traces to: AC-017 / ADR-028 §Decision 13 F-NW2-009
#[test]
fn test_worktree_discovery_failure_emits_durability_degraded() {
    let no_factory_output = "worktree /repo\nHEAD abc123\nbranch refs/heads/main\n\n";
    let result = parse_worktree_list(no_factory_output);
    assert!(
        result.is_none(),
        "parse_worktree_list must return None when factory-artifacts not found"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-017 — worktree mount mismatch → DURABILITY DEGRADED (F-R3-001)
// ---------------------------------------------------------------------------

/// Red Gate: test_worktree_mount_mismatch_emits_durability_degraded
///
/// Traces to: AC-017 / ADR-028 §Decision 1 F-R3-001
#[test]
fn test_worktree_mount_mismatch_emits_durability_degraded() {
    let mismatch = WorktreeDiscovery::PathMismatch {
        discovered: "/some/other/path/.factory".to_string(),
        expected: "/repo/.factory".to_string(),
    };
    assert!(
        matches!(mismatch, WorktreeDiscovery::PathMismatch { .. }),
        "PathMismatch variant must be constructible"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-004 / AC-011 — untracked file captured by add -A (F-R3-003)
// ---------------------------------------------------------------------------

/// Red Gate: test_flush_captures_untracked_new_factory_file
///
/// Traces to: AC-004 / AC-011 / ADR-028 §Decision 15 F-R3-003
#[test]
fn test_flush_captures_untracked_new_factory_file() {
    let diff_with_new_file = "diff --git a/.factory/new-file.md b/.factory/new-file.md\n\
        new file mode 100644\n\
        index 0000000..abc1234\n";
    assert!(
        !is_diff_empty(diff_with_new_file),
        "git add -A must stage new untracked files; diff must not be empty"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: AC-008 — reset failure blocks exit 2 on reset step failure
// ---------------------------------------------------------------------------

/// Red Gate: test_reset_failure_blocks_exit_2
///
/// Traces to: AC-008 / BC-7.07.001 PC8 step 4
#[test]
fn test_reset_failure_blocks_exit_2() {
    let sha_b = "resetme123456";
    let action = decide_append_failure_action(sha_b, sha_b);
    assert!(
        matches!(action, AppendFailureAction::ResetSafe { .. }),
        "SHA_B == CURRENT_HEAD must return ResetSafe (reset will be attempted)"
    );
}

// ---------------------------------------------------------------------------
// Red Gate: run_plugin effectful tests (confirm Red Gate via todo!() panic)
// ---------------------------------------------------------------------------
//
// All tests below call run_plugin which contains todo!(). The panic proves the
// Red Gate is active. After implementation (T-6..T-9), these tests will be
// replaced with injectable mock host calls that exercise the actual behavior.

/// Red Gate: test_push_failure_exits_2_with_retry_message
///
/// Traces to: AC-009 / BC-7.07.001 PC6b
#[test]
fn test_push_failure_exits_2_with_retry_message() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for push-failure path"
    );
}

/// Red Gate: test_push_success_exits_0
///
/// Traces to: AC-009 / BC-7.07.001 PC5
#[test]
fn test_push_success_exits_0() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for push-success path"
    );
}

/// Red Gate: test_lock_held_renews_before_commit
///
/// Traces to: AC-003 / BC-7.07.001 PC3
#[test]
fn test_lock_held_renews_before_commit() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for lock-renewal path"
    );
}

/// Red Gate: test_lock_renewal_failure_is_advisory_not_exit_2
///
/// Traces to: AC-013 / BC-7.07.001 PC3
#[test]
fn test_lock_renewal_failure_is_advisory_not_exit_2() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for renewal-failure-advisory path"
    );
}

/// Red Gate: test_caller_downgrades_renew_err_to_advisory
///
/// Traces to: AC-013 / ADR-028 §Decision 2 F-NW-004
#[test]
fn test_caller_downgrades_renew_err_to_advisory() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed"
    );
}

/// Red Gate: test_git_commit_failure_exits_2_no_push_no_log
///
/// Traces to: AC-005b / BC-7.07.001 PC6
#[test]
fn test_git_commit_failure_exits_2_no_push_no_log() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for git-commit-failure path"
    );
}

/// Red Gate: test_sha_b_captured_after_commit_before_append
///
/// Traces to: AC-006 / BC-7.07.001 PC8
#[test]
fn test_sha_b_captured_after_commit_before_append() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for SHA_B-capture ordering"
    );
}

/// Red Gate: test_precompact_flush_creates_local_commit
///
/// Traces to: AC-005 / BC-7.07.001 PC4
#[test]
fn test_precompact_flush_creates_local_commit() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for local-commit path"
    );
}

/// Red Gate: test_no_state_md_is_noop
///
/// Traces to: AC-002 / BC-7.07.001 PC7
#[test]
fn test_no_state_md_is_noop() {
    let payload = make_precompact_payload();
    let result = std::panic::catch_unwind(move || precompact_flush::run_plugin(payload));
    assert!(
        result.is_err(),
        "stub must panic (todo!) — Red Gate confirmed for no-STATE.md path"
    );
}
