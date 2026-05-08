//! AC-016: Sync-group p95 latency canary — must be ≤ 500ms.
//!
//! This test measures the p95 latency of `sync_group` dispatch across N=100
//! invocations using a representative fixture set and asserts p95 ≤ 500ms.
//!
//! # Usage
//!
//! This test is marked `#[ignore]` because it requires:
//! 1. A compiled factory-dispatcher binary (release mode for representative timings).
//! 2. A complete plugin set (at least one real WASM plugin in sync_group).
//!
//! Run with:
//!
//! ```sh
//! cargo test --release -p factory-dispatcher -- --ignored latency_canary
//! ```
//!
//! Timing evidence is stored at `docs/demo-evidence/S-15.01/latency-canary.md`
//! (AC-016 / AC-017).
//!
//! # Red Gate
//!
//! RED until T-3b + T-3c: `partition_plugins()` and the sync/async dispatch loop
//! are `todo!()` — any attempt to exercise the dispatch path panics.
//!
//! After implementation: this test must pass with p95 ≤ 500ms. If p95 > 500ms,
//! the misclassification audit in T-3h must identify and flip additional heavy
//! plugins to `async = true` before merge (AC-016).
//!
//! # DI-019
//!
//! ASYNC_DRAIN_WINDOW_MS (DI-019) contributes to total wall-clock latency:
//! `latency ≤ max(sync_plugin_durations_within_slowest_tier) + ASYNC_DRAIN_WINDOW_MS`.
//! The 500ms budget accounts for typical sync_group execution across all registered
//! blocking plugins plus the drain window overhead.
//!
//! # BC traces
//!
//! - AC-016 (S-15.01 v1.6): p95 ≤ 500ms assertion
//! - BC-1.14.001 postcondition 2: sync_group execution + verdict aggregation
//! - DI-019: ASYNC_DRAIN_WINDOW_MS contributes to total latency bound

use std::time::{Duration, Instant};

/// P95_LATENCY_BUDGET_MS: maximum acceptable p95 sync_group latency (AC-016).
///
/// This is NOT DI-019. DI-019 is ASYNC_DRAIN_WINDOW_MS (drain window after
/// sync_group completes). This budget covers the entire dispatch call including
/// sync_group execution and drain window overhead.
const P95_LATENCY_BUDGET_MS: u64 = 500;

/// CANARY_ITERATIONS: number of dispatch invocations for p95 measurement.
const CANARY_ITERATIONS: usize = 100;

/// P95_INDEX: index into sorted latency vec for the 95th percentile.
///
/// 95th percentile of 100 samples = index 94 (0-indexed).
const P95_INDEX: usize = 94; // floor(0.95 * 100) - 1

