# Evidence Report — S-18.04b
## validate-burst-log / validate-dispatch-advance PreCompact Exemption + precompact-flush-prune.sh

Story: S-18.04b v1.8  
Branch: feature/S-18.04b  
Date: 2026-06-25  
Subsystems: SS-04, SS-05, SS-07  
BCs: BC-5.41.003 v2.0  
VPs: VP-084, VP-090  

---

## Demo Format

All evidence uses VHS terminal recordings (CLI product — no UI). Each tape
exercises the real artifacts: Rust unit tests for pure-logic WASM contracts,
bats integration tests for dispatcher end-to-end and prune script behavior.
No plain-text captures are used; all recordings invoke the actual built
artifacts.

---

## Coverage Map: AC to Evidence File

| AC | Description | Evidence File | Format | Test Names |
|----|-------------|---------------|--------|------------|
| AC-001 | Exemption: log present, FIELD-4=commit, SHA match → Continue | `AC-001-003-exemption-cases-abc.gif/.webm/.tape` | VHS | `test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt` |
| AC-002 | Exemption: log absent → prefix-match-only Continue | `AC-001-003-exemption-cases-abc.gif/.webm/.tape` | VHS | `test_BC_5_41_003_precompact_prefix_log_absent_exempt` |
| AC-003 | Exemption: FIELD-4 corrupted → treat as absent → Continue | `AC-001-003-exemption-cases-abc.gif/.webm/.tape` | VHS | `test_BC_5_41_003_precompact_prefix_log_corrupted_exempt` |
| AC-004 | SHA mismatch with valid FIELD-4 → NOT exempt → MULTI_COMMIT_CHAIN_NOT_ALLOWED | `AC-004-sha-mismatch-not-exempt.gif/.webm/.tape` | VHS | `test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt`, `test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt` |
| AC-005 | Case-sensitive prefix: `precompact flush` (lowercase) NOT exempt | `AC-005-case-sensitive-prefix.gif/.webm/.tape` | VHS | `test_BC_5_41_003_precompact_prefix_case_sensitive`, `test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt` |
| AC-006 | Symmetric implementation: validate-dispatch-advance identical 3-case logic; exec-free via git_context | `AC-006-symmetry-dispatch-advance.gif/.webm/.tape` | VHS | `test_BC_5_41_003_dispatch_advance_exemption_symmetric`, `test_BC_5_41_003_precompact_flush_prefix_both_crates_identical` |
| AC-007 | VP-084: proof invokes via factory-dispatcher (NOT wasmtime); negative control blocks sentinel chain | `AC-007-vp084-dispatcher-proof.gif/.webm/.tape` | VHS | `vp084-proof.bats` Test 1, 2, 3 (all 3 via real dispatcher) |
| AC-008 | Exact prefix string `PreCompact flush ` (constant exact match) | subsumed by `AC-001-003-exemption-cases-abc` | VHS | `test_BC_5_41_003_precompact_flush_prefix_constant_exact` (passed in AC-001-003 run) |
| AC-009 | prune.sh structural precondition: file not ending with `\n` → exit non-zero, no modification | `AC-009-013-prune-behaviors.gif/.webm/.tape` | VHS | `test_prune_structural_precondition_no_newline`, `test_prune_error_message_on_no_newline` |
| AC-010 | prune.sh threshold: >1000 lines → prune to 500 (tail -n 500, atomic mv) | `AC-009-013-prune-behaviors.gif/.webm/.tape` | VHS | `test_prune_threshold_1001_prunes_to_500` |
| AC-011 | prune.sh atomic write: last line preserved; temp+mv idiom | `AC-009-013-prune-behaviors.gif/.webm/.tape` | VHS | `test_prune_atomic_write_preserves_last_line`, `test_prune_result_ends_with_newline` |
| AC-012 | prune.sh NOT in hooks-registry.toml (invocation context check) | `AC-009-013-prune-behaviors.gif/.webm/.tape` | VHS | `test_prune_not_in_hooks_registry` |
| AC-013 | prune.sh boundary: 0 lines no-op; 1000 lines no prune; 1001 lines prune to 500 | `AC-009-013-prune-behaviors.gif/.webm/.tape` | VHS | `test_prune_empty_file_noop`, `test_prune_threshold_1000_no_prune`, `test_prune_threshold_1001_prunes_to_500`, `test_prune_preserves_first_retained_line` |

---

## Exec-Free Constraint Coverage (ADR-029 §Decision 3)

The exec-free constraint (no `host::exec_subprocess` for commit-context acquisition)
is verified by:

- `test_BC_5_41_003_wiring_exec_free_constraint_documented` (validate-burst-log exemption.rs)
- `test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block` — sentinel subjects
  delivered only via `git_context` JSON in payload.extra; WASM reads them and blocks.
- `test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues` — absent
  git_context → fail-open (skip check, return None).
- `test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block` — Edit events
  do NOT trigger the chain check (Bash-only trigger per ADR-029 §Decision 1).

These are covered by the full exemption test suite visible in AC-001-003 and AC-006 recordings.

---

## Full Test Suite Results (captured at evidence time)

### validate-burst-log exemption.rs — 21 tests, all pass

