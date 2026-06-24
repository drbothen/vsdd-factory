// crates/hook-plugins/validate-dispatch-advance/tests/exemption.rs
//
// Red Gate failing tests for S-18.04b — PreCompact flush exemption logic
// in `validate-dispatch-advance`.
//
// # BC trace
// BC-5.41.003 PC1 cases (a)/(b)/(c); INV1; INV2 (symmetry); INV3; AC-001..AC-006; AC-008.
//
// # Symmetry requirement (AC-006 / BC-5.41.003 INV2)
// This file MIRRORS the test structure and inputs of
// `crates/hook-plugins/validate-burst-log/tests/exemption.rs`.
// Both gates must implement the 3-case exemption identically. The assertions
// here are the authoritative AC-006 / INV2 symmetry check: if validate-burst-log
// passes a test but validate-dispatch-advance fails it (or vice versa), INV2 is violated.
//
// # Red Gate condition
// All tests call `is_precompact_flush_exempt` and/or `check_multi_commit_chain`,
// which are `todo!()` stubs. Every test will PANIC until the implementer fills
// in the real logic.
//
// # Naming convention
// `test_BC_S_SS_NNN_xxx()` — S=5, SS=41, NNN=003.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use validate_dispatch_advance::{check_multi_commit_chain, is_precompact_flush_exempt};

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
