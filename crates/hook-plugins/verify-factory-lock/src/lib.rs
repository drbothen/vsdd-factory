//! verify-factory-lock — PreToolUse WASM hook plugin.
//!
//! Enforces the cross-session single-writer invariant on the `factory-artifacts`
//! orphan branch. Fires on every `PreToolUse` event for mutating tools
//! (Edit, Write, Agent dispatch, and Bash commands that push to factory-artifacts).
//!
//! On each invocation the guard:
//!   1. For Bash tool payloads: checks the internal push-regex
//!      (`git.*push.*factory-artifacts`). If no match, returns Continue immediately
//!      (sub-millisecond; no STATE.md read).
//!   2. Reads `.factory/STATE.md` via `host::read_file`.
//!   3. Parses the YAML frontmatter region (line-by-line scan between `---\n`
//!      delimiters) for the `factory_lock:` block and its three sub-fields.
//!   4. Resolves the caller's identity via `host::exec_subprocess(["git", "config",
//!      "user.email"])`.
//!   5. Compares the lock holder, expiry time, and caller identity.
//!
//! Outcomes:
//!   - Foreign unexpired lock: `HookResult::Block` with 5-field actionable message
//!     (PC1 ForeignLockHeld).
//!   - Expired lock: `HookResult::Continue` (PC2 LockExpired).
//!   - Self-held lock: `HookResult::Continue` (PC3).
//!   - Absent/null/malformed lock: `HookResult::Continue` + optional `log_warn` (PC4).
//!   - STATE.md read failure: `HookResult::Continue` + `log_warn` (PC6 fail-open).
//!   - git subprocess failure: `HookResult::Continue` + `log_warn` (PC7 fail-open).
//!
//! # Behavioral Contracts
//!
//! - BC-4.13.001: verify-factory-lock WASM PreToolUse guard.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced; ADR-025 Decision 1).
//! - No `regex` crate (Architecture Compliance Rule 4; manual line-by-line scan).
//! - No `serde_yaml` / `serde_norway` (Architecture Compliance Rule 4; fixed-format
//!   frontmatter does not warrant a full YAML parser).
//! - `async = false` is required in both registry entries (ADR-019; ADR-025 Decision 2).
//! - Guard is read-only: NEVER writes STATE.md (BC-4.13.001 Invariant 4).
//! - Pure `fn guard_logic(...)` takes all host I/O as injectable closures;
//!   unit tests exercise every branch without a WASM runtime.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.13.001 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md via `host::read_file`.
/// 64 KiB is sufficient for the STATE.md frontmatter; the `factory_lock` block
/// appears within the first 2 KiB of the file (per BC-4.13.001 Precondition 3).
pub const STATE_MD_MAX_BYTES: u32 = 65536;

/// Timeout in milliseconds for the `host::read_file` call.
pub const READ_FILE_TIMEOUT_MS: u32 = 5000;

/// Regex (literal string) for the factory-artifacts Bash push arm.
/// This is used internally by the plugin for Bash payloads — NOT a dependency
/// on the `regex` crate. The pattern is matched via a simple substring/contains
/// check using the three required fragments (Architecture Compliance Rule 4).
/// Pattern: `git.*push.*factory-artifacts`
pub const PUSH_PATTERN_GIT: &str = "git";
pub const PUSH_PATTERN_PUSH: &str = "push";
pub const PUSH_PATTERN_BRANCH: &str = "factory-artifacts";

// ---------------------------------------------------------------------------
// Error variants (BC-4.13.001 error taxonomy)
// ---------------------------------------------------------------------------

/// Internal error variants for classify + log purposes only.
/// These are NEVER surfaced to users except via `log_warn` messages.
/// Only `ForeignLockHeld` produces a `HookResult::Block`.
#[derive(Debug)]
pub enum LockCheckError {
    /// PC1: a foreign, unexpired lock is held — the only blocking error variant.
    ForeignLockHeld,
    /// PC2: lock TTL has elapsed (treat as unlocked; no warning).
    LockExpired,
    /// PC4: `factory_lock` block absent, null, or malformed.
    MalformedLockBlock(String),
    /// PC6: `host::read_file` returned a HostError.
    StateReadError(String),
    /// PC7: `host::exec_subprocess` failed, returned non-zero, or returned empty output.
    IdentityResolutionFailed(String),
}

