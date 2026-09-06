// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-1.18.005 (S-25.02 F4 BC-cluster 1 "cap+trigger") Red Gate integration
//! coverage for the `executor.rs` -> `shard_manager.rs` wiring path.
//!
//! `executor.rs::shard_cap_precheck` (private) is the native shard-cap gate
//! invocation point, called from `execute_tiers` BEFORE the registry-driven
//! tier loop (Invariant 1). It applies two cheap, REAL (non-stubbed) guards
//! before ever reaching the `shard_manager` stub:
//!
//! 1. Tool-name filter — only `Edit`/`Write`/`MultiEdit` PreToolUse calls are
//!    candidates (Precondition 1).
//! 2. Config-presence filter — a no-op when no `[[shard]]` config file exists
//!    at the well-known relative path `.factory/shard-config.toml`.
//!
//! This file supplies a REAL `[[shard]]` config fixture at that exact path
//! (relative to a tempdir `cwd`) to drive a matching Edit/Write/MultiEdit
//! call PAST both guards and into `ShardRegistry::load` / `shard_cap_gate_check`
//! — both still `todo!()` (S-25.02 F4 BC-cluster 1 stub). Every "drives the
//! stub" test below therefore currently FAILS (panics) — Red Gate per
//! BC-5.38.001. The two negative-control tests (no config present; a
//! non-mutating tool name) exercise the ALREADY-IMPLEMENTED (non-stub) guard
//! logic in `executor.rs` and are expected to PASS today — they lock in the
//! exact wiring conditions the positive Red Gate tests depend on, so a
//! regression in the guard logic itself is caught independently of the stub.

use std::sync::Arc;

use factory_dispatcher::engine::build_engine;
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::InternalLog;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::Registry;
use factory_dispatcher::resolver::ResolverRegistry;

/// A well-formed `"flat"`-shaped `[[shard]]` config entry, using the BC's
/// own provisional calibration constants (BC-1.18.005 Postcondition 6).
const FLAT_SHARD_CONFIG: &str = "\
[[shard]]
artifact_stem = \"decision-log\"
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36
max_single_record_bytes = 16384
safety_margin = 8192
shard_cap_bytes = 49152
shape = \"flat\"
";

fn empty_registry() -> Registry {
    Registry {
        schema_version: 1,
        defaults: Default::default(),
        hooks: vec![],
    }
}

/// Writes a real `.factory/shard-config.toml` under `cwd`, at the EXACT
/// relative path `executor.rs::SHARD_CONFIG_RELATIVE_PATH` resolves against
/// `base_host_ctx.cwd` — this is the fixture the task requires to drive
/// `shard_cap_precheck` past its config-presence guard.
fn write_shard_config(cwd: &std::path::Path, body: &str) {
    let factory_dir = cwd.join(".factory");
    std::fs::create_dir_all(&factory_dir).expect("create .factory dir");
    std::fs::write(factory_dir.join("shard-config.toml"), body).expect("write shard-config.toml");
}

#[allow(clippy::too_many_arguments)]
fn inputs_for<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
    cwd: &std::path::Path,
    tool_name: &str,
    target_path: &std::path::Path,
    tool_input_extra: serde_json::Value,
) -> ExecutorInputs<'a> {
    let mut base = HostContext::new("", "0.0.1", "sess-bc-1-18-005", "trace-bc-1-18-005");
    base.cwd = cwd.to_path_buf();
    base.internal_log = Some(internal_log.clone());

    let mut tool_input = tool_input_extra;
    if let Some(map) = tool_input.as_object_mut() {
        map.insert(
            "file_path".to_string(),
            serde_json::Value::String(target_path.to_string_lossy().into_owned()),
        );
    }

    ExecutorInputs {
        engine,
        cache,
        registry,
        payload_value: serde_json::json!({
            "tool_name": tool_name,
            "tool_input": tool_input,
        }),
        base_host_ctx: base,
        internal_log: internal_log.clone(),
        resolver_registry: Arc::new(ResolverRegistry::new()),
    }
}

// ---------------------------------------------------------------------------
// Red Gate: real config present + a candidate tool name drives execution
// through executor.rs's guards and into the still-todo!() shard_manager
// stub (ShardRegistry::load is the first function hit).
// ---------------------------------------------------------------------------

