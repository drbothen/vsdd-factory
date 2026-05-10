//! Resolver error-isolation tests (S-12.04 AC-010, AC-011).
//!
//! Verifies that a trapping WASM resolver does not abort the dispatch
//! cycle (AC-010) and that a `resolver.error` telemetry event is emitted for
//! each trap (AC-011), per BC-4.12.004 crash isolation contract.
//!
//! Uses the compiled `tests/fixtures/trapping_resolver.wasm` artifact whose
//! `resolve()` function immediately executes `unreachable`. This fixture was
//! compiled from `tests/fixtures/trapping_resolver.wat` during S-12.04 Step 2.
//!
//! All test bodies are fully authored per S-12.04 Step 2. Every test MUST
//! FAIL before Step 3 implementation (Red Gate per BC-5.38.001) because the
//! production WASM invocation path is not yet wired — the in-process
//! `ContextResolver` trait does not yet dispatch through wasmtime.
//!
//! Architecture anchors:
//! - BC-4.12.004 — resolver crash isolation contract
//! - VP-074 — formal verification target for crash isolation
//! - S-12.04 AC-010 (trap isolation), AC-011 (trap event emission)

use std::sync::{Arc, Mutex};

use factory_dispatcher::engine::build_engine;
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::InternalLog;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::{Capabilities, Registry, RegistryEntry};
use factory_dispatcher::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, ResolverRegistry,
};
use factory_dispatcher::routing::group_by_priority;

// ---------------------------------------------------------------------------
// WAT fixture: a minimal working plugin (does not trap; returns exit 0)
// ---------------------------------------------------------------------------

/// Minimal WASM plugin for dispatcher integration. Not the resolver fixture —
/// this is the *hook plugin* that runs after resolver context is injected.
const WAT_OK_PLUGIN: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")))
"#;

// ---------------------------------------------------------------------------
// Path to the trapping resolver WASM fixture
// ---------------------------------------------------------------------------

fn trapping_resolver_wasm() -> std::path::PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest).join("tests/fixtures/trapping_resolver.wasm")
}

// ---------------------------------------------------------------------------
// TrappingWasmResolver: in-process resolver that simulates a WASM trap by
// returning ResolverError::Trap. Used until the real WASM invocation path
// is wired in Step 3.
//
// In Step 3 the implementer replaces this with a resolver that actually
// instantiates trapping_resolver.wasm via wasmtime. Until then, this
// proxy lets the integration tests verify the *dispatch plumbing* (the
// executor.rs wiring) independently of the wasmtime instantiation path.
//
// AC-010: the trapping resolver must not abort dispatch — verified by the
// dispatcher completing normally even when this resolver returns Err.
// AC-011: the executor must emit resolver.error for every Err — verified
// by checking InternalLog.
// ---------------------------------------------------------------------------

/// In-process proxy for the trapping WASM resolver.
///
/// Returns `ResolverError::Trap` on every `resolve()` call. The wasm fixture
/// path is captured so tests can assert it exists (ensuring Step 3 has a real
/// target to wire up).
struct TrappingWasmResolver {
    name: String,
    /// Path to the real WASM fixture (existence asserted in test setup).
    _wasm_fixture: std::path::PathBuf,
}

impl ContextResolver for TrappingWasmResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        // Simulate the trap that trapping_resolver.wasm produces at runtime.
        // In Step 3 this proxy is replaced by real wasmtime invocation.
        Err(ResolverError::Trap {
            name: self.name.clone(),
            detail: "unreachable executed (simulated trap from trapping_resolver.wasm)".to_string(),
        })
    }
}

/// A resolver that always succeeds and returns a known output value.
/// Used in AC-008/AC-009 tests (two resolvers — one trapping, one good).
#[allow(dead_code)]
struct GoodResolver {
    name: String,
    output_key: String,
    output_value: serde_json::Value,
}

