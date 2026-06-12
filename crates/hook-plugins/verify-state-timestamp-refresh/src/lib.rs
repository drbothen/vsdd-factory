//! verify-state-timestamp-refresh — PreToolUse WASM hook plugin (D16, S-17.04).
//!
//! Enforces BC-5.40.001 PC4 (mid-burst TTL renewal) at write-time.
//!
//! On each invocation the guard:
//!   1. Checks `file_path` in the tool payload. If NOT `.factory/STATE.md`:
//!      return `Continue` immediately — zero overhead for all other files (AC-007).
//!   2. Reads the on-disk `.factory/STATE.md` via `host::read_file`.
//!      On error (HostError): return `Continue` (fail-open per §12.3).
//!   3. Extracts `timestamp:` from both proposed content (payload `new_content`)
//!      and the on-disk content.
//!   4. If `timestamp:` is absent in proposed content → Block: TimestampStale.
//!   5. If `timestamp:` is absent in on-disk content → Continue (first write ever).
//!   6. If `timestamp:` values are byte-identical → Block: TimestampStale.
//!   7. If a lock is held in proposed content (`factory_lock.holder` present and
//!      non-empty): compare `factory_lock.expires_at` byte-for-byte.
//!      If byte-identical → Block: LockExpiryStale.
//!   8. All other paths → Continue.
//!
//! Fail-open error paths (AC-008 / ADR-025 §12.3):
//!   - Proposed content unparseable → Continue
//!   - On-disk read fails → Continue
//!   - `timestamp:` absent in on-disk → Continue
//!   - Plugin crash (on_error = continue) → Continue
//!
//! # Behavioral Contracts
//!
//! - BC-5.40.001: STATE.md factory_lock schema + TTL + mid-burst renewal + state-burst CAS push.
//!   PC4 (mid-burst renewal) is the primary enforcement target.
//!   PC6 (single-dev zero friction) mandates fail-open on all error paths.
//!
//! # Architecture compliance (ADR-025 Decision 12)
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - No `serde_yaml` / `serde_norway` — manual line-by-line scan via `factory-lock-parse`.
//! - No `regex` crate — manual tokenisation only.
//! - `async = false` REQUIRED in registry entry (ADR-019; ADR-025 Decision 12).
//! - Guard is read-only: NEVER writes STATE.md (Invariant 4 from verify-factory-lock pattern).
//! - No `exec_subprocess` — reads proposed content from payload, on-disk via `host::read_file`.
//! - Pure `fn guard_logic(...)` takes all host I/O as injectable callbacks;
//!   unit tests exercise every branch without a WASM runtime.
//! - Trigger: `file_path == ".factory/STATE.md"` (exact path; bypass-proof per §12.1).

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md via `host::read_file`.
/// 64 KiB is sufficient; mirrors `verify-factory-lock` (ADR-025 Decision 12 §12.5).
pub const STATE_MD_MAX_BYTES: u32 = 65536;

/// Timeout in milliseconds for the `host::read_file` call.
pub const READ_FILE_TIMEOUT_MS: u32 = 5000;

/// Canonical path of STATE.md — exact string comparison in WASM (ADR-025 §12.1).
pub const STATE_MD_PATH: &str = ".factory/STATE.md";

// ---------------------------------------------------------------------------
// Injectable callbacks surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting host calls injected into `guard_logic` for testability.
/// In production (`main.rs`), these are wired to real vsdd_hook_sdk host fns.
pub struct GuardCallbacks<R, L>
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
{
    /// Read a file by path with `(path, max_bytes, timeout_ms)`.
    /// Returns `Ok(bytes)` or `Err(host_error_description)` on failure.
    pub read_file: R,
    /// Emit a `host::log_warn` message (advisory; non-blocking).
    pub log_warn: L,
}

// ---------------------------------------------------------------------------
// Helper: extract a top-level YAML frontmatter field value
// ---------------------------------------------------------------------------

