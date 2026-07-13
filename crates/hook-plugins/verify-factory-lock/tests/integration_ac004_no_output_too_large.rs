// VP-095 / AC-004 real-cap-enforcement integration test (F-S1902-P1-002 replacement).
//
// REPLACES the tautological T-006 test that mocked read_file to return Ok(fixture)
// regardless of max_bytes (POLICY 11 violation: the mock bypassed the cap check,
// so the test could not detect OutputTooLarge even at the old 65536 cap).
//
// This test uses a cap-enforcement mock that mirrors real host::read_file behavior:
//   if fixture_size > max_bytes → Err("OutputTooLarge..."), else → Ok(fixture).
// The cap check is NOT mocked away; max_bytes is read from STATE_MD_MAX_BYTES at
// test time, so this test would have been RED at the old cap (65536) and is GREEN
// at the raised cap (262144).
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::assertions_on_constants,
    clippy::manual_repeat_n
)]

//! T-006 (AC-004 / VP-095): Real-cap-enforcement integration test.
//!
//! AC-004: No `output_too_large` denial for STATE.md files ≤ 262144 bytes.
//! VP-095: verify-factory-lock handles STATE.md files up to 262144 bytes without
//!   output_too_large denial (ADR-025 Decision 14).
//!
//! Assertion matrix (with STATE_MD_MAX_BYTES = 262144):
//!   65535 bytes  → mock returns Ok → guard runs → Block (foreign lock detected)
//!   65536 bytes  → mock returns Ok → guard runs → Block
//!   131072 bytes → mock returns Ok → guard runs → Block
//!   262144 bytes → mock returns Ok → guard runs → Block (at-cap; inclusive upper bound)
//!   262145 bytes → mock returns Err(OutputTooLarge) → Continue (fail-open per EC-002)
//!                  + StateReadError warn emitted
//!
//! # BC Traces
//! - BC-4.13.001 v1.15 Phase-A Precondition 3 (max_bytes = 262144)
//! - VP-095: verify-factory-lock handles STATE.md files up to 262144 bytes without
//!   output_too_large denial (ADR-025 Decision 14)
//! - BC-4.13.001 PC6: STATE.md read failure → fail-open Continue (for 262145 bytes)

use std::sync::{Arc, Mutex};

use serde_json::json;
use verify_factory_lock::{GuardCallbacks, STATE_MD_MAX_BYTES, guard_logic};
use vsdd_hook_sdk::{HookPayload, HookResult};

/// Build a minimal HookPayload for a mutating tool.
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

/// Build a STATE.md fixture of exactly `target_size` bytes with a foreign unexpired lock.
///
/// The frontmatter contains a foreign lock. The body is padded with comment lines.
fn build_fixture_with_foreign_lock(target_size: usize) -> Vec<u8> {
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

/// Run the guard with a cap-enforcement mock for the given `fixture_size`.
///
/// The mock returns `Err("OutputTooLarge...")` if `fixture_size > max_bytes`, else
/// `Ok(fixture)`. This is the cap-enforcement contract that mirrors real host::read_file.
///
/// Returns `(HookResult, Vec<String>)` — the guard result and captured warn log.
fn run_with_cap_enforcement(fixture_size: usize) -> (HookResult, Vec<String>) {
    let warn_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let wl = warn_log.clone();

    let callbacks = GuardCallbacks {
        read_file: move |_path, max_bytes, _timeout| {
            // Cap-enforcement: mirrors real host::read_file — returns OutputTooLarge
            // when the file would exceed the plugin's declared max_bytes cap.
            if fixture_size as u64 > u64::from(max_bytes) {
                Err(format!(
                    "OutputTooLarge: file size {} exceeds max_bytes {}",
                    fixture_size, max_bytes
                ))
            } else {
                Ok(build_fixture_with_foreign_lock(fixture_size))
            }
        },
        exec_subprocess: |_argv| Ok((0, "self@example.com\n".to_string())),
        log_warn: move |msg: &str| {
            wl.lock().unwrap().push(msg.to_string());
        },
    };

    let payload = payload_for_tool("Edit");
    let result = guard_logic(payload, callbacks);
    let warns = warn_log.lock().unwrap().clone();
    (result, warns)
}

/// T-006 / VP-095 real-cap-enforcement: sizes ≤ 262144 produce no output_too_large.
///
/// Tests that the guard runs without triggering OutputTooLarge for five calibrated
/// fixture sizes (65535 / 65536 / 131072 / 262144 / 262145).
///
/// GREEN with STATE_MD_MAX_BYTES = 262144: all ≤262144 sizes succeed;
///   262145 fails-open correctly.
/// Would have been RED at the old cap (65536): sizes >65536 would produce OutputTooLarge
///   even though the spec requires them to succeed.
#[test]
fn t006_vp095_real_cap_enforcement_sizes() {
    // Sizes ≤ STATE_MD_MAX_BYTES: guard must run (no OutputTooLarge), fixture has foreign
    // lock → result must be Block (confirms guard ran to completion, not StateReadError).
    let below_cap_sizes: &[usize] = &[65535, 65536, 131072, 262144];
    for &size in below_cap_sizes {
        let (result, warns) = run_with_cap_enforcement(size);

        // No output_too_large or StateReadError warns for sizes ≤ cap.
        let too_large_warns: Vec<_> = warns
            .iter()
            .filter(|w| {
                let lower = w.to_lowercase();
                lower.contains("output_too_large")
                    || lower.contains("outputtoolarge")
                    || w.contains("StateReadError")
            })
            .collect();

        assert!(
            too_large_warns.is_empty(),
            "VP-095: size {} bytes must NOT produce output_too_large/StateReadError with \
             STATE_MD_MAX_BYTES={}. Got warns: {:?}",
            size,
            STATE_MD_MAX_BYTES,
            too_large_warns
        );

        // Guard must have run to completion: foreign lock fixture → Block.
        match result {
            HookResult::Block { .. } => {
                // Correct: guard ran, foreign lock detected.
            }
            HookResult::Continue => {
                panic!(
                    "VP-095: size {} bytes with foreign lock must return Block (guard ran). \
                     Got Continue. STATE_MD_MAX_BYTES={}. Warns: {:?}",
                    size, STATE_MD_MAX_BYTES, warns
                );
            }
            other => panic!(
                "VP-095: size {} bytes unexpected result: {:?}. Warns: {:?}",
                size, other, warns
            ),
        }
    }

    // Size 262145 (one byte over cap): mock returns OutputTooLarge → fail-open Continue
    // per BC-4.13.001 PC6 / EC-002.
    let over_cap_size = 262145usize;
    let (result, warns) = run_with_cap_enforcement(over_cap_size);

    assert_eq!(
        result,
        HookResult::Continue,
        "VP-095: size {} bytes (over cap) must return Continue (fail-open per PC6). \
         STATE_MD_MAX_BYTES={}. Warns: {:?}",
        over_cap_size,
        STATE_MD_MAX_BYTES,
        warns
    );

    let has_read_error_warn = warns.iter().any(|w| {
        let lower = w.to_lowercase();
        lower.contains("output_too_large")
            || lower.contains("outputtoolarge")
            || w.contains("StateReadError")
    });
    assert!(
        has_read_error_warn,
        "VP-095: size {} bytes must emit OutputTooLarge/StateReadError warn (fail-open log). \
         STATE_MD_MAX_BYTES={}. Got warns: {:?}",
        over_cap_size, STATE_MD_MAX_BYTES, warns
    );
}
