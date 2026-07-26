---
document_type: red-gate-log
level: ops
version: "1.8"
status: verified
producer: test-writer
timestamp: 2026-07-25T03:15:00Z
phase: 3
inputs:
  - .factory/stories/S-21.04-story-worktree-write-path-discipline.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
input-hash: "455740d"
traces_to: "BC-6.26.001 v1.11"
last_amended: "2026-07-26 D-906: F-S2104-P10-007 Summary line HEAD cite 9d896bf5→2c8eff8b + suite-level verification (orchestrator ran bats → 9/9 ok + 14/14 ok at 2c8eff8b, 2026-07-26); F-S2104-P10-008 §T-009 mutant evidence line :285 named list (5 named gates + 8 scratch mutants); F-S2104-P10-009 traces_to v1.10→v1.11 + BC-6.26.001 cite v1.10→v1.11 throughout + T-008/T-009 version pin parity restored; input-hash 389274b→455740d (story v1.14 + BC v1.11 drift); version 1.7→1.8; prior: 2026-07-25 D-905: F-S2104-P9-004 §T-008 BC trace corrected (PC2 + Invariant 2 caller-side dispatch gate, per story AC-007; replaces erroneous Invariant 5 caller-side propagation anchor introduced in D-904 fix wave); F-S2104-P9-007 §T-009 mutant evidence recorded (obligation-asserting gates mutant-proven at 2992b53d; class-completed at 3326e4dd; bare-alternation paper-gate confirmed); input-hash 43e6df2→389274b (story v1.13 drift); version 1.6→1.7 (F-S2104-P9-004, F-S2104-P9-007); prior: 2026-07-25 D-904: F-S2104-P8-001 §Bats Tests table T-001→RG-001†, T-002→RG-002†, T-003→RG-003† (table correction omitted at D-895, surviving 6 passes); † footnote updated to cite D-904; F-S2104-P8-007 T-007 mutant-proof recorded verbatim (qualified-path/verify/PASS-result/not-evident-run-yourself all NO MATCH on scratch reduction at 052620dc); F-S2104-P8-002 NEW T-008 addendum (AC-007/RG-008; six-surface §G.1 mandate; quote-tolerant mutant-proof 052620dc) + NEW T-009 addendum (AC-009/RG-009; awareness-clause; GREEN 4265c96c); Summary line updated (T-007/T-008/T-009 propagation gates; HEAD 9d896bf5; all T-IDs/RG rows confirmed); BC-6.26.001 cite v1.9→v1.10 (quintuple parity); input-hash 7abb656→43e6df2 (story v1.12 + BC v1.10 drift); traces_to v1.9→v1.10; version 1.5→1.6 (F-S2104-P8-001, F-S2104-P8-002, F-S2104-P8-007); prior: 2026-07-25 D-903 state-manager — F-S2104-P7-001/P7-005 attestation corrections verbatim-authored by orchestrator: §T-005 addendum heading/Test rewritten (regular-file-at-path → PC2b BLOCKED; RG-006; AC-002/EC-007); §T-006 addendum heading/trailing-slash mechanism corrected (POSIX find WITHOUT -H/-L empty-return → false PC2a; [ -L ] guard routes to PC2b; trailing-slash defense-in-depth); all 4 RG-004a occurrences replaced with RG-006; §Traces T-005/T-006 quintuple parity updated; NEW §T-007 addendum (AC-008/RG-007 devops-engineer executor-side preflight mandate; obligation-asserting at 052620dc); Summary line: 9 bats tests T-001..T-006 + 3 propagation-gate tests; BC-6.26.001 v1.8→v1.9 cites updated throughout; input-hash 4b75dba→7abb656 (story v1.11 + BC v1.9 drift); traces_to v1.8→v1.9; version 1.4→1.5 (F-S2104-P7-001, F-S2104-P7-005); prior: 2026-07-25 D-902 state-manager — T-005/T-006/RG-005 attestation addenda appended; Summary 9-test suite; Traces quintuple parity v1.5→v1.8; T-004 addendum BC cite v1.5→v1.8; RG-reconciliation note appended to D-895 erratum; input-hash 8cdfb33→a4b9ea5 (story v1.9 + BC v1.8 drift); traces_to v1.6→v1.8; version 1.3→1.4 (F-S2104-P6-004); prior: 2026-07-25 D-899 state-manager — T-004 test description corrected to verbatim §G.1 PC2c semantics (HALT + surface exit code/stderr; no PREFLIGHT BLOCKED message); input-hash 2b051ec→8cdfb33 (story v1.7 drift); traces_to v1.5→v1.6; version 1.2→1.3 (F-S2104-P4-006); prior: 2026-07-25 D-897 state-manager — fabricated PC2c implementation quote corrected to verbatim §G.1 text; Invariant TBD placeholder removed; input-hash 55904fb→2b051ec (story v1.6 drift); version 1.1→1.2 (F-S2104-P3-009, F-S2104-P3-010); prior: 2026-07-25 D-896 state-manager — T-004/RG-004 attestation addendum (F-S2104-P2-013) + quintuple parity v1.5 (F-S2104-P2-017); prior: 2026-07-25 D-895 state-manager — erratum F-S2104-P1-009 (RG-ID mapping + AC-002 attribution)"
modified:
  - "2026-07-25 D-895: Erratum appended — RG-ID mapping corrected (RG-001/002/003), fabricated RG-004/005 documented, AC-002 attribution corrected (F-S2104-P1-009)"
  - "2026-07-25 D-896: T-004/RG-004 attestation addendum appended; frontmatter version 1.0→1.1, traces_to updated to v1.5, §Traces BC cites updated to v1.5 (F-S2104-P2-013, F-S2104-P2-017)"
  - "2026-07-25 D-897: Fabricated PC2c implementation quote corrected to verbatim §G.1 text; Invariant TBD placeholder removed; input-hash 55904fb→2b051ec (story v1.6 drift correction); version 1.1→1.2 (F-S2104-P3-009, F-S2104-P3-010)"
  - "2026-07-25 D-899: T-004 test description corrected to verbatim §G.1 PC2c semantics; input-hash 2b051ec→8cdfb33 (story v1.7 drift); traces_to v1.5→v1.6; version 1.2→1.3 (F-S2104-P4-006)"
  - "2026-07-25 D-902: T-005/T-006/RG-005 attestation addenda appended; Summary updated to 9-test suite; Traces quintuple parity v1.5→v1.8; T-004 addendum BC cite updated to v1.8; RG-reconciliation note appended to D-895 erratum; input-hash 8cdfb33→4b75dba (story v1.9 + BC v1.8 drift); traces_to v1.6→v1.8; version 1.3→1.4 (F-S2104-P6-004)"
  - "2026-07-25 D-903: F-S2104-P7-001/P7-005 attestation corrections verbatim-authored by orchestrator; §T-005 rewritten (regular-file-at-path → PC2b BLOCKED; RG-006; AC-002/EC-007); §T-006 trailing-slash mechanism corrected; all 4 RG-004a→RG-006; §Traces quintuple parity updated; NEW §T-007 addendum (AC-008/RG-007); Summary line updated; BC v1.8→v1.9 cites; input-hash 4b75dba→7abb656 (story v1.11 + BC v1.9 drift); traces_to v1.8→v1.9; version 1.4→1.5 (F-S2104-P7-001, F-S2104-P7-005)"
  - "2026-07-25 D-904: F-S2104-P8-001 §Bats Tests table RG corrected (T-001→RG-001†, T-002→RG-002†, T-003→RG-003†); † footnote updated to cite D-904; F-S2104-P8-007 T-007 mutant-proof recorded verbatim; NEW T-008 + T-009 addenda (F-S2104-P8-002); Summary HEAD 3c3788d7→9d896bf5; BC-6.26.001 cite v1.9→v1.10 (quintuple parity); input-hash 7abb656→43e6df2 (story v1.12 + BC v1.10 drift); traces_to v1.9→v1.10; version 1.5→1.6 (F-S2104-P8-001, F-S2104-P8-002, F-S2104-P8-007)"
  - "2026-07-25 D-905: F-S2104-P9-004 §T-008 BC trace corrected (PC2 + Invariant 2 caller-side dispatch gate, per story AC-007); F-S2104-P9-007 §T-009 mutant evidence recorded verbatim; input-hash 43e6df2→389274b (story v1.13 drift); version 1.6→1.7"
  - "2026-07-26 D-906: F-S2104-P10-007 Summary HEAD 9d896bf5→2c8eff8b + suite-level verification (bats 9/9+14/14 at 2c8eff8b 2026-07-26); F-S2104-P10-008 §T-009 mutant line :285 unnamed→named; F-S2104-P10-009 BC-6.26.001 cite v1.10→v1.11 throughout; T-008/T-009 version-pin parity restored; traces_to v1.10→v1.11; input-hash 389274b→455740d; version 1.7→1.8"
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
| S-21.04 | 9 bats tests: T-001..T-006 (behavioral vectors) + T-007/T-008/T-009 (doc-parity propagation gates). All GREEN at worktree HEAD 2c8eff8b (suite-level verification: orchestrator ran bats story-worktree-write-path-discipline.bats → 9/9 ok and worktree-identity-preflight.bats → 14/14 ok at 2c8eff8b, 2026-07-26). All nine tests carry T-IDs, RG rows (RG-001..RG-009), and attestation sections in this log. | YES — all original 3 FAIL at Red Gate | 2265 (cargo baseline) | PASSED |

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
| T-001 | AC-003 / RG-001† | BC-6.26.001 PC2b, Invariant 2 | fixtures/story-worktree/ (stray `.factory/stories/S-021-DELIVERY.md`) | `_assert_doc_marker` gate fires — `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1; `PREFLIGHT BLOCKED` clause not present | FAIL (expected) |
| T-002 | AC-004 / RG-002† | BC-6.26.001 PC2a | fixtures/story-worktree/ (empty shadow `.factory/`) | `_assert_doc_marker` gate fires — §G.1 clean-path assertion absent from step-g-cleanup.md | FAIL (expected) |
| T-003 | AC-005 / RG-003† | BC-6.26.001 PC2b→PC2a retry path | fixtures/story-worktree/ (stray file then relocated) | `_assert_doc_marker` gate fires — §G.1 retry-path mandate absent from step-g-cleanup.md | FAIL (expected) |

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

- T-001 (AC-003 / RG-001†) → BC-6.26.001 v1.11 PC2b (Invariant 2): stray `.factory/` file triggers PREFLIGHT BLOCKED; `git worktree remove` NOT called
- T-002 (AC-004 / RG-002†) → BC-6.26.001 v1.11 PC2a: empty shadow tree → teardown proceeds; `git worktree remove` IS called
- T-003 (AC-005 / RG-003†) → BC-6.26.001 v1.11 PC2b→PC2a retry path: stray file relocated → preflight re-runs clean → teardown proceeds
- T-004 (AC-006 / RG-004) → BC-6.26.001 v1.11 PC2c: non-path-absent find error → fail-closed HALT; `git worktree remove` NOT called (addendum D-896)
- T-005 → AC-002 (EC-007) / RG-006 / BC-6.26.001 PC2b (non-directory inode)
- T-006 → AC-002 (EC-008) / RG-005 / BC-6.26.001 PC2b (symlink at path)

† applies to this table and §Traces: original log fabricated RG-003/004/005 for T-001..003; corrected to RG-001/002/003 (D-895 §Traces; D-904 this table — the table correction was omitted at D-895, surviving six passes; see F-S2104-P8-001).

## Commits

- `63b7fb79` — stub commit: bats skeleton (3 `skip` placeholders) + `fixtures/story-worktree/README.md`
- `8e3c432e` — failing-tests commit: `test(S-21.04): add failing tests for BC-6.26.001 PC2a/PC2b teardown preflight and PC1 write discipline` — 3 skips replaced with `_assert_doc_marker` assertion tests; Red Gate PASSED

## Hand-Off to Implementer

Story ready for implementation. No dependency gates outstanding (`depends_on: []`).

Implementation tasks (from story spec):

1. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`** — add "Write Discipline" clause under §Spec-Path Discipline covering `.factory/**` writes; name DELIVERY ledger + pr-review.md; mandate canonical absolute path resolution via `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel` (BC-6.26.001 PC1 + Invariants 1, 3, 4). Unblocks AC-001.
2. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` §G.1** — add mandatory teardown preflight sub-step before `git worktree remove` dispatch; implement PC2a/PC2b logic: `find .worktrees/<story>/.factory -type f` → if non-empty emit `PREFLIGHT BLOCKED` (PC2b); if empty proceed with `git worktree remove` (PC2a); include retry-path documentation (BC-6.26.001 PC2 + Invariant 2 + Invariant 5). Unblocks AC-002, T-001 (AC-003/RG-001), T-002 (AC-004/RG-002), T-003 (AC-005/RG-003).
3. **Verify bats green:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..3`, all 3 `ok`.
4. **Verify cargo regression clean:** `cargo test --workspace --all-targets` → 2265+ pass, 0 fail.

