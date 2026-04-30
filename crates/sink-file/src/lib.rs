//! File sink driver (S-1.8).
//!
//! JSONL-append sink that preserves the existing
//! `.factory/logs/events-*.jsonl` shape every downstream consumer (OTel
//! filelog, factory-query, factory-report) depends on. Drop this sink
//! into `observability-config.toml` and it routes every enabled event
//! into a daily-rotated JSONL file — exactly what the bash `emit-event`
//! wrote in v0.79.x, only with a typed writer and room to grow.
//!
//! ## Runtime ownership (pre-S-1.6 constraint)
//!
//! S-1.6 will introduce a dispatcher-wide tokio runtime that all sinks
//! can share. Until then, the dispatcher's `main()` is synchronous, so
//! [`FileSink`] spins up its own single-threaded `current_thread`
//! runtime on a dedicated OS thread — fully self-contained, no runtime
//! leakage into the dispatcher. The worker thread owns the runtime and
//! drives the mpsc consumer loop on it. Swap to a shared `Handle` as a
//! one-line edit once S-1.6 lands.
//!
//! ## Failure handling (pending integration)
//!
//! When a write fails the worker records a [`SinkFailure`] into a
//! `Mutex<Vec<_>>` that tests and the eventual dispatcher integration
//! drain. The worker does NOT emit `internal.sink_error` directly —
//! that event goes through the main dispatcher's internal-log path,
//! which is wired into this driver in a follow-up story (the spec's
//! step 5, deliberately out of scope here to keep this commit small).
//!
//! ## Backpressure
//!
//! The internal mpsc is bounded (`queue_depth`, default 1000). On full,
//! `submit` uses `try_send` and on failure increments
//! [`FileSink::queue_full_count`] instead of blocking the producer.
//! This matches the spec's "drop oldest, emit
//! `internal.sink_queue_full`" contract with the same caveat as above:
//! the event-emission side is deferred to the integration story.

#![deny(missing_docs)]

use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use thiserror::Error;

use chrono::{DateTime, Local};
use sink_core::{
    RoutingFilter, Sink, SinkConfigCommon, SinkErrorEvent, SinkEvent, emit_sink_error,
};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Default bounded-queue depth for a [`FileSink`]. Sized generously to
/// absorb bursts from the dispatcher's plugin-fan-out without dropping
/// events under normal load.
pub const DEFAULT_QUEUE_DEPTH: usize = 1000;

/// Driver-specific configuration for a file sink, deserialized from
/// the `observability-config.toml` `[[sinks]]` array-of-tables.
#[derive(Debug, Clone, Deserialize)]
pub struct FileSinkConfig {
    /// Operator-assigned name. Used in `{name}` template substitution
    /// and sink-error correlation.
    pub name: String,

    /// Whether the sink accepts events. A disabled sink is constructed
    /// but never writes.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Path template. Supports `{date}` (YYYY-MM-DD), `{name}` (the
    /// sink's `name`), and `{project}` (basename of
    /// `CLAUDE_PROJECT_DIR`).
    pub path_template: String,

    /// Internal mpsc bound. See [`DEFAULT_QUEUE_DEPTH`].
    #[serde(default = "default_queue_depth")]
    pub queue_depth: usize,

    /// Optional routing filter; `None` accepts everything.
    #[serde(default)]
    pub routing_filter: Option<RoutingFilter>,

    /// Static tags enriched onto every event before writing. Tag keys
    /// that collide with producer-populated keys do NOT overwrite
    /// (producer is authoritative for `type`, `ts`, etc.).
    #[serde(default)]
    pub tags: std::collections::BTreeMap<String, String>,
}

fn default_true() -> bool {
    true
}

fn default_queue_depth() -> usize {
    DEFAULT_QUEUE_DEPTH
}

impl FileSinkConfig {
    /// Project the driver-specific config down to the sink-core common
    /// shape, for generic inspection by the registry.
    pub fn to_common(&self) -> SinkConfigCommon {
        SinkConfigCommon {
            name: self.name.clone(),
            enabled: self.enabled,
            routing_filter: self.routing_filter.clone(),
            tags: self.tags.clone(),
        }
    }
}

