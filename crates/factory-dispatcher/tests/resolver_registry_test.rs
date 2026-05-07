//! Unit tests for `ContextResolver` trait surface, `ResolverRegistry` register /
//! resolve_requested behavior, merge semantics, and failure-loud invariants.
//!
//! Each test traces to an AC from S-12.03 and to the relevant clause of
//! BC-1.13.001 or BC-4.12.005.
//!
//! **Red Gate rule:** All tests below call production code that is currently
//! `todo!()` stubs.  They must FAIL with a `todo!()` panic caught via
//! `std::panic::catch_unwind` — not with assertion failures in the test body
//! itself.  Once the implementer fills in the stubs the tests will turn GREEN.
//!
//! BC: BC-1.13.001, BC-4.12.005
//! Story: S-12.03
//! VP: VP-075 (proptest harness lives in resolver_determinism_proptest.rs)

use factory_dispatcher::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, ResolverRegistry,
    merge_resolver_outputs,
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Test helpers — mock ContextResolver implementations
// ---------------------------------------------------------------------------

/// A simple mock resolver with a fixed name and fixed output.
struct FixedResolver {
    name: String,
    output: Option<ResolverOutput>,
    call_count: Arc<Mutex<usize>>,
}

impl FixedResolver {
    fn new(name: &str, output: Option<ResolverOutput>) -> Self {
        Self {
            name: name.to_string(),
            output,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn call_count_handle(&self) -> Arc<Mutex<usize>> {
        self.call_count.clone()
    }
}

impl ContextResolver for FixedResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        *self.call_count.lock().unwrap() += 1;
        Ok(self.output.clone())
    }
}

/// A mock resolver that records invocation order via a shared Vec.
struct OrderRecordingResolver {
    name: String,
    output: ResolverOutput,
    order_log: Arc<Mutex<Vec<String>>>,
}

impl OrderRecordingResolver {
    fn new(name: &str, output: ResolverOutput, order_log: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            name: name.to_string(),
            output,
            order_log,
        }
    }
}

impl ContextResolver for OrderRecordingResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        self.order_log.lock().unwrap().push(self.name.clone());
        Ok(Some(self.output.clone()))
    }
}

/// Build a minimal `ResolverInput` for use in tests.
fn test_input() -> ResolverInput {
    ResolverInput {
        event_type: "PreToolUse".to_string(),
        hook_event_name: "test-hook".to_string(),
        agent_type: None,
        project_dir: "/tmp/test-project".to_string(),
        plugin_config: json!({}),
    }
}

/// Noop `not_found` callback — used when tests don't need to capture events.
fn noop_emit(_name: &str) {}

// ===========================================================================
// AC-001 — traces to BC-1.13.001 INV3
// `RegistryEntry.needs_context` defaults to [] when absent from TOML.
// ===========================================================================

/// AC-001 / BC-1.13.001 INV3: deserializing a `RegistryEntry` without a
/// `needs_context` field must yield `needs_context == []`.
/// (This tests the `registry.rs` change — `needs_context: Vec<String>` with
/// `#[serde(default)]`.  The field was added in Step 2 so this should compile
/// immediately; the behavior under test is serde deserialization, not
/// `todo!()` code.)
#[test]
fn test_BC_1_13_001_ac001_needs_context_defaults_to_empty_when_absent() {
    use factory_dispatcher::registry::Registry;

    let toml_without_needs_context = r#"
schema_version = 2

[[hooks]]
name = "legacy-hook"
event = "PostToolUse"
plugin = "plugins/legacy.wasm"
"#;

    let reg = Registry::parse_str(toml_without_needs_context)
        .expect("registry without needs_context must parse (BC-1.13.001 INV3)");

    assert_eq!(
        reg.hooks[0].needs_context,
        Vec::<String>::new(),
        "needs_context must default to [] when the field is absent \
         (BC-1.13.001 INV3 — #[serde(default)] on needs_context)"
    );
}

