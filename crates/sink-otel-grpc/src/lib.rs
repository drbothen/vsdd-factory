//! OTLP/gRPC sink driver (S-1.9).
//!
//! Forwards [`SinkEvent`]s to an OTel collector over OTLP/gRPC as
//! `LogRecord`s. This is the second sink type in v1.0-beta.1 scope and
//! the one that talks directly to the local Grafana/Loki stack defined
//! in `plugins/vsdd-factory/tools/observability/otel-collector-config.yaml`
//! without needing the filelog receiver pattern (which stays available
//! through `sink-file`).
//!
//! ## Why proto-direct, not the SDK exporter
//!
//! The `opentelemetry-otlp` crate ships a higher-level exporter that
//! owns its own runtime registration, batch processor, retry policy,
//! and shutdown handshake. That's all motion we'd have to fight to
//! match the spec's contract (explicit `batch_size`/`batch_interval_ms`,
//! [`SinkFailure`] recording, S-4.4-pluggable retry). Depending only on
//! `opentelemetry-proto` (with `gen-tonic` + `logs`) gives us the
//! generated [`LogsServiceClient`] and the proto types — exactly the
//! surface a mock OTLP receiver in our integration test reproduces with
//! `LogsServiceServer` from the same crate. The trade-off (stated
//! out loud so a future reviewer doesn't have to re-derive it): we own
//! the conversion `SinkEvent → LogRecord`, but we get tighter control
//! over batching, no extra global runtime registration, and the same
//! type compatibility as anything built on the SDK exporter would have.
//!
//! ## Runtime ownership (pre-S-1.6 constraint)
//!
//! Same pattern as [`sink_file`]: a dedicated `std::thread` owns a
//! single-threaded `current_thread` tokio runtime and drives the
//! consumer + flush loop. Until S-1.6 introduces a dispatcher-wide
//! shared runtime, this keeps the sink fully self-contained — the
//! dispatcher's synchronous main never sees a tokio dependency.
//!
//! ## Failure handling (pending integration)
//!
//! Send errors record [`SinkFailure`] entries on a `Mutex<Vec<_>>`
//! that tests and the eventual S-4.4 integration drain. The worker
//! never panics and never blocks the producer.
//!
//! ## Backpressure
//!
//! Bounded mpsc (default 1000); `submit` is `try_send` and increments
//! [`OtelGrpcSink::queue_full_count`] on overflow.

#![deny(missing_docs)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use chrono::Local;
use serde::Deserialize;
use serde_json::Value;
use sink_core::{
    RoutingFilter, Sink, SinkConfigCommon, SinkErrorEvent, SinkEvent, emit_sink_error,
};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tokio::time::Instant;

use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, logs_service_client::LogsServiceClient,
};
use opentelemetry_proto::tonic::common::v1::{
    AnyValue, KeyValue, any_value::Value as AnyValueInner,
};
use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use opentelemetry_proto::tonic::resource::v1::Resource;

/// Default endpoint — matches the local OTel collector exposed in
/// `observability-config.yaml`.
pub const DEFAULT_ENDPOINT: &str = "http://localhost:4317";

/// Default mpsc bound. Same sizing as `sink-file` so backpressure
/// behavior is uniform across drivers.
pub const DEFAULT_QUEUE_DEPTH: usize = 1000;

/// Default batch size — flushes when the buffer reaches this count.
pub const DEFAULT_BATCH_SIZE: usize = 100;

/// Default batch interval (ms) — flushes after this elapsed since the
/// first event in the buffer.
pub const DEFAULT_BATCH_INTERVAL_MS: u64 = 5000;

// --- config -----------------------------------------------------------------

/// Driver-specific configuration deserialized from the
/// `observability-config.toml` `[[sinks]]` array-of-tables.
///
/// The TOML shape (per the S-1.9 spec):
///
/// ```toml
/// [[sinks]]
/// type = "otel-grpc"
/// name = "local-grafana"
/// enabled = true
/// endpoint = "http://localhost:4317"
///
/// [sinks.local-grafana.resource_attributes]
/// "deployment.env" = "dev"
///
/// [sinks.local-grafana.batch]
/// size        = 100
/// interval_ms = 5000
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct OtelGrpcConfig {
    /// Operator-assigned sink name. Used in `internal.sink_*`
    /// correlation and the gRPC client's user-agent.
    pub name: String,

    /// Disabled sinks are constructed but never receive events. Lets
    /// operators keep config in place while debugging.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// OTLP gRPC endpoint URL. Defaults to [`DEFAULT_ENDPOINT`].
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    /// Resource attributes (`service.name`, `host.name`, etc.). Merged
    /// with built-in defaults; operator values win on collision.
    #[serde(default)]
    pub resource_attributes: BTreeMap<String, String>,

    /// Optional batch tuning. `size` and `interval_ms` are independent
    /// triggers; whichever fires first flushes the batch.
    #[serde(default)]
    pub batch: BatchConfig,

    /// Internal mpsc bound. See [`DEFAULT_QUEUE_DEPTH`].
    #[serde(default = "default_queue_depth")]
    pub queue_depth: usize,

    /// Optional routing filter. `None` accepts everything.
    #[serde(default)]
    pub routing_filter: Option<RoutingFilter>,

    /// Static tags applied as flat record attributes on every event.
    /// Tag keys that collide with producer-set fields do NOT overwrite.
    #[serde(default)]
    pub tags: BTreeMap<String, String>,
}

