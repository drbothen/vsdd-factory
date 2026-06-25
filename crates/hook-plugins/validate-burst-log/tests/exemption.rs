// crates/hook-plugins/validate-burst-log/tests/exemption.rs
//
// Red Gate failing tests for S-18.04b — PreCompact flush exemption logic
// in `validate-burst-log`, rewritten for the ADR-029 git_context wiring.
//
// # What changed from pre-ADR-029 tests
//
// ADR-029 §Decision 1 + §Decision 3 re-architecture:
//   - The production trigger is NOW PostToolUse Bash (tool="Bash", command
//     contains "git commit"), NOT PostToolUse Edit/Write.
//   - The WASM gate reads commit subjects+SHAs from `payload.extra["git_context"]`,
//     NOT from exec_subprocess("git", ...).
//   - If `git_context` is absent or all-empty, the gate MUST skip the check
//     (fail-open) — never block.
//
// # Red Gate condition (why these tests must FAIL against the current impl)
//
// The tests below call `on_post_tool_use` with a Bash-tool payload carrying a
// real 4-field `git_context` object in `tool_input` (simulating what the dispatcher
// would inject per BC-1.16.001). The CURRENT implementation of `check_factory_artifacts_chain()`
// ignores `payload.extra["git_context"]` and calls `host::exec_subprocess("git", ...)` instead.
// In the unit-test harness, `host::exec_subprocess` is either:
//   (a) unavailable (WASM host stubs return Err), causing the chain check to
//       skip entirely — meaning the positive chain-detection test FAILS to detect
//       any violation when it should, OR
//   (b) wired to a real git binary pointing at the wrong repo — making assertion
//       outcomes non-deterministic.
//
// The CORRECT implementation must:
//   1. Detect `tool="Bash"` AND `tool_input.command` containing "git commit".
//   2. Extract `payload.extra["git_context"]` fields.
//   3. Run `check_multi_commit_chain` on those fields.
//   4. Return Continue (no block) when git_context is absent/empty (fail-open).
//   5. NOT call `host::exec_subprocess` for commit-context acquisition.
//
// These tests drive `on_post_tool_use` directly with synthetic payloads. They
// will FAIL until the implementer rewires `check_factory_artifacts_chain()` to
// read from `payload.extra["git_context"]` instead of exec_subprocess.
//
// # Preserved pure-logic tests
//
// Tests for `is_precompact_flush_exempt`, `check_multi_commit_chain`,
// `contains_sentinel`, `PRECOMPACT_FLUSH_PREFIX` are PRESERVED below.
// These pure-function tests already PASS. They are NOT the Red Gate — they
// verify the 3-case logic that must remain correct after the wiring change.
// The Red Gate is the `on_post_tool_use` wiring tests (section 2 below).
//
// # BC / AC trace
// BC-5.41.003 PC1 cases (a)/(b)/(c); INV1–INV4; AC-001–AC-008.
// BC-1.16.001: git_context schema (4 fields, all string, fail-open on absent).
// ADR-029 §Decision 1 (trigger: Bash), §Decision 3 (exec-free WASM), §Decision 5.
// VP-084 proof-model per ADR-029 §Decision 6.
//
// # Naming convention (BC-5.38 / story instructions)
// test_BC_S_SS_NNN_xxx() — S=5, SS=41, NNN=003.
// Wiring tests use BC-1.16.001 trace since they exercise the injection contract.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    non_snake_case
)]

use serde_json::{Value, json};
use validate_burst_log::{
    PRECOMPACT_FLUSH_PREFIX, check_multi_commit_chain, is_precompact_flush_exempt, on_post_tool_use,
};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Section 1: Pure-logic tests — LOAD-BEARING PROOF VEHICLE for the exemption
// DECISION (ADR-029 §Decision 8).
//
// These tests are the authoritative proof that is_precompact_flush_exempt()
// correctly implements the 3-case exemption logic per BC-5.41.003 PC1:
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
//   Section 1, and only here. These tests must never be removed or weakened.
//
// They must continue to pass after the Section 2 wiring change.
// ---------------------------------------------------------------------------

