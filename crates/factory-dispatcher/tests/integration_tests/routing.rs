//! AC-3: Multi-sink routing filter — allow-list routing with 3 events / 2 sinks.
//!
//! Traces to:
//! - BC-3.02.009 INVARIANT 1: per-sink filtering works at FileSink layer
//! - BC-3.01.004 PC1: allow-then-deny semantics
//! - BC-3.05.002 INVARIANT 1: end-to-end fan-out wired through Router
//!
//! SUT entry point: `Router::submit()` with 3 events (`commit.made`,
//! `plugin.timeout`, `audit.access`) through a 2-sink config:
//! - audit sink: `allow=['audit.*']` — receives only `audit.access`
//! - metrics sink: `allow=['plugin.*']` — receives only `plugin.timeout`
//!
//! Oracle: audit sink file = 1 line with type='audit.access';
//!         metrics sink file = 1 line with type='plugin.timeout';
//!         neither file contains 'commit.made'.
//!
//! RED gate: will fail until Router per-sink routing filter evaluation is
//! correctly wired for allow-list patterns.

use factory_dispatcher::sinks::{Router, SinkRegistry};
use sink_core::{RoutingFilter, SinkEvent};
use sink_file::{FileSink, FileSinkConfig};

fn make_file_sink(
    name: &str,
    tmp: &std::path::Path,
    routing_filter: Option<RoutingFilter>,
) -> FileSink {
    let path_template = format!("{}/{name}-{{date}}.jsonl", tmp.display());
    FileSink::new(
        FileSinkConfig {
            name: name.to_string(),
            enabled: true,
            path_template,
            queue_depth: 64,
            routing_filter,
            tags: Default::default(),
        },
        None,
    )
    .unwrap_or_else(|e| panic!("FileSink::new({name}) failed: {e}"))
}

/// BC-3.02.009 + BC-3.01.004 + BC-3.05.002 — AC-3:
///
/// 3 events through 2 allow-list sinks. audit sink allows only `audit.*`;
/// metrics sink allows only `plugin.*`. `commit.made` must be silently
/// dropped by both sinks (no matching allow-list entry).
///
/// RED gate: will fail if allow-list routing is not correctly applied by Router.
#[test]
fn test_BC_3_02_009_multi_sink_allow_list_routing_filters_correctly() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    // Audit sink: allow=['audit.access'] (exact match).
    let audit_sink = make_file_sink(
        "audit",
        tmp.path(),
        Some(RoutingFilter {
            event_types_allow: vec!["audit.access".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        }),
    );

    // Metrics sink: allow=['plugin.timeout'] (exact match).
    let metrics_sink = make_file_sink(
        "metrics",
        tmp.path(),
        Some(RoutingFilter {
            event_types_allow: vec!["plugin.timeout".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        }),
    );

    let registry = SinkRegistry::with_sinks(vec![Box::new(audit_sink), Box::new(metrics_sink)]);
    let router = Router::new(registry);

    // Submit 3 events per AC-3 spec.
    router.submit(SinkEvent::new().insert("type", "commit.made"));
    router.submit(SinkEvent::new().insert("type", "plugin.timeout"));
    router.submit(SinkEvent::new().insert("type", "audit.access"));

    router.flush().expect("flush must succeed");
    router.shutdown();

    // ── Oracle: audit sink ────────────────────────────────────────────────────

    let audit_path = tmp.path().join(format!("audit-{date}.jsonl"));
    assert!(
        audit_path.exists(),
        "AC-3: audit sink file must exist; checked: {audit_path:?}"
    );
    let audit_content = std::fs::read_to_string(&audit_path).unwrap();
    let audit_lines: Vec<&str> = audit_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    assert_eq!(
        audit_lines.len(),
        1,
        "BC-3.02.009: audit sink must contain exactly 1 line (audit.access only); \
         got {} lines: {:?}",
        audit_lines.len(),
        audit_lines
    );

    let audit_parsed: serde_json::Value =
        serde_json::from_str(audit_lines[0]).expect("audit line must be valid JSON");
    assert_eq!(
        audit_parsed["type"], "audit.access",
        "BC-3.01.004: audit sink must contain type='audit.access'; got: {audit_parsed}"
    );

    // ── Oracle: metrics sink ──────────────────────────────────────────────────

    let metrics_path = tmp.path().join(format!("metrics-{date}.jsonl"));
    assert!(
        metrics_path.exists(),
        "AC-3: metrics sink file must exist; checked: {metrics_path:?}"
    );
    let metrics_content = std::fs::read_to_string(&metrics_path).unwrap();
    let metrics_lines: Vec<&str> = metrics_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    assert_eq!(
        metrics_lines.len(),
        1,
        "BC-3.02.009: metrics sink must contain exactly 1 line (plugin.timeout only); \
         got {} lines: {:?}",
        metrics_lines.len(),
        metrics_lines
    );

    let metrics_parsed: serde_json::Value =
        serde_json::from_str(metrics_lines[0]).expect("metrics line must be valid JSON");
    assert_eq!(
        metrics_parsed["type"], "plugin.timeout",
        "BC-3.01.004: metrics sink must contain type='plugin.timeout'; got: {metrics_parsed}"
    );

    // ── Oracle: neither file contains 'commit.made' ───────────────────────────

    for line in &audit_lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_ne!(
            parsed["type"], "commit.made",
            "BC-3.02.009: audit sink must NOT contain commit.made"
        );
    }
    for line in &metrics_lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_ne!(
            parsed["type"], "commit.made",
            "BC-3.02.009: metrics sink must NOT contain commit.made"
        );
    }
}

/// BC-3.01.004 PC1 — empty allow-list is pass-through; deny applied after allow.
///
/// Sink with empty allow-list + deny=['internal.sink_error'] must accept
/// all event types EXCEPT `internal.sink_error`.
///
/// RED gate: will fail if empty allow-list is not treated as pass-through
/// or if deny-after-allow order is not enforced.
#[test]
fn test_BC_3_01_004_empty_allow_list_is_passthrough_deny_applied_after() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    let sink = make_file_sink(
        "passthrough-deny",
        tmp.path(),
        Some(RoutingFilter {
            event_types_allow: vec![], // empty = pass-through
            event_types_deny: vec!["internal.sink_error".into()],
            plugin_ids_allow: vec![],
        }),
    );

    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    router.submit(SinkEvent::new().insert("type", "commit.made"));
    router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
    router.submit(SinkEvent::new().insert("type", "internal.sink_error")); // denied

    router.flush().expect("flush");
    router.shutdown();

    let path = tmp.path().join(format!("passthrough-deny-{date}.jsonl"));
    assert!(path.exists(), "file must exist");
    let content = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines.len(),
        2,
        "BC-3.01.004: empty allow + deny=['internal.sink_error'] must pass 2 of 3 events; got {}",
        lines.len()
    );

    for line in &lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_ne!(
            parsed["type"], "internal.sink_error",
            "BC-3.01.004: denied type must not appear in output"
        );
    }
}
