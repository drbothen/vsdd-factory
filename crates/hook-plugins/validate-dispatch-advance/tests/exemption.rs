// crates/hook-plugins/validate-dispatch-advance/tests/exemption.rs
//
// Red Gate failing tests for S-18.04b — PreCompact flush exemption logic
// in `validate-dispatch-advance`.
//
// # BC trace
// BC-5.41.003 PC1 cases (a)/(b)/(c); INV1; INV2 (symmetry); INV3; AC-001..AC-006; AC-008.
// BC-1.16.001: git_context schema (4 fields, all string, fail-open on absent).
// ADR-029 §Decision 1 (trigger: Bash), §Decision 3 (exec-free WASM), §Decision 5.
//
// # Symmetry requirement (AC-006 / BC-5.41.003 INV2)
// This file MIRRORS the test structure and inputs of
// `crates/hook-plugins/validate-burst-log/tests/exemption.rs`.
// Both gates must implement the 3-case exemption identically. The assertions
// here are the authoritative AC-006 / INV2 symmetry check: if validate-burst-log
// passes a test but validate-dispatch-advance fails it (or vice versa), INV2 is violated.
//
// # Red Gate condition (Section 2 — ADR-029 wiring tests)
// Section 2 tests call `on_post_tool_use` with synthetic HookPayload structs
// carrying git_context in payload.extra. The current impl calls exec_subprocess
// for chain detection; the Red Gate test (sentinel chain → expect Block) FAILS
// because the current impl either reads the real repo HEAD or returns Continue
// early (no file_path in Bash payload). The corrected ADR-029 impl must read
// git_context from payload.extra and detect the sentinel chain on Bash events.
//
// # Naming convention
// `test_BC_S_SS_NNN_xxx()` — S=5, SS=41, NNN=003.
// Wiring tests use BC-1.16.001 trace for the injection contract.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    non_snake_case
)]

use serde_json::{Value, json};
use validate_dispatch_advance::{
    PRECOMPACT_FLUSH_PREFIX, check_multi_commit_chain, is_precompact_flush_exempt, on_post_tool_use,
};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Canonical constants — identical to validate-burst-log/tests/exemption.rs
// for symmetry verification (AC-006 / INV2).
// ---------------------------------------------------------------------------

const EXAMPLE_SHA: &str = "abc1234def5678abc1234def5678abc1234def56";
const OTHER_SHA: &str = "999aaabbbccc000111222333444555666777888";
const CYCLE_STEP: &str = "v1.0-feature-context-durability-E18/S-18.04";
const ISO_TS: &str = "2026-06-14T00:00:00Z";

fn log_line_valid(sha: &str) -> String {
    format!(
        "{ts} {sha} {cycle} commit",
        ts = ISO_TS,
        sha = sha,
        cycle = CYCLE_STEP
    )
}

fn log_line_corrupted(sha: &str) -> String {
    format!(
        "{ts} {sha} {cycle} push",
        ts = ISO_TS,
        sha = sha,
        cycle = CYCLE_STEP
    )
}

fn precompact_subject() -> String {
    format!(
        "PreCompact flush {cycle} {ts}",
        cycle = CYCLE_STEP,
        ts = ISO_TS
    )
}

const BACKFILL_SUBJECT: &str = "stage 1 backfill";
const BACKFILL_SUBJECT_2: &str = "stage 2 backfill";
const BURST_SUBJECT: &str = "state: burst-24 Commit E — D-477 codification";

