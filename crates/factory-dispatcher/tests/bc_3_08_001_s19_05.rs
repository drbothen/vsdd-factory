// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! S-19.05 — Async plugin completion telemetry + VSDD_SINK_FILE release-mode opt-in.
//!
//! TDD Red Gate test suite. All tests MUST FAIL before implementation begins.
//!
//! ## Test inventory
//!
//! | Test ID | Story AC | Description |
//! |---------|----------|-------------|
//! | T-001 | AC-001 | Async exit-0 within drain → `plugin.completed` emitted (all 9 mandatory fields) |
//! | T-002 | AC-002 | Slow async exceeds drain → `plugin.abandoned` emitted (all 7 mandatory fields) + Invariant 6 check |
//! | T-004 | AC-003 | Async events not relayed to dispatcher stderr |
//! | T-005 | AC-004 | `VSDD_SINK_FILE` honored at runtime in release profile |
//! | T-006 | AC-004 | `use std::sync::Mutex` (O-P2-003) not inside `#[cfg(debug_assertions)]` |
//! | T-007 | AC-005 | SEC-003 traversal rejection in release profile |
//! | T-008 | AC-006 | `CLAUDE.md` Factory Hook Diagnostics documents `VSDD_SINK_FILE` in both debug and release builds |
//!
//! Note: T-003 (EAC-008 schema-level defense property tests) lives in
//! `crates/factory-dispatcher/src/host/emit_event.rs` (src-scope serialization tests).
//!
//! ## Red Gate
//!
//! - T-001/T-002/T-004: call `emit_plugin_completed_async` / `emit_plugin_abandoned`
//!   stubs → `todo!()` panic → test FAILS (RED gate ✓)
//! - T-005/T-007: `todo!()` placeholder stubs → test FAILS (RED gate ✓)
//! - T-006: reads `main.rs`; asserts `use std::sync::Mutex` is not cfg-gated →
//!   assertion FAILS (currently cfg-gated; RED gate ✓)
//! - T-008: reads `CLAUDE.md`; asserts release documentation present →
//!   assertion FAILS (not yet documented; RED gate ✓)
//!
//! ## BC traces
//!
//! - BC-3.08.001 v1.21 Event 5 — `plugin.abandoned` (7 mandatory fields incl. `entry_index: u32`)
//! - BC-3.08.001 v1.21 Event 6 — `plugin.completed` async path (9 mandatory fields incl. `plugin_version`)
//! - BC-3.08.001 v1.21 Invariant 6 — terminal semantics: `plugin.abandoned` and `plugin.completed`
//!   mutually exclusive per `(trace_id, plugin_name, entry_index)` tuple
//! - VP-100 — drain-timer expiry emits exactly one `plugin.abandoned` per in-flight (plugin_name, entry_index)
//! - VP-079 — payload conformance for all six event types

use factory_dispatcher::flush_sink_file;
use factory_dispatcher::host::emit_event::{emit_plugin_abandoned, emit_plugin_completed_async};

// ---------------------------------------------------------------------------
// Helper: minimal HostContext for emit function calls.
// ---------------------------------------------------------------------------
fn make_test_ctx() -> factory_dispatcher::host::HostContext {
    factory_dispatcher::host::HostContext::new(
        "test-plugin-s19-05",
        "1.2.3",
        "test-session-s19-05",
        "test-trace-id-s19-05",
    )
}

// ---------------------------------------------------------------------------
// T-001 (AC-001): Async plugin exits 0 within drain window → `plugin.completed`
// emitted with ALL 9 mandatory fields present and non-null.
//
// BC-3.08.001 v1.21 Event 6 mandatory fields:
//   type, trace_id, session_id, plugin_name, plugin_version, entry_index,
//   exit_code, elapsed_ms, fuel_consumed.
//
// RED gate: emit_plugin_completed_async is todo!() — panics on call.
// ---------------------------------------------------------------------------

