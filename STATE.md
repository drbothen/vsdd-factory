---
document_type: pipeline-state
level: ops
version: "3.32"
status: draft
producer: state-manager
timestamp: 2026-06-15T10:30:00Z
phase: D-582-F2-E18-PASS-19-NOT-CLEAN-FIX-BURST+CROSS-REF-SWEEP-CODIFICATION-2026-06-15
last_amended: 2026-06-15 (v3.32) — D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION: fresh-context adversary pass-19 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 load-bearing MEDIUM (F-P19-001 BC §VP-Anchors stale VP title cite; F-P19-002 ADR §Decision-9 BC↔ADR contradiction), 1 LOW (F-P19-003 VP domain_invariants '—'), 2 obs. Fixed via comprehensive tree-wide sweep: BC-4.14.001 v1.9→v1.10 (§VP Anchors VP-083 stale title + VP-081 qualifier); BC-5.41.001 v1.11→v1.12 (§VP Anchors VP-081 qualifier swept); ADR-026 v1.12→v1.13 (§Decision 9 retitled; HandoffMissing re-attributed; §BC-Traceability + §Risk F1-R3 swept); VP-081..086 domain_invariants populated. BC-INDEX v2.86→v2.87; VP-INDEX v2.20→v2.21; ARCH-INDEX v2.40→v2.41. CODIFIED L-F2-cross-reference-title-code-sweep [process-gap]. 4-index: BC-INDEX v2.87 / VP-INDEX v2.21 / STORY-INDEX v4.01 / ARCH-INDEX v2.41. 3-CLEAN streak 0/3. D-chain cite D-581 per D-419(b); parent-commit 6dc1155162b5db57482c88f378e3949ea68a760e per D-419(b). [Prior: 2026-06-15 (v3.31) — D-581 F2 E-18 ADV PASS-18 NOT-CLEAN FIX BURST + COMPACTION. See decision-log.md SoT.]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
current_step: "D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15 — Fresh-context adversary pass-19 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 load-bearing MEDIUM (both cross-reference-strand class), 1 LOW, 2 obs. Fixed via comprehensive tree-wide sweep: (F-P19-001 MED mis-anchor) BC-4.14.001 v1.9→v1.10 §VP Anchors stale VP-083 title cite ('and HANDOFF.md Absent'→'or Non-HANDOFF.md Writes') + VP-081 qualifier; BC-5.41.001 v1.11→v1.12 §VP Anchors VP-081 cite qualifier (same class, swept). (F-P19-002 MED BC↔ADR contradiction) ADR-026 v1.12→v1.13 §Decision 9 retitled 'non-HANDOFF.md writes' + HandoffMissing re-attributed to SHELL wave-gate (BC-5.41.001 PC9, disjoint from WASM HandoffIncomplete per BC-4.14.001 EC-011) + §BC-Traceability + §Risk F1-R3 swept. (F-P19-003 LOW, adjudicated YES) VP-081..086 domain_invariant '—'→populated domain_invariants arrays (matching VP-080 pattern; POLICY 2 bidirectional). Comprehensive tree-wide grep confirmed NO remaining stranded siblings. CODIFIED L-F2-cross-reference-title-code-sweep [process-gap] (3rd occurrence: D-577/F-P19-001/F-P19-002) — candidate POLICY 5 category (i) + S-18.08 gate scope; Drift Item anchored E-18 F3. 4-index: BC-INDEX v2.87 / VP-INDEX v2.21 / STORY-INDEX v4.01 / ARCH-INDEX v2.41. 3-CLEAN streak remains 0/3 (pass-19 NOT-CLEAN). Trajectory →P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low). NEXT: adversary pass-20 (fresh-context). D-chain cite D-581 per D-419(b); parent-commit 6dc1155162b5db57482c88f378e3949ea68a760e per D-419(b)"
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
  Soft target: ≤415 lines; hard cap: 500 lines (validate-state-md-size hook enforcement).
  D-446(c) dual-margin form: margin from soft-target = 500 - 415 = 85; margin from actual tracked below.
  Historical content belongs in cycle files, NOT here.

  D-430(a) compaction authorization (D-532 burst 2026-06-08):
  Pre-D-520 banner tracker entries (D-504..D-519, 16 entries) archived per D-430(a);
  Phase Progress F5 pass-9..17 adversary+fix-burst rows (20 rows) archived per D-430(a);
  Decisions Log D-499..D-509 (11 rows) archived per D-430(a).
  All content preserved via: git show 688dd1c2:.factory/STATE.md (pre-compaction state).
  Pre-D-504 tracker preserved at: git show 20cb8e1c:.factory/STATE.md.

  D-430(a) compaction (D-568 burst 2026-06-14):
  Banner tracker D-532..D-566 (35 entries) collapsed to range-reference per D-430(a).
  Decisions Log D-557..D-567 (11 rows) archived to decision-log.md SoT per D-430(a).
  §3 older carries D-549..D-560 (12 entries) retired (topics CLOSED/ARCHIVED) per D-430(a).
  §4 Tier-A Completed Log D-549..D-555 (7 entries) trimmed; reference to decision-log.md SoT.
  All content preserved via: git show ef7eafe2:.factory/STATE.md (pre-compaction D-567 state).

  D-430(a) compaction (D-581 burst 2026-06-15):
  Phase Progress D-562..D-578 (19 rows) collapsed to range-reference per D-430(a).
  Decisions Log D-569..D-577 (9 rows) archived to decision-log.md SoT per D-430(a).
  §4 Tier-A Completed Log D-562..D-572 (11 entries) archived; kept D-573..D-581.
  All content preserved via: git show f95bbd0c:.factory/STATE.md (pre-compaction D-580 state).

  Line-growth tracker (most recent; older entries archived to git show ef7eafe2:.factory/STATE.md):
  D-532..D-566 tracker entries (35 entries) archived per D-430(a) D-568 burst; preserved at: git show ef7eafe2:.factory/STATE.md lines 40-73.
  D-567-F2-E18-ADV-PASS-6-STATE-MGR-BOOKKEEPING-2026-06-14 433 lines (wc-l; +18 over soft 415; margin 500-433=67 from hard cap; D-446(c) dual-margin form).
  D-568-F2-E18-ADV-PASS-7-FIX-BURST+COMPACTION-2026-06-14 ~370 lines (wc-l; D-430(a) compaction: 35 banner entries + 11 decision rows + 12 §3 carries + 7 §4 entries archived; target; D-446(c) dual-margin form).
  D-573-F2-E18-ADV-PASS-12-CLEAN-2026-06-14 ~370 lines (wc-l; STATE.md-only burst; NO spec changes; margin 500-370=130 from hard cap; D-446(c) dual-margin form).
  D-579-F2-E18-ADV-PASS-16-NOT-CLEAN-FIX-BURST-2026-06-15 409 lines (wc-l; D-446(c) dual-margin form).
  D-580-F2-E18-ADV-PASS-17-NOT-CLEAN-FIX-BURST-2026-06-15 416 lines (wc-l; at soft target; D-446(c) dual-margin form).
  D-581-F2-E18-ADV-PASS-18-NOT-CLEAN-FIX-BURST+COMPACTION-2026-06-15 ~395 lines (wc-l; D-430(a): 19 Phase Progress rows + 9 Decisions rows + 11 §4 entries archived; target ~395; D-446(c) dual-margin: 500-395=105 from hard cap; D-446(c) dual-margin form).
  D-582-F2-E18-ADV-PASS-19-NOT-CLEAN-FIX-BURST+CROSS-REF-SWEEP-CODIFICATION-2026-06-15 ~420 lines (wc-l; +1 Phase Progress row + §1 D-582 §4 entry + 2 Drift Items rows + §12 pass-20 row; D-446(c) dual-margin: 500-420=80 from hard cap; D-446(c) dual-margin form).
-->

