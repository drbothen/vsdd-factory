//! Frontmatter parsing primitive for `last-amended-migrate`.
//!
//! Isolates exactly the two YAML frontmatter fields BC-10.13.001 operates on
//! — `last_amended` (a single scalar) and `changelog:` (a top-level sequence,
//! when present) — without depending on a general-purpose YAML parser.
//! Invariant 3 requires this parse to stay linear-time/bounded-memory even
//! against the D-1149 calibration ceiling (a 323,499-char single scalar
//! line), which rules out any parse strategy with quadratic backtracking on
//! long lines (the same constraint `factory-lock-parse` documents for its own
//! hand-rolled scan, "serde_yaml/serde_norway FORBIDDEN").

use crate::error::MigrateError;
use std::path::{Path, PathBuf};

/// Parsed frontmatter of one of the 5 BC-10.13.001 target files, isolating
/// `last_amended` and `changelog:` for the migration/rotation subcommands.
///
/// `changelog_items_raw` holds each sequence item's exact raw YAML block text
/// verbatim (not a re-serializable struct) so PC5 rotation can move items
/// without any parse/reserialize round-trip risking a content change — see
/// `crate::changelog::ChangelogItem` for the distinct struct this tool uses
/// when *constructing* a brand-new item to prepend.
#[derive(Debug, Clone)]
pub struct FrontmatterDoc {
    /// Absolute path of the file this frontmatter was parsed from.
    pub path: PathBuf,
    /// The full raw file content, unmodified. Writers reconstruct the file by
    /// patching precise byte ranges of this string rather than
    /// re-serializing the whole document, preserving untouched content
    /// byte-for-byte — the same raw-byte-preservation discipline
    /// `compute-input-hash` documents for its own file handling.
    pub raw: String,
    /// Raw value of `last_amended:` (unescaped/unquoted), or `None` if the
    /// field is absent from frontmatter entirely. Absence is not expected on
    /// any of the 5 target files but is represented rather than assumed.
    pub last_amended_raw: Option<String>,
    /// Whether a top-level `changelog:` key exists in frontmatter at all —
    /// distinct from an empty sequence. PC1 cares about presence.
    pub changelog_present: bool,
    /// Each `changelog:` sequence item's raw YAML block text, in file order
    /// (newest-first, matching the prepend convention), verbatim.
    pub changelog_items_raw: Vec<String>,
}

/// Parse `path`'s YAML frontmatter, isolating `last_amended` and
/// `changelog:`.
///
/// This is the shared primitive every PC-satisfying function in this crate is
/// built on — parsing on its own is not itself a BC-10.13.001 postcondition,
/// but Invariant 3 (bounded-resource safety on arbitrarily long input) binds
/// this function directly.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: performs I/O (file read) and branching (frontmatter fence
/// detection, field isolation, bounded-scan discipline). Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; correct frontmatter
/// isolation on real fixture files, including the mega-line calibration
/// case, requires real parsing work. Therefore: `todo!()`.
pub fn parse_frontmatter(path: &Path) -> Result<FrontmatterDoc, MigrateError> {
    todo!(
        "read {path:?}, locate the `---` ... `---` frontmatter fence, and \
        isolate last_amended: and changelog: without a quadratic-backtracking \
        scan (BC-10.13.001 Invariant 3; D-1149 323,499-char calibration \
        ceiling)"
    )
}