// NOTE (BC-5.38.001 Red Gate discipline): these three tests deliberately do
// NOT use `#[should_panic]`. A `#[should_panic]` test that panics today
// (because `ShardRegistry::load`/`shard_cap_gate_check` are `todo!()`) would
// PASS right now — the opposite of Red Gate. Instead, each test asserts the
// REAL expected post-implementation outcome (a within-cap call MUST
// Continue: `exit_code == 0`, `block_intent == false`); today the `todo!()`
// panic makes the test FAIL, which is the correct Red Gate signal. Once the
// implementer fills in `shard_manager.rs`, these assertions should pass
// unchanged.

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_INV1_write_with_real_shard_config_reaches_native_gate_stub() {
    let dir = tempfile::tempdir().unwrap();
    write_shard_config(dir.path(), FLAT_SHARD_CONFIG);
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(5_000)).unwrap();

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        "Write",
        &target,
        serde_json::json!({"content": "x".repeat(5_000)}),
    );

    // BC-1.18.005 Invariant 1 / T-2: with a real [[shard]] config present at
    // `.factory/shard-config.toml` and a matching mutating tool name, the
    // native shard-cap gate check MUST be invoked BEFORE the (here, empty)
    // registry-driven tier loop, and a within-cap Write (5,000 <= 49,152)
    // MUST Continue — zero block intent, zero exit code.
    let summary = execute_tiers(inputs, vec![]).await;
    assert_eq!(
        summary.exit_code, 0,
        "a within-cap Write against a matched [[shard]] entry MUST Continue (exit_code 0)"
    );
    assert!(!summary.block_intent);
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_INV1_edit_with_real_shard_config_reaches_native_gate_stub() {
    let dir = tempfile::tempdir().unwrap();
    write_shard_config(dir.path(), FLAT_SHARD_CONFIG);
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "y".repeat(45_000)).unwrap();

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        "Edit",
        &target,
        serde_json::json!({"old_string": "a", "new_string": "aaaaa"}),
    );

    // Precondition 1's tool-name filter includes Edit, not just Write.
    // current shard 45,000 + net_delta (+4) = 45,004 <= 49,152 -> Continue.
    let summary = execute_tiers(inputs, vec![]).await;
    assert_eq!(
        summary.exit_code, 0,
        "a within-cap Edit against a matched [[shard]] entry MUST Continue (exit_code 0)"
    );
    assert!(!summary.block_intent);
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_INV1_multi_edit_with_real_shard_config_reaches_native_gate_stub() {
    let dir = tempfile::tempdir().unwrap();
    write_shard_config(dir.path(), FLAT_SHARD_CONFIG);
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "z".repeat(48_000)).unwrap();

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        "MultiEdit",
        &target,
        serde_json::json!({
            "edits": [
                {"old_string": "a", "new_string": "aa"},
                {"old_string": "b", "new_string": ""},
            ]
        }),
    );

    // Precondition 1's tool-name filter includes MultiEdit too. net delta =
    // (+1) + (-1) = 0; projected = 48,000 + 0 = 48,000 <= 49,152 -> Continue.
    let summary = execute_tiers(inputs, vec![]).await;
    assert_eq!(
        summary.exit_code, 0,
        "a within-cap MultiEdit against a matched [[shard]] entry MUST Continue (exit_code 0)"
    );
    assert!(!summary.block_intent);
}

// ---------------------------------------------------------------------------
// Negative controls — REAL (non-stub) executor.rs guard behavior. These MUST
// pass today: they document and lock in the exact two conditions
// (config-presence, tool-name match) the positive Red Gate tests above rely
// on to reach the stub at all. A regression in either guard would otherwise
// silently make the positive tests above "pass for the wrong reason" (e.g.
// no-op bypass rather than reaching the stub).
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_PC1_no_shard_config_present_bypasses_native_gate_no_panic() {
    let dir = tempfile::tempdir().unwrap();
    // Deliberately do NOT write .factory/shard-config.toml.
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(5_000)).unwrap();

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        "Write",
        &target,
        serde_json::json!({"content": "x".repeat(5_000)}),
    );

    let summary = execute_tiers(inputs, vec![]).await;
    assert_eq!(
        summary.exit_code, 0,
        "no [[shard]] config file present MUST be a complete no-op — never reaches the stub"
    );
    assert!(!summary.block_intent);
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_PC1_non_mutating_tool_name_bypasses_native_gate_no_panic() {
    let dir = tempfile::tempdir().unwrap();
    write_shard_config(dir.path(), FLAT_SHARD_CONFIG);
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(5_000)).unwrap();

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    // "Read" is not in {Edit, Write, MultiEdit} — Precondition 1's tool-name
    // filter MUST reject it even though a real [[shard]] config is present.
    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        "Read",
        &target,
        serde_json::json!({}),
    );

    let summary = execute_tiers(inputs, vec![]).await;
    assert_eq!(
        summary.exit_code, 0,
        "a non-Edit/Write/MultiEdit tool call MUST bypass the native gate entirely, even with a matching config present"
    );
    assert!(!summary.block_intent);
}

