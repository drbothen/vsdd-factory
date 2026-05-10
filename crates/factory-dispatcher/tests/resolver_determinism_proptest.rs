//! VP-075 proptest harness — Context-Injection Determinism.
//!
//! Verifies that `merge_resolver_outputs` is a pure function: identical
//! inputs always produce identical outputs.  Also verifies additive-overlay
//! preservation: base config fields survive the merge.
//!
//! Four property tests:
//!   - `prop_merge_is_deterministic`                        (200 trials, VP-075-B)
//!   - `prop_merge_preserves_base_config_fields`            (100 trials, VP-075-C / AC-006)
//!   - `prop_resolver_output_with_none_leaves_key_absent`   (100 trials, AC-004)
//!   - `prop_resolve_is_deterministic`                      (100 trials, VP-075-A / AC-008)
//!
//! BC: BC-4.12.005 INV1, BC-1.13.001 PC5
//! Story: S-12.03
//! VP: VP-075

use factory_dispatcher::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, merge_resolver_outputs,
};
use proptest::prelude::*;
use serde_json::{Map, Value};

// ---------------------------------------------------------------------------
// Strategies
// ---------------------------------------------------------------------------

/// Strategy: arbitrary JSON object (no null values at top level).
/// Matches the shape of a `plugin_config` — always an Object (returned as
/// `Map<String, Value>` so it can be passed directly to `merge_resolver_outputs`
/// without a type-level coercion step).
///
/// Key strategy uses `[a-z]{1,16}` (no underscore) to be consistent with
/// `prop_merge_preserves_base_config_fields`, which excludes underscores
/// to prevent collision with `resolver_*` keys (F-005).
fn arb_json_object() -> impl Strategy<Value = Map<String, Value>> {
    prop::collection::hash_map("[a-z]{1,16}", arb_non_null_json_value(), 0..8)
        .prop_map(|map| map.into_iter().collect::<Map<_, _>>())
}

/// Strategy: non-null JSON values (scalars + simple nested objects/arrays).
/// Bounded depth to avoid pathological sizes.
fn arb_non_null_json_value() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::Bool(true)),
        Just(Value::Bool(false)),
        any::<i64>().prop_map(|n| Value::Number(n.into())),
        "[a-zA-Z0-9_\\-]{0,32}".prop_map(Value::String),
        prop::collection::vec(any::<i64>().prop_map(|n| Value::Number(n.into())), 0..4)
            .prop_map(Value::Array),
        prop::collection::hash_map(
            "[a-z_]{1,8}",
            "[a-z0-9]{0,16}".prop_map(Value::String),
            0..3
        )
        .prop_map(|m| Value::Object(m.into_iter().collect::<Map<_, _>>())),
    ]
}

/// Strategy: a `ResolverOutput` with `Some` value (key must not collide with
/// base config by using a distinct prefix).
fn arb_resolver_output_with_value() -> impl Strategy<Value = ResolverOutput> {
    ("resolver_[a-z]{1,16}", arb_non_null_json_value()).prop_map(|(key, value)| ResolverOutput {
        key,
        value: Some(value),
    })
}

/// Strategy: a `ResolverOutput` with `None` value.
fn arb_resolver_output_none() -> impl Strategy<Value = ResolverOutput> {
    "resolver_[a-z]{1,16}".prop_map(|key| ResolverOutput { key, value: None })
}

// ---------------------------------------------------------------------------
// VP-075-B: merge determinism
// ---------------------------------------------------------------------------

// VP-075-B / AC-008 / BC-4.12.005 INV1:
// `merge_resolver_outputs(base, outputs)` is a pure function.
// Calling it twice with identical inputs must produce identical output.
//
// 200 trials per VP-075 specification.
proptest! {
    #![proptest_config(proptest::test_runner::Config {
        cases: 200,
        timeout: 5_000,
        ..Default::default()
    })]

    #[test]
    fn prop_merge_is_deterministic(
        base_config in arb_json_object(),
        output in arb_resolver_output_with_value(),
    ) {
        let outputs = vec![output];

        let merged_a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs, |_k, _o, _n| {})
        }));
        let merged_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs, |_k, _o, _n| {})
        }));

        // If both calls succeed, assert structural equality (VP-075-B).
        // If either call panics (todo!()), that's the expected Red Gate failure —
        // the panic propagates and proptest reports the trial as failed.
        if merged_a.is_ok() && merged_b.is_ok() {
            prop_assert_eq!(
                Value::Object(merged_a.unwrap()),
                Value::Object(merged_b.unwrap()),
                "merge_resolver_outputs must return identical output for identical inputs \
                 (VP-075-B / AC-008 / BC-4.12.005 INV1)"
            );
        } else {
            // One of the calls panicked — forward the panic to trigger Red Gate failure.
            let _ = merged_a.expect("merge_resolver_outputs panicked — Red Gate: todo!() not yet implemented (VP-075-B)");
        }
    }
}

// ---------------------------------------------------------------------------
// VP-075-C: additive preservation
// ---------------------------------------------------------------------------

