// T-006 (AC-004 / VP-097): Real-cap-enforcement integration test.
//
// Mirrors the S-19.02 integration pattern in
// crates/hook-plugins/verify-factory-lock/tests/integration_ac004_no_output_too_large.rs.
//
// Uses a cap-enforcement mock that mirrors real host::read_file behavior:
//   if fixture_size > max_bytes → Err("OutputTooLarge…"), else → Ok(fixture).
// The cap check is NOT mocked away; max_bytes is read from STATE_MD_MAX_BYTES at
// test time. This test is:
//   RED at old cap (65536): 70000-byte fixture → mock returns Err → guard returns
//     Continue (fail-open read-error) → test asserts Block → FAILS.
//   GREEN at raised cap (262144): 70000-byte fixture → mock returns Ok(fixture) →
//     guard runs → stale timestamp → Block(TimestampStale).
//
// BC Traces:
//   BC-5.40.001 v1.2 Precondition 6 (STATE_MD_MAX_BYTES = 262144)
//   BC-5.40.001 AC-004: no output_too_large denial for files ≤ 262144 bytes
//   ADR-025 Decision 7 (fail-open on read error — for 262145 over-cap case)
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::assertions_on_constants,
    clippy::manual_repeat_n
)]

//! T-006 (AC-004): Real-cap-enforcement integration test.
//!
//! AC-004: No `output_too_large` denial for `verify-state-timestamp-refresh`
//! when STATE.md is ≤ 262144 bytes.
//!
//! Assertion matrix (with STATE_MD_MAX_BYTES = 262144):
//!   70000 bytes  → mock returns Ok → guard runs → Block (stale timestamp)
//!
//! # BC Traces
//! - BC-5.40.001 v1.2 Precondition 6 (max_bytes = 262144)
//! - BC-5.40.001 AC-004: no output_too_large denial for files ≤ 262144 bytes
//! - ADR-025 Decision 7 (fail-open on read error — for 262145 over-cap case)

use std::sync::{Arc, Mutex};

use serde_json::json;
use verify_state_timestamp_refresh::{GuardCallbacks, STATE_MD_MAX_BYTES, guard_logic};
use vsdd_hook_sdk::{HookPayload, HookResult};

const TS_OLD: &str = "2026-06-11T10:00:00Z";
const TS_NEW: &str = "2026-06-11T11:00:00Z";

/// Build a Write HookPayload targeting `.factory/STATE.md` with the given content.
fn payload_write_state_md(content: &str) -> HookPayload {
    serde_json::from_value(json!({
        "event_name": "PreToolUse",
        "tool_name": "Write",
        "session_id": "integration-t006-session",
        "dispatcher_trace_id": "integration-t006-trace",
        "tool_input": {
            "file_path": ".factory/STATE.md",
            "content": content
        }
    }))
    .expect("fixture HookPayload must deserialize")
}

