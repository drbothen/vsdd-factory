---
story_id: S-8.30
cycle: v1.0-brownfield-backfill
step: 3
agent: test-writer
timestamp: 2026-05-02T00:00:00Z
red_ratio: 0.0
red_count: 0
total_new_tests: 20
exempt_count: 20
remediation: "Option B accepted: mutation_testing_required registered"
full_exception_path: true
mutation_testing_required: true
green_phase_complete: true
green_phase_timestamp: 2026-05-02T00:00:00Z
---

# Red Gate Log — S-8.30

## Summary

`cargo test --workspace` FAILS due to compile errors in
`crates/hook-plugins/capture-commit-activity/tests/` — 8 instances of
`E0063: missing fields agent_type, last_assistant_message, result and
subagent_name in initializer of HookPayload`.

These compile errors ARE the Red Gate signal: the stub-architect commit
`b01eefc` added 4 new fields to `HookPayload` without updating the
`capture-commit-activity` plugin tests that construct `HookPayload` using
struct literal syntax. The implementer must add the new fields (with `None`
defaults) to all 8 struct literal sites in those test files.

## New Tests Written (hook-sdk)

20 new tests added to `crates/hook-sdk/src/payload.rs` test module plus
2 new doctests in field doc comments.

| # | Test Name | BC Clause | Result |
|---|-----------|-----------|--------|
| 1 | `test_BC_2_02_012_subagentstop_all_four_fields_populated` | PC-1,2,3,4 | GREEN |
| 2 | `test_BC_2_02_012_subagentstop_fallback_fields_populated` | PC-1,2,3,4 | GREEN |
| 3 | `test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none` | PC-7, Inv-2 | GREEN |
| 4 | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` | EC-001, Inv-3, PC-5,6 | GREEN |
| 5 | `test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults` | EC-003 | GREEN |
| 6 | `test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields` | Inv-4 | GREEN |
| 7 | `test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop` | PC-7, Inv-2, AC-8 | GREEN |
| 8 | `test_BC_2_02_012_ec007_wrong_type_agent_type_returns_serde_error` | EC-007 | GREEN |
| 9 | `test_BC_2_02_012_ec007_wrong_type_subagent_name_returns_serde_error` | EC-007 | GREEN |
| 10 | `test_BC_2_02_012_ec007_wrong_type_last_assistant_message_returns_serde_error` | EC-007 | GREEN |
| 11 | `test_BC_2_02_012_ec007_wrong_type_result_returns_serde_error` | EC-007 | GREEN |
| 12 | `test_BC_2_02_012_ec008_empty_string_does_not_advance_fallback` | EC-008 | GREEN |
| 13 | `test_BC_2_02_012_ec006_unknown_fields_silently_ignored` | EC-006 | GREEN |
| 14 | `test_BC_2_02_012_non_subagentstop_projection_does_not_leak` | PC-7, EC-005 | GREEN |
| 15 | `test_BC_2_02_012_invariant1_host_abi_version_remains_one` | Inv-1 | GREEN |
| 16 | `test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_section` | AC-6(e) | GREEN |
| 17 | `test_BC_2_02_012_ac6_host_abi_md_documents_subagentstop_presence_semantics` | AC-6(b) | GREEN |
| 18 | `test_BC_2_02_012_ac6_host_abi_md_contains_agent_identity_fallback_chain` | AC-6(c) | GREEN |
| 19 | `test_BC_2_02_012_ac6_host_abi_md_contains_assistant_message_fallback_chain` | AC-6(c) | GREEN |
| 20 | `test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_example_json` | AC-6(a,d) | GREEN |

## RED_RATIO Analysis

```
RED_RATIO = 0 / (20 - 20) = undefined (full_exception_path: true)
```

All 20 new tests are GREEN. Exemption analysis:

| test_name | result | rationale_category | notes |
|-----------|--------|-------------------|-------|
| test_BC_2_02_012_subagentstop_all_four_fields_populated | GREEN | PURE-DATA | `#[serde(default)] Option<String>` deserialization is deterministic from type system; stub added the fields with correct type + attribute |
| test_BC_2_02_012_subagentstop_fallback_fields_populated | GREEN | PURE-DATA | Same as above |
| test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none | GREEN | PURE-DATA | `#[serde(default)]` behavior guaranteed by type system once fields are declared |
| test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains | GREEN | PURE-DATA | JSON null → None is serde_json's documented invariant for Option<T> |
| test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults | GREEN | PURE-DATA | All-absent → all None via `#[serde(default)]`; chain expressions are pure |
| test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields | GREEN | PURE-DATA | Serde round-trip for `Option<String>` is guaranteed by serde_json |
| test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop | GREEN | PURE-DATA | Non-SubagentStop default behavior is pure data/type system |
| test_BC_2_02_012_ec007_wrong_type_agent_type_returns_serde_error | GREEN | PURE-DATA | serde error on type mismatch for Option<String> is deterministic |
| test_BC_2_02_012_ec007_wrong_type_subagent_name_returns_serde_error | GREEN | PURE-DATA | Same as above |
| test_BC_2_02_012_ec007_wrong_type_last_assistant_message_returns_serde_error | GREEN | PURE-DATA | Same as above |
| test_BC_2_02_012_ec007_wrong_type_result_returns_serde_error | GREEN | PURE-DATA | Same as above |
| test_BC_2_02_012_ec008_empty_string_does_not_advance_fallback | GREEN | PURE-DATA | Option::or does not advance on Some(""); pure type semantics |
| test_BC_2_02_012_ec006_unknown_fields_silently_ignored | GREEN | FRAMEWORK-WIRING | serde_json ignores unknown fields unless deny_unknown_fields; this is framework behavior |
| test_BC_2_02_012_non_subagentstop_projection_does_not_leak | GREEN | PURE-DATA | Same as pretooluse backward-compat case |
| test_BC_2_02_012_invariant1_host_abi_version_remains_one | GREEN | STRUCTURAL-ASSERTION | Const value assertion; green by design |
| test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_section | GREEN | PRE-EXISTING-BEHAVIOR | Stub unexpectedly updated HOST_ABI.md (see note below) |
| test_BC_2_02_012_ac6_host_abi_md_documents_subagentstop_presence_semantics | GREEN | PRE-EXISTING-BEHAVIOR | Same — HOST_ABI.md updated by stub |
| test_BC_2_02_012_ac6_host_abi_md_contains_agent_identity_fallback_chain | GREEN | PRE-EXISTING-BEHAVIOR | Same |
| test_BC_2_02_012_ac6_host_abi_md_contains_assistant_message_fallback_chain | GREEN | PRE-EXISTING-BEHAVIOR | Same |
| test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_example_json | GREEN | PRE-EXISTING-BEHAVIOR | Same |

