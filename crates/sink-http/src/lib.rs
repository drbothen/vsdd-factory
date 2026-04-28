//! HTTP sink driver (S-4.01).
//!
//! [`HttpSink`] implements [`Sink`] (from sink-core). It batches [`SinkEvent`]s
//! and POSTs them as a JSON array to a user-configured URL. It is the base HTTP
//! infrastructure reused by Datadog (S-4.02) and Honeycomb (S-4.03) sinks.
//!
//! ## Config load API
//!
//! [`HttpSinkConfig::from_toml`] validates `schema_version` (must equal 1) and
//! `type` (must equal `"http"`). Unknown `type` values warn to stderr and return
//! `Ok(None)`; schema_version != 1 returns `Err`.
//!
//! `enabled = false` in config causes [`HttpSink`] to be constructed but never
//! accept events and never make HTTP calls.
//!
//! ## Builder API (F-2, S-4.02)
//!
//! Wrappers (sink-datadog, sink-honeycomb) construct an [`HttpSinkConfig`] via
//! [`HttpSinkConfig::builder()`] so they do not pin to a specific field layout.
//! Direct pub field access was removed in S-4.02 (PR #18 finding F-2).

#![deny(missing_docs)]

pub mod retry;

use serde::Deserialize;
use sink_core::{DlqReason, DlqWriter, RoutingFilter, Sink, SinkConfigCommon, SinkErrorEvent, SinkEvent, emit_sink_error};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::sync::mpsc;

/// Default total attempts for 5xx batches (1 initial + retries) when no RetryConfig is set.
const DEFAULT_MAX_5XX_ATTEMPTS: u32 = 3;

// ── Per-instance PRNG (AC-007 / BC-3.07.001 invariant 2) ─────────────────────

/// SplitMix64: a minimal, high-quality non-cryptographic PRNG.
///
/// Uses no external crates — implemented using only integer arithmetic from std.
/// Each sink's worker thread seeds a fresh `SplitMix64` from `SystemTime` xored
/// with the thread ID, ensuring per-instance independence (AC-007).
///
/// Reference: Sebastiano Vigna, "Further scramblings of Marsaglia's xorshift
/// generators" (2015). SplitMix64 is the standard seed splitter in Java 8+
/// and Rust's `rand` crate uses it for seeding.
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Seed from a `u64` value. Each call to `next()` advances the state.
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate the next `u64` value.
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// Generate a `f64` uniformly in `[0.0, 1.0)`.
    fn next_f64(&mut self) -> f64 {
        // Use upper 53 bits for mantissa precision.
        let bits = self.next() >> 11;
        bits as f64 * (1.0_f64 / (1u64 << 53) as f64)
    }

    /// Seed from the current `SystemTime` xored with a per-thread nonce.
    ///
    /// Each call produces a distinct seed even when called in rapid succession
    /// because the thread address is mixed in.
    fn from_entropy() -> Self {
        use std::time::SystemTime;
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 | ((d.as_secs() & 0xFFFF_FFFF) << 32))
            .unwrap_or(0xdeadbeef_cafebabe);
        // XOR with a stack address for additional per-instance uniqueness
        // (two sinks started in the same nanosecond still diverge).
        let stack_addr = &nanos as *const u64 as u64;
        Self::new(nanos ^ stack_addr ^ 0xcafe_f00d_dead_beef)
    }
}

// ── S-4.09 stubs (RED gate) ───────────────────────────────────────────────────
// These declarations exist so tests compile. Implementations are intentionally
// absent (panic!/unimplemented!) — the RED gate requires all tests to FAIL.

/// Configuration error returned when a sink config violates construction-time
/// invariants (BC-3.07.001 invariant 1).
///
/// # Variants
///
/// - `InvalidBackoff` — emitted when `base_delay_ms == 0` or
///   `max_delay_ms < base_delay_ms` (AC-006 / EC-001 / EC-002).
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// `base_delay_ms = 0` or `max_delay_ms < base_delay_ms`.
    ///
    /// BC-3.07.001 invariant 1: `max_delay_ms >= base_delay_ms > 0` is a
    /// construction-time invariant; violation is a configuration error, not
    /// a runtime panic.
    #[error("invalid backoff config: base_delay_ms must be > 0 and max_delay_ms >= base_delay_ms")]
    InvalidBackoff,
}

