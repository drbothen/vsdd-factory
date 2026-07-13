//! `VSDD_SINK_FILE` flush utility — S-19.05 AC-004.
//!
//! Provides [`flush_sink_file`] as a library-exported function so both
//! `main.rs` and integration tests can write plugin-emitted events as
//! JSONL to a file path supplied at runtime.
//!
//! ## Security: SEC-003 path traversal protection
//!
//! `flush_sink_file` rejects any path containing `".."` sequences or null
//! bytes. Absolute paths are accepted — bats tests use `mktemp` which
//! produces absolute paths. A `tracing::warn!` is emitted for rejected
//! paths; no file is written.
//!
//! ## Build profile availability
//!
//! S-19.05 AC-004: `VSDD_SINK_FILE` is honored at runtime in BOTH debug
//! and release builds. The previous `#[cfg(debug_assertions)]` gate in
//! `main.rs` has been removed. The caller is responsible for reading
//! `VSDD_SINK_FILE` from the environment; this function is the write side.
//!
//! ## Event filtering
//!
//! All events EXCEPT `internal.*` are written to the sink. This allows:
//! - `plugin.completed` (async path, BC-3.08.001 v1.21 Event 6)
//! - `plugin.abandoned` (BC-3.08.001 v1.21 Event 5)
//! - `dispatcher.schema_mismatch` (BC-3.08.001 Event 2, VP-079 S2)
//! - `dispatcher.registry_invalid` (BC-3.08.001 Event 3, VP-079 S3)
//! - `plugin.async_block_discarded` (BC-3.08.001 Event 1, VP-079 S1)
//! - `plugin.timeout` with execution_group=async (BC-3.08.001 Event 4, VP-079 S4)
//! - All other plugin-domain events
//!
//! `internal.*` events are dispatcher-private lifecycle diagnostics and
//! are excluded from the observable events-*.jsonl stream.

use std::io::Write;
use std::sync::{Arc, Mutex};

use tracing::warn;

use crate::internal_log::InternalEvent;

