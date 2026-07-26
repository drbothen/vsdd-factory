---
document_type: red-gate-log
level: ops
version: "1.14"
status: verified
producer: test-writer
timestamp: 2026-07-25T03:15:00Z
phase: 3
inputs:
  - .factory/stories/S-21.04-story-worktree-write-path-discipline.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
input-hash: "1baca60"
traces_to: "BC-6.26.001 v1.11; story v1.20; story v1.21"
last_amended: "2026-07-26 D-916: F-S2104-P16-005 corrections in §Pass-15 attestation (i) closing completeness-claim corrected; (ii) M-P15-A relabeled as M-P15-A-simplified [Correction at v1.14 (D-916)]; ### Pass-16 assertion-site attestation (9ab1aa32) appended (TWO-TIER: 9 adversary-verbatim vectors TIER 1 + 11 test-writer instantiation vectors TIER 2 + gate-indexed 15-row + obligation-indexed 5-row audit tables); Summary HEAD 8b39277b→9ab1aa32; input-hash 3d12427→1baca60 (story v1.21 drift); traces_to adds story v1.21; version 1.13→1.14; prior: 2026-07-26 D-914: F-S2104-P15-001/002/004 — Summary HEAD cite 26b85d8c→8b39277b; ### Pass-15 assertion-site attestation (8b39277b) section appended (verbatim test-writer: sentence-scoped Gates 1/4/5 (joined_block+sed sentence-split) with per-gate same-AC audit table; Gate 6 two-part polarity; bare-pin sweep); input-hash c74e0f8→3d12427 (story v1.20 drift); traces_to adds story v1.20; version 1.12→1.13; prior: 2026-07-26 D-912: F-S2104-P14R-002 — Summary HEAD cite 09cfce81→26b85d8c (test-writer-executed: 9/9 + 14/14); ### Pass-14R assertion-site attestation (26b85d8c) section appended (verbatim test-writer: Gate 1 affirmative two-part + Gate 5 POLICY-13 alternation; M-P14R-A + M-P14-A + worktree-relative synonym vector all RED). F-S2104-P14R-009 — Fixture column T-001/T-002/T-003 corrected to dynamic $(mktemp -d) per bats setup(). F-S2104-P14R-010 — pass-13 mutant record exact text substituted verbatim (M-P14-A; recovered from bats 6f928350:1377-1380). input-hash 89efd7e→c74e0f8 (story v1.19 drift); traces_to adds story v1.19; version 1.11→1.12; prior: 2026-07-26 D-909: F-S2104-P13-002 — Summary HEAD cite 264f53b6→09cfce81 + attestation condensed (orchestrator-executed: 9/9 + 14/14, 2026-07-26); :288 count phrase replaced with COUNT-FREE pointer to per-pass attestation sections; ### Pass-13 assertion-site attestation (09cfce81) section appended (verbatim test-writer: polarity-aware prohibition gates, 3 mutants incl. inversion vector, gate groups count-free). F-S2104-P13-004 — §Bats Tests T-001 AC-003→AC-003; AC-001; AC-002; AC-007 (a)-(c); §Bats Tests T-002 AC-004→AC-004; AC-002; §Traces T-001 and T-002 multi-AC sync from story v1.17. input-hash 53500af→89efd7e (story v1.17 drift); version 1.10→1.11; prior: 2026-07-26 D-908: F-S2104-P12-002(a) §T-009 :287 group description corrected (four primary-path gates .md-qualified form + P11-003 attestation); F-S2104-P12-002(b) P11-003 mutant record appended to T-009 section; F-S2104-P12-002(c) Summary HEAD 2c8eff8b→264f53b6 + suite-level verification updated; F-S2104-P12-010 D-907 modified[] date 2026-07-25→2026-07-26 (monotonic correction, non-monotonic per D-906 dated 2026-07-26); input-hash d1c79e9→53500af (story v1.16 drift); version 1.9→1.10; prior: 2026-07-26 D-907 (date-corrected D-908): F-S2104-P11-005 §T-009 mutant evidence line :286 count clause replaced — 5 gate GROUPS / 9 assertion sites reconciled with 8 mutants; input-hash 455740d→d1c79e9 (story v1.15 drift); version 1.8→1.9; prior: 2026-07-26 D-906: F-S2104-P10-007 Summary line HEAD cite 9d896bf5→2c8eff8b + suite-level verification (orchestrator ran bats → 9/9 ok + 14/14 ok at 2c8eff8b, 2026-07-26); F-S2104-P10-008 §T-009 mutant evidence line :285 named list (5 named gates + 8 scratch mutants); F-S2104-P10-009 traces_to v1.10→v1.11 + BC-6.26.001 cite v1.10→v1.11 throughout + T-008/T-009 version pin parity restored; input-hash 389274b→455740d (story v1.14 + BC v1.11 drift); version 1.7→1.8; prior: 2026-07-25 D-905: F-S2104-P9-004 §T-008 BC trace corrected (PC2 + Invariant 2 caller-side dispatch gate, per story AC-007; replaces erroneous Invariant 5 caller-side propagation anchor introduced in D-904 fix wave); F-S2104-P9-007 §T-009 mutant evidence recorded (obligation-asserting gates mutant-proven at 2992b53d; class-completed at 3326e4dd; bare-alternation paper-gate confirmed); input-hash 43e6df2→389274b (story v1.13 drift); version 1.6→1.7 (F-S2104-P9-004, F-S2104-P9-007); prior: 2026-07-25 D-904: F-S2104-P8-001 §Bats Tests table T-001→RG-001†, T-002→RG-002†, T-003→RG-003† (table correction omitted at D-895, surviving 6 passes); † footnote updated to cite D-904; F-S2104-P8-007 T-007 mutant-proof recorded verbatim (qualified-path/verify/PASS-result/not-evident-run-yourself all NO MATCH on scratch reduction at 052620dc); F-S2104-P8-002 NEW T-008 addendum (AC-007/RG-008; six-surface §G.1 mandate; quote-tolerant mutant-proof 052620dc) + NEW T-009 addendum (AC-009/RG-009; awareness-clause; GREEN 4265c96c); Summary line updated (T-007/T-008/T-009 propagation gates; HEAD 9d896bf5; all T-IDs/RG rows confirmed); BC-6.26.001 cite v1.9→v1.10 (quintuple parity); input-hash 7abb656→43e6df2 (story v1.12 + BC v1.10 drift); traces_to v1.9→v1.10; version 1.5→1.6 (F-S2104-P8-001, F-S2104-P8-002, F-S2104-P8-007); prior: 2026-07-25 D-903 state-manager — F-S2104-P7-001/P7-005 attestation corrections verbatim-authored by orchestrator: §T-005 addendum heading/Test rewritten (regular-file-at-path → PC2b BLOCKED; RG-006; AC-002/EC-007); §T-006 addendum heading/trailing-slash mechanism corrected (POSIX find WITHOUT -H/-L empty-return → false PC2a; [ -L ] guard routes to PC2b; trailing-slash defense-in-depth); all 4 RG-004a occurrences replaced with RG-006; §Traces T-005/T-006 quintuple parity updated; NEW §T-007 addendum (AC-008/RG-007 devops-engineer executor-side preflight mandate; obligation-asserting at 052620dc); Summary line: 9 bats tests T-001..T-006 + 3 propagation-gate tests; BC-6.26.001 v1.8→v1.9 cites updated throughout; input-hash 4b75dba→7abb656 (story v1.11 + BC v1.9 drift); traces_to v1.8→v1.9; version 1.4→1.5 (F-S2104-P7-001, F-S2104-P7-005); prior: 2026-07-25 D-902 state-manager — T-005/T-006/RG-005 attestation addenda appended; Summary 9-test suite; Traces quintuple parity v1.5→v1.8; T-004 addendum BC cite v1.5→v1.8; RG-reconciliation note appended to D-895 erratum; input-hash 8cdfb33→a4b9ea5 (story v1.9 + BC v1.8 drift); traces_to v1.6→v1.8; version 1.3→1.4 (F-S2104-P6-004); prior: 2026-07-25 D-899 state-manager — T-004 test description corrected to verbatim §G.1 PC2c semantics (HALT + surface exit code/stderr; no PREFLIGHT BLOCKED message); input-hash 2b051ec→8cdfb33 (story v1.7 drift); traces_to v1.5→v1.6; version 1.2→1.3 (F-S2104-P4-006); prior: 2026-07-25 D-897 state-manager — fabricated PC2c implementation quote corrected to verbatim §G.1 text; Invariant TBD placeholder removed; input-hash 55904fb→2b051ec (story v1.6 drift); version 1.1→1.2 (F-S2104-P3-009, F-S2104-P3-010); prior: 2026-07-25 D-896 state-manager — T-004/RG-004 attestation addendum (F-S2104-P2-013) + quintuple parity v1.5 (F-S2104-P2-017); prior: 2026-07-25 D-895 state-manager — erratum F-S2104-P1-009 (RG-ID mapping + AC-002 attribution)"
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
  - "2026-07-26 D-907 (date-corrected D-908; prior D-906 dated 2026-07-26, D-907 originally dated 2026-07-25 was non-monotonic — F-S2104-P12-010): F-S2104-P11-005 §T-009 mutant evidence :286 count clause replaced — 5 gate GROUPS spanning 9 assertion sites reconciled with 8 mutants; input-hash 455740d→d1c79e9 (story v1.15 drift); version 1.8→1.9"
  - "2026-07-26 D-908: F-S2104-P12-002(a) §T-009 :287 primary-path gate group description corrected to .md-qualified form with P11-003 attestation; F-S2104-P12-002(b) P11-003 mutant record appended; F-S2104-P12-002(c) Summary HEAD cite 2c8eff8b→264f53b6; F-S2104-P12-010 D-907 modified[] date 2026-07-25→2026-07-26 (monotonic correction); input-hash d1c79e9→53500af (story v1.16 drift); version 1.9→1.10"
  - "2026-07-26 D-909: F-S2104-P13-002 Summary HEAD 264f53b6→09cfce81 + COUNT-FREE pointer + Pass-13 attestation appended; F-S2104-P13-004 §Bats Tests T-001/T-002 multi-AC sync + §Traces T-001/T-002 multi-AC sync from story v1.17; input-hash 53500af→89efd7e (story v1.17 drift); version 1.10→1.11"
  - "2026-07-26 D-912: F-S2104-P14R-002 Summary HEAD 09cfce81→26b85d8c + Pass-14R attestation appended; F-S2104-P14R-009 Fixture column T-001/T-002/T-003 dynamic $(mktemp -d) corrected; F-S2104-P14R-010 pass-13 mutant exact text recovered + substituted; input-hash 89efd7e→c74e0f8 (story v1.19 drift); version 1.11→1.12"
  - "2026-07-26 D-914: F-S2104-P15-001/002/004 Summary HEAD 26b85d8c→8b39277b + Pass-15 attestation appended (sentence-scoped Gates 1/4/5 + Gate 6 two-part polarity + per-gate same-AC audit + bare-pin sweep); input-hash c74e0f8→3d12427 (story v1.20 drift); version 1.12→1.13"
  - "2026-07-26 D-916: F-S2104-P16-005 corrections — §Pass-15 completeness-claim corrected per D-916; M-P15-A label relabeled as M-P15-A-simplified [Correction at v1.14 (D-916)]; ### Pass-16 assertion-site attestation (9ab1aa32) appended (TWO-TIER verbatim: 9 adversary-verbatim + 11 test-writer instantiation vectors); gate-indexed 15-row + obligation-indexed 5-row audit tables appended; Summary HEAD 8b39277b→9ab1aa32; input-hash 3d12427→1baca60 (story v1.21 drift); version 1.13→1.14"
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
| S-21.04 | 9 bats tests: T-001..T-006 (behavioral vectors) + T-007/T-008/T-009 (doc-parity propagation gates). All GREEN at worktree HEAD 9ab1aa32 (test-writer-executed: 9/9 + 14/14, 2026-07-26). All nine tests carry T-IDs, RG rows (RG-001..RG-009), and attestation sections in this log. | YES — all original 3 FAIL at Red Gate | 2265 (cargo baseline) | PASSED |

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
| T-001 | AC-003; AC-001 (Write Discipline clause-content gates); AC-002 (§G.1 doc-parity + harness, with T-002); AC-007 (a)-(c) primary-surface gates / RG-001† | BC-6.26.001 PC2b, Invariant 2 | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (stray `.factory/stories/S-021-DELIVERY.md`) | `_assert_doc_marker` gate fires — `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1; `PREFLIGHT BLOCKED` clause not present | FAIL (expected) |
| T-002 | AC-004; AC-002 (§G.1 doc-parity, with T-001) / RG-002† | BC-6.26.001 PC2a | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (empty shadow `.factory/`) | `_assert_doc_marker` gate fires — §G.1 clean-path assertion absent from step-g-cleanup.md | FAIL (expected) |
| T-003 | AC-005 / RG-003† | BC-6.26.001 PC2b→PC2a retry path | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (stray file then relocated) | `_assert_doc_marker` gate fires — §G.1 retry-path mandate absent from step-g-cleanup.md | FAIL (expected) |

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

- T-001 (AC-003; AC-001; AC-002; AC-007 (a)-(c) / RG-001†) → BC-6.26.001 v1.11 PC2b (Invariant 2): stray `.factory/` file triggers PREFLIGHT BLOCKED; `git worktree remove` NOT called
- T-002 (AC-004; AC-002 / RG-002†) → BC-6.26.001 v1.11 PC2a: empty shadow tree → teardown proceeds; `git worktree remove` IS called
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

**Mutant evidence (recorded):** scratch reduction of the adversary.md corrected-model clause to '…resolve the tuple (see BC-6.26.001).' → all three obligation gates NO MATCH (RED): corrected-model, report-as-defect-signal, §G.1 enforcement-chain; the RETIRED bare alternation MATCHED the same mutant (paper-gate confirmed); restore → GREEN. Performed by test-writer at 2992b53d; gates class-completed at 3326e4dd (gate groups and assertion sites are enumerated per-pass in the attestation sections below; running totals are not maintained as prose counts: the 6-surface _assert_g1_ref helper (fully-qualified path form); the section-bounded primary-path gates (SKILL.md Step 8, orchestrator step (g), Story Split Recovery — all strengthened to .md-qualified form at 92f986ab with 3 extensionless-degradation mutants RED/restore GREEN — and winning-playbook Step 8, .md-qualified since 2c8eff8b); the §G.1 non-directory gates (routing co-occurrence, one shared mutant); the adversarial-review defect-signal gate (spec-ground-truth co-occurrence); the devops-engineer verify gate ('dispatching caller' token). 8 scratch mutants RED / 8 restores GREEN (the two non-directory sites shared one mutant) recorded by test-writer).

**BC trace:** T-009 (AC-009 / RG-009) → BC-6.26.001 v1.11 Invariant 5 (awareness-surface anchor): adversary.md + adversarial-review/SKILL.md MUST state the corrected shadow-write model and reference §G.1 preflight as the enforcement chain.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-009 | AC-009 / RG-009 | BC-6.26.001 v1.11 Invariant 5 | DOC-PARITY: neither adversary.md nor adversarial-review/SKILL.md contained §G.1/BC-6.26.001 at `60f0d2d6` | `4265c96c` |

**P11-003 mutant evidence (recorded):** three extensionless degradations (steps/step-g-cleanup §G.1 form) applied per-gate in scratch copies → each strengthened gate exit 1 (RED); originals restored → exit 0 (GREEN). Performed by test-writer at 92f986ab. Gates: SKILL.md Step 8, orchestrator step (g), Story Split Recovery — all three strengthened from bare `step-g-cleanup` alternation to `step-g-cleanup.md`-qualified form. Winning-playbook Step 8 was already .md-qualified at 2c8eff8b and was not the subject of P11-003 strengthening.

---

### Pass-13 assertion-site attestation (09cfce81)

**Appended:** 2026-07-26 (D-909 S-21.04 pass-13 closure; state-manager)

New/changed assertion sites: (1) `_extract_write_discipline_prohibition_block()` helper — extracts the normative prohibition paragraph from `_shared-context.md` (anchored to 'All .factory/** artifact writes', terminated at first blank line). (2) T-001 prohibition gate block — the 'are FORBIDDEN' call replaced by: empty-block guard, Gate 1 (mandate-polarity line-level grep 'MUST.*absolute|absolute.*MUST'), Gate 2 (CWD-relative FORBIDDEN joined-text co-occurrence), Gate 3 (kept **Forbidden:** example marker). (3-8) comment/message anchor + SHA completions in T-002/T-005/T-009 per F-P13-006/007/009. Mutant vectors (captured): (a) prohibition block deleted → T-001 RED 'DOC-PARITY FAIL [write-discipline prohibition block absent]' exit 1; (b) POLARITY INVERSION — exact substituted text (replace `_shared-context.md:66-70` with): `All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths, not canonical absolute paths anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`) are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.` → T-001 RED 'DOC-PARITY FAIL [write-discipline prohibition block mandate-polarity]' exit 1; (c) restore → T-001 ok exit 0. Gate groups and sites (count-free): P13-001 prohibition extractor: 1 helper + 3 assertion sites in T-001; P13-006: 1 header line; P13-007: 4 sites; P13-009: 2 sites. (Note: M-P14-A text identical to (b) above — both are the CWD-relative polarity inversion; recovered verbatim from bats file at 6f928350 lines 1377-1380 per F-S2104-P14R-010 recoverability gate.)