/// A recorded write failure surfaced via [`FileSink::take_failures`].
///
/// Pending the S-1.8+ integration that wires this into the dispatcher's
/// internal log, tests and the eventual integration consumer read
/// these directly.
#[derive(Debug, Clone)]
pub struct SinkFailure {
    /// The resolved path the worker tried to open/append.
    pub path: PathBuf,
    /// Human-readable reason (from the underlying `std::io::Error`).
    pub reason: String,
    /// Local-time ISO-8601 string when the failure was recorded.
    pub ts: String,
}

/// Errors surfaced from constructor / flush. The submit hot-path never
/// returns an error (by trait contract) — drops flow through
/// `queue_full_count`, write failures through [`SinkFailure`].
#[derive(Debug, Error)]
pub enum FileSinkError {
    /// The path template referenced an unknown placeholder.
    #[error("unknown placeholder in path template: {placeholder}")]
    UnknownPlaceholder {
        /// The offending placeholder literal (including braces).
        placeholder: String,
    },

    /// Flush's oneshot was dropped before the worker signaled.
    #[error("flush signal lost: worker may have exited")]
    FlushLost,

    /// Underlying I/O error during eager resolution.
    #[error("file sink I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<sink_core::PathTemplateError> for FileSinkError {
    fn from(e: sink_core::PathTemplateError) -> Self {
        match e {
            sink_core::PathTemplateError::UnknownPlaceholder { placeholder } => {
                // Preserve the existing sink-file convention of including braces
                // in the placeholder string (e.g. "{unknown}" not "unknown").
                FileSinkError::UnknownPlaceholder {
                    placeholder: format!("{{{placeholder}}}"),
                }
            }
        }
    }
}

/// Resolve a path template into a concrete [`PathBuf`].
///
/// Supported placeholders:
///
/// - `{date}` — `YYYY-MM-DD` local-time date.
/// - `{name}` — the sink name.
/// - `{project}` — basename of the `project` argument.
///
/// Unknown `{...}` placeholders return [`FileSinkError::UnknownPlaceholder`]
/// so a typo in `observability-config.toml` fails loudly at load time
/// rather than silently leaving a literal `{typo}` in the filename.
///
/// Delegates to [`sink_core::path_template::resolve_path_template`] (Task 0 extraction).
pub fn resolve_path_template(
    template: &str,
    date: DateTime<Local>,
    name: &str,
    project: Option<&str>,
) -> Result<PathBuf, FileSinkError> {
    sink_core::path_template::resolve_path_template(template, date, name, project)
        .map_err(FileSinkError::from)
}

/// Messages sent to the worker task.
enum Message {
    Event(SinkEvent),
    Flush(oneshot::Sender<()>),
}

/// Shared state between the producer-facing [`FileSink`] and its
/// worker thread.
struct Shared {
    failures: Mutex<Vec<SinkFailure>>,
    queue_full_count: AtomicU64,
    shutdown: std::sync::atomic::AtomicBool,
    /// Operator-assigned sink name for `internal.sink_error` events (AC-009).
    sink_name: String,
    /// Optional fire-and-forget channel for `internal.sink_error` events
    /// (BC-3.07.002). `None` when no error channel is wired in.
    error_tx: Option<tokio::sync::mpsc::Sender<SinkErrorEvent>>,
}

impl Shared {
    fn new(sink_name: String, error_tx: Option<tokio::sync::mpsc::Sender<SinkErrorEvent>>) -> Self {
        Self {
            failures: Mutex::new(Vec::new()),
            queue_full_count: AtomicU64::new(0),
            shutdown: std::sync::atomic::AtomicBool::new(false),
            sink_name,
            error_tx,
        }
    }
}

/// The file sink driver.
///
/// Construct with [`FileSink::new`]; send events via [`Sink::submit`];
/// call [`Sink::flush`] at tier boundaries; call [`Sink::shutdown`]
/// (or drop) to drain the queue and stop the worker.
pub struct FileSink {
    name: String,
    common: SinkConfigCommon,
    sender: Mutex<Option<mpsc::Sender<Message>>>,
    worker: Mutex<Option<JoinHandle<()>>>,
    shared: Arc<Shared>,
}

