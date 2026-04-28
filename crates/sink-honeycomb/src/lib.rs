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

use serde::Deserialize;
use sink_core::{Sink, SinkConfigCommon, SinkEvent};

/// Honeycomb base URL (without dataset path component).
pub const HONEYCOMB_BASE_URL: &str = "https://api.honeycomb.io/1/events";

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
    pub fn from_toml(_toml_src: &str) -> anyhow::Result<Option<HoneycombSinkConfig>> {
        unimplemented!("HoneycombSinkConfig::from_toml not yet implemented (S-4.03)")
    }

    /// Build the full Honeycomb endpoint URL by appending the dataset to the
    /// base URL.
    ///
    /// e.g. `https://api.honeycomb.io/1/events/my-dataset`
    pub fn endpoint_url(&self) -> String {
        unimplemented!("HoneycombSinkConfig::endpoint_url not yet implemented (S-4.03)")
    }
}

/// Honeycomb Events API sink.
///
/// Wraps [`sink_http::HttpSink`] and injects:
/// - Correct endpoint URL (`/1/events/<dataset>`)
/// - `X-Honeycomb-Team: <api_key>` auth header
/// - `time` field in RFC3339 on each event (BC-3.NN.NNN-honeycomb-time-field-rfc3339)
pub struct HoneycombSink {
    name: String,
}

impl HoneycombSink {
    /// Construct a `HoneycombSink` from a validated config.
    pub fn new(_config: HoneycombSinkConfig) -> anyhow::Result<Self> {
        unimplemented!("HoneycombSink::new not yet implemented (S-4.03)")
    }
}

impl Sink for HoneycombSink {
    fn name(&self) -> &str {
        &self.name
    }

    fn accepts(&self, _event: &SinkEvent) -> bool {
        unimplemented!("HoneycombSink::accepts not yet implemented (S-4.03)")
    }

    fn submit(&self, _event: SinkEvent) {
        unimplemented!("HoneycombSink::submit not yet implemented (S-4.03)")
    }

    fn flush(&self) -> anyhow::Result<()> {
        unimplemented!("HoneycombSink::flush not yet implemented (S-4.03)")
    }

    fn shutdown(&self) {
        unimplemented!("HoneycombSink::shutdown not yet implemented (S-4.03)")
    }
}