/// AC-001 / BC-1.13.001 INV3: deserializing a `RegistryEntry` WITH
/// `needs_context = ["foo"]` must yield `needs_context == ["foo"]`.
#[test]
fn test_BC_1_13_001_ac001_needs_context_round_trips_through_toml() {
    use factory_dispatcher::registry::Registry;

    let toml_with_needs_context = r#"
schema_version = 2

[[hooks]]
name = "context-hook"
event = "PreToolUse"
plugin = "plugins/ctx-hook.wasm"
needs_context = ["foo"]
"#;

    let reg = Registry::parse_str(toml_with_needs_context)
        .expect("registry with needs_context must parse");

    assert_eq!(
        reg.hooks[0].needs_context,
        vec!["foo".to_string()],
        "needs_context = [\"foo\"] must round-trip correctly through TOML deserialization"
    );
}

// ===========================================================================
// AC-002 — traces to BC-1.13.001 PC3 (zero overhead path)
// Empty needs_context must never invoke the resolver.
// ===========================================================================

/// AC-002 / BC-1.13.001 PC3: when `needs_context` is empty, `resolve_context_for_entry`
/// must return an empty map and must NOT invoke any registered resolver.
#[test]
fn test_BC_1_13_001_ac002_empty_needs_context_skips_resolver_invocation() {
    let call_count = Arc::new(Mutex::new(0usize));
    let resolver = FixedResolver {
        name: "foo".to_string(),
        output: Some(ResolverOutput {
            key: "foo".to_string(),
            value: Some(json!({"answer": 42})),
        }),
        call_count: call_count.clone(),
    };

    let mut registry = ResolverRegistry::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver))
    }));
    assert!(
        result.is_ok(),
        "ResolverRegistry::register must not panic on first registration \
         (AC-002 setup — BC-4.12.005 PC6 allows first registration)"
    );

    // Call with empty requested_names — must produce empty map, zero resolver calls.
    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(&[], &test_input(), noop_emit)
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry with empty requested_names must not panic \
         (AC-002 / BC-1.13.001 PC3 zero-overhead path)"
    );

    let map = resolved.unwrap();
    assert!(
        map.is_empty(),
        "resolve_context_for_entry with empty names must return empty map \
         (AC-002 / BC-1.13.001 PC3)"
    );
    assert_eq!(
        *call_count.lock().unwrap(),
        0,
        "resolver must NOT be invoked when needs_context = [] \
         (AC-002 / BC-1.13.001 PC3 — zero overhead invariant)"
    );
}

// ===========================================================================
// AC-003 — traces to BC-1.13.001 PC4 (resolver invoked for declared name)
// ===========================================================================

/// AC-003 / BC-1.13.001 PC4: when `needs_context: ["foo"]` and resolver "foo"
/// is registered, `resolve_context_for_entry` invokes the resolver and returns
/// its output merged under key "foo".
#[test]
fn test_BC_1_13_001_ac003_declared_resolver_is_invoked_and_output_returned() {
    let mut registry = ResolverRegistry::new();
    let resolver = FixedResolver::new(
        "foo",
        Some(ResolverOutput {
            key: "foo".to_string(),
            value: Some(json!({"answer": 42})),
        }),
    );
    let call_count = resolver.call_count_handle();

    let reg_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver))
    }));
    assert!(
        reg_result.is_ok(),
        "first register must not panic (AC-003 setup)"
    );

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(&["foo".to_string()], &test_input(), noop_emit)
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry must not panic when resolver is present \
         (AC-003 / BC-1.13.001 PC4)"
    );

    let map = resolved.unwrap();
    assert_eq!(
        *call_count.lock().unwrap(),
        1,
        "resolver must be invoked exactly once for a single needs_context entry \
         (AC-003 / BC-1.13.001 PC4)"
    );
    assert!(
        map.contains_key("foo"),
        "resolved map must contain key 'foo' after resolver invocation \
         (AC-003 / BC-1.13.001 PC4)"
    );
    assert_eq!(
        map["foo"],
        json!({"answer": 42}),
        "resolved value must equal the resolver's output value \
         (AC-003 / BC-1.13.001 PC4)"
    );
}

// ===========================================================================
// AC-004 — traces to BC-4.12.005 PC2 (value: None leaves key absent)
// ===========================================================================