// ---------------------------------------------------------------------------
// Section 1: Pure-logic tests — LOAD-BEARING PROOF VEHICLE for the exemption
// DECISION (ADR-029 §Decision 8).
//
// These tests are the authoritative proof that is_precompact_flush_exempt()
// (in validate-dispatch-advance) correctly implements the 3-case exemption
// logic per BC-5.41.003 PC1:
//   case (a): log present, FIELD-4="commit", SHA matches → exempt
//   case (b): log present, FIELD-4 corrupted/missing → treat as absent → exempt
//   case (c): log absent → prefix-match-only → exempt
// Breaking is_precompact_flush_exempt kills 4 of these tests (mutation-verified).
//
// WHY THESE ARE THE LOAD-BEARING LAYER (ADR-029 §Decision 8):
//   The two positive bats tests in vp084-proof.bats prove the dispatcher→WASM
//   git_context injection WIRING end-to-end (Layer 2). They do NOT prove
//   exemption-decision correctness: a broken exemption that always returned
//   "exempt" would also pass those positive bats tests (the real PreCompact
//   flush subject in their git repo makes the exemption outcome irrelevant to
//   the WASM's Continue result). Layer 2's non-tautology is closed by the
//   negative control bats test (sentinel chain → Block), not by the positive
//   bats tests. The exemption-DECISION correctness proof lives HERE in
//   Section 1 (both crates), and only here. These tests must never be removed
//   or weakened. The test_BC_5_41_003_dispatch_advance_exemption_symmetric
//   test further enforces that this crate's exemption logic is byte-for-byte
//   identical to validate-burst-log's (INV2 / AC-006 symmetry).
//
// Symmetric with crates/hook-plugins/validate-burst-log/tests/exemption.rs
// Section 1. Both Section 1 sets must continue to pass after the Section 2
// wiring change.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// AC-001 — log present, FIELD-4=commit, SHA matches → exempt
// BC-5.41.003 PC1 case (a) — symmetric with validate-burst-log
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt() {
    // AC-001 symmetric: validate-dispatch-advance must grant exemption when
    // log exists, FIELD-4 == "commit", and SHA matches FIELD-2.
    let subject = precompact_subject();
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));

    assert!(
        result,
        "AC-001 (dispatch-advance): PreCompact flush with matching SHA MUST be exempt; \
         subject={subject:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-002 — log absent → prefix-match-only exemption
// BC-5.41.003 PC1 case (c) — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_absent_exempt() {
    let subject = precompact_subject();

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, None);

    assert!(
        result,
        "AC-002 (dispatch-advance): PreCompact flush with absent log MUST be exempt on prefix alone"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — log exists but FIELD-4 corrupted → treat as absent → exempt
// BC-5.41.003 PC1 case (b) — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_corrupted_exempt() {
    let subject = precompact_subject();
    let corrupted_log = log_line_corrupted(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&corrupted_log));

    assert!(
        result,
        "AC-003 (dispatch-advance): corrupted FIELD-4 MUST trigger case (b) → exempt on prefix alone; \
         log={corrupted_log:?}"
    );
}

#[test]
fn test_BC_5_41_003_precompact_prefix_log_field4_empty_exempt() {
    let subject = precompact_subject();
    let truncated_log = format!(
        "{ts} {sha} {cycle}",
        ts = ISO_TS,
        sha = EXAMPLE_SHA,
        cycle = CYCLE_STEP
    );

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&truncated_log));

    assert!(
        result,
        "AC-003 variant (dispatch-advance): FIELD-4 absent/empty MUST be exempt on prefix alone"
    );
}

// AC-003 sub-case: FIELD-4="commit" but FIELD-2 (SHA) is absent → treat as corrupted → exempt.
// BC-5.41.003 PC1 case (b) sub-case — symmetric with validate-burst-log
#[test]
fn test_BC_5_41_003_precompact_prefix_log_field4_commit_no_sha_exempt() {
    // AC-003 sub-case: FIELD-4="commit" but FIELD-2 (SHA) is absent.
    // A 3-field line: "timestamp cycle/step commit" — FIELD-4 is "commit" but no FIELD-2 SHA.
    // Should treat as corrupted (case b) → prefix-match-only exempt.
    let subject = precompact_subject();
    // 3-field line: field1=timestamp, field2=cycle/step, field3=commit (no FIELD-2 sha, no FIELD-4)
    // Per the log format: "<ISO-timestamp> <SHA> <cycle>/<step> commit"
    // A truncated line: "<ISO-timestamp> <cycle>/<step> commit" — only 3 fields, field[3]="commit", field[1] absent
    let malformed_line = "2026-01-01T00:00:00Z cycle/step commit";
    // With this 3-field line, split_whitespace gives:
    //   field1 = "2026-01-01T00:00:00Z" (ISO-timestamp)
    //   field2 = "cycle/step"           (consumed as SHA — not a real 40-char SHA)
    //   field3 = "commit"               (consumed as cycle/step)
    //   field4 = None                   (type token is absent → case (b) → case (c))
    // Therefore FIELD-4 (type token) is absent → case (b) → exempt on prefix alone.
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(malformed_line));
    assert!(result, "FIELD-4=commit but FIELD-2 absent must be exempt via case (b) → case (c)");
}

