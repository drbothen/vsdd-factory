// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-1.18.005 (S-25.02 F4 BC-cluster 1 "cap+trigger") integration coverage
//! for the `executor.rs` -> `shard_manager.rs` wiring path.
//!
//! `executor.rs::shard_cap_precheck` (private) is the native shard-cap gate
//! invocation point, called from `execute_tiers` BEFORE the registry-driven
//! tier loop (Invariant 1). It applies two cheap, real guards before ever
//! reaching the live gate logic:
//!
//! 1. Tool-name filter — only `Edit`/`Write`/`MultiEdit` PreToolUse calls are
//!    candidates (Precondition 1).
//! 2. Config-presence filter — a no-op when no `[[shard]]` config file exists
//!    at the well-known relative path `.factory/shard-config.toml`.
//!
//! This file supplies a REAL `[[shard]]` config fixture at that exact path
//! (relative to a tempdir `cwd`) to drive a matching Edit/Write/MultiEdit
//! call PAST both guards and into `ShardRegistry::load` / `shard_cap_gate_check`
//! — both fully implemented. Every "drives the gate" test below asserts the
//! real post-implementation outcome and is green. The two negative-control
//! tests (no config present; a non-mutating tool name) exercise the
//! independent guard logic in `executor.rs` and lock in the exact wiring
//! conditions the positive tests above depend on, so a regression in either
//! guard is caught independently of the gate logic itself. The F-001 tests
//! further down additionally cover `execute_tiers`'s translation of a
//! `HookResult::Error` gate verdict into a blocking dispatch outcome
//! (non-zero `exit_code`).

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
// Real config present + a candidate tool name drives execution through
// executor.rs's guards and into the live shard_manager gate
// (ShardRegistry::load is the first function hit).
// ---------------------------------------------------------------------------

