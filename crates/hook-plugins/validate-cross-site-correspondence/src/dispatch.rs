//! dispatch.rs — File-path classification for validate-cross-site-correspondence.
//!
//! Pure-core module (ADR-035 §Decision 1): path-matching only; no I/O.
//!
//! All guards use `Path::new(file_path).components()` — raw `ends_with` or
//! `contains` on the path string is FORBIDDEN per BC-5.39.010 invariant 3
//! (path-component-strict matching, same requirement as BC-5.39.008 precedent).
//!
//! # BC trace
//! BC-5.39.010 preconditions 1, 9, 16, 22, 28, 34 — file-path trigger conditions.
//! BC-5.39.010 invariant 3 — path-component-strict matching.
//! ADR-035 §Decision 1 — Tier 2A read-only PostToolUse validator.

/// Which category of cycle artifact a triggered write maps to.
///
/// **[DEFERRED v1.6 — Class D]**: `is_cycle_artifact` always returns `None` per
/// BC-5.39.010 v1.6 / D-953. Variants are constructed only inside `#[ignore]`d tests.
/// Retained for S-21.08 Phase 2 re-enablement alongside `arm_d.rs`.
///
/// Used by Arm D to determine the scope-limited extraction strategy
/// (BC-5.39.010 precondition 30).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleArtifactKind {
    /// `burst-log.md`: scope = last H2 section (from last `^## ` heading through EOF).
    BurstLog,
    /// `lessons.md`: scope = last `^L-EDP1-[0-9]+-[0-9]+:` anchor block; fallback last 200 lines.
    Lessons,
    /// `INDEX.md` (under `.factory/cycles/`): scope = `## Adversarial Reviews` section only.
    CycleIndex,
}

/// Returns `true` if `filename` matches the canonical BC filename shape:
/// `^BC-[0-9]+\.[0-9]+\.[0-9]+\.md$` (three dot-separated numeric groups).
///
/// Excludes BC-INDEX.md and any other non-contract files that merely start with "BC-".
///
/// # BC trace
/// BC-5.39.010 PC1 — bc_id shape guard; F-S2107-P1B-005 (BC-INDEX.md exclusion).
fn is_canonical_bc_filename(filename: &str) -> bool {
    // Must start with "BC-" and end with ".md"
    let inner = match filename
        .strip_prefix("BC-")
        .and_then(|s| s.strip_suffix(".md"))
    {
        Some(s) => s,
        None => return false,
    };
    // inner must be N.N.NNN format: exactly three dot-separated non-empty digit groups
    let parts: Vec<&str> = inner.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

/// Returns `true` if `file_path` names a BC file under
/// `.factory/specs/behavioral-contracts/` using path-component-strict matching.
///
/// A BC file has:
/// - A `.factory` path component AND
/// - A `specs` path component AND
/// - A `behavioral-contracts` path component AND
/// - A filename matching `BC-*.md`
///
/// # BC trace
/// BC-5.39.010 precondition 1 (Class A Arm1 trigger condition).
/// BC-5.39.010 invariant 3 (path-component-strict — NOT `ends_with` / `contains`).
pub fn is_bc_file(file_path: &str) -> bool {
    use std::path::{Component, Path};
    let path = Path::new(file_path);
    let components: Vec<_> = path.components().collect();
    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    let has_specs = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "specs"));
    let has_bc = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "behavioral-contracts"));
    // BC-5.39.010 PC1: filename must match ^BC-[0-9]+\.[0-9]+\.[0-9]+\.md$ — three
    // dot-separated numeric groups. This excludes BC-INDEX.md (F-S2107-P1B-005)
    // and any other non-contract files that merely start with "BC-".
    let filename_ok = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(is_canonical_bc_filename)
        .unwrap_or(false);
    has_factory && has_specs && has_bc && filename_ok
}