---

## Erratum (F-S2104-P1-009) — RG-ID Mapping and AC-002 Attribution Correction

**Appended:** 2026-07-25 (D-895 S-21.04 pass-1 closure; state-manager)

The original version of this red-gate-log contained three defects identified by the pass-1 adversarial review as F-S2104-P1-009 (HIGH):

### Defect 1 — Fabricated RG-004 and RG-005 IDs

The Bats Tests table and Traces section used `RG-004` and `RG-005` as Red Gate identifiers. The story's Red Gate Test Plan (BC-6.26.001 v1.3) defines **RG-001, RG-002, RG-003** only. RG-004 and RG-005 do not exist in the story SoT.

### Defect 2 — T-001 mis-mapped to RG-003 (should be RG-001)

The Bats Tests table showed `T-001 | AC-003 / RG-003` and Traces showed `T-001 (AC-003 / RG-003)`. Correct mapping per story Red Gate Test Plan:

| Test | Correct RG / AC | Behavior |
|------|----------------|----------|
| T-001 | RG-001 / AC-003 | PC2b: stray `.factory/` file → PREFLIGHT BLOCKED |
| T-002 | RG-002 / AC-004 | PC2a: empty shadow tree → teardown proceeds |
| T-003 | RG-003 / AC-005 | PC2b→PC2a retry path |

