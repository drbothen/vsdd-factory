//! Dead-letter queue writer stub (S-4.05 Task 1 — RED gate).
//!
//! All public types are declared so the test suite compiles.
//! All method bodies are `unimplemented!()` stubs — every test that calls
//! them will panic and thus FAIL (RED gate verified).
//!
//! Implementation is the implementer's job in the GREEN phase.

#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::SinkEvent;
use crate::events::SinkDlqEvent;
use crate::path_template::PathTemplateError;

// ── DlqReason ────────────────────────────────────────────────────────────────

/// Why an event was routed to the dead-letter queue (BC-3.07.003 `reason` TV).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DlqReason {
    /// All retry attempts were exhausted (AC-002).
    RetryExhausted,
    /// The outbound queue overflowed (AC-003).
    QueueOverflow,
}

impl DlqReason {
    /// Returns the snake_case literal used in the `internal.sink_dlq_write`
    /// event's `reason` field (BC-3.07.003 TV).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RetryExhausted => "retry_exhausted",
            Self::QueueOverflow => "queue_overflow",
        }
    }
}

// ── DlqError ─────────────────────────────────────────────────────────────────

/// Filesystem errors that can occur during a DLQ write (BC-3.07.004).
#[derive(Debug)]
pub enum DlqError {
    /// `fs::create_dir_all` failed (AC-007 / EC-001).
    MkdirFailed(std::io::Error),
    /// Opening the DLQ file failed.
    OpenFailed(std::io::Error),
    /// Writing to the DLQ file failed (AC-010).
    WriteFailed(std::io::Error),
    /// Path-template error (e.g., unknown placeholder).
    TemplateFailed(PathTemplateError),
}

impl std::fmt::Display for DlqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MkdirFailed(e) => write!(f, "DLQ mkdir failed: {e}"),
            Self::OpenFailed(e) => write!(f, "DLQ open failed: {e}"),
            Self::WriteFailed(e) => write!(f, "DLQ write failed: {e}"),
            Self::TemplateFailed(e) => write!(f, "DLQ template error: {e}"),
        }
    }
}

impl std::error::Error for DlqError {}

// ── DlqWriterConfig ───────────────────────────────────────────────────────────

/// Configuration for a [`DlqWriter`] instance (S-4.05 Task 1).
#[derive(Debug, Clone)]
pub struct DlqWriterConfig {
    /// Filename-only template, e.g. `dead-letter-{name}-{date}.jsonl`.
    /// No directory prefix — directory is supplied via `dlq_root`.
    pub template: String,
    /// Per-file size cap in bytes (default 100 MiB). When reached, a new
    /// sequenced file is opened (AC-004).
    pub size_cap_bytes: u64,
    /// Optional project basename forwarded to `resolve_path_template`.
    pub project: Option<String>,
    /// Absolute (production) or relative (test) path to the DLQ directory.
    /// Auto-created on first write via `fs::create_dir_all` (AC-007).
    pub dlq_root: PathBuf,
}

impl Default for DlqWriterConfig {
    fn default() -> Self {
        Self {
            template: "dead-letter-{name}-{date}.jsonl".to_owned(),
            size_cap_bytes: 100 * 1024 * 1024,
            project: None,
            dlq_root: PathBuf::from(".factory/logs/dlq"),
        }
    }
}

// ── DlqWriter ────────────────────────────────────────────────────────────────

/// Per-sink dead-letter queue writer (S-4.05 Tasks 1 + 2).
///
/// Appends dropped events to a daily-rotated JSONL file under `dlq_root`.
/// Rotation occurs at UTC midnight (via clock injection) and at 100 MiB
/// (size-cap with sequential suffix).
///
/// All method bodies are stubs — `unimplemented!()` — for the RED gate.
pub struct DlqWriter {
    /// Writer configuration.
    pub config: DlqWriterConfig,
    /// Channel for emitting `internal.sink_dlq_write` / `_failure` events.
    pub internal_tx: mpsc::Sender<SinkDlqEvent>,
    /// Clock injection seam. Defaults to `Utc::now()` in production.
    pub clock_fn: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>,
    /// Open file handle cache: `(path, file, byte_count)`.
    /// `None` means no file is open yet (first write will create it).
    cache: std::sync::Mutex<Option<(PathBuf, std::fs::File, u64)>>,
}

impl std::fmt::Debug for DlqWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DlqWriter")
            .field("config", &self.config)
            .finish()
    }
}

impl DlqWriter {
    /// Construct a production `DlqWriter` using the system UTC clock.
    pub fn new(config: DlqWriterConfig, internal_tx: mpsc::Sender<SinkDlqEvent>) -> Self {
        // Stub — not implemented.
        unimplemented!("DlqWriter::new stub — implement in GREEN phase")
    }

    /// Test constructor accepting an arbitrary clock function.
    pub fn with_clock_fn(
        config: DlqWriterConfig,
        internal_tx: mpsc::Sender<SinkDlqEvent>,
        clock_fn: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>,
    ) -> Self {
        // Stub — not implemented.
        unimplemented!("DlqWriter::with_clock_fn stub — implement in GREEN phase")
    }

    /// Write `event` to the DLQ file, rotating as necessary.
    ///
    /// On success emits `internal.sink_dlq_write` to the internal channel
    /// (BC-3.07.003, "at most one" per F-3207).
    ///
    /// On I/O failure emits `internal.sink_dlq_failure` to the internal
    /// channel (BC-3.07.004) and returns the error.
    pub fn write_event(
        &self,
        sink_name: &str,
        sink_type: &str,
        event: &SinkEvent,
        reason: DlqReason,
    ) -> Result<(), DlqError> {
        // Stub — not implemented.
        unimplemented!("DlqWriter::write_event stub — implement in GREEN phase")
    }

    /// Return the path of the currently-open DLQ file, if any.
    ///
    /// Test helper — `pub(crate)` so it is accessible within this crate's
    /// tests without exposing it to external callers.
    pub(crate) fn current_path(&self) -> Option<PathBuf> {
        // Stub — not implemented.
        unimplemented!("DlqWriter::current_path stub — implement in GREEN phase")
    }
}

#[cfg(test)]
#[path = "dead_letter_tests.rs"]
mod tests;
