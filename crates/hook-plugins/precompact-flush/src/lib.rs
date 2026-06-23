//! precompact-flush — pure logic for the native WASM PreCompact hook plugin.
//!
//! This module contains the testable pure state machine. All effectful
//! operations (host `read_file`, `write_file`, `exec_subprocess`) are injected
//! via closures so the logic can be unit-tested without a WASM runtime.
//!
//! The effectful entry point lives in `src/main.rs` (vsdd-hook-sdk trampoline).
//!
//! # Architecture mapping (S-18.04a)
//!
//! | Component | Module | Pure/Effectful |
//! |-----------|--------|---------------|
//! | Pure state machine | `src/lib.rs` (this file) | Pure |
//! | Effectful entry point | `src/main.rs` | Effectful |
//!
//! # Canonical execution order (BC-7.07.001 INV3 / AC-011)
//!
//! 1.  Discover factory-artifacts worktree path via `git worktree list --porcelain`
//!     (AC-017); fail-open with DURABILITY DEGRADED advisory if not found.
//! 2.  Read STATE.md via host `read_file`; exit 0 + warn if unreadable (AC-002).
//! 3.  Check `factory_lock:` block via `renew_lock(state_md_content)` (AC-003, AC-018):
//!     - `Ok(NoOp)` → skip step 4 (no write_file call)
//!     - `Ok(Renewed(content))` → proceed to step 4
//!     - `Err(Malformed)` → advisory warn to stderr; skip step 4; proceed to step 5
//! 4.  If `Renewed`: call `host::write_file(".factory/STATE.md", content)` (AC-018).
//! 5.  `git -C <wt> add -A` — stage ALL changes including new untracked files (AC-004).
//!     After staging, check `git -C <wt> diff --cached`: if NoOp AND diff empty →
//!     INV5 clean-state → exit 0 silently; no commit (AC-005 / BC-7.07.001 INV5).
//! 6.  `git -C <wt> commit -m <msg>` (LOCAL; no network); exit 2 on failure (AC-005b).
//! 7.  `SHA_B = git -C <wt> rev-parse HEAD` — IMMEDIATELY after commit, BEFORE
//!     append (AC-006 / BC-7.07.001 PC8).
//! 8.  Append 4-field `\n`-terminated line to precompact-flush-log via host
//!     `write_file`; treat absent-log read error as empty baseline (AC-007).
//! 9.  If append fails: compare CURRENT_HEAD to SHA_B; if equal → `git reset --soft
//!     SHA_B^`; if not equal → no reset + human-intervention message; exit 2 (AC-008).
//! 10. If append succeeds: `git -C <wt> push origin factory-artifacts` (NETWORK) (AC-009).
//! 11. If push fails: exit 2 with retry message (AC-009).
//! 12. If push succeeds: exit 0 (AC-009).

#![cfg_attr(not(kani), allow(unexpected_cfgs))]

/// Precompact-flush log path (relative to `.factory/` host write_file root).
///
/// BC-7.07.001 Architecture Anchors / AC-015: exact path, no extension.
pub const LOG_PATH: &str = ".factory/hooks/precompact-flush-log";

/// STATE.md path (relative to `.factory/` host write_file root).
pub const STATE_MD_PATH: &str = ".factory/STATE.md";

/// Lock renewal TTL in seconds (expires_at = now + 2700s per ADR-028 §Decision 9).
pub const LOCK_RENEWAL_TTL_SECS: u64 = 2700;

/// Commit message prefix. MUST begin with exactly `PreCompact flush ` (capital P,
/// capital C, single space between words, trailing space) per BC-5.41.003 INV3 and
/// BC-7.07.001 INV4. Used as exemption key by validate-burst-log.
pub const COMMIT_PREFIX: &str = "PreCompact flush ";

/// Factory-artifacts branch name used in `git worktree list --porcelain` parsing.
pub const FACTORY_ARTIFACTS_BRANCH: &str = "refs/heads/factory-artifacts";

/// Remote and branch for `git push` (AC-009).
pub const PUSH_REMOTE: &str = "origin";
pub const PUSH_BRANCH: &str = "factory-artifacts";

// ---------------------------------------------------------------------------
// Data types used by the pure state machine
// ---------------------------------------------------------------------------

/// Outcome of worktree discovery (AC-017).
#[derive(Debug, Clone, PartialEq)]
pub enum WorktreeDiscovery {
    /// Worktree found at the given absolute path.
    Found(String),
    /// `git worktree list --porcelain` subprocess exited non-zero.
    CommandFailed { exit_code: i32 },
    /// Command succeeded but no factory-artifacts stanza found.
    BranchNotFound,
    /// Canonicalize assertion failed: discovered path != `<cwd>/.factory`.
    PathMismatch {
        discovered: String,
        expected: String,
    },
}

/// Parsed STATE.md context fields (BC-7.07.001 PC1 / AC-005).
///
/// Derived from `current_cycle:` and `current_step:` YAML frontmatter fields.
/// NOT from `current_wave:` (phantom field — must NOT be used).
#[derive(Debug, Clone, PartialEq)]
pub struct StateContext {
    /// Value of `current_cycle:` frontmatter field.
    pub current_cycle: String,
    /// Value of `current_step:` frontmatter field.
    pub current_step: String,
}

/// Decision of the append-failure concurrent-commit guard (AC-008).
#[derive(Debug, Clone, PartialEq)]
pub enum AppendFailureAction {
    /// CURRENT_HEAD == SHA_B — safe to reset. Run `git reset --soft SHA_B^`.
    ResetSafe { sha_b: String },
    /// CURRENT_HEAD != SHA_B — intervening commit; do NOT reset.
    NoResetHumanIntervention { sha_b: String, current_head: String },
}

