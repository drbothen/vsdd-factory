//! Honeycomb Events API sink driver (S-4.03).
//!
//! [`HoneycombSink`] implements [`Sink`] (from sink-core) by wrapping
//! [`HttpSink`] (from sink-http). It targets the Honeycomb Events API:
//!
//! ```text
//! POST https://api.honeycomb.io/1/events/<dataset>
//! X-Honeycomb-Team: <api_key>
//! Content-Type: application/json
//! ```
//!
//! Each event is enriched with a `time` field (RFC3339) before dispatch.
//!
//! ## Config load API
//!
//! [`HoneycombSinkConfig::from_toml`] validates:
//! - `type` must equal `"honeycomb"` (otherwise `Ok(None)`)
//! - `api_key` must be present and non-empty (hard error)
//! - `dataset` must be present and non-empty (hard error, EC-001)

#![deny(missing_docs)]

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde_json::Value;
use sink_core::{Sink, SinkConfigCommon, SinkEvent};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::sync::mpsc;

/// Honeycomb base URL (without dataset path component).
pub const HONEYCOMB_BASE_URL: &str = "https://api.honeycomb.io/1/events";

/// Number of total attempts for 5xx batches (1 initial + retries).
const MAX_5XX_ATTEMPTS: u32 = 3;

/// Driver-specific configuration for the Honeycomb sink.
///
/// Deserialized from an `[[sinks]]` stanza in `observability-config.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct HoneycombSinkConfig {
    /// Must equal `"honeycomb"`. Unknown values cause the sink to be
    /// skipped (returns `Ok(None)` from `from_toml`).
    #[serde(rename = "type")]
    pub sink_type: String,

    /// Common cross-sink fields (name, enabled, routing_filter, tags).
    #[serde(flatten)]
    pub common: SinkConfigCommon,

    /// Honeycomb API key. Sent as `X-Honeycomb-Team` header.
    /// Missing or empty is a hard error at config load time.
    pub api_key: Option<String>,

    /// Dataset name. Embedded in the URL path as `/1/events/<dataset>`.
    /// Missing or empty is a hard error at config load time (EC-001).
    pub dataset: Option<String>,

    /// Base URL override for testing. Defaults to [`HONEYCOMB_BASE_URL`].
    #[serde(default)]
    pub base_url: Option<String>,

    /// Bounded internal queue depth; defaults to 1000.
    #[serde(default = "default_queue_depth")]
    pub queue_depth: usize,
}

fn default_queue_depth() -> usize {
    1000
}

impl HoneycombSinkConfig {
    /// Parse and validate a `HoneycombSinkConfig` from a raw TOML string.
    ///
    /// Returns:
    /// - `Ok(None)` when `type != "honeycomb"` — warning emitted to stderr.
    /// - `Err(_)` when `api_key` is absent or empty (BC-3.NN.NNN-honeycomb-api-key-required).
    /// - `Err(_)` when `dataset` is absent or empty (EC-001,
    ///   BC-3.NN.NNN-honeycomb-dataset-url-routing).
    /// - `Ok(Some(config))` on a valid stanza.
    pub fn from_toml(toml_src: &str) -> anyhow::Result<Option<HoneycombSinkConfig>> {
        let config: HoneycombSinkConfig =
            toml::from_str(toml_src).map_err(|e| anyhow::anyhow!("TOML parse error: {e}"))?;

        if config.sink_type != "honeycomb" {
            eprintln!(
                "sink-honeycomb: unknown sink type {:?} — skipping",
                config.sink_type
            );
            return Ok(None);
        }

        // Validate api_key: must be present and non-empty/non-whitespace.
        match config.api_key.as_deref() {
            None | Some("") => {
                return Err(anyhow::anyhow!(
                    "api_key is required for Honeycomb sink (BC-3.NN.NNN-honeycomb-api-key-required)"
                ));
            }
            Some(key) if key.trim().is_empty() => {
                return Err(anyhow::anyhow!(
                    "api_key must not be blank for Honeycomb sink (BC-3.NN.NNN-honeycomb-api-key-required)"
                ));
            }
            _ => {}
        }

        // Validate dataset: must be present and non-empty/non-whitespace (EC-001).
        match config.dataset.as_deref() {
            None | Some("") => {
                return Err(anyhow::anyhow!(
                    "dataset is required for Honeycomb sink (EC-001, BC-3.NN.NNN-honeycomb-dataset-url-routing)"
                ));
            }
            Some(ds) if ds.trim().is_empty() => {
                return Err(anyhow::anyhow!(
                    "dataset must not be blank for Honeycomb sink (EC-001, BC-3.NN.NNN-honeycomb-dataset-url-routing)"
                ));
            }
            _ => {}
        }

        Ok(Some(config))
    }

    /// Build the full Honeycomb endpoint URL by appending the dataset to the
    /// base URL.
    ///
    /// e.g. `https://api.honeycomb.io/1/events/my-dataset`
    pub fn endpoint_url(&self) -> String {
        let base = self.base_url.as_deref().unwrap_or(HONEYCOMB_BASE_URL);
        let dataset = self.dataset.as_deref().unwrap_or("");
        format!("{base}/{dataset}")
    }
}

// ---------------------------------------------------------------------------
// Internal worker message types
// ---------------------------------------------------------------------------

/// Messages sent to the background worker via the internal mpsc channel.
enum Message {
    /// A single event to buffer until the next flush.
    Event(SinkEvent),
    /// Drain the buffer, POST the batch, then ack the sender.
    Flush(std::sync::mpsc::SyncSender<()>),
}

// ---------------------------------------------------------------------------
// Shared state between handle and worker
// ---------------------------------------------------------------------------

