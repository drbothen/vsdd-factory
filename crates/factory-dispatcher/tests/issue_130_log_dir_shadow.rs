// Test files use .expect()/.unwrap() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Red Gate integration tests for issue #130 — dispatcher log-dir shadow.
//!
//! ADR-024 Decision 1 Level D requires that when cwd is a subdirectory
//! INSIDE `.factory/` and no env-var override is present, `resolve_log_dir()`
//! walks up to the enclosing `.factory/` directory and returns
//! `<parent>/.factory/logs` — NOT `./<cwd>/.factory/logs` (which would
//! produce a nested shadow on subsequent accesses).
//!
//! These tests are INTEGRATION tests because they exercise the dispatcher
//! binary's log-dir resolution through a filesystem fixture rather than
//! testing the private `resolve_log_dir()` function directly.
//!
//! # TEST-ONLY SEAM NOTE (for implementer)
//!
//! The current `resolve_log_dir()` is a private free function in `main.rs`.
//! This integration test verifies the OBSERVABLE EFFECT of log-dir
//! resolution:  the dispatcher must NOT create a `.factory/.factory/`
//! directory when invoked from inside `.factory/cycles/`.
//!
//! To make this observable without spawning the binary (which would require
//! a full registry), the implementer MUST expose one of:
//!
//!   (a) `pub(crate) fn resolve_log_dir() -> PathBuf` in `main.rs` (already
//!       accessible to intra-crate tests but NOT to the `tests/` directory
//!       which is a separate compilation unit), OR
//!
//!   (b) A re-export from `lib.rs`:
//!       `pub fn resolve_log_dir_for_testing() -> PathBuf`  (or similar), OR
//!
//!   (c) A pure helper `pub fn resolve_log_dir_from(project_dir: Option<&str>,
//!       cwd: &Path) -> PathBuf` re-exported from lib.rs that the unit tests
//!       in `main.rs` and this file both use.
//!
//! Option (c) is the production-grade choice: it makes the function testable
//! AND eliminates global env-var side effects.  The implementer should
//! choose (c) and update the tests in `main.rs` accordingly.
//!
//! Until the seam is added, this test file will FAIL TO COMPILE when run
//! against current HEAD — which is the correct Red Gate behavior: the test
//! cannot even exercise an unimplemented seam.
//!
//! # AC-5 (ADR-024 Consequences)
//!
//! "AC-5 (regression test) is a test-writer deliverable, not an architectural
//! decision."  This test IS that deliverable.

use factory_dispatcher::resolve_log_dir_from;
use std::fs;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Test: cwd inside .factory/cycles → resolves to <project>/.factory/logs
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
/// Calls `resolve_log_dir_from(None, cwd)` where `cwd` =
/// `<tmpdir>/.factory/cycles/v1.0-pass-1/`.
///
/// Expected: result == `<tmpdir>/.factory/logs`
/// Forbidden: result contains `.factory/.factory` (the shadow).
///
/// Also asserts that calling `InternalLog::new(result).write(...)` does NOT
/// create `<tmpdir>/.factory/.factory/` anywhere on disk.
#[test]
fn test_BC_2_06_001_resolve_log_dir_cwd_inside_factory() {
    use factory_dispatcher::internal_log::{DISPATCHER_STARTED, InternalEvent, InternalLog};

    let tmpdir = tempfile::tempdir().expect("create tmpdir");
    let factory_root = tmpdir.path().join(".factory");
    let cwd = factory_root.join("cycles").join("v1.0-pass-1");
    fs::create_dir_all(&cwd).expect("create simulated cwd");

    // Call the pure helper (implementer must expose this from lib.rs).
    // No CLAUDE_PROJECT_DIR, no VSDD_LOG_DIR — pure cwd walk-up path.
    let log_dir: PathBuf = resolve_log_dir_from(None, &cwd);

    // ---------- AC-5 assertion 1: correct resolution ----------
    let expected = factory_root.join("logs");
    assert_eq!(
        log_dir, expected,
        "resolve_log_dir_from with cwd inside .factory/cycles/ must walk up to \
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