/// Result of attempting to extract a top-level frontmatter field.
///
/// Distinguishes between:
/// - `Found(value)`: the key exists and has a value
/// - `NotFound`: the frontmatter is well-formed but the key is absent
/// - `Malformed`: the frontmatter structure is unparseable (no closing `---`)
#[derive(Debug)]
enum FieldResult {
    Found(String),
    NotFound,
    Malformed,
}

/// Extract a top-level `key: value` from STATE.md frontmatter content.
///
/// Scans only the region between the first and second `---\n` delimiters
/// (no YAML parser; no `regex` crate — Architecture Compliance Rule 4).
/// Uses `factory_lock_parse::extract_yaml_string_value` for the per-line scan.
///
/// Returns:
/// - `FieldResult::Found(value)` if the key is found in the frontmatter.
/// - `FieldResult::NotFound` if the frontmatter is well-formed but key is absent.
/// - `FieldResult::Malformed` if the frontmatter is unparseable (no closing `---`
///   delimiter). Callers must fail-open on Malformed (AC-008 §12.3 row 1).
fn extract_top_level_field(content: &str, key: &str) -> FieldResult {
    // Normalise CRLF.
    let normalised;
    let content = if content.contains('\r') {
        normalised = content.replace("\r\n", "\n");
        normalised.as_str()
    } else {
        content
    };

    // No opening `---\n` — treat as no-frontmatter (not malformed, just absent).
    let after_open = match content.strip_prefix("---\n") {
        Some(rest) => rest,
        None => return FieldResult::NotFound,
    };

    // Find the closing `---` delimiter. Absent → Malformed.
    let frontmatter_end = match after_open.find("\n---\n").or_else(|| {
        if after_open.ends_with("\n---") {
            Some(after_open.len() - 4)
        } else {
            None
        }
    }) {
        Some(pos) => pos,
        None => return FieldResult::Malformed,
    };

    let frontmatter = &after_open[..frontmatter_end];

    for line in frontmatter.lines() {
        // Only scan top-level lines (not indented sub-fields).
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        if let Some(value) = factory_lock_parse::extract_yaml_string_value(line, key) {
            return FieldResult::Found(value);
        }
    }
    FieldResult::NotFound
}

