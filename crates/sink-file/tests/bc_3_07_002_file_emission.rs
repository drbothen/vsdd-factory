//! BC-3.07.002 — sink-file emits `internal.sink_error` on each recorded failure.
//!
//! Traces to: BC-3.07.002 (AC-003, AC-004, AC-005, AC-007, AC-009).
//!
//! ## RED gate discipline
//! Tests that assert event presence on the internal channel are RED:
//! `FileSink` does not yet accept an error channel sender, so the channel
//! remains empty and assertions fail with clear messages.
//!
//! Tests that assert `SinkFailure` recording (AC-004) are GREEN — they
//! exercise existing BC-3.01.008 behavior (S-1.08 shipped).
//!
//! ## Canonical test vector (BC-3.07.002)
//! "sink-file write fails (read-only dir) → one `internal.sink_error` event with
//!  `sink_type="file"`, `attempt=0`"

use sink_core::{Sink, SinkEvent, SinkErrorEvent};
use sink_file::{DEFAULT_QUEUE_DEPTH, FileSink, FileSinkConfig};
use std::time::Duration;
use tokio::sync::mpsc;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_event() -> SinkEvent {
    SinkEvent::new()
        .insert("type", "test.file_emission_check")
        .insert("payload", "x")
}

/// Construct a `FileSink` targeting a read-only directory. The first write
/// attempt will fail with a permission error, which records a `SinkFailure`.
///
/// Only available on Unix (permission bits require Unix semantics).
#[cfg(unix)]
fn read_only_sink(name: &str, ro_dir: &std::path::Path) -> FileSink {
    let cfg = FileSinkConfig {
        name: name.to_string(),
        enabled: true,
        path_template: format!("{}/{{name}}-{{date}}.jsonl", ro_dir.display()),
        queue_depth: DEFAULT_QUEUE_DEPTH,
        routing_filter: None,
        tags: Default::default(),
    };
    FileSink::new(cfg, None).expect("FileSink::new must succeed (template valid)")
}

/// Set up a read-only temporary directory. Returns the tempdir (kept alive)
/// and the path to the ro dir inside it.
#[cfg(unix)]
fn make_ro_dir() -> (tempfile::TempDir, std::path::PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    let tmp = tempfile::tempdir().expect("tempdir must be created");
    let ro = tmp.path().join("ro");
    std::fs::create_dir_all(&ro).unwrap();
    let mut perms = std::fs::metadata(&ro).unwrap().permissions();
    perms.set_mode(0o555); // r-xr-xr-x — no write
    std::fs::set_permissions(&ro, perms).unwrap();
    (tmp, ro)
}

/// Restore write permission so tempdir cleanup succeeds.
#[cfg(unix)]
fn restore_perms(ro_dir: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(ro_dir).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(ro_dir, perms).unwrap();
}

// ── AC-003 (canonical test vector: file write fails on read-only dir) ─────────

/// BC-3.07.002 postcondition 1, AC-003:
/// When sink-file records a `SinkFailure` (write to read-only dir), exactly one
/// `internal.sink_error` event is emitted with:
///   - `type = "internal.sink_error"`
///   - `sink_type = "file"`
///   - `attempt = 0` (first attempt, file sinks don't retry writes)
///   - `error_message` non-empty
///
/// RED GATE: `FileSink` does not yet accept an error channel sender; the
/// channel will be empty after the failure, and the assertion fails.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_emits_sink_error_on_read_only_dir() {
    let (tmp, ro) = make_ro_dir();
    let sink = read_only_sink("file-sink-test", &ro);

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(16);
    let _ = tx; // implementer will thread this into FileSink.

    sink.submit(make_event());
    sink.flush().unwrap_or_default();

    // Assert the channel received an internal.sink_error event.
    // RED GATE: channel is empty because FileSink doesn't send to it yet.
    let event = rx.try_recv().unwrap_or_else(|_| {
        restore_perms(&ro);
        drop(tmp);
        panic!(
            "RED GATE: expected one internal.sink_error event on the channel \
             after a read-only dir write failure; channel is empty (production not yet wired)"
        )
    });

    assert_eq!(event.r#type, "internal.sink_error");
    assert_eq!(
        event.sink_type, "file",
        "sink_type must be 'file' for FileSink"
    );
    assert_eq!(
        event.attempt, 0,
        "file sink write failure is attempt 0 (no retries)"
    );
    assert!(
        !event.error_message.is_empty(),
        "error_message must be non-empty"
    );

    restore_perms(&ro);
}

// ── AC-004 (BC-3.01.008 preservation) ────────────────────────────────────────

