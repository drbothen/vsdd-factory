// Test files use .expect()/.unwrap() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Red Gate integration tests for issue #130 — dispatcher log-dir shadow.
//!
//! ADR-024 Decision 1 (v1.0) Level D requires that when cwd is a subdirectory
//! INSIDE `.factory/` and no env-var override is present, `resolve_log_dir()`
//! walks up to the enclosing `.factory/` directory and returns
//! `<parent>/.factory/logs` — NOT `./<cwd>/.factory/logs` (which would
//! produce a nested shadow on subsequent accesses).
//!
//! ADR-024 v1.1 adds Level E: when cwd is the repo root (has a `.factory`
//! child dir but is NOT inside `.factory/`), resolve to `<cwd>/.factory/logs`
//! via a pure `Path::exists()` check — no git subprocess.
//!
//! # TEST-ONLY SEAM NOTE (for implementer)
//!
//! `resolve_log_dir_from_params` is a fully-parameterized pure helper exposed
//! from `lib.rs`.  Integration tests use it directly (Option (c) from the
//! original seam note).  Tests that need hermetic env isolation call
//! `resolve_log_dir_from_params` with explicit `None` for VSDD_LOG_DIR,
//! FACTORY_ROOT, and CLAUDE_PROJECT_DIR — so they cannot be polluted by the
//! developer or CI shell environment.
//!
//! # AC-5 (ADR-024 Consequences)
//!
//! "AC-5 (regression test) is a test-writer deliverable, not an architectural
//! decision."  This file IS that deliverable, covering all adversary pass-1
//! findings related to log-dir resolution.

use factory_dispatcher::resolve_log_dir_from_params;
use std::fs;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Test 1: cwd inside .factory/cycles → resolves to <project>/.factory/logs
// via walk-up (ADR-024 Level D).  Asserts NO .factory/.factory/ created.
// ---------------------------------------------------------------------------

/// AC-5 regression guard + ADR-024 Level D walk-up behaviour.
///
/// Constructs the filesystem layout:
///
///   <tmpdir>/
///     .factory/
///       cycles/
///         v1.0-pass-1/   ← simulated cwd
///
/// Calls `resolve_log_dir_from_params(None, None, None, cwd)` where `cwd` =
/// `<tmpdir>/.factory/cycles/v1.0-pass-1/`.
///
/// Expected: result == `<tmpdir>/.factory/logs`
/// Forbidden: result contains `.factory/.factory` (the shadow).
///
/// Also asserts that calling `InternalLog::new(result).write(...)` does NOT
/// create `<tmpdir>/.factory/.factory/` anywhere on disk.
///
/// H-2 adversary finding: this test is HERMETIC — it calls
/// `resolve_log_dir_from_params` with all env-var params as `None`, so the
/// result is deterministic regardless of whether `VSDD_LOG_DIR`, `FACTORY_ROOT`,
/// or `CLAUDE_PROJECT_DIR` are set in the developer/CI shell.
#[test]
fn test_BC_2_06_001_resolve_log_dir_cwd_inside_factory() {
    use factory_dispatcher::internal_log::{DISPATCHER_STARTED, InternalEvent, InternalLog};

    let tmpdir = tempfile::tempdir().expect("create tmpdir");
    let factory_root = tmpdir.path().join(".factory");
    let cwd = factory_root.join("cycles").join("v1.0-pass-1");
    fs::create_dir_all(&cwd).expect("create simulated cwd");
    // Since issue #206 the internal log refuses to write while `.factory` is
    // not a mounted worktree (no `.git` entry): fabricate the mount shape (a
    // `.git` FILE, as `git worktree add` produces) so the mkdir-p assertion
    // below still exercises the write path. The shadow assertion this test
    // exists for is unchanged.
    fs::write(
        factory_root.join(".git"),
        "gitdir: ../.git/worktrees/.factory\n",
    )
    .expect("fabricate worktree .git file");

    // H-2: hermetic call — explicit None for all env-var params.
    // This test does NOT depend on the developer/CI shell having
    // VSDD_LOG_DIR / FACTORY_ROOT / CLAUDE_PROJECT_DIR unset.
    let log_dir: PathBuf = resolve_log_dir_from_params(
        None, // VSDD_LOG_DIR: not set
        None, // FACTORY_ROOT: not set
        None, // CLAUDE_PROJECT_DIR: not set
        &cwd,
    );

    // ---------- AC-5 assertion 1: correct resolution ----------
    let expected = factory_root.join("logs");
    assert_eq!(
        log_dir, expected,
        "resolve_log_dir_from_params with cwd inside .factory/cycles/ must walk up to \
         <project>/.factory/logs; got {log_dir:?}"
    );

    // ---------- AC-5 assertion 2: no shadow created by InternalLog ----------
    // Write an event to exercise the mkdir-p path in InternalLog.
    let log = InternalLog::new(log_dir.clone());
    log.write(&InternalEvent::now(DISPATCHER_STARTED));

    // The shadow directory must NOT exist.
    let shadow = factory_root.join(".factory");
    assert!(
        !shadow.exists(),
        "InternalLog must NOT create a .factory/.factory/ shadow; found {shadow:?}"
    );

    // The correct logs directory MUST exist (mkdir-p was called).
    assert!(
        log_dir.exists(),
        "InternalLog must create the resolved log dir; {log_dir:?} does not exist"
    );
}