/// Returns `true` if `filename` is a canonical story basename: `^S-[0-9]+\.[0-9]+`.
///
/// Admits `S-21.07-name.md`, `S-1.1.md`, etc. Rejects `S-README.md` (no digit before
/// the dot) and `S-ARCH.md` (no dot at all).
///
/// # BC trace
/// BC-5.39.010 PC9/PC16: story basename pattern `^S-[0-9]+\.[0-9]+`.
/// F-P2-011: prior `starts_with("S-")` admitted S-README.md, S-ARCH.md, etc.
fn is_canonical_story_basename(filename: &str) -> bool {
    // Strip "S-" prefix
    let rest = match filename.strip_prefix("S-") {
        Some(r) => r,
        None => return false,
    };
    // Find the first dot; there must be at least one digit before it
    let dot_pos = match rest.find('.') {
        Some(p) if p > 0 => p,
        _ => return false,
    };
    // All characters before the dot must be ASCII digits
    if !rest[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // The character immediately after the dot must be an ASCII digit
    matches!(rest[dot_pos + 1..].chars().next(), Some(c) if c.is_ascii_digit())
}

/// Returns `true` if `file_path` names a story file under `.factory/stories/`
/// using path-component-strict matching.
///
/// A story file has:
/// - A `.factory` path component AND
/// - A `stories` path component AND
/// - Filename ending in `.md` (but NOT `STORY-INDEX.md` — that is Arm B2's trigger)
/// - Basename matching `^S-[0-9]+\.[0-9]+` (canonical story ID, excluding S-README etc.)
///
/// # BC trace
/// BC-5.39.010 precondition 9 (Class A Arm2 trigger condition).
/// BC-5.39.010 precondition 16 (Class B Arm1 trigger condition).
/// BC-5.39.010 invariant 3.
/// F-P2-011: canonical story basename requires numeric section.subsection.
pub fn is_story_file(file_path: &str) -> bool {
    use std::path::{Component, Path};
    let path = Path::new(file_path);
    let components: Vec<_> = path.components().collect();
    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    let has_stories = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "stories"));
    let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    let is_md = filename.ends_with(".md");
    // STORY-INDEX.md is Arm B2's trigger, not a story file
    let not_index = filename != "STORY-INDEX.md";
    // PC9/PC16+F-P2-011: canonical story basename (^S-[0-9]+\.[0-9]+) excludes
    // epics (E-XX-*), index files, and non-story S-README/S-ARCH etc.
    let is_story_basename = is_canonical_story_basename(filename);
    has_factory && has_stories && is_md && not_index && is_story_basename
}

/// Returns `true` if `file_path` names `STORY-INDEX.md` under `.factory/stories/`.
///
/// Uses path-component-strict matching; `file_name == "STORY-INDEX.md"` AND
/// a `stories` path component AND a `.factory` path component.
///
/// # BC trace
/// BC-5.39.010 precondition 22 (Class B Arm2 trigger condition).
/// BC-5.39.010 invariant 3.
pub fn is_story_index(file_path: &str) -> bool {
    use std::path::{Component, Path};
    let path = Path::new(file_path);
    let components: Vec<_> = path.components().collect();
    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    let has_stories = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "stories"));
    let is_story_index = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f == "STORY-INDEX.md")
        .unwrap_or(false);
    has_factory && has_stories && is_story_index
}

/// Returns `Some(CycleArtifactKind)` if `file_path` names a cycle artifact
/// under `.factory/cycles/` that Arm D should scan.
///
/// **[DEFERRED v1.6 — Class D]**: BC-5.39.010 v1.6 descopes Class D (D-953,
/// human-approved). This function always returns `None` so cycle artifact writes
/// are unclassified → `HookResult::Continue` (no primary read attempted).
/// `.factory/cycles/` is removed from `path_allow` as part of this deferral.
///
/// When Class D is re-enabled, restore the body from git history and add
/// `.factory/cycles/` back to path_allow in hooks-registry.toml.
///
/// # BC trace
/// BC-5.39.010 precondition 28 (Class D trigger condition — DEFERRED).
/// BC-5.39.010 invariant 3.
pub fn is_cycle_artifact(_file_path: &str) -> Option<CycleArtifactKind> {
    // [DEFERRED v1.6 — Class D]: cycle artifact dispatch removed per BC-5.39.010 v1.6.
    // Cycle artifact writes are unclassified → Continue (no primary read attempted).
    None
}