### Defect 3 — AC-002 attributed to `_shared-context.md` (Hand-Off task 1)

Hand-Off task 1 originally read: "Unblocks AC-001/AC-002." AC-002 (teardown preflight sub-step in step-g-cleanup.md) is **not** unblocked by `_shared-context.md` amendments. AC-002 is defined against `step-g-cleanup.md §G.1` (Hand-Off task 2). Corrected to "Unblocks AC-001." in the Hand-Off section above.

**Authority:** story v1.4 §Red Gate Test Plan (RG-001..RG-003); BC-6.26.001 v1.4 AC-002 definition. Fix committed D-895 burst, state-manager.

### RG-Reconciliation Note (appended D-902)

The D-895 erratum documented that "RG-004 and RG-005 do not exist in the story SoT." That claim was correct AT ITS TIME — story v1.4 (the SoT when the erratum was written) defined only RG-001..RG-003. Both RG-004 and RG-005 were subsequently allocated as the story grew during the adversarial cascade:

- **RG-004**: allocated in story v1.5 (commit 6149e893; story-writer F-S2104-P2-007 five-table propagation fix; PC2c fail-closed HALT test). A legitimate addition; the erratum's claim was accurate against v1.4 but does not govern v1.5+.
- **RG-005**: allocated in story v1.8 (commit 04aa9ff3; story-writer F-S2104-P5-008/F-S2104-P5-011 executor obligation + symlink vector; AC-007/AC-008 gates). A legitimate addition.

