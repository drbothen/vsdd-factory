//! Integration tests: ResolverRegistry wired into executor.rs dispatch (F-001 / AC-002 / AC-003).
//!
//! Verifies that `execute_tiers` and `spawn_async_plugin` both:
//! - AC-002: short-circuit resolver invocation when `needs_context` is empty
//!   (zero overhead — BC-1.13.001 PC3).
//! - AC-003: invoke the resolver and merge its output when `needs_context` is
//!   non-empty (BC-1.13.001 PC4 / BC-4.12.005).
//!
//! Uses inline WAT fixtures + an in-process `ContextResolver` mock; no WASM
//! I/O inspection is required — the resolver's call count is the observable.
//!
//! BC traces: BC-1.13.001 PC3/PC4, BC-4.12.005, AC-002, AC-003.
//! Story: S-12.03 (F-001 adversary finding).

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use factory_dispatcher::engine::build_engine;
use factory_dispatcher::executor::{ExecutorInputs, execute_tiers, spawn_async_plugin};
use factory_dispatcher::host::HostContext;
use factory_dispatcher::internal_log::InternalLog;
use factory_dispatcher::plugin_loader::PluginCache;
use factory_dispatcher::registry::{Capabilities, Registry, RegistryEntry};
use factory_dispatcher::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, ResolverRegistry,
};
use factory_dispatcher::routing::group_by_priority;

// ---------------------------------------------------------------------------
// Minimal WAT fixture: returns immediately (successful exit).
// ---------------------------------------------------------------------------

const WAT_NORMAL: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "_start")))
"#;

// ---------------------------------------------------------------------------
// Counting resolver: records every invocation; returns a fixed output.
// ---------------------------------------------------------------------------

struct CountingResolver {
    call_count: Arc<Mutex<usize>>,
    output_key: String,
}

impl CountingResolver {
    fn new(key: &str) -> (Self, Arc<Mutex<usize>>) {
        let count = Arc::new(Mutex::new(0usize));
        (
            Self {
                call_count: count.clone(),
                output_key: key.to_string(),
            },
            count,
        )
    }
}

impl ContextResolver for CountingResolver {
    fn name(&self) -> &str {
        &self.output_key
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        *self.call_count.lock().unwrap() += 1;
        Ok(Some(ResolverOutput {
            key: self.output_key.clone(),
            value: Some(serde_json::json!({ "injected": true })),
        }))
    }
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn compile_wasm(dir: &std::path::Path, name: &str) -> PathBuf {
    let bytes = wat::parse_str(WAT_NORMAL).expect("WAT parse");
    let path = dir.join(format!("{name}.wasm"));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn make_entry(path: &std::path::Path, name: &str, needs_context: Vec<String>) -> RegistryEntry {
    RegistryEntry {
        name: name.to_string(),
        event: "PreToolUse".to_string(),
        tool: None,
        plugin: path.to_path_buf(),
        priority: Some(100),
        enabled: true,
        timeout_ms: Some(2_000),
        fuel_cap: Some(1_000_000_000),
        on_error: None,
        capabilities: Some(Capabilities::default()),
        config: toml::Value::Table(toml::Table::new()),
        async_flag: false,
        needs_context,
    }
}

fn make_registry(entries: Vec<RegistryEntry>) -> Registry {
    Registry {
        schema_version: 2,
        defaults: Default::default(),
        hooks: entries,
    }
}

fn make_executor_inputs<'a>(
    engine: &'a wasmtime::Engine,
    cache: &'a PluginCache,
    registry: &'a Registry,
    internal_log: &Arc<InternalLog>,
    resolver_registry: Arc<ResolverRegistry>,
) -> ExecutorInputs<'a> {
    let mut base = HostContext::new("", "0.0.1", "sess-resolver-test", "trace-resolver-test");
    base.internal_log = Some(internal_log.clone());
    ExecutorInputs {
        engine,
        cache,
        registry,
        payload_value: serde_json::json!({
            "event_name": "PreToolUse",
            "tool_name": "Write",
            "session_id": "resolver-test-session",
            "dispatcher_trace_id": "resolver-test-trace"
        }),
        base_host_ctx: base,
        internal_log: internal_log.clone(),
        resolver_registry,
    }
}

