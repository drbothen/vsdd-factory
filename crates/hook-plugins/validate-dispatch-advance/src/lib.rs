//! validate-dispatch-advance — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write to `STATE.md` whose `current_step:` frontmatter field
//! contains structural violations, and any Edit/Write to `INDEX.md` containing
//! adversary-pass rows with the wrong column count.
//!
//! # Validation gates (STATE.md arm)
//!
//! 1. **Forbidden meta-commentary** (D-440(a)+D-441(a)+D-442(a)): patterns
//!    `META-LEVEL-\d+ WATCH`, `self-app TEST`, `expected verdict` must not
//!    appear in the `current_step:` frontmatter value.
//!
//! 2. **4-index version citations** (D-439(b)): all four literal substrings
//!    `BC-INDEX v`, `VP-INDEX v`, `STORY-INDEX v`, `ARCH-INDEX v` must be
//!    present in `current_step:`.
//!
//! 3. **Trajectory-tail LENGTH=4** (D-451(c)): the global regex `→(\d+)` must
//!    match exactly 4 times in the `current_step:` value.
//!
//! 4. **D-chain cite currency** (D-443(a)): `current_step:` must cite
//!    `D-382..D-N` where N >= the highest D-NNN visible in STATE.md body.
//!
//! # Validation gate (INDEX.md arm)
//!
//! 5. **6-column adversary-pass rows** (D-441(b)): every table row (trimmed
//!    content begins and ends with `|`) must have exactly 8 pipe characters
//!    (7 internal pipes = 6 columns + 2 border pipes). Separator rows excluded.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.006: blocks dispatch-advance structural violations.
//!
//! # D-NNN closures
//!
//! - D-440(a): forbidden meta-commentary prohibition.
//! - D-441(a): verbatim-strict `current_step:` — no meta-commentary injection.
//! - D-442(a): prescribed clause order — forbidden-pattern gate prevents markers.
//! - D-443(a): D-chain cite currency gate.
//! - D-439(b): all 4 index version cite patterns required.
//! - D-441(b): 6-column INDEX.md adversary-pass row schema strict.
//! - D-451(c): trajectory-tail LENGTH=4.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Fail-open on every `host::read_file` error (BC-5.39.006 invariant 9).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - File-path enforcement via in-plugin guard using `Path::file_name` (Q5/Q6 canonical
//!   pattern; NOT `ends_with("STATE.md")` or `ends_with("INDEX.md")` — false-positive
//!   on paths like `xSTATE.md` or `xINDEX.md`).
//! - `tool = "Edit|Write"` is the canonical Q5/Q6 form for this hook's registry entry.
//! - All byte-index slice expressions with possible multi-byte UTF-8 input MUST use
//!   `is_char_boundary()` guards (BC-5.39.006 invariant 10; S-15.11 F-P4-001 lesson).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md or INDEX.md via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — parity with BC-5.39.005/BC-5.39.006
/// cap and the validate-state-structure sibling crate (F-P5-002 fix).
/// Real STATE.md as of the F-P5-002 fix burst is 95185 bytes (95 KiB);
/// INDEX.md is typically smaller. The 4x growth runway accommodates future
/// expansion without requiring a further cap raise.
///
/// BC-5.39.006 precondition 4.
pub const MAX_BYTES: u32 = 524_288;

// ---------------------------------------------------------------------------
// Violation type
// ---------------------------------------------------------------------------

/// A structural violation found in the write target content.
///
/// Carries both a human-readable `description` (used verbatim in the block
/// message) and the `cited_raw` body-literal form of the offending text.
/// Structural plumbing per TD-VSDD-059 paper-fix avoidance: block messages
/// quote the exact string the author wrote, not a paraphrase.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description of the violation, used in the block message.
    pub description: String,
    /// The raw source text that triggered the violation.
    /// Populated via `.trim_end().to_string()` at every violation site.
    /// Required per TD-VSDD-059 paper-fix avoidance (structural plumbing
    /// from day 1 per S-15.07+S-15.11 cascade lessons).
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// File-path guards
// ---------------------------------------------------------------------------

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `STATE.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xSTATE.md`.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.006 precondition 1 — hook STATE.md arm activates only on exact-name match.
/// BC-5.39.006 invariant 3 — path-component-strict matching is mandatory.
/// AC-16 / AC-17 — `xSTATE.md` and `xINDEX.md` must not trigger validation.
pub fn is_state_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("STATE.md")
}

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `INDEX.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xINDEX.md`.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.006 precondition 1 — hook INDEX.md arm activates only on exact-name match.
/// BC-5.39.006 invariant 3 — path-component-strict matching is mandatory.
pub fn is_index_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("INDEX.md")
}