// ---------------------------------------------------------------------------
// F-001 (HIGH, S-25.02 Phase F4 LOCAL adversary pass-1 cluster-1) — a
// malformed [[shard]] config entry MUST fail-loud all the way to the
// dispatch outcome (non-zero exit_code). Today, `executor.rs`'s
// `execute_tiers` does `let _ = shard_gate_result;` immediately after
// `shard_cap_precheck` returns — the `HookResult::Error` produced by
// `ShardRegistry::load` for a malformed entry (EC-009 missing `shape`;
// EC-011 `low_water_mark >= N`) is computed and then completely discarded.
// `block_intent` is only ever set from tier plugin outcomes further down
// (`plugin_requests_block` / `plugin_fail_closed` / `bim_fired`), never from
// the native shard-cap gate's own verdict, and the final
// `exit_code: if block_intent { 2 } else { 0 }` therefore stays 0
// regardless of what the native gate decided. Both tests below MUST fail
// today for exactly that reason — not because `ShardRegistry::load` itself
// is wrong (it already correctly returns `Err` for both malformed shapes;
// see the shard_manager.rs unit tests `test_BC_1_18_005_EC_009_...` and
// `test_BC_1_18_005_EC_011_load_rejects_low_water_mark_equal_to_n`).
// ---------------------------------------------------------------------------

/// Shared driver for the two F-001 malformed-config cases: writes the given
/// (malformed) `[[shard]]` config body to `dir/.factory/shard-config.toml`,
/// then drives a single Edit/Write/MultiEdit call for `target` through
/// `execute_tiers` with an empty tier list — exactly the same "reaches the
/// native gate, nothing else runs" shape the existing positive/negative
/// Red Gate tests above use.
async fn run_shard_gate_for_config(
    dir: &std::path::Path,
    shard_config_body: &str,
    target: &std::path::Path,
    tool_name: &str,
    tool_input_extra: serde_json::Value,
) -> factory_dispatcher::executor::TierExecutionSummary {
    write_shard_config(dir, shard_config_body);

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.join("logs")));

    let inputs = inputs_for(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir,
        tool_name,
        target,
        tool_input_extra,
    );

    execute_tiers(inputs, vec![]).await
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_F001_malformed_config_missing_shape_ec009_blocks_dispatch_outcome() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(100)).unwrap();

    // EC-009: entry omits `shape` entirely — fail-loud MissingShape at
    // ShardRegistry::load() time.
    let malformed_missing_shape: &str = "\
[[shard]]
artifact_stem = \"decision-log\"
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36
max_single_record_bytes = 16384
safety_margin = 8192
shard_cap_bytes = 49152
";

    let summary = run_shard_gate_for_config(
        dir.path(),
        malformed_missing_shape,
        &target,
        "Write",
        serde_json::json!({"content": "x".repeat(100)}),
    )
    .await;

    assert_ne!(
        summary.exit_code, 0,
        "F-001 (HIGH): a [[shard]] entry omitting `shape` (EC-009) MUST fail-loud all the way to \
         the dispatch outcome (non-zero exit_code) — today executor.rs's \
         `let _ = shard_gate_result;` silently discards the HookResult::Error ShardRegistry::load \
         already correctly returns, leaving exit_code at 0"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_F001_malformed_config_low_water_mark_ge_n_ec011_blocks_dispatch_outcome()
{
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("BC-INDEX.md");
    std::fs::write(
        &target,
        "---\ntitle: \"BC-INDEX\"\nchangelog:\n  - version: \"1.0\"\n---\n\n# Body\n",
    )
    .unwrap();

    // EC-011: low_water_mark (50) >= N (50) — the degenerate `== N` boundary
    // — fail-loud InvalidLowWaterMark at ShardRegistry::load() time.
    let malformed_low_water_mark: &str = "\
[[shard]]
artifact_stem = \"BC-INDEX\"
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36
max_single_record_bytes = 16384
safety_margin = 8192
shard_cap_bytes = 49152
shape = \"frontmatter-changelog-array\"
n = 50
low_water_mark = 50
";

    let summary = run_shard_gate_for_config(
        dir.path(),
        malformed_low_water_mark,
        &target,
        "Edit",
        serde_json::json!({"old_string": "a", "new_string": "ab"}),
    )
    .await;

    assert_ne!(
        summary.exit_code, 0,
        "F-001 (HIGH): a [[shard]] entry with low_water_mark >= N (EC-011) MUST fail-loud all the \
         way to the dispatch outcome (non-zero exit_code) — today executor.rs's \
         `let _ = shard_gate_result;` silently discards the HookResult::Error ShardRegistry::load \
         already correctly returns, leaving exit_code at 0"
    );
}
