---
document_type: pipeline-state
level: ops
version: "6.74"
status: draft
producer: state-manager
timestamp: 2026-07-29T23:45:00Z
phase: D-943-PASS-28-RECORD-BURST
last_amended: "2026-07-29 (v6.74) — D-943-PASS-28-RECORD-BURST (state-manager): pass-28 record + fix burst; adversary-pass-28.md CREATED; D-942+D-943 codified; 3 L-BB lessons; policies.yaml B01 FIXED (v1.4.17→v1.4.18); STORY-INDEX v4.272→v4.273; BC-INDEX v4.37→v4.38; VP-INDEX v2.72→v2.73; VCM v1.8→v1.9; specialist files BC-5.39.008 v1.6/BC-6.26.001 v1.17/VP-097 v1.6; trajectory-tail →17→11→7→17; STATE.md v6.73→v6.74; SRC FULL REPLACEMENT. 4-INDEX: BC v4.38/VP v2.73/STORY v4.273/ARCH v3.37. [Prior: 2026-07-29 (v6.73) — D-941-SESSION-WRAP-PAUSED (state-manager): session-wrap burst — pipeline PAUSED per human /wrap; SRC FULL REPLACEMENT (D-930 checkpoint archived); lesson L-BB-pipeline-field-no-liveness-signal appended; Current Phase Steps D-941 row; POLICY 16: D-940 max→D-941 allocated; POLICY 16 gate stdout: 13760:## D-938 / 13860:## D-939 / 13953:## D-940 (numeric sort). 4-INDEX UNCHANGED: BC v4.37 / VP v2.72 / STORY v4.272 / ARCH v3.37. [Prior history compacted 2026-07-29 (D-941 burst); full chain SoT: decision-log.md D-862 compaction note + STATE.md Decisions Log table below for per-decision summary chain.]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
"D-943-PASS-28-RECORD-BURST 2026-07-29. pass-28 record + fix burst complete (Commits A–E); NEXT: pass-29 adversary dispatch."
current_step: "D-943-PASS-28-RECORD-BURST 2026-07-29 (POLICY 16 GLOBAL-MAX GATE: grep -n \"^## D-\" .factory/cycles/v1.0-brownfield-backfill/decision-log.md | sort -t'-' -k2 -n | tail -3 → 13953:## D-940 / 14008:## D-942 / 14042:## D-943; D-940 confirmed prior max → D-942+D-943 allocated). pass-28 record + fix burst COMPLETE: Commit A=8071cb1b (adversary-pass-28.md + INDEX.md row + Convergence Status); Commit B=541c278b (D-942+D-943 + 3 L-BB lessons); Commit C=79403c0b (policies.yaml B01 FIXED v1.4.17→v1.4.18); Commit D=e6060f8e (4-index bumps: BC v4.37→v4.38 / VP v2.72→v2.73 / STORY v4.272→v4.273 / ARCH v3.37 UNCHANGED; specialist files BC-5.39.008 v1.6/BC-6.26.001 v1.17/VP-097 v1.6; VCM v1.8→v1.9); Commit E=this-commit (STATE.md body + burst-log completion). develop 948f0fb1; main 80e5cd7b UNCHANGED. trajectory-tail →17→11→7→17. streak 0/3. NEXT: pass-29 adversary dispatch. parent-commit: e6060f8e (Commit D)."
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

  D-430(a) compaction history D-532..D-808 (see decision-log.md for full range) COLLAPSED 2026-07-12 (range 393..500 lines; 49+ entries archived; full per-burst notes in factory-artifacts git log; SoT: git show 903aa863:.factory/STATE.md for D-828 pre-compaction state).
  D-862 (see decision-log.md) frontmatter `last_amended` nested-quote chain (D-607..D-861 (see decision-log.md for full range)) COMPACTED 2026-07-20; Current Phase Steps + Decisions Log also compacted to last-5 policy same burst; full chain SoT: decision-log.md + STATE.md Decisions Log table + cycles/v1.0-brownfield-backfill/session-checkpoints.md.
  351 lines (wc-l post-update; D-890 W1-WAVE-GATE-BOOKKEEPING-FIX 2026-07-24; v6.26→v6.27; D-421(c)-class reconcile: un-swept since D-862 compaction 2026-07-20; soft-target margin 415-351=+64 UNDER-SOFT-TARGET)
  ~255 lines (estimated post-D-943 PASS-28-RECORD-BURST 2026-07-29; v6.73→v6.74; soft-target margin 415-255=+160 WELL-UNDER-SOFT-TARGET)
  D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20; v6.08→v6.09; SM governance/dispatch burst; human gate decision W1 APPROVED SEQUENTIAL; input-drift resolved (12 files, metadata-only); STATE.md stale-points corrected (W1 15pts→17pts, per-story values 5/5/5/6/6→11/3/3/5/5); frontmatter last_amended COMPACTED + Current Phase Steps/Decisions Log compacted to last-5 policy (this compaction is the size-budget remediation flagged OVER-SOFT-TARGET at D-861); PIPELINE UNPAUSED; 4-index ALL UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. Full per-burst wc-l history D-819..D-861 (see decision-log.md for full range) archived; SoT: decision-log.md + git show 9debd920:.factory/STATE.md for D-861 pre-compaction state.
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
| **Last Updated** | 2026-07-29 — D-943-PASS-28-RECORD-BURST; pass-28 record + fix burst COMPLETE (Commits A–E); trajectory-tail →17→11→7→17; 4-INDEX BC v4.38/VP v2.73/STORY v4.273/ARCH v3.37; STATE.md v6.73→v6.74. [Prior: 2026-07-29 — D-941-SESSION-WRAP-PAUSED; PIPELINE PAUSED; SRC FULL REPLACEMENT; trajectory-tail →6→17→11→7; 4-INDEX UNCHANGED BC v4.37/VP v2.72/STORY v4.272/ARCH v3.37; STATE.md v6.72→v6.73. [Prior: 2026-07-20 — D-862-E21-PHASE-3-W1-DISPATCH-APPROVED; PIPELINE UNPAUSED; trajectory-tail →0→0→0→0. [Prior: D-607..D-861 (see decision-log.md for full range) chain in Decisions Log.]]] |
| **Current Phase** | **D-943-PASS-28-RECORD-BURST 2026-07-29. develop 948f0fb1. main 80e5cd7b. merged_count 107. 4-index BC v4.38/VP v2.73/STORY v4.273/ARCH v3.37. trajectory-tail →17→11→7→17. PIPELINE ACTIVE. pass-28 record + fix burst COMPLETE (Commits A–E). feature/S-21.04 @ c7c61688 pushed, NO PR. streak 0/3. NEXT: Pass-29 adversary dispatch (human directive: grind to 3 clean).** |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B..D-647 ALL COMPLETE/ARCHIVED: Waves 1-11, rc.11..rc.20, E-10 SEALED D-531, E-17 3/3, S-15.03/S-15.17, F2 passes 1-43 CONVERGED D-606, F3 integration, F4 W1-W2 CONVERGED D-647 | **ALL COMPLETE / ARCHIVED** | git show 903aa863:.factory/STATE.md Phase Progress for pre-compaction 18-row table. SoT: decision-log.md + burst-log.md. |
| v1.0.0-rc.20 SHIPPED 2026-06-01; v1.0.0-rc.21 SHIPPED 2026-06-13; v1.0.0-rc.22 SHIPPED 2026-07-03 | **ALL SHIPPED** | PRs merged; marketplace published; v1.0.0-rc.22 tag `e4285fe5`. |
| D-856 RC23-SHIPPED 2026-07-18: v1.0.0-rc.23 SHIPPED; PR #688 --merge 45050445; recovery PR #689 (2 WASMs git add -f + bats pre-build); retag at 0f8b2a89; second pipeline run 29660640970 all 10 PASS; bot commit 80e5cd7b; POLICY 20 34/34 WASMs; marketplace claude-mp#18 MERGED 2026-07-18T22:48:17Z; RELEASE-GATE BLOCKER CLOSED; STATE.md v6.02→v6.03 | **SHIPPED** | GitHub Release v1.0.0-rc.23 (prerelease); marketplace vsdd-factory 1.0.0-rc.23 |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20: human gate decision resumed from D-861 pause; E-21 Phase-3 W1 dispatch APPROVED SEQUENTIAL (S-21.01→S-21.02→S-21.03); E-20 DEFERRED reconfirmed; pre-Phase-3 input-drift resolved; STATE.md stale-points corrected; frontmatter last_amended COMPACTED; PIPELINE UNPAUSED; 4-index ALL UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11; STATE.md v6.08→v6.09 | **COMPLETE** | PIPELINE UNPAUSED — S-21.01..S-21.04 delivered; see Current Phase Steps |
| D-941 SESSION-WRAP-PAUSED 2026-07-29: S-21.04 pass-27 CLOSED (D-940); streak 0/3; PIPELINE PAUSED post-D-940; SRC FULL REPLACEMENT; 4-INDEX UNCHANGED BC v4.37/VP v2.72/STORY v4.272/ARCH v3.37; trajectory-tail →6→17→11→7 | **COMPLETE** | PIPELINE PAUSED (session wrap) — D-943 burst follows |
| D-943 PASS-28-RECORD-BURST 2026-07-29: pass-28 record + fix burst COMPLETE (Commits A–E); policies.yaml B01 FIXED (v1.4.17→v1.4.18); BC-INDEX v4.37→v4.38; VP-INDEX v2.72→v2.73; STORY-INDEX v4.272→v4.273; VCM v1.8→v1.9; specialist files BC-5.39.008 v1.6/BC-6.26.001 v1.17/VP-097 v1.6; trajectory-tail →17→11→7→17; streak 0/3 (28 passes) | **COMPLETE** | PIPELINE ACTIVE — NEXT: Pass-29 adversary dispatch |
| **E-18 CAP-002 context-durability epic (#173): waves 1-9 + prereqs, S-18.00..S-18.14, 18 stories** | **EPIC COMPLETE 2026-07-01 D-744** | Final story S-18.12 MERGED PR #384 ec05606a. All 18 E-18 stories + 2 prereqs merged; merged_count 95→96. Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`. |

## Current Phase Steps

> **Rows before D-836 archived to** `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md` per STATE.md content-routing rules (keep last 5 only; compacted D-862 2026-07-20).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-807..D-835 (see decision-log.md for full range) (archived) | state-manager | ARCHIVED | See `cycles/v1.0-brownfield-backfill/burst-log.md`. D-807..D-823: passes 51-61 CONVERGED. D-824..D-825: POLICY 18 bookkeeping + W1-TDD-DISPATCHED. D-826..D-835: W1-reconcile+merge+governance. |
| D-836..D-857 (see decision-log.md for full range) (W2/W3 delivery+merge+governance bursts; archived 2026-07-20 D-862 compaction) | state-manager | ARCHIVED | S-19.04..S-19.09 W2/W3 delivery + convergence + merges; E-19 wave-gate W1-W2+W3-epic closure; rc.23 smoke-test + ship record; D-857 SESSION-WRAP-PAUSED. |
| D-858 BACKLOG-TRIAGE-ARC-2026-07-19 | state-manager | COMPLETE | OPERATOR-INSTALL VERIFIED; 8 fix PRs MERGED (#524..#532); PR #691 fix/go-stdin MERGED → 6444ac23; develop 584b0518→6444ac23. STATE.md v6.04→v6.05. |
| D-860 E21-REGISTRATION-AND-SPEC-CONVERGENCE-2026-07-19 | state-manager | COMPLETE | BC-INDEX v4.10→v4.11 (5 new BCs + BC-6.10.002 update; total_bcs 1,977→1,982); e-21-spec-convergence.md CREATED (11-pass 3-CLEAN P9/P10/P11); D-860 codified + 4 lessons; 5 E-21 stories added sprint-state. STATE.md v6.06→v6.07. |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20 (governance/dispatch burst; SM this-commit) | state-manager | COMPLETE | Human gate decision W1 APPROVED SEQUENTIAL (S-21.01→S-21.02→S-21.03); E-20 DEFERRED; pre-Phase-3 input-drift resolved (12 files, metadata-only); STATE.md stale-points corrected (W1 17pts); frontmatter last_amended COMPACTED + Current Phase Steps/Decisions Log compacted; PIPELINE UNPAUSED. 4-index ALL UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. |
| D-941 SESSION-WRAP-PAUSED 2026-07-29 (session-wrap burst; SM this-commit) | state-manager | COMPLETE | (1) POLICY 16 GLOBAL-MAX GATE: `grep -n "^## D-" .factory/cycles/v1.0-brownfield-backfill/decision-log.md \| sort -t'-' -k2 -n \| tail -3` → 13760:## D-938 / 13860:## D-939 / 13953:## D-940; D-940 confirmed max → D-941 allocated. (2) PIPELINE PAUSED: human /wrap directive 2026-07-29. (3) D-930 checkpoint archived. (4) STATE.md v6.72→v6.73. (5) L-BB-pipeline-field-no-liveness-signal appended. (6) 4-INDEX UNCHANGED: BC v4.37/VP v2.72/STORY v4.272/ARCH v3.37. trajectory-tail →6→17→11→7. parent-commit: c95eda57. |
| D-943 PASS-28-RECORD-BURST 2026-07-29 (pass-28 record + fix burst; SM this-commit) | state-manager | COMPLETE | (1) POLICY 16 GLOBAL-MAX GATE: `grep -n "^## D-" .factory/cycles/v1.0-brownfield-backfill/decision-log.md \| sort -t'-' -k2 -n \| tail -3` → 13953:## D-940 / 14008:## D-942 / 14042:## D-943; D-940 confirmed max → D-942+D-943 allocated. (2) pass-28 record burst COMPLETE: Commit A=8071cb1b; Commit B=541c278b; Commit C=79403c0b; Commit D=e6060f8e; Commit E=this-commit. (3) 4-INDEX: BC v4.38/VP v2.73/STORY v4.273/ARCH v3.37. trajectory-tail →17→11→7→17. streak 0/3. parent-commit: e6060f8e (Commit D). |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,982 (BC-INDEX v4.11 D-860; ADD BC-4.16.001/BC-5.43.001/BC-5.44.001/BC-6.26.001/BC-6.27.001 + UPDATE BC-6.10.002; decision-log.md SoT) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.53 D-756; VP-094..101 NEW D-753; decision-log.md SoT) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 129 file-resident + 15 stub IDs (STORY-INDEX v4.152 D-773; decision-log.md SoT) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 20 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 30 (ADR-030 v1.0 D-754 NEW; ADR-025 v1.10 D-762; ADR-029 D-694) |
| Merged Count | merged_count | `stories/sprint-state.yaml` (canonical predicate); `STATE.md` (explicit counter) | 107 (STATE.md explicit counter as of D-851; sprint-state predicate-based count: 113; canonical definition codified D-853) |

## Story Status

128 file-resident + 15 unauthored stub IDs = 143 stories registered. E-18 EPIC COMPLETE D-744 2026-07-01. Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`.

- **Merged (107):** S-19.07 MERGED PR #670 6db4c9fc D-851 (E-19 COMPLETE 9/9). S-19.09 MERGED PR #659 13ece92c D-848. S-19.06 MERGED PR #657 9787c056 D-843 (W2 COMPLETE). S-19.08 MERGED PR #646 1304d280 D-842. S-19.05 MERGED PR #640 7b35c8e4 D-841. S-19.04 MERGED PR #639 d4a23a02 D-841. S-19.03 MERGED PR #611 091ce499 D-834. S-19.01 MERGED PR #613 8d1721f7 D-833. S-19.02 MERGED PR #610 f5ea12e9 D-832. Also S-17.01..S-17.04 + S-18.00..S-18.14 (E-18 EPIC COMPLETE). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **E-21 (SPEC CONVERGED — Phase-3 W1 dispatch APPROVED D-862; SEQUENTIAL S-21.01→S-21.02→S-21.03):** S-21.01 (W1, P0, 11pts, draft, CAP-034, issue #342); S-21.02 (W1, P1, 3pts, draft, CAP-035, issue #365); S-21.03 (W1, P1, 3pts, draft, CAP-038, issue #358); S-21.04 (W2, P1, 5pts, draft, CAP-036, issue #523); S-21.05 (W2, P1, 5pts, draft, CAP-037, issue #588). Total: 5 stories, 27pts.
- **Draft (30 file-resident):** S-4.11; S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle commit 2026-07-18 |
| develop | 948f0fb1 | feat(S-21.04): pass-27 CLOSED D-940 |
| factory-artifacts | see `git -C .factory log -1` | D-943-BURST this commit (pushed 2026-07-29). Prior: D-941-BURST. NOTE: current HEAD not self-cited per TD-VSDD-053. |
| feature/S-21.04 | c7c61688 | feat(S-21.04): pass-28 CLOSED; cascade in progress; NO PR open (correct: mid-cascade); pushed clean 2026-07-29 |
| feature/S-21.01 | **DELETED** (was merged) | MERGED via PR #759 2026-07-23 (D-879); branch deleted at merge |
| feature/S-21.02 | **DELETED** (was merged) | MERGED via PR #760 2026-07-24 (D-880); branch deleted at merge |
| feature/S-21.03 | **DELETED** (was merged) | MERGED via PR #761 2026-07-24 (D-881); branch deleted at merge |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE (marketplace PR claude-mp#18 MERGED 2026-07-18T22:48:17Z) |
| v1.0.0-rc.22 (tag) | e4285fe5 | SHIPPED 2026-07-03; FULLY IN OPERATOR MARKETPLACE |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-943-PASS-28-RECORD-BURST 2026-07-29. PIPELINE ACTIVE. develop 948f0fb1; main 80e5cd7b; merged_count 107; BC-INDEX v4.38; VP-INDEX v2.73; STORY-INDEX v4.273; ARCH-INDEX v3.37; streak 0/3 (28 passes); trajectory-tail →17→11→7→17. pass-28 CLOSED (B01 FIXED; H01-H07/M01-M07/L01-L02 CLOSED; T-010/RG-010 registered). feature/S-21.04 @ c7c61688 pushed, NO PR. NEXT: Pass-29 adversary dispatch (human directive: grind to 3 clean).** | D-943 PASS-28-RECORD-BURST 2026-07-29; D-942 ORCHESTRATOR-P0-REFUTED; D-941 SESSION-WRAP-PAUSED 2026-07-29; D-940 pass-27 CLOSED; D-939 pass-26 CLOSED; D-938 pass-25 CLOSED (regression; streak reset); D-937 pass-24 CLOSED; D-936 pass-23 CLOSED; D-935 pass-22 CLOSED; D-927 D-931 falsification burst; D-931..D-934 (see decision-log.md for full range) passes 21→22 arc; D-863..D-930 (see decision-log.md for full range). |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. All archived per D-430(a) compaction bursts. D-607..D-943 (see decision-log.md for full range): this Decisions Log + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-943 | PASS-28-RECORD-BURST. Single-commit burst structure (TD-VSDD-053) 2026-07-29. (1) POLICY 16 GLOBAL-MAX GATE: `grep -n "^## D-" .factory/cycles/v1.0-brownfield-backfill/decision-log.md \| sort -t'-' -k2 -n \| tail -3` → 13953:## D-940 / 14008:## D-942 / 14042:## D-943; D-940 confirmed prior max → D-942+D-943 allocated. (2) adversary-pass-28.md CREATED: B01 policies.yaml YAML-parse BLOCKED (three invalid `\+` escapes in double-quoted scalars; passes 21–28 rubric blind to POLICY 13–22 auto-loaded portion); H01-H07 (7 HIGH); M01-M07 (7 MEDIUM); L01-L02 (2 LOW); T-010+RG-010 registered. (3) D-942(b)+(c): orchestrator P0 root-cause analysis (policies.yaml escaping) REFUTED — root cause spec-ambiguity in EC-006/DI-001/DI-006, not escaping. (4) policies.yaml v1.4.17→v1.4.18: policy_16+policy_18 regex fix. (5) 4-INDEX: BC v4.37→v4.38; VP v2.72→v2.73; STORY v4.272→v4.273; ARCH v3.37 UNCHANGED. trajectory-tail →17→11→7→17. streak 0/3 (28 passes). parent-commit: e6060f8e (Commit D). | D-943-PASS-28-RECORD-BURST 2026-07-29; pass-28 record burst COMPLETE; policies.yaml B01 FIXED; 4-index BC v4.38/VP v2.73/STORY v4.273/ARCH v3.37; streak 0/3 | D-943-PASS-28-RECORD-BURST | 2026-07-29 |
| D-942 | ORCHESTRATOR-P0-REFUTED. Root-cause claim (orchestrator P0) REFUTED. (b) Orchestrator claimed policies.yaml regex escaping was root cause of B01 block; REFUTED — root cause is spec-ambiguity in EC-006/DI-001/DI-006 (what constitutes a "factory path"), not escaping. (c) Orchestrator claimed spec update obligation; REFUTED — spec is correct; pass-29 adversary can examine the unchanged spec. Resolution: policies.yaml v1.4.17→v1.4.18 fixes the mechanical regex; no spec change required. | ORCHESTRATOR-P0-REFUTED 2026-07-29; policies.yaml v1.4.17→v1.4.18 FIXED; D-942(b)+(c) refutations recorded | D-942-ORCHESTRATOR-P0-REFUTED | 2026-07-29 |
| D-941 | SESSION-WRAP-PAUSED. Single-commit session-wrap burst (TD-VSDD-053) 2026-07-29. (1) POLICY 16 GLOBAL-MAX GATE: `grep -n "^## D-" .factory/cycles/v1.0-brownfield-backfill/decision-log.md \| sort -t'-' -k2 -n \| tail -3` → 13760:## D-938 / 13860:## D-939 / 13953:## D-940; D-940 confirmed max → D-941 allocated. (2) PIPELINE PAUSED: human /wrap directive 2026-07-29; NOTE: `pipeline:` field STALE through D-931..D-941 (see decision-log.md for full range); lesson L-BB-pipeline-field-no-liveness-signal recorded. (3) D-930 checkpoint archived to session-checkpoints.md. (4) STATE.md v6.72→v6.73: last_amended COMPACTED. (5) L-BB-pipeline-field-no-liveness-signal appended to lessons.md. (6) 4-INDEX UNCHANGED: BC v4.37/VP v2.72/STORY v4.272/ARCH v3.37. trajectory-tail →6→17→11→7. parent-commit: c95eda57. | SESSION-WRAP-PAUSED 2026-07-29; S-21.04 pass-27 CLOSED (D-940); streak 0/3; PIPELINE PAUSED post-D-940; SRC FULL REPLACEMENT; STATE.md v6.72→v6.73; 4-index UNCHANGED BC v4.37/VP v2.72/STORY v4.272/ARCH v3.37 | D-941-SESSION-WRAP-PAUSED | 2026-07-29 |
| D-862 | E21-PHASE-3-W1-DISPATCH-APPROVED. Human gate decision W1 APPROVED SEQUENTIAL (S-21.01→S-21.02→S-21.03); E-20 DEFERRED reconfirmed; pre-Phase-3 input-drift resolved (12 files, metadata-only per D-824); STATE.md stale-points corrected (W1 17pts); frontmatter last_amended COMPACTED; PIPELINE UNPAUSED. 4-index ALL UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. parent-commit: c5604f7f. | E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20; W1 APPROVED SEQUENTIAL; E-20 DEFERRED; input-drift resolved; PIPELINE UNPAUSED; 4-index UNCHANGED | D-862-E21-PHASE-3-W1-DISPATCH-APPROVED | 2026-07-20 |
| D-861 | SESSION-WRAP-PAUSED. PIPELINE PAUSED human /wrap 2026-07-19 post-D-860. D-860 checkpoint archived. STATE.md v6.07→v6.08. 4-INDEX UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. parent-commit: 23905b66. | SESSION-WRAP-PAUSED 2026-07-19; PIPELINE PAUSED; SRC FULL REPLACEMENT; 4-index UNCHANGED | D-861-SESSION-WRAP-PAUSED | 2026-07-19 |
| D-860 | E21-REGISTRATION-AND-SPEC-CONVERGENCE-2026-07-19. BC-INDEX v4.10→v4.11 (5 new BCs + BC-6.10.002 update; total_bcs 1,977→1,982); e-21-spec-convergence.md CREATED (11-pass 3-CLEAN P9/P10/P11); D-860 codified + 4 lessons; 5 E-21 stories sprint-state. 4-INDEX: BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. parent-commit: 2e4b8f89. | E21-REGISTRATION-AND-SPEC-CONVERGENCE 2026-07-19; BC v4.10→v4.11; 5 new BCs; e-21-spec-convergence.md CREATED | D-860-E21-REGISTRATION-AND-SPEC-CONVERGENCE-2026-07-19 | 2026-07-19 |
| D-859 | CLOSE-THE-LOOP-ISSUE-SWEEP-2026-07-19. 3 issues closed (#418/#472/#465); 5 partial-progress → E-20 roster; 15 adjacent-only. 4-INDEX UNCHANGED BC v4.10/VP v2.72/STORY v4.219/ARCH v3.06. parent-commit: f71129c8. | CLOSE-THE-LOOP-ISSUE-SWEEP 2026-07-19; 3 issues closed; 4-index UNCHANGED | D-859-CLOSE-THE-LOOP-ISSUE-SWEEP-2026-07-19 | 2026-07-19 |
| D-858 | BACKLOG-TRIAGE-ARC-2026-07-19. OPERATOR-INSTALL VERIFIED; 8 fix PRs MERGED (#524..#532); PR #691 fix/go-stdin MERGED → 6444ac23; develop 584b0518→6444ac23; 2 lessons. 4-INDEX UNCHANGED. parent-commit: 36f94c24. | BACKLOG-TRIAGE-ARC 2026-07-19; OPERATOR-INSTALL VERIFIED; 8 fix PRs MERGED; develop 584b0518→6444ac23 | D-858-BACKLOG-TRIAGE-ARC-2026-07-19 | 2026-07-19 |
| D-857 | SESSION-WRAP-PAUSED. PIPELINE PAUSED human /wrap 2026-07-18 post-rc.23-ship. E-19 CLOSED D-854. Prior checkpoint ARCHIVED. 4-INDEX UNCHANGED BC v4.10/VP v2.72/STORY v4.219/ARCH v3.06. parent-commit: c91f4da2. | SESSION-WRAP-PAUSED; PIPELINE PAUSED post-rc.23-ship; E-19 CLOSED; 4-index UNCHANGED | D-857-SESSION-WRAP-PAUSED | 2026-07-18 |
| D-856 | RC23-SHIPPED. v1.0.0-rc.23 SHIPPED (PR #688+#689; second pipeline 29660640970 PASS; bot commit 80e5cd7b; POLICY 20 VERIFIED; marketplace claude-mp#18 MERGED); RELEASE-GATE BLOCKER CLOSED. 4-INDEX UNCHANGED BC v4.10/VP v2.72/STORY v4.219/ARCH v3.06. parent-commit: b1b4f27a. | RC23-SHIPPED; v1.0.0-rc.23 SHIPPED; marketplace MERGED; RELEASE-GATE BLOCKER CLOSED | D-856-RC23-SHIPPED | 2026-07-18 |
| D-413..D-855 (see decision-log.md for full range) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-07-18 |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

| Blocker | Status | Risk Statement |
|---------|--------|----------------|
| **rc.23 RELEASE-GATE BLOCKER (never-lose)** | **RESOLVED — v1.0.0-rc.23 SHIPPED 2026-07-18 D-856** | linux/windows bundled binaries now parse `[hooks.capabilities.read_prefix]` registry section (bot commit 80e5cd7b rebuilt all 5 platforms). Blocker satisfied and retired. |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67/68/69/70/71/72/74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754 compaction. decision-log.md SoT. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log `host::read_file(...65536...)` against files >64KiB → silent fail-open. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml. |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 2026-05-17/18 | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` skips production STATE.md bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` + `rows_after_heading` advisory-arm defects. |
| **test_F_P2_001 / resolver-integration timing flake (F-P3-008 class)** | **RESOLVED 2026-07-02 — PR #431 35b345f4 (D-749)** | wall-clock lower-bound replaced with InternalLog JSONL behavioral assertion. bats 1684 pass/0 fail. |
| **RUSTSEC-2026-0149** | OPEN 2026-06-11 — wasmtime-wasi HIGH | wasmtime >= 44.0.2 required; awaiting upstream compatibility. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | validate-trajectory-tail-cell-completeness stale function header. Cosmetic; next spec-touch. |
| **VP-087 DEFERRED (null-SHA hard-block VP)** | DEFERRED 2026-06-15 — D-580 observation | BC test vectors cover the null-SHA hard-block path. Create VP-087 if future adversary flags missing coverage. |
| **[D-703 drift-1] stale precompact-flush.sh ref** | **RESOLVED 2026-06-27 — PR #304 e10dedc0 (D-709)** | Tree-wide TD-VSDD-060 sibling-sweep: 8 refs across 7 files. |
| **bats-full-suite not in branch-protection required-status-checks** | OPEN 2026-06-13 — D-558 capture | `bats-full-suite (linux)` job runs but NOT in branch-protection required-checks. |
| **[D-703 drift-2] S-18.07 docs cite ADR-028 §Decision 2; §Decision 6 is broader anchor** | **CLOSED-ACCEPTED 2026-06-27 (D-709)** | ADR-028 §Decision 2 prose substantively asserts native-WASM property; citation accurate. |
| **[system-level deferral] ARCH-INDEX §Future Sections** | **RESOLVED 2026-06-16 — D-607** | verification-architecture.md + verification-coverage-matrix.md materialized per D-607. |
| **[tool-fix] compute-input-hash awk+resolver bug (D-616)** | **RESOLVED D-618** — PR #189 SQUASH-MERGED c000b06f 2026-06-16. | CWE-22 guard + awk exit-condition bug + repo-root-relative path resolution. |
| **BC-INDEX count reconcile (pre-existing) + O-2 CAP/BC-INDEX drift** | **RESOLVED 2026-06-17 — D-619** | total_bcs 1968→1972. BC-2.02.013 correctly characterized as legitimately-withdrawn audit-trail BC. |
| **S-18.08 phantom-field-removal lint gate** | DRAFT-PENDING-AUTHORING 2026-06-14 — D-563 capture | Permanent enforcement story. Anchor: E-18 epic, F3 story decomposition. |
| **[process-gap] BC-Precondition registry-block shape validator gate** | OPEN 2026-06-15 — D-576 capture | BC-4.14.001 F-P14-002 class: bare logical name in `plugin=`. Deferred to E-18 F3. |
| **[process-gap] Cross-reference title/code/phrase sweep gate** | CODIFIED D-582; UPGRADED D-589 (class CLOSED) | MECHANICAL GATE NOW MANDATORY: grep-based check MUST assert VP title equals VP file H1 verbatim. Anchor: S-18.08. |
| **[process-gap] Subsystem-anchor-sweep sibling-discipline gate** | CODIFIED 2026-06-15 — D-584 capture | L-F2-subsystem-anchor-sweep: fix-burst MUST sweep ALL VPs sharing source-BC AND L2-INDEX Cross-Walk same-burst. |
| **[process-gap] Canonical-scope-verification discipline** | CODIFIED 2026-06-15 — D-587 capture | Field-4 canonical (B): shell MAY exec `git cat-file -t SHA_B`; WASM reads field-4 STATICALLY. |
| **[process-gap] Stale-term-deferral-unsafe discipline** | CODIFIED 2026-06-15 — D-594 FULL BACKLOG CLEARANCE | stale terms in normative present-tense prose MUST be fixed in-scope. Full backlog cleared D-594. |
| **F-P27..F-P37 findings (ALL FIXED D-594..D-600 (see decision-log.md for full range))** | **ALL RESOLVED** | See decision-log.md SoT. |
| **[forward-track] F3 VP obligations** | FORWARD-TRACKED — BC-5.41.002 VP: S-18.01; BC-6.24.001 VP: S-18.03; BC-7.07.002 VP: S-18.05 MANDATORY per DI-024. Anchors: E-18 F3. | E-18 F3 story decomposition. |
| **O-P9-001 + L-S18-macos-ci-leg-caught-runtime-portability** | **ANCHORED S-18.11 + S-18.12 D-649 2026-06-19; S-18.12 MERGED PR #384 ec05606a** | sprint-state.yaml producer migration (S-18.11) + portability-lint guard extension (S-18.12). |
| **[process-gap] input_hash placeholder not gated on draft→ready** | OPEN 2026-06-22 — D-684 S-7.02 capture | Candidate fix: lint gate blocking draft→ready promotion when `input_hash:` matches placeholder regex. Anchor: E-18 F3 family. |
| **[process-gap] ADR/BC-version pin lint missing — BOTH word-orders required** | OPEN 2026-06-22 — D-685; REINFORCED D-686 | Pre-ready hardening checklist does NOT scan for ADR/BC-version pins. BOTH word-orders required. Anchor: E-18 F3 self-improvement epic. |
| **[process-gap] CI-green-attestation gate — premature 'CI N/N GREEN'** | OPEN 2026-06-23 — D-692 | 'CI N/N GREEN' MUST require ALL required matrix legs in TERMINAL state. Lesson: L-BB-premature-ci-green-attestation. |
| **[process-gap] scripts/generate-registry-from-hooks-json.sh + legacy JSON tombstone** | OPEN 2026-06-25 — S-18.05 adv P1 | (a) Superseded dead code without tombstone. (b) No `env_allow` drift validator vs ADR-026 §Decision 7. Anchor: E-18 F3. |
| **[process-gap] WASM hook stories must build real .wasm + run bats before TDD-green** | OPEN 2026-06-24 — D-693 | S-18.04a integration bugs missed by mocked-test TDD green. RULE: MUST build actual .wasm + run bats as pre-green gate. |
| **[D-743] sprint-state.yaml status not auto-synced on STORY-INDEX transitions** | OPEN 2026-07-01 — D-743 | Root cause of S-18.11/S-18.12 status-fidelity drift (BC-5.41.004 INV-2). Target: extend S-18.11 producer. |
| **[D-749 process-gap] merge-race-ready-report-stale-head** | OPEN 2026-07-02 — D-749 | PR-cycle READY verdicts MUST pin the exact covered HEAD SHA. Anchor: pr-manager skill hardening. |
| **[D-750 process-gap] release-PR merge-strategy not mechanically enforced** | OPEN 2026-07-04 — D-750; AWAITING HUMAN AUTHORIZATION | Proposed cure: repo ruleset `main-merge-commits-only`. AWAITING HUMAN AUTHORIZATION. |
| **[D-751 functional] verify-factory-lock silently degraded + 3 orphan WASMs** | OPEN 2026-07-04 — anchored S-19.02/S-19.04 | FINDING-1: STATE.md `output_too_large` (Fix: S-19.02 MERGED). FINDING-F-1: 3 orphan WASMs (Fix: S-19.04 MERGED). |
| **[D-750 process-gap] simulation-shell-dialect gap** | OPEN 2026-07-04 — D-750 | `mapfile` bash 4.0+ in macOS CI (Apple /bin/bash 3.2). Anchor: S-18.12-extension or standalone portability story. |
| **[D-762 hook false-positive] validate-count-propagation regex false-positive** | OPEN 2026-07-07 — D-762 | Regex `([0-9]{2,}) BCs` matches changelog narrative text. Root fix: scope regex to count-assertion contexts. |
| **[D-766 O-P15-01] BC frontmatter `cycle:` field inconsistent across E-19 BCs** | OPEN 2026-07-08 — D-766 | Three distinct `cycle:` values. Human adjudication required. Anchor: next maintenance sweep. |
| **[D-773] Legacy epic pre-existing template drift (6 epics)** | OPEN 2026-07-08 — D-773 | E-8/E-9/E-10/E-12/E-15/E-17/E-18 missing sections. Anchor: next maintenance sweep. |
| **O-P35-001 + [D-805 out-of-perimeter] E-17-lineage 3-tool-form sites** | OPEN — added D-790/D-805 | ADR-025 v1.2 volatile-pins in BC-5.40.001 + BC-6.23.001; stale 3-tool form in S-17.02/S-17.04/E-17. Target: next maintenance sweep. |
| **[O-P60-001] ADR-025 §Decision intro "ten numbered decisions" vs 15 present** | OPEN — added D-821; ACCEPTED-WITH-RECORD | Line 108 says "ten" but 15 Decision headings present. Severity LOW. Route: vsdd-factory:architect at next ADR-025 touch. |
| **[O-P61-001] VP-097 §Source Contract §Invariant 1 over-broad NOT_FOUND clause** | OPEN — added D-823; ACCEPTED-WITH-RECORD | NOT_FOUND clause belongs in VP-098 only. Severity LOW. Route: vsdd-factory:architect at next VP-097 touch. |
| **[D-826 W1-tracked] Kani infra gap + VP-097 spec-drift** | OPEN 2026-07-11 — D-826 | Kani toolchain incompatibility (pre-existing); VP-097 stale monolithic function signature (pending S-19.03 stabilization). |
| **[D-838 process-gap] DI-001..018 in invariants.md missing Cited-by lines** | OPEN 2026-07-13 — D-838 | Pre-existing gap caught during S-19.05 pass-11. Anchor: next maintenance sweep. |
| **[D-863 hook false-positive] validate-dispatch-advance regex matches word-final-D + hyphenated-year substrings** | OPEN 2026-07-20 — D-863 | word-final-D + `-YEAR` pattern (e.g., `APPROVED-YEAR` where YEAR is a 4-digit number) triggers bogus D-NNNN decision token. Root fix: scope D-chain regex. Anchor: next maintenance sweep / hook-hardening story. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-07-29 — D-943-PASS-28-RECORD-BURST; PIPELINE ACTIVE)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT.**

**Position.** S-21.04 LOCAL adversarial cascade (BC-5.39.001), E-21 Wave 2, deliver-story Step 4.5. Pass-28 REVIEWED + RECORDED, all factory-artifacts burst commits landed (Commits A–E). Feature branch `feature/S-21.04-story-worktree-write-path-discipline` @ `c7c61688` — clean, pushed, NO PR open (correct: mid-cascade; 13 commits). develop `948f0fb1`. Suites at `c7c61688`: story-worktree-write-path-discipline 10/10; worktree-identity-preflight 16/16 (26/26 total).

**Convergence.** Streak **0/3**. Twenty-eight passes, **ZERO CLEAN verdicts**. Trajectory tail →17→11→7→17.

**In-flight.** NONE. No story mid-TDD, no abandoned sub-agent steps, all commits pushed.

**D-927 status.** FALSIFIED at D-931. Root cause: the agent-definition pin `model: opus` resolves correctly to `claude-opus-5` (verified by three live no-override dispatches: `adversary`, `spec-reviewer`, `pr-reviewer`) — agents do not die silently (this claim was FALSIFIED at D-931). The `model`-override mitigation is **RETRACTED** — it forced agents off an available opus and reduced diversity. Phases 4 and 6 are NOT categorically broken; dispatch opus-pinned agents with **NO override**. ADR-033 ruling: amend cross-family definitions (specifications claim Gemini/GPT-5 diversity; all actual dispatches are Claude); cross-family opt-in via `VSDD_CROSS_FAMILY_DISPATCH`; prior convergences **ANNOTATED not invalidated**. Two structural guards added and verified by reversion testing: **D-935** real-fixture pipeline probe (code layer); **D-937** executable mutant corpus (evidence layer) — closing the structural holes behind every BLOCKER in this cascade. Pass-28 burst: B01 policies.yaml YAML-parse FIXED (v1.4.17→v1.4.18); H01-H07 CLOSED; M01-M07 CLOSED; L01-L02 CLOSED; T-010/RG-010 registered.

**Pass summary (passes 22–28 since D-927 falsification; counts from adversary-pass-NN.md files).**
- Pass-22: B1 / H5 / M4 / L2 = 12 findings (+3 observations)
- Pass-23: B2 / H4 / M6 / L2 = 14 findings
- Pass-24: B2 / H2 / M2 / L0 = 6 findings
- Pass-25: B3 / H4 / M8 / L2 = 17 findings (regression; streak reset; POLICY-15 restore-leg H04)
- Pass-26: B2 / H4 / M3 / L2 = 11 findings (H04 CLOSED at D-939; streak 0/3)
- Pass-27: B1 / H2 / M4 / L0 = 7 findings (starvation break — BC-6.26.001 body sections after 26 passes; CONTROL-equivalence closure; ADR-033; streak 0/3)
- Pass-28: B1 / H7 / M7 / L2 = 17 findings (BLOCKER: policies.yaml YAML-parse failure FIXED; T-010/RG-010 registered; streak 0/3; H01+M05–M07+L01–L02 → test-writer feature-branch)

**Resume items.**
1. **STATE.md structural gaps (two remaining):** (a) Drift Items table lacks `wave-state.yaml` W1 (E-19) stale-state row (pre-existing; next maintenance sweep); (b) Decisions Log visible table shows abbreviated rows only; D-863..D-941 (see decision-log.md for full range) are in decision-log.md SoT per content-routing rules (D-942+D-943 rows added this burst). NOTE: prior SRC language falsified at D-931 NOT carried into this SRC.
2. **`wave-state.yaml` points to a closed epic.** It reads `wave: W1 (E-19)` — E-19 closed at D-851, three wave boundaries stale; `/wave-handoff` was never run across those boundaries. **A resuming session must NOT trust `/rehydrate-wave` output until the manifest is regenerated** — it will inject the wrong epic's 11 spec files.
3. **Input-hash drift — do NOT run `compute-input-hash --update`.** Reason: repo-wide blast radius — at the D-936 burst it churned **418 files** and had to be reverted. Current state: `TOTAL=2384 MATCH=1 STALE=2136`; POLICY 18 is effectively unenforced. D-940 directive stands.
4. **ADR-033 body not expanded.** S-22.01/S-22.02 (P0) and S-22.03 (P1) registered when ADR-033 was authored (stub state; pre-dates D-940). Route: architect.
5. **Pass-29 lead: H01 + M05–M07 + L01–L02 → test-writer (feature branch).** Gates (13)–(18) section-wide negative gates un-re-derived since pass-19 (cascade's oldest stale surface). H01 gates re-derivation; M05/M06 gate section-wide extractor + direction-statement annotation; M07 pipeline probe corpus leg extended M10–M14; L01/L02 gate-specific extractors. All 6 findings routed to test-writer for feature-branch delivery before pass-29.
6. **Pass-29 lead: T-010 / RG-010 newly registered.** T-010 (EC-009 stray-inode-inside-factory) and RG-010 (T-010 stray-symlink+FIFO-inside-factory) registered at pass-28. No tests yet on feature branch. Adversary should scrutinize coverage at pass-29.
7. **Pass-29 lead: policies.yaml regex fix scrutiny.** policies.yaml v1.4.17→v1.4.18 fixes `\+`→`\\+` in policy_16 and policy_18 regexes. Adversary should verify fix is complete and correct at pass-29; other policies with double-quoted scalars may have the same pattern.
8. **Standing backlog.** rc.24 release pending (load-bearing); 7 dependabot vulns on develop (2 high); PR #729 REQUEST_CHANGES; issue-close comments #342/#365/#358 pending human.
9. **Main repo pre-existing uncommitted state.** `M .claude/scheduled_tasks.lock` (always modified; do not commit) + untracked `plugins/vsdd-factory/tests/report.tap` (bats artifact; candidate for `.gitignore`). Deliberately uncommitted.

**Pending human decisions.** Standing ruling: **"grind to 3 clean"** — asymptotic acceptance declined. Next action = Pass-29 adversary dispatch (after test-writer delivers H01+M05–M07+L01–L02 to feature branch).

**CAUTION: `pipeline: ACTIVE` as of D-943. D-417(b) strict dispatch-side advance only modifies `phase:` + `current_step:`, not `pipeline:`. Use `phase:` field and this SRC as the authoritative liveness indicators.**

**Resume command.** `/vsdd-factory:next-step` — do NOT run `/rehydrate-wave` first (this is NOT a wave-boundary clear; resume directly from this SRC).