# Pipeline State: vsdd-factory

> **Self-referential note:** vsdd-factory IS the project being onboarded. Engine and product are the same repository.

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | vsdd-factory |
| **Repository** | /Users/jmagady/Dev/vsdd-factory |
| **Mode** | brownfield-onboarding |
| **Language** | Rust + Bash + Markdown |
| **Started** | 2026-04-25 |
| **Last Updated** | 2026-06-15 — D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION: BC-4.14.001 v1.9→v1.10 + BC-5.41.001 v1.11→v1.12 (§VP Anchors stale cites); ADR-026 v1.12→v1.13 (§Decision 9 + HandoffMissing re-attr); VP-081..086 domain_invariants populated. BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41. L-F2-cross-reference-title-code-sweep codified. 3-CLEAN streak 0/3; NEXT: adversary pass-20. |
| **Current Phase** | D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15 — Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-4.14.001 v1.10 + BC-5.41.001 v1.12 + ADR-026 v1.13 + VP-081..086 domain_invariants. BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41. 3-CLEAN streak 0/3. NEXT: adversary pass-20. |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B, Waves 1-11, S-7.03, beta.5-7, W-14, W-15 | **COMPLETE** | `cycles/v1.0-brownfield-backfill/phase-progress-archive.md` |
| Phase D-1..D-4, Waves 12-16, E-9 v1.7 sweep | **COMPLETE** | `cycles/v1.0-brownfield-backfill/` |
| Releases rc.11..rc.18, F3/F4 E-12, S-12.03..S-12.08 | **ARCHIVED 2026-06-01 per D-430(a)** | Full rows: `git show aa1f05c9:.factory/STATE.md` lines 80-93. |
| F5 passes 3-8 cycle-level adversary + fix bursts | **COMPLETE** | Trajectory 11→9→8→7→5; F5 pass-8 verdict MEDIUM; ARCH-INDEX v1.45, D-381. |
| F5 passes 9-17 adversary + fix bursts | **ARCHIVED 2026-06-08 per D-430(a)** | 20 rows archived; trajectory pass-9→17: HIGH→MEDIUM→MEDIUM→MEDIUM→HIGH→MEDIUM→MEDIUM×3; D-382..D-392 codified. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 85-106. |
| D-343..D-523 (E-10 pass-9..14, M3 cascade, S-15.03 PRIORITY-A waves, rc.19, S-15.17 cascade) | **ARCHIVED 2026-06-10 per D-430(a)** | 22 rows archived; E-10 pass-9..14 SEALED D-471; M3 BC 11-pass CONVERGED D-497; S-15.03 PRIORITY-A COMPLETE D-508; rc.19 SHIPPED d15152af; S-15.17 cascade 9-pass SEALED D-522. Full rows: `git show c62c2c03:.factory/STATE.md` lines 82-108. |
| Release v1.0.0-rc.20 | **SHIPPED 2026-06-01** at 2a191314 | PR #166 --merge e00ab1ab; tag e9e38286; marketplace PR #12 squash-merged 862e660d; plugin count 52→53. |
| POST-RC.20 MAINTENANCE SWEEP | **COMPLETE 2026-06-01** D-529 | td-74 worktree removed; Dependabot consolidated; develop b21fd358. |
| E-10 adversarial cascade | **SEALED 2026-06-01 at pass-16 (D-531)** | verdict LOW; 16-pass trend 22→…→3; asymptotic-acceptance per D-471/D-386 Option C; resumption gate = engine-surface material change. |
| D-532..D-548 (2026-06-08..2026-06-11) | **ARCHIVED 2026-06-11 per D-430(a)** | D-532..D-548 archived; S-17.01..S-17.03 MERGED; E-17 3/3 COMPLETE; ADR-025 v1.4; Full rows: decision-log.md SoT. |
| D-549..D-560 (2026-06-11..2026-06-13) | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-549 SESSION-END; D-550 ADR-025 v1.6 REDIRECT; D-551..D-555 ADR-025 v1.6 adversary corrections; D-556 S-17.04 MERGED 3b2a378c; D-557 SESSION-INTERRUPT; D-558 rc.21 RE-RELEASE; D-559 MARKETPLACE-MERGED; D-560 OPERATOR-INSTALL-VERIFIED rc.21 100% COMPLETE. Full rows: decision-log.md SoT. |
| D-561 F2 E-18 CONTEXT-DURABILITY SPEC EVOLUTION 2026-06-14 | **COMPLETE** | F1-gate APPROVED (D1–D5). ADR-026 ACCEPTED; ARCH-INDEX v2.28; VP-081..085 (VP-INDEX v2.07); 8 BCs (BC-INDEX v2.73; total_bcs 1966); CAP-032; E-18 OPEN. |
| D-562..D-578 F2 E-18 ADV PASS-1..15 FIX BURSTS 2026-06-14/15 | **ARCHIVED 2026-06-15 per D-430(a) D-581** | 19 rows archived; passes 1-15 complete: passes 1-11 consecutive NOT-CLEAN; pass-12 CLEAN (1/3 streak); pass-13 NOT-CLEAN (streak reset 0/3); pass-14 NOT-CLEAN (2med+1low); pre-pass-14 consistency sweep (VP-082-BATS-SPLIT); pass-14 re-sweep remediation; pass-15 NOT-CLEAN (1med+4low). 4-index at D-577: BC v2.84/VP v2.18/STORY v4.01/ARCH v2.38; at D-578: BC v2.84/VP v2.19/STORY v4.01/ARCH v2.39. Full rows: `git show f95bbd0c:.factory/STATE.md` lines 85-103. |
| **D-579 F2 E-18 ADV PASS-16 NOT-CLEAN FIX BURST 2026-06-15** | **COMPLETE** | Pass-16 NOT-CLEAN: 0B/0M/3med/2low. Fixed: VP-084 v1.7 (scope mis-anchor SS-07→SS-04); VP-085 v1.4 (SS-07 confirmed); VP-083 v1.5 ('and'→'or'); VP-082 v1.8 (bats-comment); invariants.md v1.19 (DI-022 conditional). VP-INDEX v2.19→v2.20; L2-INDEX v1.0.5. 4-index BC v2.84/VP v2.20/STORY v4.01/ARCH v2.39. 3-CLEAN streak 0/3. Trajectory →P14 NOT-CLEAN(2med)→P15 NOT-CLEAN(1med/4low)→P16 NOT-CLEAN(3med/2low). |
| **D-580 F2 E-18 ADV PASS-17 NOT-CLEAN FIX BURST 2026-06-15** | **COMPLETE** | Pass-17 NOT-CLEAN: 0B/0M/1med/2low. Fixed: ADR-026 v1.11→v1.12 (null rule WAVE-AGNOSTIC); BC-5.41.001 v1.9→v1.10 (PC2/PC5/EC-001/EC-011 + test vectors; EC table reordered). BC-INDEX v2.84→v2.85; ARCH-INDEX v2.39→v2.40. 3-CLEAN streak 0/3. Trajectory →P15(1med/4low)→P16(3med/2low)→P17(1med/2low). |
| **D-581 F2 E-18 ADV PASS-18 NOT-CLEAN FIX BURST + COMPACTION 2026-06-15** | **COMPLETE** | Pass-18 NOT-CLEAN: 0B/0M/1med/1low. Fixed: BC-5.41.001 v1.10→v1.11 (F-P18-001 MED: PC9+EC-014+TV handoff-missing-hard-block; HandoffMissing disjoint from HandoffIncomplete); capabilities.md v1.6→v1.7 (F-P18-O1 LOW: §CHANGELOG monotonic descending). BC-INDEX v2.85→v2.86; L2-INDEX v1.0.5→v1.0.6. STATE.md compacted per D-430(a). 4-index BC v2.86/VP v2.20/STORY v4.01/ARCH v2.40. 3-CLEAN streak 0/3. Trajectory →P16(3med/2low)→P17(1med/2low)→P18(1med/1low). |
| **D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15** | **COMPLETE** | Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-4.14.001 v1.9→v1.10 (F-P19-001 MED: §VP Anchors VP-083 stale title + VP-081 qualifier); BC-5.41.001 v1.11→v1.12 (F-P19-001-sibling: §VP Anchors VP-081 qualifier swept); ADR-026 v1.12→v1.13 (F-P19-002 MED: §Decision 9 retitled + HandoffMissing re-attr + sweep); VP-081..086 domain_invariants populated (F-P19-003 LOW). L-F2-cross-reference-title-code-sweep codified [process-gap]. BC-INDEX v2.86→v2.87; VP-INDEX v2.20→v2.21; ARCH-INDEX v2.40→v2.41. 4-index BC v2.87/VP v2.21/STORY v4.01/ARCH v2.41. 3-CLEAN streak 0/3. Trajectory →P17(1med/2low)→P18(1med/1low)→P19(2med/1low). NEXT: adversary pass-20. |