/// BC-3.07.002 postcondition 2, AC-004:
/// `SinkFailure` entries continue to be recorded in `Mutex<Vec<SinkFailure>>`
/// after the emission path is wired. BC-3.01.008 postcondition 1 is preserved.
///
/// GREEN GATE: `take_failures()` works today (S-1.08 shipped); must remain
/// green after S-4.10 implementation.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_sink_failure_still_recorded_after_write_error() {
    let (tmp, ro) = make_ro_dir();
    let sink = read_only_sink("bc3-01-008-file-check", &ro);

    sink.submit(make_event());
    sink.flush().unwrap_or_default();

    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "BC-3.01.008 regression: SinkFailure must still be recorded for read-only dir write"
    );

    restore_perms(&ro);
    drop(tmp);
}

// ── AC-005 (silent drop on full/closed channel) ───────────────────────────────

/// BC-3.07.002 postcondition 3, AC-005 (VP-007):
/// When the internal event channel is full at emission time:
///   - The sink does NOT panic.
///   - The `SinkFailure` IS still recorded.
///
/// RED GATE / GREEN GATE: no emission wired yet so no panic; after implementation
/// `try_send().ok()` ensures same behavior.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_silent_drop_on_full_channel_no_panic() {
    let (tmp, ro) = make_ro_dir();
    let sink = read_only_sink("file-full-channel", &ro);

    // Capacity 1, pre-filled.
    let (tx, _rx) = mpsc::channel::<SinkErrorEvent>(1);
    let _ = tx.try_send(SinkErrorEvent::new("fill", "file", "fill", 0));
    let _ = tx; // implementer passes this full sender to the sink.

    // Must not panic.
    sink.submit(make_event());
    sink.flush().unwrap_or_default();

    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "SinkFailure must be recorded even when the error channel is full"
    );

    restore_perms(&ro);
    drop(tmp);
}

/// BC-3.07.002 postcondition 3, AC-005 (EC-002):
/// Closed channel causes silent drop; `SinkFailure` still recorded.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_silent_drop_on_closed_channel_no_panic() {
    let (tmp, ro) = make_ro_dir();
    let sink = read_only_sink("file-closed-channel", &ro);

    let (tx, rx) = mpsc::channel::<SinkErrorEvent>(8);
    drop(rx); // Simulate dispatcher shutdown.
    let _ = tx;

    sink.submit(make_event());
    sink.flush().unwrap_or_default();

    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "SinkFailure must be recorded even when the error channel is closed"
    );

    restore_perms(&ro);
    drop(tmp);
}

// ── AC-009 (sink_name matches config) ────────────────────────────────────────

/// BC-3.07.002 invariant 4, AC-009:
/// The `sink_name` field in the emitted event matches the operator-configured
/// name for this `FileSink` instance.
///
/// RED GATE: channel is empty; assertion fails.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_sink_name_matches_config_name() {
    let (tmp, ro) = make_ro_dir();
    let sink = read_only_sink("audit-log-file", &ro);

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);
    let _ = tx;

    sink.submit(make_event());
    sink.flush().unwrap_or_default();

    let event = rx.try_recv().unwrap_or_else(|_| {
        restore_perms(&ro);
        drop(tmp);
        panic!(
            "RED GATE: expected one internal.sink_error event with sink_name; \
             channel empty"
        )
    });

    assert_eq!(
        event.sink_name, "audit-log-file",
        "sink_name must match the configured FileSinkConfig name"
    );

    restore_perms(&ro);
}

// ── AC-007 (no routing through SinkRegistry) ─────────────────────────────────

/// BC-3.07.002 invariant 2, AC-007 (VP-012):
/// `internal.sink_error` events from `FileSink` are NOT routed through the
/// SinkRegistry fan-out. A failure in sink-A must not affect sink-B.
///
/// GREEN GATE (post-implementation); trivially non-recursive now.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_file_invariant_no_routing_through_sink_registry() {
    let (tmp_a, ro_a) = make_ro_dir();

    // A second sink that writes to a writable location.
    let tmp_b = tempfile::tempdir().expect("tempdir b");
    let cfg_b = FileSinkConfig {
        name: "file-sink-b".to_string(),
        enabled: true,
        path_template: format!("{}/{{name}}-{{date}}.jsonl", tmp_b.path().display()),
        queue_depth: DEFAULT_QUEUE_DEPTH,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink_b = FileSink::new(cfg_b, None).expect("sink_b construction");

    let sink_a = read_only_sink("file-sink-a", &ro_a);

    // Only sink_a fails; sink_b must not receive any failures.
    sink_a.submit(make_event());
    sink_a.flush().unwrap_or_default();

    // Give sink_b a moment; it received nothing so no failures possible.
    std::thread::sleep(Duration::from_millis(50));
    let sink_b_failures = sink_b.take_failures();
    assert!(
        sink_b_failures.is_empty(),
        "VP-012: sink_b must not record failures caused by sink_a failure emission; \
         got {} failures",
        sink_b_failures.len()
    );

    // sink_a must have recorded its own failure.
    let sink_a_failures = sink_a.take_failures();
    assert!(
        !sink_a_failures.is_empty(),
        "sink_a must have recorded its own SinkFailure"
    );

    restore_perms(&ro_a);
    drop(tmp_a);
    drop(tmp_b);
}
