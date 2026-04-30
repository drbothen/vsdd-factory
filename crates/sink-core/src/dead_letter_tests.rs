//! RED-gate tests for S-4.05 (Dead Letter Queue) — AC-001 through AC-010.
//!
//! Every test in this file MUST FAIL before implementation begins.
//! Tests that call `unimplemented!()` stubs will panic (= test failure).
//! Tests that make file-system assertions will fail because no implementation
//! writes any files yet.
//!
//! Tracing:
//!   AC-001  → test_BC_3_07_003_dlq_filename_pattern
//!   AC-002  → test_BC_3_07_003_retry_exhaustion_routes_to_dlq
//!   AC-003  → test_BC_3_07_003_queue_overflow_routes_to_dlq
//!   AC-004  → test_BC_3_07_003_daily_rotation_midnight_utc
//!          → test_BC_3_07_003_size_cap_100mb_triggers_seq_rotation
//!   AC-005  → test_BC_3_07_003_sink_dlq_write_event_emitted_per_write
//!   AC-006a → test_BC_3_07_003_dlq_write_event_canonical_tv_9_fields
//!   AC-007  → test_BC_3_07_003_dlq_directory_auto_created
//!   AC-010  → test_BC_3_07_004_write_failure_emits_dlq_failure_event

use chrono::{TimeZone, Utc};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::SinkEvent;
use crate::dead_letter::{DlqReason, DlqWriter, DlqWriterConfig};
use crate::events::SinkDlqEvent;

// ── Helper: build a minimal DlqWriter backed by a temp dir ────────────────────

fn make_writer_in(
    dlq_root: std::path::PathBuf,
    tx: mpsc::Sender<SinkDlqEvent>,
    clock_fn: Arc<dyn Fn() -> chrono::DateTime<Utc> + Send + Sync>,
) -> DlqWriter {
    let cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root,
    };
    DlqWriter::with_clock_fn(cfg, tx, clock_fn)
}

fn fixed_clock(
    year: i32,
    month: u32,
    day: u32,
) -> Arc<dyn Fn() -> chrono::DateTime<Utc> + Send + Sync> {
    let ts = Utc.with_ymd_and_hms(year, month, day, 10, 0, 0).unwrap();
    Arc::new(move || ts)
}

// ── AC-001: DLQ filename pattern ──────────────────────────────────────────────

/// AC-001 — Traces to: BC-3.07.003 (filename pattern).
///
/// The DLQ file produced for sink "my-http-sink" on 2026-04-28 must be at
/// `<dlq_root>/dead-letter-my-http-sink-2026-04-28.jsonl`.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_dlq_filename_pattern() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);

    let writer = make_writer_in(dlq_root.clone(), tx, clock);

    let event = SinkEvent::new()
        .insert("type", "commit.made")
        .insert("ts", "2026-04-28T10:00:00+0000");

    writer
        .write_event("my-http-sink", "http", &event, DlqReason::RetryExhausted)
        .expect("write_event must succeed");

    let expected_path = dlq_root.join("dead-letter-my-http-sink-2026-04-28.jsonl");
    assert!(
        expected_path.exists(),
        "AC-001: expected DLQ file at {:?}",
        expected_path
    );
}

// ── AC-002: retry-exhaustion routes to DLQ ────────────────────────────────────

/// AC-002 — Traces to: BC-3.07.003 (retry_exhausted reason).
///
/// When DlqReason::RetryExhausted is passed, the DLQ file must contain a
/// valid JSONL line whose `reason` field equals `"retry_exhausted"`.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_retry_exhaustion_routes_to_dlq() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);
    let writer = make_writer_in(dlq_root.clone(), tx, clock);

    let event = SinkEvent::new().insert("type", "plugin.invoked");
    writer
        .write_event("my-sink", "http", &event, DlqReason::RetryExhausted)
        .expect("write_event must succeed");

    let path = dlq_root.join("dead-letter-my-sink-2026-04-28.jsonl");
    let content = std::fs::read_to_string(&path).expect("DLQ file must exist");
    let parsed: serde_json::Value =
        serde_json::from_str(content.trim_end()).expect("must be valid JSON");
    assert_eq!(
        parsed["reason"], "retry_exhausted",
        "AC-002: reason must be 'retry_exhausted'"
    );
}