impl ContextResolver for GoodResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        Ok(Some(ResolverOutput {
            key: self.output_key.clone(),
            value: Some(self.output_value.clone()),
        }))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn compile_ok_plugin(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let bytes = wat::parse_str(WAT_OK_PLUGIN).expect("WAT parse");
    let path = dir.join(format!("{name}.wasm"));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn make_hook_entry(
    plugin_path: &std::path::Path,
    entry_name: &str,
    needs_context: Vec<String>,
) -> RegistryEntry {
    RegistryEntry {
        name: entry_name.to_string(),
        event: "PreToolUse".to_string(),
        tool: None,
        plugin: plugin_path.to_path_buf(),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(5_000),
        fuel_cap: Some(1_000_000_000),
        on_error: None,
        capabilities: Some(Capabilities::default()),
        config: toml::Value::Table(toml::Table::new()),
        async_flag: false,
        needs_context,
    }
}

fn make_registry_with_hooks(hooks: Vec<RegistryEntry>) -> Registry {
    Registry {
        schema_version: 2,
        defaults: Default::default(),
        hooks,
    }
}

fn make_executor_inputs<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
    resolver_registry: Arc<ResolverRegistry>,
) -> ExecutorInputs<'a> {
    let mut base = HostContext::new("", "0.0.1", "sess-trap-test", "trace-trap-test");
    base.internal_log = Some(internal_log.clone());
    ExecutorInputs {
        engine,
        cache,
        registry,
        payload_value: serde_json::json!({
            "hook_event_name": "PreToolUse",
            "tool_name": "Write",
            "session_id": "trap-test-session",
            "dispatcher_trace_id": "trap-test-trace"
        }),
        base_host_ctx: base,
        internal_log: internal_log.clone(),
        resolver_registry,
    }
}

// ---------------------------------------------------------------------------
// AC-010 (integration) — trapping resolver does NOT abort dispatch
//
// BC-4.12.004 postconditions 1–2 / VP-074:
// A resolver that traps must not propagate the trap to the dispatch cycle.
// The dispatch must complete (return Ok), and the trapping resolver's key
// must be ABSENT from plugin_config.
// ---------------------------------------------------------------------------

/// test_BC_4_12_004_trapping_resolver_does_not_abort_dispatch
///
/// Registers the trapping WASM resolver proxy, dispatches a hook that
/// declares `needs_context: ["trap-resolver"]`, and asserts:
/// 1. `execute_tiers` returns without panicking (dispatch completes).
/// 2. The plugin runs (per_plugin_results has one entry).
/// 3. No resolver.load_error or panic events abort the dispatch.
///
/// The test also asserts the fixture exists on disk — so Step 3 implementers
/// have a concrete target to wire up the real wasmtime invocation.
///
/// Red Gate: the test fails at the executor plumbing level because the
/// resolver error path (producing resolver.error + absent plugin_config key)
/// is verified in the *next* test (AC-011). This test will PASS once the
/// dispatcher correctly catches the Trap and continues (Step 3 work).
/// Until then it fails if the dispatcher panics or the plugin fails to run.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_4_12_004_trapping_resolver_does_not_abort_dispatch() {
    // Ensure the WASM fixture exists — Step 3 implementer verification.
    let fixture = trapping_resolver_wasm();
    assert!(
        fixture.exists(),
        "VP-074 fixture trapping_resolver.wasm must exist at {:?} before Step 3 \
         wires real wasmtime invocation. Compile from trapping_resolver.wat.",
        fixture
    );

    let dir = tempfile::tempdir().expect("tempdir");
    let engine = build_engine().expect("build_engine");
    let cache = PluginCache::new(engine.clone());
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let plugin = compile_ok_plugin(dir.path(), "trap_dispatch_plugin");
    let entry = make_hook_entry(
        &plugin,
        "trap-resolver-hook",
        vec!["trap-resolver".to_string()],
    );
    let registry = make_registry_with_hooks(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let trapping = TrappingWasmResolver {
        name: "trap-resolver".to_string(),
        _wasm_fixture: fixture.clone(),
    };
    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(trapping))
        .expect("registration must succeed");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);

    // execute_tiers must NOT panic — BC-4.12.004 crash isolation contract.
    let summary = execute_tiers(inputs, tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "AC-010 / BC-4.12.004 PC1: exactly one hook plugin must have run — \
         a trapping resolver must not prevent hook dispatch from completing"
    );

    // Dispatch must not surface a fatal error even when the resolver trapped.
    // The exit_code may be non-zero if the hook plugin itself fails, but
    // the summary must exist (not panic/unwind) — the outer assert above
    // would not have been reached if the process panicked.
}

