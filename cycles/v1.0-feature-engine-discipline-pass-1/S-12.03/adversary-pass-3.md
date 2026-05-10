# Adversarial Review — S-12.03 Pass 3

## Metadata
- **Story:** S-12.03 ContextResolver trait + ResolverRegistry
- **Branch SHA reviewed:** 2fd5b659
- **Pass:** 3
- **Reviewer:** adversary (fresh context)
- **Classification:** MEDIUM
- **Within-story finding count:** 5
- **Recommendation:** PROCEED_TO_FIX

## Findings

### F-S12.03-P3-001 — MEDIUM — `resolver.merge_collision` event omits required `static_value` and `resolver_value` fields
**File(s):** `crates/factory-dispatcher/src/executor.rs:530-536`
**Anchor:** BC-4.12.005 PC5; story AC-007 narrative.
**Description:** Pure merge correctly returns `CollisionInfo { key, old_value, new_value }`. But executor's collision-event emitter writes ONLY `key`:
```rust
let ev = InternalEvent::now("resolver.merge_collision")
    .with_trace_id(trace_id)
    .with_plugin_name(&hook_name)
    .with_field("key", serde_json::Value::String(collision.key));
```
`collision.old_value` and `collision.new_value` dropped on the floor. BC-4.12.005 PC5 explicitly: "emitted with the key name, static value, and resolver value." Story AC-007 narrative repeats this. Path B's whole architectural reason for `CollisionInfo` carrying values: enable executor emission. Carrying them in pure return type only to discard at call site defeats Path B.
**Why it matters:** Operator debugging static-vs-resolver collision loses audit trail. SOUL #4 partially violated: event fires, payload incomplete. AC-007 falsifiable test was relaxed to require key emission only — test passes, BC postcondition unmet.
**Suggested resolution:** Add `static_value` + `resolver_value` fields to emitted event. Add unit/integration test asserting both value fields appear in InternalLog NDJSON (parallel to `f_p2_007_erroring_resolver_causes_resolver_error_event_in_internal_log`).

### F-S12.03-P3-002 — MEDIUM — Story Test Plan + File List still list deleted `prop_resolve_is_deterministic`
**File(s):** `.factory/stories/S-12.03-context-resolver-trait-and-registry.md:160` (File List), `.factory/stories/S-12.03-context-resolver-trait-and-registry.md:303` (Test Plan)
**Anchor:** Story spec coherence; pass-2 deleted `prop_resolve_is_deterministic` as POLICY 11 tautology. Deletion documented in code (`resolver_determinism_proptest.rs:240-245`) but NOT propagated to story body.
**Description:** File List entry still reads `prop_resolve_is_deterministic, prop_merge_is_deterministic, prop_merge_preserves_base_config_fields`. Test Plan row 303 lists `prop_resolve_is_deterministic`. Neither reflects actual delivered set: `prop_merge_is_deterministic`, `prop_merge_preserves_base_config_fields`, `prop_resolver_output_with_none_leaves_key_absent`.
**Why it matters:** Test Plan is part of falsifiable contract. Story cannot be Done if Test Plan promises non-existent test. Future readers misled. VP-075 deferred coverage (VP-075-A → S-12.07) has no story-level record.
**Suggested resolution:** Update line 160 File List + line 303 Test Plan row. Add note in Test Plan or Dev Notes: "VP-075-A (resolver-level determinism) deferred to S-12.07 — see proptest file header. Previous `prop_resolve_is_deterministic` retired in pass-2 as POLICY 11 tautology."

