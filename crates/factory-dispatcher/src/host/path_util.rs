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

// ---------------------------------------------------------------------------
// S-19.03 Red Gate tests for path_util::resolve_path_for_allowlist
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// test_S19_03_T001_helper_absent_target_under_existing_parent_returns_some
    ///
    /// T-001 support (AC-001): `resolve_path_for_allowlist` with an absent target file
    /// whose PARENT directory exists must return `Some(synthesized canonical path)`.
    /// The rejoin algorithm canonicalizes the parent, then appends the absent filename.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` body is `todo!()` (S-19.03 stub).
    ///
    /// Traces to: BC-2.07.001 part b (rejoin algorithm); S-19.03 AC-001.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T001_helper_absent_target_under_existing_parent_returns_some() {
        let dir = tempfile::tempdir().unwrap();
        let absent_target = dir.path().join("wave-state.yaml");
        assert!(!absent_target.exists(), "test setup: target must not exist");
        assert!(dir.path().exists(), "test setup: parent must exist");

        let result = resolve_path_for_allowlist(&absent_target, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "T-001 AC-001: absent file with existing parent must return \
             Some(synthesized_canonical_path); got None. Rejoin algorithm must walk \
             ancestors and rejoin the absent tail onto the deepest existing ancestor."
        );
        let resolved = result.unwrap();
        assert_eq!(
            resolved.file_name().and_then(|n| n.to_str()),
            Some("wave-state.yaml"),
            "T-001: synthesized canonical path must end with the original target filename"
        );
    }

    /// test_S19_03_EC007_mock_canonicalize_all_fail_returns_none
    ///
    /// T-001 Negative Control B (BC-2.07.001 EC-007): when the injected canonicalize
    /// function returns `Err` for every path, `resolve_path_for_allowlist` must return
    /// `None`. The caller (path_allowed) then emits `internal.capability_denied` with
    /// `reason=path_resolution_failed` (NOT `path_not_allowed`).
    ///
    /// On production Unix filesystems this branch is structurally unreachable
    /// (the root `/` always canonicalizes), but it MUST be testable via the
    /// injectable `canonicalize_fn` parameter per BC-2.07.001 EC-007.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`.
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 AC-001 negative-control B.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_EC007_mock_canonicalize_all_fail_returns_none() {
        let target = Path::new(".factory/wave-state.yaml");
        let mock_fail = |_p: &Path| -> std::io::Result<PathBuf> {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        };

        let result = resolve_path_for_allowlist(target, mock_fail);
        assert!(
            result.is_none(),
            "EC-007: when mock canonicalize returns Err for ALL ancestors, \
             resolve_path_for_allowlist must return None — the caller must then \
             emit reason=path_resolution_failed (not path_not_allowed)."
        );
    }

    /// test_S19_03_absent_path_with_dotdot_in_tail_still_resolves_to_some
    ///
    /// Traversal defense integration (BC-2.07.001 Invariant 1): an absent target that
    /// contains `..` in the absent-tail portion still resolves to `Some` — the `..`
    /// is absorbed by the real-filesystem ancestor canonicalization, producing a
    /// synthesized canonical path that the `starts_with` check in `path_allowed` can
    /// then correctly reject (or accept). The traversal defense is the `starts_with`
    /// check, not this function.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`.
    ///
    /// Traces to: BC-2.07.001 Invariant 1 (traversal defense via starts_with);
    ///            BC-2.02.011 EC-001.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_absent_path_with_dotdot_in_tail_still_resolves_to_some() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subdir");
        std::fs::create_dir_all(&sub).unwrap();
        // Target: subdir/../wave-state.yaml — `..` in the absent-tail region.
        let target = sub.join("..").join("wave-state.yaml");
        assert!(!target.exists(), "test setup: target with .. must not exist");

        let result = resolve_path_for_allowlist(&target, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "Traversal defense: path with '..' in absent tail must still return Some — \
             the starts_with check in path_allowed is where traversal escapes are caught."
        );
        // After implementation: the synthesized canonical path must be under dir (not sub),
        // because the .. was absorbed by the ancestor canonicalization.
        let resolved = result.unwrap();
        let canonical_dir = dir.path().canonicalize().unwrap();
        assert!(
            resolved.starts_with(&canonical_dir),
            "Traversal: synthesized path {:?} must be under the canonical dir {:?} \
             (.. absorbed during ancestor walk)",
            resolved,
            canonical_dir
        );
    }
}
