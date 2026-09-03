//! Atomic write-then-rename for governed-file writes (S-15.03 SEC-003,
//! CWE-367 time-of-check to time-of-use race condition).
//!
//! `migrate_file`, `rotate_changelog`, and `register_artifact_paths` all read
//! a target file, compute a new full content, then previously called plain
//! `std::fs::write(path, ...)` directly against the target — no atomicity
//! between the read and the write, so a concurrent writer to the same file
//! could interleave with this tool's own write, or a reader could observe a
//! partially-written file mid-write. `std::fs::write` is not required by any
//! platform to write its buffer as a single atomic syscall.
//!
//! This module writes to a sibling temporary file first, then
//! `std::fs::rename`s it into place — `rename(2)`/`MoveFileExW` is atomic
//! when the source and destination are on the same filesystem (true here,
//! since the temp file is always created as a sibling of the real target in
//! the same directory), so any concurrent reader of `path` observes either
//! the fully-old content or the fully-new content, never a partial write.

use crate::error::MigrateError;
use std::path::Path;

/// Write `content` to `path` atomically: write to a sibling `<basename>.tmp-<pid>`
/// file in the same directory, then `rename` it into place. On success, no
/// `.tmp-*` sibling survives — the rename consumes it. On failure to write
/// the temp file, no attempt is made to write `path` at all, so `path` is
/// left completely untouched.
pub fn write_atomic(path: &Path, content: &str) -> Result<(), MigrateError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let basename = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "last-amended-migrate-output".to_string());
    let tmp_path = parent.join(format!(".{basename}.tmp-{}", std::process::id()));

    std::fs::write(&tmp_path, content).map_err(|source| MigrateError::Io {
        path: tmp_path.clone(),
        source,
    })?;

    std::fs::rename(&tmp_path, path).map_err(|source| {
        // Best-effort cleanup of the orphaned temp file — the rename failure
        // itself is still reported; a leftover `.tmp-<pid>` here is a
        // secondary symptom, not the primary error, and this tool has no
        // other path that can safely remove it on the caller's behalf later.
        let _ = std::fs::remove_file(&tmp_path);
        MigrateError::Io {
            path: path.to_path_buf(),
            source,
        }
    })
}