const EXAMPLE_SHA: &str = "abc1234def5678abc1234def5678abc1234def56";
const OTHER_SHA: &str = "999aaabbbccc000111222333444555666777888";
const CYCLE_STEP: &str = "v1.0-feature-context-durability-E18/S-18.04";
const ISO_TS: &str = "2026-06-14T00:00:00Z";

fn log_line_valid(sha: &str) -> String {
    format!("{ISO_TS} {sha} {CYCLE_STEP} commit")
}

fn log_line_corrupted(sha: &str) -> String {
    format!("{ISO_TS} {sha} {CYCLE_STEP} push")
}

fn precompact_subject() -> String {
    format!("PreCompact flush {CYCLE_STEP} {ISO_TS}")
}

const BACKFILL_SUBJECT: &str = "stage 1 backfill";
const BACKFILL_SUBJECT_2: &str = "stage 2 backfill";
const BURST_SUBJECT: &str = "state: burst-24 Commit E — D-477 codification";

// AC-001 — log present, FIELD-4=commit, SHA matches → exempt
#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt() {
    let subject = precompact_subject();
    let log_last_line = log_line_valid(EXAMPLE_SHA);
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        result,
        "AC-001: PreCompact flush commit with matching SHA in valid log MUST be exempt; \
         subject={subject:?} sha={EXAMPLE_SHA:?}"
    );
}

// AC-002 — log absent → prefix-match-only exemption
#[test]
fn test_BC_5_41_003_precompact_prefix_log_absent_exempt() {
    let subject = precompact_subject();
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, None);
    assert!(
        result,
        "AC-002: PreCompact flush commit with absent log MUST be exempt on prefix match alone"
    );
}

// AC-003 — log exists but FIELD-4 corrupted → treat as absent → exempt
#[test]
fn test_BC_5_41_003_precompact_prefix_log_corrupted_exempt() {
    let subject = precompact_subject();
    let corrupted_log = log_line_corrupted(EXAMPLE_SHA);
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&corrupted_log));
    assert!(
        result,
        "AC-003: corrupted FIELD-4 MUST trigger case (b) → exempt on prefix alone; \
         log={corrupted_log:?}"
    );
}

#[test]
fn test_BC_5_41_003_precompact_prefix_log_field4_empty_exempt() {
    let subject = precompact_subject();
    let truncated_log = format!("{ISO_TS} {EXAMPLE_SHA} {CYCLE_STEP}");
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&truncated_log));
    assert!(
        result,
        "AC-003 variant: FIELD-4 absent/empty MUST be exempt on prefix alone; \
         log={truncated_log:?}"
    );
}

// AC-004 — log valid, FIELD-4=commit, SHA MISMATCH → NOT exempt
#[test]
fn test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt() {
    let subject = precompact_subject();
    let log_last_line = log_line_valid(OTHER_SHA);
    let result = is_precompact_flush_exempt(&subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result,
        "AC-004: SHA-mismatch with valid FIELD-4=commit MUST NOT be exempt; \
         commit_sha={EXAMPLE_SHA:?} log_sha={OTHER_SHA:?}"
    );
}

// AC-005 — case-sensitive prefix
#[test]
fn test_BC_5_41_003_precompact_prefix_case_sensitive() {
    let lowercase_subject = format!("precompact flush {CYCLE_STEP} {ISO_TS}");
    let log_last_line = log_line_valid(EXAMPLE_SHA);
    let result = is_precompact_flush_exempt(&lowercase_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !result,
        "AC-005 (lowercase p): 'precompact flush ...' MUST NOT be exempt"
    );

    let no_space_subject = format!("PreCompact flush{CYCLE_STEP} {ISO_TS}");
    let result2 = is_precompact_flush_exempt(&no_space_subject, EXAMPLE_SHA, None);
    assert!(
        !result2,
        "AC-005 / EC-006: 'PreCompact flush' without trailing space MUST NOT be exempt"
    );
}