---

### Pass-14R assertion-site attestation (26b85d8c)

**Appended:** 2026-07-26 (D-912 S-21.04 pass-14R closure; state-manager)

### F-S2104-P14R-001 — Gate 1 (affirmative mandate) and Gate 5 (POLICY-13 alternation)

**Gate 1 assertion site:** T-001 (`printf '%s\n' "$prohibition_block" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute'` + negative inversion guard)

**M-P14R-A — synonym-substituted inversion (Gate 1 negative fires)**

Exact substituted text replacing _shared-context.md prohibition block (formerly lines 66-70):
```
All `.factory/**` artifact writes performed during story delivery MUST use relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

RED stdout (bats -f "T-001"):
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 548)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block polarity-inversion]: a line matching MUST...use...canonical absolute ALSO contains 'not canonical absolute' — indicates the canonical absolute form is mentioned only as the negated alternative, not as the mandate subject; M-P14R-A ('MUST use relative paths, not canonical absolute paths') and M-P14-A ('MUST use CWD-relative paths, not canonical absolute paths') both trigger this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

GREEN stdout (restored original):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

**M-P14-A — original inversion vector re-proven (Gate 1 negative fires)**

Exact substituted text:
```
All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 548)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block polarity-inversion]: a line matching MUST...use...canonical absolute ALSO contains 'not canonical absolute' — indicates the canonical absolute form is mentioned only as the negated alternative, not as the mandate subject; M-P14R-A ('MUST use relative paths, not canonical absolute paths') and M-P14-A ('MUST use CWD-relative paths, not canonical absolute paths') both trigger this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

**Gate 5 assertion site:** T-001 (POLICY-13 alternation `CWD-relative|worktree-relative|relative[[:space:]]+path`)

**Gate 5 independent proof — worktree-relative synonym, pure form (passes Gate 1, Gate 5 fires)**

Substituted text:
```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
or alternatively MUST use worktree-relative paths when the canonical root is not available.
CWD-relative paths are FORBIDDEN.
```
(Gate 1 positive: line 1 matches MUST...canonical absolute. Gate 1 negative: does NOT fire, "not canonical absolute" absent. Gate 5: line 2 has MUST...worktree-relative → fires.)

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 582)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity]: a line in the Write Discipline prohibition paragraph contains both 'MUST' and a prohibited-subject form (CWD-relative, worktree-relative, or relative path) — in the correct text MUST mandates canonical absolute paths, not any relative form; this POLICY-13 alternation over the syntactic-form class catches M-P14-A (CWD-relative), M-P14R-A (relative path), and worktree-relative synonym variants (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

GREEN stdout (restore):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

### F-S2104-P14R-003 — Traversal-form gate (Gate 6)

**Gate 6 assertion site:** T-001 (`_assert_doc_marker '\.\./|relative[[:space:]]+traversal'`)

**Deletion mutant — exact deleted text:**
```
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — brittle and error-prone)
```
(single line deleted from _shared-context.md §Spec-Path Discipline)

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (from function `_assert_doc_marker' in file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 221,
#  in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 596)
#   `_assert_doc_marker '\.\./|relative[[:space:]]+traversal' \' failed
# DOC-PARITY FAIL [must contain: _shared-context.md §Spec-Path Discipline: relative traversal (../) Forbidden example must be present — the third Forbidden bullet (§Spec-Path Discipline **Forbidden:** example lines) documents path-traversal writes; POLICY-13 alternation covers \.\./  path form and relative-traversal label; deleting the bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003)]
```

GREEN stdout (restore):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

### F-S2104-P14R-008 — behavioral_contracts / bcs: gate

**Gate assertion site:** T-009 (F-S2104-P4-002)

**Mutant — exact substituted text in adversary.md:**
Changed `the story's \`behavioral_contracts:\` frontmatter array` → `the story's \`bcs:\` frontmatter array` (perimeter scope sentence, adversary.md Perimeter 1 scope line)

RED stdout:
```
1..1
not ok 1 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 1349)
#   `false' failed
# DOC-PARITY FAIL [adversary.md stale bcs: field present]: adversary.md must NOT reference stale bcs: frontmatter field as a standalone token — use behavioral_contracts: instead (F-S2104-P14R-008)
```

GREEN stdout (restore):
```
1..1
ok 1 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
```

Negative-gate pattern note: `(^|[^a-zA-Z0-9_])bcs:` used (not `(^|[[:space:]])bcs:`) because `bcs:` appears inside backtick code spans; the broader character-class exclusion catches the backtick-preceded form while avoiding false hits on compound identifiers.

### Suite-level verification at 26b85d8c
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

### Pass-15 assertion-site attestation (8b39277b)

### F-S2104-P15-001 — Gates 1/4/5 sentence-scoped refactor

**Change:** `story-worktree-write-path-discipline.bats` — Gates 1, 4, 5 in `_run_teardown_preflight` replaced per-physical-line predicates with sentence-scoped evaluation via `joined_block` (`tr '\n' ' '`) + `sed 's/\. /\n/g'` sentence-split.

**Gate 1(a) affirmative**: Extracts `mandate_sentence` = sentence containing 'artifact writes' from reflowed block; asserts `MUST[^.]*use[^.]*canonical[[:space:]]+absolute`.
**Gate 1(b) negative**: Same sentence must NOT match `CWD-relative|worktree-relative|relative[[:space:]]+paths?`.
**Gate 4**: No sentence in joined block may match both 'absolute' and '(FORBIDDEN|forbidden)'.
**Gate 5**: No sentence in joined block may match both 'MUST' and a prohibited-subject form.

**M-P15-A-simplified proof (RED)** [Correction at v1.14 (D-916): the vector recorded here was a simplified form used in the original pass-15 attestation; the adversary-verbatim M-P15-A appears in the Pass-16 TIER 1 section below.]:
Mutant text (mandate sentence changed to "MUST use CWD-relative paths"):
```
All `.factory/**` artifact writes performed during story delivery MUST
use CWD-relative paths anchored to the story-worktree CWD.
CANONICAL ABSOLUTE PATHS MUST use canonical absolute paths exclusively.
CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).
```
Physical-line bypass: MUST on L1, CWD-relative on L2 (different lines → OLD per-physical-line Gate 5 misses); "MUST use canonical absolute" on L3 (passes OLD Gate 1 per-line check). NEW sentence-scoped gate: mandate sentence (S1 after join+split) = "All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths anchored to the story-worktree CWD" — lacks canonical absolute, has CWD-relative.

Exit code: 1
```
1..9
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 564)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped)]: the mandate sentence (containing 'artifact writes') must contain MUST...use...canonical absolute — the mandate must be affirmative; absent or wrong mandate fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)
ok 2 … ok 9
```

**LINE-REWRAP proof (GREEN — wrap-invariant confirmed):**
Same semantic content rewrapped at different word boundaries (MUST on its own line, canonical absolute on next line, etc.) → after join+sentence-split, mandate sentence is identical → 9/9 GREEN.

**M-P14-A proof (RED at Gate 1(b)):**
Mutant: "MUST use CWD-relative paths, not canonical absolute paths" on first line.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity (mandate sentence)]: the mandate sentence contains a prohibited-subject form (CWD-relative, worktree-relative, or relative paths) — in the correct text the mandate sentence states MUST use canonical absolute paths; M-P15-A ('MUST use CWD-relative paths' in mandate sentence) triggers this gate; POLICY-13 syntactic-form class alternation (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)
```

**M-P14R-A proof (RED at Gate 1(b)):**
Mutant: "MUST use relative paths, not canonical absolute paths" → same Gate 1(b) fires. Exit code: 1, same stdout.

**Worktree-relative synonym proof (RED at Gate 1(b)):**
Mutant: "MUST use worktree-relative paths, not canonical absolute paths" → Gate 1(b) fires. Exit code: 1, same stdout.

### F-S2104-P15-002 — Gate 6 two-part polarity

**Change:** Gate 6 in `_run_teardown_preflight` replaced presence-only `_assert_doc_marker` with two-part polarity gate on `$spec_path_section`:

Gate 6(a): `grep -qE '\*\*Forbidden:\*\*.*\.\./|\.\./.*\*\*Forbidden:\*\*'` — requires **Forbidden:** AND ../ on same line.
Gate 6(b): `grep -E '\.\.\/' | grep -qE '\*\*Correct:\*\*'` fires negative if any ../ line matches **Correct:**

**Deletion mutant proof — Gate 6(a) RED:**
Removed the third `**Forbidden:**` bullet (relative traversal ../../.factory/…) entirely.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent]: a line in §Spec-Path Discipline must match **Forbidden:** AND contain ../ on the same line — the third Forbidden bullet (relative traversal ../../.factory/...) documents path-traversal writes; deleting that bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003 / F-S2104-P15-002)
```

**M-P15-B (substitute) proof — Gate 6(a) RED:**
Replaced `**Forbidden:** ...../../...` bullet with `**Correct:** ...../../...` → no **Forbidden:**+../ line remains → Gate 6(a) fires.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent]: a line in §Spec-Path Discipline must match **Forbidden:** AND contain ../ on the same line — the third Forbidden bullet (relative traversal ../../.factory/...) documents path-traversal writes; deleting that bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003 / F-S2104-P15-002)
```

**M-P15-B Gate-6(b) focus variant proof — Gate 6(b) RED:**
Kept original **Forbidden:**+../ bullet AND added a **Correct:**+../ bullet after it (Gate 6(a) passes; Gate 6(b) fires).
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Correct polarity]: a line containing ../ matches **Correct:** — the traversal form must appear only in a **Forbidden:** bullet, not a **Correct:** bullet; M-P15-B replaces the Forbidden bullet with a Correct: form, which triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-002)
```

**Unmodified proof — GREEN:**
Original _shared-context.md has `**Forbidden:** ...../../...` and no `**Correct:**+../` → both gates pass → 9/9 GREEN.

### F-S2104-P15-004 — Bare-pin elimination

**story-worktree-write-path-discipline.bats changes:**
- Extractor docblock: replaced "`~:66 of _shared-context.md`" with "`_shared-context.md §Spec-Path Discipline → §Write Discipline normative prohibition paragraph`"
- Big comment block: replaced all `~:66-70`, `line ~:66`, `line 67`, `line 68` references with stable semantic anchors (paragraph identity + sentence description)
- Gates 4+5 comments: replaced `:66-70 CWD-relative on line 67 and FORBIDDEN on line 68 (adjacent lines, not same line)` with sentence-structure anchor description

**worktree-identity-preflight.bats changes:**
- AC-005 docblock: replaced both `lines 44/59` occurrences with stable anchors: `adversary.md §Worktree-Identity Preflight opening paragraph` and `rule 6 SPEC/ADR/BC/VP bullet`
- Future tense comment rewritten to past: "Both assertions hold at HEAD — the implementer swept stale residue from the adversary.md §Worktree-Identity Preflight opening paragraph and rule 6 SPEC/ADR/BC/VP bullet."

Bare-pin verification (both files): zero matches for `~:[0-9]+|line ~[0-9]+|lines? [0-9]+(/[0-9]+)?`.

### Per-gate same-AC audit table (AC-001 / T-001 gates)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| G1(a) | Sentence-scoped: sentence containing 'artifact writes' from reflowed block | Affirmative: MUST...use...canonical absolute | M-P15-A (mandate sentence lacks canonical absolute) → RED |
| G1(b) | Same sentence | Negative: prohibited-subject absent from mandate sentence | M-P14-A (CWD-relative), M-P14R-A (relative paths), worktree-relative synonym → all RED |
| G2 | Joined block (tr '\n' ' ') | Affirmative: CWD-relative AND FORBIDDEN co-occur | Block-empty deletion → RED |
| G3 | spec_path_section (section-bounded) | Affirmative: **Forbidden:** AND 'relative path' co-occur | **Forbidden:** bullet deletion → RED |
| G4 | Sentence-scoped: sentences from joined block | Negative: no sentence has absolute+FORBIDDEN | M-P15-A variant with "Canonical absolute…FORBIDDEN" in one sentence → RED |
| G5 | Sentence-scoped: sentences from joined block | Negative: no sentence has MUST+prohibited-form | M-P14-A (CWD-relative in mandate sentence), M-P14R-A (relative path), M-P15-A → all RED |
| G6(a) | spec_path_section (per-line: admissible, bullet is single-line) | Affirmative: some line has **Forbidden:** AND ../ on same line | Deletion mutant (remove third Forbidden bullet) → RED |
| G6(b) | spec_path_section (per-line) | Negative: no line with ../ may have **Correct:** | M-P15-B keep-Forbidden-add-Correct variant → RED |

Gate inventory as of this HEAD (8b39277b); polarity coverage proven for the mutants listed per row. **Not a completeness claim: obligation-indexed coverage of AC-001(a)(i)/(ii) is asserted separately below.** [Correction at v1.14 (D-916) per F-S2104-P16-005: the prior closing line `All gates: independent, polarity-complete, zero degrees of freedom.` was falsified by four surviving vectors at this HEAD (M-P16-A, M-P16-C2, M-P16-D, M-P16-B); it has been replaced with this qualified inventory statement. The adversary-verbatim M-P15-A and the obligation-indexed coverage table appear in the Pass-16 TIER 1 section below.]

### Suite-level verification at 8b39277b
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

### Pass-16 assertion-site attestation (9ab1aa32)

test-writer 9ab1aa32 changes: Gate 1(a) negation-explicit (positive `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` + paired explicit negative `MUST[^.]*(NOT|not|never)[^.]*canonical`); Gates 4/5 sentence-complete polarity (every sentence in block checked for prohibited write targets, not only mandate sentence); abbreviation-protected sentence splitter (`cf\.\|i\.e\.\|e\.g\.\|§[0-9]+\.` protected before split); Gate 7(a) CWD-relative bullet polarity affirmative (`\*\*Forbidden:\*\*` + `file_path="\.factory/` same line in `$spec_path_section`); Gate 7(b) CWD-relative bullet polarity negative (no `\*\*Correct:\*\*` on any `file_path="\.factory/` line); Gate 3 tightened to `relative path` + `file_path="\.factory/` same line; anchor-uniqueness gate (count `All.*\.factory.*artifact writes` = 1 in `$spec_path_section`, else ambiguous-anchor error); `#### Write Discipline` child-heading bounding added. 9/9 + 14/14 green at 9ab1aa32.

---

#### TIER 1 — Adversary-verbatim vectors (re-proven RED at 9ab1aa32)

**Preamble:** TIER 1 records each adversary-assigned ID at its exact verbatim substituted text from adversary-pass-16.md (or the governing adversary pass for prior-pass vectors). All 9 vectors attested RED at 9ab1aa32.

| Vector ID | Finding | Substitution description | Gate(s) triggered | Status at 9ab1aa32 |
|-----------|---------|--------------------------|-------------------|--------------------|
| M-P16-A | F-S2104-P16-001 | §Write Discipline normative paragraph: `MUST NOT use canonical absolute paths anchored to the main-checkout root`…`CWD-relative paths were formerly FORBIDDEN…that prohibition is retired`; worked-example bullets inverted (Forbidden:canonical→Correct:CWD-relative, Correct:CWD-relative→Forbidden:canonical) | Gate 1(a) negation-explicit fires: mandate sentence matches `MUST NOT use canonical`; Gate 1(a) affirmative fires: `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` does NOT match on mandate sentence | RED |
| M-P16-C2 | F-S2104-P16-001 | §Write Discipline: `MUST use canonical absolute paths only when the target is outside the worktree, cf. CWD-relative paths for every in-worktree ledger…Duplicating a ledger onto the main checkout is FORBIDDEN` — `cf. ` is a sentence-split boundary, placing MUST+canonical in fragment 1 and CWD-relative in fragment 2 | Gate 5 sentence-complete: MUST and `CWD-relative` co-occur in one sentence of the reflowed block without a co-occurring prohibition token | RED |
| M-P16-D | F-S2104-P16-002 | §Spec-Path Discipline: `**Correct:** Write(file_path=".factory/stories/S-NNN-DELIVERY.md", …)` (was **Forbidden:**); `**Forbidden:** Write(file_path="../../.factory/…", …)` (traversal; was already Forbidden) | Gate 7(a) fires: no `\*\*Forbidden:\*\*` + `file_path="\.factory/` same line in `$spec_path_section` | RED |
| M-P16-B | F-S2104-P16-003 | Compliant 2-line decoy `All .factory/** artifact writes MUST use canonical absolute paths anchored to the main-checkout root.\nCWD-relative shadow-tree writes are FORBIDDEN.` inserted before normative paragraph in §Spec-Path Discipline; normative paragraph inverted to M-P15-A/M-P14-A shape | Anchor-uniqueness gate fires: `All.*\.factory.*artifact writes` count = 2 in `$spec_path_section`; ambiguous-anchor error | RED |
| M-P14-A | F-S2104-P14-001 | Mandate sentence changed to `MUST use CWD-relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `CWD-relative` in mandate sentence | RED |
| M-P14R-A | F-S2104-P14R-001 | Mandate sentence changed to `MUST use relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `relative[[:space:]]+paths?` in mandate sentence | RED |
| worktree-relative synonym | F-S2104-P14R-001 | Mandate sentence changed to `MUST use worktree-relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `worktree-relative` in mandate sentence | RED |
| M-P15-A | F-S2104-P15-001 | Normative paragraph (verbatim from adversary-pass-15.md Part A): `All .factory/** artifact writes performed during story delivery MUST use\nCWD-relative paths anchored to the story-worktree CWD.\nWriters MUST use canonical absolute paths only when reading spec ground-truth from the main checkout.\nCanonical absolute artifact-write paths (e.g., $CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md)\nare FORBIDDEN — relative writes land in the story worktree's shadow .factory/ subtree and are preserved at teardown.` | Gate 1(a) affirmative fires: mandate sentence after join+split is `MUST use\nCWD-relative paths anchored to the story-worktree CWD` → does not match `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| M-P15-B | F-S2104-P15-002 | §Spec-Path Discipline traversal bullet changed to `**Correct:** Write(file_path="../../.factory/…", …)` | Gate 6(a): no `\*\*Forbidden:\*\*` + `../` same line | RED |

---

#### TIER 2 — Test-writer instantiation vectors (minimal representative forms at 9ab1aa32)

**Preamble:** TIER 2 records the test-writer's working instantiation of each mutant class. Labeled with `-instantiation` suffix to distinguish from adversary-verbatim TIER 1 records. 11 vectors total (9 adversary-class instantiations + 2 new: Gate-7a-deletion and M-P16-B-in-section).

| Vector ID | Instantiates | Gate(s) triggered | Status at 9ab1aa32 |
|-----------|-------------|-------------------|--------------------|
| M-P16-A-instantiation | M-P16-A | Gate 1(a) negation-explicit (fires on `MUST NOT use canonical`); Gate 1(a) affirmative (no match on mandate sentence) | RED |
| M-P16-C2-instantiation | M-P16-C2 | Gate 5 sentence-complete: MUST + prohibited-subject co-occur in same sentence without prohibition co-token | RED |
| M-P16-D-instantiation | M-P16-D | Gate 7(a): `\*\*Forbidden:\*\*` + `file_path="\.factory/` absent; Gate 7(b): `\*\*Correct:\*\*` + `file_path="\.factory/` present | RED |
| Gate-7a-deletion | (new) deletion mutant for Gate 7(a) | Gate 7(a): no `\*\*Forbidden:\*\*` + `file_path="\.factory/` line in `$spec_path_section` | RED |
| M-P16-B-out-of-section-instantiation | M-P16-B | `#### Write Discipline` bounding: extractor does not reach decoy placed in read-discipline prose; normative paragraph inversion detected by Gate 1(a) | RED |
| M-P16-B-in-section-instantiation | M-P16-B | Anchor-uniqueness gate: decoy inside `#### Write Discipline` before normative paragraph; count = 2; ambiguous-anchor error | RED |
| M-P14-A-instantiation | M-P14-A | Gate 1(b) prohibited-subject alternation | RED |
| M-P14R-A-instantiation | M-P14R-A | Gate 1(b) prohibited-subject alternation | RED |
| worktree-relative-instantiation | worktree-relative synonym | Gate 1(b) prohibited-subject alternation | RED |
| M-P15-A-instantiation | M-P15-A | Gate 1(a) affirmative: mandate sentence `MUST use CWD-relative paths anchored to the story-worktree CWD` lacks `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| M-P15-B-instantiation | M-P15-B | Gate 6(a): `\*\*Forbidden:\*\*` + `../` absent | RED |

---

### Gate-indexed audit table (T-001 / AC-001 gates at 9ab1aa32 — 15 gates)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| Absent-block guard | `$spec_path_section` non-empty | Affirmative: prohibition block exists | §Spec-Path Discipline deletion → RED |
| G1(a) affirmative | Mandate sentence (sentence containing 'artifact writes') from reflowed block | Affirmative: `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | M-P15-A-instantiation (mandate sentence lacks canonical absolute) → RED |
| G1(a) negation-explicit | Mandate sentence | Negative: does NOT match `MUST[^.]*(NOT\|not\|never)[^.]*canonical` | M-P16-A (`MUST NOT use canonical absolute`) → RED |
| G1(b) | Mandate sentence | Negative: prohibited-subject absent (`CWD-relative\|worktree-relative\|relative[[:space:]]+paths?`) | M-P14-A / M-P14R-A / worktree-relative → all RED |
| G2 | Each sentence in reflowed block | Affirmative: at least one sentence has prohibited-subject AND FORBIDDEN co-occurring | Block-empty deletion → RED |
| G3 (tightened) | Per-line in `$spec_path_section`, lines with `file_path="\.factory/` only | Affirmative: some such line matches `\*\*Forbidden:\*\*` AND `relative path` | Deletion of Forbidden·file_path line → RED |
| G4 | Per-sentence from reflowed block matching `absolute` | Negative: no sentence matches `absolute` AND `(FORBIDDEN\|forbidden)` | Canonical-absolute-FORBIDDEN variant → RED |
| G5 sentence-complete polarity | Every sentence in reflowed block | Negative: no sentence has MUST AND `(CWD-relative\|worktree-relative\|relative[[:space:]]+path\|story-worktree[[:space:]]+CWD)` without co-occurring prohibition token | M-P16-C2-instantiation → RED |
| G6(a) | Per-line in `$spec_path_section` | Affirmative: some line matches `\*\*Forbidden:\*\*` AND `../` | Traversal bullet deletion → RED |
| G6(b) | Per-line in `$spec_path_section` | Negative: no `../` line may match `\*\*Correct:\*\*` | M-P15-B Correct swap → RED |
| G7(a) | Per-line in `$spec_path_section` with `file_path="\.factory/` | Affirmative: some such line matches `\*\*Forbidden:\*\*` | Gate-7a deletion mutant → RED |
| G7(b) | Per-line in `$spec_path_section` with `file_path="\.factory/` | Negative: no such line may match `\*\*Correct:\*\*` | M-P16-D (CWD-relative bullet relabeled Correct) → RED |
| Anchor-uniqueness | `$spec_path_section` — count of `All.*\.factory.*artifact writes` | Affirmative: count = 1; else ambiguous-anchor error | M-P16-B in-section (count = 2) → RED |
| `#### Write Discipline` bounding | Extractor restricted to `#### Write Discipline` child heading | Structural: decoy outside `#### Write Discipline` is outside extraction domain | M-P16-B out-of-section decoy → RED |
| Abbreviation-protected splitter | Sentence-split: `cf\.\|i\.e\.\|e\.g\.\|§[0-9]+\.` protected before split | Structural: abbreviation boundaries not treated as sentence boundaries | M-P16-C2 (abbreviation suppressed → sentence is one unit with MUST + CWD-relative → Gate 5 fires) → RED |

**Not a completeness claim: obligation-indexed coverage of AC-001 obligations is stated in the table below.**

---

### Obligation-indexed AC-001 coverage table

| AC-001 Clause | Obligation | Gate(s) asserting | Mutant proving |
|---------------|-----------|-------------------|----------------|
| AC-001(a)(i) normative mandate | Mandate is affirmative — canonical absolute required, CWD-relative prohibited | G1(a) affirmative + G1(a) negation-explicit + G1(b) | M-P15-A-instantiation → G1(a) affirmative RED; M-P16-A → G1(a) negation-explicit RED; M-P14-A / M-P14R-A / worktree-relative → G1(b) RED |
| AC-001(a)(i) CWD-relative bullet | CWD-relative worked-example bullet is Forbidden (not Correct) | G7(a) + G7(b) | Gate-7a deletion → G7(a) RED; M-P16-D relabeled Correct → G7(b) RED |
| AC-001(a)(ii) traversal bullet | Relative-traversal worked-example bullet is Forbidden (not Correct) | G6(a) + G6(b) | Traversal bullet deletion → G6(a) RED; M-P15-B Correct swap → G6(b) RED |
| AC-001 sentence-complete polarity | Every sentence in the block is polarity-checked; no sentence carries a prohibited write directive without a co-occurring prohibition token | G5 sentence-complete + G2 | M-P16-C2-instantiation → G5 RED; block-empty deletion → G2 RED |
| AC-001 extraction integrity | Extractor evaluates normative paragraph, not a decoy | Anchor-uniqueness + `#### Write Discipline` bounding | M-P16-B in-section → anchor-uniqueness RED; M-P16-B out-of-section → bounding RED |

### Suite-level verification at 9ab1aa32
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.
