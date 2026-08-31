// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Integration tests for the S-25.01 INDETERMINATE marker lifecycle.
//!
//! Adversary pass-1 findings encoded here (each test references the finding):
//!
//! - BLOCKER-1 (AC-012 / BC-1.18.003 PC1+INV2): marker auto-clear MUST be wired into
//!   execute_tiers dispatch path, not just exposed as a free function.
//! - BLOCKER-2 (AC-005 / BC-1.18.001 INV4 / EC-002 / EC-009): marker write MUST be
//!   PostToolUse-only; PreToolUse INDETERMINATE MUST emit advisory event only.
//! - MEDIUM-5 (AC-005 / AC-007 / BC-1.18.001 PC4): artifact_path MUST be threaded from
//!   the PostToolUse tool_input.file_path, not hardcoded to empty string.
//!
//! All tests drive the REAL execute_tiers / execute_tier path — no direct calls to
//! indeterminate_marker functions (which would constitute a paper-fix). Integration
//! tests are required per the adversary finding ("NOT a direct unit call to
//! delete_marker_if_pass").

use std::path::PathBuf;
use std::sync::Arc;

use factory_dispatcher::engine::{EpochTicker, build_engine};
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::InternalLog;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::{Capabilities, FailurePolicy, OnError, Registry, RegistryEntry};
use factory_dispatcher::resolver::ResolverRegistry;
use factory_dispatcher::routing::group_by_priority;

// ---------------------------------------------------------------------------
// WAT fixtures (mirrors executor_integration.rs)
// ---------------------------------------------------------------------------

/// Minimal WASI command: returns cleanly with exit_code=0 (→ DispatchOutcome::Pass).
const WAT_NORMAL: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")))
"#;

/// Infinite loop at an epoch yield point.
/// With fuel_cap=100, exhausts fuel almost immediately → DispatchOutcome::Indeterminate{Fuel}.
const WAT_HANG: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start") (loop (br 0))))
"#;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn compile_to(dir: &std::path::Path, name: &str, wat: &str) -> PathBuf {
    let bytes = wat::parse_str(wat).expect("wat parse");
    let path = dir.join(format!("{name}.wasm"));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn make_registry(entries: Vec<RegistryEntry>) -> Registry {
    Registry {
        schema_version: 1,
        defaults: Default::default(),
        hooks: entries,
    }
}

fn make_pass_entry(path: &std::path::Path, name: &str, event: &str) -> RegistryEntry {
    // Entry for a WASM that will return PASS (exit_code=0); ample fuel so it completes.
    RegistryEntry {
        name: name.to_string(),
        event: event.to_string(),
        tool: None,
        plugin: path.to_path_buf(),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(5_000),
        fuel_cap: Some(1_000_000_000), // ample fuel — no fuel exhaustion
        on_error: Some(OnError::Continue),
        capabilities: Some(Capabilities::default()),
        config: toml::Value::Table(toml::Table::new()),
        async_flag: false,
        needs_context: vec![],
        failure_policy: FailurePolicy::FailClosed,
    }
}

fn make_indeterminate_entry(path: &std::path::Path, name: &str, event: &str) -> RegistryEntry {
    // Entry for a WASM that will INDETERMINATE (WAT_HANG + very low fuel → Fuel exhaustion).
    RegistryEntry {
        name: name.to_string(),
        event: event.to_string(),
        tool: None,
        plugin: path.to_path_buf(),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(5_000), // wall-clock safety net (fuel exhausts first)
        fuel_cap: Some(100),     // very low: exhausts in microseconds → INDETERMINATE(Fuel)
        on_error: Some(OnError::Continue), // don't also trigger on_error=block path
        capabilities: Some(Capabilities::default()),
        config: toml::Value::Table(toml::Table::new()),
        async_flag: false,
        needs_context: vec![],
        failure_policy: FailurePolicy::FailClosed,
    }
}