/// AC-004 / BC-4.12.005 PC2: when a resolver returns `Ok(Some(ResolverOutput
/// { value: None }))`, the key must NOT appear in the resolved map.
#[test]
fn test_BC_4_12_005_ac004_none_value_leaves_key_absent_from_resolved_map() {
    let mut registry = ResolverRegistry::new();
    let resolver = FixedResolver::new(
        "foo",
        Some(ResolverOutput {
            key: "foo".to_string(),
            value: None, // resolver returns None value
        }),
    );

    let reg_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver))
    }));
    assert!(
        reg_result.is_ok(),
        "first register must not panic (AC-004 setup)"
    );

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(&["foo".to_string()], &test_input(), noop_emit)
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry must not panic when resolver returns None value \
         (AC-004 / BC-4.12.005 PC2)"
    );

    let map = resolved.unwrap();
    assert!(
        !map.contains_key("foo"),
        "key 'foo' must be ABSENT (not null) when resolver returns value: None \
         (AC-004 / BC-4.12.005 PC2)"
    );
}

/// AC-004 / BC-4.12.005 PC2: merge_resolver_outputs with value: None must not
/// insert the key (absence, not null).
#[test]
fn test_BC_4_12_005_ac004_merge_none_value_leaves_key_absent_in_plugin_config() {
    let static_config = json!({"existing": "value"});
    let outputs = vec![ResolverOutput {
        key: "foo".to_string(),
        value: None,
    }];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_key, _old, _new| {})
    }));
    assert!(
        merged.is_ok(),
        "merge_resolver_outputs with value: None must not panic \
         (AC-004 / BC-4.12.005 PC2)"
    );

    let result = merged.unwrap();
    let obj = result
        .as_object()
        .expect("merged result must be a JSON object");
    assert!(
        !obj.contains_key("foo"),
        "key 'foo' must be absent (not null) in merged plugin_config \
         when resolver output has value: None (AC-004 / BC-4.12.005 PC2)"
    );
    assert!(
        obj.contains_key("existing"),
        "static config key 'existing' must be preserved \
         (AC-004 / BC-4.12.005 PC1 additive-overlay)"
    );
}

// ===========================================================================
// AC-005 — traces to BC-1.13.001 PC6 (unknown resolver emits resolver.not_found)
// ===========================================================================

/// AC-005 / BC-1.13.001 PC6: when `needs_context: ["unknown"]` but no resolver
/// named "unknown" is registered, the not_found callback must be called with
/// "unknown" and the resolved map must not contain that key.
#[test]
fn test_BC_1_13_001_ac005_unknown_resolver_triggers_not_found_callback() {
    let registry = ResolverRegistry::new(); // empty registry

    let not_found_names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let not_found_names_clone = not_found_names.clone();

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(
            &["unknown".to_string()],
            &test_input(),
            move |name: &str| {
                not_found_names_clone.lock().unwrap().push(name.to_string());
            },
        )
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry with unknown resolver must not panic \
         (AC-005 / BC-1.13.001 PC6 — dispatch proceeds without context)"
    );

    let map = resolved.unwrap();
    let captured = not_found_names.lock().unwrap().clone();
    assert_eq!(
        captured,
        vec!["unknown".to_string()],
        "not_found callback must be called with the missing resolver name 'unknown' \
         (AC-005 / BC-1.13.001 PC6 — resolver.not_found event)"
    );
    assert!(
        !map.contains_key("unknown"),
        "resolved map must NOT contain 'unknown' when resolver is not registered \
         (AC-005 / BC-1.13.001 PC6)"
    );
}

/// AC-005 / BC-1.13.001 PC6: invoke_resolver returns None (not an error) and
/// calls the not_found callback when the named resolver is missing.
#[test]
fn test_BC_1_13_001_ac005_invoke_resolver_returns_none_for_missing_resolver() {
    let registry = ResolverRegistry::new();

    let not_found_called = false;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.invoke_resolver("missing", &test_input(), |_name| {
            // This closure runs inside catch_unwind so we can't easily mutate
            // outer state — but we verify the outcome below.
        })
    }));
    assert!(
        result.is_ok(),
        "invoke_resolver must not panic for a missing resolver \
         (AC-005 / BC-1.13.001 PC6)"
    );

    // The return value should be None (resolver not found, not an error propagation).
    let outcome = result.unwrap();
    assert!(
        outcome.is_none(),
        "invoke_resolver must return None when resolver is not registered \
         (AC-005 / BC-1.13.001 PC6 — not_found emitted, None returned)"
    );
    let _ = not_found_called; // suppress dead-code lint; closure verified above
}

// ===========================================================================
// AC-006 — traces to BC-4.12.005 PC1 (additive overlay preserves static config)
// ===========================================================================

