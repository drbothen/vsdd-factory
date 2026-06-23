//! factory-lock — pure library crate for factory_lock CRUD on STATE.md frontmatter.
//!
//! Exports:
//!   - [`RenewOutcome`] — result of `renew_lock()`: NoOp (absent or no-op) or
//!     Renewed(String) (new full STATE.md content with updated expires_at).
//!   - [`LockError`] — error variants for lock operations.
//!   - [`FactoryLock`] — parsed representation of a held factory lock.
//!   - [`renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError>`]
//!     — pure content-in/content-out renewal; no std::fs; WASM-hermetic.
//!   - [`acquire_lock`] — stub (S-18.04b scope).
//!   - [`clear_lock`] — stub (S-18.04b scope).
//!
//! # Architecture compliance (ADR-028)
//!
//! - Pure library: no `std::fs`, no host I/O, no `exec_subprocess`. WASM-hermetic.
//! - Dependency: factory-lock → factory-lock-parse; never the reverse (no cycles).
//! - No `[[bin]]` target in this crate (forbidden per story Forbidden Dependencies).
//! - `renew_lock()` performs a `factory_lock:` key presence pre-check BEFORE calling
//!   `parse_factory_lock()` — bash parity (F-NW2-006 / ADR-028 §Decision 9).
//! - Path-based `renew_lock(&Path)` form is STRUCK (F-NW2-005); only the pure
//!   content-in/content-out form `renew_lock(state_md_content: &str)` is acceptable.
//! - `expires_at` MUST be formatted as `YYYY-MM-DDTHH:MM:SSZ` (UTC, second precision,
//!   uppercase Z suffix) using `chrono::format("%Y-%m-%dT%H:%M:%SZ")` — NOT
//!   `to_rfc3339()` (AC-018 F-NW-008 / BC-5.40.001 §Invariant 2).
//! - LF-only output; CRLF normalization is performed by `parse_factory_lock()` before
//!   rewriting (F-NW-009).
//! - `Err(LockError::Malformed)` ONLY when `factory_lock:` key IS present AND block
//!   is malformed. Malformed fence WITHOUT the lock key → `Ok(RenewOutcome::NoOp)`.

// Allow the BC-based test naming convention workspace-wide.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use chrono::{DateTime, Duration, Utc};
use factory_lock_parse as flp;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Outcome of [`renew_lock`].
///
/// # Variants
///
/// - `NoOp` — `factory_lock:` key is absent (or absent regardless of fence shape),
///   OR the recomputed `expires_at` is byte-identical to the existing value.
///   STATE.md is unchanged; the caller MUST NOT call `host::write_file`.
///   Matches bash `factory-lock-write.sh renew` parity: silently exits 0 when
///   `factory_lock:` key is absent (F-NW2-006 / ADR-028 §Decision 9).
///
/// - `Renewed(String)` — lock was held and `expires_at` was updated to
///   `now + 2700s`. The `String` contains the new full STATE.md content.
///   The caller MUST call `host::write_file(".factory/STATE.md", content)`.
#[derive(Debug)]
pub enum RenewOutcome {
    /// Lock was absent (or key absent regardless of fence shape) — STATE.md unchanged.
    NoOp,
    /// Lock was held and expires_at was updated. Contains the new full STATE.md content.
    Renewed(String),
}

/// Error variants for lock operations.
///
/// # `Malformed`
///
/// `factory_lock:` key IS present in the frontmatter AND the block is malformed
/// (missing sub-field, empty sub-field, or missing closing `---` fence after the
/// lock key is found). Callers (the precompact-flush plugin) MUST downgrade this
/// to an advisory warning and proceed with the flush commit — do NOT exit 2
/// (EC-012 / BC-7.07.001 PC3 / ADR-028 §Decision 9).
#[derive(Debug, Clone, PartialEq)]
pub enum LockError {
    /// `factory_lock:` key IS present but the block is malformed.
    Malformed(String),
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockError::Malformed(msg) => write!(f, "factory_lock block is malformed: {msg}"),
        }
    }
}

impl std::error::Error for LockError {}

/// Parsed representation of a held factory lock.
///
/// All three sub-fields are required when the lock block is present.
#[derive(Debug, Clone, PartialEq)]
pub struct FactoryLock {
    /// Email of the current lock holder.
    pub holder: String,
    /// ISO-8601 timestamp when the lock was acquired.
    pub locked_at: String,
    /// ISO-8601 datetime when the lock auto-expires (YYYY-MM-DDTHH:MM:SSZ format).
    pub expires_at: String,
}

// ---------------------------------------------------------------------------
// Public functions
// ---------------------------------------------------------------------------

