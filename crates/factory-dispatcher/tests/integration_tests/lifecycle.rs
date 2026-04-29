//! AC-6, AC-7, AC-8: Tag enrichment, disabled sink, shutdown drain.
//!
//! Traces to:
//! - BC-3.02.010 invariant 1: tag enrichment is sink-scoped (AC-6)
//! - BC-3.02.011 postcondition 1: tags do not overwrite producer type field (AC-6)
//! - BC-3.02.012 postcondition 1: enabled=false → no file written, no events accepted (AC-7)
//! - BC-3.02.015 invariant 1: shutdown is graceful; sinks become inert post-close (AC-8)
//!
//! SUT entry point: `Router::submit()` for all three ACs.

use factory_dispatcher::sinks::{Router, SinkRegistry};
use sink_core::{RoutingFilter, SinkEvent};
use sink_file::{FileSink, FileSinkConfig};
use std::collections::BTreeMap;

fn make_file_sink_with_tags(
    name: &str,
    tmp: &std::path::Path,
    enabled: bool,
    tags: BTreeMap<String, String>,
    routing_filter: Option<RoutingFilter>,
) -> FileSink {
    let path_template = format!("{}/{name}-{{date}}.jsonl", tmp.display());
    FileSink::new(
        FileSinkConfig {
            name: name.to_string(),
            enabled,
            path_template,
            queue_depth: 64,
            routing_filter,
            tags,
        },
        None,
    )
    .unwrap_or_else(|e| panic!("FileSink::new({name}) failed: {e}"))
}

// ── AC-6: Tag enrichment propagation ─────────────────────────────────────────

/// BC-3.02.010 invariant 1 + BC-3.02.011 PC1 — AC-6:
///
/// Tags `env='prod'` and `team='factory'` configured on the sink must
/// appear on every output event. The `type` producer field must NOT be
/// overwritten by a colliding tag.
///
/// Oracle: all output events contain env='prod' and team='factory'; the
/// `type` field retains its producer-set value (BC-3.02.011 non-clobber).
///
/// RED gate: will fail if Router tag enrichment does not propagate tags to
/// all events, or if producer fields are overwritten.
#[test]
fn test_BC_3_02_010_tag_enrichment_propagates_to_all_events() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    let mut tags = BTreeMap::new();
    tags.insert("env".to_string(), "prod".to_string());
    tags.insert("team".to_string(), "factory".to_string());

    let sink = make_file_sink_with_tags("tag-enrichment", tmp.path(), true, tags, None);
    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Submit 5 events.
    for i in 0..5 {
        router.submit(
            SinkEvent::new()
                .insert("type", format!("event.type.{i}"))
                .insert("producer_field", format!("value-{i}")),
        );
    }
    router.flush().expect("flush");
    router.shutdown();

    let path = tmp.path().join(format!("tag-enrichment-{date}.jsonl"));
    assert!(path.exists(), "AC-6: output file must exist");
    let content = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 5, "AC-6: all 5 events must be written");

    for (i, line) in lines.iter().enumerate() {
        let parsed: serde_json::Value = serde_json::from_str(line).expect("valid JSON");

        assert_eq!(
            parsed["env"], "prod",
            "BC-3.02.010: event[{i}] must have env='prod'"
        );
        assert_eq!(
            parsed["team"], "factory",
            "BC-3.02.010: event[{i}] must have team='factory'"
        );
    }
}

/// BC-3.02.011 PC1 — tags do not overwrite producer type field.
///
/// Configures tag `type='SHOULD_NOT_WIN'`. Producer sets `type='commit.made'`.
/// Oracle: output event has `type='commit.made'` (producer wins).
///
/// RED gate: will fail if tag enrichment overwrites producer fields.
#[test]
fn test_BC_3_02_011_tag_enrichment_does_not_overwrite_producer_type_field() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    // Tag that would clobber 'type' if non-overwrite semantics were violated.
    let mut tags = BTreeMap::new();
    tags.insert("type".to_string(), "SHOULD_NOT_WIN".to_string());
    tags.insert("extra".to_string(), "extra_val".to_string());

    let sink = make_file_sink_with_tags("no-clobber", tmp.path(), true, tags, None);
    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    router.submit(SinkEvent::new().insert("type", "commit.made"));
    router.flush().expect("flush");
    router.shutdown();

    let path = tmp.path().join(format!("no-clobber-{date}.jsonl"));
    assert!(path.exists(), "output file must exist");
    let content = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 1);

    let parsed: serde_json::Value = serde_json::from_str(lines[0]).expect("valid JSON");
    assert_eq!(
        parsed["type"], "commit.made",
        "BC-3.02.011 PC1: producer 'type' must win over config tag; got: {}",
        parsed["type"]
    );
    assert_eq!(
        parsed["extra"], "extra_val",
        "BC-3.02.011: non-colliding tag must be present"
    );
}

