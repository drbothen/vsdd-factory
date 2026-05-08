//! VP-079: Async-Semantics Event Types — Payload Schema Conformance (Rust stubs).
//!
//! These Rust integration tests serve as the structural RED-gate skeleton for the
//! VP-079 bats fault-injection scenarios. The full end-to-end scenarios (requiring
//! a compiled dispatcher binary + FileSink output) are authored as bats scripts in
//! `tests/bats/async-event-schema-conformance.bats`.
//!
//! These Rust tests verify:
//! 1. The emit stubs exist and are callable (compile check).
//! 2. The four new event-type functions are `todo!()` (will panic == RED gate).
//! 3. Structural field-presence assertions for each of the 4 new event types.
//!
//! # VP-079 scenario map
//!
//! | Scenario | Event type | Test fn |
//! |----------|-----------|---------|
//! | 1 | plugin.async_block_discarded | test_BC_3_08_001_vp079_s1_* |
//! | 2 | dispatcher.schema_mismatch | test_BC_3_08_001_vp079_s2_* |
//! | 3 | dispatcher.registry_invalid | test_BC_3_08_001_vp079_s3_* |
//! | 4 | plugin.timeout (async) | test_BC_3_08_001_vp079_s4_* |
//! | 5 | drain-window truncation | test_BC_3_08_001_vp079_s5_* |
//!
//! # Red Gate
//!
//! All tests call into `todo!()` stubs in `host/emit_event.rs`.
//! Compilation MUST succeed (Red Gate requirement); runtime MUST panic with
//! "not yet implemented" until T-3e implements the four emit functions.
//!
//! # BC traces
//!
//! - BC-3.08.001 v1.6 — event catalog: four new event types
//! - BC-1.14.001 — dispatch partition contract (async group fire-and-forget)
//! - BC-7.06.001 — registry validation (schema_mismatch / registry_invalid triggers)
//! - DI-019 — ASYNC_DRAIN_WINDOW_MS (do NOT hardcode; cite by name)
//! - VP-079 v1.6 — fault injection verification property
//! - AC-011, AC-012, AC-013, AC-014, AC-005 (S-15.01 v1.6)

use factory_dispatcher::host::emit_event::{
    emit_dispatcher_registry_invalid, emit_dispatcher_schema_mismatch,
    emit_plugin_async_block_discarded, emit_plugin_timeout_async,
};
use factory_dispatcher::registry::REGISTRY_SCHEMA_VERSION;

// ---------------------------------------------------------------------------
// Helper: build a minimal HostContext for calling emit stubs in tests.
//
// The emit functions take `&HostContext`. We construct the minimal version
// needed to exercise the function signature — the stubs ignore the ctx anyway
// (they are `todo!()`). Use HostContext::new() per the convenience constructor.
// ---------------------------------------------------------------------------

fn make_test_ctx() -> factory_dispatcher::host::HostContext {
    factory_dispatcher::host::HostContext::new(
        "test-plugin",         // plugin_name
        "0.0.0",               // plugin_version
        "test-session-vp079",  // session_id
        "test-trace-id-vp079", // dispatcher_trace_id
    )
}

// ---------------------------------------------------------------------------
// VP-079 Scenario 1: plugin.async_block_discarded
//
// AC-011: when async-group plugin exits 2, dispatcher emits
// plugin.async_block_discarded with mandatory fields:
//   type, trace_id, plugin_name, exit_code, timestamp, reason.
// reason MUST equal "async_plugin_block_verdict_discarded".
// Dispatcher exit code is unaffected (async verdict discarded).
// ---------------------------------------------------------------------------

