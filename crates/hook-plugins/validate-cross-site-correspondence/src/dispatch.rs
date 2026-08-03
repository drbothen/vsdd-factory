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
    let filename_ok = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.starts_with("BC-") && f.ends_with(".md"))
        .unwrap_or(false);
    has_factory && has_specs && has_bc && filename_ok
}

/// Returns `true` if `file_path` names a story file under `.factory/stories/`
/// using path-component-strict matching.
///
/// A story file has:
/// - A `.factory` path component AND
/// - A `stories` path component AND
/// - Filename ending in `.md` (but NOT `STORY-INDEX.md` — that is Arm B2's trigger)
///
/// # BC trace
/// BC-5.39.010 precondition 9 (Class A Arm2 trigger condition).
/// BC-5.39.010 precondition 16 (Class B Arm1 trigger condition).
/// BC-5.39.010 invariant 3.
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
    has_factory && has_stories && is_md && not_index
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
/// Cycle artifacts are:
/// - `burst-log.md`: basename == "burst-log.md" AND a `cycles` path component.
/// - `lessons.md`: basename == "lessons.md" AND a `cycles` path component.
/// - `INDEX.md`: basename == "INDEX.md" AND a `cycles` path component (NOT STATE.md, not stories INDEX.md).
///
/// Returns `None` for non-cycle-artifact files (caller emits `HookResult::Continue`).
///
/// # BC trace
/// BC-5.39.010 precondition 28 (Class D trigger condition).
/// BC-5.39.010 invariant 3.
pub fn is_cycle_artifact(file_path: &str) -> Option<CycleArtifactKind> {
    use std::path::{Component, Path};
    let path = Path::new(file_path);
    let components: Vec<_> = path.components().collect();
    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    let has_cycles = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "cycles"));
    if !has_factory || !has_cycles {
        return None;
    }
    match path.file_name().and_then(|f| f.to_str()) {
        Some("burst-log.md") => Some(CycleArtifactKind::BurstLog),
        Some("lessons.md") => Some(CycleArtifactKind::Lessons),
        Some("INDEX.md") => Some(CycleArtifactKind::CycleIndex),
        _ => None,
    }
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
        .map(|f| f.starts_with("VP-") && f.ends_with(".md"))
        .unwrap_or(false);
    if has_factory && has_specs && has_vp && vp_filename_ok {
        return true;
    }
    // Epic files under .factory/epics/
    let has_epics = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "epics"));
    let epic_filename_ok = path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.ends_with(".md"))
        .unwrap_or(false);
    has_factory && has_epics && epic_filename_ok
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
        let result = is_story_file(".factory/stories/S-21.07-validate-cross-site-correspondence.md");
        assert!(result, "canonical story path must be detected");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_story_index_not_story_file() {
        // STORY-INDEX.md must NOT match is_story_file (it's Arm B2's trigger)
        let result = is_story_file(".factory/stories/STORY-INDEX.md");
        assert!(!result, "STORY-INDEX.md must not be classified as a story file");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_story_index_detected() {
        // BC-5.39.010 PC22: STORY-INDEX.md trigger
        let result = is_story_index(".factory/stories/STORY-INDEX.md");
        assert!(result, "STORY-INDEX.md must be detected as story index");
    }

    #[test]
    fn test_BC_5_39_010_dispatch_burst_log_detected() {
        // BC-5.39.010 PC28: burst-log.md under cycles/
        let result = is_cycle_artifact(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md",
        );
        assert_eq!(
            result,
            Some(CycleArtifactKind::BurstLog),
            "burst-log.md must be classified as BurstLog cycle artifact"
        );
    }

    #[test]
    fn test_BC_5_39_010_dispatch_lessons_md_detected() {
        let result = is_cycle_artifact(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md",
        );
        assert_eq!(
            result,
            Some(CycleArtifactKind::Lessons),
            "lessons.md must be classified as Lessons cycle artifact"
        );
    }

    #[test]
    fn test_BC_5_39_010_dispatch_non_cycle_index_md_rejected() {
        // .factory/stories/STORY-INDEX.md must not trigger the cycle artifact arm
        let result = is_cycle_artifact(".factory/stories/STORY-INDEX.md");
        assert!(
            result.is_none(),
            "stories/STORY-INDEX.md must not classify as cycle artifact"
        );
    }
}