/// Batch tuning — flush trigger thresholds.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchConfig {
    /// Flush when the in-memory buffer reaches this many events.
    #[serde(default = "default_batch_size")]
    pub size: usize,
    /// Flush after this many milliseconds elapsed since the first
    /// event in the current buffer.
    #[serde(default = "default_batch_interval_ms")]
    pub interval_ms: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            size: DEFAULT_BATCH_SIZE,
            interval_ms: DEFAULT_BATCH_INTERVAL_MS,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_endpoint() -> String {
    DEFAULT_ENDPOINT.to_string()
}
fn default_queue_depth() -> usize {
    DEFAULT_QUEUE_DEPTH
}
fn default_batch_size() -> usize {
    DEFAULT_BATCH_SIZE
}
fn default_batch_interval_ms() -> u64 {
    DEFAULT_BATCH_INTERVAL_MS
}

impl OtelGrpcConfig {
    /// Project to the sink-core common shape.
    pub fn to_common(&self) -> SinkConfigCommon {
        SinkConfigCommon {
            name: self.name.clone(),
            enabled: self.enabled,
            routing_filter: self.routing_filter.clone(),
            tags: self.tags.clone(),
        }
    }
}

// --- failures ---------------------------------------------------------------

/// A recorded send failure. Drained by [`OtelGrpcSink::take_failures`].
/// Pending S-4.4, the dispatcher will pull these on a cadence and emit
/// `internal.sink_error` for each. Tests read them directly.
#[derive(Debug, Clone)]
pub struct SinkFailure {
    /// Endpoint URL the failing batch targeted.
    pub endpoint: String,
    /// Event count in the failed batch.
    pub batch_size: usize,
    /// Human-readable reason (from the underlying `tonic::Status` or
    /// transport error).
    pub reason: String,
    /// Local-time ISO-8601 timestamp.
    pub ts: String,
}

/// Errors surfaced from the constructor / flush. Hot-path `submit`
/// never returns an error (Sink trait contract); drops flow through
/// `queue_full_count`, send errors through [`SinkFailure`].
#[derive(Debug, Error)]
pub enum OtelGrpcError {
    /// The endpoint URL didn't parse as a valid URI.
    #[error("invalid OTLP endpoint '{endpoint}': {source}")]
    InvalidEndpoint {
        /// The offending endpoint string.
        endpoint: String,
        /// The underlying parse error from `tonic::transport::Endpoint`.
        #[source]
        source: tonic::transport::Error,
    },

    /// Flush's oneshot was dropped before the worker signaled.
    #[error("flush signal lost: worker may have exited")]
    FlushLost,

    /// Worker thread spawn failed.
    #[error("failed to spawn worker thread: {0}")]
    Spawn(String),
}

// --- sink -------------------------------------------------------------------

/// Messages from the producer-facing sink to the worker task.
enum Message {
    Event(SinkEvent),
    Flush(oneshot::Sender<()>),
}

/// Shared state between [`OtelGrpcSink`] and its worker thread.
struct Shared {
    failures: Mutex<Vec<SinkFailure>>,
    queue_full_count: AtomicU64,
    shutdown: AtomicBool,
    /// Operator-assigned sink name for `internal.sink_error` events (AC-009).
    sink_name: String,
    /// Optional fire-and-forget channel for `internal.sink_error` events
    /// (BC-3.07.002). `None` when no error channel is wired in.
    error_tx: Option<mpsc::Sender<SinkErrorEvent>>,
}

impl Shared {
    fn new(sink_name: String, error_tx: Option<mpsc::Sender<SinkErrorEvent>>) -> Self {
        Self {
            failures: Mutex::new(Vec::new()),
            queue_full_count: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
            sink_name,
            error_tx,
        }
    }
}

/// The OTLP/gRPC sink driver.
///
/// Construct with [`OtelGrpcSink::new`]; the worker thread is spawned
/// eagerly so the first `submit` finds the runtime ready. Drop or call
/// [`Sink::shutdown`] to drain and stop.
pub struct OtelGrpcSink {
    name: String,
    common: SinkConfigCommon,
    sender: Mutex<Option<mpsc::Sender<Message>>>,
    worker: Mutex<Option<JoinHandle<()>>>,
    shared: Arc<Shared>,
}

