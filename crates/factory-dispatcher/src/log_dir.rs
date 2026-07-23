//! Worktree-aware log-directory resolution (ADR-024 Decision 1, v1.1).
//!
//! Exposes `resolve_log_dir_from` as a pure helper so that both the
//! `main.rs` startup path and the integration-test suite can exercise
//! the resolution algorithm without touching the process environment.
//!
//! # Resolution order (seven levels A–G, first match wins)
//!
//! A. `VSDD_LOG_DIR` env var — set and non-empty → use directly (append
//!    `logs` only if not already ending in `logs` or `logs/`).
//!    No `.factory` re-appended.
//! B. `FACTORY_ROOT` env var — set and non-empty → `$FACTORY_ROOT/logs`.
//! C. `project_dir` / cwd basename == `.factory` (case-insensitive) →
//!    use the path directly, append `logs`. Primary shadow fix.
//! D. Walk parent chain from `cwd` to find an enclosing `.factory` dir.
//!    Guard symlink loops via `(st_dev, st_ino)` tracking. Append `logs`.
//! E. Cwd child `.factory` directory exists (`<cwd>/.factory` is a dir) →
//!    `<cwd>/.factory/logs`. Pure `Path::is_dir()` check; no subprocess.
//!    Handles the dominant repo-root invocation pattern without spawning git.
//! F. `git worktree list --porcelain` first entry → `<path>/.factory/logs`.
//!    200ms hard timeout; git absent/timeout/non-repo → fall through.
//!    Only reached for linked-worktree invocations where Level E did not fire.
//! G. Fallback: `./.factory/logs` (cwd-relative).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve the internal log directory using the seven-level A–G ADR-024 v1.1 algorithm.
///
/// # Parameters
///
/// - `vsdd_log_dir`: the value of `VSDD_LOG_DIR` (level A override).
/// - `factory_root`: the value of `FACTORY_ROOT` (level B override).
/// - `project_dir`: the value of `CLAUDE_PROJECT_DIR` (or `None` if unset/empty).
///   This corresponds to ADR-024 level-C check. The caller (thin `resolve_log_dir()`
///   wrapper) reads env vars and passes them here so the function is testable without
///   mutating the process environment.
/// - `cwd`: the process current working directory (for levels C, D, E, F, G).
///
/// # Returns
///
/// A `PathBuf` for the directory in which daily-rotated JSONL files should be written.
/// Never panics; every error branch falls through to the next level or to G.
pub fn resolve_log_dir_from_params(
    vsdd_log_dir: Option<&str>,
    factory_root: Option<&str>,
    project_dir: Option<&str>,
    cwd: &Path,
) -> PathBuf {
    // ── Level A: VSDD_LOG_DIR override ──────────────────────────────────────
    if let Some(log_dir) = vsdd_log_dir.filter(|s| !s.is_empty()) {
        let p = PathBuf::from(log_dir);
        // Append `logs` only if not already ending in `logs`.
        return if ends_with_logs(&p) {
            p
        } else {
            p.join("logs")
        };
    }

    // ── Level B: FACTORY_ROOT override ──────────────────────────────────────
    if let Some(root) = factory_root.filter(|s| !s.is_empty()) {
        return PathBuf::from(root).join("logs");
    }

    // ── Level C: project_dir or cwd basename already `.factory` ─────────────
    // Check project_dir first; if absent use cwd directly for the basename check.
    let candidate = project_dir
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| cwd.to_path_buf());

    if is_dot_factory_basename(&candidate) {
        return candidate.join("logs");
    }

    // ── Level D: walk parents from cwd to find enclosing `.factory` ──────────
    if let Some(factory_dir) = walk_up_to_factory(cwd) {
        return factory_dir.join("logs");
    }

    // ── Level E: cwd child `.factory` directory ──────────────────────────────
    // Handles the dominant repo-root invocation pattern: cwd is the repo root
    // and `.factory/` is a child directory.  Pure `Path::is_dir()` — no subprocess.
    // This eliminates the git subprocess (Level F) for the common non-worktree case.
    {
        let child = cwd.join(".factory");
        if child.is_dir() {
            return child.join("logs");
        }
    }

    // ── Level F: git worktree list ───────────────────────────────────────────
    // Only reached for linked-worktree invocations where cwd has no .factory child
    // and no .factory ancestor.  Spawns git with a 200ms hard timeout.
    if let Some(worktree_root) = git_worktree_main_root(cwd) {
        return worktree_root.join(".factory").join("logs");
    }

    // ── Level G: cwd-relative fallback ──────────────────────────────────────
    cwd.join(".factory").join("logs")
}