// ---------------------------------------------------------------------------
// AC-002: zero-overhead short-circuit
// ---------------------------------------------------------------------------

/// AC-002 / BC-1.13.001 PC3:
/// When a hook entry has `needs_context: []`, the resolver registry MUST NOT
/// be consulted — the counting resolver's call count remains zero.
///
/// This test exercises the `needs_context.is_empty()` short-circuit in
/// `build_plugin_config` inside `executor::execute_tiers` (F-001).
#[tokio::test(flavor = "current_thread")]
async fn ac002_execute_tiers_zero_overhead_when_needs_context_empty() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let wasm = compile_wasm(dir.path(), "ok");
    // Hook with empty needs_context — resolver must NOT be called.
    let entry = make_entry(&wasm, "no-context-hook", vec![]);
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let (resolver, call_count) = CountingResolver::new("foo");
    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(resolver))
        .expect("first registration must succeed");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    let summary = execute_tiers(inputs, tiers).await;

    // Plugin must have run without error
    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "AC-002: exactly one plugin must have run"
    );
    assert_eq!(
        summary.exit_code, 0,
        "AC-002: exit_code must be 0 (normal run)"
    );

    // Resolver must NOT have been invoked (zero overhead — BC-1.13.001 PC3).
    assert_eq!(
        *call_count.lock().unwrap(),
        0,
        "AC-002 / BC-1.13.001 PC3: resolver must NOT be invoked when \
         needs_context is empty (zero-overhead short-circuit)"
    );
}

// ---------------------------------------------------------------------------
// AC-003: resolver invoked when needs_context is non-empty
// ---------------------------------------------------------------------------

/// AC-003 / BC-1.13.001 PC4:
/// When a hook entry has `needs_context: ["foo"]` and a resolver named "foo"
/// is registered, `execute_tiers` MUST invoke the resolver exactly once.
///
/// This is the load-bearing integration verification: the counting resolver
/// demonstrates that `build_plugin_config` in `executor.rs` consults the
/// `ResolverRegistry` before splicing `plugin_config` into the payload
/// (F-001 — the executor wiring is active).
#[tokio::test(flavor = "current_thread")]
async fn ac003_execute_tiers_invokes_resolver_when_needs_context_non_empty() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let wasm = compile_wasm(dir.path(), "ok");
    // Hook declares needs_context: ["foo"] — resolver "foo" must be invoked.
    let entry = make_entry(&wasm, "context-hook", vec!["foo".to_string()]);
    let registry = make_registry(vec![entry]);
    let matched: Vec<&RegistryEntry> = registry.hooks.iter().collect();
    let tiers = group_by_priority(&registry, matched);

    let (resolver, call_count) = CountingResolver::new("foo");
    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(resolver))
        .expect("first registration must succeed");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    let summary = execute_tiers(inputs, tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "AC-003: exactly one plugin must have run"
    );
    assert_eq!(summary.exit_code, 0, "AC-003: exit_code must be 0");

    // Resolver MUST have been invoked exactly once (BC-1.13.001 PC4).
    assert_eq!(
        *call_count.lock().unwrap(),
        1,
        "AC-003 / BC-1.13.001 PC4: resolver 'foo' must be invoked exactly once \
         when needs_context = [\"foo\"] (executor.rs wiring — F-001)"
    );
}

// ---------------------------------------------------------------------------
// AC-002 via spawn_async_plugin: zero-overhead on async path
// ---------------------------------------------------------------------------

