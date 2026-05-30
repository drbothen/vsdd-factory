//! validate-trajectory-tail-cell-completeness — PostToolUse WASM hook plugin.
//!
//! Validates that `trajectory_tail` is present in every D-453(d) prescribed
//! site across `.factory/STATE.md`, active-cycle `INDEX.md`, `burst-log.md`,
//! and `lessons.md`.
//!
//! # Behaviour overview
//!
//! - **STATE.md arm (5 sites — Block severity):** current_step frontmatter,
//!   Last Updated cell, Phase Progress latest row, Concurrent Cycles latest
//!   row, Session Resume Section 1. All missing sites cascade into a single
//!   Block (BC-5.39.009 invariant 8).
//! - **INDEX.md arm (2 sites — advisory):** Convergence Status row,
//!   adversarial-review summary-table latest-pass row. log_warn + Continue.
//! - **burst-log.md arm (1 site — advisory):** latest Dim-7 block.
//!   log_warn + Continue.
//! - **lessons.md arm (advisory pass-through):** PC10 OUT-OF-SCOPE per BC
//!   v1.8 F-SP5-003. Always Continue + log_warn advisory.
//!
//! # Architecture compliance
//!
//! - BC-5.39.009 postconditions 1–12; invariants 1–13.
//! - No `println!` — all output via `host::log_*`.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate — manual arrow-count scanner mandated (story U6).
//! - File-path enforcement via `Path::file_name()` — NOT `ends_with`.
//! - Path-component-walk for `.factory/` parent guard (BC v1.8 Precondition 4).
//! - `is_char_boundary()` guards on byte-index slice operations (inv-11).
//! - Fail-open on every `host::read_file` error (inv-10).
//! - STATE.md cascade: all missing sites in one Block (inv-8).
//! - inv-13 encoding gate: all extractors receive `content: &str` (post
//!   `String::from_utf8` decode); UTF-8 failure routes via EC-020 fail-open.

