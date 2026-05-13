---
document_type: red-gate-log
story_id: S-12.08
step: 2
phase: implementation
cycle: v1.0-feature-engine-discipline-pass-1
timestamp: 2026-05-10T00:00:00Z
status: RED_GATE_VERIFIED
commit: 354dddcd
---

# Red Gate Log — S-12.08 Step 2

## Summary

RED gate verified. 3 of 12 new tests FAIL (hook_logic-level integration
tests and old-function-removed assertion). All 9 should_panic tests pass
correctly because todo!() panics as expected. All 36 pre-existing tests
remain GREEN.

## Test Execution Results

```
running 48 tests
test result: FAILED. 45 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

**Pre-existing tests: 36 GREEN (no regression)**
**New tests: 12 total — 9 GREEN (by design), 3 RED**

### RED failures (correct failure mode — assertion errors, not build errors)

1. test_hook_logic_blocks_when_wave_context_absent
   FAILS: hook_logic returns Continue (old path), expected Block with WAVE_CONTEXT_MISSING

2. test_hook_logic_blocks_when_stories_wrong_type
   FAILS: hook_logic returns Continue (old path), expected Block with WAVE_CONTEXT_SCHEMA_ERROR

3. test_old_extract_stories_from_config_removed
   FAILS: pub fn extract_stories_from_config( still present in lib.rs

### Bats tests: 2 skip'd with full implementation skeletons for Step 3

## AC Coverage at Step 2

AC-001: should_panic GREEN (todo!() panics)
AC-002a/b/c: should_panic GREEN (todo!() panics)
AC-002-int: RED (hook_logic still uses old path)
AC-003: should_panic GREEN (todo!() panics)
AC-003-int: RED (hook_logic still uses old path)
AC-005: GREEN (needs_context added in Step 1)
AC-006: should_panic GREEN (todo!() panics)
AC-007: GREEN (FakeCallbacks bypasses plugin_config shape)
AC-008/009: bats skip with skeleton
AC-010: RED (old fn still present)
constants: GREEN-BY-DESIGN

## Handoff to Implementer (Step 3)

Three implementation tasks, in order:
1. Implement extract_stories_from_wave_context (remove todo!())
2. Rewire hook_logic to use extract_stories_from_wave_context (not old path)
3. Remove extract_stories_from_config and all call sites

After Step 3, also:
- Remove #[should_panic] from the 6 unit tests (they'll pass normally)
- Remove skip directives from bats and implement test bodies