// NOTE (BC-5.38.001 Red Gate discipline): these three tests deliberately do
// NOT use `#[should_panic]`. Each test asserts the real expected outcome (a
// within-cap call MUST Continue: `exit_code == 0`, `block_intent == false`),
// which is the correct assertion shape both during Red Gate (where it fails
// against the former `todo!()` stub) and now that `shard_manager.rs` is
// fully implemented (where it passes unchanged).

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_INV1_write_with_real_shard_config_reaches_native_gate() {
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
async fn test_BC_1_18_005_INV1_edit_with_real_shard_config_reaches_native_gate() {
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
async fn test_BC_1_18_005_INV1_multi_edit_with_real_shard_config_reaches_native_gate() {
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
// Negative controls — REAL executor.rs guard behavior. These MUST pass:
// they document and lock in the exact two conditions (config-presence,
// tool-name match) the positive tests above rely on to reach the live gate
// at all. A regression in either guard would otherwise silently make the
// positive tests above "pass for the wrong reason" (e.g. no-op bypass
// rather than reaching the gate).
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
        "no [[shard]] config file present MUST be a complete no-op — never reaches the live gate"
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
// dispatch outcome (non-zero exit_code). `executor.rs`'s `execute_tiers`
// matches the `HookResult` returned by `shard_cap_precheck` and sets
// `block_intent = true` for `HookResult::Error` (EC-009 missing `shape`;
// EC-011 `low_water_mark >= N`) and `HookResult::Block` alike, BEFORE the
// registry-driven tier loop runs — the same fail-loud translation
// `plugin_requests_block` / `plugin_fail_closed` / `bim_fired` already apply
// to a WASM tier plugin's verdict, extended to the native gate's own
// verdict. The final `exit_code: if block_intent { 2 } else { 0 }` therefore
// reflects the native gate's decision, not just downstream tier plugins.
// Both tests below assert that resulting non-zero `exit_code` and are green —
// `ShardRegistry::load` itself already correctly returns `Err` for both
// malformed shapes (see the shard_manager.rs unit tests
// `test_BC_1_18_005_EC_009_...` and
// `test_BC_1_18_005_EC_011_load_rejects_low_water_mark_equal_to_n`), and
// `execute_tiers` now propagates that verdict into the dispatch outcome.
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
         the dispatch outcome (non-zero exit_code) — executor.rs's execute_tiers translates the \
         HookResult::Error ShardRegistry::load returns into block_intent = true before the tier \
         loop runs, so exit_code is 2 here, not 0"
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
         way to the dispatch outcome (non-zero exit_code) — executor.rs's execute_tiers \
         translates the HookResult::Error ShardRegistry::load returns into block_intent = true \
         before the tier loop runs, so exit_code is 2 here, not 0"
    );
}

// ---------------------------------------------------------------------------
// F-C1-P2-001 (MEDIUM, S-25.02 Phase F4 LOCAL adversary pass-2 cluster-1) —
// F-001 above already pins that a malformed [[shard]] config's fail-loud
// HookResult::Error becomes a blocking dispatch outcome (non-zero exit_code).
// It does NOT pin that the operator-facing *reason text* — the same
// artifact_stem-naming, failure-kind-naming message
// `ShardConfigError`'s own `Display` impl already produces (see
// `shard_manager.rs`'s `MissingShape` / `InvalidLowWaterMark` /
// `CapExceedsFormulaCeiling` error variants) — actually reaches anywhere an
// operator (or `main.rs::extract_block_info`, which scans
// `TierExecutionSummary::per_plugin_results` for a blocking entry's reason)
// can see it.
//
// Today, `executor.rs::execute_tiers`'s shard-gate match arm only flips the
// local `block_intent` bool for `HookResult::Error` / `HookResult::Block` —
// it never appends anything describing the native gate's own verdict to
// `all_outcomes` (i.e. the `Vec<PluginOutcome>` that becomes
// `summary.per_plugin_results`). With an empty tier list (as these fixtures
// use, mirroring the F-001 tests' "reaches the native gate, nothing else
// runs" shape), `per_plugin_results` is therefore always `[]` regardless of
// what the native gate decided — `extract_block_info` (main.rs) has nothing
// to scan and falls back to its empty-string default for both
// `blocking_plugins` and `block_reason`. An operator seeing
// `block_intent=true exit_code=2` in the dispatcher's stderr summary line
// today gets `block_reason=""` for a malformed shard-cap config: the
// artifact_stem and failure-kind information `ShardConfigError`'s `Display`
// impl already carries is silently dropped on the floor between
// `ShardRegistry::load` and the operator.
//
// Each test below MUST fail today for exactly that reason: `is_empty()` is
// asserted false but the Debug-dump of `per_plugin_results` is always the
// empty-vec text, which contains neither the artifact_stem nor the
// failure-kind marker. Asserted generically against
// `format!("{:?}", summary.per_plugin_results)` (the same
// `Vec<PluginOutcome>` structure `main.rs::extract_block_info` reads) rather
// than against a specific new field name, since `extract_block_info` itself
// is a private `main.rs` fn not reachable from this integration-test crate,
// and the exact plumbing shape (a synthetic `PluginOutcome` entry vs. some
// other channel) is the implementer's call — this test only pins that the
// message ends up SOMEWHERE inside the structure the operator-facing
// surfacing path already scans.
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_P2001_missing_shape_block_reason_names_artifact_stem_and_failure_kind() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(100)).unwrap();

    // Same EC-009 malformed body as the F-001 missing-shape test above.
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
        "precondition: this config MUST still block (see F-001)"
    );

    let per_plugin_debug = format!("{:?}", summary.per_plugin_results);
    assert!(
        per_plugin_debug.contains("decision-log"),
        "F-C1-P2-001: the block_reason surfacing path (per_plugin_results, which \
         main.rs::extract_block_info scans) MUST name the offending artifact_stem \
         (\"decision-log\") somewhere so an operator can locate the bad config entry without \
         opening the internal log. Got: {per_plugin_debug}"
    );
    assert!(
        per_plugin_debug.contains("EC-009"),
        "F-C1-P2-001: the block_reason surfacing path MUST name the failure kind (EC-009 \
         missing `shape`) so an operator knows WHICH BC-1.18.005 fail-loud condition fired, not \
         just that something blocked. Got: {per_plugin_debug}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_P2001_low_water_mark_block_reason_names_artifact_stem_and_failure_kind() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("BC-INDEX.md");
    std::fs::write(
        &target,
        "---\ntitle: \"BC-INDEX\"\nchangelog:\n  - version: \"1.0\"\n---\n\n# Body\n",
    )
    .unwrap();

    // Same EC-011 malformed body as the F-001 low-water-mark test above.
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
        "precondition: this config MUST still block (see F-001)"
    );

    let per_plugin_debug = format!("{:?}", summary.per_plugin_results);
    assert!(
        per_plugin_debug.contains("BC-INDEX"),
        "F-C1-P2-001: the block_reason surfacing path MUST name the offending artifact_stem \
         (\"BC-INDEX\") so an operator can locate the bad config entry without opening the \
         internal log. Got: {per_plugin_debug}"
    );
    assert!(
        per_plugin_debug.contains("EC-011"),
        "F-C1-P2-001: the block_reason surfacing path MUST name the failure kind (EC-011 \
         low_water_mark >= N) so an operator knows WHICH BC-1.18.005 fail-loud condition fired. \
         Got: {per_plugin_debug}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_P2001_cap_exceeds_ceiling_block_reason_names_artifact_stem_and_failure_kind()
 {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("over-cap-log.md");
    std::fs::write(&target, "x".repeat(10)).unwrap();

    // Postcondition 9 / EC-013: declared shard_cap_bytes (100,000) exceeds
    // compute_shard_cap_bytes(these four inputs) = 50,640 — same worked
    // example as shard_manager.rs's
    // test_BC_1_18_005_PC9_EC013_load_rejects_cap_greater_than_formula_ceiling.
    let malformed_cap_exceeds_ceiling: &str = "\
[[shard]]
artifact_stem = \"over-cap-log\"
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36
max_single_record_bytes = 16384
safety_margin = 8192
shard_cap_bytes = 100000
shape = \"flat\"
";

    let summary = run_shard_gate_for_config(
        dir.path(),
        malformed_cap_exceeds_ceiling,
        &target,
        "Write",
        serde_json::json!({"content": "x".repeat(10)}),
    )
    .await;

    assert_ne!(
        summary.exit_code, 0,
        "Postcondition 9 / EC-013: a [[shard]] entry declaring shard_cap_bytes GREATER than its \
         own formula-derived ceiling MUST fail-loud all the way to the dispatch outcome \
         (non-zero exit_code), the same as the EC-009/EC-011 F-001 cases above"
    );

    let per_plugin_debug = format!("{:?}", summary.per_plugin_results);
    assert!(
        per_plugin_debug.contains("over-cap-log"),
        "F-C1-P2-001: the block_reason surfacing path MUST name the offending artifact_stem \
         (\"over-cap-log\") so an operator can locate the bad config entry without opening the \
         internal log. Got: {per_plugin_debug}"
    );
    assert!(
        per_plugin_debug.contains("EC-013"),
        "F-C1-P2-001: the block_reason surfacing path MUST name the failure kind (Postcondition \
         9 / EC-013 cap-exceeds-ceiling) so an operator knows WHICH BC-1.18.005 fail-loud \
         condition fired. Got: {per_plugin_debug}"
    );
}

// ---------------------------------------------------------------------------
// F-C1-P2-004 (LOW, S-25.02 Phase F4 LOCAL adversary pass-2 cluster-1) —
// negative control proving the native shard-cap gate is PreToolUse-scoped
// ONLY (BC-1.18.005 Precondition 1). `shard_cap_precheck` (executor.rs)
// filters on `tool_name` (Edit/Write/MultiEdit) and on `[[shard]]`
// config-presence — it never reads `event_name` / classifies the dispatch's
// `EventType` at all, so a PostToolUse Edit/Write/MultiEdit call against a
// matched, malformed `[[shard]]` entry reaches `ShardRegistry::load` and
// blocks EXACTLY like a PreToolUse call would (see the F-001 tests above,
// which omit `event_name` from their payload entirely). This test MUST fail
// today for that reason: it asserts the PostToolUse call does NOT block,
// which is false under today's event-blind `shard_cap_precheck`.
// ---------------------------------------------------------------------------

/// Same shape as `inputs_for`, plus an explicit `event_name` field in the
/// payload envelope — mirroring the exact `EventType` classification
/// `main.rs` performs via `EventType::from_event_str(&payload.event_name)`
/// (see `main.rs`'s `event_is_advisory_only` computation), so this fixture's
/// `payload_value` is shaped exactly like a real harness envelope's
/// `event_name` field (`HookPayload::event_name`), not an ad hoc string.
#[allow(clippy::too_many_arguments)]
fn inputs_for_event<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
    cwd: &std::path::Path,
    event_name: &str,
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
            "event_name": event_name,
            "tool_name": tool_name,
            "tool_input": tool_input,
        }),
        base_host_ctx: base,
        internal_log: internal_log.clone(),
        resolver_registry: Arc::new(ResolverRegistry::new()),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_005_PC1_post_tool_use_event_does_not_run_shard_gate_negative_control() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("decision-log.md");
    std::fs::write(&target, "x".repeat(100)).unwrap();

    // Deliberately the SAME EC-009 malformed body the F-001
    // `test_BC_1_18_005_F001_malformed_config_missing_shape_ec009_blocks_dispatch_outcome` test
    // proves blocks a PreToolUse-equivalent dispatch (that test's payload omits `event_name`
    // entirely) — if this PostToolUse call is ALSO blocked, the native gate is not honoring
    // Precondition 1's PreToolUse scoping.
    let malformed_missing_shape: &str = "\
[[shard]]
artifact_stem = \"decision-log\"
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36
max_single_record_bytes = 16384
safety_margin = 8192
shard_cap_bytes = 49152
";
    write_shard_config(dir.path(), malformed_missing_shape);

    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let registry = empty_registry();
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let inputs = inputs_for_event(
        &engine,
        &cache,
        &registry,
        &internal_log,
        dir.path(),
        factory_dispatcher::invoke::EventType::PostToolUse.as_str(),
        "Edit",
        &target,
        serde_json::json!({"old_string": "a", "new_string": "ab"}),
    );

    let summary = execute_tiers(inputs, vec![]).await;

    assert_eq!(
        summary.exit_code, 0,
        "BC-1.18.005 Precondition 1: the native shard-cap gate is PreToolUse-scoped ONLY. A \
         PostToolUse Edit call against the SAME malformed [[shard]] config that \
         test_BC_1_18_005_F001_malformed_config_missing_shape_ec009_blocks_dispatch_outcome \
         proves blocks a PreToolUse-equivalent dispatch MUST NOT be blocked here — the gate \
         must not even run for a non-PreToolUse event. shard_cap_precheck currently filters on \
         tool_name and config-presence only, never on event_name/EventType, so this PostToolUse \
         call reaches ShardRegistry::load exactly like the PreToolUse case and blocks \
         identically (exit_code=2)."
    );
    assert!(
        !summary.block_intent,
        "PC1: a PostToolUse dispatch MUST NOT have block_intent set by the shard-cap gate"
    );
}
