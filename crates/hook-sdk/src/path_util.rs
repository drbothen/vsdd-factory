//! Path utility helpers for WASM hook plugins.
//!
//! Hook plugins compile to `wasm32-wasip1` and cannot depend on `factory-dispatcher`
//! (the host crate). This module provides the path-derivation guard functions that
//! mirror the Level C logic in `factory_dispatcher::log_dir` so that the guard
//! is not duplicated across WASM plugin crates (TD-VSDD-060 sibling-site sweep,
//! F-S2107-P8-016).

use std::path::{Path, PathBuf};

/// Returns `true` if the path's final component is `.factory`
/// (case-insensitive on macOS/Windows; case-sensitive on Linux).
///
/// Mirrors `factory_dispatcher::log_dir::is_dot_factory_basename` for WASM plugins.
pub fn is_dot_factory_basename(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(is_factory_name)
        .unwrap_or(false)
}

/// Derive the factory-artifacts directory from a base path.
///
/// Guard (F-S2107-P8-016): when `base` basename is already `.factory`,
/// return `base` unchanged. Re-appending `.factory` would produce the
/// nested `.factory/.factory` double-path when `base` is the factory-artifacts
/// worktree root.
///
/// Mirrors `derive_factory_dir` in `factory_dispatcher::src::main` for WASM plugins.
pub fn derive_factory_dir(base: &Path) -> PathBuf {
    if is_dot_factory_basename(base) {
        base.to_path_buf()
    } else {
        base.join(".factory")
    }
}

/// Case-insensitive on macOS/Windows; case-sensitive on Linux.
#[cfg(target_os = "linux")]
#[inline]
fn is_factory_name(s: &str) -> bool {
    s == ".factory"
}

#[cfg(not(target_os = "linux"))]
#[inline]
fn is_factory_name(s: &str) -> bool {
    s.eq_ignore_ascii_case(".factory")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]
    use super::*;
    use std::path::PathBuf;

    // -----------------------------------------------------------------------
    // is_dot_factory_basename
    // -----------------------------------------------------------------------

    #[test]
    fn test_is_dot_factory_basename_returns_true() {
        assert!(is_dot_factory_basename(Path::new("/some/path/.factory")));
        assert!(is_dot_factory_basename(Path::new(".factory")));
    }

    #[test]
    fn test_is_dot_factory_basename_returns_false_for_normal_path() {
        assert!(!is_dot_factory_basename(Path::new("/some/project")));
        assert!(!is_dot_factory_basename(Path::new(".")));
        assert!(!is_dot_factory_basename(Path::new("/repo")));
    }

    // -----------------------------------------------------------------------
    // derive_factory_dir — negative polarity (primary guard)
    // -----------------------------------------------------------------------
    #[test]
    fn test_derive_factory_dir_no_double_when_base_is_dot_factory() {
        let base = PathBuf::from("/repo/.factory");
        let result = derive_factory_dir(&base);
        assert_eq!(
            result, base,
            "derive_factory_dir must return base unchanged when basename is .factory; \
             got {result:?}"
        );
        assert_ne!(
            result,
            base.join(".factory"),
            "derive_factory_dir must not produce .factory/.factory"
        );
    }

    // -----------------------------------------------------------------------
    // derive_factory_dir — positive polarity (over-correction guard)
    // -----------------------------------------------------------------------
    #[test]
    fn test_derive_factory_dir_appends_for_normal_base() {
        let base = PathBuf::from("/repo");
        let result = derive_factory_dir(&base);
        assert_eq!(
            result,
            base.join(".factory"),
            "derive_factory_dir must append .factory for a normal base path; \
             got {result:?}"
        );
    }
}