/// Returns `true` if `filename` is a canonical VP basename: `^VP-[0-9]+\.md$`.
///
/// Admits `VP-039.md`, `VP-100.md`, etc. Rejects `VP-INDEX.md` (explicit basename guard
/// per PC34 REQUIRED defence-in-depth) and `VP-9999-test.md` (inner "9999-test" contains
/// a non-digit).
///
/// # BC trace
/// BC-5.39.010 PC34: VP canonical filename pattern `^VP-[0-9]+\.md$`.
/// F-P2-003+F-P2-008: prior `starts_with("VP-") && ends_with(".md")` admitted VP-INDEX.md.
/// F-S2107-P3-011: explicit `filename == "VP-INDEX.md"` guard REQUIRED for defence-in-depth
/// per PC34 bullet 2 (normative "REQUIRED") — digit predicate excludes VP-INDEX.md today
/// but a future relaxation of the digit predicate would silently re-admit it without this guard.
fn is_canonical_vp_filename(filename: &str) -> bool {
    // Defence-in-depth guard: explicitly reject VP-INDEX.md regardless of digit predicate.
    // PC34 bullet 2 (normative "REQUIRED"): "an explicit basename guard
    // `file_name() != "VP-INDEX.md"` is REQUIRED for defence-in-depth".
    if filename == "VP-INDEX.md" {
        return false;
    }
    let inner = filename
        .strip_prefix("VP-")
        .and_then(|s| s.strip_suffix(".md"))
        .unwrap_or("");
    !inner.is_empty() && inner.chars().all(|c| c.is_ascii_digit())
}

/// Returns `true` if `file_path` is a BC, VP, story, or epic file (Class E trigger).
///
/// Class E (frontmatter parity) fires on any file with `version:` and `last_amended:`
/// frontmatter fields: BC files, VP files, story files, and epic files.
///
/// # BC trace
/// BC-5.39.010 precondition 34 (Class E trigger condition).
pub fn is_frontmatter_parity_target(file_path: &str) -> bool {
    use std::path::{Component, Path};
    // BC files
    if is_bc_file(file_path) {
        return true;
    }
    // Story files (but not STORY-INDEX.md)
    if is_story_file(file_path) {
        return true;
    }
    // VP files under .factory/specs/verification-properties/
    let path = Path::new(file_path);
    let components: Vec<_> = path.components().collect();
    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    let has_specs = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "specs"));
    let has_vp = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "verification-properties"));
    let vp_filename_ok = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(is_canonical_vp_filename)
        .unwrap_or(false);
    if has_factory && has_specs && has_vp && vp_filename_ok {
        return true;
    }
    // Epic files under .factory/stories/epics/ — PC34 bullet 4 (F-S2107-P3-009):
    // requires THREE components (`.factory`, `stories`, `epics`) AND basename
    // matching `^E-[0-9]+-.*\.md$`. Absent either component or wrong basename → false.
    let has_stories = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "stories"));
    let has_epics = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "epics"));
    let epic_filename_ok = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(is_canonical_epic_basename)
        .unwrap_or(false);
    has_factory && has_stories && has_epics && epic_filename_ok
}