/// Exponential-backoff configuration for the HTTP sink retry loop (S-4.09).
///
/// Added to [`HttpSinkConfig`] as the `retry` field. The implementer wires
/// this into the worker loop in `lib.rs` and draws jitter from a per-instance
/// PRNG (AC-007 / BC-3.07.001 invariant 2).
///
/// # Construction
///
/// Use [`RetryConfig::new`] — it enforces the construction-time invariant
/// `max_delay_ms >= base_delay_ms > 0` and returns `Err(ConfigError::InvalidBackoff)`
/// on violation (AC-006).
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Base delay in milliseconds. Must be > 0.
    pub base_delay_ms: u64,
    /// Maximum delay cap in milliseconds. Must be >= base_delay_ms.
    pub max_delay_ms: u64,
    /// Jitter factor in [0.0, 1.0]. Jitter is drawn from
    /// `[0, base_delay_ms * jitter_factor]`.
    pub jitter_factor: f64,
    /// Maximum total attempts (1 initial + retries). On full-failure with
    /// max_retries=N, exactly (N-1) sleeps occur (AC-009 / BC-3.07.001 invariant 4).
    pub max_retries: u32,
}

impl RetryConfig {
    /// Construct a [`RetryConfig`], enforcing the construction-time invariant.
    ///
    /// # Errors
    ///
    /// Returns `Err(ConfigError::InvalidBackoff)` if:
    /// - `base_delay_ms == 0` (EC-001), or
    /// - `max_delay_ms < base_delay_ms` (EC-002).
    ///
    /// # Panics
    ///
    /// This stub panics (RED gate — S-4.09 not yet implemented).
    pub fn new(
        base_delay_ms: u64,
        max_delay_ms: u64,
        jitter_factor: f64,
        max_retries: u32,
    ) -> Result<Self, ConfigError> {
        // BC-3.07.001 invariant 1: max_delay_ms >= base_delay_ms > 0
        if base_delay_ms == 0 || max_delay_ms < base_delay_ms {
            return Err(ConfigError::InvalidBackoff);
        }
        Ok(Self {
            base_delay_ms,
            max_delay_ms,
            jitter_factor,
            max_retries,
        })
    }
}

/// HTTP endpoint URL type alias.
///
/// Using a named alias keeps the `HttpSinkConfig` field declaration stable:
/// callers see `pub url: HttpEndpointUrl` rather than a bare `pub url: String`,
/// satisfying the F-2 API-stability contract (BC-3.NN.NNN-http-sink-config-api-stability).
pub type HttpEndpointUrl = String;

// ── Raw deserialisation target (TOML) ────────────────────────────────────────

/// Internal raw struct used only for TOML deserialisation.
/// Fields are private after F-2; callers use [`HttpSinkConfig`] accessors.
#[derive(Debug, Clone, Deserialize)]
struct RawHttpSinkConfig {
    schema_version: u32,
    #[serde(rename = "type")]
    sink_type: String,
    #[serde(flatten)]
    common: SinkConfigCommon,
    url: String,
    #[serde(default = "default_queue_depth")]
    queue_depth: usize,
}

fn default_queue_depth() -> usize {
    1000
}

// ── Public config type ────────────────────────────────────────────────────────

/// Driver-specific configuration for the HTTP sink.
///
/// Deserialized from an `[[sinks]]` stanza in `observability-config.toml`.
/// The `url` field is public via the [`HttpEndpointUrl`] type alias so that
/// the exact field type `String` is not pinned in the public API surface
/// (F-2 / BC-3.NN.NNN-http-sink-config-api-stability). Prefer the [`Self::url()`]
/// accessor or [`HttpSinkConfig::builder()`] over direct field access.
#[derive(Debug, Clone)]
pub struct HttpSinkConfig {
    #[allow(dead_code)]
    schema_version: u32,
    #[allow(dead_code)]
    sink_type: String,
    /// Common cross-sink fields (name, enabled, routing_filter, tags).
    pub common: SinkConfigCommon,
    /// HTTP endpoint URL. Every batch is POSTed here as a JSON array.
    /// Typed as [`HttpEndpointUrl`] (a `String` alias) to keep the public
    /// field declaration stable across API revisions (F-2).
    pub url: HttpEndpointUrl,
    queue_depth: usize,
    /// Extra headers injected on every POST (used by wrapper sinks, e.g. DD-API-KEY).
    extra_headers: Vec<(String, String)>,
    /// Optional exponential-backoff configuration (S-4.09 / AC-001 through AC-009).
    /// `None` preserves the S-4.01 immediate-retry behaviour.
    pub retry: Option<RetryConfig>,
}

