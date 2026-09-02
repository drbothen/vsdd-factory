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
    /// Count of historical entries PC7's full-recovery split relocated from
    /// an inline `[Prior: ...]` chain into `changelog:`, newest-first.
    /// `0` when `eligibility` is `Eligibility::CurrentEntryOnly` (no split
    /// occurred — including the PC4/PC7-step-8 no-op re-run case after a
    /// prior split has already resolved the chain).
    pub entries_recovered: usize,
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
/// (BC-10.13.001 v1.1 PC1-PC4, PC6, PC7).
///
/// Orchestration order: parse (`parse_frontmatter`) → classify eligibility
/// (`check_eligibility`, PC2/PC7) → (`Apply` only) either ensure
/// `changelog:` + D-1144 escape remediation on the current entry
/// (`CurrentEntryOnly` path: `ensure_changelog_field` PC1,
/// `needs_escaping`/`escape_value` PC3) OR perform the PC7 full-recovery
/// split (`PriorChainSplit` path: re-emit the current entry as the new
/// current-entry-only `last_amended`, PC3-escape and prepend every relocated
/// historical entry into `changelog:` newest-first, bootstrapping
/// `changelog:` first via `ensure_changelog_field` if absent per PC7 step 6)
/// → (`Apply` only) write. `Check` mode never writes (PC4 verified-clean-
/// report semantics).
///
/// Returns `Err(MigrateError::NotEligible)` only for the EC-008 case — a
/// `last_amended` field that cannot be located at all in `path`'s
/// frontmatter (a corrupted/unparseable frontmatter delimiter surfaces as
/// `Err(MigrateError::FrontmatterParse)` from the `parse_frontmatter` step
/// instead). A `PriorChainSplit` classification is no longer an error path
/// as of the v1.1 amendment — it is resolved in the same `Apply` run via
/// PC7, superseding the v1.0 EC-003 refusal behavior.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: multi-step orchestration with I/O and branching over the
/// `CurrentEntryOnly`/`PriorChainSplit` outcomes plus the idempotency
/// guarantee (PC4/PC7 step 8: a second `Apply` run — including one
/// immediately after a split — must report zero mutations). Body is
/// `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; this is the function
/// the story's canonical test vectors (happy-path, edge-case, split,
/// idempotency) directly target. Therefore: `todo!()`.
pub fn migrate_file(path: &Path, mode: MigrationMode) -> Result<FileMigrationReport, MigrateError> {
    todo!(
        "orchestrate parse_frontmatter -> check_eligibility -> either \
        (CurrentEntryOnly) ensure_changelog_field + needs_escaping/escape_value \
        or (PriorChainSplit) PC7 full-recovery split into changelog: -> \
        (Apply-mode only) write, for {path:?} in {mode:?} mode \
        (BC-10.13.001 v1.1 PC1-PC4, PC6, PC7)"
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