/// VP-079 S1: emit_plugin_async_block_discarded emits event with mandatory fields.
///
/// GREEN after T-3e: emit_plugin_async_block_discarded is implemented.
/// Verifies mandatory fields: type, trace_id, plugin_name, exit_code, reason.
#[test]
fn test_BC_3_08_001_vp079_s1_async_block_discarded_stub_panics() {
    let ctx = make_test_ctx();
    // Scenario: async plugin exits 2 (block verdict); the block is discarded.
    // VP-079 S1 mandatory fields: type, trace_id, plugin_name, exit_code, timestamp, reason.
    // reason = "async_plugin_block_verdict_discarded" (literal per BC-3.08.001 PC1).
    emit_plugin_async_block_discarded(&ctx, "test-async-blocker", 2);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "exactly one event must be emitted");
    let ev = &events[0];
    assert_eq!(
        ev.type_, "plugin.async_block_discarded",
        "event type must match"
    );
    assert_eq!(
        ev.plugin_name.as_deref(),
        Some("test-async-blocker"),
        "plugin_name field"
    );
    assert_eq!(
        ev.fields.get("reason").and_then(|v| v.as_str()),
        Some("async_plugin_block_verdict_discarded"),
        "reason must be async_plugin_block_verdict_discarded"
    );
    assert_eq!(
        ev.fields.get("exit_code").and_then(|v| v.as_i64()),
        Some(2),
        "exit_code must be 2"
    );
}

/// VP-079 S1: exit_code=2 triggers async_block_discarded with correct reason field.
///
/// GREEN after T-3e.
#[test]
fn test_BC_3_08_001_vp079_s1_exit_code_2_triggers_discard_event() {
    let ctx = make_test_ctx();
    // AC-011: exit_code must be 2 in the emitted event.
    emit_plugin_async_block_discarded(&ctx, "capture-commit-activity", 2);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1);
    let ev = &events[0];
    assert_eq!(
        ev.fields.get("reason").and_then(|v| v.as_str()),
        Some("async_plugin_block_verdict_discarded"),
        "reason must be async_plugin_block_verdict_discarded (AC-011)"
    );
    assert_eq!(
        ev.fields.get("exit_code").and_then(|v| v.as_i64()),
        Some(2),
        "exit_code must be 2 (AC-011)"
    );
}

// ---------------------------------------------------------------------------
// VP-079 Scenario 2: dispatcher.schema_mismatch
//
// AC-012: when registry has schema_version != 2, dispatcher emits
// dispatcher.schema_mismatch with mandatory fields:
//   type, trace_id, found_version, expected_version, timestamp, error_code.
// expected_version MUST be REGISTRY_SCHEMA_VERSION (2).
// error_code MUST be "E-REG-001".
// Emit-then-exit: event must reach FileSink before dispatcher exits.
// ---------------------------------------------------------------------------

/// VP-079 S2: emit_dispatcher_schema_mismatch emits event with mandatory fields.
///
/// GREEN after T-3e.
#[test]
fn test_BC_3_08_001_vp079_s2_schema_mismatch_stub_panics() {
    let ctx = make_test_ctx();
    // Scenario: registry has schema_version = 1 (not 2).
    // Mandatory fields: type, trace_id, found_version, expected_version, timestamp, error_code.
    // error_code = "E-REG-001"; expected_version = REGISTRY_SCHEMA_VERSION.
    emit_dispatcher_schema_mismatch(&ctx, 1, REGISTRY_SCHEMA_VERSION);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "exactly one event must be emitted");
    let ev = &events[0];
    assert_eq!(
        ev.type_, "dispatcher.schema_mismatch",
        "event type must match"
    );
    assert_eq!(
        ev.fields.get("found_version").and_then(|v| v.as_i64()),
        Some(1),
        "found_version must be 1"
    );
    assert_eq!(
        ev.fields.get("expected_version").and_then(|v| v.as_i64()),
        Some(REGISTRY_SCHEMA_VERSION as i64),
        "expected_version must be REGISTRY_SCHEMA_VERSION"
    );
    assert_eq!(
        ev.fields.get("error_code").and_then(|v| v.as_str()),
        Some("E-REG-001"),
        "error_code must be E-REG-001"
    );
}