// ---------------------------------------------------------------------------
// Core guard logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core verify-state-timestamp-refresh guard logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Decision tree (per ADR-025 Decision 12 / BC-5.40.001 PC4+PC6):
///   1. Extract `file_path` from payload. If NOT `.factory/STATE.md`:
///      return Continue immediately — zero overhead (AC-007 / §12.1).
///   2. Extract `new_content` (proposed write) from payload.
///      If absent or not a string: fail-open → return Continue.
///   3. Extract `timestamp:` from proposed content.
///      If absent → Block: TimestampStale (§12.3 row 6 / EC-005).
///   4. Read on-disk STATE.md via `read_file`.
///      If error → Continue (fail-open §12.3 row 2).
///   5. Extract `timestamp:` from on-disk content.
///      If absent → Continue (first write ever, §12.3 row 5 / EC-004).
///   6. If proposed `timestamp:` byte-identical to on-disk → Block: TimestampStale (§12.2).
///   7. If lock held in proposed content (factory_lock.holder present + non-empty):
///      extract `factory_lock.expires_at` from proposed and on-disk.
///      If byte-identical → Block: LockExpiryStale (§12.2).
///   8. All other paths → Continue.
///
/// # BC traces
/// - BC-5.40.001 PC4: TimestampStale block / LockExpiryStale block
/// - BC-5.40.001 PC6: fail-open on all error paths
/// - ADR-025 Decision 12 §12.1: file_path trigger (bypass-proof)
/// - ADR-025 Decision 12 §12.2: byte-comparison, not datetime parse
/// - ADR-025 Decision 12 §12.3: fail-open table
pub fn guard_logic<R, L>(payload: HookPayload, mut callbacks: GuardCallbacks<R, L>) -> HookResult
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
{
    // Step 1: Check file_path. If not STATE.md, return Continue immediately (AC-007 / §12.1).
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Normalize leading "./" (EC-006: path normalization).
    let normalized_path = file_path.strip_prefix("./").unwrap_or(file_path);
    // Also strip double-slashes (EC-006).
    let target_path = if normalized_path == STATE_MD_PATH {
        normalized_path
    } else {
        // Not STATE.md — return Continue immediately without reading any file (AC-007).
        return HookResult::Continue;
    };

    // Step 2: Extract proposed content (new_content field from payload).
    let proposed_content = match payload
        .tool_input
        .get("new_content")
        .and_then(|v| v.as_str())
    {
        Some(s) => s.to_string(),
        None => {
            // Absent new_content: fail-open (cannot validate what we cannot read).
            (callbacks.log_warn)("TimestampRefresh: new_content absent in payload — fail-open");
            return HookResult::Continue;
        }
    };

    // Step 3: Extract timestamp: from proposed content.
    // - Malformed frontmatter → Continue (fail-open, AC-008 §12.3 row 1).
    // - Absent timestamp: in well-formed proposed content → Block: TimestampStale (§12.3 row 6).
    let proposed_ts = match extract_top_level_field(&proposed_content, "timestamp") {
        FieldResult::Found(ts) => ts,
        FieldResult::NotFound => {
            // timestamp: absent in proposed write — missing-field violation.
            return HookResult::block_with_fix(
                "verify-state-timestamp-refresh",
                "TimestampStale: STATE.md timestamp not advanced in this write",
                "Update timestamp to the current UTC time before writing STATE.md",
                "TimestampStale",
            );
        }
        FieldResult::Malformed => {
            // Unparseable proposed content — fail-open (AC-008 §12.3 row 1).
            (callbacks.log_warn)(
                "TimestampRefresh: proposed content has malformed frontmatter — fail-open",
            );
            return HookResult::Continue;
        }
    };

    // Step 4: Read on-disk STATE.md. On error: fail-open (§12.3 row 2).
    let on_disk_bytes =
        match (callbacks.read_file)(target_path, STATE_MD_MAX_BYTES, READ_FILE_TIMEOUT_MS) {
            Ok(bytes) => bytes,
            Err(e) => {
                (callbacks.log_warn)(&format!(
                    "TimestampRefresh: read_file failed — fail-open: {}",
                    e
                ));
                return HookResult::Continue;
            }
        };

    let on_disk_content = match String::from_utf8(on_disk_bytes) {
        Ok(s) => s,
        Err(e) => {
            (callbacks.log_warn)(&format!(
                "TimestampRefresh: on-disk STATE.md is not valid UTF-8 — fail-open: {}",
                e
            ));
            return HookResult::Continue;
        }
    };

    // Step 5: Extract timestamp: from on-disk content.
    // Absent (or malformed) → Continue (first write ever or unreadable, §12.3 row 5 / EC-004).
    let on_disk_ts = match extract_top_level_field(&on_disk_content, "timestamp") {
        FieldResult::Found(ts) => ts,
        FieldResult::NotFound | FieldResult::Malformed => {
            // No prior timestamp to compare against — any write is valid.
            return HookResult::Continue;
        }
    };

    // Step 6: Byte-identical timestamp: → Block: TimestampStale (§12.2).
    if proposed_ts == on_disk_ts {
        return HookResult::block_with_fix(
            "verify-state-timestamp-refresh",
            "TimestampStale: STATE.md timestamp not advanced in this write",
            "Update timestamp to the current UTC time before writing STATE.md",
            "TimestampStale",
        );
    }

    // Step 7: If lock held in proposed content, check factory_lock.expires_at.
    // "Lock held" = factory_lock.holder present and non-empty in PROPOSED content.
    let lock_in_proposed = factory_lock_parse::parse_factory_lock(&proposed_content);
    let proposed_lock = match lock_in_proposed {
        Ok(Some(lock)) if !lock.holder.is_empty() => Some(lock),
        _ => None, // No lock held, or parse error → skip LockExpiryStale check.
    };

    if let Some(lock) = proposed_lock {
        // Lock is held in proposed content. Check expires_at byte-identity.
        let on_disk_lock = factory_lock_parse::parse_factory_lock(&on_disk_content);
        if let Ok(Some(on_disk_lock_state)) = on_disk_lock
            && lock.expires_at == on_disk_lock_state.expires_at
        {
            return HookResult::block_with_fix(
                "verify-state-timestamp-refresh",
                "LockExpiryStale: factory_lock.expires_at not refreshed in this write while lock is held",
                "Run: factory-lock-write.sh renew .factory/STATE.md before committing",
                "LockExpiryStale",
            );
        }
        // If on-disk has no lock or parse fails: fail-open (no prior expires_at to compare).
    }

    // Step 8: All checks passed → Continue.
    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `guard_logic`.
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
            log_warn: |msg| {
                host::log_warn(msg);
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests — Red Gate (D17 / S-17.04 v1.2 Red Gate Test Table)
//
// 9 Rust unit tests from ADR-025 §12.6 (table-driven cases a..i).
// Uses injectable callbacks so no WASM runtime is required.
//
// RED GATE: The stub `guard_logic` returns `Continue` unconditionally AND
// calls `read_file` unconditionally (no file_path check). Against this stub:
//
//   FAIL (assertion errors — correct Red Gate):
//     (a) test_lock_expiry_stale_blocks          — expects Block(LockExpiryStale)
//     (b) test_timestamp_stale_no_lock_blocks    — expects Block(TimestampStale)
//     (c) test_timestamp_stale_lock_held_blocks  — expects Block(TimestampStale)
//     (i) test_timestamp_absent_in_proposed_blocks — expects Block(TimestampStale)
//     (g) test_non_state_md_file_continues_without_read — expects Continue + read_file_calls==0;
//         stub calls read_file once → read_file call-count assertion fails
//
//   PASS (trivially satisfied by the all-Continue stub — acceptable):
//     (e) test_proposed_unparseable_continues       — expects Continue
//     (f) test_on_disk_read_fails_continues         — expects Continue
//     (h) test_timestamp_absent_on_disk_continues   — expects Continue
//         (no lock path + Continue → passes against stub)
//
// Net Red Gate: 5 FAILING tests (4 Block-assertions + 1 read_file-call assertion).
// The 3 Continue-only tests passing against the stub are structurally correct:
// the stub is fail-open and these test fail-open paths — they will remain green
// after implementation (the real guard also returns Continue for these paths).
// The test_no_lock_held_skips_expiry_check is a Continue-assertion and passes —
// see the note in that test's comment.
//
// Canonical STATE.md frontmatter fixture (used across tests):
//
//   timestamp: "2026-06-11T10:00:00Z"
//   factory_lock:
//     holder: "dev@example.com"
//     locked_at: "2026-06-11T10:00:00Z"
//     expires_at: "2026-06-11T10:45:00Z"
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

    use super::*;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    // -----------------------------------------------------------------------
    // Fixture constants
    // -----------------------------------------------------------------------

    const TS_OLD: &str = "2026-06-11T10:00:00Z";
    const TS_NEW: &str = "2026-06-11T11:00:00Z";
    const EXPIRES_OLD: &str = "2026-06-11T10:45:00Z";
    const EXPIRES_NEW: &str = "2026-06-11T11:45:00Z";
    const HOLDER: &str = "dev@example.com";

    // -----------------------------------------------------------------------
    // Fixture builders
    // -----------------------------------------------------------------------

    /// Build a HookPayload for an Edit/Write to STATE.md with the given new_content.
    fn payload_state_md(new_content: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Edit",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": ".factory/STATE.md",
                "new_content": new_content
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build a HookPayload for an Edit/Write to a non-STATE.md path.
    fn payload_non_state_md(file_path: &str, new_content: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Edit",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": file_path,
                "new_content": new_content
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build STATE.md content with a given timestamp, no lock.
    fn state_md_no_lock(timestamp: &str) -> String {
        format!(
            "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE\n",
            timestamp
        )
    }

    /// Build STATE.md content with a given timestamp and a lock block.
    fn state_md_with_lock(timestamp: &str, expires_at: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: \"{ts}\"\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"{holder}\"\n",
                "  locked_at: \"2026-06-11T10:00:00Z\"\n",
                "  expires_at: \"{exp}\"\n",
                "---\n\n# STATE\n",
            ),
            ts = timestamp,
            holder = HOLDER,
            exp = expires_at,
        )
    }

    /// Build STATE.md content with NO timestamp field (simulates first-ever write).
    fn state_md_no_timestamp() -> String {
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\n---\n\n# STATE\n"
            .to_string()
    }

    /// Build malformed frontmatter content (no closing `---`).
    fn state_md_malformed() -> String {
        "---\ndocument_type: state\nversion: broken".to_string()
    }

    // -----------------------------------------------------------------------
    // Callback builders
    // -----------------------------------------------------------------------

    /// Build callbacks where read_file returns `on_disk_content` and log_warn
    /// records to `warn_log`.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_with_disk(
        on_disk_content: String,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>, impl FnMut(&str)>
    {
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(on_disk_content.into_bytes()),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    /// Build callbacks where read_file returns an error.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_read_error(
        error_msg: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>, impl FnMut(&str)>
    {
        let err = error_msg.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Err(err),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    // -----------------------------------------------------------------------
    // Case (a) — lock held + expires_at unchanged → Block: LockExpiryStale
    // Traces to: AC-006 / ADR-025 D17(a) / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// (a) Lock held + `factory_lock.expires_at` byte-identical → Block(LockExpiryStale).
    ///
    /// Proposed content: timestamp ADVANCED (new), expires_at UNCHANGED (old).
    /// On-disk: timestamp old, expires_at old.
    ///
    /// Expected: Block with "LockExpiryStale" in the reason.
    ///
    /// RED GATE: stub returns Continue → assertion fails.
    #[test]
    fn test_lock_expiry_stale_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: timestamp advanced, but expires_at NOT advanced.
        let proposed = state_md_with_lock(TS_NEW, EXPIRES_OLD);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("LockExpiryStale"),
                    "Block reason must contain 'LockExpiryStale'. Got: {reason}"
                );
            }
            HookResult::Continue => panic!(
                "test_lock_expiry_stale_blocks: expected Block(LockExpiryStale) but got Continue. \
                 Stub returns Continue unconditionally — this is the expected Red Gate failure. \
                 Implementer: add LockExpiryStale check when lock held + expires_at byte-identical."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // Case (b) — lock held + expires_at advanced → Continue
    // Traces to: AC-003 / ADR-025 D17(b) / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// (b) Lock held + both timestamp and expires_at advanced → Continue.
    ///
    /// This is the success path: state-manager ran renew, both fields are fresh.
    ///
    /// RED GATE: stub returns Continue → assertion PASSES.
    /// This test will remain green after implementation (correct renew path).
    #[test]
    fn test_lock_held_both_advanced_continues() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: both timestamp AND expires_at advanced.
        let proposed = state_md_with_lock(TS_NEW, EXPIRES_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Lock held + both fields advanced must return Continue (success path / AC-003)"
        );
    }

    // -----------------------------------------------------------------------
    // Case (c) — no lock + timestamp unchanged → Block: TimestampStale
    // Traces to: AC-005 / ADR-025 D17(c) / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// (c) No lock held + `timestamp:` byte-identical to on-disk → Block(TimestampStale).
    ///
    /// Expected: Block with "TimestampStale" in the reason.
    ///
    /// RED GATE: stub returns Continue → assertion fails.
    #[test]
    fn test_timestamp_stale_no_lock_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: timestamp NOT advanced (same value as on-disk).
        let proposed = state_md_no_lock(TS_OLD);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("TimestampStale"),
                    "Block reason must contain 'TimestampStale'. Got: {reason}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_stale_no_lock_blocks: expected Block(TimestampStale) but got Continue. \
                 Stub returns Continue unconditionally — Red Gate. \
                 Implementer: add TimestampStale check when timestamp byte-identical to on-disk."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // Case (d) — no lock + timestamp advanced → Continue
    // Traces to: AC-003 / ADR-025 D17(d) / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// (d) No lock held + timestamp advanced → Continue (clean write path).
    ///
    /// RED GATE: stub returns Continue → assertion PASSES.
    #[test]
    fn test_timestamp_advanced_no_lock_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "No lock + timestamp advanced must return Continue (AC-003 success path)"
        );
    }

    // -----------------------------------------------------------------------
    // Case (e) — proposed content unparseable → Continue (fail-open)
    // Traces to: AC-008 / ADR-025 §12.3 / BC-5.40.001 PC6
    // -----------------------------------------------------------------------

    /// (e) Proposed content is malformed frontmatter → Continue (fail-open).
    ///
    /// RED GATE: stub returns Continue → assertion PASSES.
    /// This is structurally correct — the stub's unconditional Continue IS the
    /// fail-open path. The test remains green after implementation.
    #[test]
    fn test_proposed_unparseable_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_malformed();

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Unparseable proposed content must return Continue (fail-open, AC-008)"
        );
    }

    // -----------------------------------------------------------------------
    // Case (f) — on-disk read fails → Continue (fail-open)
    // Traces to: AC-008 / ADR-025 §12.3 / BC-5.40.001 PC6
    // -----------------------------------------------------------------------

    /// (f) `host::read_file` returns HostError → Continue (fail-open).
    ///
    /// RED GATE: stub calls read_file and ignores its error, returns Continue.
    /// Assertion PASSES (stub is fail-open by coincidence).
    /// Real implementation must also fail-open — this remains green.
    #[test]
    fn test_on_disk_read_fails_continues() {
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_read_error("HostError: Timeout", warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "On-disk read failure must return Continue (fail-open, AC-008)"
        );
    }

    // -----------------------------------------------------------------------
    // Case (g) — file_path NOT STATE.md → Continue immediately, NO read_file call
    // Traces to: AC-007 / ADR-025 §12.1 / BC-5.40.001 PC6
    // -----------------------------------------------------------------------

    /// (g) `file_path` is NOT `.factory/STATE.md` → Continue immediately without
    /// calling `read_file` (AC-007 / ADR-025 §12.1).
    ///
    /// The test verifies via a call-counting mock: if `read_file` is called even once,
    /// the assertion `read_file_calls == 0` fails.
    ///
    /// RED GATE: stub calls `read_file` unconditionally (no path check) → call-count
    /// assertion fails. This is the critical non-trivial Red Gate for the non-STATE.md path.
    #[test]
    fn test_non_state_md_file_continues_without_read() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let read_call_count = Arc::new(Mutex::new(0u32));
        let read_count_clone = read_call_count.clone();
        let wl = warn_log.clone();

        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| {
                *read_count_clone.lock().unwrap() += 1;
                // Returns any valid content — the point is to count the call.
                Ok(b"some other file content".to_vec())
            },
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        };

        // Non-STATE.md path.
        let payload = payload_non_state_md(".factory/specs/some-spec.md", "# Some spec content\n");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Non-STATE.md file must return Continue (AC-007)"
        );

        let calls = *read_call_count.lock().unwrap();
        assert_eq!(
            calls, 0,
            "Non-STATE.md file must NOT call read_file (zero-overhead path per AC-007). \
             read_file was called {} time(s). \
             RED GATE: stub does not check file_path and calls read_file unconditionally.",
            calls
        );
    }

    // -----------------------------------------------------------------------
    // Case (h) — timestamp absent in on-disk → Continue (first write ever)
    // Traces to: AC-008 / ADR-025 §12.3 row 5 / BC-5.40.001 PC6
    // -----------------------------------------------------------------------

    /// (h) `timestamp:` absent in on-disk STATE.md → Continue (first write ever, EC-004).
    ///
    /// No prior value to compare against; any write is valid.
    ///
    /// RED GATE: stub returns Continue → assertion PASSES.
    /// Remains green after implementation.
    #[test]
    fn test_timestamp_absent_on_disk_continues() {
        // On-disk: no timestamp field (brand-new repo, first write).
        let on_disk = state_md_no_timestamp();
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "Absent on-disk timestamp must return Continue (first write ever / AC-008 row 3)"
        );
    }

    // -----------------------------------------------------------------------
    // Case (i) — timestamp absent in proposed content → Block: TimestampStale
    // Traces to: AC-008 / ADR-025 §12.3 row 6 / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// (i) `timestamp:` absent in proposed content → Block(TimestampStale).
    ///
    /// Absence of `timestamp:` in the proposed write is itself a missing-field
    /// violation — state-manager is required to include `timestamp:` on every write
    /// (POLICY 14). Block even when on-disk has a timestamp.
    ///
    /// Expected: Block with "TimestampStale" in the reason.
    ///
    /// RED GATE: stub returns Continue → assertion fails.
    #[test]
    fn test_timestamp_absent_in_proposed_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: NO timestamp field.
        let proposed = state_md_no_timestamp();

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("TimestampStale"),
                    "Block reason must contain 'TimestampStale' for absent proposed timestamp. Got: {reason}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_absent_in_proposed_blocks: expected Block(TimestampStale) but got Continue. \
                 Stub returns Continue unconditionally — Red Gate. \
                 Implementer: absence of timestamp: in proposed content is a TimestampStale violation."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 additional test — lock held, timestamp stale → Block: TimestampStale
    // Traces to: AC-005 / ADR-025 D17(c) with lock / BC-5.40.001 PC4
    // -----------------------------------------------------------------------

    /// Lock held + timestamp byte-identical (NOT advanced) → Block(TimestampStale).
    ///
    /// The TimestampStale check applies regardless of lock state.
    /// Even if expires_at is advanced, a stale timestamp still triggers Block.
    ///
    /// RED GATE: stub returns Continue → assertion fails.
    #[test]
    fn test_timestamp_stale_lock_held_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: timestamp NOT advanced; expires_at advanced (to ensure only timestamp
        // triggers the block, not the expiry check).
        let proposed = state_md_with_lock(TS_OLD, EXPIRES_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        match result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("TimestampStale"),
                    "Block reason must contain 'TimestampStale'. Got: {reason}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_stale_lock_held_blocks: expected Block(TimestampStale) but got Continue. \
                 Red Gate — stub returns Continue unconditionally."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-006 additional test — no lock held, skips LockExpiryStale check
    // Traces to: AC-006 / ADR-025 §12.3 row 3 / BC-5.40.001 PC6
    // -----------------------------------------------------------------------

    /// No lock held in proposed content → LockExpiryStale check MUST NOT fire.
    ///
    /// Even if on-disk had a lock with some expires_at, when the proposed content
    /// has no lock block, the LockExpiryStale check does not apply.
    /// The TimestampStale check still applies — but here timestamp IS advanced.
    ///
    /// RED GATE: stub returns Continue → assertion PASSES.
    /// This test remains green after implementation (no lock → no expiry check).
    #[test]
    fn test_no_lock_held_skips_expiry_check() {
        // On-disk: lock held (with some expires_at).
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: no lock block (clearing the lock), timestamp advanced.
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_state_md(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "No lock in proposed content must skip LockExpiryStale check and return Continue \
             when timestamp is advanced (AC-006 / ADR-025 §12.3 row 3)"
        );
    }
}