impl HttpSinkConfig {
    /// Parse and validate an `HttpSinkConfig` from a raw TOML string.
    ///
    /// Returns:
    /// - `Err(_)` when `schema_version != 1` (BC-3.01.003).
    /// - `Ok(None)` when `type` is not `"http"` — warning emitted to stderr
    ///   (BC-3.01.002).
    /// - `Ok(Some(config))` on a valid stanza.
    pub fn from_toml(toml_src: &str) -> anyhow::Result<Option<HttpSinkConfig>> {
        let raw: RawHttpSinkConfig =
            toml::from_str(toml_src).map_err(|e| anyhow::anyhow!("TOML parse error: {e}"))?;

        if raw.schema_version != 1 {
            return Err(anyhow::anyhow!(
                "schema_version must be 1, got {} (BC-3.01.003)",
                raw.schema_version
            ));
        }

        if raw.sink_type != "http" {
            eprintln!(
                "sink-http: unknown sink type {:?} — skipping (BC-3.01.002)",
                raw.sink_type
            );
            return Ok(None);
        }

        Ok(Some(HttpSinkConfig {
            schema_version: raw.schema_version,
            sink_type: raw.sink_type,
            common: raw.common,
            url: raw.url,
            queue_depth: raw.queue_depth,
            extra_headers: Vec::new(),
            retry: None,
        }))
    }