The erratum is historically correct. The fabricated IDs documented there were fabricated by the original red-gate-log author at v1.0; the later legitimate allocations of RG-004 and RG-005 in the story are independent events. No correction to the erratum body is needed — this reconciliation note provides the temporal clarification.

---

## T-004 / RG-004 Attestation Addendum (F-S2104-P2-013)

**Appended:** 2026-07-25 (D-896 S-21.04 pass-2 closure; state-manager)

The original red-gate-log covered T-001/T-002/T-003 only. T-004 (PC2c fail-closed HALT) was added by the test-writer as a pass-1 fix leg (F-S2104-P1-003) at commit `7d38b9e6`. The original attestation record did not include T-004's Red Gate status. This addendum corrects that omission.

### T-004 — PC2c Fail-Closed HALT (AC-006 / RG-004)

**Test:** T-004 asserts that when `find <worktree>/.factory -type f` exits non-zero for a reason other than path-absence (PC2c condition), teardown MUST HALT per verbatim §G.1 PC2c (step-g-cleanup.md): "If `find` exits non-zero for a non-path-absent reason (e.g., permission denial, traversal error), teardown MUST HALT. Surface the exact find exit code and stderr to the operator. `git worktree remove` is NOT executed — find errors must not silently authorize removal of unverified worktree content (BC-6.26.001 PC2c)." The PC2b `PREFLIGHT BLOCKED` message does NOT apply to PC2c.

