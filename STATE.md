---
document_type: pipeline-state
level: ops
version: "7.70"
status: draft
producer: state-manager
timestamp: 2026-08-14T04:35:00Z
phase: SHA-PATCH-2026-08-14-D-1006
last_amended: "2026-08-14 (v7.70) — SHA-PATCH-2026-08-14-D-1006 (state-manager; commit 27877f9a): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-1006 row + Phase Progress/Current Phase Steps + all pending-SHA-patch cites updated e9fd7607→27877f9a (the D-1006 burst commit's own actual HEAD, confirmed via git rev-parse HEAD after push). No content change beyond the SHA-patch itself — D-1006's substance (pass-21 NOT-CLEAN, 1 HIGH F-S2107-P21-001 CLOSED THIS BURST — S-21.07 v1.13→v1.14 Task 10 two-phase rewrite; STORY-INDEX v4.324→v4.325; REC-P21-A/OBS-P21-B tracked §8; lesson L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers CODIFIED; streak RESETS 1/3→0/3) is UNCHANGED. [Prior: 2026-08-14 (v7.69) — D-1006-S2107-PASS21-RECORD-AND-FIX-BURST (state-manager; single-commit TD-VSDD-053; parent-commit e9fd7607; own SHA pending push): fresh-context adversary pass-21 dispatched against feature/S-21.07 at 96b4be19 and factory-artifacts at e9fd7607 (D-1005 SHA-patch HEAD). Verdict NOT-CLEAN — 1 HIGH (F-S2107-P21-001: story Task 10's build directive specified the BC-5.39.010-v1.19-PC13-NON-CONFORMING bare-optional-v regex \\bv?([0-9]+\\.[0-9]+)\\b, despite every OTHER BC-version cite in the story reading v1.19-correct — a version-cite-propagates/algorithm-content-does-not gap, distinct from the D-996/D-998/D-1000/D-1004 fix-scoped-to-named-site/predicate-adequacy family). CLOSED THIS BURST (story-writer: S-21.07 v1.13→v1.14 — Task 10 rewritten verbatim to the two-phase algorithm per BC-5.39.010 v1.19 PC13; class-complete bare-regex-literal sweep confirmed no other live directive sites; input-hash UNCHANGED 93c4a89). REC-P21-A (test-coverage recommendation) and OBS-P21-B (cosmetic bracket-count mismatch) recorded as tracked non-blocking observations in §8, NOT actioned. STORY-INDEX v4.324→v4.325 (catalog row story-version 1.13→1.14; frontmatter self-bump same-commit, no parity gap). Lesson L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers CODIFIED. STREAK RESETS 1/3 → 0/3 — in-perimeter HIGH; 3 fresh CONSECUTIVE CLEAN passes required from pass-22 to converge. trajectory-tail →1→1→0→1.]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "SHA-PATCH-2026-08-14-D-1006 (state-manager; commit 27877f9a; D-chain cite D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998+D-1000+D-1001+D-1002+D-1003+D-1004+D-1005+D-1006; trajectory-tail S-21.07 →1→1→0→1 UNCHANGED): Active Branches factory-artifacts row, Session Resume Checkpoint header, Phase Progress/Current Phase Steps, and Decisions Log D-1006 row SHA-patched e9fd7607->27877f9a (the D-1006 burst commit's own actual HEAD, confirmed via git rev-parse HEAD after push). D-1006 substance UNCHANGED by this patch: pass-21 adversary NOT-CLEAN (1 HIGH F-S2107-P21-001, CLOSED THIS BURST -- S-21.07 v1.13->v1.14 Task 10 two-phase rewrite; STORY-INDEX v4.324->v4.325; lesson CODIFIED). STREAK RESETS 1/3 -> 0/3 (in-perimeter HIGH); 3 fresh CONSECUTIVE CLEAN passes required from pass-22 to converge. No gate predicate, GateOutcome semantics, or ADR ratification status changed; feature/S-21.07 SHA 96b4be19 UNCHANGED; BC-INDEX/VP-INDEX/ARCH-INDEX untouched. This SHA-patch write is the standard immediate follow-up commit after the D-1006 burst commit's push -- permitted per project convention as a SHA-patch, not a Stage-2 chain (TD-VSDD-053). NEXT: dispatch vsdd-factory:adversary fresh-context for pass-22."
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

  D-430(a) compaction history D-532..D-808 (see decision-log.md for full range; exhaustive) COLLAPSED 2026-07-12. Full per-burst wc-l history archived; SoT: decision-log.md + git show 903aa863:.factory/STATE.md for D-828 pre-compaction state.
  D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20; v6.08→v6.09. Full per-burst wc-l history D-819..D-861 (see decision-log.md for full range; exhaustive) archived; SoT: decision-log.md + git show 9debd920:.factory/STATE.md for D-861 pre-compaction state.
  (Rows D-890..D-987, 2026-07-24..2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 9d72dc15:.factory/STATE.md for D-980 pre-compaction detail.)
  (Rows D-993..D-995, 2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 9c132dd2:.factory/STATE.md for D-995 pre-SHA-patch detail.)
  (Rows D-996, 2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 00cbc4ea:.factory/STATE.md for D-996 pre-SHA-patch detail.)
  (Rows D-997, 2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 8e5c7344:.factory/STATE.md for D-997 pre-SHA-patch detail.)
  (Rows D-998, 2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 1750bd56:.factory/STATE.md for D-998 pre-SHA-patch detail.)
  (Rows D-1000, 2026-08-13, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show ddc07cf5:.factory/STATE.md for D-1000 pre-SHA-patch detail.)
  (Rows D-1001, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show c7fa6d93:.factory/STATE.md for D-1001 pre-SHA-patch detail.)
  (Rows D-1002, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show ebe100d0:.factory/STATE.md for D-1002 pre-SHA-patch detail.)
  (Rows D-1003, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 0887ff63:.factory/STATE.md for D-1003 pre-SHA-patch detail.)
  (Rows D-1004, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show eb5ecbf1:.factory/STATE.md for D-1004 pre-SHA-patch detail.)
  (Rows D-1005, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show 3bf8561a:.factory/STATE.md for D-1005 pre-SHA-patch detail.)
  (Rows D-1006, 2026-08-14, see decision-log.md for full range; exhaustive; archived; SoT: decision-log.md + git show e9fd7607:.factory/STATE.md for D-1006 pre-SHA-patch detail.)
  (wc-l post-D-1005-S2107-PASS20-RECORD-ONLY-BURST 2026-08-14; pass-20 CLEAN, 0 findings, RECORD-ONLY — nothing to close; streak ADVANCES 0/3→1/3; v7.66→v7.67; commit 2d0c6d37)
  (wc-l post-SHA-patch 2d0c6d37 2026-08-14; Active Branches factory-artifacts 3bf8561a→2d0c6d37; v7.67→v7.68 UNCHANGED content)
  (wc-l post-D-1006-S2107-PASS21-RECORD-AND-FIX-BURST 2026-08-14; pass-21 NOT-CLEAN, F-S2107-P21-001 HIGH CLOSED — S-21.07 story v1.14 (Task 10 two-phase rewrite), STORY-INDEX v4.325; streak RESETS 1/3→0/3; v7.68→v7.69; commit 27877f9a)
  (wc-l post-SHA-patch 27877f9a 2026-08-14; Active Branches factory-artifacts e9fd7607→27877f9a; v7.69→v7.70 UNCHANGED content)
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
| **Last Updated** | 2026-08-14 — SHA-PATCH-2026-08-14-D-1006 (commit `27877f9a`): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-1006 row + all pending-SHA-patch cites updated e9fd7607→27877f9a. No content change beyond the SHA-patch itself — D-1006's substance (pass-21 **NOT-CLEAN**, 1 HIGH CLOSED THIS BURST; STORY-INDEX v4.324→v4.325; **STREAK RESETS 1/3 → 0/3**) is UNCHANGED. trajectory-tail (S-21.07) →1→1→0→1. |
| **Current Phase** | **SHA-PATCH-2026-08-14-D-1006 (commit `27877f9a`; D-chain cite D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998+D-1000+D-1001+D-1002+D-1003+D-1004+D-1005+D-1006; PIPELINE ACTIVE).** S-21.09 remains **MERGED** (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` is **UNFROZEN + sequenced-next**, still **NOT merge-ready** — streak **0/3 (RESET)**, 3 fresh CONSECUTIVE CLEAN passes required (pass-22, pass-23, pass-24). Story v1.14 (Task 10 two-phase rewrite). 4-INDEX BC v4.59 (UNCHANGED) / VP v2.76 (UNCHANGED) / STORY v4.325 (UNCHANGED this write) / ARCH v3.58 (UNCHANGED). policies.yaml v1.4.24 UNCHANGED. SHA-patch follow-up DONE this write. **Next substantive action: dispatch `vsdd-factory:adversary` fresh-context for pass-22** (reads only `adversary-pass-21.md` Part A per the Iron Law). |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B..D-647 ALL COMPLETE/ARCHIVED | **ALL COMPLETE / ARCHIVED** | SoT: decision-log.md + burst-log.md. |
| v1.0.0-rc.20/21/22 SHIPPED | **ALL SHIPPED** | PRs merged; marketplace published. |
| D-856 RC23-SHIPPED 2026-07-18 | **SHIPPED** | GitHub Release v1.0.0-rc.23; marketplace published. |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20 | **COMPLETE** | PIPELINE UNPAUSED — S-21.01..S-21.04 delivered |
| D-890..D-987 (see decision-log.md for full range; exhaustive) S-21.04/S-21.07/S-21.09 waves 2026-07-24..2026-08-13 | **COMPLETE** | SoT: decision-log.md + burst-log.md; git show 9d72dc15:.factory/STATE.md for pre-compaction detail. |
| D-988..D-992 (see decision-log.md for full range; exhaustive) S-21.09-RE-CONVERGENCE+MERGE+POST-MERGE+PASS10-FIX 2026-08-13 (single commits TD-VSDD-053; commits b31de9e2/bb2d63b6/2e4c0a7f/0b350501/b046531d) | **COMPLETE** | LOCAL adversary pass-24 CLEAN, streak TRUE 3-CLEAN RE-CONVERGED; PR #775 MERGED `2e8087af`; S-21.07 pass-10 (10 findings, D-967) dispositioned (8 CLOSED, 2 ALREADY-RESOLVED-with-residual); ADR-041 v1.1 + ADR-042 v1.3 human-RATIFIED. STATE.md v7.43. |
| D-994-S2107-PASS11-RECORD-AND-FIX-BURST..D-1003-S2107-PASS18-RECORD-AND-FIX-BURST 2026-08-13..2026-08-14 (see decision-log.md for full range; exhaustive; single commits TD-VSDD-053) | **COMPLETE** | Passes 11-18 of the S-21.07 LOCAL cascade. CLIFF 10→1 (pass-11); pass-12/13 story+STORY-INDEX sibling sweeps; pass-14 first CLEAN (streak 0/3→1/3); pass-15 human-directed IN-PERIMETER reset (1/3→0/3); pass-16 semantic-role sweep; pass-17 second CLEAN (0/3→1/3); pass-18 §VP Anchors count-parity MEDIUM RESET (1/3→0/3). STATE.md v7.47→v7.63. |
| D-1004-S2107-PASS19-RECORD-AND-FIX-BURST 2026-08-14 (single commit TD-VSDD-053; commit `eb5ecbf1`; SHA-patch done) | **COMPLETE** | Pass-19 NOT-CLEAN (1 MEDIUM F-S2107-P19-001 + 4 obs); F-S2107-P19-001 CLOSED THIS BURST (S-21.07 v1.12→v1.13; STORY-INDEX v4.322→v4.324). **STREAK HOLDS 0/3**. STATE.md v7.63→v7.65→v7.66. |
| D-1005-S2107-PASS20-RECORD-ONLY-BURST 2026-08-14 (single commit TD-VSDD-053; commit `2d0c6d37`; SHA-patch done) | **COMPLETE** | Pass-20 **CLEAN** (0 findings); F-S2107-P19-001 closure independently re-verified; RECORD-ONLY, nothing to close. **STREAK ADVANCES 0/3 → 1/3**. STATE.md v7.66→v7.67→v7.68. |
| D-1006-S2107-PASS21-RECORD-AND-FIX-BURST 2026-08-14 (single commit TD-VSDD-053; commit `27877f9a`; SHA-patch done) | **COMPLETE** | Pass-21 **NOT-CLEAN** (1 HIGH F-S2107-P21-001 — Task 10 build directive specified the BC-5.39.010-v1.19-PC13-NON-CONFORMING bare-optional-v regex despite every other version-cite reading v1.19-correct; version-cite-propagates/algorithm-content-does-not gap); CLOSED THIS BURST (story-writer: S-21.07 v1.13→v1.14 — Task 10 rewritten to the two-phase algorithm verbatim; class-complete bare-regex-literal sweep confirmed no other live sites; input-hash UNCHANGED `93c4a89`); STORY-INDEX v4.324→v4.325 (catalog row story-version 1.13→1.14); REC-P21-A (test-coverage recommendation) + OBS-P21-B (cosmetic bracket-count mismatch) tracked in §8, not actioned; lesson `L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers` CODIFIED. **STREAK RESETS 1/3 → 0/3** — in-perimeter HIGH. STATE.md v7.68→v7.69→v7.70. |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-1005 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| SHA-PATCH-2026-08-14-D-1005 (state-manager; commit 2d0c6d37) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-1005 row SHA-patched `3bf8561a`→`2d0c6d37`; STATE.md v7.67→v7.68; no content change. |
| D-1006-S2107-PASS21-RECORD-AND-FIX-BURST 2026-08-14 (single commit TD-VSDD-053; commit 27877f9a) | state-manager | COMPLETE | Pass-21 **NOT-CLEAN** (1 HIGH F-S2107-P21-001) persisted verbatim; independent Task-10 verify performed BEFORE bundling; CLOSED THIS BURST (story-writer: S-21.07 v1.13→v1.14; STORY-INDEX v4.324→v4.325); REC-P21-A/OBS-P21-B tracked §8. **STREAK RESETS 1/3 → 0/3.** STATE.md v7.68→v7.69. |
| SHA-PATCH-2026-08-14-D-1006 (state-manager; commit 27877f9a) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-1006 row SHA-patched `e9fd7607`→`27877f9a`; STATE.md v7.69→v7.70; no content change. **NEXT: adversary pass-22 dispatch (fresh-context, reads adversary-pass-21.md Part A only).** |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.59 D-1003, UNCHANGED this burst — BC-5.39.010 not amended; total_bcs UNCHANGED) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960, UNCHANGED this burst) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX v4.325 D-1006, UNCHANGED this SHA-patch write — S-21.07 catalog row story-version 1.13→1.14 landed at the D-1006 burst commit) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-040 v1.16 D-994 pass-11 fix / ADR-041 v1.2 / ADR-042 v1.4 D-993 body reconciliation, UNCHANGED this burst; ADR-043 proposed NOT RATIFIED) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | **108** (STATE.md explicit counter; sprint-state predicate tracked separately per canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (108):** S-21.09 MERGED PR #775 `2e8087af` 2026-08-13 (validate-factory-path-staging WASM artifact restore + registry parity CI check; E-21 W4). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md` (known gap: not appended between S-19.03 and S-21.09; see ledger file note — anchored to a dedicated maintenance sweep).
- **In-Flight (0):** none.
- **E-21:** S-21.07 (W4; **sequenced next**, UNFROZEN; pass-21 **NOT-CLEAN** — 1 HIGH (F-S2107-P21-001, CLOSED THIS BURST: Task 10's build directive specified the BC-5.39.010-v1.19-PC13-NON-CONFORMING bare-optional-v regex despite every other version-cite reading v1.19-correct; rewritten to the two-phase algorithm verbatim); REC-P21-A/OBS-P21-B tracked non-blocking (not actioned); **STREAK RESETS 1/3 → 0/3** (in-perimeter HIGH) — 3 fresh CONSECUTIVE CLEAN required (pass-22, pass-23, pass-24); story spec **v1.14** UNCHANGED this SHA-patch write (Task 10 fix landed at the D-1006 burst commit; input-hash UNCHANGED `93c4a89`); branch `96b4be19` pushed (unchanged, code-repo not touched — fix is spec-only); **NOT merge-ready**; pass-22 adversary dispatch is the pending gate); S-21.09 (**MERGED** PR #775 `2e8087af`); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | **2e8087af** | PR #775 (`feature/S-21.09`) merged 2026-08-13T14:16:26Z; `validate-factory-path-staging.wasm` git-tracked. Pull on next code-worktree resume. |
| factory-artifacts | **27877f9a** | D-1006-S2107-PASS21-RECORD-AND-FIX-BURST. SHA-patch done 2026-08-14. |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. **F-001's sole remaining residual (D-992) is BLOCKED-ON this branch merging to `develop`** — routed devops-engineer, anchored Drift Item `[D-969]`. |
| feature/S-21.09 | c20cf2fe | **MERGED** to `develop` via PR #775, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Branch ref retained (standard post-merge retention). LOCAL BC-5.39.001 streak 3/3 RE-CONVERGED (D-988), PRESERVED through D-989 — final state at merge. |
| feature/S-21.07-validate-cross-site-correspondence | **96b4be19** | pass-21 **NOT-CLEAN** (1 HIGH, CLOSED THIS BURST, D-1006) — **STREAK RESETS 1/3 → 0/3**; UNCHANGED this burst (no code-repo commit; fix is spec-only). Pushed; SHA-equal with origin. Still UNFROZEN + sequenced-next, **NOT merge-ready** — convergence depends on **adversary pass-22 (NEXT)**, 3 fresh CONSECUTIVE CLEAN required. |
| feature/S-21.04 | 323f440f | pass-31 pending; no PR. Pushed; SHA-equal with origin. |
| fix/nested-factory-path-derivation | 9afc3226 | F-S2107-P8-016 + P9-008 CLOSED. Pushed; SHA-equal with origin. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel. Pushed; SHA-equal with origin. |
| fix/fuel-exhaustion-fail-loud | fbb9dcb6 | ABANDONED — orchestrator dispatch error (87 files duplicating unmerged S-21.07). CONFIRMED SUPERSEDED by PR #774 (`62fbcf1a`, D-992 re-verification). Local-only; deliberately NOT pushed. |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-1006-S2107-PASS21-RECORD-AND-FIX-BURST COMPLETE (SHA-patch done 27877f9a). S-21.09 **MERGED** to `develop` (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` pass-21 (**NOT-CLEAN**, 1 HIGH, CLOSED THIS BURST, D-1006) — SHA `96b4be19` UNCHANGED; story spec **v1.14** UNCHANGED. **NOT merge-ready — STREAK RESETS 1/3 → 0/3 (in-perimeter HIGH); 3 fresh CONSECUTIVE CLEAN required (pass-22, pass-23, pass-24).** No ADR touched this burst. `develop` **2e8087af**; main 80e5cd7b; `merged_count` **108**; BC v4.59 (UNCHANGED); VP v2.76 (UNCHANGED); STORY v4.325 (UNCHANGED); ARCH v3.58 (UNCHANGED); policies.yaml v1.4.24 UNCHANGED; ADR-043 proposed NOT RATIFIED. F-001 redesign RATIFIED (ADR-040 v1.12/v1.13/v1.15/v1.16) — CI wiring still PENDING, BLOCKED-ON `feature/policy15-gate-rust`→`develop`. LOCAL BC-5.39.001 streak (S-21.07 cascade) **0/3 — RESET**. trajectory-tail (S-21.07) →1→1→0→1. | SHA-patch done 27877f9a 2026-08-14; D-1006-S2107-PASS21-RECORD-AND-FIX-BURST 2026-08-14. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range; exhaustive): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range; exhaustive) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-1006 (see decision-log.md for full range; exhaustive): this Decisions Log (D-1006 live) + decision-log.md SoT. **D-999 is SKIPPED (never allocated) per human directive** — the global D-NNN sequence continues D-998, D-1000, D-1001, D-1002, D-1003, D-1004, D-1005, D-1006 (no D-999 row exists or ever will).

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-1006 | D-1006-S2107-PASS21-RECORD-AND-FIX-BURST (state-manager; single-commit TD-VSDD-053 2026-08-14; commit `27877f9a`; SHA-patch done). Fresh-context adversary pass-21 dispatched against `feature/S-21.07-validate-cross-site-correspondence` at `96b4be19` and `factory-artifacts` at `e9fd7607` (the D-1005 SHA-patch HEAD). Verdict: **NOT-CLEAN — 1 HIGH finding (F-S2107-P21-001) + 2 non-blocking observations (REC-P21-A, OBS-P21-B).** Persisted verbatim as `cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-21.md`. **F-S2107-P21-001 (HIGH — POLICY 4 semantic-anchoring/spec-implementation contradiction + TD-VSDD-060):** story Task 10 directed the implementer to build the Arm A2 version-citation extractor using `\bv?([0-9]+\.[0-9]+)\b`, the single-pass bare-optional-v regex that BC-5.39.010 v1.19 PC13 itself declares NON-CONFORMING, annotated "per BC-5.39.010 v1.19 PC13 (unchanged since v1.14)" — a version-NUMBER annotation that was true while the regex CONTENT attached to it had silently diverged from the BC seven story-versions earlier (at BC v1.8, when PC13 was rewritten to the two-phase algorithm). Task 10 contradicted BOTH the governing BC and the story's own §BC Status section (correctly two-phase since v1.5). Root cause: a version-cite-propagates/algorithm-content-does-not gap — six consecutive propagation sweeps (v1.4 through v1.13) advanced the cited VERSION NUMBER without re-deriving the embedded REGEX CONTENT from the BC's current text. Blast radius: an implementer coding Task 10 verbatim would reintroduce a story-ID collision, a BC-section-number collision, and an ACs-column collision on S-21.07's OWN governing citation ("DEFERRED v1.6"). **CLOSED THIS BURST** (story-writer: S-21.07 v1.13→v1.14 — Task 10 rewritten verbatim to the two-phase algorithm per BC-5.39.010 v1.19 PC13, collision rationale restated inline, class-complete story-wide bare-regex-literal sweep confirmed zero other live directive sites; input-hash UNCHANGED `93c4a89`, body-only fix). Independent CLEAN axes re-derived, unaffected by the finding: version parity chain, three-way input-hash `93c4a89`, POLICY 7 H1 SoT, F-S2107-P18-001/F-S2107-P19-001 closures both hold, E-21 aggregation 14/117/8, retracted-claim class zero live members. **REC-P21-A** (test-coverage recommendation, tracked §8, not actioned): no corpus-style negative regression test exists for Arm A2's collision-avoidance behavior. **OBS-P21-B** (cosmetic, tracked §8, not fixed): pre-existing bracket-count mismatch in append-only `[Prior:]` chains, harmless (opaque YAML block-scalar, no parse impact). O-P19-01/O-P15-03, O-P17-02, O-P17-01, O-P14-03 re-observed unchanged, not reopened. **STREAK RESETS 1/3 → 0/3** — the finding is IN-PERIMETER (a live spec-implementation contradiction inside S-21.07's own build directive), not an out-of-perimeter carve-out. BC-5.39.001 requires 3 FRESH CONSECUTIVE CLEAN passes to converge — **pass-22, pass-23, pass-24 all required.** `feature/S-21.07` SHA `96b4be19` UNCHANGED (no code-repo commit — fix is spec-only). BC-INDEX/VP-INDEX/ARCH-INDEX all UNCHANGED (BC-5.39.010 not amended); STORY-INDEX v4.324→**v4.325**. Lesson `L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers` CODIFIED. Full detail: `git -C .factory show e9fd7607:.factory/STATE.md` (pre-D-1006 state) + `decision-log.md` D-1006 + `burst-log.md` `## D-1006-S2107-PASS21-RECORD-AND-FIX-BURST` + `adversary-pass-21.md`. | BC-5.39.010 v1.19 (UNCHANGED), S-21.07 story v1.13→v1.14, STORY-INDEX v4.324→v4.325; streak RESETS 1/3 → 0/3; 3 fresh CONSECUTIVE CLEAN required (pass-22, pass-23, pass-24) | D-1006-S2107-PASS21-RECORD-AND-FIX-BURST | 2026-08-14 |
| D-413..D-1006 (see decision-log.md for full range; exhaustive; D-999 never allocated) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-14 |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

| Blocker | Status | Risk Statement |
|---------|--------|----------------|
| **[P0] POLICY 15 ATTESTATION-LOCATION GATE vacuous (F-S2107-P10-001)** | **OPEN — REDESIGN RATIFIED D-970; CI-WIRING RESIDUAL ONLY (D-992)** | Root cause (D-969): category error — gate evaluated in factory-artifacts worktree where *.rs/*.bats count is permanently zero. ADR-040 v1.12/v1.13/v1.15/v1.16 RATIFIED/AMENDED; policies.yaml v1.4.23 ACTIVE (ATTESTATION-LOCATION GATE text); Codifications 1+2 APPLIED. **Closes when:** `feature/policy15-gate-rust` crate (`d2a3176a`, 16 tests, mutation-verified) merged to `develop` AND CI job wired. **BLOCKED-ON `feature/policy15-gate-rust`→`develop`** — this is a separate branch from `feature/S-21.07`, no open PR. Anchored Drift Item `[D-969]`; routed devops-engineer. |
| **[C-1] CWE-706 incorrect path resolution — exec_subprocess binary_allow load-time prefix check** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | `binary_allow` entries are bare names; prefix list inert. ADR-043 v1.5 NOT RATIFIED. **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.** Route: implementer + security-reviewer after ADR-043 ratification. |
| **[C-2] CWE-362 TOCTOU — resolve-then-check window in exec_subprocess** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | Race between resolution and spawn; threat model boundary not formally specified. Route: security-reviewer before ADR-043 ratification. |
| **[C-4] CWE-284 access control — arbitrary binary execution via misconfigured prefix list** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | No validation of prefix list entries at load time. Route: product-owner (BC amendment) + implementer. |
| **[C-5] CWE-284 no per-entry resource limit isolation** | **OPEN 2026-08-11 — MEDIUM SECURITY (D-972)** | No per-`binary_allow` resource limits. Route: product-owner + story-writer. |
| **[pass-10 carry-over] ADV-BB-P10-MED-001 directory-only `hook-plugins/sub/` staging control** | **OPEN — preserved through D-1006; does NOT block anything** | Gate (a)'s ">= 1 component after hook-plugins/" threshold admits directory-only declarations; spurious MISSING report possible. Anchor: next maintenance sweep. |
| **[pass-10 carry-over] ADV-BB-P10-LOW-001/002/003 (NUL/trailing-space names; fail-open arms; `workspace_root()` untested)** | **OPEN — preserved through D-1006; does NOT block anything** | Low-severity residuals from the S-21.09 cascade's pass-10; not addressed through the merge or this burst. Anchor: next maintenance sweep. |
| **[D-1006] `feature/S-21.07` pass-21 NOT-CLEAN — streak RESETS 1/3 → 0/3, 3 fresh CONSECUTIVE CLEAN required** | **OPEN — sequenced next, NOT merge-ready** | `feature/S-21.07` SHA `96b4be19` UNCHANGED; story spec v1.14 UNCHANGED. F-S2107-P21-001 CLOSED. **Pass-22 adversary dispatch (fresh-context, reads `adversary-pass-21.md` Part A only) is the pending gate.** |
| **[D-1000] E-18 STORY-INDEX delivery-blockquote total (107 pts) disagrees with current catalog sum (125 pts)** | **OPEN — OUT-OF-PERIMETER, does NOT block anything; master-line-arithmetic layer RESOLVED D-1000** | Frozen-historical record of a COMPLETE/merged epic, per D-996(d) precedent; the master "Total story points" line (L763) now correctly cites E-18's own canonical 107 (D-1000), closing the narrower master-line-vs-canonical-blockquote disagreement. The deeper question — whether 107 should itself be reconciled against the current 125-pt catalog sum — remains open. Companion item: O-P17-01 (master-total leading-aggregate "533+" stale floor). Anchor: next maintenance sweep or explicit human direction. |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67..74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754. |
| **TD-VSDD-061/062/063** | OPEN 2026-05-17/19 | validate-index-cite-refresh large-file fail-open; schema inconsistencies; VP allocation deferred. |
| **TD-VSDD-095..100** | CODIFIED-AND-FORWARDED | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101** | OPEN 2026-05-18 — anchored S-15.15 | VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI; dangling ref confirmed D-972. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | check_index_sites + rows_after_heading advisory-arm defects. |
| **[D-945] VP-102..VP-120 pending allocation** | DEFERRED — anchored `feature/S-21.07` post-convergence | 19 VPs per BC-5.39.010 §VP Anchors (corrected from 17 at D-1003/F-S2107-P18-001 — reserved range reconciled to match the BC's own 19-row §Verification Properties table; no VP allocated yet, still a forward reservation). |
| **[D-952] compute-input-hash operator cache binary divergence** | BOOTSTRAP-MIGRATED — anchored rc.24 #715 | Self-heals at rc.24. |
| **[D-953] 27 unparseable frontmatter files** | OPEN 2026-08-04 | Needs remediation story. |
| **[D-953] ADR-037 19-story volatile-inputs remediation** | OPEN 2026-08-04 | S-19.01 CRITICAL. |
| **[D-954] Duplicate T-038 test ID** | OPEN 2026-08-04 | POLICY 1 violated; anchor next fix burst. |
| **[D-954] decision-log.md >17,500 lines** | OPEN 2026-08-04 | WASM validators time out on every edit (confirmed again this burst — advisory-only, writes land). |
| **[D-991] `validate-factory-path-staging.wasm` operator-runtime effectiveness pending rc.24** | OPEN 2026-08-13 | Artifact now git-tracked on `develop` (S-21.09 merged) but the operator marketplace cache remains at rc.23 until the next release cut. |
| **[D-991] `merged-stories-ledger.md` gap S-19.04..S-21.08** | OPEN 2026-08-13 | Ledger not appended between 2026-07-13 (S-19.03) and 2026-08-13 (S-21.09); out of scope for the single-story D-991 burst. Anchor: dedicated maintenance sweep. |
| **[D-955] 18 Dependabot vulnerabilities** | OPEN 2026-08-10 (corrected D-971) | Anchor: next maintenance sweep. |
| **[D-957] F-S2107-P7-019 D-693 stale WASM size** | OPEN 2026-08-05 | Anchor: `feature/S-21.07` fix burst or next SHA-patch. |
| **[D-958] 60 of 158 stories lack tdd_mode** | OPEN 2026-08-06 | Anchor: S-15.03 PRIORITY-A. |
| **[D-958] GATE DEFECT: validate-pr-review-posted + validate-changelog-monotonicity** | OPEN 2026-08-06 | Paper-gate; header-skip misread. |
| **[D-961] SEC-001 + RUSTSEC-2026-0222/0204 + 18 Dependabot + EAC-002 + ADR-033** | OPEN 2026-08-07 — SECURITY | E-22 scope re-anchored to E-21 W4. Dependabot count corrected D-971. |
| **[D-963] ADR-035 §Decision 5 quadratic not observed** | OPEN 2026-08-08 | Linear R²=0.998790. Route: architect at next ADR-035 touch. |
| **[D-963] BC-5.39.010 live-operation silent exhaustion gap** | OPEN 2026-08-08 | plugin.timeout exits 0/empty. Anchor: `feature/S-21.07` + margin gate implementation. |
| **[D-964] fix/fuel-cap-raise-20m NOT YET EFFECTIVE** | OPEN 2026-08-10 (D-968) — release-gated | On develop (`2e8087af`); operator cache rc.23 still embeds 10M; requires rc.24. |
| **[SESSION-WRAP-2026-08-09 / 2026-08-11] Dispatcher log deletion recurrence — 4 occurrences** | OPEN 2026-08-09 — root cause unestablished | `.factory/.factory/logs/` EXISTS with 2 files. Anchor: maintenance sweep. |
| **[D-966] F-002 retroactive-attestation (permanent)** | **REMEDIATED D-992** | Erratum note committed `96b4be19` (implementer, `feature/S-21.07-validate-cross-site-correspondence`). Underlying historical violation at `67ffbdcc`/`38c70f9e` remains permanent/immutable by design — history cannot be rewritten; only the remediation obligation closed. |
| **[D-969] feature/policy15-gate-rust pending integration** | OPEN 2026-08-10; ratification complete D-970 — **now F-001's sole residual (D-992)** | Awaits: crate merged to `develop`; CI job wired. Routed devops-engineer. |
| **[D-971] RUSTSEC-2026-0204/0190/0052 unanchored advisories** | OPEN 2026-08-10 — SECURITY | cargo-deny fails with 5 findings total. Anchor: E-22 security scope. |
| **[D-971] RUSTSEC-2026-0188 exploitability framing** | OPEN 2026-08-10 — SECURITY | Route: security-reviewer. Anchor: E-22. |
| **[D-971] refuse_setuid gate inert — HIGH SECURITY** | OPEN 2026-08-10 | Route: security-reviewer + implementer. Anchor: E-22 or dedicated story. |
| **[D-972] 6 vacuous gate drift items** | OPEN 2026-08-11 | All linked to C-1..C-5 or ADR-043. Anchor: ADR-043 ratification + S-21.14. |
| **[D-989] Cross-platform CI is a convergence prerequisite, not just a merge prerequisite** | CODIFIED — anchored S-15.03 PRIORITY-A 2026-08-13 | An all-macOS LOCAL cascade + mutation audit cannot catch platform-specific (OS path-separator) defects; fold a Windows-portability fixture check into test-writer discipline. |
| **[D-989] github-ops push delegate non-functional mid-session** | OPEN — anchored S-15.03 PRIORITY-A 2026-08-13 | pr-manager→github-ops push delegate failed after the first push; orchestrator pushed directly under human authorization. Investigate root cause. |
| **[D-991] state-manager delegate death requires decision-log backfill discipline** | CODIFIED — `L-BB-state-manager-delegate-death-requires-decision-log-backfill-not-silent-gap` anchored S-15.03 PRIORITY-A 2026-08-13 | A decision surfaced only in STATE.md narrative (never persisted to decision-log.md) is a gap the NEXT burst must backfill, preserving the already-surfaced D-NNN ID. Closed the D-990 gap this burst; consider a mechanical max-D cross-check gate at session resume. |
| **[D-992] orchestrator→state-manager relay-verification gap (F-010)** | CODIFIED — `L-BB-orchestrator-to-state-manager-relay-verification-gap` anchored S-15.03 PRIORITY-A 2026-08-13 | An unverified numeric/attributional claim in an orchestrator dispatch instruction was persisted by state-manager without independent re-derivation (D-966 F-006 precision note, corrected D-967). Extends POLICY 22's ratification-channel discipline one layer down to the dispatch layer. |
| **[D-994] ADR-040 partial-fix reconciliation recurrence risk (POLICY 4 S-7.01)** | **CODIFIED — `L-BB-adr-reconciliation-sweep-scope-on-ratification` anchored S-15.03 PRIORITY-A 2026-08-13** | ADR ratification/reconciliation must sweep the ENTIRE ADR body for live "Do NOT apply" directives, not only the §Status paragraph — nearly recurred within the same cascade that had just done it completely for ADR-041/042. See D-994(e). |
| **[D-995] governing-BC normative-prose bump has no story-propagation-enqueue convention (POLICY 8 companion)** | **CODIFIED — `L-BB-story-propagation-obligation-on-governing-bc-normative-prose-amendment` anchored S-15.03 PRIORITY-A 2026-08-13** | A story that copies normative BC prose (not just a bare version-cite) can silently lag the governing BC across multiple versions with zero drift signal — POLICY 8's version-cite check does not reach copied-prose staleness. See D-995(d). |
| **[D-996] fix-scoped-to-named-site-not-defect-class recurs a third time in this cascade (POLICY 8/TD-VSDD-060 companion)** | **CODIFIED — `L-BB-retracted-claim-class-complete-sibling-sweep-on-fuel-claim-amendment` anchored S-15.03 PRIORITY-A 2026-08-13** | D-995's F-S2107-P12-001 fix corrected only §AC-020 Notes, leaving the Out-of-Scope row + AC-019 (story-side) and the coverage blockquote (STORY-INDEX-side) carrying the same retracted premise/stale pin, caught next pass as F-S2107-P13-001/002. After any retraction/supersession fix, the fixer MUST grep the WHOLE affected artifact (backtick-tolerant) and every index aggregation cell, not just the finding's named site. See D-996(e). |
| **[D-998] fix-scoped-to-named-cell-not-every-blockquote recurs a fourth time in this cascade (POLICY 5/TD-VSDD-060 companion)** | **CODIFIED — `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` anchored S-15.03 PRIORITY-A 2026-08-13** | An epic can carry several independently-maintained aggregation-total blockquotes (authored-provenance, delivery, coverage) computing the same total over the same story set; a review or fix that spot-checks only one cell can leave a sibling stale for multiple passes (F-S2107-P15-001 survived passes 13-14). See D-998(e). |
| **[D-1000] the D-998 sweep GATE ITSELF was scoped to a named phrasing, not the semantic class — a fifth-generation recurrence one level up (POLICY 5/TD-VSDD-060 companion)** | **CODIFIED — `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` STRENGTHENED (same lesson, D-1000 amendment) anchored S-15.03 PRIORITY-A 2026-08-13** | D-998's sweep operationalized its own obligation via a single grep phrasing (`stories total.`), which structurally could not catch DAG-header, footnote, or master-line cells asserting the identical total under different surface forms — 3 stale E-21 cells and 1 newly-discovered stale E-19 cell survived the "class-complete" sweep that was supposed to catch exactly this. The lesson is now stated by SEMANTIC ROLE (any total-bearing cell, any form) rather than by grep phrasing. See D-1000(e). |
| **[D-1004] Completeness ATTESTATION narrower than the SWEEP CLAIMED — attestation-scoping gap one layer inside fix-scoping (POLICY 5/TD-VSDD-060/POLICY 15/TD-VSDD-059 companion)** | **CLOSED D-1004** | D-1003's story-writer fix burst's own "zero live v1.18 residuals confirmed by story-wide grep sweep" claim was FALSE — the grep predicate used structurally could not match a line-wrapped cite or a bare-token provenance-boundary citation. CLOSED same-burst; lesson `L-BB-attestation-predicate-must-be-whitespace-tolerant-and-line-wrap-aware` CODIFIED. |
| **[D-1005] Pass-20 CLEAN — F-S2107-P19-001 closure holds, zero new findings** | **VERIFIED D-1005** | Fresh context independently re-confirmed D-1004's fix is genuinely complete — no adjacent gap found on this axis. No new lesson filed. |
| **[D-1006] Task 10 version-cite-propagates/algorithm-content-does-not gap — sixth-generation recurrence of the fix-scoped-to-named-site failure shape, one layer further in (POLICY 4/POLICY 8/TD-VSDD-060 companion)** | **CLOSED D-1006** | Task 10's build directive correctly advanced its cited BC VERSION NUMBER across six consecutive propagation sweeps (v1.4-v1.13) while its embedded REGEX CONTENT silently diverged from what that version specified, undetected for seven story-versions (v1.8-v1.14). CLOSED same-burst; lesson `L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers` CODIFIED — distinct from the D-1004 sweep-predicate-adequacy lesson: this is about a build directive's CONTENT never having been re-derived from its cited source, not about a verification predicate too narrow to detect an already-known defect. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-14 — HEAD `27877f9a`; PIPELINE ACTIVE; D-1006 pass-21 record+fix burst COMPLETE — 1 HIGH CLOSED, streak RESETS 1/3 → 0/3; SHA-patch done; pass-22 adversary NEXT)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. S-21.09 is **MERGED** (PR #775, `2e8087af`, UNCHANGED by this burst). The prior burst (D-1006) was a record+fix adversary burst: fresh-context `vsdd-factory:adversary` pass-21 dispatched against `feature/S-21.07` at `96b4be19` and `factory-artifacts` at `e9fd7607` (the D-1005 SHA-patch HEAD). Verdict **NOT-CLEAN — 1 HIGH finding (F-S2107-P21-001)**, CLOSED THIS BURST. 4-INDEX: BC v4.59 (UNCHANGED) / VP v2.76 (UNCHANGED) / STORY v4.325 (UNCHANGED this write) / ARCH v3.58 (UNCHANGED). `policies.yaml` **v1.4.24** UNCHANGED. factory-artifacts HEAD `27877f9a` (D-1006 burst commit; SHA-patch done this write).

**Last decisions: D-1006.** Pass-21 NOT-CLEAN — 1 HIGH: story Task 10's `extract_story_bc_version_citations` build directive specified `\bv?([0-9]+\.[0-9]+)\b`, the single-pass bare-optional-v regex BC-5.39.010 v1.19 PC13 itself declares NON-CONFORMING, annotated "per BC-5.39.010 v1.19 PC13 (unchanged since v1.14)" — true of the cited version NUMBER, false of the regex CONTENT attached to it (which had silently diverged from the BC at v1.8, seven story-versions earlier). Task 10 contradicted both the governing BC and the story's own §BC Status section (correctly two-phase since v1.5). Blast radius: an implementer coding Task 10 verbatim would reintroduce a story-ID collision, a BC-section-number collision, and an ACs-column collision on S-21.07's OWN governing citation. Independent Task-10 verify performed BEFORE bundling (defense-in-depth): confirmed the two-phase algorithm now live at Task 10 (Phase 1 `^v?([0-9]+\.[0-9]+)$`; Phase 2 `\bv([0-9]+\.[0-9]+)\b`), confirmed zero remaining live bare-form directive sites (4 residual hits are all narrative/rationale prose describing the rejected form, not directives), confirmed input-hash UNCHANGED `93c4a89` via `compute-input-hash --check` exit 0. CLOSED THIS BURST via story-writer (S-21.07 v1.13→v1.14). STORY-INDEX v4.324→v4.325 (catalog row story-version 1.13→1.14; frontmatter self-bumped same-commit, no parity gap this time). Independent CLEAN axes re-derived: version parity chain, three-way input-hash `93c4a89`, POLICY 7 H1 SoT, F-S2107-P18-001/F-S2107-P19-001 closures both hold, E-21 aggregation 14/117/8, retracted-claim class zero live members. REC-P21-A (test-coverage recommendation) and OBS-P21-B (cosmetic bracket-count mismatch) recorded as tracked non-blocking items in §8 below, NOT actioned this burst — the human directive was to keep the story stable. Lesson `L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers` CODIFIED (D-1006) — distinct from D-1004's sweep-predicate-adequacy lesson: this is about a build directive's own CONTENT never having been re-derived from its cited BC source, not about a verification predicate too narrow to detect an already-known defect. **STREAK RESETS 1/3 → 0/3** — the finding is IN-PERIMETER (a live spec-implementation contradiction inside S-21.07's own build directive), not an out-of-perimeter carve-out. BC-5.39.001 requires 3 FRESH CONSECUTIVE CLEAN passes to converge — **pass-22, pass-23, pass-24 all required from here.** `feature/S-21.07` SHA `96b4be19` UNCHANGED (no code-repo commit — fix is spec-only). BC-INDEX/VP-INDEX/ARCH-INDEX all UNCHANGED — nothing was touched this burst beyond STORY-INDEX and the story itself. This SHA-patch write updates the Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-1006 row to the D-1006 commit's own actual HEAD (`27877f9a`) — no content change beyond the SHA-patch. Carried forward unaffected: O-P17-01 (STORY-INDEX master-total stale-floor, §8 item 7 below) and O-P17-02 (BC-INDEX epic-column convention, §8 item 8 below) remain open, pending human intent-confirmation; the E-18 STORY-INDEX total-drift question (§8 item 7 companion) also remains open.

### §2 S-21.09 — MERGED (Unrelated to This Burst, Carried Forward)

**MERGED.** PR #775 → `develop`, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Story spec v1.32, impl `c20cf2fe`, LOCAL BC-5.39.001 streak **3/3 RE-CONVERGED (D-988), PRESERVED through D-989** — final state at merge, UNCHANGED by this burst. No further work owed.

### §3 `feature/S-21.07` — Sequenced Next, NOT Merge-Ready, Streak RESET 0/3, Pass-22 Is the Pending Gate

`feature/S-21.07` (branch `feature/S-21.07-validate-cross-site-correspondence`, SHA **`96b4be19`**, UNCHANGED this burst — no code-repo commit, fix is spec-only) remains UNFROZEN and sequenced next for E-21 W4. Pass-21 was **NOT-CLEAN** — 1 HIGH finding, CLOSED THIS BURST — and the LOCAL BC-5.39.001 streak **RESETS 1/3 → 0/3** (in-perimeter HIGH; the finding is a live spec-implementation contradiction inside S-21.07's own Task 10 build directive, not an out-of-perimeter carve-out). The branch is **still explicitly NOT merge-ready**: convergence requires 3 FRESH CONSECUTIVE CLEAN adversary verdicts (pass-22, pass-23, pass-24). A single finding at any of those three resets the streak again. **The next substantive action is to dispatch `vsdd-factory:adversary` fresh-context for pass-22**, reading only `adversary-pass-21.md` Part A per the Iron Law, against the state landed at D-1006 (S-21.07 v1.14, STORY-INDEX v4.325, BC-5.39.010 v1.19 UNCHANGED, `feature/S-21.07`'s unchanged `96b4be19`). D-1006's SHA-patch follow-up is **COMPLETE** — no precondition blocks the pass-22 dispatch.

### §4 F-001's Residual — the Only Genuinely Open Pass-10-Class Item

F-S2107-P10-001 (originally BLOCKER, vacuous POLICY 15 gate) is **ALREADY-RESOLVED at the design level**: ADR-040 v1.12/v1.13/v1.15/v1.16 RATIFIED/AMENDED, root cause (category error — gate evaluated in the wrong worktree) diagnosed and redesigned, mechanism implemented as `crates/policy15-attestation-gate/` (16 tests, mutation-verified) on branch `feature/policy15-gate-rust` at `d2a3176a` (pushed, no PR). **What remains open is purely operational: the CI job that makes the gate demonstrably RUNNING is not yet wired.** This is Drift Item `[D-969]`, routed to `devops-engineer`, and is **BLOCKED-ON `feature/policy15-gate-rust` merging to `develop`** — note this is a SEPARATE branch from `feature/S-21.07`, with its own independent merge path and no open PR. Do not conflate the two branches' merge readiness. UNCHANGED by this burst.

### §5 ADR-043

v1.5, `status: proposed`, **NOT RATIFIED**. Three fresh-context DO-NOT-RATIFY reviews (4, then 10, then 9 blockers) then amended. POLICY 22 requires human ratification. Reviews persisted as `adv-adr-043-pass-{1,2,3}.md`. UNCHANGED by this burst.

### §6 Blocking Issues

- **C-1 CWE-706** — `binary_allow` basename allow-list escape (structural HIGH / practical LOW). **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.**
- **C-2 CWE-362** — TOCTOU window; ADR-043 threat model boundary unformalized.
- **C-4 CWE-284** — prefix list empty/writable fallthrough; BC amendment pending.
- **C-5 CWE-284** — no per-entry resource limits; anchor S-21.14.
- **POLICY 15 gate — CI-wiring residual only (D-992).** See §4.
- **4 pass-10 carry-over findings** (MED-001, LOW-001/002/003, from the S-21.09 cascade) — anchor: next maintenance sweep; NOT a blocker on anything.
- **`feature/S-21.07` pass-21 NOT-CLEAN, streak RESETS 0/3 (D-1006)** — sequenced next; **pass-22 adversary dispatch is the immediate next substantive pipeline action.**
- **E-18 STORY-INDEX total drift (107 vs 125 pts)** — out-of-perimeter, does not block anything; master-line layer RESOLVED D-1000; deeper catalog-count question remains open; anchor: next maintenance sweep or explicit human direction.

### §7 Infrastructure Blockers

(a) **STATE.md narrative sections** — full-file Write convention continues (`verify-state-timestamp-refresh` guard requires a `timestamp:` advance within EVERY individual Edit/Write call's own diff, confirmed to apply even to small isolated Edits with no other content change).
(b) **`STORY-INDEX.md`** triggers advisory PostToolUse fuel/timeout signals on every edit — NOT touched this burst (SHA-patch only touches STATE.md).
(c) **`decision-log.md`/`burst-log.md`/`lessons.md` exhaust WASM validator fuel on every edit** — NOT touched this burst (SHA-patch only touches STATE.md).
(d) **`validate-trajectory-tail-cell-completeness` (D-453(d))**: requires a 4-value trajectory-tail arrow-sequence present in BOTH the frontmatter `current_step` AND the Project Metadata `Last Updated` cell on every STATE.md write. Applied again this burst (`→1→1→0→1` UNCHANGED).
(e) **`validate-dispatch-advance` bare-sentinel-token scan (ADR-041 §Decision 4)**: no D-999-adjacent narrative drafted this burst — not applicable.
(f) **SHA-patch follow-up — DONE.** Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-1006 row's "Full detail" cite updated `e9fd7607`→`27877f9a` (the D-1006 burst commit's own actual HEAD) in this follow-up write, landed in the immediate follow-up commit after the D-1006 burst commit's push.
(g) **`validate-table-cell-count` gate**: any literal shell snippet cited inside a table cell that contains a pipe character must be pipe-escaped (`` \| ``) or rephrased.
(h) **Large-line frontmatter fields resist the `Read` tool.** `stories/STORY-INDEX.md`'s `last_amended:` field is a single large line (many nested `[Prior: ...]` wraps, now 231,052 characters) — `Read` cannot load it even with `offset`/`limit` (per-line token cap). This burst used `Bash`/`python3` to locate byte offsets and construct small, uniquely-matching `old_string`/`new_string` anchors for `Edit`, rather than reading the full line. Not applicable this SHA-patch write — STORY-INDEX not touched.

### §8 Pending Human Decisions

1. **`feature/S-21.07` adversarial correction cascade** — dispatch adversary fresh-context against the state after D-1006 (pass-22); this is now the sequenced-next E-21 W4 action. Streak is **0/3 (RESET)** — 3 fresh CONSECUTIVE CLEAN passes needed to converge, starting from pass-22.
2. **ADR-043 ratification** — v1.5 converged (no BLOCKERs per pass-3); human to decide: ratify v1.5 / request pass-4 / redirect design. UNCHANGED.
3. **S-21.12 blocker B1** — `cargo deny` has 5 advisories (not 2); `deny.toml` read-only; `async-std` no upgrade path; AC-004 unsatisfiable. UNCHANGED.
4. **Four orphan advisories + 18 Dependabot alerts** — scope assignment pending (E-22 or dedicated fix). UNCHANGED.
5. **github-ops push-delegate reliability** — investigate root cause of mid-session push failures (S-15.03 PRIORITY-A or dedicated devops follow-up). UNCHANGED from D-989.
6. **`merged-stories-ledger.md` backfill (S-19.04..S-21.08)** — scope a dedicated maintenance sweep, or accept the gap as permanent-historical. UNCHANGED from D-991.
7. **E-18 STORY-INDEX total drift (107 vs 125 pts)** — **PARTIALLY RESOLVED D-1000**: the master "Total story points" line (L763) now correctly cites E-18's own canonical delivery-blockquote total (107), closing the narrower master-line-vs-canonical-blockquote disagreement this item originally flagged at D-998. The residual, still-open question is deeper: whether 107 (the blockquote, frozen at epic completion) should itself be reconciled against the current 18-row/125-pt catalog sum (which includes two `-prereq` stub rows the original blockquote may not have counted as full stories). **Companion (D-1001, O-P17-01):** the master line's OWN leading aggregate ("Total story points: 533+ across 136 stories", same L763) is itself a stale floor against the sum of its own per-epic terms (~630) — a related but distinct cross-epic hygiene question, also anchored here. Human to decide: scope a dedicated maintenance sweep covering both, accept as permanent-historical estimate-vs-actual, or explicitly direct a reconciliation edit. UNCHANGED by D-1006.
8. **(D-1001) O-P17-02 — BC-INDEX BC-5.39.010 epic-column convention** — row shows `E-12` (the engine-governance epic owning the validate-* hook family) while the anchoring story is `S-21.07` (E-21); matches the entire sibling BC-5.39.003-008 validate-hook cohort, stable across all 20 prior passes now. Human to confirm: is this an intentional "owning epic ≠ implementing story's epic" convention (in which case no action needed, disposition this item CLOSED-AS-INTENTIONAL), or should it be corrected to E-21? UNCHANGED by D-1006.
9. ~~**(D-1001) STORY-INDEX.md frontmatter version/body-content parity gap**~~ — **RESOLVED D-1002.** No further human decision needed; item retained here for audit-trail continuity.
10. **(D-1006) REC-P21-A — Arm A2 collision-avoidance negative test** [tracked recommendation, non-blocking, NOT a pending decision requiring adjudication]: no corpus-style negative regression test analogous to BC-5.39.010's own `corpus_arm_a1` exists for Arm A2's collision-avoidance behavior — S-21.07's own BC-table ACs cell "DEFERRED v1.6" is a live instance of the ACs-column collision hazard the two-phase algorithm exists to avoid. Recommend a negative test (e.g. `test_BC_5_39_010_arm_a2_no_false_block_on_acs_column`) anchored to S-21.07's implementation phase (test-writer, when the story is implemented) or a future adversary pass. Recorded per human direction to track without actioning this burst (keep the story stable). UNCHANGED by this SHA-patch write.
11. **(D-1006) OBS-P21-B — cosmetic bracket-count mismatch** [tracked observation, non-blocking, NOT a pending decision requiring adjudication]: a pre-existing bracket-count mismatch was observed in the append-only `[Prior:...]` nesting chains of both S-21.07's and BC-5.39.010's `last_amended:` fields (opener count ≠ trailing `]` count). Harmless — the field is an opaque YAML block-scalar string, not machine-parsed nesting, so there is no parse impact — and predates this fix. Not corrected per POLICY 1 (append-only history is not edited). Recorded per human direction to track without actioning this burst. UNCHANGED by this SHA-patch write.

### §9 Follow-up Stories Registered (Unrelated to This Burst, Carried Forward)

- **S-21.14** (W8, 8 pts): release-pipeline weak-predicate sweep across 5 sites + resolver-arm floor + T-017 first-match extractor + artifact-freshness gate.
- **S-21.15** (W8, 5 pts): `compute-input-hash` search-path gap + `traces_to:` bare-filename question.

### §10 Resume Command

`/vsdd-factory:next-step` — **S-21.09 is MERGED and CLOSED** (PR #775, `2e8087af`); no further action owed. The D-1006 pass-21 record+fix burst is **COMPLETE** — 1 HIGH finding (F-S2107-P21-001) CLOSED THIS BURST (S-21.07 v1.13→v1.14, Task 10 two-phase rewrite; STORY-INDEX v4.324→v4.325); REC-P21-A/OBS-P21-B tracked in §8, not actioned. **LOCAL BC-5.39.001 streak for the S-21.07 cascade RESETS 1/3 → 0/3** — in-perimeter HIGH. The pipeline is **ACTIVE**. **The immediate next substantive action is: dispatch `vsdd-factory:adversary` fresh-context for pass-22**, reading only `adversary-pass-21.md` Part A per the Iron Law, against the state landed at D-1006 (S-21.07 v1.14, STORY-INDEX v4.325, BC-5.39.010 v1.19 UNCHANGED, `feature/S-21.07`'s unchanged `96b4be19`). D-1006's SHA-patch follow-up is **COMPLETE** — Active Branches `factory-artifacts` row and this checkpoint's header now cite the actual commit HEAD `27877f9a`; no precondition blocks the pass-22 dispatch. **3 fresh CONSECUTIVE CLEAN passes (pass-22, pass-23, pass-24) are required to converge.** Separately, ADR-043 ratification, S-21.12's cargo-deny blocker, the `merged-stories-ledger.md` backfill scope decision, the (partially-resolved) E-18 STORY-INDEX total-drift scope decision (with its O-P17-01 companion), and the O-P17-02 BC-INDEX epic-column intent-confirmation remain open per §8, unaffected by this burst.
