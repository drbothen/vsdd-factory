//! Integration test: per-sink routing filters — two FileSinks with
//! different event-type filters. (S-4.06 AC#7, BC-3.04.004)
//!
//! AC#7 SETUP:
//!   - Sink-A: `event_types_allow=['commit.made']`, path `{tmp}/sink_a.jsonl`
//!   - Sink-B: `event_types_deny=['plugin.invoked']`, path `{tmp}/sink_b.jsonl`
//!
//! AC#7 INPUT: submit 3 events via Router::submit():
//!   1. type='commit.made'
//!   2. type='plugin.invoked'
//!   3. type='sink_error'
//!
//! AC#7 ORACLE:
//!   - sink_a.jsonl: exactly 1 line with type='commit.made'
//!   - sink_b.jsonl: exactly 2 lines with type='commit.made' and
//!     type='sink_error' (plugin.invoked denied)

use factory_dispatcher::sinks::{Router, SinkRegistry};
use sink_core::{RoutingFilter, SinkEvent};
use sink_file::{FileSink, FileSinkConfig};

fn make_sink(name: &str, path_template: &str, routing_filter: Option<RoutingFilter>) -> FileSink {
    FileSink::new(
        FileSinkConfig {
            name: name.to_string(),
            enabled: true,
            path_template: path_template.to_string(),
            queue_depth: 64,
            routing_filter,
            tags: Default::default(),
        },
        None,
    )
    .unwrap_or_else(|e| panic!("FileSink::new({name}) failed: {e}"))
}

/// AC#7 — BC-3.04.004 (Router-layer dispatch wiring):
/// Two FileSinks with different filters receive the correct subset of
/// events.
///
/// PRODUCTION ENTRY POINT: Router::submit() directly.
#[test]
fn test_BC_3_04_004_two_sinks_different_filters_correct_routing() {
    let tmp = tempfile::tempdir().unwrap();

    // Sink-A: allow-list ['commit.made'] only.
    let path_a = format!("{}/sink_a_{{date}}.jsonl", tmp.path().display());
    let sink_a = make_sink(
        "Sink-A",
        &path_a,
        Some(RoutingFilter {
            event_types_allow: vec!["commit.made".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        }),
    );

    // Sink-B: deny-list ['plugin.invoked'] — everything else passes.
    let path_b = format!("{}/sink_b_{{date}}.jsonl", tmp.path().display());
    let sink_b = make_sink(
        "Sink-B",
        &path_b,
        Some(RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec!["plugin.invoked".into()],
            plugin_ids_allow: vec![],
        }),
    );

    let registry = SinkRegistry::with_sinks(vec![Box::new(sink_a), Box::new(sink_b)]);
    let router = Router::new(registry);

    // Submit 3 events via Router::submit().
    router.submit(SinkEvent::new().insert("type", "commit.made"));
    router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
    router.submit(SinkEvent::new().insert("type", "sink_error"));

    router.flush().unwrap();
    router.shutdown();

    let date = chrono::Local::now().format("%Y-%m-%d");

    // ── ORACLE: Sink-A must have exactly 1 line: commit.made ─────────────────
    let sink_a_path = tmp.path().join(format!("sink_a_{date}.jsonl"));
    let content_a = std::fs::read_to_string(&sink_a_path).unwrap_or_default();
    let lines_a: Vec<&str> = content_a.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines_a.len(),
        1,
        "BC-3.04.004 AC#7: Sink-A must have exactly 1 line (commit.made only); got {} lines: {:?}",
        lines_a.len(),
        lines_a
    );

    let parsed_a: serde_json::Value =
        serde_json::from_str(lines_a[0]).expect("Sink-A line must be valid JSON");
    assert_eq!(
        parsed_a["type"], "commit.made",
        "BC-3.04.004 AC#7: Sink-A line must be commit.made; got: {parsed_a}"
    );

    // ── ORACLE: Sink-B must have exactly 2 lines: commit.made + sink_error ───
    let sink_b_path = tmp.path().join(format!("sink_b_{date}.jsonl"));
    let content_b = std::fs::read_to_string(&sink_b_path).unwrap_or_default();
    let lines_b: Vec<&str> = content_b.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines_b.len(),
        2,
        "BC-3.04.004 AC#7: Sink-B must have exactly 2 lines (commit.made + sink_error); got {} lines: {:?}",
        lines_b.len(),
        lines_b
    );

    let types_b: Vec<String> = lines_b
        .iter()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter_map(|v| v["type"].as_str().map(|s| s.to_string()))
        .collect();

    assert!(
        types_b.contains(&"commit.made".to_string()),
        "BC-3.04.004 AC#7: Sink-B must contain commit.made; got: {types_b:?}"
    );
    assert!(
        types_b.contains(&"sink_error".to_string()),
        "BC-3.04.004 AC#7: Sink-B must contain sink_error; got: {types_b:?}"
    );
    assert!(
        !types_b.contains(&"plugin.invoked".to_string()),
        "BC-3.04.004 AC#7: Sink-B must NOT contain plugin.invoked (denied); got: {types_b:?}"
    );
}

