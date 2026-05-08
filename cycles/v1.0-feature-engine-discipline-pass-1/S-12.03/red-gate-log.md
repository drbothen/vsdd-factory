---
document_type: red-gate-log
story_id: S-12.03
step: 3
phase: implementation
cycle: v1.0-feature-engine-discipline-pass-1
timestamp: 2026-05-07T00:00:00Z
status: RED_GATE_VERIFIED
---

# Red Gate Log — S-12.03 Step 3

## Summary

All new tests FAIL with `todo!()` panics caught via `std::panic::catch_unwind`
and re-asserted with behavioral context messages. Three AC-001 tests that
exercise already-implemented `serde(default)` deserialization pass correctly
(they test `registry.rs` serde, not stubs). Zero build errors. Red Gate is
VERIFIED for all stubs-under-test.

## Test Execution Results

### resolver_registry_test.rs

```
test result: FAILED. 3 passed; 17 failed; 0 ignored; 0 measured; 0 filtered out
```

**Passing (correctly):**
- `test_BC_1_13_001_ac001_needs_context_defaults_to_empty_when_absent` — serde deserialization, already implemented
- `test_BC_1_13_001_ac001_needs_context_round_trips_through_toml` — serde deserialization, already implemented
- `test_BC_1_13_001_ac011_empty_registry_construction_does_not_error` — only calls `new()`, which is implemented

**Failing (Red Gate — all `todo!()` panics):**
- `test_BC_1_13_001_ac002_empty_needs_context_skips_resolver_invocation`
- `test_BC_1_13_001_ac003_declared_resolver_is_invoked_and_output_returned`
- `test_BC_1_13_001_ac005_invoke_resolver_returns_none_for_missing_resolver`
- `test_BC_1_13_001_ac005_unknown_resolver_triggers_not_found_callback`
- `test_BC_1_13_001_ac009_declaration_order_is_invocation_order`
- `test_BC_1_13_001_ac010_resolved_context_is_fully_populated_before_return`
- `test_BC_1_13_001_ac011_empty_registry_emits_not_found_and_does_not_panic`
- `test_BC_4_12_005_ac004_merge_none_value_leaves_key_absent_in_plugin_config`
- `test_BC_4_12_005_ac004_none_value_leaves_key_absent_from_resolved_map`
- `test_BC_4_12_005_ac006_additive_overlay_preserves_static_config_fields`
- `test_BC_4_12_005_ac007_resolver_wins_on_static_key_collision`
- `test_BC_4_12_005_ac007_resolver_wins_with_whole_value_replacement_no_deep_merge`
- `test_BC_4_12_005_ac012_duplicate_name_registration_returns_error`
- `test_BC_4_12_005_ac012_first_registration_preserved_after_duplicate_fails`
- `test_BC_4_12_005_ec002_empty_object_value_produces_present_key_with_empty_object`
- `test_BC_4_12_005_merge_canonical_vector_1_additive_no_collision`
- `test_BC_4_12_005_merge_canonical_vector_4_two_resolvers_different_keys`

### resolver_determinism_proptest.rs (VP-075)

```
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

**Failing (Red Gate — all `todo!()` panics from `merge_resolver_outputs`):**
- `prop_merge_is_deterministic` (200 trials, VP-075-B)
- `prop_merge_preserves_base_config_fields` (100 trials, VP-075-C)
- `prop_resolver_output_with_none_leaves_key_absent` (100 trials, VP-075-D / AC-004)

### Existing tests (regression check)

All 10 existing test binaries pass. Zero regressions.

```
test result: ok. 112 passed; 0 failed  (lib unit tests)
test result: ok. 7 passed; 0 failed    (bc_2_02_011_parity)
test result: ok. 11 passed; 0 failed   (bc_7_03_079_080_parity)
test result: ok. 23 passed; 0 failed   (executor_integration)
test result: ok. 5 passed; 0 failed    (bc_9_01_rc1_release_gate_test)
test result: ok. 2 passed; 0 failed    (host_write_file_integration)
test result: ok. 10 passed; 0 failed   (host_functions)
test result: ok. 2 passed; 0 failed    (internal_log_integration)
test result: ok. 1 passed; 0 failed    (loads_legacy_registry)
(+ router_integration, routing_integration, s4_07_integration, sinks_*)
```

## Red Gate Density

- resolver_registry_test.rs: 17 failing / 20 total = **85% RED** (≥50% target met)
- resolver_determinism_proptest.rs: 3 failing / 3 total = **100% RED**

## Failure Mode Verification

All failures are `todo!()` panics from production stubs caught via
`std::panic::catch_unwind` and re-asserted with behavioral messages naming:
- the BC clause being tested
- the AC being exercised
- the expected behavior once implemented

No `#[should_panic]` masking used.
No vacuously-passing tests.