// ---------------------------------------------------------------------------
// AC-004 — log valid, FIELD-4=commit, SHA MISMATCH → NOT exempt
// BC-5.41.003 PC1 in-body + INV1 — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt() {
    let subject = precompact_subject();
    let log_last_line = log_line_valid(OTHER_SHA); // log records OTHER_SHA

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));

    assert!(
        !result,
        "AC-004 (dispatch-advance): SHA-mismatch with valid FIELD-4 MUST NOT be exempt; \
         commit_sha={EXAMPLE_SHA:?} log_sha={OTHER_SHA:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — case-sensitive prefix
// BC-5.41.003 INV3 + EC-004 — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_case_sensitive() {
    let lowercase_subject = format!("precompact flush {CYCLE_STEP} {ISO_TS}");
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&lowercase_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result,
        "AC-005 (dispatch-advance): lowercase 'precompact flush' MUST NOT be exempt"
    );

    let no_space_subject = format!("PreCompact flush{CYCLE_STEP} {ISO_TS}");
    let result2 = is_precompact_flush_exempt(&no_space_subject, EXAMPLE_SHA, None);
    assert!(
        !result2,
        "AC-005 / EC-006 (dispatch-advance): missing trailing space MUST NOT be exempt"
    );
}

#[test]
fn test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt() {
    let mixed_subject = format!("preCompact flush {CYCLE_STEP} {ISO_TS}");

    let result = is_precompact_flush_exempt(&mixed_subject, EXAMPLE_SHA, None);
    assert!(
        !result,
        "AC-005 variant (dispatch-advance): mixed-case MUST NOT be exempt"
    );
}

// ---------------------------------------------------------------------------
// Non-PreCompact subject — not exempt
// BC-5.41.003 PC3 / INV3 — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_non_precompact_subject_not_exempt() {
    let normal_subject = "state: burst-24 Commit E — D-477 codification";

    let result_no_log = is_precompact_flush_exempt(normal_subject, EXAMPLE_SHA, None);
    assert!(
        !result_no_log,
        "Normal burst subject MUST NOT be exempt (log absent)"
    );

    let log_last_line = log_line_valid(EXAMPLE_SHA);
    let result_with_log =
        is_precompact_flush_exempt(normal_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result_with_log,
        "Normal burst subject MUST NOT be exempt (log present)"
    );
}

// ---------------------------------------------------------------------------
// check_multi_commit_chain tests — symmetric with validate-burst-log
// AC-006 / BC-5.41.003 INV2: both hooks must produce identical outcomes
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_chain_head_precompact_log_match_head_parent_burst_no_violation() {
    let head_subject = precompact_subject();
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA,
        BURST_SUBJECT,
        OTHER_SHA,
        Some(&log_last_line),
    );

    assert!(
        result.is_none(),
        "AC-001 chain (dispatch-advance): HEAD=PreCompact(log-match)+HEAD^=burst MUST NOT violate; \
         got: {result:?}"
    );
}

#[test]
fn test_BC_5_41_003_chain_head_burst_head_parent_precompact_log_match_no_violation() {
    let head_parent_subject = precompact_subject();
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = check_multi_commit_chain(
        BURST_SUBJECT,
        OTHER_SHA,
        &head_parent_subject,
        EXAMPLE_SHA,
        Some(&log_last_line),
    );

    assert!(
        result.is_none(),
        "AC-001 chain reversed (dispatch-advance): HEAD=burst+HEAD^=PreCompact(log-match) MUST NOT violate; \
         got: {result:?}"
    );
}

#[test]
fn test_BC_5_41_003_chain_both_sentinel_emits_violation() {
    let result = check_multi_commit_chain(
        BACKFILL_SUBJECT,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT_2,
        OTHER_SHA,
        None,
    );

    assert!(
        result.is_some(),
        "AC-006 (dispatch-advance): both HEAD+HEAD^ with sentinels MUST produce violation"
    );
    let violation = result.unwrap();
    assert!(
        violation
            .description
            .contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
        "Violation description MUST contain 'MULTI_COMMIT_CHAIN_NOT_ALLOWED'; \
         got: {:?}",
        violation.description
    );
}

