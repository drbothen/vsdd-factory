---
document_type: red-gate-log
level: ops
version: "1.0"
status: verified
producer: test-writer
timestamp: 2026-07-25T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.04-story-worktree-write-path-discipline.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
input-hash: "55904fb"
traces_to: "BC-6.26.001 v1.3"
stub_architect_agent: "N/A — no code stubs (skill-doc + bats story; ADR-031 §Decision 4 class; POLICY 21 satisfied)"
stub_compile_verified: true
test_writer_agent: vsdd-factory:test-writer
red_gate_verified: true
---

# Red Gate Log — S-21.04 (story-worktree write-path discipline and teardown preflight)

**Date:** 2026-07-24
**Branch:** feature/S-21.04-story-worktree-write-path-discipline @ 8e3c432e (failing-tests commit; base 948f0fb1)
**Test Writer:** vsdd-factory:test-writer
**Status:** RED_GATE_VERIFIED

## Summary

| Story | New Tests Written | All New Tests Fail (Red)? | Pre-existing Tests | Gate |
|-------|------------------|--------------------------|-------------------|------|
| S-21.04 | 3 bats | YES — all 3 FAIL | 2265 (cargo baseline) | PASSED |

Orchestrator-verified 2026-07-24: all 3 bats tests `not ok` (ASSERTION failures via `_assert_doc_marker`; DOC-PARITY: `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1). Pre-implementation cargo-test baseline: 2265 pass, 0 fail, clean build.

## Stubs Created

### S-21.04: story-worktree write-path discipline and teardown preflight

Stub commit: `63b7fb79` (bats skeleton + `plugins/vsdd-factory/tests/fixtures/story-worktree/README.md`; 3 `skip` placeholders). N/A for code stubs — S-21.04 File Structure Requirements contain no code modules. Deliverables are skill-doc amendments (`_shared-context.md` + `step-g-cleanup.md`) plus one bats suite. ADR-031 §Decision 4 skill-doc mandate; POLICY 21 satisfied — no new `.sh` files added (no new executable scripts; existing fixture convention used). Workspace unchanged; `cargo check` trivially green.

Failing-tests commit: `8e3c432e` — replaced the 3 `skip` placeholders with assertion-bearing tests referencing absent step-g-cleanup.md §G.1 preflight mandate.

## Red Gate Verification

**Command:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`

**Result:** RED GATE PASSED. Output: `1..3` — all 3 `not ok`. Each test fails at `_assert_doc_marker` (DOC-PARITY) assertions referencing the absent §G.1 teardown preflight mandate in `step-g-cleanup.md`. Zero bash errors; zero skips.

### Bats Tests (`plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, commit 8e3c432e)

| Test | AC / RG ID | BC Trace | Fixture | Failure Reason | Status |
|------|-----------|----------|---------|----------------|--------|
| T-001 | AC-003 / RG-003 | BC-6.26.001 PC2b, Invariant 2 | fixtures/story-worktree/ (stray `.factory/stories/S-021-DELIVERY.md`) | `_assert_doc_marker` gate fires — `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1; `PREFLIGHT BLOCKED` clause not present | FAIL (expected) |
| T-002 | AC-004 / RG-004 | BC-6.26.001 PC2a | fixtures/story-worktree/ (empty shadow `.factory/`) | `_assert_doc_marker` gate fires — §G.1 clean-path assertion absent from step-g-cleanup.md | FAIL (expected) |
| T-003 | AC-005 / RG-005 | BC-6.26.001 PC2b→PC2a retry path | fixtures/story-worktree/ (stray file then relocated) | `_assert_doc_marker` gate fires — §G.1 retry-path mandate absent from step-g-cleanup.md | FAIL (expected) |

## Anti-Tautology Mechanism (TD-VSDD-059)

`_extract_g1_section` awk extraction gate: the harness extracts §G.1 from `step-g-cleanup.md` at test setup time and executes only doc-extracted preflight logic. Pre-implementation extraction gate fires "preflight mandate absent" — the awk extraction finds no §G.1 teardown-preflight section, so the gate itself confirms the absence rather than executing stub code that might vacuously pass. Verified present: 5 references to `_extract_g1_section` in the bats suite at commit `8e3c432e`.

This mechanism guarantees non-tautology: no amount of fixture manipulation can cause T-001/T-002/T-003 to pass until `step-g-cleanup.md` §G.1 contains the actual preflight mandate text.

## Mutant Vector (POLICY 15 v1.4.10 per-guard mutant verification)

**T-001 mutant vector:** stray file `.factory/stories/S-021-DELIVERY.md` in fixture story-worktree + load-bearing `$REMOVE_LOG` sentinel assertion `[ ! -s ]`. Mutant: omitting the `[ ! -s $REMOVE_LOG ]` assertion allows a passthrough where `git worktree remove` is called even when the stray file is present. The `$REMOVE_LOG` sentinel records whether the remove was invoked; the `[ ! -s ]` check (file not non-empty) is load-bearing. Removing this assertion would cause T-001 to pass falsely post-implementation if the BLOCKED path is not wired.

## Regression Check

| Existing Tests | Status |
|---------------|--------|
| 2265 pre-existing cargo tests (pre-implementation baseline) | all pass |

Zero regressions. Bats suite is additive; no existing bats fixture modified. No Rust crate changes at stub or failing-tests commits.

## Failure Mode Verification

All three tests fail via DOC-PARITY `_assert_doc_marker` assertions — the tests extract §G.1 from `step-g-cleanup.md` via the `_extract_g1_section` awk gate; the section does not exist pre-implementation, causing the extraction gate to fire "preflight mandate absent" rather than executing any preflight logic. Tests will turn green only once the implementation amendments land in `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` (AC-003/AC-004/AC-005) and `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` (AC-001/AC-002, Write Discipline clause).

No `#[should_panic]` masking. No vacuously-passing new tests. Failure mechanism is behavioral (gate absence / DOC-PARITY extraction), not infrastructure panic.

## Traces

- T-001 (AC-003 / RG-003) → BC-6.26.001 v1.3 PC2b (Invariant 2): stray `.factory/` file triggers PREFLIGHT BLOCKED; `git worktree remove` NOT called
- T-002 (AC-004 / RG-004) → BC-6.26.001 v1.3 PC2a: empty shadow tree → teardown proceeds; `git worktree remove` IS called
- T-003 (AC-005 / RG-005) → BC-6.26.001 v1.3 PC2b→PC2a retry path: stray file relocated → preflight re-runs clean → teardown proceeds

## Commits

- `63b7fb79` — stub commit: bats skeleton (3 `skip` placeholders) + `fixtures/story-worktree/README.md`
- `8e3c432e` — failing-tests commit: `test(S-21.04): add failing tests for BC-6.26.001 PC2a/PC2b teardown preflight and PC1 write discipline` — 3 skips replaced with `_assert_doc_marker` assertion tests; Red Gate PASSED

## Hand-Off to Implementer

Story ready for implementation. No dependency gates outstanding (`depends_on: []`).

Implementation tasks (from story spec):

1. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`** — add "Write Discipline" clause under §Spec-Path Discipline covering `.factory/**` writes; name DELIVERY ledger + pr-review.md; mandate canonical absolute path resolution via `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel` (BC-6.26.001 PC1 + Invariants 1, 3, 4). Unblocks AC-001/AC-002.
2. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` §G.1** — add mandatory teardown preflight sub-step before `git worktree remove` dispatch; implement PC2a/PC2b logic: `find .worktrees/<story>/.factory -type f` → if non-empty emit `PREFLIGHT BLOCKED` (PC2b); if empty proceed with `git worktree remove` (PC2a); include retry-path documentation (BC-6.26.001 PC2 + Invariant 2 + Invariant 5). Unblocks T-001 (AC-003/RG-003), T-002 (AC-004/RG-004), T-003 (AC-005/RG-005).
3. **Verify bats green:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..3`, all 3 `ok`.
4. **Verify cargo regression clean:** `cargo test --workspace --all-targets` → 2265+ pass, 0 fail.