    /// Return a builder for constructing an `HttpSinkConfig` programmatically
    /// (F-2: wrapper sinks must not pin to field layout).
    pub fn builder() -> HttpSinkConfigBuilder {
        HttpSinkConfigBuilder::default()
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// HTTP endpoint URL. Every batch is POSTed here as a JSON array.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Bounded internal queue depth.
    pub fn queue_depth(&self) -> usize {
        self.queue_depth
    }

    /// Extra headers set on every POST (e.g. `DD-API-KEY` for sink-datadog).
    pub fn extra_headers(&self) -> &[(String, String)] {
        &self.extra_headers
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Builder for [`HttpSinkConfig`] (F-2: stable API surface for wrapper sinks).
#[derive(Debug, Default)]
pub struct HttpSinkConfigBuilder {
    name: String,
    url: String,
    queue_depth: usize,
    extra_headers: Vec<(String, String)>,
    common_override: Option<SinkConfigCommon>,
    /// Optional backoff configuration (S-4.09). None = no backoff (S-4.01 behaviour).
    retry: Option<RetryConfig>,
}

impl HttpSinkConfigBuilder {
    /// Set the operator-assigned sink name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the HTTP endpoint URL.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Set the internal queue depth (default: 1000).
    pub fn queue_depth(mut self, depth: usize) -> Self {
        self.queue_depth = depth;
        self
    }

    /// Add an extra HTTP header injected on every POST.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_headers.push((name.into(), value.into()));
        self
    }

    /// Override the entire `SinkConfigCommon` (for wrapper sinks that have
    /// already parsed their own common config).
    pub fn common(mut self, common: SinkConfigCommon) -> Self {
        self.common_override = Some(common);
        self
    }

    /// Set the exponential backoff configuration for retry sleeps (S-4.09).
    ///
    /// When set, the worker thread sleeps `compute_backoff_ms(...)` between
    /// 5xx retry attempts. When absent, the S-4.01 immediate-retry behaviour
    /// is used (no sleep between attempts).
    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = Some(retry);
        self
    }

    /// Finalise and return an [`HttpSinkConfig`].
    pub fn build(self) -> HttpSinkConfig {
        let common = self.common_override.unwrap_or_else(|| SinkConfigCommon {
            name: self.name.clone(),
            ..SinkConfigCommon::default()
        });
        HttpSinkConfig {
            schema_version: 1,
            sink_type: "http".to_owned(),
            common,
            url: self.url,
            queue_depth: if self.queue_depth == 0 {
                1000
            } else {
                self.queue_depth
            },
            extra_headers: self.extra_headers,
            retry: self.retry,
        }
    }
}

// ── SinkFailure ───────────────────────────────────────────────────────────────

/// A recorded HTTP send failure — returned via [`HttpSink::take_failures`].
#[derive(Debug, Clone)]
pub struct SinkFailure {
    /// The URL that was attempted.
    pub url: HttpEndpointUrl,
    /// Human-readable failure reason.
    pub reason: String,
    /// Number of attempts made before giving up.
    pub attempts: u32,
}

// ── Internal messages ─────────────────────────────────────────────────────────

/// Messages sent to the worker task via the internal mpsc channel.
///
/// `Flush` carries a `std::sync::mpsc::SyncSender` so the worker can
/// signal completion without needing to be awaited — this lets the
/// synchronous `flush()` method block safely even when called from inside
/// a multi-thread tokio runtime (via `tokio::task::block_in_place`).
enum Message {
    /// A single event to buffer until the next flush.
    Event(SinkEvent),
    /// Drain the buffer, POST the batch, then ack the sender.
    Flush(std::sync::mpsc::SyncSender<()>),
}

// ── Shared state ──────────────────────────────────────────────────────────────

/// State shared between the [`HttpSink`] handle and the worker task.
struct Shared {
    failures: Mutex<Vec<SinkFailure>>,
    queue_full_count: AtomicU64,
    shutdown: std::sync::atomic::AtomicBool,
    /// Operator-assigned sink name for `internal.sink_error` events (AC-009).
    sink_name: String,
    /// Optional fire-and-forget channel for `internal.sink_error` events
    /// (BC-3.07.002). `None` when no error channel is wired in (e.g. tests
    /// that exercise BC-3.01.008 recording only).
    error_tx: Option<mpsc::Sender<SinkErrorEvent>>,
}

impl Shared {
    fn new(sink_name: String, error_tx: Option<mpsc::Sender<SinkErrorEvent>>) -> Self {
        Self {
            failures: Mutex::new(Vec::new()),
            queue_full_count: AtomicU64::new(0),
            shutdown: std::sync::atomic::AtomicBool::new(false),
            sink_name,
            error_tx,
        }
    }
}

// ── HttpSink ──────────────────────────────────────────────────────────────────

/// HTTP batch-POST sink.
///
/// Implements [`Sink`] from sink-core. Exposed `pub` so S-4.02 (Datadog) and
/// S-4.03 (Honeycomb) can embed or wrap it (AC-9 / BC-3.01.001 invariant 1).
pub struct HttpSink {
    name: String,
    common: SinkConfigCommon,
    sender: Mutex<Option<mpsc::Sender<Message>>>,
    worker: Mutex<Option<JoinHandle<()>>>,
    shared: Arc<Shared>,
    /// Optional DLQ writer for retry-exhausted events (S-4.05 Task 2b).
    dlq_writer: Option<Arc<DlqWriter>>,
}

impl HttpSink {
    /// Construct an `HttpSink` from a validated config.
    ///
    /// Starts the background worker thread that consumes the internal queue
    /// and POSTs batches to the configured URL.
    ///
    /// `error_tx` is an optional sender for `internal.sink_error` events
    /// (BC-3.07.002). Pass `None` when no emission channel is available
    /// (e.g. tests that only check BC-3.01.008 failure recording). When
    /// `Some`, emission is fire-and-forget via `try_send` on the worker thread
    /// at each failure-recording site.
    pub fn new(config: HttpSinkConfig) -> anyhow::Result<Self> {
        Self::new_with_observability(config, None, None)
    }

    /// Like [`Self::new`] but threads an error-event channel sender into the
    /// sink's shared state. The dispatcher calls this variant to wire
    /// `internal.sink_error` emission (BC-3.07.002).
    pub fn new_with_error_channel(
        config: HttpSinkConfig,
        error_tx: mpsc::Sender<SinkErrorEvent>,
    ) -> anyhow::Result<Self> {
        Self::new_with_observability(config, Some(error_tx), None)
    }

