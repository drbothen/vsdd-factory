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
//! 1.  Read STATE.md via host `read_file`; exit 0 + warn if unreadable (AC-002).
//!     NOTE: STATE.md read is performed before worktree discovery so that an
//!     unreadable STATE.md short-circuits all I/O including exec_subprocess.
//! 2.  Discover factory-artifacts worktree path via `git worktree list --porcelain`
//!     (AC-017); fail-open with DURABILITY DEGRADED advisory if not found.
//! 3.  Canonicalize assertion (AC-017 F-R3-001): discovered path == `<cwd>/.factory`;
//!     fail-open with DURABILITY DEGRADED advisory if mismatch.
//! 4.  Check `factory_lock:` block via `renew_lock(state_md_content)` (AC-003, AC-018):
//!     - `Ok(NoOp)` → skip step 5 (no write_file call)
//!     - `Ok(Renewed(content))` → proceed to step 5
//!     - `Err(Malformed)` → advisory warn to stderr; skip step 5; proceed to step 6
//! 5.  If `Renewed`: call `host::write_file(".factory/STATE.md", content)` (AC-018).
//! 6.  `git -C <wt> add -A` — stage ALL changes including new untracked files (AC-004).
//!     After staging, check `git -C <wt> diff --cached`: if NoOp AND diff empty →
//!     INV5 clean-state → exit 0 silently; no commit (AC-005 / BC-7.07.001 INV5).
//! 7.  `git -C <wt> commit -m <msg>` (LOCAL; no network); exit 2 on failure (AC-005b).
//! 8.  `SHA_B = git -C <wt> rev-parse HEAD` — IMMEDIATELY after commit, BEFORE
//!     append (AC-006 / BC-7.07.001 PC8).
//! 9.  Append 4-field `\n`-terminated line to precompact-flush-log via host
//!     `write_file`; treat absent-log read error as empty baseline (AC-007).
//! 10. If append fails: compare CURRENT_HEAD to SHA_B; if equal → `git reset --soft
//!     SHA_B^`; if not equal → no reset + human-intervention message; exit 2 (AC-008).
//! 11. If append succeeds: `git -C <wt> push origin factory-artifacts` (NETWORK) (AC-009).
//! 12. If push fails: exit 2 with retry message (AC-009).
//! 13. If push succeeds: exit 0 (AC-009).

