//! Datadog Logs Intake sink driver (S-4.02).
//!
//! [`DatadogSink`] implements [`Sink`] (from sink-core) by wrapping
//! [`HttpSink`] for transport and adding Datadog-specific configuration:
//! the DD-API-KEY auth header, the regional endpoint URL, Datadog schema
//! field mapping, and the 5MB per-batch payload limit.
//!
//! ## Config load API
//!
//! [`DatadogSinkConfig::from_toml`] validates `schema_version` (must equal 1)
//! and `type` (must equal `"datadog"`). Missing `api_key` is a hard error
//! (EC-001). Unknown `type` values return `Ok(None)`.

#![deny(missing_docs)]

use serde::Deserialize;
use sink_core::{Sink, SinkConfigCommon, SinkEvent};

/// Default Datadog Logs Intake v2 endpoint (us1 region).
pub const DEFAULT_DATADOG_ENDPOINT: &str =
    "https://http-intake.logs.datadoghq.com/api/v2/logs";

/// Maximum bytes per Datadog batch POST (5MB per Datadog's documented limit).
pub const DATADOG_MAX_BATCH_BYTES: usize = 5 * 1024 * 1024;

/// Driver-specific configuration for the Datadog Logs Intake sink.
///
/// Deserialized from an `[[sinks]]` stanza in `observability-config.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct DatadogSinkConfig {
    /// Must equal `1`. Any other value is a hard error at load time.
    pub schema_version: u32,

    /// Must equal `"datadog"`. Unknown values warn and return `Ok(None)`.
    #[serde(rename = "type")]
    pub sink_type: String,

    /// Common cross-sink fields (name, enabled, routing_filter, tags).
    #[serde(flatten)]
    pub common: SinkConfigCommon,

    /// Datadog API key. Required; missing key is a hard error (EC-001).
    pub api_key: String,

    /// Optional regional endpoint URL. Defaults to [`DEFAULT_DATADOG_ENDPOINT`]
    /// (us1) when absent.
    pub endpoint: Option<String>,
}

impl DatadogSinkConfig {
    /// Parse and validate a `DatadogSinkConfig` from a raw TOML string.
    ///
    /// Returns:
    /// - `Err(_)` when `schema_version != 1`.
    /// - `Err(_)` when `api_key` is absent or empty (EC-001).
    /// - `Ok(None)` when `type` is not `"datadog"` — warning emitted to stderr.
    /// - `Ok(Some(config))` on a valid stanza.
    pub fn from_toml(_toml_src: &str) -> anyhow::Result<Option<DatadogSinkConfig>> {
        unimplemented!("DatadogSinkConfig::from_toml — RED gate stub")
    }

    /// Return the effective endpoint URL: caller-supplied or the default us1 URL.
    pub fn effective_endpoint(&self) -> &str {
        unimplemented!("DatadogSinkConfig::effective_endpoint — RED gate stub")
    }
}

/// Datadog Logs Intake batch-POST sink.
///
/// Implements [`Sink`] from sink-core. Wraps [`sink_http::HttpSink`] for HTTP
/// transport and injects Datadog-specific headers and schema mapping.
pub struct DatadogSink {
    // Implementation fields — populated by the implementer in the GREEN phase.
    _config: DatadogSinkConfig,
}

impl DatadogSink {
    /// Construct a `DatadogSink` from a validated config.
    pub fn new(_config: DatadogSinkConfig) -> anyhow::Result<Self> {
        unimplemented!("DatadogSink::new — RED gate stub")
    }

    /// Drain recorded send failures accumulated since the last call.
    pub fn take_failures(&self) -> Vec<sink_http::SinkFailure> {
        unimplemented!("DatadogSink::take_failures — RED gate stub")
    }
}

impl Sink for DatadogSink {
    fn name(&self) -> &str {
        unimplemented!("DatadogSink::name — RED gate stub")
    }

    fn accepts(&self, _event: &SinkEvent) -> bool {
        unimplemented!("DatadogSink::accepts — RED gate stub")
    }

    fn submit(&self, _event: SinkEvent) {
        unimplemented!("DatadogSink::submit — RED gate stub")
    }

    fn flush(&self) -> anyhow::Result<()> {
        unimplemented!("DatadogSink::flush — RED gate stub")
    }

    fn shutdown(&self) {
        unimplemented!("DatadogSink::shutdown — RED gate stub")
    }
}