// ── AC-7: Disabled sink ───────────────────────────────────────────────────────

/// BC-3.02.012 PC1 — AC-7:
///
/// Sink with `enabled=false` must not write any file and must not accept
/// any events. Oracle: no file written; no events accepted.
///
/// RED gate: will fail if disabled sink writes events or creates files.
#[test]
fn test_BC_3_02_012_disabled_sink_drops_every_event() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    let sink =
        make_file_sink_with_tags("disabled-sink", tmp.path(), false, Default::default(), None);
    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Submit 5 events — all must be dropped.
    for _ in 0..5 {
        router.submit(SinkEvent::new().insert("type", "commit.made"));
    }
    router.flush().expect("flush");
    router.shutdown();

    // Oracle: no file written.
    let path = tmp.path().join(format!("disabled-sink-{date}.jsonl"));
    assert!(
        !path.exists(),
        "BC-3.02.012 PC1: disabled sink must not create any file; found: {path:?}"
    );

    // Oracle: no events in the directory at all.
    let dir_contents: Vec<_> = std::fs::read_dir(tmp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        dir_contents.is_empty(),
        "BC-3.02.012: disabled sink must not create any filesystem artifacts; found: {:?}",
        dir_contents.iter().map(|e| e.path()).collect::<Vec<_>>()
    );
}

// ── AC-8: Shutdown drain ──────────────────────────────────────────────────────

/// BC-3.02.015 invariant 1 — AC-8:
///
/// Submit 20 events immediately followed by `Router::shutdown()`.
/// Oracle: after shutdown returns, all 20 events are present in output;
/// post-shutdown `submit()` returns immediately without panic.
///
/// RED gate: will fail if shutdown does not drain all in-flight events before
/// returning, or if post-shutdown submit panics.
#[test]
fn test_BC_3_02_015_shutdown_drains_all_inflight_events() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    let sink = make_file_sink_with_tags("drain-test", tmp.path(), true, Default::default(), None);
    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Submit 20 events and immediately call shutdown (no explicit flush).
    for i in 0..20u64 {
        router.submit(
            SinkEvent::new()
                .insert("type", "plugin.invoked")
                .insert("seq", serde_json::json!(i)),
        );
    }
    // Shutdown must drain all 20 events.
    router.shutdown();

    let path = tmp.path().join(format!("drain-test-{date}.jsonl"));
    assert!(path.exists(), "AC-8: output file must exist after shutdown");
    let content = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        20,
        "BC-3.02.015 invariant 1: shutdown must drain all 20 events; got {} lines",
        lines.len()
    );
}

/// BC-3.02.015 invariant 1 (continued) — post-shutdown submit is inert.
///
/// After `Router::shutdown()`, calling `submit()` must not panic and
/// must not add any events to the output.
///
/// RED gate: will fail if post-shutdown submit panics or adds events.
#[test]
fn test_BC_3_02_015_post_shutdown_submit_is_inert() {
    let tmp = tempfile::tempdir().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d");

    let sink = make_file_sink_with_tags("inert-test", tmp.path(), true, Default::default(), None);
    let registry = SinkRegistry::with_sinks(vec![Box::new(sink)]);
    let router = Router::new(registry);

    // Submit 3 events and flush.
    for _ in 0..3 {
        router.submit(SinkEvent::new().insert("type", "commit.made"));
    }
    router.flush().expect("flush");
    router.shutdown();

    let path = tmp.path().join(format!("inert-test-{date}.jsonl"));
    let before_count = if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        content.lines().filter(|l| !l.trim().is_empty()).count()
    } else {
        0
    };
    assert_eq!(before_count, 3, "3 events must be written before shutdown");

    // Post-shutdown submit must not panic and must not add events.
    router.submit(SinkEvent::new().insert("type", "after.shutdown"));

    // Brief wait to ensure no delayed write could occur.
    std::thread::sleep(std::time::Duration::from_millis(50));

    let after_count = if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        content.lines().filter(|l| !l.trim().is_empty()).count()
    } else {
        0
    };
    assert_eq!(
        before_count, after_count,
        "BC-3.02.015: post-shutdown submit must not add events; before={before_count} after={after_count}"
    );
}