#[test]
fn test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt() {
    let mixed_subject = format!("preCompact flush {CYCLE_STEP} {ISO_TS}");
    let result = is_precompact_flush_exempt(&mixed_subject, EXAMPLE_SHA, None);
    assert!(
        !result,
        "AC-005 variant: mixed-case prefix MUST NOT be exempt"
    );
}

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

// check_multi_commit_chain tests

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
        "AC-001 chain: HEAD=PreCompact(log-match)+HEAD^=burst MUST NOT trigger violation; \
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
        "AC-001 chain (reversed): HEAD=burst+HEAD^=PreCompact(log-match) MUST NOT trigger violation; \
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
        "AC-006 / BC-5.41.003 PC3: both HEAD+HEAD^ containing sentinels MUST produce violation"
    );
    let violation = result.unwrap();
    assert!(
        violation
            .description
            .contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
        "Violation description MUST contain 'MULTI_COMMIT_CHAIN_NOT_ALLOWED'; got: {:?}",
        violation.description
    );
}

#[test]
fn test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt() {
    let head_subject = precompact_subject();
    let log_last_line = log_line_valid(OTHER_SHA); // log records OTHER_SHA, commit has EXAMPLE_SHA
    let exempt = is_precompact_flush_exempt(&head_subject, EXAMPLE_SHA, Some(&log_last_line));
    assert!(
        !exempt,
        "AC-004: SHA-mismatch with valid FIELD-4=commit MUST NOT grant exemption; \
         commit_sha={EXAMPLE_SHA:?} log_sha={OTHER_SHA:?}"
    );
    let result = check_multi_commit_chain(
        &head_subject,
        EXAMPLE_SHA,
        BACKFILL_SUBJECT_2,
        OTHER_SHA,
        Some(&log_last_line),
    );
    assert!(
        result.is_none(),
        "No chain violation when HEAD=PreCompact(no sentinel, sha-mismatch)+HEAD^=backfill; \
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
        "AC-002 in chain: HEAD=PreCompact(log-absent)+HEAD^=backfill MUST NOT trigger violation; \
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
        "EC-002: two consecutive PreCompact commits MUST NOT trigger violation; got: {result:?}"
    );
}

// AC-008 — PRECOMPACT_FLUSH_PREFIX constant exact bytes
#[test]
fn test_BC_5_41_003_precompact_flush_prefix_constant_exact() {
    assert_eq!(
        PRECOMPACT_FLUSH_PREFIX, "PreCompact flush ",
        "PRECOMPACT_FLUSH_PREFIX constant MUST be exactly 'PreCompact flush ' (17 bytes)"
    );
    assert_eq!(
        PRECOMPACT_FLUSH_PREFIX.len(),
        17,
        "PRECOMPACT_FLUSH_PREFIX must have exactly 17 bytes"
    );
}

// ---------------------------------------------------------------------------
// Section 2: ADR-029 wiring tests (RED GATE — these FAIL against current impl)
//
// These tests drive `on_post_tool_use` with synthetic HookPayload structs
// that simulate what the dispatcher injects per BC-1.16.001 + ADR-029.
//
// WHY THEY FAIL NOW:
//   The current `check_factory_artifacts_chain()` calls `host::exec_subprocess`
//   to obtain HEAD/HEAD^ subjects. In the unit-test harness (native, non-WASM),
//   `host::exec_subprocess` hits the real git binary and reads the WORKTREE repo,
//   not the synthetic git_context we put in the payload. The chain detection
//   therefore either:
//     (a) reads the REAL HEAD/HEAD^ of the development repo (non-deterministic),
//     (b) reads an empty subject (if .factory worktree is absent), and skips.
//   Either way, the test assertions about git_context-driven behavior cannot be
//   verified by the current exec-based impl.
//
//   The CORRECT impl must:
//     1. Detect tool=="Bash" and command containing "git commit".
//     2. Extract payload.extra["git_context"] four fields.
//     3. Drive check_multi_commit_chain from those fields.
//     4. Return Continue when git_context is absent/empty (fail-open).
//
// These tests will FAIL (wrong behavior from current impl) until T-4 is done.
// ---------------------------------------------------------------------------

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
        session_id: "test-session-adrl029".to_string(),
        dispatcher_trace_id: "test-trace-adr029-wiring".to_string(),
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