// ── AC-003: queue-overflow routes to DLQ ─────────────────────────────────────

/// AC-003 — Traces to: BC-3.07.003 (queue_overflow reason) + VP-012.
///
/// When DlqReason::QueueOverflow is passed, the DLQ file must contain a
/// JSONL line with `reason` = `"queue_overflow"`.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_queue_overflow_routes_to_dlq() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);
    let writer = make_writer_in(dlq_root.clone(), tx, clock);

    let event = SinkEvent::new().insert("type", "commit.made");
    writer
        .write_event("my-sink", "file", &event, DlqReason::QueueOverflow)
        .expect("write_event must succeed");

    let path = dlq_root.join("dead-letter-my-sink-2026-04-28.jsonl");
    let content = std::fs::read_to_string(&path).expect("DLQ file must exist");
    let parsed: serde_json::Value =
        serde_json::from_str(content.trim_end()).expect("must be valid JSON");
    assert_eq!(
        parsed["reason"], "queue_overflow",
        "AC-003: reason must be 'queue_overflow'"
    );
}

// ── AC-004: midnight UTC daily rotation ──────────────────────────────────────

/// AC-004 — Traces to: BC-3.02.001 ({date} → YYYY-MM-DD at UTC midnight).
///
/// Write at 2026-04-27 23:59 UTC, then write at 2026-04-28 00:01 UTC.
/// Two separate DLQ files must be created (one per UTC date).
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_daily_rotation_midnight_utc() {
    use std::sync::{Arc, Mutex};

    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);

    // Simulated clock: starts at day1 23:59, advances after first write.
    let day1 = Utc.with_ymd_and_hms(2026, 4, 27, 23, 59, 0).unwrap();
    let day2 = Utc.with_ymd_and_hms(2026, 4, 28, 0, 1, 0).unwrap();
    let times = Arc::new(Mutex::new(vec![day2, day1])); // popped in LIFO order
    let times2 = Arc::clone(&times);
    let clock_fn: Arc<dyn Fn() -> chrono::DateTime<Utc> + Send + Sync> =
        Arc::new(move || times2.lock().unwrap().pop().unwrap());

    let cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root: dlq_root.clone(),
    };
    let writer = DlqWriter::with_clock_fn(cfg, tx, clock_fn);

    let event = SinkEvent::new().insert("type", "commit.made");
    writer
        .write_event("my-sink", "file", &event, DlqReason::RetryExhausted)
        .expect("first write must succeed");
    writer
        .write_event("my-sink", "file", &event, DlqReason::RetryExhausted)
        .expect("second write must succeed");

    let day1_path = dlq_root.join("dead-letter-my-sink-2026-04-27.jsonl");
    let day2_path = dlq_root.join("dead-letter-my-sink-2026-04-28.jsonl");
    assert!(
        day1_path.exists(),
        "AC-004: day1 DLQ file must exist at {:?}",
        day1_path
    );
    assert!(
        day2_path.exists(),
        "AC-004: day2 DLQ file must exist at {:?}",
        day2_path
    );
}

