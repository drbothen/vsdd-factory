// crates/hook-plugins/validate-burst-log/tests/exemption.rs
//
// Red Gate failing tests for S-18.04b — PreCompact flush exemption logic
// in `validate-burst-log`.
//
// # BC trace
// BC-5.41.003 PC1 cases (a)/(b)/(c); INV1; INV3; AC-001..AC-006; AC-008.
//
// # Red Gate condition
// All tests in this file call `is_precompact_flush_exempt` and/or
// `check_multi_commit_chain`, which are `todo!()` stubs in the production
// source. Every test will PANIC with "not yet implemented" until the
// implementer fills in the real logic. That panic is the expected Red Gate
// failure: the tests compile cleanly but fail at runtime.
//
// # Naming convention (BC-5.38 / story instructions)
// Test names follow `test_BC_S_SS_NNN_xxx()` where S=5, SS=41, NNN=003.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use validate_burst_log::{check_multi_commit_chain, is_precompact_flush_exempt};

// ---------------------------------------------------------------------------
// Canonical log-line constants for use across test vectors.
//
// Format (BC-5.41.003 Architecture Anchors):
//   <ISO-timestamp> <SHA> <cycle>/<step> commit
// ---------------------------------------------------------------------------

const EXAMPLE_SHA: &str = "abc1234def5678abc1234def5678abc1234def56";
const OTHER_SHA: &str = "999aaabbbccc000111222333444555666777888";
const CYCLE_STEP: &str = "v1.0-feature-context-durability-E18/S-18.04";
const ISO_TS: &str = "2026-06-14T00:00:00Z";

/// Build a canonical 4-field log last-line with FIELD-4 == "commit".
fn log_line_valid(sha: &str) -> String {
    format!(
        "{ts} {sha} {cycle} commit",
        ts = ISO_TS,
        sha = sha,
        cycle = CYCLE_STEP
    )
}

/// Build a 4-field log last-line with FIELD-4 != "commit" (simulating corruption).
fn log_line_corrupted(sha: &str) -> String {
    format!(
        "{ts} {sha} {cycle} push",
        ts = ISO_TS,
        sha = sha,
        cycle = CYCLE_STEP
    )
}

/// A valid `PreCompact flush` commit subject as produced by precompact-flush.sh.
fn precompact_subject() -> String {
    format!(
        "PreCompact flush {cycle} {ts}",
        cycle = CYCLE_STEP,
        ts = ISO_TS
    )
}

// ---------------------------------------------------------------------------
// AC-001 — log present, FIELD-4=commit, SHA matches → exempt
// BC-5.41.003 PC1 case (a)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt() {
    // Exercises VP-084 Postcondition A + BC-5.41.003 PC1 case (a).
    // The log exists, FIELD-4 == "commit", and the commit SHA matches FIELD-2.
    // Expected: is_precompact_flush_exempt returns true.
    let subject = precompact_subject();
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));

    assert!(
        result,
        "AC-001: PreCompact flush commit with matching SHA in valid log MUST be exempt; \
         subject={subject:?} sha={EXAMPLE_SHA:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-002 — log absent → prefix-match-only exemption
// BC-5.41.003 PC1 case (c)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_absent_exempt() {
    // Exercises BC-5.41.003 PC1 case (c): log file is absent (None).
    // When the log is absent, prefix match alone is sufficient for exemption.
    // Expected: is_precompact_flush_exempt returns true.
    let subject = precompact_subject();

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, None);

    assert!(
        result,
        "AC-002: PreCompact flush commit with absent log MUST be exempt on prefix match alone; \
         subject={subject:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — log exists but FIELD-4 corrupted → treat as absent → exempt
// BC-5.41.003 PC1 case (b)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_corrupted_exempt() {
    // Exercises BC-5.41.003 PC1 case (b): log exists but last-line FIELD-4
    // is not the literal "commit" (here it is "push" — simulating corruption).
    // Gate must fall through to case (c): prefix-match-only exemption.
    // Expected: is_precompact_flush_exempt returns true.
    let subject = precompact_subject();
    let corrupted_log = log_line_corrupted(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&corrupted_log));

    assert!(
        result,
        "AC-003: PreCompact flush commit with corrupted FIELD-4 MUST be exempt on prefix alone; \
         log_line={corrupted_log:?}"
    );
}