```
test test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues ... ok
test test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues ... ok
test test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues ... ok
test test_BC_5_41_003_chain_both_sentinel_emits_violation ... ok
test test_BC_5_41_003_chain_head_burst_head_parent_precompact_log_match_no_violation ... ok
test test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block ... ok
test test_BC_5_41_003_chain_both_precompact_no_violation ... ok
test test_BC_5_41_003_chain_precompact_log_absent_exemption_fires ... ok
test test_BC_5_41_003_chain_head_precompact_log_match_head_parent_burst_no_violation ... ok
test test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block ... ok
test test_BC_5_41_003_non_precompact_subject_not_exempt ... ok
test test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt ... ok
test test_BC_5_41_003_precompact_flush_prefix_constant_exact ... ok
test test_BC_5_41_003_precompact_prefix_case_sensitive ... ok
test test_BC_5_41_003_precompact_prefix_log_absent_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_corrupted_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_field4_empty_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt ... ok
test test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt ... ok
test test_BC_5_41_003_wiring_exec_free_constraint_documented ... ok
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### validate-dispatch-advance exemption.rs — 24 tests, all pass

```
test test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues ... ok
test test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues ... ok
test test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues ... ok
test test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block ... ok
test test_BC_5_41_003_chain_both_precompact_no_violation ... ok
test test_BC_5_41_003_chain_head_burst_head_parent_precompact_log_match_no_violation ... ok
test test_BC_5_41_003_chain_head_precompact_log_match_head_parent_burst_no_violation ... ok
test test_BC_5_41_003_chain_precompact_log_absent_exemption_fires ... ok
test test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt ... ok
test test_BC_5_41_003_dispatch_advance_precompact_flush_prefix_accessible ... ok
test test_BC_5_41_003_chain_both_sentinel_emits_violation ... ok
test test_BC_5_41_003_non_precompact_subject_not_exempt ... ok
test test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block ... ok
test test_BC_5_41_003_precompact_flush_prefix_both_crates_identical ... ok
test test_BC_5_41_003_dispatch_advance_exemption_symmetric ... ok
test test_BC_5_41_003_precompact_flush_prefix_constant_exact ... ok
test test_BC_5_41_003_precompact_prefix_case_sensitive ... ok
test test_BC_5_41_003_precompact_prefix_log_absent_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_corrupted_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_field4_empty_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_valid_sha_match_exempt ... ok
test test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt ... ok
test test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt ... ok
test test_BC_5_41_003_wiring_exec_free_constraint_documented ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### vp084-proof.bats — 3 tests, all pass

```
1..3
ok 1 test_vp084_exemption_via_dispatcher_not_wasmtime: validate-burst-log exempts PreCompact via dispatcher
ok 2 test_vp084_dispatch_advance_exemption_via_dispatcher: validate-dispatch-advance exempts PreCompact via dispatcher
ok 3 test_vp084_non_precompact_chain_blocks_via_dispatcher: normal backfill chain triggers MULTI_COMMIT_CHAIN
```

### precompact-flush-prune.bats — 11 tests, all pass

```
1..11
ok 1 test_prune_structural_precondition_no_newline: exits non-zero without modification
ok 2 test_prune_error_message_on_no_newline: emits canonical error message on stderr
ok 3 test_prune_empty_file_noop: empty file exits 0 with no modification
ok 4 test_prune_threshold_1000_no_prune: 1000-line file exits 0 without modification
ok 5 test_prune_threshold_500_no_prune: 500-line file exits 0 without modification
ok 6 test_prune_threshold_1001_prunes_to_500: 1001-line file pruned to 500 lines
ok 7 test_prune_atomic_write_preserves_last_line: last line is unchanged after prune
ok 8 test_prune_result_ends_with_newline: pruned file ends with \n
ok 9 test_prune_preserves_first_retained_line: first line after prune is entry 502 of 1001
ok 10 test_prune_not_in_hooks_registry: precompact-flush-prune.sh not registered as hook
ok 11 test_prune_script_syntax_valid: passes bash syntax check
```

---

## Files in This Directory

| File | Type | Covers |
|------|------|--------|
| `AC-001-003-exemption-cases-abc.gif` | VHS recording | AC-001, AC-002, AC-003 |
| `AC-001-003-exemption-cases-abc.webm` | VHS recording | AC-001, AC-002, AC-003 |
| `AC-001-003-exemption-cases-abc.tape` | VHS source script | AC-001, AC-002, AC-003 |
| `AC-004-sha-mismatch-not-exempt.gif` | VHS recording | AC-004 |
| `AC-004-sha-mismatch-not-exempt.webm` | VHS recording | AC-004 |
| `AC-004-sha-mismatch-not-exempt.tape` | VHS source script | AC-004 |
| `AC-005-case-sensitive-prefix.gif` | VHS recording | AC-005 |
| `AC-005-case-sensitive-prefix.webm` | VHS recording | AC-005 |
| `AC-005-case-sensitive-prefix.tape` | VHS source script | AC-005 |
| `AC-006-symmetry-dispatch-advance.gif` | VHS recording | AC-006 |
| `AC-006-symmetry-dispatch-advance.webm` | VHS recording | AC-006 |
| `AC-006-symmetry-dispatch-advance.tape` | VHS source script | AC-006 |
| `AC-007-vp084-dispatcher-proof.gif` | VHS recording | AC-007 |
| `AC-007-vp084-dispatcher-proof.webm` | VHS recording | AC-007 |
| `AC-007-vp084-dispatcher-proof.tape` | VHS source script | AC-007 |
| `AC-009-013-prune-behaviors.gif` | VHS recording | AC-009 through AC-013 |
| `AC-009-013-prune-behaviors.webm` | VHS recording | AC-009 through AC-013 |
| `AC-009-013-prune-behaviors.tape` | VHS source script | AC-009 through AC-013 |
| `evidence-report.md` | This file | All ACs |

Note: AC-008 (exact prefix string) is covered within `AC-001-003-exemption-cases-abc`
by `test_BC_5_41_003_precompact_flush_prefix_constant_exact` which passes in the
same test run. AC-012 (not in hooks-registry) is covered within `AC-009-013-prune-behaviors`
by `test_prune_not_in_hooks_registry`.
