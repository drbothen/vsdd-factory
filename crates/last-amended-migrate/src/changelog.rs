//! `changelog:` sequence handling (BC-10.13.001 PC1) and the BC-5.45.001 PC2
//! prepend discipline this tool's migration output must conform to
//! (BC-10.13.001 Invariant 4).

use crate::frontmatter::FrontmatterDoc;

/// One `changelog:` sequence item, as constructed by this tool when
/// prepending a newly displaced `last_amended` entry during migration —
/// distinct from `FrontmatterDoc::changelog_items_raw`, which holds
/// pre-existing items' raw text verbatim for lossless rotation (PC5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangelogItem {
    /// `date:` field, `YYYY-MM-DD`.
    pub date: String,
    /// `version:` field, when the target file's own convention includes
    /// one.
    pub version: Option<String>,
    /// `summary:`/`change:` field text — MUST be passed through
    /// `crate::escape::escape_value` before being written (PC3).
    pub summary: String,
}

/// Outcome of `ensure_changelog_field` (BC-10.13.001 PC1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogMutation {
    /// `changelog:` was absent and has been added (expected only for
    /// `STORY-INDEX.md`, EC-002).
    Added,
    /// `changelog:` already existed — verified no-op.
    AlreadyPresent,
    /// Target is `STATE.md`, which never gains a `changelog:` field
    /// (ADR-049 Decision 4, EC-006) — not attempted.
    SkippedStateFile,
}

/// Ensure `doc` carries a `changelog:` top-level sequence, adding an empty
/// one only when absent and the file is not `STATE.md` (BC-10.13.001 PC1).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: branches on `is_state_file` and on whether `changelog:`
/// already exists, and must produce a byte-precise frontmatter mutation.
/// Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; the 3-way outcome
/// (Added / AlreadyPresent / SkippedStateFile) requires real branching that
/// this story's canonical test vectors specifically target. Therefore:
/// `todo!()`.
pub fn ensure_changelog_field(doc: &mut FrontmatterDoc, is_state_file: bool) -> ChangelogMutation {
    todo!(
        "add an empty changelog: sequence to {:?} if absent and not \
        STATE.md; return SkippedStateFile when is_state_file={is_state_file}; \
        return AlreadyPresent when doc.changelog_present is already true \
        (BC-10.13.001 PC1, EC-002, EC-006)",
        doc.path
    )
}

/// Prepend exactly one newly displaced entry to `doc`'s `changelog:`
/// sequence, newest-first, leaving every existing item byte-for-byte
/// untouched (BC-5.45.001 PC2 discipline that this tool's own migration
/// output must satisfy per BC-10.13.001 Invariant 4).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: performs a frontmatter mutation and must guarantee
/// byte-for-byte preservation of unrelated content. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; the
/// byte-preservation guarantee is exactly what a fixture-based test will
/// probe. Therefore: `todo!()`.
pub fn prepend_changelog_item(doc: &mut FrontmatterDoc, item: ChangelogItem) {
    todo!(
        "prepend {item:?} as the new first changelog: item in {:?}, leaving \
        all existing items untouched (BC-5.45.001 PC2)",
        doc.path
    )
}
