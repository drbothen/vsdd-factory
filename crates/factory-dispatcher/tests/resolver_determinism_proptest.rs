//! VP-075 proptest harness — Context-Injection Determinism.
//!
//! Verifies that `merge_resolver_outputs` is a pure function: identical
//! inputs always produce identical outputs.  Also verifies additive-overlay
//! preservation: base config fields survive the merge.
//!
//! Three property tests:
//!   - `prop_merge_is_deterministic`                        (200 trials, VP-075-B)
//!   - `prop_merge_preserves_base_config_fields`            (100 trials, VP-075-C / AC-006)
//!   - `prop_resolver_output_with_none_leaves_key_absent`   (100 trials, AC-004)
//!
//! VP-075-A (resolver-level determinism) is deferred to S-12.07 when the first
//! real `ContextResolver` implementation exists. The previous `prop_resolve_is_deterministic`
//! was a tautology: `FixedDeterministicResolver::resolve` ignored its input and
//! returned a stored clone — testing that `Option::clone()` is deterministic, not
//! that the registry path or any real resolver is. See VP-INDEX VP-075 entry.
//!
//! BC: BC-4.12.005 INV1, BC-1.13.001 PC5
//! Story: S-12.03
//! VP: VP-075

use factory_dispatcher::resolver::{ResolvedContext, ResolverOutput, merge_resolver_outputs};
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

/// Strategy: a `ResolvedContext` with `Some` value.
/// The context_key carries a `"resolver_"` prefix so it cannot collide with
/// base config keys (which use `[a-z]{1,16}` without prefix).
/// F-P3-001: resolver_name is a distinct string from context_key.
fn arb_resolver_output_with_value() -> impl Strategy<Value = ResolvedContext> {
    ("resolver_[a-z]{1,16}", arb_non_null_json_value()).prop_map(|(context_key, value)| {
        let resolver_name = format!("{}_resolver", context_key); // distinct from context_key
        ResolvedContext {
            context_key: context_key.clone(),
            resolver_name,
            output: ResolverOutput {
                key: context_key, // informational only
                value: Some(value),
            },
        }
    })
}

/// Strategy: a `ResolvedContext` with `None` value.
/// F-P3-001: resolver_name is a distinct string from context_key.
fn arb_resolver_output_none() -> impl Strategy<Value = ResolvedContext> {
    "resolver_[a-z]{1,16}".prop_map(|context_key| {
        let resolver_name = format!("{}_resolver", context_key);
        ResolvedContext {
            context_key: context_key.clone(),
            resolver_name,
            output: ResolverOutput {
                key: context_key,
                value: None,
            },
        }
    })
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
            merge_resolver_outputs(base_config.clone(), &outputs)
        }));
        let merged_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs)
        }));

        // If both calls succeed, assert structural equality (VP-075-B).
        // If either call panics (todo!()), that's the expected Red Gate failure —
        // the panic propagates and proptest reports the trial as failed.
        if merged_a.is_ok() && merged_b.is_ok() {
            let (map_a, _) = merged_a.unwrap();
            let (map_b, _) = merged_b.unwrap();
            prop_assert_eq!(
                Value::Object(map_a),
                Value::Object(map_b),
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
        // F-P2-002 / F-P3-001: ResolvedContext.context_key is the merge key;
        // resolver_name is the registry name (distinct from context_key).
        let merge_key = output.context_key.clone(); // context_key = the actual merge key (F-P2-002)
        let outputs = vec![output];

        let merged = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs)
        }));

        let (merged_obj, _collisions) = merged.expect(
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

        // The registry-declared context_key (merge_key) must be present in the merged result.
        // F-P2-002: the merge key is the tuple's first String, not output.key.
        prop_assert!(
            merged_obj.contains_key(merge_key.as_str()),
            "context_key '{}' must be present in merged result \
             (VP-075-C / BC-4.12.005 PC3 / F-P2-002 — context_key is the merge key)",
            merge_key
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
        // F-P2-002 / F-P3-001: ResolvedContext.context_key is the merge key.
        // With value: None, nothing is inserted — context_key is absent from result.
        let context_key = output.context_key.clone(); // F-P2-002: this would be the merge key if value were Some
        let outputs = vec![output];

        let merged_a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs)
        }));
        let merged_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            merge_resolver_outputs(base_config.clone(), &outputs)
        }));

        let (result_a, _) = merged_a.expect(
            "merge_resolver_outputs panicked — Red Gate: todo!() not yet implemented \
             (VP-075-D / AC-004 / BC-4.12.005 PC2)"
        );
        let (result_b, _) = merged_b.expect(
            "merge_resolver_outputs (second call) panicked — \
             Red Gate: todo!() not yet implemented (VP-075-D)"
        );

        // Determinism: both calls produce same result.
        prop_assert_eq!(
            Value::Object(result_a.clone()),
            Value::Object(result_b),
            "merge is deterministic for None-value outputs (VP-075-D)"
        );

        // None value: context_key must be absent from merged result (nothing inserted).
        // F-P2-002: with value: None, neither context_key nor output.key is inserted.
        prop_assert!(
            !result_a.contains_key(context_key.as_str()),
            "context_key '{}' must be ABSENT when resolver returns value: None \
             (VP-075-D / AC-004 / BC-4.12.005 PC2 / F-P2-002)",
            context_key
        );
    }
}

// VP-075-A: deferred to S-12.07.
// The previous prop_resolve_is_deterministic was a tautology (POLICY 11 violation):
// FixedDeterministicResolver::resolve ignored its input and returned a stored clone,
// exercising only Option::clone() — not the registry path or any real resolver.
// Real VP-075-A coverage requires a non-trivial ContextResolver implementation,
// which lands in S-12.07. See VP-INDEX entry for VP-075.
