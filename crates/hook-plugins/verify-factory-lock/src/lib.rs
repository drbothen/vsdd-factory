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

use factory_lock_parse as flp;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.13.001 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md via `host::read_file`.
///
/// 256 KiB cap per BC-4.13.001 v1.14 Phase-A Precondition 3 (ADR-025 Decision 14).
/// Worst-case observed STATE.md is <200 KiB under 500-line compaction discipline
/// (ADR-026); 262144 gives ≥25% headroom over the observed range.
pub const STATE_MD_MAX_BYTES: u32 = 262144;

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
// Re-exported from factory_lock_parse (D15 / S-17.04 AC-004).
// ---------------------------------------------------------------------------

/// A successfully-parsed `factory_lock` block from STATE.md frontmatter.
///
/// All three fields are required; absence of any field routes to
/// `MalformedLockBlock`.
///
/// Re-exported from `factory_lock_parse` crate (D15 / S-17.04 AC-004).
pub use factory_lock_parse::LockState;

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
// Pure helper functions
// ---------------------------------------------------------------------------

/// Check whether a Bash command payload matches the factory-artifacts push pattern.
///
/// Returns `true` if the command is a `git push` targeting `factory-artifacts`.
/// Does NOT read STATE.md. A non-matching Bash command must return `false` so
/// `guard_logic` can return `HookResult::Continue` immediately (EC-011; AC-013).
///
/// Architecture Compliance Rule 4 — no `regex` crate: uses whitespace-tokenization
/// to require all three tokens (`git`, `push`, `factory-artifacts`) as distinct words
/// in order. This prevents false-matches on contrived filenames and echo strings where
/// the pattern appears inside a quoted string or as part of a longer word (O1 fix).
///
/// Matched real forms:
///   - `git push origin factory-artifacts`
///   - `git -C .factory push origin factory-artifacts`
///   - `git push --force-with-lease=factory-artifacts:<sha> factory-artifacts`
///   - `git push factory-artifacts`
///
/// The `PUSH_PATTERN_*` constants are retained for documentation purposes.
pub fn matches_factory_artifacts_push(command: &str) -> bool {
    // Tokenize the command on ASCII whitespace.
    // Architecture Compliance Rule 4 — no `regex` crate.
    let tokens: Vec<&str> = command.split_ascii_whitespace().collect();

    // Require a "git" token, a "push" token appearing after it, and a
    // "factory-artifacts" token (exact) appearing after the "push" token.
    // This matches `git -C .factory push ... factory-artifacts` (options allowed
    // between git and push, and between push and the branch name).
    let git_idx = tokens.iter().position(|&t| t == PUSH_PATTERN_GIT);
    let git_idx = match git_idx {
        Some(i) => i,
        None => return false,
    };

    let push_idx = tokens[git_idx + 1..]
        .iter()
        .position(|&t| t == PUSH_PATTERN_PUSH);
    let push_idx = match push_idx {
        Some(i) => git_idx + 1 + i,
        None => return false,
    };

    // factory-artifacts may appear as an exact token (bare branch name or remote)
    // anywhere after the "push" token.
    tokens[push_idx + 1..].contains(&PUSH_PATTERN_BRANCH)
}

/// Scan the YAML frontmatter of STATE.md content for the `factory_lock:` block.
///
/// Delegates to `factory_lock_parse::parse_factory_lock` (D15 / S-17.04 AC-004).
/// The `LockParseError::MalformedLockBlock` is bridged to `LockCheckError::MalformedLockBlock`.
///
/// Returns:
/// - `Ok(None)` if the `factory_lock` key is absent (EC-001 unlocked path).
/// - `Ok(Some(LockState))` if all three sub-fields are present and non-empty.
/// - `Err(MalformedLockBlock)` if the block is present but malformed (EC-004,
///   EC-005, EC-012, EC-013).
pub fn parse_factory_lock(content: &str) -> Result<Option<LockState>, LockCheckError> {
    flp::parse_factory_lock(content).map_err(|e| match e {
        flp::LockParseError::MalformedLockBlock(msg) => LockCheckError::MalformedLockBlock(msg),
    })
}

/// Extract the string value from a YAML key-value line like `key: "value"` or `key: value`.
///
/// Delegates to `factory_lock_parse::extract_yaml_string_value` (D15 / S-17.04 AC-004).
pub fn extract_yaml_string_value(line: &str, key: &str) -> Option<String> {
    flp::extract_yaml_string_value(line, key)
}

/// Parse an ISO-8601 datetime string into a `chrono::DateTime<chrono::Utc>`.
///
/// Delegates to `factory_lock_parse::parse_iso8601` (D15 / S-17.04 AC-004).
/// Returns `Ok(dt)` on success, `Err(MalformedLockBlock)` if unparseable (EC-005).
pub fn parse_iso8601(s: &str) -> Result<chrono::DateTime<chrono::Utc>, LockCheckError> {
    flp::parse_iso8601(s).map_err(|e| match e {
        flp::LockParseError::MalformedLockBlock(msg) => LockCheckError::MalformedLockBlock(msg),
    })
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
    let duration = expires_at.signed_duration_since(now);
    // Round down to the nearest minute (integer division truncates).
    let total_seconds = duration.num_seconds().max(0);
    let minutes = total_seconds / 60;
    format!("{} min remaining", minutes)
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
    format!(
        "BLOCKED by verify-factory-lock: factory-artifacts branch is locked by {holder}.\n\
         locked_at: {locked_at}\n\
         expires_at: {expires_at} ({time_remaining})\n\
         To break the lock: /factory-unlock --force"
    )
}

