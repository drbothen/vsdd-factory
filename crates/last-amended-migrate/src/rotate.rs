//! Rotation subcommand orchestration (BC-10.13.001 PC5, PC6).

use crate::error::MigrateError;
use crate::migrate::MigrationMode;
use std::path::{Path, PathBuf};

/// Tool-default number of most-recent `changelog:` items to retain in the
/// source file when a `--keep-recent` count is not supplied
/// (BC-10.13.001 PC5 "all-but-the-most-recent-K by tool default").
pub const DEFAULT_KEEP_RECENT: usize = 20;

/// Outcome of a single rotation invocation.
#[derive(Debug, Clone)]
pub struct RotationReport {
    pub path: PathBuf,
    pub archive_path: PathBuf,
    /// Number of `changelog:` items moved out of `path` into the archive.
    /// `0` for a below-threshold no-op (EC-004).
    pub items_moved: usize,
    pub mutated: bool,
}

/// Rotate `path`'s `changelog:` sequence: move the oldest items past
/// `keep_recent` verbatim into
/// `.factory/cycles/<cycle_name>/<file-basename>-changelog-archive.md`,
/// removing exactly those items from `path` and leaving a discoverability
/// pointer (BC-10.13.001 PC5).
///
/// No-op (EC-004) when the sequence does not exceed `keep_recent`. Creates
/// `.factory/cycles/<cycle_name>/` if it does not already exist (EC-005).
/// Every `changelog:` item's `date:`/`summary:` text is preserved verbatim —
/// only its location (source vs. archive) changes (PC5). Re-running rotation
/// immediately after a successful rotation, before the threshold is
/// exceeded again, is a verified-clean no-op (Invariant 2).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: threshold branching, directory creation, verbatim item
/// transplantation across two files, and idempotency. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; PC5's
/// no-data-loss/verbatim guarantee across a two-file move is exactly what a
/// proptest-based VP (see BC-10.13.001 §Verification Properties) will
/// probe. Therefore: `todo!()`.
pub fn rotate_changelog(
    path: &Path,
    cycle_name: &str,
    keep_recent: usize,
    mode: MigrationMode,
) -> Result<RotationReport, MigrateError> {
    todo!(
        "move changelog: items beyond the most-recent {keep_recent} out of \
        {path:?} verbatim into \
        .factory/cycles/{cycle_name}/<basename>-changelog-archive.md in \
        {mode:?} mode, leaving a discoverability pointer in the source \
        (BC-10.13.001 PC5, EC-004, EC-005)"
    )
}