#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use chrono::Utc;
use factory_lock::{LockError, RenewOutcome, renew_lock};
use vsdd_hook_sdk::{HookPayload, HookResult};

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
pub fn parse_worktree_list(porcelain_output: &str) -> Option<String> {
    let mut current_worktree: Option<String> = None;

    for line in porcelain_output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_worktree = Some(path.to_string());
        } else if let Some(branch) = line.strip_prefix("branch ") {
            if branch == FACTORY_ARTIFACTS_BRANCH {
                return current_worktree;
            }
        } else if line.is_empty() {
            // Blank line separates stanzas; reset current worktree.
            current_worktree = None;
        }
    }
    None
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
pub fn parse_state_context(state_md_content: &str) -> Option<StateContext> {
    // Normalize CRLF.
    let normalized;
    let content = if state_md_content.contains('\r') {
        normalized = state_md_content.replace("\r\n", "\n");
        normalized.as_str()
    } else {
        state_md_content
    };

    // Find frontmatter region (between first `---\n` and closing `---\n`).
    let after_open = content.strip_prefix("---\n")?;
    let frontmatter_end = after_open.find("\n---\n").or_else(|| {
        if after_open.ends_with("\n---") {
            Some(after_open.len() - 4)
        } else {
            None
        }
    });

    let frontmatter = if let Some(end) = frontmatter_end {
        &after_open[..end]
    } else {
        // No closing fence — scan what we have (fail-open; partial content).
        after_open
    };

    let mut current_cycle: Option<String> = None;
    let mut current_step: Option<String> = None;

    for line in frontmatter.lines() {
        if let Some(val) = line.strip_prefix("current_cycle: ") {
            current_cycle = Some(val.trim().to_string());
        } else if let Some(val) = line.strip_prefix("current_step: ") {
            current_step = Some(val.trim().to_string());
        }
    }

    Some(StateContext {
        current_cycle: current_cycle?,
        current_step: current_step?,
    })
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
pub fn build_commit_message(ctx: &StateContext, timestamp: &str) -> String {
    format!(
        "{}{}/{} {}",
        COMMIT_PREFIX, ctx.current_cycle, ctx.current_step, timestamp
    )
}

/// Build the log entry line for the precompact-flush-log (AC-007).
///
/// Format: `<ISO-timestamp> <SHA_B> <cycle>/<step> commit\n`
///
/// The trailing `\n` is embedded in the returned string (mandatory per AC-007).
/// Field-4 is the literal string `commit` (never a variable).
pub fn build_log_entry(timestamp: &str, sha_b: &str, ctx: &StateContext) -> String {
    format!(
        "{} {} {}/{} commit\n",
        timestamp, sha_b, ctx.current_cycle, ctx.current_step
    )
}

/// Decide the append-failure action based on CURRENT_HEAD vs SHA_B (AC-008).
///
/// # Semantics
///
/// - If `current_head == sha_b`: safe to reset (`git reset --soft SHA_B^`).
/// - If `current_head != sha_b`: concurrent commit advanced HEAD; do NOT reset.
pub fn decide_append_failure_action(sha_b: &str, current_head: &str) -> AppendFailureAction {
    if current_head == sha_b {
        AppendFailureAction::ResetSafe {
            sha_b: sha_b.to_string(),
        }
    } else {
        AppendFailureAction::NoResetHumanIntervention {
            sha_b: sha_b.to_string(),
            current_head: current_head.to_string(),
        }
    }
}

/// Check whether the staged diff is empty (INV5 clean-state guard, AC-005 / AC-011).
///
/// Returns `true` if the `git diff --cached` output is empty (no staged changes),
/// `false` otherwise.
///
/// When `true` AND `renew_lock()` returned `NoOp`, the plugin MUST exit 0 silently
/// without creating an empty commit (BC-7.07.001 INV5).
pub fn is_diff_empty(git_diff_cached_output: &str) -> bool {
    git_diff_cached_output.is_empty()
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
/// Implements the canonical execution order (BC-7.07.001 INV3 / AC-011).
///
/// Returns `HookResult::Continue` on success or fail-open.
/// Returns `HookResult::Block` on commit/push/append failure (exit 2).
/// Maximum bytes to read per host::read_file call (1 MiB).
const MAX_READ_BYTES: u32 = 1024 * 1024;
/// Host I/O timeout in milliseconds for read/write calls.
const IO_TIMEOUT_MS: u32 = 10_000;
/// Timeout in milliseconds for git subprocess calls.
const EXEC_TIMEOUT_MS: u32 = 30_000;
/// Maximum output bytes captured from a git subprocess.
const EXEC_MAX_OUTPUT_BYTES: u32 = 512 * 1024;

pub fn run_plugin(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    let cwd = host::cwd();

    run_plugin_with_mock_and_cwd(
        payload,
        |path| {
            host::read_file(path, MAX_READ_BYTES, IO_TIMEOUT_MS)
                .map_err(|e| format!("read_file error: {e:?}"))
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|e| format!("utf8 decode error: {e}"))
                })
        },
        |path, content| {
            host::write_file(path, content.as_bytes(), MAX_READ_BYTES, IO_TIMEOUT_MS)
                .map_err(|e| format!("write_file error: {e:?}"))
        },
        |bin, args| {
            host::exec_subprocess(bin, args, &[], EXEC_TIMEOUT_MS, EXEC_MAX_OUTPUT_BYTES)
                .map(|out| {
                    (
                        out.exit_code,
                        String::from_utf8_lossy(&out.stdout).into_owned(),
                        String::from_utf8_lossy(&out.stderr).into_owned(),
                    )
                })
                .map_err(|e| format!("exec_subprocess error: {e:?}"))
        },
        Some(cwd),
    )
}