impl OtelGrpcSink {
    /// Construct a new OTLP/gRPC sink. Validates the endpoint URL
    /// (lazy connection — the actual gRPC channel is built on the
    /// worker thread inside the tokio runtime), spawns the worker, and
    /// returns. Producers may call [`Sink::submit`] immediately.
    ///
    /// To wire `internal.sink_error` emission (BC-3.07.002), use
    /// [`Self::new_with_error_channel`] instead.
    pub fn new(config: OtelGrpcConfig) -> Result<Self, OtelGrpcError> {
        Self::new_inner(config, None)
    }

    /// Like [`Self::new`] but threads an error-event channel sender into the
    /// sink's shared state so failures are emitted as `internal.sink_error`
    /// events (BC-3.07.002).
    pub fn new_with_error_channel(
        config: OtelGrpcConfig,
        error_tx: mpsc::Sender<SinkErrorEvent>,
    ) -> Result<Self, OtelGrpcError> {
        Self::new_inner(config, Some(error_tx))
    }

    fn new_inner(
        config: OtelGrpcConfig,
        error_tx: Option<mpsc::Sender<SinkErrorEvent>>,
    ) -> Result<Self, OtelGrpcError> {
        // Validate endpoint shape eagerly so a typo in
        // observability-config.toml fails loudly at load time. The
        // actual connection happens lazily on the first send inside
        // the worker — `tonic` defers connect anyway, so we mirror
        // that and only check parse-validity here.
        tonic::transport::Endpoint::from_shared(config.endpoint.clone()).map_err(|e| {
            OtelGrpcError::InvalidEndpoint {
                endpoint: config.endpoint.clone(),
                source: e,
            }
        })?;

        let queue_depth = config.queue_depth.max(1);
        let (tx, rx) = mpsc::channel::<Message>(queue_depth);
        let sink_name_for_shared = if config.name.is_empty() {
            "<unnamed>".to_owned()
        } else {
            config.name.clone()
        };
        let shared = Arc::new(Shared::new(sink_name_for_shared, error_tx));
        let worker_shared = Arc::clone(&shared);

        // Pre-compute the merged resource attributes once so the worker
        // doesn't redo the merge per batch. Defaults: `service.name =
        // "vsdd-factory"`, `host.name = <gethostname()>`. Operator
        // overrides win.
        let resource_attrs = build_resource_attributes(&config.resource_attributes);
        let endpoint = config.endpoint.clone();
        let batch_size = config.batch.size.max(1);
        let batch_interval = Duration::from_millis(config.batch.interval_ms.max(1));
        let tags = config.tags.clone();

        let handle = std::thread::Builder::new()
            .name(format!("sink-otel-grpc:{}", config.name))
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(e) => {
                        record_failure(
                            &worker_shared,
                            &endpoint,
                            0,
                            format!("failed to build tokio runtime: {e}"),
                        );
                        return;
                    }
                };
                runtime.block_on(worker_loop(
                    rx,
                    endpoint,
                    resource_attrs,
                    batch_size,
                    batch_interval,
                    tags,
                    Arc::clone(&worker_shared),
                ));
            })
            .map_err(|e| OtelGrpcError::Spawn(e.to_string()))?;

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

    /// Number of events dropped because the internal queue was full.
    /// Monotonic; tests read this to assert backpressure behavior.
    pub fn queue_full_count(&self) -> u64 {
        self.shared.queue_full_count.load(Ordering::Relaxed)
    }

    /// Drain the recorded send failures. The S-4.4 integration story
    /// will pull these on a cadence and emit `internal.sink_error`;
    /// tests use this directly.
    pub fn take_failures(&self) -> Vec<SinkFailure> {
        let mut guard = self
            .shared
            .failures
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        std::mem::take(&mut *guard)
    }

    /// Borrow the common-shape config (enabled / routing_filter / tags).
    pub fn config(&self) -> &SinkConfigCommon {
        &self.common
    }
}
// Note: OtelGrpcSink::enrich() removed in S-4.06 — tag enrichment is now
// the Router's responsibility (BC-3.04.004 PC3). Events arrive at
// OtelGrpcSink::submit() pre-enriched from Router::submit().

impl Sink for OtelGrpcSink {
    fn name(&self) -> &str {
        &self.name
    }

    fn accepts(&self, event: &SinkEvent) -> bool {
        if !self.common.enabled {
            return false;
        }
        if self.shared.shutdown.load(Ordering::Acquire) {
            return false;
        }
        // NOTE: RoutingFilter evaluation removed per BC-3.04.004 invariant 1.
        // Router is the single dispatch gate; OtelGrpcSink::accepts handles only
        // enabled-flag and shutdown-state checks.
        true
    }

    fn routing_filter(&self) -> Option<&RoutingFilter> {
        self.common.routing_filter.as_ref()
    }

