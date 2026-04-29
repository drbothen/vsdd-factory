//! # Sinks
//!
//! Sink registry + loader for Router (S-4.06).
//!
//! The Router (`crates/factory-dispatcher/src/sinks/router.rs`) is the dispatch entry point;
//! it applies RoutingFilter and tag enrichment before delegating to enabled sinks via
//! SinkRegistry. Sink::accepts is now reduced to enabled-flag + shutdown-state checks
//! (per BC-3.04.004 invariant 1; Router is the single dispatch gate).
//!
//! ## Integration Status
//!
//! Router is the dispatch entry point at the API layer (Router::submit).
//! Integration with `main.rs` (constructing the registry, draining HostContext.events,
//! flushing at tier boundaries) remains a follow-up. See S-4.07 for E2E integration tests
//! exercising Router::submit directly; main.rs wiring is tracked separately.

use std::path::Path;
use std::sync::Arc;

use serde::Deserialize;
use sink_core::{DlqWriter, DlqWriterConfig, Sink, SinkDlqEvent, SinkEvent};
use sink_datadog::{DatadogSink, DatadogSinkConfig};
use sink_file::{FileSink, FileSinkConfig};
use sink_honeycomb::{HoneycombSink, HoneycombSinkConfig};
use sink_http::{HttpSink, HttpSinkConfig};
use sink_otel_grpc::{OtelGrpcConfig, OtelGrpcSink};
use thiserror::Error;

/// Errors raised during sink configuration validation (AC-011 / EC-003).
///
/// Stub — variants declared; validation logic not yet implemented.
#[derive(Debug, Error)]
pub enum SinkConfigError {
    /// `sink_name` contains `/`, `\`, or `..` path tokens (AC-011 / EC-003).
    #[error("invalid sink name '{name}': must not contain path separators or '..' tokens")]
    InvalidSinkName {
        /// The rejected sink name.
        name: String,
    },
    /// Two stanzas share the same `sink_name` (EC-004).
    #[error("sink name collision: '{name}' appears more than once")]
    NameCollision {
        /// The duplicate name.
        name: String,
    },
    /// Invalid prune configuration (e.g., `dlq_retention_days = 0`).
    #[error("invalid prune config: {reason}")]
    InvalidPruneConfig {
        /// Human-readable reason.
        reason: String,
    },
}

pub mod router;

pub use router::Router;

/// The top-level `observability-config.toml` shape.
///
/// `schema_version` is accepted and ignored for now — we're at version
/// 1 and there is nothing to migrate. The router has space to bump it
/// when a future change makes migration necessary.
#[derive(Debug, Deserialize)]
pub struct ObservabilityConfig {
    /// Schema-version gate. Reserved; currently accepts 1 only.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,

    /// Array-of-tables: each `[[sinks]]` stanza is one sink instance.
    /// The `type` discriminator chooses the driver.
    #[serde(default)]
    pub sinks: Vec<SinkStanza>,
}

fn default_schema_version() -> u32 {
    1
}

/// A single `[[sinks]]` stanza, pre-dispatch on the `type` field.
///
/// Driver-specific fields are captured in `extra` so we can re-
/// deserialize against the driver's typed config (e.g.
/// [`FileSinkConfig`]) without a second parse pass over the whole
/// file. This also keeps unknown-driver entries parseable for the
/// "warn and skip" branch — see [`SinkRegistry::load`].
#[derive(Debug, Deserialize)]
pub struct SinkStanza {
    /// Driver discriminator. `"file"` is the only type implemented in
    /// S-1.8; `"http"`, `"otel-grpc"`, `"datadog"`, `"honeycomb"` will
    /// be added by S-1.9 and S-4.x.
    #[serde(rename = "type")]
    pub type_: String,

    /// Operator-assigned sink name; bubbles up from the driver's
    /// typed config.
    pub name: String,

    /// Whether the dead-letter queue is enabled for this sink (AC-008).
    ///
    /// Defaults to `true` (DLQ on-by-default). Set to `false` to opt out.
    /// When absent from TOML the deserializer uses the `default_true` fn.
    #[serde(default = "default_dlq_enabled")]
    pub dlq_enabled: bool,

    /// Catch-all for driver-specific fields. Re-serialized to a TOML
    /// value and then deserialized into the target type, so per-driver
    /// configs keep their strict typing.
    #[serde(flatten)]
    pub extra: toml::value::Table,
}

fn default_dlq_enabled() -> bool {
    true
}

