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
#[allow(dead_code)]
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

// ---------------------------------------------------------------------------
// S-17.05 Red Gate test suite (BC-5.38.001 strict tdd_mode)
//
// 17 Rust unit tests covering BC-4.17.001 PC1–PC5 + AC-001..AC-012.
// Every test calls guard_logic(), which is todo!() in the stub phase.
// ALL 17 tests MUST FAIL (via todo!() panic) before any implementation.
// After S-17.05 T-3 is complete, all 17 tests MUST PASS (Green Gate).
//
// Plus 2 source-scan / constant-equality tests that do NOT call guard_logic:
//   - test_ttl_seconds_constant_equals_2700 (FAILS: stub TTL_SECONDS = 0)
//   - test_ttl_seconds_is_imported_not_redeclared (PASSES: no duplicate in stub)
//
// Test naming follows the S-17.05 v1.3 Red Gate Test Table (authoritative).
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::missing_panics_doc
    )]

    use super::*;
    use chrono::{DateTime, Duration, Utc};
    use factory_lock_parse as flp;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    // -----------------------------------------------------------------------
    // HookPayload fixture builder
    // -----------------------------------------------------------------------

    /// Build a minimal PostToolUse HookPayload for the given tool name targeting STATE.md.
    fn payload_for_post_tool_use(tool_name: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PostToolUse",
            "tool_name": tool_name,
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": { "file_path": ".factory/STATE.md" }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    // -----------------------------------------------------------------------
    // STATE.md content fixture builders
    // -----------------------------------------------------------------------

    /// STATE.md with a stale timestamp and NO factory_lock block.
    fn state_no_lock_old_ts() -> String {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "timestamp: 2020-01-01T00:00:00Z\n",
            "phase: test\n",
            "---\n",
            "\n# STATE\n",
            "Body content here.\n",
        )
        .to_string()
    }

    /// STATE.md with stale timestamp, factory_lock.holder = "holder@example.com" (foreign).
    /// Caller identity is "caller@example.com" in these tests — deliberately mismatches.
    fn state_with_foreign_lock(expires_at: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: 2020-01-01T00:00:00Z\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"holder@example.com\"\n",
                "  locked_at: \"2026-01-01T10:00:00Z\"\n",
                "  expires_at: \"{expires}\"\n",
                "---\n",
                "\n# STATE\n",
                "Body content here.\n",
            ),
            expires = expires_at,
        )
    }

    /// STATE.md with stale timestamp, factory_lock.holder = "caller@example.com" (self-held).
    /// Identity match: caller == holder → PC2 gate opens → renewal occurs.
    fn state_with_self_lock() -> String {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "timestamp: 2020-01-01T00:00:00Z\n",
            "phase: test\n",
            "factory_lock:\n",
            "  holder: \"caller@example.com\"\n",
            "  locked_at: \"2026-01-01T10:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
            "---\n",
            "\n# STATE\n",
            "Body content here.\n",
        )
        .to_string()
    }

    /// STATE.md with factory_lock.holder = "" (empty string).
    /// parse_factory_lock returns Err(MalformedLockBlock) for empty holder.
    /// PC2 gate must skip renewal; PC1 timestamp stamp must still proceed.
    fn state_with_empty_holder() -> String {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "timestamp: 2020-01-01T00:00:00Z\n",
            "phase: test\n",
            "factory_lock:\n",
            "  holder: \"\"\n",
            "  locked_at: \"2026-01-01T10:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
            "---\n",
            "\n# STATE\n",
        )
        .to_string()
    }

    /// Malformed STATE.md: valid opening `---` but no closing `---` delimiter.
    fn state_malformed_no_closing_delimiter() -> String {
        concat!(
            "---\n",
            "document_type: state\n",
            "timestamp: 2020-01-01T00:00:00Z\n",
            "phase: test\n",
        )
        .to_string()
        // No closing `---` → extract_frontmatter / parse_factory_lock sees malformed structure
    }

    /// Valid STATE.md frontmatter but NO `timestamp:` anchor line.
    fn state_no_timestamp_line() -> String {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "---\n",
            "\n# STATE\n",
        )
        .to_string()
    }

    /// Fixed UTC instant for deterministic test assertions.
    /// 2026-08-27T12:00:00Z → now + 2700s = 2026-08-27T12:45:00Z.
    fn fixed_now() -> DateTime<Utc> {
        "2026-08-27T12:00:00Z"
            .parse::<DateTime<Utc>>()
            .expect("fixed_now must parse as DateTime<Utc>")
    }

    // -----------------------------------------------------------------------
    // AC-001: PC1 unconditional timestamp re-stamp (no lock present)
    // -----------------------------------------------------------------------

    /// test_timestamp_always_restamped_no_lock_present (AC-001)
    ///
    /// BC-4.17.001 PC1: timestamp: re-stamped regardless of lock presence.
    /// Fixture: STATE.md with stale timestamp, no factory_lock block.
    /// Expected: write_file called; written content has timestamp = fixed_now;
    ///           no factory_lock block created.
    ///
    /// RED GATE: guard_logic is todo!() → panics before write_file can be called.
    #[test]
    fn test_timestamp_always_restamped_no_lock_present() {
        let content = state_no_lock_old_ts();
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("AC-001: write_file must be called — PC1 always stamps");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            written_str.contains("timestamp: 2026-08-27T12:00:00Z"),
            "AC-001 PC1: timestamp must be re-stamped to now (2026-08-27T12:00:00Z). \
             Got:\n{written_str:?}"
        );
        assert!(
            !written_str.contains("timestamp: 2020-01-01T00:00:00Z"),
            "AC-001 PC1: original stale timestamp must NOT remain"
        );
    }

    // -----------------------------------------------------------------------
    // AC-002: PC1 + Invariant 1 — timestamp re-stamped even with identity mismatch
    // -----------------------------------------------------------------------

    /// test_timestamp_restamped_when_lock_held_regardless_of_identity_match (AC-002)
    ///
    /// BC-4.17.001 PC1 / Invariant 1: timestamp re-stamp has no identity gate.
    /// Fixture: STATE.md with foreign holder (identity MISMATCH with caller).
    /// Expected: timestamp changed; expires_at byte-identical (mismatch → no renewal).
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_timestamp_restamped_when_lock_held_regardless_of_identity_match() {
        let original_expires = "2099-01-01T00:00:00Z";
        let content = state_with_foreign_lock(original_expires);
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Write"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // Identity MISMATCH: "caller@example.com" != "holder@example.com"
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            written_str.contains("timestamp: 2026-08-27T12:00:00Z"),
            "AC-002 Invariant 1: timestamp MUST be re-stamped even when identity mismatches. \
             Got:\n{written_str:?}"
        );
        assert!(
            written_str.contains(original_expires),
            "AC-002: expires_at must remain byte-identical on identity mismatch. \
             Got:\n{written_str:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-003: PC2 identity-gate row 1 — identity match renews expires_at
    // -----------------------------------------------------------------------

    /// test_identity_match_renews_expires_at (AC-003)
    ///
    /// BC-4.17.001 PC2: identity match → expires_at = now + TTL_SECONDS;
    /// holder + locked_at must remain byte-identical.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_identity_match_renews_expires_at() {
        let content = state_with_self_lock();
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();
        // Expected expires_at = fixed_now() + 2700s = 2026-08-27T12:45:00Z
        let expected_expires = (fixed_now() + Duration::seconds(2700))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();

        let _result = guard_logic(
            payload_for_post_tool_use("MultiEdit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // Identity MATCH: "caller@example.com" == holder "caller@example.com"
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            written_str.contains(&format!("expires_at: {expected_expires}")),
            "AC-003 PC2: expires_at must equal now + TTL_SECONDS ({expected_expires}). \
             Got:\n{written_str:?}"
        );
        // PC4: holder and locked_at must remain byte-identical
        assert!(
            written_str.contains("holder: \"caller@example.com\""),
            "AC-003 PC4: holder must remain byte-identical after renewal"
        );
        assert!(
            written_str.contains("locked_at: \"2026-01-01T10:00:00Z\""),
            "AC-003 PC4: locked_at must remain byte-identical after renewal"
        );
        // Stale expires_at must be gone
        assert!(
            !written_str.contains("2099-01-01T00:00:00Z"),
            "AC-003: stale fixture expires_at must be replaced by the renewed value"
        );
    }

    // -----------------------------------------------------------------------
    // AC-004: PC2 identity-gate row 2 — no lock block → no renewal attempted
    // -----------------------------------------------------------------------

    /// test_no_lock_block_skips_renewal_entirely (AC-004)
    ///
    /// BC-4.17.001 PC2 row 2: no factory_lock block → hook MUST NOT create one.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_no_lock_block_skips_renewal_entirely() {
        let content = state_no_lock_old_ts();
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            !written_str.contains("factory_lock:"),
            "AC-004 PC2: hook must NOT create a factory_lock block when none was present. \
             Got:\n{written_str:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-005: PC2 identity-gate row 3 — holder present but empty → no renewal
    // -----------------------------------------------------------------------

    /// test_empty_holder_skips_renewal (AC-005)
    ///
    /// BC-4.17.001 PC2 row 3: empty holder → no renewal; timestamp still re-stamped.
    /// parse_factory_lock returns Err(MalformedLockBlock) for empty holder;
    /// hook must treat this as "no renewal" (safe direction) while PC1 proceeds.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_empty_holder_skips_renewal() {
        let content = state_with_empty_holder();
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Write"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written.lock().unwrap().clone().expect(
            "write_file must be called — timestamp re-stamp proceeds even with empty holder",
        );
        let written_str = String::from_utf8_lossy(&written_bytes);
        // expires_at must remain unchanged (empty holder → no renewal)
        assert!(
            written_str.contains("2099-01-01T00:00:00Z"),
            "AC-005 PC2: expires_at must remain byte-identical when holder is empty. \
             Got:\n{written_str:?}"
        );
        // PC1 still proceeds: timestamp re-stamped
        assert!(
            written_str.contains("timestamp: 2026-08-27T12:00:00Z"),
            "AC-005: timestamp must still be re-stamped even when holder is empty (PC1 has no identity gate). \
             Got:\n{written_str:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-006: PC2 + Invariant 2 — identity mismatch, SAFETY-CRITICAL
    // -----------------------------------------------------------------------

    /// test_identity_mismatch_never_renews_expires_at (AC-006)
    ///
    /// BC-4.17.001 PC2 + Invariant 2 (SAFETY-CRITICAL):
    /// identity mismatch → expires_at NEVER renewed; foreign lock MUST NOT be resurrected.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_identity_mismatch_never_renews_expires_at() {
        let original_expires = "2099-06-01T00:00:00Z";
        let content = state_with_foreign_lock(original_expires);
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // "caller@example.com" != "holder@example.com" → MISMATCH
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            written_str.contains(original_expires),
            "AC-006 (SAFETY-CRITICAL) Invariant 2: expires_at must be byte-identical — \
             foreign holder must NEVER be renewed. Got:\n{written_str:?}"
        );
        let renewed = (fixed_now() + Duration::seconds(2700))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        assert!(
            !written_str.contains(&format!("expires_at: {renewed}")),
            "AC-006 (SAFETY-CRITICAL): expires_at must NOT be advanced to now+2700s on identity mismatch. \
             Got:\n{written_str:?}"
        );
    }

    /// test_lock_expired_admitted_non_holder_writer_never_renews (AC-006)
    ///
    /// BC-4.17.001 PC2 / Invariant 2 — BC-4.13.001-PC2-admission scenario (SAFETY-CRITICAL):
    /// a non-holder writer was admitted through an expired lock window (LockExpired path).
    /// The hook MUST NOT resurrect the dead lock's expires_at.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_lock_expired_admitted_non_holder_writer_never_renews() {
        // Expired lock: expires_at in the past → BC-4.13.001 PC2 LockExpired admission
        let expired_expires = "2020-01-01T00:45:00Z";
        let content = format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: 2020-01-01T00:00:00Z\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"holder@example.com\"\n",
                "  locked_at: \"2020-01-01T00:00:00Z\"\n",
                "  expires_at: \"{expires}\"\n",
                "---\n",
                "\n# STATE (expired-lock, non-holder writer admitted via LockExpired)\n",
            ),
            expires = expired_expires,
        );
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // Non-holder writer admitted via expired lock (different identity)
                exec_subprocess: |_argv| Ok((0, "newwriter@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        assert!(
            written_str.contains(expired_expires),
            "AC-006 (BC-4.13.001-PC2-admission, SAFETY-CRITICAL): \
             expired lock's expires_at must remain unchanged — NEVER resurrected. \
             Got:\n{written_str:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-007: PC2 + PC3 — identity-resolution failure → skip renewal, still stamp
    // -----------------------------------------------------------------------

    /// test_identity_resolution_failure_skips_renewal_but_timestamp_still_restamped (AC-007)
    ///
    /// BC-4.17.001 PC2 + PC3: exec_subprocess Err → expires_at unchanged;
    /// timestamp: STILL re-stamped (PC1 has no identity gate).
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_identity_resolution_failure_skips_renewal_but_timestamp_still_restamped() {
        let original_expires = "2099-01-01T00:00:00Z";
        let content = format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: 2020-01-01T00:00:00Z\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"holder@example.com\"\n",
                "  locked_at: \"2026-01-01T10:00:00Z\"\n",
                "  expires_at: \"{expires}\"\n",
                "---\n",
                "\n# STATE\n",
            ),
            expires = original_expires,
        );
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Write"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // Identity resolution FAILURE
                exec_subprocess: |_argv| Err("git config user.email failed".to_string()),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written.lock().unwrap().clone().expect(
            "write_file must be called — timestamp re-stamp proceeds even on identity-resolution failure",
        );
        let written_str = String::from_utf8_lossy(&written_bytes);
        // PC2 gate failed → expires_at unchanged
        assert!(
            written_str.contains(original_expires),
            "AC-007 PC2+PC3: expires_at must be unchanged when identity resolution fails. \
             Got:\n{written_str:?}"
        );
        // PC1 unaffected → timestamp still re-stamped
        assert!(
            written_str.contains("timestamp: 2026-08-27T12:00:00Z"),
            "AC-007: timestamp must still be re-stamped even on identity-resolution failure \
             (PC1 has no identity gate). Got:\n{written_str:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008: PC3 fail-open — various structural error conditions → zero writes
    // -----------------------------------------------------------------------

    /// test_malformed_frontmatter_writes_nothing (AC-008)
    ///
    /// BC-4.17.001 PC3: malformed frontmatter (no closing --- delimiter) →
    /// hook writes NOTHING (fail-open; agent's write is not touched).
    ///
    /// RED GATE: guard_logic is todo!() → panics before any write_file invocation.
    #[test]
    fn test_malformed_frontmatter_writes_nothing() {
        let content = state_malformed_no_closing_delimiter();
        let write_called = Arc::new(Mutex::new(false));
        let wc = write_called.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, _bytes| {
                    *wc.lock().unwrap() = true;
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        assert!(
            !*write_called.lock().unwrap(),
            "AC-008 PC3: write_file must NOT be called when frontmatter is malformed (fail-open)"
        );
    }

    /// test_read_file_error_writes_nothing (AC-008)
    ///
    /// BC-4.17.001 PC3: read_file returns Err → hook writes NOTHING.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_read_file_error_writes_nothing() {
        let write_called = Arc::new(Mutex::new(false));
        let wc = write_called.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: |_path, _max, _timeout| {
                    Err("HostError: read_file capability denied".to_string())
                },
                write_file: move |_path, _bytes| {
                    *wc.lock().unwrap() = true;
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        assert!(
            !*write_called.lock().unwrap(),
            "AC-008 PC3: write_file must NOT be called when read_file fails (fail-open)"
        );
    }

    /// test_non_utf8_content_writes_nothing (AC-008)
    ///
    /// BC-4.17.001 PC3: non-UTF-8 bytes in file content → hook writes NOTHING.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_non_utf8_content_writes_nothing() {
        // Valid ASCII frontmatter followed by invalid UTF-8 bytes in the body region.
        let invalid_utf8: Vec<u8> =
            b"---\ntimestamp: 2020-01-01T00:00:00Z\nphase: test\n---\n\xFF\xFE".to_vec();
        let write_called = Arc::new(Mutex::new(false));
        let wc = write_called.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Write"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(invalid_utf8),
                write_file: move |_path, _bytes| {
                    *wc.lock().unwrap() = true;
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        assert!(
            !*write_called.lock().unwrap(),
            "AC-008 PC3: write_file must NOT be called when content is non-UTF-8 (fail-open)"
        );
    }

    /// test_missing_timestamp_anchor_writes_nothing (AC-008)
    ///
    /// BC-4.17.001 PC3: valid frontmatter delimiters but no `timestamp:` anchor line →
    /// hook writes NOTHING (no line to replace → fail-open).
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_missing_timestamp_anchor_writes_nothing() {
        let content = state_no_timestamp_line();
        let write_called = Arc::new(Mutex::new(false));
        let wc = write_called.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, _bytes| {
                    *wc.lock().unwrap() = true;
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        assert!(
            !*write_called.lock().unwrap(),
            "AC-008 PC3: write_file must NOT be called when no timestamp: anchor line is present (fail-open)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-009: PC3 — no accumulated failure state across invocations
    // -----------------------------------------------------------------------

    /// test_failure_then_success_is_independent_per_invocation (AC-009)
    ///
    /// BC-4.17.001 PC3: hook is stateless per-invocation.
    /// After a fail-open invocation (read error), the next well-formed invocation
    /// MUST proceed normally — hook MUST NOT carry poisoned state.
    ///
    /// RED GATE: guard_logic is todo!() → panics on the first call.
    #[test]
    fn test_failure_then_success_is_independent_per_invocation() {
        // First invocation: read_file fails → fail-open, no write
        let _first = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: |_path, _max, _timeout| Err("HostError: transient failure".to_string()),
                write_file: |_path, _bytes| Ok(()),
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        // Second invocation: valid content → write MUST proceed normally
        let content = state_no_lock_old_ts();
        let write_called = Arc::new(Mutex::new(false));
        let wc = write_called.clone();
        let _second = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, _bytes| {
                    *wc.lock().unwrap() = true;
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );
        assert!(
            *write_called.lock().unwrap(),
            "AC-009 PC3: second invocation must write normally — no carry-over from prior failure"
        );
    }

    // -----------------------------------------------------------------------
    // AC-010: PC4 idempotent frontmatter-only rewrite
    // -----------------------------------------------------------------------

    /// test_rewrite_touches_only_two_frontmatter_lines (AC-010)
    ///
    /// BC-4.17.001 PC4: at most 2 lines changed (timestamp: + expires_at:).
    /// Fixture: STATE.md with multiple frontmatter lines, identity match → both lines change.
    /// Expected: exactly 2 lines differ between original and written content.
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_rewrite_touches_only_two_frontmatter_lines() {
        let content = state_with_self_lock();
        let original_bytes = content.as_bytes().to_vec();
        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("MultiEdit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                // Identity match → both timestamp + expires_at change
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let orig_str = std::str::from_utf8(&original_bytes).expect("original must be valid UTF-8");
        let written_str = std::str::from_utf8(&written_bytes).expect("written must be valid UTF-8");
        let orig_lines: Vec<&str> = orig_str.lines().collect();
        let written_lines: Vec<&str> = written_str.lines().collect();

        assert_eq!(
            orig_lines.len(),
            written_lines.len(),
            "AC-010 PC4: rewrite must not add or remove lines — only in-place replacement"
        );

        let changed: Vec<usize> = orig_lines
            .iter()
            .zip(written_lines.iter())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, _)| i)
            .collect();

        assert!(
            changed.len() <= 2,
            "AC-010 PC4: at most 2 lines must change (timestamp: + expires_at:). \
             Got {} changes at line indices {:?}",
            changed.len(),
            changed
        );
        assert!(
            !changed.is_empty(),
            "AC-010 PC4: at least 1 line must change (timestamp: always re-stamped)"
        );
    }

    /// test_body_content_after_closing_delimiter_never_modified (AC-010)
    ///
    /// BC-4.17.001 PC4 + Invariant 5: bytes after the closing `---` delimiter
    /// must be byte-identical (body never read for decision purposes or modified).
    ///
    /// RED GATE: guard_logic is todo!() → panics.
    #[test]
    fn test_body_content_after_closing_delimiter_never_modified() {
        let content = state_with_self_lock();
        // Extract expected body: everything after the closing `---\n`
        let body_start = content
            .find("\n---\n")
            .map(|pos| pos + "\n---\n".len())
            .unwrap_or(content.len());
        let expected_body = content[body_start..].to_string();

        let written = Arc::new(Mutex::new(None::<Vec<u8>>));
        let w = written.clone();

        let _result = guard_logic(
            payload_for_post_tool_use("Edit"),
            StampCallbacks {
                read_file: move |_path, _max, _timeout| Ok(content.into_bytes()),
                write_file: move |_path, bytes| {
                    *w.lock().unwrap() = Some(bytes.to_vec());
                    Ok(())
                },
                exec_subprocess: |_argv| Ok((0, "caller@example.com\n".to_string())),
                now_fn: fixed_now,
            },
        );

        let written_bytes = written
            .lock()
            .unwrap()
            .clone()
            .expect("write_file must be called");
        let written_str = String::from_utf8_lossy(&written_bytes);
        let written_body_start = written_str
            .find("\n---\n")
            .map(|pos| pos + "\n---\n".len())
            .unwrap_or(written_str.len());
        let written_body = &written_str[written_body_start..];

        assert_eq!(
            written_body, expected_body,
            "AC-010 Invariant 5: body content after closing --- must be byte-identical. \
             Got:\n{written_body:?}"
        );
    }

    // -----------------------------------------------------------------------
    // AC-012: TTL_SECONDS canonical constant — import check
    // -----------------------------------------------------------------------

    /// test_ttl_seconds_constant_equals_2700 (AC-012, stamp-state-timestamp crate)
    ///
    /// BC-4.17.001 Precondition 3 / Invariant 3 / ADR-046 F-006:
    /// factory_lock_parse::TTL_SECONDS must equal 2700 (canonical factory_lock TTL).
    /// This crate imports it; the value must be correct before guard_logic can be used.
    ///
    /// RED GATE: factory_lock_parse::TTL_SECONDS = 0 (stub) → assert_eq fails.
    #[test]
    fn test_ttl_seconds_constant_equals_2700() {
        assert_eq!(
            flp::TTL_SECONDS,
            2700u32,
            "AC-012 BC-4.17.001 Precondition 3 / Invariant 3: factory_lock_parse::TTL_SECONDS \
             must equal 2700. Stub has 0 — S-17.05 T-2 must set to 2700."
        );
    }

    /// test_ttl_seconds_is_imported_not_redeclared (AC-012, source-scan)
    ///
    /// Architecture Compliance Rule 7: stamp-state-timestamp/src/lib.rs MUST NOT contain
    /// a second `2700`-valued literal or const in production code — TTL_SECONDS is imported
    /// from factory-lock-parse, never redeclared here.
    ///
    /// Source-scan: reads stamp-state-timestamp/src/lib.rs, strips the test module and
    /// comment-only lines, then asserts no occurrence of `= 2700` (which would indicate
    /// a redeclaration of the constant or a bare literal assignment).
    ///
    /// NOTE: This test PASSES in the stub phase (no `2700` literal in the stub).
    /// It is a stay-green correctness guard for the implementation phase — fails if
    /// the implementer accidentally redeclares the literal.
    #[test]
    fn test_ttl_seconds_is_imported_not_redeclared() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lib_path = std::path::Path::new(manifest_dir).join("src/lib.rs");
        let source = std::fs::read_to_string(&lib_path)
            .unwrap_or_else(|e| panic!("failed to read {:?}: {}", lib_path, e));

        // Restrict scan to production code only: strip the test module.
        let non_test_source = if let Some(test_mod_pos) = source.find("#[cfg(test)]\nmod tests") {
            &source[..test_mod_pos]
        } else {
            source.as_str()
        };

        // Collect non-comment production lines containing `= 2700`.
        // A violation would be e.g. `const TTL_SECONDS: u32 = 2700;` (redeclaration) or
        // `let ttl = 2700;` (bare literal). Doc comments (`///`) are excluded (start with `//`).
        let violations: Vec<&str> = non_test_source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter(|line| line.contains("= 2700"))
            .collect();

        assert!(
            violations.is_empty(),
            "AC-012 Architecture Compliance Rule 7: stamp-state-timestamp/src/lib.rs \
             must NOT redeclare TTL_SECONDS as a literal `= 2700`. \
             Import factory_lock_parse::TTL_SECONDS instead. \
             Found in {:?}:\n{:?}",
            lib_path,
            violations
        );
    }
}
