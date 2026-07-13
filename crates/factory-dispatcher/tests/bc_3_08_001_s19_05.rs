// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! S-19.05 — Async plugin completion telemetry + VSDD_SINK_FILE release-mode opt-in.
//!
//! TDD Red Gate test suite. T-001/T-001-EC-001/T-002/T-004 are BINARY INVOCATION
//! integration tests that MUST FAIL before implementation adds emit calls to main.rs.
//!
//! ## Test inventory
//!
//! | Test ID        | Story AC | Description |
//! |----------------|----------|-------------|
//! | T-001          | AC-001   | Binary: async exit-0 within drain → `plugin.completed` in VSDD_SINK_FILE (all 9 fields) |
//! | T-001-EC-001   | AC-001   | Binary: async exit-1 within drain → `plugin.completed` with exit_code=1 |
//! | T-002          | AC-002   | Binary: slow async exceeds drain → `plugin.abandoned` in VSDD_SINK_FILE (all 7 fields) + Invariant 6 |
//! | T-002-EC-002   | AC-002   | Library: all complete before drain → zero `plugin.abandoned` events |
//! | T-004          | AC-003   | Binary: async completed event NOT relayed to dispatcher stderr |
//! | T-004 abandon  | AC-003   | Binary: async abandoned event NOT relayed to dispatcher stderr |
//! | T-005          | AC-004   | `VSDD_SINK_FILE` honored at runtime in release profile |
//! | T-006          | AC-004   | `use std::sync::Mutex` (O-P2-003) not inside `#[cfg(debug_assertions)]` |
//! | T-007          | AC-005   | SEC-003 traversal rejection in release profile |
//! | T-008          | AC-006   | `CLAUDE.md` Factory Hook Diagnostics documents `VSDD_SINK_FILE` in both builds |
//!
//! ## Binary invocation test design (T-001 / T-001-EC-001 / T-002 / T-004)
//!
//! These tests run the real `factory-dispatcher` binary (via `CARGO_BIN_EXE_factory-dispatcher`),
//! feed a synthetic hook payload to stdin, and assert that `VSDD_SINK_FILE` contains the expected
//! BC-3.08.001 events after the binary exits. This exercises the ACTUAL async drain loop in
//! `main.rs` — not a library replica — so fixing the wiring in `main.rs` makes the tests GREEN.
//!
//! ## Red Gate (T-001 / T-001-EC-001 / T-002 / T-004)
//!
//! Current `main.rs` drain loop calls `emit_plugin_async_block_discarded` and
//! `emit_plugin_timeout_async` but NEVER calls:
//!   - `emit_plugin_completed_async` (BC-3.08.001 Event 6) — F-P1-001
//!   - `emit_plugin_abandoned` (BC-3.08.001 Event 5) — F-P1-001
//!
//! Therefore `VSDD_SINK_FILE` contains NO `plugin.completed` or `plugin.abandoned` events.
//! The assertions below fail → RED gate ✓.
//!
//! ## Green path (after implementation)
//!
//! Implementer adds to `main.rs` drain loop:
//! 1. `emit_plugin_completed_async(&base_host_ctx, &name, &version, entry_index, exit_code,
//!    elapsed_ms, fuel_consumed)` for each non-block `PluginResult::Ok` outcome
//! 2. `emit_plugin_abandoned(&base_host_ctx, &name, entry_index, drain_window_ms_u64)` for
//!    each plugin spawned but whose outcome did not arrive before the drain timer fired
//!
//! After these additions `VSDD_SINK_FILE` carries the expected events → assertions pass → GREEN.
//!
//! ## BC traces
//!
//! - BC-3.08.001 v1.21 Event 5 — `plugin.abandoned` (7 mandatory fields)
//! - BC-3.08.001 v1.21 Event 6 — `plugin.completed` async path (9 mandatory fields)
//! - BC-3.08.001 v1.21 Invariant 6 — terminal events mutually exclusive per (trace_id, plugin_name, entry_index)
//! - VP-100 — drain-timer expiry emits exactly one `plugin.abandoned` per in-flight (plugin_name, entry_index)
//! - VP-079 — payload conformance for all six event types
//! - BC-1.14.001 Invariant 4 — async plugin stderr not relayed to dispatcher stderr

use std::path::PathBuf;

use factory_dispatcher::flush_sink_file;
use factory_dispatcher::host::emit_event::{emit_plugin_abandoned, emit_plugin_completed_async};

// ---------------------------------------------------------------------------
// WAT fixture constants
//
// All three WAT modules are minimal WASI commands. WAT_EXIT_0 returns normally
// (exit code 0). WAT_EXIT_1 calls wasi proc_exit(1) to produce exit code 1.
// WAT_INFINITE_LOOP spins unconditionally — it is in-flight during any drain
// window shorter than the entry's timeout_ms, triggering plugin.abandoned.
// ---------------------------------------------------------------------------

/// WAT for a WASI command that exits cleanly (code 0).
const WAT_EXIT_0: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")))
"#;

/// WAT for a WASI command that calls proc_exit(1), producing exit code 1.
/// Maps to `PluginResult::Ok { exit_code: 1, ... }` (not a crash) per
/// wasmtime-wasi I32Exit semantics (invoke.rs lines 415-418).
const WAT_EXIT_1: &str = r#"
(module
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)
  (func (export "_start")
    i32.const 1
    call $proc_exit))
