//! AC-10, AC-15: Config load — unknown sink type, file sink config load.
//!
//! Traces to:
//! - BC-3.01.002 PC1: unknown sink type warns to stderr but does not fail (AC-10)
//! - BC-3.05.001 PC1: ObservabilityConfig with one type=file stanza loads into
//!   a single-sink registry (AC-15)
//!
//! SUT entry point: `ObservabilityConfig::load_from_toml()` →
//! `SinkRegistry::from_config()`.
//!
//! Note: BC-3.01.002 and BC-3.05.002 use `type='splunk'` (NOT `type='datadog'`)
//! as the unknown-type example per the BCs to Update section — sink-datadog
//! is now a real driver.

use factory_dispatcher::sinks::{ObservabilityConfig, SinkRegistry, SinkStanza};

// ── AC-10: Unknown sink type does not fail config load ────────────────────────

/// BC-3.01.002 PC1 — AC-10:
///
/// Config containing `type='splunk'` (an unimplemented driver) loads without
/// error. Oracle: config loads; `reg.sinks().len() == 0` (unknown type excluded).
///
/// Note: uses `type='splunk'` NOT `type='datadog'` per BC-3.01.002 + BC-3.05.002
/// BCs to Update (datadog is now a real driver as of S-4.02).
///
/// RED gate: will fail if unknown sink type panics or returns an error during
/// config load (it must only warn to stderr).
#[test]
fn test_BC_3_01_002_unknown_sink_type_splunk_warns_but_does_not_fail() {
    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![SinkStanza {
            type_: "splunk".into(), // unknown driver (NOT datadog — see BCs to Update)
            name: "splunk-prod".into(),
            dlq_enabled: false,
            extra: {
                let mut t = toml::value::Table::new();
                // Some arbitrary splunk-specific field (unknown to registry).
                t.insert("hec_url".into(), toml::Value::String("https://hec.splunk.example/events".into()));
                t
            },
        }],
    };

    // Must not fail — unknown type is warn-and-skip.
    let reg = SinkRegistry::from_config(cfg)
        .expect("BC-3.01.002: unknown sink type must not cause config load failure");

    // Oracle: unknown type excluded; registry is empty.
    assert_eq!(
        reg.sinks().len(),
        0,
        "BC-3.01.002 PC1: unknown sink type 'splunk' must be excluded; \
         registry must have 0 sinks, got {}",
        reg.sinks().len()
    );
}

/// BC-3.01.002 PC1 — mixed config: known + unknown sink types.
///
/// Config with one `type='file'` + one `type='splunk'` stanza. Oracle:
/// config loads; `reg.sinks().len() == 1` (only the file sink is constructed).
///
/// RED gate: will fail if the unknown type blocks construction of the known type.
#[test]
fn test_BC_3_01_002_mixed_known_unknown_types_unknown_excluded() {
    let tmp = tempfile::tempdir().unwrap();

    let mut file_extra = toml::value::Table::new();
    file_extra.insert(
        "path_template".into(),
        toml::Value::String(format!("{}/events-{{date}}.jsonl", tmp.path().display())),
    );
    file_extra.insert("enabled".into(), toml::Value::Boolean(true));

    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![
            SinkStanza {
                type_: "file".into(),
                name: "known-file".into(),
                dlq_enabled: false,
                extra: file_extra,
            },
            SinkStanza {
                type_: "splunk".into(), // unknown
                name: "unknown-splunk".into(),
                dlq_enabled: false,
                extra: {
                    let mut t = toml::value::Table::new();
                    t.insert("hec_url".into(), toml::Value::String("https://hec.example".into()));
                    t
                },
            },
        ],
    };

    let reg = SinkRegistry::from_config(cfg)
        .expect("mixed config with unknown type must load successfully");

    assert_eq!(
        reg.sinks().len(),
        1,
        "BC-3.01.002: mixed config must have 1 sink (file only; splunk excluded); got {}",
        reg.sinks().len()
    );
    assert_eq!(
        reg.sinks()[0].name(),
        "known-file",
        "BC-3.01.002: the surviving sink must be 'known-file'"
    );
    reg.shutdown_all();
}

// ── AC-15: Config load integration ───────────────────────────────────────────

