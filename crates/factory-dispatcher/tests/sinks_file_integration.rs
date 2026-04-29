//! End-to-end integration for S-1.8: construct a
//! `SinkRegistry` from a real `observability-config.toml`, push
//! multiple events through it, and verify the file-on-disk shape
//! matches the contract every downstream consumer (OTel filelog,
//! factory-query, factory-report) depends on.
//!
//! Covers:
//! - TOML parse → registry construction with multiple sink instances.
//! - Per-sink routing filter (deny-list applied).
//! - Tag enrichment (static tags land on every event).
//! - Tier-boundary flush + clean shutdown drain.

use std::fs;
use std::io::BufRead;
use std::path::Path;

use chrono::Local;
use factory_dispatcher::sinks::{Router, SinkRegistry};
use serde_json::Value;
use sink_core::SinkEvent;

fn read_lines(path: &Path) -> Vec<String> {
    let f = fs::File::open(path).unwrap();
    std::io::BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

/// Shape matches the `observability-config.toml` example in the v1.0
/// spec: one file sink with tag enrichment, one file sink with a
/// routing filter, and one unknown-driver stanza to exercise the
/// forward-compat warn-and-skip branch.
fn write_config(dir: &Path) -> std::path::PathBuf {
    let local_path = dir.join("local-{date}.jsonl");
    let audit_path = dir.join("audit-{date}.jsonl");
    let toml_src = format!(
        r#"
schema_version = 1

[[sinks]]
type = "file"
name = "local-events"
enabled = true
path_template = "{local}"
tags = {{ env = "dev", host = "ci" }}

[[sinks]]
type = "file"
name = "audit-filtered"
enabled = true
path_template = "{audit}"
routing_filter = {{ event_types_deny = ["internal.sink_error", "plugin.timeout"] }}

[[sinks]]
type = "splunk"
name = "splunk-prod"
"#,
        local = local_path.display().to_string().replace('\\', "/"),
        audit = audit_path.display().to_string().replace('\\', "/"),
    );
    let cfg_path = dir.join("observability-config.toml");
    fs::write(&cfg_path, toml_src).unwrap();
    cfg_path
}

#[test]
fn registry_fans_events_to_file_sinks_with_filter_and_tags() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_path = write_config(tmp.path());

    let registry = SinkRegistry::load(&cfg_path).expect("registry load");
    assert_eq!(
        registry.sinks().len(),
        2,
        "expected 2 file sinks (splunk is unknown — should be skipped per from_config warn-and-skip; datadog/honeycomb were promoted to known types in S-4.07)"
    );

    let router = Router::new(registry);

    // Submit a mix of event types — 10 total. Denied by the audit
    // filter: `internal.sink_error`, `plugin.timeout`.
    let events: Vec<(&str, serde_json::Map<String, Value>)> = vec![
        ("plugin.invoked", new_map([("plugin_name", "a")])),
        ("plugin.invoked", new_map([("plugin_name", "b")])),
        ("plugin.completed", new_map([("plugin_name", "a")])),
        ("plugin.completed", new_map([("plugin_name", "b")])),
        ("commit.made", new_map([("sha", "deadbeef")])),
        ("commit.made", new_map([("sha", "cafebabe")])),
        ("pr.merged", new_map([("pr", "#1")])),
        ("plugin.timeout", new_map([("plugin_name", "c")])), // filtered out of audit
        ("internal.sink_error", new_map([("sink", "x")])),   // filtered out of audit
        ("dispatcher.started", new_map([("pid", "1234")])),
    ];
    for (t, extras) in &events {
        let mut ev = SinkEvent::new().insert("type", *t);
        for (k, v) in extras {
            ev = ev.insert(k.clone(), v.clone());
        }
        router.submit(ev);
    }

    router.flush().expect("flush succeeds");

    // Read both files back. Resolved {date} is the current local date.
    let date = Local::now().format("%Y-%m-%d").to_string();
    let local_file = tmp.path().join(format!("local-{date}.jsonl"));
    let audit_file = tmp.path().join(format!("audit-{date}.jsonl"));
    assert!(local_file.exists(), "local-events file exists");
    assert!(audit_file.exists(), "audit-filtered file exists");

    let local_lines = read_lines(&local_file);
    let audit_lines = read_lines(&audit_file);
    assert_eq!(local_lines.len(), 10, "local sink accepts everything");
    assert_eq!(audit_lines.len(), 8, "audit sink denies 2 event types");

    // Tag enrichment landed on every local event.
    for line in &local_lines {
        let parsed: Value = serde_json::from_str(line).unwrap();
        assert_eq!(parsed["env"], "dev");
        assert_eq!(parsed["host"], "ci");
    }
    // Filter behavior: audit has no timeout / sink_error entries.
    for line in &audit_lines {
        let parsed: Value = serde_json::from_str(line).unwrap();
        let t = parsed["type"].as_str().unwrap();
        assert_ne!(t, "plugin.timeout");
        assert_ne!(t, "internal.sink_error");
    }

    // Type distribution spot-checks.
    let audit_types: Vec<String> = audit_lines
        .iter()
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap();
            v["type"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(
        audit_types
            .iter()
            .filter(|t| t.as_str() == "commit.made")
            .count(),
        2
    );
    assert_eq!(
        audit_types
            .iter()
            .filter(|t| t.as_str() == "plugin.invoked")
            .count(),
        2
    );

    // Clean shutdown — must drain anything still in-flight and
    // return without panic. (The flush above should have already
    // drained, but this asserts the idempotent shutdown path too.)
    router.shutdown();

    // Post-shutdown submit is a no-op; line counts stay the same.
    router.submit(SinkEvent::new().insert("type", "after.shutdown"));
    let local_after = read_lines(&local_file);
    let audit_after = read_lines(&audit_file);
    assert_eq!(local_after.len(), 10);
    assert_eq!(audit_after.len(), 8);
}

fn new_map<const N: usize>(kvs: [(&str, &str); N]) -> serde_json::Map<String, Value> {
    let mut m = serde_json::Map::new();
    for (k, v) in kvs {
        m.insert(k.into(), Value::String(v.into()));
    }
    m
}