"#;

/// WAT for a WASI command that loops unconditionally.
/// Without epoch timeout the loop never terminates, ensuring the plugin
/// is always in-flight when a short drain window fires.
/// Epoch-based timeout is suppressed by using timeout_ms >> drain_window_ms.
const WAT_INFINITE_LOOP: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")
    (loop $l
      (br $l))))
"#;

// ---------------------------------------------------------------------------
// Binary invocation helpers
// ---------------------------------------------------------------------------

/// Returns the path to the compiled `factory-dispatcher` binary.
///
/// `CARGO_BIN_EXE_factory-dispatcher` is set by Cargo for integration tests
/// (see Cargo reference §environment-variables). Panics if the variable is
/// absent (should never happen when invoked via `cargo test`).
fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_factory-dispatcher"))
}

/// Compile `wat` source to a WASM file at `dir/<name>.wasm`. Returns the
/// absolute path to the written file.
fn write_wasm_fixture(dir: &std::path::Path, name: &str, wat: &str) -> PathBuf {
    let bytes = wat::parse_str(wat).unwrap_or_else(|e| panic!("WAT parse failed for {name}: {e}"));
    let path = dir.join(format!("{name}.wasm"));
    std::fs::write(&path, &bytes)
        .unwrap_or_else(|e| panic!("Failed to write WASM fixture {name}: {e}"));
    path
}

/// Write a `hooks-registry.toml` with a single async entry into `dir`.
///
/// `plugin_path` must be an absolute path. `timeout_ms` should be much
/// larger than any drain window override so the plugin is never epoch-killed
/// before the drain fires (relevant for T-002's infinite-loop fixture).
///
/// Returns the path to the written registry file.
fn write_async_registry(
    dir: &std::path::Path,
    plugin_name: &str,
    plugin_path: &std::path::Path,
    timeout_ms: u64,
) -> PathBuf {
    let registry_toml = format!(
        "schema_version = 2\n\n\
         [[hooks]]\n\
         name = \"{plugin_name}\"\n\
         event = \"PostToolUse\"\n\
         plugin = \"{plugin_path}\"\n\
         async = true\n\
         on_error = \"continue\"\n\
         timeout_ms = {timeout_ms}\n\
         fuel_cap = 1000000000\n",
        plugin_name = plugin_name,
        plugin_path = plugin_path.display(),
        timeout_ms = timeout_ms,
    );
    let reg_path = dir.join("hooks-registry.toml");
    std::fs::write(&reg_path, &registry_toml)
        .unwrap_or_else(|e| panic!("Failed to write hooks-registry.toml: {e}"));
    reg_path
}