## Current Phase Steps

> **Rows before pass-57 archived to** `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` per STATE.md content-routing rules (keep last 5 only).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| F5 passes 18-60 fix bursts (archived) | state-manager | ARCHIVED | See `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`. Passes 57-59: D-437..D-439 (META-LEVEL-12/13/14); pass-60: D-440 META-LEVEL-15 CONFIRMED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,966 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 86 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 105 file-resident + 15 stub IDs (STORY-INDEX v4.01) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 18 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 26 |

## Story Status

105 file-resident + 15 unauthored stub IDs = 120 stories registered.

- **Merged (78):** Includes S-17.01 (PR #181 c64b46d2) + S-17.02 (PR #182 df4f26b8) + S-17.03 (PR #183 60fd0233) + S-17.04 (PR #184 3b2a378c). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **Draft (29 file-resident):** S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Unauthored stub IDs (15):** S-9.01..S-9.07 (W-16); S-11.01..S-11.08 (E-11 W-17 Tier 3)
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | caf06c68 | rc.21 bot binary bundle commit 2026-06-13; prior: 2a191314 (rc.20) |
| develop | 7e99f6ef | PR #186 fix a431ff47 + release.yml sync back-merge 2026-06-13; prior: 3b2a378c (D-556) |
| factory-artifacts | 152d193f | D-582 F2 pass-19 NOT-CLEAN fix burst + cross-ref-sweep codification 2026-06-15 (BC-4.14.001 v1.10 + BC-5.41.001 v1.12 + ADR-026 v1.13 + VP-081..086 domain_invariants + BC-INDEX v2.87 + VP-INDEX v2.21 + ARCH-INDEX v2.41 + lessons.md + STATE.md); prior: fa4fbb8e D-581 |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE (marketplace PR #13 MERGED); annotated tag object |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; marketplace PR #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-582 2026-06-15; F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION; develop 7e99f6ef; main caf06c68** | rc.21 100% COMPLETE D-560; D-581 PASS-18 FIX BURST COMPLETE; **D-582 ADV PASS-19 NOT-CLEAN FIX BURST** (2med+1low fixed; BC-4.14.001 v1.10 + BC-5.41.001 v1.12 §VP-Anchors sweep; ADR-026 v1.13 §Decision-9 retitle + HandoffMissing re-attr; VP-081..086 domain_invariants populated; L-F2-cross-reference-title-code-sweep codified [process-gap]; **BC-INDEX v2.87**; VP-INDEX v2.21; ARCH-INDEX v2.41; 3-CLEAN streak 0/3 unchanged); 4-index **BC v2.87**/VP v2.21/STORY v4.01/ARCH v2.41; trajectory →P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low); **Next: adversary pass-20 → (need 3 consecutive clean) → F3 story decomposition (S-18.00..S-18.07+S-18.08).** |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-556: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`
> D-557..D-577 archived to decision-log.md SoT per D-430(a) (D-568 compaction + D-581 compaction bursts).
> Latest: D-578..D-581 — in table below.
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md`
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-582 | F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15 — Fresh-context adversary pass-19 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 load-bearing MEDIUM (both cross-reference-strand class), 1 LOW, 2 obs. Fixed via comprehensive tree-wide sweep: (F-P19-001 MED mis-anchor) BC-4.14.001 v1.9→v1.10 — §VP Anchors: VP-083 cite stale title ('and HANDOFF.md Absent'→'or Non-HANDOFF.md Writes') + VP-081 cite qualifier '(wave_id > 1)' added. (F-P19-001-sibling cross-ref sweep) BC-5.41.001 v1.11→v1.12 — §VP Anchors: VP-081 cite qualifier '(wave_id > 1)' added (same class, swept same-burst per POLICY 5). (F-P19-002 MED BC↔ADR contradiction) ADR-026 v1.12→v1.13 — §Decision 9 heading retitled 'non-HANDOFF.md writes' (prior 'HANDOFF.md Absent' was residue of F-P15-004 BC-title fix that was not propagated to ADR); HandoffMissing re-attributed to SHELL wave-gate (BC-5.41.001 PC9) with disjoint-boundary paragraph (HandoffMissing=shell/file-absent vs HandoffIncomplete=WASM/fields-missing); §BC-Traceability table + §Risk F1-R3 swept. (F-P19-003 LOW, adjudicated YES) VP-081..086 domain_invariants: '—'→populated arrays per VP-080 pattern (POLICY 2 bidirectional): VP-081 [DI-020,DI-021,DI-023]; VP-082 [DI-021,DI-022,DI-025]; VP-083 [DI-020]; VP-084 [DI-020,DI-025]; VP-085 [DI-021,DI-022,DI-025]; VP-086 [DI-020]. Comprehensive tree-wide grep confirmed NO remaining stranded siblings. CODIFIED L-F2-cross-reference-title-code-sweep [process-gap] (3rd occurrence D-577/F-P19-001/F-P19-002 — candidate POLICY 5 category (i) + S-18.08 gate scope extension); Drift Item anchored E-18 F3. POLICY-14 4-index gate: all PASS (BC-INDEX v2.87/VP-INDEX v2.21/STORY-INDEX v4.01/ARCH-INDEX v2.41). 3-CLEAN streak remains 0/3 (pass-19 NOT-CLEAN). Trajectory →P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low). NEXT: adversary pass-20 (fresh-context). D-chain cite D-581 per D-419(b); parent-commit 6dc1155162b5db57482c88f378e3949ea68a760e per D-419(b). | feature-mode-f2-e18-adv-pass-19-fix + cross-ref-sweep-codification | 2026-06-15 |
| D-581 | F2 E-18 ADV PASS-18 NOT-CLEAN FIX BURST + STATE.md COMPACTION 2026-06-15 — Fresh-context adversary pass-18 returned NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM, 1 LOW, 2 informational (verified non-findings). Fixed: (F-P18-001 MED traceability) BC-5.41.001 v1.10→v1.11 — HandoffMissing error-code contract added to shell wave-gate: PC9 defines HARD BLOCK when wave-close is attempted but no current/verified HANDOFF.md is present on factory-artifacts; EC-014 added (absent HANDOFF.md at wave-close → HandoffMissing); canonical test vector handoff-missing-hard-block added; boundary with HandoffIncomplete (WASM gate BC-4.14.001, which validates payload fields, NOT file presence) made explicit in PC9 and EC-014. BC-5.41.001 is now the defining source for `Error("HandoffMissing")` asserted by VP-081 Postcondition A and integration fixture `test_wave_close_blocked_without_handoff`. VP-081 row in VP-INDEX updated with informational note (no version bump — no VP body change). (F-P18-O1 LOW) capabilities.md v1.6→v1.7 — §CHANGELOG reordered monotonic descending (append-only IDs preserved; all v1.0–v1.6 rows confirmed present). O-2/O-3 informational: VP-082/085 scope divergence intentional and documented (sibling-scope divergence per D-579 F-P16-002 MED resolution); BC-5.41.003 SS-05 orchestration anchor defensible (PC1 orchestrates WASM exemption decision). BC-INDEX v2.85→v2.86; L2-INDEX v1.0.5→v1.0.6. input-hash: BC-5.41.001 c2426d5 (MATCH); capabilities.md a6c6f62 (MATCH). STATE.md compacted per D-430(a): Phase Progress D-562..D-578 (19 rows) archived; Decisions Log D-569..D-577 (9 rows) archived to decision-log.md SoT; §4 Tier-A D-562..D-572 (11 entries) archived. POLICY-14 4-index gate: all PASS (BC-INDEX v2.86/VP-INDEX v2.20/STORY-INDEX v4.01/ARCH-INDEX v2.40). 3-CLEAN streak remains 0/3 (pass-18 NOT-CLEAN). Trajectory →P16 NOT-CLEAN(3med/2low)→P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low). NEXT: adversary pass-19 (fresh-context). D-chain cite D-580 per D-419(b); parent-commit f95bbd0cbb94cff776cd33a1b52da52c2e520857 per D-419(b). | feature-mode-f2-e18-adv-pass-18-fix + state-compaction | 2026-06-15 |
| D-580 | F2 E-18 ADV PASS-17 NOT-CLEAN FIX BURST 2026-06-15 — Pass-17 NOT-CLEAN: 0B/0M/1med/2low. Fixed: ADR-026 v1.11→v1.12 (F-P17-001 MED: null rule WAVE-AGNOSTIC); BC-5.41.001 v1.9→v1.10 (PC2/PC5/EC-001/EC-011 + test vectors; EC table reordered ascending F-P17-002 LOW). (F-P17-003 LOW) PC6b sub-numbering ACCEPTED. VP-081 DECLINED. VP-087 DEFERRED. BC-INDEX v2.84→v2.85; ARCH-INDEX v2.39→v2.40. 3-CLEAN streak 0/3. Trajectory →P15(1med/4low)→P16(3med/2low)→P17(1med/2low). D-chain cite D-579 per D-419(b); parent-commit bde33ec5869110bd per D-419(b). | feature-mode-f2-e18-adv-pass-17-fix | 2026-06-15 |
| D-579 | F2 E-18 ADV PASS-16 NOT-CLEAN FIX BURST 2026-06-15 — Pass-16 NOT-CLEAN: 0B/0M/3med/2low. Fixed: VP-084 v1.7 (scope SS-05,SS-07→SS-05,SS-04); VP-085 v1.4 (SS-07 confirmed); VP-083 v1.5 ('and'→'or'); VP-082 v1.8 (bats-comment); invariants.md v1.19 (DI-022 conditional). VP-INDEX v2.20; L2-INDEX v1.0.5. BC-INDEX v2.84/ARCH-INDEX v2.39 UNCHANGED. 3-CLEAN streak 0/3. D-chain cite D-578 per D-419(b); parent-commit a37c054813 per D-419(b). | feature-mode-f2-e18-adv-pass-16-fix | 2026-06-15 |
| D-578 | F2 E-18 ADV PASS-15 NOT-CLEAN FIX BURST 2026-06-15 — Pass-15 NOT-CLEAN: 0B/0M/1med/4low. Fixed: VP-082 v1.7 (F-P15-001 MED: Postcondition F formal Property block); ADR-026 v1.11 (F-P15-002 LOW: cite de-versioned + provenance leg); VP-081 v1.4 (F-P15-003 LOW: volatile pin removed); VP-083 v1.4 (F-P15-004 LOW: title corrected). VP-INDEX v2.19; ARCH-INDEX v2.39. BC-INDEX v2.84 UNCHANGED. 3-CLEAN streak 0/3. D-chain cite D-577 per D-419(b); parent-commit 5a9bb36e3baf46cf8bac899b510ab23c03aceac1 per D-419(b). | feature-mode-f2-e18-adv-pass-15-fix | 2026-06-15 |
| D-569..D-577 archived | **ARCHIVED 2026-06-15 per D-430(a) D-581** | D-569 F2 adv-pass-8 fix; D-570 F2 adv-pass-9 fix; D-571 F2 adv-pass-10 fix; D-572 F2 adv-pass-11 fix; D-573 F2 adv-pass-12 CLEAN; D-574 F2 adv-pass-13 NOT-CLEAN fix; D-575 pre-pass-14 consistency sweep; D-576 F2 adv-pass-14 NOT-CLEAN fix; D-577 pass-14 consistency-re-sweep remediation. Full rows: decision-log.md SoT. |
| D-562..D-568 archived | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-562 F2 adv-pass-1 fix; D-563 F2 adv-pass-2 fix; D-564 F2 adv-pass-3 complete-sweep fix; D-565 F2 adv-pass-4 fix; D-566 F2 adv-pass-5 fix; D-567 F2 adv-pass-6 state-mgr bookkeeping; D-568 F2 adv-pass-7 fix + compaction. Full rows: decision-log.md SoT. |
| D-557..D-561 archived | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-557 SESSION-INTERRUPT; D-558 rc.21 RELEASED; D-559 marketplace MERGED; D-560 operator-install VERIFIED rc.21 100% COMPLETE; D-561 F2 E-18 spec evolution COMPLETE. Full rows: decision-log.md SoT. |
| D-549..D-556 archived | **ARCHIVED 2026-06-11..2026-06-12 per D-430(a)** | D-549..D-555 ADR-025/S-17.04 adversary passes; D-556 S-17.04 MERGED. Full rows: decision-log.md SoT. |
| D-548 | ADR-025 v1.3→v1.4 + S-17.04 AUTO-RENEW WIRING CODIFIED 2026-06-11 — ADR-025 Decision 11; S-17.04 E-17 wave 4; ARCH-INDEX v2.20→v2.21; STORY-INDEX v3.92→v3.93. | story-authoring | 2026-06-11 |
| D-527..D-538 archived | **COMPACTED 2026-06-11 per D-430(a)** | D-527 SESSION-END; D-528 rc.20 SHIPPED; D-529 MAINT SWEEP; D-530 E-10 pass-16 LOW; D-531 E-10 CASCADE SEALED; D-532..D-538 various. Full rows: decision-log.md SoT. |
| D-499..D-509 archived | **COMPACTED 2026-06-08 per D-430(a)** | 11 rows archived. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 249-259. |
| D-413..D-498 archived | **COMPACTED 2026-05-27 per D-430(a)** | 36 rows archived. Full content: decision-log.md. Pre-compaction state: `git show 20cb8e1c:.factory/STATE.md`. |
| D-510+D-522+D-525+D-526 archived | **COMPACTED 2026-06-10 per D-430(a) D-542 burst** | D-510 F5 pass-75; D-522 S-15.17 SEALED; D-525 ADR-023; D-526 S-15.17 SHIPPED. Full rows: decision-log.md SoT. |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers on active stories. -->

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67** 4 timing-flaky e2e tests | **RESOLVED 2026-05-15 PR #143 + RECURRENCE RESOLVED 2026-05-31 PR #165** | F-P3-008 pattern fully resolved. |
| **TD #68/69/70/71/72/74** | **ALL RESOLVED** 2026-05-13/14/15 | See PRs #114/#116/#117/#140/#138/#139/#141. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log `host::read_file(...65536...)` against files >64KiB → silent fail-open. Story needed to raise max_bytes to 524288. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml. Target: S-15.03 PRIORITY-A automation wave. |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 2026-05-17/18 | 6-class META-LEVEL perimeter; TDD micro-commit + registry-priority + compaction-burst-sibling-sweep + own-burst-log-structural-integrity + dim2-pc-must-read-production disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` skips production STATE.md bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` + `rows_after_heading` advisory-arm defects. Revisit on next spec-touch. |
| **test_F_P2_001 timing flake** | OPEN 2026-06-08; RECURRED 2026-06-13 | darwin-x64 at 3761ms / 3179ms vs 3000ms threshold; cleared by re-run; de-flake candidate story. |
| **RUSTSEC-2026-0149** | OPEN 2026-06-11 — wasmtime-wasi HIGH | wasmtime >= 44.0.2 required; awaiting upstream compatibility. Anchor: next rc cycle. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | validate-trajectory-tail-cell-completeness stale function header. Cosmetic; next spec-touch. |
| **VP-087 DEFERRED (null-SHA hard-block VP)** | DEFERRED 2026-06-15 — D-580 observation | No adversary coverage finding; BC test vectors (wave-1-null-sha-log-present-hard-block in BC-5.41.001 v1.10) cover the null-SHA hard-block path for any wave_id. If a future adversary pass flags missing VP coverage, create VP-087 at that time. Anchor: E-18 F3 story decomposition. |
| **bats-full-suite not in branch-protection required-status-checks** | OPEN 2026-06-13 — D-558 capture | New ci.yml `bats-full-suite (linux)` job runs but NOT in branch-protection required-checks. Follow-up: add to branch-protection settings. |
| **BC-INDEX count reconcile (pre-existing)** | OPEN 2026-06-14 — D-562 capture | disk truth = 1970 BC files, BC-INDEX frontmatter total_bcs=1966; orphan BC-2.02.013; stale SS header counts. Routing: state-manager + product-owner. Anchor: dedicated BC-INDEX reconcile burst. |
| **S-18.08 phantom-field-removal lint gate** | DRAFT-PENDING-AUTHORING 2026-06-14 — D-563 capture | L-F2-phantom-field-gate lesson (D-563): permanent enforcement story. Anchor: E-18 epic, F3 story decomposition. |
| **[process-gap] BC-Precondition registry-block shape validator gate** | OPEN 2026-06-15 — D-576 capture | BC-4.14.001 F-P14-002 class: bare logical name in `plugin=` (missing `name=` + canonical WASM path). Single-instance corrected in-spec; NO automated validator gate detects this class. Deferred to E-18 F3 story decomposition as a candidate validator-gate story (alongside S-18.08-class gate stories). Anchor: E-18 F3. |
| **[process-gap] Cross-reference title/code/phrase sweep gate** | CODIFIED 2026-06-15 — D-582 capture | L-F2-cross-reference-title-code-sweep codified (3rd recurrence: D-577/F-P19-001/F-P19-002). When a VP H1, BC error-code, or ADR §Decision heading is changed, fix-burst MUST grep `.factory/specs/` for OLD value and sweep ALL cross-citing sections (BC §VP Anchors, ADR §Decision, §BC-Traceability, §Risk) same-burst. Candidate POLICY 5 category (i) extension + S-18.08 gate scope extension. Anchor: E-18 F3 (S-18.08-class gate story). |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-15 — D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION; BC-4.14.001 v1.10; BC-5.41.001 v1.12; ADR-026 v1.13; VP-081..086 domain_invariants; BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41; 3-CLEAN streak 0/3; next: adversary pass-20 → 3-CLEAN → F3)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION OR NEW MACHINE**
> Read this section alone to resume. Assumes ZERO prior context. All decisions, directives, and anchors stated explicitly.

### §1. Where We Are

**E-18 CAP-032 context-durability (GitHub issue #173) — Feature Mode, Phase F2 (spec evolution) adversarial 3-CLEAN convergence cascade (BC-5.39.001). D-582 2026-06-15.**

F1-gate APPROVED. F2 spec evolution COMPLETE (D-561). F2 adversarial cascade passes 1-19 COMPLETE (D-562..D-582). Consistency-validator sweep D-575 COMPLETE (CLEAN). Pass-12 CLEAN (1/3 streak); pass-13 NOT-CLEAN (streak RESET 0/3); passes 14-19 all NOT-CLEAN (streak remains 0/3). STATE.md compacted at D-581 per D-430(a).

**D-582 pass-19 fix-burst summary:**
- Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Two cross-reference-strand class findings (same recurrence class as D-577/O-P6-001).
- Fixed: (F-P19-001 MED mis-anchor) BC-4.14.001 v1.9→v1.10 — §VP Anchors: VP-083 cite title corrected ('and HANDOFF.md Absent' → 'or Non-HANDOFF.md Writes' per current VP-083 H1 F-P15-004); VP-081 cite qualifier '(wave_id > 1)' added matching current VP-081 H1.
- Fixed: (F-P19-001-sibling cross-ref sweep) BC-5.41.001 v1.11→v1.12 — §VP Anchors: VP-081 cite qualifier '(wave_id > 1)' added (same strand; swept same-burst per POLICY 5).
- Fixed: (F-P19-002 MED BC↔ADR contradiction) ADR-026 v1.12→v1.13 — §Decision 9 heading retitled 'non-HANDOFF.md writes'; HandoffMissing re-attributed to SHELL wave-gate (BC-5.41.001 PC9) with disjoint-boundary paragraph; §BC-Traceability + §Risk F1-R3 swept.
- Fixed: (F-P19-003 LOW, adjudicated YES) VP-081..086 domain_invariants populated per VP-080 pattern (VP-INDEX v2.20→v2.21).
- BC-INDEX v2.86→v2.87; VP-INDEX v2.20→v2.21; ARCH-INDEX v2.40→v2.41. CODIFIED L-F2-cross-reference-title-code-sweep [process-gap] (3rd occurrence: D-577/F-P19-001/F-P19-002).

**3-CLEAN streak: 0/3** (pass-19 NOT-CLEAN; unchanged). Need 3 NEW consecutive clean passes for F2 convergence.

**Convergence trajectory (last 4 passes per D-433(e)+D-439(c)):** →P16 NOT-CLEAN(3med/2low)→P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low).

**4-index at D-582:** BC-INDEX v2.87, VP-INDEX v2.21, STORY-INDEX v4.01, ARCH-INDEX v2.41.

**NEXT ACTION (explicit, in order):**
1. **START HERE: adversary pass-20** (fresh-context; reads package at D-582 versions: ADR-026 v1.13 + BC-4.14.001 v1.10 + BC-5.41.001 v1.12 + VP-081..086 with populated domain_invariants + all other E-18 artifacts). Need 3 CONSECUTIVE CLEAN passes for F2 convergence (BC-5.39.001). 3-CLEAN streak 0/3.
2. On F2 convergence → human gate → Feature Mode F3 story decomposition (story-writer authors S-18.00..S-18.07 + S-18.08 the codified O-P8-002 gate-story).

**RECURRING LESSON (sibling-sweep):** Every semantic fix that changes an artifact TITLE, ERROR-CODE, or canonical PHRASE MUST grep tree-wide (.factory/specs/) for the old value and sweep ALL cross-referencing sections same-burst. Track via lessons.md L-F2-cross-reference-title-code-sweep (new, D-582; 3rd occurrence of cross-reference-strand class). Also L-F2-payload-only-discriminator-recurrence-gate for layer-propagation discipline.

**Artifact versions at D-582:**
- ADR-026 v1.13; BC-1.15.001 v1.3; BC-4.14.001 v1.10; BC-7.07.001 v1.7; BC-5.41.001 v1.12; BC-5.41.002 v1.6; BC-5.41.003 v1.6; BC-6.24.001 v1.6; BC-7.07.002 v1.6; VP-081 v1.5; VP-082 v1.9; VP-083 v1.6; VP-084 v1.8; VP-085 v1.5; VP-086 v1.2; invariants.md v1.19 (DI-022 conditional); capabilities.md v1.7 (CAP-032; §CHANGELOG monotonic descending); BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41; STORY-INDEX v4.01; L2-INDEX v1.0.6.

**Key design facts (so restart agent has them without re-deriving):**
- wave-1 no-op = `payload.wave_id == 1` (pure-parse WASM; wave_id absent → fail-closed HandoffIncomplete); EPIC-COMPLETE = `payload.next_wave_stories: []`; WASM gate pure-parse (shell wave-handoff derives wave_id from substrate).
- flush = commit(LOCAL, capture SHA_B) → append-to-log → push(REMOTE); each step failure → exit 2; append-fail → reset --soft SHA_B^ if HEAD==SHA_B else exit 2 (human intervention).
- push-fail (network/remote after successful local commit + log append) → exit 2; local commit + log entry retained; retry is push-only (no re-commit, no re-append) per ADR-026 §F-P10-002 + BC-7.07.001 PC6b.
- precompact-flush-log 4 fields: `<ISO-timestamp> <SHA> <cycle>/<step> commit` (field-2=SHA, field-4=`commit` token); WASM reads fields, no git-exec.
- exemption logic: prefix + field-2 + field-4 all valid → NOT-EXEMPT (anti-forgery); field-4 ≠ `commit` (corruption) → EXEMPT via prefix-alone; field-4 absent/empty = corruption = EXEMPT.
- HANDOFF: 9 base fields + epic_status conditional 10th; terminal = {merged, withdrawn, cancelled}; factory_lock opt-in default null; PostCompact best-effort outside CAP-032; harness >= v2.1.105; POLICY 19 = stable §Decision N anchor (no version pins in BC cites).
- O-P8-002 gate MANDATORY (3rd recurrence D-571): for any BC with Invariant 1 pure-parse, consistency-validator must verify VP files with matching `source_bc`/`bcs[]` do NOT describe gate behavior via external-substrate reads.

rc.21 FULLY SHIPPED D-560 (2026-06-13). main caf06c68. develop 7e99f6ef. tag 03054524. Marketplace #13 MERGED.

### §2. Operating Mode

- vsdd-factory brownfield-onboarding; cycle `v1.0-brownfield-backfill`; self-referential.
- **E-10 CASCADE FULLY SEALED D-531** (2026-06-01; pass-16 asymptotic-acceptance). **Do NOT resume E-10 without engine-surface material change.**
- **F5 PAUSED D-386 Option C** (trajectory →9→9→9→11). **Do NOT resume without explicit human direction.**
- **S-15.03 PRIORITY-A COMPLETE D-508** (2026-05-27; 11 stories; 40pts). **RC.21 FULLY SHIPPED D-560** (2026-06-13; marketplace #13 MERGED).

### §3. User Directives (Carry Across CLEAR)

ALL ACTIVE AND MANDATORY on every dispatch:
- **TD-VSDD-097-EXT:** current_step: MUST satisfy ALL 5 BC-5.39.006 v1.7 PCs simultaneously.
- **TD-VSDD-099:** Every burst-log entry MUST include all 4 Dim blocks (Dim-2+Dim-5+Dim-6+Dim-7); Dim-6 MUST contain literal-shell count with captured stdout.
- **TD-VSDD-100:** Dim-2 PC attestations MUST read production artifact (`grep ^current_step: .factory/STATE.md`); synthetic echo/printf FORBIDDEN.
- **POLICY 14 5-leg quintuple parity MANDATORY** on all BC/VP/story/epic version bumps: (1) version: frontmatter, (2) body Changelog row, (3) modified[] array, (4) last_amended: text-prefix, (5) upstream-index body-table cells.
- **Verification_step 7** literal-shell 4-index gate MANDATORY (D-494).
- **INV-019 cure (a)/(b)/(c) MANDATORY** in ALL BC changelog rows AND persisted adversary reports.
- **INV-020 / POLICY 14:** Cross-BC parity sweep whenever ANY BC in a group is modified.
- **Adversary MUST grep `origin/develop` or `factory-artifacts`** for literal-shell evidence (NOT stale local main; per L-EDP1-067-CANDIDATE).
- **Cure-extension parsimony (D-497):** EXTEND existing cure for same-class META-LEVEL recurrence; no new INV-N abstraction.
- **POLICY 8 v1.3 EC-mirror routing-rule (D-517); bidirectional AC↔PC parity (D-515+D-516); POLICY 5 v1.3.1/v1.3.3/v1.3.4/v1.3.5/v1.3.6 SDK-grounding + sibling-sweep mandates.**
- **D-537 [process-gap] spec-drift routing:** When TDD fix changes ADR-specified behavior, route architect ADR amendment in SAME burst.
- **D-539 multi-family adversary obligation:** prompt-contract + shell-logic issues require cross-family AND same-family Claude adversary passes before convergence.
- **D-582 carry:** F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15. 2med+1low fixed (BC-4.14.001 v1.10 §VP Anchors VP-083 stale title + VP-081 qualifier; BC-5.41.001 v1.12 §VP Anchors VP-081 qualifier swept; ADR-026 v1.13 §Decision 9 retitled + HandoffMissing re-attr; VP-081..086 domain_invariants populated). BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41. L-F2-cross-reference-title-code-sweep codified [process-gap] (3rd occurrence: D-577/F-P19-001/F-P19-002). 3-CLEAN 0/3 unchanged. 4-index BC v2.87/VP v2.21/STORY v4.01/ARCH v2.41. Trajectory →P17 NOT-CLEAN(1med/2low)→P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low). NEXT: adversary pass-20. D-chain cite D-581. parent-commit 6dc1155162b5db57482c88f378e3949ea68a760e.
- **D-560 carry:** OPERATOR-INSTALL-VERIFIED 2026-06-13. rc.21 100% COMPLETE. NO remaining release action.
- **D-556 carry:** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-541 carry (partial):** VP IDs TBD per TD-VSDD-063. BC-6.23.001 ACTIVE per POL-14.

### §4. Tier-A Completed Log (most recent first)

- **D-582 (2026-06-15):** F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION. Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-4.14.001 v1.9→v1.10 (F-P19-001 MED: §VP Anchors VP-083 stale title 'and HANDOFF.md Absent'→'or Non-HANDOFF.md Writes' + VP-081 qualifier '(wave_id > 1)'); BC-5.41.001 v1.11→v1.12 (F-P19-001-sibling: §VP Anchors VP-081 qualifier '(wave_id > 1)' swept); ADR-026 v1.12→v1.13 (F-P19-002 MED: §Decision 9 retitled 'non-HANDOFF.md writes' + HandoffMissing re-attr SHELL wave-gate PC9 + §BC-Traceability + §Risk F1-R3 swept); VP-081..086 domain_invariants populated (F-P19-003 LOW, adjudicated YES). L-F2-cross-reference-title-code-sweep codified [process-gap] (3rd occurrence: D-577/F-P19-001/F-P19-002). BC-INDEX v2.86→v2.87; VP-INDEX v2.20→v2.21; ARCH-INDEX v2.40→v2.41. 3-CLEAN streak 0/3. 4-index: BC v2.87/VP v2.21/STORY v4.01/ARCH v2.41. NEXT: adversary pass-20.
- **D-581 (2026-06-15):** F2 E-18 ADV PASS-18 NOT-CLEAN FIX BURST + COMPACTION. Pass-18 NOT-CLEAN: 0B/0M/1med/1low. Fixed: BC-5.41.001 v1.10→v1.11 (F-P18-001 MED: PC9+EC-014+TV handoff-missing-hard-block; HandoffMissing shell-wave-gate error code defined; disjoint from HandoffIncomplete/BC-4.14.001 EC-011); capabilities.md v1.6→v1.7 (F-P18-O1 LOW: §CHANGELOG reordered monotonic descending). O-2/O-3 verified non-findings (VP-082/085 scope intentional; BC-5.41.003 SS-05 defensible). VP-INDEX v2.20 row updated (note only). BC-INDEX v2.85→v2.86; L2-INDEX v1.0.5→v1.0.6. STATE.md compacted per D-430(a). 3-CLEAN streak 0/3. 4-index: BC v2.86/VP v2.20/STORY v4.01/ARCH v2.40. NEXT: adversary pass-19.
- **D-580 (2026-06-15):** F2 E-18 ADV PASS-17 NOT-CLEAN FIX BURST. Pass-17 NOT-CLEAN: 0B/0M/1med/2low. Fixed: ADR-026 v1.11→v1.12 (F-P17-001 MED: §Decision 2 precompact_flush_sha null rule WAVE-AGNOSTIC); BC-5.41.001 v1.9→v1.10 (F-P17-002 LOW: EC table reordered ascending). BC-INDEX v2.84→v2.85; ARCH-INDEX v2.39→v2.40. 3-CLEAN streak 0/3. 4-index: BC v2.85/VP v2.20/STORY v4.01/ARCH v2.40.
- **D-579 (2026-06-15):** F2 E-18 ADV PASS-16 NOT-CLEAN FIX BURST. Pass-16 NOT-CLEAN: 0B/0M/3med/2low. Fixed: VP-084 v1.7 (scope mis-anchor); VP-085 v1.4 (SS-07); VP-083 v1.5 ('and'→'or'); VP-082 v1.8 (bats-comment); invariants.md v1.19 (DI-022 conditional). VP-INDEX v2.20; L2-INDEX v1.0.5. 3-CLEAN streak 0/3. 4-index: BC v2.84/VP v2.20/STORY v4.01/ARCH v2.39.
- **D-578 (2026-06-15):** F2 E-18 ADV PASS-15 NOT-CLEAN FIX BURST. Pass-15 NOT-CLEAN: 0B/0M/1med/4low. Fixed: VP-082 v1.7; ADR-026 v1.11; VP-081 v1.4; VP-083 v1.4. VP-INDEX v2.19; ARCH-INDEX v2.39. 3-CLEAN streak 0/3. 4-index: BC v2.84/VP v2.19/STORY v4.01/ARCH v2.39.
- **D-577 (2026-06-15):** F2 E-18 PASS-14 CONSISTENCY-RE-SWEEP REMEDIATION. 1 MAJOR + 1 MINOR fixed (ADR-026 v1.10; BC-5.41.001 v1.9). BC-INDEX v2.84; ARCH-INDEX v2.38. 3-CLEAN streak 0/3.
- **D-576 (2026-06-15):** F2 E-18 ADV PASS-14 NOT-CLEAN FIX BURST. Pass-14 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-5.41.001 v1.8; BC-4.14.001 v1.9. BC-INDEX v2.83. 3-CLEAN streak 0/3.
- **D-575 (2026-06-15):** F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION. VP-082-BATS-SPLIT closed (VP-082 v1.6; VP-INDEX v2.18). Sweep CLEAN.
- **D-574 (2026-06-14):** F2 E-18 ADV PASS-13 NOT-CLEAN FIX BURST. 4 findings fixed. 3-CLEAN streak 1/3→0/3 RESET. 4-index: BC v2.82/VP v2.17/STORY v4.01/ARCH v2.37.
- **D-573 (2026-06-14):** F2 E-18 ADV PASS-12 CLEAN. 0B/0M/0 load-bearing. 3-CLEAN streak 0/3→1/3.
- **D-562..D-572 archived** to decision-log.md SoT per D-430(a) D-581 compaction. D-562 pass-1; D-563 pass-2; D-564 pass-3; D-565 pass-4; D-566 pass-5; D-567 pass-6; D-568 pass-7+compaction; D-569 pass-8; D-570 pass-9; D-571 pass-10; D-572 pass-11. Full entries: `git show f95bbd0c:.factory/STATE.md` §4.
- **D-561 (2026-06-14):** F2 E-18 CONTEXT-DURABILITY SPEC EVOLUTION. F2 COMPLETE: ADR-026; VP-081..085; 8 BCs; CAP-032; ARCH-INDEX v2.28; BC-INDEX v2.73.
- **D-560 (2026-06-13):** rc.21 OPERATOR-INSTALL-VERIFIED. Step 9 PASSED. 100% COMPLETE end-to-end.
- **D-559+D-558 (2026-06-13):** rc.21 marketplace MERGED + RELEASED via re-release (PR #186+#188; release.yml all-PASS). main caf06c68; develop 7e99f6ef; tag 03054524.
- **D-556 (2026-06-12):** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-549..D-555 archived** to decision-log.md SoT per D-430(a) D-568. D-549 SESSION-END; D-550 REDIRECT; D-551..D-555 ADR-025 v1.6 adversary corrections.
- **D-547 (2026-06-11):** S-17.03 MERGED PR #183. E-17 3/3 COMPLETE. issue #170 CLOSED.
- **D-531 (2026-06-01):** E-10 CASCADE SEALED. D-528 RC.20 SHIPPED. D-508 S-15.03 PRIORITY-A COMPLETE.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-582 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: **D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15 — BC-4.14.001 v1.10 + BC-5.41.001 v1.12 (§VP Anchors stale cites) + ADR-026 v1.13 (§Decision 9 retitle + HandoffMissing re-attr) + VP-081..086 domain_invariants; BC-INDEX v2.87 / VP-INDEX v2.21 / ARCH-INDEX v2.41; STORY-INDEX v4.01 UNCHANGED; L-F2-cross-reference-title-code-sweep codified [process-gap]; 3-CLEAN streak 0/3; adversary pass-20 NEXT.**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + L-issue-169-176-worktree-identity + L-F2-phantom-field-gate + L-F2-sibling-sweep-tree-wide-gate + L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class + L-F2-payload-only-discriminator-recurrence-gate + **L-F2-cross-reference-title-code-sweep** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.87 | Changed D-582 (BC-4.14.001 v1.10 + BC-5.41.001 v1.12; F-P19-001). total_bcs 1966 UNCHANGED. |
| VP-INDEX | v2.21 | Changed D-582 (VP-081..086 domain_invariants populated; F-P19-003 LOW). total_vps 86 UNCHANGED. |
| STORY-INDEX | v4.01 | UNCHANGED at D-561..D-582. E-18 stories S-18.00..S-18.08 NOT YET AUTHORED (F3 next after 3-CLEAN). |
| ARCH-INDEX | v2.41 | Changed D-582 (ADR-026 v1.13 amendment row; F-P19-002). |
| L2-INDEX | v1.0.6 | UNCHANGED at D-582 (capabilities.md v1.7 already at D-581). |

4-index at D-582 (literal-shell): `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "2.87"; `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` → "2.21"; `grep "^version:" .factory/stories/STORY-INDEX.md` → "4.01"; `grep "^version:" .factory/specs/architecture/ARCH-INDEX.md` → "2.41".

### §9. Critical Anchors

- **factory-artifacts HEAD:** `152d193f` (D-582 F2 pass-19 NOT-CLEAN fix burst + cross-ref-sweep codification 2026-06-15; prior: `fa4fbb8e` D-581; prior-prior: `f95bbd0c` D-580)
- **develop HEAD:** `7e99f6ef` (PR #186 fix + release.yml sync back-merge 2026-06-13)
- **main HEAD:** `caf06c68` (rc.21 bot bundle commit 2026-06-13)
- **v1.0.0-rc.21 tag:** `03054524` (SHIPPED; FULLY IN OPERATOR MARKETPLACE)
- **ADR-026 v1.13:** `decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md` (F-P19-002 MED: §Decision 9 retitled 'non-HANDOFF.md writes'; HandoffMissing re-attr SHELL wave-gate BC-5.41.001 PC9; §BC-Traceability + §Risk F1-R3 swept)
- **BC-4.14.001 v1.10:** `ss-04/BC-4.14.001.md` (F-P19-001 MED: §VP Anchors VP-083 stale title 'and HANDOFF.md Absent'→'or Non-HANDOFF.md Writes' + VP-081 qualifier '(wave_id > 1)' added)
- **BC-5.41.001 v1.12:** `ss-05/BC-5.41.001.md` (F-P19-001-sibling: §VP Anchors VP-081 qualifier '(wave_id > 1)' swept; HandoffMissing = shell wave-gate; disjoint from HandoffIncomplete/BC-4.14.001 EC-011)
- **VP-081..086 domain_invariants populated:** (F-P19-003 LOW, adjudicated YES) per VP-080 pattern (POLICY 2 bidirectional): VP-081 [DI-020,DI-021,DI-023]; VP-082 [DI-021,DI-022,DI-025]; VP-083 [DI-020]; VP-084 [DI-020,DI-025]; VP-085 [DI-021,DI-022,DI-025]; VP-086 [DI-020]
- **capabilities.md v1.7:** `domain-spec/capabilities.md` (F-P18-O1 LOW: §CHANGELOG reordered monotonic descending; all v1.0–v1.6 rows confirmed present)
- **invariants.md v1.19:** `domain-spec/invariants.md` (F-P16-005 LOW: DI-022 lock-renewal conditional)
- **ADR-025 v1.6 SHIPPED:** guard at `3b2a378c`; ARCH-INDEX v2.27
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.7 MERGED; E-17 W4 COMPLETE; PR #184 3b2a378c
- **Verify on resume:** `git rev-parse --short origin/develop` → expect `7e99f6ef`; `git rev-parse --short origin/main` → expect `caf06c68`

### §10. PR Status

- **0 open feature PRs. 0 open release PRs. 0 open marketplace PRs. rc.21 100% COMPLETE. E-18 F2 spec evolution staged (no PR yet — F2 adversarial passes in progress; F3 next).**
- **marketplace PR drbothen/claude-mp #13 MERGED** 2026-06-13 — rc.21 FULLY SHIPPED.
- **RELEASING.md Step 9 VERIFIED (D-560):** operator cache 1.0.0-rc.21 confirmed (plugin.json + 132 entries). rc.21 end-to-end CLOSED.

### §11. Post-CLEAR/Post-RESET Resume Checklist (zero-context; D-582 refresh)

1. **Verify worktree state:** `git rev-parse --short origin/develop` → expect `7e99f6ef`. `git rev-parse --short origin/main` → expect `caf06c68`. `git -C .factory log -1` (expect D-582 pass-19 NOT-CLEAN fix burst + cross-ref-sweep codification commit; branch factory-artifacts; clean status).
2. **Read §1-§12 this checkpoint** (all of it; D-582 self-sufficient).
3. **Verify trajectory-tail PC4:** `grep "^current_step:" .factory/STATE.md | grep -oE "trajectory-tail [→0-9/a-zA-Z(]+"` → expect trajectory containing P19 NOT-CLEAN.
4. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
5. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
6. **RC.21 100% COMPLETE D-560.** NO remaining release action. Operators: `/plugin update vsdd-factory@claude-mp`.
7. **D-582 F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION COMPLETE.** BC-4.14.001 v1.10 (§VP Anchors VP-083 stale title + VP-081 qualifier); BC-5.41.001 v1.12 (§VP Anchors VP-081 qualifier swept); ADR-026 v1.13 (§Decision 9 retitled + HandoffMissing re-attr); VP-081..086 domain_invariants populated. BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41. L-F2-cross-reference-title-code-sweep codified [process-gap]. 3-CLEAN streak **0/3 unchanged**. 4-index BC v2.87/VP v2.21/STORY v4.01/ARCH v2.41. **NEXT: adversary pass-20 → (3 consecutive clean) → F3 story decomp (S-18.00..S-18.07+S-18.08).**
8. **4-index at D-582:** BC-INDEX v2.87, VP-INDEX v2.21, STORY-INDEX v4.01, ARCH-INDEX v2.41.
9. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary + O-P8-002 MANDATORY (3rd recurrence) + L-F2-cross-reference-title-code-sweep [process-gap] (3rd recurrence: D-577/F-P19-001/F-P19-002).
10. **Latest decision D-582.** F2 pass-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION (BC-4.14.001 v1.10/BC-5.41.001 v1.12/ADR-026 v1.13/VP-081..086 domain_invariants; BC-INDEX v2.87/VP-INDEX v2.21/ARCH-INDEX v2.41; STORY-INDEX v4.01 UNCHANGED; streak 0/3). Adversary pass-20 NEXT. Then F3 S-18.00..S-18.08.

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-15 D-581)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| ~~1~~-~~prev~~ | ~~rc.21 through E-18 F2 adv passes 1-15~~ | ~~—~~ | ~~—~~ | **ALL CLOSED — D-560..D-578 2026-06-13/15.** |
| ~~1a~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-16)~~ | ~~feature~~ | ~~D-578 pass-15 fix complete~~ | **DONE D-579 2026-06-15 — NOT-CLEAN (3med+2low); fixed VP-084 v1.7 + VP-085 v1.4 + VP-083 v1.5 + VP-082 v1.8 + invariants.md v1.19; VP-INDEX v2.20; streak 0/3.** |
| ~~1b~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-17)~~ | ~~feature~~ | ~~D-579 pass-16 fix complete~~ | **DONE D-580 2026-06-15 — NOT-CLEAN (1med+2low); fixed ADR-026 v1.12 + BC-5.41.001 v1.10 (wave-agnostic null rule); BC-INDEX v2.85; ARCH-INDEX v2.40; streak 0/3.** |
| ~~1c~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-18)~~ | ~~feature~~ | ~~D-580 pass-17 fix complete~~ | **DONE D-581 2026-06-15 — NOT-CLEAN (1med+1low); fixed BC-5.41.001 v1.11 (HandoffMissing PC9+EC-014+TV) + capabilities.md v1.7 (CHANGELOG order); BC-INDEX v2.86; L2-INDEX v1.0.6; streak 0/3.** |
| ~~1d~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-19)~~ | ~~feature~~ | ~~D-581 pass-18 fix complete~~ | **DONE D-582 2026-06-15 — NOT-CLEAN (2med+1low); fixed BC-4.14.001 v1.10 + BC-5.41.001 v1.12 (§VP Anchors stale cites) + ADR-026 v1.13 (§Decision 9 retitle + HandoffMissing re-attr) + VP-081..086 domain_invariants; BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41; L-F2-cross-reference-title-code-sweep codified [process-gap]; streak 0/3.** |
| **1** | **#173/E-18 F2 adversarial re-cascade (pass-20)** | **feature** | D-582 pass-19 fix complete | Fresh-context adversary reads package at D-582 versions: BC-4.14.001 v1.10 + BC-5.41.001 v1.12 + ADR-026 v1.13 + VP-081..086 domain_invariants populated (+ all other E-18 artifacts). **3-CLEAN streak 0/3**; need 3 consecutive clean for BC-5.39.001 convergence. **START HERE.** |
| **2** | **#173/E-18 F3 story decomposition** | **feature** | F2 3-CLEAN convergence (or human waiver) | Author S-18.00..S-18.07+S-18.08 (S-18.08 ships mandatory O-P8-002 pure-parse invariant gate). STORY-INDEX v4.01→v4.02+. |
| **4** | **#173 wave-checkpoint** | **implementation** | E-18 F3 done OR human re-sequence | State-durability chain stories S-18.01..S-18.05. Blocked on F3. |
| **5** | **#171 deferred-revalidate** | **implementation** | #173 stories done | Deferred-revalidation story. |
| **6** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin. |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 cascade; rc.19+rc.20+rc.21; E-17 4 stories; S-15.17~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED/SHIPPED** |
| **7c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **8/9** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-581 F2-E18-ADV-PASS-18-NOT-CLEAN-FIX-BURST+COMPACTION-2026-06-15) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