// ---------------------------------------------------------------------------
// Pure functions (pure state machine logic — no I/O)
// ---------------------------------------------------------------------------

/// Parse the factory-artifacts worktree path from `git worktree list --porcelain`
/// output.
///
/// Finds the stanza whose `branch` line is `refs/heads/factory-artifacts` and
/// returns the corresponding `worktree` line as the absolute path.
///
/// Returns `Some(absolute_path)` if found, `None` if no matching stanza.
///
/// Used by the effectful entry point in `src/main.rs` (AC-017).
pub fn parse_worktree_list(_porcelain_output: &str) -> Option<String> {
    todo!()
}

/// Parse `current_cycle` and `current_step` from STATE.md content.
///
/// Scans the YAML frontmatter block (between `---` fences) for `current_cycle:`
/// and `current_step:` lines. Returns `None` if either field is missing.
///
/// # Note on phantom field
///
/// MUST NOT read `current_wave:` — that is a phantom field (BC-7.07.001 PC1).
///
/// Used by the effectful entry point to derive commit message context (AC-005).
pub fn parse_state_context(_state_md_content: &str) -> Option<StateContext> {
    todo!()
}

/// Build the commit message for the flush commit (BC-7.07.001 INV4 / AC-005).
///
/// Format: `PreCompact flush <cycle>/<step> <ISO-8601-timestamp>`
///
/// The subject MUST begin with `PreCompact flush ` (capital P, capital C, single
/// space between words, trailing space — case-sensitive per BC-5.41.003 INV3).
///
/// `timestamp` must be in YYYY-MM-DDTHH:MM:SSZ format (UTC, second precision,
/// uppercase Z suffix).
pub fn build_commit_message(_ctx: &StateContext, _timestamp: &str) -> String {
    todo!()
}

/// Build the log entry line for the precompact-flush-log (AC-007).
///
/// Format: `<ISO-timestamp> <SHA_B> <cycle>/<step> commit\n`
///
/// The trailing `\n` is embedded in the returned string (mandatory per AC-007).
/// Field-4 is the literal string `commit` (never a variable).
pub fn build_log_entry(_timestamp: &str, _sha_b: &str, _ctx: &StateContext) -> String {
    todo!()
}

/// Decide the append-failure action based on CURRENT_HEAD vs SHA_B (AC-008).
///
/// # Semantics
///
/// - If `current_head == sha_b`: safe to reset (`git reset --soft SHA_B^`).
/// - If `current_head != sha_b`: concurrent commit advanced HEAD; do NOT reset.
pub fn decide_append_failure_action(_sha_b: &str, _current_head: &str) -> AppendFailureAction {
    todo!()
}

/// Check whether the staged diff is empty (INV5 clean-state guard, AC-005 / AC-011).
///
/// Returns `true` if the `git diff --cached` output is empty (no staged changes),
/// `false` otherwise.
///
/// When `true` AND `renew_lock()` returned `NoOp`, the plugin MUST exit 0 silently
/// without creating an empty commit (BC-7.07.001 INV5).
pub fn is_diff_empty(_git_diff_cached_output: &str) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// Effectful plugin entry point (called from src/main.rs)
// ---------------------------------------------------------------------------

/// Run the precompact-flush plugin logic given a hook payload.
///
/// This is the effectful entry point invoked by the vsdd-hook-sdk trampoline via
/// `src/main.rs`. All host I/O (`read_file`, `write_file`, `exec_subprocess`) is
/// routed through the SDK's host bindings.
///
/// # Canonical execution order
///
/// Steps 1–12 per BC-7.07.001 INV3 / AC-011 (see module-level doc).
///
/// # Return value
///
/// Returns a `vsdd_hook_sdk::HookResult`:
/// - `HookResult::Continue` — exit 0 (push succeeded, or clean-state no-op, or
///   fail-open on STATE.md/worktree discovery error).
/// - `HookResult::Block(message)` — exit 2 (commit failure, push failure, or
///   append failure after SHA-pinned reset).
pub fn run_plugin(_payload: vsdd_hook_sdk::HookPayload) -> vsdd_hook_sdk::HookResult {
    todo!()
}

/// Injectable variant of `run_plugin` for unit testing (TDD Red Gate stub).
///
/// Accepts mock closures for host I/O so that the effectful plugin logic can be
/// exercised without a WASM runtime. The implementer MUST implement this function
/// as the testable core of the plugin, with `run_plugin` delegating to it via
/// the real host bindings.
///
/// # Parameters
///
/// - `payload` — the PreCompact hook payload (contains event_name, session_id, etc.)
/// - `read_file` — mock for `host::read_file(path) -> Result<String, String>`
/// - `write_file` — mock for `host::write_file(path, content) -> Result<(), String>`
/// - `exec_subprocess` — mock for `host::exec_subprocess(binary, args) -> Result<(exit_code: i32, stdout: String, stderr: String), String>`
///
/// # Canonical execution order
///
/// Same as `run_plugin` — all 12 steps of BC-7.07.001 INV3 / AC-011 apply.
///
/// # Return value
///
/// Same as `run_plugin` — `HookResult::Continue` on success, `HookResult::Block`
/// on commit/push/append failure.
pub fn run_plugin_with_mock<RF, WF, ES>(
    _payload: vsdd_hook_sdk::HookPayload,
    _read_file: RF,
    _write_file: WF,
    _exec_subprocess: ES,
) -> vsdd_hook_sdk::HookResult
where
    RF: Fn(&str) -> Result<String, String>,
    WF: Fn(&str, &str) -> Result<(), String>,
    ES: Fn(&str, &[&str]) -> Result<(i32, String, String), String>,
{
    todo!()
}
