---
document_type: pipeline-state
level: ops
version: "3.71"
status: draft
producer: state-manager
timestamp: 2026-06-17T00:00:00Z
phase: D-621-E18-PASS4-INDEX-SYNC-2026-06-17
last_amended: "2026-06-17 (v3.71) — D-621 E-18 STORY PASS-4 INDEX SYNC BURST: STORY-INDEX v4.05→v4.06 (S-18.04b story v1.4; S-18.09 story v1.4 + AC↔PC parity gate title; E-18 epic v1.0→v1.1); 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN fix-burst); pass-5 NEXT. Lesson L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 4-index: BC-INDEX v3.06 / VP-INDEX v2.36 / STORY-INDEX v4.06 / ARCH-INDEX v2.51. Refs: D-621, E-18, F-P4-001, O-P4-004, S-18.09, AC-008. [Prior: 2026-06-17 (v3.70) — D-620 E-18 STORY PASS-3 INDEX SYNC BURST: STORY-INDEX v4.04→v4.05 (S-18.04b SS-07→SS-04+SS-05+SS-07; S-18.09 wave 7→8; E-18 intro SS-08 removed; 8-wave schedule); VP-INDEX v2.35→v2.36 (9 wave cells TBD/wrong→correct: VP-081→wave 2/3; VP-082→wave 2; VP-083→wave 3; VP-085→wave 2; VP-086→wave 1; VP-087→wave 2; VP-088→wave 4; VP-089→wave 3; VP-092→wave 7 ME-001); epic count 18→19 (E-18 added D-614); 3-CLEAN streak reset 0/3; pass-4 re-verify NEXT. 4-index: BC v3.06/VP v2.36/STORY v4.05/ARCH v2.51/L2 v1.0.13. [Prior: 2026-06-17 (v3.69) — D-619 BC-INDEX COUNT RECONCILE BURST: total_bcs 1968→1972 (counting rule: all catalog entries including withdrawn per POLICY 1 append-only = 1972); Summary table BC-1 118→117; BC-3 53→56; BC-5 660→655; BC-7 200→201; BC-8 214→222; Total 1966→1972; subsystem headers corrected; Drift Item D-562 RESOLVED (BC-2.02.013 correctly characterized as legit-withdrawn audit-trail BC); D-619 added Decisions Log + decision-log.md SoT; process-gap lesson codified; BC-INDEX v3.05→v3.06. [Prior: 2026-06-16 (v3.68) — D-618 STATE.md durability + resume refresh. See prior entries. [Prior: 2026-06-16 (v3.67) — D-617 banner-block repair: D-609 leading integer fixed (410 lines); D-610..D-616 entries appended; L-F2-statemd-banner-wcl-each-burst [process-gap] codified; bats-full-suite CI unblocked. [Prior: 2026-06-16 (v3.66) — D-616 E-18 STORY PASS-2 FIX WAVE INTEGRATION BURST: compute-input-hash awk+resolver bug FIXED (PR→develop PENDING→MERGED c000b06f D-618); all 12 E-18 input-hashes recomputed; SS-08 sweep; S-18.10 W6→W7; VP anchor_story swept; BC-6.25.001 input-hash 2d42b26; verification-architecture.md (91)→(92); 4-index: BC-INDEX v3.05/VP-INDEX v2.35/STORY-INDEX v4.04/ARCH-INDEX v2.51. [Prior: 2026-06-16 (v3.64/v3.65) — D-614/D-615 F3 STORY REGISTRATION + PASS-1 FIX WAVE. See decision-log.md SoT.]]]]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
"D-621 E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17. STORY-INDEX v4.06 (S-18.04b v1.4; S-18.09 v1.4 + AC↔PC parity gate; E-18 epic v1.1); L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN). 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13. POSTURE: E-18 story pass-5 re-verification NEXT — START HERE."
current_step: "D-621-E18-PASS4-INDEX-SYNC-2026-06-17"
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

  D-430(a) compaction history D-596..D-603 COLLAPSED to range-reference 2026-06-15 D-605:
  D-596 ~395 lines (git show a46b67af); D-598 ~421 lines (git show 58a1cfe0→archived); D-599 ~390 lines (git show b1c2b7e0); D-600 ~430 lines (git show 2cdc70c7); D-601 ~405 lines; D-603 (git show c53b42a1). Full compaction notes per burst preserved in factory-artifacts git log.

  D-604..D-608 compaction history COLLAPSED 2026-06-16 D-609 (per D-430(a); full per-burst notes in factory-artifacts git log; git show 81f240e4/88993db1/6da38863/c4ed73bf/[D-608-SHA]:.factory/STATE.md for per-burst state).

  410 lines (wc-l; D-609: E-18 CONFIRMING-PASS FIX BURST — BC-7.07.001 v1.12 + VP-087/090 v1.2 + BC-INDEX v3.01 + VP-INDEX v2.32; D-430(a) compaction; D-446(c) dual-margin).
  403 lines (wc-l; D-610: E-18 CONFIRMING adversary pass (round 2) CLEAN — E-18 F2 FULLY COMPLETE; L-F2-no-bypass-on-edit-failure codified; 4-index UNCHANGED).
  404 lines (wc-l; D-611: E-18 F3 decomposition plan APPROVED (human gate) — 11 stories S-18.00..S-18.09).
  411 lines (wc-l; D-612: BC-4.15.001 + VP-091 + ADR-026 v1.20 integration burst; 4-index BC v3.02/VP v2.33/ARCH v2.50).
  416 lines (wc-l; D-613: BC-4.15.001 v1.1 cleanup burst; BC-INDEX v3.03; 4-index BC v3.03/VP v2.33/ARCH v2.50).
  415 lines (wc-l; D-614: E-18 F3 story registration — 11 stories S-18.00..S-18.09 STORY-INDEX v4.02; story_count 108→119; 431+ pts).
  424 lines (wc-l; D-615: E-18 story pass-1 fix wave integration — S-18.10 added; STORY-INDEX v4.03; 4-index BC v3.04/VP v2.34).
  415 lines (wc-l; D-616: E-18 story pass-2 fix wave integration — input-hashes recomputed + SS-08 sweep + VP anchor_story + STORY-INDEX v4.04; 4-index BC v3.05/VP v2.35).
  425 lines (wc-l; D-617: STATE.md SIZE BUDGET banner block repair — D-609 leading integer added; D-610..D-616 banner entries appended; unblocks bats-full-suite CI job; L-F2-statemd-banner-wcl-each-burst codified).
  405 lines (wc-l; D-618: STATE.md durability + resume refresh — develop HEAD c000b06f (PR #189 MERGED); §1/§3/§4/§5/§9/§10/§11 stale-anchor corrections; compaction D-430(a); banner entry dogfooded; 4-index UNCHANGED).
  408 lines (wc-l; D-619: BC-INDEX COUNT RECONCILE BURST — total_bcs 1968→1972; Summary table reconciled; Drift Item D-562 RESOLVED; BC-INDEX v3.06).
  412 lines (wc-l; D-620: E-18 STORY PASS-3 INDEX SYNC BURST — STORY-INDEX v4.05; VP-INDEX v2.36; S-18.09 wave 7→8; S-18.04b subsystems fix; 9 VP wave cells; epic count 19; streak reset 0/3; pass-4 NEXT).
  417 lines (wc-l; D-621: E-18 STORY PASS-4 INDEX SYNC BURST — STORY-INDEX v4.06; S-18.04b v1.4; S-18.09 v1.4 + AC↔PC parity gate; E-18 epic v1.1; L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008; streak 0/3; pass-5 NEXT).
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
| **Last Updated** | 2026-06-17 — D-621 E-18 STORY PASS-4 INDEX SYNC BURST. STORY-INDEX v4.06 (S-18.04b v1.4; S-18.09 v1.4 + AC↔PC parity gate; E-18 v1.1). L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 3-CLEAN streak 0/3. 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13. |
| **Current Phase** | D-621 E-18 STORY PASS-4 INDEX SYNC BURST COMPLETE 2026-06-17. 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13. **POSTURE: E-18 story pass-5 re-verification NEXT — START HERE.** |
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
| **D-601..D-606 F2 E-18 ADV PASS-38..43 2026-06-15** | **ARCHIVED per D-607** | P-38 CLEAN(1/3); P-39 NOT-CLEAN RESET; P-40 NOT-CLEAN; P-41 CLEAN ZERO-FINDINGS(1/3; FROZEN); P-42 CLEAN(2/3); P-43 CLEAN(3/3)-CONVERGED; BC-5.39.001 satisfied; 4-index UNCHANGED BC v2.99/VP v2.29/STORY v4.01/ARCH v2.47. Full: `git show c4ed73bf:.factory/STATE.md` Phase Progress. |
| **D-607..D-616 F2+F3 E-18 INTEGRATION + FIX WAVES 2026-06-16** | **ARCHIVED per D-430(a)** | D-607 spec-completion integration; D-608 delta fix (VP v2.31/ARCH v2.49); D-609 fix (BC v3.01/VP v2.32); D-610 CONFIRMING CLEAN — E-18 F2 FULLY COMPLETE; D-611..D-615 F3 story authoring + pass-1 fix wave (S-18.10 added; 120 stories; STORY-INDEX v4.03; 4-index BC v3.04/VP v2.34); D-616 pass-2 fix wave (input-hashes recomputed; SS-08 sweep; VP anchor_story; STORY-INDEX v4.04; 4-index BC v3.05/VP v2.35). Full: decision-log.md SoT. |

## Current Phase Steps

> **Rows before pass-57 archived to** `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` per STATE.md content-routing rules (keep last 5 only).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| F5 passes 18-60 fix bursts (archived) | state-manager | ARCHIVED | See `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`. Passes 57-59: D-437..D-439 (META-LEVEL-12/13/14); pass-60: D-440 META-LEVEL-15 CONFIRMED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,972 (BC-INDEX v3.06) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 92 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 117 file-resident + 15 stub IDs (STORY-INDEX v4.06) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 19 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 26 |

## Story Status

117 file-resident + 15 unauthored stub IDs = 132 stories registered (12 E-18 stories; S-18.10 added D-615; input-hashes corrected D-616).

- **Merged (78):** Includes S-17.01 (PR #181 c64b46d2) + S-17.02 (PR #182 df4f26b8) + S-17.03 (PR #183 60fd0233) + S-17.04 (PR #184 3b2a378c). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **Draft (41 file-resident):** S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); S-18.00..S-18.10 (E-18; 12 stories); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Unauthored stub IDs (15):** S-9.01..S-9.07 (W-16); S-11.01..S-11.08 (E-11 W-17 Tier 3)
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | caf06c68 | rc.21 bot binary bundle commit 2026-06-13; prior: 2a191314 (rc.20) |
| develop | c000b06f | PR #189 compute-input-hash fix SQUASH-MERGED 2026-06-16; prior: 7e99f6ef (PR #186 fix 2026-06-13) |
| factory-artifacts | b26ef433 | D-621 E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17; prior: a5aaeea5 D-620 Commit-E (SHA-patch e12a6b35) |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE (marketplace PR #13 MERGED); annotated tag object |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; marketplace PR #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-621 2026-06-17; E-18 STORY PASS-4 INDEX SYNC COMPLETE (STORY-INDEX v4.06; S-18.04b v1.4; S-18.09 v1.4 + AC↔PC parity gate; E-18 v1.1); 3-CLEAN streak 0/3 (pass-4 fix-burst); develop c000b06f (PR #189 MERGED); main caf06c68** | rc.21 100% COMPLETE D-560; D-606 PASS-43 3-CLEAN CONVERGED; D-617 banner repair; D-618 resume-durability refresh; D-619 BC-INDEX count reconcile (total_bcs 1972; BC-INDEX v3.06); D-620 E-18 pass-3 index-sync (STORY-INDEX v4.05; VP-INDEX v2.36; epic count 19); **D-621 E-18 pass-4 index-sync: STORY-INDEX v4.06; L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. POSTURE: E-18 story pass-5 adversary + consistency NEXT — START HERE.** |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. All archived per D-430(a) compaction bursts.

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-621 | E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17 — STORY-INDEX v4.05→v4.06: S-18.04b BCs cell — story v1.4 annotation (F-P4-001 MAJOR: AC-002/003/004 PC traces corrected via exhaustive 12-story sibling sweep; POLICY 5 v1.3.3); S-18.09 title — appended `, AC↔PC parity gate` (O-P4-004 process-gap class fix; closes recurring AC↔PC mis-trace recurrence S-18.02/S-18.04a/S-18.04b); S-18.09 BCs cell — story v1.4 + AC-008 AC↔PC parity gate (mandatory gate; F-P4-004 task/AC updates); E-18 epic heading v1.0→v1.1. Lesson L-F2-ac-pc-parity-sibling-sweep [process-gap] codified + tagged [codified] with anchor S-18.09 AC-008 per Cycle-Closing Checklist process-gap requirement. 3-CLEAN streak: pass-4 NOT-CLEAN→fix-burst; streak RESET 0/3; pass-5 NEXT. Parent-commit: e12a6b35 (D-620 SHA-patch). | E-18-pass4-index-sync | 2026-06-17 |
| D-620 | E-18 STORY PASS-3 INDEX SYNC BURST 2026-06-17 — STORY-INDEX v4.04→v4.05: S-18.04b subsystems SS-07→SS-04+SS-05+SS-07 (M-001; validate-burst-log=SS-04, validate-dispatch-advance=SS-05, precompact-flush-prune.sh=SS-07; ground truth: story frontmatter); S-18.09 wave 7→8 (F-SP3-001 BLOCKER — intra-wave dep on S-18.08 W7; story frontmatter verified); E-18 intro subsystems SS-01/04/05/06/07/08→SS-01/04/05/06/07 (M-003; no E-18 story has SS-08); DAG wave-schedule 7-wave→8-wave (W7={S-18.08,S-18.10}; W8=S-18.09); delivery note updated. VP-INDEX v2.35→v2.36: 9 wave cells corrected from anchor_story ground truth (literal grep): VP-081 TBD→wave 2/3; VP-082 wave 3→wave 2; VP-083 TBD→wave 3; VP-085 wave 3→wave 2; VP-086 TBD→wave 1; VP-087 TBD→wave 2; VP-088 TBD→wave 4; VP-089 TBD→wave 3; VP-092 wave 6→wave 7 (ME-001). Epic count STATE.md Identifier Conventions 18→19 (E-18 added D-614; count not previously bumped). 3-CLEAN streak: pass-3 NOT-CLEAN→fix-burst; streak RESET 0/3; pass-4 re-verify NEXT. Parent-commit: a828686b (D-619 SHA-patch). | E-18-pass3-index-sync | 2026-06-17 |
| D-619 | BC-INDEX COUNT RECONCILE BURST 2026-06-17 — Counting rule established: all catalog entries including withdrawn (POLICY 1 append-only) count toward total_bcs and per-subsystem Summary rows. Literal-shell recount: catalog=disk=1972 (1971 active + 1 withdrawn BC-2.02.013). total_bcs 1968→1972. Summary corrections: BC-1 118→117; BC-3 53→56; BC-5 660→655; BC-7 200→201; BC-8 214→222; Total 1966→1972. Subsystem header corrections: SS-01 118→117; SS-03 53→56; SS-07 200→201. BC-INDEX v3.05→v3.06. Drift Item D-562 RESOLVED: BC-2.02.013 correctly characterized as legitimately-withdrawn audit-trail BC per POLICY 1 (NOT orphan). Process-gap lesson: total_bcs not auto-recounted on BC add/withdraw; feeds S-18.08/S-18.09 gate-story scope. Parent-commit: 0bf5cc7a (D-618 SHA-patch). | BC-INDEX-count-reconcile | 2026-06-17 |
| D-618 | STATE.md durability + resume refresh 2026-06-16 — stale-anchor corrections: develop HEAD 7e99f6ef→c000b06f (PR #189 compute-input-hash fix SQUASH-MERGED to develop; fix branch deleted); PR #189 status PENDING→MERGED in §10 + Drift Items; §1 NEXT ACTION updated; §3 D-618 carry added; §4 Tier-A entry added; §5 updated latest; §9 Critical Anchors develop+factory-artifacts corrected; §11 Resume Checklist verify-on-resume develop=c000b06f + posture pass-3 START HERE; §12 PR-first DONE confirmed; SIZE BUDGET banner D-618 entry appended (L-F2-statemd-banner-wcl-each-burst dogfooded); compaction D-430(a). 4-index UNCHANGED: BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51/L2 v1.0.13. Single-commit + SHA-patch follow-up per D-447(c)/D-449(e). Parent-commit: cea9e0a6 (D-617 Commit-E). | STATE.md-durability-resume | 2026-06-16 |
| D-617 | STATE.md SIZE BUDGET banner block repair 2026-06-16 — D-609 banner entry corrected (leading integer 410); D-610..D-616 entries appended; L-F2-statemd-banner-wcl-each-burst codified; bats-full-suite CI unblocked. 4-index UNCHANGED. Parent-commit: e7dc11bf (D-616-cleanup). | STATE.md-banner-repair | 2026-06-16 |
| D-616 | E-18 STORY PASS-2 FIX WAVE INTEGRATION 2026-06-16 — compute-input-hash awk+resolver bug FIXED (ea6cf1af+5b0d5e5c; PR #189→develop MERGED D-618); 12 E-18 input-hashes recomputed; BC-6.25.001 hash 2d42b26; SS-08 sweep; S-18.10 W6→W7; VP anchor_story corrected; verification-architecture.md (91)→(92); 4-index BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51. 2 lessons: L-F2-fix-wave-must-sweep-downstream + L-F2-input-hash-tool-trust. | feature-mode-e18-story-pass2-fix-wave-integration | 2026-06-16 |
| D-413..D-615 archived | **ARCHIVED per D-430(a)** | D-413..D-606: F2 adv passes 1-43 + CONVERGED; D-607..D-610 F2 integration+confirming (E-18 F2 FULLY COMPLETE); D-611..D-613 F3 plan+BC-4.15.001; D-614 11 stories S-18.00..S-18.09 STORY-INDEX v4.02 (story_count 108→119); D-615 pass-1 fix wave (S-18.10 added; 4-index BC v3.04/VP v2.34). All rows: decision-log.md SoT. Pre-D-413: `git show 20cb8e1c`. |

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
| **[system-level deferral] ARCH-INDEX §Future Sections (Deferred) rows — verification-architecture.md + verification-coverage-matrix.md** | **RESOLVED 2026-06-16 — D-607** | Both files materialized per D-607 F2 gate human directive. ARCH-INDEX v2.48: both rows REMOVED from §Future Sections (Deferred) and ADDED to §Document Map as active registered documents. Story-ID anchor question moot — files are materialized. D-606 Drift Item CLOSED. |
| **[tool-fix] compute-input-hash awk+resolver bug (D-616)** | **RESOLVED D-618** — PR #189 fix/compute-input-hash-multi-input-awk SQUASH-MERGED to develop c000b06f 2026-06-16. CWE-22 guard + awk exit-condition bug + repo-root-relative path resolution on develop. Fix branch deleted. POLICY 18 sound project-wide. |
| **BC-INDEX count reconcile (pre-existing) + O-2 CAP/BC-INDEX drift** | **RESOLVED 2026-06-17 — D-619** | total_bcs 1968→1972 (catalog=disk=1972 verified literal-shell). Per-subsystem Summary reconciled. BC-2.02.013 correctly characterized as legitimately-withdrawn audit-trail BC per POLICY 1 append-only (NOT orphan — prior "orphan" label was incorrect). Counting rule established: all catalog entries including withdrawn count toward total_bcs and Summary rows. Process-gap lesson codified: total_bcs not auto-recounted on BC add/withdraw; feeds S-18.08/S-18.09 gate-story scope. |
| **S-18.08 phantom-field-removal lint gate** | DRAFT-PENDING-AUTHORING 2026-06-14 — D-563 capture | L-F2-phantom-field-gate lesson (D-563): permanent enforcement story. Anchor: E-18 epic, F3 story decomposition. |
| **[process-gap] BC-Precondition registry-block shape validator gate** | OPEN 2026-06-15 — D-576 capture | BC-4.14.001 F-P14-002 class: bare logical name in `plugin=` (missing `name=` + canonical WASM path). Single-instance corrected in-spec; NO automated validator gate detects this class. Deferred to E-18 F3 story decomposition as a candidate validator-gate story (alongside S-18.08-class gate stories). Anchor: E-18 F3. |
| **[process-gap] Cross-reference title/code/phrase sweep gate + title-cite-parity gate** | CODIFIED D-582; UPGRADED D-589 (4th recurrence; class CLOSED) | L-F2-cross-reference-title-code-sweep UPGRADED (D-589 4th recurrence: F-P26-001 BC-7.07.001 VP-085 truncated cite; D-589 ran FIRST exhaustive all-8-BC sweep → class CLOSED). Reactive one-site-at-a-time fixing insufficient (4 recurrences: F-P19-001 BC-4.14.001, F-P19-001-sibling BC-5.41.001, F-P22-001/F-P25-003 ADR, F-P26-001 BC-7.07.001). MECHANICAL GATE NOW MANDATORY: for EVERY `VP-NNN — <title>` cite in any BC §VP Anchors / §Traceability AND ADR §VP Allocations, grep-based check MUST assert `<title>` equals VP file H1 verbatim. Gate runs exhaustively across ALL cite sites every spec-touch. Anchor: S-18.08 consistency-validator MANDATORY scope extension. |
| **[process-gap] Subsystem-anchor-sweep sibling-discipline gate** | CODIFIED 2026-06-15 — D-584 capture | L-F2-subsystem-anchor-sweep codified (2nd recurrence: F-P16-001/F-P21-002 VP sibling; F-P20-001/F-P21-001 Cross-Walk vs Document Map). When a VP/BC subsystem anchor changes OR a capability's Subsystems: line is referenced, fix-burst MUST sweep ALL VPs sharing source-BC AND L2-INDEX Cross-Walk AND Document Map same-burst. Candidate POLICY 5 category (j) + S-18.08 gate scope extension (VP-cluster scope changes trigger Cross-Walk audit). Anchor: E-18 F3 (S-18.08-class gate story or dedicated S-18.NNN). |
| **[process-gap] Canonical-scope-verification discipline** | CODIFIED 2026-06-15 — D-587 capture | L-F2-canonical-scope-verification codified (field-4 provenance ambiguity: 5 passes from D-572/D-573 over-correction to D-587 (B)-reconciliation). When authoring an invariant constraining a field produced by one agent type and consumed by another (shell writes / WASM reads), MUST explicitly name the scope boundary. Field-4 canonical (B) now enshrined: shell MAY exec `git cat-file -t SHA_B`; WASM reads field-4 STATICALLY. S-7.02 defensive sweep applies to invariant-scope-propagation sweeps. Anchor: E-18 F3 gate-story candidate (consistency-validator check for ambiguous no-git-exec constructs in WASM-adjacent prose). |
| **[process-gap] Stale-term-deferral-unsafe discipline** | CODIFIED 2026-06-15 — D-594 FULL BACKLOG CLEARANCE | L-F2-stale-term-deferral-unsafe codified (3-pass deferral cycle: O-P29-001→F-P30-001 LOW→F-P31-001 MED re-escalation; streak RESET 2/3→0/3). RULE: stale terms in normative present-tense prose MUST be fixed in-scope; deferral as LOW is convergence-risk (fresh adversary severity independent). Stale-term sweeps MUST be exhaustive. Full backlog cleared D-594: F-P27-001+002+F-P30-001/002/003+O-P29-001/002/003 all FIXED. Package zero-known-findings for pass-32. Candidate S-18.08 WASM stale-term detector gate (retired-terminology list against normative BC/VP/ADR prose). Anchor: E-18 F3. |
| **F-P27..F-P37 findings (ALL FIXED D-594..D-600)** | **ALL RESOLVED** — F-P27-001/002 (D-594); F-P30/O-P29 class (D-594 FULL-BACKLOG-CLEARANCE); F-P32-001..006 (D-595); F-P33-001 (D-596); F-P34-001/002 (D-597); F-P35-001/002 (D-598); F-P37-001/002/003 (D-600 ADJUDICATED). L-F2-annotation-must-be-self-contained + L-F2-exhaustive-sweep-enumerate-and-count codified D-598/D-599. Full rows: decision-log.md SoT. |
| **[forward-track] F3 VP obligations (3 TBD-VPs as DEFERRED-VP-F3)** | FORWARD-TRACKED — BC-5.41.002 VP: S-18.01; BC-6.24.001 VP: S-18.03; BC-7.07.002 VP: S-18.05 MANDATORY per DI-024. BC-7.07.001 Inv7 log-pruning: S-18.04 AC-N. Anchors: E-18 F3. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-17 — D-621 E-18 STORY PASS-4 INDEX SYNC COMPLETE; STORY-INDEX v4.06; 3-CLEAN streak 0/3; develop HEAD c000b06f (PR #189 MERGED); 4-index: BC-INDEX v3.06/VP-INDEX v2.36/STORY-INDEX v4.06/ARCH-INDEX v2.51; L2-INDEX v1.0.13; POSTURE: E-18 story pass-5 re-verification NEXT — START HERE)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION OR NEW MACHINE**
> Read this section alone to resume. Assumes ZERO prior context. All decisions, directives, and anchors stated explicitly.

### §1. Where We Are

**E-18 CAP-032 context-durability (GitHub issue #173) — Feature Mode, Phase F3 (story decomposition). D-621 2026-06-17. E-18 STORY PASS-4 INDEX SYNC COMPLETE. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN → fix-burst D-621). develop HEAD c000b06f (PR #189 MERGED 2026-06-16). 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51. All 12 stories S-18.00..S-18.10 authored + registered in STORY-INDEX v4.06. Lesson L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008.**

F1-gate APPROVED. F2 spec evolution COMPLETE (D-561). F2 adversarial cascade passes 1-43 COMPLETE (D-562..D-606). D-607..D-613 integration + delta fix + confirming pass + BC-4.15.001 COMPLETE. D-611 F3 decomposition plan APPROVED. D-614 F3 story registration COMPLETE (11 stories). D-615 story pass-1 fix wave integration COMPLETE (S-18.10 added; 4-index synced). D-616 story pass-2 fix wave integration COMPLETE (input-hashes recomputed; SS-08 sweep; VP anchor_story corrected; wave-schedule fixed). D-617 banner repair complete (bats-full-suite CI unblocked). D-618 durability refresh. D-619 BC-INDEX count reconcile (total_bcs 1972; BC-INDEX v3.06). D-620 pass-3 index sync (STORY-INDEX v4.05; VP-INDEX v2.36). D-621 pass-4 index sync (STORY-INDEX v4.06; E-18 epic v1.1; L-F2-ac-pc-parity-sibling-sweep [codified]).

**D-616 E-18 STORY PASS-2 FIX WAVE INTEGRATION summary:**
- compute-input-hash awk+resolver bug FIXED (devops, branch fix/compute-input-hash-multi-input-awk, commits ea6cf1af+5b0d5e5c; **PR #189 SQUASH-MERGED to develop c000b06f 2026-06-16 — DONE**).
- All 12 E-18 input-hashes recomputed: S-18.00=e5bc551; S-18.01=1b4ea21; S-18.02=fd98182; S-18.03=ba7f736; S-18.04a=449dcc4; S-18.04b=026bb4c; S-18.05=df32db5; S-18.06=cf37976; S-18.07=698e6cb; S-18.08=747b3eb; S-18.09=0f747df; S-18.10=aa7d723. Collision resolved: S-18.02/08/09 now distinct.
- BC-6.25.001 input-hash TBD→2d42b26 (POLICY 18 compliance).
- STORY-INDEX SS-08 row sweep: S-18.07→SS-06 (was SS-06+SS-08); S-18.08→SS-06+SS-05 (was SS-05+SS-08); S-18.09→SS-05 (was SS-05+SS-08).
- S-18.10 wave 6→7; W7={S-18.08, S-18.09, S-18.10}; W6=S-18.07 only.
- VP anchor_story swept: VP-082→S-18.04a; VP-085→S-18.04a; VP-084→S-18.04b; VP-090→S-18.04b.
- verification-architecture.md arithmetic invariant total_vps (91)→(92).
- 4-index: BC-INDEX v3.05 (BC-7.07.001 v1.13 + BC-5.41.003 v1.9 cells added); VP-INDEX v2.35 (anchor_story corrections noted); STORY-INDEX v4.04 (all 12 hash rows + SS-08 sweep + wave corrections); ARCH-INDEX v2.51 UNCHANGED; L2-INDEX v1.0.13 (Document Map invariants.md v1.22→v1.25).
- 2 new lessons: L-F2-fix-wave-must-sweep-downstream + L-F2-input-hash-tool-trust.

**4-index at D-616:** BC-INDEX v3.05 (1968 BCs), VP-INDEX v2.35 (92 VPs), STORY-INDEX v4.04 (120 stories; 436+ pts), ARCH-INDEX v2.51. L2-INDEX v1.0.13.

**NEXT ACTION:**
1. **E-18 story pass-5 adversary dispatch + consistency re-verify** — START HERE. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN → D-621 fix-burst). Pass-5 re-verify NEXT. Then story-approval human gate before F4 TDD dispatch.
2. ~~**tool-fix PR merge:** fix/compute-input-hash-multi-input-awk~~ — **DONE D-618: PR #189 MERGED to develop c000b06f.**

**RECURRING LESSONS (carry):** (1) L-F2-stale-term-deferral-unsafe [process-gap] (D-594): stale terms in normative prose MUST be fixed in-scope; deferral as LOW is convergence-risk. (2) L-F2-canonical-scope-verification [process-gap] (D-587): When authoring an invariant for multi-agent-type scope boundary, MUST name the scope explicitly. Field-4 canonical (B) enshrined. (3) L-F2-subsystem-anchor-sweep [process-gap] (D-584; 2nd recurrence): When VP scope/capability Subsystems: line changes, sweep ALL sibling VPs + L2-INDEX Cross-Walk same-burst. (4) L-F2-cross-reference-title-code-sweep [process-gap] (D-582/D-589 UPGRADED; 4th recurrence): For EVERY `VP-NNN — <title>` cite in BC §VP Anchors / ADR §VP Allocations, grep-based check MUST assert title equals VP H1 verbatim. Exhaustive across ALL sites. S-18.08 MANDATORY. (5) L-F2-annotation-must-be-self-contained [process-gap] (D-598): annotation text MUST NOT enumerate sibling BC IDs; structural-parity sweeps MUST VERIFY §Changelog EXISTS before attesting exhaustive coverage. (6) L-F2-exhaustive-sweep-enumerate-and-count [process-gap] (D-599 NEW): cohort sweeps MUST enumerate all N files AND capture per-file grep stdout; subset-scoping FORBIDDEN — false-green generator; enumerate-count gate MANDATORY for ALL cohort structural assertions; S-18.08 MANDATORY scope extension. (7) L-F2-prior-chain-append-only-history [adjudication D-600]: nested [Prior:] chains in last_amended frontmatter are FAITHFUL HISTORICAL RECORDS (POLICY 1 append-only); NOT retroactively rewritten even when the content they reference was factually incorrect at time-of-writing. The LIVE last_amended head is the authoritative state; [Prior:] is immutable archaeological record. Do NOT re-flag [Prior:] residue as a new finding if the live/current text is correct. (8) L-F2-index-quad-cite-reflects-last-bump [convention D-601]: 4-index self-cited quad in any index's changelog reflects the index state AS OF THAT INDEX'S OWN LAST BUMP; an untouched index correctly retains its prior quad — per-pass lag is EXPECTED, NOT a propagation gap; "fixing" it would inject false history; adversary MUST treat quad-lag on untouched indices as benign. (9) L-F2-deferred-table-semantics [process-gap D-606]: consistency/perimeter audit MUST read table HEADING and COLUMN semantics before classifying a missing-file reference as BLOCKER; ARCH-INDEX §Future Sections (Deferred) with "Deferred File | Covered By" columns = intentional documented deferrals whose content lives in the Covered-By file; the correct gate check is whether the Covered-By file exists and covers the domain — NOT whether the deferred file itself exists. (10) L-F2-machine-stable-count-assertion [process-gap D-608]: VP proof harnesses MUST use machine-stable signals (sentinel lines, JSON arrays, exit codes) for count/structure assertions; NOT presentation-coupled regexes like grep-c '^  - '; the correct pattern is a sentinel line INJECTED_FILE_COUNT=<n> written by the injector and asserted by the harness, or JSON array length; feeds F3 S-18.08 gate-story scope. (11) L-F2-fix-at-correct-layer [process-gap D-609]: when a VP fix adds a precondition that depends on an upstream guarantee (BC/ADR clause), the guarantee MUST exist at the cited guarantor BEFORE the VP cites it — do NOT close a VP finding by citing a property the guarantor does not yet make (fix-at-wrong-layer / assert-the-bug-away; POLICY 4/5 mis-anchoring); establish the guarantee at the owning artifact first; root: F-CONF-001 MAJOR VP-090 v1.1 cited non-existent LF clause in BC-7.07.001 v1.11; fix: BC-7.07.001 v1.12 + VP-090 v1.2 + VP-087 v1.2; feeds F3 S-18.08 guarantor-cite verification gate scope.

**Artifact versions at D-613:**
- **ADR-026 v1.20** (D-612: §Decision 12 corrected); BC-1.15.001 v1.5; BC-4.14.001 v1.13; **BC-4.15.001 v1.1** (D-613: proof_method micro-fix; input-hash 0a64afe; D-612 initial v1.0); BC-5.41.001 v1.17; BC-5.41.002 v1.12 (D-607); BC-5.41.003 v1.8; BC-6.24.001 v1.10 (D-607); **BC-7.07.001 v1.12** (D-609 F-CONF-001); BC-7.07.002 v1.12 (D-607); VP-081 v1.7; VP-082 v1.12; VP-083 v1.10; VP-084 v1.9; VP-085 v1.7; VP-086 v1.4; **VP-087 v1.2** (D-609); **VP-088 v1.1** (D-608); **VP-089 v1.0** (D-607); **VP-090 v1.2** (D-609); **VP-091 v1.0** (D-612: validate-heavy-op-delegation; unit-test; SS-04; DI-020; S-18.06); **invariants.md v1.23** (D-612: DI-020 Cited-by += VP-091); capabilities.md v1.7; BC-INDEX **v3.03** (D-613; total_bcs 1967); VP-INDEX **v2.33** (D-612; total_vps 91); ARCH-INDEX **v2.50** (D-612); STORY-INDEX v4.01; L2-INDEX v1.0.12 (D-607). **verification-architecture.md v1.2** (D-612; '90'→'91' prose fixed D-613); **verification-coverage-matrix.md v1.1** (D-612; body §Changelog v1.1 row added D-613).

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

rc.21 FULLY SHIPPED D-560 (2026-06-13). main caf06c68. develop c000b06f (PR #189 MERGED D-618). tag 03054524. Marketplace #13 MERGED. D-605 pass-42 CLEAN; streak 2/3.

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
- **D-621 carry:** E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17. STORY-INDEX v4.05→v4.06: S-18.04b story v1.4 (F-P4-001 MAJOR AC↔PC mis-traces; exhaustive 12-story sweep); S-18.09 story v1.4 + AC↔PC parity gate title suffix (O-P4-004 class fix; AC-008 mandatory gate); E-18 epic v1.0→v1.1. Lesson L-F2-ac-pc-parity-sibling-sweep [process-gap] [codified] — anchor S-18.09 AC-008. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN). VP-INDEX/BC-INDEX/ARCH-INDEX UNCHANGED. Parent-commit: e12a6b35 (D-620 SHA-patch).
- **D-620 carry:** E-18 STORY PASS-3 INDEX SYNC BURST 2026-06-17. STORY-INDEX v4.04→v4.05: S-18.04b subsystems SS-07→SS-04+SS-05+SS-07 (M-001); S-18.09 wave 7→8 (F-SP3-001 BLOCKER); E-18 intro SS-08 removed (M-003; union=SS-01/04/05/06/07); 8-wave DAG. VP-INDEX v2.35→v2.36: 9 wave cells fixed (VP-081 TBD→wave 2/3; VP-082 wave 3→wave 2; VP-083 TBD→wave 3; VP-085 wave 3→wave 2; VP-086 TBD→wave 1; VP-087 TBD→wave 2; VP-088 TBD→wave 4; VP-089 TBD→wave 3; VP-092 wave 6→wave 7 ME-001). Epic count 18→19. 3-CLEAN streak 0/3. Parent-commit: a828686b (D-619 SHA-patch).
- **D-619 carry:** BC-INDEX COUNT RECONCILE BURST 2026-06-17. total_bcs 1968→1972 (catalog=disk=1972; counting rule: all catalog entries including withdrawn per POLICY 1). Summary BC-1 118→117; BC-3 53→56; BC-5 660→655; BC-7 200→201; BC-8 214→222; Total 1966→1972. BC-INDEX v3.05→v3.06. Drift Item D-562 RESOLVED (BC-2.02.013 legit-withdrawn, not orphan). Process-gap lesson codified. Parent-commit: 0bf5cc7a (D-618 SHA-patch).
- **D-618 carry:** STATE.md durability + resume refresh 2026-06-16. Stale-anchor corrections: develop HEAD 7e99f6ef→c000b06f (PR #189 SQUASH-MERGED 2026-06-16; CWE-22 guard + awk fix + path resolution on develop; fix branch deleted; POLICY 18 sound project-wide). factory-artifacts D-618 HEAD (SHA-patch follow-up). §1/§3/§4/§5/§9/§10/§11 fully refreshed. SIZE BUDGET banner D-618 entry dogfooded (L-F2-statemd-banner-wcl-each-burst). Compaction D-430(a). 4-index UNCHANGED: BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51/L2 v1.0.13. Parent-commit: cea9e0a6 (D-617 Commit-E). main HEAD UNCHANGED: caf06c68.
- **D-617 carry:** STATE.md SIZE BUDGET banner block repair 2026-06-16. D-609 entry fixed (410 lines); D-610..D-616 entries appended; L-F2-statemd-banner-wcl-each-burst [process-gap] codified; bats-full-suite CI unblocked. 4-index UNCHANGED.
- **D-616 carry:** E-18 STORY PASS-2 FIX WAVE INTEGRATION 2026-06-16. compute-input-hash awk+resolver bug FIXED (fix/compute-input-hash-multi-input-awk, ea6cf1af+5b0d5e5c; **PR #189 MERGED develop c000b06f D-618**). All 12 E-18 input-hashes recomputed (S-18.00=e5bc551; S-18.01=1b4ea21; S-18.02=fd98182; S-18.03=ba7f736; S-18.04a=449dcc4; S-18.04b=026bb4c; S-18.05=df32db5; S-18.06=cf37976; S-18.07=698e6cb; S-18.08=747b3eb; S-18.09=0f747df; S-18.10=aa7d723; collision S-18.02/08/09 resolved). BC-6.25.001 input-hash 2d42b26. SS-08 sweep: S-18.07→SS-06; S-18.08→SS-06+SS-05; S-18.09→SS-05. S-18.10 W6→W7; W7={S-18.08,S-18.09,S-18.10}. VP anchor_story: VP-082/085→S-18.04a; VP-084/090→S-18.04b. verification-architecture.md (91)→(92). 4-index: BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51/L2 v1.0.13. 2 lessons: L-F2-fix-wave-must-sweep-downstream + L-F2-input-hash-tool-trust. D-chain cite D-615; parent-commit 9d8f2d22.
- **D-615 carry:** E-18 STORY PASS-1 FIX WAVE INTEGRATION 2026-06-16. S-18.10 added (BC-6.25.001; VP-092; SS-06; DI-020; wave 6; 5 pts P1; input-hash aa7d723 per D-616 correction). 12 stories S-18.00..S-18.10. story_count 119→120; 436+ pts. ADR-026 v1.21. invariants.md v1.24. W7={S-18.08,S-18.09,S-18.10}. 2 lessons: L-F2-cross-story-claim-verification + L-F2-story-pc-cite-verbatim. 4-index at D-615: BC v3.04/VP v2.34/STORY v4.03/ARCH v2.51. D-chain cite D-614; parent-commit c317da86.
- **D-609..D-614 archived carries** — D-609: BC-7.07.001 v1.12 + VP-087/090 v1.2 + L-F2-fix-at-correct-layer. D-610: E-18 F2 FULLY COMPLETE + L-F2-no-bypass-on-edit-failure. D-611: F3 plan APPROVED (human gate; 11 stories S-18.00..S-18.09). D-612: BC-4.15.001 v1.0+VP-091+ADR-026 v1.20; S-7.01 Spec-First Gate SATISFIED. D-613: BC-4.15.001 v1.1 + BC-INDEX v3.03. D-614: F3 STORY REGISTRATION (story_count 108→119; STORY-INDEX v4.02). Full: decision-log.md SoT. [D-560..D-611 archived to decision-log.md SoT.]

### §4. Tier-A Completed Log (most recent first)

- **D-621 (2026-06-17):** E-18 STORY PASS-4 INDEX SYNC. STORY-INDEX v4.06 (S-18.04b story v1.4; S-18.09 story v1.4 + AC↔PC parity gate title; E-18 epic v1.0→v1.1). L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN fix-burst). 4-index BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13.
- **D-620 (2026-06-17):** E-18 STORY PASS-3 INDEX SYNC. STORY-INDEX v4.05 (S-18.04b SS corrected; S-18.09 wave 8; E-18 intro SS-08 removed; 8-wave DAG). VP-INDEX v2.36 (9 VP wave cells fixed; VP-092 ME-001 wave 7). Epic count 18→19. 3-CLEAN streak 0/3 (pass-3 NOT-CLEAN fix-burst). 4-index BC v3.06/VP v2.36/STORY v4.05/ARCH v2.51/L2 v1.0.13.
- **D-618 (2026-06-16):** STATE.md durability + resume refresh. develop HEAD corrected 7e99f6ef→c000b06f (PR #189 MERGED SQUASH). §1/§3/§4/§5/§9/§10/§11 refreshed. PR #189 Drift Item RESOLVED. D-618 banner entry appended (L-F2-statemd-banner-wcl-each-burst dogfooded). Compaction D-430(a). 4-index UNCHANGED: BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51/L2 v1.0.13.
- **D-617 (2026-06-16):** STATE.md SIZE BUDGET banner block repair. D-609 banner entry fixed (leading integer 410 added); D-610..D-616 entries appended; L-F2-statemd-banner-wcl-each-burst [process-gap] codified; bats-full-suite CI unblocked. 4-index UNCHANGED.
- **D-616 (2026-06-16):** E-18 STORY PASS-2 FIX WAVE INTEGRATION. compute-input-hash awk+resolver bug FIXED; all 12 E-18 input-hashes recomputed (collision S-18.02/08/09 resolved); BC-6.25.001 input-hash→2d42b26; SS-08 sweep (S-18.07/08/09 subsystems corrected); S-18.10 W6→W7; VP anchor_story swept (VP-082/085→S-18.04a; VP-084/090→S-18.04b); verification-architecture.md (91)→(92). 4-index BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51/L2 v1.0.13. 2 lessons codified. POSTURE: story adversarial 3-CLEAN + consistency NEXT.
- **D-615 (2026-06-16):** E-18 STORY PASS-1 FIX WAVE INTEGRATION. S-18.10 added (BC-6.25.001; VP-092; SS-06; DI-020; wave 6; 5 pts P1). 12 stories S-18.00..S-18.10. story_count 119→120; 436+ pts. ADR-026 v1.21. invariants.md v1.24. 4-index BC v3.04/VP v2.34/STORY v4.03/ARCH v2.51. 2 lessons codified.
- **D-614 (2026-06-16):** F3 STORY REGISTRATION. 11 stories S-18.00..S-18.09 registered STORY-INDEX v4.02; input-hashes computed; BC/VP coverage confirmed; DAG/7-wave confirmed; story_count 108→119; 431+ pts. POSTURE: story adversarial 3-CLEAN + consistency NEXT.
- **D-607..D-615 archived** per D-430(a). D-607..D-610: F2 integration+confirming passes (E-18 F2 FULLY COMPLETE); D-611..D-613: F3 plan+BC-4.15.001+parity; D-614: 11 stories registered (story_count 108→119); D-615: pass-1 fix wave (S-18.10 added; ADR-026 v1.21; invariants.md v1.24; 120 stories; BC v3.04/VP v2.34). Full: decision-log.md SoT.
- **D-562..D-606 archived** per D-430(a). F2 adv passes 1-43. Full: `git show c4ed73bf:.factory/STATE.md` §4.
- **D-531..D-561 archived** per prior compaction. Full: decision-log.md SoT.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-621 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: **D-621 E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17 — STORY-INDEX v4.06; S-18.04b story v1.4 (AC↔PC exhaustive sweep); S-18.09 story v1.4 + AC↔PC parity gate; E-18 epic v1.1. L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN). 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13. POSTURE: E-18 story pass-5 adversary + consistency NEXT.**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + L-issue-169-176-worktree-identity + L-F2-phantom-field-gate + L-F2-sibling-sweep-tree-wide-gate + L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class + L-F2-payload-only-discriminator-recurrence-gate + **L-F2-cross-reference-title-code-sweep (UPGRADED D-589: title-cite-parity gate MANDATORY)** + L-F2-subsystem-anchor-sweep + L-F2-canonical-scope-verification + **L-F2-stale-term-deferral-unsafe (D-594 NEW [process-gap]: stale terms in normative prose MUST be fixed in-scope; deferral as LOW is convergence-risk; sweeps must be exhaustive)** + **L-F2-annotation-must-be-self-contained (D-598 NEW [process-gap]: annotation text MUST NOT enumerate sibling BC IDs; structural-parity sweeps MUST VERIFY §Changelog section EXISTS before claiming exhaustive coverage; candidate S-18.08 gate)** + **L-F2-exhaustive-sweep-enumerate-and-count (D-599 NEW [process-gap]: exhaustive-sweep attestations MUST enumerate all N inputs + per-file grep stdout; subset-scoping FORBIDDEN; FALSE-GREEN generator if not; enumerate-count-gate candidate MANDATORY S-18.08)** + **L-F2-prior-chain-append-only-history (D-600 ADJUDICATION: nested [Prior:] chains in last_amended frontmatter are FAITHFUL HISTORICAL RECORDS per POLICY 1; NOT retroactively rewritten; LIVE head is authoritative; do NOT re-flag [Prior:] residue if live text is correct)** + **L-F2-index-quad-cite-reflects-last-bump (D-601 CONVENTION [convention]: 4-index self-cited quad in any index's changelog reflects the index state AS OF THAT INDEX'S OWN LAST BUMP; untouched-index quad-lag is EXPECTED per-pass behavior, NOT a propagation gap; fixing it would inject false history; adversary MUST treat as benign)** + **L-F2-deferred-table-semantics (D-606 NEW [process-gap]: consistency/perimeter audit MUST read table HEADING and COLUMN semantics before classifying missing-file as BLOCKER; ARCH-INDEX §Future Sections (Deferred) with 'Deferred File | Covered By' columns = intentional deferrals; correct gate = does 'Covered By' file exist and cover the domain?)** + **L-F2-machine-stable-count-assertion (D-608 NEW [process-gap]: VP proof harnesses MUST use machine-stable signals (sentinel lines, JSON arrays, exit codes) for count/structure assertions; NOT presentation-coupled regexes like grep-c '^  - '; root: F-D607-003 VP-088 + F-D607-001 VP-090; canonical fix: sentinel line INJECTED_FILE_COUNT=<n> or JSON array length; feeds F3 S-18.08 gate-story scope)** + **L-F2-fix-at-correct-layer (D-609 NEW [process-gap]: when a VP fix adds a precondition that depends on an upstream guarantee, the guarantee MUST exist at the cited guarantor (BC/ADR) before the VP cites it — do NOT close a VP finding by citing a property the guarantor does not actually make (fix-at-wrong-layer / assert-the-bug-away anti-pattern; POLICY 4/5 mis-anchoring); establish the guarantee at the owning artifact first; root: F-CONF-001 MAJOR VP-090 v1.1 cited non-existent LF clause in BC-7.07.001 v1.11; fix: BC-7.07.001 v1.12 + VP-090 v1.2; companion O-CONF-001 VP-087 §3 mis-attribution corrected via VP-087 v1.2; feeds F3 S-18.08 guarantor-cite verification gate scope)** + **L-F2-no-bypass-on-edit-failure (D-610 NEW [process-gap]: when an Edit tool call fails with 'File has not been read yet' or any other error, the ONLY correct recovery is Read-then-Edit/Write — NEVER fall back to python3/sed/echo heredoc mutation of .factory/ files; the python/sed/echo bypass is TD-FACTORY-HOOK-BYPASS-001 P0 / POL-3 violation: it bypasses the factory-dispatcher PreToolUse/PostToolUse hook chain (validate-state-structure, validate-artifact-path, etc.); incident: D-609 integration burst reflexively used python heredocs after an Edit failed; orchestrator intervened mid-burst; sections re-audited via Read and commit-time hooks validated final file; recovery rule: Read the file/region first, then Edit with a unique old_string; feeds F3 S-18.08 gate-story scope; this rule carries into ALL F3 dispatches)** + **L-F2-cross-story-claim-verification (D-615 NEW [process-gap]: when a story-writer makes cross-story claims ('S-18.04a covers VP-085 via AC-003'), those claims MUST be verified against the ACTUAL ACs of the referenced story; ADR §VP Allocation tables are architectural intent, not story-level verification; cross-story claims that cannot be verified must be amended in the same burst; phantom-VP class)** + **L-F2-story-pc-cite-verbatim (D-615 NEW [process-gap]: every Traceability 'BC-N.NN.NNN PC-M' cite MUST resolve to a real PC heading in the cited BC; phantom PCs not caught by current index integration; prevention is the only gate at authoring time; if PC doesn't exist: cite closest real PC with note, cite Inv instead, or flag to product-owner for BC amendment)** + **L-F2-fix-wave-must-sweep-downstream (D-616 NEW [process-gap]: a fix wave that changes story-file frontmatter MUST sweep ALL downstream references in the SAME burst — STORY-INDEX rows, BC file frontmatter, VP file frontmatter, wave-schedule notes; partial sweeps are incomplete and require a follow-up correction burst; in D-616 the compute-input-hash awk+resolver bug fix required correcting 12 story files + STORY-INDEX + BC-6.25.001 + 4 VP files + wave-schedule note in same burst)** + **L-F2-input-hash-tool-trust (D-616 NEW [process-gap]: POLICY 18 input-hash was a silent no-op for multi-input artifacts — compute-input-hash awk bug hashed only the FIRST listed input file; resolver failed on .factory/-prefixed paths; result: hash collision across distinct-input artifacts (S-18.02/S-18.08/S-18.09 all showing 69dcbd9 = hash of BC-4.14.001.md, the first input); detection signal: identical hash across stories with different input sets; prevention: after any tool-bug fix, all artifacts computed by that tool MUST be recomputed in the same burst; do NOT assume computed hashes are correct without verifying distinctness for distinct-input artifacts)** + **L-F2-statemd-banner-wcl-each-burst (D-617 NEW [process-gap]: every state-manager STATE.md burst MUST append a canonical `N lines (wc-l; ...)` entry to the SIZE BUDGET HTML-comment block with a LEADING integer line-count N matching the wc -l of STATE.md at that burst's Commit-E; the validate-state-structure WASM hook's extract_banner_line_count parses the LAST such entry and compares to the actual newline count — if absent (D-610..D-616 omitted entries) or malformed (D-609 missing leading digit), the hook reports 'no SIZE BUDGET banner found' and bats-full-suite (linux) CI fails for ALL develop PRs; detection: caught by PR #189 CI bats-full-suite run 27649845315; the per-write hook (PostToolUse) did not catch this because it validated state structure at write-time before the final line-count was known; the bats pass-real-state-md-snapshot test is the authoritative integration gate; prevention: at Commit-E of every fix burst, run `wc -l .factory/STATE.md` and append the result as the final banner entry before the closing `-->`)** + **L-F2-ac-pc-parity-sibling-sweep (D-621 NEW [process-gap] [codified]: recurring AC↔PC mis-trace class across E-18 stories (S-18.02 pass-3 / S-18.04a pass-3 / S-18.04b pass-4); instance-scoped fixes caused 3-recurrence; class fix = exhaustive 12-story sweep of ALL `(traces to BC-X PC-N / INV-N)` parentheticals against actual BC §Postconditions/§Invariants sections + mandatory automated gate; gate codified as S-18.09 AC-008 (machine-checkable assertion that every AC↔PC trace resolves to a real numbered clause); anchor S-18.09 AC-008; feeds F4 TDD for S-18.09)** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v3.06 | Bumped D-619. COUNT RECONCILE: total_bcs 1968→1972; Summary BC-1 118→117; BC-3 53→56; BC-5 660→655; BC-7 200→201; BC-8 214→222; Total 1966→1972; D-562 Drift Item RESOLVED. [D-616: v3.05 BC-7.07.001 v1.13 + BC-5.41.003 v1.9 cells; total_bcs UNCHANGED 1968.] |
| VP-INDEX | v2.36 | Bumped D-620. 9 Story Anchors wave cells corrected (anchor_story ground truth): VP-081→wave 2/3; VP-082→wave 2; VP-083→wave 3; VP-085→wave 2; VP-086→wave 1; VP-087→wave 2; VP-088→wave 4; VP-089→wave 3; VP-092→wave 7 (ME-001). total_vps UNCHANGED 92. [D-616: v2.35 VP file anchor_story corrections.] |
| STORY-INDEX | v4.06 | Bumped D-621. S-18.04b story v1.4 (AC↔PC mis-traces corrected; exhaustive sibling sweep); S-18.09 story v1.4 + AC↔PC parity gate title suffix; E-18 epic v1.0→v1.1. story_count UNCHANGED 120. [D-620: v4.05 S-18.04b SS; S-18.09 wave 8; VP-INDEX wave cells; epic count 19.] |
| ARCH-INDEX | v2.51 | UNCHANGED at D-616. (Bumped D-615: ADR-026 v1.20→v1.21 S-18.10 §Deliverables row.) |
| L2-INDEX | v1.0.13 | Bumped D-616. Document Map invariants.md v1.22→v1.25. [D-615: v1.0.12 UNCHANGED.] |

4-index at D-621 (literal-shell): `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "3.06"; `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` → "2.36"; `grep "^version:" .factory/stories/STORY-INDEX.md` → "4.06"; `grep "^version:" .factory/specs/architecture/ARCH-INDEX.md` → "2.51"; `grep "^version:" .factory/specs/domain-spec/L2-INDEX.md` → "1.0.13".

### §9. Critical Anchors

- **factory-artifacts HEAD:** `b26ef433` (D-621 E-18 STORY PASS-4 INDEX SYNC BURST 2026-06-17; prior: `a5aaeea5` D-620 Commit-E / SHA-patch `e12a6b35`)
- **develop HEAD:** `c000b06f` (PR #189 compute-input-hash fix SQUASH-MERGED 2026-06-16; prior: `7e99f6ef` PR #186 fix 2026-06-13)
- **main HEAD:** `caf06c68` (rc.21 bot bundle commit 2026-06-13; UNCHANGED)
- **v1.0.0-rc.21 tag:** `03054524` (SHIPPED; FULLY IN OPERATOR MARKETPLACE)
- **BC-4.15.001 v1.1:** `specs/behavioral-contracts/ss-04/BC-4.15.001.md` (D-613: proof_method micro-fix 'unit-test + integration'→'unit-test'; input-hash 0a64afe; D-612 initial: validate-heavy-op-delegation WASM gate; advisory-only DelegationRecommended; never blocks; pure-parse; dual-channel emission stderr B-1 + plugin.log B-2; first-match semantics; command_preview ≤120-char truncation; non-Bash no-op; CAP-032; S-18.06; draft)
- **VP-091 v1.0:** `specs/verification-properties/VP-091.md` (D-612: validate-heavy-op-delegation behavioral-invariant; unit-test proof; SS-04; DI-020; source_bc BC-4.15.001; anchor S-18.06; 5 properties: always-Continue, dual-channel advisory, no-match no-op, non-Bash no-op, command_preview truncation; draft)
- **BC-6.25.001 v1.0:** `specs/behavioral-contracts/ss-06/BC-6.25.001.md` (D-615: check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE settings.json verification; advisory-only; never blocks; PC1=absent→ADVISORY; PC2=value>80→ADVISORY; PC3=value≤80→PASS; PC4=project-local precedence over global; PC5=never-blocks; SS-06; DI-020; S-18.10; CAP-032; draft)
- **VP-092 v1.0:** `specs/verification-properties/VP-092.md` (D-615: check-state-health reads CLAUDE_AUTOCOMPACT_PCT_OVERRIDE from project-local settings.json (global fallback); emits ADVISORY when absent or value >80; emits PASS when value ≤80; never blocks; always emits check row; behavioral-invariant; unit-test; SS-06; DI-020; source_bc BC-6.25.001; S-18.10; draft)
- **ADR-026 v1.21:** `decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md` (D-615: S-18.10 §Deliverables row added — check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE advisory verification; BC-6.25.001; VP-092; wave 6; DI-020; prior v1.20: D-612 §Decision 12 'no-BC in v1' corrected to 'BC-4.15.001 required')
- **invariants.md v1.24:** `domain-spec/invariants.md` (D-615: DI-020 Cited-by += VP-092 per POLICY 2 bidirectional traceability; prior v1.23: D-612 DI-020 Cited-by += VP-091)
- **BC-4.14.001 v1.13:** `ss-04/BC-4.14.001.md` (D-595 F-P32-001 MED PC7 field-validation contract rewritten — SCALAR empty→malformed; LIST-typed next_wave_stories/open_decisions/pending_fixes/process_gaps empty→VALID; PC7×PC2a contradiction closed; prior v1.12: D-594 F-P27-002)
- **BC-5.41.001 v1.17:** `ss-05/BC-5.41.001.md` (D-598 F-P35-002 MEDIUM: v1.5-skip-marker de-enumerated — removed false 'only BC-4.14.001+BC-7.07.001 changed' claim; self-contained local-fact wording; prior v1.16: D-595 F-P32-003 LOW)
- **BC-5.41.002 v1.12:** `ss-05/BC-5.41.002.md` (D-607 VP-087 wired: integration proof, SS-05, DI-023; DEFERRED-VP resolved to VP-087; prior v1.11: D-602 F-P39-002 LOW status tokens)
- **BC-1.15.001 v1.5:** `ss-01/BC-1.15.001.md` (D-599 F-P36-001 MEDIUM: §Changelog section added — was structurally absent; POLICY 17 body-section type-parity gap; 6 rows reconstructed from modified[] v1.0..v1.5; no skip-marker needed; TRUE-EXHAUSTIVE 8-BC sweep all=1; BC-INDEX cell updated v1.4→v1.5; prior v1.4: D-597 F-P34-001 LOW)
- **BC-5.41.003 v1.8:** `ss-05/BC-5.41.003.md` (D-598 F-P35-001 MEDIUM: §Changelog section added — was structurally absent; type-parity gap POLICY 17; 9 rows transcribed from modified[]; v1.5 real F-P5-002 row, no skip-marker; prior v1.7: D-591 stale-term class flush)
- **BC-6.24.001 v1.10:** `ss-06/BC-6.24.001.md` (D-607 VP-088 wired: integration proof, SS-06, DI-023; DEFERRED-VP resolved to VP-088; prior v1.9: D-598 F-P35-002 MEDIUM v1.5-skip-marker de-enumerated)
- **BC-7.07.001 v1.12:** `ss-07/BC-7.07.001.md` (D-609 F-CONF-001 MAJOR: PC8 + Inv3 step 7 explicit LF newline-termination obligation added — each appended precompact-flush-log entry MUST be terminated by exactly one LF including the final line; additive only; upstream guarantee for VP-090 §0 newline-termination precondition; prior v1.11: D-607 VP-090 wired for Inv7)
- **BC-7.07.002 v1.12:** `ss-07/BC-7.07.002.md` (D-607 VP-089 wired: unit-test proof, SS-07, DI-024; DEFERRED-VP resolved to VP-089; prior v1.11: D-598 F-P35-002 MEDIUM v1.5-skip-marker de-enumerated)
- **VP-083 v1.10:** `verification-properties/VP-083.md` (D-597 F-P34-002 LOW: proof-harness preamble note pinning EPIC-COMPLETE derivation to handoff_content.next_wave_stories==[] not is_first_wave; POLICY-11 tautology-risk closed; prior v1.9: D-595 F-P32-002 MED discriminating fixture)
- **VP-086 v1.4:** `verification-properties/VP-086.md` (D-595 F-P32-004 LOW; UNCHANGED at D-607)
- **VP-087 v1.2:** `verification-properties/VP-087.md` (D-609 O-CONF-001 LOW: §3 attribution corrected — set-complement/unknown-token semantics re-attributed to ADR-026 §Terminal-Wave Discriminator; BC-5.41.002 EC-001b paraphrased accurately; no behavioral change; prior v1.1: D-608 §3 BrokenSprintState Precondition B unenumerated token + PC3b)
- **VP-088 v1.1:** `verification-properties/VP-088.md` (D-608 delta fix F-D607-003 LOW: §2 PC2 count assertion re-specified against machine-stable INJECTED_FILE_COUNT=<n> sentinel line; replaces presentation-coupled grep-c '^  - ' regex; prior v1.0: D-607 F2 gate integration proof)
- **VP-089 v1.0:** `verification-properties/VP-089.md` (D-607 F2 gate: unit-test proof, SS-07, DI-024, BC-7.07.002; postcompact-reanchor.sh best-effort advisory; commit-free; exits 0 on all error paths; UNCHANGED at D-608)
- **VP-090 v1.2:** `verification-properties/VP-090.md` (D-609 F-CONF-001 MAJOR: §0 guarantor cite tightened from 'BC-7.07.001 PC8/Inv3' to 'BC-7.07.001 PC8 newline-termination clause / Inv3 step 7'; citation now resolves to a REAL guarantee following BC-7.07.001 v1.12 additive PC; no behavioral change to precondition semantics; prior v1.1: D-608 §0 newline-terminated-line precondition added)
- **verification-architecture.md v1.1:** `specs/architecture/verification-architecture.md` (D-608 FINDING-1 MINOR: SS-08 removed from subsystems_affected frontmatter; SS-08 has zero VPs in §1 Provable Properties Catalog body; subsystems_affected now: SS-01, SS-02, SS-03, SS-04, SS-05, SS-06, SS-07, SS-09; sibling verification-coverage-matrix.md correctly omits SS-08; prior v1.0: D-607 materialized)
- **VP-085 v1.7:** `verification-properties/VP-085.md` (D-594 F-P31-001 MED stale-term; UNCHANGED at D-597)
- **VP-082 v1.12:** `verification-properties/VP-082.md` (D-588 F-P25-004 LOW; UNCHANGED at D-597)
- **VP-084 v1.9:** `verification-properties/VP-084.md` (D-587 F-P24-003 LOW; UNCHANGED at D-597)
- **VP-081 v1.7:** `verification-properties/VP-081.md` (D-602 F-P39-001 MED: PC-A HandoffMissing mis-attribution corrected — now explicitly SHELL wave-gate (BC-5.41.001 PC9); WASM NEVER emits HandoffMissing (BC-4.14.001 EC-011); exhaustive grep 41 hits VP-081 PC-A sole defective site; prior v1.6: F-P21-002 MED subsystem mis-anchor)
- **L2-INDEX v1.0.13:** `specs/domain-spec/L2-INDEX.md` (D-616: Document Map invariants.md v1.22→v1.25; prior v1.0.12: D-607 Document Map invariants.md v1.21→v1.22)
- **invariants.md v1.23:** `domain-spec/invariants.md` (D-612: DI-020 Cited-by += VP-091 per POLICY 2 bidirectional; prior v1.22 D-607: DI-023/024/025 Cited-by complete through VP-090)
- **VP-081..090 domain_invariants populated:** VP-081 [DI-020,DI-021,DI-023]; VP-082 [DI-021,DI-022,DI-025]; VP-083 [DI-020]; VP-084 [DI-020,DI-025]; VP-085 [DI-021,DI-022,DI-025]; VP-086 [DI-020]; VP-087 [DI-023]; VP-088 [DI-023]; VP-089 [DI-024]; VP-090 [DI-025]
- **capabilities.md v1.7:** `domain-spec/capabilities.md` (UNCHANGED at D-597)
- **ADR-025 v1.6 SHIPPED:** guard at `3b2a378c`; ARCH-INDEX v2.27
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.7 MERGED; E-17 W4 COMPLETE; PR #184 3b2a378c
- **Verify on resume:** `git rev-parse --short origin/develop` → expect `c000b06f`; `git rev-parse --short origin/main` → expect `caf06c68`; `git -C .factory log -1 --format='%h'` → expect `b26ef433` (D-621 Commit-E) or later SHA-patch

### §10. PR Status

- **0 open feature PRs. 0 open release PRs. 0 open marketplace PRs. rc.21 100% COMPLETE. E-18 F2 FULLY COMPLETE (D-610). E-18 F3 STORY REGISTRATION + PASS-1+PASS-2 FIX WAVES COMPLETE (D-614..D-616; STORY-INDEX v4.04; 120 stories). PR #189 compute-input-hash fix SQUASH-MERGED to develop c000b06f 2026-06-16 (D-618; CWE-22+awk+path fix; fix branch deleted; POLICY 18 sound). POSTURE: story adversarial 3-CLEAN cascade + consistency audit NEXT (START HERE), then story-approval human gate.**
- **marketplace PR drbothen/claude-mp #13 MERGED** 2026-06-13 — rc.21 FULLY SHIPPED.
- **RELEASING.md Step 9 VERIFIED (D-560):** operator cache 1.0.0-rc.21 confirmed (plugin.json + 132 entries). rc.21 end-to-end CLOSED.

### §11. Post-CLEAR/Post-RESET Resume Checklist (zero-context; D-618 refresh)

1. **Verify worktree state:** `git rev-parse --short origin/develop` → expect `c000b06f`. `git rev-parse --short origin/main` → expect `caf06c68`. `git -C .factory log -1` (expect D-621 Commit-E HEAD; branch factory-artifacts; clean status).
2. **Read §1-§12 this checkpoint** (all of it; D-620 self-sufficient).
3. **Verify 4-index:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "3.06"; ARCH-INDEX → "2.51"; VP-INDEX → "2.36"; STORY-INDEX → "4.06"; L2-INDEX → "1.0.13".
4. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
5. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
6. **RC.21 100% COMPLETE D-560.** NO remaining release action. Operators: `/plugin update vsdd-factory@claude-mp`.
7. **D-621 E-18 STORY PASS-4 INDEX SYNC COMPLETE 2026-06-17.** STORY-INDEX v4.06: S-18.04b story v1.4 (AC↔PC exhaustive sweep); S-18.09 story v1.4 + AC↔PC parity gate title; E-18 epic v1.1. L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008. 3-CLEAN streak 0/3 (pass-4 NOT-CLEAN fix-burst). **POSTURE: E-18 story pass-5 re-verification NEXT — START HERE.**
8. **4-index at D-621:** BC-INDEX v3.06 (total_bcs 1972; D-619 count-reconcile), VP-INDEX v2.36 (total_vps 92; 9 wave cells corrected D-620), STORY-INDEX v4.06 (120 stories; 436+ pts; 8-wave DAG; S-18.04b v1.4; S-18.09 v1.4), ARCH-INDEX v2.51. L2-INDEX v1.0.13. invariants.md v1.24 (DI-020..025 Cited-by complete through VP-092). BC-7.07.001 v1.12. VP-082/085 anchor_story=S-18.04a (wave 2); VP-084/090 anchor_story=S-18.04b (wave 3). BC-4.15.001 v1.1. VP-091 v1.0. BC-6.25.001 v1.0 (input-hash 2d42b26). VP-092 v1.0. ADR-026 v1.21.
9. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary + O-P8-002 MANDATORY (3rd recurrence) + **L-F2-cross-reference-title-code-sweep [process-gap] UPGRADED (D-589; 4th recurrence; MANDATORY)** + L-F2-subsystem-anchor-sweep [process-gap] + L-F2-canonical-scope-verification [process-gap] (D-587) + **L-F2-stale-term-deferral-unsafe (D-594)** + **L-F2-annotation-must-be-self-contained (D-598)** + **L-F2-exhaustive-sweep-enumerate-and-count (D-599)** + **L-F2-prior-chain-append-only-history (D-600)** + **L-F2-index-quad-cite-reflects-last-bump (D-601)** + **L-F2-deferred-table-semantics (D-606)** + **L-F2-machine-stable-count-assertion (D-608)** + **L-F2-fix-at-correct-layer (D-609)** + **L-F2-no-bypass-on-edit-failure (D-610: MANDATORY ALL F3 dispatches)** + **L-F2-cross-story-claim-verification (D-615)** + **L-F2-story-pc-cite-verbatim (D-615)** + **L-F2-fix-wave-must-sweep-downstream (D-616)** + **L-F2-input-hash-tool-trust (D-616)** + **L-F2-statemd-banner-wcl-each-burst (D-617: every burst MUST append wc-l entry to SIZE BUDGET block before Commit-E push)** + **D-621 E-18 pass-4 index-sync COMPLETE; STORY-INDEX v4.06; L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008; 4-index BC v3.06/VP v2.36/STORY v4.06; story map in §12**.
10. **Latest decision D-621.** E-18 STORY PASS-4 INDEX SYNC COMPLETE. STORY-INDEX v4.06; S-18.04b v1.4; S-18.09 v1.4 + AC↔PC parity gate; E-18 epic v1.1. 3-CLEAN streak 0/3. develop HEAD c000b06f. 4-index: BC v3.06/VP v2.36/STORY v4.06/ARCH v2.51/L2 v1.0.13. **POSTURE: E-18 story pass-5 re-verification NEXT — START HERE.**

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-16 D-618)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| ~~1~~-~~prev~~ | ~~rc.21 through E-18 F2 adv passes 1-15~~ | ~~—~~ | ~~—~~ | **ALL CLOSED — D-560..D-578 2026-06-13/15.** |
| ~~1a~~-~~1j~~ | ~~#173/E-18 F2 adversarial re-cascade (passes 16-25)~~ | ~~feature~~ | ~~—~~ | **ALL DONE D-579..D-588 2026-06-15.** |
| ~~1k~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-26)~~ | ~~feature~~ | ~~D-588 fix burst complete~~ | **DONE D-589 2026-06-15 — NOT-CLEAN (1med/3obs); BC-7.07.001 v1.8; 3-CLEAN streak 0/3.** |
| ~~1l~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-27)~~ | ~~feature~~ | ~~D-589 fix burst complete~~ | **DONE D-590 2026-06-15 — CLEAN; 2 LOWs deferred; streak 0/3→1/3.** |
| ~~1m..1z~~ | ~~#173/E-18 F2 adversarial re-cascade (passes 28-43)~~ | ~~feature~~ | ~~D-590..D-605 complete~~ | **ALL DONE D-591..D-606 2026-06-15.** Passes 28-43: streak resets / clearances / full convergence 3/3 D-606. Full rows: `git show 88993db1:.factory/STATE.md` §12 (D-605 checkpoint). |
| **2a** | **#173/E-18 F2 spec-completion integration burst** | **feature** | D-607 | **DONE 2026-06-16 D-607.** VP-087..090 + verification-architecture.md v1.0 + verification-coverage-matrix.md v1.0 + 4 BCs wired. 4-index BC v3.00/VP v2.30/ARCH v2.48 bumped. D-606 Drift Item RESOLVED. |
| **2a-delta** | **#173/E-18 F2 delta re-validation FIX BURST** | **feature** | D-608 | **DONE 2026-06-16 D-608.** VP-087/088/090 v1.1 + verification-architecture.md v1.1. VP-INDEX v2.31 / ARCH-INDEX v2.49. L-F2-machine-stable-count-assertion [process-gap] codified. E-18 F2 delta-fix COMPLETE. |
| **2a-confirm** | **#173/E-18 CONFIRMING adversary pass (round 1)** | **feature** | D-608 COMPLETE | **DONE 2026-06-16 D-609.** Confirming pass NOT-CLEAN (F-CONF-001 MAJOR + O-CONF-001 LOW). Fix burst: BC-7.07.001 v1.12 + VP-087/090 v1.2. BC-INDEX v3.01 / VP-INDEX v2.32. L-F2-fix-at-correct-layer codified. E-18 F2 delta-fix round-2 COMPLETE. |
| ~~**2a-confirm-2**~~ | ~~**#173/E-18 CONFIRMING adversary pass (round 2)**~~ | ~~feature~~ | ~~D-609 COMPLETE~~ | **DONE 2026-06-16 D-610.** CLEAN — zero BLOCKER/MAJOR/load-bearing-MEDIUM/mis-anchor; 1 cosmetic obs non-actionable. Delta loop DONE. E-18 F2 FULLY COMPLETE. L-F2-no-bypass-on-edit-failure [process-gap] codified. |
| **2b-gate** | **BC-4.15.001 spec addition (S-7.01 Spec-First Gate)** | **feature** | D-613 | **DONE 2026-06-16 D-613.** BC-4.15.001 v1.1 + parity fixes complete. Adversary CLEAN + consistency CLEAN. Gate satisfied. **F3 story authoring NOW UNBLOCKED — START HERE.** |
| **2b** | **#173/E-18 F3 story registration + fix wave** | **feature** | **COMPLETE D-614/D-615/D-616** | Plan APPROVED D-611. 11 stories S-18.00..S-18.09 registered STORY-INDEX v4.02 (D-614). D-615 fix wave: S-18.10 added; STORY-INDEX v4.03. D-616 fix wave: input-hashes recomputed (awk bug fixed); SS-08 sweep; VP anchor_story; S-18.10 W6→W7; STORY-INDEX v4.04; 4-index BC v3.05/VP v2.35/STORY v4.04/ARCH v2.51. Tool-fix PR→develop PENDING. **NEXT: story adversarial 3-CLEAN + consistency → story-approval human gate — START HERE.** |
| **2c** | **#173/E-18 story adversarial 3-CLEAN cascade + consistency + human-approval gate** | **feature** | D-621 COMPLETE (fix-burst) | Pass-3 NOT-CLEAN → D-620 fix-burst. Pass-4 NOT-CLEAN → fix-burst D-621 COMPLETE (STORY-INDEX v4.06; S-18.04b v1.4 exhaustive AC↔PC sweep; S-18.09 v1.4 + AC↔PC parity gate; E-18 epic v1.1; L-F2-ac-pc-parity-sibling-sweep [codified] S-18.09 AC-008). 3-CLEAN streak 0/3. **Pass-5 adversary dispatch + consistency re-verify NEXT — START HERE.** |
| **4** | **#173 wave-checkpoint** | **implementation** | E-18 F3 + story-approval done | State-durability chain stories S-18.01..S-18.05. Blocked on F3 story-approval gate. |
| **5** | **#171 deferred-revalidate** | **implementation** | #173 stories done | Deferred-revalidation story. |
| **6** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin. |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 cascade; rc.19+rc.20+rc.21; E-17 4 stories; S-15.17~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED/SHIPPED** |
| **7c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **8/9** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**
> Previous checkpoint (D-620 E-18-PASS3-INDEX-SYNC-2026-06-17) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