impl FileSink {
    /// Build a new file sink from the driver-specific config and an
    /// optional project-dir basename source (commonly
    /// `env::var("CLAUDE_PROJECT_DIR")`). The background thread is
    /// spawned eagerly so the worker is ready before the first
    /// `submit`.
    ///
    /// To wire `internal.sink_error` emission (BC-3.07.002), use
    /// [`Self::new_with_error_channel`] instead.
    pub fn new(config: FileSinkConfig, project_dir: Option<String>) -> Result<Self, FileSinkError> {
        Self::new_inner(config, project_dir, None)
    }

    /// Like [`Self::new`] but threads an error-event channel sender into the
    /// sink's shared state so failures are emitted as `internal.sink_error`
    /// events (BC-3.07.002).
    pub fn new_with_error_channel(
        config: FileSinkConfig,
        project_dir: Option<String>,
        error_tx: tokio::sync::mpsc::Sender<SinkErrorEvent>,
    ) -> Result<Self, FileSinkError> {
        Self::new_inner(config, project_dir, Some(error_tx))
    }

    fn new_inner(
        config: FileSinkConfig,
        project_dir: Option<String>,
        error_tx: Option<tokio::sync::mpsc::Sender<SinkErrorEvent>>,
    ) -> Result<Self, FileSinkError> {
        // Validate the template early by attempting a resolution with
        // today's date — surfaces typos before any event is submitted.
        let _ = resolve_path_template(
            &config.path_template,
            Local::now(),
            &config.name,
            project_dir.as_deref(),
        )?;

        let (tx, rx) = mpsc::channel::<Message>(config.queue_depth.max(1));
        let sink_name_for_shared = if config.name.is_empty() {
            "<unnamed>".to_owned()
        } else {
            config.name.clone()
        };
        let shared = Arc::new(Shared::new(sink_name_for_shared, error_tx));
        let worker_shared = Arc::clone(&shared);
        let worker_name = config.name.clone();
        let worker_template = config.path_template.clone();
        let worker_tags = config.tags.clone();
        let worker_project = project_dir.clone();

        // Dedicated std::thread owning a single-threaded tokio runtime.
        // This keeps the sink fully self-contained until S-1.6 wires
        // a dispatcher-wide runtime.
        let handle = std::thread::Builder::new()
            .name(format!("sink-file:{}", config.name))
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(e) => {
                        // Worker died before it could start; surface on
                        // the failure log and return.
                        worker_shared
                            .failures
                            .lock()
                            .unwrap_or_else(|p| p.into_inner())
                            .push(SinkFailure {
                                path: PathBuf::new(),
                                reason: format!("failed to build tokio runtime: {e}"),
                                ts: Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string(),
                            });
                        return;
                    }
                };
                runtime.block_on(worker_loop(
                    rx,
                    worker_template,
                    worker_name,
                    worker_project,
                    worker_tags,
                    Arc::clone(&worker_shared),
                ));
            })
            .map_err(|e| FileSinkError::Io(std::io::Error::other(format!("spawn worker: {e}"))))?;

        Ok(Self {
            name: config.name.clone(),
            common: SinkConfigCommon {
                name: config.name,
                enabled: config.enabled,
                routing_filter: config.routing_filter,
                tags: config.tags,
            },
            sender: Mutex::new(Some(tx)),
            worker: Mutex::new(Some(handle)),
            shared,
        })
    }

    /// Current count of events dropped because the internal queue was
    /// full. Monotonic; tests read this to assert backpressure
    /// behavior.
    pub fn queue_full_count(&self) -> u64 {
        self.shared.queue_full_count.load(Ordering::Relaxed)
    }

    /// Drain the recorded failure log. Pending integration, the
    /// dispatcher will pull these on a cadence and emit
    /// `internal.sink_error` for each. Tests use this directly.
    pub fn take_failures(&self) -> Vec<SinkFailure> {
        let mut guard = self
            .shared
            .failures
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        std::mem::take(&mut *guard)
    }

    /// Access the common-shape config (enabled / routing_filter / tags).
    pub fn config(&self) -> &SinkConfigCommon {
        &self.common
    }
}
// Note: FileSink::enrich() removed in S-4.06 — tag enrichment is now
// the Router's responsibility (BC-3.04.004 PC3). Events arrive at
// FileSink::submit() pre-enriched from Router::submit().

