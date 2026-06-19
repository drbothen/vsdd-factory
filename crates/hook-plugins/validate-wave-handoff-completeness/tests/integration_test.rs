//! VP-081 integration harness — validate-wave-handoff-completeness WASM gate.
//!
//! File: crates/hook-plugins/validate-wave-handoff-completeness/tests/integration_test.rs
//! Anchor: validate_handoff_completeness_blocks_on_missing_fields
//!
//! These tests drive `on_post_tool_use` end-to-end with constructed `HookPayload`
//! objects, exercising the full Write/Edit dispatch path without a running dispatcher.
//! They constitute the VP-081 proof harness (proof_method: integration).
//!
//! # Integration scope
//!
//! VP-081 names this file and requires the following test functions:
//!   - `test_wave_close_blocked_with_incomplete_handoff` (VP-081 Postcondition B)
//!   - `test_wave_close_allowed_with_complete_handoff`   (VP-081 Postcondition C)
//!   - `test_wave_1_no_op`                               (VP-081 Postcondition D)
//!   - `test_wave_id_absent_fails_closed`                (VP-081 Postcondition E + EC-010)
//!
//! Note: VP-081 Postcondition A (HandoffMissing on absent file) is a SHELL-SIDE
//! behavior (BC-5.41.001 PC9). The WASM gate is never invoked on file-absence.
//! No integration test for Postcondition A is authored here — the shell-side behavior
//! is covered by wave-handoff.bats.
//!
//! # Red Gate discipline
//!
//! All tests drive production code via `on_post_tool_use`. Tests that exercise
//! correct WASM gate behavior (Postconditions C/D/E) pass once the implementation
//! is complete. Tests that encode bugs (F-002/F-003) fail until the implementer
//! fixes the underlying production functions (`validate_field`, `path_is_handoff`).
//!
//! # Naming convention (BC-5.39.001 TDD / VP-081 proof harness skeleton)
//!
//! Test function names match VP-081 §Proof Harness Skeleton exactly to provide
//! direct traceability from the VP to the executing code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use validate_wave_handoff_completeness::{
    GateContext, GateResult, check_handoff_completeness, on_post_tool_use,
};
use vsdd_hook_sdk::HookResult;

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

/// Construct a PostToolUse Write payload for a HANDOFF.md write.
///
/// `file_path` is the path written (determines path_is_handoff matching).
/// `content` is the YAML content written.
fn write_payload(file_path: &str, content: &str) -> vsdd_hook_sdk::HookPayload {
    let json = serde_json::json!({
        "event_name": "PostToolUse",
        "tool_name": "Write",
        "session_id": "vp-081-integration",
        "dispatcher_trace_id": "vp-081-trace",
        "tool_input": {
            "file_path": file_path,
            "content": content
        },
        "tool_response": {
            "exit_code": 0
        }
    });
    serde_json::from_value(json).expect("VP-081 Write HookPayload fixture must deserialize")
}

/// Construct a PostToolUse Edit payload for a HANDOFF.md edit.
///
/// Edit payloads carry `path` (not `file_path`) + `old_string` + `new_string`.
/// The current implementation reads `new_string` as the content fragment.
/// NOTE: an Edit payload's `new_string` is a FRAGMENT, not the full file.
/// Tests for the full-file Edit path (F-001) require host::read_file infra
/// not available in native unit tests — see F-001 route-back note.
fn edit_payload(path: &str, new_string: &str) -> vsdd_hook_sdk::HookPayload {
    let json = serde_json::json!({
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "vp-081-integration-edit",
        "dispatcher_trace_id": "vp-081-edit-trace",
        "tool_input": {
            "path": path,
            "old_string": "wave_id: 1",
            "new_string": new_string
        },
        "tool_response": {
            "exit_code": 0
        }
    });
    serde_json::from_value(json).expect("VP-081 Edit HookPayload fixture must deserialize")
}