// ---------------------------------------------------------------------------
// current_step: extraction
// ---------------------------------------------------------------------------

/// Extract the `current_step:` frontmatter value from YAML frontmatter.
///
/// YAML frontmatter is bounded by `---` delimiters. Scans for the line
/// starting with `current_step:` within the first frontmatter block and
/// returns the rest of that line as the value (trimming leading whitespace).
///
/// Byte-index slice expressions guard `is_char_boundary()` before slicing
/// to avoid panics on multi-byte UTF-8 content (em-dash, en-dash, NBSP in
/// STATE.md narrative text). BC-5.39.006 invariant 10; S-15.11 F-P4-001.
///
/// Returns `None` if the frontmatter block is absent, the delimiter is not
/// found, or `current_step:` is not present in frontmatter.
///
/// # BC trace
/// BC-5.39.006 invariant 4 — validation is anchored to `current_step:` only.
fn extract_current_step<'a>(content: &'a str) -> Option<&'a str> {
    todo!(
        "BC-5.39.006: extract `current_step:` value from YAML frontmatter bounded \
         by `---` delimiters; guard is_char_boundary() on all byte-index slices; \
         return None if frontmatter absent, delimiter missing, or field absent"
    )
}

// ---------------------------------------------------------------------------
// STATE.md validation checks
// ---------------------------------------------------------------------------

/// Check for forbidden meta-commentary patterns in `current_step:` value.
///
/// Scans for any of:
/// - `META-LEVEL-\d+ WATCH` (hand-rolled: `META-LEVEL-` prefix + digit run + ` WATCH`)
/// - `self-app TEST` (literal substring)
/// - `expected verdict` (literal substring)
///
/// If matched: returns `Some(Violation)` with `cited_raw` set to the matched
/// substring and `decision_refs` embedded in the description citing
/// D-440(a)+D-441(a)+D-442(a).
///
/// # BC trace
/// BC-5.39.006 postcondition 2; D-440(a)+D-441(a)+D-442(a); AC-1/AC-2/AC-3.
fn check_forbidden_meta_commentary(current_step_value: &str) -> Option<Violation> {
    todo!(
        "BC-5.39.006 PC-2: scan current_step_value for forbidden meta-commentary patterns \
         (`META-LEVEL-\\d+ WATCH`, `self-app TEST`, `expected verdict`); \
         return Some(Violation) naming offending pattern and citing D-440(a)+D-441(a)+D-442(a)"
    )
}

/// Check that all 4 required index version citation patterns are present in
/// `current_step:` value.
///
/// Required literal substrings (each followed by a version token):
/// - `BC-INDEX v`
/// - `VP-INDEX v`
/// - `STORY-INDEX v`
/// - `ARCH-INDEX v`
///
/// If any are absent: returns `Some(Violation)` naming each missing cite,
/// citing D-439(b).
///
/// # BC trace
/// BC-5.39.006 postcondition 3; D-439(b); AC-4; invariant 5.
fn check_index_version_cites(current_step_value: &str) -> Option<Violation> {
    todo!(
        "BC-5.39.006 PC-3: check for presence of all 4 literal substrings \
         (`BC-INDEX v`, `VP-INDEX v`, `STORY-INDEX v`, `ARCH-INDEX v`) in \
         current_step_value via str::contains; return Some(Violation) naming \
         each missing cite and citing D-439(b)"
    )
}