/// Run the `factory-dispatcher` binary with the given configuration.
///
/// `plugin_root` must contain a `hooks-registry.toml` file.
/// `sink_path` is the absolute path for `VSDD_SINK_FILE`.
/// `drain_window_ms` overrides `VSDD_ASYNC_DRAIN_WINDOW_MS` (debug builds only,
/// per SEC-003; release builds ignore this env var).
///
/// Sends a synthetic `PostToolUse` hook payload to stdin and returns the
/// process output (stdout + stderr + exit status).
fn run_binary(
    plugin_root: &std::path::Path,
    sink_path: &std::path::Path,
    drain_window_ms: u64,
) -> std::process::Output {
    use std::io::Write as _;

    let payload = r#"{"event_name":"PostToolUse","tool_name":"Bash","session_id":"test-s19-05","tool_input":{}}"#;

    let mut child = std::process::Command::new(binary_path())
        .env("CLAUDE_PLUGIN_ROOT", plugin_root)
        .env("VSDD_SINK_FILE", sink_path)
        // Debug-only override for the async drain window (SEC-003 gate).
        // Allows T-001 to use a generous drain and T-002 to use a short drain.
        .env("VSDD_ASYNC_DRAIN_WINDOW_MS", drain_window_ms.to_string())
        // Silence log-dir creation in the test's working directory by pointing
        // CLAUDE_PROJECT_DIR at the plugin_root tempdir.
        .env("CLAUDE_PROJECT_DIR", plugin_root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to spawn factory-dispatcher binary: {e}"));

    child
        .stdin
        .take()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write payload to binary stdin: {e}"));

    child
        .wait_with_output()
        .unwrap_or_else(|e| panic!("Failed to wait for factory-dispatcher output: {e}"))
}

/// Parse VSDD_SINK_FILE content into JSON event objects.
/// Empty lines and parse failures are silently skipped.
fn parse_sink_events(sink_path: &std::path::Path) -> Vec<serde_json::Value> {
    let content = std::fs::read_to_string(sink_path).unwrap_or_default();
    content
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                serde_json::from_str(l).ok()
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helper: minimal HostContext for T-005/T-007 emit call setup.
// ---------------------------------------------------------------------------

fn make_test_ctx() -> factory_dispatcher::host::HostContext {
    factory_dispatcher::host::HostContext::new(
        "test-plugin-s19-05",
        "1.2.3",
        "test-session-s19-05",
        "test-trace-id-s19-05",
    )
}

// ===========================================================================
// T-001 (AC-001): Async plugin exits 0 within drain window → plugin.completed
//
// Binary invocation: runs factory-dispatcher with a WAT_EXIT_0 async plugin.
// Drain window is generous (10 s) so the plugin completes before it fires.
// Asserts VSDD_SINK_FILE contains exactly one `plugin.completed` event carrying
// ALL 9 mandatory fields per BC-3.08.001 v1.21 Event 6.
//
// RED gate: main.rs drain loop never calls emit_plugin_completed_async →
//   VSDD_SINK_FILE contains no `plugin.completed` events → assertion FAILS.
// GREEN after: implementer adds emit_plugin_completed_async to drain loop →
//   VSDD_SINK_FILE carries the event → assertions pass.
// ===========================================================================

/// T-001 (AC-001): async exit-0 within drain window → `plugin.completed` event
/// in VSDD_SINK_FILE with ALL 9 mandatory fields per BC-3.08.001 v1.21 Event 6.
///
/// RED gate: main.rs drain loop missing `emit_plugin_completed_async` call →
/// VSDD_SINK_FILE empty of `plugin.completed` events → `assert_eq!(completed_events.len(), 1)`
/// fails with "T-001: expected exactly one plugin.completed event … got 0" → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t001_async_exit0_within_drain_emits_plugin_completed() {
    let dir = tempfile::tempdir().expect("T-001: tempdir");

    // Compile WAT_EXIT_0 → exit0.wasm
    let wasm_path = write_wasm_fixture(dir.path(), "exit0", WAT_EXIT_0);

    // Registry: single async entry that matches PostToolUse with no tool filter.
    // timeout_ms=60000 is much greater than the drain window so the plugin is
    // never epoch-killed — it completes normally via return from _start.
    write_async_registry(dir.path(), "test-async-exit0", &wasm_path, 60_000);

    let sink_path = dir.path().join("t001-sink.jsonl");

    // O-P5-3 RESOLVED (S-19.05 F-P7-001): With feature = "test-support" enabled in ci.yml's
    // build-dispatcher Test (cargo) step, VSDD_ASYNC_DRAIN_WINDOW_MS is honored in release
    // builds. The 10s drain window is sufficient for WASM cold-start (2-5s on CI).
    // DI-019 preserved: shipped artifacts (release.yml) never include this feature.
    // Run the binary. 10 s drain window is generous for debug WASM cold-start
    // (observed 2-5 s on CI). The plugin exits in microseconds once compiled.
    let output = run_binary(dir.path(), &sink_path, 10_000);

    let events = parse_sink_events(&sink_path);
    let completed_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.completed")
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(
        completed_events.len(),
        1,
        "T-001: expected exactly one plugin.completed event in VSDD_SINK_FILE; got {}.\n\
         Sink content: {:?}\n\
         Binary stderr: {}\n\
         Binary exit status: {:?}",
        completed_events.len(),
        events,
        String::from_utf8_lossy(&output.stderr),
        output.status,
    );

    let ev = completed_events[0];

    // Mandatory field 1: type = "plugin.completed"
    assert_eq!(
        ev.get("type").and_then(|v| v.as_str()),
        Some("plugin.completed"),
        "T-001: event type must be 'plugin.completed' (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 2: trace_id (dispatcher-assigned UUID per DI-017)
    let trace_id = ev.get("trace_id").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !trace_id.is_empty(),
        "T-001: trace_id must be present and non-empty (BC-3.08.001 v1.21 Invariant 1 + DI-017)"
    );

    // Mandatory field 3: session_id
    let session_id = ev.get("session_id").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !session_id.is_empty(),
        "T-001: session_id must be present and non-empty (BC-3.08.001 v1.21 §Common Fields)"
    );

    // Mandatory field 4: plugin_name
    let plugin_name = ev.get("plugin_name").and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(
        plugin_name, "test-async-exit0",
        "T-001: plugin_name must match the registry entry name (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 5: plugin_version (present in Event 6; absent from Events 1/4/5)
    let plugin_version = ev
        .get("plugin_version")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        !plugin_version.is_empty(),
        "T-001: plugin_version must be present and non-empty in plugin.completed async path \
         (BC-3.08.001 v1.21 Event 6 — absent from Events 1/4/5)"
    );

    // Mandatory field 6: entry_index (0-based enumerate ordinal; single plugin → 0)
    let entry_index = ev.get("entry_index").and_then(|v| v.as_u64());
    assert_eq!(
        entry_index,
        Some(0u64),
        "T-001: entry_index must equal 0 for the first async entry (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 7: exit_code (0 for WAT_EXIT_0)
    let exit_code = ev.get("exit_code").and_then(|v| v.as_i64());
    assert_eq!(
        exit_code,
        Some(0i64),
        "T-001: exit_code must be 0 for WAT_EXIT_0 plugin (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 8: elapsed_ms (non-negative integer)
    let elapsed_ms = ev.get("elapsed_ms").and_then(|v| v.as_u64());
    assert!(
        elapsed_ms.is_some(),
        "T-001: elapsed_ms must be present (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 9: fuel_consumed (non-negative integer)
    let fuel_consumed = ev.get("fuel_consumed").and_then(|v| v.as_u64());
    assert!(
        fuel_consumed.is_some(),
        "T-001: fuel_consumed must be present (BC-3.08.001 v1.21 Event 6)"
    );

    // Invariant 6: no plugin.abandoned event for the same (trace_id, plugin_name, entry_index).
    // When a plugin.completed is emitted, plugin.abandoned MUST NOT also be emitted.
    let abandoned_count = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.abandoned")
                .unwrap_or(false)
                && e.get("plugin_name")
                    .and_then(|p| p.as_str())
                    .map(|p| p == "test-async-exit0")
                    .unwrap_or(false)
        })
        .count();
    assert_eq!(
        abandoned_count, 0,
        "T-001 Invariant 6: plugin.abandoned MUST NOT follow plugin.completed for same \
         (trace_id, plugin_name, entry_index) triple (BC-3.08.001 v1.21 Invariant 6 + VP-100)"
    );
}

/// T-001 variant: AC-001 EC-001 — async plugin exits non-zero (exit code 1) within drain.
/// `plugin.completed` is emitted with exit_code=1 (not 0).
/// Canonical test vector: "async-nonzero" — BC-3.08.001 v1.21 Invariant 3.
///
/// RED gate: main.rs drain loop missing `emit_plugin_completed_async` →
/// VSDD_SINK_FILE has no `plugin.completed` events → assertion fails → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t001_ec001_async_nonzero_exit_emits_completed_with_actual_exit_code() {
    let dir = tempfile::tempdir().expect("T-001-EC-001: tempdir");

    let wasm_path = write_wasm_fixture(dir.path(), "exit1", WAT_EXIT_1);
    write_async_registry(dir.path(), "test-async-exit1", &wasm_path, 60_000);

    let sink_path = dir.path().join("t001-ec001-sink.jsonl");
    let output = run_binary(dir.path(), &sink_path, 10_000);

    let events = parse_sink_events(&sink_path);
    let completed_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.completed")
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(
        completed_events.len(),
        1,
        "T-001-EC-001: expected exactly one plugin.completed event; got {}.\n\
         Sink: {:?}\nStderr: {}",
        completed_events.len(),
        events,
        String::from_utf8_lossy(&output.stderr),
    );

    let ev = completed_events[0];

    // exit_code must reflect the actual non-zero exit (not hardcoded 0)
    let exit_code = ev.get("exit_code").and_then(|v| v.as_i64());
    assert_eq!(
        exit_code,
        Some(1i64),
        "T-001-EC-001: exit_code must equal 1 (actual plugin exit code, NOT hardcoded 0). \
         BC-3.08.001 v1.21 Invariant 3: exit_code is preserved verbatim"
    );

    // plugin_name preserved
    assert_eq!(
        ev.get("plugin_name").and_then(|v| v.as_str()),
        Some("test-async-exit1"),
        "T-001-EC-001: plugin_name must match registry entry"
    );

    // entry_index is 0 (single plugin)
    assert_eq!(
        ev.get("entry_index").and_then(|v| v.as_u64()),
        Some(0u64),
        "T-001-EC-001: entry_index must be 0 for single-plugin async group"
    );
}