**Red Gate state at test-writer commit `7d38b9e6`:**

Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..4`; all 4 `not ok`. T-004 fails at `_assert_doc_marker` (DOC-PARITY): "PC2c fail-closed HALT branch must be documented in §G.1" — the PC2c HALT clause was not yet present in `step-g-cleanup.md §G.1`. Pre-implementation, the awk extraction gate confirms absence; T-004 cannot pass until the PC2c branch is explicitly documented.

**Implementation commit that turned T-004 green:** `19271a65` — added PC2c HALT clause to `step-g-cleanup.md §G.1`. Verbatim from the actual §G.1 text (read from commit `19271a65` worktree): "teardown MUST HALT. Surface the exact find exit code and stderr to the operator. `git worktree remove` is NOT executed — find errors must not silently authorize removal of unverified worktree content (BC-6.26.001 PC2c)." Post-implementation suite: `1..4`; all 4 `ok`.

**RG-004 source of truth:** story v1.5 §Red Gate Test Plan (6149e893). Story v1.4 covered RG-001..RG-003 only; RG-004 (PC2c) was added by story-writer at commit 6149e893 as part of F-S2104-P2-007 five-table propagation fix.

**BC trace:** T-004 (AC-006 / RG-004) → BC-6.26.001 v1.11 PC2c: non-path-absent find error → fail-closed HALT; `git worktree remove` NOT called.

**Summary row for completeness:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-004 | AC-006 / RG-004 | BC-6.26.001 v1.11 PC2c | DOC-PARITY: PC2c fail-closed HALT clause absent from §G.1 at `7d38b9e6` | `19271a65` |

---

## T-005 Attestation Addendum (F-S2104-P6-004)

**Appended:** 2026-07-25 (D-902 S-21.04 pass-6 closure; state-manager)

T-005 (AC-002 regular-file-at-path / RG-006 non-directory-inode) was added by the test-writer as part of the pass-4 fix burst. The original red-gate-log did not include T-005's Red Gate attestation.

### T-005 — Regular File at .factory Path (AC-002 EC-007 / RG-006)

**Test:** T-005 creates a REGULAR FILE at <worktree>/.factory (fixture: touch $MOCK_WORKTREE/.factory) and asserts the preflight routes to PC2b BLOCKED with the path reported, find NOT invoked, non-zero exit, and git worktree remove NOT called. Under the pre-fix §G.1 (predicate [ ! -d ], no non-directory branch) a regular file at the path would satisfy 'not a directory' and authorize teardown — destroying the file. Observed RED at worktree commit 60f0d2d6 (DOC-PARITY: '[ ! -e ] predicate and non-directory→PC2b clause absent from step-g-cleanup.md §G.1'); GREEN at 73c2bade. Registered as RG-006 in story §Red Gate Test Plan (v1.11).

**Red Gate state — worktree commit `60f0d2d6` (pass-4 baseline):**

Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..8`; 5 of 8 `not ok` (orchestrator independently ran the suite). T-005 failed at `_assert_doc_marker` (DOC-PARITY): "step-g-cleanup.md missing `[ ! -e ]` predicate and non-directory→PC2b clause" — the non-directory→PC2b branch was not yet present in §G.1. Pre-implementation, the awk extraction gate confirms absence of the non-directory clause.

**Implementation commit that turned T-005 green:** `73c2bade` — added `[ ! -e ]` existence pre-test and non-directory→PC2b branch to §G.1.

**BC trace:** T-005 (AC-002 / RG-006) → BC-6.26.001 v1.11 PC2b (non-directory inode): non-directory inode at .factory path → PREFLIGHT BLOCKED; `git worktree remove` NOT called.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-005 | AC-002 / RG-006 | BC-6.26.001 v1.11 PC2b (non-directory inode) | DOC-PARITY: non-directory→PC2b clause absent from §G.1 at `60f0d2d6` | `73c2bade` |

---

## T-006 / RG-005 Attestation Addendum (F-S2104-P6-004)

**Appended:** 2026-07-25 (D-902 S-21.04 pass-6 closure; state-manager)

T-006 (AC-002 EC-008 / RG-005 symlink-at-path) was added by the test-writer as part of the pass-5 fix burst (F-S2104-P5-011 symlink-to-DIRECTORY data-loss vector; test-writer commit `93ec340a`). The original red-gate-log did not include T-006's Red Gate attestation.