/// Check that trajectory-tail in `current_step:` contains exactly 4 `→(\d+)` matches.
///
/// Uses hand-rolled scanning for the `→` character (U+2192, 3 UTF-8 bytes)
/// followed immediately by one or more ASCII digits. Counts the number of
/// such sequences globally in `current_step_value`.
///
/// If count != 4: returns `Some(Violation)` naming actual count and required
/// count (4), citing D-451(c).
///
/// # UTF-8 note
/// The arrow `→` is U+2192, encoded as `\xE2\x86\x92` in UTF-8. Hand-rolled
/// scanning uses `str::contains`/`char::find`-style iteration on `&str` chars
/// to avoid byte-index boundary issues. No raw byte slicing performed here.
///
/// # BC trace
/// BC-5.39.006 postcondition 4; D-451(c); AC-5/AC-6; invariant 6.
fn check_trajectory_tail_length(current_step_value: &str) -> Option<Violation> {
    todo!(
        "BC-5.39.006 PC-4: count `→(\\d+)` sequences in current_step_value via hand-rolled \
         char iteration (no regex crate; fuel budget); if count != 4 return Some(Violation) \
         naming actual count and required count 4, citing D-451(c)"
    )
}

/// Check that the D-chain cite in `current_step:` is current.
///
/// Expected pattern in `current_step_value`: `D-382..D-N` where N is an
/// integer. Compares N against the highest D-NNN visible in the full
/// STATE.md `content` body. Fail-open design:
///
/// - If pattern absent in `current_step_value`: return `Some(Violation)`
///   naming absent cite, citing D-443(a).
/// - If present: extract terminal integer N. Scan full STATE.md `content`
///   for all `D-(\d+)` occurrences (excluding the `D-382..D-N` pattern
///   itself to avoid self-reference). Find max M.
///   - If M > N: return `Some(Violation)` naming `D-382..D-{N}` as stale,
///     terminal {N} < latest {M}.
///   - If M <= N: return `None` (current; fail-open per BC-5.39.006 invariant 7).
///
/// Fail-open on ambiguous cases prevents false-positive blocks on in-progress
/// writes where the Decisions Log is mid-authorship.
///
/// # BC trace
/// BC-5.39.006 postcondition 5; D-443(a); AC-7/AC-8; invariant 7.
fn check_d_chain_currency(content: &str, current_step_value: &str) -> Option<Violation> {
    todo!(
        "BC-5.39.006 PC-5: extract D-382..D-N from current_step_value; if absent return \
         Some(Violation) citing D-443(a); if present compare N against max D-NNN in full \
         content (excluding self-reference pattern); fail-open if M <= N per invariant 7"
    )
}

/// Orchestrate all STATE.md validation checks.
///
/// Extracts `current_step:` value, runs all 4 checks, accumulates non-None
/// results into a `Vec<Violation>`. Returns an empty Vec for a clean write.
///
/// If `current_step:` cannot be extracted, returns a single Violation
/// describing the extraction failure.
///
/// # BC trace
/// BC-5.39.006 postcondition 1/6 — all checks run; multiple violations produce
/// a single BlockWithFix enumerating all.
pub fn validate_state_md(content: &str) -> Vec<Violation> {
    todo!(
        "BC-5.39.006: orchestrate extract_current_step + all 4 check functions; \
         accumulate violations into Vec; return empty Vec on clean write"
    )
}

// ---------------------------------------------------------------------------
// INDEX.md validation
// ---------------------------------------------------------------------------

/// Validate adversary-pass table rows in INDEX.md content.
///
/// Scans `content` line by line. For each line whose trimmed form begins and
/// ends with a pipe character (`|`):
///
/// - Skip separator rows matching pattern `\|\s*---` (header separator lines).
/// - Count pipe occurrences in the trimmed line.
/// - If count != 8 (7 internal pipes = 6 columns + 2 border pipes):
///   add a Violation naming the line text (truncated to 120 chars), actual
///   column count (`pipe_count - 2`, subtracting 2 border pipes), required
///   count 6, citing D-441(b).
///
/// Returns all accumulated violations (may be empty for a clean write).
///
/// # BC trace
/// BC-5.39.006 postcondition 8/9; D-441(b); AC-11/AC-12/AC-13; invariant 8.
pub fn validate_index_md(content: &str) -> Vec<Violation> {
    todo!(
        "BC-5.39.006 PC-8/9: scan content line-by-line; for each pipe-bordered row, \
         skip separator rows (`|\\s*---` pattern); count pipes; if count != 8 \
         add Violation naming row text (truncated 120 chars), actual col count, \
         required 6, citing D-441(b)"
    )
}

// ---------------------------------------------------------------------------
// Hook entry point
// ---------------------------------------------------------------------------

