//! Unit tests for validate-heavy-op-delegation (S-18.06).
//!
//! Every test in this file maps to an acceptance criterion in S-18.06 and a
//! postcondition or invariant in BC-4.15.001. Tests exercise `evaluate_patterns`
//! and `truncate_command_preview` directly (pure-core functions; no WASM runtime
//! required). All tests are GREEN — implementation is complete (S-18.06 T-3).
//!
//! Dispatcher-integration tests for `on_pre_tool_use` (including config-reading
//! via `plugin_config["patterns"]` per F-P1-001 fix) live in the bats suite:
//! `plugins/vsdd-factory/tests/validate-heavy-op-delegation.bats`.
//!
//! # Test Table (S-18.06 v1.5 — unit.rs subset)
//!
//! | Test name | AC | BC clause | State |
//! |-----------|----|-----------|-|
//! | `test_heavy_op_gate_first_match_semantics_single_advisory` | AC-005 | INV3 | GREEN |
//! | `test_heavy_op_gate_truncates_command_preview_at_120_chars` | AC-006 | INV4 | GREEN |
//! | `test_heavy_op_gate_no_truncation_on_short_command` | AC-006 | INV4 | GREEN |
//! | `test_heavy_op_gate_empty_pattern_list_no_emission` | AC-011 | EC-012 | GREEN |
//! | `test_heavy_op_gate_always_returns_continue_on_match` | AC-004 | INV2 | GREEN |
//! | `test_heavy_op_gate_ec002_cargo_test_workspace_no_match` | EC-002 | INV3 | GREEN |
//! | `test_heavy_op_gate_ec004_grep_uppercase_r_matches` | EC-004 | INV3 | GREEN |
//! | `test_heavy_op_gate_ec005_find_name_matches` | EC-005 | INV3 | GREEN |
//! | `test_heavy_op_gate_ec006_run_all_sh_matches` | EC-006 | INV3 | GREEN |
//! | `test_heavy_op_gate_ec013_custom_pattern_triggers_advisory` | EC-013 | INV3 | GREEN |

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use validate_heavy_op_delegation::{
    COMMAND_PREVIEW_MAX_CHARS, DEFAULT_PATTERNS, ELLIPSIS, GateResult, evaluate_patterns,
    truncate_command_preview,
};

// ---------------------------------------------------------------------------
// AC-005 — First-match semantics; exactly one advisory per invocation (INV3)
// ---------------------------------------------------------------------------

/// AC-005 / INV3: when multiple patterns from the configured list would match
/// the same command string, the gate stops at the FIRST matching pattern and
/// emits exactly ONE `DelegationRecommended` advisory. Subsequent patterns are
/// not evaluated after a match.
///
/// Setup: Bash command `grep -r TODO . | grep -R FIXME`. Both `grep -r` and
/// `grep -R` are in the default pattern list. `grep -r` appears BEFORE `grep -R`
/// in the list declaration order.
///
/// Assert: `evaluate_patterns` returns `GateResult::Advisory` (not Continue);
/// `matched_pattern` is `"grep -r"` (the first match in list order, not `"grep -R"`).
///
/// Asserts first-match single-advisory semantics (AC-005 / INV3).
#[test]
fn test_heavy_op_gate_first_match_semantics_single_advisory() {
    let command = "grep -r TODO . | grep -R FIXME";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    // Must produce an advisory (first match found).
    assert!(
        matches!(result, GateResult::Advisory(_)),
        "AC-005/INV3: command matching multiple patterns must produce Advisory (first-match), \
        got: {result:?}"
    );

    // The matched_pattern must be "grep -r" (first in list order, not "grep -R").
    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "grep -r",
            "AC-005/INV3: matched_pattern must be \"grep -r\" (first match in list order); \
            \"grep -R\" appears later in the list and must NOT be the first match. \
            Got: {:?}",
            advisory.matched_pattern
        );
    }
}

// ---------------------------------------------------------------------------
// AC-006 — command_preview ≤120-char truncation invariant (INV4)
// ---------------------------------------------------------------------------

