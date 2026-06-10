//! Worktree-aware log-directory resolution (ADR-024 Decision 1).
//!
//! Exposes `resolve_log_dir_from` as a pure helper so that both the
//! `main.rs` startup path and the integration-test suite can exercise
//! the resolution algorithm without touching the process environment.
//!
//! # Resolution order (six levels, first match wins)
//!
//! A. `VSDD_LOG_DIR` env var — set and non-empty → use directly (append
//!    `logs` only if not already ending in `logs`). No `.factory` re-appended.
//! B. `FACTORY_ROOT` env var — set and non-empty → `$FACTORY_ROOT/logs`.
//! C. `project_dir` / cwd basename == `.factory` (case-insensitive) →
//!    use the path directly, append `logs`. Primary shadow fix.
//! D. Walk parent chain from `cwd` to find an enclosing `.factory` dir.
//!    Guard symlink loops via `(st_dev, st_ino)` tracking. Append `logs`.
//! E. `git worktree list --porcelain` first entry → `<path>/.factory/logs`.
//!    200ms hard timeout; git absent/timeout/non-repo → fall through.
//! F. Fallback: `./.factory/logs` (cwd-relative).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve the internal log directory using the six-level ADR-024 algorithm.
///
/// # Parameters
///
/// - `project_dir`: the value of `CLAUDE_PROJECT_DIR` (or `None` if unset/empty).
///   This corresponds to ADR-024 level-C check. The caller (thin `resolve_log_dir()`
///   wrapper) reads env vars and passes them here so the function is testable without
///   mutating the process environment.
/// - `vsdd_log_dir`: the value of `VSDD_LOG_DIR` (level A override).
/// - `factory_root`: the value of `FACTORY_ROOT` (level B override).
/// - `cwd`: the process current working directory (for levels C, D, E, F).
///
/// # Returns
///
/// A `PathBuf` for the directory in which daily-rotated JSONL files should be written.
/// Never panics; every error branch falls through to the next level or to F.
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

    // ── Level E: git worktree list ───────────────────────────────────────────
    if let Some(worktree_root) = git_worktree_main_root(cwd) {
        let candidate = worktree_root.join(".factory").join("logs");
        return candidate;
    }

    // ── Level F: cwd-relative fallback ──────────────────────────────────────
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
        .map(|s| is_factory_name(s))
        .unwrap_or(false)
}

/// Returns `true` if the path already ends in `logs` or `logs/`.
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
        if let Some(key) = symlink_inode(&current) {
            if !seen.insert(key) {
                // Already visited — symlink loop detected, stop.
                break;
            }
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
                    return None;
                }
                std::thread::sleep(poll_interval);
            }
            Err(_) => {
                let _ = child.kill();
                return None;
            }
        }
    }
}

/// Drain stdout from a child whose stdout pipe we captured.
fn read_piped_stdout(child: &mut std::process::Child) -> Vec<u8> {
    use std::io::Read;
    let mut buf = Vec::new();
    if let Some(ref mut stdout) = child.stdout {
        let _ = stdout.read_to_end(&mut buf);
    }
    buf
}
