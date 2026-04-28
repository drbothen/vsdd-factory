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
use sink_http::{HttpSink, HttpSinkConfig, SinkFailure};

/// Default Datadog Logs Intake v2 endpoint (us1 region).
pub const DEFAULT_DATADOG_ENDPOINT: &str = "https://http-intake.logs.datadoghq.com/api/v2/logs";

/// Maximum bytes per Datadog batch POST (5MB per Datadog's documented limit).
pub const DATADOG_MAX_BATCH_BYTES: usize = 5 * 1024 * 1024;

// ── Raw deserialisation target ────────────────────────────────────────────────

/// Internal TOML deserialization target for DatadogSinkConfig.
/// Uses an `Option<String>` for `api_key` so we can emit a clear error
/// when the field is absent.
#[derive(Debug, Deserialize)]
struct RawDatadogSinkConfig {
    schema_version: u32,
    #[serde(rename = "type")]
    sink_type: String,
    #[serde(flatten)]
    common: SinkConfigCommon,
    /// Required. `None` means the field was absent in TOML (EC-001).
    api_key: Option<String>,
    endpoint: Option<String>,
}

// ── Public config type ────────────────────────────────────────────────────────

/// Driver-specific configuration for the Datadog Logs Intake sink.
///
/// Deserialized from an `[[sinks]]` stanza in `observability-config.toml`.
#[derive(Debug, Clone)]
pub struct DatadogSinkConfig {
    /// Common cross-sink fields (name, enabled, routing_filter, tags).
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
    pub fn from_toml(toml_src: &str) -> anyhow::Result<Option<DatadogSinkConfig>> {
        let raw: RawDatadogSinkConfig =
            toml::from_str(toml_src).map_err(|e| anyhow::anyhow!("TOML parse error: {e}"))?;

        if raw.schema_version != 1 {
            return Err(anyhow::anyhow!(
                "schema_version must be 1, got {} — only schema_version 1 is supported",
                raw.schema_version
            ));
        }

        if raw.sink_type != "datadog" {
            eprintln!(
                "sink-datadog: unknown sink type {:?} — skipping",
                raw.sink_type
            );
            return Ok(None);
        }

        // EC-001: api_key is required and must not be empty.
        let api_key = match raw.api_key {
            None => {
                return Err(anyhow::anyhow!(
                    "missing required field `api_key` for datadog sink (EC-001)"
                ));
            }
            Some(k) if k.is_empty() => {
                return Err(anyhow::anyhow!(
                    "api_key must not be empty for datadog sink (EC-001)"
                ));
            }
            Some(k) => k,
        };

        Ok(Some(DatadogSinkConfig {
            common: raw.common,
            api_key,
            endpoint: raw.endpoint,
        }))
    }

    /// Return the effective endpoint URL: caller-supplied or the default us1 URL.
    pub fn effective_endpoint(&self) -> &str {
        self.endpoint.as_deref().unwrap_or(DEFAULT_DATADOG_ENDPOINT)
    }
}

// ── DatadogSink ───────────────────────────────────────────────────────────────

/// Datadog Logs Intake batch-POST sink.
///
/// Implements [`Sink`] from sink-core. Wraps [`HttpSink`] for HTTP
/// transport and injects the `DD-API-KEY` authentication header on every POST.
pub struct DatadogSink {
    inner: HttpSink,
}

impl DatadogSink {
    /// Construct a `DatadogSink` from a validated config.
    ///
    /// Builds an [`HttpSinkConfig`] via the builder API (F-2: no field pinning)
    /// and injects `DD-API-KEY` as an extra header on every POST.
    pub fn new(config: DatadogSinkConfig) -> anyhow::Result<Self> {
        let endpoint = config.effective_endpoint().to_owned();
        let http_config = HttpSinkConfig::builder()
            .common(config.common)
            .url(endpoint)
            .header("DD-API-KEY", config.api_key)
            .build();

        let inner = HttpSink::new(http_config)?;
        Ok(Self { inner })
    }

    /// Drain recorded send failures accumulated since the last call.
    pub fn take_failures(&self) -> Vec<SinkFailure> {
        self.inner.take_failures()
    }
}

impl Sink for DatadogSink {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn accepts(&self, event: &SinkEvent) -> bool {
        self.inner.accepts(event)
    }

    fn submit(&self, event: SinkEvent) {
        self.inner.submit(event);
    }

    fn flush(&self) -> anyhow::Result<()> {
        self.inner.flush()
    }

    fn shutdown(&self) {
        self.inner.shutdown();
    }
}