/// AC-006 / INV4: when a Bash command exceeds 120 Unicode code points, the
/// `command_preview` MUST be the first 120 code points followed by U+2026
/// (ELLIPSIS), yielding a (COMMAND_PREVIEW_MAX_CHARS + 1)-code-point string.
///
/// Setup: Bash command of 200 characters containing `grep -r` (to ensure a
/// match is found when evaluated, but here we test `truncate_command_preview`
/// directly).
///
/// Assert: `truncate_command_preview` returns a string of exactly 121 code
/// points; the final code point is U+2026; the first 120 code points match the
/// first 120 code points of the input.
///
/// Asserts 120-char truncation with U+2026 ellipsis (AC-006 / INV4).
#[test]
fn test_heavy_op_gate_truncates_command_preview_at_120_chars() {
    // Build a 200-char command string containing "grep -r".
    let base = "grep -r TODO ";
    let padding: String = "x".repeat(200 - base.len());
    let command = format!("{base}{padding}");
    assert_eq!(
        command.chars().count(),
        200,
        "test fixture must be exactly 200 code points"
    );

    let preview = truncate_command_preview(&command);

    // Assert: exactly COMMAND_PREVIEW_MAX_CHARS + 1 code points (120 + ellipsis).
    let preview_len = preview.chars().count();
    assert_eq!(
        preview_len,
        COMMAND_PREVIEW_MAX_CHARS + 1,
        "AC-006/INV4: preview of a 200-char command must be exactly {} code points \
        ({} chars + U+2026), got {} code points. Preview: {:?}",
        COMMAND_PREVIEW_MAX_CHARS + 1,
        COMMAND_PREVIEW_MAX_CHARS,
        preview_len,
        preview
    );

    // Assert: final code point is U+2026.
    let last_char = preview.chars().last().expect("preview must be non-empty");
    assert_eq!(
        last_char, ELLIPSIS,
        "AC-006/INV4: final code point of truncated preview must be U+2026 (…), \
        got: {:?}",
        last_char
    );

    // Assert: first COMMAND_PREVIEW_MAX_CHARS code points match the input.
    let expected_prefix: String = command.chars().take(COMMAND_PREVIEW_MAX_CHARS).collect();
    let actual_prefix: String = preview.chars().take(COMMAND_PREVIEW_MAX_CHARS).collect();
    assert_eq!(
        actual_prefix, expected_prefix,
        "AC-006/INV4: first {} code points of preview must match the input command",
        COMMAND_PREVIEW_MAX_CHARS
    );
}

/// AC-006 / INV4: when a Bash command is ≤120 Unicode code points, the
/// `command_preview` MUST equal the full command string with NO truncation
/// and NO ellipsis appended.
///
/// Setup: Bash command `grep -r .` (9 characters; well under 120).
///
/// Assert: `truncate_command_preview` returns the full command string unchanged;
/// the return value does NOT end with U+2026.
///
/// Asserts no-truncation for commands within the 120-char limit (AC-006 / INV4).
#[test]
fn test_heavy_op_gate_no_truncation_on_short_command() {
    let command = "grep -r .";
    assert!(
        command.chars().count() <= COMMAND_PREVIEW_MAX_CHARS,
        "test fixture must be ≤{} code points",
        COMMAND_PREVIEW_MAX_CHARS
    );

    let preview = truncate_command_preview(command);

    // Assert: preview equals the full command string (no truncation).
    assert_eq!(
        preview, command,
        "AC-006/INV4: short command preview must equal full command string (no truncation); \
        got: {:?}",
        preview
    );

    // Assert: no ellipsis appended.
    let last_char = preview.chars().last();
    assert_ne!(
        last_char,
        Some(ELLIPSIS),
        "AC-006/INV4: short command preview must NOT end with U+2026 (…); \
        got last char: {:?}",
        last_char
    );
}

// ---------------------------------------------------------------------------
// AC-011 — Empty patterns list: no emission on any Bash command (EC-012)
// ---------------------------------------------------------------------------

/// AC-011 / EC-012: when the patterns list is empty (`patterns = []`), no
/// pattern can ever match. ALL Bash commands must pass silently (PC-A);
/// no advisory emitted for any command.
///
/// Setup: patterns list `[]` (empty); Bash command `cargo test --release
/// --workspace` (would normally match the default pattern list).
///
/// Assert: `evaluate_patterns` returns `GateResult::Continue` (not Advisory).
///
/// Asserts empty-patterns-list produces Continue for any command (AC-011 / EC-012).
#[test]
fn test_heavy_op_gate_empty_pattern_list_no_emission() {
    let command = "cargo test --release --workspace";
    let patterns: &[&str] = &[]; // empty pattern list

    let result = evaluate_patterns(command, patterns);

    assert_eq!(
        result,
        GateResult::Continue,
        "AC-011/EC-012: empty pattern list must return Continue for any command; \
        no advisory must be emitted. Command: {:?}. Got: {result:?}",
        command
    );
}