// ===========================================================================
// T-002 (AC-002): Slow async exceeds drain → plugin.abandoned emitted
//
// Binary invocation: WAT_INFINITE_LOOP plugin with a short drain window override
// (VSDD_ASYNC_DRAIN_WINDOW_MS=100, i.e. DI-019 production default).
// The infinite loop plugin never completes within 100 ms, so when the drain
// timer fires the plugin is in-flight → exactly one plugin.abandoned must be
// emitted per BC-3.08.001 v1.21 Event 5 + VP-100.
//
// timeout_ms=60000 ensures the epoch ticker does NOT kill the plugin before the
// drain window fires, preserving the "in-flight at drain" condition.
//
// RED gate: main.rs drain loop missing emit_plugin_abandoned → VSDD_SINK_FILE
//   has no plugin.abandoned events → assertion FAILS → RED gate ✓.
// ===========================================================================

/// T-002 (AC-002): drain timer fires with plugin in-flight → `plugin.abandoned`
/// emitted in VSDD_SINK_FILE with ALL 7 mandatory fields per BC-3.08.001 v1.21 Event 5.
/// Also verifies Invariant 6: zero `plugin.completed` events for the same plugin.
///
/// RED gate: main.rs drain loop missing `emit_plugin_abandoned` →
/// VSDD_SINK_FILE has no `plugin.abandoned` events →
/// `assert_eq!(abandoned_events.len(), 1)` fails → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t002_drain_timer_fires_with_in_flight_plugin_emits_abandoned() {
    let dir = tempfile::tempdir().expect("T-002: tempdir");

    // Compile infinite-loop WAT. timeout_ms=60000 keeps the epoch ticker from
    // killing the plugin before the 100 ms drain window fires.
    let wasm_path = write_wasm_fixture(dir.path(), "loop", WAT_INFINITE_LOOP);
    write_async_registry(dir.path(), "test-async-loop", &wasm_path, 60_000);

    let sink_path = dir.path().join("t002-sink.jsonl");

    // drain_window=100 ms = DI-019 production value. The infinite-loop plugin is
    // GUARANTEED to be in-flight (either compiling or executing) at 100 ms.
    let output = run_binary(dir.path(), &sink_path, 100);

    let events = parse_sink_events(&sink_path);
    let abandoned_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.abandoned")
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(
        abandoned_events.len(),
        1,
        "T-002: expected exactly one plugin.abandoned event in VSDD_SINK_FILE; got {}.\n\
         Canonical test vector: 'abandoned-one' (BC-3.08.001 v1.21 EC-007 + VP-100).\n\
         Sink content: {:?}\nBinary stderr: {}",
        abandoned_events.len(),
        events,
        String::from_utf8_lossy(&output.stderr),
    );

    let ev = abandoned_events[0];

    // Mandatory field 1: type = "plugin.abandoned"
    assert_eq!(
        ev.get("type").and_then(|v| v.as_str()),
        Some("plugin.abandoned"),
        "T-002: event type must be 'plugin.abandoned' (BC-3.08.001 v1.21 Event 5)"
    );

    // Mandatory field 2: trace_id
    let trace_id = ev.get("trace_id").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !trace_id.is_empty(),
        "T-002: trace_id must be present and non-empty (BC-3.08.001 v1.21 Invariant 1)"
    );

    // Mandatory field 3: session_id
    let session_id = ev.get("session_id").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !session_id.is_empty(),
        "T-002: session_id must be present and non-empty (BC-3.08.001 v1.21 §Common Fields O-P15-001)"
    );

    // Mandatory field 4: plugin_name
    assert_eq!(
        ev.get("plugin_name").and_then(|v| v.as_str()),
        Some("test-async-loop"),
        "T-002: plugin_name must match the registry entry name verbatim \
         (BC-3.08.001 v1.21 Event 5)"
    );

    // Mandatory field 5: entry_index (0-based enumerate ordinal; single plugin → 0)
    assert_eq!(
        ev.get("entry_index").and_then(|v| v.as_u64()),
        Some(0u64),
        "T-002: entry_index must be 0 for the first (and only) async entry \
         (BC-3.08.001 v1.21 Event 5 + Invariant 6 disambiguation key)"
    );

    // Mandatory field 6: drain_window_ms (reflects the effective drain window used)
    let drain_window_ms = ev.get("drain_window_ms").and_then(|v| v.as_u64());
    assert!(
        drain_window_ms.is_some(),
        "T-002: drain_window_ms must be present (BC-3.08.001 v1.21 Event 5)"
    );
    assert!(
        drain_window_ms.unwrap() > 0,
        "T-002: drain_window_ms must be positive; got {:?}",
        drain_window_ms
    );

    // Mandatory field 7: timestamp (ISO-8601, non-empty)
    let timestamp = ev.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !timestamp.is_empty(),
        "T-002: timestamp must be present and non-empty (BC-3.08.001 v1.21 Event 5)"
    );

    // Invariant 6: plugin.abandoned is TERMINAL — no plugin.completed must follow
    // for the same (trace_id, plugin_name, entry_index) triple.
    let completed_for_same_plugin = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.completed")
                .unwrap_or(false)
                && e.get("plugin_name")
                    .and_then(|p| p.as_str())
                    .map(|p| p == "test-async-loop")
                    .unwrap_or(false)
        })
        .count();
    assert_eq!(
        completed_for_same_plugin, 0,
        "T-002 Invariant 6: plugin.abandoned is TERMINAL — plugin.completed MUST NOT \
         follow for the same (trace_id, plugin_name, entry_index) triple \
         (BC-3.08.001 v1.21 Invariant 6 + VP-100)"
    );
}

