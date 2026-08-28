//! stamp-state-timestamp — PostToolUse WASM hook plugin (S-17.05 / ADR-046).
//!
//! Fires on every `PostToolUse` event for `Edit`, `Write`, or `MultiEdit` tools
//! that land a write to `.factory/STATE.md`.
//!
//! On each invocation the hook:
//!   1. Reads the on-disk `.factory/STATE.md` via `host::read_file` (full content,
//!      post-write — the tool's write has already landed by the time PostToolUse fires).
//!   2. Validates frontmatter (UTF-8, opening + closing `---` delimiters, `timestamp:`
//!      anchor line present). On any failure: fail-open (no write) per PC3.
//!   3. PC1 (unconditional timestamp re-stamp): replaces the `timestamp:` line with
//!      `timestamp: <now_fn() formatted as YYYY-MM-DDTHH:MM:SSZ>`, regardless of what
//!      the agent's own write proposed.
//!   4. PC2 (identity-gated expires_at renewal): calls `callbacks.exec_subprocess`
//!      to get git email, classifies via `factory_lock::classify_identity_resolution`,
//!      calls `factory_lock::renew_lock_if_holder` to conditionally rewrite
//!      `factory_lock.expires_at = now + TTL_SECONDS` only when the resolved identity
//!      byte-equals the recorded `holder`. Foreign or expired holders are never renewed.
//!   5. Writes the reconstructed full content (frontmatter substitution + unchanged body)
//!      back via `host::write_file`. On write error: swallow (fail-open per PC3).
//!   6. Never touches acquire/release/CAS-push (PC5 — `event = "PostToolUse"`,
//!      `tool = "^(Edit|Write|MultiEdit)$"` in the registry ensures structural exclusion).
//!
//! # Behavioral Contracts
//!
//! - BC-4.17.001 v1.27: stamp-state-timestamp PostToolUse hook — unconditional
//!   `timestamp:` re-stamp (PC1), identity-gated `expires_at` renewal (PC2),
//!   fail-open (PC3), idempotent frontmatter-only rewrite (PC4), no lock-lifecycle
//!   involvement (PC5).
//! - BC-5.40.001 v1.21 PC4: mid-burst TTL keep-alive — actor reassigned to this hook.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (ADR-025 Decision 1; BC-4.17.001 architecture compliance).
//! - No `serde_yaml` / `serde_norway` (Architecture Compliance Rule 6; manual
//!   line-by-line frontmatter scan via `factory_lock_parse::extract_frontmatter`).
//! - No `regex` crate (Architecture Compliance Rule 6).
//! - No independent TTL literal (Architecture Compliance Rule 7; imports
//!   `factory_lock_parse::TTL_SECONDS` — never redeclares 2700 as its own const/literal).
//! - Frontmatter-only rewrite (Architecture Compliance Rule 8; body bytes never read
//!   for decision purposes or modified).
//! - Identity comparison via `factory_lock::renew_lock_if_holder` →
//!   `factory_lock::classify_identity_resolution` (Architecture Compliance Rule 9;
//!   no direct `trim_git_email` call; flows through renew_lock_if_holder transparently).
//! - `async = false` required in registry entry (ADR-019; ADR-046; BC-4.17.001).
//! - `on_error = "continue"` (BC-4.17.001 PC3/Invariant 4; fail-open; no block_intent).
//! - Pure `fn guard_logic(...)` takes all host I/O as injectable closures;
//!   unit tests exercise every branch without a WASM runtime.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use chrono::{DateTime, Utc};
use factory_lock_parse as flp;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.17.001 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Injectable callbacks surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting host calls injected into `guard_logic` for testability.
/// In production (`on_post_tool_use`), these are wired to real vsdd_hook_sdk host fns.
///
/// Mirrors `verify-factory-lock`'s `GuardCallbacks` pattern, extended with
/// `write_file` (PostToolUse write-back) and `now_fn` (injectable clock for
/// deterministic testing of AC-003/AC-010 timestamp-equality assertions).
pub struct StampCallbacks<R, W, E, NF>
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    W: FnOnce(&str, &[u8]) -> Result<(), String>,
    E: FnOnce(&[&str]) -> Result<(i32, String), String>,
    NF: FnOnce() -> DateTime<Utc>,
{
    /// Read the full content of `.factory/STATE.md` via `host::read_file`.
    /// `(path, max_bytes, timeout_ms)` → `Ok(bytes)` or `Err(host_error_description)`.
    /// Production caller passes `factory_lock_parse::STATE_MD_MAX_BYTES` as `max_bytes`
    /// (BC-5.40.001 / BC-4.13.001 cap-parity requirement; AC-008).
    pub read_file: R,
    /// Write the rewritten content back to `.factory/STATE.md` via `host::write_file`.
    /// `(path, content_bytes)` → `Ok(())` or `Err(host_error_description)`.
    pub write_file: W,
    /// Execute a subprocess with the given argv slice via `host::exec_subprocess`.
    /// Returns `Ok((exit_code, stdout))` or `Err(host_error_description)`.
    /// Used for `git config user.email` identity resolution (PC2 gate; AC-007).
    pub exec_subprocess: E,
    /// Injectable clock returning the current UTC instant.
    /// Production: `Utc::now`. Tests: inject a fixed timestamp for deterministic
    /// AC-003/AC-010 `expires_at = now + TTL_SECONDS` assertions.
    pub now_fn: NF,
}