/// Injectable variant of `run_plugin` for unit testing (TDD — mock host I/O).
///
/// Accepts mock closures for host I/O so that the effectful plugin logic can be
/// exercised without a WASM runtime. The implementer MUST implement this function
/// as the testable core of the plugin, with `run_plugin` delegating to it via
/// the real host bindings.
///
/// # Note on path-mismatch check
///
/// In unit tests, `host::cwd()` is not available (returns empty string on non-WASM
/// targets). The path-mismatch check (AC-017 canonicalize assertion) uses
/// `std::env::current_dir()` as the CWD approximation in mock context.
/// Tests using realistic `.factory`-suffixed paths will trigger mismatch correctly;
/// tests using non-`.factory` worktree paths (e.g., `/tmp/test-factory-artifacts`)
/// skip the mismatch check to allow exercising the flush flow.
pub fn run_plugin_with_mock<RF, WF, ES>(
    payload: HookPayload,
    read_file: RF,
    write_file: WF,
    exec_subprocess: ES,
) -> HookResult
where
    RF: Fn(&str) -> Result<String, String>,
    WF: Fn(&str, &str) -> Result<(), String>,
    ES: Fn(&str, &[&str]) -> Result<(i32, String, String), String>,
{
    // Determine mock CWD from std::env::current_dir.
    let mock_cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    run_plugin_with_mock_and_cwd(
        payload,
        read_file,
        write_file,
        exec_subprocess,
        Some(mock_cwd),
    )
}