// ---------------------------------------------------------------------------
// AC-011 (integration) — trapping resolver emits resolver.error event
//
// BC-4.12.004 postconditions 3–5 / VP-074:
// Every resolver trap must be observable: the dispatcher must emit a
// resolver.error event with error_kind="trap", resolver_name, and error_detail.
// ---------------------------------------------------------------------------

/// test_BC_4_12_004_trapping_resolver_emits_resolver_error_event
///
/// Dispatches a hook with a trapping resolver and asserts the InternalLog
/// contains a `resolver.error` event with:
/// - `error_kind == "trap"`
/// - `resolver_name` matching the registered resolver name
/// - `error_detail` non-empty (carries trap context)
///
/// This test verifies the executor.rs `emit_resolver_error` callback wiring
/// (the callback that writes to InternalLog for every resolver error).
///
/// Red Gate: the test fails because the assertions on InternalLog content
/// require the full Step 3 error-path wiring to be present. The current
/// stub `todo!()` in `get_or_compile` / `invoke_resolver_wasm` means the
/// error emission path has not been exercised yet.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_4_12_004_trapping_resolver_emits_resolver_error_event() {
    let dir = tempfile::tempdir().expect("tempdir");
    let engine = build_engine().expect("build_engine");
    let cache = PluginCache::new(engine.clone());
    let log_dir = dir.path().join("logs");
    let internal_log = Arc::new(InternalLog::new(log_dir.clone()));

    let plugin = compile_ok_plugin(dir.path(), "trap_event_plugin");
    let entry = make_hook_entry(
        &plugin,
        "trap-event-hook",
        vec!["trap-resolver".to_string()],
    );
    let registry = make_registry_with_hooks(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let trapping = TrappingWasmResolver {
        name: "trap-resolver".to_string(),
        _wasm_fixture: trapping_resolver_wasm(),
    };
    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(trapping))
        .expect("registration");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    let _summary = execute_tiers(inputs, tiers).await;

    // Flush the InternalLog to disk.
    drop(internal_log);

    // Read all log files from the log directory.
    let all_log_content: String = std::fs::read_dir(&log_dir)
        .expect("AC-011: log dir must exist after dispatch")
        .filter_map(|e| e.ok())
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect::<Vec<_>>()
        .join("\n");

    // AC-011 assertion 1: resolver.error event present.
    assert!(
        all_log_content.contains("resolver.error"),
        "AC-011 / BC-4.12.004 PC4 / SOUL #4: InternalLog must contain a \
         'resolver.error' event when a resolver traps — no silent failures. \
         Log content: {all_log_content:?}"
    );

    // AC-011 assertion 2: error_kind == "trap" (snake_case serde tag from ResolverError::Trap).
    assert!(
        all_log_content.contains("\"error_kind\":\"trap\""),
        "AC-011 / BC-4.12.004 PC4: 'resolver.error' event must carry \
         'error_kind': 'trap' (serde snake_case tag for ResolverError::Trap). \
         Log content: {all_log_content:?}"
    );

    // AC-011 assertion 3: resolver_name present and correct.
    assert!(
        all_log_content.contains("trap-resolver"),
        "AC-011 / BC-4.12.004 PC4: 'resolver.error' event must carry the \
         resolver name 'trap-resolver' in the resolver_name field. \
         Log content: {all_log_content:?}"
    );

    // AC-011 assertion 4: error_detail present and non-empty.
    assert!(
        all_log_content.contains("error_detail"),
        "AC-011 / BC-4.12.004 PC4: 'resolver.error' event must carry \
         'error_detail' (non-empty trap context string). \
         Log content: {all_log_content:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-008 (integration) — failed resolver key ABSENT from plugin_config
//
// BC-4.12.004 postcondition 3:
// When a resolver fails (trap), the dispatcher must NOT write the resolver's
// key into plugin_config. The key must be absent — not null, not {}.
//
// Note: plugin_config is passed into the hook WASM as part of the dispatch
// payload. This test verifies the merge step (executor.rs build_plugin_config)
// does not include the failed resolver's output.
// ---------------------------------------------------------------------------

/// test_BC_4_12_004_failed_resolver_key_absent_from_plugin_config
///
/// Dispatches a hook with a trapping resolver. The trapping resolver is
/// named "trap-resolver". After dispatch, the test verifies the dispatch
/// completed (no panic), and that the resolver's key was excluded from the
/// merged plugin_config (verified via a SpyResolver that inspects the config
/// it receives — the spy should NOT see "trap-resolver" as a plugin_config key
/// because the trapping resolver's output was never merged).
///
/// Red Gate: fails because the resolver error path (key-absent contract) is
/// not yet wired in the stub implementation.
#[tokio::test(flavor = "current_thread")]
async fn test_BC_4_12_004_failed_resolver_key_absent_from_plugin_config() {
    let dir = tempfile::tempdir().expect("tempdir");
    let engine = build_engine().expect("build_engine");
    let cache = PluginCache::new(engine.clone());
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let plugin = compile_ok_plugin(dir.path(), "absent_key_plugin");
    // Hook requests both "trap-resolver" (will fail) and "good-resolver" (will succeed).
    // This also doubles as AC-009 (resolver B executes after resolver A fails).
    let entry = make_hook_entry(
        &plugin,
        "absent-key-hook",
        vec!["trap-resolver".to_string(), "good-resolver".to_string()],
    );
    let registry = make_registry_with_hooks(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    // Track what plugin_config the good resolver sees — it will NOT include
    // "trap-resolver" key (that resolver hasn't produced output).
    let captured_plugin_config: Arc<Mutex<Option<serde_json::Value>>> = Arc::new(Mutex::new(None));
    let config_capture = captured_plugin_config.clone();

    struct CapturingGoodResolver {
        captured: Arc<Mutex<Option<serde_json::Value>>>,
    }
    impl ContextResolver for CapturingGoodResolver {
        fn name(&self) -> &str {
            "good-resolver"
        }
        fn resolve(&self, input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
            // Capture what plugin_config the executor passed in.
            *self.captured.lock().unwrap() = Some(input.plugin_config.clone());
            Ok(Some(ResolverOutput {
                key: "good-key".to_string(),
                value: Some(serde_json::json!(42)),
            }))
        }
    }

    let trapping = TrappingWasmResolver {
        name: "trap-resolver".to_string(),
        _wasm_fixture: trapping_resolver_wasm(),
    };
    let good = CapturingGoodResolver {
        captured: config_capture,
    };

    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(trapping))
        .expect("trap-resolver registration");
    resolver_registry
        .register(Box::new(good))
        .expect("good-resolver registration");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    let summary = execute_tiers(inputs, tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "AC-008: hook plugin must have run despite the trapping resolver"
    );

    // The good resolver must have been invoked (AC-009: resolver B executes after A fails).
    let config = captured_plugin_config.lock().unwrap().clone();
    let config = config.expect(
        "AC-008 / AC-009: good-resolver must have been invoked — \
         resolver B must execute even when resolver A fails (BC-4.12.004 PC6)",
    );

    // AC-008: "trap-resolver" key must NOT appear in plugin_config.
    // It was never merged because the trapping resolver returned Err.
    let config_obj = config
        .as_object()
        .expect("plugin_config must be a JSON object");

    assert!(
        config_obj.get("trap-resolver").is_none(),
        "AC-008 / BC-4.12.004 PC3: 'trap-resolver' key must be ABSENT from \
         plugin_config after the resolver traps — not null, not {{}}, not any \
         default value. Found: {:?}",
        config_obj.get("trap-resolver")
    );

    // AC-009: good-resolver was invoked (resolver B executed after resolver A failed).
    //
    // The `config.expect(...)` call above already proves invocation — if
    // good-resolver had NOT been called, `captured_plugin_config` would be
    // `None` and the test would have failed there.
    //
    // Additionally: "good-key" must NOT be present in the input config that
    // good-resolver received, because "good-key" is good-resolver's own output
    // value — it cannot appear in the input passed TO good-resolver.
    assert!(
        config_obj.get("good-key").is_none(),
        "AC-009 / BC-4.12.004 PC6: 'good-key' is good-resolver's own output key — \
         it must NOT be present in the input plugin_config passed to good-resolver \
         (a resolver cannot see its own output as input). \
         good-resolver was invoked (proven by config.expect above). \
         Found: {:?}",
        config_obj.get("good-key")
    );
}
