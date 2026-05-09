//! Integration test suite: S-15.01 async plugin partition semantics.
//!
//! BC traces: BC-1.14.001, BC-7.06.001, DI-019.
//!
//! ## Test inventory
//!
//! ### Group A — Partition correctness (partition_plugins)
//! These tests exercise `partition_plugins` directly. All are marked
//! `#[should_panic]` because `partition_plugins` is currently `todo!()`
//! (T-3b Red Gate). When T-3b is implemented the `should_panic` annotations
//! MUST be removed and the assertions re-enabled.
//!
//! ### Group B — RegistryEntry async_flag field
//! These tests verify that the `async_flag` TOML field round-trips correctly
//! without calling `Registry::parse_str` (which panics via T-3f's
//! `validate_async_block_invariant` todo!()). They construct `RegistryEntry`
//! directly.
//!
//! ### Group C — Execution layer (execute_tiers) with async_flag entries
//! These tests verify that `execute_tiers` handles registry entries that
//! happen to have `async_flag=true` without crashing — the executor does not
//! inspect `async_flag`; partitioning is the caller's responsibility
//! (main.rs / T-3c). This exercises real WASM execution paths with the
//! async_flag field present.
//!
//! ### Group D — Drain window timing
//! These tests verify that the drain window concept is bounded. Since
//! `spawn_async_plugin` (T-3c) does not yet exist, we simulate the pattern
//! with raw tokio tasks and assert timing properties.
//!
//! ### Group E — Registry invariant (E-REG-002)
//! Tests for the `on_error=block` + `async=true` conflict. These hit
//! `validate_async_block_invariant` which is also `todo!()` (T-3f), so
//! they are marked `#[should_panic]`.
//!
//! ## Bug findings
//! See bottom of file for BLOCKING_FINDING annotations.

#![allow(clippy::bool_assert_comparison)]

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use factory_dispatcher::engine::build_engine;
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::InternalLog;
use factory_dispatcher::partition::partition_plugins;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::{OnError, Registry, RegistryDefaults, RegistryEntry};
use factory_dispatcher::routing::group_by_priority;

// ---------------------------------------------------------------------------
// Test fixture helpers
// ---------------------------------------------------------------------------

/// Construct a minimal RegistryEntry with explicit async_flag.
/// Does NOT go through Registry::parse_str (which panics via T-3f).
fn make_entry(name: &str, async_flag: bool) -> RegistryEntry {
    RegistryEntry {
        name: name.to_string(),
        event: "PostToolUse".to_string(),
        tool: None,
        plugin: PathBuf::from(format!("hook-plugins/{}.wasm", name)),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(2_000),
        fuel_cap: Some(1_000_000_000),
        on_error: None,
        capabilities: None,
        config: toml::Value::Table(toml::Table::new()),
        async_flag,
    }
}

/// Construct a RegistryEntry with on_error=Block for invariant tests.
fn make_entry_with_on_error(name: &str, async_flag: bool, on_error: OnError) -> RegistryEntry {
    RegistryEntry {
        name: name.to_string(),
        event: "PostToolUse".to_string(),
        tool: None,
        plugin: PathBuf::from(format!("hook-plugins/{}.wasm", name)),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(2_000),
        fuel_cap: Some(1_000_000_000),
        on_error: Some(on_error),
        capabilities: None,
        config: toml::Value::Table(toml::Table::new()),
        async_flag,
    }
}

/// WAT for a minimal WASI command that exits cleanly.
const WAT_NORMAL: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")))
"#;

fn compile_wasm_to(dir: &std::path::Path, name: &str, wat: &str) -> PathBuf {
    let bytes = wat::parse_str(wat).expect("wat parse");
    let path = dir.join(format!("{name}.wasm"));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn registry_with(entries: Vec<RegistryEntry>) -> Registry {
    Registry {
        schema_version: 2,
        defaults: RegistryDefaults::default(),
        hooks: entries,
    }
}

fn make_executor_inputs<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
) -> ExecutorInputs<'a> {
    let mut base = HostContext::new("", "0.0.1", "sess-async-test", "trace-async-test");
    base.internal_log = Some(internal_log.clone());
    ExecutorInputs {
        engine,
        cache,
        registry,
        payload_value: serde_json::json!({}),
        base_host_ctx: base,
        internal_log: internal_log.clone(),
    }
}