/// AC-016: Sync-group p95 latency canary.
///
/// Measures dispatch latency across CANARY_ITERATIONS invocations and asserts
/// that the 95th percentile is ≤ P95_LATENCY_BUDGET_MS (500ms).
///
/// # Why #[ignore]
///
/// Requires a release build and a populated plugin set. Run explicitly:
/// `cargo test --release -- --ignored test_BC_1_14_001_ac016_sync_group_p95_latency`
///
/// # Red Gate
///
/// RED until T-3b (partition_plugins) and T-3c (dispatch loop) are implemented.
/// Will also be RED until T-3h classifies telemetry plugins as async=true
/// (without this, all plugins run in sync_group and p95 may exceed 500ms).
#[test]
#[ignore = "latency canary: requires --release build and populated plugin set; run with --ignored"]
fn test_BC_1_14_001_ac016_sync_group_p95_latency() {
    // DI-019: reference ASYNC_DRAIN_WINDOW_MS by name for the total latency bound.
    // Do NOT hardcode 100ms. The 500ms budget here covers sync_group + drain window.
    let _drain_window = factory_dispatcher::ASYNC_DRAIN_WINDOW_MS;

    let mut latencies: Vec<Duration> = Vec::with_capacity(CANARY_ITERATIONS);

    // Fixture: use the live registry for a realistic plugin set.
    // After T-3h: 9 telemetry plugins are async=true; sync_group contains
    // only blocking validators, reducing latency.
    // CARGO_MANIFEST_DIR points to crates/factory-dispatcher; walk two levels
    // up to the workspace root (precedent: loads_legacy_registry.rs).
    let registry_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root")
        .join("plugins/vsdd-factory/hooks-registry.toml");
    let registry = factory_dispatcher::registry::Registry::load(registry_path.as_path())
        .expect("latency canary: Registry::load failed — ensure hooks-registry.toml exists with schema_version=2");

    // Minimal event envelope for PostToolUse (representative workload).
    let payload_json = r#"{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"latency-canary-001","tool_input":{}}"#;
    let payload: factory_dispatcher::payload::HookPayload =
        serde_json::from_str(payload_json).expect("payload must parse");

    // Partition the registry to measure sync_group latency only.
    // RED: partition_plugins is todo!() until T-3b.
    let matched = factory_dispatcher::routing::match_plugins(&registry, &payload);
    // matched is Vec<&RegistryEntry>; clone to owned for partition.
    let matched_owned: Vec<factory_dispatcher::registry::RegistryEntry> =
        matched.into_iter().cloned().collect();
    let _partition = factory_dispatcher::partition::partition_plugins(&matched_owned);

    // Measurement loop.
    // After T-3b + T-3c: replace with actual dispatch call and measure wall clock.
    // RED: the loop below is a placeholder; dispatch_sync_group does not yet exist.
    for _ in 0..CANARY_ITERATIONS {
        let start = Instant::now();

        // TODO (T-3c): replace with actual sync_group dispatch call:
        //   dispatch_sync_group(&partition.sync_group, &payload, ...).await;
        //
        // For now, simulate a dispatch with a no-op to establish the harness.
        // This will be replaced by the implementer in T-3c.
        //
        // RED: this placeholder makes the test compile but measures zero latency.
        // The actual latency measurement requires T-3c implementation.
        let _ = std::hint::black_box(&registry);

        latencies.push(start.elapsed());
    }

    // Sort for percentile calculation.
    latencies.sort_unstable();

    let p95 = latencies[P95_INDEX];
    let p95_ms = p95.as_millis() as u64;

    // Record evidence to stdout (captured in CI logs).
    println!(
        "latency_canary: N={} iterations, p50={:?}, p95={:?}, p99={:?}",
        CANARY_ITERATIONS, latencies[49], p95, latencies[98],
    );

    // The primary assertion: p95 ≤ 500ms.
    assert!(
        p95_ms <= P95_LATENCY_BUDGET_MS,
        "test_BC_1_14_001_ac016_sync_group_p95_latency: \
         AC-016 FAIL — sync_group p95 latency is {}ms, budget is {}ms. \
         Run T-3h misclassification audit to flip heavy sync plugins to async=true \
         (DI-019 ASYNC_DRAIN_WINDOW_MS contributes to total latency bound).",
        p95_ms,
        P95_LATENCY_BUDGET_MS
    );

    println!(
        "latency_canary: PASS — p95={}ms ≤ budget={}ms",
        p95_ms, P95_LATENCY_BUDGET_MS
    );
}

/// AC-016 structural check: the latency constant is within a reasonable range.
///
/// GREEN: this test does not exercise todo!() paths.
/// Verifies the budget constant itself is sane.
#[test]
fn test_BC_1_14_001_ac016_latency_budget_constant_is_500ms() {
    // The 500ms budget covers sync_group execution + ASYNC_DRAIN_WINDOW_MS (DI-019).
    // This is not a tight bound — it is a regression guard for gross misclassification.
    assert_eq!(
        P95_LATENCY_BUDGET_MS, 500,
        "test_BC_1_14_001_ac016_latency_budget_constant_is_500ms: \
         P95 latency budget must be 500ms per AC-016 (S-15.01 v1.6)"
    );
}

/// AC-016 structural check: CANARY_ITERATIONS is 100 samples (sufficient for p95).
///
/// GREEN: verifies the test design is correct.
#[test]
fn test_BC_1_14_001_ac016_canary_sample_size_is_100() {
    // 100 samples required for meaningful p95 calculation (AC-016).
    // P95_INDEX = floor(0.95 * 100) - 1 = 94 (0-indexed).
    assert_eq!(CANARY_ITERATIONS, 100);
    assert_eq!(P95_INDEX, 94);
}