// Suppress dead_code + unused warnings for stubs; these will be removed as
// functions are implemented.
#![allow(dead_code)]
#![allow(unused_variables)]
#![cfg_attr(test, allow(unused_imports))]

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Maximum bytes to read from any target file via `host::read_file`.
///
/// 512 KiB — parity with validate-policies-schema and validate-state-structure
/// siblings; prevents META-LEVEL-24 false-green truncation. u32 per
/// `host::read_file` signature (sibling parity with validate-policies-schema).
pub const MAX_BYTES: u32 = 524_288;

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Declared locally in each hook plugin crate (systemic pattern across
/// all native-WASM hook plugins — each plugin statically declares the ABI
/// version it compiled against so the dispatcher can reject mismatched plugins
/// at load time).
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Target arm discrimination
// ---------------------------------------------------------------------------

/// Which file type triggered the PostToolUse event.
///
/// Determined by path-component-strict basename detection (BC-5.39.009 inv-3).
#[derive(Debug, PartialEq)]
enum TargetArm {
    /// `.factory/STATE.md` — Block severity on missing sites.
    State,
    /// Active-cycle `INDEX.md` — advisory severity.
    Index,
    /// `burst-log.md` — advisory severity.
    BurstLog,
    /// `lessons.md` — advisory pass-through (PC10 OUT-OF-SCOPE).
    Lessons,
}

/// Determine which arm (if any) should handle `file_path`.
///
/// Uses `Path::file_name()` for basename extraction (NOT `ends_with` —
/// inv-3 path-component-strict). Uses `Path::components()` walk for
/// `.factory/` parent-guard (BC v1.8 Precondition 4; F-SP3-008 mirror).
fn target_arm(file_path: &str) -> Option<TargetArm> {
    todo!()
}

// ---------------------------------------------------------------------------
// Arrow-count scanner (manual; regex deliberately absent — story U6 mandate)
// ---------------------------------------------------------------------------

/// Count `→(\d+)` arrow-segments in an ALREADY SCOPED segment.
///
/// Callers (PC1/PC2/PC4/PC5) MUST first locate the `trajectory-tail ` marker
/// and pass only the sub-string from marker-end to the first `;` (or
/// end-of-text) to this counter.
///
/// CRITICAL: uses `count == 4` equality semantics (not `>= 4`) per inv-4
/// STRICT — LENGTH=5+ sequences are equally a violation (EC-018).
///
/// `→` is U+2192 encoded as `[0xE2, 0x86, 0x92]` (3 UTF-8 bytes).
fn count_trajectory_arrows(text: &str) -> usize {
    todo!()
}

/// Returns `true` if `segment` contains exactly 4 `→\d+` arrow-segments.
///
/// `segment` MUST be the already-scoped text from marker-end to the first `;`
/// (or end-of-text). Do NOT pass full cell/value text — callers apply the
/// inv-4 two-step marker-prefix check before invoking this.
fn has_trajectory_tail(segment: &str) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// STATE.md extraction functions (5 sites — Block severity)
// ---------------------------------------------------------------------------

/// Represents a STATE.md site that is missing its trajectory_tail.
struct MissingStateSite {
    site_name: &'static str,
}

/// PC1: Extract `current_step:` value from YAML frontmatter region.
///
/// Scans bytes between first `---\n` and second `---\n` for a line matching
/// `^current_step:`. Handles multi-line YAML block-scalar (`|` or `>`) by
/// joining continuation lines into the full logical value (EC-017, EC-018).
/// If frontmatter absent: log_warn + return None (fail-open per EC-016).
/// If key absent: return None (treat as missing site → Block).
fn extract_frontmatter_current_step(content: &str) -> Option<String> {
    todo!()
}

/// PC2: Extract the `| **Last Updated** |` table row cell value.
///
/// Scans for the markdown table row pattern `| **Last Updated** |` within the
/// STATE.md body; captures the second pipe-delimited column value.
/// NOTE: there is NO `## Last Updated` heading in production STATE.md — do
/// NOT scan for that heading. If the row is absent: return None (Block).
/// inv-4 two-step marker-prefix check applied by caller (`check_state_md`).
fn extract_last_updated_cell(content: &str) -> Option<&str> {
    todo!()
}

/// PC3: Extract the latest (bottommost non-archived) Phase Progress table row.
///
/// Scans for `## Phase Progress` heading; captures table rows until the next
/// `##` heading; returns the SINGLE bottommost non-archived/non-compacted row.
/// Skip rows whose Status cell contains only "ARCHIVED" or "COMPACTED".
/// Do NOT skip "COMPLETE", "SHIPPED", "MERGED", or "CYCLE CLOSED".
/// If all rows are archived/compacted: treat site as present (pass-through).
fn extract_phase_progress_latest_row(content: &str) -> Option<String> {
    todo!()
}

/// PC4: Extract the latest (bottommost active) Concurrent Cycles table row.
///
/// Scans for `## Concurrent Cycles` heading; returns SINGLE bottommost
/// active/in-progress row (skips rows with "CLOSED", "COMPACTED", "ARCHIVED"
/// in Status cell). ONE-tail-per-extracted-region: single row text only.
/// If section absent or all rows closed/compacted/archived: pass-through.
fn extract_concurrent_cycles_latest_row(content: &str) -> Option<String> {
    todo!()
}

/// PC5: Extract the `### §1.` sub-section body under `## Session Resume Checkpoint`.
///
/// Matches by PREFIX `## Session Resume Checkpoint` (NOT exact match) to
/// tolerate the evolving parenthetical suffix. Captures the `### §1.` block
/// (first `### §1.` heading to the next `###` or `##` heading).
/// If heading or sub-section absent: return None (Block).
fn extract_session_resume_section_1(content: &str) -> Option<&str> {
    todo!()
}

/// Run all 5 STATE.md site extractors and collect missing sites.
///
/// Cascade accumulator: never short-circuits — collects ALL missing sites so
/// the caller can emit a single Block enumerating them all (inv-8).
fn check_state_md(content: &str) -> Vec<MissingStateSite> {
    todo!()
}

// ---------------------------------------------------------------------------
// Non-STATE.md site check functions (advisory severity)
// ---------------------------------------------------------------------------

/// Represents a non-STATE.md site advisory warning.
struct AdvisoryWarning {
    message: String,
}

/// PC7 + PC8: Check INDEX.md for missing trajectory_tail in prescribed sites.
///
/// Site 6: Convergence Status row — scan for `Convergence Status` row in table.
/// Site 7: adversarial-review summary-table latest-pass row — scan for latest
/// pass row. Advisory: log_warn per missing site; Continue always.
///
/// NOTE: The INDEX.md arm also performs dynamic cycle-path resolution via a
/// secondary `host::read_file` on STATE.md to obtain `current_cycle:` from
/// frontmatter via `extract_current_cycle()` (F-SP3-001 + F-SP4-006). This
/// secondary read and path-component-walk cycle guard happens in
/// `on_post_tool_use` BEFORE calling this function.
fn check_index_sites(content: &str) -> Vec<AdvisoryWarning> {
    todo!()
}

/// PC9: Check burst-log.md for missing trajectory_tail in Dim-7 block.
///
/// Scans all `^### Dim-7` headings; selects the BOTTOMMOST occurrence.
/// Block = text from that heading up to (not including) next `^## ` or `^### `.
/// Applies inv-4 two-step marker-prefix check to block text.
/// If `^### Dim-7` is absent: treat as PRESENT (fail-open per inv-10) + log_warn.
/// Advisory: log_warn + Continue if marker absent or count != 4 (NOT Block; inv-6).
fn check_burst_log_sites(content: &str) -> Vec<AdvisoryWarning> {
    todo!()
}

/// PC10 (OUT-OF-SCOPE): Advisory pass-through for lessons.md arm.
///
/// PC10 is out-of-scope per BC v1.8 F-SP5-003 (lessons.md trend-table is
/// inline prose, not a machine-extractable table). Always returns a single
/// log_warn advisory and Continue — no blocking or count check runs.
fn check_lessons_sites(content: &str) -> Vec<AdvisoryWarning> {
    todo!()
}

// ---------------------------------------------------------------------------
// Dynamic cycle resolution (INDEX.md arm secondary read)
// ---------------------------------------------------------------------------

/// Extract `current_cycle:` value from STATE.md YAML frontmatter.
///
/// Used by the INDEX.md arm to perform dynamic cycle-path resolution at
/// runtime (F-SP3-001 + F-SP4-006; BC v1.8 Precondition 4). Returns None
/// if frontmatter is absent or the key is not found — callers fall back to
/// fail-open (Continue + log_warn) for the INDEX.md arm only.
fn extract_current_cycle(content: &str) -> Option<String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Hook entry point (effectful)
// ---------------------------------------------------------------------------

/// PostToolUse hook entry point.
///
/// Reads the target file (max_bytes = MAX_BYTES), decodes as UTF-8 (inv-13),
/// dispatches to the correct arm, and returns Block or Continue.
///
/// DOUBLE-MATCH pattern (F-SP3-006 + BC v1.8 inv-13 encoding gate):
/// `host::read_file` returns `Vec<u8>`; decode via `String::from_utf8`
/// BEFORE passing to section extractors. UTF-8 failure → EC-020 fail-open.
///
/// INDEX.md arm performs a secondary `host::read_file` on `.factory/STATE.md`
/// to resolve the active cycle path via `extract_current_cycle()`, then
/// applies PATH-COMPONENT-WALK cycle guard (F-SP4-006; FORBIDDEN: substring
/// check via `contains`).
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    todo!()
}