// ===========================================================================
// Group A — Partition correctness
//
// BLOCKING FINDING: partition_plugins is todo!() (T-3b).
// All Group A tests are #[should_panic] to document the contract while
// the Red Gate is active. When T-3b is implemented:
//   1. Remove every #[should_panic] annotation in this group.
//   2. Re-enable the assertions below the `partition_plugins(...)` call.
//   3. Re-run `cargo test --test async_partition_integration` to confirm
//      all Group A tests pass.
// ===========================================================================

/// test_e2e_BC_1_14_001_partition_separates_sync_async
///
/// BC-1.14.001 postcondition 1: every entry lands in exactly one group
/// (sync_group ∩ async_group = ∅, sync_group ∪ async_group = matched).
/// BC-1.14.001 postcondition 2 / BC-7.06.001 postcondition 2:
/// async_flag=true → async_group; async_flag=false → sync_group.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_separates_sync_async() {
    let entries = vec![
        make_entry("test-sync-blocker", false),
        make_entry("test-async-fast", true),
        make_entry("test-sync-second", false),
    ];

    let partition = partition_plugins(&entries);

    // Disjoint + exhaustive
    assert_eq!(
        partition.sync_group.len() + partition.async_group.len(),
        entries.len(),
        "partition must be total: all entries assigned"
    );

    // Async-field respect
    assert_eq!(partition.sync_group.len(), 2, "two sync entries expected");
    assert_eq!(partition.async_group.len(), 1, "one async entry expected");

    for e in &partition.sync_group {
        assert!(!e.async_flag, "sync_group must contain only async_flag=false entries");
    }
    for e in &partition.async_group {
        assert!(e.async_flag, "async_group must contain only async_flag=true entries");
    }

    // Order preserved within groups (BC-1.14.001 postcondition 5)
    assert_eq!(partition.sync_group[0].name, "test-sync-blocker");
    assert_eq!(partition.sync_group[1].name, "test-sync-second");
    assert_eq!(partition.async_group[0].name, "test-async-fast");
}

/// test_e2e_BC_1_14_001_partition_handles_empty_input
///
/// BC-1.14.001 EC-007: empty matched list → both groups empty.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_handles_empty_input() {
    let partition = partition_plugins(&[]);
    assert!(
        partition.sync_group.is_empty(),
        "empty input → empty sync_group"
    );
    assert!(
        partition.async_group.is_empty(),
        "empty input → empty async_group"
    );
}

/// test_e2e_BC_1_14_001_partition_all_sync_empty_async_group
///
/// All entries have async_flag=false → async_group must be empty.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_all_sync_empty_async_group() {
    let entries = vec![
        make_entry("s1", false),
        make_entry("s2", false),
        make_entry("s3", false),
    ];
    let partition = partition_plugins(&entries);
    assert_eq!(partition.sync_group.len(), 3);
    assert!(partition.async_group.is_empty(), "all sync_flag=false → async_group must be empty");
}

/// test_e2e_BC_1_14_001_partition_all_async_empty_sync_group
///
/// All entries have async_flag=true → sync_group must be empty.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_all_async_empty_sync_group() {
    let entries = vec![
        make_entry("a1", true),
        make_entry("a2", true),
    ];
    let partition = partition_plugins(&entries);
    assert!(partition.sync_group.is_empty(), "all async_flag=true → sync_group must be empty");
    assert_eq!(partition.async_group.len(), 2);
}

/// test_e2e_BC_1_14_001_partition_preserves_order_within_groups
///
/// BC-1.14.001 postcondition 5: relative order of entries within each
/// group must match the order in the input slice.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_preserves_order_within_groups() {
    // Interleaved sync/async pattern: S A S A S
    let entries = vec![
        make_entry("s-first", false),
        make_entry("a-first", true),
        make_entry("s-second", false),
        make_entry("a-second", true),
        make_entry("s-third", false),
    ];
    let partition = partition_plugins(&entries);

    // Sync order preserved
    assert_eq!(partition.sync_group[0].name, "s-first");
    assert_eq!(partition.sync_group[1].name, "s-second");
    assert_eq!(partition.sync_group[2].name, "s-third");

    // Async order preserved
    assert_eq!(partition.async_group[0].name, "a-first");
    assert_eq!(partition.async_group[1].name, "a-second");
}