/// Variant: FIELD-4 is empty string (not the literal "commit").
#[test]
fn test_BC_5_41_003_precompact_prefix_log_field4_empty_exempt() {
    // FIELD-4 empty → treat as absent/corrupted → case (b) → prefix-match-only exemption.
    let subject = precompact_subject();
    // Log line: 3 fields only (no FIELD-4).
    let truncated_log = format!(
        "{ts} {sha} {cycle}",
        ts = ISO_TS,
        sha = EXAMPLE_SHA,
        cycle = CYCLE_STEP
    );

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&truncated_log));

    assert!(
        result,
        "AC-003 variant: FIELD-4 absent/empty MUST trigger case (b) → exempt on prefix alone; \
         log_line={truncated_log:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — log valid, FIELD-4=commit, SHA MISMATCH → NOT exempt
// BC-5.41.003 PC1 in-body + INV1
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt() {
    // Exercises BC-5.41.003 PC1 in-body + INV1: SHA-mismatch with valid FIELD-4
    // is NOT exempt. The log records a DIFFERENT SHA than the commit being evaluated.
    // This prevents arbitrary-prefix bypass.
    // Expected: is_precompact_flush_exempt returns false.
    let subject = precompact_subject();
    // Log records OTHER_SHA, but commit being evaluated has EXAMPLE_SHA.
    let log_last_line = log_line_valid(OTHER_SHA);

    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));

    assert!(
        !result,
        "AC-004: SHA-mismatch with valid FIELD-4=commit MUST NOT be exempt; \
         commit_sha={EXAMPLE_SHA:?} log_sha={OTHER_SHA:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — case-sensitive prefix: lowercase "precompact flush " NOT exempt
// BC-5.41.003 INV3 + EC-004
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_prefix_case_sensitive() {
    // Exercises BC-5.41.003 INV3: the exempt prefix is exactly "PreCompact flush "
    // (capital P, capital C). Lowercase variant must NOT be exempt.
    // Expected: is_precompact_flush_exempt returns false regardless of log state.

    // Case 1: completely lowercase.
    let lowercase_subject = format!("precompact flush {CYCLE_STEP} {ISO_TS}");
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = is_precompact_flush_exempt(&lowercase_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result,
        "AC-005 (lowercase p): 'precompact flush ...' MUST NOT be exempt; \
         only 'PreCompact flush ' (canonical case) is exempt"
    );

    // Case 2: no trailing space (exact prefix requires trailing space per EC-006).
    let no_space_subject = format!("PreCompact flush{CYCLE_STEP} {ISO_TS}");
    let result2 = is_precompact_flush_exempt(&no_space_subject, EXAMPLE_SHA, None);
    assert!(
        !result2,
        "AC-005 / EC-006: 'PreCompact flush' without trailing space MUST NOT be exempt"
    );
}

/// Variant: mixed-case "preCompact flush" is also NOT exempt.
#[test]
fn test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt() {
    let mixed_subject = format!("preCompact flush {CYCLE_STEP} {ISO_TS}");

    let result = is_precompact_flush_exempt(&mixed_subject, EXAMPLE_SHA, None);
    assert!(
        !result,
        "AC-005 variant: mixed-case prefix MUST NOT be exempt; only canonical 'PreCompact flush ' applies"
    );
}

// ---------------------------------------------------------------------------
// Non-PreCompact subject — exempt flag must be false regardless of log
// BC-5.41.003 PC3 / INV3 (normal chain detection preserved)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_non_precompact_subject_not_exempt() {
    // A normal burst subject must never be exempt.
    let normal_subject = "state: burst-24 Commit E — D-477 codification";

    let result_no_log = is_precompact_flush_exempt(normal_subject, EXAMPLE_SHA, None);
    assert!(
        !result_no_log,
        "Normal burst subject MUST NOT be exempt (log absent path)"
    );

    let log_last_line = log_line_valid(EXAMPLE_SHA);
    let result_with_log =
        is_precompact_flush_exempt(normal_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result_with_log,
        "Normal burst subject MUST NOT be exempt (log present path)"
    );
}