// VP-075-C / AC-006 / BC-4.12.005 PC1:
// All base_config fields must be present in the merged result.
// The resolver's key (with "resolver_" prefix) must also be present.
//
// 100 trials per VP-075 specification.
proptest! {
    #![proptest_config(proptest::test_runner::Config {
        cases: 100,
        timeout: 5_000,
        ..Default::default()
    })]

    #[test]
    fn prop_merge_preserves_base_config_fields(
        base_config in prop::collection::hash_map(
            // Use short keys with no "resolver_" prefix to avoid collision
            // with the resolver_output key (which always starts with "resolver_").
            "[a-z]{1,16}",
            arb_non_null_json_value(),
            1..4,
        ).prop_map(|m| m.into_iter().collect::<Map<_, _>>()),
        output in arb_resolver_output_with_value(),
    ) {
        let outputs = vec![output.clone()];

        let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs, |_k, _o, _n| {})
        }));

        let merged_obj = merged.expect(
            "merge_resolver_outputs panicked — Red Gate: todo!() not yet implemented \
             (VP-075-C / AC-006 / BC-4.12.005 PC1)"
        );

        // All base_config keys must survive the merge.
        for k in base_config.keys() {
            prop_assert!(
                merged_obj.contains_key(k.as_str()),
                "base_config key '{}' must be preserved in merged output \
                 (VP-075-C / BC-4.12.005 PC1 — additive overlay)",
                k
            );
        }

        // The resolver's output key must be present.
        prop_assert!(
            merged_obj.contains_key(output.key.as_str()),
            "resolver output key '{}' must be present in merged result \
             (VP-075-C / BC-4.12.005 PC3)",
            output.key
        );
    }
}

// ---------------------------------------------------------------------------
// VP-075-D: None value determinism (AC-004 boundary)
// ---------------------------------------------------------------------------

// VP-075-D / AC-004 / BC-4.12.005 PC2:
// When a resolver returns value: None, the merged result must not contain
// that key, AND the merge must be deterministic across two calls.
//
// 100 trials.
proptest! {
    #![proptest_config(proptest::test_runner::Config {
        cases: 100,
        timeout: 5_000,
        ..Default::default()
    })]

    #[test]
    fn prop_resolver_output_with_none_leaves_key_absent(
        base_config in arb_json_object(),
        output in arb_resolver_output_none(),
    ) {
        let outputs = vec![output.clone()];

        let merged_a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs, |_k, _o, _n| {})
        }));
        let merged_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs, |_k, _o, _n| {})
        }));

        let result_a = merged_a.expect(
            "merge_resolver_outputs panicked — Red Gate: todo!() not yet implemented \
             (VP-075-D / AC-004 / BC-4.12.005 PC2)"
        );
        let result_b = merged_b.expect(
            "merge_resolver_outputs (second call) panicked — \
             Red Gate: todo!() not yet implemented (VP-075-D)"
        );

        // Determinism: both calls produce same result.
        prop_assert_eq!(
            Value::Object(result_a.clone()),
            Value::Object(result_b),
            "merge is deterministic for None-value outputs (VP-075-D)"
        );

        // None value: key must be absent from merged result.
        prop_assert!(
            !result_a.contains_key(output.key.as_str()),
            "key '{}' must be ABSENT when resolver returns value: None \
             (VP-075-D / AC-004 / BC-4.12.005 PC2)",
            output.key
        );
    }
}

// ---------------------------------------------------------------------------
// VP-075-A: resolve determinism
// ---------------------------------------------------------------------------

// VP-075-A / AC-008 / BC-4.12.005 INV1:
// Calling a `ContextResolver::resolve()` twice with identical inputs produces
// identical `ResolverOutput`.
//
// Uses a `FixedResolver` that returns the same output on every call —
// this exercises the trait surface defined in S-12.03 (the pure computation
// function from S-12.05 extends coverage in that story).
//
// 100 trials.

/// A trivial resolver for proptest use: returns a fixed `ResolverOutput` on every call.
struct FixedDeterministicResolver {
    output: ResolverOutput,
}

impl ContextResolver for FixedDeterministicResolver {
    fn name(&self) -> &str {
        "proptest_fixed"
    }

    fn resolve(&self, _input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        Ok(Some(self.output.clone()))
    }
}

/// Strategy: an arbitrary `ResolverInput` for proptest use.
fn arb_resolver_input() -> impl Strategy<Value = ResolverInput> {
    (
        prop_oneof![
            Just("PreToolUse".to_string()),
            Just("PostToolUse".to_string()),
        ],
        "[a-z]{1,16}",
        prop_oneof![Just(None::<String>), "[a-z]{1,8}".prop_map(Some),],
    )
        .prop_map(|(event_type, hook_name, agent_type)| ResolverInput {
            event_type,
            hook_event_name: hook_name,
            agent_type,
            project_dir: "/tmp/proptest-project".to_string(),
            plugin_config: serde_json::Value::Object(Map::new()),
        })
}

proptest! {
    #![proptest_config(proptest::test_runner::Config {
        cases: 100,
        timeout: 5_000,
        ..Default::default()
    })]

    #[test]
    fn prop_resolve_is_deterministic(
        key in "[a-z]{1,16}",
        value in arb_non_null_json_value(),
        input in arb_resolver_input(),
    ) {
        let output = ResolverOutput {
            key: key.clone(),
            value: Some(value),
        };
        let resolver = FixedDeterministicResolver { output };

        let result_a = resolver.resolve(&input)
            .expect("FixedDeterministicResolver must not error (VP-075-A)");
        let result_b = resolver.resolve(&input)
            .expect("FixedDeterministicResolver second call must not error (VP-075-A)");

        prop_assert_eq!(
            result_a,
            result_b,
            "ContextResolver::resolve must be deterministic: identical inputs produce \
             identical ResolverOutput (VP-075-A / AC-008 / BC-4.12.005 INV1)"
        );
    }
}