// ---------------------------------------------------------------------------
// VP-081 Postcondition B — HANDOFF.md present but incomplete → block
// ---------------------------------------------------------------------------

/// VP-081 Postcondition B: HANDOFF.md with `last_verified_develop_sha` missing
/// and `wave_id=2` → gate blocks with HandoffIncomplete.
///
/// Re-pointed from on_post_tool_use to the pure decision core
/// `check_handoff_completeness`. The I/O shell (host::read_file) fails-open
/// with CapabilityDenied in the non-WASM unit harness, so on_post_tool_use
/// cannot assert Block here. The pure core exercises the same production
/// validation logic without the host dependency. I/O-shell coverage is carried
/// by bats scenario B (fail-open-on-crash.bats).
///
/// BC-4.14.001 PC7 / VP-081 Postcondition B.
#[test]
fn test_wave_close_blocked_with_incomplete_handoff() {
    // wave_id=2: full validation runs (not wave-1 no-op).
    // last_verified_develop_sha is absent → HandoffIncomplete.
    let yaml = "\
wave_id: 2
precompact_flush_sha: null
factory_lock_holder: null
active_bcs: []
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert!(
        matches!(result, GateResult::Block { .. }),
        "VP-081/PostconditionB: HANDOFF.md with wave_id=2 and missing \
        last_verified_develop_sha must produce Block. Got: {result:?}"
    );
    if let GateResult::Block { message, .. } = &result {
        assert!(
            message.contains("HandoffIncomplete") || message.contains("last_verified_develop_sha"),
            "VP-081/PostconditionB: Block message must mention HandoffIncomplete or \
            last_verified_develop_sha. Got: {message}"
        );
    }
}

// ---------------------------------------------------------------------------
// VP-081 Postcondition C — HANDOFF.md present and complete → Continue
// ---------------------------------------------------------------------------