/// AC#4 + AC#5 — BC-3.06.007: plugin_ids_allow field.
/// Only events from listed plugins pass; empty list = pass-through.
#[test]
fn test_BC_3_06_007_plugin_ids_allow_filters_non_listed_plugins() {
    let tmp = tempfile::tempdir().unwrap();
    let path = format!("{}/plugin-filter-{{date}}.jsonl", tmp.path().display());

    // Sink: only accept events from 'capture-commit-activity'.
    let sink = make_sink(
        "plugin-filter",
        &path,
        Some(RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec![],
            plugin_ids_allow: vec!["capture-commit-activity".into()],
        }),
    );

    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Event from listed plugin — must pass.
    router.submit(
        SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_id", "capture-commit-activity"),
    );

    // Event from unlisted plugin — must be filtered.
    router.submit(
        SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_id", "other-plugin"),
    );

    // Event with no plugin_id — must be filtered (BC-3.06.007 EC-001).
    router.submit(SinkEvent::new().insert("type", "commit.made"));

    router.flush().unwrap();
    router.shutdown();

    let date = chrono::Local::now().format("%Y-%m-%d");
    let sink_path = tmp.path().join(format!("plugin-filter-{date}.jsonl"));
    let content = std::fs::read_to_string(&sink_path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines.len(),
        1,
        "BC-3.06.007: only the event from 'capture-commit-activity' must pass; got {} lines: {:?}",
        lines.len(),
        lines
    );

    let parsed: serde_json::Value =
        serde_json::from_str(lines[0]).expect("line must be valid JSON");
    assert_eq!(
        parsed["plugin_id"], "capture-commit-activity",
        "BC-3.06.007: surviving event must be from capture-commit-activity"
    );
}

/// AC#6 — BC-3.06.006: case-sensitive matching in allow-list.
/// 'Commit.Made' does NOT match 'commit.made'.
#[test]
fn test_BC_3_06_006_allow_list_case_sensitive_in_router_dispatch() {
    let tmp = tempfile::tempdir().unwrap();
    let path = format!("{}/case-sensitive-{{date}}.jsonl", tmp.path().display());

    // Allow-list contains 'Commit.Made' (uppercase C+M).
    let sink = make_sink(
        "case-sensitive",
        &path,
        Some(RoutingFilter {
            event_types_allow: vec!["Commit.Made".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        }),
    );

    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Submit lowercase 'commit.made' — must NOT pass (case-sensitive).
    router.submit(SinkEvent::new().insert("type", "commit.made"));
    // Submit 'Commit.Made' — must pass.
    router.submit(SinkEvent::new().insert("type", "Commit.Made"));

    router.flush().unwrap();
    router.shutdown();

    let date = chrono::Local::now().format("%Y-%m-%d");
    let sink_path = tmp.path().join(format!("case-sensitive-{date}.jsonl"));
    let content = std::fs::read_to_string(&sink_path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines.len(),
        1,
        "BC-3.06.006: only 'Commit.Made' (exact case) must pass; got {} lines: {:?}",
        lines.len(),
        lines
    );

    let parsed: serde_json::Value =
        serde_json::from_str(lines[0]).expect("line must be valid JSON");
    assert_eq!(
        parsed["type"], "Commit.Made",
        "BC-3.06.006: surviving event must have type='Commit.Made'"
    );
}