/// Convenience wrapper that reads env vars from the process environment and
/// calls `resolve_log_dir_from_params`. Used by `main.rs` at startup.
///
/// Exposed for integration tests via the `resolve_log_dir_from` re-export in
/// `lib.rs` — tests that need to control env vars directly should call
/// `resolve_log_dir_from_params` instead.
pub fn resolve_log_dir_from(project_dir: Option<&str>, cwd: &Path) -> PathBuf {
    let vsdd_log_dir_val = std::env::var("VSDD_LOG_DIR").ok();
    let factory_root_val = std::env::var("FACTORY_ROOT").ok();
    resolve_log_dir_from_params(
        vsdd_log_dir_val.as_deref(),
        factory_root_val.as_deref(),
        project_dir,
        cwd,
    )
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Returns `true` if the path's final component is `.factory`
/// (case-insensitive on macOS/Windows; case-sensitive on Linux).
fn is_dot_factory_basename(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(is_factory_name)
        .unwrap_or(false)
}

/// Returns `true` if the path already ends in `logs` or `logs/`.
///
/// `Path::file_name()` uses basename semantics: it strips any trailing path
/// separators before extracting the last component. This means both
/// `/some/dir/logs` and `/some/dir/logs/` return `file_name() == "logs"`, so
/// the check `s == "logs"` handles both the plain and trailing-slash forms
/// transparently.  A unit test (`test_ends_with_logs_trailing_slash`) confirms
/// this behaviour so it is not silently lost if the implementation changes.
fn ends_with_logs(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s == "logs")
        .unwrap_or(false)
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

/// Returns `true` when writing into `log_dir` cannot race the `.factory`
/// worktree bootstrap (issue #206).
///
/// Every level C–G resolution is shaped `<root>/.factory/logs`. Creating that
/// directory while `.factory` is absent — or while it exists only as a plain
/// directory with no `.git` entry — plants exactly the plain `.factory/` that
/// makes a later `git worktree add .factory factory-artifacts` mount NESTED at
/// `.factory/.factory` (issues #203/#205). The dispatcher fires on every tool
/// use, so its unconditional `mkdir -p` continuously recreated that state
/// during `/factory-health` setup.
///
/// Ready iff the `.factory` parent exists AND carries a `.git` entry that is
/// a plausible git marker — a worktree mount has a `.git` *file* whose
/// content is a `gitdir:` pointer, a plain checkout a `.git` *directory*;
/// both count, but a dangling/garbage `.git` file does not. A `log_dir`
/// whose parent is not named `.factory` (an override pointing elsewhere) is
/// always ready: that path cannot collide with the mount. This helper only
/// classifies the path shape — the explicit level A override (`VSDD_LOG_DIR`)
/// is exempted wholesale by the caller ([`mount_gate_exempt`] →
/// `InternalLog::with_mount_gate(false)`, wired in `main.rs`), so an operator
/// pointing `VSDD_LOG_DIR` at a `.factory/logs` path is honored verbatim.
/// Level B (`FACTORY_ROOT`) is NOT exempted: it resolves to
/// `$FACTORY_ROOT/logs`, and the conventional `FACTORY_ROOT=<repo>/.factory`
/// is exactly the racing shape this gate holds back.
pub fn factory_mount_ready(log_dir: &Path) -> bool {
    let Some(parent) = log_dir.parent() else {
        return true;
    };
    if !is_dot_factory_basename(parent) {
        return true;
    }
    let git_entry = parent.join(".git");
    // A `.git` DIRECTORY is a plain checkout of the artifact branch.
    if git_entry.is_dir() {
        return true;
    }
    // `git worktree add` mounts carry a `.git` FILE whose content is a
    // `gitdir:` pointer. Any other content (or an unreadable/absent entry)
    // is not a mount — the gate stays closed rather than trusting an
    // arbitrary entry that happens to be named `.git`.
    std::fs::read_to_string(&git_entry).is_ok_and(|s| s.trim_start().starts_with("gitdir:"))
}

/// Level-A override detection for the #206 mount-gate exemption: ONLY
/// `VSDD_LOG_DIR` — the per-invocation diagnostic override, whose value is
/// used verbatim — bypasses the gate. `FACTORY_ROOT` (level B) deliberately
/// does not qualify; see [`factory_mount_ready`]. Parameterized on the env
/// value so the exemption rule is unit-testable without process-global env
/// mutation.
pub fn mount_gate_exempt(vsdd_log_dir: Option<&str>) -> bool {
    vsdd_log_dir.is_some_and(|v| !v.is_empty())
}

/// Walk the parent chain from `start` upward. Returns the first ancestor
/// whose basename is `.factory`, or `None` if the filesystem root is reached
/// or a symlink loop is detected.
///
/// Symlink-loop detection tracks visited `(st_dev, st_ino)` pairs via
/// `symlink_metadata`. If a path has already been seen, the walk stops.
fn walk_up_to_factory(start: &Path) -> Option<PathBuf> {
    let mut seen: HashSet<(u64, u64)> = HashSet::new();
    let mut current = start.to_path_buf();

    loop {
        // Record this node to detect symlink loops using (dev, ino) on Unix.
        // On non-Unix platforms we skip loop detection (no stable inode API).
        if let Some(key) = symlink_inode(&current)
            && !seen.insert(key)
        {
            // Already visited — symlink loop detected, stop.
            break;
        }

        // Check if THIS directory is `.factory`.
        if is_dot_factory_basename(&current) {
            return Some(current);
        }

        // Move to parent.
        match current.parent() {
            Some(parent) if parent != current => {
                current = parent.to_path_buf();
            }
            _ => break,
        }
    }

    None
}

/// Returns `(dev, ino)` for symlink-loop detection on Unix.
/// Returns `None` on non-Unix or if metadata is unavailable.
#[cfg(unix)]
fn symlink_inode(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    std::fs::symlink_metadata(path)
        .ok()
        .map(|m| (m.dev(), m.ino()))
}

#[cfg(not(unix))]
fn symlink_inode(_path: &Path) -> Option<(u64, u64)> {
    None
}

/// Attempt to resolve the main worktree root via `git worktree list --porcelain`.
///
/// Spawns `git` with a hard 200ms timeout. Returns `None` if:
/// - `git` is not on PATH
/// - the process times out
/// - git exits non-zero (not a repo)
/// - the first `worktree <path>` line is absent
/// - the path does not exist as a directory
fn git_worktree_main_root(cwd: &Path) -> Option<PathBuf> {
    use std::process::Command;
    use std::time::Duration;

    // Spawn git. If it's not on PATH, Command::spawn returns Err — treat as
    // fallthrough with no error logged (git absent is not an error condition per ADR-024).
    let mut child = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(cwd)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    // Wait with a hard 200ms timeout.
    let timeout = Duration::from_millis(200);
    let output = wait_with_timeout(&mut child, timeout)?;

    if !output.status.success() {
        return None;
    }

    // Parse the first `worktree <path>` line.
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(path_str) = line.strip_prefix("worktree ") {
            let p = PathBuf::from(path_str.trim());
            if p.is_dir() {
                return Some(p);
            }
        }
    }

    None
}