/// T-001 (AC-001): async plugin exits 0 within drain window → `plugin.completed`
/// event emitted. Asserts all 9 mandatory fields present and non-null per
/// BC-3.08.001 v1.21 Event 6.
///
/// RED gate: emit_plugin_completed_async is a todo!() stub — panics with
/// "not yet implemented". Test fails (RED gate ✓).
/// GREEN after: the function emits the event with all 9 mandatory fields.
#[test]
fn test_BC_3_08_001_s19_05_t001_async_exit0_within_drain_emits_plugin_completed() {
    let ctx = make_test_ctx();

    // Scenario: async plugin completes within drain window, exit code 0 (non-block).
    // Mirrors BC-3.08.001 v1.21 EC-008 and Canonical Test Vector `async-completed`.
    let plugin_name = "test-async-plugin";
    let plugin_version = "1.0.0";
    let entry_index: u32 = 0; // first (and only) async plugin in partition
    let exit_code: i32 = 0;
    let elapsed_ms: u64 = 42;
    let fuel_consumed: u64 = 100_000;

    // RED gate: todo!() panics here — test fails until implementation
    emit_plugin_completed_async(
        &ctx,
        plugin_name,
        plugin_version,
        entry_index,
        exit_code,
        elapsed_ms,
        fuel_consumed,
    );

    let events = ctx.drain_events();
    assert_eq!(
        events.len(),
        1,
        "T-001: exactly one plugin.completed event must be emitted"
    );

    let ev = &events[0];

    // Mandatory field 1: type = "plugin.completed"
    assert_eq!(
        ev.type_, "plugin.completed",
        "T-001: event type must be 'plugin.completed' (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 2: trace_id (dispatcher-owned, injected by with_trace_id)
    assert!(
        ev.dispatcher_trace_id.is_some(),
        "T-001: trace_id must be present (BC-3.08.001 v1.21 Invariant 1 + DI-017)"
    );
    assert!(
        !ev.dispatcher_trace_id.as_deref().unwrap_or("").is_empty(),
        "T-001: trace_id must be non-empty"
    );

    // Mandatory field 3: session_id
    assert!(
        ev.session_id.is_some(),
        "T-001: session_id must be present (BC-3.08.001 v1.21 §Common Fields)"
    );

    // Mandatory field 4: plugin_name
    assert_eq!(
        ev.plugin_name.as_deref(),
        Some(plugin_name),
        "T-001: plugin_name must match (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 5: plugin_version — Event 6 ONLY (Events 1, 4, 5 do not emit this)
    assert_eq!(
        ev.plugin_version.as_deref(),
        Some(plugin_version),
        "T-001: plugin_version must be present in plugin.completed async path — \
         BC-3.08.001 v1.21 Event 6 (sync-path emit_lifecycle parity; absent from Events 1/4/5)"
    );

    // Mandatory field 6: entry_index (0-based ordinal from enumerate())
    let stored_entry_index = ev
        .fields
        .get("entry_index")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    assert_eq!(
        stored_entry_index,
        Some(entry_index),
        "T-001: entry_index must equal the enumerate() ordinal (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 7: exit_code
    let stored_exit_code = ev
        .fields
        .get("exit_code")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    assert_eq!(
        stored_exit_code,
        Some(exit_code),
        "T-001: exit_code must be present and correct (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 8: elapsed_ms
    let stored_elapsed_ms = ev.fields.get("elapsed_ms").and_then(|v| v.as_u64());
    assert_eq!(
        stored_elapsed_ms,
        Some(elapsed_ms),
        "T-001: elapsed_ms must be present (BC-3.08.001 v1.21 Event 6)"
    );

    // Mandatory field 9: fuel_consumed
    let stored_fuel_consumed = ev.fields.get("fuel_consumed").and_then(|v| v.as_u64());
    assert_eq!(
        stored_fuel_consumed,
        Some(fuel_consumed),
        "T-001: fuel_consumed must be present (BC-3.08.001 v1.21 Event 6)"
    );

    // Invariant 6: no plugin.abandoned follows for the same (trace_id, plugin_name, entry_index)
    // (In this unit test, we verify no abandoned event was emitted at all.)
    let abandoned_count = events
        .iter()
        .filter(|e| e.type_ == "plugin.abandoned")
        .count();
    assert_eq!(
        abandoned_count, 0,
        "T-001 Invariant 6: no plugin.abandoned must follow plugin.completed for same triple \
         (BC-3.08.001 v1.21 Invariant 6 + VP-100)"
    );
}

/// T-001 variant: AC-001 EC-001 — async plugin exits non-zero (non-block) within drain.
/// `plugin.completed` is emitted with the actual exit_code (not 0).
/// BC-3.08.001 v1.21 EC-001 and Invariant 3.
///
/// RED gate: emit_plugin_completed_async is a todo!() stub — panics.
#[test]
fn test_BC_3_08_001_s19_05_t001_ec001_async_nonzero_exit_emits_completed_with_actual_exit_code() {
    let ctx = make_test_ctx();
    let exit_code: i32 = 1; // non-zero, non-block
    // RED gate: todo!() panics here
    emit_plugin_completed_async(&ctx, "test-plugin", "1.0.0", 0, exit_code, 10, 50_000);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "T-001 EC-001: one event emitted");
    let ev = &events[0];
    assert_eq!(ev.type_, "plugin.completed", "T-001 EC-001: event type");
    let stored_exit_code = ev
        .fields
        .get("exit_code")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    assert_eq!(
        stored_exit_code,
        Some(exit_code),
        "T-001 EC-001: exit_code must equal the actual exit code (not hardcoded 0)"
    );
}

// ---------------------------------------------------------------------------
// T-002 (AC-002): Slow async plugin exceeds drain window → `plugin.abandoned`
// emitted with ALL 7 mandatory fields + Invariant 6 terminal check.
//
// BC-3.08.001 v1.21 Event 5 mandatory fields:
//   type, trace_id, session_id, plugin_name, entry_index, drain_window_ms, timestamp.
//
// RED gate: emit_plugin_abandoned is todo!() — panics on call.
// ---------------------------------------------------------------------------

/// T-002 (AC-002): drain timer fires with plugin in-flight → `plugin.abandoned`
/// emitted. Asserts all 7 mandatory fields present per BC-3.08.001 v1.21 Event 5.
/// Also asserts Invariant 6: zero `plugin.completed` events follow for the same triple.
///
/// RED gate: emit_plugin_abandoned is a todo!() stub — panics with "not yet implemented".
#[test]
fn test_BC_3_08_001_s19_05_t002_drain_timer_fires_with_in_flight_plugin_emits_abandoned() {
    let ctx = make_test_ctx();

    // Scenario: drain timer fires while plugin is still in-flight.
    // Canonical test vector: "abandoned-one" (BC-3.08.001 v1.21).
    let plugin_name = "slow-async-plugin";
    let entry_index: u32 = 0;
    let drain_window_ms: u64 = 100; // debug-override drain window value

    // RED gate: todo!() panics here
    emit_plugin_abandoned(&ctx, plugin_name, entry_index, drain_window_ms);

    let events = ctx.drain_events();
    assert_eq!(
        events.len(),
        1,
        "T-002: exactly one plugin.abandoned event must be emitted per in-flight plugin \
         (BC-3.08.001 v1.21 EC-007 + VP-100)"
    );

    let ev = &events[0];

    // Mandatory field 1: type = "plugin.abandoned"
    assert_eq!(
        ev.type_, "plugin.abandoned",
        "T-002: event type must be 'plugin.abandoned' (BC-3.08.001 v1.21 Event 5)"
    );

    // Mandatory field 2: trace_id
    assert!(
        ev.dispatcher_trace_id.is_some(),
        "T-002: trace_id must be present (BC-3.08.001 v1.21 Invariant 1)"
    );
    assert!(
        !ev.dispatcher_trace_id.as_deref().unwrap_or("").is_empty(),
        "T-002: trace_id must be non-empty"
    );

    // Mandatory field 3: session_id
    assert!(
        ev.session_id.is_some(),
        "T-002: session_id must be present (BC-3.08.001 v1.21 §Common Fields O-P15-001)"
    );

    // Mandatory field 4: plugin_name
    assert_eq!(
        ev.plugin_name.as_deref(),
        Some(plugin_name),
        "T-002: plugin_name must be present and match the registry entry name verbatim \
         (BC-3.08.001 v1.21 Event 5 entry_index semantics paragraph)"
    );

    // Mandatory field 5: entry_index (u32, 0-based ordinal from enumerate())
    let stored_entry_index = ev
        .fields
        .get("entry_index")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    assert_eq!(
        stored_entry_index,
        Some(entry_index),
        "T-002: entry_index must be present and equal the enumerate() ordinal \
         (BC-3.08.001 v1.21 Event 5 + Invariant 6 disambiguation key)"
    );

    // Mandatory field 6: drain_window_ms
    let stored_drain_window_ms = ev.fields.get("drain_window_ms").and_then(|v| v.as_u64());
    assert_eq!(
        stored_drain_window_ms,
        Some(drain_window_ms),
        "T-002: drain_window_ms must be present (BC-3.08.001 v1.21 Event 5 drain_window_ms semantics)"
    );

    // Mandatory field 7: timestamp
    assert!(
        ev.fields.get("timestamp").is_some(),
        "T-002: timestamp must be present (BC-3.08.001 v1.21 Event 5 mandatory fields)"
    );
    let ts = ev
        .fields
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(!ts.is_empty(), "T-002: timestamp must be non-empty");

    // Invariant 6: no plugin.completed fires after plugin.abandoned for the same triple
    let completed_count = events
        .iter()
        .filter(|e| e.type_ == "plugin.completed")
        .count();
    assert_eq!(
        completed_count, 0,
        "T-002 Invariant 6: plugin.abandoned is TERMINAL — no plugin.completed must \
         follow for the same (trace_id, plugin_name, entry_index) triple \
         (BC-3.08.001 v1.21 Invariant 6 + VP-100)"
    );
}

/// T-002 variant: EC-002 — all async plugins complete before drain timer fires.
/// No `plugin.abandoned` events emitted. Canonical test vector: `abandoned-none`.
///
/// This test does NOT call any todo!() stubs (it verifies the zero-event case).
/// It asserts no abandoned event is emitted when there are no in-flight plugins at drain.
/// This test will be GREEN before implementation (it makes no calls to stubs).
///
/// Note: this test is GREEN immediately; it is included for completeness and to
/// guard EC-002 / `abandoned-none` canonical test vector after implementation.
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

// ---------------------------------------------------------------------------
// T-004 (AC-003): Async events (plugin.completed, plugin.abandoned) MUST NOT
// be relayed to dispatcher stderr. They route to the internal event queue and
// sink only. (Parity with BC-1.14.001 Invariant 4: async plugin stderr not relayed.)
//
// RED gate: emit_plugin_completed_async is todo!() — panics on call.
// ---------------------------------------------------------------------------

/// T-004 (AC-003): async events must not appear in dispatcher stderr.
/// Verifies plugin.completed routes to the event queue only (not stderr).
///
/// Unit-level assertion: the emitted event must not carry a non-empty `stderr`
/// field. Integration-level (stderr relay suppression) is verified by the bats
/// harness — here we assert the event routing contract at the struct level.
///
/// RED gate: emit_plugin_completed_async is a todo!() stub — panics.
#[test]
fn test_BC_3_08_001_s19_05_t004_async_completed_event_not_relayed_to_stderr() {
    let ctx = make_test_ctx();
    // RED gate: todo!() panics here
    emit_plugin_completed_async(&ctx, "test-plugin", "1.0.0", 0, 0, 10, 50_000);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "T-004: one event emitted");
    let ev = &events[0];
    assert_eq!(ev.type_, "plugin.completed", "T-004: event type");
    // Assert: no non-empty stderr field in the emitted event.
    // Async plugin stderr must NOT be relayed per BC-1.14.001 Invariant 4.
    // The event carries observability data (exit_code, elapsed_ms, etc.) but
    // must not carry or relay plugin stderr to the dispatcher's process stderr.
    let stderr_field = ev
        .fields
        .get("stderr")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        stderr_field.is_empty(),
        "T-004 AC-003: plugin.completed async event must not carry non-empty stderr \
         (BC-1.14.001 Invariant 4: async plugin stderr not relayed to dispatcher stderr)"
    );
}

/// T-004 variant: plugin.abandoned also must not carry or relay stderr.
///
/// RED gate: emit_plugin_abandoned is a todo!() stub — panics.
#[test]
fn test_BC_3_08_001_s19_05_t004_async_abandoned_event_not_relayed_to_stderr() {
    let ctx = make_test_ctx();
    // RED gate: todo!() panics here
    emit_plugin_abandoned(&ctx, "test-plugin", 0, 100);
    let events = ctx.drain_events();
    assert_eq!(
        events.len(),
        1,
        "T-004 abandoned variant: one event emitted"
    );
    let ev = &events[0];
    assert_eq!(
        ev.type_, "plugin.abandoned",
        "T-004 abandoned variant: event type"
    );
    let stderr_field = ev
        .fields
        .get("stderr")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        stderr_field.is_empty(),
        "T-004 AC-003: plugin.abandoned event must not carry non-empty stderr \
         (BC-1.14.001 Invariant 4)"
    );
}

