//! HTTP sink driver stub (S-4.01).
//!
//! This file is a minimal stub — every method panics with `unimplemented!()`.
//! The RED gate is enforced: all tests in `crates/sink-http/tests/` must fail
//! until the implementer fills these in.
//!
//! ## Contract
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
    pub fn from_toml(_toml_src: &str) -> anyhow::Result<Option<HttpSinkConfig>> {
        unimplemented!("HttpSinkConfig::from_toml — implementer fills this in (S-4.01)")
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

/// HTTP batch-POST sink.
///
/// Implements [`Sink`] from sink-core. Exposed `pub` so S-4.02 (Datadog) and
/// S-4.03 (Honeycomb) can embed or wrap it (AC-9 / BC-3.01.001 invariant 1).
pub struct HttpSink {
    _config: HttpSinkConfig,
}

impl HttpSink {
    /// Construct an `HttpSink` from a validated config.
    ///
    /// Starts the background worker thread that consumes the internal queue
    /// and POSTs batches to the configured URL.
    pub fn new(_config: HttpSinkConfig) -> anyhow::Result<Self> {
        unimplemented!("HttpSink::new — implementer fills this in (S-4.01)")
    }

    /// Drain recorded send failures accumulated since the last call.
    ///
    /// Used by tests and the dispatcher integration to inspect failure state
    /// without panicking.
    pub fn take_failures(&self) -> Vec<SinkFailure> {
        unimplemented!("HttpSink::take_failures — implementer fills this in (S-4.01)")
    }

    /// Number of events dropped due to a full internal queue (VP-011 overflow
    /// pathway). Non-blocking: callers read this to verify backpressure.
    pub fn queue_full_count(&self) -> u64 {
        unimplemented!("HttpSink::queue_full_count — implementer fills this in (S-4.01)")
    }
}

impl Sink for HttpSink {
    fn name(&self) -> &str {
        unimplemented!("HttpSink::name — implementer fills this in (S-4.01)")
    }

    fn accepts(&self, _event: &SinkEvent) -> bool {
        unimplemented!("HttpSink::accepts — implementer fills this in (S-4.01)")
    }

    fn submit(&self, _event: SinkEvent) {
        unimplemented!("HttpSink::submit — implementer fills this in (S-4.01)")
    }

    fn flush(&self) -> anyhow::Result<()> {
        unimplemented!("HttpSink::flush — implementer fills this in (S-4.01)")
    }

    fn shutdown(&self) {
        unimplemented!("HttpSink::shutdown — implementer fills this in (S-4.01)")
    }
}