#[test]
fn test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt() {
    let head_subject = precompact_subject();
    let log_last_line = log_line_valid(OTHER_SHA); // log SHA != commit SHA

    let exempt = is_precompact_flush_exempt(&head_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !exempt,
        "AC-004 (dispatch-advance): SHA-mismatch with valid FIELD-4 MUST NOT grant exemption"
    );

    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT_2,
        OTHER_SHA,
        Some(&log_last_line),
    );
    // PreCompact subject has no sentinel → no chain violation even without exemption.
    assert!(
        result.is_none(),
        "No violation expected: HEAD=PreCompact(no sentinel, sha-mismatch)+HEAD^=backfill; \
         got: {result:?}"
    );
}

#[test]
fn test_BC_5_41_003_chain_precompact_log_absent_exemption_fires() {
    let head_subject = precompact_subject();

    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT,
        OTHER_SHA,
        None,
    );

    assert!(
        result.is_none(),
        "AC-002 in chain (dispatch-advance): HEAD=PreCompact(log-absent)+HEAD^=backfill MUST NOT violate; \
         got: {result:?}"
    );
}

#[test]
fn test_BC_5_41_003_chain_both_precompact_no_violation() {
    let subject_1 = precompact_subject();
    let subject_2 = format!("PreCompact flush {CYCLE_STEP} 2026-06-15T00:00:00Z");
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = check_multi_commit_chain(
        &subject_1,
        EXAMPLE_SHA,
        &subject_2,
        OTHER_SHA,
        Some(&log_last_line),
    );

    assert!(
        result.is_none(),
        "EC-002 (dispatch-advance): two consecutive PreCompact commits MUST NOT violate; got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-006 — symmetry assertion: validate-dispatch-advance identical to validate-burst-log
// BC-5.41.003 INV2
//
// This test drives both gates through the same input vectors and asserts
// they produce identical exemption outcomes. Until both are implemented,
// both will panic with todo!() — which is the correct Red Gate state.
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_dispatch_advance_exemption_symmetric() {
    // The set of inputs below covers all three cases (a)/(b)/(c) + SHA-mismatch.
    // For each input, we call BOTH crates and assert identical results.
    // This is the canonical AC-006 / INV2 symmetry test.

    struct TestCase {
        label: &'static str,
        subject: String,
        sha: &'static str,
        log: Option<String>,
        expected_exempt: bool,
    }

    let cases = [
        TestCase {
            label: "case-a: log present, sha match, field4=commit → exempt",
            subject: precompact_subject(),
            sha: EXAMPLE_SHA,
            log: Some(log_line_valid(EXAMPLE_SHA)),
            expected_exempt: true,
        },
        TestCase {
            label: "case-c: log absent → exempt",
            subject: precompact_subject(),
            sha: EXAMPLE_SHA,
            log: None,
            expected_exempt: true,
        },
        TestCase {
            label: "case-b: log corrupted → exempt",
            subject: precompact_subject(),
            sha: EXAMPLE_SHA,
            log: Some(log_line_corrupted(EXAMPLE_SHA)),
            expected_exempt: true,
        },
        TestCase {
            label: "sha-mismatch with valid field4 → NOT exempt",
            subject: precompact_subject(),
            sha: EXAMPLE_SHA,
            log: Some(log_line_valid(OTHER_SHA)),
            expected_exempt: false,
        },
        TestCase {
            label: "lowercase prefix → NOT exempt",
            subject: format!("precompact flush {CYCLE_STEP} {ISO_TS}"),
            sha: EXAMPLE_SHA,
            log: None,
            expected_exempt: false,
        },
    ];

    for case in &cases {
        let da_result = is_precompact_flush_exempt(&case.subject, case.sha, case.log.as_deref());

        let vbl_result = validate_burst_log::is_precompact_flush_exempt(
            &case.subject,
            case.sha,
            case.log.as_deref(),
        );

        assert_eq!(
            da_result,
            vbl_result,
            "AC-006 INV2 SYMMETRY VIOLATION: validate-dispatch-advance ({da_result}) \
             and validate-burst-log ({vbl_result}) produced DIFFERENT exemption outcomes \
             for case '{label}'; inputs: subject={subject:?} sha={sha:?} log={log:?}",
            label = case.label,
            subject = case.subject,
            sha = case.sha,
            log = case.log,
        );

        assert_eq!(
            da_result,
            case.expected_exempt,
            "AC-006 symmetry case '{label}': expected_exempt={expected} but got {actual}; \
             inputs: subject={subject:?} sha={sha:?}",
            label = case.label,
            expected = case.expected_exempt,
            actual = da_result,
            subject = case.subject,
            sha = case.sha,
        );
    }
}

// ---------------------------------------------------------------------------
// PRECOMPACT_FLUSH_PREFIX constant correctness
// AC-008: exact byte sequence — symmetric
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_flush_prefix_constant_exact() {
    assert_eq!(
        validate_dispatch_advance::PRECOMPACT_FLUSH_PREFIX,
        "PreCompact flush ",
        "PRECOMPACT_FLUSH_PREFIX (dispatch-advance) MUST be exactly 'PreCompact flush '"
    );
    assert_eq!(
        validate_dispatch_advance::PRECOMPACT_FLUSH_PREFIX.len(),
        17,
        "PRECOMPACT_FLUSH_PREFIX must have exactly 17 bytes"
    );
}

/// Assert that both crates expose the same constant value (AC-006 / AC-008 together).
#[test]
fn test_BC_5_41_003_precompact_flush_prefix_both_crates_identical() {
    assert_eq!(
        validate_dispatch_advance::PRECOMPACT_FLUSH_PREFIX,
        validate_burst_log::PRECOMPACT_FLUSH_PREFIX,
        "AC-006+AC-008: both crates MUST export the identical PRECOMPACT_FLUSH_PREFIX constant"
    );
}

// ===========================================================================
// Section 2: ADR-029 wiring tests (RED GATE — these FAIL against current impl)
//
// Symmetric with crates/hook-plugins/validate-burst-log/tests/exemption.rs
// Section 2. Both gates must implement the ADR-029 git_context wiring
// identically (AC-006 / INV2 symmetry).
//
// WHY THEY FAIL NOW:
//   The current `check_factory_artifacts_chain()` calls `host::exec_subprocess`
//   to obtain HEAD/HEAD^ subjects. In the unit-test harness (native), this reads
//   the REAL repo HEAD — not the synthetic git_context in the payload. A Bash
//   payload with no `file_path` triggers early Continue, causing the sentinel
//   chain Red Gate test to fail the assertion (got Continue, expected Block).
//
//   The CORRECT impl (ADR-029) must:
//     1. Detect tool=="Bash" and command containing "git commit".
//     2. Extract payload.extra["git_context"] four fields.
//     3. Drive check_multi_commit_chain from those fields.
//     4. Return Continue when git_context is absent/empty (fail-open).
//     5. NOT call exec_subprocess for commit context.
// ===========================================================================

/// Build a synthetic `HookPayload` representing a PostToolUse Bash event
/// with a git commit command, carrying the given git_context fields.
///
/// BC-1.16.001 PC1: all four fields always present (string, never null).
fn bash_commit_payload_with_git_context(
    git_commit_command: &str,
    head_subject: &str,
    head_sha: &str,
    head_parent_subject: &str,
    head_parent_sha: &str,
) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("command".to_string(), json!(git_commit_command));

    let git_context = json!({
        "head_subject": head_subject,
        "head_sha": head_sha,
        "head_parent_subject": head_parent_subject,
        "head_parent_sha": head_parent_sha
    });

    let mut extra = serde_json::Map::new();
    extra.insert("git_context".to_string(), git_context);

    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "test-session-da-adr029".to_string(),
        dispatcher_trace_id: "test-trace-da-wiring".to_string(),
        tool_input: Value::Object(tool_input),
        tool_response: None,
        plugin_config: Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
        extra: extra.into_iter().collect(),
    }
}