fn executor_inputs_with_cwd<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
    cwd: PathBuf,
    payload: serde_json::Value,
) -> ExecutorInputs<'a> {
    let mut base = HostContext::new("", "0.0.1", "sess-marker-integ", "trace-marker-integ");
    base.cwd = cwd;
    base.internal_log = Some(internal_log.clone());
    ExecutorInputs {
        engine,
        cache,
        registry,
        payload_value: payload,
        base_host_ctx: base,
        internal_log: internal_log.clone(),
        resolver_registry: Arc::new(ResolverRegistry::new()),
    }
}

fn write_test_marker(path: &std::path::Path, plugin_name: &str) {
    std::fs::write(
        path,
        format!(
            "timestamp = \"2026-08-31T00:00:00Z\"\n\
             plugin_name = \"{plugin_name}\"\n\
             artifact_path = \"\"\n\
             cause = \"fuel\"\n\
             trace_id = \"trace-integ-test\"\n"
        ),
    )
    .expect("test setup: write marker");
}

// ---------------------------------------------------------------------------
// BLOCKER-1 tests: marker auto-clear via execute_tiers dispatch path
// ---------------------------------------------------------------------------

/// BLOCKER-1 PRIMARY (RED): PASS from the named plugin via execute_tiers MUST delete
/// the marker. BC-1.18.003 PC1 + INV2. AC-012.
///
/// **Why this is an integration test (not a unit test):** The adversary finding is that
/// `delete_marker_if_pass` exists as a free function but is NEVER CALLED from the
/// dispatch path. Writing a unit test that calls `delete_marker_if_pass` directly
/// would be the paper-fix the adversary warned against (TD-VSDD-059). This test
/// drives `execute_tiers → execute_tier → spawn_blocking` so the implementer MUST
/// wire the clear into the dispatch path itself.
///
/// Currently FAILS (RED) because execute_tier has no `delete_marker_if_pass` call
/// in its PASS arm — the marker is never deleted by the dispatch path.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_003_named_plugin_pass_clears_marker_via_execute_tiers() {
    // AC-012 / BC-1.18.003 PC1 + INV2
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());

    // Set up .factory dir and pre-write a marker for "plugin-p"
    let factory_dir = dir.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let marker_path = factory_dir.join("unvalidated-mutation.marker");
    write_test_marker(&marker_path, "plugin-p");
    assert!(
        marker_path.exists(),
        "pre-condition: marker must exist before dispatch"
    );

    // WAT_NORMAL returns exit_code=0 → DispatchOutcome::Pass
    let pass_wasm = compile_to(dir.path(), "plugin-p", WAT_NORMAL);
    let entry = make_pass_entry(&pass_wasm, "plugin-p", "PostToolUse");
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    let summary = execute_tiers(
        executor_inputs_with_cwd(
            &engine,
            &cache,
            &registry,
            &internal_log,
            dir.path().to_path_buf(),
            serde_json::json!({}),
        ),
        tiers,
    )
    .await;

    // Plugin produced PASS (WAT_NORMAL exits 0 → no block)
    assert_eq!(
        summary.exit_code, 0,
        "PASS plugin must not set block intent"
    );

    // AC-012 / BC-1.18.003 PC1 + INV2:
    // A PASS result from "plugin-p" MUST cause the dispatcher to call
    // `delete_marker_if_pass` and remove the marker.
    //
    // RED: currently fails because execute_tier has NO delete_marker_if_pass call
    // in its PASS path. The marker persists untouched after dispatch completes.
    assert!(
        !marker_path.exists(),
        "AC-012 / BC-1.18.003 PC1: PASS from 'plugin-p' via execute_tiers MUST delete \
         the marker at .factory/unvalidated-mutation.marker. \
         FAILS because execute_tier has no delete_marker_if_pass in its PASS arm."
    );
}