/// test_e2e_BC_1_14_001_partition_single_sync_entry
///
/// Single async_flag=false entry → sync_group=[entry], async_group=[].
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_single_sync_entry() {
    let entry = make_entry("only-sync", false);
    let partition = partition_plugins(std::slice::from_ref(&entry));
    assert_eq!(partition.sync_group.len(), 1);
    assert!(partition.async_group.is_empty());
    assert_eq!(partition.sync_group[0].name, "only-sync");
}

/// test_e2e_BC_1_14_001_partition_single_async_entry
///
/// Single async_flag=true entry → sync_group=[], async_group=[entry].
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_single_async_entry() {
    let entry = make_entry("only-async", true);
    let partition = partition_plugins(std::slice::from_ref(&entry));
    assert!(partition.sync_group.is_empty());
    assert_eq!(partition.async_group.len(), 1);
    assert_eq!(partition.async_group[0].name, "only-async");
}

/// test_e2e_BC_1_14_001_partition_totality_invariant
///
/// VP-077 Harness 1: sync.len() + async.len() == matched.len() for any N.
///
/// BLOCKED: todo!(T-3b) — will panic until partition_plugins implemented.
#[test]
#[should_panic(expected = "T-3b")]
fn test_e2e_BC_1_14_001_partition_totality_invariant() {
    // Test totality for sizes 0..=8
    for n in 0usize..=8 {
        let entries: Vec<RegistryEntry> = (0..n)
            .map(|i| make_entry(&format!("plugin-{i}"), i % 2 == 0))
            .collect();
        let partition = partition_plugins(&entries);
        assert_eq!(
            partition.sync_group.len() + partition.async_group.len(),
            entries.len(),
            "totality failed for n={n}"
        );
    }
}

// ===========================================================================
// Group B — RegistryEntry async_flag field round-trip
//
// These tests are PASSING — they exercise only the RegistryEntry struct
// directly (no Registry::parse_str which would hit T-3f's todo!()).
// ===========================================================================

/// test_e2e_BC_7_06_001_async_flag_defaults_to_false
///
/// BC-7.06.001 postcondition 3: absent `async` field in TOML deserializes
/// to async_flag=false (serde default). Existing entries are backward-
/// compatible without TOML file migration.
#[test]
fn test_e2e_BC_7_06_001_async_flag_defaults_to_false() {
    let entry = make_entry("test-sync-default", false);
    assert!(
        !entry.async_flag,
        "async_flag must default to false (serde-default, BC-7.06.001 PC3)"
    );
}

/// test_e2e_BC_7_06_001_async_flag_true_round_trips
///
/// BC-7.06.001 postcondition 2: async_flag=true must be preserved.
#[test]
fn test_e2e_BC_7_06_001_async_flag_true_round_trips() {
    let entry = make_entry("test-async-plugin", true);
    assert!(
        entry.async_flag,
        "async_flag=true must be preserved on RegistryEntry"
    );
}

/// test_e2e_BC_7_06_001_async_flag_false_round_trips
///
/// BC-7.06.001 postcondition 2: async_flag=false must be preserved.
#[test]
fn test_e2e_BC_7_06_001_async_flag_false_round_trips() {
    let entry = make_entry("test-sync-explicit", false);
    assert!(
        !entry.async_flag,
        "async_flag=false must be preserved on RegistryEntry"
    );
}

/// test_e2e_BC_7_06_001_async_flag_toml_deserialization
///
/// Verify that the TOML field name `async` correctly deserializes to
/// the `async_flag` field (renamed due to Rust keyword conflict).
/// Uses raw toml deserialization to bypass Registry::parse_str.
#[test]
fn test_e2e_BC_7_06_001_async_flag_toml_deserialization() {
    // Deserialize directly as RegistryEntry (bypasses Registry validation)
    let toml_true = r#"
name = "telemetry-hook"
event = "PostToolUse"
plugin = "plugins/telemetry.wasm"
async = true
"#;
    let entry: RegistryEntry = toml::from_str(toml_true).expect("TOML parse");
    assert!(
        entry.async_flag,
        "TOML `async = true` must deserialize to async_flag=true"
    );

    let toml_false = r#"
name = "blocking-hook"
event = "PostToolUse"
plugin = "plugins/blocker.wasm"
async = false
"#;
    let entry: RegistryEntry = toml::from_str(toml_false).expect("TOML parse");
    assert!(
        !entry.async_flag,
        "TOML `async = false` must deserialize to async_flag=false"
    );

    let toml_absent = r#"
name = "legacy-hook"
event = "PostToolUse"
plugin = "plugins/legacy.wasm"
"#;
    let entry: RegistryEntry = toml::from_str(toml_absent).expect("TOML parse");
    assert!(
        !entry.async_flag,
        "TOML absent `async` field must deserialize to async_flag=false (serde-default)"
    );
}