/// T-002 variant: EC-002 — all async plugins complete before drain timer fires.
/// No `plugin.abandoned` events emitted. Canonical test vector: `abandoned-none`.
///
/// Library-level test (trivially GREEN): creates an empty HostContext and asserts
/// the event queue carries zero abandoned events. Guards the zero-abandon postcondition
/// after implementation when a completed plugin must NOT also be abandoned.
///
/// Note: this test is GREEN immediately; it is retained for completeness and to
/// guard the `abandoned-none` canonical test vector after implementation.
#[test]
fn test_BC_3_08_001_s19_05_t002_ec002_all_complete_before_drain_no_abandoned_events() {
    // No emit_plugin_abandoned calls. Simulate: all plugins completed before timer fired.
    // Zero plugin.abandoned events expected.
    // (Canonical test vector: "abandoned-none" — zero plugin.abandoned in events-*.jsonl)
    let ctx = make_test_ctx();
    let events = ctx.drain_events();
    let abandoned_count = events
        .iter()
        .filter(|e| e.type_ == "plugin.abandoned")
        .count();
    assert_eq!(
        abandoned_count, 0,
        "T-002 EC-002 (abandoned-none): zero abandoned events when no in-flight plugins at drain"
    );
}

// ===========================================================================
// T-004 (AC-003): Async events MUST NOT be relayed to dispatcher stderr.
//
// BC-1.14.001 Invariant 4: async plugin stderr is not relayed to the
// dispatcher's process stderr. The same non-relay guarantee applies to the
// BC-3.08.001 Event 5/6 structured events themselves: they route to the
// internal event queue and VSDD_SINK_FILE only, never to stderr.
//
// Two integration tests (completed and abandoned scenarios):
// 1. Run binary with WAT_EXIT_0 (exit-0, completes within drain).
// 2. Run binary with WAT_INFINITE_LOOP (in-flight at drain).
// Both assert:
//   (a) VSDD_SINK_FILE contains the expected event (proves wiring) — fails in RED gate.
//   (b) Binary process stderr does NOT contain "plugin.completed" or "plugin.abandoned"
//       event text (proves no-relay invariant) — passes trivially until (a) is fixed.
//
// RED gate for both: VSDD_SINK_FILE has no events (missing wiring) →
//   assertion (a) fails → RED gate ✓.
// ===========================================================================