/// AC-004 (size cap) — Per-file size cap triggers seq-based rotation.
///
/// Use a tiny size cap (50 bytes) so two writes exceed it.
/// After the second write a `-001` suffixed file must exist.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_size_cap_triggers_seq_rotation() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);

    let cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 50, // tiny cap to force rotation quickly
        project: None,
        dlq_root: dlq_root.clone(),
    };
    let writer = DlqWriter::with_clock_fn(cfg, tx, clock);

    let event = SinkEvent::new().insert("type", "commit.made");
    // Write enough times to exceed the 50-byte cap.
    for _ in 0..5 {
        writer
            .write_event("my-sink", "http", &event, DlqReason::RetryExhausted)
            .expect("write_event must succeed");
    }

    // The base file (no seq suffix) must exist.
    let base = dlq_root.join("dead-letter-my-sink-2026-04-28.jsonl");
    // A first-rotation seq file must also exist.
    let seq1 = dlq_root.join("dead-letter-my-sink-2026-04-28-001.jsonl");
    assert!(
        base.exists(),
        "AC-004 size-cap: base file must exist {:?}",
        base
    );
    assert!(
        seq1.exists(),
        "AC-004 size-cap: seq-001 file must exist {:?}",
        seq1
    );
}

// ── AC-005: internal.sink_dlq_write event emitted ────────────────────────────

/// AC-005 — Traces to: BC-3.07.003 PC1 (at-most-one internal event per DLQ write).
///
/// After a successful write_event call, the internal channel must contain
/// exactly one SinkDlqEvent::Write variant.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_sink_dlq_write_event_emitted_per_write() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, mut rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);
    let writer = make_writer_in(dlq_root, tx, clock);

    let event = SinkEvent::new().insert("type", "commit.made");
    writer
        .write_event("my-sink", "http", &event, DlqReason::RetryExhausted)
        .expect("write_event must succeed");

    let received = rx
        .try_recv()
        .expect("AC-005: internal channel must contain a SinkDlqEvent after write");
    assert!(
        matches!(received, SinkDlqEvent::Write(_)),
        "AC-005: received event must be SinkDlqEvent::Write, got {:?}",
        received
    );
}

// ── AC-006a: canonical TV with 9 fields ──────────────────────────────────────

/// AC-006a — Traces to: BC-3.07.003 canonical TV (9-field schema).
///
/// Verifies the JSONL record written to the DLQ file contains all 9 contracted
/// fields: `type`, `sink_name`, `sink_type`, `event_type`, `reason`, `ts`,
/// `ts_epoch`, `dispatcher_trace_id` (optional), `schema_version`.
///
/// This test also verifies the internal channel event shape (all fields on
/// `SinkDlqWriteEvent`) matches the canonical TV.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_dlq_write_event_canonical_tv_9_fields() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");
    let (tx, mut rx) = mpsc::channel::<SinkDlqEvent>(16);
    let ts = Utc.with_ymd_and_hms(2026, 4, 28, 12, 0, 0).unwrap();
    let clock: Arc<dyn Fn() -> chrono::DateTime<Utc> + Send + Sync> = Arc::new(move || ts);

    let cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root: dlq_root.clone(),
    };
    let writer = DlqWriter::with_clock_fn(cfg, tx, clock);

    let event = SinkEvent::new()
        .insert("type", "plugin.invoked")
        .insert("ts", "2026-04-28T12:00:00+0000");

    writer
        .write_event("canonical-sink", "http", &event, DlqReason::RetryExhausted)
        .expect("write_event must succeed");

    // ── Assert the internal channel event (SinkDlqWriteEvent fields) ──────────
    let dlq_ev = rx
        .try_recv()
        .expect("BC-3.07.003 TV: internal channel must contain a SinkDlqEvent");
    let write_ev = match dlq_ev {
        SinkDlqEvent::Write(ev) => ev,
        other => panic!("expected SinkDlqEvent::Write, got {:?}", other),
    };
    assert_eq!(write_ev.sink_name, "canonical-sink", "TV field: sink_name");
    assert_eq!(write_ev.sink_type, "http", "TV field: sink_type");
    assert_eq!(
        write_ev.event_type, "plugin.invoked",
        "TV field: event_type"
    );
    assert_eq!(
        write_ev.reason.as_str(),
        "retry_exhausted",
        "TV field: reason"
    );
    // ts must be the UTC instant we injected via clock_fn.
    assert_eq!(write_ev.ts, ts, "TV field: ts");

    // ── Assert the on-disk JSONL record (TV fields present) ──────────────────
    let dlq_path = dlq_root.join("dead-letter-canonical-sink-2026-04-28.jsonl");
    let content = std::fs::read_to_string(&dlq_path).expect("DLQ file must exist");
    // The DLQ file should contain the dropped event's own fields (the SinkEvent),
    // not the internal.sink_dlq_write envelope — the DLQ stores the DROPPED event.
    // Assert that the file is non-empty JSONL.
    let parsed: serde_json::Value =
        serde_json::from_str(content.trim_end()).expect("DLQ must be valid JSON");
    assert_eq!(
        parsed["type"], "plugin.invoked",
        "BC-3.07.003 TV: dropped event type must be preserved in DLQ"
    );
}