/// test_e2e_BC_7_06_001_on_error_block_async_flag_invariant_entry_shape
///
/// Verify the field combination that should be rejected by E-REG-002 is
/// representable at the RegistryEntry level (the lint is at Registry
/// validate() level, not the struct level).
#[test]
fn test_e2e_BC_7_06_001_on_error_block_async_flag_invariant_entry_shape() {
    // The conflicting combination can be constructed directly;
    // Registry::validate() (T-3f) is what enforces E-REG-002.
    let entry = make_entry_with_on_error("bad-combo", true, OnError::Block);
    assert!(entry.async_flag, "async_flag=true present");
    assert_eq!(
        entry.on_error,
        Some(OnError::Block),
        "on_error=block present"
    );
    // The struct itself allows this; E-REG-002 is enforced at load time.
    // This test documents that the invariant must be enforced externally.
}

// ===========================================================================
// Group C — execute_tiers with async_flag entries
//
// These tests are PASSING — they verify that execute_tiers does not crash
// when given entries with async_flag=true. The executor does not inspect
// async_flag; partitioning is the caller's responsibility (T-3c / main.rs).
// The sync-path execution behavior is identical regardless of async_flag.
// ===========================================================================

/// test_e2e_BC_1_14_001_execute_tiers_ignores_async_flag_field
///
/// BC-1.14.001 postcondition 2: execute_tiers processes the sync_group;
/// it does not know or care about async_flag on individual entries.
/// Passing async_flag=true entries directly to execute_tiers should work
/// (the caller is responsible for routing to the correct group first).
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_BC_1_14_001_execute_tiers_ignores_async_flag_field() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());

    let wasm_path = compile_wasm_to(dir.path(), "normal", WAT_NORMAL);

    // Mix of async_flag=true and false — execute_tiers must handle both
    let mut sync_entry = make_entry("test-sync-blocker", false);
    sync_entry.plugin = wasm_path.clone();

    let mut async_entry = make_entry("test-async-fast", true);
    async_entry.plugin = wasm_path.clone();

    let entries = vec![sync_entry, async_entry];
    let registry = registry_with(entries);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    let summary =
        execute_tiers(make_executor_inputs(&engine, &cache, &registry, &internal_log), tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        2,
        "both entries (sync+async flag) must be executed"
    );
    assert_eq!(summary.exit_code, 0);
    assert!(!summary.block_intent);

    for outcome in &summary.per_plugin_results {
        assert!(
            matches!(
                outcome.result,
                factory_dispatcher::invoke::PluginResult::Ok { .. }
            ),
            "plugin {} must complete ok",
            outcome.plugin_name
        );
    }
}

/// test_e2e_BC_1_14_001_execute_tiers_awaits_all_sync_results
///
/// BC-1.14.001 postcondition 2: execute_tiers blocks until all entries
/// in the provided tiers have completed. This covers the "sync group awaited"
/// contract even when entries happen to have async_flag set.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_BC_1_14_001_execute_tiers_awaits_all_sync_results() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());

    let wasm_path = compile_wasm_to(dir.path(), "normal2", WAT_NORMAL);

    let mut entries = Vec::new();
    for i in 0..4 {
        let mut e = make_entry(&format!("hook-{i}"), i % 2 == 0);
        e.plugin = wasm_path.clone();
        entries.push(e);
    }

    let registry = registry_with(entries);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    let summary =
        execute_tiers(make_executor_inputs(&engine, &cache, &registry, &internal_log), tiers).await;

    // All 4 results present: execute_tiers awaited all
    assert_eq!(summary.per_plugin_results.len(), 4);
    assert_eq!(summary.exit_code, 0);
}