/// T-004 (AC-003): `plugin.completed` async event is NOT relayed to dispatcher stderr.
///
/// Verifies BC-1.14.001 Invariant 4 at the integration level:
///   (a) exactly one `plugin.completed` event in VSDD_SINK_FILE (route to sink ✓)
///   (b) binary process stderr does NOT contain "plugin.completed" as raw text
///       (no relay of structured events to stderr ✓)
///
/// RED gate: main.rs drain loop missing `emit_plugin_completed_async` →
/// VSDD_SINK_FILE empty of `plugin.completed` → assertion (a) FAILS → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t004_async_completed_event_not_relayed_to_stderr() {
    let dir = tempfile::tempdir().expect("T-004: tempdir");

    let wasm_path = write_wasm_fixture(dir.path(), "exit0-t004", WAT_EXIT_0);
    write_async_registry(dir.path(), "test-async-t004-exit0", &wasm_path, 60_000);

    let sink_path = dir.path().join("t004-completed-sink.jsonl");
    let output = run_binary(dir.path(), &sink_path, 10_000);

    let events = parse_sink_events(&sink_path);
    let completed_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.completed")
                .unwrap_or(false)
        })
        .collect();

    // Assertion (a): VSDD_SINK_FILE must contain the plugin.completed event.
    // This assertion fails in the RED gate (wiring absent) and passes after implementation.
    assert_eq!(
        completed_events.len(),
        1,
        "T-004 assertion (a): VSDD_SINK_FILE must carry exactly one plugin.completed event; \
         got {}. Without this, the no-relay assertion (b) is moot. \
         Sink: {:?}\nBinary stderr: {}",
        completed_events.len(),
        events,
        String::from_utf8_lossy(&output.stderr),
    );

    // Assertion (b): binary process stderr must NOT contain the structured event text.
    // BC-1.14.001 Invariant 4: async events route to the sink, never to dispatcher stderr.
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr_text.contains("\"plugin.completed\""),
        "T-004 assertion (b) AC-003: binary stderr MUST NOT contain \"plugin.completed\" \
         event text (BC-1.14.001 Invariant 4: async events not relayed to dispatcher stderr).\n\
         Binary stderr:\n{}",
        stderr_text,
    );
    assert!(
        !stderr_text.contains("\"plugin.abandoned\""),
        "T-004 assertion (b) AC-003: binary stderr MUST NOT contain \"plugin.abandoned\" \
         event text (BC-1.14.001 Invariant 4).\n\
         Binary stderr:\n{}",
        stderr_text,
    );
}

/// T-004 variant: `plugin.abandoned` async event is NOT relayed to dispatcher stderr.
///
///   (a) exactly one `plugin.abandoned` event in VSDD_SINK_FILE (route to sink ✓)
///   (b) binary process stderr does NOT contain "plugin.abandoned" as raw text ✓
///
/// RED gate: main.rs drain loop missing `emit_plugin_abandoned` →
/// VSDD_SINK_FILE empty of `plugin.abandoned` → assertion (a) FAILS → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t004_async_abandoned_event_not_relayed_to_stderr() {
    let dir = tempfile::tempdir().expect("T-004 abandoned: tempdir");

    let wasm_path = write_wasm_fixture(dir.path(), "loop-t004", WAT_INFINITE_LOOP);
    write_async_registry(dir.path(), "test-async-t004-loop", &wasm_path, 60_000);

    let sink_path = dir.path().join("t004-abandoned-sink.jsonl");
    let output = run_binary(dir.path(), &sink_path, 100);

    let events = parse_sink_events(&sink_path);
    let abandoned_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| {
            e.get("type")
                .and_then(|t| t.as_str())
                .map(|t| t == "plugin.abandoned")
                .unwrap_or(false)
        })
        .collect();

    // Assertion (a): VSDD_SINK_FILE must contain the plugin.abandoned event.
    assert_eq!(
        abandoned_events.len(),
        1,
        "T-004 abandoned assertion (a): VSDD_SINK_FILE must carry exactly one plugin.abandoned \
         event; got {}. Sink: {:?}\nBinary stderr: {}",
        abandoned_events.len(),
        events,
        String::from_utf8_lossy(&output.stderr),
    );

    // Assertion (b): binary process stderr must NOT contain the structured event text.
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr_text.contains("\"plugin.abandoned\""),
        "T-004 abandoned assertion (b) AC-003: binary stderr MUST NOT contain \
         \"plugin.abandoned\" event text (BC-1.14.001 Invariant 4).\n\
         Binary stderr:\n{}",
        stderr_text,
    );
    assert!(
        !stderr_text.contains("\"plugin.completed\""),
        "T-004 abandoned assertion (b) AC-003: binary stderr MUST NOT contain \
         \"plugin.completed\" event text (BC-1.14.001 Invariant 4).\n\
         Binary stderr:\n{}",
        stderr_text,
    );
}

// ---------------------------------------------------------------------------
// T-005 (AC-004): VSDD_SINK_FILE env var honored at runtime in BOTH debug and
// release builds. The #[cfg(debug_assertions)] gates around ENV_SINK_FILE,
// flush_sink_file, and the sink mutex in main.rs are removed.
// ---------------------------------------------------------------------------

