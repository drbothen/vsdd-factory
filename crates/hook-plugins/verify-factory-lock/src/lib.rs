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
    // Check that "git", "push", "factory-artifacts" all appear in the command
    // in left-to-right order (no regex crate; Architecture Compliance Rule 4).
    if let Some(git_pos) = command.find(PUSH_PATTERN_GIT) {
        let after_git = &command[git_pos..];
        if let Some(push_pos) = after_git.find(PUSH_PATTERN_PUSH) {
            return after_git[push_pos..].contains(PUSH_PATTERN_BRANCH);
        }
    }
    false
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
    // Normalise Windows-style CRLF line endings to LF before scanning.
    // This ensures STATE.md files edited on Windows or by certain editors (which
    // emit `\r\n`) are parsed identically to LF-only files (O2 fix: CRLF support).
    let normalised;
    let content = if content.contains('\r') {
        normalised = content.replace("\r\n", "\n");
        normalised.as_str()
    } else {
        content
    };

    // Extract frontmatter region: between first and second `---\n`.
    // The file starts with `---\n`; we skip that delimiter and find the closing one.
    let frontmatter = if let Some(after_open) = content.strip_prefix("---\n") {
        if let Some(close_pos) = after_open.find("\n---\n").or_else(|| {
            // Also handle `---\n` at end of file with no trailing newline after the block close
            if after_open.ends_with("\n---") {
                Some(after_open.len() - 4)
            } else {
                None
            }
        }) {
            &after_open[..close_pos]
        } else {
            // No closing `---` delimiter — EC-013 malformed frontmatter.
            // Return MalformedLockBlock so guard_logic emits log_warn + Continue (PC4 fail-open).
            // Do NOT fall through and scan the body: body-resident factory_lock blocks must NOT
            // be treated as real locks (O4 fix: over-blocking prevention).
            return Err(LockCheckError::MalformedLockBlock(
                "missing closing --- delimiter (EC-013)".to_string(),
            ));
        }
    } else {
        // No opening `---\n` — no frontmatter at all, treat as unlocked.
        return Ok(None);
    };

    // Scan lines for `factory_lock:` key.
    let mut in_factory_lock = false;
    let mut holder: Option<String> = None;
    let mut locked_at: Option<String> = None;
    let mut expires_at: Option<String> = None;

    for line in frontmatter.lines() {
        if line == "factory_lock:" || line.starts_with("factory_lock:") {
            // Check it's the bare key (no inline value after the colon+space).
            // e.g. "factory_lock:" — the value is null/absent (unlocked path)
            // or the sub-fields follow on subsequent lines.
            let after_colon = line["factory_lock:".len()..].trim();
            if after_colon.is_empty() || after_colon == "~" || after_colon == "null" {
                in_factory_lock = true;
            } else {
                // Inline value — treat as malformed (unexpected shape).
                return Err(LockCheckError::MalformedLockBlock(
                    "factory_lock key has unexpected inline value".to_string(),
                ));
            }
            continue;
        }

        if in_factory_lock {
            // Sub-fields must be indented with exactly 2 spaces.
            if line.starts_with("  ") && !line.starts_with("   ") {
                // 2-space indent — a sub-field of factory_lock.
                let field_line = &line[2..]; // strip the 2-space indent
                if let Some(value) = extract_yaml_string_value(field_line, "holder") {
                    holder = Some(value);
                } else if let Some(value) = extract_yaml_string_value(field_line, "locked_at") {
                    locked_at = Some(value);
                } else if let Some(value) = extract_yaml_string_value(field_line, "expires_at") {
                    expires_at = Some(value);
                }
                // Unknown sub-field lines under factory_lock are ignored (fail-open).
            } else if !line.is_empty() {
                // Non-indented, non-empty line after factory_lock: — we've exited the block.
                in_factory_lock = false;
            }
            // Empty lines: stay in in_factory_lock state (blank lines between fields allowed).
        }
    }

    // If factory_lock was not found, return Ok(None) (unlocked).
    if !in_factory_lock && holder.is_none() && locked_at.is_none() && expires_at.is_none() {
        return Ok(None);
    }

    // If factory_lock was found but all fields are None, treat as absent/null (unlocked).
    // This covers `factory_lock: ~` or `factory_lock: null` which would set in_factory_lock
    // briefly but never populate sub-fields. Actually in the current logic, null/~ sets
    // in_factory_lock=true but no sub-fields appear, so we need to handle this:
    if in_factory_lock && holder.is_none() && locked_at.is_none() && expires_at.is_none() {
        // factory_lock block present but null/empty — treat as unlocked (EC-001 variant).
        return Ok(None);
    }

    // factory_lock was found — validate all three required sub-fields.
    let holder_val = match holder {
        Some(h) if !h.is_empty() => h,
        Some(_) => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.holder is empty string (EC-004)".to_string(),
            ));
        }
        None => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.holder field is absent (EC-012 variant)".to_string(),
            ));
        }
    };

    let locked_at_val = match locked_at {
        Some(v) if !v.is_empty() => v,
        Some(_) => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.locked_at is empty string".to_string(),
            ));
        }
        None => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.locked_at field is absent (EC-012)".to_string(),
            ));
        }
    };

    let expires_at_val = match expires_at {
        Some(v) if !v.is_empty() => v,
        Some(_) => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.expires_at is empty string".to_string(),
            ));
        }
        None => {
            return Err(LockCheckError::MalformedLockBlock(
                "factory_lock.expires_at field is absent".to_string(),
            ));
        }
    };

    Ok(Some(LockState {
        holder: holder_val,
        locked_at: locked_at_val,
        expires_at: expires_at_val,
    }))
}