// ---------------------------------------------------------------------------
// check_multi_commit_chain tests — chain detection with exemption
// BC-5.41.003 PC1+PC2+PC3; INV1; TD-VSDD-053
// ---------------------------------------------------------------------------

/// A log last-line matching HEAD SHA (the PreCompact commit).
fn log_matching_head() -> String {
    log_line_valid(EXAMPLE_SHA)
}

/// Sentinel: subject that triggers the multi-commit-chain detector.
const BACKFILL_SUBJECT: &str = "stage 1 backfill";
const BACKFILL_SUBJECT_2: &str = "stage 2 backfill";

/// A normal burst commit subject that does NOT contain a sentinel.
const BURST_SUBJECT: &str = "state: burst-24 Commit E — D-477 codification";

// --- test vector 1: HEAD=PreCompact (log match), HEAD^=normal → no violation ---

#[test]
fn test_BC_5_41_003_chain_head_precompact_log_match_head_parent_burst_no_violation() {
    // BC-5.41.003 Canonical Test Vector: happy-path-exempt
    // HEAD is a PreCompact commit with matching SHA in valid log.
    // HEAD^ is a normal burst commit.
    // Expected: check_multi_commit_chain returns None (no violation).
    let head_subject = precompact_subject();
    let head_sha = EXAMPLE_SHA;
    let log_last_line = log_matching_head();

    let result = check_multi_commit_chain(
        &head_subject,
        head_sha,
        BURST_SUBJECT,
        OTHER_SHA,
        Some(&log_last_line),
    );

    assert!(
        result.is_none(),
        "AC-001 chain: HEAD=PreCompact(log-match) + HEAD^=burst MUST NOT trigger violation; \
         got: {result:?}"
    );
}

// --- test vector 2: HEAD=burst, HEAD^=PreCompact (log match) → no violation ---

#[test]
fn test_BC_5_41_003_chain_head_burst_head_parent_precompact_log_match_no_violation() {
    // BC-5.41.003 Canonical Test Vector: burst-after-precompact
    // HEAD is a normal burst commit, HEAD^ is a PreCompact commit.
    // Expected: None (HEAD^ is exempt; no chain violation).
    let head_parent_subject = precompact_subject();
    let head_parent_sha = EXAMPLE_SHA;
    let log_last_line = log_line_valid(EXAMPLE_SHA);

    let result = check_multi_commit_chain(
        BURST_SUBJECT,
        OTHER_SHA,
        &head_parent_subject,
        head_parent_sha,
        Some(&log_last_line),
    );

    assert!(
        result.is_none(),
        "AC-001 chain (reversed): HEAD=burst + HEAD^=PreCompact(log-match) MUST NOT trigger violation; \
         got: {result:?}"
    );
}

// --- test vector 3: HEAD=backfill, HEAD^=backfill → chain violation ---