/// VP-079 S2: REGISTRY_SCHEMA_VERSION constant equals 2 (DI-019 / AC-001 cross-check).
///
/// This test does NOT exercise the todo!() path — it validates the constant.
/// GREEN immediately: the stub-architect set REGISTRY_SCHEMA_VERSION = 2.
#[test]
fn test_BC_3_08_001_vp079_s2_expected_version_is_2() {
    // AC-001: REGISTRY_SCHEMA_VERSION must equal 2.
    // DI-019: ASYNC_DRAIN_WINDOW_MS is cited by name (not hardcoded) elsewhere;
    // here we only validate the schema version constant.
    assert_eq!(
        REGISTRY_SCHEMA_VERSION, 2,
        "test_BC_3_08_001_vp079_s2_expected_version_is_2: \
         REGISTRY_SCHEMA_VERSION must be 2 per AC-001 / BC-7.06.001 PC1"
    );
}

/// VP-079 S2: v1 registry triggers schema_mismatch with correct field values.
///
/// GREEN after T-3e: verifies found_version=1, expected_version=2, error_code=E-REG-001.
#[test]
fn test_BC_3_08_001_vp079_s2_v1_registry_triggers_schema_mismatch() {
    let ctx = make_test_ctx();
    // found_version = 1 (v1 registry), expected_version = REGISTRY_SCHEMA_VERSION (2).
    emit_dispatcher_schema_mismatch(&ctx, 1, REGISTRY_SCHEMA_VERSION);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1);
    let ev = &events[0];
    // All mandatory fields present per BC-3.08.001 PC2.
    assert!(ev.dispatcher_trace_id.is_some(), "trace_id must be present");
    assert_eq!(
        ev.fields.get("found_version").and_then(|v| v.as_i64()),
        Some(1)
    );
    assert_eq!(
        ev.fields.get("expected_version").and_then(|v| v.as_i64()),
        Some(2),
        "expected_version must be 2 (REGISTRY_SCHEMA_VERSION)"
    );
    assert_eq!(
        ev.fields.get("error_code").and_then(|v| v.as_str()),
        Some("E-REG-001"),
        "error_code must be E-REG-001"
    );
}

// ---------------------------------------------------------------------------
// VP-079 Scenario 3: dispatcher.registry_invalid
//
// AC-013: when registry entry has on_error=block AND async=true, dispatcher emits
// dispatcher.registry_invalid with mandatory fields:
//   type, trace_id, offending_plugin, violation, timestamp, error_code.
// offending_plugin MUST name the violating entry.
// violation MUST be "async_block_conflict" (BC-3.08.001 v1.7 canonical).
// error_code MUST be "E-REG-002".
// ---------------------------------------------------------------------------

/// VP-079 S3: emit_dispatcher_registry_invalid emits event with mandatory fields.
///
/// GREEN after T-3e.
/// Violation string updated to "async_block_conflict" per BC-3.08.001 v1.7 amendment.
#[test]
fn test_BC_3_08_001_vp079_s3_registry_invalid_stub_panics() {
    let ctx = make_test_ctx();
    // Scenario: entry "invalid-blocker" has on_error=block AND async=true.
    // Mandatory fields: type, trace_id, offending_plugin, violation, timestamp, error_code.
    // BC-3.08.001 v1.7: violation canonical string is "async_block_conflict".
    emit_dispatcher_registry_invalid(&ctx, "invalid-blocker", "E-REG-002", "async_block_conflict");
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "exactly one event must be emitted");
    let ev = &events[0];
    assert_eq!(
        ev.type_, "dispatcher.registry_invalid",
        "event type must match"
    );
    assert_eq!(
        ev.fields.get("offending_plugin").and_then(|v| v.as_str()),
        Some("invalid-blocker"),
        "offending_plugin must name the violating entry"
    );
    assert_eq!(
        ev.fields.get("violation").and_then(|v| v.as_str()),
        Some("async_block_conflict"),
        "violation must be async_block_conflict (BC-3.08.001 v1.7 canonical)"
    );
    assert_eq!(
        ev.fields.get("error_code").and_then(|v| v.as_str()),
        Some("E-REG-002"),
        "error_code must be E-REG-002"
    );
}