/// Build a STATE.md fixture of exactly `target_size` bytes with stale timestamp.
///
/// The frontmatter contains `timestamp: TS_OLD`. The body is padded with
/// `# padding\n` comment lines (all valid ASCII/UTF-8).
fn build_stale_timestamp_fixture(target_size: usize) -> Vec<u8> {
    let header = format!(
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE body\n",
        TS_OLD
    );
    let mut bytes = header.into_bytes();
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

/// Run `guard_logic` with a cap-enforcement mock for the given `fixture_size`.
///
/// The mock returns `Err("OutputTooLarge…")` when `fixture_size > max_bytes`,
/// else `Ok(fixture)`. This mirrors real `host::read_file` behavior.
///
/// Returns `(HookResult, Vec<String>)` — guard result and captured warn log.
fn run_with_stale_fixture_cap_enforcement(fixture_size: usize) -> (HookResult, Vec<String>) {
    let warn_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let wl = warn_log.clone();

    let callbacks = GuardCallbacks {
        read_file: move |_path, max_bytes, _timeout| {
            // Cap-enforcement: mirrors real host::read_file behavior —
            // returns OutputTooLarge when fixture would exceed max_bytes.
            if fixture_size as u64 > u64::from(max_bytes) {
                Err(format!(
                    "OutputTooLarge: fixture_size={} exceeds max_bytes={}",
                    fixture_size, max_bytes
                ))
            } else {
                Ok(build_stale_timestamp_fixture(fixture_size))
            }
        },
        log_warn: move |msg: &str| {
            wl.lock().unwrap().push(msg.to_string());
        },
        write_stderr: |_msg| {},
    };

    // Proposed: Write payload with STALE timestamp (same as on-disk TS_OLD).
    // Stale timestamp → guard should Block if it ran to completion.
    let proposed_stale = format!(
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE body\n",
        TS_OLD
    );
    let payload = payload_write_state_md(&proposed_stale);
    let result = guard_logic(payload, callbacks);
    let warns = warn_log.lock().unwrap().clone();
    (result, warns)
}

/// T-006 (AC-004): 70 KiB fixture produces zero `output_too_large` denials.
///
/// Red Gate: with `STATE_MD_MAX_BYTES = 65536`, the cap-enforcement mock returns
/// `Err("OutputTooLarge…")` for a 70000-byte fixture → guard returns Continue
/// (fail-open read-error) → test asserts `Block` → FAILS.
///
/// Green: with `STATE_MD_MAX_BYTES = 262144`, mock returns `Ok(fixture)` →
/// guard processes stale timestamp → `Block(TimestampStale)`.
///
/// The primary Red Gate is the `assert!(STATE_MD_MAX_BYTES >= 70_000u32)`
/// pre-condition, which fails until Task 9 raises the cap.
#[test]
fn t006_zero_output_too_large_on_70kib_state_md() {
    // Pre-condition: cap must be at least 70 KiB for this test to exercise the
    // raised-cap behaviour. Fails until Task 9 raises STATE_MD_MAX_BYTES to 262144.
    assert!(
        STATE_MD_MAX_BYTES >= 70_000u32,
        "T-006 (AC-004): STATE_MD_MAX_BYTES ({}) must be >= 70000. \
         Raise to 262144 per BC-5.40.001 v1.2 Precondition 6.",
        STATE_MD_MAX_BYTES
    );

    let fixture_size: usize = 70_000;
    let (result, warns) = run_with_stale_fixture_cap_enforcement(fixture_size);

    // Assert zero output_too_large / StateReadError in captured warn stream.
    // These events indicate the guard was denied access to the file due to the cap.
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
        "T-006 (AC-004): 70 KiB fixture must NOT produce output_too_large or StateReadError \
         warns with STATE_MD_MAX_BYTES={}. Got: {:?}",
        STATE_MD_MAX_BYTES,
        too_large_warns
    );

    // Guard must have run to completion: stale timestamp fixture → Block.
    // (Confirms guard actually processed the file, not just fail-opened.)
    match result {
        HookResult::Block { .. } => {
            // Correct: guard ran, stale timestamp detected.
        }
        HookResult::Continue => {
            panic!(
                "T-006 (AC-004): 70 KiB fixture with stale timestamp must return Block \
                 (guard ran to completion). Got Continue. STATE_MD_MAX_BYTES={}. \
                 At current cap (65536): cap-enforcement mock returns Err → fail-open Continue. \
                 Fix: raise STATE_MD_MAX_BYTES to 262144 (Task 9). Warns: {:?}",
                STATE_MD_MAX_BYTES, warns
            );
        }
        other => panic!(
            "T-006 (AC-004): unexpected result {:?}. STATE_MD_MAX_BYTES={}. Warns: {:?}",
            other, STATE_MD_MAX_BYTES, warns
        ),
    }
}

/// T-006 companion: advanced timestamp on 70 KiB fixture → Continue (guard operational).
///
/// Complements the stale-timestamp test by verifying the guard returns Continue
/// (not Block) when the proposed timestamp is advanced on a 70 KiB file.
/// The pre-condition asserts the cap is raised — same Red Gate as above.
#[test]
fn t006_companion_advanced_timestamp_70kib_continues() {
    assert!(
        STATE_MD_MAX_BYTES >= 70_000u32,
        "T-006 companion (AC-004): STATE_MD_MAX_BYTES ({}) must be >= 70000. \
         Raise to 262144 per BC-5.40.001 v1.2 Precondition 6.",
        STATE_MD_MAX_BYTES
    );

    let fixture_size: usize = 70_000;
    let warn_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let wl = warn_log.clone();

    let callbacks = GuardCallbacks {
        read_file: move |_path, max_bytes, _timeout| {
            if fixture_size as u64 > u64::from(max_bytes) {
                Err(format!(
                    "OutputTooLarge: fixture_size={} exceeds max_bytes={}",
                    fixture_size, max_bytes
                ))
            } else {
                Ok(build_stale_timestamp_fixture(fixture_size))
            }
        },
        log_warn: move |msg: &str| {
            wl.lock().unwrap().push(msg.to_string());
        },
        write_stderr: |_msg| {},
    };

    // Proposed: advanced timestamp (TS_NEW > TS_OLD on-disk).
    let proposed_advanced = format!(
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE body\n",
        TS_NEW
    );
    let payload = payload_write_state_md(&proposed_advanced);
    let result = guard_logic(payload, callbacks);

    assert_eq!(
        result,
        HookResult::Continue,
        "T-006 companion (AC-004): 70 KiB fixture with advanced timestamp must return Continue. \
         Got: {:?}. Warns: {:?}",
        result,
        warn_log.lock().unwrap()
    );
}