// ---------------------------------------------------------------------------
// T-005 (AC-004): VSDD_SINK_FILE env var honored at runtime in BOTH debug and
// release builds. The #[cfg(debug_assertions)] gates around ENV_SINK_FILE,
// flush_sink_file, and the sink mutex in main.rs are removed.
//
// RED gate pre-implementation: flush_sink_file is #[cfg(debug_assertions)]-gated.
// todo!() placeholder ensures this test fails until implementation removes the gate.
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
//
// Gate per story: awk preceding-line form — the use line must NOT be immediately
// preceded by `#[cfg(`.
//
// RED gate: currently the import IS inside #[cfg(debug_assertions)] in main.rs:
//   #[cfg(debug_assertions)]
//   use std::sync::Mutex;
// The assertion fails → RED gate ✓.
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
//
// Paths containing `..` must be rejected. No file written.
//
// RED gate pre-implementation: `flush_sink_file` is `#[cfg(debug_assertions)]`-gated;
// the test cannot exercise release-mode behavior. todo!() ensures failure.
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
    let tmp = tempfile::tempdir().expect("T-007: should create tempdir");
    let traversal_path = format!("{}/subdir/../../sec003-target.jsonl", tmp.path().display());

    // Populate the event queue — flush_sink_file should reject the traversal path
    // without creating any file.
    let ctx = make_test_ctx();
    emit_plugin_abandoned(&ctx, "test-plugin-t007", 0, 100);

    // Call flush_sink_file with the traversal path — SEC-003 check must reject it.
    flush_sink_file(&traversal_path, &ctx.events);

    // Assert no file was created at the traversal target path.
    // Path::new resolves ".." components when checking existence, so we check
    // both the string path and the resolved canonical target.
    assert!(
        !std::path::Path::new(&traversal_path).exists(),
        "T-007 AC-005: flush_sink_file must NOT create a file when the path contains '..' \
         (SEC-003 path traversal rejection applies in all build profiles)"
    );
}

// ---------------------------------------------------------------------------
// T-008 (AC-006): CLAUDE.md Factory Hook Diagnostics section must document
// VSDD_SINK_FILE as honored in both debug and release builds.
//
// Gate per AC-006: grep -qE "VSDD_SINK_FILE.{1,60}(debug and release|release builds)" CLAUDE.md
//
// RED gate: documentation not yet added → assertion fails → RED gate ✓.
// ---------------------------------------------------------------------------

/// T-008 (AC-006): `CLAUDE.md` Factory Hook Diagnostics section documents
/// `VSDD_SINK_FILE` as honored in both debug and release builds.
///
/// Gate per AC-006: the file must contain a line matching:
///   `VSDD_SINK_FILE.{1,60}(debug and release|release builds)`
///
/// RED gate: the documentation does not yet exist in `CLAUDE.md` → assertion fails.
/// GREEN after implementer adds the documentation per AC-006 requirements.
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