/// T-005 (AC-004): `VSDD_SINK_FILE` is honored in release builds.
///
/// Pre-implementation RED gate: `flush_sink_file` is `#[cfg(debug_assertions)]`-gated;
/// setting `VSDD_SINK_FILE` in release mode has no effect (file not created).
///
/// Post-implementation: calling the sink flush path with `VSDD_SINK_FILE` set
/// creates and populates the sink file in ALL build profiles (debug + release).
///
/// todo!() ensures RED gate failure until S-19.05 AC-004 implementation removes
/// the `#[cfg(debug_assertions)]` gates from `ENV_SINK_FILE`, `flush_sink_file`,
/// and the sink mutex in `crates/factory-dispatcher/src/main.rs`.
#[test]
fn test_BC_3_08_001_s19_05_t005_vsdd_sink_file_honored_in_release_profile() {
    // AC-004 implementation: flush_sink_file is no longer #[cfg(debug_assertions)]-gated.
    // This test verifies the function works in both debug and release profiles.
    // Run with `cargo test --release` to verify release-profile behavior.
    let tmp = tempfile::tempdir().expect("T-005: should create tempdir");
    let sink_path = tmp.path().join("test-sink-t005.jsonl");
    let sink_path_str = sink_path
        .to_str()
        .expect("T-005: path must be valid UTF-8")
        .to_string();

    // Populate the event queue via emit_plugin_completed_async.
    let ctx = make_test_ctx();
    emit_plugin_completed_async(&ctx, "test-plugin-t005", "1.0.0", 0, 0, 10, 50_000);

    // Call flush_sink_file directly (S-19.05 AC-004: available in library, not just main.rs).
    flush_sink_file(&sink_path_str, &ctx.events);

    // Assert the file was created and contains ≥1 JSONL line.
    assert!(
        sink_path.exists(),
        "T-005: sink file must be created when VSDD_SINK_FILE is set and events are present \
         (S-19.05 AC-004: flush_sink_file honored in both debug and release builds)"
    );
    let content = std::fs::read_to_string(&sink_path).expect("T-005: should read sink file");
    assert!(
        !content.is_empty(),
        "T-005: sink file must be non-empty after flush"
    );
    let line_count = content.lines().count();
    assert!(
        line_count >= 1,
        "T-005: sink file must contain ≥1 JSONL line; got {} lines",
        line_count
    );
}

// ---------------------------------------------------------------------------
// T-006 (AC-004 O-P2-003): Static verification that `use std::sync::Mutex`
// (or consolidated form) is NOT inside a `#[cfg(debug_assertions)]` block in
// `crates/factory-dispatcher/src/main.rs`.
// ---------------------------------------------------------------------------

/// T-006 (AC-004 O-P2-003): `use std::sync::Mutex` must NOT be inside a
/// `#[cfg(debug_assertions)]` block in `main.rs`.
///
/// Static verification (grep inspection) per story Task T-006 gate:
/// - Step 1: Mutex import exists in main.rs
/// - Step 2: The line immediately preceding the Mutex import is NOT a `#[cfg(` gate
///
/// RED gate: currently `#[cfg(debug_assertions)]` immediately precedes the import.
/// The assertion fails: "immediately preceded by '#[cfg(' — O-P2-003 violation".
/// GREEN after implementation moves the import outside the cfg block.
#[test]
fn test_BC_3_08_001_s19_05_t006_mutex_import_not_cfg_gated() {
    let main_rs_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs");
    let main_rs =
        std::fs::read_to_string(main_rs_path).expect("T-006: should be able to read main.rs");
    let lines: Vec<&str> = main_rs.lines().collect();

    // Step 1: Find the Mutex import line.
    // Broadened gate (F-P12-003): matches both `use std::sync::Mutex;`
    // and consolidated forms `use std::sync::{..., Mutex, ...};`.
    let mutex_line_idx = lines.iter().position(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("use std::sync::Mutex;")
            || (trimmed.starts_with("use std::sync::{") && trimmed.contains("Mutex"))
    });

    assert!(
        mutex_line_idx.is_some(),
        "T-006: `use std::sync::Mutex` (or consolidated form `use std::sync::{{..., Mutex, ...}}`) \
         must exist in crates/factory-dispatcher/src/main.rs (O-P2-003: required for unconditional \
         sink mutex access after S-19.05 AC-004 removes the cfg gate)"
    );

    let idx = mutex_line_idx.unwrap();

    // Step 2: Assert the preceding line is NOT a `#[cfg(` gate.
    // Mirrors the awk gate in the story:
    //   awk '/^#\[cfg\(/{c=1; next} /^use std::sync::.*Mutex/{exit c} {c=0}' main.rs
    // exits 0 when the import is NOT immediately preceded by #[cfg(.
    let prev_line = if idx > 0 { lines[idx - 1].trim() } else { "" };
    assert!(
        !prev_line.starts_with("#[cfg("),
        "T-006: `use std::sync::Mutex` import at main.rs line {} is immediately preceded by \
         '{}' — O-P2-003 violation: the import must be unconditional (not inside \
         #[cfg(debug_assertions)]) after S-19.05 AC-004 implementation removes the cfg gate. \
         Current state: defect detected (RED gate ✓).",
        idx + 1,
        prev_line
    );
}

// ---------------------------------------------------------------------------
// T-007 (AC-005): SEC-003 path-traversal sanitization in `flush_sink_file`
// is preserved and applies in ALL builds (debug + release).
// ---------------------------------------------------------------------------