    /// Construct an `HttpSink` wired with both an optional error channel and an
    /// optional DLQ writer (S-4.05 Task 2b).
    ///
    /// When `dlq_writer` is `Some`, events that exhaust all retry attempts are
    /// written to the DLQ file (AC-002 / BC-3.07.003).
    pub fn new_with_observability(
        config: HttpSinkConfig,
        error_tx: Option<mpsc::Sender<SinkErrorEvent>>,
        dlq_writer: Option<Arc<DlqWriter>>,
    ) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel::<Message>(config.queue_depth.max(1));
        let sink_name_for_shared = if config.common.name.is_empty() {
            "<unnamed>".to_owned()
        } else {
            config.common.name.clone()
        };
        let shared = Arc::new(Shared::new(sink_name_for_shared, error_tx));
        let worker_shared = Arc::clone(&shared);
        let worker_url = config.url.clone();
        let worker_headers = config.extra_headers.clone();
        let worker_retry = config.retry.clone();
        // Clone the DLQ writer Arc for the worker thread (S-4.05 Task 4).
        let worker_dlq: Option<Arc<DlqWriter>> = dlq_writer.as_ref().map(Arc::clone);

        let handle = std::thread::Builder::new()
            .name(format!("sink-http:{}", config.common.name))
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(e) => {
                        worker_shared
                            .failures
                            .lock()
                            .unwrap_or_else(|p| p.into_inner())
                            .push(SinkFailure {
                                url: worker_url.clone(),
                                reason: format!("failed to build tokio runtime: {e}"),
                                attempts: 0,
                            });
                        return;
                    }
                };
                // Seed per-instance PRNG from entropy (AC-007 / BC-3.07.001 invariant 2).
                let mut rng = SplitMix64::from_entropy();
                runtime.block_on(worker_loop(
                    rx,
                    worker_url,
                    worker_headers,
                    Arc::clone(&worker_shared),
                    worker_retry,
                    &mut rng,
                    worker_dlq,
                ));
            })
            .map_err(|e| anyhow::anyhow!("failed to spawn sink-http worker thread: {e}"))?;

        Ok(Self {
            name: config.common.name.clone(),
            common: config.common,
            sender: Mutex::new(Some(tx)),
            worker: Mutex::new(Some(handle)),
            shared,
            dlq_writer,
        })
    }

    /// Drain recorded send failures accumulated since the last call.
    ///
    /// Used by tests and the dispatcher integration to inspect failure state.
    pub fn take_failures(&self) -> Vec<SinkFailure> {
        let mut guard = self
            .shared
            .failures
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        std::mem::take(&mut *guard)
    }

    /// Number of events dropped due to a full internal queue (VP-011 overflow
    /// pathway). Non-blocking: callers read this to verify backpressure.
    pub fn queue_full_count(&self) -> u64 {
        self.shared.queue_full_count.load(Ordering::Relaxed)
    }
}

