---
document_type: pipeline-state
level: ops
version: "7.06"
status: draft
producer: state-manager
timestamp: 2026-08-10T20:30:00Z
phase: D-968-PR-774-POST-MERGE-RECORD-BURST
last_amended: "2026-08-10 (v7.06) — D-968-PR-774-POST-MERGE-RECORD-BURST (state-manager): PR #774 merged (62fbcf1a); develop 700b4dd3→62fbcf1a; fix/fuel-cap-raise-20m MERGED+DELETED; F-S2107-P10-007 CLOSED; F-S2107-P10-004 SHIFTED; Drift Items updated; L-BB-gate-never-invoked-is-functionally-absent appended; D-968 allocated; STATE.md v7.05→v7.06. [Prior: 2026-08-10 (v7.05) — D-967-PASS-10-CORRECTION-BURST (state-manager): orchestrator relay error in D-966 corrected; F-006 precision note attribution fixed (adapter model, not cross-site); F-S2107-P10-010 MEDIUM added; pass-10 count 9→10; trajectory-tail →20→16→8→10; L-BB-correction-same-verification-obligation appended; D-967 allocated; STATE.md v7.04→v7.05. [Prior: 2026-08-09 (v7.04) — D-966-PASS-10-RECORD-BURST (state-manager): adversary pass-10 persisted (NOT-CLEAN B2/H4/M2/L1; 9 findings; streak 0/3 reset); Blocking Issue P0-F-001 added; 7 Drift Items added; trajectory-tail →20→16→8→9; D-966 allocated; STATE.md v7.03→v7.04.]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "D-968-PR-774-POST-MERGE-RECORD-BURST (state-manager; parent-commit: 48cb6862; STATE.md v7.05→v7.06; PR #774 merged 2026-08-10T17:34:37Z; develop 700b4dd3→62fbcf1a; fix/fuel-cap-raise-20m MERGED+DELETED from origin; F-S2107-P10-007 CLOSED by merge; F-S2107-P10-004 SHIFTED (ambiguous referent); Drift Items updated; L-BB-gate-never-invoked-is-functionally-absent appended to lessons.md; D-968 allocated; POL-14 N/A (fix branch); streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED; 4-INDEX BC v4.55/VP v2.76/STORY v4.291/ARCH v3.52 UNCHANGED; policies.yaml UNCHANGED)."
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
| **Last Updated** | 2026-08-10 — D-968-PR-774-POST-MERGE-RECORD-BURST: PR #774 merged (62fbcf1a); develop 700b4dd3→62fbcf1a; fix/fuel-cap-raise-20m MERGED+DELETED; F-007 CLOSED; F-004 SHIFTED; D-968 allocated; trajectory-tail →20→16→8→10; STATE.md v7.05→v7.06. [Prior: 2026-08-10 — D-967-PASS-10-CORRECTION-BURST: F-006 precision note fixed; F-010 added; pass-10 9→10; STATE.md v7.04→v7.05.] |
| **Current Phase** | **D-968-PR-774-POST-MERGE-RECORD-BURST (2026-08-10). PIPELINE ACTIVE. adversary pass-10 NOT-CLEAN B2/H4/M3/L1 (10 findings); streak 0/3 UNCHANGED; trajectory 47→18→25→25→24→20→16→8→10 (tail: →20→16→8→10); 9 adversary passes; 0 CLEAN verdicts; BLOCKER P0-F-001: POLICY 15 gate vacuous; D-965 ratification PROCURED-ON-MISCHARACTERIZATION (F-003); F-007 CLOSED by 62fbcf1a; F-004 SHIFTED (ambiguous referent); last D-NNN: D-968. develop 62fbcf1a. main 80e5cd7b. merged_count 107. 4-INDEX BC v4.55/VP v2.76/STORY v4.291/ARCH v3.52. pass-11 adversary NEXT.** |
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
| D-968-PR-774-POST-MERGE-RECORD-BURST 2026-08-10 (single commit TD-VSDD-053; parent 48cb6862) | state-manager | COMPLETE | PR #774 merged (62fbcf1a); develop advanced; fix/fuel-cap-raise-20m removed; F-007 CLOSED; F-004 SHIFTED; Drift Items updated; L-BB-gate-never-invoked lesson appended; STATE.md v7.05→v7.06; streak 0/3 UNCHANGED; trajectory-tail →20→16→8→10 UNCHANGED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.55 D-964) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 15 stub IDs (STORY-INDEX v4.291 D-964) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 42 (ADR-042 NEW D-964; ADR-041 NEW D-961; ADR-040 NEW D-960) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | 107 (STATE.md explicit counter; sprint-state predicate: 113; canonical D-853) |

## Story Status

130 file-resident + 15 stub IDs = 145 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (107):** S-19.07 MERGED PR #670 6db4c9fc (E-19 COMPLETE 9/9). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** --
- **E-21:** S-21.07 (W4; pass-10 NOT-CLEAN 10 findings D-967; BC-5.39.010 v1.17; branch 5370db80 pushed; NO REBASE; S-21.09 MUST land first); S-21.09 (NO branch — MUST land before S-21.07); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | 62fbcf1a | PR #774 squash-merged 2026-08-10T17:34:37Z. Pull on next code-worktree resume. |
| factory-artifacts | 48cb6862 | D-968-PR-774-POST-MERGE-RECORD-BURST parent SHA (D-967 commit; actual D-968 commit SHA post-push per codification 3). |
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
| v1.0-brownfield-backfill | brownfield | D-968 PR-774-POST-MERGE-RECORD-BURST 2026-08-10 PIPELINE ACTIVE. develop 62fbcf1a; main 80e5cd7b; merged_count 107; BC v4.55; VP v2.76; STORY v4.291; ARCH v3.52; streak 0/3 (9 passes); trajectory-tail →20→16→8→10 UNCHANGED. F-007 CLOSED; F-004 SHIFTED. BLOCKER P0-F-001: POLICY 15 gate vacuous; D-965 ratification PROCURED-ON-MISCHARACTERIZATION. pass-11 adversary NEXT. | D-968 2026-08-10; D-967 2026-08-10; D-966 2026-08-09; D-965 2026-08-09. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-968 (see decision-log.md for full range): this Decisions Log (D-958..D-968 (see decision-log.md for full range) live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
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
| **[P0] POLICY 15 ATTESTATION-LOCATION GATE vacuous (F-S2107-P10-001)** | **OPEN — D-966 2026-08-09** | Gate fires in factory-artifacts context where `*.rs`/`*.bats` count is permanently zero; pre-check always EMPTY → always INAPPLICABLE; gate can never produce a non-trivially-true result for any factory-artifacts commit. Additionally: `find "$FACTORY_ROOT" -name red-gate-log.md` resolves to wrong cycle (14 candidates in factory-artifacts; none is S-21.07 governing file). Requires architect ADR-040 v1.2 redesign + human re-ratification. Route: architect. |
| **[P0] `validate-factory-path-staging` WASM guard inert since 2026-07-23** | **OPEN** | 0 fires vs 889 sibling invocations; `on_error = "continue"` masks absence. Fix story: S-21.09. MUST land before S-21.07. |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67..74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754. |
| **TD-VSDD-061/062/063** | OPEN 2026-05-17/19 | validate-index-cite-refresh large-file fail-open; schema inconsistencies; VP allocation deferred. |
| **TD-VSDD-095..100** | CODIFIED-AND-FORWARDED | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101** | OPEN 2026-05-18 — anchored S-15.15 | VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | check_index_sites + rows_after_heading advisory-arm defects. |
| **[D-945] ADR-035 §Decision 5 fuel budget advisory** | **DISCHARGED D-964** | Discharged by ADR-042 §Decision 4. S-21.07 benchmarks are the measurements it awaited. |
| **[D-945] VP-102..VP-118 pending allocation** | DEFERRED — anchored S-21.07 post-merge | 17 VPs per BC-5.39.010 §VP Anchors. |
| **[D-952] compute-input-hash operator cache binary divergence** | BOOTSTRAP-MIGRATED — anchored rc.24 #715 | Self-heals at rc.24. |
| **[D-953] 27 unparseable frontmatter files** | OPEN 2026-08-04 | Needs remediation story. |
| **[D-953] ADR-037 19-story volatile-inputs remediation** | OPEN 2026-08-04 | S-19.01 CRITICAL. |
| **[D-954] Duplicate T-038 test ID** | OPEN 2026-08-04 | POLICY 1 violated; anchor pass-11 or next fix burst. |
| **[D-954] decision-log.md >14,800 lines** | OPEN 2026-08-04 | WASM validators time out on every edit. |
| **[D-955] 8 Dependabot vulnerabilities** | OPEN 2026-08-04 | Anchor: next maintenance sweep. |
| **[D-957] F-S2107-P7-019 D-693 stale WASM size** | OPEN 2026-08-05 | Anchor: pass-11 fix burst or next SHA-patch. |
| **[D-958] 60 of 158 stories lack tdd_mode** | OPEN 2026-08-06 | Anchor: S-15.03 PRIORITY-A. |
| **[D-958] GATE DEFECT: validate-pr-review-posted + validate-changelog-monotonicity** | OPEN 2026-08-06 | Paper-gate; header-skip misread. |
| **[D-961] SEC-001 + RUSTSEC-2026-0222/0204 + 7 Dependabot + EAC-002 + ADR-033** | OPEN 2026-08-07 — SECURITY | E-22 scope re-anchored to E-21 W4. |
| **[D-961] fix/nested-factory-path-derivation + fix/d999-sentinel-code-migration** | RESOLVED 2026-08-09 — both pushed | Both branches pushed to origin (SESSION-WRAP-2026-08-09); team-lead decides PR/merge path. |
| **[D-962] F-S2107-P9-002 fuel-exhaustion gate impl** | **CLOSED D-964** | ADR-042 10M→20M cap; gate impl `5370db80`; S-21.13 W7 anchor for §Decision 5. |
| **[D-962] F-S2107-P9-003 POLICY 15 gate → 0** | **CLOSED D-964** | ADR-040 §Decision 6 conditional pre-check + line-anchored predicate; 3 attestation sections at `5370db80`. |
| **[D-963] ADR-035 §Decision 5 quadratic not observed** | OPEN 2026-08-08 | Direct measurement linear R²=0.998790. Route: architect at next ADR-035 touch. |
| **[D-963] BC-5.39.010 live-operation silent exhaustion gap** | OPEN 2026-08-08 | plugin.timeout exits 0/empty; live agents receive no signal. Bats-only margin gate insufficient. Anchor: S-21.07 + margin gate implementation. |
| **[D-964] policies.yaml v1.4.22 PROPOSED** | **RESOLVED 2026-08-09 (D-965)** | ADR-040 §Decision 6 replacement text ratified by human 2026-08-09; policies.yaml v1.4.22 active; ADR-040 v1.1 active. |
| **[D-964] fix/fuel-cap-raise-20m NOT YET EFFECTIVE** | OPEN 2026-08-10 (D-968) — release-gated | Fix is now **on develop** (`62fbcf1a`); operator cache `1.0.0-rc.23` still embeds 10M; requires rc.24 release to become effective in hook chain. Events continue. |
| **[SESSION-WRAP-2026-08-09] Dispatcher log deletion recurrence — 3 occurrences** | OPEN 2026-08-09 — root cause unestablished | Three unexplained working-tree deletions of tracked dispatcher-internal logs in a single session. Restored each time via `git restore`. Nested `.factory/.factory` confirmed (2 log files); plausible mechanism per O-S2107-P10-02; causal link unconfirmed. Anchor: next maintenance sweep + post-release of fix/nested-factory-path-derivation. |
| **[D-966] F-002 retroactive-attestation (permanent)** | OPEN 2026-08-09 — permanent historical violation | 67ffbdcc + 38c70f9e lack "at that commit" attestation (retroactively added at 5370db80); history immutable; note in ADR-040 v1.2 redesign context. Route: architect. |
| **[D-966] F-005 ADR-041/ADR-042 status-field drift** | OPEN 2026-08-09 | ADR-041 + ADR-042 frontmatter `status: proposed`; pipeline assets assert "ratified"; POLICY 16 gate governed by proposed ADR-041. Route: architect. |
| **[D-966] F-006 ADR-042 §Decision 1 vs §Decision 2 self-contradiction** | OPEN 2026-08-09 (D-967 correction: adapter threshold `725,832` confirmed; rc.24 fuel-cap raise INSUFFICIENT for large cycle artifacts; size budgets + compaction are load-bearing fix) | §Decision 1 row-4 claims independent per-plugin budgets; §Decision 2 declares global raise only; POLICY 13 BOUNDARY-POLARITY mutant absent; re-ratification required. Route: architect. |
| **[D-966] F-007 ADR-042 §Decision 3 class (b) unanchored dispatch** | **CLOSED D-968 (2026-08-10)** | `extract_reason_from_outcome` fuel/epoch disambiguation shipped at `62fbcf1a`. `PluginResult::Timeout` now carries `fuel_cap` field; fuel arm emits `fail-closed: FUEL_EXHAUSTED: fuel cap of {fuel_cap} units exhausted…` distinct from epoch `fail-closed: plugin timed out`. |
| **[D-966] F-008 TD-VSDD-091 line-number pins** | OPEN 2026-08-09 | ADR-040 §Context "line 294"; ADR-042 §Empirical "BC-INDEX line 1464"; ARCH-INDEX ADR-042 row "~415KB". Route: architect. |
| **[D-966] F-009 BC-5.39.010 modified[]-erratum parity** | OPEN 2026-08-09 | Body `1.15-erratum` row has no `modified[]` entry; POLICY 14 leg-3 gap. Route: product-owner. |
| **[D-966] D-965-mischaracterization: ADR-040 §Decision 6 justifying premise false** | OPEN 2026-08-09 | 5370db80 described as "stability entry" but is docs-only attestation-provision commit; D-965 ratification = PROCURED-ON-MISCHARACTERIZATION; ADR-040 v1.2 redesign + human re-ratification required. Route: architect. |
| **[D-968] F-004 BC-5.39.010 present-perfect SHIFTED** | OPEN 2026-08-10 | "has been raised to 20,000,000 … satisfiable at HEAD" now TRUE on develop (`62fbcf1a`) but ambiguous referent: operator cache `1.0.0-rc.23` still has 10M cap. Defect = ambiguous "HEAD" (unqualified). Route: product-owner to clarify referent to post-`62fbcf1a` develop-HEAD. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-10 — D-968-PR-774-POST-MERGE-RECORD-BURST COMPLETE; trajectory-tail →20→16→8→10; pass-11 adversary NEXT; PIPELINE ACTIVE)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. Adversary **pass-10 COMPLETE — NOT-CLEAN (B2/H4/M3/L1; 10 findings).** D-966 record burst, D-967 correction burst, D-968 post-merge burst all persisted. Streak **0/3** (reset by NOT-CLEAN). Nine adversary passes total, zero CLEAN verdicts; pass-11 adversary is NEXT. Trajectory `47→18→25→25→24→20→16→8→10`; tail LENGTH=4 `→20→16→8→10`. factory-artifacts parent `48cb6862` (D-968 single commit pending push). 4-INDEX: **BC v4.55 / VP v2.76 / STORY v4.291 / ARCH v3.52** (UNCHANGED). `policies.yaml` v1.4.22. develop `62fbcf1a` (PR #774 merged).

**CRITICAL:** D-965 ratification = **PROCURED-ON-MISCHARACTERIZATION** (F-003). ADR-040 §Decision 6 justifying premise false. Architect must re-open ADR-040 for v1.2 redesign before pass-11 adversary reviews POLICY 15 again.

### §2 Convergence

Streak **0/3**. Nine adversary passes, ZERO CLEAN verdicts. Trajectory `47→18→25→25→24→20→16→8→10`; trajectory-tail →20→16→8→10. Dominant pattern: **vacuous-gate** — 2 BLOCKERs are gate-domain-mismatch class. F-007 CLOSED by `62fbcf1a` (fuel/epoch disambiguation). F-004 SHIFTED (ambiguous referent). 9 pass-10 findings still OPEN. pass-11 adversary NEXT on resume.

### §3 Branch States (ALL PUSHED)

| Branch | SHA | Note |
|--------|-----|------|
| `feature/S-21.07-validate-cross-site-correspondence` | `5370db80` | pushed; pass-10 NOT-CLEAN 10 findings D-967 (correction burst complete; F-010 added) |
| `feature/S-21.04-story-worktree-write-path-discipline` | `323f440f` | pushed; 30 passes / 0 CLEAN, pass-31 pending |
| `fix/nested-factory-path-derivation` | `9afc3226` | pushed; F-S2107-P8-016 (3 sites) + P9-008 |
| `fix/d999-sentinel-code-migration` | `bf642fd9` | pushed; ADR-041, 8 sites |
| `fix/fuel-exhaustion-fail-loud` | `fbb9dcb6` | **ABANDONED, local-only, deliberately NOT pushed** |
| `factory-artifacts` | `48cb6862` | D-968-PR-774-POST-MERGE-RECORD-BURST parent SHA (D-967 commit; actual D-968 commit TBD after push) |
| `develop` | `62fbcf1a` | PR #774 merged 2026-08-10; local main worktree stale — pull on next code resume |
| `main` | `80e5cd7b` | rc.23 bot binary bundle 2026-07-18 |

No PRs open. CI does not fire on feature-branch pushes.

### §4 CRITICAL — Fuel Fix ON DEVELOP, NOT YET IN OPERATOR CACHE

`fix/fuel-cap-raise-20m` MERGED (`62fbcf1a`). `DEFAULT_FUEL_CAP = 20_000_000` is now on develop. **The fix takes effect only after rc.24 release cross-compiles the bundle** — bundled binaries and operator cache at `1.0.0-rc.23` still embed 10M. Hook validation chain silently non-functional on large-file `.factory/` writes until release. Every PostToolUse `fail-closed: FUEL_EXHAUSTED:` during this burst is an instance. **Cut a release first on resume.**

### §5 Resume Order

1. **Cut a release (rc.24)** — makes the fuel fix effective; stops ~1,200/day validation-skips. Gated: F-006 size budgets (decision-log.md/burst-log.md/lessons.md exhaust 20M at 1.8–2.6× — compaction needed first or simultaneously).
2. **Architect: ADR-040 v1.2 redesign** — F-001 (BLOCKER: gate domain empty) + F-003 (D-965 ratification procured-on-mischaracterization). Gate must fire in code-repo context, not factory-artifacts. Negative-control demonstration required (META-LEVEL-25 codification 1, PROPOSED). Human re-ratification required after redesign.
3. **Architect: ADR-041/042** — F-005 (frontmatter proposed→active); F-006 (§Decision 1 vs §Decision 2 self-contradiction + POLICY 13 mutant); F-008 (TD-VSDD-091 line pins). F-007 CLOSED — no longer needs implementer action.
4. **Product-owner: BC-5.39.010** — F-004 (clarify "HEAD" referent to post-`62fbcf1a` develop; note rc.23 operator cache still 10M); F-009 (modified[] -erratum parity or convention codification).
5. **Pass-11 adversary** — fresh context, Iron Law (D-959). Nine passes, zero CLEAN.

### §6 Pending Human Decisions

1. **POLICY 15 re-ratification** — after architect ADR-040 v1.2 redesign, new POLICY 15 v1.4.23 must go to human with negative-control demonstration (META-LEVEL-25).
2. **META-LEVEL-25 and POLICY 22 ratification-channel codifications** — both PROPOSED in D-966; require human ratification before applying to policies.yaml.
3. **Merge sequencing** — recorded order: S-21.12 → S-21.09 → S-21.07. S-21.12 and S-21.09 not yet built.
4. **E-22 security scope sequencing** — SEC-001, RUSTSEC-0222/0204, 7 Dependabot, EAC-002, ADR-033.
5. **fix/nested + fix/d999 delivery path** — both pushed; team-lead decides PR/merge path.
6. **rc.24 gating decision** — F-006 confirms 20M cap is insufficient for 3 cycle artifacts; compaction or size reduction needed before release is clean.

### §7 Open Items

- **S-21.13** (E-21 W7, `depends_on: [S-21.10, S-21.11]`) — human-directed deferral anchor for ADR-042 §Decision 5 `read_file_range`. Do NOT convert to tech-debt-register entry.
- **`validate-index-cite-refresh`** — cannot use regex even at 20M (structural constraint orthogonal to payload size).
- **Dispatcher log deletion recurrence** — nested `.factory/.factory` still active (D-968 burst: `?? .factory/logs/dispatcher-internal-2026-08-06.jsonl` in factory-artifacts git status); plausible hypothesis per O-S2107-P10-02; causal link UNCONFIRMED. Anchor: next maintenance sweep post-release of fix/nested-factory-path-derivation.
- **ADR-043 candidate** — architect to formalize burst-log Block 8 parent-SHA convention (codification 3, applied by convention at D-966).

### §8 Cautions

- Do NOT run `/rehydrate-wave` — wave-state points at a closed epic.
- Do NOT run `compute-input-hash --scan --update` — 418-file blast radius per D-936; single-file only.
- Use explicit refspecs for pushes: `git push origin HEAD:<branch>`.
- Main worktree sits on `fix/nested-factory-path-derivation` @ `9afc3226`. On next code resume: `git checkout develop && git pull`.
- `.claude/scheduled_tasks.lock` (M) and `plugins/vsdd-factory/tests/report.tap` (??) are harness artifacts — leave them.
- fuel cap fix (`62fbcf1a`) is on develop but NOT YET in operator cache — hook chain still fuel-limited until rc.24 release.
- D-965 ratification = PROCURED-ON-MISCHARACTERIZATION per F-003 — do NOT treat POLICY 15 v1.4.22 as correctly designed; architect must re-open ADR-040 before any POLICY 15 compliance work.

### §9 Resume Command

`/vsdd-factory:next-step`.