// ---------------------------------------------------------------------------
// AC-004 — Gate always returns Continue on match (INV2)
// ---------------------------------------------------------------------------

/// AC-004 / INV2: the gate MUST return Continue (block_intent = false) even
/// when a pattern matches. Setting block_intent = true is a specification
/// violation.
///
/// This test drives `evaluate_patterns` directly to verify that a match result
/// is `GateResult::Advisory` (Continue+advisory), NOT a blocking result. The
/// `GateResult` enum has no Block variant — its absence in the type system is
/// the static enforcement of INV2. This test additionally checks that:
/// (a) a match is found (Advisory is returned, not Continue), and
/// (b) the advisory carries the expected matched_pattern (not block_intent).
///
/// Asserts Advisory (never Block) returned on match; GateResult::Block absent (AC-004 / INV2).
#[test]
fn test_heavy_op_gate_always_returns_continue_on_match() {
    // Test vector from BC-4.15.001 §Canonical Test Vectors:
    // "cargo test --release" → Continue + DelegationRecommended advisory.
    let command = "cargo test --release";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    // Must produce Advisory (not Continue — the pattern DOES match).
    // The absence of a GateResult::Block variant statically enforces INV2.
    assert!(
        matches!(result, GateResult::Advisory(_)),
        "AC-004/INV2: 'cargo test --release' must match default patterns and return \
        GateResult::Advisory (Continue+advisory, never Block). Got: {result:?}"
    );

    // The matched_pattern must be "cargo test --release" (exact first match).
    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "cargo test --release",
            "AC-004/INV2: matched_pattern must be \"cargo test --release\". \
            Got: {:?}",
            advisory.matched_pattern
        );

        // command_preview must satisfy the truncation invariant (INV4):
        // "cargo test --release" is well under 120 chars — no truncation expected.
        assert_eq!(
            advisory.command_preview, command,
            "AC-004/INV4: short command preview must equal full command (no truncation). \
            Got: {:?}",
            advisory.command_preview
        );
    }
}

// ---------------------------------------------------------------------------
// Supplementary EC-coverage tests (L-BB-red-gate-test-plan-ec-coverage-parity)
//
// The mandatory EC-coverage-parity discipline requires every testable EC-NNN
// in BC-4.15.001 §Edge Cases to have a corresponding test. The 5 tests below
// cover ECs not addressed by the 5 primary unit tests or the 7 bats tests.
//
// EC-002: cargo test --workspace does NOT match cargo test --release (substring semantics)
// EC-004: grep -R "pattern" . matches grep -R (case-sensitive; distinct from grep -r)
// EC-005: find . -name "*.rs" matches find . -name
// EC-006: ./run-all.sh matches ./run-all.sh (exact substring in list)
// EC-013: custom pattern ./ci.sh matches when added to the list
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// EC-002 — substring non-match: cargo test --workspace does NOT match
//          cargo test --release (BC-4.15.001 EC-002)
// ---------------------------------------------------------------------------