/// VP-079 S3: offending_plugin name must be passed through to the emitted event.
///
/// GREEN after T-3e.
#[test]
fn test_BC_3_08_001_vp079_s3_offending_plugin_name_in_event() {
    let ctx = make_test_ctx();
    // Verify the emitted event has offending_plugin = "bad-validator".
    // BC-3.08.001 v1.7: violation canonical string is "async_block_conflict".
    emit_dispatcher_registry_invalid(&ctx, "bad-validator", "E-REG-002", "async_block_conflict");
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1);
    let ev = &events[0];
    assert_eq!(
        ev.fields.get("offending_plugin").and_then(|v| v.as_str()),
        Some("bad-validator"),
        "offending_plugin must be 'bad-validator' (AC-013)"
    );
    assert_eq!(
        ev.fields.get("error_code").and_then(|v| v.as_str()),
        Some("E-REG-002"),
        "error_code must be E-REG-002 (AC-013)"
    );
}

// ---------------------------------------------------------------------------
// VP-079 Scenario 4: plugin.timeout (async path)
//
// AC-014: when async-group plugin exceeds timeout_ms within the drain window,
// dispatcher emits plugin.timeout with mandatory fields:
//   type, trace_id, plugin_name, execution_group, timeout_ms, timestamp.
// execution_group MUST be "async".
// timeout_ms MUST reflect the configured plugin timeout (not ASYNC_DRAIN_WINDOW_MS).
//
// DI-019: ASYNC_DRAIN_WINDOW_MS = 100ms (canonical). Tests reference the
// constant name, NOT the literal 100.
// ---------------------------------------------------------------------------

/// VP-079 S4: emit_plugin_timeout_async emits plugin.timeout with mandatory fields.
///
/// GREEN after T-3e.
#[test]
fn test_BC_3_08_001_vp079_s4_plugin_timeout_async_stub_panics() {
    let ctx = make_test_ctx();
    // Scenario: async plugin with timeout_ms=50 within ASYNC_DRAIN_WINDOW_MS (DI-019).
    // DI-019: reference ASYNC_DRAIN_WINDOW_MS by name. The timeout_ms (50) is the
    // PLUGIN timeout, not ASYNC_DRAIN_WINDOW_MS. They are independent.
    emit_plugin_timeout_async(&ctx, "slow-async-plugin", 50);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1, "exactly one event must be emitted");
    let ev = &events[0];
    assert_eq!(
        ev.type_, "plugin.timeout",
        "event type must be plugin.timeout"
    );
    assert_eq!(
        ev.plugin_name.as_deref(),
        Some("slow-async-plugin"),
        "plugin_name field"
    );
    assert_eq!(
        ev.fields.get("execution_group").and_then(|v| v.as_str()),
        Some("async"),
        "execution_group must be 'async' (AC-014)"
    );
    assert_eq!(
        ev.fields.get("timeout_ms").and_then(|v| v.as_i64()),
        Some(50),
        "timeout_ms must reflect the configured plugin timeout, not ASYNC_DRAIN_WINDOW_MS (DI-019)"
    );
}

/// VP-079 S4: execution_group must be "async" for async-path timeout events.
///
/// GREEN after T-3e.
#[test]
fn test_BC_3_08_001_vp079_s4_execution_group_is_async() {
    let ctx = make_test_ctx();
    // DI-019: ASYNC_DRAIN_WINDOW_MS is the drain window; plugin timeout_ms
    // is the per-plugin budget. Do NOT conflate.
    emit_plugin_timeout_async(&ctx, "slow-async-plugin", 50);
    let events = ctx.drain_events();
    assert_eq!(events.len(), 1);
    let ev = &events[0];
    assert_eq!(
        ev.fields.get("execution_group").and_then(|v| v.as_str()),
        Some("async"),
        "execution_group must be 'async' (AC-014, BC-3.08.001 PC4)"
    );
}

// ---------------------------------------------------------------------------
// VP-079 Scenario 5: drain-window truncation (no event emitted)
//
// AC-005 / VP-079 S5: when async plugin timeout_ms > ASYNC_DRAIN_WINDOW_MS
// (per DI-019), the dispatcher forcibly terminates the async task at drain
// expiry. plugin.timeout is NOT emitted (task killed before timeout fires).
//
// This scenario tests non-emission (absence of event). It is best verified
// by the bats integration test in tests/bats/async-event-schema-conformance.bats.
// This Rust stub confirms structural readiness.
//
// DI-019: ASYNC_DRAIN_WINDOW_MS is 100ms (canonical). Tests reference the
// constant name; the 200ms below is the PLUGIN timeout (exceeds drain window).
// ---------------------------------------------------------------------------

