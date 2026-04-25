//! End-to-end internal-log test: construct an `InternalLog` in a
//! tempdir, write a dispatcher-startup-style flow (`dispatcher.started`
//! plus a handful of companion events), read each file back, parse
//! every line as JSON, and verify the shape.
//!
//! Distinct from the unit tests in `internal_log.rs` — those operate
//! against in-module helpers; this one exercises the library-level
//! re-exports so we catch accidental pub-surface breakage before a
//! downstream caller (`main.rs`, future plugin_loader, S-1.8 sink
//! wiring) has to.

use factory_dispatcher::internal_log::{
    DISPATCHER_STARTED, INTERNAL_DISPATCHER_ERROR, INTERNAL_EVENT_SCHEMA_VERSION, InternalEvent,
    InternalLog, PLUGIN_INVOKED, PLUGIN_LOADED,
};
use serde_json::Value;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn read_lines(path: &Path) -> Vec<String> {
    let f = fs::File::open(path).unwrap();
    std::io::BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

#[test]
fn startup_flow_writes_parseable_jsonl() {
    let dir = tempfile::tempdir().unwrap();
    // Intentionally point at a nested path that does not exist yet to
    // exercise the mkdir-p semantics in the same call path main.rs
    // takes on a fresh checkout.
    let log_dir = dir.path().join("repo").join(".factory").join("logs");
    let log = InternalLog::new(log_dir.clone());

    let trace_id = "trace-integration-1";
    let session_id = "sess-integration-1";

    log.write(
        &InternalEvent::now(DISPATCHER_STARTED)
            .with_trace_id(trace_id)
            .with_session_id(session_id)
            .with_field("dispatcher_version", "0.0.1")
            .with_field("host_abi_version", 1_i64)
            .with_field("platform", "macos-aarch64")
            .with_field("pid", 4242_i64)
            .with_field("registry_path", "/tmp/hooks-registry.toml")
            .with_field("loaded_plugin_count", 3_i64),
    );

    log.write(
        &InternalEvent::now(PLUGIN_LOADED)
            .with_trace_id(trace_id)
            .with_plugin_name("capture-commit-activity")
            .with_plugin_version("0.1.0"),
    );

    log.write(
        &InternalEvent::now(PLUGIN_INVOKED)
            .with_trace_id(trace_id)
            .with_session_id(session_id)
            .with_plugin_name("capture-commit-activity")
            .with_plugin_version("0.1.0")
            .with_field("event_name", "PostToolUse")
            .with_field("tool_name", "Bash"),
    );

    log.write(
        &InternalEvent::now(INTERNAL_DISPATCHER_ERROR)
            .with_trace_id(trace_id)
            .with_field("message", "synthetic failure for integration coverage"),
    );

    // Exactly one rotated file for "today" (all events use `now`), and
    // it should contain four lines.
    let files: Vec<_> = fs::read_dir(&log_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("dispatcher-internal-")
        })
        .collect();
    assert_eq!(
        files.len(),
        1,
        "expected exactly one rotated file, got {files:?}"
    );

    let path = files[0].path();
    let lines = read_lines(&path);
    assert_eq!(lines.len(), 4, "expected 4 events, got {}", lines.len());

    let events: Vec<Value> = lines
        .iter()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    // All lines carry the common envelope fields.
    for ev in &events {
        assert_eq!(ev["schema_version"], INTERNAL_EVENT_SCHEMA_VERSION);
        assert!(ev["ts"].is_string());
        assert!(ev["ts_epoch"].is_i64());
        assert_eq!(ev["dispatcher_trace_id"], trace_id);
    }

    assert_eq!(events[0]["type"], DISPATCHER_STARTED);
    assert_eq!(events[0]["session_id"], session_id);
    assert_eq!(events[0]["dispatcher_version"], "0.0.1");
    assert_eq!(events[0]["loaded_plugin_count"], 3);

    assert_eq!(events[1]["type"], PLUGIN_LOADED);
    assert_eq!(events[1]["plugin_name"], "capture-commit-activity");
    assert_eq!(events[1]["plugin_version"], "0.1.0");
    assert!(
        events[1].get("session_id").is_none(),
        "plugin.loaded was not given a session id, should be omitted"
    );

    assert_eq!(events[2]["type"], PLUGIN_INVOKED);
    assert_eq!(events[2]["tool_name"], "Bash");

    assert_eq!(events[3]["type"], INTERNAL_DISPATCHER_ERROR);
    assert_eq!(
        events[3]["message"],
        "synthetic failure for integration coverage"
    );
}

#[test]
fn write_is_best_effort_when_path_is_a_file() {
    // If the log_dir path points at an existing *file*, create_dir_all
    // will fail; verify we swallow the error rather than panic.
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not-a-dir");
    fs::write(&file_path, b"hello").unwrap();

    let log = InternalLog::new(file_path);
    log.write(&InternalEvent::now(DISPATCHER_STARTED));
    // Reaching this assertion means no panic; that is the contract.
}