/// Check whether the lock is expired relative to the current time.
///
/// Returns `true` if `now >= expires_at` — meaning the lock has expired or
/// is expiring at this exact instant.
///
/// BC-4.13.001 EC-002: `now == expires_at` is treated as EXPIRED (returns `true`),
/// so `guard_logic` will return `HookResult::Continue` (not Block) at the exact
/// expiry boundary. The strict `>` form would block at `now == expires_at`; this
/// `>=` form correctly treats the exact-expiry instant as already-expired.
pub fn is_expired(
    now: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> bool {
    now >= expires_at
}

/// Trim trailing whitespace (including `\n`) from a git subprocess stdout line.
pub fn trim_git_email(raw: &str) -> String {
    raw.trim_end().to_string()
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
///   5. Compare `now >= expires_at`. If true: return Continue (PC2 LockExpired).
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
    // Step 1: For Bash tool, apply internal push-regex filter.
    // Non-push Bash commands return Continue immediately (EC-011; AC-013).
    if payload.tool_name == "Bash" {
        let command = payload
            .tool_input
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if !matches_factory_artifacts_push(command) {
            return HookResult::Continue;
        }
    }

    // Step 2: Read STATE.md. On HostError: log_warn + return Continue (PC6).
    let state_bytes = match (callbacks.read_file)(
        ".factory/STATE.md",
        STATE_MD_MAX_BYTES,
        READ_FILE_TIMEOUT_MS,
    ) {
        Ok(bytes) => bytes,
        Err(e) => {
            // PC6 + Invariant 6 capability-denied graceful degrade.
            let msg = if e.contains("CapabilityDenied") {
                format!("capability_denied: read_file ({})", e)
            } else {
                format!("StateReadError: {}", e)
            };
            (callbacks.log_warn)(&msg);
            return HookResult::Continue;
        }
    };

    // BC-4.13.001 Invariant 10 soft-warning: emit diagnostic when
    // bytes_read > soft_warn_threshold (200000) AND bytes_read <= STATE_MD_MAX_BYTES (262144).
    // Observability-only; never alters the Continue/Block verdict.
    let bytes_read = state_bytes.len();
    if bytes_read > 200_000 && bytes_read <= STATE_MD_MAX_BYTES as usize {
        (callbacks.log_warn)(&format!(
            "state_md_approaching_cap: bytes_read={} cap_bytes={}",
            bytes_read, STATE_MD_MAX_BYTES
        ));
    }

    // BC-4.13.001 Invariant 9 frontmatter-only mandate: extract the YAML
    // frontmatter prefix before passing bytes to the YAML parser. The guard
    // MUST NOT parse file body content.
    //
    // extract_frontmatter returns bytes[0..delimiter_start_offset] (exclusive
    // boundary per AC-005/VP-096) when a closing `\n---\n` or `\n---`-at-EOF
    // delimiter is found, or the full input when absent. Calling `.to_vec()`
    // immediately releases the borrow on state_bytes so state_bytes can be
    // moved below without borrow-checker conflict.
    let frontmatter_owned: Vec<u8> = flp::extract_frontmatter(&state_bytes).to_vec();

    // When a closing delimiter was found (frontmatter_owned is strictly shorter
    // than state_bytes), the extracted bytes omit the delimiter itself. Append
    // a synthetic `\n---\n` so parse_factory_lock can locate its boundary.
    //
    // When no delimiter was found (frontmatter_owned == state_bytes), pass the
    // original full content unchanged so parse_factory_lock returns
    // MalformedLockBlock("missing closing --- delimiter") per EC-013/PC4.
    let delimiter_found = frontmatter_owned.len() < state_bytes.len();

    let content = if delimiter_found {
        let mut parse_input = frontmatter_owned;
        parse_input.extend_from_slice(b"\n---\n");
        match String::from_utf8(parse_input) {
            Ok(s) => s,
            Err(e) => {
                (callbacks.log_warn)(&format!(
                    "StateReadError: STATE.md frontmatter is not valid UTF-8: {}",
                    e
                ));
                return HookResult::Continue;
            }
        }
    } else {
        // Delimiter absent: move state_bytes (borrow by frontmatter_owned has ended).
        match String::from_utf8(state_bytes) {
            Ok(s) => s,
            Err(e) => {
                (callbacks.log_warn)(&format!(
                    "StateReadError: STATE.md is not valid UTF-8: {}",
                    e
                ));
                return HookResult::Continue;
            }
        }
    };

    // Step 3: Parse frontmatter for factory_lock.
    // Absent: return Continue (EC-001). Malformed: log_warn + return Continue (PC4).
    let lock = match parse_factory_lock(&content) {
        Ok(None) => {
            // factory_lock block absent — factory is unlocked.
            return HookResult::Continue;
        }
        Ok(Some(l)) => l,
        Err(LockCheckError::MalformedLockBlock(detail)) => {
            (callbacks.log_warn)(&format!("MalformedLockBlock: {}", detail));
            return HookResult::Continue;
        }
        Err(e) => {
            (callbacks.log_warn)(&format!(
                "MalformedLockBlock: unexpected parse error: {:?}",
                e
            ));
            return HookResult::Continue;
        }
    };

    // Step 4: Parse expires_at as ISO-8601. On parse fail: log_warn + return Continue (EC-005).
    let expires_at_dt = match parse_iso8601(&lock.expires_at) {
        Ok(dt) => dt,
        Err(LockCheckError::MalformedLockBlock(detail)) => {
            (callbacks.log_warn)(&format!("MalformedLockBlock: {}", detail));
            return HookResult::Continue;
        }
        Err(e) => {
            (callbacks.log_warn)(&format!("MalformedLockBlock: {:?}", e));
            return HookResult::Continue;
        }
    };

    // Step 5: Compare now >= expires_at. If true (expired): return Continue (PC2 LockExpired).
    // EC-002: now == expires_at is treated as expired (is_expired returns true), so the guard
    // returns Continue at the exact-expiry boundary (not Block). Uses is_expired(now, expires_at)
    // pure helper for testability and correct `>=` semantics.
    let now = chrono::Utc::now();
    if is_expired(now, expires_at_dt) {
        // Lock has expired — treat as unlocked (PC2 LockExpired). No log_warn per BC-4.13.001 PC2.
        return HookResult::Continue;
    }

    // Step 6: Resolve caller email via exec_subprocess.
    // On failure/empty/HostError: log_warn + return Continue (PC7).
    let git_email_raw = match (callbacks.exec_subprocess)(&["git", "config", "user.email"]) {
        Err(e) => {
            let msg = if e.contains("CapabilityDenied") {
                format!("capability_denied: exec_subprocess ({})", e)
            } else {
                format!("IdentityResolutionFailed: exec_subprocess error: {}", e)
            };
            (callbacks.log_warn)(&msg);
            return HookResult::Continue;
        }
        Ok((exit_code, stdout)) => {
            if exit_code != 0 || stdout.trim().is_empty() {
                (callbacks.log_warn)(&format!(
                    "IdentityResolutionFailed: git config user.email returned exit_code={} output='{}'",
                    exit_code,
                    stdout.trim()
                ));
                return HookResult::Continue;
            }
            stdout
        }
    };

    // Step 7: Trim trailing newline from git email output.
    let caller_email = trim_git_email(&git_email_raw);

    // Step 8: If holder == caller_email: return Continue (PC3 self-held).
    if lock.holder == caller_email {
        return HookResult::Continue;
    }

    // Step 9: Compute time_remaining = expires_at - now, rounded down to nearest minute.
    let time_remaining = format_time_remaining(expires_at_dt, now);

    // Step 10: Return Block with 5-field message (PC1 ForeignLockHeld).
    let message = build_block_message(
        &lock.holder,
        &lock.locked_at,
        &lock.expires_at,
        &time_remaining,
    );
    HookResult::Block { reason: message }
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
            read_file: |path, max_bytes, timeout_ms| match host::read_file(
                path, max_bytes, timeout_ms,
            ) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(format!("{:?}", e)),
            },
            exec_subprocess: |argv| {
                // argv is ["git", "config", "user.email"]
                // host API: exec_subprocess(cmd, args, stdin, timeout_ms, max_output_bytes)
                match argv.split_first() {
                    Some((cmd, args)) => match host::exec_subprocess(cmd, args, &[], 5000, 512) {
                        Ok(result) => {
                            let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                            Ok((result.exit_code, stdout))
                        }
                        Err(e) => Err(format!("{:?}", e)),
                    },
                    None => Err("exec_subprocess: empty argv".to_string()),
                }
            },
            log_warn: |msg| {
                host::log_warn(msg);
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests — Red Gate (BC-4.13.001)
//
// All tests in this module exercise the production functions declared above via
// injectable mock closures — no WASM runtime required. Each test is named per
// the BC-based convention: test_BC_S_SS_NNN_xxx() for full traceability.
//
// All tests exercise the production functions via injectable mock closures —
// no WASM runtime required. Implementation is complete (T-3 done).
//
// Canonical STATE.md fixture content with a factory_lock block
// conforming to BC-5.40.001 PC1 (2-space indented sub-fields):
//
//   factory_lock:
//     holder: "holder@example.com"
//     locked_at: "2026-06-10T14:00:00Z"
//     expires_at: "2026-06-10T14:45:00Z"
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // Test files may use expect/unwrap/panic for failure reporting. Red-gate tests assert on
    // constants intentionally (the constant has the wrong value until the implementation lands).
    // Padded fixture helpers use repeat().take() for clarity, not performance.
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::assertions_on_constants,
        clippy::manual_repeat_n
    )]

    use super::*;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    // -----------------------------------------------------------------------
    // Test fixture builders
    // -----------------------------------------------------------------------

    /// Build a minimal HookPayload for a mutating tool (Edit, Write, or Agent).
    /// Uses serde_json deserialization so that HookPayload fields with #[serde(default)]
    /// are populated automatically — the same pattern used by validate-artifact-path tests.
    fn payload_for_tool(tool_name: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": tool_name,
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": { "file_path": ".factory/STATE.md" }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build a HookPayload for a Bash tool with the given command string.
    fn payload_for_bash(command: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Bash",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": { "command": command }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// STATE.md content with an unlocked baseline (no factory_lock key).
    fn state_md_no_lock() -> Vec<u8> {
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\n---\n\n# STATE\n"
            .to_vec()
    }

    /// STATE.md content with a foreign UNEXPIRED factory_lock block.
    /// holder is different from caller; expires_at is far in the future.
    fn state_md_foreign_unexpired_lock() -> Vec<u8> {
        // expires_at: 2099-01-01T00:00:00Z — will be expired by then but not "now".
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"other@example.com\"\n  locked_at: \"2026-06-10T14:00:00Z\"\n  expires_at: \"2099-01-01T00:00:00Z\"\n---\n\n# STATE\n"
            .to_vec()
    }

    /// STATE.md content with a foreign EXPIRED factory_lock block.
    /// expires_at is well in the past.
    fn state_md_foreign_expired_lock() -> Vec<u8> {
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"other@example.com\"\n  locked_at: \"2020-01-01T00:00:00Z\"\n  expires_at: \"2020-01-01T00:45:00Z\"\n---\n\n# STATE\n"
            .to_vec()
    }

    /// STATE.md content with a self-held UNEXPIRED factory_lock block.
    /// holder matches the mock git email "self@example.com".
    fn state_md_self_held_lock() -> Vec<u8> {
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"self@example.com\"\n  locked_at: \"2026-06-10T14:00:00Z\"\n  expires_at: \"2099-01-01T00:00:00Z\"\n---\n\n# STATE\n"
            .to_vec()
    }

    /// STATE.md content with a malformed block — holder field is empty string (EC-004).
    fn state_md_malformed_empty_holder() -> Vec<u8> {
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"\"\n  locked_at: \"2026-06-10T14:00:00Z\"\n  expires_at: \"2099-01-01T00:00:00Z\"\n---\n\n# STATE\n"
            .to_vec()
    }

    /// STATE.md content with a malformed block — expires_at not ISO-8601 (EC-005).
    /// Used by bats T-9 fixture builder; defined here for parity with other fixtures.
    #[allow(dead_code)]
    fn state_md_malformed_expires_at() -> Vec<u8> {
        b"---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"other@example.com\"\n  locked_at: \"2026-06-10T14:00:00Z\"\n  expires_at: \"not-a-timestamp\"\n---\n\n# STATE\n"
            .to_vec()
    }

    // -----------------------------------------------------------------------
    // Helper: build GuardCallbacks with FnOnce-compatible closures.
    //
    // Because R: FnOnce and E: FnOnce, we use Cell/RefCell wrapping with
    // a Box<dyn FnOnce> to allow re-use in the callback signature. Each
    // test builds fresh callbacks via the helpers below.
    // -----------------------------------------------------------------------

    /// Build callbacks where:
    ///   - read_file returns `Ok(content)` immediately (success path)
    ///   - exec_subprocess returns `Ok((0, git_email))` (success path)
    ///   - log_warn captures messages into `warn_log`
    #[allow(clippy::type_complexity)]
    fn make_callbacks_success(
        content: Vec<u8>,
        git_email: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnOnce(&[&str]) -> Result<(i32, String), String>,
        impl FnMut(&str),
    > {
        let email = git_email.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(content),
            exec_subprocess: move |_argv| Ok((0, format!("{}\n", email))),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    /// Build callbacks where read_file returns an error string.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_read_error(
        error_msg: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnOnce(&[&str]) -> Result<(i32, String), String>,
        impl FnMut(&str),
    > {
        let err = error_msg.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Err(err),
            exec_subprocess: |_argv| Ok((0, "self@example.com\n".to_string())),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    /// Build callbacks where exec_subprocess returns an error string.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_subprocess_error(
        content: Vec<u8>,
        error_msg: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnOnce(&[&str]) -> Result<(i32, String), String>,
        impl FnMut(&str),
    > {
        let err = error_msg.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(content),
            exec_subprocess: move |_argv| Err(err),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    // -----------------------------------------------------------------------
    // BC-4.13.001 unit tests
    // -----------------------------------------------------------------------

    /// PC1: Foreign unexpired lock → Block with all 5 required fields.
    ///
    /// Mock setup:
    ///   - read_file returns STATE.md with foreign holder "other@example.com",
    ///     expires_at "2099-01-01T00:00:00Z" (far future, unexpired).
    ///   - exec_subprocess returns "self@example.com" (different from holder).
    ///
    /// Expected: HookResult::Block with a reason message containing:
    ///   1. "other@example.com" (holder)
    ///   2. "2026-06-10T14:00:00Z" (locked_at)
    ///   3. "2099-01-01T00:00:00Z" (expires_at)
    ///   4. "min remaining" (time_remaining human-readable)
    ///   5. "/factory-unlock --force" (exact break-glass command)
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_foreign_unexpired_lock_blocks_with_all_five_fields() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(
            state_md_foreign_unexpired_lock(),
            "self@example.com",
            warn_log.clone(),
        );
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        // Must be a Block variant.
        match result {
            HookResult::Block { reason } => {
                // All five required fields per BC-4.13.001 PC1 / AC-001.
                assert!(
                    reason.contains("other@example.com"),
                    "Block message must contain holder email. Got: {reason}"
                );
                assert!(
                    reason.contains("2026-06-10T14:00:00Z"),
                    "Block message must contain locked_at timestamp. Got: {reason}"
                );
                assert!(
                    reason.contains("2099-01-01T00:00:00Z"),
                    "Block message must contain expires_at timestamp. Got: {reason}"
                );
                assert!(
                    reason.contains("min remaining"),
                    "Block message must contain time_remaining like 'N min remaining'. Got: {reason}"
                );
                assert!(
                    reason.contains("/factory-unlock --force"),
                    "Block message must contain '/factory-unlock --force'. Got: {reason}"
                );
            }
            other => panic!("Expected HookResult::Block, got: {:?}", other),
        }
    }

    /// PC2: Foreign holder + expired lock → Continue (LockExpired path).
    ///
    /// Mock setup:
    ///   - read_file returns STATE.md with holder "other@example.com",
    ///     expires_at "2020-01-01T00:45:00Z" (well in the past).
    ///   - exec_subprocess returns "self@example.com".
    ///
    /// Expected: HookResult::Continue. No log_warn on expired-lock path.
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_expired_lock_returns_continue() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(
            state_md_foreign_expired_lock(),
            "self@example.com",
            warn_log.clone(),
        );
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Expired lock must return Continue (PC2 LockExpired path)"
        );
        // BC-4.13.001 PC2: no log_warn for expired-lock pass-through
        let warns = warn_log.lock().unwrap();
        assert!(
            warns.is_empty(),
            "Expired lock must NOT emit log_warn (it is the normal TTL expiry path). Got: {:?}",
            warns
        );
    }

    /// PC3: Self-held lock → Continue unconditionally.
    ///
    /// Mock setup:
    ///   - read_file returns STATE.md with holder "self@example.com",
    ///     expires_at "2099-01-01T00:00:00Z" (unexpired).
    ///   - exec_subprocess returns "self@example.com" (same as holder).
    ///
    /// Expected: HookResult::Continue (developer not blocked by own lock).
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_self_held_lock_returns_continue() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(
            state_md_self_held_lock(),
            "self@example.com", // same as the holder in the fixture
            warn_log.clone(),
        );
        let payload = payload_for_tool("Write");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Self-held lock must return Continue (PC3 — developer never blocked by own lock)"
        );
    }

    /// PC4: Malformed lock block (empty holder) → Continue + log_warn("MalformedLockBlock…").
    ///
    /// Mock setup:
    ///   - read_file returns STATE.md with factory_lock.holder = "" (EC-004).
    ///   - exec_subprocess would return "self@example.com" (but should not be called).
    ///
    /// Expected: HookResult::Continue, AND log_warn captured containing "MalformedLockBlock".
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_malformed_block_returns_continue_with_log_warn() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(
            state_md_malformed_empty_holder(),
            "self@example.com",
            warn_log.clone(),
        );
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Malformed lock block must return Continue (PC4 fail-open)"
        );
        let warns = warn_log.lock().unwrap();
        assert!(
            warns.iter().any(|w| w.contains("MalformedLockBlock")),
            "Malformed lock must emit log_warn containing 'MalformedLockBlock'. Got: {:?}",
            warns
        );
    }

    /// PC6: read_file HostError → Continue + log_warn (StateReadError).
    ///
    /// Mock setup:
    ///   - read_file returns Err("OutputTooLarge") simulating a HostError variant.
    ///
    /// Expected: HookResult::Continue + log_warn containing the error description.
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_read_file_host_error_returns_continue() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_read_error("OutputTooLarge", warn_log.clone());
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "STATE.md read failure must return Continue (PC6 fail-open)"
        );
        let warns = warn_log.lock().unwrap();
        assert!(
            !warns.is_empty(),
            "read_file HostError must emit log_warn. No warns captured."
        );
    }

    /// PC7: git subprocess failure → Continue + log_warn (IdentityResolutionFailed).
    ///
    /// Mock setup:
    ///   - read_file returns a valid STATE.md with a foreign unexpired lock.
    ///   - exec_subprocess returns Err("Timeout") simulating a subprocess failure.
    ///
    /// Expected: HookResult::Continue + log_warn containing identity-resolution info.
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_git_subprocess_failure_returns_continue() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_subprocess_error(
            state_md_foreign_unexpired_lock(),
            "Timeout",
            warn_log.clone(),
        );
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "git subprocess failure must return Continue (PC7 fail-open)"
        );
        let warns = warn_log.lock().unwrap();
        assert!(
            !warns.is_empty(),
            "git subprocess failure must emit log_warn. No warns captured."
        );
    }

    /// Invariant 6: CapabilityDenied on read_file → Continue + log_warn("capability_denied: ...").
    ///
    /// The error string "CapabilityDenied" simulates what the host returns when the
    /// [hooks.capabilities.read_file] block is omitted from the registry (EC-007).
    ///
    /// Expected: HookResult::Continue + log_warn containing "capability_denied:".
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_capability_denied_graceful_degrades_to_continue() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        // The CapabilityDenied variant is surfaced as the Err string from the host.
        let callbacks = make_callbacks_read_error("CapabilityDenied", warn_log.clone());
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "CapabilityDenied on read_file must graceful-degrade to Continue (BC-4.13.001 Invariant 6)"
        );
        let warns = warn_log.lock().unwrap();
        assert!(
            warns.iter().any(|w| w.contains("capability_denied")),
            "CapabilityDenied must emit log_warn containing 'capability_denied:'. Got: {:?}",
            warns
        );
    }

    /// T-6 (D9): Bash push factory-artifacts + foreign unexpired lock → Block.
    ///
    /// Mock setup:
    ///   - payload.tool_name = "Bash"; tool_input.command = "git push origin factory-artifacts"
    ///   - read_file returns a foreign unexpired lock.
    ///   - exec_subprocess returns "self@example.com".
    ///
    /// Expected: HookResult::Block (push arm intercepted by internal push-regex).
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_bash_factory_artifacts_push_blocked_when_foreign_lock() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(
            state_md_foreign_unexpired_lock(),
            "self@example.com",
            warn_log.clone(),
        );
        let payload = payload_for_bash("git push origin factory-artifacts");

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { .. } => {
                // Correct: Bash push to factory-artifacts with foreign lock must block.
            }
            other => panic!(
                "Expected HookResult::Block for factory-artifacts push + foreign lock, got: {:?}",
                other
            ),
        }
    }

    /// T-7 (D9) + EC-011: Non-push Bash command → Continue immediately WITHOUT reading STATE.md.
    ///
    /// The guard must return Continue immediately for non-push Bash without calling
    /// read_file at all (sub-millisecond path per BC-4.13.001 EC-011 + AC-013).
    ///
    /// Test verifies via a call-counting mock on read_file: if read_file is called,
    /// the test fails (assert read_file_call_count == 0).
    ///
    /// GREEN: guard_logic implemented; test exercises this BC path.
    #[test]
    fn test_BC_4_13_001_non_push_bash_returns_continue_immediately() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let read_call_count = Arc::new(Mutex::new(0u32));
        let read_count_clone = read_call_count.clone();
        let wl = warn_log.clone();

        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| {
                *read_count_clone.lock().unwrap() += 1;
                // If this closure is called, the test will detect it via the counter.
                Ok(state_md_foreign_unexpired_lock())
            },
            exec_subprocess: |_argv| Ok((0, "self@example.com\n".to_string())),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        };
        let payload = payload_for_bash("cat .factory/STATE.md");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Non-push Bash command must return Continue (EC-011)"
        );
        let calls = *read_call_count.lock().unwrap();
        assert_eq!(
            calls, 0,
            "Non-push Bash command must NOT call read_file (sub-millisecond short-circuit). read_file was called {} time(s).",
            calls
        );
    }

    // -----------------------------------------------------------------------
    // Pure helper tests — each pure fn gets at least one focused test.
    // Each pure helper is exercised by at least one focused test.
    // -----------------------------------------------------------------------

    /// matches_factory_artifacts_push: push command → true.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_push_regex_matches_factory_artifacts_push() {
        assert!(
            matches_factory_artifacts_push("git push origin factory-artifacts"),
            "push command must match factory-artifacts push pattern"
        );
    }

    /// matches_factory_artifacts_push: non-push command → false.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_push_regex_does_not_match_non_push_command() {
        assert!(
            !matches_factory_artifacts_push("cat .factory/STATE.md"),
            "non-push command must NOT match factory-artifacts push pattern"
        );
        assert!(
            !matches_factory_artifacts_push("git status"),
            "git status must NOT match factory-artifacts push pattern"
        );
        assert!(
            !matches_factory_artifacts_push("ls"),
            "ls must NOT match factory-artifacts push pattern"
        );
    }

    /// parse_factory_lock: valid block present → Ok(Some(LockState)).
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_parse_factory_lock_returns_some_on_valid_block() {
        let raw = state_md_foreign_unexpired_lock();
        let content = std::str::from_utf8(&raw).expect("fixture is valid UTF-8");
        let result = parse_factory_lock(content);
        let lock = result
            .expect("parse must succeed on valid content")
            .expect("lock block must be present");
        assert_eq!(lock.holder, "other@example.com");
        assert_eq!(lock.locked_at, "2026-06-10T14:00:00Z");
        assert_eq!(lock.expires_at, "2099-01-01T00:00:00Z");
    }

    /// parse_factory_lock: no factory_lock block → Ok(None).
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_parse_factory_lock_returns_none_on_absent_block() {
        let raw = state_md_no_lock();
        let content = std::str::from_utf8(&raw).expect("fixture is valid UTF-8");
        let result = parse_factory_lock(content)
            .expect("parse must not error on valid content with no lock");
        assert!(
            result.is_none(),
            "Absent factory_lock block must return Ok(None)"
        );
    }

    /// parse_factory_lock: malformed block (empty holder) → Err(MalformedLockBlock).
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_parse_factory_lock_errors_on_empty_holder() {
        let raw = state_md_malformed_empty_holder();
        let content = std::str::from_utf8(&raw).expect("fixture is valid UTF-8");
        let result = parse_factory_lock(content);
        match result {
            Err(LockCheckError::MalformedLockBlock(_)) => {
                // Correct: empty holder must cause MalformedLockBlock.
            }
            Ok(Some(lock)) => panic!(
                "Expected MalformedLockBlock for empty holder, but got Ok(Some(lock)) with holder: '{}'",
                lock.holder
            ),
            Ok(None) => panic!("Expected MalformedLockBlock for empty holder, but got Ok(None)"),
            Err(other) => panic!(
                "Expected MalformedLockBlock for empty holder, but got: {:?}",
                other
            ),
        }
    }

    /// parse_iso8601: valid ISO-8601 UTC string → Ok(DateTime).
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_parse_iso8601_succeeds_on_valid_timestamp() {
        let result = parse_iso8601("2026-06-10T14:00:00Z");
        assert!(
            result.is_ok(),
            "Valid ISO-8601 string must parse successfully"
        );
    }

    /// parse_iso8601: invalid string → Err(MalformedLockBlock).
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_parse_iso8601_errors_on_invalid_timestamp() {
        let result = parse_iso8601("not-a-timestamp");
        match result {
            Err(LockCheckError::MalformedLockBlock(_)) => {
                // Correct.
            }
            Ok(_) => panic!("Invalid timestamp must NOT parse successfully"),
            Err(other) => panic!(
                "Expected MalformedLockBlock for invalid timestamp, got: {:?}",
                other
            ),
        }
    }

    /// format_time_remaining: correct "N min remaining" format.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_format_time_remaining_returns_n_min_remaining() {
        use chrono::{TimeZone, Utc};
        let now = Utc.with_ymd_and_hms(2026, 6, 10, 14, 0, 0).unwrap();
        // expires_at = now + 37 minutes + 30 seconds → rounded down = 37 min.
        let expires_at = Utc.with_ymd_and_hms(2026, 6, 10, 14, 37, 30).unwrap();
        let result = format_time_remaining(expires_at, now);
        assert_eq!(
            result, "37 min remaining",
            "format_time_remaining must round down to nearest minute"
        );
    }

    // -----------------------------------------------------------------------
    // F-S1702-002: Real boundary test via injectable `is_expired` pure helper.
    //
    // Tests the `is_expired(now, expires_at) -> bool` pure helper with correct
    // `>=` semantics: `now >= expires_at` returns true (expired).
    // -----------------------------------------------------------------------

    /// EC-002: `is_expired` pure helper — now == expires_at → true (just-expired → Continue).
    ///
    /// BC-4.13.001 EC-002: the lock is expired (and the guard must return Continue) when
    /// `now >= expires_at`. The `==` case is treated as expired (not blocked), so
    /// `is_expired` returns true when `now == expires_at`.
    ///
    /// Three cases asserted:
    ///   1. now == expires_at → true (exact boundary, just-expired → Continue per EC-002).
    ///   2. now 1 second BEFORE expires_at → false (not expired → would Block if foreign).
    ///   3. now 1 second AFTER expires_at → true (expired → Continue).
    #[test]
    #[allow(non_snake_case)]
    fn test_BC_4_13_001_is_expired_now_equals_expires_at_is_expired() {
        use chrono::{TimeZone, Utc};
        let expires_at = Utc.with_ymd_and_hms(2026, 6, 10, 15, 0, 0).unwrap();

        // Case 1: now == expires_at → expired (EC-002: exact boundary treated as expired).
        let now_equal = expires_at;
        assert!(
            is_expired(now_equal, expires_at),
            "EC-002: is_expired(now == expires_at) must return true (lock expired at exact boundary → Continue)"
        );

        // Case 2: now 1 second BEFORE expires_at → NOT expired (lock is still active).
        let now_before = expires_at - chrono::Duration::seconds(1);
        assert!(
            !is_expired(now_before, expires_at),
            "is_expired(now 1s before expires_at) must return false (lock still active → would Block)"
        );

        // Case 3: now 1 second AFTER expires_at → expired.
        let now_after = expires_at + chrono::Duration::seconds(1);
        assert!(
            is_expired(now_after, expires_at),
            "is_expired(now 1s after expires_at) must return true (lock expired)"
        );
    }

    // -----------------------------------------------------------------------
    // LOW sweeps (production-grade — RED tests for current parse_factory_lock bugs)
    // -----------------------------------------------------------------------

    /// CRLF STATE.md with a foreign unexpired lock → guard_logic must return Block.
    ///
    /// BC-4.13.001 compliance requires handling Windows-style CRLF line endings in
    /// STATE.md (e.g., files edited on Windows or by certain editors). A CRLF file
    /// has `\r\n` line endings throughout.
    ///
    /// Current bug: `parse_factory_lock` uses `strip_prefix("---\n")` which fails on
    /// `"---\r\n"` — the frontmatter opening delimiter is not recognised, so the
    /// function returns Ok(None) (no lock found) → guard returns Continue instead of Block.
    ///
    /// This test RED because: with a CRLF file holding a foreign unexpired lock and a
    /// foreign git email, `guard_logic` currently returns Continue (parse failure treats
    /// content as unlocked). The correct behavior per BC-4.13.001 is Block.
    ///
    /// Implementer fix: `parse_factory_lock` must normalise `\r\n` → `\n` before scanning,
    /// or use a delimiter that tolerates CRLF.
    ///
    /// GREEN: CRLF normalisation implemented; guard now returns Block as required.
    #[test]
    #[allow(non_snake_case)]
    fn test_BC_4_13_001_crlf_state_md_foreign_lock_blocks() {
        // Build a CRLF STATE.md with a foreign unexpired lock. Every line ends with \r\n.
        let crlf_content = concat!(
            "---\r\n",
            "document_type: state\r\n",
            "version: \"0.0.1-test\"\r\n",
            "phase: test\r\n",
            "factory_lock:\r\n",
            "  holder: \"other@example.com\"\r\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\r\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\r\n",
            "---\r\n",
            "\r\n",
            "# STATE\r\n",
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let content_bytes = crlf_content.as_bytes().to_vec();
        // Foreign git email — different from the lock holder.
        let email = "self@example.com";
        let wl = warn_log.clone();
        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(content_bytes),
            exec_subprocess: move |_argv| Ok((0, format!("{}\n", email))),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        };
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        // Must block: CRLF file with foreign unexpired lock must be parsed correctly
        // and the guard must return Block (not Continue due to a parse failure).
        match result {
            HookResult::Block { .. } => {
                // Correct — CRLF frontmatter parsed, foreign lock detected, Block returned.
            }
            HookResult::Continue => panic!(
                "CRLF STATE.md with foreign unexpired lock must return Block, not Continue. \
                 Current bug: parse_factory_lock strips '---\\n' prefix but CRLF files start with '---\\r\\n'. \
                 Implementer must normalise CRLF before parsing."
            ),
            other => panic!(
                "Expected HookResult::Block for CRLF STATE.md with foreign lock, got: {:?}",
                other
            ),
        }
    }

    /// Missing closing `---` delimiter + body-resident factory_lock block → Continue (fail-open).
    ///
    /// BC-4.13.001 EC-013: STATE.md with an opening `---` frontmatter delimiter but no
    /// closing `---` is malformed. The factory_lock block that appears indented in the
    /// body (2-space indent) must NOT cause a Block — the guard must return Continue
    /// (MalformedLockBlock fail-open per PC4).
    ///
    /// Current bug: when the closing delimiter is absent, `parse_factory_lock` falls through
    /// and scans the entire content after the opening delimiter as the "frontmatter region".
    /// If a `factory_lock:` block appears in that region (as body content), the parser may
    /// incorrectly find it and return Ok(Some(LockState)) → guard proceeds to the Block path
    /// (over-blocking on a malformed file).
    ///
    /// The correct behavior: absent closing delimiter + body-resident factory_lock block
    /// → treat as MalformedLockBlock → log_warn + return Continue.
    ///
    /// GREEN: missing-closing-delimiter detection implemented; guard returns Continue (fail-open).
    #[test]
    #[allow(non_snake_case)]
    fn test_BC_4_13_001_missing_closing_delimiter_returns_continue() {
        // STATE.md with opening `---` but no closing `---`.
        // A factory_lock block appears in the body (2-space indented sub-fields).
        let malformed_content = concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "\n",
            "# Body content (no closing --- delimiter)\n",
            "\n",
            "factory_lock:\n",
            "  holder: \"other@example.com\"\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let content_bytes = malformed_content.as_bytes().to_vec();
        let email = "self@example.com";
        let wl = warn_log.clone();
        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(content_bytes),
            exec_subprocess: move |_argv| Ok((0, format!("{}\n", email))),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        };
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        // Must be Continue (MalformedLockBlock fail-open per PC4 / EC-013).
        // A body-resident factory_lock block in a file without a closing delimiter must NOT
        // cause a Block — the file structure is malformed and the guard must fail-open.
        assert_eq!(
            result,
            HookResult::Continue,
            "Missing closing --- delimiter: body-resident factory_lock block must return Continue \
             (MalformedLockBlock fail-open, EC-013 / PC4). \
             Current bug: the scanner falls through and may parse body content as frontmatter, \
             producing an over-block. Implementer must detect absent closing delimiter and return \
             Err(MalformedLockBlock) rather than scanning the whole file body."
        );
        // Additionally verify a log_warn was emitted (PC4 requires log_warn for MalformedLockBlock).
        let warns = warn_log.lock().unwrap();
        assert!(
            warns.iter().any(|w| w.contains("MalformedLockBlock")),
            "Missing closing delimiter must emit log_warn containing 'MalformedLockBlock'. Got: {:?}",
            warns
        );
    }

    /// trim_git_email: trailing newline stripped.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_trim_git_email_strips_trailing_newline() {
        let result = trim_git_email("dev@example.com\n");
        assert_eq!(result, "dev@example.com");
    }

    /// trim_git_email: no trailing newline unchanged.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_trim_git_email_unchanged_when_no_newline() {
        let result = trim_git_email("dev@example.com");
        assert_eq!(result, "dev@example.com");
    }

    // -----------------------------------------------------------------------
    // S-19.02 Red Gate tests (T-001, T-002, T-003, T-009)
    //
    // These tests FAIL with the current stub/unimplemented state and will pass
    // only after the implementation tasks for S-19.02 are complete.
    //
    // T-001: Asserts STATE_MD_MAX_BYTES == 262144 (AC-001).
    //   RED because: current value is 65536.
    //
    // T-002: 70 KiB fixture + foreign lock → Block (AC-002).
    //   This test verifies guard_logic handles a 70 KiB mock read correctly.
    //   The mock bypasses the host cap; the test also asserts the constant is
    //   at least 70000 (which fails now, ensuring Red Gate).
    //
    // T-003: 70 KiB fixture + no lock → Continue (AC-002).
    //   Same cap assertion makes this a Red Gate.
    //
    // T-009: Soft-warning tests A–E (AC-006, BC-4.13.001 Invariant 10).
    //   RED because: guard_logic does not yet emit state_md_approaching_cap.
    // -----------------------------------------------------------------------

    /// T-001 (AC-001): STATE_MD_MAX_BYTES == 262144 (256 KiB).
    ///
    /// BC-4.13.001 v1.14 Phase-A Precondition 3: the plugin-side compile-time
    /// cap MUST be 262144. ADR-025 Decision 14.
    ///
    /// RED: current value is 65536; assertion fails until Task 9.
    #[test]
    fn test_S1902_T001_state_md_max_bytes_is_262144() {
        assert_eq!(
            STATE_MD_MAX_BYTES, 262144u32,
            "AC-001: STATE_MD_MAX_BYTES must equal 262144 (256 KiB) per \
             BC-4.13.001 v1.14 Phase-A Precondition 3 / ADR-025 Decision 14. \
             Current value: {}",
            STATE_MD_MAX_BYTES
        );
    }

    /// Build a STATE.md fixture padded to `target_size` bytes.
    ///
    /// The frontmatter contains a foreign unexpired factory_lock block. The
    /// body is padded with comment lines to reach the target byte count.
    fn state_md_padded_with_foreign_lock(target_size: usize) -> Vec<u8> {
        let header = concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "factory_lock:\n",
            "  holder: \"other@example.com\"\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
            "---\n",
            "\n",
            "# STATE\n",
        );
        let mut bytes = header.as_bytes().to_vec();
        // Pad with `# padding\n` lines until we reach the target size.
        let pad_line = b"# padding\n";
        while bytes.len() < target_size {
            let remaining = target_size - bytes.len();
            if remaining >= pad_line.len() {
                bytes.extend_from_slice(pad_line);
            } else {
                bytes.extend(std::iter::repeat(b'#').take(remaining));
            }
        }
        bytes.truncate(target_size);
        bytes
    }

    /// Build a STATE.md fixture padded to `target_size` bytes with NO lock.
    fn state_md_padded_no_lock(target_size: usize) -> Vec<u8> {
        let header = concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "---\n",
            "\n",
            "# STATE\n",
        );
        let mut bytes = header.as_bytes().to_vec();
        let pad_line = b"# padding\n";
        while bytes.len() < target_size {
            let remaining = target_size - bytes.len();
            if remaining >= pad_line.len() {
                bytes.extend_from_slice(pad_line);
            } else {
                bytes.extend(std::iter::repeat(b'#').take(remaining));
            }
        }
        bytes.truncate(target_size);
        bytes
    }

    /// T-002 (AC-002): 70 KiB fixture with foreign unexpired lock → Block.
    ///
    /// AC-002: Plugin reads STATE.md successfully when the file is between 64 KiB
    /// and 256 KiB and the factory_lock: block is present; correctly detects a
    /// foreign unexpired lock and returns block intent.
    ///
    /// The test also asserts STATE_MD_MAX_BYTES >= 70000 as a compile-time Red
    /// Gate for the cap-raise requirement.
    ///
    /// RED: STATE_MD_MAX_BYTES < 70000 currently (65536); cap assertion fails
    /// until Task 9 raises it to 262144.
    #[test]
    fn test_S1902_T002_70kib_fixture_foreign_lock_returns_block() {
        // Pre-condition: cap must be at least 70 KiB for this test to be valid.
        // This assertion is the Red Gate: fails until STATE_MD_MAX_BYTES = 262144.
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "AC-002: STATE_MD_MAX_BYTES ({}) must be >= 70000 for this test to exercise \
             the raised-cap behavior. Raise STATE_MD_MAX_BYTES to 262144 (Task 9).",
            STATE_MD_MAX_BYTES
        );

        let fixture = state_md_padded_with_foreign_lock(70_000);
        assert_eq!(fixture.len(), 70_000, "fixture must be exactly 70000 bytes");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log);
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { .. } => {
                // Correct: 70 KiB fixture with foreign unexpired lock must Block.
            }
            other => panic!(
                "AC-002: 70 KiB fixture with foreign lock must return Block. Got: {:?}",
                other
            ),
        }
    }

    /// T-003 (AC-002): 70 KiB fixture with no lock → Continue.
    ///
    /// AC-002 complement: when a > 64 KiB STATE.md has no lock, the guard
    /// must return Continue (factory is unlocked).
    ///
    /// RED: STATE_MD_MAX_BYTES < 70000 currently; cap assertion fails until
    /// Task 9 raises it to 262144.
    #[test]
    fn test_S1902_T003_70kib_fixture_no_lock_returns_continue() {
        // Pre-condition Red Gate: same cap assertion as T-002.
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "AC-002: STATE_MD_MAX_BYTES ({}) must be >= 70000. Raise to 262144 (Task 9).",
            STATE_MD_MAX_BYTES
        );

        let fixture = state_md_padded_no_lock(70_000);
        assert_eq!(fixture.len(), 70_000, "fixture must be exactly 70000 bytes");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log);
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "AC-002: 70 KiB fixture with no lock must return Continue (unlocked path)"
        );
    }

    /// T-009 (AC-006): Soft-warning emission tests A–E.
    ///
    /// BC-4.13.001 v1.14 Invariant 10: guard must emit a `plugin.log` warn entry
    /// containing `state_md_approaching_cap` when bytes_read is strictly > 200000
    /// and at or below 262144.
    ///
    /// Five sub-tests:
    ///   A: 210000 bytes → state_md_approaching_cap warn emitted.
    ///   B: 150000 bytes → zero state_md_approaching_cap log entries.
    ///   C: 200000 bytes exactly → zero state_md_approaching_cap log entries (strict > threshold).
    ///   D: 262144 bytes exactly → warn AND read succeeds (cap-exact; inclusive upper bound).
    ///   E: 262145 bytes → StateReadError (OUTPUT_TOO_LARGE/fail-open) AND zero warn.
    ///
    /// RED: guard_logic does not yet emit state_md_approaching_cap; all sub-tests
    /// that assert warn presence will fail until Task 13.
    #[test]
    fn test_S1902_T009_state_md_approaching_cap_warn_logic() {
        // Pre-condition: constants must be set correctly for these tests.
        // This assertion is also a Red Gate for the cap raise (Task 9).
        assert_eq!(
            STATE_MD_MAX_BYTES, 262144u32,
            "T-009 requires STATE_MD_MAX_BYTES == 262144 (Task 9 must complete first)"
        );

        // ---- Sub-test A: 210000 bytes → warn emitted ----
        {
            let fixture = state_md_padded_no_lock(210_000);
            assert_eq!(fixture.len(), 210_000);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log.clone());
            let payload = payload_for_tool("Edit");
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            assert!(
                warns.iter().any(|w| w.contains("state_md_approaching_cap")),
                "T-009 A: 210000-byte fixture must emit state_md_approaching_cap warn. Got: {:?}",
                warns
            );
        }

        // ---- Sub-test B: 150000 bytes → NO warn ----
        {
            let fixture = state_md_padded_no_lock(150_000);
            assert_eq!(fixture.len(), 150_000);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log.clone());
            let payload = payload_for_tool("Edit");
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            let approaching_warns: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching_warns.is_empty(),
                "T-009 B: 150000-byte fixture must NOT emit state_md_approaching_cap warn. Got: {:?}",
                approaching_warns
            );
        }

        // ---- Sub-test C: 200000 bytes exactly → NO warn (strict > threshold) ----
        {
            let fixture = state_md_padded_no_lock(200_000);
            assert_eq!(fixture.len(), 200_000);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log.clone());
            let payload = payload_for_tool("Edit");
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            let approaching_warns: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching_warns.is_empty(),
                "T-009 C: 200000-byte fixture (exact threshold) must NOT emit warn \
                 (threshold is strictly > 200000). Got: {:?}",
                approaching_warns
            );
        }

        // ---- Sub-test D: 262144 bytes exactly → warn AND read succeeds ----
        {
            let fixture = state_md_padded_no_lock(262_144);
            assert_eq!(fixture.len(), 262_144);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_success(fixture, "self@example.com", warn_log.clone());
            let payload = payload_for_tool("Edit");
            let result = guard_logic(payload, callbacks);
            // Read must succeed (not StateReadError).
            let read_errored = match &result {
                HookResult::Continue => false,
                HookResult::Block { .. } => false,
                // If we had a StateReadError variant surfaced, detect via warn.
                _ => false,
            };
            let warns = warn_log.lock().unwrap();
            let has_read_error_warn = warns.iter().any(|w| w.contains("StateReadError"));
            assert!(
                !has_read_error_warn && !read_errored,
                "T-009 D: 262144-byte fixture must NOT return StateReadError (read succeeds at cap). \
                 Warns: {:?}",
                warns
            );
            assert!(
                warns.iter().any(|w| w.contains("state_md_approaching_cap")),
                "T-009 D: 262144-byte fixture must emit state_md_approaching_cap warn (inclusive upper bound). \
                 Got: {:?}",
                warns
            );
        }

        // ---- Sub-test E: 262145 bytes → StateReadError + zero warn ----
        {
            let fixture_len = 262_145usize;
            // Simulate OUTPUT_TOO_LARGE: mock read_file returns Err for oversized files.
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let wl = warn_log.clone();
            let callbacks = GuardCallbacks {
                read_file: move |_path, max_bytes, _timeout| {
                    if fixture_len as u32 > max_bytes {
                        Err("OutputTooLarge".to_string())
                    } else {
                        Ok(state_md_padded_no_lock(fixture_len))
                    }
                },
                exec_subprocess: |_argv| Ok((0, "self@example.com\n".to_string())),
                log_warn: move |msg: &str| {
                    wl.lock().unwrap().push(msg.to_string());
                },
            };
            let payload = payload_for_tool("Edit");
            let result = guard_logic(payload, callbacks);
            // Must return Continue (fail-open on OutputTooLarge per PC6).
            assert_eq!(
                result,
                HookResult::Continue,
                "T-009 E: 262145-byte fixture must return Continue (fail-open per PC6 / EC-002)"
            );
            let warns = warn_log.lock().unwrap();
            // Must emit StateReadError warn (fail-open log).
            assert!(
                warns
                    .iter()
                    .any(|w| w.contains("StateReadError") || w.contains("OutputTooLarge")),
                "T-009 E: 262145-byte fixture must emit StateReadError/OutputTooLarge warn. Got: {:?}",
                warns
            );
            // Must NOT emit state_md_approaching_cap (file exceeded cap; warn path not reached).
            let approaching_warns: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching_warns.is_empty(),
                "T-009 E: 262145-byte fixture must NOT emit state_md_approaching_cap \
                 (exceeds cap; warn path never reached). Got: {:?}",
                approaching_warns
            );
        }
    }

    // -----------------------------------------------------------------------
    // F-S1902-P1-001: CRLF wiring test (pass-1 adversary finding)
    //
    // BC-4.13.001 v1.14→v1.15 amendment (human approved): extract_frontmatter
    // must recognize `\r\n---\r\n` CRLF delimiter. The wiring test verifies
    // Invariant 9 via a CRLF STATE.md whose body contains non-UTF-8 bytes.
    //
    // With LF-only extract_frontmatter (current):
    //   - extract_frontmatter returns full content (no CRLF delimiter matched)
    //   - delimiter_found = false → full bytes passed to String::from_utf8
    //   - Non-UTF-8 body bytes cause String::from_utf8 to fail
    //   - guard returns Continue + StateReadError log_warn (fail-open, Invariant 9 violated)
    //
    // With CRLF-aware extract_frontmatter (after implementation):
    //   - extract_frontmatter finds \r\n---\r\n → returns frontmatter-only (UTF-8 valid)
    //   - delimiter_found = true → frontmatter + "\n---\n" passed to parse_factory_lock
    //   - Foreign lock detected → guard returns Block
    // -----------------------------------------------------------------------

    /// F-S1902-P1-001 / T-012: Wiring test — CRLF STATE.md with non-UTF-8 body bytes.
    ///
    /// Verifies that guard_logic takes the frontmatter-only path (Invariant 9) for
    /// CRLF-delimited STATE.md inputs by constructing a fixture where:
    ///   - The CRLF frontmatter has a valid foreign unexpired factory_lock block.
    ///   - The body contains `\xFF\xFE` (invalid UTF-8) bytes.
    ///
    /// Correct behavior (after fix): Block — frontmatter-only bytes are valid UTF-8;
    ///   foreign lock is detected and the guard blocks.
    ///
    /// Current behavior (RED): Continue + StateReadError warn — the LF-only
    ///   extract_frontmatter returns the full content including non-UTF-8 body bytes;
    ///   String::from_utf8 fails on the non-UTF-8 bytes; guard takes the fail-open path.
    #[test]
    fn test_S1902_crlf_wiring_non_utf8_body_blocks_on_foreign_lock() {
        // CRLF frontmatter with foreign unexpired lock — all UTF-8 valid bytes.
        let mut crlf_bytes: Vec<u8> = b"---\r\n\
            document_type: state\r\n\
            version: \"0.0.1-test\"\r\n\
            phase: test\r\n\
            factory_lock:\r\n\
            \x20\x20holder: \"other@example.com\"\r\n\
            \x20\x20locked_at: \"2026-06-10T14:00:00Z\"\r\n\
            \x20\x20expires_at: \"2099-01-01T00:00:00Z\"\r\n\
            ---\r\n"
            .to_vec();
        // Append body with non-UTF-8 bytes (\xFF\xFE = invalid UTF-8 start sequence).
        // The body follows the CRLF closing delimiter.
        crlf_bytes.extend_from_slice(b"\r\n# Body section\r\n\xFF\xFE padding body\r\n");

        let warn_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let wl = warn_log.clone();
        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(crlf_bytes),
            exec_subprocess: move |_argv| Ok((0, "self@example.com\n".to_string())),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        };
        let payload = payload_for_tool("Edit");

        let result = guard_logic(payload, callbacks);

        // Must be Block: CRLF frontmatter has a valid foreign unexpired lock.
        // Correct path (after fix): extract_frontmatter recognizes \r\n---\r\n →
        //   frontmatter-only bytes (UTF-8 valid) → parse succeeds → Block.
        match result {
            HookResult::Block { .. } => {
                // Correct: frontmatter-only path taken; non-UTF-8 body bytes did not interfere.
            }
            HookResult::Continue => {
                let warns = warn_log.lock().unwrap();
                panic!(
                    "F-S1902-P1-001: CRLF STATE.md with foreign unexpired lock must return Block. \
                     Got Continue. Warns: {:?}. \
                     Likely cause: extract_frontmatter (LF-only) returned full content including \
                     non-UTF-8 body bytes; String::from_utf8 failed → fail-open StateReadError path. \
                     Fix: update extract_frontmatter to recognize \\r\\n---\\r\\n \
                     per BC-4.13.001 v1.15.",
                    warns
                );
            }
            other => panic!(
                "F-S1902-P1-001: expected HookResult::Block for CRLF STATE.md with foreign lock, \
                 got: {:?}",
                other
            ),
        }
    }

    /// build_block_message: all 5 fields present in the message.
    ///
    /// GREEN: pure helper implemented; test verifies this case.
    #[test]
    fn test_BC_4_13_001_build_block_message_contains_all_five_fields() {
        let msg = build_block_message(
            "other@example.com",
            "2026-06-10T14:00:00Z",
            "2099-01-01T00:00:00Z",
            "37 min remaining",
        );
        assert!(
            msg.contains("other@example.com"),
            "Block message must contain holder. Got: {msg}"
        );
        assert!(
            msg.contains("2026-06-10T14:00:00Z"),
            "Block message must contain locked_at. Got: {msg}"
        );
        assert!(
            msg.contains("2099-01-01T00:00:00Z"),
            "Block message must contain expires_at. Got: {msg}"
        );
        assert!(
            msg.contains("37 min remaining"),
            "Block message must contain time_remaining. Got: {msg}"
        );
        assert!(
            msg.contains("/factory-unlock --force"),
            "Block message must contain '/factory-unlock --force'. Got: {msg}"
        );
    }
}