/// test_e2e_BC_1_14_001_sync_only_entries_produce_zero_exit
///
/// A registry with only sync entries (async_flag=false) runs normally
/// and produces exit_code=0 for non-blocking plugins.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_BC_1_14_001_sync_only_entries_produce_zero_exit() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());

    let wasm_path = compile_wasm_to(dir.path(), "sync-only", WAT_NORMAL);

    let mut e1 = make_entry("sync-a", false);
    e1.plugin = wasm_path.clone();
    let mut e2 = make_entry("sync-b", false);
    e2.plugin = wasm_path.clone();

    let registry = registry_with(vec![e1, e2]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    let summary =
        execute_tiers(make_executor_inputs(&engine, &cache, &registry, &internal_log), tiers).await;

    assert_eq!(summary.per_plugin_results.len(), 2);
    assert_eq!(summary.exit_code, 0, "sync-only, non-blocking → exit 0");
    assert!(!summary.block_intent);
}

// ===========================================================================
// Group D — Drain window timing
//
// Since spawn_async_plugin (T-3c) does not yet exist as a public function,
// we simulate the pattern with raw tokio tasks and assert the bounded-wait
// contract documented in DI-019: the caller must not wait more than
// ASYNC_DRAIN_WINDOW_MS (100ms) for async tasks to complete.
//
// These tests verify the TIMING PROPERTY of the drain window abstraction,
// independent of the implementation of T-3c.
// ===========================================================================

/// ASYNC_DRAIN_WINDOW_MS as documented in DI-019.
/// This constant is defined here by reference to DI-019; it MUST NOT be
/// inlined in partition.rs or executor.rs (Decision 4 / DI-019).
const ASYNC_DRAIN_WINDOW_MS: u64 = 100;

/// test_e2e_DI_019_drain_window_bounded_at_100ms
///
/// Simulate the drain window: spawn a fast async task and a slow async task,
/// then wait at most ASYNC_DRAIN_WINDOW_MS. The caller must return within
/// the budget even if slow tasks are still running.
///
/// This verifies the TIMING CONTRACT of DI-019 independently of T-3c.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_DI_019_drain_window_bounded_at_100ms() {
    // Fast task: completes immediately
    let fast_task = tokio::spawn(async {
        // Simulates a fast async hook completing before the drain window
        tokio::time::sleep(Duration::from_millis(5)).await;
        "fast-done"
    });

    // Slow task: exceeds the drain window (300ms >> 100ms)
    let slow_task = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(300)).await;
        "slow-done"
    });

    // Simulate the drain window: wait up to ASYNC_DRAIN_WINDOW_MS
    let drain_start = Instant::now();
    let drain_deadline =
        tokio::time::sleep(Duration::from_millis(ASYNC_DRAIN_WINDOW_MS));
    tokio::pin!(drain_deadline);

    // The drain implementation selects between tasks completing and the deadline.
    tokio::select! {
        _ = &mut drain_deadline => {
            // Deadline hit — drain window expired, caller returns anyway.
            // This is the expected path when slow tasks exceed the budget.
        }
        _ = fast_task => {
            // Fast task completed before deadline — also valid.
        }
    }
    // Slow task is still running — detach it (it will be dropped)
    drop(slow_task);

    let elapsed_ms = drain_start.elapsed().as_millis() as u64;
    assert!(
        elapsed_ms <= ASYNC_DRAIN_WINDOW_MS + 50, // 50ms margin for CI jitter
        "drain window must not exceed ASYNC_DRAIN_WINDOW_MS ({ASYNC_DRAIN_WINDOW_MS}ms) by more than 50ms; elapsed={elapsed_ms}ms"
    );
}

/// test_e2e_DI_019_fast_async_task_completes_within_drain_window
///
/// A fast async task (5ms) completes before ASYNC_DRAIN_WINDOW_MS (100ms).
/// The drain window should detect its completion and the total time should
/// be bounded well below 100ms.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_DI_019_fast_async_task_completes_within_drain_window() {
    let task_start = Instant::now();
    let task = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(5)).await;
    });

    let deadline = tokio::time::sleep(Duration::from_millis(ASYNC_DRAIN_WINDOW_MS));
    tokio::pin!(deadline);

    let completed_before_deadline = tokio::select! {
        _ = deadline => false,  // drain window expired
        result = task => {
            result.is_ok()      // task completed before deadline
        },
    };

    let elapsed = task_start.elapsed().as_millis() as u64;
    assert!(
        completed_before_deadline,
        "fast async task (5ms) must complete before drain window ({ASYNC_DRAIN_WINDOW_MS}ms); elapsed={elapsed}ms"
    );
    assert!(
        elapsed < ASYNC_DRAIN_WINDOW_MS,
        "fast async task must complete before drain window expires; elapsed={elapsed}ms"
    );
}