#[test]
fn test_BC_5_41_003_chain_both_sentinel_emits_violation() {
    // BC-5.41.003 Canonical Test Vector: normal-chain-detection-preserved
    // Both HEAD and HEAD^ contain sentinel words. No PreCompact commit involved.
    // Expected: Some(Violation) with MULTI_COMMIT_CHAIN_NOT_ALLOWED.
    let result = check_multi_commit_chain(
        BACKFILL_SUBJECT,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT_2,
        OTHER_SHA,
        None,
    );

    assert!(
        result.is_some(),
        "AC-006 / BC-5.41.003 PC3: both HEAD+HEAD^ containing sentinels MUST produce violation"
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

// --- test vector 4: HEAD=PreCompact (SHA mismatch in valid log) → violation ---

#[test]
fn test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt() {
    // BC-5.41.003 EC-003 (SHA mismatch): HEAD is a PreCompact commit but its SHA
    // does NOT match FIELD-2 of the log last line (which has FIELD-4=commit).
    // This is a suspicious case — treated as chain detection applying normally.
    //
    // For check_multi_commit_chain to detect a violation, HEAD^ must also contain
    // a sentinel word. We use a backfill HEAD^ to confirm the gate fires.
    let head_subject = precompact_subject();
    // Log records OTHER_SHA but the head_sha being evaluated is EXAMPLE_SHA.
    let log_last_line = log_line_valid(OTHER_SHA);

    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA, // SHA does NOT match log's OTHER_SHA
        BACKFILL_SUBJECT_2,
        OTHER_SHA,
        Some(&log_last_line),
    );

    // Because the PreCompact commit is not exempt (SHA mismatch + valid FIELD-4),
    // it may or may not itself contain a sentinel. The key assertion per AC-004 /
    // INV1 is that exemption is NOT granted. The function must NOT return None on
    // the grounds of "PreCompact prefix found". Whether a violation fires depends
    // on whether the PreCompact subject contains a sentinel — it does not, so this
    // specific combo may return None for chain (no sentinel in head) but it must
    // not grant exemption silently.
    //
    // To make this a direct load-bearing assertion: confirm the exemption path
    // did NOT fire by calling is_precompact_flush_exempt directly.
    let exempt = is_precompact_flush_exempt(&head_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !exempt,
        "AC-004: SHA-mismatch with valid FIELD-4=commit MUST NOT grant exemption; \
         commit_sha={EXAMPLE_SHA:?} log_sha={OTHER_SHA:?}"
    );

    // The result from check_multi_commit_chain should be None here because the
    // PreCompact subject itself does not contain a sentinel word, so even without
    // exemption, the chain detector would not trigger (only one half of the chain
    // has a sentinel: HEAD^=backfill). This is correct behavior — verify result is None.
    assert!(
        result.is_none(),
        "Expected no chain violation when HEAD=PreCompact(no sentinel) + HEAD^=backfill, \
         even with SHA mismatch; got: {result:?}"
    );
}

// --- PreCompact log absent, HEAD^=backfill: HEAD is still exempt → no violation ---

#[test]
fn test_BC_5_41_003_chain_precompact_log_absent_exemption_fires() {
    // AC-002 in chain context: log absent → prefix-match-only exemption → no violation.
    let head_subject = precompact_subject();

    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT,
        OTHER_SHA,
        None, // log absent
    );

    assert!(
        result.is_none(),
        "AC-002 in chain: HEAD=PreCompact(log-absent) + HEAD^=backfill MUST NOT trigger violation; \
         got: {result:?}"
    );
}

// --- Both commits are PreCompact: no violation (EC-002 double-compaction) ---

#[test]
fn test_BC_5_41_003_chain_both_precompact_no_violation() {
    // BC-5.41.003 EC-002: two consecutive PreCompact commits.
    // Neither triggers MULTI_COMMIT_CHAIN_NOT_ALLOWED; both are individually exempt.
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
        "EC-002: two consecutive PreCompact commits MUST NOT trigger violation; got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// PRECOMPACT_FLUSH_PREFIX constant correctness
// AC-008: exact byte sequence, no deviation
// ---------------------------------------------------------------------------

#[test]
fn test_BC_5_41_003_precompact_flush_prefix_constant_exact() {
    // AC-008: the PRECOMPACT_FLUSH_PREFIX constant must be exactly "PreCompact flush "
    // (capital P, capital C, trailing space — 17 bytes).
    assert_eq!(
        validate_burst_log::PRECOMPACT_FLUSH_PREFIX,
        "PreCompact flush ",
        "PRECOMPACT_FLUSH_PREFIX constant MUST be exactly 'PreCompact flush ' (17 bytes)"
    );
    assert_eq!(
        validate_burst_log::PRECOMPACT_FLUSH_PREFIX.len(),
        17,
        "PRECOMPACT_FLUSH_PREFIX must have exactly 17 bytes"
    );
}