/// Renew `factory_lock.expires_at` in STATE.md frontmatter.
///
/// Pure content-in/content-out; no `std::fs`; WASM-hermetic.
///
/// # Algorithm (ADR-028 §Decision 9 / F-NW2-005/006)
///
/// 1. Presence pre-check: if `factory_lock:` key is NOT in the frontmatter
///    (regardless of fence shape), return `Ok(RenewOutcome::NoOp)` immediately.
///    This matches bash parity — bash checks `factory_lock:` presence first.
/// 2. Call `factory_lock_parse::parse_factory_lock(content)`.
///    - `Ok(None)` (key absent or holder null/absent) → `Ok(RenewOutcome::NoOp)`.
///    - `Err(MalformedLockBlock)` (key present but malformed) → `Err(LockError::Malformed)`.
///    - `Ok(Some(lock_state))` (key present, holder non-empty) → proceed to step 3.
/// 3. Compute `new_expires_at = Utc::now() + 2700s`, formatted as
///    `YYYY-MM-DDTHH:MM:SSZ` via `chrono::format("%Y-%m-%dT%H:%M:%SZ")` (NOT
///    `to_rfc3339()` — which emits `+00:00` / sub-seconds and violates AC-018 F-NW-008).
/// 4. If `new_expires_at == existing expires_at` (byte-identical recomputation,
///    F-R3-005 / ADR-028 §Decision 16): return `Ok(RenewOutcome::NoOp)` to suppress
///    spurious renewal.
/// 5. Rewrite the `expires_at:` line in the frontmatter block; preserve `holder` and
///    `locked_at` unchanged. Return `Ok(RenewOutcome::Renewed(new_content))`.
///
/// CRLF normalization: `parse_factory_lock()` calls `content.replace("\r\n", "\n")`
/// internally; the output is always LF-only (F-NW-009).
///
/// # Errors
///
/// Returns `Err(LockError::Malformed)` ONLY when `factory_lock:` key IS present AND
/// the block is malformed. The caller must downgrade to an advisory warning and
/// proceed with flush (EC-012 / ADR-028 §Decision 9).
pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError> {
    renew_lock_with_now(state_md_content, Utc::now)
}

/// Injectable clock variant of `renew_lock` for testability.
///
/// Identical semantics to `renew_lock`, but accepts a `now_fn` closure that
/// returns the current UTC time as a `chrono::DateTime<chrono::Utc>`.
/// This enables deterministic testing of the byte-identical expires_at guard
/// (ADR-028 §Decision 16 F-R3-005) without relying on wall-clock timing.
///
/// `renew_lock` delegates to this with `|| Utc::now()`.
pub fn renew_lock_with_now<F>(state_md_content: &str, now_fn: F) -> Result<RenewOutcome, LockError>
where
    F: Fn() -> DateTime<Utc>,
{
    // Step 1: presence pre-check — bash parity (F-NW2-006 / ADR-028 §Decision 9).
    // If factory_lock: key is NOT present (regardless of fence shape), return NoOp.
    if !has_factory_lock_key(state_md_content) {
        return Ok(RenewOutcome::NoOp);
    }

    // Step 2: parse the factory_lock block.
    let lock_state = match flp::parse_factory_lock(state_md_content) {
        Ok(Some(ls)) => ls,
        Ok(None) => {
            // Key was present but lock is null/absent holder → NoOp.
            return Ok(RenewOutcome::NoOp);
        }
        Err(flp::LockParseError::MalformedLockBlock(msg)) => {
            return Err(LockError::Malformed(msg));
        }
    };

    // Step 3: compute new expires_at = now + 2700s, formatted as YYYY-MM-DDTHH:MM:SSZ.
    // MUST use format("%Y-%m-%dT%H:%M:%SZ") — NOT to_rfc3339() (AC-018 F-NW-008).
    let new_expires_at = (now_fn() + Duration::seconds(2700))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();

    // Step 4: spurious renewal guard — if byte-identical, return NoOp (F-R3-005).
    if new_expires_at == lock_state.expires_at {
        return Ok(RenewOutcome::NoOp);
    }

    // Step 5: rewrite expires_at in the frontmatter, preserving holder and locked_at.
    // CRLF normalization: replace \r\n with \n first (F-NW-009).
    let normalized = state_md_content.replace("\r\n", "\n");
    let new_content = rewrite_expires_at(&normalized, &new_expires_at);

    Ok(RenewOutcome::Renewed(new_content))
}