**Note on HOST_ABI.md tests (tests 16-20):** Stub commit `b01eefc` updated
HOST_ABI.md with the SubagentStop section (AC-6). These tests are GREEN
because the stub over-delivered: it implemented both AC-1 (struct fields) AND
AC-6 (HOST_ABI.md documentation). The AC-6 tests would have been RED if the
stub had only added the struct fields (as is conventional for stub-architect).

## Actual Red Gate — Downstream Compile Failures

The genuine Red Gate signal is `cargo test --workspace` failure:

```
error[E0063]: missing fields `agent_type`, `last_assistant_message`, `result`
and 1 other field in initializer of `HookPayload`
```

8 instances across 4 test files in `crates/hook-plugins/capture-commit-activity/tests/`:
- `contract_emit_event.rs`: lines 17, 114, 183
- `contract_payload_parsing.rs`: lines 15, 157
- `edge_cases.rs`: lines 18, 196
- `contract_hook_macro.rs`: line 19

These tests fail because `HookPayload` gained 4 new fields (no `..Default::default()`)
and existing struct literal constructions are now incomplete. The implementer
must add the four new fields (all `None`) to each struct literal site.

## full_exception_path: true Acknowledgment

All 20 new hook-sdk tests are exempt (PURE-DATA, FRAMEWORK-WIRING, STRUCTURAL-ASSERTION,
or PRE-EXISTING-BEHAVIOR). Denominator = 0. This is acknowledged per
`per-story-delivery.md §"Full-Exception Path"`. The over-complete stub
(which implemented both AC-1 and AC-6 simultaneously) is the reason all
serde tests are green.

The Red Gate IS active via downstream compile failures in `capture-commit-activity`.
The implementer's first task is to fix those 8 struct literal sites.

## Orchestrator Decision Required

Per `per-story-delivery.md §Remediation`:
- **Option A**: Roll back b01eefc and re-dispatch stub-architect with a stricter
  prompt (partial stub: add fields only, use `todo!()` for HOST_ABI.md update).
  RED_RATIO recomputed on second test-writer pass.
- **Option B**: Accept full_exception_path + register `mutation_testing_required: true`
  in S-8.30 frontmatter. Wave gate must run `cargo mutants` as compensating control.

Recommendation: **Option B**. The stub's implementation is verified correct by
20 passing tests. The downstream compile failures represent the real RED gate work
(the implementer must update `capture-commit-activity` struct literals). Rolling
back the stub would lose that correctness verification.