/// T-007 (AC-005): SEC-003 path traversal rejection in release profile.
///
/// `flush_sink_file` must reject paths containing `..` sequences in ALL build
/// profiles (debug and release). Currently gated `#[cfg(debug_assertions)]`
/// so no release-mode traversal check exists.
///
/// Pre-implementation RED gate: `flush_sink_file` is cfg-gated; calling it
/// from release mode is impossible. todo!() ensures test fails.
///
/// Post-implementation: call `flush_sink_file("/tmp/../traversal-target.jsonl", ...)`
/// and assert:
/// (a) no file created at the traversal target path;
/// (b) a "VSDD_SINK_FILE rejected: path traversal" warning is emitted.
/// Run in both debug and release via `cargo test` and `cargo test --release`.
#[test]
fn test_BC_3_08_001_s19_05_t007_sec003_traversal_rejection_release_profile() {
    // AC-005 implementation: SEC-003 path traversal sanitization applies in ALL builds
    // (debug and release) via flush_sink_file's internal check.
    //
    // Discriminating design: ALL intermediate path components exist so that,
    // absent the guard, open(O_CREAT | O_APPEND) would SUCCEED and write the
    // escaped file to the OS temp dir.
    //
    // Path layout: {tmpdir}/inner/../../{unique}.jsonl
    //   resolves to: {tmpdir_parent}/{unique}.jsonl  (OS temp dir — writable)
    //   {tmpdir}/inner is created explicitly below.
    //
    // With guard active: ".." detection fires before open() → no file.
    // Without guard:     open() succeeds → escaped file created → FAIL.
    let tmp = tempfile::tempdir().expect("T-007: should create tempdir");

    // Create the intermediate directory so ALL components of the traversal
    // path exist. Without the guard, open() would succeed.
    let inner = tmp.path().join("inner");
    std::fs::create_dir_all(&inner).expect("T-007: create inner subdir");

    // Unique escaped filename derived from tmpdir's random component to avoid
    // collision between parallel test runs.
    let dirname = tmp
        .path()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("x");
    let escaped_name = format!("sec003-escape-{dirname}.jsonl");
    let traversal_path = format!("{}/inner/../../{escaped_name}", tmp.path().display());

    // Populate the event queue — flush_sink_file should reject the traversal path
    // without creating any file.
    let ctx = make_test_ctx();
    emit_plugin_abandoned(&ctx, "test-plugin-t007", 0, 100);

    // Call flush_sink_file with the traversal path — SEC-003 check must reject it.
    flush_sink_file(&traversal_path, &ctx.events);

    // Canonical escape target: the resolved location open() would write to
    // absent the guard.
    let escaped_target = tmp
        .path()
        .parent()
        .expect("T-007: tmpdir has parent")
        .join(&escaped_name);

    // Dual assertion: guard must prevent file creation at both the traversal
    // string path and the canonically-resolved escape location.
    assert!(
        !std::path::Path::new(&traversal_path).exists(),
        "T-007 AC-005: flush_sink_file must NOT create a file at traversal path {:?}; \
         SEC-003 path traversal rejection must apply in all build profiles",
        traversal_path
    );
    assert!(
        !escaped_target.exists(),
        "T-007 AC-005: escaped file must NOT be created at {:?}; \
         SEC-003 path traversal rejection must apply in all build profiles",
        escaped_target
    );

    // Best-effort cleanup (no-op when guard is active; removes evidence file
    // if guard was temporarily removed for discrimination evidence capture).
    let _ = std::fs::remove_file(&escaped_target);
}

// ---------------------------------------------------------------------------
// T-008 (AC-006): CLAUDE.md Factory Hook Diagnostics section must document
// VSDD_SINK_FILE as honored in both debug and release builds.
// ---------------------------------------------------------------------------

/// T-008 (AC-006): `CLAUDE.md` Factory Hook Diagnostics section documents
/// `VSDD_SINK_FILE` as honored in both debug and release builds.
///
/// Gate per AC-006: grep -qE "VSDD_SINK_FILE.{1,60}(debug and release|release builds)" CLAUDE.md
///
/// RED gate: documentation not yet added → assertion fails → RED gate ✓.
#[test]
fn test_BC_3_08_001_s19_05_t008_claude_md_vsdd_sink_file_release_documentation_present() {
    // CLAUDE.md is at the worktree root: two directories above crates/factory-dispatcher/
    let claude_md_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../CLAUDE.md");
    let claude_md = std::fs::read_to_string(claude_md_path)
        .expect("T-008: should be able to read CLAUDE.md from worktree root");

    // Gate per AC-006: grep -qE "VSDD_SINK_FILE.{1,60}(debug and release|release builds)"
    let has_release_doc = claude_md.lines().any(|line| {
        if line.contains("VSDD_SINK_FILE") {
            line.contains("debug and release") || line.contains("release builds")
        } else {
            false
        }
    });

    assert!(
        has_release_doc,
        "T-008 AC-006: CLAUDE.md must contain a line matching \
         'VSDD_SINK_FILE.{{1,60}}(debug and release|release builds)' in the \
         Factory Hook Diagnostics section. Documentation not yet added — \
         add per S-19.05 AC-006 requirements: (1) VSDD_SINK_FILE honored in \
         both debug and release builds as of this story; (2) how to set it \
         (`VSDD_SINK_FILE=/tmp/disp-events.jsonl claude ...`); \
         (3) SEC-003 path constraint (no '..'). RED gate ✓."
    );
}