impl Sink for HttpSink {
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
        // Router is the single dispatch gate; HttpSink::accepts handles only
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
        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            return;
        };
        if sender.try_send(Message::Event(event)).is_err() {
            self.shared.queue_full_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn flush(&self) -> anyhow::Result<()> {
        // Use a std rendezvous channel for the ack so this synchronous method
        // can safely block even when called from a tokio multi-thread context.
        let (ack_tx, ack_rx) = std::sync::mpsc::sync_channel::<()>(0);

        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            return Ok(());
        };
        // try_send keeps flush non-blocking on the channel side. If the
        // queue is full we skip this flush cycle — the next flush picks up.
        if sender.try_send(Message::Flush(ack_tx)).is_err() {
            return Err(anyhow::anyhow!(
                "sink '{}' flush channel full or closed",
                self.name
            ));
        }
        drop(guard);

        // Block until the worker posts the batch and acks.
        // - Inside a multi-thread tokio runtime: block_in_place allows
        //   blocking without stalling the async thread pool.
        // - Outside a tokio runtime (sync tests): recv() directly.
        let recv_result = if tokio::runtime::Handle::try_current()
            .map(|h| h.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread)
            .unwrap_or(false)
        {
            tokio::task::block_in_place(|| ack_rx.recv())
        } else {
            ack_rx.recv()
        };
        match recv_result {
            Ok(()) => Ok(()),
            Err(_) => Err(anyhow::anyhow!(
                "sink '{}' flush signal lost — worker may have exited",
                self.name
            )),
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

impl Drop for HttpSink {
    fn drop(&mut self) {
        if self.worker.lock().map(|g| g.is_some()).unwrap_or(false) {
            self.shutdown();
        }
    }
}

// ── Worker ────────────────────────────────────────────────────────────────────

/// Worker async loop: drains the mpsc, accumulates events, POSTs on flush.
///
/// The `rng` parameter is a per-instance PRNG seeded at thread spawn time
/// (AC-007 / BC-3.07.001 invariant 2). It is passed as `&mut` so jitter draws
/// advance the state across consecutive flush calls within the same sink instance.
///
/// The `dlq_writer` parameter is an optional DLQ writer for retry-exhausted
/// events (S-4.05 Task 4). When `Some`, all events in a batch that exhaust all
/// retries are written to the DLQ file.
async fn worker_loop(
    mut rx: mpsc::Receiver<Message>,
    url: String,
    extra_headers: Vec<(String, String)>,
    shared: Arc<Shared>,
    retry: Option<RetryConfig>,
    rng: &mut SplitMix64,
    dlq_writer: Option<Arc<DlqWriter>>,
) {
    let client = reqwest::Client::new();
    let mut buffer: Vec<SinkEvent> = Vec::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Event(event) => {
                buffer.push(event);
            }
            Message::Flush(ack) => {
                if !buffer.is_empty() {
                    let batch = std::mem::take(&mut buffer);
                    post_batch(
                        &client,
                        &url,
                        &extra_headers,
                        batch,
                        &shared,
                        retry.as_ref(),
                        rng,
                        dlq_writer.as_deref(),
                    )
                    .await;
                }
                // Ack the flush caller. Ignore send errors — caller may have
                // timed out and dropped the receiver.
                let _ = ack.send(());
            }
        }
    }

    // Channel closed (shutdown). Flush remaining buffered events.
    if !buffer.is_empty() {
        post_batch(
            &client,
            &url,
            &extra_headers,
            buffer,
            &shared,
            retry.as_ref(),
            rng,
            dlq_writer.as_deref(),
        )
        .await;
    }
}