// ---------------------------------------------------------------------------
// Core guard logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core stamp-state-timestamp guard logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Decision tree (per BC-4.17.001):
///   1. Call `read_file(".factory/STATE.md", STATE_MD_MAX_BYTES, timeout_ms)`.
///      On error: return `HookResult::Continue` (PC3 fail-open; no write).
///   2. Validate UTF-8, opening `---\n`, closing `---` delimiter, `timestamp:`
///      anchor line present. On any failure: return Continue (PC3).
///   3. PC1: replace `timestamp:` line → `timestamp: <now formatted YYYY-MM-DDTHH:MM:SSZ>`.
///   4. PC2: call `exec_subprocess(["git", "config", "user.email"])`, classify via
///      `factory_lock::classify_identity_resolution`, call
///      `factory_lock::renew_lock_if_holder(content_after_pc1, resolve_identity, now_fn)`
///      to conditionally rewrite `factory_lock.expires_at = now + TTL_SECONDS`.
///      Identity-mismatch, absent/expired lock, or identity-resolution failure →
///      `expires_at` left byte-identical (SAFETY-CRITICAL for AC-006).
///   5. Call `write_file(".factory/STATE.md", reconstructed_full_content)`.
///      On write error: swallow (PC3 fail-open; agent's write is not reverted).
///   6. Return `HookResult::Continue` (Invariant 4: no `block_intent` capability).
///
/// # BC traces
/// - BC-4.17.001 PC1: unconditional `timestamp:` re-stamp (AC-001, AC-002)
/// - BC-4.17.001 PC2: identity-gated `factory_lock.expires_at` renewal (AC-003..AC-007)
/// - BC-4.17.001 PC3: fail-open on any read/parse/UTF-8/write error (AC-008, AC-009)
/// - BC-4.17.001 PC4: idempotent, frontmatter-only rewrite (AC-010)
/// - BC-4.17.001 PC5: no acquire/release/CAS involvement (AC-011)
/// - BC-5.40.001 PC4: mid-burst TTL keep-alive (AC-014)
pub fn guard_logic<R, W, E, NF>(
    _payload: HookPayload,
    _callbacks: StampCallbacks<R, W, E, NF>,
) -> HookResult
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    W: FnOnce(&str, &[u8]) -> Result<(), String>,
    E: FnOnce(&[&str]) -> Result<(i32, String), String>,
    NF: FnOnce() -> DateTime<Utc>,
{
    // BC-5.38.001 stub obligation: non-trivial body uses todo!() until T-3 is implemented.
    // BC-5.38.005 self-check: including real logic here would make AC-001..AC-010 tests pass
    // trivially — therefore todo!() is mandatory.
    todo!(
        "S-17.05 T-3: implement stamp-state-timestamp guard logic \
         (BC-4.17.001 PC1 timestamp re-stamp, PC2 identity-gated expires_at renewal, \
         PC3 fail-open, PC4 frontmatter-only rewrite, PC5 no lock-lifecycle involvement)"
    )
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns — WIRING-EXEMPT per BC-5.38.003)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `guard_logic`. WIRING-EXEMPT (BC-5.38.003): this function is
/// purely delegating to `guard_logic` via single-call wiring to each host
/// capability; no domain logic resides here.
///
/// host::exec_subprocess signature: `(cmd, args, stdin, timeout_ms, max_output_bytes)`.
/// We call `git config user.email` with no stdin. Max output is 512 bytes
/// (email addresses are short; prevents wasting WASM fuel on large output).
///
/// host::read_file uses `STATE_MD_MAX_BYTES` as the byte cap
/// (BC-5.40.001 / BC-4.13.001 cap-parity requirement).
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    guard_logic(
        payload,
        StampCallbacks {
            read_file: |path, max_bytes, timeout_ms| match host::read_file(
                path, max_bytes, timeout_ms,
            ) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(format!("{:?}", e)),
            },
            write_file: |path, content| match host::write_file(
                path,
                content,
                flp::STATE_MD_MAX_BYTES,
                5000,
            ) {
                Ok(()) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            exec_subprocess: |argv| match argv.split_first() {
                Some((cmd, args)) => match host::exec_subprocess(cmd, args, &[], 5000, 512) {
                    Ok(result) => {
                        let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                        Ok((result.exit_code, stdout))
                    }
                    Err(e) => Err(format!("{:?}", e)),
                },
                None => Err("exec_subprocess: empty argv".to_string()),
            },
            now_fn: Utc::now,
        },
    )
}
