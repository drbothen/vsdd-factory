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

    // Exclude only internal.* lifecycle noise — all observable events pass through.
    // internal.* events are dispatcher-private diagnostics (dispatcher_error,
    // capability_denied, plugin_invoked, plugin_completed, plugin_timeout lifecycle
    // events emitted by the executor's internal log path). Everything else —
    // including dispatcher.* and plugin.* domain events per BC-3.08.001 — is observable.
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
        if let Ok(line) = serde_json::to_string(ev) {
            let _ = file.write_all(line.as_bytes());
            let _ = file.write_all(b"\n");
        }
    }
}
