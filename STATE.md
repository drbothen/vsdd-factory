---
document_type: pipeline-state
level: ops
version: "3.53"
status: draft
producer: state-manager
timestamp: 2026-06-15T23:59:59Z
phase: D-603-F2-E18-PASS-40-NOT-CLEAN-FIX-BURST-2026-06-15
last_amended: 2026-06-15 (v3.53) — D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM (F-P40-001), 2 LOW (O-P40-001, O-P40-002). Streak 0/3 (NOT-CLEAN; 1 MED). Fixed: (F-P40-001 MED, POLICY 2) invariants.md v1.20→v1.21 — added `Cited by: VP-NNN` back-refs to DI-020..025 (DI-019 precedent; VP→DI forward arrays existed; DI→VP reverse missing at F-P19-003); completes bidirectional DI↔VP. DI-020←VP-081/083/084/086; DI-021←VP-081/082/085; DI-022←VP-082/085; DI-023←VP-081; DI-024←VP-deferred-F3-S-18.05; DI-025←VP-082/084/085. (O-P40-001 LOW) preamble corrected CAP-032 DIs trace to Source+ADR not BR-NN. (O-P40-002 LOW) BC-7.07.001 v1.3 changelog: NO action — append-only history. L2-INDEX v1.0.10→v1.0.11. 4-index: BC-INDEX v2.99 / VP-INDEX v2.29 / STORY-INDEX v4.01 / ARCH-INDEX v2.47 (unchanged). NEXT: adversary pass-41. D-chain cite D-602 per D-419(b); parent-commit c53b42a1 per D-419(b). [Prior: 2026-06-15 (v3.52) — D-602 F2 E-18 ADV PASS-39 NOT-CLEAN FIX BURST. See decision-log.md SoT.]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
"D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15 — Fresh-context adversary pass-40 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM, 2 LOW. Streak remains 0/3 (1 MED finding; HUMAN-DIRECTED strict 3-CLEAN). Fixed: (F-P40-001 MED, POLICY 2) invariants.md v1.20→v1.21 — added `Cited by: VP-NNN` back-refs to DI-020..025 (DI-019 precedent; VP→DI forward arrays existed but DI→VP reverse wasn't propagated at F-P19-003); completes bidirectional DI↔VP traceability. Mapping verified vs VP-INDEX: DI-020←VP-081/083/084/086, DI-021←VP-081/082/085, DI-022←VP-082/085, DI-023←VP-081, DI-024←VP-deferred-F3-S-18.05, DI-025←VP-082/084/085. Orchestrator adjudication: fixed (not self-demoted to CLEAN) — DI-019 precedent makes the omission re-flaggable; cheap consistent fix removes the vector. (O-P40-001 LOW) invariants.md preamble corrected — CAP-032 DIs (DI-020..025) trace to Source capability+ADR, not BR-NN business rule. (O-P40-002 LOW) BC-7.07.001 v1.3 changelog superseded-description: NO action — append-only historical record, confirmed non-defect. Adversary confirmed ALL behavioral contracts CLEAN (16+ consecutive passes). L2-INDEX v1.0.10→v1.0.11 (Document Map sync + changelog row). 4-index: BC-INDEX v2.99 / VP-INDEX v2.29 / STORY-INDEX v4.01 / ARCH-INDEX v2.47 (UNCHANGED). 3-CLEAN streak 0/3 (NOT-CLEAN). Trajectory →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(1med POLICY-2 back-ref). NEXT: adversary pass-41 (fresh-context; DI↔VP bidirectional complete). D-chain cite D-602 per D-419(b); parent-commit c53b42a1 per D-419(b)."
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

  D-430(a) compaction history (D-532..D-595 — COLLAPSED 2026-06-15 per D-596 compaction):
  D-532 (2026-06-08): 16 banner entries + 20 Phase Progress rows + 11 Decisions rows archived. SoT: git show 688dd1c2:.factory/STATE.md.
  D-568 (2026-06-14): 35 banner entries + 11 Decisions rows + 12 §3 carries + 7 §4 entries. SoT: git show ef7eafe2:.factory/STATE.md.
  D-581 (2026-06-15): 19 Phase Progress rows + 9 Decisions rows + 11 §4 entries. SoT: git show f95bbd0c:.factory/STATE.md.
  D-586 (2026-06-15): 5 PP + 4 Dec + 9 §4 rows. SoT: git show 907ca48e:.factory/STATE.md.
  D-591 (2026-06-15): 5 PP + 7 Dec + 9 §4 rows. SoT: git show 5f55628b:.factory/STATE.md.
  D-593 (2026-06-15): 4 PP + 4 Dec + 2 §4 entries. SoT: git show 3a911cb7:.factory/STATE.md.
  D-594 (2026-06-15): 1 PP + 1 Dec + 1 §4 entry + 7 Drift rows CLEARED. SoT: git show 9361bcaf:.factory/STATE.md.
  D-595 (2026-06-15): 1 PP + 1 Dec + 1 §4 entry. SoT: git show 8ae759dd:.factory/STATE.md.

  D-430(a) compaction (D-596 burst 2026-06-15):
  Phase Progress D-595 (1 row) collapsed to range-reference per D-430(a).
  Decisions Log D-595 (1 row) archived to range-reference (decision-log.md SoT) per D-430(a).
  §4 Tier-A Completed Log D-595 (1 entry) archived; kept D-596.
  Banner history D-532..D-595 collapsed to 8-line summary (this burst) per D-430(a).
  All content preserved via: git show a46b67af:.factory/STATE.md (pre-compaction D-595 state).

  D-596-F2-E18-ADV-PASS-33-NOT-CLEAN-FIX-BURST-COMPACTION-2026-06-15 ~395 lines (wc-l; D-430(a): 1 Phase Progress row D-595 + 1 Decisions row D-595 + 1 §4 entry D-595 archived; +1 Phase Progress row D-596 + D-596 Decisions Log row + §3 D-596 carry + §4 D-596+D-595-archived entries + §1/§5/§8/§9/§10/§11/§12/checkpoint full refresh; D-446(c) dual-margin: 500-395=105 from hard cap; D-446(c) dual-margin form).

  D-598-F2-E18-ADV-PASS-35-NOT-CLEAN-FIX-BURST-2026-06-15 ~421 lines pre-compaction (wc-l; D-430(a): Phase Progress D-597 row archived (content now in range-reference pointing git show 58a1cfe0); Decisions Log D-597 row archived to decision-log.md SoT; §4 D-597 entry archived; +1 Phase Progress D-598 row + D-598 Decisions Log row + §3 D-598 carry + §4 D-598 entry + §1/§5/§6/§8/§9/§10/§11/§12/checkpoint full refresh + 4 new Drift rows; D-446(c) dual-margin: 500-421=79 from hard cap).

  D-599-F2-E18-ADV-PASS-36-NOT-CLEAN-FIX-BURST-COMPACTION-2026-06-15 ~390 lines post-compaction (wc-l; D-430(a): Phase Progress D-598 row archived (content now in range-reference); Decisions Log D-598 row archived to decision-log.md SoT; §4 D-598 entry archived; +1 Phase Progress D-599 row + D-599 Decisions Log row + §3 D-599 carry + §4 D-599 entry + §1/§5/§6/§8/§9/§10/§11/§12/checkpoint full refresh; D-446(c) dual-margin: 500-390=110 from hard cap).
  D-600-F2-E18-ADV-PASS-37-NOT-CLEAN-FIX-BURST-2026-06-15 ~430 lines post-update (wc-l; D-430(a): Phase Progress D-599 row archived (range-reference git show b1c2b7e0); Decisions Log D-599 row archived; §4 D-599 entry archived; +D-600 Phase Progress row + D-600 Decisions Log row + §3 D-600 carry + §4 D-600 entry + 3 new Drift rows (F-P37-001/002/003) + §1/§5/§6/§8/§9/§10/§11/§12/checkpoint full refresh; D-446(c) dual-margin: 500-430=70 from hard cap).

  D-601-F2-E18-ADV-PASS-38-CLEAN-STREAK-1-3-2026-06-15 ~405 lines post-compaction (wc-l; D-430(a): Phase Progress D-600 row archived (range-reference git show 2cdc70c7); Decisions Log D-600 row archived + D-598-archived/D-597-archived rows collapsed to range-reference; §4 D-600 entry archived; +D-601 Phase Progress row + D-601 Decisions Log row + §4 D-601 entry + §1/§5/§6/§8/§9/§10/§11/§12/checkpoint full refresh; L-F2-index-quad-cite-reflects-last-bump convention note appended to lessons.md; D-446(c) dual-margin: 500-405=95 from hard cap).

  D-603-F2-E18-ADV-PASS-40-NOT-CLEAN-FIX-BURST-2026-06-15 (wc-l; D-430(a): Phase Progress D-601+D-602 rows collapsed to range-reference (content at git show c53b42a1:.factory/STATE.md); Decisions Log D-600..D-602 rows collapsed to range-reference; §4 D-600..D-602 entries collapsed; +D-603 Phase Progress row + D-603 Decisions Log row + §4 D-603 entry + §1/§5/§6/§8/§9/§10/§11/§12/checkpoint full refresh; invariants.md v1.21 + L2-INDEX v1.0.11; D-446(c) dual-margin).
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
| **Last Updated** | 2026-06-15 — D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM (F-P40-001 invariants.md v1.20→v1.21 POLICY 2 DI↔VP back-refs), 2 LOW (O-P40-001 preamble; O-P40-002 no-action). Streak 0/3. L2-INDEX v1.0.10→v1.0.11. 4-index: BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47 (UNCHANGED). NEXT: adversary pass-41. |
| **Current Phase** | D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15 — STREAK 0/3. 0 BLOCKER, 0 MAJOR, 1 MED (F-P40-001 invariants.md v1.21 DI↔VP bidirectional back-refs POLICY 2), 2 LOW. Fixed. L2-INDEX v1.0.11. 4-index: BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47. NEXT: adversary pass-41 (fresh-context; DI↔VP bidirectional complete; streak 0/3). |
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
| D-579..D-583 F2 E-18 ADV PASS-16..20 FIX BURSTS 2026-06-15 | **ARCHIVED 2026-06-15 per D-430(a) D-586** | 5 rows archived; passes 16-20 all NOT-CLEAN: P16(3med/2low)→P17(1med/2low)→P18(1med/1low)→P19(2med/1low)→P20(1med/4low). 4-index at D-583: BC v2.88/VP v2.22/STORY v4.01/ARCH v2.42. Full rows: `git show 907ca48e:.factory/STATE.md` lines 97-103. |
| **D-584..D-588 F2 E-18 ADV PASS-21..25 FIX BURSTS 2026-06-15** | **ARCHIVED 2026-06-15 per D-430(a) D-591** | 5 rows archived; passes 21-25: P21 NOT-CLEAN(2med/1low); P22 CLEAN(1/3 streak); P23 CLEAN(2/3 streak); P24 NOT-CLEAN(RESET 2/3→0/3 comprehensive cleanup ALL 9 findings); P25 NOT-CLEAN(2med/2low). 4-index at D-588: BC v2.90/VP v2.25/STORY v4.01/ARCH v2.44. Full rows: `git show 5f55628b:.factory/STATE.md` lines 97-103. |
| **D-589..D-592 F2 E-18 ADV PASS-26..29 + STALE-TERM CLASS FLUSH 2026-06-15** | **ARCHIVED 2026-06-15 per D-430(a) D-593** | 4 rows archived; P26 NOT-CLEAN(1med/3obs, BC-7.07.001 v1.8 title-cite-parity class CLOSED); P27 CLEAN(1/3); P28 NOT-CLEAN(RESET, D-591 stale-term class flush BC-5.41.003 v1.7+ADR-026 v1.17); P29 CLEAN(1/3). 4-index at D-592: BC v2.92/VP v2.25/STORY v4.01/ARCH v2.45 (UNCHANGED). Full rows: `git show 3a911cb7:.factory/STATE.md` lines 107-110. |
| **D-593 F2 E-18 ADV PASS-30 CLEAN — STREAK 2/3 2026-06-15** | **ARCHIVED 2026-06-15 per D-430(a) D-594** | Pass-30 CLEAN: 0B/0M/0 load-bearing MED/0 mis-anchor. Adversary declared 'deep asymptotic convergence.' 3 LOWs deferred (F-P30-001/002/003); 8 deferred LOWs total. 3-CLEAN streak 1/3→2/3. 4-index UNCHANGED: BC v2.92/VP v2.25/STORY v4.01/ARCH v2.45. Full rows: `git show 9361bcaf:.factory/STATE.md` Decisions Log. |
| **D-594 F2 E-18 ADV PASS-31 NOT-CLEAN FULL BACKLOG CLEARANCE 2026-06-15** | **ARCHIVED 2026-06-15 per D-430(a) D-595** | Pass-31 NOT-CLEAN: F-P31-001 MED + F-P31-002 MED; streak RESET 2/3→0/3; FULL BACKLOG CLEARANCE (all 9 deferred items fixed); 4-index BC v2.93/VP v2.26. Full rows: decision-log.md SoT. |
| **D-595 F2 E-18 ADV PASS-32 NOT-CLEAN FIX BURST 2026-06-15** | **ARCHIVED 2026-06-15 per D-430(a) D-596** | Pass-32 NOT-CLEAN (2med/4low all fixed; BC-4.14.001 v1.13/VP-083 v1.9; streak 0/3; 4-index BC v2.94/VP v2.27). Full: `git show a46b67af:.factory/STATE.md` Phase Progress. |
| **D-596 F2 E-18 ADV PASS-33 NOT-CLEAN FIX BURST + COMPACTION 2026-06-15** | **ARCHIVED per D-597** | Pass-33 NOT-CLEAN: F-P33-001 MED (v1.5-skip-marker sibling gap; BC-5.41.002 v1.9/BC-6.24.001 v1.8/BC-7.07.002 v1.10; 8-BC sweep; class CLOSED). Streak 0/3; 4-index BC v2.95/VP v2.27. Full: `git show 7b546299:.factory/STATE.md` Phase Progress. |
| **D-597 F2 E-18 ADV PASS-34 CLEAN — STREAK 1/3 + 2 PROACTIVE LOW FIXES 2026-06-15** | **ARCHIVED per D-598** | Pass-34 CLEAN: 0 BLOCKER, 0 MAJOR, 0 load-bearing MEDIUM, 0 mis-anchor. 2 LOWs fixed proactively: F-P34-001 BC-1.15.001 v1.4 (volatile-pin removed); F-P34-002 VP-083 v1.10 (EPIC-COMPLETE derivation-source clarity). 3-CLEAN streak 0/3→1/3. 4-index BC v2.96/VP v2.28. Full: `git show 58a1cfe0:.factory/STATE.md` Phase Progress. |
| **D-598 F2 E-18 ADV PASS-35 NOT-CLEAN FIX BURST + ANNOTATION LESSON 2026-06-15** | **ARCHIVED per D-599** | Pass-35 NOT-CLEAN: 0B/0M; 2 MEDIUM self-inflicted; 2 LOW. Streak RESET 1/3→0/3. F-P35-001 BC-5.41.003 v1.8 §Changelog; F-P35-002 4-BC de-enumerated. L-F2-annotation-must-be-self-contained codified. 4-index BC v2.97. Full rows: `git show 7d4b600b:.factory/STATE.md` Phase Progress. |
| **D-599 F2 E-18 ADV PASS-36 NOT-CLEAN FIX BURST + ENUMERATE-AND-COUNT GATE + COMPACTION 2026-06-15** | **ARCHIVED per D-600** | Pass-36 NOT-CLEAN (1 MEDIUM self-inflicted; BC-1.15.001 v1.5 §Changelog; TRUE-EXHAUSTIVE all-8; enumerate-count gate; streak 0/3; 4-index BC v2.98/ARCH v2.46). Full: `git show b1c2b7e0:.factory/STATE.md` Phase Progress. |
| **D-600 F2 E-18 ADV PASS-37 NOT-CLEAN FIX BURST 2026-06-15** | **ARCHIVED per D-430(a) D-601** | Pass-37 NOT-CLEAN: 2 MEDIUM genuine latent (F-P37-001 ADR-026 v1.19 SS-01; F-P37-002 L2-INDEX v1.0.10 DI-025); 1 LOW F-P37-003 ADJUDICATED PRESERVED-HISTORY; streak 0/3; ARCH-INDEX v2.47/L2-INDEX v1.0.10. Full: `git show 2cdc70c7:.factory/STATE.md` Phase Progress. |
| **D-601..D-602 F2 E-18 ADV PASS-38..39 2026-06-15** | **ARCHIVED per D-603** | P-38 CLEAN(1/3); P-39 NOT-CLEAN RESET(0/3): VP-081 v1.7 HandoffMissing→SHELL + BC-5.41.002 v1.11 status tokens. 4-index: BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47. Full: `git show c53b42a1:.factory/STATE.md` Phase Progress. |
| **D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15** | **COMPLETE** | Pass-40 NOT-CLEAN: 0B/0M; 1 MED (F-P40-001 POLICY 2: invariants.md v1.20→v1.21 DI-020..025 `Cited by: VP-NNN` back-refs; bidirectional DI↔VP complete); 2 LOW (O-P40-001 preamble; O-P40-002 no-action). Streak 0/3. L2-INDEX v1.0.10→v1.0.11. 4-index UNCHANGED: BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47. Trajectory →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(1med POLICY-2). NEXT: adversary pass-41. |

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
| factory-artifacts | dfda6224 | D-603 F2 pass-40 NOT-CLEAN FIX BURST (invariants.md v1.21 DI↔VP back-refs POLICY 2; L2-INDEX v1.0.11; streak 0/3; 4-index BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47 UNCHANGED); prior: c53b42a1 D-602 pass-39 NOT-CLEAN |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE (marketplace PR #13 MERGED); annotated tag object |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; marketplace PR #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-603 2026-06-15; F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST; develop 7e99f6ef; main caf06c68** | rc.21 100% COMPLETE D-560; D-603 PASS-40 NOT-CLEAN FIX BURST (0 BLOCKER/MAJOR; 1 MED F-P40-001 invariants.md v1.21 DI↔VP back-refs POLICY 2; 2 LOW; streak 0/3; 4-index BC-INDEX v2.99 / VP-INDEX v2.29 / STORY-INDEX v4.01 / ARCH-INDEX v2.47 UNCHANGED; L2-INDEX v1.0.11); trajectory →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(1med); 3-CLEAN streak **0/3**; **Next: adversary pass-41 (fresh-context; DI↔VP bidirectional complete; 3-CLEAN strict human-directed; on 3/3 → F2 CONVERGED → pre-gate cleanup → F2 human gate → F3).** |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-556: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`
> D-557..D-588 archived to decision-log.md SoT per D-430(a) (D-568 + D-581 + D-586 + D-591 + D-593 compaction bursts).
> D-589..D-595 — D-589..D-593 archived per D-430(a) D-594; D-595 archived per D-430(a) D-596. D-596 archived per D-430(a) D-597. D-597 archived per D-430(a) D-598. Latest: D-598 in table below.
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md`
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-603 | F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15 — Fresh-context adversary pass-40 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM, 2 LOW. Streak remains 0/3. Fixed: (F-P40-001 MED, POLICY 2) invariants.md v1.20→v1.21 — added `Cited by: VP-NNN` back-refs to DI-020..025 (DI-019 precedent; VP→DI forward arrays existed but DI→VP reverse wasn't propagated at F-P19-003); completes bidirectional DI↔VP traceability. Mapping verified vs VP-INDEX: DI-020←VP-081/083/084/086, DI-021←VP-081/082/085, DI-022←VP-082/085, DI-023←VP-081, DI-024←VP-deferred-F3-S-18.05, DI-025←VP-082/084/085. Orchestrator adjudication: fixed (not self-demoted to CLEAN) — DI-019 precedent makes the omission re-flaggable; cheap consistent fix removes the vector. (O-P40-001 LOW) invariants.md preamble corrected — CAP-032 DIs (DI-020..025) trace to Source capability+ADR, not BR-NN business rule. (O-P40-002 LOW) BC-7.07.001 v1.3 changelog superseded-description: NO action — append-only historical record, confirmed non-defect. Adversary confirmed ALL behavioral contracts CLEAN (16+ consecutive passes). L2-INDEX v1.0.10→v1.0.11 (Document Map invariants.md v1.21 + changelog row D-603). 4-index: BC-INDEX v2.99 / VP-INDEX v2.29 / STORY-INDEX v4.01 / ARCH-INDEX v2.47 (UNCHANGED). 3-CLEAN streak 0/3 (NOT-CLEAN; 1 MED finding). Trajectory →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(1med POLICY-2 back-ref). NEXT: adversary pass-41 (fresh-context; DI↔VP bidirectional complete). D-chain cite D-602 per D-419(b); parent-commit c53b42a1 per D-419(b). | feature-mode-f2-e18-adv-pass-40-not-clean-fix-burst | 2026-06-15 |
| D-600..D-602 archived | **ARCHIVED 2026-06-15 per D-430(a) D-603** | D-600 pass-37 NOT-CLEAN (ADR-026 v1.19 SS-01; L2-INDEX v1.0.10); D-601 pass-38 CLEAN(1/3); D-602 pass-39 NOT-CLEAN RESET(VP-081 v1.7; BC-5.41.002 v1.11). 4-index BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47. Full rows: `git show c53b42a1:.factory/STATE.md` Decisions Log. |
| D-582..D-599 archived | **ARCHIVED 2026-06-15 per D-430(a) D-594..D-601** | D-582..D-599: passes 19-36 fix bursts; D-594 FULL-BACKLOG-CLEARANCE; D-595 pass-32; D-596 pass-33; D-597 pass-34 CLEAN(1/3); D-598 pass-35 NOT-CLEAN; D-599 pass-36 NOT-CLEAN+ENUMERATE-COUNT-GATE. All rows: decision-log.md SoT. |
| D-413..D-581 archived | **COMPACTED 2026-06-15 per D-430(a) D-599** | D-413..D-498 (36 rows); D-499..D-509 (11 rows); D-510+D-522+D-525+D-526; D-527..D-548; D-549..D-581 (F2 adv passes 1-18 + pre-pass-14 sweep). All rows: decision-log.md SoT. Pre-D-413: `git show 20cb8e1c`. |

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
| **[process-gap] Cross-reference title/code/phrase sweep gate + title-cite-parity gate** | CODIFIED D-582; UPGRADED D-589 (4th recurrence; class CLOSED) | L-F2-cross-reference-title-code-sweep UPGRADED (D-589 4th recurrence: F-P26-001 BC-7.07.001 VP-085 truncated cite; D-589 ran FIRST exhaustive all-8-BC sweep → class CLOSED). Reactive one-site-at-a-time fixing insufficient (4 recurrences: F-P19-001 BC-4.14.001, F-P19-001-sibling BC-5.41.001, F-P22-001/F-P25-003 ADR, F-P26-001 BC-7.07.001). MECHANICAL GATE NOW MANDATORY: for EVERY `VP-NNN — <title>` cite in any BC §VP Anchors / §Traceability AND ADR §VP Allocations, grep-based check MUST assert `<title>` equals VP file H1 verbatim. Gate runs exhaustively across ALL cite sites every spec-touch. Anchor: S-18.08 consistency-validator MANDATORY scope extension. |
| **[process-gap] Subsystem-anchor-sweep sibling-discipline gate** | CODIFIED 2026-06-15 — D-584 capture | L-F2-subsystem-anchor-sweep codified (2nd recurrence: F-P16-001/F-P21-002 VP sibling; F-P20-001/F-P21-001 Cross-Walk vs Document Map). When a VP/BC subsystem anchor changes OR a capability's Subsystems: line is referenced, fix-burst MUST sweep ALL VPs sharing source-BC AND L2-INDEX Cross-Walk AND Document Map same-burst. Candidate POLICY 5 category (j) + S-18.08 gate scope extension (VP-cluster scope changes trigger Cross-Walk audit). Anchor: E-18 F3 (S-18.08-class gate story or dedicated S-18.NNN). |
| **[process-gap] Canonical-scope-verification discipline** | CODIFIED 2026-06-15 — D-587 capture | L-F2-canonical-scope-verification codified (field-4 provenance ambiguity: 5 passes from D-572/D-573 over-correction to D-587 (B)-reconciliation). When authoring an invariant constraining a field produced by one agent type and consumed by another (shell writes / WASM reads), MUST explicitly name the scope boundary. Field-4 canonical (B) now enshrined: shell MAY exec `git cat-file -t SHA_B`; WASM reads field-4 STATICALLY. S-7.02 defensive sweep applies to invariant-scope-propagation sweeps. Anchor: E-18 F3 gate-story candidate (consistency-validator check for ambiguous no-git-exec constructs in WASM-adjacent prose). |
| **[process-gap] Stale-term-deferral-unsafe discipline** | CODIFIED 2026-06-15 — D-594 FULL BACKLOG CLEARANCE | L-F2-stale-term-deferral-unsafe codified (3-pass deferral cycle: O-P29-001→F-P30-001 LOW→F-P31-001 MED re-escalation; streak RESET 2/3→0/3). RULE: stale terms in normative present-tense prose MUST be fixed in-scope; deferral as LOW is convergence-risk (fresh adversary severity independent). Stale-term sweeps MUST be exhaustive. Full backlog cleared D-594: F-P27-001+002+F-P30-001/002/003+O-P29-001/002/003 all FIXED. Package zero-known-findings for pass-32. Candidate S-18.08 WASM stale-term detector gate (retired-terminology list against normative BC/VP/ADR prose). Anchor: E-18 F3. |
| **F-P27-001 (LOW) ARCH-INDEX ADR-026 changelog §Decision-9-order** | **FIXED D-594** — ARCH-INDEX v2.45→v2.46; ADR-026 v1.16 narrative corrected to stable reference 'see ADR-026 §Decision 9 canonical 5-step order' (non-load-bearing; no behavioral change). |
| **F-P27-002 (LOW) BC-4.14.001 EC-015/016 undeclared 4th column** | **FIXED D-594** — BC-4.14.001 v1.11→v1.12; EC-015/EC-016 4th column folded into 3-col layout (cosmetic; content unchanged). |
| **O-P29-001/O-P29-002/O-P29-003 + F-P30-001/F-P30-002/F-P30-003 (LOW deferred)** | **ALL FIXED D-594** — F-P30-001 = O-P29-001 class: VP-085 v1.7 + ADR-026 v1.18 stale 'side-channel' adjective removed; F-P30-002 = O-P29-002: BC-5.41.001 v1.15 PC5 non-null-when-log-absent VALID clarification + SS-05/06 split note; F-P30-003 = O-P29-002: BC-5.41.002 v1.7 BrokenSprintState message aligned to ADR; O-P29-003 = VP-086 v1.3 SS-04 justification strengthened. Package zero-known-findings. |
| **F-P32-001..F-P32-006 (ALL FIXED D-595)** | **ALL FIXED D-595** — F-P32-001 MED: BC-4.14.001 v1.13 PC7 SCALAR/LIST discrimination fixed. F-P32-002 MED: VP-083 v1.9 discriminating fixture (EPIC-COMPLETE ordering proof). F-P32-003 LOW: BC-5.41.001 v1.16 v1.5-skip annotation. F-P32-004 LOW: VP-086 v1.4 SS-04 shell-hook-via-adapter reword. F-P32-005 LOW: BC-7.07.001 v1.10 Inv7 log-pruning note. F-P32-006 LOW: 3 BC TBD-VPs → DECIDED DEFERRED-VP-F3 (BC-5.41.002 S-18.01; BC-6.24.001 S-18.03; BC-7.07.002 S-18.05 MANDATORY per DI-024). Package zero-known-findings. |
| **F-P33-001 (MED FIXED D-596)** | **FIXED D-596** — v1.5-skip-marker sibling-sweep gap: BC-5.41.002 v1.9 + BC-6.24.001 v1.8 + BC-7.07.002 v1.10 skip markers added. Exhaustive 8-BC sweep; class CLOSED. Package zero-known-findings; documentary asymptotic floor (zero behavioral defects). |
| **F-P34-001 (LOW FIXED D-597)** | **FIXED D-597** — BC-1.15.001 v1.3→v1.4: §VP Anchors + §Traceability ADR frozen-bound 'behaviorally unchanged through v1.6'→'behaviorally stable' (POLICY-19 stable-anchor form; volatile-pin escalation vector removed). |
| **F-P34-002 (LOW FIXED D-597)** | **FIXED D-597** — VP-083 v1.9→v1.10: proof-harness preamble note added pinning EPIC-COMPLETE derivation to handoff_content.next_wave_stories==[] evaluated at gate time, NOT precomputed is_first_wave. POLICY-11 tautology-risk degree-of-freedom closed. |
| **F-P35-001 (MEDIUM FIXED D-598)** | **FIXED D-598** — BC-5.41.003 v1.7→v1.8: §Changelog section added (was structurally absent; type-parity gap POLICY 17; 9 rows transcribed from modified[] frontmatter; v1.5 is a real behavioral entry per F-P5-002 re-grounding — no skip-marker needed). |
| **F-P35-002 (MEDIUM FIXED D-598)** | **FIXED D-598** — BC-5.41.001 v1.16→v1.17, BC-5.41.002 v1.9→v1.10, BC-6.24.001 v1.8→v1.9, BC-7.07.002 v1.10→v1.11: v1.5-skip-marker de-enumerated — removed false enumeration 'only BC-4.14.001 and BC-7.07.001 changed at pass-5' (BC-5.41.003 also changed at v1.5 per F-P5-002); reworded to self-contained local-fact-only form 'this BC received no behavioral change at the coordinated pass-5 burst'; premise-error class permanently closed via de-enumeration. |
| **[process-gap] L-F2-annotation-must-be-self-contained** | CODIFIED D-598 — annotation/marker text MUST NOT enumerate sibling BC IDs; structural-parity sweeps MUST VERIFY §Changelog section EXISTS before claiming exhaustive coverage. Candidate S-18.08 gate scope extension: consistency-validator check for BC-ID enumeration in skip-marker annotation text. Extends L-F2-stale-term-deferral-unsafe + L-F2-cross-reference-title-code-sweep. Anchor: E-18 F3 story decomposition (S-18.08 class). |
| **[process-gap] L-F2-exhaustive-sweep-enumerate-and-count** | CODIFIED D-599 — exhaustive-sweep attestations MUST enumerate all N target files AND capture per-file grep stdout as evidence; subset-scoping (sweeping only amended BCs) is FORBIDDEN and generates false-green attestations. Root cause: F-P35-001 (BC-5.41.003 missed) + F-P36-001 (BC-1.15.001 missed) from same false-exhaustive-attestation at D-596. Mechanical gate: `grep -c <pattern>` across all-N; all must satisfy predicate. Candidate S-18.08 MANDATORY scope extension. D-599 TRUE-EXHAUSTIVE run: all 8 E-18 BCs §Changelog count=1. Anchor: E-18 F3 story decomposition (S-18.08 class). |
| **F-P37-001 (MED FIXED D-600)** | **FIXED D-600** — ADR-026 v1.18→v1.19: SS-01 added to anchors: + subsystems_affected: frontmatter (latent scope-list gap: Decision 11 makes an SS-01 dispatcher-routing decision; BC-1.15.001/VP-086/CAP-032 all confirm SS-01 in-scope; ADR was sole artifact dropping it). §SS-08 Scope Clarification extended to explain SS-01 inclusion. §Traceability 'potentially SS-01 per S-18.00 outcome' stale hedge corrected to definitive. ARCH-INDEX v2.46→v2.47 (amendment row + SS-01 column in ADR-026 table row). |
| **F-P37-002 (MED FIXED D-600)** | **FIXED D-600** — L2-INDEX v1.0.9→v1.0.10: v1.0.9 changelog row corrected — DI-022→DI-025 and F-P24-001→F-P24-002. The invariants.md v1.20 change was to DI-025 (field-4/git-cat-file enforcement-owner clause); DI-022 is the hermetic-flush invariant (unrelated). F-P24-002 was the finding covering DI-025; F-P24-001 was the VP-082 PC-A fix. Factual mislabel corrected; no behavioral change. |
| **F-P37-003 (LOW ADJUDICATED D-600)** | **RESOLVED-AS-PRESERVED-HISTORY D-600** — Nested [Prior:] false-enumeration residue in BC-5.41.002/6.24.001/7.07.002 last_amended frontmatter: these [Prior:] chains are FAITHFUL HISTORICAL RECORDS per POLICY 1 (append-only-history). The live/current last_amended head + visible §Changelog tables were correctly de-enumerated by F-P35-002 (D-598). Retroactive rewriting of [Prior:] is forbidden — append-only history is sacrosanct. NOT a defect in live artifacts. Future adversary passes MUST NOT re-flag as a new finding; this entry documents the adjudication. |
| **[forward-track] F3 VP obligations (3 TBD-VPs decided as DEFERRED-VP-F3)** | FORWARD-TRACKED — BC-5.41.002 VP: S-18.01 integration anchor; BC-6.24.001 VP: S-18.03 integration anchor; BC-7.07.002 VP: S-18.05 MANDATORY per DI-024 (postcompact-reanchor.sh built at S-18.05). BC-7.07.001 Inv7 log-pruning: S-18.04 AC-N. Anchors: E-18 F3 story decomposition. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-15 — D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST; 4-index: BC-INDEX v2.99/VP-INDEX v2.29/STORY-INDEX v4.01/ARCH-INDEX v2.47 UNCHANGED; L2-INDEX v1.0.11; streak 0/3; NEXT: adversary pass-41 → fresh-context; STRICT 3-CLEAN human-directed)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION OR NEW MACHINE**
> Read this section alone to resume. Assumes ZERO prior context. All decisions, directives, and anchors stated explicitly.

### §1. Where We Are

**E-18 CAP-032 context-durability (GitHub issue #173) — Feature Mode, Phase F2 (spec evolution) adversarial 3-CLEAN convergence cascade (BC-5.39.001). D-603 2026-06-15.**

F1-gate APPROVED. F2 spec evolution COMPLETE (D-561). F2 adversarial cascade passes 1-40 COMPLETE (D-562..D-603). Consistency-validator sweep D-575 COMPLETE (CLEAN). Pass-22 CLEAN (1/3); pass-23 CLEAN (2/3); pass-24 NOT-CLEAN (streak RESET; D-587); passes 25-26 NOT-CLEAN; pass-27 CLEAN (1/3 D-590); pass-28 NOT-CLEAN (RESET D-591); pass-29 CLEAN (1/3 D-592); pass-30 CLEAN (2/3 D-593); pass-31 NOT-CLEAN (RESET 2/3→0/3; D-594 FULL BACKLOG CLEARANCE); pass-32 NOT-CLEAN (streak 0/3; D-595); pass-33 NOT-CLEAN (streak 0/3; D-596); pass-34 CLEAN (streak 1/3; D-597); pass-35 NOT-CLEAN (streak RESET; D-598); pass-36 NOT-CLEAN (streak 0/3; D-599); pass-37 NOT-CLEAN (streak 0/3; D-600 — 2 genuine latent MEDIUM); pass-38 CLEAN (streak 1/3; D-601); pass-39 NOT-CLEAN (streak RESET 1/3→0/3; D-602); pass-40 NOT-CLEAN (streak 0/3; D-603 — 1 MED POLICY-2 back-ref). STATE.md compacted at D-581+D-586+D-591+D-593+D-594+D-596+D-599+D-601+D-603 per D-430(a).

**D-603 pass-40 NOT-CLEAN summary:**
- Pass-40 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 load-bearing MEDIUM (F-P40-001), 2 LOW (O-P40-001, O-P40-002). Streak 0/3.
- F-P40-001 MEDIUM: invariants.md DI-020..025 missing `Cited by: VP-NNN` back-refs — POLICY 2 bidirectional DI↔VP traceability gap. DI-019 set the precedent (back-ref present); VP→DI forward arrays in VP files existed; DI→VP reverse missing at F-P19-003. Orchestrator adjudication: fixed (not demoted to CLEAN). FIXED: invariants.md v1.20→v1.21. Mapping: DI-020←VP-081/083/084/086; DI-021←VP-081/082/085; DI-022←VP-082/085; DI-023←VP-081; DI-024←VP-deferred-F3-S-18.05; DI-025←VP-082/084/085.
- O-P40-001 LOW: invariants.md preamble — CAP-032 DIs (DI-020..025) trace to Source capability+ADR, not BR-NN. FIXED in same burst.
- O-P40-002 LOW: BC-7.07.001 v1.3 changelog superseded-description: NO action — append-only historical record.
- L2-INDEX v1.0.10→v1.0.11 (Document Map invariants.md v1.21 sync + changelog D-603 row).
- 4-index UNCHANGED: BC-INDEX v2.99; VP-INDEX v2.29; STORY-INDEX v4.01; ARCH-INDEX v2.47.

**3-CLEAN streak: 0/3** (HUMAN-DIRECTED strict 3-CLEAN).

**Convergence trajectory (last 4 passes per D-433(e)+D-439(c)):** →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(0/3).

**4-index at D-603:** BC-INDEX v2.99, VP-INDEX v2.29, STORY-INDEX v4.01, ARCH-INDEX v2.47 (UNCHANGED). L2-INDEX v1.0.11.

**NEXT ACTION (explicit, in order):**
1. **START HERE: adversary pass-41** (fresh-context; DI↔VP bidirectional traceability complete; streak 0/3). STRICT 3-CLEAN HUMAN-DIRECTED.
2. On CLEAN pass-41 → 3-CLEAN streak 1/3 → adversary pass-42 → etc. until 3/3.
3. On 3/3 CLEAN → F2 CONVERGED → pre-gate cleanup of any leaveable items → F2 human gate approval → Feature Mode F3 story decomposition (story-writer authors S-18.00..S-18.07 + S-18.08 codified O-P8-002 gate-story).
4. On NOT-CLEAN pass-41 → fix burst → continue cascade.

**RECURRING LESSONS (carry):** (1) L-F2-stale-term-deferral-unsafe [process-gap] (D-594): stale terms in normative prose MUST be fixed in-scope; deferral as LOW is convergence-risk. (2) L-F2-canonical-scope-verification [process-gap] (D-587): When authoring an invariant for multi-agent-type scope boundary, MUST name the scope explicitly. Field-4 canonical (B) enshrined. (3) L-F2-subsystem-anchor-sweep [process-gap] (D-584; 2nd recurrence): When VP scope/capability Subsystems: line changes, sweep ALL sibling VPs + L2-INDEX Cross-Walk same-burst. (4) L-F2-cross-reference-title-code-sweep [process-gap] (D-582/D-589 UPGRADED; 4th recurrence): For EVERY `VP-NNN — <title>` cite in BC §VP Anchors / ADR §VP Allocations, grep-based check MUST assert title equals VP H1 verbatim. Exhaustive across ALL sites. S-18.08 MANDATORY. (5) L-F2-annotation-must-be-self-contained [process-gap] (D-598): annotation text MUST NOT enumerate sibling BC IDs; structural-parity sweeps MUST VERIFY §Changelog EXISTS before attesting exhaustive coverage. (6) L-F2-exhaustive-sweep-enumerate-and-count [process-gap] (D-599 NEW): cohort sweeps MUST enumerate all N files AND capture per-file grep stdout; subset-scoping FORBIDDEN — false-green generator; enumerate-count gate MANDATORY for ALL cohort structural assertions; S-18.08 MANDATORY scope extension. (7) L-F2-prior-chain-append-only-history [adjudication D-600]: nested [Prior:] chains in last_amended frontmatter are FAITHFUL HISTORICAL RECORDS (POLICY 1 append-only); NOT retroactively rewritten even when the content they reference was factually incorrect at time-of-writing. The LIVE last_amended head is the authoritative state; [Prior:] is immutable archaeological record. Do NOT re-flag [Prior:] residue as a new finding if the live/current text is correct. (8) L-F2-index-quad-cite-reflects-last-bump [convention D-601]: 4-index self-cited quad in any index's changelog reflects the index state AS OF THAT INDEX'S OWN LAST BUMP; an untouched index correctly retains its prior quad — per-pass lag is EXPECTED, NOT a propagation gap; "fixing" it would inject false history; adversary MUST treat quad-lag on untouched indices as benign.

**Artifact versions at D-603:**
- **ADR-026 v1.19** (D-600; UNCHANGED at D-603); **BC-1.15.001 v1.5** (D-599; UNCHANGED at D-603); **BC-4.14.001 v1.13** (UNCHANGED at D-603); **BC-5.41.001 v1.17** (UNCHANGED at D-603); **BC-5.41.002 v1.11** (D-602; UNCHANGED at D-603); **BC-5.41.003 v1.8** (UNCHANGED at D-603); **BC-6.24.001 v1.9** (UNCHANGED at D-603); **BC-7.07.001 v1.10** (UNCHANGED at D-603); **BC-7.07.002 v1.11** (UNCHANGED at D-603); **VP-081 v1.7** (D-602; UNCHANGED at D-603); VP-082 v1.12; **VP-083 v1.10** (UNCHANGED at D-603); VP-084 v1.9; VP-085 v1.7; **VP-086 v1.4** (UNCHANGED at D-603); **invariants.md v1.21** (D-603 F-P40-001 MED: DI-020..025 `Cited by: VP-NNN` back-refs; bidirectional DI↔VP complete); capabilities.md v1.7; **BC-INDEX v2.99** (D-602; UNCHANGED at D-603); **VP-INDEX v2.29** (D-602; UNCHANGED at D-603); **ARCH-INDEX v2.47** (UNCHANGED at D-603); STORY-INDEX v4.01 (UNCHANGED at D-603); **L2-INDEX v1.0.11** (D-603 Document Map invariants.md v1.21 sync).

**Key design facts (so restart agent has them without re-deriving):**
- wave-1 no-op = `payload.wave_id == 1` (pure-parse WASM; wave_id absent → fail-closed HandoffIncomplete); EPIC-COMPLETE = `payload.next_wave_stories: []`; WASM gate pure-parse (shell wave-handoff derives wave_id from substrate).
- **PC7 SCALAR/LIST discrimination (D-595 F-P32-001 BC-4.14.001 v1.13):** SCALAR fields (wave_id, epic_status, etc.) empty→malformed per EC-004; LIST-typed fields (next_wave_stories, open_decisions, pending_fixes, process_gaps) empty list→PRESENT+VALID; next_wave_stories:[] = EPIC-COMPLETE signal per PC2a, NOT malformed. EPIC-COMPLETE ordering: branch fires (Inv3 step 2) BEFORE wave-1 no-op (step 3) — discriminating fixture VP-083 v1.9 proves ordering.
- flush = commit(LOCAL, capture SHA_B) → append-to-log → push(REMOTE); each step failure → exit 2; append-fail → reset --soft SHA_B^ if HEAD==SHA_B else exit 2 (human intervention).
- push-fail (network/remote after successful local commit + log append) → exit 2; local commit + log entry retained; retry is push-only (no re-commit, no re-append) per ADR-026 §F-P10-002 + BC-7.07.001 PC6b.
- precompact-flush-log 4 fields: `<ISO-timestamp> <SHA> <cycle>/<step> commit` (field-2=SHA, field-4=`commit` token); WASM reads fields, no git-exec.
- exemption logic: prefix + field-2 + field-4 all valid → NOT-EXEMPT (anti-forgery); field-4 ≠ `commit` (corruption) → EXEMPT via prefix-alone; field-4 absent/empty = corruption = EXEMPT.
- HANDOFF: 9 base fields + epic_status conditional 10th; terminal = {merged, withdrawn, cancelled}; factory_lock opt-in default null; PostCompact best-effort outside CAP-032; harness >= v2.1.105; POLICY 19 = stable §Decision N anchor (no version pins in BC cites).
- O-P8-002 gate MANDATORY (3rd recurrence D-571): for any BC with Invariant 1 pure-parse, consistency-validator must verify VP files with matching `source_bc`/`bcs[]` do NOT describe gate behavior via external-substrate reads.
- **STALE-TERM RULE (D-594 L-F2-stale-term-deferral-unsafe):** 'side-channel' adjective on precompact-flush-log is RETIRED. Canonical term is 'precompact-flush-log (append-only log)'. Historical-rationale prose retains 'side-channel' only for documenting the OLD superseded point-file design. No normative present-tense sites should remain.

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
- **D-603 carry:** F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15. NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 MED (F-P40-001 invariants.md v1.21 DI↔VP back-refs POLICY 2), 2 LOW. Streak 0/3. 4-index: BC-INDEX v2.99; VP-INDEX v2.29; STORY-INDEX v4.01; ARCH-INDEX v2.47 (UNCHANGED); L2-INDEX v1.0.11. Trajectory →P38 CLEAN(1/3)→P39 NOT-CLEAN(RESET)→P40 NOT-CLEAN(0/3). NEXT: adversary pass-41 (fresh-context; DI↔VP bidirectional complete). D-chain cite D-602 per D-419(b). parent-commit c53b42a1.
- **D-560 carry:** OPERATOR-INSTALL-VERIFIED 2026-06-13. rc.21 100% COMPLETE. NO remaining release action.
- **D-556 carry:** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-541 carry (partial):** VP IDs TBD per TD-VSDD-063. BC-6.23.001 ACTIVE per POL-14.

### §4. Tier-A Completed Log (most recent first)

- **D-603 (2026-06-15):** F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST. 0 BLOCKER, 0 MAJOR, 1 MED (F-P40-001 POLICY 2: invariants.md v1.20→v1.21 DI-020..025 `Cited by: VP-NNN` back-refs; DI-019 precedent; bidirectional DI↔VP complete), 2 LOW (O-P40-001 preamble; O-P40-002 no-action). Fixed. Streak 0/3. L2-INDEX v1.0.11. 4-index: BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47 (UNCHANGED). NEXT: adversary pass-41.
- **D-600..D-602 archived** per D-430(a) D-603. D-600 pass-37 NOT-CLEAN (ADR-026 v1.19; L2-INDEX v1.0.10; streak 0/3); D-601 pass-38 CLEAN(1/3); D-602 pass-39 NOT-CLEAN RESET(VP-081 v1.7; BC-5.41.002 v1.11; streak 0/3). Full: `git show c53b42a1:.factory/STATE.md` §4.
- **D-562..D-599 archived** per D-430(a) D-596..D-601 compactions. F2 adv passes 1-36 (D-562..D-599). Full entries: `git show 2cdc70c7:.factory/STATE.md` §4 and earlier SHAs per prior checkpoints.
- **D-531..D-561 archived** per prior compaction bursts. E-10 CASCADE SEALED D-531; rc.20 SHIPPED D-528; rc.21 SHIPPED D-558+D-559; operator-install verified D-560; F2 E-18 spec evolution COMPLETE D-561. Full: decision-log.md SoT.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-603 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: **D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST 2026-06-15 — Pass-40 NOT-CLEAN (0 BLOCKER/MAJOR; 1 MED F-P40-001 invariants.md v1.21 POLICY 2 DI↔VP back-refs; 2 LOW; streak 0/3); 4-index BC-INDEX v2.99/VP-INDEX v2.29/STORY-INDEX v4.01/ARCH-INDEX v2.47 (UNCHANGED); L2-INDEX v1.0.11; adversary pass-41 NEXT (fresh-context; DI↔VP bidirectional complete).**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + L-issue-169-176-worktree-identity + L-F2-phantom-field-gate + L-F2-sibling-sweep-tree-wide-gate + L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class + L-F2-payload-only-discriminator-recurrence-gate + **L-F2-cross-reference-title-code-sweep (UPGRADED D-589: title-cite-parity gate MANDATORY)** + L-F2-subsystem-anchor-sweep + L-F2-canonical-scope-verification + **L-F2-stale-term-deferral-unsafe (D-594 NEW [process-gap]: stale terms in normative prose MUST be fixed in-scope; deferral as LOW is convergence-risk; sweeps must be exhaustive)** + **L-F2-annotation-must-be-self-contained (D-598 NEW [process-gap]: annotation text MUST NOT enumerate sibling BC IDs; structural-parity sweeps MUST VERIFY §Changelog section EXISTS before claiming exhaustive coverage; candidate S-18.08 gate)** + **L-F2-exhaustive-sweep-enumerate-and-count (D-599 NEW [process-gap]: exhaustive-sweep attestations MUST enumerate all N inputs + per-file grep stdout; subset-scoping FORBIDDEN; FALSE-GREEN generator if not; enumerate-count-gate candidate MANDATORY S-18.08)** + **L-F2-prior-chain-append-only-history (D-600 ADJUDICATION: nested [Prior:] chains in last_amended frontmatter are FAITHFUL HISTORICAL RECORDS per POLICY 1; NOT retroactively rewritten; LIVE head is authoritative; do NOT re-flag [Prior:] residue if live text is correct)** + **L-F2-index-quad-cite-reflects-last-bump (D-601 CONVENTION [convention]: 4-index self-cited quad in any index's changelog reflects the index state AS OF THAT INDEX'S OWN LAST BUMP; untouched-index quad-lag is EXPECTED per-pass behavior, NOT a propagation gap; fixing it would inject false history; adversary MUST treat as benign)** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.99 | UNCHANGED at D-603. Last bumped D-602 (BC-5.41.002 v1.11 status tokens). total_bcs 1966 UNCHANGED. |
| VP-INDEX | v2.29 | UNCHANGED at D-603. Last bumped D-602 (VP-081 v1.7 HandoffMissing→SHELL attribution). |
| STORY-INDEX | v4.01 | UNCHANGED at D-603. E-18 stories S-18.00..S-18.08 NOT YET AUTHORED (F3 next after 3-CLEAN convergence). |
| ARCH-INDEX | v2.47 | UNCHANGED at D-603. Last bumped D-600 (ADR-026 v1.19 SS-01 scope added). |
| L2-INDEX | v1.0.11 | Bumped D-603 (F-P40-001 MED: Document Map invariants.md v1.20→v1.21 + changelog D-603 row; DI↔VP back-refs complete). |

4-index at D-603 (literal-shell): `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "2.99"; `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` → "2.29"; `grep "^version:" .factory/stories/STORY-INDEX.md` → "4.01"; `grep "^version:" .factory/specs/architecture/ARCH-INDEX.md` → "2.47"; `grep "^version:" .factory/specs/domain-spec/L2-INDEX.md` → "1.0.11".

### §9. Critical Anchors

- **factory-artifacts HEAD:** `dfda6224` (D-603 F2 pass-40 NOT-CLEAN FIX BURST 2026-06-15; prior: `c53b42a1` D-602 pass-39 NOT-CLEAN; prior prior: `cb90f9be` D-601 pass-38 CLEAN)
- **develop HEAD:** `7e99f6ef` (PR #186 fix + release.yml sync back-merge 2026-06-13)
- **main HEAD:** `caf06c68` (rc.21 bot bundle commit 2026-06-13)
- **v1.0.0-rc.21 tag:** `03054524` (SHIPPED; FULLY IN OPERATOR MARKETPLACE)
- **ADR-026 v1.19:** `decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md` (D-600 F-P37-001 MED: SS-01 added to anchors: + subsystems_affected:; §SS-08 Scope Clarification extended; §Traceability corrected)
- **BC-4.14.001 v1.13:** `ss-04/BC-4.14.001.md` (D-595 F-P32-001 MED PC7 field-validation contract rewritten — SCALAR empty→malformed; LIST-typed next_wave_stories/open_decisions/pending_fixes/process_gaps empty→VALID; PC7×PC2a contradiction closed; prior v1.12: D-594 F-P27-002)
- **BC-5.41.001 v1.17:** `ss-05/BC-5.41.001.md` (D-598 F-P35-002 MEDIUM: v1.5-skip-marker de-enumerated — removed false 'only BC-4.14.001+BC-7.07.001 changed' claim; self-contained local-fact wording; prior v1.16: D-595 F-P32-003 LOW)
- **BC-5.41.002 v1.11:** `ss-05/BC-5.41.002.md` (D-602 F-P39-002 LOW: status tokens in_progress→in-progress canonical hyphenated per ADR §Terminal-Wave enum; in_review→blocked; exhaustive grep: sole normative site; prior v1.10: D-598 F-P35-002 MEDIUM v1.5-skip-marker de-enumerated)
- **BC-1.15.001 v1.5:** `ss-01/BC-1.15.001.md` (D-599 F-P36-001 MEDIUM: §Changelog section added — was structurally absent; POLICY 17 body-section type-parity gap; 6 rows reconstructed from modified[] v1.0..v1.5; no skip-marker needed; TRUE-EXHAUSTIVE 8-BC sweep all=1; BC-INDEX cell updated v1.4→v1.5; prior v1.4: D-597 F-P34-001 LOW)
- **BC-5.41.003 v1.8:** `ss-05/BC-5.41.003.md` (D-598 F-P35-001 MEDIUM: §Changelog section added — was structurally absent; type-parity gap POLICY 17; 9 rows transcribed from modified[]; v1.5 real F-P5-002 row, no skip-marker; prior v1.7: D-591 stale-term class flush)
- **BC-6.24.001 v1.9:** `ss-06/BC-6.24.001.md` (D-598 F-P35-002 MEDIUM: v1.5-skip-marker de-enumerated — same de-enumeration; prior v1.8: D-596 F-P33-001 MED v1.5-skip-marker added)
- **BC-7.07.001 v1.10:** `ss-07/BC-7.07.001.md` (D-595 F-P32-005 LOW Inv7 log-pruning; UNCHANGED at D-597)
- **BC-7.07.002 v1.11:** `ss-07/BC-7.07.002.md` (D-598 F-P35-002 MEDIUM: v1.5-skip-marker de-enumerated — same de-enumeration; prior v1.10: D-596 F-P33-001 MED v1.5-skip-marker added)
- **VP-083 v1.10:** `verification-properties/VP-083.md` (D-597 F-P34-002 LOW: proof-harness preamble note pinning EPIC-COMPLETE derivation to handoff_content.next_wave_stories==[] not is_first_wave; POLICY-11 tautology-risk closed; prior v1.9: D-595 F-P32-002 MED discriminating fixture)
- **VP-086 v1.4:** `verification-properties/VP-086.md` (D-595 F-P32-004 LOW; UNCHANGED at D-597)
- **VP-085 v1.7:** `verification-properties/VP-085.md` (D-594 F-P31-001 MED stale-term; UNCHANGED at D-597)
- **VP-082 v1.12:** `verification-properties/VP-082.md` (D-588 F-P25-004 LOW; UNCHANGED at D-597)
- **VP-084 v1.9:** `verification-properties/VP-084.md` (D-587 F-P24-003 LOW; UNCHANGED at D-597)
- **VP-081 v1.7:** `verification-properties/VP-081.md` (D-602 F-P39-001 MED: PC-A HandoffMissing mis-attribution corrected — now explicitly SHELL wave-gate (BC-5.41.001 PC9); WASM NEVER emits HandoffMissing (BC-4.14.001 EC-011); exhaustive grep 41 hits VP-081 PC-A sole defective site; prior v1.6: F-P21-002 MED subsystem mis-anchor)
- **L2-INDEX v1.0.11:** `specs/domain-spec/L2-INDEX.md` (D-603 F-P40-001 MED: Document Map invariants.md v1.20→v1.21 + changelog row; prior: D-600 F-P37-002 MED v1.0.10)
- **invariants.md v1.21:** `domain-spec/invariants.md` (D-603 F-P40-001 MED: DI-020..025 `Cited by: VP-NNN` back-refs added; bidirectional DI↔VP traceability complete; DI-019 precedent followed)
- **VP-081..086 domain_invariants populated:** VP-081 [DI-020,DI-021,DI-023]; VP-082 [DI-021,DI-022,DI-025]; VP-083 [DI-020]; VP-084 [DI-020,DI-025]; VP-085 [DI-021,DI-022,DI-025]; VP-086 [DI-020]
- **capabilities.md v1.7:** `domain-spec/capabilities.md` (UNCHANGED at D-597)
- **ADR-025 v1.6 SHIPPED:** guard at `3b2a378c`; ARCH-INDEX v2.27
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.7 MERGED; E-17 W4 COMPLETE; PR #184 3b2a378c
- **Verify on resume:** `git rev-parse --short origin/develop` → expect `7e99f6ef`; `git rev-parse --short origin/main` → expect `caf06c68`

### §10. PR Status

- **0 open feature PRs. 0 open release PRs. 0 open marketplace PRs. rc.21 100% COMPLETE. E-18 F2 adversarial cascade in progress (no PR yet — D-603 pass-40 NOT-CLEAN; streak 0/3; adversary pass-41 NEXT on same frozen package; F3 follows 3-CLEAN convergence).**
- **marketplace PR drbothen/claude-mp #13 MERGED** 2026-06-13 — rc.21 FULLY SHIPPED.
- **RELEASING.md Step 9 VERIFIED (D-560):** operator cache 1.0.0-rc.21 confirmed (plugin.json + 132 entries). rc.21 end-to-end CLOSED.

### §11. Post-CLEAR/Post-RESET Resume Checklist (zero-context; D-603 refresh)

1. **Verify worktree state:** `git rev-parse --short origin/develop` → expect `7e99f6ef`. `git rev-parse --short origin/main` → expect `caf06c68`. `git -C .factory log -1` (expect D-603 F2 pass-40 NOT-CLEAN FIX BURST 2026-06-15; branch factory-artifacts; clean status).
2. **Read §1-§12 this checkpoint** (all of it; D-603 self-sufficient).
3. **Verify 4-index:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "2.99"; ARCH-INDEX → "2.47"; VP-INDEX → "2.29"; STORY-INDEX → "4.01"; L2-INDEX → "1.0.11". L2-INDEX BUMPED at D-603. 4-index proper UNCHANGED.
4. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
5. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
6. **RC.21 100% COMPLETE D-560.** NO remaining release action. Operators: `/plugin update vsdd-factory@claude-mp`.
7. **D-603 F2 E-18 ADV PASS-40 NOT-CLEAN FIX BURST.** Pass-40 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 1 MED (F-P40-001 invariants.md v1.21 DI↔VP back-refs POLICY 2), 2 LOW. Fixed. Streak 0/3. **NEXT: adversary pass-41 (fresh-context; DI↔VP bidirectional complete; STRICT 3-CLEAN human-directed).**
8. **4-index at D-603:** BC-INDEX v2.99, VP-INDEX v2.29, STORY-INDEX v4.01, ARCH-INDEX v2.47 (UNCHANGED). L2-INDEX v1.0.11.
9. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary + O-P8-002 MANDATORY (3rd recurrence) + **L-F2-cross-reference-title-code-sweep [process-gap] UPGRADED (D-589; 4th recurrence; title-cite-parity gate MANDATORY)** + L-F2-subsystem-anchor-sweep [process-gap] (2nd recurrence) + L-F2-canonical-scope-verification [process-gap] (D-587) + **L-F2-stale-term-deferral-unsafe [process-gap] (D-594: stale terms in normative prose MUST be fixed in-scope; exhaustive sweep mandatory)** + **L-F2-annotation-must-be-self-contained [process-gap] (D-598: annotation text must not enumerate sibling BC IDs; structural-parity sweeps must verify §Changelog EXISTS)** + **L-F2-exhaustive-sweep-enumerate-and-count [process-gap] (D-599 NEW: cohort sweeps MUST enumerate all N files + per-file grep stdout; subset-scoping FORBIDDEN; enumerate-count gate MANDATORY for ALL cohort structural assertions)** + **L-F2-prior-chain-append-only-history [adjudication D-600: nested [Prior:] chains are POLICY 1 immutable history; NOT re-flaggable if live text is correct]** + **L-F2-index-quad-cite-reflects-last-bump [convention D-601: 4-index quad-lag on untouched indices is EXPECTED; adversary MUST treat as benign; fixing it = false history injection]**.
10. **Latest decision D-603.** F2 pass-40 NOT-CLEAN FIX BURST (streak 0/3; invariants.md v1.21 + L2-INDEX v1.0.11). Adversary pass-41 NEXT (fresh-context; DI↔VP bidirectional complete; streak 0/3). On 3/3 CLEAN → pre-gate cleanup → human F2 gate → F3 S-18.00..S-18.08.

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-15 D-599)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| ~~1~~-~~prev~~ | ~~rc.21 through E-18 F2 adv passes 1-15~~ | ~~—~~ | ~~—~~ | **ALL CLOSED — D-560..D-578 2026-06-13/15.** |
| ~~1a~~-~~1j~~ | ~~#173/E-18 F2 adversarial re-cascade (passes 16-25)~~ | ~~feature~~ | ~~—~~ | **ALL DONE D-579..D-588 2026-06-15.** |
| ~~1k~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-26)~~ | ~~feature~~ | ~~D-588 fix burst complete~~ | **DONE D-589 2026-06-15 — NOT-CLEAN (1med/3obs); BC-7.07.001 v1.8; 3-CLEAN streak 0/3.** |
| ~~1l~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-27)~~ | ~~feature~~ | ~~D-589 fix burst complete~~ | **DONE D-590 2026-06-15 — CLEAN; 2 LOWs deferred; streak 0/3→1/3.** |
| ~~1m~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-28)~~ | ~~feature~~ | ~~D-590 bookkeeping complete~~ | **DONE D-591 2026-06-15 — NOT-CLEAN (2med); BC-5.41.003 v1.7 + ADR-026 v1.17; streak RESET 1/3→0/3.** |
| ~~1n~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-29)~~ | ~~feature~~ | ~~D-591 fix burst complete~~ | **DONE D-592 2026-06-15 — CLEAN; 3 LOWs deferred (O-P29-001/002/003); streak 0/3→1/3.** |
| ~~1o~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-30)~~ | ~~feature~~ | ~~D-592 bookkeeping complete~~ | **DONE D-593 2026-06-15 — CLEAN; 3 LOWs deferred (F-P30-001/002/003); 8 total deferred; streak 1/3→2/3.** |
| ~~1p~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-31)~~ | ~~feature~~ | ~~D-593 bookkeeping + compaction complete~~ | **DONE D-594 2026-06-15 — NOT-CLEAN (2med); streak RESET 2/3→0/3; FULL BACKLOG CLEARANCE (9 items fixed); 4-index BC v2.93/VP v2.26.** |
| ~~1q~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-32)~~ | ~~feature~~ | ~~D-594 FULL BACKLOG CLEARANCE complete~~ | **DONE D-595 2026-06-15 — NOT-CLEAN (2med: F-P32-001 PC7×PC2a + F-P32-002 EPIC-COMPLETE ordering; 4 LOW); ALL fixed in-scope; 4-index BC v2.94/VP v2.27/STORY v4.01/ARCH v2.46.** |
| ~~1r~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-33)~~ | ~~feature~~ | ~~D-595 fix burst complete~~ | **DONE D-596 2026-06-15 — NOT-CLEAN (1med: F-P33-001 v1.5-skip-marker sibling gap; BC-5.41.002 v1.9/BC-6.24.001 v1.8/BC-7.07.002 v1.10; 8-BC sweep CLOSED); 4-index BC v2.95.** |
| ~~1s~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-34)~~ | ~~feature~~ | ~~D-596 fix burst + compaction complete~~ | **DONE D-597 2026-06-15 — CLEAN (0B/0M/0LBM/0mis-anchor; 2 LOWs fixed proactively: F-P34-001 BC-1.15.001 v1.4 / F-P34-002 VP-083 v1.10); streak 0/3→1/3; 4-index BC v2.96/VP v2.28.** |
| ~~1t~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-35)~~ | ~~feature~~ | ~~D-597 CLEAN (streak 1/3)~~ | **DONE D-598 2026-06-15 — NOT-CLEAN (0B/0M; 2 MEDIUM self-inflicted: F-P35-001 BC-5.41.003 v1.8/F-P35-002 4-BC de-enumerated; class closed); streak RESET 1/3→0/3; 4-index BC v2.97.** |
| ~~1u..1x~~ | ~~#173/E-18 F2 adversarial re-cascade (passes 36-39)~~ | ~~feature~~ | ~~D-598..D-601 complete~~ | **ALL DONE D-599..D-602 2026-06-15.** Full rows: prior §12 (archived at `git show c53b42a1:.factory/STATE.md` §12). |
| ~~1y~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-40)~~ | ~~feature~~ | ~~D-602 NOT-CLEAN fix burst complete~~ | **DONE D-603 2026-06-15 — NOT-CLEAN (0B/0M; 1 MED F-P40-001 invariants.md v1.21 DI↔VP back-refs POLICY 2; 2 LOW; streak 0/3; L2-INDEX v1.0.11; 4-index UNCHANGED).** |
| **1** | **#173/E-18 F2 adversarial re-cascade (pass-41)** | **feature** | D-603 NOT-CLEAN fix burst complete | Fresh-context adversary; DI↔VP bidirectional traceability complete; streak 0/3; 3 CLEAN needed for convergence. STRICT 3-CLEAN human-directed. **START HERE.** |
| **2** | **#173/E-18 F3 story decomposition** | **feature** | F2 3-CLEAN convergence (3/3) | Author S-18.00..S-18.07+S-18.08 (S-18.08 ships mandatory O-P8-002 pure-parse invariant gate). STORY-INDEX v4.01→v4.02+. |
| **4** | **#173 wave-checkpoint** | **implementation** | E-18 F3 done OR human re-sequence | State-durability chain stories S-18.01..S-18.05. Blocked on F3. |
| **5** | **#171 deferred-revalidate** | **implementation** | #173 stories done | Deferred-revalidation story. |
| **6** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin. |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 cascade; rc.19+rc.20+rc.21; E-17 4 stories; S-15.17~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED/SHIPPED** |
| **7c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **8/9** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-602 F2-E18-ADV-PASS-39-NOT-CLEAN-FIX-BURST-2026-06-15) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