    fn tags(&self) -> &BTreeMap<String, String> {
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
            // Post-shutdown submit is a no-op by trait contract.
            return;
        };
        if sender.try_send(Message::Event(enriched)).is_err() {
            self.shared.queue_full_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn flush(&self) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            return Ok(());
        };
        if sender.try_send(Message::Flush(tx)).is_err() {
            return Err(anyhow::anyhow!(
                "sink '{}' flush channel full or closed",
                self.name
            ));
        }
        drop(guard);
        match rx.blocking_recv() {
            Ok(()) => Ok(()),
            Err(_) => Err(anyhow::Error::from(OtelGrpcError::FlushLost)),
        }
    }

    fn shutdown(&self) {
        self.shared.shutdown.store(true, Ordering::Release);
        {
            let mut guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
            *guard = None;
        }
        let handle_opt = {
            let mut guard = self.worker.lock().unwrap_or_else(|p| p.into_inner());
            guard.take()
        };
        if let Some(h) = handle_opt {
            let _ = h.join();
        }
    }
}

impl Drop for OtelGrpcSink {
    fn drop(&mut self) {
        if self.worker.lock().map(|g| g.is_some()).unwrap_or(false) {
            self.shutdown();
        }
    }
}

// --- worker -----------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
async fn worker_loop(
    mut rx: mpsc::Receiver<Message>,
    endpoint: String,
    resource_attrs: Vec<KeyValue>,
    batch_size: usize,
    batch_interval: Duration,
    _tags: BTreeMap<String, String>,
    shared: Arc<Shared>,
) {
    // Lazy-built client so a connect failure on startup doesn't kill
    // the worker — we record a failure and retry on the next batch.
    let mut client: Option<LogsServiceClient<tonic::transport::Channel>> = None;
    let mut buffer: Vec<SinkEvent> = Vec::with_capacity(batch_size);
    // Deadline for the current buffer — set on first event.
    let mut buffer_deadline: Option<Instant> = None;

    loop {
        // Sleep until either the deadline fires, a message arrives, or
        // the channel closes. tokio's `recv` is cancel-safe so we can
        // race it against the deadline.
        let recv_fut = rx.recv();

        let msg_opt = match buffer_deadline {
            Some(deadline) => {
                tokio::select! {
                    biased;
                    msg = recv_fut => msg,
                    _ = tokio::time::sleep_until(deadline) => {
                        // Deadline hit; flush and continue.
                        flush_buffer(&mut client, &endpoint, &resource_attrs, &mut buffer, &shared).await;
                        buffer_deadline = None;
                        continue;
                    }
                }
            }
            None => recv_fut.await,
        };

        let Some(msg) = msg_opt else {
            // rx closed — drain any pending and exit.
            flush_buffer(
                &mut client,
                &endpoint,
                &resource_attrs,
                &mut buffer,
                &shared,
            )
            .await;
            return;
        };

        match msg {
            Message::Event(event) => {
                if buffer.is_empty() {
                    buffer_deadline = Some(Instant::now() + batch_interval);
                }
                buffer.push(event);
                if buffer.len() >= batch_size {
                    flush_buffer(
                        &mut client,
                        &endpoint,
                        &resource_attrs,
                        &mut buffer,
                        &shared,
                    )
                    .await;
                    buffer_deadline = None;
                }
            }
            Message::Flush(signal) => {
                flush_buffer(
                    &mut client,
                    &endpoint,
                    &resource_attrs,
                    &mut buffer,
                    &shared,
                )
                .await;
                buffer_deadline = None;
                let _ = signal.send(());
            }
        }
    }
}

async fn flush_buffer(
    client_slot: &mut Option<LogsServiceClient<tonic::transport::Channel>>,
    endpoint: &str,
    resource_attrs: &[KeyValue],
    buffer: &mut Vec<SinkEvent>,
    shared: &Arc<Shared>,
) {
    if buffer.is_empty() {
        return;
    }
    let batch_len = buffer.len();
    let log_records: Vec<LogRecord> = buffer.drain(..).map(event_to_log_record).collect();

    let request = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(Resource {
                attributes: resource_attrs.to_vec(),
                dropped_attributes_count: 0,
                entity_refs: Vec::new(),
            }),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };

    // Lazy connect / reconnect on every send error so a transient
    // network blip self-heals on the next batch.
    if client_slot.is_none() {
        match build_client(endpoint).await {
            Ok(c) => *client_slot = Some(c),
            Err(reason) => {
                record_failure(shared, endpoint, batch_len, reason);
                return;
            }
        }
    }

    let Some(client) = client_slot.as_mut() else {
        // Unreachable post-build, but the compiler can't prove it.
        return;
    };

    if let Err(status) = client.export(request).await {
        // Drop the client so the next attempt rebuilds the channel —
        // h2 connections can sour silently after a peer reset and
        // the cheapest fix is reconnect-on-error.
        *client_slot = None;
        record_failure(shared, endpoint, batch_len, format!("{status}"));
    }
}