/// Wait for a child process to finish, killing it after `timeout` elapses.
///
/// Returns `Some(output)` if the process finished within the timeout,
/// `None` if the timeout was exceeded (and the child is killed).
fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: std::time::Duration,
) -> Option<std::process::Output> {
    use std::time::Instant;

    let deadline = Instant::now() + timeout;
    let poll_interval = std::time::Duration::from_millis(5);

    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                // Process finished. Collect output from stdio we captured.
                // We need to call wait_with_output but we've already called try_wait;
                // use a custom drain approach.
                let stdout = read_piped_stdout(child);
                let status = child.wait().ok()?;
                return Some(std::process::Output {
                    status,
                    stdout,
                    stderr: vec![],
                });
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    // Reap the zombie: wait for the child after kill so the OS can
                    // reclaim its process table entry (M-3 adversary finding).
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(poll_interval);
            }
            Err(_) => {
                let _ = child.kill();
                // Reap the zombie on error path as well.
                let _ = child.wait();
                return None;
            }
        }
    }
}

/// Drain all bytes from the captured stdout pipe of a child process.
///
/// Called after `try_wait()` reports the process has exited, so `read_to_end`
/// will reach EOF promptly.  The function name `read_piped_stdout` reflects
/// that only the piped stdout handle is drained — stderr is discarded
/// (`Stdio::null()` in the spawner).
fn read_piped_stdout(child: &mut std::process::Child) -> Vec<u8> {
    use std::io::Read;
    let mut buf = Vec::new();
    if let Some(ref mut stdout) = child.stdout {
        let _ = stdout.read_to_end(&mut buf);
    }
    buf
}

