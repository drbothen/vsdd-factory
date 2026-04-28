//! Dead-letter queue writer (S-4.05 Task 1).
//!
//! Appends dropped sink events to daily-rotated JSONL files under a
//! configurable `dlq_root` directory. Per-file size-cap with sequential
//! suffix rotation is supported.

#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::SinkEvent;
use crate::events::{SinkDlqEvent, SinkDlqFailureEvent, SinkDlqWriteEvent};
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
        let clock_fn: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync> =
            Arc::new(|| Utc::now());
        Self::with_clock_fn(config, internal_tx, clock_fn)
    }

    /// Test constructor accepting an arbitrary clock function.
    pub fn with_clock_fn(
        config: DlqWriterConfig,
        internal_tx: mpsc::Sender<SinkDlqEvent>,
        clock_fn: Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>,
    ) -> Self {
        Self {
            config,
            internal_tx,
            clock_fn,
            cache: std::sync::Mutex::new(None),
        }
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
        // Capture event_type before entering the lock (read-only, no mutex needed).
        let event_type = event.event_type().unwrap_or("").to_owned();

        // Hold the lock for the entire duration: clock + rotate + serialize + write + flush + size update.
        // This ensures atomicity per F-3103.
        // The lock scope returns (now, Result<(), DlqError>) so ALL error paths
        // can be handled uniformly in the post-lock section (emit failure event + return).
        let (now, result): (DateTime<Utc>, Result<(), DlqError>) = {
            let mut guard = self.cache.lock().unwrap_or_else(|p| p.into_inner());

            let now = (self.clock_fn)();

            // Use a closure to allow `?` for error propagation while still returning
            // a (now, Result) tuple from the lock scope.
            let r: Result<(), DlqError> = (|| {
                let base_filename = crate::path_template::resolve_path_template(
                    &self.config.template,
                    now,
                    sink_name,
                    self.config.project.as_deref(),
                )
                .map_err(DlqError::TemplateFailed)?;
                let base_path = self.config.dlq_root.join(&base_filename);

                // Determine target path: check if we need rotation.
                let target_path = match guard.as_ref() {
                    None => base_path.clone(),
                    Some((current_path, _file, current_size)) => {
                        let current_base = strip_seq_suffix(current_path);
                        if current_base != base_path {
                            base_path.clone()
                        } else if *current_size >= self.config.size_cap_bytes {
                            next_seq_path(current_path)
                        } else {
                            current_path.clone()
                        }
                    }
                };

                // If target differs from current, open a new file.
                let needs_open = guard
                    .as_ref()
                    .map(|(p, _, _)| p != &target_path)
                    .unwrap_or(true);

                if needs_open {
                    // mkdir-p inline per AC-007.
                    fs::create_dir_all(&self.config.dlq_root)
                        .map_err(DlqError::MkdirFailed)?;
                    let f = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&target_path)
                        .map_err(DlqError::OpenFailed)?;
                    let initial_size = f.metadata().map(|m| m.len()).unwrap_or(0);
                    *guard = Some((target_path.clone(), f, initial_size));
                }

                // Build JSONL record: dropped event fields + DLQ metadata fields.
                let line = {
                    let mut map = event.fields.clone();
                    map.insert(
                        "reason".to_owned(),
                        serde_json::Value::String(reason.as_str().to_owned()),
                    );
                    serde_json::to_string(&map)
                        .map_err(|e| DlqError::WriteFailed(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?
                };

                // Write the JSONL line + newline.
                if let Some((_, ref mut f, ref mut size)) = *guard {
                    let bytes_to_write = line.len() + 1; // +1 for newline
                    f.write_all(line.as_bytes()).map_err(DlqError::WriteFailed)?;
                    f.write_all(b"\n").map_err(DlqError::WriteFailed)?;
                    f.flush().map_err(DlqError::WriteFailed)?;
                    *size += bytes_to_write as u64;
                }

                Ok(())
            })();

            (now, r)
        };

        // Channel events fired AFTER releasing the lock (fire-and-forget).
        match result {
            Ok(()) => {
                let write_ev = SinkDlqWriteEvent {
                    sink_name: sink_name.to_owned(),
                    sink_type: sink_type.to_owned(),
                    event_type,
                    reason,
                    ts: now,
                    dispatcher_trace_id: None,
                };
                let _ = self.internal_tx.try_send(SinkDlqEvent::Write(write_ev));
                Ok(())
            }
            Err(e) => {
                let err_str = e.to_string();
                let failure_ev = SinkDlqFailureEvent {
                    sink_name: sink_name.to_owned(),
                    sink_type: sink_type.to_owned(),
                    error: err_str.clone(),
                    ts: now,
                    dispatcher_trace_id: None,
                };
                let _ = self.internal_tx.try_send(SinkDlqEvent::Failure(failure_ev));
                eprintln!("DLQ write failure for sink '{}': {}", sink_name, err_str);
                Err(e)
            }
        }
    }

    /// Return the path of the currently-open DLQ file, if any.
    ///
    /// Test helper — `pub(crate)` so it is accessible within this crate's
    /// tests without exposing it to external callers.
    pub(crate) fn current_path(&self) -> Option<PathBuf> {
        self.cache
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .as_ref()
            .map(|(p, _, _)| p.clone())
    }
}

/// Strip a seq suffix from a DLQ path, returning the base path.
///
/// e.g. `dead-letter-sink-2026-04-28-001.jsonl` → `dead-letter-sink-2026-04-28.jsonl`
fn strip_seq_suffix(path: &PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    // Pattern: ends with `-NNN.jsonl` where NNN is exactly 3 digits.
    if let Some(stem) = path_str.strip_suffix(".jsonl") {
        // Check if the last segment after the last '-' is 3 digits.
        if let Some(dash_pos) = stem.rfind('-') {
            let suffix = &stem[dash_pos + 1..];
            if suffix.len() == 3 && suffix.chars().all(|c| c.is_ascii_digit()) {
                let base = format!("{}.jsonl", &stem[..dash_pos]);
                return PathBuf::from(base);
            }
        }
    }
    path.clone()
}

/// Compute the next seq-suffixed path from the current path.
///
/// e.g. `dead-letter-sink-2026-04-28.jsonl` → `dead-letter-sink-2026-04-28-001.jsonl`
/// e.g. `dead-letter-sink-2026-04-28-001.jsonl` → `dead-letter-sink-2026-04-28-002.jsonl`
fn next_seq_path(current: &PathBuf) -> PathBuf {
    let path_str = current.to_string_lossy();
    if let Some(stem) = path_str.strip_suffix(".jsonl") {
        // Check if already has a seq suffix.
        if let Some(dash_pos) = stem.rfind('-') {
            let suffix = &stem[dash_pos + 1..];
            if suffix.len() == 3 && suffix.chars().all(|c| c.is_ascii_digit()) {
                let seq: u32 = suffix.parse().unwrap_or(0);
                let next = format!("{}-{:03}.jsonl", &stem[..dash_pos], seq + 1);
                return PathBuf::from(next);
            }
        }
        // No seq suffix yet — add -001.
        let next = format!("{}-001.jsonl", stem);
        return PathBuf::from(next);
    }
    current.clone()
}


#[cfg(test)]
#[path = "dead_letter_tests.rs"]
mod tests;