async fn build_client(
    endpoint: &str,
) -> Result<LogsServiceClient<tonic::transport::Channel>, String> {
    let ep = tonic::transport::Endpoint::from_shared(endpoint.to_string())
        .map_err(|e| format!("endpoint parse: {e}"))?
        // 5s connect timeout — long enough for a reasonable network,
        // short enough that a wedged collector doesn't stall the
        // worker through the entire batch_interval.
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10));
    let channel = ep.connect().await.map_err(|e| format!("connect: {e}"))?;
    Ok(LogsServiceClient::new(channel))
}

fn record_failure(shared: &Shared, endpoint: &str, batch_size: usize, reason: String) {
    // Emit internal.sink_error BEFORE locking the failures mutex, per the
    // S-4.10 previous-story intelligence note (prefer releasing the lock before
    // try_send to avoid holding the lock across the channel operation).
    if let Some(ref tx) = shared.error_tx {
        let event = SinkErrorEvent::new(
            shared.sink_name.clone(),
            "otel-grpc",
            reason.clone(),
            0u32, // otel-grpc has no per-batch retries; attempt is always 0.
        );
        emit_sink_error(tx, event);
    }

    let ts = Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
    let mut guard = shared.failures.lock().unwrap_or_else(|p| p.into_inner());
    guard.push(SinkFailure {
        endpoint: endpoint.to_string(),
        batch_size,
        reason,
        ts,
    });
}

// --- conversion --------------------------------------------------------------

/// Reserved field names lifted to top-level KeyValue attributes per
/// the spec's "OTLP record mapping" table. Anything else in the event
/// fields map flattens into a record attribute too — see
/// [`event_to_log_record`].
const RESERVED_TOP_LEVEL_ATTRS: &[&str] = &[
    "dispatcher_trace_id",
    "session_id",
    "plugin_name",
    "plugin_version",
];

/// Convert a [`SinkEvent`] into an OTLP [`LogRecord`].
///
/// Mapping (per S-1.9 spec):
///
/// - `event.type` → body string
/// - `event.ts_epoch` (epoch millis) → `time_unix_nano`
/// - `event.dispatcher_trace_id`, `event.session_id`, `event.plugin_name`,
///   `event.plugin_version` → top-level KeyValue attributes
/// - All other keys → flat KeyValue attributes (no nesting; matches
///   Loki label compatibility expectations)
///
/// Producer-side fields like `ts` (the human-readable timestamp), `schema_version`,
/// and any plugin-specific extras all flow through as flat attributes.
pub fn event_to_log_record(event: SinkEvent) -> LogRecord {
    let mut fields = event.fields;

    let body_string = fields
        .remove("type")
        .and_then(|v| match v {
            Value::String(s) => Some(s),
            _ => None,
        })
        .unwrap_or_default();

    // ts_epoch is contractually milliseconds (per InternalEvent and the
    // S-1.8 sample event in the spec). Multiply to nanos for OTLP.
    let time_unix_nano = fields
        .remove("ts_epoch")
        .and_then(|v| match v {
            Value::Number(n) => n.as_u64().or_else(|| n.as_i64().map(|i| i.max(0) as u64)),
            _ => None,
        })
        .map(|ms| ms.saturating_mul(1_000_000))
        .unwrap_or(0);

    let mut attributes: Vec<KeyValue> = Vec::with_capacity(fields.len());

    // Reserved fields first so a Loki query can rely on them being
    // present in stable positions even when an event omits them.
    for key in RESERVED_TOP_LEVEL_ATTRS {
        if let Some(v) = fields.remove(*key) {
            attributes.push(KeyValue {
                key: (*key).to_string(),
                value: Some(json_to_any_value(v)),
            });
        }
    }

    // Everything left flattens. BTreeMap ordering would be nicer for
    // deterministic test diffs, but `Map` is `IndexMap` under the hood
    // and preserves insertion order — that's good enough.
    for (k, v) in fields {
        attributes.push(KeyValue {
            key: k,
            value: Some(json_to_any_value(v)),
        });
    }

    LogRecord {
        time_unix_nano,
        observed_time_unix_nano: time_unix_nano,
        severity_number: 0,
        severity_text: String::new(),
        body: Some(AnyValue {
            value: Some(AnyValueInner::StringValue(body_string)),
        }),
        attributes,
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: Vec::new(),
        span_id: Vec::new(),
        event_name: String::new(),
    }
}

