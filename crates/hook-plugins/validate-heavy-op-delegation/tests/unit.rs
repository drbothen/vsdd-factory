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
//! # Test Table (S-18.06 v1.10 — unit.rs subset)
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
//! | `test_heavy_op_gate_redacts_flag_arg_secret` | AC-012 | INV5/EC-014 | GREEN |
//! | `test_heavy_op_gate_redacts_env_assignment_secret` | AC-012 | INV5/EC-015 | GREEN |
//! | `test_heavy_op_gate_redacts_authorization_header` | AC-012 | INV5/EC-016 | GREEN |
//! | `test_heavy_op_gate_redacts_url_credentials` | AC-012 | INV5/EC-017 | GREEN |
//! | `test_heavy_op_gate_no_redaction_on_clean_command` | AC-012 | INV5/EC-018 | GREEN |
//! | `test_heavy_op_gate_allowlist_env_var_not_redacted` | AC-012 | INV5/EC-019 | GREEN |
//! | `test_heavy_op_gate_bare_key_flag_not_redacted` | AC-012 | INV5/EC-020 | GREEN |
//! | `test_heavy_op_gate_redact_then_truncate_ordering` | AC-012 | INV5/EC-021 | GREEN |
//! | `test_heavy_op_gate_no_redaction_preserves_whitespace` | AC-012 | INV5/PC6b/F-RD1-001 | RED |
//! | `test_heavy_op_gate_unquoted_auth_header_preserves_trailing_args` | AC-012 | INV5/Pass3/F-RD1-002 | RED |

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

// ---------------------------------------------------------------------------
// AC-009 — Pure-parse; no filesystem, subprocess, or context access (INV1)
// ---------------------------------------------------------------------------