/// Build a PostToolUse Bash payload WITHOUT git_context (dispatcher fail-open path).
fn bash_commit_payload_no_git_context(git_commit_command: &str) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("command".to_string(), json!(git_commit_command));

    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "test-session-da-no-ctx".to_string(),
        dispatcher_trace_id: "test-trace-da-no-ctx".to_string(),
        tool_input: Value::Object(tool_input),
        tool_response: None,
        plugin_config: Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
        extra: std::collections::HashMap::new(),
    }
}

/// Build a PostToolUse Bash payload with all-empty git_context fields.
fn bash_commit_payload_empty_git_context(git_commit_command: &str) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("command".to_string(), json!(git_commit_command));

    let git_context = json!({
        "head_subject": "",
        "head_sha": "",
        "head_parent_subject": "",
        "head_parent_sha": ""
    });

    let mut extra = std::collections::HashMap::new();
    extra.insert("git_context".to_string(), git_context);

    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "test-session-da-empty-ctx".to_string(),
        dispatcher_trace_id: "test-trace-da-empty-ctx".to_string(),
        tool_input: Value::Object(tool_input),
        tool_response: None,
        plugin_config: Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
        extra,
    }
}

/// Build a PostToolUse Edit payload (non-qualifying per ADR-029 §Decision 1).
fn edit_payload_with_sentinel_git_context(file_path: &str) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("file_path".to_string(), json!(file_path));

    let git_context = json!({
        "head_subject": "stage 1 backfill",
        "head_sha": EXAMPLE_SHA,
        "head_parent_subject": "stage 2 backfill",
        "head_parent_sha": OTHER_SHA
    });
    let mut extra = std::collections::HashMap::new();
    extra.insert("git_context".to_string(), git_context);

    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Edit".to_string(),
        session_id: "test-session-da-edit".to_string(),
        dispatcher_trace_id: "test-trace-da-edit".to_string(),
        tool_input: Value::Object(tool_input),
        tool_response: None,
        plugin_config: Value::Null,
        agent_type: None,
        subagent_name: None,
        last_assistant_message: None,
        result: None,
        extra,
    }
}