/// Returns `true` if `filename` is a canonical epic basename: `^E-[0-9]+-.*\.md$`.
///
/// Admits `E-21-name.md`, `E-1-foo.md`. Rejects `README.md`, `E-.md`, `E-21.md`
/// (missing `-` separator after digits), `E-21-name.txt`.
///
/// # BC trace
/// BC-5.39.010 PC34 bullet 4: epic basename pattern `^E-[0-9]+-.*\.md$`.
/// F-S2107-P3-009: prior `ends_with(".md")` admitted README.md and any .md file.
fn is_canonical_epic_basename(filename: &str) -> bool {
    let Some(rest) = filename.strip_prefix("E-") else {
        return false;
    };
    let bytes = rest.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Digits must be followed by '-'
    if i >= bytes.len() || bytes[i] != b'-' {
        return false;
    }
    // Remainder must end with ".md"
    filename.ends_with(".md")
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // is_bc_file — BC-5.39.010 precondition 1 + invariant 3
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_dispatch_bc_file_factory_path_detected() {
        // BC-5.39.010 PC1: canonical BC path under .factory/specs/behavioral-contracts/
        let result = is_bc_file(".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md");
        assert!(result, "canonical BC path must be detected");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_bc_file_non_factory_path_rejected() {
        // Invariant 3: path-component-strict — missing .factory component must reject
        let result = is_bc_file("specs/behavioral-contracts/ss-05/BC-5.39.010.md");
        assert!(!result, "path without .factory component must not match");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_story_file_detected() {
        // BC-5.39.010 PC9: story file under .factory/stories/
        let result =
            is_story_file(".factory/stories/S-21.07-validate-cross-site-correspondence.md");
        assert!(result, "canonical story path must be detected");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_story_index_not_story_file() {
        // STORY-INDEX.md must NOT match is_story_file (it's Arm B2's trigger)
        let result = is_story_file(".factory/stories/STORY-INDEX.md");
        assert!(
            !result,
            "STORY-INDEX.md must not be classified as a story file"
        );
    }

    #[test]
    fn test_BC_5_39_010_dispatch_story_index_detected() {
        // BC-5.39.010 PC22: STORY-INDEX.md trigger
        let result = is_story_index(".factory/stories/STORY-INDEX.md");
        assert!(result, "STORY-INDEX.md must be detected as story index");
    }

    // ADV-RECON-005: the former test_BC_5_39_010_dispatch_burst_log_detected and
    // test_BC_5_39_010_dispatch_lessons_md_detected `#[ignore]`d tests asserted
    // `Some(CycleArtifactKind::BurstLog/Lessons)` — directly contradicting the active
    // Class-D-deferred `..._returns_none` tests below (which assert `is_cycle_artifact`
    // returns `None` for the SAME two paths, per BC-5.39.010 v1.6 / D-953 Class D
    // deferral). An `#[ignore]`d test that asserts the OPPOSITE of current normative
    // behavior is dead, misleading test content — removed rather than retained
    // ignored. The `returns_none` assertions below are the sole active source of
    // truth for `is_cycle_artifact` until S-21.08 re-enables Class D.

    #[test]
    fn test_BC_5_39_010_dispatch_non_cycle_index_md_rejected() {
        // .factory/stories/STORY-INDEX.md must not trigger the cycle artifact arm
        let result = is_cycle_artifact(".factory/stories/STORY-INDEX.md");
        assert!(
            result.is_none(),
            "stories/STORY-INDEX.md must not classify as cycle artifact"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-005: BC-INDEX.md must not be classified as a BC file.
    // is_bc_file uses `starts_with("BC-") && ends_with(".md")` which admits
    // "BC-INDEX.md" — but BC-INDEX.md is the index file, not a behavioral contract.
    // When classified as a BC file, arm A1 runs with BC-INDEX.md as primary target,
    // treating "BC-INDEX.md" as the bc_id, producing spurious violations.
    // BC-5.39.010 v1.19 §Classification invariant: bc_id guard excludes index.
    // -----------------------------------------------------------------------

    /// T-035: BC-INDEX.md must NOT be classified as a BC file (F-S2107-P1B-005).
    ///
    /// RED GATE: current `starts_with("BC-") && ends_with(".md")` matches "BC-INDEX.md".
    /// `is_bc_file` returns true → assert!(!result) FAILS → RED gate.
    /// After fix (exclude index file by name): returns false → PASSES.
    #[test]
    fn test_BC_5_39_010_dispatch_bc_index_not_bc_file() {
        let result = is_bc_file(".factory/specs/behavioral-contracts/BC-INDEX.md");
        assert!(
            !result,
            "BC-INDEX.md must NOT be classified as a BC file — it is the index, \
            not a behavioral contract (F-S2107-P1B-005)"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-010: Epic file under .factory/stories/epics/ must not be
    // classified as a story file. is_story_file has no basename guard against
    // the epics/ subdirectory, so any .md file under .factory/stories/ passes.
    // BC-5.39.010 v1.19 §Classification invariant: story_id must match S-XX.YY format.
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-010: epic file in stories/epics/ must NOT be classified as story file.
    ///
    /// RED GATE: current code has no basename guard for `S-` prefix; any `.md` not named
    /// STORY-INDEX.md under `.factory/stories/` matches. `is_story_file` returns true for
    /// "E-21-W4.md" → assert!(!result) FAILS → RED gate.
    /// After fix (basename must start with "S-"): returns false → PASSES.
    #[test]
    fn test_BC_5_39_010_dispatch_epic_file_not_story_file() {
        let result = is_story_file(".factory/stories/epics/E-21-W4.md");
        assert!(
            !result,
            "epic file under .factory/stories/epics/ must NOT be classified as a story file \
            (F-S2107-P1B-010)"
        );
    }

    /// F-S2107-P1B-010: canonical story basename IS classified as story file.
    ///
    /// Complement test: verifies the story guard still fires for real story basenames
    /// after the basename guard is applied.
    #[test]
    fn test_BC_5_39_010_dispatch_story_file_s_prefix_basename_accepted() {
        let result =
            is_story_file(".factory/stories/S-21.07-validate-cross-site-correspondence.md");
        assert!(
            result,
            "canonical story file with S-XX.YY basename must be classified as story file"
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-003 + F-P2-008 (BLOCKER): VP-INDEX.md admitted by is_frontmatter_parity_target.
    //
    // Bug: vp_filename_ok uses `f.starts_with("VP-") && f.ends_with(".md")` which
    // admits "VP-INDEX.md". BC-5.39.010 PC34 requires the basename to match
    // `^VP-[0-9]+\.md$` and mandates an explicit `file_name() != "VP-INDEX.md"` guard.
    //
    // When VP-INDEX.md is classified as a frontmatter parity target, Class E1 runs
    // against it. VP-INDEX.md has no `version:` or `last_amended:` frontmatter →
    // precondition 37 fires → advisory + Continue (not blocking). But the primary-read
    // still fires and the spurious arm call wastes cycles and may emit noise.
    //
    // F-P2-003 and F-P2-008 both trace to the same root (flat VP path + INDEX guard).
    // -----------------------------------------------------------------------

    /// F-P2-003+F-P2-008 (BLOCKER): VP-INDEX.md must NOT be a frontmatter parity target.
    ///
    /// RED GATE: `f.starts_with("VP-") && f.ends_with(".md")` → true for "VP-INDEX.md" →
    /// is_frontmatter_parity_target returns true → `assert!(!result)` FAILS.
    /// After fix (guard `file_name() != "VP-INDEX.md"` OR digit-anchored regex): returns false.
    #[test]
    fn test_BC_5_39_010_dispatch_vp_index_excluded_from_class_e() {
        let result =
            is_frontmatter_parity_target(".factory/specs/verification-properties/VP-INDEX.md");
        assert!(
            !result,
            "VP-INDEX.md must NOT be classified as a frontmatter parity target. \
            BC-5.39.010 PC34: explicit VP-INDEX.md guard required. \
            F-P2-003+F-P2-008: starts_with('VP-')&&ends_with('.md') admits VP-INDEX.md. \
            RED GATE: current check returns true."
        );
    }

    /// F-P2-003+F-P2-008 complement: canonical VP file IS classified as parity target.
    ///
    /// Regression guard: verifies the guard does not over-exclude real VP files.
    /// This test PASSES in Red Gate (is_frontmatter_parity_target already returns true for
    /// canonical VP files).
    #[test]
    fn test_BC_5_39_010_dispatch_vp_canonical_file_accepted_as_class_e_target() {
        let result =
            is_frontmatter_parity_target(".factory/specs/verification-properties/VP-039.md");
        assert!(
            result,
            "canonical VP file VP-039.md must be classified as a frontmatter parity target \
            (Class E regression guard after VP-INDEX.md exclusion fix)"
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-011 (HIGH): is_story_file must require numeric section.subsection ID.
    //
    // Bug: `filename.starts_with("S-")` is too broad — admits S-README.md, S-ARCH.md,
    // S-TODO.md, S-NOTES.md, and any other `.md` file starting with "S-" that is not
    // a story. BC-5.39.010 PC9: story basename must match `^S-[0-9]+\.[0-9]+.*\.md$`.
    // -----------------------------------------------------------------------

    /// F-P2-011 (HIGH): S-README.md must not be classified as a story file.
    ///
    /// RED GATE: `"S-README.md".starts_with("S-")` → true → is_story_file returns true
    /// → `assert!(!result)` FAILS.
    /// After fix (regex guard `^S-[0-9]+\.[0-9]+`): "S-README.md" has no numeric part
    /// → returns false → PASSES.
    #[test]
    fn test_BC_5_39_010_dispatch_story_file_s_readme_rejected_requires_numeric_id() {
        let result = is_story_file(".factory/stories/S-README.md");
        assert!(
            !result,
            "S-README.md must NOT be classified as a story file. \
            PC9: basename must match ^S-[0-9]+\\.[0-9]+.*\\.md$ (has numeric section+subsection). \
            F-P2-011: starts_with('S-') too broad — admits S-README, S-ARCH, etc. \
            RED GATE: current check returns true."
        );
    }

    // -----------------------------------------------------------------------
    // Class D deferral (BC-5.39.010 v1.6): is_cycle_artifact MUST return None
    // for all paths once the Class D dispatch arm is removed.
    //
    // F-P2-007: invariant-6 I/O-BLOCK vs content-advisory split.
    // Resolution per BC-5.39.010 v1.6: Class D is DEFERRED. The implementer
    // removes the is_cycle_artifact dispatch, run_arm_d, and .factory/cycles/
    // from path_allow. After removal, cycle artifact writes are unclassified →
    // Continue (no primary read attempted).
    //
    // The EXISTING tests test_BC_5_39_010_dispatch_burst_log_detected and
    // test_BC_5_39_010_dispatch_lessons_md_detected assert Some(BurstLog/Lessons).
    // The implementer must update those tests to assert None (or DEFERRED-skip them)
    // when removing the Class D arm. Both cannot coexist with these new tests.
    // -----------------------------------------------------------------------

    /// [DEFERRED v1.6 — Class D] burst-log.md must return None after Class D removal.
    ///
    /// RED GATE: is_cycle_artifact currently returns Some(BurstLog) →
    /// `result.is_none()` is false → assertion FAILS.
    /// After fix (remove is_cycle_artifact cycle-artifact dispatch): returns None.
    #[test]
    fn test_BC_5_39_010_dispatch_class_d_deferred_burst_log_returns_none() {
        // BC-5.39.010 v1.6 Class D DEFERRED: is_cycle_artifact must return None.
        // Cycle artifact writes become unclassified → no primary read → Continue.
        // F-P2-007 resolution: Class D arm removed → is_cycle_artifact → None for all paths.
        let result =
            is_cycle_artifact(".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md");
        assert!(
            result.is_none(),
            "burst-log.md must NOT classify as cycle artifact after Class D deferral \
            (BC-5.39.010 v1.6 Class D DEFERRED). \
            F-P2-007 resolution: is_cycle_artifact dispatch removed. \
            RED GATE: currently returns Some(BurstLog)."
        );
    }

    /// [DEFERRED v1.6 — Class D] lessons.md must return None after Class D removal.
    ///
    /// RED GATE: is_cycle_artifact currently returns Some(Lessons) →
    /// `result.is_none()` is false → assertion FAILS.
    /// After fix: returns None → PASSES.
    #[test]
    fn test_BC_5_39_010_dispatch_class_d_deferred_lessons_returns_none() {
        let result =
            is_cycle_artifact(".factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md");
        assert!(
            result.is_none(),
            "lessons.md must NOT classify as cycle artifact after Class D deferral \
            (BC-5.39.010 v1.6 Class D DEFERRED). \
            RED GATE: currently returns Some(Lessons)."
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P3-009 (HIGH): is_frontmatter_parity_target epic arm defects
    //
    // PC34 bullet 4 requires epic files to satisfy THREE conditions:
    //   1. `.factory` component present
    //   2. `stories` component present  ← MISSING from current impl
    //   3. `epics` component present
    //   4. Basename matches `^E-[0-9]+-`  ← MISSING from current impl
    //
    // Current impl checks only `.factory` + `epics` + ends-with-`.md`.
    // Consequence: `.factory/epics/README.md` → true (should be false).
    // Blast radius: any non-story .md under a path with "epics" component fires Class E.
    // -----------------------------------------------------------------------

    /// F-S2107-P3-009 RED GATE: epic file without `stories` component must NOT be
    /// classified as a frontmatter parity target.
    ///
    /// `.factory/epics/E-21-test.md` lacks the `stories` path component (correct path
    /// is `.factory/stories/epics/E-21-test.md`). Current impl: has `.factory` + `epics`
    /// + ends `.md` → true. After fix (requires `stories`): false.
    ///
    /// RED GATE: current impl returns true → assert!(!result) FAILS.
    #[test]
    fn test_BC_5_39_010_dispatch_epic_missing_stories_component_rejected() {
        let result = is_frontmatter_parity_target(".factory/epics/E-21-test.md");
        assert!(
            !result,
            "'.factory/epics/E-21-test.md' lacks the `stories` path component and must \
            NOT be classified as a frontmatter parity target. \
            PC34 bullet 4: epic path requires both `stories` AND `epics` components. \
            F-S2107-P3-009: current impl checks only `.factory`+`epics`+`.md` → true. \
            RED GATE."
        );
    }

    /// F-S2107-P3-009 RED GATE: epic file without numeric basename must NOT be classified
    /// as a frontmatter parity target.
    ///
    /// `.factory/stories/epics/README.md` has the correct path components but
    /// `README.md` does not match `^E-[0-9]+-`. Current impl: any `.md` under
    /// `.factory/...epics/` → true. After fix (requires `^E-[0-9]+-` basename): false.
    ///
    /// RED GATE: current impl returns true → assert!(!result) FAILS.
    #[test]
    fn test_BC_5_39_010_dispatch_epic_non_numeric_basename_rejected() {
        let result = is_frontmatter_parity_target(".factory/stories/epics/README.md");
        assert!(
            !result,
            "'.factory/stories/epics/README.md' has basename 'README.md' which does not \
            match `^E-[0-9]+-`. Must NOT be classified as frontmatter parity target. \
            PC34 bullet 4: epic basename must match `^E-[0-9]+-.*\\.md$`. \
            F-S2107-P3-009: current `ends_with('.md')` admits all .md files under epics/. \
            RED GATE."
        );
    }

    /// F-S2107-P3-009 GREEN: canonical epic file at correct path IS a parity target.
    ///
    /// Complement test: after fix, `.factory/stories/epics/E-21-name.md` must still
    /// be classified correctly. Verifies the guard does not over-exclude real epics.
    #[test]
    fn test_BC_5_39_010_dispatch_epic_correct_path_accepted() {
        let result = is_frontmatter_parity_target(
            ".factory/stories/epics/E-21-factory-state-data-loss-hardening.md",
        );
        assert!(
            result,
            "canonical epic path '.factory/stories/epics/E-21-*.md' must be a \
            frontmatter parity target (PC34 bullet 4). GREEN guard test."
        );
    }
}