/// AC-009 / INV1 / BC-4.15.001 Architecture Compliance Rule 1:
/// The WASM gate source MUST NOT import `std::fs`, `std::process`, or `std::net`
/// (or any qualified `fs::` / `process::` / `net::` usage that would bypass the
/// `std::` prefix).
///
/// This is a source-fence test: it embeds the crate source at compile time via
/// `include_str!` and scans for actual import/use tokens at test runtime.
///
/// **Why source-fence rather than feature-flag or linker trick:**
/// The pure-parse invariant (INV1) is a source-level architectural constraint.
/// A linker check would only catch linked symbols; a feature-flag approach only
/// catches conditional compilation paths. Source-fence catches ANY `use std::fs`
/// or `std::fs::` reference regardless of whether it's dead code — if it appears
/// There is no clippy disallowed-types configuration for this crate; this
/// source-fence test is the SOLE INV1 enforcement mechanism and runs on every
/// `cargo test`.
///
/// **False-positive prevention:**
/// The doc comments in lib.rs say "NO `std::fs::`" (a negation sentence). Scanning
/// for the bare substring `std::fs` would match those doc comments. Instead this
/// test scans the non-comment source lines (filtering out lines whose trimmed form
/// starts with `//`) for actual use/import tokens:
///   - `use std::fs`      — direct import
///   - `use std::process` — direct import
///   - `use std::net`     — direct import
///   - `std::fs::`        — qualified path usage (e.g. `std::fs::read_to_string(...)`)
///   - `std::process::`   — qualified path usage
///   - `std::net::`       — qualified path usage
///
/// These patterns cannot appear in a negation sentence without the `::`  suffix or
/// the `use ` prefix, so they are safe against the doc-comment false-positive.
///
/// **Mutation evidence (from development):**
/// Adding `use std::fs;` to lib.rs causes this test to fail immediately.
/// The test is non-tautological: a clean implementation passes; a dirty one fails.
///
/// Asserts BC-4.15.001 INV1 / AC-009 / ADR-026 §Decision 8 at the source level.
#[test]
fn test_heavy_op_gate_pure_parse_no_filesystem_access() {
    // Embed both source files at compile time.
    // The paths are relative to this test file's location:
    //   tests/unit.rs  →  ../src/lib.rs  and  ../src/main.rs
    const LIB_SRC: &str = include_str!("../src/lib.rs");
    const MAIN_SRC: &str = include_str!("../src/main.rs");

    // Filter out comment lines (lines whose trimmed form starts with `//`).
    // This prevents false-positives from doc-comment negation sentences like
    // "NO `std::fs::`" in the lib.rs module-level doc comment.
    let lib_non_comment: String = LIB_SRC
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    let main_non_comment: String = MAIN_SRC
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    // Forbidden import patterns (actual use/import tokens, not prose mentions).
    // Each pattern is chosen to be unambiguous in non-comment Rust source:
    //   - `use std::fs`      triggers on `use std::fs;` / `use std::fs::{...}`
    //   - `std::fs::`        triggers on qualified path calls
    //   - `use std::process` triggers on `use std::process;` / `use std::process::Command`
    //   - `std::process::`   triggers on qualified `std::process::exit(...)` etc.
    //   - `use std::net`     triggers on `use std::net;` / `use std::net::TcpStream` etc.
    //   - `std::net::`       triggers on qualified path calls
    let forbidden: &[(&str, &str)] = &[
        (
            "use std::fs",
            "std::fs import (BC-4.15.001 INV1: NO filesystem reads)",
        ),
        (
            "std::fs::",
            "std::fs:: qualified usage (BC-4.15.001 INV1: NO filesystem reads)",
        ),
        (
            "use std::process",
            "std::process import (BC-4.15.001 INV1: NO subprocess execution)",
        ),
        (
            "std::process::",
            "std::process:: qualified usage (BC-4.15.001 INV1: NO subprocess execution)",
        ),
        (
            "use std::net",
            "std::net import (BC-4.15.001 INV1: NO network calls)",
        ),
        (
            "std::net::",
            "std::net:: qualified usage (BC-4.15.001 INV1: NO network calls)",
        ),
    ];

    for (pattern, description) in forbidden {
        assert!(
            !lib_non_comment.contains(pattern),
            "AC-009/INV1: forbidden pattern {:?} found in src/lib.rs non-comment source.\n\
            Description: {}\n\
            BC-4.15.001 INV1: 'Pure-parse; no filesystem, subprocess, or context access.'\n\
            ADR-026 §Decision 8: 'WASM for pure-function command-string matching; no side effects.'\n\
            Architecture Compliance Rule 1: 'MUST NOT import std::fs, std::process, std::net.'",
            pattern,
            description
        );

        assert!(
            !main_non_comment.contains(pattern),
            "AC-009/INV1: forbidden pattern {:?} found in src/main.rs non-comment source.\n\
            Description: {}\n\
            BC-4.15.001 INV1: 'Pure-parse; no filesystem, subprocess, or context access.'\n\
            ADR-026 §Decision 8: 'WASM for pure-function command-string matching; no side effects.'",
            pattern,
            description
        );
    }
}

// ---------------------------------------------------------------------------
// AC-012 / INV5 — 4-pass secret redaction (SEC-002)
//
// BC-4.15.001 INV5: "command_preview MUST apply 4-pass redaction before
// truncation (redact-then-truncate ordering)".
//
// All 8 tests call `evaluate_patterns(command, patterns)` via the existing
// public API.  No new function signatures are introduced.  Tests 1-4 and 8
// are RED against the current (no-redaction) implementation and will turn
// GREEN once INV5 is implemented.  Tests 5-7 are GREEN now and serve as
// regression guards (they verify the gate does NOT over-redact).
// ---------------------------------------------------------------------------