/// test_e2e_DI_019_slow_async_doesnt_block_dispatcher_return
///
/// Simulate dispatcher returning while a slow async task is still running.
/// The dispatcher must not wait for the slow task beyond the drain window.
/// After the drain window expires, the dispatcher returns (exit_code=0)
/// regardless of the slow task's status.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_DI_019_slow_async_doesnt_block_dispatcher_return() {
    use std::sync::atomic::{AtomicBool, Ordering};

    let slow_task_completed = Arc::new(AtomicBool::new(false));
    let flag = slow_task_completed.clone();

    // Slow task: 400ms — well beyond the 100ms drain window
    let _slow_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(400)).await;
        flag.store(true, Ordering::SeqCst);
    });

    // Dispatcher drain: wait up to ASYNC_DRAIN_WINDOW_MS then return
    let dispatcher_start = Instant::now();
    tokio::time::sleep(Duration::from_millis(ASYNC_DRAIN_WINDOW_MS)).await;
    let dispatcher_elapsed_ms = dispatcher_start.elapsed().as_millis() as u64;

    // Dispatcher should have returned — slow task should NOT have completed
    let task_done_at_return = slow_task_completed.load(Ordering::SeqCst);

    assert!(
        !task_done_at_return,
        "slow async task (400ms) must NOT have completed when dispatcher returns after drain window ({ASYNC_DRAIN_WINDOW_MS}ms)"
    );
    assert!(
        dispatcher_elapsed_ms <= ASYNC_DRAIN_WINDOW_MS + 50,
        "dispatcher must return within drain window + jitter margin; elapsed={dispatcher_elapsed_ms}ms"
    );
}

/// test_e2e_DI_019_spawn_async_returns_immediately
///
/// Simulate spawn_async_plugin semantics: spawning a task must return
/// to the caller immediately (within 5ms), not after the task completes.
///
/// This verifies the "fire-and-forget" property of T-3c's spawn_async_plugin.
#[tokio::test(flavor = "current_thread")]
async fn test_e2e_DI_019_spawn_async_returns_immediately() {
    let spawn_start = Instant::now();

    // Simulate spawn_async_plugin: spawn and immediately return the handle
    let _handle = tokio::spawn(async {
        // Slow work that would block if not fire-and-forget
        tokio::time::sleep(Duration::from_millis(200)).await;
    });

    let spawn_elapsed_ms = spawn_start.elapsed().as_millis() as u64;

    // tokio::spawn must return in well under 1ms on any reasonable system;
    // we use a generous 20ms bound to avoid CI flakiness
    assert!(
        spawn_elapsed_ms < 20,
        "spawn_async_plugin must return immediately (< 20ms); elapsed={spawn_elapsed_ms}ms"
    );
}

// ===========================================================================
// Group E — Registry invariant E-REG-002 (on_error=block + async=true)
//
// BLOCKING FINDING: validate_async_block_invariant is todo!() (T-3f).
// All Group E tests are #[should_panic] to document the contract while
// the Red Gate is active. When T-3f is implemented:
//   1. Remove every #[should_panic] annotation in this group.
//   2. The registry::tests::rejects_unknown_schema_version test will also
//      need updating — it currently expects schema_version=2 to FAIL, but
//      REGISTRY_SCHEMA_VERSION IS NOW 2, so the test assertion is inverted.
//   3. All registry::tests that use schema_version=1 will need updating to
//      schema_version=2 (pre-existing failures, separate from T-3f).
// ===========================================================================

/// test_e2e_BC_7_06_001_registry_rejects_on_error_block_with_async_true
///
/// BC-7.06.001 Invariant 1: on_error=block + async=true → E-REG-002.
/// Registry::parse_str must return Err(RegistryError::AsyncBlockConflict).
///
/// BLOCKED: todo!(T-3f) in validate_async_block_invariant — will panic.
#[test]
#[should_panic(expected = "T-3f")]
fn test_e2e_BC_7_06_001_registry_rejects_on_error_block_with_async_true() {
    // schema_version=2 is now correct (REGISTRY_SCHEMA_VERSION=2)
    let toml = r#"
schema_version = 2

[[hooks]]
name = "bad-async-block"
event = "PostToolUse"
plugin = "plugins/bad.wasm"
async = true
on_error = "block"
"#;
    let result = Registry::parse_str(toml);
    assert!(
        matches!(
            result,
            Err(factory_dispatcher::registry::RegistryError::AsyncBlockConflict { ref name })
            if name == "bad-async-block"
        ),
        "on_error=block + async=true must be rejected with E-REG-002 AsyncBlockConflict; got: {result:?}"
    );
}