// ---------------------------------------------------------------------------
// Unit tests — Red Gate phase
//
// These tests exercise the pure extractor + counter functions defined above.
// All functions are currently todo!() stubs, so every test will panic at
// runtime. This is the expected Red Gate state: the tests COMPILE but FAIL
// (panic from todo!() = "not yet implemented"). Tests go GREEN once the
// implementer fills in the function bodies.
//
// Test naming: test_BC_5_39_009_<assertion>() per DF-TestWriter convention.
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // count_trajectory_arrows — BC-5.39.009 inv-4 STRICT equality semantics
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-4: LENGTH=0 (empty string) => count=0 (not 4 => absent)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_empty_segment() {
        // todo!() will panic — that is the Red Gate failure mode for stub functions
        let count = count_trajectory_arrows("");
        assert_eq!(count, 0, "empty segment must count 0 arrows");
    }

    /// BC-5.39.009 inv-4: single arrow →9 => count=1
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_single() {
        let count = count_trajectory_arrows("→9");
        assert_eq!(count, 1);
    }

    /// BC-5.39.009 inv-4: 3 arrows →9→9→9 => count=3 (LENGTH=3 ≠ 4 => absent)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_3() {
        let count = count_trajectory_arrows("→9→9→9");
        assert_eq!(count, 3, "LENGTH=3 must count 3 arrows");
    }

    /// BC-5.39.009 inv-4: 4 arrows →9→9→9→9 => count=4 (LENGTH=4 STRICT => present)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_4() {
        let count = count_trajectory_arrows("→9→9→9→9");
        assert_eq!(count, 4, "LENGTH=4 canonical tail must count 4 arrows");
    }

    /// BC-5.39.009 inv-4: 5 arrows →9→9→9→9→9 => count=5 (LENGTH=5 ≠ 4 => absent per EC-018)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_5() {
        let count = count_trajectory_arrows("→9→9→9→9→9");
        assert_eq!(count, 5, "LENGTH=5 must count 5 arrows");
    }

    /// BC-5.39.009 inv-4: multi-digit values →10→12→11→13 => count=4 (valid per EC-013)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_multi_digit() {
        let count = count_trajectory_arrows("→10→12→11→13");
        assert_eq!(count, 4, "multi-digit LENGTH=4 tail must count 4 arrows");
    }

    /// BC-5.39.009 inv-4: plain text with no arrows => count=0
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_no_arrows() {
        let count = count_trajectory_arrows("Phase 3 step 42 no tail");
        assert_eq!(count, 0);
    }

    /// BC-5.39.009 inv-4: arrow without digits (malformed) does NOT count
    /// The byte-walk should only count valid →\d+ segments (arrow + at least one digit)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_arrow_no_digit_not_counted() {
        // "→ abc" — arrow not followed by digit; must not count
        let count = count_trajectory_arrows("→ abc");
        assert_eq!(count, 0, "arrow not followed by digit must not count");
    }

    // -----------------------------------------------------------------------
    // has_trajectory_tail — BC-5.39.009 inv-4 STRICT equality (count == 4)
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-4: has_trajectory_tail("→9→9→9→9") == true (LENGTH=4 STRICT)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_length_4_returns_true() {
        let result = has_trajectory_tail("→9→9→9→9");
        assert!(result, "→9→9→9→9 must be recognized as present (count==4)");
    }

    /// BC-5.39.009 inv-4: has_trajectory_tail("→9→9→9") == false (LENGTH=3 ≠ 4)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_length_3_returns_false() {
        let result = has_trajectory_tail("→9→9→9");
        assert!(
            !result,
            "→9→9→9 (LENGTH=3) must NOT be recognized as present"
        );
    }

    /// BC-5.39.009 EC-018 + inv-4: has_trajectory_tail("→9→9→9→9→9") == false (LENGTH=5 ≠ 4)
    /// CRITICAL REGRESSION GUARD: non-anchored (→[0-9]+){4} match would falsely pass on LENGTH=5
    #[test]
    fn test_BC_5_39_009_EC018_has_tail_length_5_returns_false() {
        let result = has_trajectory_tail("→9→9→9→9→9");
        assert!(
            !result,
            "→9→9→9→9→9 (LENGTH=5) must NOT pass — equality semantics required (count==4 not >=4)"
        );
    }

    /// BC-5.39.009 inv-4: has_trajectory_tail("") == false (empty => count=0 ≠ 4)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_empty_returns_false() {
        let result = has_trajectory_tail("");
        assert!(!result, "empty segment must not have tail");
    }

    /// BC-5.39.009 EC-013: multi-digit values →10→12→11→13 pass (count=4)
    #[test]
    fn test_BC_5_39_009_EC013_has_tail_multi_digit_returns_true() {
        let result = has_trajectory_tail("→10→12→11→13");
        assert!(
            result,
            "multi-digit →10→12→11→13 must be recognized as present (count==4)"
        );
    }

    // -----------------------------------------------------------------------
    // target_arm — BC-5.39.009 inv-3 path-component-strict + Precondition 4
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-3 + Precondition 4: ".factory/STATE.md" => State arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_factory_state_md() {
        let arm = target_arm(".factory/STATE.md");
        assert!(
            matches!(arm, Some(TargetArm::State)),
            ".factory/STATE.md must resolve to State arm"
        );
    }

    /// BC-5.39.009 EC-019 + Precondition 4: non-factory STATE.md => None (Continue)
    #[test]
    fn test_BC_5_39_009_EC019_target_arm_non_factory_state_md_returns_none() {
        // Path lacks .factory component — Precondition 4 parent-guard must reject it
        let arm = target_arm("some-other/STATE.md");
        assert!(
            arm.is_none(),
            "STATE.md without .factory component must NOT trigger State arm"
        );
    }

    /// BC-5.39.009 EC-015: /factory-artifacts/STATE.md has basename STATE.md but no .factory component
    #[test]
    fn test_BC_5_39_009_EC015_target_arm_factory_artifacts_path_not_factory_component() {
        // "factory-artifacts" is NOT the ".factory" component — path-component-walk must reject
        let arm = target_arm("factory-artifacts/STATE.md");
        assert!(
            arm.is_none(),
            "factory-artifacts/STATE.md must NOT trigger State arm (no .factory component)"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/INDEX.md" => Index arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_factory_index_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/INDEX.md");
        assert!(
            matches!(arm, Some(TargetArm::Index)),
            ".factory/.../INDEX.md must resolve to Index arm"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/burst-log.md" => BurstLog arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_burst_log_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/burst-log.md");
        assert!(
            matches!(arm, Some(TargetArm::BurstLog)),
            "burst-log.md must resolve to BurstLog arm"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/lessons.md" => Lessons arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_lessons_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/lessons.md");
        assert!(
            matches!(arm, Some(TargetArm::Lessons)),
            "lessons.md must resolve to Lessons arm"
        );
    }

    /// BC-5.39.009 inv-3 + EC-005: ".factory/not-STATE.md" => None (Continue immediately)
    #[test]
    fn test_BC_5_39_009_EC005_target_arm_wrong_basename_returns_none() {
        let arm = target_arm(".factory/not-STATE.md");
        assert!(
            arm.is_none(),
            "not-STATE.md must NOT trigger any arm (basename mismatch)"
        );
    }

    /// BC-5.39.009 inv-3: "STORY-INDEX.md" not in any target set => None
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_story_index_returns_none() {
        let arm = target_arm(".factory/stories/STORY-INDEX.md");
        assert!(arm.is_none(), "STORY-INDEX.md must NOT trigger any arm");
    }

    // -----------------------------------------------------------------------
    // extract_frontmatter_current_step — BC-5.39.009 PC1
    // -----------------------------------------------------------------------

    /// BC-5.39.009 PC1: standard inline current_step with trajectory-tail marker => Some(value)
    #[test]
    fn test_BC_5_39_009_PC1_extract_frontmatter_current_step_inline_with_marker() {
        let content = "---\ndocument_type: state\ncurrent_step: \"Phase 3 — trajectory-tail →9→9→9→9; done\"\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_some(),
            "should extract current_step value from frontmatter"
        );
        let val = result.expect("current_step value must be Some after is_some() assertion");
        assert!(
            val.contains("trajectory-tail"),
            "extracted value must contain the trajectory-tail marker"
        );
    }

    /// BC-5.39.009 PC1 / EC-001: current_step key absent => None (treat as missing site)
    #[test]
    fn test_BC_5_39_009_EC001_extract_frontmatter_current_step_absent_key_returns_none() {
        let content = "---\ndocument_type: state\nversion: \"1.0\"\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_none(),
            "absent current_step key must return None (treat as missing site)"
        );
    }

    /// BC-5.39.009 PC1 / EC-016: frontmatter absent (no --- delimiters) => None (fail-open)
    #[test]
    fn test_BC_5_39_009_EC016_extract_frontmatter_no_delimiters_returns_none() {
        let content = "No frontmatter at all — just body text.\nSome content here.";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_none(),
            "absent frontmatter region must return None (fail-open per EC-016)"
        );
    }

    /// BC-5.39.009 PC1 / EC-017: multi-line block-scalar current_step => joined value returned
    #[test]
    fn test_BC_5_39_009_EC017_extract_frontmatter_current_step_multiline_block_scalar() {
        // Using | block scalar with continuation line containing the trajectory-tail marker
        let content = "---\ndocument_type: state\ncurrent_step: |\n  Phase 3 step.\n  trajectory-tail →9→9→9→9; continuation line.\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_some(),
            "multi-line block-scalar current_step must be extracted"
        );
        let val =
            result.expect("multi-line current_step value must be Some after is_some() assertion");
        assert!(
            val.contains("trajectory-tail"),
            "joined multi-line value must contain trajectory-tail marker"
        );
    }

    // -----------------------------------------------------------------------
    // extract_current_cycle — BC-5.39.009 Precondition 4 / F-SP3-001
    // -----------------------------------------------------------------------

    /// BC-5.39.009 Precondition 4: standard current_cycle: value extracted
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_bare_value() {
        let content =
            "---\ndocument_type: state\ncurrent_cycle: v1.0-brownfield-backfill\n---\n\nbody";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0-brownfield-backfill".to_string()),
            "bare current_cycle value must be extracted correctly"
        );
    }

    /// BC-5.39.009 Precondition 4: quoted current_cycle value extracted
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_quoted_value() {
        let content = "---\ncurrent_cycle: \"v1.0-brownfield-backfill\"\n---\n";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0-brownfield-backfill".to_string()),
            "quoted current_cycle value must be extracted (quotes stripped)"
        );
    }

    /// BC-5.39.009 Precondition 4: absent current_cycle key => None
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_absent_returns_none() {
        let content = "---\ndocument_type: state\n---\n\nbody";
        let result = extract_current_cycle(content);
        assert!(
            result.is_none(),
            "absent current_cycle key must return None"
        );
    }

    // -----------------------------------------------------------------------
    // check_state_md — BC-5.39.009 PC1-6 + inv-8 cascade
    // -----------------------------------------------------------------------

    /// BC-5.39.009 PC12: all 5 sites present => empty Vec (no missing sites)
    #[test]
    fn test_BC_5_39_009_PC12_check_state_md_all_sites_present_empty_vec() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9; done\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; good |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; all sites present.