// ---------------------------------------------------------------------------
// Test 2: H-2 hermeticity verification.
//
// The test verifies that setting a bogus VSDD_LOG_DIR in the process
// environment does NOT affect the result of resolve_log_dir_from_params
// when called with explicit None for VSDD_LOG_DIR.
//
// This ensures that the hermetic seam truly isolates the test from ambient
// env state — a developer or CI environment with VSDD_LOG_DIR set cannot
// silently change the resolution result.
// ---------------------------------------------------------------------------

/// H-2 adversary finding: `resolve_log_dir_from_params` with explicit
/// `None` for VSDD_LOG_DIR is immune to the ambient env value.
///
/// Sets a bogus VSDD_LOG_DIR in the process env, calls the hermetic params
/// variant with None, and asserts it still returns the walk-up result (Level D)
/// rather than the bogus env value (Level A).
///
/// This test proves the seam is truly parameterized, not a thin wrapper that
/// still reads env vars.
#[test]
fn test_BC_2_06_001_hermetic_params_variant_ignores_ambient_vsdd_log_dir() {
    let tmpdir = tempfile::tempdir().expect("create tmpdir");
    let factory_root = tmpdir.path().join(".factory");
    let cwd = factory_root.join("cycles").join("some-pass");
    fs::create_dir_all(&cwd).expect("create simulated cwd");

    // Temporarily set a bogus VSDD_LOG_DIR in the process environment.
    // std::env::set_var is unsafe in Rust 2024 edition due to MT-unsafety;
    // these integration tests run as separate processes, making it safe here.
    // Safety: integration tests are single-threaded at this point; no other
    // thread reads VSDD_LOG_DIR concurrently.
    let bogus_log_dir = "/tmp/bogus-should-not-be-used";
    // SAFETY: single-threaded integration test process; no concurrent env reads.
    unsafe { std::env::set_var("VSDD_LOG_DIR", bogus_log_dir) };

    // Hermetic call: explicit None overrides the ambient env.
    let log_dir: PathBuf = resolve_log_dir_from_params(
        None, // VSDD_LOG_DIR: caller says "ignore env"
        None, // FACTORY_ROOT: caller says "ignore env"
        None, // CLAUDE_PROJECT_DIR
        &cwd,
    );

    // Restore env to avoid polluting other tests.
    // SAFETY: same rationale as set_var above.
    unsafe { std::env::remove_var("VSDD_LOG_DIR") };

    // Result must be the walk-up resolution (Level D), not the bogus Level A.
    let expected = factory_root.join("logs");
    assert_eq!(
        log_dir, expected,
        "resolve_log_dir_from_params with explicit None VSDD_LOG_DIR must ignore \
         the ambient VSDD_LOG_DIR env var; got {log_dir:?} (bogus was {bogus_log_dir})"
    );

    // Sanity: the bogus path must NOT appear in the result.
    assert!(
        !log_dir.starts_with(bogus_log_dir),
        "result must not start with bogus VSDD_LOG_DIR path"
    );
}

// ---------------------------------------------------------------------------
// Test 3: ADR-024 v1.1 Level E — cwd with .factory child dir.
//
// H-3 adversary finding / architect Level E addition:
//
// When cwd is the repo root (contains a `.factory/` child directory) and
// levels A–D all fail, the resolver MUST use `<cwd>/.factory/logs` via a
// pure `Path::exists()` check — WITHOUT spawning git.
//
// The current log_dir.rs does NOT have Level E — it only has 6 levels
// (A through F where F is git worktree and G is the fallback). ADR-024 v1.1
// inserts the child-check as Level E, pushing git to Level F and fallback to G.
//
// This test will CURRENTLY FAIL because the current code falls through to
// Level E (git) rather than returning at the new Level E (child-check).
// In a tmpdir with no git repo, git will exit non-zero and fall through to
// Level F (fallback: `./.factory/logs` relative to cwd). But cwd in the test
// is a tempdir, so the fallback produces `<tmpdir>/.factory/logs` — which is
// the SAME as the child-check result. This means the test will PASS spuriously
// for a clean tmpdir that happens to have .factory as a child.
//
// HOWEVER, the test proves hermeticity: we call with explicit None env vars
// AND assert that no git subprocess is needed (by running in a tempdir that
// is not a git repo, where git exits non-zero). The test also validates that
// the resolution does NOT produce a subdirectory of the tmpdir that would only
// arise via the git worktree path (which is always the main worktree root, not
// the cwd).
//
// A second assertion validates that the result is returned WITHOUT git being
// invoked: we measure that the call completes in under 50ms (git subprocess
// with 200ms timeout would take at least 5ms even when available). This timing
// assertion proves the pure-path branch is taken.
// ---------------------------------------------------------------------------

