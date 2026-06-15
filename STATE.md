---
document_type: pipeline-state
level: ops
version: "3.37"
status: draft
producer: state-manager
timestamp: 2026-06-15T16:30:00Z
phase: D-587-F2-E18-PASS-24-NOT-CLEAN-COMPREHENSIVE-CLEANUP-2026-06-15
last_amended: 2026-06-15 (v3.37) — D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B) RECONCILIATION: pass-24 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 MEDIUM (F-P24-001 VP-082 PC-A field-4 provenance ambiguity; F-P24-002 ADR-026 Decision 6 step-order + VP Allocations stale titles), 2 LOW (F-P24-003 VP-084 SS-04/SS-05 split intentional; F-P24-004 VP-INDEX changelog v2.21 DI-rationale stale). Full cleanup burst: ARCH-INDEX v2.42→v2.43 (ADR-026 v1.15 amendment); VP-INDEX v2.23→v2.24 (VP-082 v1.11 field-4 (B); VP-084 v1.9 SS split confirmed; v2.21 DI-rationale prose fixed); BC-INDEX v2.88→v2.89 (BC-5.41.001 v1.13 'side-channel file'→'precompact-flush-log'; BC-7.07.002 v1.8 phantom 'wave' token); L2-INDEX v1.0.8→v1.0.9 (invariants.md v1.20); L-F2-canonical-scope-verification lesson codified [process-gap]. 3-CLEAN streak RESET 2/3→0/3. D-chain cite D-586 per D-419(b); parent-commit 155e133b88cb8e3914c7825dd22bb3b8e974fce3 per D-419(b). [Prior: 2026-06-15 (v3.36) — D-586 F2 E-18 ADV PASS-23 CLEAN — STREAK 2/3. See decision-log.md SoT.]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
"D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP 2026-06-15 — Pass-24 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 MEDIUM (F-P24-001 VP-082 PC-A field-4 provenance ambiguity canonical (B) reconciled; F-P24-002 ADR-026 v1.15 Decision 6 step-order re-sequenced + VP Allocations stale titles corrected), 2 LOW (F-P24-003 VP-084 SS-04/SS-05 split confirmed intentional + justification added; F-P24-004 VP-INDEX changelog v2.21 DI-rationale prose misdescription fixed). 3-CLEAN streak RESET 2/3→0/3 per BC-5.39.001. Comprehensive cleanup: ARCH-INDEX v2.43; VP-INDEX v2.24; BC-INDEX v2.89; L2-INDEX v1.0.9; L-F2-canonical-scope-verification [process-gap] codified; ALL 5 deferred LOWs (F-P22-001/002/003 + F-P23-001/002) + 2 MEDIUM (F-P24-001/002) + 2 LOW (F-P24-003/004) FIXED in this burst. Trajectory →P22 CLEAN(1/3)→P23 CLEAN(2/3)→P24 NOT-CLEAN(2med/2low) RESET→0/3. NEXT: adversary pass-25 (fresh-context; reads updated artifact set after this cleanup). D-chain cite D-586 per D-419(b); parent-commit 155e133b88cb8e3914c7825dd22bb3b8e974fce3 per D-419(b)"
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

  D-430(a) compaction (D-586 burst 2026-06-15):
  Phase Progress D-579..D-583 (5 rows) collapsed to range-reference per D-430(a).
  Decisions Log D-578..D-581 (4 rows) archived to range-reference (decision-log.md SoT) per D-430(a).
  §4 Tier-A Completed Log D-573..D-581 (9 entries) collapsed to range-reference per D-430(a).
  All content preserved via: git show 907ca48e:.factory/STATE.md (pre-compaction D-585 state).

  Line-growth tracker (most recent; older entries archived to git show ef7eafe2:.factory/STATE.md):
  D-532..D-566 tracker entries (35 entries) archived per D-430(a) D-568 burst; preserved at: git show ef7eafe2:.factory/STATE.md lines 40-73.
  D-573..D-585 tracker entries (9 entries) archived per D-430(a) D-586 burst; preserved at: git show 907ca48e:.factory/STATE.md lines 51-61.
  D-586-F2-E18-ADV-PASS-23-CLEAN-STREAK-2-3+COMPACTION-2026-06-15 ~390 lines (wc-l; D-430(a): 5 Phase Progress rows + 4 Decisions rows + 9 §4 entries archived; +1 Phase Progress row + D-586 Decisions Log row + 2 Drift Items rows + §3 D-586 carry + §4 entry + §12 pass-23/24 rows + §11 D-586 refresh; D-446(c) dual-margin: 500-390=110 from hard cap; D-446(c) dual-margin form).
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
| **Last Updated** | 2026-06-15 — D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B) RECONCILIATION: pass-24 NOT-CLEAN (0B/0M/2med/2low). Full cleanup burst fixes ALL 9 deferred findings (5 LOWs from F-P22-001/002/003+F-P23-001/002 + 2 MEDIUM F-P24-001/002 + 2 LOW F-P24-003/004). 4-index bumped: BC-INDEX v2.89 / VP-INDEX v2.24 / ARCH-INDEX v2.43 / STORY-INDEX v4.01 UNCHANGED; L2-INDEX v1.0.9. 3-CLEAN streak RESET 2/3→0/3. L-F2-canonical-scope-verification [process-gap] codified. NEXT: adversary pass-25. |
| **Current Phase** | D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP 2026-06-15 — Pass-24 NOT-CLEAN (0B/0M/2med/2low). 3-CLEAN streak RESET 2/3→0/3. ALL 9 findings fixed (F-P22-001/002/003+F-P23-001/002+F-P24-001/002/003/004). 4-index BC v2.89/VP v2.24/STORY v4.01/ARCH v2.43; L2-INDEX v1.0.9. NEXT: adversary pass-25 (fresh-context; reads updated artifact set). |
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
| **D-584 F2 E-18 ADV PASS-21 NOT-CLEAN FIX BURST + SUBSYSTEM-ANCHOR-SWEEP CODIFICATION 2026-06-15** | **COMPLETE** | Pass-21 NOT-CLEAN: 0B/0M/2med/1low. Fixed: VP-081 v1.5→v1.6 (F-P21-002 MED mis-anchor: scope SS-05,SS-06,SS-07→SS-04,SS-05,SS-07; SS-04 added for WASM gate BC-4.14.001; SS-06 removed unjustified); L2-INDEX v1.0.7→v1.0.8 (F-P21-001 MED: Cross-Walk CAP-032 added SS-01+SS-04; comprehensive full-audit run); VP-085 v1.5→v1.6 (F-P21-003 LOW: Property intro 'wave ID'→'cycle/step context identity'). VP-INDEX v2.22→v2.23. CODIFIED L-F2-subsystem-anchor-sweep [process-gap] (2nd recurrence). BC-INDEX v2.88 UNCHANGED; ARCH-INDEX v2.42 UNCHANGED. 4-index BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. 3-CLEAN streak 0/3. Trajectory →P19(2med/1low)→P20(1med/4low)→P21(2med/1low). |
| **D-585 F2 E-18 ADV PASS-22 CLEAN — STREAK 1/3 2026-06-15** | **COMPLETE** | Pass-22 CLEAN: 0B/0M/0 load-bearing MEDIUM/0 mis-anchor. 3 LOWs (F-P22-001/002/003 non-load-bearing stale-label/prose residue) DEFERRED to pre-F2-gate cleanup burst; package FROZEN. 3-CLEAN streak 0/3→1/3 per BC-5.39.001. 4-index UNCHANGED: BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. Trajectory →P20 NOT-CLEAN(1med/4low)→P21 NOT-CLEAN(2med/1low)→P22 CLEAN(1/3). |
| **D-586 F2 E-18 ADV PASS-23 CLEAN — STREAK 2/3 + COMPACTION 2026-06-15** | **COMPLETE** | Pass-23 CLEAN: 0B/0M/0 load-bearing MEDIUM/0 mis-anchor. Adversary independently re-derived full package on FROZEN D-584 artifact set; declared convergence at asymptotic floor. 2 LOW (F-P23-001 ADR-026 §Decision 6 step-order; F-P23-002 BC-7.07.002 test-vector stale 'wave' token) DEFERRED to pre-F2-gate cleanup burst (5 deferred LOWs total); package FROZEN. 3-CLEAN streak 1/3→2/3. 4-index UNCHANGED: BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. STATE.md compacted per D-430(a). Trajectory →P21 NOT-CLEAN(2med/1low)→P22 CLEAN(1/3)→P23 CLEAN(2/3). NEXT: adversary pass-24 (CLEAN → 3/3 = F2 CONVERGED). |
| **D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B) RECONCILIATION 2026-06-15** | **COMPLETE** | Pass-24 NOT-CLEAN: 0B/0M/2med/2low. Fixed ALL 9 outstanding findings: (F-P24-001 MEDIUM) VP-082 v1.11 PC-A field-4 provenance — canonical (B) reconciled: shell MAY exec `git cat-file -t SHA_B` at write time (or embed literal `commit`); WASM reads field-4 STATICALLY. (F-P24-002 MEDIUM) ADR-026 v1.15: Decision 6 steps re-sequenced (commit→SHA_B→append→push→exit order); VP Allocations table VP-081+VP-083 stale titles corrected. (F-P24-003 LOW) VP-084 v1.9 SS-04/SS-05 split confirmed intentional + in-file justification added. (F-P24-004 LOW) VP-INDEX v2.24 changelog v2.21 DI-rationale prose fixed (DI-021 anti-fabrication, DI-022 hermetic-flush, DI-025 lifecycle-orthogonal). ALSO: ALL 5 deferred LOWs (F-P22-001/002/003 + F-P23-001/002) fixed in this burst: BC-5.41.001 v1.13 ('side-channel file'→'precompact-flush-log'); BC-7.07.002 v1.8 (phantom 'wave'→'cycle'); invariants.md v1.20 (field-4 (B)); BC-7.07.002 v1.8 (phantom wave token). L-F2-canonical-scope-verification [process-gap] codified. 4-index: BC-INDEX v2.89 / VP-INDEX v2.24 / STORY-INDEX v4.01 / ARCH-INDEX v2.43; L2-INDEX v1.0.9. 3-CLEAN streak RESET 2/3→0/3. Trajectory →P22 CLEAN(1/3)→P23 CLEAN(2/3)→P24 NOT-CLEAN(2med/2low). NEXT: adversary pass-25 (fresh-context; reads updated post-cleanup artifact set). |

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
| factory-artifacts | faba6fd0 | D-587 F2 pass-24 NOT-CLEAN comprehensive cleanup + field-4 canonical (B) 2026-06-15 (streak RESET 2/3→0/3; BC-INDEX v2.89/VP-INDEX v2.24/ARCH-INDEX v2.43); prior: 193db2a4 D-586 pass-23 CLEAN STATE.md-only burst + compaction; prior-prior: 907ca48e D-585 pass-22 CLEAN |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE (marketplace PR #13 MERGED); annotated tag object |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; marketplace PR #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-587 2026-06-15; F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP; develop 7e99f6ef; main caf06c68** | rc.21 100% COMPLETE D-560; D-587 PASS-24 NOT-CLEAN (0B/0M/2med/2low; ALL 9 outstanding findings fixed; 3-CLEAN streak RESET 2/3→**0/3**; 4-index BC-INDEX v2.89 / VP-INDEX v2.24 / STORY-INDEX v4.01 / ARCH-INDEX v2.43; L2-INDEX v1.0.9; L-F2-canonical-scope-verification [process-gap] codified; field-4 canonical (B) reconciled); trajectory →P22 CLEAN(1/3)→P23 CLEAN(2/3)→P24 NOT-CLEAN(2med/2low); **Next: adversary pass-25 (fresh-context; reads updated post-cleanup artifact set; CLEAN → streak begins → 3/3 = F2 CONVERGED → F2 human gate → F3 S-18.00..S-18.08).** |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-556: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`
> D-557..D-581 archived to decision-log.md SoT per D-430(a) (D-568 + D-581 + D-586 compaction bursts).
> Latest: D-582..D-586 — in table below.
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md`
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-587 | F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B) RECONCILIATION 2026-06-15 — Pass-24 NOT-CLEAN: 0 BLOCKER, 0 MAJOR, 2 MEDIUM (F-P24-001 VP-082 PC-A field-4 provenance ambiguity; F-P24-002 ADR-026 Decision 6 step-order + VP Allocations stale titles), 2 LOW (F-P24-003 VP-084 SS-04/SS-05 intentional split confirmed; F-P24-004 VP-INDEX changelog DI-rationale prose). 3-CLEAN streak RESET 2/3→0/3. Full cleanup burst resolves ALL 9 deferred findings. Field-4 canonical (B): shell hook MAY exec `git cat-file -t SHA_B` at write time (or embed literal `commit`); WASM reads field-4 STATICALLY from persisted log (never git-exec); v1.16 BLOCKER scope = WASM-ONLY. Artifacts changed: ADR-026 v1.15; invariants.md v1.20; VP-082 v1.11; VP-084 v1.9; BC-5.41.001 v1.13; BC-7.07.002 v1.8; ARCH-INDEX v2.43; VP-INDEX v2.24; BC-INDEX v2.89; L2-INDEX v1.0.9. L-F2-canonical-scope-verification [process-gap] codified. Lesson count: brownfield lessons.md +1. 4-index: BC-INDEX v2.89 / VP-INDEX v2.24 / STORY-INDEX v4.01 / ARCH-INDEX v2.43. D-chain cite D-586 per D-419(b); parent-commit 155e133b88cb8e3914c7825dd22bb3b8e974fce3 per D-419(b). | feature-mode-f2-e18-adv-pass-24-not-clean + comprehensive-cleanup + field-4-canonical-B | 2026-06-15 |
| D-586 | F2 E-18 ADV PASS-23 CLEAN — STREAK 2/3 + STATE.md COMPACTION 2026-06-15 — Fresh-context adversary pass-23 returned CLEAN: 0 BLOCKER, 0 MAJOR, 0 load-bearing MEDIUM, 0 mis-anchor. Adversary independently re-derived full package on the FROZEN D-584 artifact set and declared convergence at asymptotic floor ('could not find any substantive gap, contradiction, or mis-anchor'). 2 LOW findings: F-P23-001 (LOW) ADR-026 §Decision 6 numbered step list (~lines 297-304) logically out of order — `git push` never explicitly enumerated (step 5 references push-failure before step 6 introduces append-before-push); canonical order codified correctly downstream in BC-7.07.001 Inv3 (commit→SHA_B→append→push→exit); SoT clarity defect (owner: architect). F-P23-002 (LOW) BC-7.07.002 Canonical Test Vector (~line 113) stale "sha/wave/step" leftover (should be "sha/cycle/step"); residual phantom-wave token inconsistent with PC1/PC2 (current_cycle/current_step, no wave) (owner: product-owner). Both LOWs DEFERRED to pre-F2-gate cleanup burst (now 5 deferred LOWs total); NOT fixed (package FROZEN per strict 3-CLEAN). 3-CLEAN streak 1/3→2/3 per BC-5.39.001. NO spec changes; 4-index UNCHANGED (BC-INDEX v2.88/VP-INDEX v2.23/STORY-INDEX v4.01/ARCH-INDEX v2.42; L2-INDEX v1.0.8). STATE.md compacted per D-430(a): 5 Phase Progress rows (D-579..D-583) + 4 Decisions Log rows (D-578..D-581) + 9 §4 entries (D-573..D-581) archived to range-references. Trajectory →P21 NOT-CLEAN(2med/1low)→P22 CLEAN(1/3)→P23 CLEAN(2/3). NEXT: adversary pass-24 (fresh-context; SAME frozen package; CLEAN → 3/3 = F2 CONVERGED → pre-gate LOW cleanup → F2 human gate → F3). D-chain cite D-585 per D-419(b); parent-commit 907ca48ee0645e91f0e06c5a16eabcafaace3a2e per D-419(b). | feature-mode-f2-e18-adv-pass-23-clean + streak-2-3 + state-compaction | 2026-06-15 |
| D-585 | F2 E-18 ADV PASS-22 CLEAN — STREAK 1/3 2026-06-15 — Fresh-context adversary pass-22 returned CLEAN: 0 BLOCKER, 0 MAJOR, 0 load-bearing MEDIUM, 0 mis-anchor. 3 LOW (F-P22-001 ADR-026 §VP Allocations stale VP titles; F-P22-002 VP-INDEX changelog v2.21 DI-rationale prose; F-P22-003 BC-5.41.001 'side-channel file' legacy term) DEFERRED to pre-F2-gate cleanup burst; NOT fixed (package FROZEN). 3-CLEAN streak 0/3→1/3 per BC-5.39.001. 4-index UNCHANGED: BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. D-chain cite D-584 per D-419(b); parent-commit f9d77e1926db907eaf993af4b63f8b7957477a18 per D-419(b). | feature-mode-f2-e18-adv-pass-22-clean + streak-1-3 | 2026-06-15 |
| D-584 | F2 E-18 ADV PASS-21 NOT-CLEAN FIX BURST + SUBSYSTEM-ANCHOR-SWEEP CODIFICATION 2026-06-15 — Pass-21 NOT-CLEAN: 0B/0M/2med/1low. Fixed: VP-081 v1.6 (mis-anchor SS-06→SS-04+SS-07); L2-INDEX v1.0.8 (Cross-Walk CAP-032 SS-01+SS-04; full audit); VP-085 v1.6 (wave ID→cycle/step). VP-INDEX v2.23. CODIFIED L-F2-subsystem-anchor-sweep [process-gap] (2nd recurrence). 4-index BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. D-chain cite D-583 per D-419(b); parent-commit 08c623ded36cfbb5af14022c5d401db213506dc8 per D-419(b). | feature-mode-f2-e18-adv-pass-21-fix + subsystem-anchor-sweep-codification | 2026-06-15 |
| D-583 | F2 E-18 ADV PASS-20 NOT-CLEAN FIX BURST 2026-06-15 — Pass-20 NOT-CLEAN: 0B/0M/1med/4low. Fixed: L2-INDEX v1.0.7 (CAP range CAP-001..CAP-028→CAP-001..CAP-032); ADR-026 v1.14 (§Wave-Identity-Derivation cross-note); BC-7.07.002 v1.7 (PC2 phantom wave_id→current_cycle); VP-082 v1.10 (PC-A push named); VP-083 v1.7 (wave-agnostic). BC-INDEX v2.88; VP-INDEX v2.22; ARCH-INDEX v2.42. D-chain cite D-582 per D-419(b); parent-commit a16a5752adbd3cbac153be48ab5bb4c4a2506807 per D-419(b). | feature-mode-f2-e18-adv-pass-20-fix | 2026-06-15 |
| D-582 | F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION 2026-06-15 — Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-4.14.001 v1.10 (§VP Anchors VP-083 stale title + VP-081 qualifier); BC-5.41.001 v1.12 (§VP Anchors VP-081 qualifier swept); ADR-026 v1.13 (§Decision 9 retitled + HandoffMissing re-attr + sweep); VP-081..086 domain_invariants populated. L-F2-cross-reference-title-code-sweep codified [process-gap] (3rd occurrence). BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41. D-chain cite D-581 per D-419(b); parent-commit 6dc1155162b5db57482c88f378e3949ea68a760e per D-419(b). | feature-mode-f2-e18-adv-pass-19-fix + cross-ref-sweep-codification | 2026-06-15 |
| D-578..D-581 archived | **ARCHIVED 2026-06-15 per D-430(a) D-586** | D-578 F2 adv-pass-15 fix; D-579 F2 adv-pass-16 fix; D-580 F2 adv-pass-17 fix; D-581 F2 adv-pass-18 fix + compaction. Full rows: decision-log.md SoT. |
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
| **[process-gap] Subsystem-anchor-sweep sibling-discipline gate** | CODIFIED 2026-06-15 — D-584 capture | L-F2-subsystem-anchor-sweep codified (2nd recurrence: F-P16-001/F-P21-002 VP sibling; F-P20-001/F-P21-001 Cross-Walk vs Document Map). When a VP/BC subsystem anchor changes OR a capability's Subsystems: line is referenced, fix-burst MUST sweep ALL VPs sharing source-BC AND L2-INDEX Cross-Walk AND Document Map same-burst. Candidate POLICY 5 category (j) + S-18.08 gate scope extension (VP-cluster scope changes trigger Cross-Walk audit). Anchor: E-18 F3 (S-18.08-class gate story or dedicated S-18.NNN). |
| **[process-gap] Canonical-scope-verification discipline** | CODIFIED 2026-06-15 — D-587 capture | L-F2-canonical-scope-verification codified (field-4 provenance ambiguity: 5 passes from D-572/D-573 over-correction to D-587 (B)-reconciliation). When authoring an invariant constraining a field produced by one agent type and consumed by another (shell writes / WASM reads), MUST explicitly name the scope boundary. Field-4 canonical (B) now enshrined: shell MAY exec `git cat-file -t SHA_B`; WASM reads field-4 STATICALLY. S-7.02 defensive sweep applies to invariant-scope-propagation sweeps. Anchor: E-18 F3 gate-story candidate (consistency-validator check for ambiguous no-git-exec constructs in WASM-adjacent prose). |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-15 — D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP; streak RESET 0/3; 4-index: BC-INDEX v2.89/VP-INDEX v2.24/STORY-INDEX v4.01/ARCH-INDEX v2.43; L2-INDEX v1.0.9; ALL 9 findings fixed; next: adversary pass-25 → fresh-context reads updated set)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION OR NEW MACHINE**
> Read this section alone to resume. Assumes ZERO prior context. All decisions, directives, and anchors stated explicitly.

### §1. Where We Are

**E-18 CAP-032 context-durability (GitHub issue #173) — Feature Mode, Phase F2 (spec evolution) adversarial 3-CLEAN convergence cascade (BC-5.39.001). D-587 2026-06-15.**

F1-gate APPROVED. F2 spec evolution COMPLETE (D-561). F2 adversarial cascade passes 1-24 COMPLETE (D-562..D-587). Consistency-validator sweep D-575 COMPLETE (CLEAN). Pass-12 CLEAN (1/3 streak); pass-13 NOT-CLEAN (streak RESET 0/3); passes 14-21 all NOT-CLEAN; pass-22 CLEAN (streak 0/3→1/3); pass-23 CLEAN (streak 1/3→2/3); pass-24 NOT-CLEAN (streak RESET 2/3→0/3 — comprehensive cleanup burst applied). STATE.md compacted at D-581 + D-586 per D-430(a).

**D-587 pass-24 NOT-CLEAN summary (comprehensive cleanup burst; ALL 9 findings fixed):**
- Pass-24 NOT-CLEAN: 0B/0M/2 MEDIUM/2 LOW. 3-CLEAN streak RESET 2/3→0/3 per BC-5.39.001.
- F-P24-001 MEDIUM: VP-082 PC-A field-4 provenance reconciled to canonical (B) — shell MAY exec `git cat-file -t SHA_B` at write time (or embed literal `commit`); WASM reads field-4 STATICALLY; v1.16 BLOCKER scope = WASM-ONLY. VP-082 v1.10→v1.11.
- F-P24-002 MEDIUM: ADR-026 Decision 6 step-order re-sequenced (commit→SHA_B→append→push→exit); VP Allocations VP-081 + VP-083 stale titles corrected. ADR-026 v1.14→v1.15; ARCH-INDEX v2.42→v2.43.
- F-P24-003 LOW: VP-084 SS-04/SS-05 split confirmed intentional + in-file justification added. VP-084 v1.8→v1.9.
- F-P24-004 LOW: VP-INDEX changelog v2.21 DI-rationale prose fixed (DI-021 anti-fabrication; DI-022 hermetic-flush; DI-025 lifecycle-orthogonal). VP-INDEX v2.23→v2.24.
- ALL 5 deferred LOWs (F-P22-001/002/003 + F-P23-001/002) ALSO fixed in this burst: BC-5.41.001 v1.13 (F-P22-003); BC-7.07.002 v1.8 (F-P23-002); ADR-026 v1.15 covers F-P22-001 + F-P23-001; VP-INDEX v2.24 covers F-P22-002. BC-INDEX v2.88→v2.89.
- invariants.md v1.19→v1.20 (DI-022 field-4 canonical (B) scoping); L2-INDEX v1.0.8→v1.0.9.
- L-F2-canonical-scope-verification [process-gap] codified in lessons.md.

**3-CLEAN streak: 0/3** (streak RESET at pass-24 NOT-CLEAN). Need 3 consecutive clean passes for F2 convergence.

**Convergence trajectory (last 4 passes per D-433(e)+D-439(c)):** →P21 NOT-CLEAN(2med/1low)→P22 CLEAN(1/3)→P23 CLEAN(2/3)→P24 NOT-CLEAN(2med/2low).

**4-index at D-587:** BC-INDEX v2.89, VP-INDEX v2.24, STORY-INDEX v4.01, ARCH-INDEX v2.43.

**NEXT ACTION (explicit, in order):**
1. **START HERE: adversary pass-25** (fresh-context; reads UPDATED post-cleanup artifact set: ADR-026 v1.15 + invariants.md v1.20 + VP-082 v1.11 + VP-084 v1.9 + BC-5.41.001 v1.13 + BC-7.07.002 v1.8 + all other E-18 artifacts). Need 3 CONSECUTIVE CLEAN passes for F2 convergence (BC-5.39.001). 3-CLEAN streak 0/3.
2. On F2 convergence (3/3) → human gate → Feature Mode F3 story decomposition (story-writer authors S-18.00..S-18.07 + S-18.08 the codified O-P8-002 gate-story).

**RECURRING LESSONS (carry):** (1) L-F2-canonical-scope-verification [process-gap] (D-587): When authoring an invariant constraining a field produced by one agent type and consumed by another, MUST explicitly name scope boundary. Field-4 canonical (B) enshrined. (2) L-F2-subsystem-anchor-sweep [process-gap] (D-584; 2nd recurrence): When any VP scope field or capability Subsystems: line changes, MUST sweep ALL sibling VPs sharing source-BC AND L2-INDEX Cross-Walk rows for affected SS-NN same-burst. (3) L-F2-cross-reference-title-code-sweep [process-gap] (D-582; 3rd recurrence): Every TITLE/ERROR-CODE/canonical PHRASE change MUST grep tree-wide for old value.

**Artifact versions at D-587 (post-cleanup):**
- ADR-026 v1.15; BC-1.15.001 v1.3; BC-4.14.001 v1.10; BC-7.07.001 v1.7; BC-5.41.001 v1.13; BC-5.41.002 v1.6; BC-5.41.003 v1.6; BC-6.24.001 v1.6; BC-7.07.002 v1.8; VP-081 v1.6; VP-082 v1.11; VP-083 v1.7; VP-084 v1.9; VP-085 v1.6; VP-086 v1.2; invariants.md v1.20 (DI-022 field-4 (B) canonical); capabilities.md v1.7 (CAP-032; §CHANGELOG monotonic descending); BC-INDEX v2.89; VP-INDEX v2.24; ARCH-INDEX v2.43; STORY-INDEX v4.01; L2-INDEX v1.0.9.

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
- **D-587 carry:** F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B) RECONCILIATION 2026-06-15. Pass-24 NOT-CLEAN (0B/0M/2med/2low). ALL 9 outstanding findings fixed. 3-CLEAN streak RESET 2/3→0/3. 4-index: BC-INDEX v2.89; VP-INDEX v2.24; STORY-INDEX v4.01; ARCH-INDEX v2.43; L2-INDEX v1.0.9. L-F2-canonical-scope-verification [process-gap] codified. Field-4 canonical (B): shell MAY exec `git cat-file -t SHA_B` at write time; WASM reads STATICALLY. Trajectory →P22 CLEAN(1/3)→P23 CLEAN(2/3)→P24 NOT-CLEAN(2med/2low). NEXT: adversary pass-25 (fresh-context; reads updated post-cleanup artifact set). D-chain cite D-586 per D-419(b). parent-commit 155e133b88cb8e3914c7825dd22bb3b8e974fce3.
- **D-583 carry:** F2 E-18 ADV PASS-20 NOT-CLEAN FIX BURST 2026-06-15. 1med+4low fixed (L2-INDEX v1.0.7 Document Map CAP range corrected; ADR-026 v1.14 wave-identity cross-note; BC-7.07.002 v1.7 PC2 phantom wave_id→current_cycle; VP-082 v1.10 PC-A push-success named step; VP-083 v1.7 wave-agnostic qualifier removed). BC-INDEX v2.88; VP-INDEX v2.22; ARCH-INDEX v2.42; L2-INDEX v1.0.7. 3-CLEAN 0/3 unchanged. 4-index BC v2.88/VP v2.22/STORY v4.01/ARCH v2.42. Trajectory →P18 NOT-CLEAN(1med/1low)→P19 NOT-CLEAN(2med/1low)→P20 NOT-CLEAN(1med/4low). NEXT: adversary pass-21. D-chain cite D-582. parent-commit a16a5752adbd3cbac153be48ab5bb4c4a2506807.
- **D-560 carry:** OPERATOR-INSTALL-VERIFIED 2026-06-13. rc.21 100% COMPLETE. NO remaining release action.
- **D-556 carry:** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-541 carry (partial):** VP IDs TBD per TD-VSDD-063. BC-6.23.001 ACTIVE per POL-14.

### §4. Tier-A Completed Log (most recent first)

- **D-587 (2026-06-15):** F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP + FIELD-4 CANONICAL (B). Pass-24 NOT-CLEAN: 0B/0M/2med/2low. ALL 9 findings fixed: ADR-026 v1.15 (Decision 6 step-order + VP Allocations); VP-082 v1.11 (field-4 (B)); VP-084 v1.9 (SS split confirmed); BC-5.41.001 v1.13 ('side-channel file'→'precompact-flush-log'); BC-7.07.002 v1.8 (phantom 'wave'→'cycle'); invariants.md v1.20 (DI-022 (B)); VP-INDEX v2.24 (DI-rationale prose); L2-INDEX v1.0.9. L-F2-canonical-scope-verification [process-gap] codified. 3-CLEAN streak RESET 2/3→0/3. 4-index BC v2.89/VP v2.24/STORY v4.01/ARCH v2.43. NEXT: adversary pass-25.
- **D-586 (2026-06-15):** F2 E-18 ADV PASS-23 CLEAN — STREAK 2/3 + STATE.md COMPACTION. Pass-23 CLEAN: 0B/0M/0 load-bearing MEDIUM/0 mis-anchor. 2 LOWs (F-P23-001/002) DEFERRED to pre-F2-gate cleanup burst (5 deferred LOWs total); package FROZEN per human directive. 3-CLEAN streak 1/3→2/3 per BC-5.39.001. 4-index UNCHANGED: BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42. STATE.md compacted per D-430(a). NEXT: adversary pass-24 (COMPLETE → D-587).
- **D-585 (2026-06-15):** F2 E-18 ADV PASS-22 CLEAN — STREAK 1/3. Pass-22 CLEAN: 0B/0M/0 load-bearing MEDIUM/0 mis-anchor. 3 LOWs (F-P22-001/002/003) DEFERRED; package FROZEN. 3-CLEAN streak 0/3→1/3. 4-index UNCHANGED: BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42.
- **D-584 (2026-06-15):** F2 E-18 ADV PASS-21 NOT-CLEAN FIX BURST + SUBSYSTEM-ANCHOR-SWEEP CODIFICATION. Pass-21 NOT-CLEAN: 0B/0M/2med/1low. Fixed: VP-081 v1.6 (SS-06→SS-04+SS-07); L2-INDEX v1.0.8 (Cross-Walk full audit); VP-085 v1.6 (wave ID→cycle/step). VP-INDEX v2.23. CODIFIED L-F2-subsystem-anchor-sweep [process-gap] (2nd recurrence). 4-index BC v2.88/VP v2.23/STORY v4.01/ARCH v2.42.
- **D-583 (2026-06-15):** F2 E-18 ADV PASS-20 NOT-CLEAN FIX BURST. Pass-20 NOT-CLEAN: 0B/0M/1med/4low. Fixed: L2-INDEX v1.0.7 (CAP range); ADR-026 v1.14 (wave-identity cross-note); BC-7.07.002 v1.7 (PC2 phantom wave_id); VP-082 v1.10 (PC-A push named); VP-083 v1.7 (wave-agnostic). BC-INDEX v2.88; VP-INDEX v2.22; ARCH-INDEX v2.42.
- **D-582 (2026-06-15):** F2 E-18 ADV PASS-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION. Pass-19 NOT-CLEAN: 0B/0M/2med/1low. Fixed: BC-4.14.001 v1.10 + BC-5.41.001 v1.12 (§VP Anchors stale cites) + ADR-026 v1.13 (§Decision 9 retitle + HandoffMissing re-attr) + VP-081..086 domain_invariants. L-F2-cross-reference-title-code-sweep codified [process-gap] (3rd recurrence). BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41.
- **D-573..D-581 archived** per D-430(a) D-586 compaction. D-573 pass-12 CLEAN (1st streak); D-574 pass-13 NOT-CLEAN (streak reset); D-575 pre-pass-14 sweep; D-576 pass-14 fix; D-577 pass-14 re-sweep; D-578 pass-15 fix; D-579 pass-16 fix; D-580 pass-17 fix; D-581 pass-18 fix + compaction. Full entries: `git show 907ca48e:.factory/STATE.md` §4.
- **D-562..D-572 archived** per D-430(a) D-581 compaction. D-562 pass-1; D-563 pass-2; D-564 pass-3; D-565 pass-4; D-566 pass-5; D-567 pass-6; D-568 pass-7+compaction; D-569 pass-8; D-570 pass-9; D-571 pass-10; D-572 pass-11. Full entries: `git show f95bbd0c:.factory/STATE.md` §4.
- **D-561 (2026-06-14):** F2 E-18 CONTEXT-DURABILITY SPEC EVOLUTION. F2 COMPLETE: ADR-026; VP-081..085; 8 BCs; CAP-032; ARCH-INDEX v2.28; BC-INDEX v2.73.
- **D-560 (2026-06-13):** rc.21 OPERATOR-INSTALL-VERIFIED. Step 9 PASSED. 100% COMPLETE end-to-end.
- **D-559+D-558 (2026-06-13):** rc.21 marketplace MERGED + RELEASED via re-release (PR #186+#188; release.yml all-PASS). main caf06c68; develop 7e99f6ef; tag 03054524.
- **D-556 (2026-06-12):** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-549..D-555 archived** per D-430(a) D-568. D-549 SESSION-END; D-550 REDIRECT; D-551..D-555 ADR-025 v1.6 adversary corrections.
- **D-547 (2026-06-11):** S-17.03 MERGED PR #183. E-17 3/3 COMPLETE. issue #170 CLOSED.
- **D-531 (2026-06-01):** E-10 CASCADE SEALED. D-528 RC.20 SHIPPED. D-508 S-15.03 PRIORITY-A COMPLETE.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-587 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: **D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP 2026-06-15 — Pass-24 NOT-CLEAN (0B/0M/2med/2low); ALL 9 findings fixed; 3-CLEAN streak RESET 2/3→0/3; 4-index BC v2.89/VP v2.24/STORY v4.01/ARCH v2.43; L2-INDEX v1.0.9; L-F2-canonical-scope-verification [process-gap] codified; adversary pass-25 NEXT (fresh-context; reads updated post-cleanup artifact set).**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + L-issue-169-176-worktree-identity + L-F2-phantom-field-gate + L-F2-sibling-sweep-tree-wide-gate + L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class + L-F2-payload-only-discriminator-recurrence-gate + L-F2-cross-reference-title-code-sweep + L-F2-subsystem-anchor-sweep + **L-F2-canonical-scope-verification** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.89 | Bumped D-587 (BC-5.41.001 v1.13 + BC-7.07.002 v1.8 synced; F-P22-003+F-P23-002 fixed). total_bcs 1966 UNCHANGED. |
| VP-INDEX | v2.24 | Bumped D-587 (VP-082 v1.11 field-4 (B); VP-084 v1.9 SS split confirmed; v2.21 DI-rationale prose fixed; F-P24-001/003/004 + F-P22-002 fixed). total_vps 86 UNCHANGED. |
| STORY-INDEX | v4.01 | UNCHANGED at D-561..D-587. E-18 stories S-18.00..S-18.08 NOT YET AUTHORED (F3 next after 3-CLEAN). |
| ARCH-INDEX | v2.43 | Bumped D-587 (ADR-026 v1.15 amendment; F-P24-002 + F-P22-001 fixed). |
| L2-INDEX | v1.0.9 | Bumped D-587 (invariants.md v1.20 citation updated). |

4-index at D-587 (literal-shell): `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "2.89"; `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` → "2.24"; `grep "^version:" .factory/stories/STORY-INDEX.md` → "4.01"; `grep "^version:" .factory/specs/architecture/ARCH-INDEX.md` → "2.43".

### §9. Critical Anchors

- **factory-artifacts HEAD:** `faba6fd0` (D-587 F2 pass-24 NOT-CLEAN comprehensive cleanup 2026-06-15; prior: `193db2a4` D-586 pass-23 CLEAN STATE.md-only burst + compaction; prior-prior: `907ca48e` D-585 pass-22 CLEAN)
- **develop HEAD:** `7e99f6ef` (PR #186 fix + release.yml sync back-merge 2026-06-13)
- **main HEAD:** `caf06c68` (rc.21 bot bundle commit 2026-06-13)
- **v1.0.0-rc.21 tag:** `03054524` (SHIPPED; FULLY IN OPERATOR MARKETPLACE)
- **ADR-026 v1.15:** `decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md` (D-587 F-P24-002 MED: Decision 6 step-order re-sequenced commit→SHA_B→append→push→exit; VP Allocations VP-081/VP-083 stale titles corrected; F-P22-001 MED fixed; prior v1.14: F-P20-002 §Wave-Identity-Derivation cross-note)
- **BC-7.07.002 v1.8:** `ss-07/BC-7.07.002.md` (D-587 F-P23-002 LOW: Canonical Test Vector phantom 'wave'→'cycle'; F-P24-002 companion; prior v1.7: F-P20-003 PC2 phantom wave_id)
- **BC-5.41.001 v1.13:** `ss-05/BC-5.41.001.md` (D-587 F-P22-003 LOW: 'side-channel file'→'precompact-flush-log (append-only)' in Related-BCs + ADR Traceability; prior v1.12: F-P19-001-sibling VP Anchors)
- **VP-082 v1.11:** `verification-properties/VP-082.md` (D-587 F-P24-001 MED: PC-A field-4 provenance reconciled to canonical (B) — shell MAY exec `git cat-file -t SHA_B` at write time; WASM reads field-4 STATICALLY; v1.16 BLOCKER scope WASM-ONLY; prior v1.10: F-P20-004 PC-A push named)
- **VP-084 v1.9:** `verification-properties/VP-084.md` (D-587 F-P24-003 LOW: SS-04/SS-05 split confirmed intentional + in-file justification added; prior v1.8: F-P16-001 scope fix)
- **VP-081 v1.6:** `verification-properties/VP-081.md` (F-P21-002 MED mis-anchor: scope SS-05,SS-06,SS-07→SS-04,SS-05,SS-07; SS-04 added for WASM gate; SS-06 removed unjustified; comprehensive VP-080..086 sweep clean)
- **VP-083 v1.7:** `verification-properties/VP-083.md` (F-P20-005 LOW: Property §2 wave-agnostic; '(wave > 1)' qualifier removed)
- **VP-085 v1.6:** `verification-properties/VP-085.md` (F-P21-003 LOW: Property intro 'wave ID'→'cycle/step context identity')
- **L2-INDEX v1.0.9:** `specs/domain-spec/L2-INDEX.md` (D-587: invariants.md v1.19→v1.20 Document Map updated; prior v1.0.8: F-P21-001 MED Cross-Walk full audit)
- **invariants.md v1.20:** `domain-spec/invariants.md` (D-587: DI-022 field-4 canonical (B) — shell MAY exec git cat-file at write time; WASM reads STATICALLY)
- **BC-4.14.001 v1.10:** `ss-04/BC-4.14.001.md` (F-P19-001 MED: §VP Anchors VP-083 stale title 'and HANDOFF.md Absent'→'or Non-HANDOFF.md Writes' + VP-081 qualifier '(wave_id > 1)' added)
- **VP-081..086 domain_invariants populated:** (F-P19-003 LOW, adjudicated YES) per VP-080 pattern (POLICY 2 bidirectional): VP-081 [DI-020,DI-021,DI-023]; VP-082 [DI-021,DI-022,DI-025]; VP-083 [DI-020]; VP-084 [DI-020,DI-025]; VP-085 [DI-021,DI-022,DI-025]; VP-086 [DI-020]
- **capabilities.md v1.7:** `domain-spec/capabilities.md` (F-P18-O1 LOW: §CHANGELOG reordered monotonic descending; all v1.0–v1.6 rows confirmed present)
- **invariants.md v1.19 (historical):** see v1.20 above (F-P16-005 LOW: DI-022 lock-renewal conditional)
- **ADR-025 v1.6 SHIPPED:** guard at `3b2a378c`; ARCH-INDEX v2.27
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.7 MERGED; E-17 W4 COMPLETE; PR #184 3b2a378c
- **Verify on resume:** `git rev-parse --short origin/develop` → expect `7e99f6ef`; `git rev-parse --short origin/main` → expect `caf06c68`

### §10. PR Status

- **0 open feature PRs. 0 open release PRs. 0 open marketplace PRs. rc.21 100% COMPLETE. E-18 F2 spec evolution in progress (no PR yet — F2 adversarial passes in progress; pass-24 comprehensive cleanup DONE D-587; adversary pass-25 NEXT; F3 follows 3-CLEAN convergence).**
- **marketplace PR drbothen/claude-mp #13 MERGED** 2026-06-13 — rc.21 FULLY SHIPPED.
- **RELEASING.md Step 9 VERIFIED (D-560):** operator cache 1.0.0-rc.21 confirmed (plugin.json + 132 entries). rc.21 end-to-end CLOSED.

### §11. Post-CLEAR/Post-RESET Resume Checklist (zero-context; D-587 refresh)

1. **Verify worktree state:** `git rev-parse --short origin/develop` → expect `7e99f6ef`. `git rev-parse --short origin/main` → expect `caf06c68`. `git -C .factory log -1` (expect D-587 F2 pass-24 NOT-CLEAN comprehensive cleanup 2026-06-15; branch factory-artifacts; clean status).
2. **Read §1-§12 this checkpoint** (all of it; D-587 self-sufficient).
3. **Verify trajectory-tail PC4:** `grep "^current_step:" .factory/STATE.md | grep -oE "→P[0-9]+ [A-Z]+"` → expect trajectory containing P24 NOT-CLEAN.
4. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
5. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
6. **RC.21 100% COMPLETE D-560.** NO remaining release action. Operators: `/plugin update vsdd-factory@claude-mp`.
7. **D-587 F2 E-18 ADV PASS-24 NOT-CLEAN COMPREHENSIVE CLEANUP COMPLETE.** Pass-24 NOT-CLEAN (0B/0M/2med/2low). ALL 9 outstanding findings fixed in this burst. 3-CLEAN streak RESET **2/3→0/3**. 4-index: BC v2.89/VP v2.24/STORY v4.01/ARCH v2.43; L2-INDEX v1.0.9. L-F2-canonical-scope-verification [process-gap] codified. **NEXT: adversary pass-25 (fresh-context; reads updated post-cleanup artifact set; CLEAN→streak begins; 3/3 = F2 CONVERGED → human gate → F3 S-18.00..S-18.08).**
8. **4-index at D-587:** BC-INDEX v2.89, VP-INDEX v2.24, STORY-INDEX v4.01, ARCH-INDEX v2.43. L2-INDEX v1.0.9.
9. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary + O-P8-002 MANDATORY (3rd recurrence) + L-F2-cross-reference-title-code-sweep [process-gap] (3rd recurrence) + L-F2-subsystem-anchor-sweep [process-gap] (2nd recurrence) + **L-F2-canonical-scope-verification [process-gap] (D-587; field-4 (B) canonical; scope-boundary discipline)**.
10. **Latest decision D-587.** F2 pass-24 NOT-CLEAN comprehensive cleanup (ALL 9 findings fixed; 3-CLEAN streak RESET 0/3). Adversary pass-25 NEXT (reads updated artifact set). On 3/3 CLEAN → human gate → F3 S-18.00..S-18.08.

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-15 D-587)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| ~~1~~-~~prev~~ | ~~rc.21 through E-18 F2 adv passes 1-15~~ | ~~—~~ | ~~—~~ | **ALL CLOSED — D-560..D-578 2026-06-13/15.** |
| ~~1a~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-16)~~ | ~~feature~~ | ~~D-578 pass-15 fix complete~~ | **DONE D-579 2026-06-15 — NOT-CLEAN (3med+2low); fixed VP-084 v1.7 + VP-085 v1.4 + VP-083 v1.5 + VP-082 v1.8 + invariants.md v1.19; VP-INDEX v2.20; streak 0/3.** |
| ~~1b~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-17)~~ | ~~feature~~ | ~~D-579 pass-16 fix complete~~ | **DONE D-580 2026-06-15 — NOT-CLEAN (1med+2low); fixed ADR-026 v1.12 + BC-5.41.001 v1.10 (wave-agnostic null rule); BC-INDEX v2.85; ARCH-INDEX v2.40; streak 0/3.** |
| ~~1c~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-18)~~ | ~~feature~~ | ~~D-580 pass-17 fix complete~~ | **DONE D-581 2026-06-15 — NOT-CLEAN (1med+1low); fixed BC-5.41.001 v1.11 (HandoffMissing PC9+EC-014+TV) + capabilities.md v1.7 (CHANGELOG order); BC-INDEX v2.86; L2-INDEX v1.0.6; streak 0/3.** |
| ~~1d~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-19)~~ | ~~feature~~ | ~~D-581 pass-18 fix complete~~ | **DONE D-582 2026-06-15 — NOT-CLEAN (2med+1low); fixed BC-4.14.001 v1.10 + BC-5.41.001 v1.12 (§VP Anchors stale cites) + ADR-026 v1.13 (§Decision 9 retitle + HandoffMissing re-attr) + VP-081..086 domain_invariants; BC-INDEX v2.87; VP-INDEX v2.21; ARCH-INDEX v2.41; L-F2-cross-reference-title-code-sweep codified [process-gap]; streak 0/3.** |
| ~~1e~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-20)~~ | ~~feature~~ | ~~D-582 pass-19 fix complete~~ | **DONE D-583 2026-06-15 — NOT-CLEAN (1med+4low); fixed L2-INDEX v1.0.7 (Document Map CAP range) + ADR-026 v1.14 (wave-identity cross-note) + BC-7.07.002 v1.7 (PC2 phantom wave_id) + VP-082 v1.10 (PC-A push named) + VP-083 v1.7 (wave-agnostic); BC-INDEX v2.88; VP-INDEX v2.22; ARCH-INDEX v2.42; streak 0/3.** |
| ~~1f~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-21)~~ | ~~feature~~ | ~~D-583 pass-20 fix complete~~ | **DONE D-584 2026-06-15 — NOT-CLEAN (2med+1low); fixed VP-081 v1.6 (scope mis-anchor SS-06→SS-04) + L2-INDEX v1.0.8 (Cross-Walk full audit + CAP-032 SS-01+SS-04) + VP-085 v1.6 (wave ID→cycle/step); VP-INDEX v2.23; L-F2-subsystem-anchor-sweep codified [process-gap]; BC-INDEX v2.88 UNCHANGED; streak 0/3.** |
| ~~1g~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-22)~~ | ~~feature~~ | ~~D-584 pass-21 fix complete~~ | **DONE D-585 2026-06-15 — CLEAN (0B/0M/0 load-bearing MEDIUM/0 mis-anchor); 3 LOWs deferred pre-F2-gate cleanup; package FROZEN; streak 0/3→1/3.** |
| ~~1h~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-23)~~ | ~~feature~~ | ~~D-585 pass-22 CLEAN recorded~~ | **DONE D-586 2026-06-15 — CLEAN (0B/0M/0 load-bearing MEDIUM/0 mis-anchor); 2 LOWs (F-P23-001/002) deferred pre-F2-gate cleanup (5 total); package FROZEN; streak 1/3→2/3.** |
| ~~1i~~ | ~~#173/E-18 F2 adversarial re-cascade (pass-24)~~ | ~~feature~~ | ~~D-586 pass-23 CLEAN recorded~~ | **DONE D-587 2026-06-15 — NOT-CLEAN (2med+2low); ALL 9 outstanding findings fixed (F-P22-001/002/003+F-P23-001/002+F-P24-001/002/003/004); field-4 canonical (B) reconciled; 3-CLEAN streak RESET 2/3→0/3; 4-index BC v2.89/VP v2.24/ARCH v2.43; L-F2-canonical-scope-verification codified.** |
| **1** | **#173/E-18 F2 adversarial re-cascade (pass-25)** | **feature** | D-587 comprehensive cleanup complete | Fresh-context adversary reads UPDATED post-cleanup artifact set (ADR-026 v1.15 + VP-082 v1.11 + VP-084 v1.9 + BC-5.41.001 v1.13 + BC-7.07.002 v1.8 + invariants.md v1.20 + all other E-18 artifacts). **3-CLEAN streak 0/3**; need 3 consecutive clean passes for BC-5.39.001 convergence. **START HERE.** |
| **2** | **#173/E-18 F3 story decomposition** | **feature** | F2 3-CLEAN convergence | Author S-18.00..S-18.07+S-18.08 (S-18.08 ships mandatory O-P8-002 pure-parse invariant gate). STORY-INDEX v4.01→v4.02+. |
| **4** | **#173 wave-checkpoint** | **implementation** | E-18 F3 done OR human re-sequence | State-durability chain stories S-18.01..S-18.05. Blocked on F3. |
| **5** | **#171 deferred-revalidate** | **implementation** | #173 stories done | Deferred-revalidation story. |
| **6** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin. |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 cascade; rc.19+rc.20+rc.21; E-17 4 stories; S-15.17~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED/SHIPPED** |
| **7c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **8/9** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-586 F2-E18-ADV-PASS-23-CLEAN-STREAK-2-3-2026-06-15) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