/// Extract the string value from a YAML key-value line like `key: "value"` or `key: value`.
///
/// Returns `Some(value)` if the line starts with `{key}: `, otherwise `None`.
/// Strips surrounding double-quotes from quoted values.
/// Returns `Some("")` for empty quoted values `""`.
fn extract_yaml_string_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{}: ", key);
    let bare_prefix = format!("{}:", key);

    let raw_value = if let Some(rest) = line.strip_prefix(&prefix) {
        rest
    } else if line == bare_prefix {
        // `key:` with no value — treat as empty.
        ""
    } else {
        return None;
    };

    // Strip surrounding double-quotes if present.
    let value = if raw_value.starts_with('"') && raw_value.ends_with('"') && raw_value.len() >= 2 {
        &raw_value[1..raw_value.len() - 1]
    } else {
        raw_value
    };

    Some(value.to_string())
}

/// Parse an ISO-8601 datetime string into a `chrono::DateTime<chrono::Utc>`.
///
/// Returns `Ok(dt)` on success, `Err(MalformedLockBlock)` if unparseable
/// (EC-005).
pub fn parse_iso8601(s: &str) -> Result<chrono::DateTime<chrono::Utc>, LockCheckError> {
    s.parse::<chrono::DateTime<chrono::Utc>>().map_err(|e| {
        LockCheckError::MalformedLockBlock(format!("invalid ISO-8601 datetime '{}': {}", s, e))
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

    let content = match String::from_utf8(state_bytes) {
        Ok(s) => s,
        Err(e) => {
            (callbacks.log_warn)(&format!(
                "StateReadError: STATE.md is not valid UTF-8: {}",
                e
            ));
            return HookResult::Continue;
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
// RED GATE: every test MUST FAIL before implementation begins (todo!() panics).
// Tests will pass once the implementer fills in the helper bodies in T-3.
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
    // Test files may use expect/unwrap/panic for failure reporting.
    #![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

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

    /// STATE.md content with factory_lock.expires_at exactly at the Unix epoch
    /// boundary we use for boundary testing (EC-002). The test uses a fixed
    /// past timestamp for the "exactly now" scenario — actual boundary test
    /// is implemented by calling parse_iso8601 + comparison directly.
    fn state_md_exact_boundary_lock(expires_at: &str) -> Vec<u8> {
        format!(
            "---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\nfactory_lock:\n  holder: \"other@example.com\"\n  locked_at: \"2026-06-10T14:00:00Z\"\n  expires_at: \"{expires_at}\"\n---\n\n# STATE\n"
        )
        .into_bytes()
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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
    /// RED GATE: guard_logic is todo!() — panics immediately.
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

    /// EC-002: expires_at == now exactly → Continue (just-expired, not greater-than).
    ///
    /// BC-4.13.001 EC-002: `now > expires_at` semantics mean `now == expires_at`
    /// evaluates as NOT greater-than, so the lock is treated as just-expired → Continue.
    ///
    /// This test uses parse_iso8601 + the comparison semantics directly.
    /// Strategy: parse a fixed timestamp, use it as both `expires_at` and `now`;
    /// verify that `now > expires_at` is false (so the lock is considered expired).
    ///
    /// RED GATE: parse_iso8601 is todo!() — panics immediately.
    #[test]
    fn test_BC_4_13_001_expires_at_exact_boundary_treated_as_expired() {
        let ts = "2026-06-10T15:00:00Z";
        // parse_iso8601 must succeed on a valid ISO-8601 UTC string.
        let expires_at_dt = parse_iso8601(ts).expect("parse_iso8601 must parse valid ISO-8601");
        // Simulate: now == expires_at exactly.
        let now_dt = expires_at_dt;
        // The BC-4.13.001 EC-002 semantics: lock is blocked only when now > expires_at.
        // When now == expires_at: now > expires_at is false → treat as expired → Continue.
        assert!(
            now_dt <= expires_at_dt,
            "EC-002: now == expires_at must evaluate as NOT greater-than (lock expired). \
             The `now > expires_at` check must use strict greater-than semantics."
        );
        // Additionally, verify via guard_logic end-to-end using a timestamp that has
        // already passed (exact boundary relative to a fixed past time is non-trivial
        // with real wall-clock; the unit test for the pure boundary uses the above).
        // The integration-level boundary test uses a past timestamp (already expired).
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        // Use a timestamp in the distant past as the "exact boundary" anchor — any time
        // in the past is already expired, exercising the same `now > expires_at` false path.
        let callbacks = make_callbacks_success(
            state_md_exact_boundary_lock("2020-01-01T00:45:00Z"),
            "self@example.com",
            warn_log.clone(),
        );
        let payload = payload_for_tool("Edit");
        let result = guard_logic(payload, callbacks);
        assert_eq!(
            result,
            HookResult::Continue,
            "EC-002: past timestamp must return Continue (expired lock = unlocked)"
        );
    }

    // -----------------------------------------------------------------------
    // Pure helper tests — each pure fn gets at least one focused test.
    // These fail with todo!() panics before implementation.
    // -----------------------------------------------------------------------

    /// matches_factory_artifacts_push: push command → true.
    ///
    /// RED GATE: todo!() panics immediately.
    #[test]
    fn test_BC_4_13_001_push_regex_matches_factory_artifacts_push() {
        assert!(
            matches_factory_artifacts_push("git push origin factory-artifacts"),
            "push command must match factory-artifacts push pattern"
        );
    }

    /// matches_factory_artifacts_push: non-push command → false.
    ///
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    /// RED GATE: todo!() panics immediately.
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
    // The existing boundary test (test_BC_4_13_001_expires_at_exact_boundary_treated_as_expired)
    // is tautological: it sets now == expires_at then asserts now <= expires_at, which
    // is trivially true regardless of guard logic. This test replaces that tautology with
    // a real API call to `is_expired(now, expires_at) -> bool` which the implementer must
    // extract from guard_logic's inlined `now > expires_at_dt` comparison.
    //
    // RED GATE STATUS: COMPILE ERROR — `is_expired` does not exist yet.
    // The implementer must add:
    //   pub fn is_expired(now: chrono::DateTime<chrono::Utc>, expires_at: chrono::DateTime<chrono::Utc>) -> bool {
    //       now > expires_at
    //   }
    // and thread calls to it through guard_logic's Step 5.
    // -----------------------------------------------------------------------

    /// EC-002: `is_expired` pure helper — now == expires_at → true (just-expired → Continue).
    ///
    /// BC-4.13.001 EC-002: the lock is expired (and the guard must return Continue) when
    /// `now > expires_at`. The `==` case must also be expired (not blocked), so
    /// `is_expired` returns true when `now == expires_at`.
    ///
    /// Three cases asserted:
    ///   1. now == expires_at → true (exact boundary, just-expired → Continue per EC-002).
    ///   2. now 1 second BEFORE expires_at → false (not expired → would Block if foreign).
    ///   3. now 1 second AFTER expires_at → true (expired → Continue).
    ///
    /// RED GATE: `is_expired` does not exist — COMPILE ERROR until implementer adds it.
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
    /// RED GATE: assertion fails — guard returns Continue instead of Block.
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
    /// RED GATE: assertion fails — guard may return Block instead of Continue for this
    /// malformed input.
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
    /// RED GATE: todo!() panics immediately.
    #[test]
    fn test_BC_4_13_001_trim_git_email_strips_trailing_newline() {
        let result = trim_git_email("dev@example.com\n");
        assert_eq!(result, "dev@example.com");
    }

    /// trim_git_email: no trailing newline unchanged.
    ///
    /// RED GATE: todo!() panics immediately.
    #[test]
    fn test_BC_4_13_001_trim_git_email_unchanged_when_no_newline() {
        let result = trim_git_email("dev@example.com");
        assert_eq!(result, "dev@example.com");
    }

    /// build_block_message: all 5 fields present in the message.
    ///
    /// RED GATE: todo!() panics immediately.
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