";
        let missing = check_state_md(content);
        assert!(
            missing.is_empty(),
            "all sites present => no missing sites (PC12 pass case)"
        );
    }

    /// BC-5.39.009 PC1 + PC6: current_step missing tail => Vec with 1 site named "frontmatter current_step"
    #[test]
    fn test_BC_5_39_009_PC1_check_state_md_frontmatter_missing_returns_site() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 step — no tail\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; good |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; session resume OK.
";
        let missing = check_state_md(content);
        assert_eq!(
            missing.len(),
            1,
            "exactly 1 site should be missing (current_step)"
        );
        assert!(
            missing[0].site_name.contains("current_step"),
            "missing site_name must reference current_step"
        );
    }

    /// BC-5.39.009 PC6 + inv-8: multiple missing sites => Vec with all missing (cascade)
    #[test]
    fn test_BC_5_39_009_invariant_8_check_state_md_cascade_accumulates_all_sites() {
        // 3 sites missing: Last Updated (site 2), Concurrent Cycles (site 4), Session Resume (site 5)
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9; OK\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — no trajectory tail here |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | no trajectory tail here |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

No trajectory tail in session resume section 1.
";
        let missing = check_state_md(content);
        assert!(
            missing.len() >= 2,
            "cascade must accumulate ALL missing sites (not short-circuit at first); got {} missing",
            missing.len()
        );
    }

    // -----------------------------------------------------------------------
    // EC-018 regression guard — BC-5.39.009 inv-4 LENGTH=5 scoped count
    // -----------------------------------------------------------------------

    /// BC-5.39.009 EC-018: current_step with LENGTH=5 scoped count => site missing (Block)
    /// This is the CRITICAL regression guard: naive (→[0-9]+){4} non-anchored match
    /// would falsely pass on "trajectory-tail →9→9→9→9→9" (matching first 4 of 5 arrows).
    /// Equality count == 4 MUST reject count == 5.
    #[test]
    fn test_BC_5_39_009_EC018_check_state_md_length_5_in_current_step_blocks() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9→9; LENGTH=5 violation\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; OK |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; session resume OK.
";
        let missing = check_state_md(content);
        assert_eq!(
            missing.len(),
            1,
            "LENGTH=5 in current_step must block (count=5 ≠ 4; equality semantics)"
        );
        assert!(
            missing[0].site_name.contains("current_step"),
            "missing site must be current_step for LENGTH=5 violation"
        );
    }
}
