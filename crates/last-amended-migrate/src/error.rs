//! Error taxonomy for `last-amended-migrate`.
//!
//! Named error variants for every fallible operation this crate performs, per
//! CLAUDE.md's "No `unwrap()`/`expect()` in critical code paths" convention.
//! Pure data declarations — the BC-5.38.001 `todo!()` obligation governs
//! function *bodies*, not type/variant declarations, so these are written out
//! in full at stub stage (mirrors every sibling crate's `error.rs`/`lib.rs`
//! error-enum convention, e.g. `factory-lock`, `validate-dispatch-advance`).

use std::path::PathBuf;
use thiserror::Error;

/// Errors surfaced by `last-amended-migrate`'s migration, rotation, and
/// registry-update operations (BC-10.13.001).
#[derive(Debug, Error)]
pub enum MigrateError {
    /// A filesystem read/write failed.
    #[error("I/O error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// `path`'s frontmatter could not be isolated/parsed.
    #[error("frontmatter parse error in {path}: {reason}")]
    FrontmatterParse { path: PathBuf, reason: String },

    /// `path`'s `last_amended` still contains a nested `[Prior: ...]` chain
    /// (BC-10.13.001 Precondition 2 / EC-003) — out of scope for this tool.
    #[error(
        "{path} is not eligible for migration: last_amended still contains \
        a [Prior: ...] chain (Precondition 2 / EC-003; out of scope for this \
        tool — that surgery remains a human-authorized POL-3 exception if it \
        recurs)"
    )]
    NotEligible { path: PathBuf },

    /// A value this tool was about to write failed strict YAML `safe_load`
    /// validation (BC-10.13.001 Invariant 4 / PC3).
    #[error("YAML produced by this tool failed strict safe_load validation: {reason}")]
    InvalidYamlProduced { reason: String },

    /// Writing to `plugins/vsdd-factory/config/artifact-path-registry.yaml`
    /// failed (S-15.03 AC-006).
    #[error("artifact-path-registry.yaml write error: {reason}")]
    RegistryWrite { reason: String },
}
