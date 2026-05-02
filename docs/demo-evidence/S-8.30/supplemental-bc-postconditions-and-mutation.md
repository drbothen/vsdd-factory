# Supplemental: BC-2.02.012 Postconditions, Mutation Testing Flag, Consumer Sites

**Story:** S-8.30

---

## BC-2.02.012 Postconditions 1-7 — test coverage map

| Postcondition | Description | Test |
|---|---|---|
| PC-1 | `agent_type` is `Some(string)` when envelope carries non-null string | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-2 | `subagent_name` is `Some(string)` when envelope carries non-null string | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-3 | `last_assistant_message` is `Some(string)` when envelope carries non-null string | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-4 | `result` is `Some(string)` when envelope carries non-null string | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-5 | Canonical agent identity chain: `agent_type.as_deref().or(subagent_name.as_deref()).unwrap_or("unknown")` | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` |
| PC-6 | Canonical assistant-message chain: `last_assistant_message.as_deref().or(result.as_deref()).unwrap_or("")` | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` |
| PC-7 | Non-SubagentStop envelopes have all four fields = `None` | `test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none`, `test_BC_2_02_012_ac8_all_event_types_...` |

---

## BC-2.02.012 Invariants — test coverage map

| Invariant | Description | Test |
|---|---|---|
| Inv-1 | `HOST_ABI_VERSION` remains 1; additive extension under D-6 Option A | `test_BC_2_02_012_invariant1_host_abi_version_remains_one` |
| Inv-2 | `#[serde(default)]` ensures backward-compat; non-SubagentStop fields default to `None` | `test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none`, `test_BC_2_02_012_ac8_all_event_types_...` |
| Inv-3 | JSON `null` deserializes to `None` (jq `//` null-advance parity) | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` |
| Inv-4 | Field names are canonical and immutable; round-trip preserves values | `test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields` |

---

## Mutation Testing Flag Registration

**Status:** `mutation_testing_required: true` registered in
`.factory/cycles/v1.0-brownfield-backfill/S-8.30/implementation/red-gate-log.md`

**Reason (Option B remediation):**

The stub-architect commit `b01eefc` over-delivered: it implemented both AC-1
(struct fields) and AC-6 (HOST_ABI.md documentation) before the Red Gate test
pass. As a result, all 20 new serde tests were already GREEN when the test-writer
ran — `RED_RATIO = 0/0` (full_exception_path: true).

Per `per-story-delivery.md §Remediation`, Option B was accepted:
- `mutation_testing_required: true` registered in the red-gate-log frontmatter
- Wave gate must run `cargo mutants -p vsdd-hook-sdk` as compensating control
  before S-8.30 can be counted toward wave completion

**red-gate-log.md frontmatter excerpt:**

```yaml
red_ratio: 0.0
red_count: 0
total_new_tests: 20
exempt_count: 20
remediation: "Option B accepted: mutation_testing_required registered"
full_exception_path: true
mutation_testing_required: true
```

---

## Consumer Struct Literal Sites Updated (GREEN commit b82adcb)

All sites that construct `HookPayload` using struct literal syntax required
adding the four new fields with `None` defaults. The GREEN commit propagated
these fields to 9 struct literal sites across 5 files:

| File | Lines with `agent_type: None` | Notes |
|---|---|---|
| `crates/hook-plugins/capture-commit-activity/tests/contract_emit_event.rs` | 28, 126, 202 | 3 sites |
| `crates/hook-plugins/capture-commit-activity/tests/contract_payload_parsing.rs` | 23, 172 | 2 sites |
| `crates/hook-plugins/capture-commit-activity/tests/edge_cases.rs` | 26, 208 | 2 sites |
| `crates/hook-plugins/capture-commit-activity/tests/contract_hook_macro.rs` | 27 | 1 site |
| `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` | 201 | 1 site (pre-existing) |

**Total:** 9 struct literal sites updated with:
```rust
agent_type: None,
subagent_name: None,
last_assistant_message: None,
result: None,
```

The red-gate-log documented 8 compile-failing sites in `capture-commit-activity`
(the original E0063 compile errors). The `legacy-bash-adapter` site was also
updated in the same GREEN commit as part of the complete propagation pass.

---

## Full Test Suite Summary

```
running 26 tests
test payload::tests::plugin_config_defaults_to_null_when_missing ... ok
test payload::tests::pretooluse_payload_deserializes ... ok
test payload::tests::test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop ... ok
test payload::tests::payload_round_trip_via_serde ... ok
test payload::tests::test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults ... ok
test payload::tests::lifecycle_payload_without_tool_name ... ok
test payload::tests::test_BC_2_02_012_ec006_unknown_fields_silently_ignored ... ok
test payload::tests::test_BC_2_02_012_ec007_wrong_type_agent_type_returns_serde_error ... ok
test payload::tests::plugin_config_passes_through_when_present ... ok
test payload::tests::test_BC_2_02_012_ec007_wrong_type_last_assistant_message_returns_serde_error ... ok
test payload::tests::posttooluse_payload_with_response ... ok
test payload::tests::test_BC_2_02_012_ec007_wrong_type_result_returns_serde_error ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_section ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_example_json ... ok
test payload::tests::test_BC_2_02_012_ec007_wrong_type_subagent_name_returns_serde_error ... ok
test payload::tests::test_BC_2_02_012_ec008_empty_string_does_not_advance_fallback ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_assistant_message_fallback_chain ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_agent_identity_fallback_chain ... ok
test payload::tests::test_BC_2_02_012_invariant1_host_abi_version_remains_one ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_documents_subagentstop_presence_semantics ... ok
test payload::tests::test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields ... ok
test payload::tests::test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains ... ok
test payload::tests::test_BC_2_02_012_non_subagentstop_projection_does_not_leak ... ok
test payload::tests::test_BC_2_02_012_subagentstop_fallback_fields_populated ... ok
test payload::tests::test_BC_2_02_012_subagentstop_all_four_fields_populated ... ok
test payload::tests::test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
```

6 pre-existing tests + 20 new BC-2.02.012 tests. All PASS.