/// AC-002 (async path) / BC-1.13.001 PC3:
/// `spawn_async_plugin` also uses `build_plugin_config` — verify the
/// short-circuit applies equally to the async dispatch path.
#[tokio::test(flavor = "current_thread")]
async fn ac002_spawn_async_plugin_zero_overhead_when_needs_context_empty() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = Arc::new(PluginCache::new(engine.clone()));
    let internal_log = Arc::new(InternalLog::new(dir.path().join("logs")));

    let wasm = compile_wasm(dir.path(), "ok_async");
    let mut entry = make_entry(&wasm, "no-context-async", vec![]);
    entry.async_flag = true;
    let registry = make_registry(vec![entry.clone()]);

    let (resolver, call_count) = CountingResolver::new("bar");
    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(resolver))
        .expect("first registration must succeed");
    let resolver_registry = Arc::new(resolver_registry);

    let mut base_ctx = HostContext::new("", "0.0.1", "sess-async-resolver", "trace-async-resolver");
    base_ctx.internal_log = Some(internal_log.clone());

    let handle = spawn_async_plugin(
        engine.clone(),
        cache.clone(),
        registry.defaults.clone(),
        entry,
        serde_json::json!({
            "event_name": "PreToolUse",
            "session_id": "async-resolver-session",
            "dispatcher_trace_id": "async-resolver-trace"
        }),
        base_ctx,
        internal_log.clone(),
        resolver_registry,
    );

    let outcome = tokio::time::timeout(std::time::Duration::from_secs(10), handle)
        .await
        .expect("JoinHandle join must not timeout")
        .expect("JoinHandle must not panic");

    assert!(
        matches!(
            outcome.result,
            factory_dispatcher::invoke::PluginResult::Ok { .. }
        ),
        "AC-002 (async): plugin must complete successfully"
    );

    // Resolver must NOT have been called (needs_context is empty).
    assert_eq!(
        *call_count.lock().unwrap(),
        0,
        "AC-002 (async path) / BC-1.13.001 PC3: resolver must NOT be invoked \
         when needs_context is empty on the async dispatch path"
    );
}

// ---------------------------------------------------------------------------
// F-P2-007 (test 3): ErroringResolver causes resolver.error event in InternalLog
// ---------------------------------------------------------------------------

/// An in-process resolver that always returns `Err(ResolverError::Crashed)`.
struct ErroringResolver {
    key: String,
}

impl ContextResolver for ErroringResolver {
    fn name(&self) -> &str {
        &self.key
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        Err(ResolverError::Crashed {
            name: self.key.clone(),
            detail: "injected test failure".to_string(),
        })
    }
}

// ---------------------------------------------------------------------------
// F-P3-003: resolver.not_found event written to InternalLog when resolver missing
// ---------------------------------------------------------------------------

/// F-P3-003 / BC-4.12.005 PC4 / SOUL #4:
/// When a hook declares `needs_context: ["unknown"]` and NO resolver named
/// "unknown" is registered, `execute_tiers` must:
/// 1. Complete without panicking (dispatch continues — BC-4.12.005 INV3).
/// 2. Write a `resolver.not_found` event to InternalLog, with `resolver_name`
///    equal to "unknown" and the correct hook name.
#[tokio::test(flavor = "current_thread")]
async fn ac005_resolver_not_found_event_appears_in_internal_log() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let log_dir = dir.path().join("logs");
    let internal_log = Arc::new(InternalLog::new(log_dir.clone()));

    let wasm = compile_wasm(dir.path(), "ok_not_found");
    // Hook declares needs_context: ["unknown"] — no resolver named "unknown" exists.
    let entry = make_entry(&wasm, "not-found-hook", vec!["unknown".to_string()]);
    let registry = make_registry(vec![entry]);
    let matched: Vec<&factory_dispatcher::registry::RegistryEntry> =
        registry.hooks.iter().collect();
    let tiers = factory_dispatcher::routing::group_by_priority(&registry, matched);

    // Register zero resolvers — "unknown" will not be found.
    let resolver_registry = Arc::new(ResolverRegistry::new());

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    // execute_tiers must NOT panic — dispatch continues even with missing resolver.
    let summary = execute_tiers(inputs, tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "F-P3-003: exactly one plugin must have run despite missing resolver"
    );

    // Flush InternalLog and verify resolver.not_found event is present.
    drop(internal_log);

    let all_log_content: String = std::fs::read_dir(&log_dir)
        .expect("log dir must exist after dispatch")
        .filter_map(|e| e.ok())
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        all_log_content.contains("resolver.not_found"),
        "F-P3-003 / SOUL #4: InternalLog must contain a 'resolver.not_found' event \
         when needs_context names a resolver that is not registered. \
         Log content: {all_log_content:?}"
    );
    assert!(
        all_log_content.contains("unknown"),
        "F-P3-003: 'resolver.not_found' event must carry resolver_name == 'unknown'. \
         Log content: {all_log_content:?}"
    );
    assert!(
        all_log_content.contains("not-found-hook"),
        "F-P3-003: 'resolver.not_found' event must carry the correct hook_name. \
         Log content: {all_log_content:?}"
    );
}

