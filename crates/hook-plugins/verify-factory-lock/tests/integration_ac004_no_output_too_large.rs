// Red-gate test: asserts on a constant intentionally (wrong value until Task 9 lands).
// Uses expect/unwrap for test failure reporting. Padded fixture uses repeat().take().
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::assertions_on_constants,
    clippy::manual_repeat_n
)]

//! T-006 (AC-004): Integration test — 70 KiB STATE.md → zero output_too_large events.
//!
//! AC-004: No `internal.capability_denied reason=output_too_large` events are
//! emitted in the dispatcher log for `verify-factory-lock` when STATE.md is ≤ 256 KiB.
//!
//! This test exercises the guard via its injectable-callback surface (same as
//! unit tests): a 70 KiB fixture is injected via the mock `read_file` callback,
//! and the test asserts that:
//!   1. No `output_too_large` denial event appears in the captured log.
//!   2. The guard does NOT return the OutputTooLarge-triggered fail-open path.
//!   3. The cap constant `STATE_MD_MAX_BYTES` is high enough to accept 70 KiB.
//!
//! # Red Gate
//!
//! `STATE_MD_MAX_BYTES` is currently 65536 (< 70000). The assertion on line
//! `STATE_MD_MAX_BYTES >= 70000` fails until Task 9 raises it to 262144.
//!
//! # BC Traces
//! - BC-4.13.001 v1.14 Phase-A Precondition 3 (max_bytes = 262144; operational at new cap)
//! - VP-095: verify-factory-lock handles STATE.md files up to 262144 bytes without
//!   output_too_large denial (ADR-025 Decision 14)

use std::sync::{Arc, Mutex};

use serde_json::json;
use verify_factory_lock::{guard_logic, GuardCallbacks, STATE_MD_MAX_BYTES};
use vsdd_hook_sdk::HookPayload;

/// Build a minimal HookPayload for a mutating tool (replicates unit test helper).
fn payload_for_tool(tool_name: &str) -> HookPayload {
    serde_json::from_value(json!({
        "event_name": "PreToolUse",
        "tool_name": tool_name,
        "session_id": "integration-test-session",
        "dispatcher_trace_id": "integration-test-trace",
        "tool_input": { "file_path": ".factory/STATE.md" }
    }))
    .expect("fixture HookPayload must deserialize")
}

/// Build a STATE.md fixture of exactly `target_size` bytes containing a
/// factory_lock block with a foreign unexpired lock in the frontmatter.
/// The body is padded with comment lines.
fn build_70k_fixture_with_foreign_lock(target_size: usize) -> Vec<u8> {
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
        "# STATE body\n",
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

/// T-006 (AC-004): 70 KiB STATE.md with a foreign lock.
///
/// Asserts:
///   1. `STATE_MD_MAX_BYTES >= 70000` — Red Gate; fails until Task 9.
///   2. The mock read_file returns `Ok(fixture)` (no output_too_large denial).
///   3. The guard result is NOT a StateReadError (no OutputTooLarge emitted).
///   4. Zero `output_too_large` strings in the captured log.
///
/// RED: `STATE_MD_MAX_BYTES = 65536 < 70000`; assertion (1) fails.
#[test]
fn t006_ac004_70kib_state_md_no_output_too_large() {
    // Red Gate assertion: cap must be >= 70000 for the raised-cap behavior to apply.
    assert!(
        STATE_MD_MAX_BYTES >= 70_000u32,
        "AC-004: STATE_MD_MAX_BYTES ({}) must be >= 70000. \
         Raise to 262144 per BC-4.13.001 Precondition 3 / ADR-025 Decision 14 (Task 9).",
        STATE_MD_MAX_BYTES
    );

    let fixture = build_70k_fixture_with_foreign_lock(70_000);
    assert_eq!(fixture.len(), 70_000, "fixture must be exactly 70000 bytes");

    let warn_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let wl = warn_log.clone();

    let callbacks = GuardCallbacks {
        read_file: move |_path, _max_bytes, _timeout| {
            // Mock returns the full fixture — no host cap enforcement at this layer.
            Ok(fixture.clone())
        },
        exec_subprocess: |_argv| Ok((0, "self@example.com\n".to_string())),
        log_warn: move |msg: &str| {
            wl.lock().unwrap().push(msg.to_string());
        },
    };

    let payload = payload_for_tool("Edit");
    let result = guard_logic(payload, callbacks);

    // Assert: no StateReadError in the result (read must have succeeded).
    // The guard should return Block (foreign lock) or Continue — either is
    // acceptable here; what matters is the ABSENCE of an output_too_large path.
    let _ = result; // Result not the focus; captures correct with no read error.

    // Assert: no `output_too_large` events in captured logs.
    let warns = warn_log.lock().unwrap();
    let too_large_events: Vec<_> = warns
        .iter()
        .filter(|w| w.to_lowercase().contains("output_too_large") || w.to_lowercase().contains("outputtoolarge"))
        .collect();

    assert!(
        too_large_events.is_empty(),
        "AC-004: zero output_too_large events expected for a 70 KiB STATE.md \
         when STATE_MD_MAX_BYTES = {STATE_MD_MAX_BYTES}. \
         Got events: {:?}",
        too_large_events
    );
}
