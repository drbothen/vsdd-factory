//! Migration subcommand orchestration (BC-10.13.001 PC1-PC4, PC6).

use crate::changelog::ChangelogMutation;
use crate::eligibility::Eligibility;
use crate::error::MigrateError;
use std::path::{Path, PathBuf};

/// The 5 ADR-049-governed files this tool's migration subcommand targets,
/// relative to a supplied `.factory/` root (BC-10.13.001 Precondition 1,
/// §Description; D-1149).
pub const TARGET_FILES: [&str; 5] = [
    "stories/STORY-INDEX.md",
    "specs/behavioral-contracts/BC-INDEX.md",
    "specs/architecture/ARCH-INDEX.md",
    "specs/verification-properties/VP-INDEX.md",
    "STATE.md",
];

/// Whether a migration/rotation invocation reports violations only
/// (`Check`, mirroring `compute-input-hash`'s `--check`) or performs the
/// write (`Apply`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationMode {
    /// Report only; `migrate_file`/`migrate_all`/`rotate_changelog` MUST NOT
    /// write to any target file in this mode.
    Check,
    /// Perform the write.
    Apply,
}

/// Per-file migration outcome.
#[derive(Debug, Clone)]
pub struct FileMigrationReport {
    pub path: PathBuf,
    pub eligibility: Eligibility,
    pub changelog_mutation: ChangelogMutation,
    pub escape_fixed: bool,
    /// `true` iff any of the above resulted in an actual file write —
    /// always `false` in `MigrationMode::Check`, by definition.
    pub mutated: bool,
}

/// Aggregate report across every file a migration invocation touched.
#[derive(Debug, Clone, Default)]
pub struct MigrationReport {
    pub files: Vec<FileMigrationReport>,
}

impl MigrationReport {
    /// Count of files this report recorded an actual mutation for.
    ///
    /// # BC-5.38.002 GREEN-BY-DESIGN
    ///
    /// Pure fold over already-computed `FileMigrationReport::mutated` flags:
    /// zero branching beyond the closure's boolean field read (no
    /// if/match/?/unwrap), no I/O, no non-trivial helper calls
    /// (`Iterator::filter`/`count` are primitive standard-library
    /// operations, not domain logic), body is 3 lines. All four BC-5.38.002
    /// criteria hold — see the stub commit report's GREEN-BY-DESIGN table.
    pub fn total_mutated(&self) -> usize {
        self.files.iter().filter(|f| f.mutated).count()
    }
}

/// Run the migration subcommand against exactly one target file
/// (BC-10.13.001 PC1-PC4, PC6).
///
/// Orchestration order: parse (`parse_frontmatter`) → classify eligibility
/// (`check_eligibility`, PC2/EC-003) → (`Apply` only) ensure `changelog:`
/// (`ensure_changelog_field`, PC1) → (`Apply` only) D-1144 escape remediation
/// (`needs_escaping`/`escape_value`, PC3) → (`Apply` only) write. `Check`
/// mode never writes (PC4 verified-clean-report semantics).
///
/// Returns `Err(MigrateError::NotEligible)` per EC-003 rather than attempting
/// any bracket-splitting surgery when `check_eligibility` classifies the
/// file as `NotEligiblePriorChain` — that remains out of this tool's scope
/// (Precondition 2).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: multi-step orchestration with I/O and branching over 3
/// independent outcomes plus the idempotency guarantee (PC4: a second
/// `Apply` run must report zero mutations). Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; this is the function
/// the story's 3 canonical test vectors (happy-path, edge-case, idempotency)
/// directly target. Therefore: `todo!()`.
pub fn migrate_file(path: &Path, mode: MigrationMode) -> Result<FileMigrationReport, MigrateError> {
    todo!(
        "orchestrate parse_frontmatter -> check_eligibility -> \
        ensure_changelog_field -> needs_escaping/escape_value -> \
        (Apply-mode only) write, for {path:?} in {mode:?} mode \
        (BC-10.13.001 PC1-PC4, PC6)"
    )
}

/// Run the migration subcommand against all 5 `TARGET_FILES`, resolved
/// relative to `factory_root` (BC-10.13.001 §Description, Precondition 1).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: calls the non-trivial `migrate_file` in a loop and
/// aggregates results/errors across 5 files. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; aggregation and
/// error-propagation semantics across all 5 files are themselves untested
/// behavior this story's fixtures target. Therefore: `todo!()`.
pub fn migrate_all(
    factory_root: &Path,
    mode: MigrationMode,
) -> Result<MigrationReport, MigrateError> {
    todo!(
        "invoke migrate_file for each of TARGET_FILES under {factory_root:?} \
        in {mode:?} mode and aggregate into a MigrationReport \
        (BC-10.13.001 Precondition 1)"
    )
}