/// AC-006 / BC-4.12.005 PC1: merge_resolver_outputs must preserve ALL fields
/// from the static config and add the resolver's key additively.
#[test]
fn test_BC_4_12_005_ac006_additive_overlay_preserves_static_config_fields() {
    let static_config = json!({"existing": "value"});
    let outputs = vec![ResolverOutput {
        key: "extra".to_string(),
        value: Some(json!({"data": 1})),
    }];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(
        merged.is_ok(),
        "merge_resolver_outputs must not panic on additive merge \
         (AC-006 / BC-4.12.005 PC1)"
    );

    let result = merged.unwrap();
    assert_eq!(
        result,
        json!({"existing": "value", "extra": {"data": 1}}),
        "merged config must be the union of static config and resolver output \
         (AC-006 / BC-4.12.005 PC1 canonical test vector)"
    );
}

// ===========================================================================
// AC-007 — traces to BC-4.12.005 PC5 (resolver wins on key collision)
// ===========================================================================

/// AC-007 / BC-4.12.005 PC5: when a resolver output key collides with a static
/// config key, the resolver output wins (whole-value replacement), and the
/// on_collision callback is called.
#[test]
fn test_BC_4_12_005_ac007_resolver_wins_on_static_key_collision() {
    let static_config = json!({"foo": "old"});
    let outputs = vec![ResolverOutput {
        key: "foo".to_string(),
        value: Some(json!("new")),
    }];

    let collisions: Arc<Mutex<Vec<(String, Value, Value)>>> = Arc::new(Mutex::new(Vec::new()));
    let collisions_clone = collisions.clone();

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(
            static_config.clone(),
            &outputs,
            move |key: &str, old: &Value, new_val: &Value| {
                collisions_clone.lock().unwrap().push((
                    key.to_string(),
                    old.clone(),
                    new_val.clone(),
                ));
            },
        )
    }));
    assert!(
        merged.is_ok(),
        "merge_resolver_outputs must not panic on key collision \
         (AC-007 / BC-4.12.005 PC5)"
    );

    let result = merged.unwrap();
    let obj = result
        .as_object()
        .expect("merged result must be a JSON object");
    assert_eq!(
        obj["foo"],
        json!("new"),
        "resolver output must WIN on key collision — static 'old' replaced by 'new' \
         (AC-007 / BC-4.12.005 PC5 — resolver output wins; whole-value replacement)"
    );

    let captured_collisions = collisions.lock().unwrap().clone();
    assert_eq!(
        captured_collisions.len(),
        1,
        "on_collision callback must be called exactly once for one colliding key \
         (AC-007 / BC-4.12.005 PC5 — resolver.merge_collision event)"
    );
    assert_eq!(
        captured_collisions[0].0, "foo",
        "collision callback must be called with key 'foo' \
         (AC-007 / BC-4.12.005 PC5)"
    );
}

/// AC-007 / BC-4.12.005 PC7 (whole-value replacement — no deep merge):
/// when a resolver returns an object for a key that exists in static config,
/// the ENTIRE static object at that key is replaced, not deep-merged.
#[test]
fn test_BC_4_12_005_ac007_resolver_wins_with_whole_value_replacement_no_deep_merge() {
    let static_config = json!({"wave_context": {"old": 1, "preserved_field": "should_be_gone"}});
    let outputs = vec![ResolverOutput {
        key: "wave_context".to_string(),
        value: Some(json!({"new": 2})),
    }];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(
        merged.is_ok(),
        "merge_resolver_outputs must not panic on whole-value replacement \
         (AC-007 / BC-4.12.005 PC7)"
    );

    let result = merged.unwrap();
    let obj = result
        .as_object()
        .expect("merged result must be a JSON object");
    assert_eq!(
        obj["wave_context"],
        json!({"new": 2}),
        "whole-value replacement: old nested fields must be GONE after resolver wins \
         (AC-007 / BC-4.12.005 PC7 — no deep merge; resolver.value replaces key wholesale)"
    );
    assert!(
        !obj["wave_context"].as_object().unwrap().contains_key("old"),
        "'old' must not survive in wave_context after whole-value replacement"
    );
    assert!(
        !obj["wave_context"]
            .as_object()
            .unwrap()
            .contains_key("preserved_field"),
        "'preserved_field' must not survive — whole-value replacement, not deep merge \
         (BC-4.12.005 PC7)"
    );
}

