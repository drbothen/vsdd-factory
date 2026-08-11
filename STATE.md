---
document_type: pipeline-state
level: ops
version: "7.11"
status: draft
producer: state-manager
timestamp: 2026-08-11T17:32:26Z
phase: D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD
last_amended: "2026-08-11 (v7.11) — D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD (state-manager): Option C adjudication; 5 BC amendments (D-972 sentinel replaced); ADR-043 v1.5 proposed NOT RATIFIED; ARCH-INDEX v3.55 (ADR-043 row + SS-01 host/); STORY-INDEX v4.293 (S-21.09 5→10 pts; S-21.14/S-21.15 registered); E-21 epic 98→111 pts 12→14 stories; sprint-state.yaml S-21.09 in-flight; ADR-043 3 DO-NOT-RATIFY passes + S-21.09 LOCAL 3 DO-NOT-RATIFY passes persisted; 7 L-BB lessons; C-1/C-2/C-4/C-5 blocking issues + 6 vacuous gate drift items registered; STATE.md v7.10→v7.11. [Prior: 2026-08-10 (v7.10) — D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS (state-manager): S-21.09 draft→ready 5 pts; STORY-INDEX v4.291→v4.292; 6 factual corrections; refuse_setuid inert HIGH SECURITY finding; ADR-043 DO-NOT-RATIFY review persisted; 4 L-BB lessons; D-971 allocated; trajectory-tail →20→16→8→10; STATE.md v7.09→v7.10. [Prior: 2026-08-10 (v7.09) — SESSION-WRAP-2026-08-10 (state-manager): pipeline ACTIVE→PAUSED; telemetry committed; checkpoint archived + refreshed; STATE.md v7.08→v7.09. [Prior: 2026-08-10 (v7.08) — D-970-ADR-040-V1.12-RATIFICATION-BURST (state-manager): ADR-040 v1.12 RATIFIED by human 2026-08-10 per D-970; policies.yaml v1.4.22→v1.4.23 applied; Codifications 1+2 APPLIED; ARCH-INDEX v3.53→v3.54; 1 L-BB lesson appended; CI-wiring dependency chain recorded; F-001 updated (redesign RATIFIED — not yet in force); D-970 allocated; STATE.md v7.07→v7.08. [Prior: 2026-08-10 (v7.07) — D-969-ADR-040-V1.12-REDESIGN-RECORD-BURST (state-manager): ADR-040 v1.12 AMENDED + REOPENED (proposed); D-965 ratification PROCURED-ON-MISCHARACTERIZATION erratum recorded; F-001 root-cause: category error (empty domain in factory-artifacts); redesign at d2a3176a; ARCH-INDEX v3.52→v3.53; 3 L-BB lessons appended; feature/policy15-gate-rust d2a3176a added; D-969 allocated; STATE.md v7.06→v7.07. [Prior: 2026-08-10 (v7.06) — D-968-PR-774-POST-MERGE-RECORD-BURST (state-manager): PR #774 merged (62fbcf1a); develop 700b4dd3→62fbcf1a; fix/fuel-cap-raise-20m MERGED+DELETED; F-S2107-P10-007 CLOSED; F-S2107-P10-004 SHIFTED; Drift Items updated; L-BB-gate-never-invoked-is-functionally-absent appended; D-968 allocated; STATE.md v7.05→v7.06. [Prior: 2026-08-10 (v7.05) — D-967-PASS-10-CORRECTION-BURST (state-manager): orchestrator relay error in D-966 corrected; F-006 precision note attribution fixed (adapter model, not cross-site); F-S2107-P10-010 MEDIUM added; pass-10 count 9→10; trajectory-tail →20→16→8→10; L-BB-correction-same-verification-obligation appended; D-967 allocated; STATE.md v7.04→v7.05.]]]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD (state-manager; parent-commit: b278d978; D-chain cite D-972 latest brownfield; Option C adjudication; 5 BC amendments (BC-1.05.002/004/028/035/036 D-972 sentinel replaced); ADR-043 v1.5 proposed NOT RATIFIED (3 DO-NOT-RATIFY passes persisted); ARCH-INDEX v3.54→v3.55 (ADR-043 row + SS-01 host/ catalog); STORY-INDEX v4.292→v4.293 (S-21.09 5→10 pts; S-21.14/S-21.15 registered draft); E-21 epic v1.12 98→111 pts 12→14 stories W8; sprint-state.yaml S-21.09 in-flight at 12f280d1; S-21.09 LOCAL 3 DO-NOT-RATIFY passes persisted (streak 0/3); 7 L-BB lessons; C-1/C-2/C-4/C-5 blocking security issues + 6 vacuous gate drift items registered; TD-VSDD-101 dangling ref recorded; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX BC v4.56/VP v2.76/STORY v4.293/ARCH v3.55; policies.yaml v1.4.23 UNCHANGED)."
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
  Soft target: <=415 lines; hard cap: 500 lines (validate-state-md-size hook enforcement).
  D-446(c) dual-margin form: margin from soft-target = 500 - 415 = 85; margin from actual tracked below.
  Historical content belongs in cycle files, NOT here.

  D-430(a) compaction history D-532..D-808 (see decision-log.md for full range) COLLAPSED 2026-07-12. Full per-burst wc-l history archived; SoT: decision-log.md + git show 903aa863:.factory/STATE.md for D-828 pre-compaction state.
  D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20; v6.08→v6.09. Full per-burst wc-l history D-819..D-861 (see decision-log.md for full range) archived; SoT: decision-log.md + git show 9debd920:.factory/STATE.md for D-861 pre-compaction state.
  351 lines (wc-l post-update; D-890 W1-WAVE-GATE-BOOKKEEPING-FIX 2026-07-24; v6.26→v6.27)
  ~255 lines (estimated post-D-943 PASS-28-RECORD-BURST 2026-07-29; v6.73→v6.74)
  ~268 lines (estimated post-D-945 BC-5.39.010-ADR-035-S-21.07-DESIGN-ARC 2026-07-30; v6.75→v6.76)
  ~273 lines (estimated post-D-947 PASS-30-FIX-BURST 2026-07-31; v6.77→v6.78)
  ~290 lines (estimated post-D-949 S-21.07-PASS-1-FIX-BURST-PARTIAL 2026-08-03; v6.79→v6.80)
  ~278 lines (estimated post-D-950 S-21.07-INTEGRATION-CLOSURE 2026-08-03; v6.81→v6.82)
  ~295 lines (estimated post-D-952 ADR-036-HASH-AUTHORITY-MIGRATION 2026-08-03; v6.84)
  ~318 lines (estimated post-D-953 ADR-037-VOLATILE-INPUTS-RULING 2026-08-04; v6.85)
  ~335 lines (estimated post-D-955 S-21.07-PASS-4-RECORD-BURST-INDEX-SYNCS 2026-08-04; v6.87→v6.88)
  ~310 lines (estimated post-D-956 S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS 2026-08-05; v6.88→v6.89)
  314 lines (wc-l post-update; D-957-sha-patch 2026-08-06; v6.91)
  331 lines (wc-l; SESSION-WRAP-2026-08-06 2026-08-06; v6.93→v6.94)
  329 lines (wc-l post-D-960 SHA-patch 2026-08-07; v6.96; Commit E 46b7cef2 pushed)
  338 lines (wc-l post-SESSION-WRAP-2026-08-07 2026-08-07; v6.96→v6.97)
  337 lines (wc-l post-D-961 RECORDING-BURST SHA-patch 2026-08-07; v6.98)
  336 lines (wc-l post-E-22-RETAINED record correction 2026-08-08; v6.98 UNCHANGED)
  ~400 lines (estimated post-D-962 PASS-9-RECORD-BURST + SHA-patch 2026-08-08; v6.98→v6.99)
  ~415 lines (estimated post-D-963 BC-CORRECTION-BURST 2026-08-08; v6.99→v7.00; at soft-target)
  ~415 lines (estimated post-D-963 SHA-patch e4bc6683 2026-08-08; v7.00 UNCHANGED)
  281 lines (wc-l post-D-964 PASS-9-CLOSURE-BURST 2026-08-08; v7.00→v7.01)
  282 lines (wc-l post-D-964 SHA-patch d8334693 2026-08-08; v7.01 UNCHANGED)
  286 lines (wc-l post-SESSION-WRAP-2026-08-09 2026-08-09; v7.01→v7.02)
  287 lines (wc-l post-D-965 POLICY-15-ATTESTATION-GATE-RATIFICATION-BURST 2026-08-09; v7.02→v7.03)
  ~340 lines (estimated post-D-966 PASS-10-RECORD-BURST 2026-08-09; v7.03→v7.04)
  ~315 lines (estimated post-D-967 PASS-10-CORRECTION-BURST 2026-08-10; v7.04→v7.05)
  ~335 lines (estimated post-D-968 PR-774-POST-MERGE-RECORD-BURST 2026-08-10; v7.05→v7.06)
  316 lines (wc-l post-D-968 SHA-patch a055459f 2026-08-10; v7.06 UNCHANGED)
  ~330 lines (estimated post-D-969 ADR-040-V1.12-REDESIGN-RECORD-BURST 2026-08-10; v7.06→v7.07)
  ~350 lines (estimated post-D-970 ADR-040-V1.12-RATIFICATION-BURST 2026-08-10; v7.07→v7.08)
  ~355 lines (estimated post-SESSION-WRAP-2026-08-10 2026-08-10; v7.08→v7.09; pipeline PAUSED)
  ~390 lines (estimated post-D-971 S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS 2026-08-10; v7.09→v7.10)
  ~395 lines (estimated post-D-971 SHA-patch a06fae30 2026-08-11; v7.10 UNCHANGED)
  ~430 lines (estimated post-D-972 SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD 2026-08-11; v7.10→v7.11)
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
| **Last Updated** | 2026-08-11 — D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD: Option C adjudication; 5 BC amendments (D-972 sentinel replaced); ADR-043 v1.5 proposed NOT RATIFIED; ARCH-INDEX v3.55 (ADR-043 row + SS-01 host/); STORY-INDEX v4.293 (S-21.09 5→10 pts; S-21.14/S-21.15 registered draft); E-21 98→111 pts 14 stories; sprint-state.yaml S-21.09 in-flight at 12f280d1; ADR-043+S-21.09 LOCAL 3 DO-NOT-RATIFY passes persisted; 7 L-BB lessons; C-1/C-2/C-4/C-5 blocking issues + 6 vacuous drift items registered; D-972 allocated; trajectory-tail →20→16→8→10 UNCHANGED; STATE.md v7.10→v7.11. [Prior: 2026-08-10 — D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS: S-21.09 draft→ready 5 pts; STORY-INDEX v4.292; 6 factual corrections; refuse_setuid inert HIGH SECURITY; ADR-043 DO-NOT-RATIFY; 4 L-BB lessons; D-971 allocated; trajectory-tail →20→16→8→10; STATE.md v7.09→v7.10. SHA-patch a06fae30 2026-08-11. [Prior: 2026-08-10 — SESSION-WRAP-2026-08-10: pipeline ACTIVE→PAUSED; STATE.md v7.08→v7.09. [Prior: 2026-08-10 — D-970-ADR-040-V1.12-RATIFICATION-BURST: ADR-040 v1.12 RATIFIED; policies.yaml v1.4.23; Codifications 1+2; ARCH-INDEX v3.54; D-970 allocated; STATE.md v7.07→v7.08.]]] |
| **Current Phase** | **D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD (2026-08-11). PIPELINE ACTIVE. D-972 single commit pending push. Option C adjudication complete; 5 BC amendments (BC-1.05.002/004/028/035/036 D-972 sentinel replaced); ADR-043 v1.5 status: proposed NOT RATIFIED (3 DO-NOT-RATIFY passes; two prevented regressions); ARCH-INDEX v3.55 (ADR-043 row + SS-01 host/ catalog extended); STORY-INDEX v4.293 (S-21.09 5→10 pts re-estimated; S-21.14/S-21.15 registered draft W8); E-21 epic v1.12 14 stories 111 pts; sprint-state.yaml S-21.09 in-flight at 12f280d1; S-21.09 LOCAL 3 DO-NOT-RATIFY passes (adv-s21.09-local-pass-1/2/3.md persisted; BC-5.39.001 streak 0/3); 7 L-BB lessons; C-1 (CWE-706), C-2 (CWE-362), C-4 (CWE-284), C-5 (CWE-284) blocking security issues registered; 6 vacuous gate drift items registered; adversary pass-10 NOT-CLEAN B2/H4/M3/L1 (10 findings); streak 0/3; 10 adversary passes; 0 CLEAN verdicts; trajectory 47→18→25→25→24→20→16→8→10 (tail: →20→16→8→10); last D-NNN: D-972. develop 62fbcf1a. main 80e5cd7b. merged_count 107. 4-INDEX BC v4.56/VP v2.76/STORY v4.293/ARCH v3.55. policies.yaml v1.4.23. pass-11 adversary NEXT.** |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B..D-647 ALL COMPLETE/ARCHIVED | **ALL COMPLETE / ARCHIVED** | SoT: decision-log.md + burst-log.md. |
| v1.0.0-rc.20/21/22 SHIPPED | **ALL SHIPPED** | PRs merged; marketplace published. |
| D-856 RC23-SHIPPED 2026-07-18 | **SHIPPED** | GitHub Release v1.0.0-rc.23; marketplace published. |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20 | **COMPLETE** | PIPELINE UNPAUSED — S-21.01..S-21.04 delivered |
| D-941 SESSION-WRAP-PAUSED 2026-07-29: S-21.04 pass-27 CLOSED; streak 0/3; trajectory-tail →6→17→11→7 | **COMPLETE** | PIPELINE PAUSED (session wrap) |
| D-943..D-947 (see decision-log.md for full range) PASSES-28-30 2026-07-29..2026-07-31: passes 28/29/30 record+fix; BC v4.38→v4.43; STORY v4.272→v4.277; ARCH v3.37→v3.40; streak 0/3 | **COMPLETE** | PIPELINE ACTIVE |
| D-948..D-953 (see decision-log.md for full range) SESSION-WRAP + PASSES-1-2 + ADR-036/037 2026-08-03..2026-08-04: S-21.07 pass-1+2; BC v4.43→v4.46; ARCH v3.40→v3.42; STATE.md v6.78→v6.85 | **COMPLETE** | PIPELINE ACTIVE |
| D-954 S-21.07-PASS-3-RECORD-BURST 2026-08-04: NOT-CLEAN B3/H7/M12/L3 (25); REGRESSION 18→25; trajectory-tail →47→18→25→25 | **COMPLETE** | PIPELINE ACTIVE |
| D-955 S-21.07-PASS-4-RECORD-BURST-INDEX-SYNCS 2026-08-04: NOT-CLEAN B4/H9/M9/L3 (25 findings); FLAT; BC v4.47; STORY v4.284 | **COMPLETE** | PIPELINE ACTIVE |
| D-956 S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS 2026-08-05: NOT-CLEAN B3/H8/M10/L3 (24); FLAT-MINUS-ONE; trajectory-tail →18→25→25→24 | **COMPLETE** | PIPELINE ACTIVE |
| D-957 S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS 2026-08-05 (SHA-patch f0f25194): NOT-CLEAN B4/H7/M8/L1 (20); IMPROVING 24→20; trajectory-tail →25→25→24→20 | **COMPLETE** | PIPELINE ACTIVE |
| D-958/D-959 PASS-7-RECORD-BURST + AUTHORSHIP-CORRECTION 2026-08-06: D-958 retracted (Iron Law); correction a0d87706; streak 0/3; BC v4.50; ARCH v3.45 | **COMPLETE** | trajectory-tail →25→25→24→20 |
| SESSION-WRAP-2026-08-06 PIPELINE PAUSED 2026-08-06 | **COMPLETE** | 4-INDEX UNCHANGED BC v4.50/VP v2.75/STORY v4.287/ARCH v3.45 |
| D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07: adversary pass-7 COMPLETE (NOT-CLEAN B2/H5/M7/L2; 16 findings; IMPROVING 20→16); D-960 codified; BC v4.51; STORY v4.288; ARCH v3.46; VP v2.76; trajectory-tail →25→24→20→16; SHA-patch DONE: 46b7cef2. | **COMPLETE** | PIPELINE ACTIVE — D-961 recording burst NEXT |
| SESSION-WRAP-2026-08-07 PIPELINE PAUSED 2026-08-07 (SM 9750700d+SHA-patch) | **COMPLETE** | 4-INDEX UNCHANGED BC v4.51/VP v2.76/STORY v4.288/ARCH v3.46; parent-commit: ada929d4. |
| D-961-RECORDING-BURST 2026-08-07: all pass-8 findings resolved; E-22 DISSOLVED (file RETAINED per human ruling 2026-08-08); ADR-041 sentinel COMPLETE; BC v4.52; STORY v4.289; ARCH v3.47; SHA-patch e2bfec65. | **COMPLETE** | pass-9 adversary dispatch NEXT |
| D-962-PASS-9-RECORD-BURST 2026-08-08: adversary-pass-9.md persisted (NOT-CLEAN B0/H3/M3/L1/NIT1; HALVING 16→8; first zero-BLOCKER; reviewed HEAD 67ffbdcc). D-962 codified. BC v4.53; STORY v4.290; trajectory-tail →24→20→16→8 ADVANCED. SHA-patch c4e1e66d. | **COMPLETE** | D-963 correction NEXT |
| D-963-BC-CORRECTION-BURST 2026-08-08: BC-5.39.010 ~110 FALSIFIED (4 safe/5th exhausts SS-05; early-return row 921); ERRATUM inserted; 1.15-erratum changelog; input-hash 2db1ebe; 2 Drift Items; D-963 codified; 4-INDEX UNCHANGED; STATE.md v6.99→v7.00; SHA-patch e4bc6683 | **COMPLETE** | pass-10 adversary NEXT |
| D-964-PASS-9-CLOSURE-FUEL-REMEDIATION-BURST 2026-08-08: pass-9 ALL 8 FINDINGS CLOSED; ADR-042 v1.2 ratified (10M→20M); ADR-040 v1.1 §Decision 6; S-21.13 registered E-21 W7; D-945 DISCHARGED; ORCHESTRATOR ERROR D-963 corrected; BC v4.55; STORY v4.291; ARCH v3.51; STATE.md v7.00→v7.01 | **COMPLETE** | pass-11 adversary NEXT |
| SESSION-WRAP-2026-08-09 PIPELINE PAUSED 2026-08-09 | **COMPLETE** | 5 branches pushed; checkpoint refreshed; drift item added (log-deletion recurrence); trajectory-tail →24→20→16→8 UNCHANGED; resume: cut release first |
| D-965-POLICY-15-ATTESTATION-GATE-RATIFICATION-BURST 2026-08-09 | **COMPLETE** | policies.yaml v1.4.21→v1.4.22 RATIFIED; ADR-040 v1.1 active; ARCH-INDEX v3.51→v3.52; D-965 allocated; pipeline PAUSED→ACTIVE; STATE.md v7.02→v7.03. |
| D-966-PASS-10-RECORD-BURST 2026-08-09 | **COMPLETE** | adversary-pass-10.md persisted (NOT-CLEAN B2/H4/M2/L1; 9 findings); Blocking Issue P0-F-001 added; 7 Drift Items added; trajectory-tail →20→16→8→9; D-966 allocated; STATE.md v7.03→v7.04. |
| D-967-PASS-10-CORRECTION-BURST 2026-08-10 | **COMPLETE** | orchestrator relay error corrected (F-006 precision note attribution: adapter model, not cross-site); F-S2107-P10-010 MEDIUM added; pass-10 count 9→10 (B2/H4/M3/L1); trajectory-tail →20→16→8→10; L-BB lesson appended; D-967 allocated; STATE.md v7.04→v7.05. |
| D-968-PR-774-POST-MERGE-RECORD-BURST 2026-08-10 | **COMPLETE** | PR #774 merged (62fbcf1a); develop 700b4dd3→62fbcf1a; fix/fuel-cap-raise-20m MERGED+DELETED; F-007 CLOSED; F-004 SHIFTED (ambiguous referent); L-BB-gate-never-invoked lesson appended; D-968 allocated; STATE.md v7.05→v7.06. pass-11 adversary NEXT. |
| D-969-ADR-040-V1.12-REDESIGN-RECORD-BURST 2026-08-10 | **COMPLETE** | ADR-040 v1.12 AMENDED + REOPENED (proposed); F-001 root-cause: category error (empty domain in factory-artifacts); redesign at feature/policy15-gate-rust d2a3176a; ARCH-INDEX v3.52→v3.53; 3 L-BB lessons appended; codifications 1+2 PROPOSED (NOT applied to policies.yaml); D-969 allocated; STATE.md v7.06→v7.07. pass-11 adversary NEXT. |
| D-970-ADR-040-V1.12-RATIFICATION-BURST 2026-08-10 | **COMPLETE** | ADR-040 v1.12 RATIFIED by human 2026-08-10 (D-970); policies.yaml v1.4.22→v1.4.23 applied; Codifications 1+2 APPLIED to POLICY 15 + POLICY 22; ARCH-INDEX v3.53→v3.54; 1 L-BB lesson appended; D-970 allocated; F-001 redesign RATIFIED — not yet in force (CI wiring pending; dependency chain: S-21.09→S-21.07→wire CI job); STATE.md v7.07→v7.08. pass-11 adversary NEXT. |
| SESSION-WRAP-2026-08-10 PIPELINE PAUSED 2026-08-10 (human /wrap directive) | **COMPLETE** | telemetry committed; checkpoint archived + refreshed; D-970 ratification complete; F-001 redesign ratified not-yet-in-force; trajectory-tail →20→16→8→10 UNCHANGED; STATE.md v7.08→v7.09. |
| D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS 2026-08-10 (single commit a06fae30; SHA-patch done) | **COMPLETE** | S-21.09 draft→ready 5 pts; STORY-INDEX v4.291→v4.292; sprint-state.yaml updated; 6 factual corrections applied; refuse_setuid inert HIGH SECURITY registered; ADR-043 v1.0 DO-NOT-RATIFY (ADR-scoped, not pass-11); 4 L-BB lessons codified; D-971 allocated; STATE.md v7.09→v7.10. |
| D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD 2026-08-11 (single commit pending push; parent b278d978) | **COMPLETE** | Option C adjudication; 5 BC amendments (D-972 sentinel replaced); ADR-043 v1.5 proposed NOT RATIFIED (3 DO-NOT-RATIFY passes persisted); ARCH-INDEX v3.55 (ADR-043 row + SS-01 host/ catalog); STORY-INDEX v4.293 (S-21.09 5→10 pts; S-21.14/S-21.15 draft W8 111 pts); sprint-state.yaml S-21.09 in-flight at 12f280d1; S-21.09 LOCAL 3 DO-NOT-RATIFY passes (adv-s21.09-local-pass-1/2/3.md; streak 0/3); 7 L-BB lessons; C-1/C-2/C-4/C-5 blocking issues + 6 vacuous gate drift items; D-972 allocated; STATE.md v7.10→v7.11. |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-957 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-807..D-957 (see decision-log.md for full range) (archived) | state-manager | ARCHIVED | See `cycles/v1.0-brownfield-backfill/burst-log.md`. |
| D-960 S-21.07-PASS-8 2026-08-07 (SHA-patch 46b7cef2) | state-manager | COMPLETE | NOT-CLEAN B2/H5/M7/L2 (16+9); IMPROVING 20→16; BC v4.51; VP v2.76; STORY v4.288; ARCH v3.46; trajectory-tail →25→24→20→16. |
| D-961-RECORDING-BURST 2026-08-07 (SHA-patch e2bfec65) | state-manager | COMPLETE | All pass-8 findings resolved; ADR-041 sentinel COMPLETE; E-22 dissolved (file RETAINED); BC v4.52; STORY v4.289; ARCH v3.47; trajectory-tail UNCHANGED. |
| D-962-PASS-9-RECORD-BURST 2026-08-08 (SHA-patch c4e1e66d) | state-manager | COMPLETE | adversary-pass-9.md persisted (NOT-CLEAN B0/H3/M3/L1/NIT1; HALVING 16→8); BC v4.53; STORY v4.290; trajectory-tail →24→20→16→8; P9-003 OPEN. |
| D-963-BC-CORRECTION-BURST 2026-08-08 (SHA-patch e4bc6683) | state-manager | COMPLETE | BC-5.39.010 ~110 FALSIFIED; ERRATUM inserted; 2 Drift Items; D-963 codified; 4-INDEX UNCHANGED; STATE.md v7.00. |
| D-964-PASS-9-CLOSURE-FUEL-REMEDIATION-BURST 2026-08-08 (SHA-patch d8334693) | state-manager | COMPLETE | pass-9 ALL 8 FINDINGS CLOSED; ADR-042 v1.2 ratified; S-21.13 registered; D-945 DISCHARGED; ORCH ERROR corrected; BC v4.55; STORY v4.291; ARCH v3.51; STATE.md v7.01. |
| SESSION-WRAP-2026-08-09 (session-wrap burst; human /wrap directive) | state-manager | COMPLETE | 5 branches pushed; telemetry committed; checkpoint refreshed; drift item added (log-deletion recurrence); trajectory-tail →24→20→16→8 UNCHANGED; STATE.md v7.01→v7.02. SHA-patch 2fc890e4. |
| D-965-POLICY-15-ATTESTATION-GATE-RATIFICATION-BURST 2026-08-09 (SHA-patch 7540c669) | state-manager | COMPLETE | policies.yaml v1.4.21→v1.4.22 RATIFIED; ADR-040 v1.1 active; ARCH-INDEX v3.51→v3.52; D-965 allocated; pipeline PAUSED→ACTIVE; STATE.md v7.02→v7.03. |
| D-966-PASS-10-RECORD-BURST 2026-08-09 (single commit TD-VSDD-053; parent cbff0801) | state-manager | COMPLETE | adversary-pass-10.md created; INDEX.md pass-10 row + Convergence Status updated; decision-log D-966 appended; burst-log D-966 8-block appended; lessons.md 2 PROPOSED lessons; STATE.md v7.03→v7.04; streak 0/3 RESET; trajectory-tail →20→16→8→9. |
| D-967-PASS-10-CORRECTION-BURST 2026-08-10 (single commit TD-VSDD-053; parent 38cd1037) | state-manager | COMPLETE | adversary-pass-10.md F-006 corrected + F-010 added; INDEX.md pass-10 count 9→10; decision-log D-967 appended; burst-log D-967 8-block appended; lessons.md L-BB-correction lesson appended; STATE.md v7.04→v7.05; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10. |
| D-968-PR-774-POST-MERGE-RECORD-BURST 2026-08-10 (single commit TD-VSDD-053; parent 48cb6862; SHA-patch a055459f) | state-manager | COMPLETE | PR #774 merged (62fbcf1a); develop advanced; fix/fuel-cap-raise-20m removed; F-007 CLOSED; F-004 SHIFTED; Drift Items updated; L-BB-gate-never-invoked lesson appended; STATE.md v7.05→v7.06; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |
| D-969-ADR-040-V1.12-REDESIGN-RECORD-BURST 2026-08-10 (single commit TD-VSDD-053; parent 70b4ecd1) | state-manager | COMPLETE | ADR-040 v1.12 AMENDED + REOPENED; F-001 root-cause recorded; redesign at d2a3176a; ARCH-INDEX v3.52→v3.53; 3 L-BB lessons appended; D-969 allocated; codifications 1+2 PROPOSED; STATE.md v7.06→v7.07; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |
| D-970-ADR-040-V1.12-RATIFICATION-BURST 2026-08-10 (single commit TD-VSDD-053; parent 13d7ef6b) | state-manager | COMPLETE | ADR-040 v1.12 RATIFIED by human 2026-08-10; policies.yaml v1.4.22→v1.4.23 applied; Codifications 1+2 APPLIED; ARCH-INDEX v3.53→v3.54; 1 L-BB lesson appended; D-970 allocated; F-001 redesign RATIFIED — not yet in force; STATE.md v7.07→v7.08; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |
| SESSION-WRAP-2026-08-10 (session-wrap burst; human /wrap directive; parent-commit: 43337a44) | state-manager | COMPLETE | telemetry committed; checkpoint archived + refreshed; pipeline ACTIVE→PAUSED; D-chain cite D-970; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; STATE.md v7.08→v7.09. |
| D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS 2026-08-10 (single commit TD-VSDD-053; parent 3197b79a; SHA-patch a06fae30) | state-manager | COMPLETE | S-21.09 draft→ready 5 pts; STORY-INDEX v4.291→v4.292; sprint-state.yaml updated; 6 factual corrections; refuse_setuid inert HIGH SECURITY registered; ADR-043 DO-NOT-RATIFY (ADR-scoped); 4 L-BB lessons; D-971 allocated; STATE.md v7.09→v7.10; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |
| D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD 2026-08-11 (single commit TD-VSDD-053; parent b278d978) | state-manager | COMPLETE | Option C adjudication; 5 BC amendments; ADR-043 v1.5 proposed NOT RATIFIED; ARCH-INDEX v3.55; STORY-INDEX v4.293; E-21 v1.12 14 stories 111 pts; S-21.09 in-flight 12f280d1; LOCAL streak 0/3; 7 L-BB lessons; C-1/C-2/C-4/C-5 + 6 vacuous drift items; D-972 allocated; STATE.md v7.10→v7.11; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.56 D-972) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX v4.293 D-972) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-043 NEW D-972 proposed NOT RATIFIED; ADR-042 NEW D-964; ADR-041 NEW D-961) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | 107 (STATE.md explicit counter; sprint-state predicate: 113; canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (107):** S-19.07 MERGED PR #670 6db4c9fc (E-19 COMPLETE 9/9). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (1):** S-21.09 (feature/S-21.09 @ 12f280d1; 10 pts re-estimated; BC-5.39.001 LOCAL streak 0/3; 3 DO-NOT-RATIFY passes; adv-s21.09-local-pass-1/2/3.md; C-1/C-2/C-4/C-5 blocking issues open)
- **E-21:** S-21.07 (W4; pass-10 NOT-CLEAN 10 findings D-967; BC-5.39.010 v1.17; branch 5370db80 pushed; NO REBASE; S-21.09 MUST land first); S-21.09 (in-flight; 10 pts; feature/S-21.09 @ 12f280d1; MUST land before S-21.07); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft; stub pending story-writer); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft; stub pending story-writer).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | 62fbcf1a | PR #774 squash-merged 2026-08-10T17:34:37Z. Pull on next code-worktree resume. |
| factory-artifacts | a06fae30 | D-971 Commit E — single commit a06fae30 pushed. D-972 single commit staged, not yet pushed. |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. |
| feature/S-21.09 | 12f280d1 | S-21.09 in-flight (10 pts; LOCAL streak 0/3; 3 DO-NOT-RATIFY passes). Not pushed. No PR. |
| feature/S-21.07 | 5370db80 | pass-10 NOT-CLEAN 10 findings D-967 (correction burst complete). Pushed; SHA-equal with origin. FROZEN per team-lead; NO REBASE; MERGE-ORDER: S-21.09 first. |
| feature/S-21.04 | 323f440f | pass-31 pending; no PR. Pushed; SHA-equal with origin. |
| fix/nested-factory-path-derivation | 9afc3226 | F-S2107-P8-016 + P9-008 CLOSED. Pushed; SHA-equal with origin. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel. Pushed; SHA-equal with origin. |
| fix/fuel-exhaustion-fail-loud | fbb9dcb6 | ABANDONED — orchestrator dispatch error (87 files duplicating unmerged S-21.07). Local-only; deliberately NOT pushed. |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-972 COMPLETE 2026-08-11 (single commit pending push; parent b278d978). develop 62fbcf1a; main 80e5cd7b; merged_count 107; BC v4.56; VP v2.76; STORY v4.293; ARCH v3.55; ADR-043 proposed NOT RATIFIED; streak 0/3 (10 passes); trajectory-tail →20→16→8→10 UNCHANGED. S-21.09 in-flight at 12f280d1 (10 pts; LOCAL streak 0/3; 3 DO-NOT-RATIFY). F-001 redesign RATIFIED (ADR-040 v1.12; policies.yaml v1.4.23 ACTIVE; CI wiring PENDING: S-21.09→S-21.07→wire CI job). pass-11 adversary NEXT. | D-972 2026-08-11; D-971 2026-08-10; D-970 2026-08-10; D-969 2026-08-10. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-972 (see decision-log.md for full range): this Decisions Log (D-958..D-972 (see decision-log.md for full range) live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-972 | D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD (state-manager; single-commit TD-VSDD-053 2026-08-11; parent b278d978). (a) POLICY 16 GATE PASS — D-972 allocated; parent-commit b278d978; (b) Option C security-model adjudication complete; 5 BC amendments (BC-1.05.002/004/028/035/036 D-{TBD-option-c-security-model-adjudication} sentinel replaced with D-972); POLICY 7 titles corrected on BC-1.05.028 + BC-1.05.035 (stale canonicalize framing); (c) ADR-043 v1.5 status: proposed; NOT RATIFIED; 3-pass DO-NOT-RATIFY trajectory (adv-adr-043-pass-1/2/3.md persisted; NOT added to INDEX.md); two prevented regressions: (1) pass-1 B-3 inverted security rationale; (2) pass-3 B-1 cfg(windows) missing fallback; (d) ARCH-INDEX v3.54→v3.55: ADR-043 row added (total_adrs 42→43); SS-01 host/ catalog extended with host/{mod,exec_subprocess,read_file,memory,path_util}.rs; (e) S-21.09 delivery record: re-estimated 5→10 pts (correct scope); STORY-INDEX v4.292→v4.293; sprint-state.yaml S-21.09 in-flight at 12f280d1; BC-5.39.001 LOCAL 3 DO-NOT-RATIFY passes persisted (adv-s21.09-local-pass-1/2/3.md); streak 0/3; (f) S-21.14 + S-21.15 registered draft W8 in STORY-INDEX + E-21 epic; E-21 v1.12 14 stories 111 pts; (g) 7 L-BB lessons appended (D-972): subagent-idle-without-report-recurrence; context-loss-reverts-to-false-premise; false-spec-claim-propagation; controls-that-reimplement-gate; recorded-magnitude-drift-pts; adversarial-review-earns-cost-in-prevented-regressions; estimate-volatility-as-scope-signal; (h) C-1 (CWE-706 incorrect path resolution), C-2 (CWE-362 TOCTOU), C-4 (CWE-284 access control — prefix list misconfiguration), C-5 (CWE-284 no per-entry resource limit isolation) registered as blocking security issues; (i) 6 vacuous/inert gate drift items registered (refusal_path_not_found inert, platform_block_inert, wildcard_bypass_inert, time_of_check_time_of_use_gap, prefix_list_empty_fallthrough, resource_limit_unenforced); TD-VSDD-101 dangling reference (VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1) recorded; (j) fuel exhaustion trace 65cbf37e-99d4-44c2-84bd-432615f895cd documented (rc.23 misreports fuel exhaustion as "plugin timed out"); (k) streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX BC v4.56/VP v2.76/STORY v4.293/ARCH v3.55; policies.yaml v1.4.23 UNCHANGED; STATE.md v7.10→v7.11. | Option C adjudication; 5 BC amendments; ADR-043 proposed NOT RATIFIED; ARCH-INDEX v3.55; STORY-INDEX v4.293 (S-21.09 10 pts; S-21.14/S-21.15 draft W8 111 pts); C-1/C-2/C-4/C-5 blocking; 7 L-BB lessons | D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION | 2026-08-11 |
| D-971 | D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS (state-manager; single-commit TD-VSDD-053 2026-08-10; commit a06fae30; SHA-patch done 2026-08-11). (a) POLICY 16 GATE PASS — D-971 allocated; parent-commit 3197b79a; (b) S-21.09 promoted draft→ready 5 pts (re-estimated 8→5 pts); STORY-INDEX v4.291→v4.292; sprint-state.yaml S-21.09 status draft→ready; (c) 6 factual corrections: (i) validate-factory-path-staging invocation count 889→14,151 — root cause release-lag: rc.23 tagged 2026-07-18, S-21.01 merged 2026-07-23 (7bb0e797) — guard was never in operator runtime; (ii) Dependabot vulnerability count 7-8→18 (cumulative as of 2026-08-10); (iii) RUSTSEC-2026-0204 advisory unanchored — not tracked to a specific story/wave; (iv) RUSTSEC-2026-0190 + RUSTSEC-2026-0052 also unanchored; cargo-deny fails with 5 findings total; (v) RUSTSEC-2026-0188 exploitability framing corrected — prior phrasing overstated inapplicability; (d) refuse_setuid HIGH SECURITY finding registered: `fs::metadata(PathBuf::from(bare_name))` always returns Err for bare names (stat syscall on non-path-containing string = ENOENT); gate in `crates/factory-dispatcher/src/host/exec_subprocess.rs` never fires in production; all 44 exec_subprocess registry entries use bare names (not absolute paths); module doc's categorical claim "refuses to execute setuid binaries" has never been true; (e) ADR-043 v1.0 DO-NOT-RATIFY review persisted as `adv-adr-043-pass-1.md` (ADR-scoped artifact, NOT cycle pass-11; 4B/7H/6M/2L/1N); NOT added to INDEX.md adversarial-reviews table; Convergence Status UNCHANGED; (f) 4 L-BB lessons appended: L-BB-subagent-idle-without-report [process-gap], L-BB-adversary-tool-profile-mismatch [process-gap], L-BB-adversarial-review-before-ratification [implementation], L-BB-recorded-magnitude-drift [process-gap]; (g) W3 wave-gate WAIVED per human; sequencing ruling REVISED: S-21.09 can be built FIRST — R-S2112-03 ci.yml-conflict premise falsified (S-21.07 story file has zero occurrences of ci.yml or .github/workflow); (h) streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX BC v4.55/VP v2.76/STORY v4.292/ARCH v3.54; policies.yaml v1.4.23 ACTIVE; STATE.md v7.09→v7.10. | S-21.09 draft→ready 5 pts; STORY-INDEX v4.292; 6 factual corrections; refuse_setuid HIGH SECURITY; ADR-043 DO-NOT-RATIFY; 4 L-BB lessons | D-971-S-21.09-READY-PROMOTION-AND-RECORDED-FACT-CORRECTIONS | 2026-08-10 |
| D-970 | D-970-ADR-040-V1.12-RATIFICATION-BURST (state-manager; single-commit TD-VSDD-053 2026-08-10). (a) POLICY 16 GATE PASS — D-970 allocated; parent-commit 13d7ef6b; (b) ADR-040 v1.12 RATIFIED by human 2026-08-10 on own merits — status proposed→active; ratified 2026-08-10; D-965 erratum preserved in §Status; ratification_note updated; (c) policies.yaml v1.4.22→v1.4.23 applied: POLICY 15 ATTESTATION-LOCATION GATE replacement text (ADR-040 §Decisions 7-10) + Codification 1 (PROJECT-WIDE GATE OUTCOME DISCIPLINE: every mechanical gate MUST report distinct individually-identifiable outcomes; ship a control for every outcome; assert outcome identifier not category; positive control = violating fixture; negative control = compliant fixture) + Codification 2 (POLICY 22 HUMAN-RATIFICATION CHANNEL EXTENSION: load-bearing premises require literal-shell backing; two demonstrations: D-965 false-premise on 5370db80; orchestrator model-attribution D-966 no division performed); (d) ARCH-INDEX v3.53→v3.54; ADR-040 row updated to RATIFIED 2026-08-10 (D-970); (e) 1 L-BB lesson appended: L-BB-stale-pin-guard-is-deployment-signal [process-gap]; (f) DEPLOYMENT BLOCKER: crates/hook-plugins/validate-cross-site-correspondence NOT on origin/develop (only on feature/S-21.07-validate-cross-site-correspondence 5370db80); stale-pin guard correctly EMPTY-or-UNREACHABLE; dependency chain: S-21.09→S-21.07→wire CI job; (g) F-001 status: redesign RATIFIED; not yet in force; closes when Rust crate merged to develop AND CI job wired and demonstrably running; (h) F-003 superseded — v1.12 ratified on own merits per D-970; D-965 erratum preserved; (i) streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX BC v4.55/VP v2.76/STORY v4.291/ARCH v3.54; policies.yaml v1.4.23 ACTIVE; STATE.md v7.07→v7.08. | ADR-040 v1.12 RATIFIED; policies.yaml v1.4.23 ACTIVE; Codifications 1+2 APPLIED; ARCH-INDEX v3.54; F-001 RATIFIED not-in-force | D-970-ADR-040-V1.12-RATIFICATION-BURST | 2026-08-10 |
| D-969 | D-969-ADR-040-V1.12-REDESIGN-RECORD-BURST (state-manager; single-commit TD-VSDD-053 2026-08-10). (a) POLICY 16 GATE PASS — D-969 allocated; parent-commit 70b4ecd1; (b) ADR-040 v1.12 AMENDED + REOPENED: F-001 root-cause = category error (gate evaluated in factory-artifacts worktree; 0 *.rs/*.bats by construction; pre-check permanently EMPTY); INAPPLICABLE branch retired; obligation unconditional + path-pinned; (c) redesign at feature/policy15-gate-rust d2a3176a: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, no #[non_exhaustive], mutation-verified (guard1 + attestation-count); (d) F-001 REMEDIATION DESIGNED — OPEN pending human re-ratification + policies.yaml v1.4.23 + CI wiring; (e) F-003 OPEN — D-965 ratification PROCURED-ON-MISCHARACTERIZATION erratum recorded in ADR-040 v1.12; (f) F-002 permanent historical exception — erratum owed at S-21.07 merge, anchored; (g) codifications 1+2 PROPOSED (NOT applied); (h) ARCH-INDEX v3.52→v3.53; 3 L-BB lessons appended; (i) streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; policies.yaml v1.4.22 UNCHANGED; STATE.md v7.06→v7.07. | ADR-040 v1.12 REOPENED; F-001 redesign at d2a3176a; 3 lessons; ARCH-INDEX v3.53; streak UNCHANGED | D-969-ADR-040-V1.12-REDESIGN-RECORD-BURST | 2026-08-10 |
| D-968 | D-968-PR-774-POST-MERGE-RECORD-BURST (state-manager; single-commit TD-VSDD-053 2026-08-10). (a) POLICY 16 GATE PASS — D-968 allocated; parent-commit 48cb6862; (b) PR #774 (`fix/fuel-cap-raise-20m`) merged 2026-08-10T17:34:37Z; develop `700b4dd3` → `62fbcf1a`; main `80e5cd7b` UNCHANGED; (c) F-S2107-P10-007 CLOSED by `62fbcf1a` — `extract_reason_from_outcome` now distinguishes fuel/epoch; (d) F-S2107-P10-004 SHIFTED — "has been raised to 20M" now true on develop but ambiguous referent (operator cache rc.23 still 10M); (e) POL-14 N/A (fix branch, no story behavioral_contracts); (f) 5 GitHub reviews all COMMENTED; last at `783b88e6`; content verdict no BLOCKER; (g) L-BB-gate-never-invoked-is-functionally-absent appended; (h) streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX UNCHANGED; policies.yaml UNCHANGED; STATE.md v7.05→v7.06. | PR #774 merged; develop 62fbcf1a; F-007 CLOSED; F-004 SHIFTED; lesson appended; streak UNCHANGED | D-968-PR-774-POST-MERGE-RECORD-BURST | 2026-08-10 |
| D-967 | D-967-PASS-10-CORRECTION-BURST (state-manager; single-commit TD-VSDD-053 2026-08-10). (a) POLICY 16 GATE PASS — D-967 allocated; parent-commit 38cd1037; (b) orchestrator relay error corrected: F-006 precision note attribution fixed — `725,832 bytes` is adapter-class model threshold (`29,452 + 27.514 × payload_bytes`) NOT cross-site model (`2,585,970 + 53.18 × var_bytes`); (c) F-S2107-P10-010 MEDIUM [process-gap] added; pass-10 count 9→10; MEDIUM 2→3; (d) adversary-pass-10.md + INDEX.md + trajectory-tail corrected →20→16→8→10; (e) L-BB-correction-same-verification-obligation appended: enforcer not exempt from POLICY 22; (f) streak 0/3 UNCHANGED; 4-INDEX UNCHANGED; policies.yaml UNCHANGED; STATE.md v7.04→v7.05. | orchestrator relay error corrected; F-010 added; pass-10 9→10; lesson appended; streak UNCHANGED | D-967-PASS-10-CORRECTION-BURST | 2026-08-10 |
| D-966 | D-966-PASS-10-RECORD-BURST (state-manager; single-commit TD-VSDD-053 2026-08-09). (a) POLICY 16 GATE PASS — D-966 allocated; parent-commit cbff0801; (b) adversary-pass-10.md persisted NOT-CLEAN B2/H4/M2/L1 9 findings; (c) Codification 1 PROPOSED: META-LEVEL-25 vacuous gate; (d) Codification 2 PROPOSED: POLICY 22 ratification-channel extension; (e) Codification 3 applied: Block 8 parent-SHA convention; (f) Codification 4 candidate: nested .factory/.factory plausible link; (g) 9 findings routed; (h) streak 0/3 RESET; trajectory-tail →20→16→8→9. 4-INDEX UNCHANGED. STATE.md v7.03→v7.04. NOTE: F-006 precision note corrected by D-967; F-010 added retroactively. | adversary pass-10 persisted; 10 findings (D-967 corrected); 2 PROPOSED codifications; Blocking Issue P0-F-001; streak 0/3 reset | D-966-PASS-10-RECORD-BURST | 2026-08-09 |
| D-965 | D-965-POLICY-15-ATTESTATION-GATE-RATIFICATION-BURST (state-manager; single-commit TD-VSDD-053 2026-08-09). policies.yaml v1.4.21→v1.4.22 RATIFIED; ADR-040 v1.1 active; ARCH-INDEX v3.51→v3.52; pipeline PAUSED→ACTIVE; STATE.md v7.02→v7.03. | policies.yaml v1.4.22 RATIFIED; ADR-040 active; ARCH-INDEX v3.52; pipeline ACTIVE | D-965-POLICY-15-ATTESTATION-GATE-RATIFICATION | 2026-08-09 |
| D-964 | D-964-PASS-9-CLOSURE-FUEL-REMEDIATION-BURST (state-manager; single-commit TD-VSDD-053 2026-08-08). pass-9 8/8 CLOSED; ADR-042 v1.2 ratified (10M→20M); S-21.13 registered; D-945 DISCHARGED; ORCH ERROR corrected. BC v4.55; STORY v4.291; ARCH v3.51; STATE.md v7.00→v7.01. | pass-9 ALL 8 CLOSED; ADR-042 ratified; ORCH ERROR corrected; 4-INDEX ADVANCED | D-964-PASS-9-CLOSURE-FUEL-REMEDIATION | 2026-08-08 |
| D-963 | D-963-BC-CORRECTION-BURST (state-manager; single-commit TD-VSDD-053 2026-08-08). BC-5.39.010 ~110 FALSIFIED; ERRATUM inserted; 2 Drift Items; 4-INDEX UNCHANGED; STATE.md v6.99→v7.00. | ~110 falsified; 4 safe/5th exhausts SS-05; 2 Drift Items; 4-INDEX UNCHANGED | D-963-BC-CORRECTION-BURST | 2026-08-08 |
| D-962 | D-962-PASS-9-RECORD-BURST (state-manager; single-commit TD-VSDD-053 2026-08-08). adversary-pass-9.md persisted (NOT-CLEAN B0/H3/M3/L1/NIT1; HALVING 16→8; first zero-BLOCKER; reviewed HEAD 67ffbdcc). BC v4.53; STORY v4.290; trajectory-tail →24→20→16→8. | NOT-CLEAN B0/H3/M3/L1/NIT1 (HALVING 16→8); first zero-BLOCKER; BC v4.53; streak 0/3 (8 passes) | D-962-PASS-9-RECORD-BURST | 2026-08-08 |
| D-961 | D-961-RECORDING-BURST (state-manager; single-commit 2026-08-07). All pass-8 findings resolved; E-22 DISSOLVED (file RETAINED per human ruling); ADR-041 sentinel COMPLETE; BC v4.52; ARCH v3.47; STORY v4.289; SHA-patch e2bfec65. | All pass-8 resolved; E-22 dissolved (RETAINED); ADR-041 COMPLETE; BC v4.52/ARCH v3.47; streak 0/3 | D-961-RECORDING-BURST | 2026-08-07 |
| D-960 | S-21.07-PASS-8-RECORD-BURST (state-manager; single-commit 2026-08-07; SHA-patch 46b7cef2). NOT-CLEAN B2/H5/M7/L2 (16 findings; IMPROVING 20→16). BC v4.51; VP v2.76; STORY v4.288; ARCH v3.46; trajectory-tail →25→24→20→16. | PASS-8 NOT-CLEAN B2/H5/M7/L2 (16+9); IMPROVING 20→16; BC v4.51; ARCH v3.46; streak 0/3 (7) | D-960-S-21.07-PASS-8 | 2026-08-07 |
| D-959 | ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION (2026-08-06; a0d87706). D-958 retracted — Iron Law violated; CLEAN VOID; streak reverted. 4-INDEX UNCHANGED. | D-958 RETRACTED; streak 0/3; trajectory-tail →25→25→24→20 | D-959-AUTHORSHIP-CORRECTION | 2026-08-06 |
| D-958 | [D-959 RETRACTION] S-21.07-PASS-7-RECORD-BURST. FIX-BURST-CLOSURE-VERIFICATION. BC v4.50; VP v2.75; STORY v4.287; ARCH v3.45; streak 0/3 UNCHANGED (6 true passes). | [RETRACTED per D-959]; BC v4.50; ARCH v3.45; streak 0/3 UNCHANGED | D-958-S-21.07-PASS-7 | 2026-08-06 |
| D-413..D-957 (see decision-log.md for full range) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-05 |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

| Blocker | Status | Risk Statement |
|---------|--------|----------------|
| **[P0] POLICY 15 ATTESTATION-LOCATION GATE vacuous (F-S2107-P10-001)** | **OPEN — D-966 2026-08-09; REDESIGN RATIFIED D-970 — NOT YET IN FORCE** | Root cause (D-969): category error — gate evaluated in factory-artifacts worktree where *.rs/*.bats count is permanently zero; INAPPLICABLE branch was the only reachable outcome. Redesign: ADR-040 v1.12 RATIFIED by human 2026-08-10 (D-970); policies.yaml v1.4.23 ACTIVE; Codifications 1+2 APPLIED. **Closes when:** Rust crate (`crates/hook-plugins/validate-cross-site-correspondence`) merged to develop AND CI job wired and demonstrably running. Deployment blocker: crate NOT on origin/develop (only on feature/S-21.07-validate-cross-site-correspondence `5370db80`); stale-pin guard correctly EMPTY-or-UNREACHABLE. Dependency chain: S-21.09 → S-21.07 → wire CI job. |
| **[P0] `validate-factory-path-staging` WASM guard inert since 2026-07-23** | **OPEN** | 0 fires vs 14,151 sibling invocations (corrected D-971; root cause: release-lag — rc.23 tagged 2026-07-18, guard shipped in S-21.01 merged 2026-07-23 7bb0e797, never in operator runtime); `on_error = "continue"` masks absence. Fix story: S-21.09 (in-flight at 12f280d1; BC-5.39.001 LOCAL streak 0/3). MUST land before S-21.07. |
| **[C-1] CWE-706 incorrect path resolution — exec_subprocess binary_allow load-time prefix check** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | `binary_allow` entries are bare names (e.g., `"wasm-tools"`); `trusted_prefixes` list contains directory paths (e.g., `/usr/local/bin`). At check time, bare name is compared against full prefix paths with no path-join or resolution step; comparison always fails for bare names vs directory prefixes, rendering the prefix list inert. ADR-043 v1.5 §Decision 1 specifies load-time resolution but is NOT RATIFIED. Fix: implement load-time trusted-prefix resolution per ADR-043 §Decision 1 (pending ratification) OR block ratification until implementation is validated. Route: implementer + security-reviewer after ADR-043 ratification. |
| **[C-2] CWE-362 TOCTOU — resolve-then-check window in exec_subprocess path handling** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | Even with load-time resolution, a race exists between resolution and spawn: binary could be replaced after prefix check but before exec. ADR-043 v1.5 §Decision 1 accepts this residual risk under threat model. However, threat model boundary is not formally specified; no documentation of which adversary classes the TOCTOU window is acceptable for. Route: security-reviewer to formalize threat model boundary before ADR-043 ratification. |
| **[C-4] CWE-284 access control — arbitrary binary execution via misconfigured prefix list** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | If `trusted_prefixes` is empty or world-writable directory, any binary can satisfy the prefix check. No validation of prefix list entries at load time (e.g., writable check, non-empty check). ADR-043 v1.5 does not specify prefix validation invariants. Route: product-owner (BC amendment to add prefix-list validation AC) + implementer. |
| **[C-5] CWE-284 no per-entry resource limit isolation** | **OPEN 2026-08-11 — MEDIUM SECURITY (D-972)** | Each `binary_allow` entry has no associated resource limits (CPU time, memory, file descriptor cap). A trusted binary can exhaust host resources. ADR-043 v1.5 §Decision 2 acknowledges this as out-of-scope for load-time resolution but provides no follow-up story. Route: product-owner + story-writer (S-21.14 may cover this if scope includes resource-limit per-entry gates). |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67..74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754. |
| **TD-VSDD-061/062/063** | OPEN 2026-05-17/19 | validate-index-cite-refresh large-file fail-open; schema inconsistencies; VP allocation deferred. |
| **TD-VSDD-095..100** | CODIFIED-AND-FORWARDED | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101** | OPEN 2026-05-18 — anchored S-15.15 (D-972: dangling ref confirmed — VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI; confirmed in test suite) | VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | check_index_sites + rows_after_heading advisory-arm defects. |
| **[D-945] ADR-035 §Decision 5 fuel budget advisory** | **DISCHARGED D-964** | Discharged by ADR-042 §Decision 4. S-21.07 benchmarks are the measurements it awaited. |
| **[D-945] VP-102..VP-118 pending allocation** | DEFERRED — anchored S-21.07 post-merge | 17 VPs per BC-5.39.010 §VP Anchors. |
| **[D-952] compute-input-hash operator cache binary divergence** | BOOTSTRAP-MIGRATED — anchored rc.24 #715 | Self-heals at rc.24. |
| **[D-953] 27 unparseable frontmatter files** | OPEN 2026-08-04 | Needs remediation story. |
| **[D-953] ADR-037 19-story volatile-inputs remediation** | OPEN 2026-08-04 | S-19.01 CRITICAL. |
| **[D-954] Duplicate T-038 test ID** | OPEN 2026-08-04 | POLICY 1 violated; anchor pass-11 or next fix burst. |
| **[D-954] decision-log.md >14,800 lines** | OPEN 2026-08-04 | WASM validators time out on every edit. |
| **[D-955] 18 Dependabot vulnerabilities** | OPEN 2026-08-10 (corrected D-971; was 7-8) | Cumulative as of 2026-08-10. Anchor: next maintenance sweep. |
| **[D-957] F-S2107-P7-019 D-693 stale WASM size** | OPEN 2026-08-05 | Anchor: pass-11 fix burst or next SHA-patch. |
| **[D-958] 60 of 158 stories lack tdd_mode** | OPEN 2026-08-06 | Anchor: S-15.03 PRIORITY-A. |
| **[D-958] GATE DEFECT: validate-pr-review-posted + validate-changelog-monotonicity** | OPEN 2026-08-06 | Paper-gate; header-skip misread. |
| **[D-961] SEC-001 + RUSTSEC-2026-0222/0204 + 18 Dependabot + EAC-002 + ADR-033** | OPEN 2026-08-07 — SECURITY | E-22 scope re-anchored to E-21 W4. Dependabot count corrected D-971. |
| **[D-961] fix/nested-factory-path-derivation + fix/d999-sentinel-code-migration** | RESOLVED 2026-08-09 — both pushed | Both branches pushed to origin (SESSION-WRAP-2026-08-09); team-lead decides PR/merge path. |
| **[D-962] F-S2107-P9-002 fuel-exhaustion gate impl** | **CLOSED D-964** | ADR-042 10M→20M cap; gate impl `5370db80`; S-21.13 W7 anchor for §Decision 5. |
| **[D-962] F-S2107-P9-003 POLICY 15 gate → 0** | **CLOSED D-964** | ADR-040 §Decision 6 conditional pre-check + line-anchored predicate; 3 attestation sections at `5370db80`. |
| **[D-963] ADR-035 §Decision 5 quadratic not observed** | OPEN 2026-08-08 | Direct measurement linear R²=0.998790. Route: architect at next ADR-035 touch. |
| **[D-963] BC-5.39.010 live-operation silent exhaustion gap** | OPEN 2026-08-08 | plugin.timeout exits 0/empty; live agents receive no signal. Bats-only margin gate insufficient. Anchor: S-21.07 + margin gate implementation. |
| **[D-964] policies.yaml v1.4.22 PROPOSED** | **RESOLVED 2026-08-09 (D-965)** | ADR-040 §Decision 6 replacement text ratified by human 2026-08-09; policies.yaml v1.4.22 active; ADR-040 v1.1 active. |
| **[D-964] fix/fuel-cap-raise-20m NOT YET EFFECTIVE** | OPEN 2026-08-10 (D-968) — release-gated | Fix is now **on develop** (`62fbcf1a`); operator cache `1.0.0-rc.23` still embeds 10M; requires rc.24 release to become effective in hook chain. Events continue. |
| **[SESSION-WRAP-2026-08-09] Dispatcher log deletion recurrence — 3 occurrences** | OPEN 2026-08-09 — root cause unestablished | Three unexplained working-tree deletions of tracked dispatcher-internal logs in a single session. Restored each time via `git restore`. Nested `.factory/.factory` confirmed (2 log files); plausible mechanism per O-S2107-P10-02; causal link unconfirmed. Anchor: next maintenance sweep + post-release of fix/nested-factory-path-derivation. |
| **[D-966] F-002 retroactive-attestation (permanent)** | OPEN 2026-08-09 — permanent historical violation | 67ffbdcc + 38c70f9e lack "at that commit" attestation (retroactively added at 5370db80); history immutable; note in ADR-040 v1.12 redesign context. Route: architect. |
| **[D-966] F-005 ADR-041/ADR-042 status-field drift** | OPEN 2026-08-09 | ADR-041 + ADR-042 frontmatter `status: proposed`; pipeline assets assert "ratified"; POLICY 16 gate governed by proposed ADR-041. Route: architect. |
| **[D-966] F-006 ADR-042 §Decision 1 vs §Decision 2 self-contradiction** | OPEN 2026-08-09 (D-967 correction: adapter threshold `725,832` confirmed; rc.24 fuel-cap raise INSUFFICIENT for large cycle artifacts; size budgets + compaction are load-bearing fix) | §Decision 1 row-4 claims independent per-plugin budgets; §Decision 2 declares global raise only; POLICY 13 BOUNDARY-POLARITY mutant absent; re-ratification required. Route: architect. |
| **[D-966] F-007 ADR-042 §Decision 3 class (b) unanchored dispatch** | **CLOSED D-968 (2026-08-10)** | `extract_reason_from_outcome` fuel/epoch disambiguation shipped at `62fbcf1a`. `PluginResult::Timeout` now carries `fuel_cap` field; fuel arm emits `fail-closed: FUEL_EXHAUSTED: fuel cap of {fuel_cap} units exhausted…` distinct from epoch `fail-closed: plugin timed out`. |
| **[D-966] F-008 TD-VSDD-091 line-number pins** | OPEN 2026-08-09 | ADR-040 §Context "line 294"; ADR-042 §Empirical "BC-INDEX line 1464"; ARCH-INDEX ADR-042 row "~415KB". Route: architect. |
| **[D-966] F-009 BC-5.39.010 modified[]-erratum parity** | OPEN 2026-08-09 | Body `1.15-erratum` row has no `modified[]` entry; POLICY 14 leg-3 gap. Route: product-owner. |
| **[D-966] D-965-mischaracterization: ADR-040 §Decision 6 justifying premise false** | **SUPERSEDED D-970** | D-965 ratification = PROCURED-ON-MISCHARACTERIZATION; erratum preserved. ADR-040 v1.12 RATIFIED by human 2026-08-10 (D-970) on own merits. policies.yaml v1.4.23 ACTIVE. F-003 SUPERSEDED. |
| **[D-968] F-004 BC-5.39.010 present-perfect SHIFTED** | OPEN 2026-08-10 | "has been raised to 20,000,000 … satisfiable at HEAD" now TRUE on develop (`62fbcf1a`) but ambiguous referent: operator cache `1.0.0-rc.23` still has 10M cap. Defect = ambiguous "HEAD" (unqualified). Route: product-owner to clarify referent to post-`62fbcf1a` develop-HEAD. |
| **[D-969] feature/policy15-gate-rust pending integration** | OPEN 2026-08-10; ratification complete D-970 | `crates/policy15-attestation-gate/` at `d2a3176a` on `feature/policy15-gate-rust`. ADR-040 v1.12 RATIFIED. Still awaits: (1) Rust crate merged to develop via S-21.07 after S-21.09; (2) CI job wired; (3) hooks-registry.toml entry; (4) demonstrably running in CI. |
| **[D-970] CI-wiring deployment blocker** | OPEN 2026-08-10 | `crates/hook-plugins/validate-cross-site-correspondence` NOT on origin/develop; only on `feature/S-21.07-validate-cross-site-correspondence` `5370db80`. Stale-pin guard correctly returning EMPTY-or-UNREACHABLE on origin/develop (not a defect). Dependency chain: S-21.09 MUST merge first → then S-21.07 → then wire CI job. F-001 closes when CI job wired and demonstrably running. |
| **[D-971] RUSTSEC-2026-0204/0190/0052 unanchored advisories** | OPEN 2026-08-10 — SECURITY | Three RUSTSEC advisories unanchored (no story/wave assignment); cargo-deny fails with 5 findings total (includes RUSTSEC-2026-0222 already listed under D-961). Anchor: E-22 security scope decision (§6 pending human decision). |
| **[D-971] RUSTSEC-2026-0188 exploitability framing** | OPEN 2026-08-10 — SECURITY | Prior STATE.md phrasing overstated inapplicability; actual exploitability assessment requires architect + security-reviewer review. Route: security-reviewer. Anchor: E-22 security scope. |
| **[D-971] refuse_setuid gate inert — HIGH SECURITY** | OPEN 2026-08-10 — HIGH SECURITY | `crates/factory-dispatcher/src/host/exec_subprocess.rs`: `fs::metadata(PathBuf::from(bare_name))` always returns Err for bare names (stat(2) on non-path string = ENOENT); gate never fires in production; all 44 exec_subprocess registry entries use bare names; module doc's claim "refuses to execute setuid binaries" has never been true. Fix scope: human to direct (implement proper path-resolve-then-stat logic OR acknowledge documentation-only fix). Route: security-reviewer + implementer. Anchor: E-22 or dedicated fix story. |
| **[D-972] refusal_path_not_found inert gate** | OPEN 2026-08-11 — VACUOUS GATE | `exec_subprocess.rs` refusal branch for `path_not_found` error variant is unreachable in production (same root cause as refuse_setuid: bare names always produce ENOENT before path lookup). Gate counter permanently zero. Route: implementer. Anchor: C-1 fix scope. |
| **[D-972] platform_block_inert gate** | OPEN 2026-08-11 — VACUOUS GATE | `cfg(not(unix))` block in exec_subprocess.rs returns `Err(ExecError::PlatformNotSupported)` but hook-registry has no `platforms` constraint; dispatch continues on windows, reaching the unix-only path with no fallback. Platform guard is inert on CI (linux/macos). Route: implementer. Anchor: ADR-043 §Decision 1 cfg(windows) finding (pass-3 B-1). |
| **[D-972] wildcard_bypass_inert gate** | OPEN 2026-08-11 — VACUOUS GATE | `binary_allow` entries with glob-like characters (e.g., `"*"`) are compared literally; no glob expansion; wildcard entry neither allows all nor denies all, it allows only the literal string `"*"`. Registry currently has no such entries, but the behavior is undocumented and could be mistaken for a wildcard. Route: product-owner (BC amendment to document literal-match-only semantics). |
| **[D-972] time_of_check_time_of_use gap (TOCTOU inert gate)** | OPEN 2026-08-11 — VACUOUS GATE (see C-2) | Even with load-time resolution (ADR-043 proposed), no check verifies the resolved path still matches the prefix at spawn time. The gate that would close this (spawn-time re-verification) does not exist. Linked to C-2 (CWE-362). Route: security-reviewer to scope. |
| **[D-972] prefix_list_empty_fallthrough** | OPEN 2026-08-11 — VACUOUS GATE | When `trusted_prefixes` is empty list `[]`, the prefix-check loop exits with `allowed = false` which is correct — but `on_error = "continue"` in the hook registry entry means a failed prefix check does NOT block dispatch; it produces an advisory. The block gate is therefore vacuous when operator misconfigures `on_error`. Route: devops-engineer (hooks-registry.toml `on_error` value for exec_subprocess hook). Anchor: ADR-043 ratification. |
| **[D-972] resource_limit_unenforced** | OPEN 2026-08-11 — VACUOUS GATE | No per-`binary_allow` entry resource limit (ulimit, cgroup, timeout) is applied at spawn time. Any trusted binary can consume unbounded host resources. Linked to C-5 (CWE-284). Route: product-owner + story-writer. Anchor: S-21.14 or dedicated story. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-11 — D-972-SS01-EXEC-SUBPROCESS-OPTION-C-ADJUDICATION-AND-S-21.09-DELIVERY-RECORD COMPLETE; single commit pending push; PIPELINE ACTIVE; pass-11 adversary NEXT)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. Pass-10 recorded; **no pass-11 yet**. Streak **0/3** (10 adversary passes, zero CLEAN). Trajectory `47→18→25→25→24→20→16→8→10`, tail `→20→16→8→10`. 4-INDEX: BC v4.56 / VP v2.76 / **STORY v4.293** / ARCH v3.55. `policies.yaml` **v1.4.23**. factory-artifacts HEAD pending push (D-972 single commit over parent `b278d978`).

**D-972 burst complete (single commit, not yet pushed).** Option C adjudication: 5 BCs amended (D-972 sentinel replaced); ADR-043 v1.5 proposed NOT RATIFIED (3-pass DO-NOT-RATIFY trajectory; two prevented regressions); ARCH-INDEX v3.55 (ADR-043 row + SS-01 host/ catalog); STORY-INDEX v4.293 (S-21.09 5→10 pts re-estimated; S-21.14/S-21.15 registered draft W8); E-21 epic v1.12 14 stories 111 pts; sprint-state.yaml S-21.09 in-flight; S-21.09 LOCAL 3 DO-NOT-RATIFY passes (streak 0/3); 7 L-BB lessons; C-1/C-2/C-4/C-5 blocking security issues + 6 vacuous gate drift items registered.

**D-971 burst (prior).** S-21.09 promoted draft→ready (5 pts); STORY-INDEX v4.292; 6 factual corrections; refuse_setuid inert HIGH SECURITY registered; ADR-043 v1.0 DO-NOT-RATIFY (ADR-scoped); 4 L-BB lessons; commit a06fae30 pushed.

**Bursts.** D-965 (POLICY 15 v1.4.22) → D-966 (pass-10 record, 9 findings) → D-967 (relay error corrected; F-010 added; count 9→10) → D-968 (post-merge; F-007 CLOSED; F-004 SHIFTED) → D-969 (ADR-040 v1.12 amended + reopened) → D-970 (RATIFIED: ADR-040 v1.12 `active`; `policies.yaml` v1.4.23; Codifications 1+2 applied) → D-971 (S-21.09 READY 5 pts; 6 factual corrections; refuse_setuid HIGH SECURITY; ADR-043 DO-NOT-RATIFY; 4 L-BB lessons; STORY v4.292; commit a06fae30; SHA-patch done) → **D-972 (Option C adjudication; 5 BC amendments; ADR-043 v1.5 proposed NOT RATIFIED; ARCH v3.55; STORY v4.293; E-21 14 stories 111 pts; S-21.09 in-flight 12f280d1; LOCAL streak 0/3; 7 L-BB lessons; C-1/C-2/C-4/C-5; 6 vacuous drift items; single commit pending push)**.

### §2 Convergence

Streak **0/3**. Ten passes, zero CLEAN. Trajectory-tail `→20→16→8→10`. Dominant pattern: **vacuous-gate** (gate-domain-mismatch class). F-001 redesign RATIFIED (ADR-040 v1.12; D-970) — not yet in force (dependency chain: S-21.09→S-21.07→wire CI job). pass-11 adversary NEXT on resume.

### §3 Branch States

| Branch | SHA | Note |
|--------|-----|------|
| `factory-artifacts` | b278d978 (HEAD) | D-972 single commit staged, NOT YET PUSHED. Push via: `git -C .factory push origin HEAD:factory-artifacts` |
| `feature/S-21.09` | 12f280d1 | S-21.09 in-flight (10 pts; LOCAL streak 0/3; 3 DO-NOT-RATIFY passes; C-1/C-2/C-4/C-5 open). Not pushed. No PR. |
| `feature/S-21.07-validate-cross-site-correspondence` | `5370db80` | pass-10 NOT-CLEAN 10 findings; FROZEN; NO REBASE; S-21.09 must land first |
| `feature/S-21.04` | `323f440f` | pass-31 pending; no PR |
| `feature/policy15-gate-rust` | `d2a3176a` | F-001 redesign crate; 16 tests; mutation-verified; no PR |
| `fix/nested-factory-path-derivation` | `9afc3226` | P8-016 + P9-008 CLOSED; pushed; team-lead decides merge path |
| `fix/d999-sentinel-code-migration` | `bf642fd9` | ADR-041 sentinel; pushed; team-lead decides merge path |
| `develop` | `62fbcf1a` | PR #774 merged 2026-08-10 |
| `main` | `80e5cd7b` | rc.23 bot binary bundle 2026-07-18 |

No PRs open (S-21.09 in-flight; no PR yet). CI does not fire on feature-branch pushes.

### §4 F-001 — Redesign Ratified, Not In Force

Root cause: **category error** — gate evaluated in `factory-artifacts` worktree (0 `*.rs`/`*.bats` by construction; pre-check permanently EMPTY). Redesign: obligation **unconditional + path-pinned**; `INAPPLICABLE` retired; 4 outcomes each with a control. Implementation: **`crates/policy15-attestation-gate/`** on `feature/policy15-gate-rust` at `d2a3176a`; 16 tests; `GateOutcome` enum; not `#[non_exhaustive]`; mutation-verified. **Status: REDESIGN RATIFIED D-970 — NOT YET IN FORCE.** CI wiring BLOCKED until `crates/hook-plugins/validate-cross-site-correspondence` reaches `origin/develop` via S-21.09→S-21.07. Branch tip `f81ab5a6` bundles gate crate with unrelated CLAUDE.md docs commit — **split before opening PR.**

### §5 Resume Order

1. **Push D-972 factory-artifacts single commit:** `git -C /Users/zious/Documents/GITHUB/vsdd-factory/.factory push origin HEAD:factory-artifacts` (then SHA-patch: update factory-artifacts row with actual Commit E SHA).
2. S-21.09 active development — resolve C-1/C-2/C-4/C-5 blocking issues per ADR-043 §Decision 1 path + LOCAL adversary pass-4 toward 3-CLEAN.
3. PR the gate crate (`feature/policy15-gate-rust`) — split or note CLAUDE.md bundling explicitly.
4. pass-11 adversary — fresh context, Iron Law (D-959).

### §6 Pending Human Decisions

1. Merge sequencing confirmed: S-21.09 FIRST (sequencing ruling REVISED D-971 — R-S2112-03 ci.yml-conflict premise falsified); then S-21.07; then wire CI job.
2. E-22 security scope: SEC-001, RUSTSEC-0222/0204/0190/0052 (18 Dependabot total), EAC-002, ADR-033.
3. fix/nested + fix/d999 delivery path (both pushed; team-lead decides PR/merge path).
4. rc.24 gating: F-006 confirms 20M insufficient for 3 cycle artifacts; compaction needed.
5. ADR-043 DO-NOT-RATIFY at v1.5 (converging but NOT RATIFIED): 3 passes complete; v1.5 has no remaining BLOCKERs per pass-3 verdict; human to decide ratification disposition (ratify v1.5 / request pass-4 / redirect design).
6. refuse_setuid HIGH SECURITY: gate in `crates/factory-dispatcher/src/host/exec_subprocess.rs` is functionally absent; human to direct fix scope (proper path-resolve-then-stat vs documentation-only fix).
7. C-1/C-2/C-4/C-5 blocking security issues: human to confirm ADR-043 ratification path before implementer proceeds with load-time resolution.

### §7 Open Items

- **[P0] `validate-factory-path-staging` inert** — 0 fires vs **14,151** sibling invocations (corrected D-971); S-21.09 in-flight at 12f280d1 (LOCAL streak 0/3; MUST land before S-21.07).
- **[HIGH SECURITY] refuse_setuid gate inert** — `fs::metadata(PathBuf::from(bare_name))` always Err; 44 registry entries use bare names; module doc claim has never been true; fix scope per human direction.
- **[HIGH SECURITY] C-1 CWE-706** — exec_subprocess binary_allow prefix check inert; ADR-043 v1.5 proposed NOT RATIFIED; load-time resolution pending ratification + implementation.
- **[HIGH SECURITY] C-2 CWE-362** — TOCTOU window; ADR-043 threat model boundary unformalized; security-reviewer review needed before ratification.
- **[HIGH SECURITY] C-4 CWE-284** — prefix list empty/writable fallthrough; BC amendment for validation invariants pending.
- **[MEDIUM SECURITY] C-5 CWE-284** — no per-entry resource limits; S-21.14 anchor or dedicated story.
- **6 vacuous gate drift items registered** — refusal_path_not_found, platform_block, wildcard_bypass, TOCTOU gap, prefix_list_empty_fallthrough, resource_limit_unenforced; all linked to C-1..C-5 or ADR-043.
- **8 pass-10 findings OPEN**: F-002 (permanent historical; erratum at S-21.07 merge), F-004 (SHIFTED; product-owner), F-005 (ADR-041/042 proposed vs ratified; architect), F-006 (§Decision 1 vs §Decision 2; architect), F-008 (TD-VSDD-091 line pins; architect), F-009 (modified[] erratum; product-owner), F-010 (process-gap). F-003 SUPERSEDED D-970.
- **ADR-043 v1.5 NOT RATIFIED** — converging; human to decide disposition (pending §6 item 5).
- **Log-deletion hypothesis**: nested `.factory/.factory` path; unconfirmed; anchor: maintenance sweep post-release of `fix/nested-factory-path-derivation`.

### §8 Cautions

- Do NOT `/rehydrate-wave` — wave-state points at a closed epic.
- Do NOT `compute-input-hash --scan --update` — 418-file blast radius (D-936); single-file only.
- Use explicit refspecs: `git push origin HEAD:<branch>`.
- Main worktree on `fix/nested-factory-path-derivation` @ `9afc3226`. Next code resume: `git checkout develop && git pull`.
- `.claude/scheduled_tasks.lock` (M) + `plugins/vsdd-factory/tests/report.tap` (??) are harness artifacts — leave them.
- Fuel cap fix (`62fbcf1a`) on develop; NOT in operator cache until rc.24 release.
- F-001 NOT YET IN FORCE — close only when CI job wired and demonstrably running.
- `feature/policy15-gate-rust` tip `f81ab5a6` bundles unrelated CLAUDE.md docs commit — split or make bundling explicit before PR.
- `validate-factory-path-staging` inert guard count is **14,151** sibling invocations (corrected D-971; prior figure 889 was incorrect).
- ADR-043 v1.5 NOT RATIFIED — 3 passes complete; human decides disposition before implementer proceeds on C-1 fix.
- C-1/C-2/C-4/C-5 blocking issues OPEN — do not merge S-21.09 until security issues addressed or explicitly deferred by human.
- D-972 factory-artifacts commit NOT YET PUSHED — push before any SHA-patch step.

### §9 Resume Command

`/vsdd-factory:next-step`. Suggested order: (1) Push D-972 factory-artifacts commit + SHA-patch, (2) S-21.09 LOCAL pass-4 (resolve C-1/C-2/C-4/C-5 path toward 3-CLEAN), (3) pass-11 adversary, (4) PR gate crate (split CLAUDE.md commit).
