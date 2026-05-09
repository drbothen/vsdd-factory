//! AC-016: Sync-group p95 latency canary — must be ≤ 1500ms.
//!
//! This test measures the p95 latency of `sync_group` dispatch across N=100
//! invocations using a representative fixture set and asserts p95 ≤ 1500ms.
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
//! After implementation: this test must pass with p95 ≤ 1500ms. If p95 > 1500ms,
//! the misclassification audit in T-3h must identify and flip additional heavy
//! plugins to `async = true` before merge (AC-016).
//!
//! # DI-019
//!
//! ASYNC_DRAIN_WINDOW_MS (DI-019) contributes to total wall-clock latency:
//! `latency ≤ max(sync_plugin_durations_within_slowest_tier) + ASYNC_DRAIN_WINDOW_MS`.
//! The 1500ms budget accounts for typical sync_group execution across all registered
//! blocking plugins plus the drain window overhead (Class A — cold-start dispatch; ADR-020).
//!
//! # BC traces
//!
//! - AC-016 (S-15.01 v1.6): p95 ≤ 500ms assertion
//! - AC-016 (S-15.01 v1.8): p95 ≤ 1500ms assertion (Class A — cold-start dispatch; per ADR-020)
//! - BC-1.14.001 postcondition 2: sync_group execution + verdict aggregation
//! - DI-019: ASYNC_DRAIN_WINDOW_MS contributes to total latency bound

use std::time::{Duration, Instant};

/// P95_LATENCY_BUDGET_MS: maximum acceptable p95 sync_group latency (AC-016).
///
/// This is NOT DI-019. DI-019 is ASYNC_DRAIN_WINDOW_MS (drain window after
/// sync_group completes). This budget covers the entire dispatch call including
/// sync_group execution and drain window overhead.
// AC-016 budget per ADR-020 (Class A — cold-start dispatch). Original 500ms revised after F5 pass-1 finding F-P1-003 + F-P1-009.
const P95_LATENCY_BUDGET_MS: u64 = 1500;

/// CANARY_ITERATIONS: number of dispatch invocations for p95 measurement.
const CANARY_ITERATIONS: usize = 100;

/// P95_INDEX: index into sorted latency vec for the 95th percentile.
///
/// 95th percentile of 100 samples = index 94 (0-indexed).
const P95_INDEX: usize = 94; // floor(0.95 * 100) - 1

/// AC-016: Sync-group p95 latency canary.
///
/// Measures dispatch latency across CANARY_ITERATIONS invocations and asserts
/// that the 95th percentile is ≤ P95_LATENCY_BUDGET_MS (1500ms).
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
/// (without this, all plugins run in sync_group and p95 may exceed 1500ms).
#[test]
#[ignore = "latency canary: requires --release build and populated plugin set; run with --ignored"]
fn test_BC_1_14_001_ac016_sync_group_p95_latency() {
    // DI-019: reference ASYNC_DRAIN_WINDOW_MS by name for the total latency bound.
    // Do NOT hardcode 100ms. The 1500ms budget here covers sync_group + drain window.
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

    // Measurement loop (F5-T-D: replace black_box placeholder with real dispatch).
    //
    // Option (a): spawn the factory-dispatcher binary as a child process per iteration
    // with a representative envelope on stdin. This exercises the full production path
    // including WASM load, partition, sync_group execution, and drain.
    //
    // Precondition: the binary must exist at the --release target path.
    // This test is already #[ignore] and requires --release for representative timings.
    //
    // F-P1-003: "Replace black_box placeholder with actual sync_group dispatch invocation."
    // F-P1-009: real p95 numbers must be recorded in latency-canary.md.
    let dispatcher_bin = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .expect("crates/")
        .parent() // workspace root
        .expect("workspace root")
        .join("target")
        .join("release")
        .join("factory-dispatcher");

    assert!(
        dispatcher_bin.exists(),
        "latency_canary: factory-dispatcher release binary not found at {dispatcher_bin:?}. \
         Build with: cargo build --release -p factory-dispatcher"
    );

    let plugin_root = registry_path
        .parent()
        .expect("registry path must have parent")
        .to_str()
        .expect("plugin root path must be UTF-8")
        .to_string();

    // Representative envelope: PostToolUse on Write — exercises sync validators.
    let envelope_json = serde_json::to_string(&payload).expect("payload must serialize");

    for _ in 0..CANARY_ITERATIONS {
        let start = Instant::now();

        // Spawn factory-dispatcher binary with the representative envelope on stdin.
        // This is the full production dispatch path — no shortcuts.
        let output = std::process::Command::new(&dispatcher_bin)
            .env("CLAUDE_PLUGIN_ROOT", &plugin_root)
            .env("CLAUDE_PROJECT_DIR", env!("CARGO_MANIFEST_DIR"))
            // Redirect stderr to /dev/null to suppress dispatcher diagnostic output
            // (it clutters the test output; the binary still runs the full path).
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(envelope_json.as_bytes());
                }
                child.wait_with_output()
            });

        let elapsed = start.elapsed();
        // Record elapsed regardless of success — a failing invocation still
        // contributes to the latency distribution (startup + error path).
        latencies.push(elapsed);

        // Best-effort: log any unexpected non-zero exit (not exit 2 which is a valid block).
        if let Ok(ref out) = output
            && !out.status.success()
            && out.status.code() != Some(2)
        {
            eprintln!(
                "latency_canary: unexpected exit code {:?} on iteration",
                out.status.code()
            );
        }
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

    // The primary assertion: p95 ≤ 1500ms (Class A — cold-start dispatch, per ADR-020).
    assert!(
        p95_ms <= P95_LATENCY_BUDGET_MS,
        "test_BC_1_14_001_ac016_sync_group_p95_latency: \
         AC-016 FAIL — sync_group p95 latency is {}ms, budget is {}ms (ADR-020 Class A). \
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
fn test_BC_1_14_001_ac016_latency_budget_constant_is_1500ms() {
    // The 1500ms budget covers sync_group execution + ASYNC_DRAIN_WINDOW_MS (DI-019).
    // Revised from 500ms per ADR-020 (Class A — cold-start dispatch).
    // This is not a tight bound — it is a regression guard for gross misclassification.
    assert_eq!(
        P95_LATENCY_BUDGET_MS, 1500,
        "test_BC_1_14_001_ac016_latency_budget_constant_is_1500ms: \
         P95 latency budget must be 1500ms per AC-016 (ADR-020 Class A)"
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