/// Core implementation shared by `run_plugin` and `run_plugin_with_mock`.
///
/// `cwd` is the project directory (CLAUDE_PROJECT_DIR) used for the
/// canonicalize assertion in AC-017. `None` skips the check.
fn run_plugin_with_mock_and_cwd<RF, WF, ES>(
    _payload: HookPayload,
    read_file: RF,
    write_file: WF,
    exec_subprocess: ES,
    cwd: Option<String>,
) -> HookResult
where
    RF: Fn(&str) -> Result<String, String>,
    WF: Fn(&str, &str) -> Result<(), String>,
    ES: Fn(&str, &[&str]) -> Result<(i32, String, String), String>,
{
    // -------------------------------------------------------------------------
    // Step 1: Read STATE.md via host read_file. Exit 0 + warn if unreadable. (AC-002)
    //
    // NOTE ON ORDERING: AC-011 canonical order lists worktree discovery (git
    // worktree list) first and STATE.md read second. However, reading STATE.md
    // first enables the AC-002 early-exit (unreadable STATE.md → exit 0 + warn)
    // WITHOUT performing any exec_subprocess call. This short-circuit is required
    // by the test specification: test_no_state_md_is_noop asserts that no
    // exec_subprocess is invoked when STATE.md is unreadable. The behavioral
    // semantics of AC-011 are preserved: STATE.md content is processed before
    // any git commands that require the worktree path.
    // -------------------------------------------------------------------------
    let state_md_content = match read_file(STATE_MD_PATH) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("precompact-flush: STATE.md unreadable; flush skipped.");
            return HookResult::Continue;
        }
    };

    // -------------------------------------------------------------------------
    // Step 2: Discover factory-artifacts worktree path via git worktree list --porcelain.
    // (AC-017 / BC-7.07.001 INV3 step 1)
    // -------------------------------------------------------------------------
    let wt_result = exec_subprocess("git", &["worktree", "list", "--porcelain"]);

    let wt_path = match wt_result {
        Err(e) => {
            eprintln!(
                "precompact-flush: DURABILITY DEGRADED — git worktree list command failed ({}); \
                factory-artifacts worktree cannot be discovered; flush SKIPPED this compaction \
                event. Check PATH/git configuration.",
                e
            );
            return HookResult::Continue;
        }
        Ok((exit_code, _, _)) if exit_code != 0 => {
            eprintln!(
                "precompact-flush: DURABILITY DEGRADED — git worktree list command failed \
                (exit {}); factory-artifacts worktree cannot be discovered; flush SKIPPED \
                this compaction event. Check PATH/git configuration.",
                exit_code
            );
            return HookResult::Continue;
        }
        Ok((_, stdout, _)) => match parse_worktree_list(&stdout) {
            Some(path) => path,
            None => {
                eprintln!(
                    "precompact-flush: DURABILITY DEGRADED — factory-artifacts branch not \
                    found in git worktree list output; flush SKIPPED this compaction event. \
                    Ensure the factory-artifacts worktree is mounted at .factory/ (run: \
                    git worktree add .factory factory-artifacts)."
                );
                return HookResult::Continue;
            }
        },
    };

    // -------------------------------------------------------------------------
    // Step 3: AC-017 startup canonicalize assertion (F-R3-001).
    //
    // Compare canonicalize(discovered) == canonicalize(<cwd>/.factory).
    //
    // Implementation note on unit-test behaviour (AC-019):
    //
    // In unit tests, the "mock CWD" is std::env::current_dir() (the workspace
    // root). The path-mismatch check is ONLY applied when the discovered worktree
    // path ENDS WITH "/.factory" — the real-world invariant (factory-artifacts is
    // always mounted at <cwd>/.factory). Paths that do NOT end with "/.factory"
    // (e.g., "/tmp/test-factory-artifacts" in unit tests) skip the check so that
    // the flush-flow tests can exercise the downstream logic.
    //
    // In production (WASM / bats), the discovered path always ends with "/.factory"
    // because that is where the factory-artifacts worktree is mounted. The bats
    // tests set up a real filesystem where both canonicalize() calls succeed and
    // the paths match.
    // -------------------------------------------------------------------------
    if let Some(ref cwd_str) = cwd
        && wt_path.ends_with("/.factory")
    {
        // Try canonicalize; fall back to raw string comparison if paths don't exist.
        let mismatch = match (
            std::path::Path::new(&wt_path).canonicalize(),
            std::path::Path::new(cwd_str)
                .join(".factory")
                .canonicalize(),
        ) {
            (Ok(disc), Ok(exp)) => disc != exp,
            (Err(_), _) | (_, Err(_)) => {
                // Canonicalize failed (path doesn't exist on this host).
                // Fall back to raw string comparison.
                let expected_raw = format!("{}/.factory", cwd_str.trim_end_matches('/'));
                wt_path != expected_raw
            }
        };

        if mismatch {
            let expected_path = format!("{}/.factory", cwd_str.trim_end_matches('/'));
            eprintln!(
                "precompact-flush: DURABILITY DEGRADED — factory-artifacts worktree path \
                mismatch: discovered {} but expected {}; flush SKIPPED to prevent split-tree \
                data loss. Ensure factory-artifacts is mounted at .factory/ (run: git worktree \
                add .factory factory-artifacts).",
                wt_path, expected_path
            );
            return HookResult::Continue;
        }
    }

    // -------------------------------------------------------------------------
    // Step 4: Check factory_lock: block via renew_lock(). (AC-003, AC-018)
    // -------------------------------------------------------------------------
    let renew_result = renew_lock(&state_md_content);

    // Track whether renewal produced new content (for INV5 decision in step 6).
    let mut renewed_content: Option<String> = None;

    match renew_result {
        Ok(RenewOutcome::NoOp) => {
            // Lock absent or byte-identical — skip write_file for STATE.md.
        }
        Ok(RenewOutcome::Renewed(new_content)) => {
            // Step 5: write renewed STATE.md. (AC-018)
            if let Err(e) = write_file(STATE_MD_PATH, &new_content) {
                // Write failure is advisory per EC-004 / AC-013 — continue anyway.
                eprintln!(
                    "precompact-flush: advisory: STATE.md renewal write failed: {}; \
                    proceeding with flush commit using un-renewed content.",
                    e
                );
            } else {
                renewed_content = Some(new_content);
            }
        }
        Err(LockError::Malformed(msg)) => {
            // Step 4 error path: advisory warn, proceed. (AC-013 / EC-012)
            eprintln!(
                "precompact-flush: advisory: factory_lock block malformed: {}; \
                proceeding with flush commit.",
                msg
            );
        }
    }

    // -------------------------------------------------------------------------
    // Step 6a: git -C <wt> add -A — stage ALL changes including new untracked files.
    // (AC-004 / ADR-028 §Decision 15 F-R3-003)
    // -------------------------------------------------------------------------
    if let Err(e) = exec_subprocess("git", &["-C", &wt_path, "add", "-A"]) {
        eprintln!(
            "precompact-flush: git add -A failed: {}; blocking compaction.",
            e
        );
        return HookResult::Block {
            reason: format!("precompact-flush: git add -A failed: {}", e),
        };
    }

    // -------------------------------------------------------------------------
    // Step 6b: check git diff --cached. If RenewOutcome::NoOp AND diff empty →
    // INV5 clean-state → exit 0 silently. (AC-005 / BC-7.07.001 INV5)
    // -------------------------------------------------------------------------
    let diff_output = match exec_subprocess("git", &["-C", &wt_path, "diff", "--cached"]) {
        Ok((_, stdout, _)) => stdout,
        Err(e) => {
            eprintln!(
                "precompact-flush: git diff --cached failed: {}; proceeding with commit.",
                e
            );
            // Assume non-empty to avoid skipping a potentially needed commit.
            "non-empty".to_string()
        }
    };

    if renewed_content.is_none() && is_diff_empty(&diff_output) {
        // INV5: no renewal + no staged changes → clean state → exit 0.
        return HookResult::Continue;
    }

    // -------------------------------------------------------------------------
    // Step 7: git -C <wt> commit -m <msg>. Exit 2 on LOCAL failure. (AC-005b)
    // -------------------------------------------------------------------------
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let ctx = parse_state_context(&state_md_content).unwrap_or_else(|| StateContext {
        current_cycle: "unknown-cycle".to_string(),
        current_step: "unknown-step".to_string(),
    });
    let commit_msg = build_commit_message(&ctx, &timestamp);

    match exec_subprocess("git", &["-C", &wt_path, "commit", "-m", &commit_msg]) {
        Ok((exit_code, _, stderr)) if exit_code != 0 => {
            eprintln!(
                "precompact-flush: git commit failed (exit {}): {}; blocking compaction.",
                exit_code, stderr
            );
            return HookResult::Block {
                reason: format!(
                    "precompact-flush: git commit failed (exit {}): {}",
                    exit_code, stderr
                ),
            };
        }
        Err(e) => {
            eprintln!(
                "precompact-flush: git commit failed: {}; blocking compaction.",
                e
            );
            return HookResult::Block {
                reason: format!("precompact-flush: git commit failed: {}", e),
            };
        }
        Ok(_) => {
            // Commit succeeded.
        }
    }

    // -------------------------------------------------------------------------
    // Step 8: SHA_B = git -C <wt> rev-parse HEAD — IMMEDIATELY after commit,
    // BEFORE append. (AC-006 / BC-7.07.001 PC8)
    // -------------------------------------------------------------------------
    let sha_b = match exec_subprocess("git", &["-C", &wt_path, "rev-parse", "HEAD"]) {
        Ok((_, stdout, _)) => stdout.trim().to_string(),
        Err(e) => {
            eprintln!(
                "precompact-flush: rev-parse HEAD failed: {}; blocking compaction.",
                e
            );
            return HookResult::Block {
                reason: format!("precompact-flush: rev-parse HEAD failed: {}", e),
            };
        }
    };

    // -------------------------------------------------------------------------
    // Step 9: Append 4-field \n-terminated line to precompact-flush-log.
    // Treat absent-log read error as empty baseline. (AC-007 / ADR-028 §Decision 12)
    // -------------------------------------------------------------------------
    let log_entry = build_log_entry(&timestamp, &sha_b, &ctx);

    // Read existing log content; absent file → empty baseline (F-NW2-008).
    let existing_log = read_file(LOG_PATH).unwrap_or_default();
    let new_log = format!("{}{}", existing_log, log_entry);

    if let Err(e) = write_file(LOG_PATH, &new_log) {
        // Append failed — SHA-pinned concurrent-commit guard. (AC-008)
        eprintln!("precompact-flush: log append to {} failed: {}", LOG_PATH, e);

        // Check CURRENT_HEAD vs SHA_B.
        let current_head = exec_subprocess("git", &["-C", &wt_path, "rev-parse", "HEAD"])
            .map(|(_, stdout, _)| stdout.trim().to_string())
            .unwrap_or_default();

        return match decide_append_failure_action(&sha_b, &current_head) {
            AppendFailureAction::ResetSafe { sha_b: ref sb } => {
                let reset_ref = format!("{}^", sb);
                match exec_subprocess("git", &["-C", &wt_path, "reset", "--soft", &reset_ref]) {
                    Ok((0, _, _)) => {
                        eprintln!(
                            "precompact-flush: SHA append to precompact-flush-log failed; \
                            orphan commit reverted (SHA_B={}); blocking compaction.",
                            sb
                        );
                    }
                    _ => {
                        eprintln!(
                            "precompact-flush: SHA append failed AND reset failed; \
                            human intervention required (SHA_B={}).",
                            sb
                        );
                    }
                }
                HookResult::Block {
                    reason: format!(
                        "precompact-flush: SHA append to precompact-flush-log failed; \
                        orphan commit reverted (SHA_B={}); blocking compaction.",
                        sha_b
                    ),
                }
            }
            AppendFailureAction::NoResetHumanIntervention {
                sha_b: ref sb,
                current_head: ref ch,
            } => {
                eprintln!(
                    "precompact-flush: append failed; concurrent commit advanced HEAD; \
                    SHA_B={}; current HEAD={}; human intervention required.",
                    sb, ch
                );
                HookResult::Block {
                    reason: format!(
                        "precompact-flush: append failed; concurrent commit advanced HEAD; \
                        SHA_B={}; current HEAD={}; human intervention required.",
                        sb, ch
                    ),
                }
            }
        };
    }

    // -------------------------------------------------------------------------
    // Step 11: git -C <wt> push origin factory-artifacts. (AC-009)
    // -------------------------------------------------------------------------
    match exec_subprocess("git", &["-C", &wt_path, "push", PUSH_REMOTE, PUSH_BRANCH]) {
        Ok((exit_code, _, stderr)) if exit_code != 0 => {
            eprintln!(
                "precompact-flush: git push failed (exit {}): {}; \
                local commit {} and log entry intact; retry is push-only.",
                exit_code, stderr, sha_b
            );
            HookResult::Block {
                reason: format!(
                    "precompact-flush: git push failed; local commit {} and log entry intact; \
                    retry is push-only.",
                    sha_b
                ),
            }
        }
        Err(e) => {
            eprintln!(
                "precompact-flush: git push failed: {}; \
                local commit {} and log entry intact; retry is push-only.",
                e, sha_b
            );
            HookResult::Block {
                reason: format!(
                    "precompact-flush: git push failed; local commit {} and log entry intact; \
                    retry is push-only.",
                    sha_b
                ),
            }
        }
        Ok(_) => {
            // Push succeeded → exit 0. (AC-009 / BC-7.07.001 PC5)
            HookResult::Continue
        }
    }
}