/// BLOCKER-1 SCOPING CONSTRAINT: PASS from a DIFFERENT plugin MUST NOT clear the
/// marker for plugin-p. BC-1.18.003 INV2. AC-012.
///
/// Currently GREEN (accidentally: no clear call at all → marker stays for wrong plugin
/// too). Must STAY GREEN after fix (implementer must check plugin_name before clearing).
///
/// Included to prevent a naive implementation that clears on ANY pass from ANY plugin.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_003_different_plugin_pass_does_not_clear_named_plugin_marker() {
    // AC-012 / BC-1.18.003 INV2 (scoping)
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());

    let factory_dir = dir.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let marker_path = factory_dir.join("unvalidated-mutation.marker");
    // Marker records plugin-p; we dispatch plugin-q (DIFFERENT)
    write_test_marker(&marker_path, "plugin-p");

    let pass_wasm = compile_to(dir.path(), "plugin-q", WAT_NORMAL);
    let entry = make_pass_entry(&pass_wasm, "plugin-q", "PostToolUse"); // <-- DIFFERENT plugin
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    execute_tiers(
        executor_inputs_with_cwd(
            &engine,
            &cache,
            &registry,
            &internal_log,
            dir.path().to_path_buf(),
            serde_json::json!({}),
        ),
        tiers,
    )
    .await;

    // BC-1.18.003 INV2 (scoping): plugin-q's PASS MUST NOT clear plugin-p's marker.
    // The clear is scoped to the plugin named in the marker.
    assert!(
        marker_path.exists(),
        "AC-012 / BC-1.18.003 INV2: PASS from 'plugin-q' MUST NOT delete 'plugin-p's marker \
         (scoped clear — only the named plugin's PASS clears)"
    );
}

// ---------------------------------------------------------------------------
// BLOCKER-2 tests: PostToolUse-only marker write
// ---------------------------------------------------------------------------

/// BLOCKER-2 PRIMARY (RED): INDETERMINATE for a fail-closed plugin dispatched on a
/// PreToolUse event MUST NOT write the marker. BC-1.18.001 INV4. AC-005. EC-002 / EC-009.
///
/// Currently FAILS (RED) because execute_tier has no PostToolUse event-type guard —
/// it calls `write_indeterminate_marker` for BOTH PreToolUse and PostToolUse
/// INDETERMINATE events when `should_write_marker` returns true.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_001_pretooluse_indeterminate_does_not_write_marker() {
    // AC-005 / BC-1.18.001 INV4 / EC-002 / EC-009
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(engine.clone()); // safety net for epoch timeout
    let cache = PluginCache::new(engine.clone());

    // Pre-create .factory so the write WOULD succeed if the guard is absent.
    // (Without the dir, write_indeterminate_marker silently fails → test passes vacuously.)
    let factory_dir = dir.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let marker_path = factory_dir.join("unvalidated-mutation.marker");
    assert!(
        !marker_path.exists(),
        "pre-condition: marker must NOT exist at dispatch start"
    );

    // WAT_HANG + fuel_cap=100 → fuel exhaustion → DispatchOutcome::Indeterminate{Fuel}.
    // Event = "PreToolUse": BC-1.18.001 INV4 says marker write MUST be suppressed.
    let hang_wasm = compile_to(dir.path(), "validate-factory-path-staging", WAT_HANG);
    let entry = make_indeterminate_entry(
        &hang_wasm,
        "validate-factory-path-staging",
        "PreToolUse", // <-- KEY: PreToolUse; marker write MUST be suppressed per INV4
    );
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    execute_tiers(
        executor_inputs_with_cwd(
            &engine,
            &cache,
            &registry,
            &internal_log,
            dir.path().to_path_buf(),
            serde_json::json!({}),
        ),
        tiers,
    )
    .await;

    // AC-005 / BC-1.18.001 INV4 / EC-009:
    // PreToolUse INDETERMINATE → advisory `plugin.indeterminate` event only.
    // NO marker written (PostToolUse-only invariant).
    //
    // RED: currently fails because execute_tier writes the marker for both event types.
    assert!(
        !marker_path.exists(),
        "AC-005 / BC-1.18.001 INV4 / EC-009: PreToolUse INDETERMINATE for fail-closed \
         plugin MUST NOT write the marker (PostToolUse-only per BC-1.18.001 INV4). \
         FAILS because execute_tier has no event-type guard in its INDETERMINATE arm."
    );
}