/// PostToolUse hook entry point.
///
/// Called by the SDK trampoline (`__internal::run`) for every Edit/Write
/// PostToolUse event. Dispatches to the STATE.md arm or INDEX.md arm based on
/// path-component-strict guards, then returns the appropriate HookResult.
///
/// # Control flow
///
/// 1. Extract `file_path` from `payload.tool_input`. If absent: Continue
///    (graceful degrade; log_warn).
/// 2. If `is_state_md_target(file_path)`: read via `host::read_file`. On error:
///    Continue + log_warn (fail-open per invariant 9). On success: run
///    `validate_state_md`. If violations non-empty: emit single BlockWithFix
///    enumerating all. Else: Continue.
/// 3. Else if `is_index_md_target(file_path)`: read via `host::read_file`. On
///    error: Continue + log_warn. On success: run `validate_index_md`. If
///    violations non-empty: emit single BlockWithFix enumerating all. Else:
///    Continue.
/// 4. Else: Continue (file not in scope of this hook).
///
/// # BC trace
/// BC-5.39.006 postconditions 1–10; invariants 1–10.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    todo!(
        "BC-5.39.006: implement full hook dispatch — extract file_path from tool_input; \
         dispatch to STATE.md arm (validate_state_md) or INDEX.md arm (validate_index_md) \
         via path-component-strict guards; fail-open on host::read_file errors; \
         emit single BlockWithFix enumerating all violations if non-empty"
    )
}