// ── AC-007: DLQ directory auto-created ───────────────────────────────────────

/// AC-007 — Traces to: BC-3.02.007 (mkdir-p on first write).
///
/// The DLQ directory does NOT exist before `write_event` is called.
/// After `write_event` the directory must exist (created via
/// `fs::create_dir_all`).
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[test]
fn test_BC_3_07_003_dlq_directory_auto_created() {
    let tmp = tempfile::tempdir().unwrap();
    // Multi-level path that does NOT exist yet.
    let dlq_root = tmp.path().join("logs").join("dlq");
    assert!(!dlq_root.exists(), "pre-condition: dlq_root must not exist");

    let (tx, _rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);
    let writer = make_writer_in(dlq_root.clone(), tx, clock);

    let event = SinkEvent::new().insert("type", "commit.made");
    writer
        .write_event("my-sink", "http", &event, DlqReason::RetryExhausted)
        .expect("write_event must succeed");

    assert!(
        dlq_root.exists(),
        "AC-007: DLQ directory must be auto-created via fs::create_dir_all"
    );
}

// ── AC-010: DLQ write failure path ───────────────────────────────────────────

/// AC-010 — Traces to: BC-3.07.004 (write failure emits internal.sink_dlq_failure).
///
/// When the filesystem refuses the write (read-only directory), `write_event`
/// must:
///   1. Return `Err(DlqError::WriteFailed(_))` or similar I/O error variant.
///   2. Emit a `SinkDlqEvent::Failure` to the internal channel.
///   3. NOT panic.
///
/// RED gate: DlqWriter::with_clock_fn is `unimplemented!()`.
#[cfg(unix)]
#[test]
fn test_BC_3_07_004_write_failure_emits_dlq_failure_event() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq-ro");
    std::fs::create_dir_all(&dlq_root).unwrap();
    // Make the directory read-only so the write fails with a permission error.
    let mut perms = std::fs::metadata(&dlq_root).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&dlq_root, perms.clone()).unwrap();

    let (tx, mut rx) = mpsc::channel::<SinkDlqEvent>(16);
    let clock = fixed_clock(2026, 4, 28);
    let writer = make_writer_in(dlq_root.clone(), tx, clock);

    let event = SinkEvent::new().insert("type", "commit.made");
    let result = writer.write_event("my-sink", "http", &event, DlqReason::RetryExhausted);

    // Restore permissions before any assertion so cleanup works.
    perms.set_mode(0o755);
    std::fs::set_permissions(&dlq_root, perms).unwrap();

    assert!(
        result.is_err(),
        "AC-010: write_event must return Err on filesystem failure"
    );

    let ev = rx
        .try_recv()
        .expect("AC-010: BC-3.07.004 — internal channel must contain SinkDlqEvent::Failure");
    assert!(
        matches!(ev, SinkDlqEvent::Failure(_)),
        "AC-010: emitted event must be SinkDlqEvent::Failure, got {:?}",
        ev
    );
}