### F-S12.03-P3-003 — LOW — AC-005 has no integration-level assertion that `resolver.not_found` event reaches `InternalLog` from production path
**File(s):** `crates/factory-dispatcher/tests/resolver_registry_test.rs:377-412` (callback-only); `crates/factory-dispatcher/tests/executor_resolver_integration.rs` (no parallel test).
**Anchor:** AC-005 narrative; BC-1.13.001 PC6.
**Description:** AC-005 verified at unit-test level by injecting callback. Production wire-up `executor::build_plugin_config → emit_not_found closure → internal_log.write(&InternalEvent::now("resolver.not_found"))` has no integration test round-tripping through actual InternalLog NDJSON file. Precedent exists for `resolver.error` (`f_p2_007_erroring_resolver_causes_resolver_error_event_in_internal_log`).
**Why it matters:** Future refactor breaking closure wiring would silently swallow misconfiguration warnings. Most common operator failure mode is misconfigured `needs_context`.
**Suggested resolution:** Add `ac005_resolver_not_found_event_appears_in_internal_log()` in executor_resolver_integration.rs. Modeled on f_p2_007 test (lines 339-410). `needs_context: ["unknown"]`, register zero resolvers, dispatch via `execute_tiers`, drop InternalLog, grep for `resolver.not_found`.

### F-S12.03-P3-004 — LOW — AC-012 test asserts `is_err()` not variant identity
**File(s):** `crates/factory-dispatcher/tests/resolver_registry_test.rs:806-860`
**Anchor:** AC-012; BC-4.12.005 PC6/EC-005.
**Description:** `test_BC_4_12_005_ac012_duplicate_name_registration_returns_error` asserts only `dup_outcome.is_err()`. With `#[non_exhaustive]` + 7 variants, `is_err()` accepts ALL of them. Regression returning `Err(ResolverError::Malformed{...})` would pass test but break operator log parsing. F-P2-007 test (lines 1052-1110) DID upgrade to variant-identity assertion; AC-012 not given same treatment.
**Why it matters:** Variant-identity discipline applied to new test but not back-propagated. AC-012 is lone holdout among new error tests.
**Suggested resolution:** Replace `assert!(dup_outcome.is_err())` with `assert_eq!(err, ResolverError::DuplicateName { name: "foo".to_string() }, ...)` per F-P2-003 PartialEq derives.

### F-S12.03-P3-005 — NITPICK — Stale module-level doc comment in resolver_registry_test.rs references todo!() Red Gate behavior
**File(s):** `crates/factory-dispatcher/tests/resolver_registry_test.rs:7-11`
**Description:** Module doc comment still reads "All tests below call production code that is currently `todo!()` stubs. They must FAIL with a `todo!()` panic caught via `std::panic::catch_unwind`..." Production is implemented; tests are GREEN. `catch_unwind` boilerplate now defensive-only.
**Suggested resolution:** Update comment to reflect GREEN state; note `catch_unwind` wrappers are historical Red Gate scaffolding serving as defense-in-depth.

## Cross-cutting observations
- POLICY 10: not blocking, demo dir deferred to post-convergence step.
- POLICY 11: Sampled 6 unit + 3 proptest + 4 integration tests — all call production fns. Pass-2 retired only known tautology. No new tautologies.
- POLICY 12: ResolverInput/ResolverOutput serialized fields match BC TVs. **PARTIAL CONCERN intersecting F-001:** `resolver.merge_collision` TV expects `{key, static_value, resolver_value}` but production emits `{key}` only — captured as F-S12.03-P3-001.
- `#![deny(unsafe_code)]` audit: confirmed at lib.rs:13. Grep returned only comment matches. No unsafe blocks.
- `resolve_context_for_entry` ordering: `requested_names` order preserved through `for name in requested_names → outputs.push((output.key, value))`. Determinism CONFIRMED.
- `merge_resolver_outputs` purity: no I/O, no global state, no callbacks. CollisionInfo derives sound.
- ResolverError serde round-trip: derives present + non_exhaustive + tag="kind". No JSON round-trip test exists; flagged for S-12.04.

## Convergence assessment
- Within-story findings: 5 (2 MEDIUM, 2 LOW, 1 NITPICK)
- Severity floor: MEDIUM (F-P3-001, F-P3-002)
- Classification: MEDIUM
- Reasoning: F-001 BC violation (BC-4.12.005 PC5 specifies 3-field event payload, production emits 1). F-002 partial-fix regression: pass-2 deletion not propagated to spec body. Both block NITPICK_ONLY classification. Pass-4 should classify NITPICK_ONLY if F-001 + F-002 land cleanly + F-003/F-004 addressed.
- Recommendation: PROCEED_TO_FIX