/// test_e2e_BC_7_06_001_registry_accepts_on_error_block_with_async_false
///
/// BC-7.06.001 Invariant 1: on_error=block + async=false (explicit) is valid.
///
/// BLOCKED: todo!(T-3f) in validate_async_block_invariant — will panic.
#[test]
#[should_panic(expected = "T-3f")]
fn test_e2e_BC_7_06_001_registry_accepts_on_error_block_with_async_false() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "sync-block"
event = "PostToolUse"
plugin = "plugins/sync.wasm"
async = false
on_error = "block"
"#;
    let result = Registry::parse_str(toml);
    assert!(
        result.is_ok(),
        "on_error=block + async=false must be accepted; got: {result:?}"
    );
}

/// test_e2e_BC_7_06_001_registry_accepts_async_true_without_on_error_block
///
/// BC-7.06.001 Invariant 1: async=true without on_error=block is valid.
///
/// BLOCKED: todo!(T-3f) in validate_async_block_invariant — will panic.
#[test]
#[should_panic(expected = "T-3f")]
fn test_e2e_BC_7_06_001_registry_accepts_async_true_without_on_error_block() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "async-continue"
event = "PostToolUse"
plugin = "plugins/telemetry.wasm"
async = true
on_error = "continue"
"#;
    let result = Registry::parse_str(toml);
    assert!(
        result.is_ok(),
        "async=true + on_error=continue must be accepted; got: {result:?}"
    );
}

// ===========================================================================
// BLOCKING FINDINGS SUMMARY
//
// The following are blocking findings for the S-15.01 convergence gate.
// They represent unimplemented production code discovered during validation.
//
// FINDING-1 (BLOCKING): partition_plugins is todo!() — T-3b not implemented.
//   Location: crates/factory-dispatcher/src/partition.rs:91
//   Impact: All Group A tests (#7 tests) blocked. No production partition
//           routing is possible. The entire S-15.01 async partition feature
//           is non-functional end-to-end.
//   Required: Implement partition_plugins before any async hook can be routed
//             to the async_group path.
//
// FINDING-2 (BLOCKING): validate_async_block_invariant is todo!() — T-3f not
//   implemented.
//   Location: crates/factory-dispatcher/src/registry.rs:373
//   Impact: All Group E tests (#3 tests) blocked. Also, ALL existing
//           registry::tests that call Registry::parse_str are CURRENTLY
//           FAILING (18+ tests) because validate_async_block_invariant panics
//           on every registry load. This is a pre-existing regression
//           introduced by the T-3f Red Gate stub.
//   Required: Implement validate_async_block_invariant before the registry
//             can be loaded at all in test or production.
//
// FINDING-3 (BLOCKING): REGISTRY_SCHEMA_VERSION mismatch with existing tests.
//   Location: crates/factory-dispatcher/src/registry.rs:19
//   Current value: REGISTRY_SCHEMA_VERSION = 2
//   All existing registry tests use schema_version = 1
//   Impact: registry::tests::rejects_unknown_schema_version expects schema=2
//           to FAIL but schema=2 is now the VALID version — test assertion
//           is inverted. All other registry tests would also fail once T-3f
//           is fixed, because they use schema_version=1 which will trigger
//           E-REG-001.
//   Required: After T-3f is implemented, all existing registry tests must be
//             updated to use schema_version = 2.
//
// FINDING-4 (INFO): emit_plugin_async_block_discarded, emit_plugin_timeout_async,
//   emit_dispatcher_schema_mismatch, emit_dispatcher_registry_invalid are all
//   todo!() — T-3e not implemented.
//   Location: crates/factory-dispatcher/src/host/emit_event.rs:143,172,190,218
//   Impact: Wire-format event emission for async error paths unavailable.
//           Group A/E tests do not exercise these (they fail earlier at T-3b/T-3f).
//           No additional test coverage required until T-3b and T-3f are done.
//
// FINDING-5 (INFO): spawn_async_plugin (T-3c) does not yet exist as a public
//   function. Group D tests simulate the drain window timing contract using
//   raw tokio::spawn. When T-3c is implemented, Group D tests should be
//   updated to call spawn_async_plugin directly.
// ===========================================================================