/// Check whether the `factory_lock:` key appears in the frontmatter.
///
/// Presence pre-check used by `renew_lock()` BEFORE calling `parse_factory_lock()`.
/// Matches bash parity: `factory_lock:` absent → silent NoOp; malformed fence WITHOUT
/// the lock key → NoOp (not Malformed). Only a present key with a malformed block
/// yields `Err(Malformed)` (F-NW2-006 / ADR-028 §Decision 9).
///
/// Scans lines in the "open frontmatter region" (after the opening `---` line,
/// until the closing `---` line or end of content). Per ADR-028 §Decision 14 F-R3-002,
/// awk open-region semantics apply: the scan continues even if there is no closing `---`,
/// so `factory_lock:` in the "body region" (after an unclosed `---`) is still found.
pub fn has_factory_lock_key(state_md_content: &str) -> bool {
    // Normalize CRLF.
    let normalized;
    let content = if state_md_content.contains('\r') {
        normalized = state_md_content.replace("\r\n", "\n");
        normalized.as_str()
    } else {
        state_md_content
    };

    // Must start with `---\n` to have a frontmatter region.
    let after_open = match content.strip_prefix("---\n") {
        Some(rest) => rest,
        None => return false,
    };

    // Scan all lines in the open region (awk open-region semantics: no closing `---` required).
    // This matches bash parity: the region ends at the closing `---` fence if present,
    // but the scan does not require the fence to close (F-R3-002 / ADR-028 §Decision 14).
    for line in after_open.lines() {
        if line == "---" {
            // Closing fence found — stop scanning.
            break;
        }
        // Check if this line starts with `factory_lock:` (with optional trailing content).
        if line == "factory_lock:" || line.starts_with("factory_lock:") {
            return true;
        }
    }
    false
}

/// Rewrite the `expires_at:` sub-field inside the `factory_lock:` block.
///
/// Finds the first occurrence of `  expires_at:` (2-space indent) and replaces
/// its value. Content before and after is preserved byte-for-byte.
///
/// Internal helper for `renew_lock_with_now`.
fn rewrite_expires_at(content: &str, new_expires_at: &str) -> String {
    let mut result = String::with_capacity(content.len() + 32);
    let mut in_factory_lock = false;
    let mut expires_at_rewritten = false;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');

        if trimmed == "factory_lock:" || trimmed.starts_with("factory_lock:") {
            in_factory_lock = true;
            result.push_str(line);
            continue;
        }

        if in_factory_lock && !expires_at_rewritten {
            // Look for `  expires_at:` with exactly 2-space indent.
            if trimmed.starts_with("  expires_at:") {
                // Replace value while preserving leading whitespace and any trailing newline.
                let has_lf = line.ends_with('\n');
                result.push_str(&format!("  expires_at: {new_expires_at}"));
                if has_lf {
                    result.push('\n');
                }
                expires_at_rewritten = true;
                continue;
            }
            // Exit factory_lock block on non-indented, non-empty line.
            if !trimmed.is_empty() && !trimmed.starts_with(' ') {
                in_factory_lock = false;
            }
        }

        result.push_str(line);
    }

    result
}

/// Acquire the factory lock in STATE.md frontmatter content.
///
/// Pure content-in/content-out; no `std::fs`.
/// Intended for S-18.04b and related lock-management stories.
pub fn acquire_lock(
    _state_md_content: &str,
    _holder: &str,
    _locked_at: &str,
    _expires_at: &str,
) -> Result<String, LockError> {
    todo!("acquire_lock: scoped to S-18.04b")
}

/// Clear the factory lock in STATE.md frontmatter content.
///
/// Pure content-in/content-out; no `std::fs`.
/// Removes the `factory_lock:` block (or nulls `holder`) so subsequent
/// `renew_lock()` calls return `Ok(RenewOutcome::NoOp)`.
pub fn clear_lock(_state_md_content: &str) -> Result<String, LockError> {
    todo!("clear_lock: scoped to S-18.04b")
}

// ---------------------------------------------------------------------------
// Internal helper — re-exported for use by has_factory_lock_key tests
// ---------------------------------------------------------------------------

/// Parse the `factory_lock:` block from STATE.md content, delegating to
/// `factory_lock_parse::parse_factory_lock`.
///
/// Internal helper surfaced for testability. Use `renew_lock` for the public API.
///
/// # Returns
///
/// - `Ok(Some(FactoryLock))` — key present, holder non-empty, all fields valid.
/// - `Ok(None)` — key absent or `holder` is null/absent/empty.
/// - `Err(LockError::Malformed)` — key present but block is malformed.
pub fn parse_lock(state_md_content: &str) -> Result<Option<FactoryLock>, LockError> {
    match flp::parse_factory_lock(state_md_content) {
        Ok(Some(ls)) => Ok(Some(FactoryLock {
            holder: ls.holder,
            locked_at: ls.locked_at,
            expires_at: ls.expires_at,
        })),
        Ok(None) => Ok(None),
        Err(flp::LockParseError::MalformedLockBlock(msg)) => Err(LockError::Malformed(msg)),
    }
}
