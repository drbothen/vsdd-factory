# Evidence Report — S-8.30: SDK extension: HookPayload SubagentStop top-level fields

**Story ID:** S-8.30  
**BC anchor:** BC-2.02.012  
**Feature branch:** `feature/S-8.30-sdk-extension-hookpayload-subagentstop-fields`  
**Commits:**
- `b01eefc` — Stub Architect (4 new `#[serde(default)] pub Option<String>` fields + HOST_ABI.md update)
- `18bfd25` — Red Gate (20 unit tests + 2 doctests for serde behavior + BC-2.02.012 postconditions)
- `b82adcb` — GREEN (propagated fields to 9 consumer struct literal sites)

---

## Coverage Summary

| AC | Title | Evidence | Status |
|---|---|---|---|
| AC-1 | HookPayload struct gains 4 SubagentStop fields | `AC-1.md` — struct definition excerpt | PASS |
| AC-2 | Backward-compat: PreToolUse has all 4 fields = None | `AC-2.md` — cargo test output | PASS |
| AC-3 | SubagentStop envelope with all 4 fields populates correctly | `AC-3.md` — cargo test output | PASS |
| AC-4 | jq-`//` parity: JSON null to None; fallback chains match bash | `AC-4.md` — cargo test output | PASS |
| AC-5 | Round-trip preservation | `AC-5.md` — cargo test output | PASS |
| AC-6 | HOST_ABI.md updated with SubagentStop envelope schema | `AC-6.md` — HOST_ABI.md excerpt + 5 tests | PASS |
| AC-7 | HOST_ABI_VERSION remains 1 in both crates | `AC-7.md` — grep output + test | PASS |
| AC-8 | All 5 event types deserialize; non-SubagentStop fields = None | `AC-8.md` — cargo test output | PASS |

---

## Full Cargo Test Output

Command: `PATH="$HOME/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH" cargo test -p vsdd-hook-sdk --lib payload`

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

---

## BC-2.02.012 Postconditions Coverage

| Postcondition | Test |
|---|---|
| PC-1: agent_type Some when non-null | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-2: subagent_name Some when non-null | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-3: last_assistant_message Some when non-null | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-4: result Some when non-null | `test_BC_2_02_012_subagentstop_all_four_fields_populated` |
| PC-5: canonical agent identity fallback chain | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` |
| PC-6: canonical assistant-message fallback chain | `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains` |
| PC-7: non-SubagentStop fields default to None | `test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none`, `test_BC_2_02_012_ac8_all_event_types_...` |

---

## Consumer Struct Literal Sites Updated

9 sites across 5 files received `agent_type: None, subagent_name: None, last_assistant_message: None, result: None` in GREEN commit `b82adcb`:

| File | Site count |
|---|---|
| `crates/hook-plugins/capture-commit-activity/tests/contract_emit_event.rs` | 3 (lines 28, 126, 202) |
| `crates/hook-plugins/capture-commit-activity/tests/contract_payload_parsing.rs` | 2 (lines 23, 172) |
| `crates/hook-plugins/capture-commit-activity/tests/edge_cases.rs` | 2 (lines 26, 208) |
| `crates/hook-plugins/capture-commit-activity/tests/contract_hook_macro.rs` | 1 (line 27) |
| `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` | 1 (line 201) |

---

## Mutation Testing Flag

`mutation_testing_required: true` is registered in
`.factory/cycles/v1.0-brownfield-backfill/S-8.30/implementation/red-gate-log.md`
per Option B remediation (full_exception_path: true — stub over-delivered).
Wave gate must run `cargo mutants -p vsdd-hook-sdk` as compensating control.

See `supplemental-bc-postconditions-and-mutation.md` for full details.

---

## HOST_ABI_VERSION Verification

```
crates/hook-sdk/src/lib.rs:58:pub const HOST_ABI_VERSION: u32 = 1;
crates/factory-dispatcher/src/lib.rs:43:pub const HOST_ABI_VERSION: u32 = 1;
```

No bump. Additive HookPayload extension under D-6 Option A and D-183.

---

## Files in This Evidence Directory

| File | Purpose |
|---|---|
| `evidence-report.md` | This file — top-level coverage summary |
| `AC-1.md` | Struct definition excerpt (4 new fields) |
| `AC-2.md` | Backward-compat test (PreToolUse → all None) |
| `AC-3.md` | SubagentStop happy-path deserialization test |
| `AC-4.md` | JSON null parity + fallback chain tests |
| `AC-5.md` | Serde round-trip preservation test |
| `AC-6.md` | HOST_ABI.md SubagentStop section excerpt + 5 doc tests |
| `AC-7.md` | HOST_ABI_VERSION = 1 grep + test |
| `AC-8.md` | All 5 event types deserialization test |
| `supplemental-bc-postconditions-and-mutation.md` | PC/Inv coverage map, mutation flag, consumer sites |