// ---------------------------------------------------------------------------
// F-P2-007 (test 3): ErroringResolver causes resolver.error event in InternalLog
// ---------------------------------------------------------------------------

/// F-P2-007 (integration test) / SOUL #4:
/// When a hook declares `needs_context: ["boom"]` and the resolver "boom"
/// returns `Err(ResolverError::Crashed)`, `execute_tiers` must:
/// 1. Complete without panicking (dispatch continues — BC-4.12.005 INV3).
/// 2. Write a `resolver.error` event to InternalLog (no silent failures).
///
/// Verifies that executor.rs wires the error callback correctly in production.
#[tokio::test(flavor = "current_thread")]
async fn f_p2_007_erroring_resolver_causes_resolver_error_event_in_internal_log() {
    let dir = tempfile::tempdir().unwrap();
    let engine = build_engine().unwrap();
    let cache = PluginCache::new(engine.clone());
    let log_dir = dir.path().join("logs");
    let internal_log = Arc::new(InternalLog::new(log_dir.clone()));

    let wasm = compile_wasm(dir.path(), "ok_boom");
    // Hook declares needs_context: ["boom"] — resolver "boom" will error.
    let entry = make_entry(&wasm, "boom-hook", vec!["boom".to_string()]);
    let registry = make_registry(vec![entry]);
    let matched: Vec<&factory_dispatcher::registry::RegistryEntry> =
        registry.hooks.iter().collect();
    let tiers = factory_dispatcher::routing::group_by_priority(&registry, matched);

    let mut resolver_registry = ResolverRegistry::new();
    resolver_registry
        .register(Box::new(ErroringResolver {
            key: "boom".to_string(),
        }))
        .expect("first registration must succeed");
    let resolver_registry = Arc::new(resolver_registry);

    let inputs = make_executor_inputs(&engine, &cache, &registry, &internal_log, resolver_registry);
    // execute_tiers must NOT panic — dispatch continues even with erroring resolver.
    let summary = execute_tiers(inputs, tiers).await;

    assert_eq!(
        summary.per_plugin_results.len(),
        1,
        "F-P2-007: exactly one plugin must have run despite erroring resolver"
    );

    // Flush InternalLog and verify resolver.error event is present.
    drop(internal_log);

    // Read the NDJSON log file and look for a resolver.error event.
    let log_entries: Vec<String> = std::fs::read_dir(&log_dir)
        .expect("log dir must exist")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| x == "ndjson" || x == "jsonl" || x == "log")
                .unwrap_or(false)
        })
        .flat_map(|e| {
            std::fs::read_to_string(e.path())
                .unwrap_or_default()
                .lines()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect();

    // Also check any file in log_dir (InternalLog may use different extension).
    let all_log_content: String = std::fs::read_dir(&log_dir)
        .expect("log dir must exist after dispatch")
        .filter_map(|e| e.ok())
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect::<Vec<_>>()
        .join("\n");

    let _ = log_entries; // checked via all_log_content below
    assert!(
        all_log_content.contains("resolver.error"),
        "F-P2-007 / SOUL #4: InternalLog must contain a 'resolver.error' event \
         when a registered resolver returns Err — no silent failures. \
         Log content: {all_log_content:?}"
    );
}