// ---------------------------------------------------------------------------
// Parsed lock state (output of frontmatter scanner)
// ---------------------------------------------------------------------------

/// A successfully-parsed `factory_lock` block from STATE.md frontmatter.
///
/// All three fields are required; absence of any field routes to
/// `MalformedLockBlock`.
#[derive(Debug, Clone)]
pub struct LockState {
    /// Email of the current lock holder.
    pub holder: String,
    /// ISO-8601 timestamp when the lock was acquired (required for refusal message).
    pub locked_at: String,
    /// ISO-8601 datetime when the lock auto-expires.
    pub expires_at: String,
}

// ---------------------------------------------------------------------------
// Injectable callbacks surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting host calls injected into `guard_logic` for testability.
/// In production (`main.rs`), these are wired to real vsdd_hook_sdk host fns.
pub struct GuardCallbacks<R, E, L>
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    E: FnOnce(&[&str]) -> Result<(i32, String), String>,
    L: FnMut(&str),
{
    /// Read a file by path with `(path, max_bytes, timeout_ms)`.
    /// Returns `Ok(bytes)` or `Err(host_error_description)` on failure.
    pub read_file: R,
    /// Execute a subprocess with the given argv slice.
    /// Returns `Ok((exit_code, stdout))` or `Err(host_error_description)` on failure.
    pub exec_subprocess: E,
    /// Emit a `host::log_warn` message (advisory; non-blocking).
    pub log_warn: L,
}

// ---------------------------------------------------------------------------
// Pure helper fn stubs (implementer fills in T-3)
// ---------------------------------------------------------------------------

/// Check whether a Bash command payload matches the factory-artifacts push pattern.
///
/// Returns `true` if the command contains all three fragments required by the
/// internal push-regex `git.*push.*factory-artifacts`. Does NOT read STATE.md.
/// A non-matching Bash command must return `false` so `guard_logic` can
/// return `HookResult::Continue` immediately (EC-011; AC-013).
///
/// Architecture Compliance Rule 4 — no `regex` crate: uses substring matching
/// on the three required literal fragments in order.
pub fn matches_factory_artifacts_push(command: &str) -> bool {
    todo!(
        "implement: check command contains '{}', '{}', '{}' in order",
        PUSH_PATTERN_GIT,
        PUSH_PATTERN_PUSH,
        PUSH_PATTERN_BRANCH
    )
}

/// Scan the YAML frontmatter of STATE.md content for the `factory_lock:` block.
///
/// Reads only the region between the first and second `---\n` delimiters.
/// Uses a line-by-line scan (no YAML parser; no `regex` crate).
/// Sub-fields are indented with exactly 2 spaces under `factory_lock:`.
///
/// Returns:
/// - `Ok(None)` if the `factory_lock` key is absent (EC-001 unlocked path).
/// - `Ok(Some(LockState))` if all three sub-fields are present and non-empty.
/// - `Err(MalformedLockBlock)` if the block is present but malformed (EC-004,
///   EC-005, EC-012, EC-013).
pub fn parse_factory_lock(content: &str) -> Result<Option<LockState>, LockCheckError> {
    todo!("implement: scan frontmatter between --- delimiters for factory_lock block")
}

/// Parse an ISO-8601 datetime string into a `chrono::DateTime<chrono::Utc>`.
///
/// Returns `Ok(dt)` on success, `Err(MalformedLockBlock)` if unparseable
/// (EC-005).
pub fn parse_iso8601(s: &str) -> Result<chrono::DateTime<chrono::Utc>, LockCheckError> {
    todo!("implement: parse ISO-8601 string via chrono; return MalformedLockBlock on error")
}

/// Compute `expires_at - now` as a human-readable duration string.
///
/// Format: `"N min remaining"` where N is rounded down to the nearest minute.
/// Input: `expires_at` and `now` as `chrono::DateTime<chrono::Utc>`.
/// Precondition: caller guarantees `expires_at > now` (called only on the
/// ForeignLockHeld path after the TTL comparison passes).
pub fn format_time_remaining(
    expires_at: chrono::DateTime<chrono::Utc>,
    now: chrono::DateTime<chrono::Utc>,
) -> String {
    todo!("implement: compute expires_at - now, round down to minutes, return 'N min remaining'")
}