impl Sink for FileSink {
    fn name(&self) -> &str {
        &self.name
    }

    fn accepts(&self, _event: &SinkEvent) -> bool {
        if !self.common.enabled {
            return false;
        }
        if self.shared.shutdown.load(Ordering::Acquire) {
            return false;
        }
        // NOTE: RoutingFilter evaluation removed per BC-3.04.004 invariant 1.
        // Router is the single dispatch gate; FileSink::accepts handles only
        // enabled-flag and shutdown-state checks.
        true
    }

    fn routing_filter(&self) -> Option<&RoutingFilter> {
        self.common.routing_filter.as_ref()
    }

    fn tags(&self) -> &std::collections::BTreeMap<String, String> {
        &self.common.tags
    }

    fn submit(&self, event: SinkEvent) {
        if !self.accepts(&event) {
            return;
        }
        // Tag enrichment is now the Router's responsibility (BC-3.04.004 PC3).
        // Events arrive pre-enriched from Router::submit.
        let enriched = event;
        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            // Post-shutdown submit is a no-op by contract.
            return;
        };
        if sender.try_send(Message::Event(enriched)).is_err() {
            // Either queue full or worker gone. Increment the metric
            // in both cases — "event did not make it through" is the
            // operator-observable signal we care about.
            self.shared.queue_full_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn flush(&self) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            // Post-shutdown flush is a no-op.
            return Ok(());
        };
        // try_send on the control message so flush never blocks
        // the caller on a full queue; if the control path is itself
        // full, the worker has plenty of buffered work and the next
        // flush will pick up everything. Report success either way:
        // flush's contract is "make best-effort progress", not "block
        // until drained".
        if sender.try_send(Message::Flush(tx)).is_err() {
            return Err(anyhow::anyhow!(
                "sink '{}' flush channel full or closed",
                self.name
            ));
        }
        drop(guard);
        // Block the caller on the oneshot. `flush` is only called at
        // tier boundaries, not on the hot path.
        match rx.blocking_recv() {
            Ok(()) => Ok(()),
            Err(_) => Err(anyhow::Error::from(FileSinkError::FlushLost)),
        }
    }

    fn shutdown(&self) {
        // Toggle shutdown first so any concurrent submit calls become
        // no-ops.
        self.shared.shutdown.store(true, Ordering::Release);
        // Drop the sender to signal the worker loop to exit after draining.
        {
            let mut guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
            *guard = None;
        }
        // Join the worker so the caller knows the drain finished.
        let handle_opt = {
            let mut guard = self.worker.lock().unwrap_or_else(|p| p.into_inner());
            guard.take()
        };
        if let Some(h) = handle_opt {
            let _ = h.join();
        }
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        // Cheap re-entrant shutdown on drop so tests and integrations
        // that forget to call it still drain cleanly.
        if self.worker.lock().map(|g| g.is_some()).unwrap_or(false) {
            self.shutdown();
        }
    }
}

/// Worker loop body: pull messages, rotate on date change, append
/// JSONL. Any I/O error is recorded in `shared.failures` and the loop
/// continues — the dispatcher must never lose visibility because one
/// write failed.
async fn worker_loop(
    mut rx: mpsc::Receiver<Message>,
    template: String,
    name: String,
    project_dir: Option<String>,
    _tags: std::collections::BTreeMap<String, String>,
    shared: Arc<Shared>,
) {
    // Cache the currently-open path so we only re-resolve the template
    // when the date rolls over. Lazy open on first event keeps the
    // parent-dir `mkdir -p` inside the worker.
    let mut current_path: Option<PathBuf> = None;

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Event(event) => {
                let now = Local::now();
                let resolved =
                    match resolve_path_template(&template, now, &name, project_dir.as_deref()) {
                        Ok(p) => p,
                        Err(e) => {
                            record_failure(&shared, PathBuf::new(), format!("template: {e}"));
                            continue;
                        }
                    };
                if current_path.as_ref() != Some(&resolved) {
                    current_path = Some(resolved.clone());
                }
                if let Err(e) = append_jsonl(&resolved, &event) {
                    record_failure(&shared, resolved, format!("{e}"));
                }
            }
            Message::Flush(signal) => {
                // Current-thread runtime + sync file ops: all prior
                // writes are done by the time we get here. Ack.
                let _ = signal.send(());
            }
        }
    }
    // rx closed: shutdown. No additional action required — all
    // accumulated messages have been drained above.
}