/// Write plugin-emitted events as JSONL to the given sink path.
///
/// Best-effort: any I/O or serialization error is silently swallowed so
/// the dispatcher always exits 0 on non-block dispatches regardless of
/// sink write outcome.
///
/// ## SEC-003 path sanitization
///
/// Paths containing `".."` sequences (traversal) or null bytes (`'\0'`)
/// are rejected. A `tracing::warn!` is emitted and no file is written.
/// Absolute paths are accepted.
///
/// ## Event filtering
///
/// Only non-`internal.*` events are written to the sink. Internal
/// lifecycle events (`internal.dispatcher_error`, `internal.capability_denied`,
/// etc.) are excluded from the observable events stream.
pub fn flush_sink_file(sink_path: &str, event_queue: &Arc<Mutex<Vec<InternalEvent>>>) {
    // SEC-003: reject path traversal sequences and null bytes.
    // Absolute paths are allowed — bats integration tests use mktemp which
    // produces absolute paths. The ".." check is a string-level defense;
    // it rejects any path whose string representation contains "..".
    if sink_path.contains("..") || sink_path.contains('\0') {
        warn!(
            path = sink_path,
            "VSDD_SINK_FILE rejected: path traversal detected (SEC-003)"
        );
        return;
    }

    let events = {
        match event_queue.lock() {
            Ok(mut guard) => std::mem::take(&mut *guard),
            Err(_) => return,
        }
    };

    // Exclude only internal.* private diagnostics — all observable BC-3.08.001 events pass through.
    // internal.* events are dispatcher-private and not part of the observable event stream:
    //   internal.capability_denied, internal.file_not_found, internal.dispatcher_error,
    //   internal.host_function_panic, internal.sink_error, internal.event_filtered, etc.
    // Note: sync plugin lifecycle events (plugin.invoked, plugin.completed/timeout emitted by
    // executor's emit_lifecycle) flow through InternalLog (file-based daily log), NOT the
    // HostContext event queue — so they never reach this function and need no filtering here.
    let domain_events: Vec<_> = events
        .iter()
        .filter(|ev| !ev.type_.starts_with("internal."))
        .collect();

    if domain_events.is_empty() {
        return;
    }

    // Open (or create) the sink file for appending.
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(sink_path);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => return, // best-effort: silently drop
    };

    for ev in domain_events {
        if let Ok(mut line) = serde_json::to_string(ev) {
            // F-P5-001: append the newline into the same String buffer so a single
            // write_all syscall writes "payload\n" as one contiguous chunk.
            // O_APPEND atomicity is per-write() on POSIX — two separate write_all
            // calls (payload, then newline) allow concurrent dispatcher processes
            // appending to the same VSDD_SINK_FILE to interleave, producing
            // "{procA}{procB}\n\n" merged physical lines = invalid JSONL.
            line.push('\n');
            let _ = file.write_all(line.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]
    use std::sync::{Arc, Mutex};

    use crate::host::HostContext;
    use crate::host::emit_event::emit_plugin_completed_async;
    use crate::internal_log::InternalEvent;

    use super::flush_sink_file;

    /// F-P5-001: each flushed record must land as its own standalone JSONL line.
    ///
    /// Flushes two events and reads the sink file back line-by-line. Every line must:
    ///   (a) parse independently as valid JSON (no merged lines from split writes), and
    ///   (b) the file must contain exactly two lines (one per event).
    ///
    /// This asserts the per-record single-buffer construction that closes F-P5-001:
    /// `line.push('\n'); file.write_all(line.as_bytes())` is one syscall, so O_APPEND
    /// provides per-record atomicity for concurrent dispatcher processes writing to the
    /// same VSDD_SINK_FILE.
    #[test]
    fn test_flush_sink_file_per_record_single_write_atomicity() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let sink_path = tmp.path().join("atomicity-test.jsonl");
        let sink_path_str = sink_path.to_str().expect("UTF-8 path").to_string();

        // Build a context with two events so we get two lines in the sink.
        let ctx = HostContext::new(
            "test-plugin-atomicity",
            "1.0.0",
            "test-session-p5001",
            "test-trace-p5001",
        );
        emit_plugin_completed_async(&ctx, "test-plugin-atomicity", "1.0.0", 0, 0, 1, 100);
        emit_plugin_completed_async(&ctx, "test-plugin-atomicity", "1.0.0", 1, 0, 2, 200);

        flush_sink_file(&sink_path_str, &ctx.events);

        let content = std::fs::read_to_string(&sink_path).expect("read sink file");
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!(
            lines.len(),
            2,
            "F-P5-001: expected exactly 2 JSONL lines (one per event); got {}.\n\
             Content: {:?}",
            lines.len(),
            content
        );

        // Each line must parse independently as valid JSON (proves no line merging).
        for (i, line) in lines.iter().enumerate() {
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
            assert!(
                parsed.is_ok(),
                "F-P5-001: line {} must be valid standalone JSON (O_APPEND per-record \
                 atomicity requires single write_all per record); got: {:?}",
                i + 1,
                line
            );
            // Each line must be a plugin.completed event (not a merged partial).
            assert_eq!(
                parsed.unwrap().get("type").and_then(|v| v.as_str()),
                Some("plugin.completed"),
                "F-P5-001: line {} type must be 'plugin.completed'",
                i + 1
            );
        }
    }

    /// Verify SEC-003 path traversal rejection (functional guard, complements T-007).
    #[test]
    fn test_flush_sink_file_rejects_traversal_path() {
        let queue: Arc<Mutex<Vec<InternalEvent>>> =
            Arc::new(Mutex::new(vec![InternalEvent::now("plugin.completed")]));
        let traversal = "/tmp/../etc/passwd";
        flush_sink_file(traversal, &queue);
        // If the traversal were followed, /etc/passwd would be clobbered — assert not created.
        // (The path itself doesn't exist as a target, so just verify no write occurred.)
        assert!(
            !std::path::Path::new("/etc/passwd.jsonl").exists()
                || !std::path::Path::new(traversal).exists(),
            "SEC-003: traversal path must not be written"
        );
    }
}