/// Build the block message for PC1 (ForeignLockHeld).
///
/// The message MUST include ALL five required fields per AC-001:
///   1. holder — exact git email of current lock holder
///   2. locked_at — ISO-8601 timestamp when lock was acquired
///   3. expires_at — ISO-8601 expiry timestamp
///   4. time_remaining — human-readable, e.g. "37 min remaining"
///   5. `/factory-unlock --force` — exact break-glass command string
pub fn build_block_message(
    holder: &str,
    locked_at: &str,
    expires_at: &str,
    time_remaining: &str,
) -> String {
    todo!(
        "implement: format block message with all 5 required fields including /factory-unlock --force"
    )
}

/// Trim trailing whitespace (including `\n`) from a git subprocess stdout line.
pub fn trim_git_email(raw: &str) -> String {
    todo!("implement: trim trailing newline and whitespace from git email output")
}

// ---------------------------------------------------------------------------
// Core guard logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core verify-factory-lock guard logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Decision tree (per BC-4.13.001):
///   1. Extract `tool` from payload. If tool is "Bash":
///      - Extract `tool_input.command`. If command does NOT match push pattern:
///        return Continue immediately (EC-011).
///   2. Read STATE.md via `read_file`. On error: log_warn + return Continue (PC6).
///   3. Parse frontmatter for `factory_lock`. On absent: return Continue (EC-001).
///      On malformed: log_warn + return Continue (PC4).
///   4. Parse `expires_at`. On parse fail: log_warn + return Continue (EC-005).
///   5. Compare `now > expires_at`. If true: return Continue (PC2 LockExpired).
///   6. Resolve caller email via `exec_subprocess`. On failure: log_warn +
///      return Continue (PC7).
///   7. Trim email output.
///   8. If `holder == caller_email`: return Continue (PC3 self-held).
///   9. Compute `time_remaining` = `expires_at - now`.
///  10. Return Block with 5-field message (PC1 ForeignLockHeld).
///
/// # BC traces
/// - BC-4.13.001 PC1: ForeignLockHeld block
/// - BC-4.13.001 PC2: LockExpired pass
/// - BC-4.13.001 PC3: self-held pass
/// - BC-4.13.001 PC4: absent/malformed fail-open
/// - BC-4.13.001 PC6: read failure fail-open
/// - BC-4.13.001 PC7: identity resolution fail-open
/// - BC-4.13.001 EC-002: `now == expires_at` treated as expired (not greater-than)
/// - BC-4.13.001 EC-011: non-push Bash → Continue immediately
pub fn guard_logic<R, E, L>(
    payload: HookPayload,
    mut callbacks: GuardCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    E: FnOnce(&[&str]) -> Result<(i32, String), String>,
    L: FnMut(&str),
{
    todo!("implement guard_logic decision tree per BC-4.13.001 T-3 specification")
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `guard_logic`.
///
/// host::exec_subprocess signature: `(cmd, args, stdin, timeout_ms, max_output_bytes)`.
/// We call `git config user.email` with no stdin. Max output is 512 bytes
/// (an email address is short; this prevents wasting WASM fuel on large output).
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    guard_logic(
        payload,
        GuardCallbacks {
            read_file: |path, max_bytes, timeout_ms| {
                match host::read_file(path, max_bytes, timeout_ms) {
                    Ok(bytes) => Ok(bytes),
                    Err(e) => Err(format!("{:?}", e)),
                }
            },
            exec_subprocess: |argv| {
                // argv is ["git", "config", "user.email"]
                // host API: exec_subprocess(cmd, args, stdin, timeout_ms, max_output_bytes)
                match argv.split_first() {
                    Some((cmd, args)) => {
                        match host::exec_subprocess(cmd, args, &[], 5000, 512) {
                            Ok(result) => {
                                let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                                Ok((result.exit_code, stdout))
                            }
                            Err(e) => Err(format!("{:?}", e)),
                        }
                    }
                    None => Err("exec_subprocess: empty argv".to_string()),
                }
            },
            log_warn: |msg| {
                host::log_warn(msg);
            },
        },
    )
}
