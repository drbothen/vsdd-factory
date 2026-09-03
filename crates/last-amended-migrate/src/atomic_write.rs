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
//!
//! # S-15.03 pr-reviewer N2 — permission preservation + fsync durability
//!
//! Two gaps in the original write-then-rename implementation, closed here:
//!
//! 1. **Permission preservation.** `File::create` on a brand-new temp file
//!    gets the platform/umask default mode, NOT the pre-existing target
//!    file's own mode — a target deliberately `chmod`'d to something
//!    non-default (e.g. `0o600` for a sensitive file) would silently have
//!    its permission bits reset to the default on every write. This module
//!    now reads the pre-existing target's `Permissions` (mode on Unix,
//!    read-only flag on Windows) and applies them to the temp file before
//!    the rename, so a rename never changes a file's permission bits as a
//!    side effect. A target that does not exist yet (first-ever write) has
//!    nothing to preserve, and is left at the platform default.
//! 2. **fsync durability.** A `rename` that lands before the temp file's
//!    written bytes are actually durable on disk (still sitting in a
//!    filesystem write-back cache) leaves a crash window where `path` could
//!    point at a temp file whose content is lost or truncated after an
//!    unclean shutdown, even though the rename itself succeeded. This module
//!    now `fsync`s the temp file's contents before the rename, and
//!    best-effort `fsync`s the containing directory afterward (Unix only —
//!    directory fsync has no equivalent/is not meaningful on Windows) so the
//!    rename's directory-entry update is also durable, not just the file's
//!    bytes.

use crate::error::MigrateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write `content` to `path` atomically: write to a sibling `<basename>.tmp-<pid>`
/// file in the same directory (preserving `path`'s pre-existing permission
/// bits, if any, and `fsync`ing the data before rename — S-15.03 N2), then
/// `rename` it into place. On success, no `.tmp-*` sibling survives — the
/// rename consumes it. On failure to write or fsync the temp file, no
/// attempt is made to write `path` at all, so `path` is left completely
/// untouched, and the temp file is best-effort removed.
pub fn write_atomic(path: &Path, content: &str) -> Result<(), MigrateError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let basename = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "last-amended-migrate-output".to_string());
    let tmp_path = parent.join(format!(".{basename}.tmp-{}", std::process::id()));

    if let Err(e) = write_and_sync_temp(&tmp_path, content) {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(e);
    }

    // N2: preserve the pre-existing target's permission bits across the
    // rename. Best-effort — a target that doesn't exist yet (first write)
    // or a permissions API that fails for an unrelated reason must not
    // block the substantive content write, which `std::fs::write` never
    // guarded against either.
    if let Ok(existing_meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&tmp_path, existing_meta.permissions());
    }

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
    })?;

    // N2: best-effort directory fsync so the rename's directory-entry
    // update is itself durable across a crash, not just the file's bytes.
    // Unix-only: Windows has no directly equivalent operation, and a
    // failure here must never fail the overall write — the rename already
    // succeeded, which is the operation this function's contract cares
    // about; this is defense-in-depth for the crash-durability window, not
    // a correctness requirement of the write itself.
    #[cfg(unix)]
    if let Ok(dir) = File::open(parent) {
        let _ = dir.sync_all();
    }

    Ok(())
}

/// Write `content` to `tmp_path` (creating or truncating it) and `fsync` it
/// before returning, so the caller's subsequent `rename` never lands ahead
/// of the data actually being durable on disk (S-15.03 N2).
fn write_and_sync_temp(tmp_path: &Path, content: &str) -> Result<(), MigrateError> {
    let mut file = File::create(tmp_path).map_err(|source| MigrateError::Io {
        path: tmp_path.to_path_buf(),
        source,
    })?;
    file.write_all(content.as_bytes())
        .map_err(|source| MigrateError::Io {
            path: tmp_path.to_path_buf(),
            source,
        })?;
    file.sync_all().map_err(|source| MigrateError::Io {
        path: tmp_path.to_path_buf(),
        source,
    })
}