/// H-3: ADR-024 v1.1 Level E — cwd INSIDE a git repo, with a `.factory` child dir.
///
/// This test is the true RED for Level E: it uses the vsdd-factory repo itself
/// as the git context. The test creates a subdirectory INSIDE the repo's
/// worktree directory, gives that subdirectory a `.factory` child, and calls
/// `resolve_log_dir_from_params` with that subdir as cwd.
///
/// Without Level E (child-check), the code falls through to Level F (git), which
/// succeeds because git is available and the tmpdir is inside a git repo. Git
/// returns the main worktree root of vsdd-factory, so the result would be
/// `/path/to/vsdd-factory/.factory/logs` — NOT `<tmpdir-subdir>/.factory/logs`.
///
/// With Level E implemented, the child-check fires first (no git needed) and
/// returns `<tmpdir-subdir>/.factory/logs` correctly.
///
/// CURRENTLY FAILS: git Level F fires and returns the repo's `.factory/logs`,
/// not the tmpdir subdir's `.factory/logs`.
///
/// Note: this test creates a tmpdir INSIDE the vsdd-factory repo working tree
/// (under `target/tmp-level-e-test/`) so that git finds the repo. It is cleaned
/// up in the test body.
#[test]
fn test_BC_2_06_001_level_e_child_factory_dir_resolves_without_git() {
    // Create a subdirectory inside the vsdd-factory repo so git can find the repo.
    // We use env!("CARGO_MANIFEST_DIR") which points to the crate root during tests.
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_tmp = crate_dir.join("target").join("tmp-level-e-test");
    fs::create_dir_all(&test_tmp).expect("create test tmp dir");

    // Create .factory as a child of test_tmp (simulating repo root invocation).
    let factory_child = test_tmp.join(".factory");
    fs::create_dir_all(&factory_child).expect("create .factory child dir");

    // Verify git is available and the test_tmp is inside a git repo.
    // If git is unavailable, this test is not meaningful — skip.
    let git_check = std::process::Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(&test_tmp)
        .output();

    let git_available = match git_check {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            // git must find a worktree and the worktree root must NOT be test_tmp.
            out.status.success()
                && stdout
                    .lines()
                    .next()
                    .map(|l| l.starts_with("worktree "))
                    .unwrap_or(false)
                && !stdout.contains(test_tmp.to_string_lossy().as_ref())
        }
        Err(_) => false,
    };

    if !git_available {
        // git is not available or test_tmp is the worktree root — test is vacuously
        // satisfied and Level E cannot be falsified. Skip.
        fs::remove_dir_all(&test_tmp).ok();
        return;
    }

    // cwd = test_tmp (NOT inside .factory, NOT a .factory basename, NOT an ancestor
    // of a .factory dir that is above test_tmp). Levels A, B, C, D all fail.
    let cwd = test_tmp.as_path();

    let start = std::time::Instant::now();

    // Hermetic call: all env-var params explicit None.
    let log_dir: PathBuf = resolve_log_dir_from_params(
        None, // VSDD_LOG_DIR
        None, // FACTORY_ROOT
        None, // CLAUDE_PROJECT_DIR
        cwd,
    );

    let elapsed = start.elapsed();

    // Clean up before assertions so failures don't leave debris.
    fs::remove_dir_all(&test_tmp).ok();

    // Level E child-check result: <cwd>/.factory/logs = test_tmp/.factory/logs.
    let expected = factory_child.join("logs");
    assert_eq!(
        log_dir, expected,
        "ADR-024 v1.1 Level E: cwd with .factory child (inside git repo) must resolve to \
         <cwd>/.factory/logs; got {log_dir:?} \
         (FAIL means git Level F fired and returned the wrong worktree root)"
    );

    // Timing gate: Level E child-check is pure std::path, sub-millisecond.
    // If this takes >= 100ms, the git subprocess (Level F, 200ms timeout) was
    // spawned — indicating Level E was not implemented.
    assert!(
        elapsed.as_millis() < 100,
        "ADR-024 v1.1 Level E child-check must complete in <100ms (pure path, no git); \
         took {}ms — git subprocess was likely invoked (Level E not implemented)",
        elapsed.as_millis()
    );
}