/// Convert a `serde_json::Value` to an OTLP [`AnyValue`]. Numbers stay
/// numeric (int when possible, else float); booleans stay boolean;
/// strings stay strings; objects/arrays serialize back to JSON strings
/// so the flat-attribute promise still holds for Loki.
fn json_to_any_value(value: Value) -> AnyValue {
    let inner = match value {
        Value::Null => AnyValueInner::StringValue(String::new()),
        Value::Bool(b) => AnyValueInner::BoolValue(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                AnyValueInner::IntValue(i)
            } else if let Some(f) = n.as_f64() {
                AnyValueInner::DoubleValue(f)
            } else {
                AnyValueInner::StringValue(n.to_string())
            }
        }
        Value::String(s) => AnyValueInner::StringValue(s),
        // Nested structures are rare on the hot path (the dispatcher
        // emits flat events) but defensible: serialize back to JSON
        // so no information is lost. Keeps the "flat for Loki" rule
        // intact at the OTel layer; analysts can `parse_json` in LogQL
        // if they need to dig in.
        other => AnyValueInner::StringValue(serde_json::to_string(&other).unwrap_or_default()),
    };
    AnyValue { value: Some(inner) }
}

/// Build the resource attribute set for this sink: defaults + operator
/// overrides. Defaults: `service.name = "vsdd-factory"`, `host.name =
/// <gethostname()>`. Operator values win on key collision.
fn build_resource_attributes(operator: &BTreeMap<String, String>) -> Vec<KeyValue> {
    let mut merged: BTreeMap<String, String> = BTreeMap::new();
    merged.insert("service.name".to_string(), "vsdd-factory".to_string());
    if let Ok(host) = hostname::get()
        && let Some(s) = host.to_str()
    {
        merged.insert("host.name".to_string(), s.to_string());
    }
    for (k, v) in operator {
        merged.insert(k.clone(), v.clone());
    }
    merged
        .into_iter()
        .map(|(k, v)| KeyValue {
            key: k,
            value: Some(AnyValue {
                value: Some(AnyValueInner::StringValue(v)),
            }),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use std::time::Duration as StdDuration;

    fn extract_attr<'a>(record: &'a LogRecord, key: &str) -> Option<&'a AnyValue> {
        record
            .attributes
            .iter()
            .find(|kv| kv.key == key)
            .and_then(|kv| kv.value.as_ref())
    }

    fn body_string(record: &LogRecord) -> Option<&str> {
        match record.body.as_ref()?.value.as_ref()? {
            AnyValueInner::StringValue(s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn attr_map(record: &LogRecord) -> HashMap<String, AnyValueInner> {
        record
            .attributes
            .iter()
            .filter_map(|kv| {
                kv.value
                    .as_ref()
                    .and_then(|v| v.value.clone())
                    .map(|inner| (kv.key.clone(), inner))
            })
            .collect()
    }

    // --- conversion / mapping ----------------------------------------

    #[test]
    fn event_to_log_record_maps_reserved_fields() {
        let event = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("ts_epoch", json!(1_777_003_425_000_u64))
            .insert("dispatcher_trace_id", "trace-xyz")
            .insert("session_id", "sess-1")
            .insert("plugin_name", "capture-commit-activity")
            .insert("plugin_version", "0.1.0");

        let rec = event_to_log_record(event);

        assert_eq!(body_string(&rec), Some("plugin.invoked"));
        // ms → ns conversion
        assert_eq!(rec.time_unix_nano, 1_777_003_425_000 * 1_000_000);
        assert_eq!(rec.observed_time_unix_nano, rec.time_unix_nano);

        let m = attr_map(&rec);
        assert!(matches!(
            m.get("dispatcher_trace_id"),
            Some(AnyValueInner::StringValue(s)) if s == "trace-xyz"
        ));
        assert!(matches!(
            m.get("session_id"),
            Some(AnyValueInner::StringValue(s)) if s == "sess-1"
        ));
        assert!(matches!(
            m.get("plugin_name"),
            Some(AnyValueInner::StringValue(s)) if s == "capture-commit-activity"
        ));
        assert!(matches!(
            m.get("plugin_version"),
            Some(AnyValueInner::StringValue(s)) if s == "0.1.0"
        ));
        // ts_epoch and type are NOT also attributes — they were lifted.
        assert!(extract_attr(&rec, "type").is_none());
        assert!(extract_attr(&rec, "ts_epoch").is_none());
    }

    #[test]
    fn event_attributes_flatten_non_reserved_fields() {
        let event = SinkEvent::new()
            .insert("type", "commit.made")
            .insert("ts_epoch", json!(1_000_u64))
            .insert("sha", "deadbeef")
            .insert("files_changed", json!(7))
            .insert("dirty", json!(true))
            .insert("score", json!(0.42));

        let rec = event_to_log_record(event);
        let m = attr_map(&rec);

        assert!(matches!(
            m.get("sha"),
            Some(AnyValueInner::StringValue(s)) if s == "deadbeef"
        ));
        assert!(matches!(
            m.get("files_changed"),
            Some(AnyValueInner::IntValue(7))
        ));
        assert!(matches!(
            m.get("dirty"),
            Some(AnyValueInner::BoolValue(true))
        ));
        assert!(matches!(
            m.get("score"),
            Some(AnyValueInner::DoubleValue(d)) if (*d - 0.42).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn event_to_log_record_nested_value_serialized_to_string() {
        // Nested values are rare for InternalEvent but we don't drop
        // them — they round-trip to a JSON string so analysts can
        // parse_json in LogQL if needed.
        let event = SinkEvent::new()
            .insert("type", "plugin.completed")
            .insert("ts_epoch", json!(1_u64))
            .insert("nested", json!({"inner": "value", "n": 1}));
        let rec = event_to_log_record(event);
        let m = attr_map(&rec);
        match m.get("nested") {
            Some(AnyValueInner::StringValue(s)) => {
                let parsed: Value = serde_json::from_str(s).unwrap();
                assert_eq!(parsed["inner"], "value");
                assert_eq!(parsed["n"], 1);
            }
            other => panic!("expected stringified nested, got {other:?}"),
        }
    }

    #[test]
    fn event_to_log_record_missing_type_yields_empty_body() {
        // Producer bug; we don't panic.
        let event = SinkEvent::new().insert("ts_epoch", json!(1_u64));
        let rec = event_to_log_record(event);
        assert_eq!(body_string(&rec), Some(""));
    }

    #[test]
    fn event_to_log_record_missing_ts_yields_zero_timestamp() {
        let event = SinkEvent::new().insert("type", "x");
        let rec = event_to_log_record(event);
        assert_eq!(rec.time_unix_nano, 0);
    }

    #[test]
    fn resource_attributes_merge_defaults_with_config() {
        let mut overrides = BTreeMap::new();
        overrides.insert("deployment.env".to_string(), "dev".to_string());
        overrides.insert("service.name".to_string(), "custom-svc".to_string());
        let attrs = build_resource_attributes(&overrides);

        let map: HashMap<String, String> = attrs
            .iter()
            .filter_map(|kv| {
                kv.value.as_ref().and_then(|v| match v.value.as_ref()? {
                    AnyValueInner::StringValue(s) => Some((kv.key.clone(), s.clone())),
                    _ => None,
                })
            })
            .collect();

        // Operator override wins.
        assert_eq!(
            map.get("service.name").map(String::as_str),
            Some("custom-svc")
        );
        // Operator-supplied attribute present.
        assert_eq!(map.get("deployment.env").map(String::as_str), Some("dev"));
        // host.name is auto-populated from gethostname() — present and
        // non-empty on every supported platform.
        assert!(map.get("host.name").is_some_and(|h| !h.is_empty()));
    }

    #[test]
    fn config_deserializes_with_defaults() {
        let src = r#"
            name = "local-grafana"
            type = "otel-grpc"
        "#;
        // Note: the registry strips `type` before passing to the driver
        // config, but for unit-test purposes serde just ignores it
        // unless we deny_unknown_fields (we don't — operators may add
        // tags/extra fields freely).
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(rename = "type")]
            _type_: Option<String>,
            #[serde(flatten)]
            inner: OtelGrpcConfig,
        }
        let cfg: Wrapper = toml::from_str(src).unwrap();
        let cfg = cfg.inner;
        assert_eq!(cfg.name, "local-grafana");
        assert!(cfg.enabled);
        assert_eq!(cfg.endpoint, DEFAULT_ENDPOINT);
        assert_eq!(cfg.batch.size, DEFAULT_BATCH_SIZE);
        assert_eq!(cfg.batch.interval_ms, DEFAULT_BATCH_INTERVAL_MS);
        assert_eq!(cfg.queue_depth, DEFAULT_QUEUE_DEPTH);
    }

    #[test]
    fn config_deserializes_with_batch_overrides() {
        let src = r#"
            name = "n"
            endpoint = "http://otel:4317"
            queue_depth = 50

            [batch]
            size = 25
            interval_ms = 250

            [resource_attributes]
            "deployment.env" = "test"
        "#;
        let cfg: OtelGrpcConfig = toml::from_str(src).unwrap();
        assert_eq!(cfg.endpoint, "http://otel:4317");
        assert_eq!(cfg.batch.size, 25);
        assert_eq!(cfg.batch.interval_ms, 250);
        assert_eq!(cfg.queue_depth, 50);
        assert_eq!(
            cfg.resource_attributes.get("deployment.env"),
            Some(&"test".to_string())
        );
    }

    // --- routing / lifecycle (unreachable-endpoint exercises the
    // batch-flush path against a closed port; tests that need a real
    // OTLP roundtrip live in `crates/factory-dispatcher/tests/`) -----

    fn unreachable_sink(name: &str, batch_size: usize, interval_ms: u64) -> OtelGrpcSink {
        let cfg = OtelGrpcConfig {
            name: name.to_string(),
            enabled: true,
            // RFC 5736 reserved port 1 — guaranteed unreachable.
            endpoint: "http://127.0.0.1:1".to_string(),
            resource_attributes: BTreeMap::new(),
            batch: BatchConfig {
                size: batch_size,
                interval_ms,
            },
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: BTreeMap::new(),
        };
        OtelGrpcSink::new(cfg).unwrap()
    }

    // NOTE: routing_filter_drops_unmatched_events was updated in S-4.06.
    // RoutingFilter evaluation was removed from OtelGrpcSink::accepts() per
    // BC-3.04.004 invariant 1 (Router is the single dispatch gate). The test
    // now verifies only the retained shutdown-state check in accepts().
    // Router-layer filter coverage lives in:
    //   crates/factory-dispatcher/src/sinks/router.rs::tests::
    //     test_BC_3_04_004_routing_filter_applied_in_dispatch_path
    #[test]
    fn accepts_returns_false_after_shutdown() {
        let cfg = OtelGrpcConfig {
            name: "filtered".into(),
            enabled: true,
            endpoint: "http://127.0.0.1:1".into(),
            resource_attributes: BTreeMap::new(),
            batch: BatchConfig::default(),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: Some(RoutingFilter {
                event_types_allow: vec!["commit.made".into()],
                event_types_deny: vec![],
                plugin_ids_allow: vec![],
            }),
            tags: BTreeMap::new(),
        };
        let sink = OtelGrpcSink::new(cfg).unwrap();
        // Enabled sink accepts all events (filter check is Router's responsibility).
        assert!(sink.accepts(&SinkEvent::new().insert("type", "commit.made")));
        assert!(sink.accepts(&SinkEvent::new().insert("type", "plugin.invoked")));
        // Shutdown — flips accepts() to false.
        sink.shutdown();
        assert!(!sink.accepts(&SinkEvent::new().insert("type", "commit.made")));
    }

    #[test]
    fn batch_size_triggers_flush() {
        // Tight batch size + huge interval so size is the only
        // possible trigger. Endpoint is unreachable, so the flush
        // attempt records a SinkFailure — that's our observable.
        let sink = unreachable_sink("size-flush", 3, 60_000);
        for i in 0..3 {
            sink.submit(
                SinkEvent::new()
                    .insert("type", "x")
                    .insert("ts_epoch", json!(i as u64)),
            );
        }
        // Wait for the worker to attempt a flush (connect timeout up
        // to 5s, but the rejection from a closed port is immediate).
        let deadline = std::time::Instant::now() + StdDuration::from_secs(8);
        while std::time::Instant::now() < deadline {
            if !sink.take_failures().is_empty() {
                return;
            }
            std::thread::sleep(StdDuration::from_millis(50));
        }
        // If we get here we never observed a flush — fail loud.
        let cnt = sink.queue_full_count();
        panic!("batch_size=3 flush never attempted (queue_full={cnt})");
    }

    #[test]
    fn batch_interval_triggers_flush() {
        // Large size, tiny interval — only the timer can fire.
        let sink = unreachable_sink("interval-flush", 10_000, 50);
        // Submit one event; no size-trigger possible.
        sink.submit(
            SinkEvent::new()
                .insert("type", "x")
                .insert("ts_epoch", json!(1_u64)),
        );
        let deadline = std::time::Instant::now() + StdDuration::from_secs(8);
        while std::time::Instant::now() < deadline {
            if !sink.take_failures().is_empty() {
                return;
            }
            std::thread::sleep(StdDuration::from_millis(50));
        }
        panic!("batch_interval=50ms flush never attempted");
    }

    #[test]
    fn shutdown_drains_queued_events_to_failure_log() {
        // Queue events to an unreachable endpoint, shutdown, verify the
        // worker drained the buffer and recorded the send failure.
        let sink = unreachable_sink("drain", 2, 60_000);
        for i in 0..5 {
            sink.submit(
                SinkEvent::new()
                    .insert("type", "x")
                    .insert("ts_epoch", json!(i as u64)),
            );
        }
        sink.shutdown();
        // After shutdown the worker must have attempted at least one
        // flush; the failure log records the attempt.
        let failures = sink.take_failures();
        assert!(
            !failures.is_empty(),
            "shutdown must drain pending events through the flush path"
        );
        // Post-shutdown submit is a no-op.
        sink.submit(SinkEvent::new().insert("type", "post"));
    }

    #[test]
    fn invalid_endpoint_url_errors_at_construction() {
        let cfg = OtelGrpcConfig {
            name: "bad".into(),
            enabled: true,
            // Missing scheme — `tonic::transport::Endpoint` rejects.
            endpoint: "not a url".into(),
            resource_attributes: BTreeMap::new(),
            batch: BatchConfig::default(),
            queue_depth: DEFAULT_QUEUE_DEPTH,
            routing_filter: None,
            tags: BTreeMap::new(),
        };
        let err = match OtelGrpcSink::new(cfg) {
            Ok(_) => panic!("expected invalid-endpoint error"),
            Err(e) => e,
        };
        assert!(matches!(err, OtelGrpcError::InvalidEndpoint { .. }));
    }
}
