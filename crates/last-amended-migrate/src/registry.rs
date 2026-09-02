//! Registration of this tool's own output paths and the pre-existing D-1149
//! `*-amendment-history.md` sidecar paths in
//! `plugins/vsdd-factory/config/artifact-path-registry.yaml`
//! (S-15.03 AC-006 delivery requirement; BC-10.13.001 §Architecture
//! Anchors).

use crate::error::MigrateError;
use std::path::Path;

/// Register this tool's own output paths (migration reports, rotation
/// archives) and the 5 pre-existing D-1149 `*-amendment-history.md` sidecar
/// paths into the artifact path registry at `registry_path`.
///
/// Read-only with respect to the 5 sidecar files themselves (BC-10.13.001
/// PC6) — this function only appends entries to the registry config; it
/// never opens a sidecar file for writing. Idempotent: re-running against an
/// already-registered set of paths must not add duplicate entries.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: performs I/O and must de-duplicate against existing
/// registry entries. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; the
/// de-duplication/idempotency requirement is exactly what a re-run test
/// will probe. Therefore: `todo!()`.
pub fn register_artifact_paths(registry_path: &Path) -> Result<(), MigrateError> {
    todo!(
        "append this tool's own output paths + the 5 \
        *-amendment-history.md sidecar paths to {registry_path:?}, \
        de-duplicating against existing entries (S-15.03 AC-006; \
        BC-10.13.001 §Architecture Anchors, PC6)"
    )
}
