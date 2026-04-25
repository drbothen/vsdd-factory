//! Sink registry + loader (S-1.8).
//!
//! Parses `observability-config.toml` into a fleet of [`Sink`]
//! implementations and fans submitted events out to every enabled sink
//! whose [`Sink::accepts`] returns true. Driver types that are not yet
//! implemented (HTTP / OTel / Datadog / Honeycomb — S-1.9 and S-4.x)
//! are warned and skipped so config can forward-declare them without
//! failing the dispatcher.
//!
//! TODO(integration): wire SinkRegistry into main.rs startup — this
//! story deliberately keeps the dispatcher's synchronous main
//! untouched. The follow-up integration commit will:
//! 1. Construct the registry in `main()` once per dispatcher process.
//! 2. Drain `HostContext.events` after each plugin call and
//!    `submit_all` the result.
//! 3. `flush_all` at tier boundaries and a final `flush_all` +
//!    shutdown before `main` returns.
//!
//! The present story exercises this surface only through
//! `tests/sinks_file_integration.rs` so the contract is pinned before
//! the wiring lands.

use std::path::Path;

use serde::Deserialize;
use sink_core::{Sink, SinkEvent};
use sink_file::{FileSink, FileSinkConfig};
use sink_otel_grpc::{OtelGrpcConfig, OtelGrpcSink};

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

    /// Catch-all for driver-specific fields. Re-serialized to a TOML
    /// value and then deserialized into the target type, so per-driver
    /// configs keep their strict typing.
    #[serde(flatten)]
    pub extra: toml::value::Table,
}

/// Fleet of constructed sinks. `load` parses config; `submit_all` /
/// `flush_all` fan out; `empty` is the test-only constructor.
pub struct SinkRegistry {
    sinks: Vec<Box<dyn Sink>>,
}

impl SinkRegistry {
    /// Empty registry — integration tests and a few callers that want
    /// to add sinks programmatically use this.
    pub fn empty() -> Self {
        Self { sinks: Vec::new() }
    }

    /// Programmatic constructor for tests and for the eventual
    /// integration-wiring story. Accepts any pre-built
    /// `Box<dyn Sink>` (e.g. a `FileSink` wrapped in a box).
    pub fn with_sinks(sinks: Vec<Box<dyn Sink>>) -> Self {
        Self { sinks }
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
                other => {
                    eprintln!(
                        "factory-dispatcher: unknown sink type '{}' (stanza '{}'); skipping. Supported in v1.0-beta.1: file, otel-grpc",
                        other, stanza.name
                    );
                }
            }
        }

        Ok(Self { sinks })
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
        let cfg = ObservabilityConfig {
            schema_version: 1,
            sinks: vec![SinkStanza {
                type_: "datadog".into(),
                name: "not-yet-implemented".into(),
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
                extra,
            }],
        };
        let reg = SinkRegistry::from_config(cfg).unwrap();
        assert_eq!(reg.sinks().len(), 1);
        assert_eq!(reg.sinks()[0].name(), "local-grafana");
        // Cleanly drain the worker before the registry drops.
        reg.shutdown_all();
    }
}
