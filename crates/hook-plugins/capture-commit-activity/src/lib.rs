//! capture-commit-activity — PostToolUse/Bash WASM hook plugin.
//!
//! Parses `git commit` invocations from `tool_input.command`, extracts the
//! commit SHA via `exec_subprocess(["git", "log", "-1", "--format=%H"])`,
//! and emits a `commit.made` event with fields: sha, branch, message,
//! author, timestamp.
//!
//! Non-commit bash commands and failed git operations are no-ops (Continue).
//!
//! This stub satisfies the crate shape required by the test suite. Every
//! exported public item here is called by at least one RED-gate test.
//! Implementer: replace `unimplemented!()` bodies with real logic.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Schema for the commit.made event fields.
// ---------------------------------------------------------------------------

/// The five canonical fields emitted with every `commit.made` event.
/// Must match `commit.made` consumers (S-4.07, S-4.08).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitEventFields {
    pub sha: String,
    pub branch: String,
    pub message: String,
    pub author: String,
    pub timestamp: String,
}

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without wasmtime).
// ---------------------------------------------------------------------------

/// Returns `true` when `command` contains a `git commit` invocation.
///
/// Matches `git commit` as a real subcommand — not inside a string literal,
/// echo, or comment. Anchored the same way the legacy bash hook does it.
///
/// AC: "Parses git commit invocations from PostToolUse/Bash tool_input.command"
/// EC-002: non-git bash commands must return `false`.
pub fn is_git_commit_command(command: &str) -> bool {
    unimplemented!(
        "is_git_commit_command: not yet implemented — \
         must return true iff command contains `git commit` as a real subcommand; \
         command was: {command:?}"
    )
}

/// Outcome of `exec_subprocess("git", ["log", "-1", "--format=%H"])`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitLogOutcome {
    /// The subprocess succeeded and produced a non-empty SHA string.
    Sha(String),
    /// The subprocess succeeded but stdout was empty (EC-003).
    EmptyOutput,
    /// The subprocess failed (non-zero exit code) — EC-001.
    Failed { exit_code: i32, stderr: String },
}

/// Call `git log -1 --format=%H` via the provided runner and interpret the
/// result.
///
/// The `run_git_log` callback abstracts the host's `exec_subprocess` so unit
/// tests can drive the logic without a WASM runtime.
///
/// AC: "Invokes git log -1 --format=%H via exec_subprocess() to get commit sha"
/// EC-001: git commit fails (empty repo) → `GitLogOutcome::Failed`
/// EC-003: git log returns empty output → `GitLogOutcome::EmptyOutput`
pub fn call_git_log<F>(run_git_log: F) -> GitLogOutcome
where
    F: FnOnce() -> Result<(i32, String), String>,
{
    unimplemented!(
        "call_git_log: not yet implemented — \
         must call run_git_log(), map (exit_code=0, non-empty stdout) -> Sha, \
         (exit_code=0, empty stdout) -> EmptyOutput, \
         (non-zero exit_code) -> Failed"
    )
}

/// Build the five `CommitEventFields` from a git-log SHA and the original
/// `HookPayload`.
///
/// The `sha` comes from `call_git_log`; `branch`, `message`, `author`,
/// `timestamp` are derived from the payload and/or additional host calls.
///
/// AC: "Emits commit.made event with fields: sha, branch, message, author, timestamp"
pub fn build_commit_fields(sha: &str, payload: &HookPayload) -> CommitEventFields {
    unimplemented!(
        "build_commit_fields: not yet implemented — \
         must build CommitEventFields{{sha, branch, message, author, timestamp}} \
         from sha={sha:?} and payload fields"
    )
}

/// Top-level hook logic. Accepts a `HookPayload` and two injectable
/// callbacks so tests can drive every branch without host function calls.
///
/// `run_git_log`: returns `(exit_code, stdout)` or `Err(message)`.
/// `emit`: called with `("commit.made", &CommitEventFields)` when a commit is detected.
///
/// AC: all five ACs + EC-001/002/003.
pub fn commit_hook_logic<F, E>(payload: HookPayload, run_git_log: F, emit: E) -> HookResult
where
    F: FnOnce() -> Result<(i32, String), String>,
    E: FnOnce(&CommitEventFields),
{
    unimplemented!(
        "commit_hook_logic: not yet implemented — \
         must (1) check tool_name == Bash, (2) check is_git_commit_command on tool_input.command, \
         (3) call call_git_log, (4) build_commit_fields, (5) emit, (6) return Continue; \
         EC-001/003 -> Continue (no emit); EC-002 -> Continue (no subprocess call)"
    )
}
