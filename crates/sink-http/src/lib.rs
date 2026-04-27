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

#![deny(missing_docs)]

use serde::Deserialize;
use sink_core::{Sink, SinkConfigCommon, SinkEvent};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::sync::mpsc;

/// Number of total attempts for 5xx batches (1 initial + retries).
const MAX_5XX_ATTEMPTS: u32 = 3;

/// Driver-specific configuration for the HTTP sink.
///
/// Deserialized from an `[[sinks]]` stanza in `observability-config.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpSinkConfig {
    /// Must equal `1`. Any other value is a hard error at load time
    /// (BC-3.01.003).
    pub schema_version: u32,

    /// Must equal `"http"`. Unknown values warn to stderr and cause the
    /// sink to be skipped (BC-3.01.002).
    #[serde(rename = "type")]
    pub sink_type: String,

    /// Common cross-sink fields (name, enabled, routing_filter, tags).
    #[serde(flatten)]
    pub common: SinkConfigCommon,

    /// HTTP endpoint URL. Every batch is POSTed here as a JSON array.
    pub url: String,

    /// Bounded internal queue depth. Overflow drops events without blocking
    /// the caller (VP-011).
    #[serde(default = "default_queue_depth")]
    pub queue_depth: usize,
}

fn default_queue_depth() -> usize {
    1000
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
        let config: HttpSinkConfig =
            toml::from_str(toml_src).map_err(|e| anyhow::anyhow!("TOML parse error: {e}"))?;

        if config.schema_version != 1 {
            return Err(anyhow::anyhow!(
                "schema_version must be 1, got {} (BC-3.01.003)",
                config.schema_version
            ));
        }

        if config.sink_type != "http" {
            eprintln!(
                "sink-http: unknown sink type {:?} — skipping (BC-3.01.002)",
                config.sink_type
            );
            return Ok(None);
        }

        Ok(Some(config))
    }
}

/// A recorded HTTP send failure — returned via [`HttpSink::take_failures`].
#[derive(Debug, Clone)]
pub struct SinkFailure {
    /// The URL that was attempted.
    pub url: String,
    /// Human-readable failure reason.
    pub reason: String,
    /// Number of attempts made before giving up.
    pub attempts: u32,
}

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

/// State shared between the [`HttpSink`] handle and the worker task.
struct Shared {
    failures: Mutex<Vec<SinkFailure>>,
    queue_full_count: AtomicU64,
    shutdown: std::sync::atomic::AtomicBool,
}

impl Shared {
    fn new() -> Self {
        Self {
            failures: Mutex::new(Vec::new()),
            queue_full_count: AtomicU64::new(0),
            shutdown: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

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
}

impl HttpSink {
    /// Construct an `HttpSink` from a validated config.
    ///
    /// Starts the background worker thread that consumes the internal queue
    /// and POSTs batches to the configured URL.
    pub fn new(config: HttpSinkConfig) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel::<Message>(config.queue_depth.max(1));
        let shared = Arc::new(Shared::new());
        let worker_shared = Arc::clone(&shared);
        let worker_url = config.url.clone();

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
                runtime.block_on(worker_loop(rx, worker_url, Arc::clone(&worker_shared)));
            })
            .map_err(|e| anyhow::anyhow!("failed to spawn sink-http worker thread: {e}"))?;

        Ok(Self {
            name: config.common.name.clone(),
            common: config.common,
            sender: Mutex::new(Some(tx)),
            worker: Mutex::new(Some(handle)),
            shared,
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
        if let Some(filter) = self.common.routing_filter.as_ref() {
            let event_type = event.event_type().unwrap_or("");
            return filter.accepts(event_type);
        }
        true
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

/// Worker async loop: drains the mpsc, accumulates events, POSTs on flush.
async fn worker_loop(mut rx: mpsc::Receiver<Message>, url: String, shared: Arc<Shared>) {
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
                    post_batch(&client, &url, batch, &shared).await;
                }
                // Ack the flush caller. Ignore send errors — caller may have
                // timed out and dropped the receiver.
                let _ = ack.send(());
            }
        }
    }

    // Channel closed (shutdown). Flush remaining buffered events.
    if !buffer.is_empty() {
        post_batch(&client, &url, buffer, &shared).await;
    }
}

/// POST a batch of events as a JSON array to the configured URL.
///
/// - 5xx or network error: retry up to `MAX_5XX_ATTEMPTS` total, then record
///   a [`SinkFailure`].
/// - 4xx: drop immediately (no retry), record a [`SinkFailure`].
/// - 2xx: success, no failure recorded.
async fn post_batch(
    client: &reqwest::Client,
    url: &str,
    batch: Vec<SinkEvent>,
    shared: &Arc<Shared>,
) {
    let body = match serde_json::to_string(&batch) {
        Ok(b) => b,
        Err(e) => {
            record_failure(shared, url, format!("serialization error: {e}"), 0);
            return;
        }
    };

    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        let result = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.clone())
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    return;
                } else if status.is_server_error() {
                    // 5xx — retry until MAX_5XX_ATTEMPTS exhausted.
                    if attempts < MAX_5XX_ATTEMPTS {
                        continue;
                    }
                    record_failure(
                        shared,
                        url,
                        format!("HTTP {} after {attempts} attempts", status.as_u16()),
                        attempts,
                    );
                    return;
                } else {
                    // 4xx or other — drop immediately, record failure.
                    record_failure(
                        shared,
                        url,
                        format!("HTTP {} (client error, no retry)", status.as_u16()),
                        attempts,
                    );
                    return;
                }
            }
            Err(e) => {
                if attempts < MAX_5XX_ATTEMPTS {
                    continue;
                }
                record_failure(shared, url, format!("request error: {e}"), attempts);
                return;
            }
        }
    }
}

fn record_failure(shared: &Arc<Shared>, url: &str, reason: String, attempts: u32) {
    let mut guard = shared.failures.lock().unwrap_or_else(|p| p.into_inner());
    guard.push(SinkFailure {
        url: url.to_owned(),
        reason,
        attempts,
    });
}