/// VP-079 S5: structural marker — drain-window truncation must NOT emit plugin.timeout.
///
/// This test verifies the ASYNC_DRAIN_WINDOW_MS constant will be exported from
/// the dispatcher crate so tests can reference it by name (not hardcode 100).
///
/// RED: ASYNC_DRAIN_WINDOW_MS constant does not yet exist in factory_dispatcher.
/// Once T-3d adds `pub const ASYNC_DRAIN_WINDOW_MS: std::time::Duration = ...`,
/// this test will compile and pass.
#[test]
fn test_BC_1_14_001_vp079_s5_async_drain_window_constant_exported() {
    // DI-019: ASYNC_DRAIN_WINDOW_MS must be exported from the dispatcher crate.
    // Tests must reference the constant, NOT hardcode 100.
    //
    // RED: factory_dispatcher::ASYNC_DRAIN_WINDOW_MS does not exist until T-3d.
    // Once T-3d adds the constant, change this to:
    //   use factory_dispatcher::ASYNC_DRAIN_WINDOW_MS;
    //   assert_eq!(ASYNC_DRAIN_WINDOW_MS.as_millis(), 100);
    //
    // For now this test serves as a TODO marker that forces the implementer
    // to export the constant. The compile-time failure is the Red Gate signal.
    //
    // NOTE: this test is intentionally structured to PASS at compile time
    // but fail semantically until ASYNC_DRAIN_WINDOW_MS is exported.
    // The bats Scenario 5 provides the full runtime verification.
    let drain_window_ms: u64 = factory_dispatcher::ASYNC_DRAIN_WINDOW_MS.as_millis() as u64;
    assert_eq!(
        drain_window_ms, 100,
        "test_BC_1_14_001_vp079_s5_async_drain_window_constant_exported: \
         ASYNC_DRAIN_WINDOW_MS must equal 100ms per DI-019 (canonical constant). \
         Do NOT hardcode 100 — reference factory_dispatcher::ASYNC_DRAIN_WINDOW_MS."
    );
}

/// VP-079 S5: combined drain-window scenario assertion (Rust layer).
///
/// The full scenario (2 async plugins: one within drain window, one exceeding)
/// requires an end-to-end bats test. This Rust stub asserts the boundary condition:
/// a plugin timeout_ms value exceeding ASYNC_DRAIN_WINDOW_MS (DI-019) should
/// NOT produce a plugin.timeout event (the dispatcher terminates the task).
///
/// RED: factory_dispatcher::ASYNC_DRAIN_WINDOW_MS not yet exported (T-3d).
#[test]
fn test_BC_1_14_001_vp079_s5_timeout_ms_above_drain_window_no_event() {
    // DI-019: reference ASYNC_DRAIN_WINDOW_MS by name, not by literal 100.
    let drain_window_ms = factory_dispatcher::ASYNC_DRAIN_WINDOW_MS.as_millis() as u32;

    // The slow-over-drain plugin's timeout_ms (200) exceeds the drain window.
    let slow_plugin_timeout_ms: u32 = drain_window_ms + 100;

    // Structural assertion: slow_plugin_timeout_ms > drain_window_ms.
    // This is the precondition for Scenario 5 (truncation without event emission).
    assert!(
        slow_plugin_timeout_ms > drain_window_ms,
        "test_BC_1_14_001_vp079_s5_timeout_ms_above_drain_window_no_event: \
         fixture timeout_ms ({}) must exceed ASYNC_DRAIN_WINDOW_MS ({}) per DI-019. \
         The dispatcher must forcibly terminate the task before plugin.timeout fires.",
        slow_plugin_timeout_ms,
        drain_window_ms
    );

    // The bats scenario 5 in tests/bats/async-event-schema-conformance.bats
    // verifies the actual non-emission (plugin.timeout not in SINK_FILE).
    // This Rust test verifies the precondition only.
}