// --- RED GATE TEST 1: Bash commit with chain sentinels in git_context → must block ---
//
// Symmetric with validate-burst-log Section 2 RED GATE TEST 1.
// ADR-029 §Decision 1 + §Decision 5: on PostToolUse Bash (git commit), the WASM gate
// reads git_context from payload.extra and runs chain check. Sentinel chain → Block.
//
// RED GATE: Current exec-based impl ignores git_context. Bash payload has no file_path
// → early Continue. The corrected impl must detect Bash events and read git_context.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block() {
    let payload = bash_commit_payload_with_git_context(
        "git -C .factory commit -m \"stage 1 backfill\"",
        "stage 1 backfill",
        EXAMPLE_SHA,
        "stage 2 backfill",
        OTHER_SHA,
    );

    let result = on_post_tool_use(payload);

    match &result {
        HookResult::Block { reason } => {
            assert!(
                reason.contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
                "Block reason MUST contain 'MULTI_COMMIT_CHAIN_NOT_ALLOWED'; got: {reason:?}"
            );
        }
        HookResult::Continue => {
            panic!(
                "RED GATE (dispatch-advance): on_post_tool_use returned Continue when sentinel \
                 chain was present in git_context. The corrected ADR-029 impl must read \
                 git_context from payload.extra and detect MULTI_COMMIT_CHAIN_NOT_ALLOWED. \
                 Current exec-based impl reads exec_subprocess instead — Red Gate confirmed."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error (plugin internal failure); \
                 expected Block for sentinel chain. message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 2: Bash commit with PreCompact HEAD → must Continue ---
//
// Symmetric with validate-burst-log Section 2 RED GATE TEST 2.
// git_context HEAD=PreCompact (exempt, log absent → case c), HEAD^=burst.
// Expected after T-4: Continue (exemption fires via git_context).
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues() {
    let precompact_head =
        "PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z"
            .to_string();
    let payload = bash_commit_payload_with_git_context(
        "git -C .factory commit -m \"state: burst-24 Commit E\"",
        &precompact_head,
        EXAMPLE_SHA,
        "state: burst-24 Commit E — D-477 codification",
        OTHER_SHA,
    );

    let result = on_post_tool_use(payload);

    match &result {
        HookResult::Continue => {
            // Correct: exemption fired via git_context, no block.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE (dispatch-advance): blocked when PreCompact HEAD in git_context \
                 should be exempt. Reason: {reason:?}\n\
                 Corrected ADR-029 impl must read git_context and apply PreCompact exemption."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error; expected Continue for PreCompact-exempt HEAD. \
                 message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 3: Fail-open — git_context absent → must Continue ---
//
// Symmetric with validate-burst-log Section 2 RED GATE TEST 3.
// BC-1.16.001 INV3: absent git_context → skip chain check → Continue.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues() {
    let payload = bash_commit_payload_no_git_context(
        "git -C .factory commit -m \"state: burst-24 Commit E\"",
    );

    let result = on_post_tool_use(payload);

    match &result {
        HookResult::Continue => {
            // Correct fail-open behavior.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE (dispatch-advance): blocked when git_context absent; \
                 fail-open requires Continue. Reason: {reason:?}\n\
                 BC-1.16.001 INV3: WASM gate must skip chain check when git_context absent."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error; expected Continue for absent git_context. \
                 message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 4: Fail-open — all-empty git_context → must Continue ---
//
// Symmetric with validate-burst-log Section 2 RED GATE TEST 4.
// BC-1.16.001 PC2: dispatcher fail-open injects all-empty git_context.
// WASM gate must skip chain check and Continue.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues() {
    let payload =
        bash_commit_payload_empty_git_context("git -C .factory commit -m \"stage 1 backfill\"");

    let result = on_post_tool_use(payload);

    match &result {
        HookResult::Continue => {
            // Correct: all-empty git_context → fail-open → skip chain check.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE (dispatch-advance): blocked when all git_context fields empty; \
                 fail-open requires Continue. Reason: {reason:?}\n\
                 BC-1.16.001 INV3: all-empty git_context = dispatcher fail-open path."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error; expected Continue for all-empty git_context. \
                 message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 5: Edit event with sentinel git_context → must NOT trigger chain block ---
//
// Symmetric with validate-burst-log Section 2 RED GATE TEST 5.
// ADR-029 §Decision 1: chain detection ONLY fires on PostToolUse Bash git-commit events.
// Edit events MUST NOT trigger chain detection even if git_context is present.
#[test]
fn test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block() {
    // Edit event on STATE.md with git_context containing sentinel subjects.
    // The chain check MUST NOT fire for Edit events per ADR-029 §Decision 1.
    let payload = edit_payload_with_sentinel_git_context("/some/path/.factory/STATE.md");

    let result = on_post_tool_use(payload);

    // Result may be Continue (fail-open if STATE.md unreadable) or a STATE.md structural block.
    // What must NOT happen: a MULTI_COMMIT_CHAIN_NOT_ALLOWED block.
    match &result {
        HookResult::Block { reason } => {
            assert!(
                !reason.contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
                "RED GATE (dispatch-advance): Edit event must NOT trigger \
                 MULTI_COMMIT_CHAIN_NOT_ALLOWED block, even with sentinel git_context. \
                 ADR-029 §Decision 1: chain detection is Bash-only. Got: {reason:?}"
            );
        }
        HookResult::Continue => {
            // Expected: no chain block on Edit events.
        }
        HookResult::Error { .. } => {
            // Error is acceptable (host I/O unavailable in test); chain block didn't fire.
        }
    }
}

// --- Exec-free constraint documentation (AC-006 / ADR-029 §Decision 3) ---
//
// Symmetric with validate-burst-log Section 2 exec-free constraint test.
#[test]
fn test_BC_5_41_003_wiring_exec_free_constraint_documented() {
    // AC-006: validate-dispatch-advance WASM MUST NOT call exec_subprocess
    // for commit context. The wiring tests above are the load-bearing Red Gate assertions.
    let _ =
        "AC-006 (dispatch-advance): WASM reads git_context from payload.extra, not exec_subprocess";
}

// --- PRECOMPACT_FLUSH_PREFIX constant export (AC-008 / ADR-029 symmetry) ---
//
// Verify the constant is accessible from the test (mirrors validate-burst-log).
#[test]
fn test_BC_5_41_003_dispatch_advance_precompact_flush_prefix_accessible() {
    assert_eq!(
        PRECOMPACT_FLUSH_PREFIX, "PreCompact flush ",
        "PRECOMPACT_FLUSH_PREFIX (dispatch-advance) must be accessible and exact"
    );
}