#[cfg(test)]
mod factory_mount_ready_tests {
    use super::*;

    /// Issue #206: `.factory` absent → NOT ready. The dispatcher must not
    /// plant a plain `.factory/` ahead of the worktree mount.
    #[test]
    fn test_not_ready_when_factory_absent() {
        let dir = tempfile::tempdir().unwrap();
        let log_dir = dir.path().join(".factory").join("logs");
        assert!(
            !factory_mount_ready(&log_dir),
            "must not be ready when .factory does not exist"
        );
    }

    /// Issue #206/#203: `.factory` exists as a plain directory (the
    /// onboard-before-health state) → NOT ready. Writing into it is the race.
    #[test]
    fn test_not_ready_for_plain_factory_dir() {
        let dir = tempfile::tempdir().unwrap();
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(&factory).unwrap();
        assert!(
            !factory_mount_ready(&factory.join("logs")),
            "plain .factory dir without .git is the bootstrap-conflict state"
        );
    }

    /// `.factory` with a `.git` FILE — the `git worktree add` mount shape —
    /// is ready.
    #[test]
    fn test_ready_for_worktree_mount_git_file() {
        let dir = tempfile::tempdir().unwrap();
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(&factory).unwrap();
        std::fs::write(factory.join(".git"), "gitdir: ../.git/worktrees/.factory\n").unwrap();
        assert!(factory_mount_ready(&factory.join("logs")));
    }

    /// `.factory` with a `.git` DIRECTORY — a plain checkout of the artifact
    /// branch (the CI mount shape) — is ready.
    #[test]
    fn test_ready_for_checkout_git_dir() {
        let dir = tempfile::tempdir().unwrap();
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(factory.join(".git")).unwrap();
        assert!(factory_mount_ready(&factory.join("logs")));
    }

    /// A log dir whose parent is not named `.factory` (an override pointing
    /// elsewhere) is always ready — it cannot collide with the worktree mount.
    #[test]
    fn test_override_path_always_ready() {
        let dir = tempfile::tempdir().unwrap();
        let log_dir = dir.path().join("custom-diagnostics").join("logs");
        assert!(factory_mount_ready(&log_dir));
    }