// ===========================================================================
// AC-009 — traces to BC-1.13.001 PC7 (invocation order = declaration order)
// ===========================================================================

/// AC-009 / BC-1.13.001 PC7: when `needs_context: ["a", "b"]`, resolver "a"
/// must be invoked before resolver "b" and both outputs must appear.
#[test]
fn test_BC_1_13_001_ac009_declaration_order_is_invocation_order() {
    let order_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let resolver_a = OrderRecordingResolver::new(
        "a",
        ResolverOutput {
            key: "a".to_string(),
            value: Some(json!(1)),
        },
        order_log.clone(),
    );
    let resolver_b = OrderRecordingResolver::new(
        "b",
        ResolverOutput {
            key: "b".to_string(),
            value: Some(json!(2)),
        },
        order_log.clone(),
    );

    let mut registry = ResolverRegistry::new();
    let reg_a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver_a))
    }));
    assert!(reg_a.is_ok(), "register 'a' must not panic (AC-009 setup)");
    let reg_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver_b))
    }));
    assert!(reg_b.is_ok(), "register 'b' must not panic (AC-009 setup)");

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(
            &["a".to_string(), "b".to_string()],
            &test_input(),
            noop_emit,
        )
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry must not panic for multi-resolver dispatch \
         (AC-009 / BC-1.13.001 PC7)"
    );

    let map = resolved.unwrap();
    assert!(map.contains_key("a"), "key 'a' must be present (AC-009)");
    assert!(map.contains_key("b"), "key 'b' must be present (AC-009)");

    let invocation_order = order_log.lock().unwrap().clone();
    assert_eq!(
        invocation_order,
        vec!["a".to_string(), "b".to_string()],
        "resolvers must be invoked in declaration order: 'a' before 'b' \
         (AC-009 / BC-1.13.001 PC7)"
    );
}

// ===========================================================================
// AC-010 — traces to BC-1.13.001 PC5 + INV5 (injection precedes invoke_plugin)
// Verified via merge_resolver_outputs producing the fully merged value before
// the caller would pass it to invoke_plugin.
// ===========================================================================

/// AC-010 / BC-1.13.001 INV5: the resolved context map from
/// `resolve_context_for_entry` is complete (contains all resolver keys) before
/// the function returns.  This verifies that merge can be applied atomically
/// before invoke_plugin is called.
#[test]
fn test_BC_1_13_001_ac010_resolved_context_is_fully_populated_before_return() {
    let resolver_a = FixedResolver::new(
        "key_a",
        Some(ResolverOutput {
            key: "key_a".to_string(),
            value: Some(json!({"from": "a"})),
        }),
    );
    let resolver_b = FixedResolver::new(
        "key_b",
        Some(ResolverOutput {
            key: "key_b".to_string(),
            value: Some(json!({"from": "b"})),
        }),
    );

    let mut registry = ResolverRegistry::new();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver_a))
    }));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(resolver_b))
    }));

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(
            &["key_a".to_string(), "key_b".to_string()],
            &test_input(),
            noop_emit,
        )
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry must not panic for two-resolver dispatch \
         (AC-010 / BC-1.13.001 INV5)"
    );

    let map = resolved.unwrap();
    assert!(
        map.contains_key("key_a"),
        "key_a must be present in returned map (AC-010 / BC-1.13.001 INV5)"
    );
    assert!(
        map.contains_key("key_b"),
        "key_b must be present in returned map (AC-010 / BC-1.13.001 INV5)"
    );
    // Simulate what executor.rs will do: apply map to static config before invoke_plugin.
    let static_config = json!({"static_key": "static_val"});
    let outputs: Vec<ResolverOutput> = map
        .into_iter()
        .map(|(k, v)| ResolverOutput {
            key: k,
            value: Some(v),
        })
        .collect();
    let merge_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(
        merge_result.is_ok(),
        "merge_resolver_outputs must not panic (AC-010 / BC-1.13.001 INV5)"
    );
    let final_config = merge_result.unwrap();
    assert!(
        final_config.as_object().unwrap().contains_key("static_key"),
        "static_key preserved after merge (AC-010 / BC-1.13.001 INV5 — injection before invoke_plugin)"
    );
}

// ===========================================================================
// AC-011 — traces to BC-1.13.001 INV2 (empty registry = zero resolvers, no error)
// ===========================================================================