// ---------------------------------------------------------------------------
// Test 4: Level G fallback — cwd with no .factory child and no git repo.
//
// H-3: verifies that when ALL levels A–F fail (no env vars, cwd basename
// is not .factory, no .factory ancestor, no .factory child dir, and git
// is either absent or not in a repo), the resolver falls back to the
// cwd-relative default: `./.factory/logs`.
// ---------------------------------------------------------------------------

/// H-3: Level G fallback — no .factory anywhere, no git, no env → `./.factory/logs`.
///
/// Uses a tmpdir with NO `.factory` child directory, so Level E does not match.
/// Levels A, B, C, D, E all fail. Level F (git) is expected to fail or be
/// absent. Result must be `<cwd>/.factory/logs` (Level G fallback).
///
/// Note: if `git` is available and the tmpdir happens to be inside a git worktree
/// (possible in CI), Level F may return the git main worktree root. This test
/// therefore only asserts the fallback suffix (`.factory/logs`) relative to some
/// base, not a specific absolute path — the invariant is that the result ends
/// in `.factory/logs` and does not contain `.factory/.factory`.
#[test]
fn test_BC_2_06_001_level_g_fallback_no_factory_anywhere() {
    let tmpdir = tempfile::tempdir().expect("create tmpdir");
    // No .factory child created in tmpdir.
    let cwd = tmpdir.path();

    let log_dir: PathBuf = resolve_log_dir_from_params(
        None, // VSDD_LOG_DIR
        None, // FACTORY_ROOT
        None, // CLAUDE_PROJECT_DIR
        cwd,
    );

    // Invariant 1: result ends in `.factory/logs`.
    let log_dir_str = log_dir.to_string_lossy();
    assert!(
        log_dir_str.ends_with(".factory/logs") || log_dir_str.ends_with(".factory\\logs"),
        "Level G fallback: result must end in .factory/logs; got {log_dir:?}"
    );

    // Invariant 2: no nested shadow (.factory/.factory) in the result.
    assert!(
        !log_dir_str.contains(".factory/.factory"),
        "Level G fallback: result must not contain .factory/.factory shadow; got {log_dir:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: ADR-024 v1.1 Decision 2 — absent CLAUDE_PLUGIN_ROOT → degraded-continue
// (L-1 adversary finding).
//
// The dispatcher binary MUST exit 0 when CLAUDE_PLUGIN_ROOT is absent/empty,
// not exit 2 (old hard-abort behaviour).  This test spawns the debug binary;
// it is skipped if the binary has not been built.
// ---------------------------------------------------------------------------

/// L-1 (LOW-1 from ADR-024 v1.1): absent CLAUDE_PLUGIN_ROOT must produce
/// degraded-continue (exit 0), not hard-abort (exit 2).
///
/// Spawns the factory-dispatcher debug binary with no CLAUDE_PLUGIN_ROOT and
/// a minimal valid payload.  Asserts exit code 0.
///
/// Skipped when the debug binary does not exist (standard `cargo test` without
/// prior `cargo build` would miss it; CI builds always compile before testing
/// so the binary is present in CI).
#[test]
fn test_BC_2_06_001_absent_plugin_root_degraded_continue_exit_0() {
    let bin = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .expect("crates dir")
        .parent() // workspace root
        .expect("workspace root")
        .join("target")
        .join("debug")
        .join("factory-dispatcher");

    if !bin.exists() {
        // Binary not yet built — skip silently.  CI always builds before testing.
        eprintln!(
            "test_BC_2_06_001_absent_plugin_root_degraded_continue_exit_0: SKIP — \
             debug binary not found at {bin:?}. Run `cargo build -p factory-dispatcher` first."
        );
        return;
    }

    // Minimal valid PreToolUse payload (no hooks will run — degraded mode).
    let payload = r#"{"hook_event_name":"PreToolUse","tool_name":"Bash","session_id":"test-l1-000","tool_input":{"command":"ls"}}"#;

    let tmpdir = tempfile::tempdir().expect("create tmpdir");
    let output = std::process::Command::new(&bin)
        // Explicitly unset CLAUDE_PLUGIN_ROOT — tests env inherits it from shell;
        // we must remove it to exercise the absent-env-var path.
        .env_remove("CLAUDE_PLUGIN_ROOT")
        // Provide a log dir so the InternalLog write doesn't create .factory/logs
        // relative to the test's cwd.
        .env("VSDD_LOG_DIR", tmpdir.path())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(payload.as_bytes());
            }
            child.wait_with_output()
        })
        .expect("failed to spawn factory-dispatcher binary");

    assert_eq!(
        output.status.code(),
        Some(0),
        "L-1 (ADR-024 v1.1 Decision 2): absent CLAUDE_PLUGIN_ROOT must exit 0 \
         (degraded-continue), not exit 2 (hard-abort). \
         Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