/// EC-002 / BC-4.15.001 EC-002: `cargo test --workspace` does NOT match the
/// pattern `cargo test --release` because `cargo test --release` is NOT a
/// substring of `cargo test --workspace`.
///
/// This tests the correctness of the substring-containment semantics (INV3):
/// pattern P matches command C if `C.contains(P)` — not prefix-match, not
/// word-boundary match. `cargo test --workspace` does not contain the literal
/// substring `cargo test --release`.
///
/// Assert: `evaluate_patterns` returns `GateResult::Continue` (no match; PC-A).
///
/// Asserts substring non-match: `--workspace` ≠ `--release` (EC-002 / INV3).
#[test]
fn test_heavy_op_gate_ec002_cargo_test_workspace_no_match() {
    // BC-4.15.001 EC-002: "Bash command is 'cargo test --workspace'; pattern list
    // contains 'cargo test --release' but NOT 'cargo test --workspace'"
    let command = "cargo test --workspace";

    // Use default patterns (which include "cargo test --release" but not "cargo test --workspace").
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    assert_eq!(
        result,
        GateResult::Continue,
        "EC-002: 'cargo test --workspace' must NOT match pattern 'cargo test --release'. \
        Pattern matching is substring containment: 'cargo test --workspace'.contains('cargo test --release') == false. \
        Got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// EC-004 — grep -R matches grep -R (case-sensitive; distinct from grep -r)
// ---------------------------------------------------------------------------

/// EC-004 / BC-4.15.001 EC-004: `grep -R "pattern" .` matches pattern `grep -R`
/// (case-sensitive match; the capital R is a distinct pattern from lowercase r).
///
/// This tests that the gate correctly distinguishes case-sensitive patterns:
/// `grep -R` and `grep -r` are different patterns and both must be in the list
/// to catch both variants.
///
/// Assert: `evaluate_patterns` returns `GateResult::Advisory` with
/// `matched_pattern = "grep -R"` (NOT "grep -r").
///
/// Asserts case-sensitive match: `grep -R` (uppercase) is distinct from `grep -r` (EC-004 / INV3).
#[test]
fn test_heavy_op_gate_ec004_grep_uppercase_r_matches() {
    // BC-4.15.001 EC-004: "Bash command is 'grep -R \"pattern\" .'; pattern 'grep -R' is in the list"
    let command = r#"grep -R "pattern" ."#;

    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    // Must produce Advisory — grep -R matches the "grep -R" pattern.
    assert!(
        matches!(result, GateResult::Advisory(_)),
        "EC-004: 'grep -R \"pattern\" .' must match pattern 'grep -R' and return Advisory. \
        Got: {result:?}"
    );

    // The matched_pattern must be "grep -R" (capital R, case-sensitive).
    // Note: DEFAULT_PATTERNS has "grep -r" BEFORE "grep -R".
    // "grep -R \"pattern\" ." contains "grep -R" but does it contain "grep -r"?
    // It does NOT — the command uses uppercase -R, not lowercase -r.
    // So the first matching pattern should be "grep -R".
    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "grep -R",
            "EC-004: matched_pattern must be 'grep -R' (capital R) for command 'grep -R ...' \
            (case-sensitive matching; INV3). Got: {:?}",
            advisory.matched_pattern
        );
    }
}

// ---------------------------------------------------------------------------
// EC-005 — find . -name "*.rs" matches find . -name
// ---------------------------------------------------------------------------

/// EC-005 / BC-4.15.001 EC-005: `find . -name "*.rs"` matches pattern `find . -name`.
///
/// Assert: `evaluate_patterns` returns `GateResult::Advisory` with
/// `matched_pattern = "find . -name"`.
///
/// Asserts `find . -name "*.rs"` matches substring pattern `find . -name` (EC-005 / INV3).
#[test]
fn test_heavy_op_gate_ec005_find_name_matches() {
    // BC-4.15.001 EC-005: "Bash command is 'find . -name \"*.rs\"'; pattern 'find . -name' is in the list"
    let command = r#"find . -name "*.rs""#;

    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    assert!(
        matches!(result, GateResult::Advisory(_)),
        "EC-005: 'find . -name \"*.rs\"' must match pattern 'find . -name' and return Advisory. \
        Got: {result:?}"
    );

    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "find . -name",
            "EC-005: matched_pattern must be 'find . -name'. Got: {:?}",
            advisory.matched_pattern
        );
    }
}

// ---------------------------------------------------------------------------
// EC-006 — ./run-all.sh matches ./run-all.sh (exact substring match)
// ---------------------------------------------------------------------------

/// EC-006 / BC-4.15.001 EC-006: `./run-all.sh` matches pattern `./run-all.sh`.
///
/// Assert: `evaluate_patterns` returns `GateResult::Advisory` with
/// `matched_pattern = "./run-all.sh"`.
///
/// Asserts exact substring match for `./run-all.sh` in the default pattern list (EC-006 / INV3).
#[test]
fn test_heavy_op_gate_ec006_run_all_sh_matches() {
    // BC-4.15.001 EC-006: "Bash command is './run-all.sh'; pattern './run-all.sh' is in the list"
    let command = "./run-all.sh";

    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    assert!(
        matches!(result, GateResult::Advisory(_)),
        "EC-006: './run-all.sh' must match pattern './run-all.sh' and return Advisory. \
        Got: {result:?}"
    );

    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "./run-all.sh",
            "EC-006: matched_pattern must be './run-all.sh'. Got: {:?}",
            advisory.matched_pattern
        );
    }
}

// ---------------------------------------------------------------------------
// EC-013 — custom pattern ./ci.sh: evaluated in list order; triggers advisory
// ---------------------------------------------------------------------------