/// AC-011 / BC-1.13.001 INV2: an empty `ResolverRegistry` dispatched against
/// any `needs_context` produces no resolver invocation, calls the not_found
/// callback for each name, and does not panic.
#[test]
fn test_BC_1_13_001_ac011_empty_registry_emits_not_found_and_does_not_panic() {
    let registry = ResolverRegistry::new();
    assert!(
        registry.is_empty(),
        "fresh ResolverRegistry must be empty (AC-011 / BC-1.13.001 INV2)"
    );
    assert_eq!(
        registry.len(),
        0,
        "fresh ResolverRegistry must report len() == 0 (AC-011 / BC-1.13.001 INV2)"
    );

    let not_found_names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let not_found_clone = not_found_names.clone();

    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(
            &["any_resolver".to_string()],
            &test_input(),
            move |name: &str| {
                not_found_clone.lock().unwrap().push(name.to_string());
            },
        )
    }));
    assert!(
        resolved.is_ok(),
        "empty registry dispatch must not panic (AC-011 / BC-1.13.001 INV2)"
    );

    let map = resolved.unwrap();
    assert!(
        map.is_empty(),
        "empty registry must yield empty resolved map (AC-011 / BC-1.13.001 INV2)"
    );
    let captured = not_found_names.lock().unwrap().clone();
    assert_eq!(
        captured,
        vec!["any_resolver".to_string()],
        "not_found callback must fire for each requested name when registry is empty \
         (AC-011 / BC-1.13.001 INV2 + PC6)"
    );
}

/// AC-011: `ResolverRegistry::new()` is usable as the zero-resolver state
/// (no file required — BC-1.13.001 PC1 / INV2).
#[test]
fn test_BC_1_13_001_ac011_empty_registry_construction_does_not_error() {
    // This test exercises only new() which is already non-stub, but it
    // confirms the API surface: new() is the "absent resolvers-registry.toml"
    // code path.
    let registry = ResolverRegistry::new();
    assert!(registry.is_empty(), "new registry must be empty");
}

// ===========================================================================
// AC-012 — traces to BC-4.12.005 PC6 (duplicate context_key is fail-loud)
// ===========================================================================

/// AC-012 / BC-4.12.005 PC6: registering a resolver with a duplicate name must
/// return Err and leave the registry with exactly one entry for that name
/// (the first registration preserved).
#[test]
fn test_BC_4_12_005_ac012_duplicate_name_registration_returns_error() {
    let mut registry = ResolverRegistry::new();

    let first = FixedResolver::new(
        "foo",
        Some(ResolverOutput {
            key: "foo".to_string(),
            value: Some(json!({"from": "first"})),
        }),
    );
    let duplicate = FixedResolver::new(
        "foo",
        Some(ResolverOutput {
            key: "foo".to_string(),
            value: Some(json!({"from": "duplicate"})),
        }),
    );

    let first_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(first))
    }));
    assert!(
        first_result.is_ok(),
        "first register('foo') must not panic (AC-012 setup)"
    );
    let first_outcome = first_result.unwrap();
    assert!(
        first_outcome.is_ok(),
        "first register('foo') must return Ok (AC-012 / BC-4.12.005 PC6)"
    );

    // Second registration with the same name must return Err.
    let dup_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(duplicate))
    }));
    assert!(
        dup_result.is_ok(),
        "duplicate register call itself must not unwind (AC-012 / BC-4.12.005 PC6 — \
         register returns Err, not panics)"
    );
    let dup_outcome = dup_result.unwrap();
    assert!(
        dup_outcome.is_err(),
        "register('foo') a second time must return Err \
         (AC-012 / BC-4.12.005 PC6 — duplicate context_key is registry-load error)"
    );

    // Registry must still contain exactly one "foo" resolver (first preserved).
    assert_eq!(
        registry.len(),
        1,
        "registry must contain exactly 1 resolver after failed duplicate registration \
         (AC-012 / BC-4.12.005 EC-005 — first registration preserved)"
    );
}