/// BC-3.05.001 PC1 — AC-15:
///
/// `ObservabilityConfig` with one `type=file` stanza loads into a single-sink
/// registry where `reg.sinks().len() == 1` AND `reg.sinks()[0].name() == 'local-events'`.
///
/// SUT entry point: `ObservabilityConfig::load_from_toml()` →
/// `SinkRegistry::from_config()`.
///
/// RED gate: will fail if `from_config` fails to build a file sink from a
/// correctly formed file stanza.
#[test]
fn test_BC_3_05_001_file_stanza_loads_into_single_sink_registry() {
    let tmp = tempfile::tempdir().unwrap();

    let mut extra = toml::value::Table::new();
    extra.insert("enabled".into(), toml::Value::Boolean(true));
    extra.insert(
        "path_template".into(),
        toml::Value::String(format!(
            "{}/local-events-{{date}}.jsonl",
            tmp.path().display()
        )),
    );

    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![SinkStanza {
            type_: "file".into(),
            name: "local-events".into(),
            dlq_enabled: false,
            extra,
        }],
    };

    let reg = SinkRegistry::from_config(cfg)
        .expect("BC-3.05.001: config load must not return error");

    // Oracle: exactly 1 sink.
    assert_eq!(
        reg.sinks().len(),
        1,
        "BC-3.05.001 PC1: registry must have exactly 1 sink; got {}",
        reg.sinks().len()
    );

    // Oracle: sink name is 'local-events'.
    assert_eq!(
        reg.sinks()[0].name(),
        "local-events",
        "BC-3.05.001 PC1: sink name must be 'local-events'; got '{}'",
        reg.sinks()[0].name()
    );

    reg.shutdown_all();
}

/// BC-3.05.001 PC1 — TOML file path variant.
///
/// Uses `SinkRegistry::load()` (disk-based) to exercise the full
/// `load_from_toml → from_config` path. Oracle: same as above.
///
/// RED gate: will fail if TOML file loading or config parsing fails.
#[test]
fn test_BC_3_05_001_toml_file_load_builds_single_file_sink_registry() {
    let tmp = tempfile::tempdir().unwrap();
    let local_path = tmp.path().join("local-events-{date}.jsonl");
    let toml_src = format!(
        r#"
schema_version = 1

[[sinks]]
type = "file"
name = "local-events"
enabled = true
path_template = "{path}"
"#,
        path = local_path.display().to_string().replace('\\', "/")
    );

    let cfg_path = tmp.path().join("observability-config.toml");
    std::fs::write(&cfg_path, toml_src).unwrap();

    let reg = SinkRegistry::load(&cfg_path)
        .expect("BC-3.05.001: TOML file load must succeed");

    assert_eq!(reg.sinks().len(), 1, "BC-3.05.001: 1 sink from TOML file load");
    assert_eq!(
        reg.sinks()[0].name(),
        "local-events",
        "BC-3.05.001: sink name must be 'local-events'"
    );

    reg.shutdown_all();
}

/// AC-9 context — AC-15 validates config-load integration independently.
///
/// This test verifies the assembled `Router::submit()` path works end-to-end
/// from a TOML config file, exercising the full S-4.07 integration surface.
///
/// RED gate: will fail if any part of the config → registry → router chain
/// is broken.
#[test]
fn test_BC_3_05_001_router_submit_works_end_to_end_from_toml_config() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");
    let events_path_template = format!(
        "{}/events-{{date}}.jsonl",
        tmp.path().display()
    );
    let toml_src = format!(
        r#"
schema_version = 1

[[sinks]]
type = "file"
name = "local-events"
enabled = true
path_template = "{template}"
"#,
        template = events_path_template.replace('\\', "/")
    );

    let cfg_path = tmp.path().join("observability-config.toml");
    std::fs::write(&cfg_path, toml_src).unwrap();

    let registry = factory_dispatcher::sinks::SinkRegistry::load(&cfg_path).unwrap();
    let router = factory_dispatcher::sinks::Router::new(registry);

    router.submit(sink_core::SinkEvent::new().insert("type", "commit.made"));
    router.flush().expect("flush");
    router.shutdown();

    let events_path = tmp.path().join(format!("events-{date}.jsonl"));
    assert!(events_path.exists(), "output file must exist after submit+flush+shutdown");
    let content = std::fs::read_to_string(&events_path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "BC-3.05.001: 1 event must be written end-to-end; got {}",
        lines.len()
    );
}