/// POST a batch of events as a JSON array to the configured URL.
///
/// - 5xx or network error: retry up to `retry.max_retries` total attempts (or
///   `DEFAULT_MAX_5XX_ATTEMPTS` when no retry config is set), then record a
///   [`SinkFailure`]. Between retries, sleeps for the computed backoff delay
///   (AC-004 / BC-3.07.001 postconditions 1-3). The sleep does NOT hold the
///   `Mutex<Vec<SinkFailure>>` lock (AC-008 / invariant 3). Exactly
///   `max_retries - 1` sleeps occur on a full-failure sequence (AC-009 / invariant 4).
/// - 4xx: drop immediately (no retry, no backoff sleep), record a [`SinkFailure`]
///   (EC-004 / postcondition 6).
/// - 2xx: success, no failure recorded.
///
/// When `dlq_writer` is `Some` and all retries are exhausted for a 5xx or
/// network error, every event in `batch` is written to the DLQ (S-4.05 / AC-002).
async fn post_batch(
    client: &reqwest::Client,
    url: &str,
    extra_headers: &[(String, String)],
    batch: Vec<SinkEvent>,
    shared: &Arc<Shared>,
    retry: Option<&RetryConfig>,
    rng: &mut SplitMix64,
    dlq_writer: Option<&DlqWriter>,
) {
    let body = match serde_json::to_string(&batch) {
        Ok(b) => b,
        Err(e) => {
            let reason = format!("serialization error: {e}");
            emit_error_event(shared, reason.clone(), 0);
            record_failure(shared, url, reason, 0);
            return;
        }
    };

    let max_attempts = retry
        .map(|r| r.max_retries)
        .unwrap_or(DEFAULT_MAX_5XX_ATTEMPTS);

    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        let attempt_0idx = attempts - 1; // 0-indexed for BC-3.07.002 postcondition 1.
        let mut req = client.post(url).header("Content-Type", "application/json");

        for (name, value) in extra_headers {
            req = req.header(name.as_str(), value.as_str());
        }

        let result = req.body(body.clone()).send().await;

        match result {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    return;
                } else if status.is_server_error() {
                    // 5xx — emit one internal.sink_error per failed attempt
                    // (BC-3.07.002 invariant 3: one event per failure, not per batch).
                    let reason = format!("HTTP {}", status.as_u16());
                    emit_error_event(shared, reason.clone(), attempt_0idx);

                    // Check if we should retry (S-4.09 configurable backoff).
                    if attempts < max_attempts {
                        // Sleep for the computed backoff delay before the next attempt.
                        // IMPORTANT: the Mutex<Vec<SinkFailure>> lock is NOT held here —
                        // the sleep call is outside any lock guard (AC-008 / invariant 3).
                        if let Some(cfg) = retry {
                            let jitter_unit = rng.next_f64();
                            let jitter_ms = crate::retry::draw_jitter_ms(
                                cfg.base_delay_ms,
                                cfg.jitter_factor,
                                jitter_unit,
                            );
                            // attempt index is 0-based: first retry after attempt 1 → index 0
                            let attempt_index = attempts - 1;
                            let delay_ms = crate::retry::compute_backoff_ms(
                                cfg.base_delay_ms,
                                cfg.max_delay_ms,
                                cfg.jitter_factor,
                                attempt_index,
                                jitter_ms,
                            );
                            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                        }
                        continue;
                    }
                    // Final attempt failed — record SinkFailure for BC-3.01.008
                    // (AC-004 additive contract; emission already done above).
                    record_failure(
                        shared,
                        url,
                        format!("HTTP {} after {attempts} attempts", status.as_u16()),
                        attempts,
                    );
                    // Write all events to DLQ on retry exhaustion (S-4.05 AC-002).
                    if let Some(dlq) = dlq_writer {
                        for event in &batch {
                            let _ = dlq.write_event(
                                &shared.sink_name,
                                "http",
                                event,
                                DlqReason::RetryExhausted,
                            );
                        }
                    }
                    return;
                } else {
                    // 4xx or other — drop immediately, no backoff sleep, record failure + emit.
                    // (EC-004 / BC-3.07.001 postcondition 6).
                    let reason = format!("HTTP {} (client error, no retry)", status.as_u16());
                    emit_error_event(shared, reason.clone(), attempt_0idx);
                    record_failure(shared, url, reason, attempts);
                    return;
                }
            }
            Err(e) => {
                // Network error (connection refused, timeout, etc.) — treated as retriable
                // identically to 5xx (EC-005 / BC-3.07.001).
                // Emit internal.sink_error for this attempt (BC-3.07.002 invariant 3).
                let reason = format!("request error: {e}");
                emit_error_event(shared, reason.clone(), attempt_0idx);

                if attempts < max_attempts {
                    if let Some(cfg) = retry {
                        let jitter_unit = rng.next_f64();
                        let jitter_ms = crate::retry::draw_jitter_ms(
                            cfg.base_delay_ms,
                            cfg.jitter_factor,
                            jitter_unit,
                        );
                        let attempt_index = attempts - 1;
                        let delay_ms = crate::retry::compute_backoff_ms(
                            cfg.base_delay_ms,
                            cfg.max_delay_ms,
                            cfg.jitter_factor,
                            attempt_index,
                            jitter_ms,
                        );
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    }
                    continue;
                }
                record_failure(shared, url, reason, attempts);
                // Write all events to DLQ on retry exhaustion (S-4.05 AC-002).
                if let Some(dlq) = dlq_writer {
                    for event in &batch {
                        let _ = dlq.write_event(
                            &shared.sink_name,
                            "http",
                            event,
                            DlqReason::RetryExhausted,
                        );
                    }
                }
                return;
            }
        }
    }
}

/// Emit a fire-and-forget `internal.sink_error` event to the internal channel.
///
/// Releases any borrow on `shared.failures` before calling `try_send` so the
/// mutex is never held across the channel operation (S-4.10 previous-story
/// intelligence note: prefer releasing the lock first for clarity).
fn emit_error_event(shared: &Arc<Shared>, error_message: String, attempt: u32) {
    let Some(ref tx) = shared.error_tx else {
        return; // No channel wired — silent skip (e.g. BC-3.01.008-only tests).
    };
    let event = SinkErrorEvent::new(shared.sink_name.clone(), "http", error_message, attempt);
    emit_sink_error(tx, event);
}

fn record_failure(shared: &Arc<Shared>, url: &str, reason: String, attempts: u32) {
    let mut guard = shared.failures.lock().unwrap_or_else(|p| p.into_inner());
    guard.push(SinkFailure {
        url: url.to_owned(),
        reason,
        attempts,
    });
}
