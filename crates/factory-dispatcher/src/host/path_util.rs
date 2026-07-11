//! Shared path resolution helper for host read/write allowlist enforcement.
//!
//! S-19.03: Extracted from `write_file.rs` to provide a single, testable
//! ancestor-walk + rejoin algorithm for both `read_file.rs` and `write_file.rs`.
//! The injectable `canonicalize_fn` parameter enables unit testing without a
//! real sandboxed filesystem (BC-2.07.001 EC-007).
//!
//! Purity classification: pure-core — no I/O; the canonicalize function is
//! injected by the caller.

use std::path::{Path, PathBuf};

/// Resolve a target path for allowlist comparison using an ancestor-walk + rejoin
/// algorithm, canonicalizing to defeat `..` traversal attacks.
///
/// Accepts an injectable `canonicalize_fn` (signature `fn(&Path) -> std::io::Result<PathBuf>`)
/// to enable unit testing without a real sandboxed filesystem (BC-2.07.001 EC-007). In
/// production, callers pass `|p| p.canonicalize()`.
///
/// Algorithm: if the full path canonicalizes, return it directly. Otherwise walk
/// ancestor components bottom-up, collecting non-existent tail components, until an
/// ancestor canonicalizes. Rejoin the collected tail onto the canonical ancestor in
/// original order. If no ancestor canonicalizes (all ancestors fail), return `None` —
/// the caller emits `path_resolution_failed` reason token (not `path_not_allowed`).
///
/// Purity classification: pure-core (path manipulation only; no filesystem I/O
/// beyond the injected canonicalize_fn calls).
pub fn resolve_path_for_allowlist(
    _target: &Path,
    _canonicalize_fn: impl Fn(&Path) -> std::io::Result<PathBuf>,
) -> Option<PathBuf> {
    todo!("S-19.03: implement ancestor-walk + rejoin algorithm — see Architecture Mapping in S-19.03 story spec")
}