/// EC-013 / BC-4.15.001 EC-013: when an operator adds a custom pattern `./ci.sh`
/// to the patterns list, `./ci.sh` commands trigger a DelegationRecommended advisory.
///
/// This tests that the gate correctly uses the injected pattern list (not a hardcoded
/// list). The `evaluate_patterns` function MUST accept arbitrary patterns from the
/// caller — not just the DEFAULT_PATTERNS constant.
///
/// Setup: custom patterns list `["./ci.sh"]`; command `./ci.sh`.
/// Assert: `evaluate_patterns` returns `GateResult::Advisory` with
/// `matched_pattern = "./ci.sh"`.
///
/// Asserts operator-injected custom pattern `./ci.sh` triggers advisory (EC-013 / INV3).
#[test]
fn test_heavy_op_gate_ec013_custom_pattern_triggers_advisory() {
    // BC-4.15.001 EC-013: "Operator adds custom pattern './ci.sh' to the patterns list"
    let command = "./ci.sh --deploy staging";
    let custom_patterns: &[&str] = &["./ci.sh"];

    let result = evaluate_patterns(command, custom_patterns);

    assert!(
        matches!(result, GateResult::Advisory(_)),
        "EC-013: './ci.sh --deploy staging' must match custom pattern './ci.sh' and return Advisory. \
        The gate must accept operator-injected patterns, not only DEFAULT_PATTERNS. \
        Got: {result:?}"
    );

    if let GateResult::Advisory(ref advisory) = result {
        assert_eq!(
            advisory.matched_pattern, "./ci.sh",
            "EC-013: matched_pattern must be './ci.sh' (first-match in custom list). Got: {:?}",
            advisory.matched_pattern
        );
    }
}

// ---------------------------------------------------------------------------
// F-P2-003 — channel-identity: stderr message and plugin.log command_preview
//            must carry byte-identical preview strings (AC-006)
// ---------------------------------------------------------------------------

/// F-P2-003 / AC-006: `build_recommendation_message` must embed the raw
/// `command_preview` bytes in the message (not a Debug-quoted variant).
///
/// The message is used verbatim as BOTH the stderr nudge body (PC-B-B1) and
/// the plugin.log `message` field (PC-B-B2). AC-006 requires the `command_preview`
/// to be identical across both channels. Using `{:?}` would add surrounding
/// quotes and escape special characters (backslashes, embedded quotes), causing
/// the preview string as emitted to stderr to differ from `advisory.command_preview`.
///
/// This test uses a command containing a double-quote so that the `{}` vs `{:?}`
/// distinction is observable: the message must contain the raw `"` byte, not `\"`.
///
/// Asserts AC-006 channel-identity: message contains raw command_preview bytes (F-P2-003).
#[test]
fn test_heavy_op_gate_channel_identity_command_preview_not_debug_quoted() {
    // Command containing a double-quote — the classic `{:?}` vs `{}` discriminator.
    let command = r#"grep -r "TODO" ."#; // contains literal " characters
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    if let GateResult::Advisory(ref advisory) = result {
        // Verify the command_preview itself is the raw string (no Debug quoting).
        // truncate_command_preview should return the unchanged short string.
        assert_eq!(
            advisory.command_preview, command,
            "command_preview must be the raw command string (no Debug escaping). \
            Got: {:?}",
            advisory.command_preview
        );

        // The message must contain the raw command_preview bytes, NOT a Debug-quoted
        // variant. If `{:?}` were used in build_recommendation_message, the message
        // would contain `\"TODO\"` (backslash-escaped quotes) rather than `"TODO"`.
        assert!(
            advisory.message.contains(command),
            "AC-006/F-P2-003: message must contain the raw command_preview (byte-identical). \
            Expected message to contain {:?}. \
            Got message: {:?}",
            command,
            advisory.message
        );

        // Confirm the message does NOT contain the Debug-escaped form.
        let debug_escaped = format!("{:?}", command); // would be `"grep -r \"TODO\" ."` with {:?}
        // The message should not contain the inner escaped-quote form `\"TODO\"`
        assert!(
            !advisory.message.contains("\\\""),
            "AC-006/F-P2-003: message must NOT contain Debug-escaped quotes (backslash-quote). \
            Found '\\\"' in message — this indicates {:?} was used instead of {{}}. \
            debug_escaped form would be: {:?}. Message: {:?}",
            "{:?}",
            debug_escaped,
            advisory.message
        );
    } else {
        panic!(
            "F-P2-003 fixture: expected Advisory for command {:?} with DEFAULT_PATTERNS; got Continue",
            command
        );
    }
}