fn append_jsonl(path: &Path, event: &SinkEvent) -> std::io::Result<()> {
    use std::fs::{OpenOptions, create_dir_all};
    use std::io::Write;

    if let Some(parent) = path.parent() {
        // `""` parent (template resolved to a relative bare filename)
        // is fine to skip — `create_dir_all("")` would error.
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    let mut line = serde_json::to_string(event)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    line.push('\n');

    let mut f = OpenOptions::new().append(true).create(true).open(path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}

fn record_failure(shared: &Shared, path: PathBuf, reason: String) {
    // Emit internal.sink_error BEFORE locking the failures mutex, per the
    // S-4.10 previous-story intelligence note: prefer releasing the lock before
    // try_send for clarity. EC-006: OS error messages from std::io::Error are
    // valid UTF-8 strings in Rust (they come from the OS via to_string()), but
    // apply lossy_utf8 sanitization defensively.
    if let Some(ref tx) = shared.error_tx {
        // EC-006: sanitize error_message via lossy UTF-8 (covers any OS error
        // that somehow contains invalid UTF-8, though Rust strings are always
        // valid UTF-8; this is a no-op in practice but satisfies the contract).
        let sanitized_reason = String::from_utf8_lossy(reason.as_bytes()).into_owned();
        let event = SinkErrorEvent::new(
            shared.sink_name.clone(),
            "file",
            sanitized_reason,
            0u32, // file sink has no retries; attempt is always 0.
        );
        emit_sink_error(tx, event);
    }

    let ts = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
    let mut guard = shared.failures.lock().unwrap_or_else(|p| p.into_inner());
    guard.push(SinkFailure { path, reason, ts });
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::Value;
    use std::fs;
    use std::io::BufRead;
    use std::time::Duration;

    fn fixed_date() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap()
    }

    fn read_lines(path: &Path) -> Vec<String> {
        let f = fs::File::open(path).unwrap();
        std::io::BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    }

    // --- path template tests -----------------------------------------

    #[test]
    fn template_date_only() {
        let p = resolve_path_template(".factory/logs/events-{date}.jsonl", fixed_date(), "n", None)
            .unwrap();
        assert_eq!(p, PathBuf::from(".factory/logs/events-2026-04-24.jsonl"));
    }

    #[test]
    fn template_name_only() {
        let p = resolve_path_template("/var/log/{name}.jsonl", fixed_date(), "audit-archive", None)
            .unwrap();
        assert_eq!(p, PathBuf::from("/var/log/audit-archive.jsonl"));
    }

    #[test]
    fn template_project_basename() {
        let p = resolve_path_template(
            "{project}/logs/{date}.jsonl",
            fixed_date(),
            "n",
            Some("/home/dev/vsdd-factory"),
        )
        .unwrap();
        assert_eq!(p, PathBuf::from("vsdd-factory/logs/2026-04-24.jsonl"));
    }

    #[test]
    fn template_all_placeholders() {
        let p = resolve_path_template(
            "{project}/{name}/events-{date}.jsonl",
            fixed_date(),
            "local",
            Some("/opt/work/myproj/"),
        )
        .unwrap();
        assert_eq!(p, PathBuf::from("myproj/local/events-2026-04-24.jsonl"));
    }

    #[test]
    fn template_no_project_yields_empty_basename() {
        let p =
            resolve_path_template("{project}events-{date}.jsonl", fixed_date(), "n", None).unwrap();
        assert_eq!(p, PathBuf::from("events-2026-04-24.jsonl"));
    }

    #[test]
    fn template_unknown_placeholder_errors() {
        let err = resolve_path_template("{unknown}.jsonl", fixed_date(), "n", None).unwrap_err();
        match err {
            FileSinkError::UnknownPlaceholder { placeholder } => {
                assert_eq!(placeholder, "{unknown}");
            }
            other => panic!("wrong error: {other}"),
        }
    }

    #[test]
    fn template_unbalanced_brace_treated_literally() {
        let p = resolve_path_template("weird-{date-only.jsonl", fixed_date(), "n", None).unwrap();
        assert_eq!(p, PathBuf::from("weird-{date-only.jsonl"));
    }

    // --- end-to-end worker tests -------------------------------------

    fn mk_sink(dir: &Path, name: &str, extra: Option<FileSinkConfig>) -> FileSink {
        let cfg = extra.unwrap_or_else(|| FileSinkConfig {
            name: name.to_string(),
            enabled: true,
            path_template: format!("{}/{{name}}-{{date}}.jsonl", dir.display()),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: Default::default(),
        });
        FileSink::new(cfg, None).unwrap()
    }

    fn submit_event(sink: &FileSink, t: &str, extra: &[(&str, Value)]) {
        let mut ev = SinkEvent::new().insert("type", t);
        for (k, v) in extra {
            ev = ev.insert(*k, v.clone());
        }
        sink.submit(ev);
    }

    #[test]
    fn auto_creates_parent_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let nested = tmp.path().join("a").join("b").join("c");
        let cfg = FileSinkConfig {
            name: "nested".into(),
            enabled: true,
            path_template: format!("{}/events-{{date}}.jsonl", nested.display()),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: Default::default(),
        };
        let sink = FileSink::new(cfg, None).unwrap();
        submit_event(&sink, "dispatcher.started", &[]);
        sink.flush().unwrap();
        assert!(nested.exists(), "parent dirs auto-created");
    }

    #[test]
    fn jsonl_append_preserves_three_events() {
        let tmp = tempfile::tempdir().unwrap();
        let sink = mk_sink(tmp.path(), "local-events", None);
        submit_event(
            &sink,
            "plugin.invoked",
            &[("plugin_name", Value::String("a".into()))],
        );
        submit_event(
            &sink,
            "plugin.completed",
            &[("plugin_name", Value::String("a".into()))],
        );
        submit_event(
            &sink,
            "commit.made",
            &[("sha", Value::String("deadbeef".into()))],
        );
        sink.flush().unwrap();

        let date = Local::now().format("%Y-%m-%d").to_string();
        let path = tmp.path().join(format!("local-events-{date}.jsonl"));
        let lines = read_lines(&path);
        assert_eq!(lines.len(), 3);
        let first: Value = serde_json::from_str(&lines[0]).unwrap();
        assert_eq!(first["type"], "plugin.invoked");
        let last: Value = serde_json::from_str(&lines[2]).unwrap();
        assert_eq!(last["sha"], "deadbeef");
    }

    // NOTE: routing_filter_drops_excluded_events was removed in S-4.06.
    // RoutingFilter evaluation was removed from FileSink::accepts() per
    // BC-3.04.004 invariant 1 (Router is the single dispatch gate).
    // FileSink::accepts() now only checks enabled-flag and shutdown-state.
    // Router-layer filter coverage lives in:
    //   crates/factory-dispatcher/src/sinks/router.rs::tests::
    //     test_BC_3_04_004_routing_filter_applied_in_dispatch_path
    //   crates/factory-dispatcher/tests/router_integration.rs::
    //     test_BC_3_04_004_two_sinks_different_filters_correct_routing

    // NOTE: tag_enrichment_writes_tags_onto_every_event and
    // tag_enrichment_does_not_overwrite_producer_fields were removed in S-4.06.
    // Tag enrichment was refactored from FileSink to Router::submit() per
    // BC-3.04.004 PC3+PC4 (Migration Decision: Option (a) — Refactor).
    // Equivalent coverage now lives in:
    //   crates/factory-dispatcher/src/sinks/router.rs::tests::
    //     test_BC_3_04_004_static_tags_merged_before_submit
    //     test_BC_3_04_004_tag_enrichment_does_not_overwrite_producer_fields

    #[test]
    fn disabled_sink_drops_every_event() {
        let tmp = tempfile::tempdir().unwrap();
        let cfg = FileSinkConfig {
            name: "off".into(),
            enabled: false,
            path_template: format!("{}/{{name}}-{{date}}.jsonl", tmp.path().display()),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: Default::default(),
        };
        let sink = FileSink::new(cfg, None).unwrap();
        submit_event(&sink, "commit.made", &[]);
        sink.flush().unwrap();
        let date = Local::now().format("%Y-%m-%d").to_string();
        let path = tmp.path().join(format!("off-{date}.jsonl"));
        assert!(!path.exists(), "disabled sink must not write");
    }

    #[test]
    #[cfg(unix)]
    fn read_only_path_records_failure_without_panic() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let ro = tmp.path().join("ro");
        fs::create_dir_all(&ro).unwrap();
        let mut perms = fs::metadata(&ro).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(&ro, perms.clone()).unwrap();

        let cfg = FileSinkConfig {
            name: "ro-sink".into(),
            enabled: true,
            path_template: format!("{}/{{name}}-{{date}}.jsonl", ro.display()),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: Default::default(),
        };
        let sink = FileSink::new(cfg, None).unwrap();
        submit_event(&sink, "commit.made", &[]);
        sink.flush().unwrap();

        let failures = sink.take_failures();
        assert!(
            !failures.is_empty(),
            "expected at least one recorded failure"
        );
        // Restore perms so tempdir cleanup works.
        perms.set_mode(0o755);
        fs::set_permissions(&ro, perms).unwrap();
    }

    #[test]
    fn backpressure_fills_queue_and_increments_counter() {
        let tmp = tempfile::tempdir().unwrap();
        // Tiny queue so we can reliably fill it. We submit many events
        // without ever yielding so the worker has no chance to drain.
        let cfg = FileSinkConfig {
            name: "tiny".into(),
            enabled: true,
            path_template: format!("{}/{{name}}-{{date}}.jsonl", tmp.path().display()),
            queue_depth: 2,
            routing_filter: None,
            tags: Default::default(),
        };
        let sink = FileSink::new(cfg, None).unwrap();
        // Flood the queue. Some will fit (queue_depth=2) and flow
        // through; the rest must bump queue_full_count.
        for i in 0..500 {
            submit_event(&sink, "flood", &[("i", Value::from(i as i64))]);
        }
        // Give the worker a moment to consume what fit, then check.
        std::thread::sleep(Duration::from_millis(50));
        let dropped = sink.queue_full_count();
        assert!(
            dropped > 0,
            "expected some drops under flood; got {dropped}"
        );
    }

    #[test]
    fn shutdown_drains_queued_events() {
        let tmp = tempfile::tempdir().unwrap();
        let cfg = FileSinkConfig {
            name: "drain".into(),
            enabled: true,
            path_template: format!("{}/{{name}}-{{date}}.jsonl", tmp.path().display()),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: Default::default(),
        };
        let sink = FileSink::new(cfg, None).unwrap();
        for i in 0..5 {
            submit_event(&sink, "plugin.completed", &[("i", Value::from(i as i64))]);
        }
        sink.shutdown();

        let date = Local::now().format("%Y-%m-%d").to_string();
        let path = tmp.path().join(format!("drain-{date}.jsonl"));
        let lines = read_lines(&path);
        assert_eq!(lines.len(), 5, "shutdown must drain queued events");

        // Post-shutdown submit is a no-op.
        submit_event(&sink, "after", &[]);
        let lines = read_lines(&path);
        assert_eq!(lines.len(), 5, "post-shutdown submit must not append");
    }

    #[test]
    fn config_deserializes_from_toml() {
        let src = r#"
            name = "local-events"
            enabled = true
            path_template = ".factory/logs/events-{date}.jsonl"
        "#;
        let cfg: FileSinkConfig = toml::from_str(src).unwrap();
        assert_eq!(cfg.name, "local-events");
        assert!(cfg.enabled);
        assert_eq!(cfg.queue_depth, DEFAULT_QUEUE_DEPTH);
    }
}