impl SinkStanza {
    /// Validate the stanza's `name` field for path-injection safety (AC-011 / EC-003).
    ///
    /// Rejects names containing `/` or `\` (path separators that could allow
    /// directory traversal when the name is used as a DLQ filename component).
    pub fn validate(&self) -> Result<(), SinkConfigError> {
        if self.name.contains('/') || self.name.contains('\\') {
            return Err(SinkConfigError::InvalidSinkName {
                name: self.name.clone(),
            });
        }
        Ok(())
    }
}

/// Fleet of constructed sinks. `load` parses config; `submit_all` /
/// `flush_all` fan out; `empty` is the test-only constructor.
pub struct SinkRegistry {
    sinks: Vec<Box<dyn Sink>>,
    /// Parallel boolean sidecar tracking DLQ wire-up per sink, indexed
    /// alongside `sinks`. `true` means the sink was constructed with
    /// `Some(Arc<DlqWriter>)`; `false` means `None`.
    ///
    /// Populated by `from_config_with_dlq` at construction time (AC-012,
    /// F-4302 resolution). Stub — field declared; population logic not yet
    /// implemented.
    dlq_present_per_sink: Vec<bool>,
}

impl std::fmt::Debug for SinkRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SinkRegistry")
            .field("sinks_count", &self.sinks.len())
            .field("dlq_present_per_sink", &self.dlq_present_per_sink)
            .finish()
    }
}

impl SinkRegistry {
    /// Empty registry — integration tests and a few callers that want
    /// to add sinks programmatically use this.
    pub fn empty() -> Self {
        Self {
            sinks: Vec::new(),
            dlq_present_per_sink: Vec::new(),
        }
    }

    /// Programmatic constructor for tests and for the eventual
    /// integration-wiring story. Accepts any pre-built
    /// `Box<dyn Sink>` (e.g. a `FileSink` wrapped in a box).
    pub fn with_sinks(sinks: Vec<Box<dyn Sink>>) -> Self {
        let len = sinks.len();
        Self {
            sinks,
            dlq_present_per_sink: vec![false; len],
        }
    }