### T-006 — Symlink at .factory Path (AC-002 EC-008 / RG-005)

**Test:** T-006 asserts that when the worktree's `.factory/` path exists as a symlink (symlink-to-directory), the preflight protocol detects the symlink via `[ -L ]` (step 2 of the 4-step chain) BEFORE invoking `find`, and classifies the symlink-at-path case as PC2b PREFLIGHT BLOCKED. The escape mechanism: POSIX test -d follows symlinks (a symlink-to-directory satisfies [ -d ]), while POSIX find WITHOUT -H/-L does not descend a symlink argument and returns empty output — a false PC2a. The [ -L ] guard (before any [ -d ] test) routes any symlink to PC2b without invoking find; the mandated trailing-slash find form is defense-in-depth that forces traversal entry if a symlink were ever to reach the find branch.

**Red Gate state — worktree commit `93ec340a` (pass-5 baseline, test-writer's own commit adding T-006):**

At commit `93ec340a` (the failing-tests commit for T-006), the §G.1 `[ -L ]` symlink guard was not yet present in `step-g-cleanup.md`. Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → T-006 `not ok` (orchestrator-verified). Failure: DOC-PARITY `_assert_doc_marker` — "§G.1 symlink→PC2b clause absent."

**Implementation commit that turned T-006 green:** `4833a642` — added `[ -L ]` symlink guard as step 2 of the §G.1 4-step chain in `step-g-cleanup.md`.

**Mutant self-check (pass-6 hardening, commit `772096f4`):** Test-writer performed scratch deletion of §G.1 L31-40 (the `[ -L ]` guard block) at commit `772096f4` as a mutant self-check — T-006 turned RED on the mutated §G.1, proving the load-bearing `[ -L ]` gate is not satisfied by the PC2b header line alone. Deletion confirmed as FAIL, restoring §G.1 confirmed GREEN. This closes the pass-6 F-S2104-P6-003 gate-weakening finding.

**BC trace:** T-006 (AC-002 / RG-005) → BC-6.26.001 v1.11 PC2b symlink-at-path: `[ -L ]` step 2 of the 4-step chain; symlink-at-path → PREFLIGHT BLOCKED; `git worktree remove` NOT called.

**RG-005 source of truth:** story v1.8 §Red Gate Test Plan (commit `04aa9ff3`). Story v1.7 covered RG-001..RG-004; RG-005 (symlink-at-path/PC2b) was added by the story-writer at commit `04aa9ff3` as part of F-S2104-P5-011 five-table propagation.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-006 | AC-002 / RG-005 | BC-6.26.001 v1.11 PC2b symlink-at-path | DOC-PARITY: §G.1 `[ -L ]` symlink→PC2b clause absent at `93ec340a` | `4833a642` |

---

## T-007 — devops-engineer Executor-Side Preflight Mandate (AC-008 / RG-007)

**Appended:** 2026-07-25 (D-903 S-21.04 pass-7 closure; state-manager)

### T-007 — devops-engineer Executor-Side Preflight Mandate (AC-008 / RG-007)

T-007 is the doc-parity gate authored at pass-4 as F-S2104-P4-003 (bats gate asserting agents/devops-engineer.md §Worktree Cleanup carries the §G.1 preflight-verification mandate). Observed RED at 60f0d2d6 (pass-4 baseline: '§Worktree Cleanup section has no §G.1/step-g-cleanup/BC-6.26.001 mandate'); GREEN at 0c0922e1. Retro-registered as T-007↔AC-008 at story v1.9 and RG-007 at story v1.11; its red-gate history predates AC-008's authoring — recorded as-is. Strengthened to obligation-asserting form (verify-PASS + not-evident-run-yourself) at 052620dc. Mutant evidence (recorded): scratch reduction of devops-engineer.md §Worktree Cleanup to 'Run git worktree remove (see BC-6.26.001).' → all four obligation gates NO MATCH (RED): qualified-path, verify, PASS-result, not-evident-run-yourself; restoring the section → GREEN. Performed by test-writer at 052620dc.

**BC trace:** T-007 → AC-008 / RG-007 / BC-6.26.001 v1.11 Precondition 3: devops-engineer.md §Worktree Cleanup MUST carry the §G.1 preflight-verification mandate unconditionally; obligation-asserting gate confirms PASS result + not-evident-run-yourself attestation.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-007 | AC-008 / RG-007 | BC-6.26.001 v1.11 Precondition 3 | DOC-PARITY: §Worktree Cleanup had no §G.1/step-g-cleanup/BC-6.26.001 mandate at `60f0d2d6` | `0c0922e1` |

---

## T-008 — Six-Surface §G.1 Mandate Regression Gates (AC-007 / RG-008)

**Appended:** 2026-07-25 (D-904 S-21.04 pass-8 closure; state-manager)

### T-008 — Six-Surface §G.1 Mandate Regression Gates (AC-007 / RG-008)

T-008 is the doc-parity gate authored at pass-4 as F-S2104-P4-009 (anti-inline-find + qualified §G.1 references across worktree-manage/SKILL.md, code-delivery/SKILL.md, fix-pr-delivery/SKILL.md, code-delivery.lobster, greenfield.lobster, rules/worktree-protocol.md). Observed RED at 60f0d2d6 (pass-4 baseline: 5 of 6 surfaces carried the inline-find anti-pattern or lacked qualified refs); GREEN at a317fd77. Strengthened quote-tolerant at 052620dc — recorded mutant evidence: pasting the canonical quoted line (find "<worktree-path>/.factory/" -type f) into a scratch copy of a delegating surface → OLD regex NO MATCH (false-green confirmed), NEW regex MATCH (gate fires RED); unquoted no-slash and unquoted with-slash forms MATCH under both. Registered T-008/RG-008 at story v1.12.

**BC trace:** T-008 (AC-007 / RG-008) → BC-6.26.001 v1.11 PC2 + Invariant 2 (caller-side dispatch gate), per story AC-007.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-008 | AC-007 / RG-008 | BC-6.26.001 v1.11 PC2 + Invariant 2 (caller-side dispatch gate), per story AC-007 | DOC-PARITY: 5 of 6 surfaces carried inline-find anti-pattern or lacked qualified refs at `60f0d2d6` | `a317fd77` |

---

## T-009 — Awareness-Clause Doc-Parity (AC-009 / RG-009)

**Appended:** 2026-07-25 (D-904 S-21.04 pass-8 closure; state-manager)

### T-009 — Awareness-Clause Doc-Parity (AC-009 / RG-009)

T-009 is the doc-parity gate authored at pass-4 as F-S2104-P4-002 (agents/adversary.md + skills/adversarial-review/SKILL.md must state the corrected shadow-write model and reference the §G.1 preflight as the enforcement chain). Observed RED at 60f0d2d6 (pass-4 baseline: neither file contained §G.1/BC-6.26.001); GREEN at 4265c96c. Registered T-009/RG-009 and anchored to NEW AC-009 (BC-6.26.001 Invariant 5) at story v1.12.

**Mutant evidence (recorded):** scratch reduction of the adversary.md corrected-model clause to '…resolve the tuple (see BC-6.26.001).' → all three obligation gates NO MATCH (RED): corrected-model, report-as-defect-signal, §G.1 enforcement-chain; the RETIRED bare alternation MATCHED the same mutant (paper-gate confirmed); restore → GREEN. Performed by test-writer at 2992b53d; gates class-completed at 3326e4dd (5 additional bare survivors strengthened at 3326e4dd (named): the 6-surface _assert_g1_ref helper (fully-qualified path form); the four section-bounded primary-path gates (SKILL.md Step 8, orchestrator step (g), Story Split Recovery, winning-playbook Step 8 — co-occurrence form); the two §G.1 non-directory gates (routing co-occurrence form); the adversarial-review defect-signal gate (spec-ground-truth co-occurrence); the devops-engineer verify gate ('dispatching caller' token). 8 scratch mutants RED / 8 restores GREEN recorded by test-writer).

**BC trace:** T-009 (AC-009 / RG-009) → BC-6.26.001 v1.11 Invariant 5 (awareness-surface anchor): adversary.md + adversarial-review/SKILL.md MUST state the corrected shadow-write model and reference §G.1 preflight as the enforcement chain.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-009 | AC-009 / RG-009 | BC-6.26.001 v1.11 Invariant 5 | DOC-PARITY: neither adversary.md nor adversarial-review/SKILL.md contained §G.1/BC-6.26.001 at `60f0d2d6` | `4265c96c` |