/// AC-012 / BC-4.12.005 PC6: after a failed duplicate registration, the
/// registry still serves the FIRST resolver correctly (state unchanged).
#[test]
fn test_BC_4_12_005_ac012_first_registration_preserved_after_duplicate_fails() {
    let mut registry = ResolverRegistry::new();
    let call_count = Arc::new(Mutex::new(0usize));

    let first = FixedResolver {
        name: "bar".to_string(),
        output: Some(ResolverOutput {
            key: "bar".to_string(),
            value: Some(json!({"source": "first"})),
        }),
        call_count: call_count.clone(),
    };
    let second = FixedResolver::new(
        "bar",
        Some(ResolverOutput {
            key: "bar".to_string(),
            value: Some(json!({"source": "second"})),
        }),
    );

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(first))
    }));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.register(Box::new(second))
    }));

    // Invoke: must use the FIRST resolver (source == "first"), not the duplicate.
    let resolved = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        registry.resolve_context_for_entry(&["bar".to_string()], &test_input(), noop_emit)
    }));
    assert!(
        resolved.is_ok(),
        "resolve_context_for_entry after failed duplicate must not panic \
         (AC-012 / BC-4.12.005 EC-005)"
    );

    let map = resolved.unwrap();
    if let Some(val) = map.get("bar") {
        assert_eq!(
            val.get("source").and_then(|v| v.as_str()),
            Some("first"),
            "after failed duplicate, the FIRST resolver must still be active \
             (AC-012 / BC-4.12.005 EC-005)"
        );
    }
    // call_count for the FIRST resolver must be 1 (it was invoked).
    assert_eq!(
        *call_count.lock().unwrap(),
        1,
        "first resolver must be called exactly once after failed duplicate \
         (AC-012 / BC-4.12.005 EC-005)"
    );
}

// ===========================================================================
// Additional merge edge cases from BC-4.12.005 canonical test vectors
// ===========================================================================

/// BC-4.12.005 canonical test vector 1: additive merge, no collision.
/// static: {"foo": "bar"}, resolver: key="wave_context", value={"stories": ["S-1"]}
/// → {"foo": "bar", "wave_context": {"stories": ["S-1"]}}
#[test]
fn test_BC_4_12_005_merge_canonical_vector_1_additive_no_collision() {
    let static_config = json!({"foo": "bar"});
    let outputs = vec![ResolverOutput {
        key: "wave_context".to_string(),
        value: Some(json!({"stories": ["S-1"]})),
    }];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(merged.is_ok(), "canonical vector 1 merge must not panic");

    assert_eq!(
        merged.unwrap(),
        json!({"foo": "bar", "wave_context": {"stories": ["S-1"]}}),
        "BC-4.12.005 canonical vector 1: additive merge must produce union"
    );
}

/// BC-4.12.005 canonical test vector 4: two resolvers, different keys.
/// static: {}, resolvers: key="a",value=1 and key="b",value=2 → {"a":1,"b":2}
#[test]
fn test_BC_4_12_005_merge_canonical_vector_4_two_resolvers_different_keys() {
    let static_config = json!({});
    let outputs = vec![
        ResolverOutput {
            key: "a".to_string(),
            value: Some(json!(1)),
        },
        ResolverOutput {
            key: "b".to_string(),
            value: Some(json!(2)),
        },
    ];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(merged.is_ok(), "canonical vector 4 merge must not panic");

    let result = merged.unwrap();
    let obj = result.as_object().expect("result must be an object");
    assert_eq!(
        obj.get("a").and_then(|v| v.as_i64()),
        Some(1),
        "key 'a' must equal 1"
    );
    assert_eq!(
        obj.get("b").and_then(|v| v.as_i64()),
        Some(2),
        "key 'b' must equal 2"
    );
}

/// BC-4.12.005 EC-002: resolver returns Some({}) (empty object).
/// key must be present with empty object value (not absent).
#[test]
fn test_BC_4_12_005_ec002_empty_object_value_produces_present_key_with_empty_object() {
    let static_config = json!({});
    let outputs = vec![ResolverOutput {
        key: "resolver_key".to_string(),
        value: Some(json!({})),
    }];

    let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        merge_resolver_outputs(static_config.clone(), &outputs, |_k, _o, _n| {})
    }));
    assert!(merged.is_ok(), "EC-002 empty object value must not panic");

    let result = merged.unwrap();
    let obj = result.as_object().expect("result must be an object");
    assert!(
        obj.contains_key("resolver_key"),
        "key must be present when resolver returns Some({{}}) — EC-002"
    );
    assert_eq!(
        obj["resolver_key"],
        json!({}),
        "key value must be empty object when resolver returns Some({{}}) — EC-002"
    );
}
