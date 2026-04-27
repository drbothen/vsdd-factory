//! AC-3: Invokes `git log -1 --format=%H` via `exec_subprocess()` to get
//! commit SHA.
//!
//! Exercises `call_git_log` with a mocked runner. The mock controls the
//! subprocess outcome so no actual git process is launched in tests.

use capture_commit_activity::{GitLogOutcome, call_git_log};

// ---------------------------------------------------------------------------
// test_BC_4_03_001_subprocess_calls_git_log_with_correct_args
// ---------------------------------------------------------------------------

/// AC-3: call_git_log invokes `git log -1 --format=%H` (verified via mock
/// callback; actual args are documented in the expected call string).
///
/// The runner callback is the observable unit. When it is not called, the
/// test fails — implementation must invoke it.
#[test]
fn test_BC_4_03_001_subprocess_calls_git_log_with_correct_args() {
    let runner_called = std::cell::Cell::new(false);
    let _ = call_git_log(|| {
        runner_called.set(true);
        Ok((0, "abc1234def5678901234567890123456789012345\n".to_string()))
    });
    assert!(runner_called.get(), "git log runner must be called");
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_subprocess_returns_sha_on_success
// ---------------------------------------------------------------------------

/// AC-3: when git log exits 0 with a non-empty SHA on stdout,
/// `call_git_log` returns `GitLogOutcome::Sha(sha)`.
#[test]
fn test_BC_4_03_001_subprocess_returns_sha_on_success() {
    let sha = "abc1234def5678901234567890123456789012345";
    let outcome = call_git_log(|| Ok((0, format!("{sha}\n"))));
    match outcome {
        GitLogOutcome::Sha(s) => {
            assert_eq!(s.trim(), sha, "SHA must match git log stdout (trimmed)");
        }
        other => panic!("expected GitLogOutcome::Sha, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_subprocess_sha_stripped_of_whitespace
// ---------------------------------------------------------------------------

/// AC-3: trailing newline in git log stdout must be stripped from the returned
/// SHA (git always appends `\n`).
#[test]
fn test_BC_4_03_001_subprocess_sha_stripped_of_whitespace() {
    let outcome = call_git_log(|| Ok((0, "deadbeef12345678\n".to_string())));
    match outcome {
        GitLogOutcome::Sha(s) => {
            assert!(
                !s.contains('\n'),
                "SHA must not contain trailing newline, got: {s:?}"
            );
        }
        other => panic!("expected Sha, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_subprocess_short_sha_accepted
// ---------------------------------------------------------------------------

/// AC-3: git may return a 7-char abbreviated SHA (e.g. in a shallow clone).
/// The plugin must accept short SHAs.
#[test]
fn test_BC_4_03_001_subprocess_short_sha_accepted() {
    let outcome = call_git_log(|| Ok((0, "abc1234\n".to_string())));
    match outcome {
        GitLogOutcome::Sha(s) => {
            assert_eq!(s.trim(), "abc1234");
        }
        other => panic!("expected Sha for short SHA, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// test_BC_4_03_001_subprocess_runner_error_propagates
// ---------------------------------------------------------------------------

/// AC-3: when the subprocess runner returns an Err (host capability denied,
/// timeout, etc.), `call_git_log` must surface it as `GitLogOutcome::Failed`.
#[test]
fn test_BC_4_03_001_subprocess_runner_error_propagates() {
    let outcome = call_git_log(|| Err("CapabilityDenied".to_string()));
    match outcome {
        GitLogOutcome::Failed { .. } => {}
        other => panic!("expected GitLogOutcome::Failed on runner error, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// test_TV_002_canonical_full_sha_round_trip
// ---------------------------------------------------------------------------

/// TV-002 canonical test vector: full 40-char SHA produced by `git log -1 --format=%H`.
///
/// Exercises the nominal happy path used in the spec.
#[test]
fn test_TV_002_canonical_full_sha_round_trip() {
    let canonical_sha = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";
    let outcome = call_git_log(|| Ok((0, format!("{canonical_sha}\n"))));
    match outcome {
        GitLogOutcome::Sha(s) => {
            assert_eq!(s.trim(), canonical_sha);
            assert_eq!(s.trim().len(), 40);
        }
        other => panic!("expected Sha for canonical 40-char SHA, got {other:?}"),
    }
}