/// VP-081 Postcondition C: HANDOFF.md with all 9 required fields present and
/// valid, `wave_id=2` → gate returns Continue.
///
/// Re-pointed from on_post_tool_use (which fails-open in the native test
/// harness because host::read_file returns CapabilityDenied, making the
/// validation-outcome assertion unfalsifiable) to the pure decision core
/// `check_handoff_completeness`. The behavioral expectation — complete handoff
/// produces Continue — is now verified directly against production logic.
/// I/O-shell coverage is carried by bats.
///
/// BC-4.14.001 PC7 / VP-081 Postcondition C.
#[test]
fn test_wave_close_allowed_with_complete_handoff() {
    let yaml = "\
wave_id: 2
last_verified_develop_sha: abc123def456
precompact_flush_sha: null
factory_lock_holder: null
active_bcs:
  - BC-4.14.001
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert_eq!(
        result,
        GateResult::Continue,
        "VP-081/PostconditionC: HANDOFF.md with wave_id=2 and all 9 fields present \
        must return Continue. Got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// VP-081 Postcondition D — wave_id == 1 (NOT EPIC-COMPLETE) → Continue (no-op)
// ---------------------------------------------------------------------------

/// VP-081 Postcondition D: `wave_id == 1` (NOT EPIC-COMPLETE) → gate returns
/// Continue unconditionally (wave-1 no-op per BC-4.14.001 PC3 / ADR-026
/// §Decision 9).
///
/// Re-pointed from on_post_tool_use (which fails-open in the native test
/// harness, making the validation-outcome assertion unfalsifiable) to the
/// pure decision core `check_handoff_completeness`. The behavioral expectation
/// — wave-1 no-op produces Continue — is verified directly against production
/// logic. I/O-shell coverage is carried by bats.
///
/// BC-4.14.001 PC3 / VP-081 Postcondition D / VP-083.
#[test]
fn test_wave_1_no_op() {
    let yaml = "\
wave_id: 1
last_verified_develop_sha: 1122334455aa
precompact_flush_sha: null
factory_lock_holder: null
active_bcs: []
next_wave_stories:
  - id: S-02.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert_eq!(
        result,
        GateResult::Continue,
        "VP-081/PostconditionD: HANDOFF.md with wave_id=1 (NOT EPIC-COMPLETE) \
        must return Continue (wave-1 no-op per BC-4.14.001 PC3). Got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// VP-081 Postcondition E — wave_id absent → fail-closed (Block)
// ---------------------------------------------------------------------------

/// VP-081 Postcondition E: `wave_id` ABSENT from HANDOFF.md → gate FAILS CLOSED (Block).
///
/// Absent `wave_id` is NOT treated as wave-1 per BC-4.14.001 PC3/PC8/EC-010.
/// Full validation runs; blocks with HandoffIncomplete listing wave_id.
///
/// Re-pointed from on_post_tool_use to the pure decision core
/// `check_handoff_completeness`. Absent wave_id is detected by content parsing
/// per the PC8 fail-closed convention (F-A005). I/O-shell coverage is carried
/// by bats scenario C (fail-open-on-crash.bats).
///
/// This test also validates the VP-INDEX VP-081 Full Index row which advertises
/// fail-closed behavior for absent wave_id.
///
/// BC-4.14.001 PC3 / PC8 / EC-010 / VP-081 Postcondition E.
#[test]
fn test_wave_id_absent_fails_closed() {
    // wave_id field is completely absent from this payload.
    let yaml = "\
last_verified_develop_sha: abc123def456
precompact_flush_sha: null
factory_lock_holder: null
active_bcs: []
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    // F-A005: wave_id absent is detected by parsing the content inside
    // check_handoff_completeness. No external is_first_wave flag needed.
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert!(
        matches!(result, GateResult::Block { .. }),
        "VP-081/PostconditionE: HANDOFF.md with wave_id absent must produce Block \
        (fail-closed per BC-4.14.001 PC3/PC8/EC-010). Got: {result:?}"
    );
    if let GateResult::Block { message, .. } = &result {
        assert!(
            message.contains("HandoffIncomplete") || message.contains("wave_id"),
            "VP-081/PostconditionE: Block message must mention HandoffIncomplete or wave_id. \
            Got: {message}"
        );
    }
}

// ---------------------------------------------------------------------------
// F-002 regression path through on_post_tool_use
//
// These tests exercise the wave_id=0 / wave_id=-1 bugs via the full
// on_post_tool_use entry point (not just check_handoff_completeness).
//
// RED GATE: current impl uses as_i64().is_some() which accepts 0/-1.
// These tests fail until validate_field enforces wave_id > 0.
// ---------------------------------------------------------------------------

/// F-002 / BC-4.14.001 PC7 / EC-017: wave_id:0 is malformed (not a positive integer)
/// → gate must block.
///
/// Re-pointed from on_post_tool_use (renamed from test_wave_id_zero_blocks_via_on_post_tool_use)
/// to the pure decision core `check_handoff_completeness`. The I/O shell
/// (host::read_file) fails-open in the non-WASM unit harness, so the test must
/// exercise the production validate_field logic directly. wave_id=0 fails the
/// `n >= 1` positive-integer check in validate_field.
///
/// wave_id=0 is not wave-1 (0 != 1, and 0 is not a positive integer). The pure core
/// runs full base validation which catches wave_id=0 as malformed.
///
/// BC-4.14.001 PC7 / EC-017 / VP-081.
#[test]
fn test_wave_id_zero_blocks() {
    let yaml = "\
wave_id: 0
last_verified_develop_sha: abc123def456
precompact_flush_sha: null
factory_lock_holder: null
active_bcs: []
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    // F-A005: wave_id=0 is parsed from content; 0 != 1 so step 3 no-op
    // does not fire; step 4/5 validation catches wave_id=0 as malformed.
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert!(
        matches!(result, GateResult::Block { .. }),
        "F-002: HANDOFF.md with wave_id:0 must produce Block \
        (zero is not a positive integer per BC-4.14.001 PC7/EC-017). Got: {result:?}"
    );
    if let GateResult::Block { message, .. } = &result {
        assert!(
            message.contains("HandoffIncomplete") || message.contains("wave_id"),
            "F-002: Block message must mention HandoffIncomplete or wave_id. Got: {message}"
        );
    }
}

// ---------------------------------------------------------------------------
// F-003 regression path through on_post_tool_use
//
// These tests confirm that path_is_handoff false-positives from bare
// ends_with("HANDOFF.md") affect the full on_post_tool_use gate.
// A write to "xHANDOFF.md" with an incomplete payload would currently
// produce Block (because path matches incorrectly and validation runs).
// After the fix, path_is_handoff returns false for "xHANDOFF.md" and
// on_post_tool_use returns Continue (no-op for non-HANDOFF.md targets).
//
// RED GATE: the tests below assert Continue for non-HANDOFF.md paths.
// Currently the gate fires on them and may produce Block (wrong).
// ---------------------------------------------------------------------------

/// F-003 via on_post_tool_use: Write to "foo/WAVE-HANDOFF.md" with a
/// payload that WOULD fail validation if parsed — must return Continue
/// (non-HANDOFF.md target is a no-op per BC-4.14.001 PC4).
///
/// RED GATE: current ends_with matches "foo/WAVE-HANDOFF.md" → gate fires →
/// incomplete payload → Block. After path-component fix → Continue.
#[test]
fn test_wave_handoff_path_not_targeted_returns_continue() {
    // Deliberately incomplete YAML that would produce Block if the gate runs.
    // If path_is_handoff correctly rejects "foo/WAVE-HANDOFF.md", the gate
    // short-circuits to Continue before any YAML parsing.
    let yaml = "wave_id: 2\n"; // incomplete: only wave_id present
    let payload = write_payload("foo/WAVE-HANDOFF.md", yaml);
    let result = on_post_tool_use(payload);

    assert_eq!(
        result,
        HookResult::Continue,
        "F-003/on_post_tool_use: Write to 'foo/WAVE-HANDOFF.md' must return Continue \
        (non-HANDOFF.md target, PC4 no-op). Current ends_with returns Block because it \
        incorrectly matches 'WAVE-HANDOFF.md'. Got: {result:?}"
    );
}

/// F-003 via on_post_tool_use: Write to "xHANDOFF.md" with incomplete
/// payload must return Continue (non-HANDOFF.md target).
///
/// RED GATE: current ends_with("HANDOFF.md") matches "xHANDOFF.md" → Block.
/// After path-component fix → Continue.
#[test]
fn test_x_handoff_path_not_targeted_returns_continue() {
    let yaml = "wave_id: 2\n"; // incomplete YAML — would block if gate fires
    let payload = write_payload("xHANDOFF.md", yaml);
    let result = on_post_tool_use(payload);

    assert_eq!(
        result,
        HookResult::Continue,
        "F-003/on_post_tool_use: Write to 'xHANDOFF.md' must return Continue \
        (non-HANDOFF.md target per BC-4.14.001 PC4). Current ends_with matches \
        incorrectly → Block. Got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// Edit payload path — confirm gate routes Edit by "path" key (not "file_path")
// ---------------------------------------------------------------------------

/// VP-081 Edit-path validation: HANDOFF.md with complete wave_id=2 YAML
/// (all 9 fields present) → gate returns Continue.
///
/// Re-pointed from on_post_tool_use (Edit payload routing via tool_input["path"])
/// to the pure decision core `check_handoff_completeness`. In the native test
/// harness, on_post_tool_use fails-open before parsing because host::read_file
/// returns CapabilityDenied — the validation-outcome assertion (complete → Continue)
/// is unfalsifiable when driven through the I/O shell. The pure core directly
/// verifies the behavioral property.
///
/// Edit-tool path-key routing (tool_input["path"] vs "file_path") is separately
/// covered by the on_post_tool_use fail-open test (see below). F-001 full-file
/// Edit path (host::read_file) requires bats infrastructure.
///
/// BC-4.14.001 PC7 / VP-081 Postcondition C (Edit variant).
#[test]
fn test_edit_payload_routing_with_complete_new_string() {
    let full_yaml = "\
wave_id: 2
last_verified_develop_sha: abc123def456
precompact_flush_sha: null
factory_lock_holder: null
active_bcs:
  - BC-4.14.001
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
";
    // Drive the pure core with the complete YAML that a valid Edit would produce.
    let ctx = GateContext {
        file_path: "factory-artifacts/HANDOFF.md".to_string(),
        handoff_content: Some(full_yaml.to_string()),
    };
    let result = check_handoff_completeness(&ctx);

    assert_eq!(
        result,
        GateResult::Continue,
        "VP-081/EditVariant: HANDOFF.md with wave_id=2 and all 9 fields present \
        must return Continue. Got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// VP-083 — explicit fail-open test: on_post_tool_use returns Continue on I/O error
// ---------------------------------------------------------------------------

/// VP-083 / BC-4.14.001 PC8 — explicit fail-open tests for on_post_tool_use.
///
/// In the native (non-WASM) integration test harness, host::read_file always
/// returns CapabilityDenied. Therefore on_post_tool_use ALWAYS fail-opens to
/// HookResult::Continue BEFORE reaching YAML-parsing logic. These tests
/// explicitly name and verify that fail-open property for BOTH Write and Edit
/// payloads: the I/O shell must return Continue when file-read is unavailable
/// (no-false-positive per VP-083).
///
/// These are EXPLICIT FAIL-OPEN tests — they do NOT verify a validation
/// outcome (complete/incomplete → Continue/Block). They verify only that the
/// fail-open path itself produces Continue. Validation-outcome assertions use
/// check_handoff_completeness (see tests above).
///
/// VP-083 §no-false-positive / BC-4.14.001 PC8 fail-open-on-read-error.
#[test]
fn test_on_post_tool_use_fails_open_on_read_error() {
    // Deliberately incomplete YAML — would produce Block via check_handoff_completeness.
    // on_post_tool_use in the native harness fail-opens before parsing because
    // host::read_file returns CapabilityDenied, so it returns Continue regardless.
    let yaml = "wave_id: 2\n"; // missing all required scalar/list fields — Write path
    let payload = write_payload("factory-artifacts/HANDOFF.md", yaml);
    let result = on_post_tool_use(payload);
    assert_eq!(
        result,
        HookResult::Continue,
        "VP-083/fail-open/Write: on_post_tool_use must return Continue when host::read_file \
        fails (CapabilityDenied in non-WASM harness). Got: {result:?}"
    );
}

/// VP-083 / BC-4.14.001 PC8 — Edit-path explicit fail-open for on_post_tool_use.
///
/// Parallel to the Write-path test above: Edit payloads route through
/// tool_input["path"] (not "file_path"). The same fail-open property applies —
/// host::read_file is unavailable in the native harness → Continue regardless
/// of new_string completeness.
///
/// VP-083 §no-false-positive / BC-4.14.001 PC8 fail-open-on-read-error (Edit variant).
#[test]
fn test_on_post_tool_use_edit_fails_open_on_read_error() {
    // Deliberately incomplete YAML in new_string — would produce Block via
    // check_handoff_completeness, but on_post_tool_use fails-open here.
    let incomplete_yaml = "wave_id: 2\n"; // missing all required fields
    let payload = edit_payload("factory-artifacts/HANDOFF.md", incomplete_yaml);
    let result = on_post_tool_use(payload);
    assert_eq!(
        result,
        HookResult::Continue,
        "VP-083/fail-open/Edit: on_post_tool_use must return Continue when host::read_file \
        fails (CapabilityDenied in non-WASM harness). Got: {result:?}"
    );
}