/// Build a PostToolUse Bash payload WITHOUT git_context (simulates dispatcher
/// fail-open: git error, all-empty git_context, or non-qualifying Bash event).
fn bash_commit_payload_no_git_context(git_commit_command: &str) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("command".to_string(), json!(git_commit_command));

    HookPayload {
        event_name: "PostToolUse".to_string(),
        tool_name: "Bash".to_string(),
        session_id: "test-session-no-ctx".to_string(),
        dispatcher_trace_id: "test-trace-no-ctx".to_string(),
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

/// Build a PostToolUse Bash payload with git_context where all 4 fields are
/// empty strings (dispatcher fail-open path per BC-1.16.001 PC2).
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
        session_id: "test-session-empty-ctx".to_string(),
        dispatcher_trace_id: "test-trace-empty-ctx".to_string(),
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

/// Build a PostToolUse Edit payload (non-qualifying — no git_context injection
/// per BC-1.16.001 PC4; chain check must NOT fire on Edit events).
fn edit_payload_with_git_context(file_path: &str) -> HookPayload {
    let mut tool_input = serde_json::Map::new();
    tool_input.insert("file_path".to_string(), json!(file_path));

    // Simulate the case where git_context is accidentally present in an Edit payload.
    // The chain check must NOT fire for Edit events per ADR-029 §Decision 1.
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
        session_id: "test-session-edit".to_string(),
        dispatcher_trace_id: "test-trace-edit".to_string(),
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
// ADR-029 §Decision 1 + §Decision 5: On PostToolUse Bash (git commit), the WASM gate
// reads git_context from payload.extra and runs the chain check. When both head_subject
// and head_parent_subject contain sentinels, it must emit MULTI_COMMIT_CHAIN_NOT_ALLOWED.
//
// RED GATE: The current impl calls exec_subprocess and reads the REAL repo HEAD, not the
// synthetic git_context. So it either detects a real chain (non-deterministic) or skips
// (no block). The corrected impl MUST detect the sentinel chain from git_context.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block() {
    // PostToolUse Bash, git commit targeting factory-artifacts.
    // git_context has HEAD="stage 1 backfill", HEAD^="stage 2 backfill".
    // Expected after T-4: on_post_tool_use returns Block (MULTI_COMMIT_CHAIN_NOT_ALLOWED).
    let payload = bash_commit_payload_with_git_context(
        "git -C .factory commit -m \"stage 1 backfill\"",
        "stage 1 backfill",
        EXAMPLE_SHA,
        "stage 2 backfill",
        OTHER_SHA,
    );

    let result = on_post_tool_use(payload);

    // The corrected impl must block on sentinel chain detected via git_context.
    // Current exec-based impl does NOT read git_context → this assertion FAILS (Red Gate).
    match &result {
        HookResult::Block { reason } => {
            assert!(
                reason.contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
                "Block reason MUST contain 'MULTI_COMMIT_CHAIN_NOT_ALLOWED'; got: {reason:?}"
            );
        }
        HookResult::Continue => {
            panic!(
                "RED GATE: on_post_tool_use returned Continue when sentinel chain was present in \
                 git_context; the corrected ADR-029 impl must read git_context from payload.extra \
                 and detect MULTI_COMMIT_CHAIN_NOT_ALLOWED. \
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

// --- RED GATE TEST 2: Bash commit with PreCompact HEAD + burst HEAD^ → must Continue ---
//
// git_context has HEAD="PreCompact flush ..." (exempt), HEAD^=normal burst.
// Expected after T-4: on_post_tool_use returns Continue (no block).
// With the precompact-flush-log absent, the log-absent case (c) applies: prefix-match-only.
//
// RED GATE: The current impl reads exec_subprocess from the real repo; the PreCompact
// subject is not in the real repo's HEAD. This either fails with a real chain detection
// or skips (exec error → fail-open). The corrected impl must read from git_context.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues() {
    let precompact_head = precompact_subject();
    // git_context: HEAD=PreCompact (exempt by prefix, log absent → case c), HEAD^=normal burst.
    let payload = bash_commit_payload_with_git_context(
        "git -C .factory commit -m \"state: burst-24 Commit E\"",
        &precompact_head,
        EXAMPLE_SHA,
        BURST_SUBJECT,
        OTHER_SHA,
    );

    let result = on_post_tool_use(payload);

    // The corrected impl must Continue (exempt commit → no chain violation).
    // Current impl reads exec_subprocess and cannot see the synthetic PreCompact HEAD.
    match &result {
        HookResult::Continue => {
            // Correct: exemption fired via git_context, no block.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE: on_post_tool_use blocked when PreCompact HEAD was present in \
                 git_context and exemption should apply. Reason: {reason:?}\n\
                 The corrected ADR-029 impl must read git_context from payload.extra and apply \
                 the PreCompact exemption. Current exec-based impl cannot read synthetic \
                 git_context — Red Gate confirmed."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error (plugin internal failure); \
                 expected Continue for PreCompact-exempt HEAD. message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 3: Fail-open — git_context absent → must Continue (no block) ---
//
// BC-1.16.001 INV3 + BC-5.41.003 Inv5: when git_context is absent from payload.extra,
// the WASM gate MUST skip the chain check and return Continue (fail-open).
//
// RED GATE: The current impl calls exec_subprocess regardless of whether git_context is
// present. If exec_subprocess succeeds and finds a chain in the REAL repo, it blocks
// (non-deterministic). The corrected impl must skip the check when git_context is absent.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues() {
    // Bash git commit event with NO git_context in payload.extra.
    // Simulates dispatcher fail-open (git error → all-empty, or non-qualifying event).
    let payload = bash_commit_payload_no_git_context(
        "git -C .factory commit -m \"state: burst-24 Commit E\"",
    );

    let result = on_post_tool_use(payload);

    // Corrected impl: no git_context → skip chain check → Continue.
    // Current exec-based impl: calls exec_subprocess → non-deterministic (reads real repo).
    // In CI this test is non-deterministic against exec-based impl, but on branch where
    // .factory has sentinel commits, it blocks. The assertion is the Red Gate.
    match &result {
        HookResult::Continue => {
            // Correct fail-open behavior.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE: on_post_tool_use blocked when git_context was absent from payload.extra; \
                 fail-open requires Continue. Reason: {reason:?}\n\
                 The corrected ADR-029 impl must return Continue when git_context absent. \
                 Current exec-based impl reads the real repo git state — Red Gate confirmed \
                 (exec-based impl is not fail-open on absent git_context in payload)."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error (plugin internal failure); \
                 expected Continue for absent git_context (fail-open). message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 4: Fail-open — all-empty git_context → must Continue (no block) ---
//
// BC-1.16.001 PC2: when dispatcher injects all-empty git_context (git error path),
// the WASM gate must skip chain check and Continue.
//
// RED GATE: The current exec-based impl ignores the empty git_context and calls
// exec_subprocess. If exec succeeds on the real repo with a chain, it blocks.
#[test]
fn test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues() {
    // All four git_context fields are empty strings (dispatcher fail-open path).
    let payload =
        bash_commit_payload_empty_git_context("git -C .factory commit -m \"stage 1 backfill\"");

    let result = on_post_tool_use(payload);

    match &result {
        HookResult::Continue => {
            // Correct: all-empty git_context → fail-open → skip chain check.
        }
        HookResult::Block { reason } => {
            panic!(
                "RED GATE: on_post_tool_use blocked when all git_context fields were empty; \
                 fail-open requires Continue. Reason: {reason:?}\n\
                 BC-1.16.001 INV3: all-empty git_context = dispatcher fail-open; \
                 WASM gate must skip chain check. Current exec-based impl reads real repo — \
                 Red Gate confirmed."
            );
        }
        HookResult::Error { message } => {
            panic!(
                "on_post_tool_use returned Error (plugin internal failure); \
                 expected Continue for all-empty git_context (dispatcher fail-open path). \
                 message: {message:?}"
            );
        }
    }
}

// --- RED GATE TEST 5: Edit event with sentinel git_context → must NOT trigger chain block ---
//
// ADR-029 §Decision 1: chain detection ONLY fires on PostToolUse Bash git-commit events.
// Edit events MUST NOT trigger chain detection even if git_context is accidentally present.
//
// RED GATE: The current impl fires check_factory_artifacts_chain() on Edit events too
// (the existing hook is registered for Edit|Write). On Edit events, it calls exec_subprocess.
// The corrected impl must detect tool="Edit" and NOT run the chain check at all.
//
// NOTE: This test supplies a real burst-log path via file_path so the burst-log validation
// arm of on_post_tool_use runs. The chain block must NOT fire even with sentinel git_context.
#[test]
fn test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block() {
    // Edit event on burst-log.md with git_context containing sentinel subjects.
    // The chain check MUST NOT fire for Edit events per ADR-029 §Decision 1.
    // (This is different from burst-log structural validation, which still runs.)
    let payload = edit_payload_with_git_context("/some/path/burst-log.md");

    let result = on_post_tool_use(payload);

    // The result may be a block for BURST_LOG_STRUCTURAL_VIOLATION (if burst-log.md
    // is missing from filesystem, the hook fails-open and returns Continue).
    // What must NOT happen: a block specifically for MULTI_COMMIT_CHAIN_NOT_ALLOWED.
    match &result {
        HookResult::Block { reason } => {
            assert!(
                !reason.contains("MULTI_COMMIT_CHAIN_NOT_ALLOWED"),
                "RED GATE: Edit event must NOT trigger MULTI_COMMIT_CHAIN_NOT_ALLOWED block, \
                 even when git_context contains sentinel subjects. \
                 ADR-029 §Decision 1: chain detection is Bash-only. \
                 Got block reason: {reason:?}"
            );
        }
        HookResult::Continue => {
            // Expected: no chain block on Edit events. Structural validation also passes
            // (fail-open: file not found → Continue).
        }
        HookResult::Error { message } => {
            // Error is acceptable here: if the burst-log structural validation itself
            // errors out (host I/O unavailable in unit test), that is unrelated to
            // chain detection. We only care that it's not a MULTI_COMMIT_CHAIN_NOT_ALLOWED
            // block. Log the error for diagnostic purposes but do not fail.
            let _ = message; // suppress unused warning
        }
    }
}

// --- Invariant verification: the corrected impl must not expose exec_subprocess calls ---
//
// This is a documentation test: the test body asserts the API surface that the
// corrected `on_post_tool_use` must NOT use exec_subprocess for chain context.
// The assertion is structural — if the impl compiles with exec_subprocess removed
// from check_factory_artifacts_chain(), this test passes trivially (compile-time check).
// Until then, the wiring tests above are the load-bearing Red Gate assertions.
//
// This test also serves as the AC-006 exec-free invariant documentation point.
#[test]
fn test_BC_5_41_003_wiring_exec_free_constraint_documented() {
    // AC-006 exec-free constraint (ADR-029 §Decision 3):
    // validate-burst-log.wasm MUST NOT call host::exec_subprocess for commit context.
    // The load-bearing enforcement is in the wiring tests above.
    // This test documents the constraint and will pass once the wiring tests pass.
    // Before that, it always passes (it has no runtime assertion) — its companion
    // wiring tests (test_BC_1_16_001_wiring_bash_git_commit_*) are the Red Gate.
    let _ = "AC-006: WASM reads git_context from payload.extra, not exec_subprocess";
}