// ---------------------------------------------------------------------------
// Unit tests (all red until implementer fills in real logic)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Path-guard tests (GREEN-BY-DESIGN: pure Path::file_name delegation) --

    #[test]
    fn test_is_state_md_target_positive() {
        assert!(is_state_md_target(".factory/STATE.md"));
    }

    #[test]
    fn test_xstate_md_is_not_target() {
        assert!(!is_state_md_target("/some/dir/xSTATE.md"));
    }

    #[test]
    fn test_is_index_md_target_positive() {
        assert!(is_index_md_target(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md"
        ));
    }

    #[test]
    fn test_xindex_md_is_not_target() {
        assert!(!is_index_md_target("/some/dir/xINDEX.md"));
    }

    // -- Forbidden meta-commentary tests --

    #[test]
    fn test_forbidden_meta_commentary_watch() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             META-LEVEL-5 WATCH: self-application →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect META-LEVEL-5 WATCH pattern");
        let v = v.unwrap();
        assert!(
            v.description.contains("D-440(a)"),
            "block message must cite D-440(a)"
        );
    }

    #[test]
    fn test_forbidden_meta_commentary_self_app() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 \
             self-app TEST example →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect self-app TEST pattern");
    }

    #[test]
    fn test_forbidden_meta_commentary_expected_verdict() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 \
             expected verdict: PASS →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect expected verdict pattern");
    }

    // -- Index version cite tests --

    #[test]
    fn test_index_version_cites_all_present() {
        let v = check_index_version_cites(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_none(), "all 4 cites present — should not violate");
    }

    #[test]
    fn test_index_version_cites_missing_arch() {
        let v = check_index_version_cites(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12 →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "missing ARCH-INDEX v cite — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains("ARCH-INDEX v"),
            "block message must name missing ARCH-INDEX v"
        );
        assert!(
            v.description.contains("D-439(b)"),
            "block message must cite D-439(b)"
        );
    }

    // -- Trajectory-tail length tests --

    #[test]
    fn test_trajectory_tail_length_3() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "tail has 3 components — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains('3'),
            "block message must name actual count 3"
        );
        assert!(
            v.description.contains("D-451(c)"),
            "block message must cite D-451(c)"
        );
    }

    #[test]
    fn test_trajectory_tail_length_5() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "tail has 5 components — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains('5'),
            "block message must name actual count 5"
        );
    }

    #[test]
    fn test_trajectory_tail_length_4() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_none(), "tail has exactly 4 components — should not violate");
    }

    // -- D-chain currency tests --

    #[test]
    fn test_d_chain_absent() {
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9'\n---\n| D-477 |";
        let v = check_d_chain_currency(
            content,
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 →9→9→9→9",
        );
        assert!(v.is_some(), "absent D-chain cite — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains("D-443(a)"),
            "block message must cite D-443(a)"
        );
    }

    #[test]
    fn test_d_chain_stale() {
        // current_step cites D-382..D-476 but body shows D-477
        let content =
            "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-382..D-476'\n---\n\
             | D-477 | some row |\n";
        let current_step =
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-476";
        let v = check_d_chain_currency(content, current_step);
        assert!(v.is_some(), "stale D-chain cite (476 < 477) — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains("476"),
            "block message must name stale terminal 476"
        );
    }

    #[test]
    fn test_d_chain_current() {
        // current_step cites D-382..D-477 and body max is also D-477
        let content =
            "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-382..D-477'\n---\n\
             | D-477 | some row |\n";
        let current_step =
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477";
        let v = check_d_chain_currency(content, current_step);
        assert!(v.is_none(), "D-chain cite is current — should not violate");
    }

    // -- INDEX.md column tests --

    #[test]
    fn test_index_md_5_col_row() {
        let content = "| col1 | col2 | col3 | col4 | col5 |\n";
        let violations = validate_index_md(content);
        assert!(!violations.is_empty(), "5-column row should violate");
        assert!(
            violations[0].description.contains("D-441(b)"),
            "violation must cite D-441(b)"
        );
        assert!(
            violations[0].description.contains('5'),
            "violation must name actual count 5"
        );
    }

    #[test]
    fn test_index_md_7_col_row() {
        let content = "| col1 | col2 | col3 | col4 | col5 | col6 | col7 |\n";
        let violations = validate_index_md(content);
        assert!(!violations.is_empty(), "7-column row should violate");
        assert!(
            violations[0].description.contains('7'),
            "violation must name actual count 7"
        );
    }

    #[test]
    fn test_index_md_6_col_row() {
        let content = "| col1 | col2 | col3 | col4 | col5 | col6 |\n";
        let violations = validate_index_md(content);
        assert!(violations.is_empty(), "6-column row — should not violate");
    }

    #[test]
    fn test_index_md_header_row_skipped() {
        // Separator row (|---|---|) must not be flagged regardless of pipe count.
        let content = "| --- | --- | --- | --- |\n";
        let violations = validate_index_md(content);
        assert!(
            violations.is_empty(),
            "separator row must be skipped by validate_index_md"
        );
    }

    // -- UTF-8 / em-dash safety (AC-20) --

    /// AC-20: `current_step:` value containing em-dash (U+2014) or other
    /// multi-byte UTF-8 characters must not cause a panic. This test invokes
    /// `extract_current_step` on content with `—` adjacent to a digit and
    /// asserts the call completes without panicking.
    ///
    /// "If I include this real implementation, will the test for this function
    /// pass trivially without any implementer work?" — No. The extraction
    /// logic must guard is_char_boundary() correctly for this test to pass.
    #[test]
    fn validate_current_step_with_em_dash() {
        let content = "---\ncurrent_step: 'pass 74—D-382..D-477 →9→9→9→9'\n---\nbody\n";
        // Must not panic; correctness of returned value tested by implementer.
        let _ = extract_current_step(content);
    }

    // -- Production false-positive regression (AC-21 preemptive; S-15.09 lesson) --

    /// Load-bearing preemptive test: reads the real `.factory/STATE.md` from
    /// disk and asserts `validate_state_md` emits no violations. This catches
    /// false-positive blocks that would fire against production content before
    /// any bats test runs. Added preemptively per S-15.09 AC-13 cascade lesson.
    ///
    /// Path resolved via `CARGO_MANIFEST_DIR` relative traversal to the
    /// workspace root `.factory/STATE.md`.
    ///
    /// If STATE.md does not exist (e.g., CI environment without factory
    /// worktree mounted), the test is silently skipped rather than failing.
    #[test]
    fn validate_production_state_md_no_false_positive() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let state_md_path = format!("{manifest_dir}/../../../.factory/STATE.md");
        let content = match std::fs::read_to_string(&state_md_path) {
            Ok(c) => c,
            Err(_) => {
                // STATE.md not present in this build environment; skip silently.
                return;
            }
        };
        let violations = validate_state_md(&content);
        assert!(
            violations.is_empty(),
            "validate_state_md must not produce false-positive violations \
             against production STATE.md; violations found: {violations:?}"
        );
    }
}
