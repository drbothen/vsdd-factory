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
pub fn renew_lock(_state_md_content: &str) -> Result<RenewOutcome, LockError> {
    todo!()
}

/// Injectable clock variant of `renew_lock` for testability (TDD Red Gate stub).
///
/// Identical semantics to `renew_lock`, but accepts a `now_fn` closure that
/// returns the current UTC time as a `YYYY-MM-DDTHH:MM:SSZ`-formatted string.
/// This enables deterministic testing of the byte-identical expires_at guard
/// (ADR-028 §Decision 16 F-R3-005) without relying on wall-clock timing.
///
/// The implementer MUST implement this as the core of `renew_lock`, with
/// `renew_lock` delegating to `renew_lock_with_now(content, || Utc::now()...)`.
///
/// # Test usage
///
/// ```rust
/// // Test the NoOp path when now + 2700s exactly matches existing expires_at
/// let fixed_now = "2026-06-22T12:00:00Z"; // 2026-06-22T12:00:00Z + 2700s = 2026-06-22T12:45:00Z
/// let expires_at_matches = "2026-06-22T12:45:00Z";
/// let content = make_state_md_with_expires_at(expires_at_matches);
/// let result = renew_lock_with_now(&content, || fixed_now.to_string()... + 2700s);
/// assert!(matches!(result, Ok(RenewOutcome::NoOp)));
/// ```
pub fn renew_lock_with_now<F>(
    _state_md_content: &str,
    _now_fn: F,
) -> Result<RenewOutcome, LockError>
where
    F: Fn() -> chrono::DateTime<chrono::Utc>,
{
    todo!()
}

/// Check whether the `factory_lock:` key appears in the frontmatter.
///
/// Presence pre-check used by `renew_lock()` BEFORE calling `parse_factory_lock()`.
/// Matches bash parity: `factory_lock:` absent → silent NoOp; malformed fence WITHOUT
/// the lock key → NoOp (not Malformed). Only a present key with a malformed block
/// yields `Err(Malformed)` (F-NW2-006 / ADR-028 §Decision 9).
///
/// Scans only the frontmatter region (between opening `---` and closing `---` fences).
/// Returns `true` if any line starts with `factory_lock:` (with optional trailing
/// whitespace / colon-only) inside the frontmatter.
pub fn has_factory_lock_key(_state_md_content: &str) -> bool {
    todo!()
}

/// Acquire the factory lock in STATE.md frontmatter content.
///
/// Pure content-in/content-out; no `std::fs`.
/// Intended for S-18.04b and related lock-management stories.
///
/// # Parameters
///
/// - `state_md_content` — current STATE.md content as a string.
/// - `holder` — email of the agent acquiring the lock.
/// - `locked_at` — ISO-8601 timestamp (YYYY-MM-DDTHH:MM:SSZ) of acquisition.
/// - `expires_at` — ISO-8601 timestamp (YYYY-MM-DDTHH:MM:SSZ) of expiry.
///
/// Returns `Ok(new_content)` with the `factory_lock:` block written, or
/// `Err(LockError::Malformed)` if existing frontmatter is unrecoverably malformed.
pub fn acquire_lock(
    _state_md_content: &str,
    _holder: &str,
    _locked_at: &str,
    _expires_at: &str,
) -> Result<String, LockError> {
    todo!()
}

/// Clear the factory lock in STATE.md frontmatter content.
///
/// Pure content-in/content-out; no `std::fs`.
/// Removes the `factory_lock:` block (or nulls `holder`) so subsequent
/// `renew_lock()` calls return `Ok(RenewOutcome::NoOp)`.
///
/// Returns `Ok(new_content)` with the lock cleared, or `Err(LockError::Malformed)`
/// if the frontmatter is unrecoverably malformed.
pub fn clear_lock(_state_md_content: &str) -> Result<String, LockError> {
    todo!()
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