/// BLOCKER-2 COMPLEMENT: PostToolUse INDETERMINATE for fail-closed MUST write marker.
/// Currently GREEN (current code writes for both event types, no guard).
/// Must STAY GREEN after the implementer adds the PostToolUse guard.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_001_posttooluse_indeterminate_writes_marker_for_fail_closed() {
    // AC-005 / BC-1.18.001 PC4 + INV4 (the write IS expected for PostToolUse)
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(engine.clone());
    let cache = PluginCache::new(engine.clone());

    let factory_dir = dir.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let marker_path = factory_dir.join("unvalidated-mutation.marker");

    let hang_wasm = compile_to(dir.path(), "validate-factory-path-staging", WAT_HANG);
    let entry = make_indeterminate_entry(
        &hang_wasm,
        "validate-factory-path-staging",
        "PostToolUse", // PostToolUse: marker MUST be written
    );
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    execute_tiers(
        executor_inputs_with_cwd(
            &engine,
            &cache,
            &registry,
            &internal_log,
            dir.path().to_path_buf(),
            serde_json::json!({}),
        ),
        tiers,
    )
    .await;

    // AC-005 / BC-1.18.001 PC4 + INV4:
    // PostToolUse INDETERMINATE for fail-closed MUST write the marker.
    assert!(
        marker_path.exists(),
        "AC-005 / BC-1.18.001 PC4: PostToolUse INDETERMINATE for fail-closed \
         MUST write the .factory/unvalidated-mutation.marker file"
    );
}

// ---------------------------------------------------------------------------
// MEDIUM-5: artifact_path threaded from tool_input.file_path
// ---------------------------------------------------------------------------

/// MEDIUM-5 (RED): The marker's artifact_path field MUST be threaded from the
/// PostToolUse tool_input.file_path, not hardcoded to empty string. AC-005 / AC-007 /
/// BC-1.18.001 PC4.
///
/// Currently FAILS (RED) because execute_tier hardcodes `artifact_path: String::new()`
/// in the MarkerFields construction inside the spawn_blocking closure.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_1_18_001_artifact_path_threaded_from_tool_input_file_path() {
    // AC-005 / AC-007 / BC-1.18.001 PC4
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(engine.clone());
    let cache = PluginCache::new(engine.clone());

    let factory_dir = dir.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let marker_path = factory_dir.join("unvalidated-mutation.marker");

    let expected_artifact_path = "/path/to/.factory/STATE.md";

    // PostToolUse Edit dispatch with file_path set — the INDETERMINATE event must
    // capture this path in the marker's artifact_path field.
    let payload = serde_json::json!({
        "tool_name": "Edit",
        "tool_input": {
            "file_path": expected_artifact_path
        }
    });

    let hang_wasm = compile_to(dir.path(), "validate-factory-path-staging", WAT_HANG);
    let entry =
        make_indeterminate_entry(&hang_wasm, "validate-factory-path-staging", "PostToolUse");
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));
    execute_tiers(
        executor_inputs_with_cwd(
            &engine,
            &cache,
            &registry,
            &internal_log,
            dir.path().to_path_buf(),
            payload,
        ),
        tiers,
    )
    .await;

    assert!(
        marker_path.exists(),
        "pre-condition for MEDIUM-5: PostToolUse fail-closed INDETERMINATE MUST write marker"
    );

    let content =
        std::fs::read_to_string(&marker_path).expect("marker file must be readable after write");

    // AC-005 / AC-007 / BC-1.18.001 PC4:
    // The marker MUST record the artifact_path from tool_input.file_path.
    //
    // RED: currently fails because execute_tier uses `artifact_path: String::new()`
    // (hardcoded empty) instead of extracting from payload_value.tool_input.file_path.
    assert!(
        content.contains(expected_artifact_path),
        "AC-005 / AC-007 / BC-1.18.001 PC4: marker artifact_path MUST be threaded from \
         tool_input.file_path '{}'. \
         FAILS because execute_tier hardcodes artifact_path = String::new(). \
         Actual marker content: '{}'",
        expected_artifact_path,
        content
    );
}
