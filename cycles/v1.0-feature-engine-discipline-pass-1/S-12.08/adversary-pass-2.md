---
document_type: adversary-review
level: ops
version: "1.0"
status: draft
producer: adversary
timestamp: 2026-05-11T04:00:00Z
phase: engine-discipline-F4-S-12.08
inputs:
  - story_spec_v1.1
  - bcs (BC-1.13.001, BC-4.10.001 v1.3, BC-4.12.001..005, BC-8.14.009)
  - vps (VP-071, VP-073..076)
  - ADR-017, ADR-018
  - branch_diff (origin/develop..7cf280f2)
  - factory-artifacts (a30ddef3)
input-hash: "[pending-recompute]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-12.08 adversary pass-2 review (fresh context)"
current_cycle: v1.0-feature-engine-discipline-pass-1
story_id: S-12.08
pass: 2
previous_review: adversary-pass-1.md
verdict: MEDIUM
findings_count: { critical: 0, high: 1, medium: 3, low: 2, nitpick: 0 }
deferred_findings: 1
convergence_reached: false
---

# S-12.08 Adversary Pass-2 (Fresh Context)

## Finding ID Convention

`ADV-BF1-P02-<SEV>-<SEQ>`.

## Severity Verdict

**MEDIUM.** Pass-1 fixes all verified in-place. Pass-2 fresh context discovers 1 NEW HIGH (partial-fix-regression: cycle_id Block branch has zero unit test coverage), 3 MEDIUM (rustdoc drift, null boundary test gap, missing bats empty-wave case), 2 LOW (stale narration). NOT NITPICK_ONLY -> streak NOT yet started.

## Part A — Pass-1 Fix Verification (S-7.01)

All pass-1 fixes verified:
- HIGH-001 ✓ BC-4.10.001 v1.3 PC9/PC10
- HIGH-002 ✓ resolver emits Some({stories:[],...}) for empty active wave
- HIGH-003 ✓ is_object() guard + 4-type test
- MED-001 ✓ cycle_id-absent Block (production verified, **test gap ESCALATED to P02-HIGH-001**)
- MED-002 ✓ data-shape pin opt-out
- MED-003 ✓ canonical block-format regression test
- MED-004 ✓ bats setup() pre-builds dispatcher
- MED-005 ✓ gate_status seeded
- MED-006 ✓ test renamed
- LOW-001 ✓ resolver_name now used (or removed); not applicable
- LOW-003 ✓ RESOLVER_TIMEOUT_MS doc-comment

## Part B — New Findings

### HIGH

#### ADV-BF1-P02-HIGH-001: MED-001 cycle_id-absent Block branch has zero unit test coverage

- **Files:** lib.rs:473-499 (production branch); test module 832-2533 (no test)
- **Evidence:** Pass-1 MED-001 added Block path for cycle_id absent/empty. Grep for tests passing None cycle_id returns zero. AC-002-int and AC-003-int hit Block via extract_stories BEFORE cycle_id branch. New code is **unverified**.
- **Severity rationale:** Partial-Fix Regression Discipline violation (S-7.01). New code path without test. DoD says "all unit tests pass GREEN" but new branch has no test to pass or fail.
- **Fix:** Add `test_hook_logic_blocks_when_cycle_id_absent` (pass None) + `test_hook_logic_blocks_when_cycle_id_empty_string` (pass `""`). Assert Block + WAVE_CONTEXT_SCHEMA_ERROR code + cycle_id message.

### MEDIUM

#### ADV-BF1-P02-MED-001: hook_logic rustdoc (lib.rs:152-158, 408-417) doesn't document EC-001 Continue path

- **File:** lib.rs:152-158 + 408-417
- **Evidence:** Production code at 502-508 returns Continue for empty-stories (BC-4.10.001 EC-001). Rustdoc describes Block-on-malformed but never mentions EC-001 vacuous-convergence Continue.
- **Fix:** Update hook_logic rustdoc to add "If story list is empty (active wave with zero stories) -> Continue (EC-001 vacuous convergence)" step.

#### ADV-BF1-P02-MED-002: HIGH-003 test missing null boundary

- **File:** lib.rs:2514-2532 test_non_object_wave_context_returns_schema_error
- **Evidence:** Covers string/number/bool/array but not null. Production correctly distinguishes null->Missing vs non-object->SchemaError but the boundary isn't pinned.
- **Fix:** Add assertion `{"wave_context": null}` returns Missing (not SchemaError) inside the test, with comment documenting boundary.

#### ADV-BF1-P02-MED-003: Bats lacks empty-active-wave end-to-end case (HIGH-002 fix has no integration proof)

- **File:** plugins/vsdd-factory/tests/resolver-integration.bats
- **Evidence:** Pass-1 HIGH-002 changed resolver semantics. Unit tests cover both sides. Bats only tests unconverged (Block) + converged (Continue). Empty-active-wave Continue case is NOT exercised end-to-end. F-P2-001 closure evidence weakened.
- **Fix:** Add third bats case: empty stories array seeded -> assert exit 0 + no WAVE_CONTEXT_MISSING in output.

### LOW

#### ADV-BF1-P02-LOW-001: lib.rs:285-291 stale rustdoc references removed extract_stories_from_config

- **File:** lib.rs:285-291
- **Evidence:** Doc says "RealCallbacks::list_stories WILL call this function instead of extract_stories_from_config" (future-tense; refactor is complete; both functions removed).
- **Fix:** Rewrite to present-tense referencing canonical path.

#### ADV-BF1-P02-LOW-002: Stale RED/GREEN narration in test docstrings

- **File:** lib.rs:2336-2370 + 2380-2411
- **Evidence:** Tests carry "RED at Step 2 / GREEN at Step 3" historical commentary. Cosmetic; may be intentional documentation of TDD ladder.
- **Fix:** Either strip RED/GREEN sentences OR add trailer "Status: GREEN as of S-12.08 Step 3".

## Deferred Cross-Story Findings

- **DEFER-P02-001 (cross-story to wave-gate):** S-12.07 spec line 88 still reads "active wave with empty stories -> value: None". S-12.07 test plan table at line 341 references renamed `test_BC_4_12_002_empty_stories_yields_none` (now `test_resolve_pure_with_empty_stories_emits_some_with_empty_array`). S-12.07 demo evidence at `docs/demo-evidence/S-12.07/AC-003-empty-waves-yields-none.txt` lines 14, 20, 41 + `evidence-report.md:43` cite old behavior. Pass-1 HIGH-002 changed resolver behavior but S-12.07 artifacts retain old narrative. **Route to wave-gate or pre-emptively fix in pass-3 burst.**

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 3 |
| LOW | 2 |
| NITPICK | 0 |
| Deferred | 1 |
| **Total** | **7** |

## Novelty Assessment

HIGH. All findings are genuinely new. HIGH-001 is the classic Partial-Fix-Regression catch (added code without test). MED-001 is doc-code drift in same diff. MED-003 is the missing integration proof for HIGH-002 fix. None reworded from pass-1.

## Convergence

`convergence_reached`: false. P1 MEDIUM -> P2 MEDIUM (held). Per BC-5.39.001 need 3 consecutive NITPICK_ONLY for convergence; current streak 0/3.

## R-PLAT-004 Self-Check

PASS. Pass-2 review reads code/spec directly; doesn't invoke hook on itself. Bats integration test is the recursive-bootstrap forcing function.

## Process-Gap Findings

None new in pass-2 (PG-1/PG-2 from pass-1 carry over).