struct Shared {
    failures: Mutex<Vec<WorkerFailure>>,
    queue_full_count: AtomicU64,
    shutdown: AtomicBool,
}

impl Shared {
    fn new() -> Self {
        Self {
            failures: Mutex::new(Vec::new()),
            queue_full_count: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
        }
    }
}

/// A recorded send failure (internal diagnostic).
struct WorkerFailure {
    #[allow(dead_code)]
    reason: String,
}

// ---------------------------------------------------------------------------
// HoneycombSink
// ---------------------------------------------------------------------------

/// Honeycomb Events API sink.
///
/// Wraps [`sink_http::HttpSink`] and injects:
/// - Correct endpoint URL (`/1/events/<dataset>`)
/// - `X-Honeycomb-Team: <api_key>` auth header
/// - `time` field in RFC3339 on each event (BC-3.NN.NNN-honeycomb-time-field-rfc3339)
pub struct HoneycombSink {
    name: String,
    common: SinkConfigCommon,
    sender: Mutex<Option<mpsc::Sender<Message>>>,
    worker: Mutex<Option<JoinHandle<()>>>,
    shared: Arc<Shared>,
}

impl HoneycombSink {
    /// Construct a `HoneycombSink` from a validated config.
    pub fn new(config: HoneycombSinkConfig) -> anyhow::Result<Self> {
        let url = config.endpoint_url();
        let api_key = config
            .api_key
            .clone()
            .expect("api_key validated present by from_toml");

        let (tx, rx) = mpsc::channel::<Message>(config.queue_depth.max(1));
        let shared = Arc::new(Shared::new());
        let worker_shared = Arc::clone(&shared);

        let handle = std::thread::Builder::new()
            .name(format!("sink-honeycomb:{}", config.common.name))
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
                            .push(WorkerFailure {
                                reason: format!("failed to build tokio runtime: {e}"),
                            });
                        return;
                    }
                };
                runtime.block_on(worker_loop(rx, url, api_key, Arc::clone(&worker_shared)));
            })
            .map_err(|e| anyhow::anyhow!("failed to spawn sink-honeycomb worker thread: {e}"))?;

        Ok(Self {
            name: config.common.name.clone(),
            common: config.common,
            sender: Mutex::new(Some(tx)),
            worker: Mutex::new(Some(handle)),
            shared,
        })
    }
}

impl Sink for HoneycombSink {
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
        let (ack_tx, ack_rx) = std::sync::mpsc::sync_channel::<()>(0);

        let guard = self.sender.lock().unwrap_or_else(|p| p.into_inner());
        let Some(sender) = guard.as_ref() else {
            return Ok(());
        };
        if sender.try_send(Message::Flush(ack_tx)).is_err() {
            return Err(anyhow::anyhow!(
                "sink '{}' flush channel full or closed",
                self.name
            ));
        }
        drop(guard);

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

impl Drop for HoneycombSink {
    fn drop(&mut self) {
        if self.worker.lock().map(|g| g.is_some()).unwrap_or(false) {
            self.shutdown();
        }
    }
}

// ---------------------------------------------------------------------------
// Worker async loop
// ---------------------------------------------------------------------------

/// Worker: drain the mpsc, accumulate events, POST on flush.
async fn worker_loop(
    mut rx: mpsc::Receiver<Message>,
    url: String,
    api_key: String,
    shared: Arc<Shared>,
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
                    post_batch(&client, &url, &api_key, batch, &shared).await;
                }
                let _ = ack.send(());
            }
        }
    }

    // Channel closed (shutdown). Flush remaining buffered events.
    if !buffer.is_empty() {
        post_batch(&client, &url, &api_key, buffer, &shared).await;
    }
}

/// Inject a `time` field (RFC3339) derived from `ts_epoch` or wall clock.
fn enrich_event(event: SinkEvent) -> SinkEvent {
    let ts: DateTime<Utc> = match event.fields.get("ts_epoch") {
        Some(Value::Number(n)) => {
            if let Some(secs) = n.as_i64() {
                Utc.timestamp_opt(secs, 0).single().unwrap_or_else(Utc::now)
            } else {
                Utc::now()
            }
        }
        _ => Utc::now(),
    };
    let rfc3339 = ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    event.insert("time", rfc3339)
}

/// POST a batch of events to the Honeycomb endpoint with auth header.
///
/// - 5xx or network error: retry up to `MAX_5XX_ATTEMPTS` total.
/// - 4xx (including 429): record failure, no retry.
/// - 2xx: success.
async fn post_batch(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    batch: Vec<SinkEvent>,
    shared: &Arc<Shared>,
) {
    // Enrich each event with the `time` RFC3339 field.
    let enriched: Vec<SinkEvent> = batch.into_iter().map(enrich_event).collect();

    let body = match serde_json::to_string(&enriched) {
        Ok(b) => b,
        Err(e) => {
            record_failure(shared, format!("serialization error: {e}"));
            return;
        }
    };

    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        let result = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Honeycomb-Team", api_key)
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
                        format!("HTTP {} after {attempts} attempts", status.as_u16()),
                    );
                    return;
                } else {
                    // 4xx (including 429) — record failure, no retry.
                    record_failure(
                        shared,
                        format!("HTTP {} (client error, no retry)", status.as_u16()),
                    );
                    return;
                }
            }
            Err(e) => {
                if attempts < MAX_5XX_ATTEMPTS {
                    continue;
                }
                record_failure(shared, format!("request error: {e}"));
                return;
            }
        }
    }
}

fn record_failure(shared: &Arc<Shared>, reason: String) {
    let mut guard = shared.failures.lock().unwrap_or_else(|p| p.into_inner());
    guard.push(WorkerFailure { reason });
}