    /// Construct a registry from an already-parsed config, wiring DLQ writers
    /// for stanzas that have `dlq_enabled = true` (or absent — default-on).
    ///
    /// Validates all sink names for path-injection safety (AC-011 / EC-003)
    /// before constructing any sinks. Returns `Err` on the first invalid name.
    ///
    /// Populates `dlq_present_per_sink` so `has_dlq_for_test` can be used in
    /// tests to verify wire-up without adding a public accessor to the `Sink`
    /// trait (AC-012, F-4302 resolution).
    pub fn from_config_with_dlq(cfg: ObservabilityConfig) -> anyhow::Result<Self> {
        if cfg.schema_version != 1 {
            return Err(anyhow::anyhow!(
                "unsupported observability-config schema_version {}; expected 1",
                cfg.schema_version
            ));
        }

        // AC-011: validate all sink names before constructing any sinks.
        for stanza in &cfg.sinks {
            stanza.validate().map_err(|e| anyhow::anyhow!("{}", e))?;
        }

        let project_dir = std::env::var("CLAUDE_PROJECT_DIR").ok();

        // Use a system-temp-based DLQ root for now (the full dispatcher
        // integration story will pass the real log_dir; here we create
        // a throwaway per-instance channel for the DLQ event bus).
        let dlq_root = std::env::temp_dir().join("vsdd-factory-dlq");

        // Throwaway DLQ event channel — events are dropped if no consumer
        // is attached (acceptable for this registry-construction path;
        // the full integration wires a real consumer in factory-dispatcher lib.rs).
        let (dlq_event_tx, _dlq_event_rx) = tokio::sync::mpsc::channel::<SinkDlqEvent>(256);

        let mut sinks: Vec<Box<dyn Sink>> = Vec::with_capacity(cfg.sinks.len());
        let mut dlq_present: Vec<bool> = Vec::with_capacity(cfg.sinks.len());

        for stanza in cfg.sinks {
            // Resolve DLQ writer for this stanza if dlq_enabled.
            let dlq_writer: Option<Arc<DlqWriter>> = if stanza.dlq_enabled {
                let dlq_cfg = DlqWriterConfig {
                    template: "dead-letter-{name}-{date}.jsonl".to_owned(),
                    size_cap_bytes: 100 * 1024 * 1024,
                    project: project_dir.clone(),
                    dlq_root: dlq_root.clone(),
                };
                Some(Arc::new(DlqWriter::new(dlq_cfg, dlq_event_tx.clone())))
            } else {
                None
            };
            let has_dlq = dlq_writer.is_some();

            match stanza.type_.as_str() {
                "file" => {
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    let file_cfg: FileSinkConfig = merged.try_into().map_err(|e| {
                        anyhow::anyhow!("file sink '{}' config invalid: {}", stanza.name, e)
                    })?;
                    let sink = FileSink::new(file_cfg, project_dir.clone())?;
                    sinks.push(Box::new(sink));
                }
                "http" => {
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    // Reconstruct minimal TOML so HttpSinkConfig::from_toml can parse it.
                    // We need schema_version and type fields for the driver's validator.
                    merged.insert("schema_version".into(), toml::Value::Integer(1));
                    merged.insert("type".into(), toml::Value::String("http".into()));
                    let toml_str = toml::to_string(&merged).map_err(|e| {
                        anyhow::anyhow!("http sink '{}' config serialization: {}", stanza.name, e)
                    })?;
                    let http_cfg = HttpSinkConfig::from_toml(&toml_str)
                        .map_err(|e| {
                            anyhow::anyhow!("http sink '{}' config invalid: {}", stanza.name, e)
                        })?
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "http sink '{}' config returned None (unexpected type)",
                                stanza.name
                            )
                        })?;
                    let sink =
                        HttpSink::new_with_observability(http_cfg, None, dlq_writer.clone())?;
                    sinks.push(Box::new(sink));
                }
                "otel-grpc" => {
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    let otel_cfg: OtelGrpcConfig = merged.try_into().map_err(|e| {
                        anyhow::anyhow!("otel-grpc sink '{}' config invalid: {}", stanza.name, e)
                    })?;
                    let sink = OtelGrpcSink::new(otel_cfg)?;
                    sinks.push(Box::new(sink));
                }
                other => {
                    eprintln!(
                        "factory-dispatcher: unknown sink type '{}' (stanza '{}'); skipping. Supported in v1.0-beta.1: file, otel-grpc",
                        other, stanza.name
                    );
                    // Unknown sink: don't add to sinks; skip dlq tracking for it.
                    continue;
                }
            }

            dlq_present.push(has_dlq);
        }

        Ok(Self {
            sinks,
            dlq_present_per_sink: dlq_present,
        })
    }

    /// Test helper: returns `true` if the sink at `idx` was wired with a DLQ
    /// writer (`Some(Arc<DlqWriter>)`).
    ///
    /// `pub(crate)` — not part of the public API; used only from `#[cfg(test)]`
    /// blocks within this crate (AC-012, F-4302 resolution).
    #[allow(dead_code)]
    pub(crate) fn has_dlq_for_test(&self, idx: usize) -> bool {
        self.dlq_present_per_sink.get(idx).copied().unwrap_or(false)
    }

    /// Parse `observability-config.toml` from disk and construct every
    /// supported-driver sink. Unknown driver types are warned to
    /// stderr and skipped so a config that forward-declares a
    /// not-yet-implemented sink type still loads; the pending
    /// integration story may route these through
    /// `tracing::warn!` once the dispatcher owns a subscriber on the
    /// sink-construction path.
    pub fn load(config_path: &Path) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(config_path).map_err(|e| {
            anyhow::anyhow!(
                "failed to read observability config at {}: {}",
                config_path.display(),
                e
            )
        })?;
        let cfg: ObservabilityConfig = toml::from_str(&raw).map_err(|e| {
            anyhow::anyhow!(
                "failed to parse observability config at {}: {}",
                config_path.display(),
                e
            )
        })?;
        Self::from_config(cfg)
    }

    /// Construct a registry from an already-parsed config. Split out
    /// from [`Self::load`] for test ergonomics.
    pub fn from_config(cfg: ObservabilityConfig) -> anyhow::Result<Self> {
        if cfg.schema_version != 1 {
            return Err(anyhow::anyhow!(
                "unsupported observability-config schema_version {}; expected 1",
                cfg.schema_version
            ));
        }

        // Read the project dir basename once — file sinks use it for
        // `{project}` substitution.
        let project_dir = std::env::var("CLAUDE_PROJECT_DIR").ok();

        let mut sinks: Vec<Box<dyn Sink>> = Vec::with_capacity(cfg.sinks.len());
        for stanza in cfg.sinks {
            match stanza.type_.as_str() {
                "file" => {
                    let mut merged = stanza.extra.clone();
                    // The `name` field lives at the stanza level but
                    // the driver expects it inside its own config
                    // block.
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    let cfg: FileSinkConfig = merged.try_into().map_err(|e| {
                        anyhow::anyhow!("file sink '{}' config invalid: {}", stanza.name, e)
                    })?;
                    let sink = FileSink::new(cfg, project_dir.clone())?;
                    sinks.push(Box::new(sink));
                }
                "otel-grpc" => {
                    // S-1.9: OTLP/gRPC log forwarder. Same name-merge
                    // dance as file: the `name` lives at the stanza
                    // level, and the driver expects it inside its
                    // typed config.
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    let cfg: OtelGrpcConfig = merged.try_into().map_err(|e| {
                        anyhow::anyhow!("otel-grpc sink '{}' config invalid: {}", stanza.name, e)
                    })?;
                    let sink = OtelGrpcSink::new(cfg)?;
                    sinks.push(Box::new(sink));
                }
                "datadog" => {
                    // S-4.02: Datadog Logs Intake sink. Build a DatadogSinkConfig
                    // from the stanza's extra fields and common name.
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    merged.insert("schema_version".into(), toml::Value::Integer(1));
                    merged.insert("type".into(), toml::Value::String("datadog".into()));
                    let toml_str = toml::to_string(&merged).map_err(|e| {
                        anyhow::anyhow!(
                            "datadog sink '{}' config serialization: {}",
                            stanza.name,
                            e
                        )
                    })?;
                    let dd_cfg = DatadogSinkConfig::from_toml(&toml_str)
                        .map_err(|e| {
                            anyhow::anyhow!("datadog sink '{}' config invalid: {}", stanza.name, e)
                        })?
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "datadog sink '{}' config returned None (unexpected type)",
                                stanza.name
                            )
                        })?;
                    let sink = DatadogSink::new(dd_cfg)?;
                    sinks.push(Box::new(sink));
                }
                "honeycomb" => {
                    // S-4.03: Honeycomb Events API sink.
                    let mut merged = stanza.extra.clone();
                    merged.insert("name".into(), toml::Value::String(stanza.name.clone()));
                    merged.insert("type".into(), toml::Value::String("honeycomb".into()));
                    let toml_str = toml::to_string(&merged).map_err(|e| {
                        anyhow::anyhow!(
                            "honeycomb sink '{}' config serialization: {}",
                            stanza.name,
                            e
                        )
                    })?;
                    let hc_cfg = HoneycombSinkConfig::from_toml(&toml_str)
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "honeycomb sink '{}' config invalid: {}",
                                stanza.name,
                                e
                            )
                        })?
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "honeycomb sink '{}' config returned None (unexpected type)",
                                stanza.name
                            )
                        })?;
                    let sink = HoneycombSink::new(hc_cfg)?;
                    sinks.push(Box::new(sink));
                }
                other => {
                    eprintln!(
                        "factory-dispatcher: unknown sink type '{}' (stanza '{}'); skipping. Supported in v1.0-beta.1: file, otel-grpc, datadog, honeycomb",
                        other, stanza.name
                    );
                }
            }
        }

        let n = sinks.len();
        Ok(Self {
            sinks,
            dlq_present_per_sink: vec![false; n],
        })
    }

    /// Fan an event out to every sink that accepts it. Non-blocking:
    /// individual sinks `try_send` onto their bounded queues and drop
    /// on overflow.
    pub fn submit_all(&self, event: SinkEvent) {
        for sink in &self.sinks {
            if sink.accepts(&event) {
                sink.submit(event.clone());
            }
        }
    }

    /// Flush every sink. Errors are collected and the first is
    /// returned; callers may instead iterate via [`Self::sinks`] if
    /// they need per-sink error isolation.
    pub fn flush_all(&self) -> anyhow::Result<()> {
        let mut first_err: Option<anyhow::Error> = None;
        for sink in &self.sinks {
            if let Err(e) = sink.flush()
                && first_err.is_none()
            {
                first_err = Some(e);
            }
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Shut down every sink (drains queues, joins workers).
    pub fn shutdown_all(&self) {
        for sink in &self.sinks {
            sink.shutdown();
        }
    }

    /// Borrow the underlying sink slice — for per-sink inspection in
    /// tests and the eventual integration wiring.
    pub fn sinks(&self) -> &[Box<dyn Sink>] {
        &self.sinks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_submit_is_a_noop() {
        let reg = SinkRegistry::empty();
        reg.submit_all(SinkEvent::new().insert("type", "commit.made"));
        reg.flush_all().unwrap();
        reg.shutdown_all();
    }

    #[test]
    fn load_warns_on_unknown_sink_type_but_still_succeeds() {
        // Use 'splunk' as the unknown-type example per BC-3.01.002 BCs to Update
        // (datadog is now a real driver in v1.0-beta.1 per S-4.02; using it here
        // would create a false negative — it would be recognized, not warned about).
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "splunk".into(),
                name: "not-yet-implemented".into(),
                dlq_enabled: true,
                extra: toml::value::Table::new(),
            }],
        };
        let reg = SinkRegistry::from_config(cfg).unwrap();
        assert!(reg.sinks().is_empty(), "unknown type is skipped");
    }

    #[test]
    fn load_rejects_unsupported_schema_version() {
        let cfg = ObservabilityConfig {
            schema_version: 99,
            sinks: vec![],
        };
        let err = match SinkRegistry::from_config(cfg) {
            Ok(_) => panic!("expected schema_version rejection"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("schema_version"));
    }

    #[test]
    fn load_builds_file_sink_from_parsed_config() {
        let tmp = tempfile::tempdir().unwrap();
        let mut extra = toml::value::Table::new();
        extra.insert("enabled".into(), toml::Value::Boolean(true));
        extra.insert(
            "path_template".into(),
            toml::Value::String(format!("{}/{{name}}-{{date}}.jsonl", tmp.path().display())),
        );
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "file".into(),
                name: "local-events".into(),
                dlq_enabled: true,
                extra,
            }],
        };
        let reg = SinkRegistry::from_config(cfg).unwrap();
        assert_eq!(reg.sinks().len(), 1);
        assert_eq!(reg.sinks()[0].name(), "local-events");
    }

    #[test]
    fn load_builds_otel_grpc_sink_from_parsed_config() {
        // S-1.9: a `type = "otel-grpc"` stanza with an unreachable
        // endpoint still constructs (worker connects lazily) — the
        // registry path is what we're exercising here, not the gRPC
        // round-trip. Endpoint-unreachable behavior is covered in the
        // integration test in `tests/sinks_otel_grpc.rs`.
        let mut extra = toml::value::Table::new();
        extra.insert(
            "endpoint".into(),
            toml::Value::String("http://127.0.0.1:1".into()),
        );
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "otel-grpc".into(),
                name: "local-grafana".into(),
                dlq_enabled: true,
                extra,
            }],
        };
        let reg = SinkRegistry::from_config(cfg).unwrap();
        assert_eq!(reg.sinks().len(), 1);
        assert_eq!(reg.sinks()[0].name(), "local-grafana");
        // Cleanly drain the worker before the registry drops.
        reg.shutdown_all();
    }

    // ── AC-008: DLQ default-on per sink stanza ────────────────────────────────

    /// AC-008 — `dlq_enabled` defaults to `true` when absent from TOML.
    ///
    /// Parses a TOML stanza WITHOUT the `dlq_enabled` key and verifies the
    /// parsed `SinkStanza.dlq_enabled` is `true`.
    ///
    /// This test exercises the `default_dlq_enabled` serde default.
    /// It does NOT call any `unimplemented!()` stub, so it should PASS
    /// if serde is correctly wired — which means it will PASS in RED state.
    /// Kept as a guard to prevent regressions on the default.
    #[test]
    fn test_BC_3_07_003_dlq_enabled_defaults_to_true_when_absent() {
        let toml_src = r#"
            schema_version = 1

            [[sinks]]
            type = "file"
            name = "my-file-sink"
            path_template = "/tmp/events-{date}.jsonl"
            # dlq_enabled is intentionally absent — must default to true
        "#;
        let cfg: ObservabilityConfig = toml::from_str(toml_src).unwrap();
        assert_eq!(cfg.sinks.len(), 1);
        assert!(
            cfg.sinks[0].dlq_enabled,
            "AC-008: dlq_enabled must default to true when absent from TOML stanza"
        );
    }

    /// AC-008 — `dlq_enabled = false` is honoured.
    ///
    /// Parses a TOML stanza with `dlq_enabled = false` and verifies the
    /// parsed `SinkStanza.dlq_enabled` is `false`.
    #[test]
    fn test_BC_3_07_003_dlq_enabled_false_is_honoured_by_parser() {
        let toml_src = r#"
            schema_version = 1

            [[sinks]]
            type = "file"
            name = "no-dlq-sink"
            path_template = "/tmp/events-{date}.jsonl"
            dlq_enabled = false
        "#;
        let cfg: ObservabilityConfig = toml::from_str(toml_src).unwrap();
        assert_eq!(cfg.sinks.len(), 1);
        assert!(
            !cfg.sinks[0].dlq_enabled,
            "AC-008: dlq_enabled = false must be honoured"
        );
    }

    // ── AC-011: SinkStanza dlq_enabled validation (invalid sink_name) ─────────

    /// AC-011 — Traces to: v1.1 BC candidate `BC-3.NN.NNN-sink-name-sanitization`.
    ///
    /// `from_config_with_dlq` must reject a `sink_name` containing `/`.
    ///
    /// RED gate: `from_config_with_dlq` is `unimplemented!()`.
    #[test]
    fn test_BC_3_07_003_rejects_sink_name_with_path_separator_slash() {
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "file".into(),
                name: "../../etc/passwd".into(), // path traversal
                dlq_enabled: true,
                extra: {
                    let mut t = toml::value::Table::new();
                    t.insert(
                        "path_template".into(),
                        toml::Value::String("/tmp/x-{date}.jsonl".into()),
                    );
                    t
                },
            }],
        };

        // from_config_with_dlq must reject the invalid name.
        let result = SinkRegistry::from_config_with_dlq(cfg);
        assert!(
            result.is_err(),
            "AC-011: sink_name with path traversal must be rejected"
        );
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("invalid") || err_str.contains("InvalidSinkName"),
            "AC-011: error must mention the invalid name; got: {err_str}"
        );
    }

    /// AC-011 — `sink_name` containing backslash must be rejected.
    ///
    /// RED gate: `from_config_with_dlq` is `unimplemented!()`.
    #[test]
    fn test_BC_3_07_003_rejects_sink_name_with_backslash() {
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "file".into(),
                name: r"a\b".into(),
                dlq_enabled: true,
                extra: {
                    let mut t = toml::value::Table::new();
                    t.insert(
                        "path_template".into(),
                        toml::Value::String("/tmp/x-{date}.jsonl".into()),
                    );
                    t
                },
            }],
        };
        let result = SinkRegistry::from_config_with_dlq(cfg);
        assert!(
            result.is_err(),
            "AC-011: sink_name with backslash must be rejected"
        );
    }

    // ── AC-012: SinkRegistry loader wires DlqWriter when dlq_enabled ─────────

    /// AC-012 — Traces to: v1.1 BC candidate `BC-3.NN.NNN-dlq-config-toggle-and-size-cap`.
    ///
    /// `from_config_with_dlq` on a 2-stanza config (one with `dlq_enabled = false`,
    /// one with default/absent) must produce a `SinkRegistry` where:
    ///   - `has_dlq_for_test(0)` is `true` (default-on stanza)
    ///   - `has_dlq_for_test(1)` is `false` (explicitly disabled stanza)
    ///
    /// RED gate: `from_config_with_dlq` is `unimplemented!()`.
    #[test]
    fn test_BC_3_07_003_sink_registry_wires_dlq_when_dlq_enabled() {
        let tmp = tempfile::tempdir().unwrap();

        // Stanza 0: dlq_enabled absent (defaults to true).
        let mut extra0 = toml::value::Table::new();
        extra0.insert(
            "path_template".into(),
            toml::Value::String(format!("{}/s0-{{date}}.jsonl", tmp.path().display())),
        );

        // Stanza 1: dlq_enabled = false.
        let mut extra1 = toml::value::Table::new();
        extra1.insert(
            "path_template".into(),
            toml::Value::String(format!("{}/s1-{{date}}.jsonl", tmp.path().display())),
        );

        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![
                SinkStanza {
                    type_: "file".into(),
                    name: "dlq-on-sink".into(),
                    dlq_enabled: true, // explicit true
                    extra: extra0,
                },
                SinkStanza {
                    type_: "file".into(),
                    name: "dlq-off-sink".into(),
                    dlq_enabled: false,
                    extra: extra1,
                },
            ],
        };

        let reg = SinkRegistry::from_config_with_dlq(cfg)
            .expect("from_config_with_dlq must succeed with valid stanzas");

        assert_eq!(
            reg.sinks().len(),
            2,
            "AC-012: registry must contain 2 sinks"
        );

        assert!(
            reg.has_dlq_for_test(0),
            "AC-012: sink[0] (dlq_enabled=true) must have DLQ wired"
        );
        assert!(
            !reg.has_dlq_for_test(1),
            "AC-012: sink[1] (dlq_enabled=false) must NOT have DLQ wired"
        );

        reg.shutdown_all();
    }
}