    /// A garbage `.git` FILE with no `gitdir:` pointer is not a mount — the
    /// gate must not trust an arbitrary entry that is merely named `.git`.
    #[test]
    fn test_not_ready_for_garbage_git_file() {
        let dir = tempfile::tempdir().unwrap();
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(&factory).unwrap();
        std::fs::write(factory.join(".git"), "not a git pointer\n").unwrap();
        assert!(
            !factory_mount_ready(&factory.join("logs")),
            "a .git file without a gitdir: pointer is not a worktree mount"
        );
    }

    /// An empty `.git` FILE (e.g. truncated by a crashed process) is not a
    /// mount either.
    #[test]
    fn test_not_ready_for_empty_git_file() {
        let dir = tempfile::tempdir().unwrap();
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(&factory).unwrap();
        std::fs::write(factory.join(".git"), "").unwrap();
        assert!(!factory_mount_ready(&factory.join("logs")));
    }
}

#[cfg(test)]
mod mount_gate_exempt_tests {
    use super::*;

    /// Level A (VSDD_LOG_DIR) is the one and only gate exemption.
    #[test]
    fn test_vsdd_log_dir_is_exempt() {
        assert!(mount_gate_exempt(Some("/tmp/scratch/.factory/logs")));
    }

    /// Absent or empty VSDD_LOG_DIR is not an override.
    #[test]
    fn test_absent_or_empty_vsdd_log_dir_not_exempt() {
        assert!(!mount_gate_exempt(None));
        assert!(!mount_gate_exempt(Some("")));
    }

    /// Level B (FACTORY_ROOT) stays gated end-to-end: with no VSDD_LOG_DIR
    /// the exemption is off regardless of FACTORY_ROOT, the resolver maps
    /// `FACTORY_ROOT=<x>/.factory` to `<x>/.factory/logs`, and
    /// `factory_mount_ready` holds that path back until the worktree is
    /// mounted — then admits it. (The #738 re-review HIGH scenario.)
    #[test]
    fn test_factory_root_derived_path_stays_gated_until_mounted() {
        let dir = tempfile::tempdir().unwrap();
        let factory_root = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_root).unwrap();
        let factory_str = factory_root.to_str().unwrap();

        // main.rs computes the exemption from VSDD_LOG_DIR alone — a set
        // FACTORY_ROOT contributes nothing to it:
        assert!(!mount_gate_exempt(None));

        // The level-B resolution lands inside `.factory`...
        let resolved = resolve_log_dir_from_params(None, Some(factory_str), None, dir.path());
        assert_eq!(resolved, factory_root.join("logs"));

        // ...which the gate refuses while `.factory` is a plain dir...
        assert!(!factory_mount_ready(&resolved));

        // ...and admits once the worktree mount shape exists.
        std::fs::write(
            factory_root.join(".git"),
            "gitdir: ../.git/worktrees/.factory\n",
        )
        .unwrap();
        assert!(factory_mount_ready(&resolved));
    }
}

#[cfg(test)]
mod ends_with_logs_tests {
    use super::*;

    /// M-2 / N-2: `ends_with_logs` must handle both `logs` and `logs/` forms.
    /// `Path::file_name()` strips trailing separators, so both should return true.
    #[test]
    fn test_ends_with_logs_trailing_slash() {
        // Plain form.
        assert!(
            ends_with_logs(Path::new("/some/dir/logs")),
            "ends_with_logs must return true for plain 'logs' suffix"
        );
        // Trailing-slash form — file_name() strips the separator.
        assert!(
            ends_with_logs(Path::new("/some/dir/logs/")),
            "ends_with_logs must return true for 'logs/' trailing-slash form"
        );
        // Negative: not a logs path.
        assert!(
            !ends_with_logs(Path::new("/some/dir/.factory")),
            "ends_with_logs must return false for non-logs path"
        );
    }
}