/// AC-012 / INV5 Pass 1 / EC-014:
/// A flag-argument secret (`--token <value>`) is redacted before the preview
/// is returned. The raw secret value — including ANY substring of it — must
/// not appear in `command_preview`.
///
/// Setup: command `grep -r . --token abc123secret` — pattern `grep -r` matches
/// (so `evaluate_patterns` returns `Advisory`). Pass 1 applies:
///   `--token abc123secret` → `--token ***REDACTED***`
///
/// Assert:
///   1. `command_preview` contains `--token ***REDACTED***` (full mask applied).
///   2. `command_preview` does NOT contain `abc123secret` (full secret absent).
///   3. `command_preview` does NOT contain `123secret` (no recognizable remnant;
///      blocks crippled "replace-only-first-char" partial implementations that
///      would pass assertion 2 but leave `bc123secret` or `23secret` intact).
///
/// BC-4.15.001 INV5 / AC-012. Hardened per SEC-002.
#[test]
fn test_heavy_op_gate_redacts_flag_arg_secret() {
    let command = "grep -r . --token abc123secret";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                advisory.command_preview.contains("--token ***REDACTED***"),
                "AC-012/INV5 Pass 1: command_preview must contain '--token ***REDACTED***'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: flag-arg secret MUST be fully masked with ***REDACTED***.",
                advisory.command_preview
            );
            assert!(
                !advisory.command_preview.contains("abc123secret"),
                "AC-012/INV5 Pass 1: raw secret 'abc123secret' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: FULL-value redaction — the entire secret token must be replaced.",
                advisory.command_preview
            );
            // SEC-002 hardening: assert no recognizable remnant of the secret
            // survives. A crippled "replace only the first character" implementation
            // would leave "bc123secret" or "123secret" in the output — reject that.
            assert!(
                !advisory.command_preview.contains("123secret"),
                "AC-012/INV5 Pass 1 SEC-002: remnant '123secret' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: FULL-value masking required; partial replacement is a security defect.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 Pass 1: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 Pass 2 / EC-015:
/// An environment-variable assignment (`KEY=value`) is redacted before the
/// preview is returned. The raw secret value must not appear in `command_preview`.
///
/// Setup: command `API_KEY=sk-abc123 grep -r .` — pattern `grep -r` matches.
/// Pass 2 applies: `API_KEY=sk-abc123` → `API_KEY=***REDACTED***`
///
/// Assert: `command_preview` contains `API_KEY=***REDACTED***`; does NOT contain
/// `sk-abc123`.
///
/// RED against current implementation (no redaction). BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_redacts_env_assignment_secret() {
    let command = "API_KEY=sk-abc123 grep -r .";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                advisory.command_preview.contains("API_KEY=***REDACTED***"),
                "AC-012/INV5 Pass 2: command_preview must contain 'API_KEY=***REDACTED***'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: env-var assignment secret MUST be redacted.",
                advisory.command_preview
            );
            assert!(
                !advisory.command_preview.contains("sk-abc123"),
                "AC-012/INV5 Pass 2: raw secret 'sk-abc123' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: redaction MUST replace env-var value with ***REDACTED***.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 Pass 2: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 Pass 3 / EC-016:
/// An Authorization/Cookie header value is redacted before the preview is
/// returned.
///
/// Setup: command `grep -r . -H "Authorization: Bearer eyJtoken123"` — pattern
/// `grep -r` matches.  Pass 3 applies:
///   `Authorization: Bearer eyJtoken123` → `Authorization:***REDACTED***`
///
/// Assert: `command_preview` contains `Authorization:***REDACTED***`; does NOT
/// contain `eyJtoken123`.
///
/// RED against current implementation (no redaction). BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_redacts_authorization_header() {
    let command = r#"grep -r . -H "Authorization: Bearer eyJtoken123""#;
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                advisory
                    .command_preview
                    .contains("Authorization:***REDACTED***"),
                "AC-012/INV5 Pass 3: command_preview must contain 'Authorization:***REDACTED***'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: Authorization header value MUST be redacted.",
                advisory.command_preview
            );
            assert!(
                !advisory.command_preview.contains("eyJtoken123"),
                "AC-012/INV5 Pass 3: raw token 'eyJtoken123' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: redaction MUST replace header secret with ***REDACTED***.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 Pass 3: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 Pass 4 / EC-017:
/// Inline URL credentials (`user:pass@host`) are redacted before the preview
/// is returned.
///
/// Setup: command `./run-all.sh https://user:pass@example.com/db` — pattern
/// `./run-all.sh` matches.  Pass 4 applies:
///   `https://user:pass@example.com/db` → `https://***REDACTED***@example.com/db`
///
/// Assert: `command_preview` contains `https://***REDACTED***@example.com/db`;
/// does NOT contain `user:pass`.
///
/// RED against current implementation (no redaction). BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_redacts_url_credentials() {
    let command = "https://user:pass@example.com/db ./run-all.sh";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                advisory
                    .command_preview
                    .contains("https://***REDACTED***@example.com/db"),
                "AC-012/INV5 Pass 4: command_preview must contain 'https://***REDACTED***@example.com/db'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: URL credentials MUST be replaced with ***REDACTED***.",
                advisory.command_preview
            );
            assert!(
                !advisory.command_preview.contains("user:pass"),
                "AC-012/INV5 Pass 4: raw credentials 'user:pass' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: redaction MUST replace URL user:pass with ***REDACTED***.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 Pass 4: expected Advisory for command containing './run-all.sh'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 negative / EC-018:
/// A clean command with no secret patterns produces NO `***REDACTED***` in
/// the preview.  This guards against over-redaction.
///
/// Setup: command `grep -r TODO . --include="*.rs"` — no flag-args, no env
/// assignments, no auth headers, no URL credentials.  Pattern `grep -r` matches.
///
/// Assert: `command_preview` does NOT contain `***REDACTED***`.
///
/// GREEN against current implementation (no redaction). Regression guard.
/// BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_no_redaction_on_clean_command() {
    let command = r#"grep -r TODO . --include="*.rs""#;
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                !advisory.command_preview.contains("***REDACTED***"),
                "AC-012/INV5 EC-018: clean command must NOT produce '***REDACTED***' in preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: redaction MUST NOT over-apply to commands with no secrets.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 EC-018: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 allowlist / EC-019:
/// An allowlisted env-var (`SSH_AUTH_SOCK`) must NOT be redacted.
///
/// Setup: command `SSH_AUTH_SOCK=/tmp/agent.1 grep -r .` — `SSH_AUTH_SOCK` is
/// explicitly on the Pass 2 allowlist.  Pattern `grep -r` matches.
///
/// Assert: `command_preview` contains `/tmp/agent.1` (the value is preserved);
/// does NOT contain `***REDACTED***`.
///
/// GREEN against current implementation (no redaction). Regression guard.
/// BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_allowlist_env_var_not_redacted() {
    let command = "SSH_AUTH_SOCK=/tmp/agent.1 grep -r .";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                advisory.command_preview.contains("/tmp/agent.1"),
                "AC-012/INV5 EC-019: allowlisted SSH_AUTH_SOCK value must be preserved in preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5 allowlist: SSH_AUTH_SOCK MUST NOT be redacted.",
                advisory.command_preview
            );
            assert!(
                !advisory.command_preview.contains("***REDACTED***"),
                "AC-012/INV5 EC-019: allowlisted SSH_AUTH_SOCK must NOT produce ***REDACTED***.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5 allowlist: SSH_AUTH_SOCK is exempt from Pass 2 redaction.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 EC-019: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 bare-flag negative / EC-020:
/// A bare `--key` flag (no `<space>value` following it) must NOT be redacted.
/// Pass 1 only redacts `--<flag> <value>` pairs where a whitespace-separated
/// value follows the flag.
///
/// Setup: command `find . -name "*.key" -type f` — pattern `find . -name` matches.
/// No flag-arg pair where a secret value follows.
///
/// Assert: `command_preview` does NOT contain `***REDACTED***`.
///
/// GREEN against current implementation (no redaction). Regression guard.
/// BC-4.15.001 INV5 / AC-012 / EC-020.
#[test]
fn test_heavy_op_gate_bare_key_flag_not_redacted() {
    let command = r#"find . -name "*.key" -type f"#;
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            assert!(
                !advisory.command_preview.contains("***REDACTED***"),
                "AC-012/INV5 EC-020: bare flag with no following value must NOT produce ***REDACTED***.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: Pass 1 only redacts '--flag value' pairs, not bare flags.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 EC-020: expected Advisory for command containing 'find . -name'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}

/// AC-012 / INV5 redact-then-truncate ordering / EC-021:
/// Redaction MUST be applied BEFORE truncation so that the 120-char window
/// shows the redacted string, not the raw secret.
///
/// Fixture construction (BC-4.15.001 EC-021):
///   Prefix = `"grep -r . --token shh "` — 22 chars.
///   The secret `shh` is a whitespace-delimited value token (space after it).
///   Padding = `"x"` × 97 chars — a separate clean positional arg after the space.
///   Raw command = 22 + 97 = 119 chars (≤ 120 → NO truncation on the raw string).
///
///   After Pass 1 FULL-value redaction:
///     `--token shh` → `--token ***REDACTED***`
///     (`"shh"` = 3 chars → `"***REDACTED***"` = 14 chars; net +11)
///   Post-redaction string = `"grep -r . --token ***REDACTED*** "` (35 chars)
///     + 97 `x` chars = 132 chars (> 120 → truncation kicks in).
///   Truncation yields first 120 chars of 132-char string + ELLIPSIS (U+2026) =
///   121 code points.
///
///   The 120-char window = `"grep -r . --token ***REDACTED*** "` (35) + 85 `x`s.
///   The raw secret `"shh"` does NOT appear in this window.
///
/// Assert:
///   1. `command_preview` ends with `ELLIPSIS` (was truncated).
///   2. `command_preview.chars().count() == 121` (COMMAND_PREVIEW_MAX_CHARS + 1).
///   3. `command_preview` contains `"--token ***REDACTED***"` (redaction before
///      the truncation window).
///   4. `command_preview` does NOT contain `"shh"` (raw secret absent).
///
/// BC-4.15.001 INV5 / AC-012.
#[test]
fn test_heavy_op_gate_redact_then_truncate_ordering() {
    // "grep -r . --token shh " = 22 chars (trailing space separates x-block)
    // + 97 'x' chars = 119 chars total (< 120 → no truncation on raw string)
    //
    // After FULL-value Pass 1 redaction:
    //   "shh" (3 chars) → "***REDACTED***" (14 chars): net +11
    //   "grep -r . --token ***REDACTED*** " (35 chars) + "x"×97 = 132 chars
    //   132 > 120 → truncation: first 120 chars + U+2026 = 121 code points
    let raw_command = format!("grep -r . --token shh {}", "x".repeat(97));
    assert_eq!(
        raw_command.chars().count(),
        119,
        "fixture: raw command must be exactly 119 chars (22 prefix + 97 x-padding)"
    );

    let result = evaluate_patterns(&raw_command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            // Assert 1: post-redaction string (132 chars) exceeds 120 → ELLIPSIS
            assert!(
                advisory.command_preview.ends_with(ELLIPSIS),
                "AC-012/INV5 EC-021: preview must end with ELLIPSIS.\n\
                Raw = 119 chars (no truncation). Post-redaction = 132 chars → truncation required.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: redact-then-truncate ordering; truncation window applied to post-redaction string.",
                advisory.command_preview
            );
            // Assert 2: exactly 121 code points (120 + ELLIPSIS)
            assert_eq!(
                advisory.command_preview.chars().count(),
                COMMAND_PREVIEW_MAX_CHARS + 1,
                "AC-012/INV5 EC-021: preview must be exactly 121 code points (120 + ELLIPSIS).\n\
                Got {} code points: {:?}\n\
                BC-4.15.001 INV5: COMMAND_PREVIEW_MAX_CHARS = {}.",
                advisory.command_preview.chars().count(),
                advisory.command_preview,
                COMMAND_PREVIEW_MAX_CHARS
            );
            // Assert 3: redaction token visible in the 120-char window
            assert!(
                advisory.command_preview.contains("--token ***REDACTED***"),
                "AC-012/INV5 EC-021: preview must contain '--token ***REDACTED***'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: Pass 1 redaction applied BEFORE INV4 truncation window.",
                advisory.command_preview
            );
            // Assert 4: raw secret absent from the 120-char window
            assert!(
                !advisory.command_preview.contains("shh"),
                "AC-012/INV5 EC-021: raw secret 'shh' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5: raw secret replaced by ***REDACTED*** before preview window.",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "AC-012/INV5 EC-021: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                raw_command
            );
        }
    }
}

// ---------------------------------------------------------------------------
// F-RD1-001 — whitespace-normalization bug on clean commands
//
// BC-4.15.001 PC6b / VP-091 §6 PC6b: "the command_preview of a command that
// triggers no redaction MUST equal truncate_command_preview(original_command)".
// The current implementation pipes EVERY command through all 4 redaction passes,
// each of which does split_whitespace().join(" ") unconditionally, collapsing
// tabs and multiple spaces even when no secret is present.
//
// RED against current implementation (whitespace normalised to single spaces).
// ---------------------------------------------------------------------------

/// F-RD1-001 / BC-4.15.001 PC6b:
/// A clean command (no sensitive flags, env vars, auth headers, or URL creds)
/// MUST have a `command_preview` that is byte-for-byte equal to the truncated
/// original command string.  Irregular whitespace (double space, tab) MUST be
/// preserved exactly.
///
/// Setup: `"grep -r  TODO\t. --include=*.rs"` — contains a double space between
/// `grep -r` and `TODO`, and a tab between `TODO` and `.`.  No sensitive tokens
/// are present.  Pattern `grep -r` matches (Advisory returned).
///
/// Assert: `advisory.command_preview == truncate_command_preview(raw)`.
/// Equivalently (command is short — no truncation): `advisory.command_preview == raw`.
///
/// Current implementation FAILS: all 4 redaction passes call
/// `split_whitespace().collect::<Vec<_>>().join(" ")` unconditionally, collapsing
/// `"  "` → `" "` and `"\t"` → `" "`, producing `"grep -r TODO . --include=*.rs"`
/// instead of the original.
///
/// RED against current implementation. BC-4.15.001 PC6b / VP-091 §6.
#[test]
fn test_heavy_op_gate_no_redaction_preserves_whitespace() {
    // Fixture: double space between "-r" and "TODO", tab between "TODO" and ".".
    // No sensitive flags, assignments, auth headers, or URL creds.
    let raw = "grep -r  TODO\t. --include=*.rs";

    // This command is 30 chars — well under 120; truncate_command_preview is a no-op.
    assert!(
        raw.chars().count() < COMMAND_PREVIEW_MAX_CHARS,
        "fixture: raw command must be < {} chars for truncation to be a no-op",
        COMMAND_PREVIEW_MAX_CHARS
    );

    let result = evaluate_patterns(raw, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            // truncate_command_preview is a no-op for short inputs, so this is
            // equivalent to asserting advisory.command_preview == raw.
            let expected = truncate_command_preview(raw);
            assert_eq!(
                advisory.command_preview,
                expected,
                "F-RD1-001 / BC-4.15.001 PC6b: command_preview MUST equal truncate_command_preview(original).\n\
                A clean command (no redaction triggers) must not have its whitespace normalized.\n\
                Expected: {:?}\n\
                Got:      {:?}\n\
                Bug: all 4 redaction passes call split_whitespace().join(\" \") unconditionally,\n\
                collapsing double-space and tab to single space.",
                expected,
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "F-RD1-001: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                raw
            );
        }
    }
}

// ---------------------------------------------------------------------------
// F-RD1-002 — Pass 3 over-consumption on unquoted Authorization header
//
// BC-4.15.001 INV5 Pass 3 two-token form: only the NEXT (value) token after
// the header name is consumed. The current skip_until_end_of_value loop
// continues consuming ALL subsequent tokens until it finds one ending with
// `"` or `'`, which never happens for unquoted args — dropping all trailing
// content.
//
// RED against current implementation (trailing args consumed/dropped).
// ---------------------------------------------------------------------------

/// F-RD1-002 / BC-4.15.001 INV5 Pass 3:
/// An unquoted `Authorization: Bearer <token>` header in a Bash command MUST
/// have only the bearer token value redacted.  Trailing arguments that follow
/// the bearer token MUST appear in `command_preview`.
///
/// Setup: `"grep -r . -H Authorization: Bearer tok123 --include=*.rs"`
///   - `Authorization:` header prefix — Pass 3 fires.
///   - `Bearer` — the scheme/type token; consumed by skip_until_end_of_value.
///   - `tok123` — the secret; MUST be redacted.
///   - `--include=*.rs` — a trailing clean arg; MUST survive in the preview.
///
/// Assert:
///   1. `command_preview` contains `***REDACTED***` (redaction applied).
///   2. `command_preview` does NOT contain `tok123` (secret absent).
///   3. `command_preview` contains `--include=*.rs` (trailing arg preserved).
///   4. `command_preview` contains `grep -r .` (prefix preserved).
///
/// Current implementation FAILS assertion 3: the `skip_until_end_of_value`
/// loop in `redact_pass3_auth_headers` has no termination condition for
/// unquoted values — it consumes every token after `Authorization:` until it
/// finds one ending with `"` or `'`, which never happens here, so
/// `--include=*.rs` is silently dropped.
///
/// RED against current implementation. BC-4.15.001 INV5 Pass 3 / F-RD1-002.
#[test]
fn test_heavy_op_gate_unquoted_auth_header_preserves_trailing_args() {
    // Unquoted Authorization header with a trailing clean arg after the token.
    let command = "grep -r . -H Authorization: Bearer tok123 --include=*.rs";
    let result = evaluate_patterns(command, DEFAULT_PATTERNS);

    match result {
        GateResult::Advisory(ref advisory) => {
            // Assert 1: redaction mask present (Pass 3 fired).
            assert!(
                advisory.command_preview.contains("***REDACTED***"),
                "F-RD1-002 / INV5 Pass 3: command_preview must contain '***REDACTED***'.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5 Pass 3: Authorization header value MUST be masked.",
                advisory.command_preview
            );
            // Assert 2: raw bearer token absent.
            assert!(
                !advisory.command_preview.contains("tok123"),
                "F-RD1-002 / INV5 Pass 3: raw bearer token 'tok123' must NOT appear in command_preview.\n\
                Got: {:?}\n\
                BC-4.15.001 INV5 Pass 3: secret MUST be replaced by ***REDACTED***.",
                advisory.command_preview
            );
            // Assert 3: trailing clean arg preserved — the core of F-RD1-002.
            assert!(
                advisory.command_preview.contains("--include=*.rs"),
                "F-RD1-002 / INV5 Pass 3: trailing arg '--include=*.rs' MUST appear in command_preview.\n\
                Got: {:?}\n\
                Bug: skip_until_end_of_value loop consumes all tokens after Authorization: until\n\
                a quote-terminated token is found — for unquoted headers this drops all trailing args.\n\
                BC-4.15.001 INV5 Pass 3 two-token form: only the NEXT value token is consumed.",
                advisory.command_preview
            );
            // Assert 4: command prefix preserved.
            assert!(
                advisory.command_preview.contains("grep -r ."),
                "F-RD1-002 / INV5 Pass 3: prefix 'grep -r .' MUST appear in command_preview.\n\
                Got: {:?}",
                advisory.command_preview
            );
        }
        GateResult::Continue => {
            panic!(
                "F-RD1-002: expected Advisory for command containing 'grep -r'; got Continue.\n\
                Command: {:?}",
                command
            );
        }
    }
}
