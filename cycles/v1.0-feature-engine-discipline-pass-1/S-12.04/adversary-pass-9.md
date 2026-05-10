# Adversarial Review — S-12.04 Pass 9

## Metadata
- Story: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation
- Branch SHA reviewed: 1f51a26b
- Pass: 9
- Reviewer: adversary (fresh context)
- Classification: NITPICK_ONLY
- Within-story finding count: 2
- Recommendation: ADVANCE — pass-9 converges cleanly; passes_clean=1 (1st of 3 required NITPICK_ONLY).

## Findings

### F-S12.04-P9-001 — NITPICK — resolver.error integration test omits explicit provenance-triplet positive-coverage assertions
**Files:** resolver_error_isolation_test.rs:273-303 (test_BC_4_12_004_trapping_resolver_emits_resolver_error_event)
**Description:** Test asserts `resolver.error`, `error_kind`, `resolver_name` (via "trap-resolver" literal), and `error_detail` field key. Does NOT explicitly assert trace_id/session_id/plugin_name literals (e.g., "sess-trap-test", "trace-trap-test", "trap-event-hook"). Compare to resolver.not_found (F-P7-003) and resolver.merge_collision (F-P8-002) which DO assert literals.
**Why nitpick:** The provenance wiring is shared across all 3 callbacks at executor.rs — regression in one would be caught by the other tests. Functional adequacy met for AC-011.
**Suggested:** Add 3 `assert!(all_log_content.contains(literal))` lines for parallelism. Post-convergence polish.

### F-S12.04-P9-002 — NITPICK — resolver.registry_loaded + load_warning lack dedicated startup-path integration tests for provenance literals
**Files:** main.rs:316-326 (registry_loaded), 331-336 (load_warning); resolver_load_test.rs:1095-1131 (F-P4-004)
**Description:** F-P4-004 test mirrors main.rs emission inline rather than exercising main.rs's actual code path. A regression in main.rs's with_trace_id/with_session_id wiring would not be caught. No equivalent test for resolver.registry_loaded.
**Why nitpick:** Both events emitted from main() which is end-to-end exercised via cargo test suite. HOST_ABI tables present. Field structure straightforward. Unit-test mirror sufficient for AC coverage, just not defense-in-depth.
**Suggested:** Future polish PR — `cargo test --test main_startup` integration that boots dispatcher with malformed registry. Post-convergence.

## Cross-cutting observations

Pass-8 fixes verified clean:
1. HOST_ABI field tables — All 6 resolver-tier events have field tables. Provenance triplet present where applicable; load_warning + load_error correctly omit plugin_name (no specific dispatch context at startup).
2. Field naming alignment — `grep "with_field(\"error\""` and `grep "with_field(\"detail\""` both ZERO matches workspace-wide. All 4 emission sites use canonical error_detail.
3. P-005 convention codified — executor.rs:418-442 has full 6-event enumeration with cross-file annotations + S-7.01 pointer + grep recipe.
4. Integration test provenance assertions — not_found (F-P7-003) and merge_collision (F-P8-002) both have explicit trace_id/session_id/plugin_name literal assertions.

Anchor checks: BC-1.13.001, BC-4.12.001, BC-4.12.003, BC-4.12.004 referenced consistently. VP-073, VP-074 anchored. subsystems [SS-01, SS-04] correct. target_module crates/factory-dispatcher matches all touched files.

Architecture compliance:
- Rule 1 (compilation startup-time only): Module::from_file only in resolver_loader.rs ✓
- Rule 2 (Store-per-call): per-dispatch Store creation ✓
- Rule 4 (no unwrap on resolver result): clean ✓
- Rule 5 (absent registry never an error): Ok((empty, vec![])) for absent files ✓

## Convergence assessment

- Within-story findings: 2 (both NITPICK)
- Severity floor: NITPICK
- Classification: NITPICK_ONLY
- Reasoning: Pass-8 exhaustive sweep was thorough; ALL 4 fixes propagated correctly. The convention codification at executor.rs:418-442 with explicit grep recipe is strong S-7.01 sibling-coverage discipline. Two findings are minor parallelism polish, not substantive defects.
- passes_clean transitions 0 → 1.
- Recommendation: ADVANCE to pass-10. If pass-10 + pass-11 also NITPICK_ONLY, S-12.04 satisfies BC-5.39.001 convergence at pass-11.
