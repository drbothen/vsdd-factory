---
document_type: pipeline-state
level: ops
version: "7.56"
status: draft
producer: state-manager
timestamp: 2026-08-13T23:20:00Z
phase: SHA-PATCH-2026-08-13-D-998
last_amended: "2026-08-13 (v7.56) — SHA-PATCH-2026-08-13-D-998 (state-manager; commit 1750bd56): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-998 row + all pending-SHA-patch cites updated 1199aae3→1750bd56 (the D-998 burst commit's own actual HEAD, confirmed via git rev-parse HEAD after push). No content change beyond the SHA-patch itself — D-998's substance (fresh-context adversary pass-15: NOT-CLEAN, 1 MEDIUM finding F-S2107-P15-001 + 3 non-blocking observations; F-S2107-P15-001 CLOSED — STORY-INDEX v4.320→v4.321, L721 111→117 pts, TD-VSDD-060 class-complete sweep; HUMAN PERIMETER DECISION IN-PERIMETER — STREAK RESET 1/3→0/3) is UNCHANGED. [Prior: 2026-08-13 (v7.54) — D-998-S2107-PASS15-RECORD-AND-FIX-BURST (state-manager; parent-commit 1199aae3): fresh-context adversary pass-15 dispatched against feature/S-21.07 96b4be19 (unchanged; story unbuilt) and factory-artifacts 1199aae3 (the D-997 SHA-patch HEAD) — NOT-CLEAN, 1 MEDIUM finding (F-S2107-P15-001: STORY-INDEX E-21 epic-total aggregation cells disagree, L721 authored-provenance blockquote tail 111 pts vs L741 delivery blockquote + catalog-row sum both 117 pts) + 3 non-blocking observations (O-P15-01/02/03). F-S2107-P15-001 CLOSED THIS BURST (state-manager; STORY-INDEX v4.320→v4.321, L721 111→117 pts; TD-VSDD-060 class-complete sweep across EVERY epic blockquote in STORY-INDEX confirmed zero further live disagreements — E-18's own frozen-historical 107-vs-125 variance noted out-of-perimeter per D-996(d) precedent, left untouched). HUMAN PERIMETER DECISION this session: F-S2107-P15-001 classified IN-PERIMETER — STREAK EXPLICITLY RESET 1/3 → 0/3, recorded as human-directed, not an automatic classification. O-P15-01 [process-gap] CODIFIED (lesson L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit + D-998(e)); O-P15-02 ACCEPTED-OBSERVATION-WITH-RATIONALE (not fixed, story kept stable); O-P15-03 dispositioned out-of-perimeter non-finding. `feature/S-21.07` SHA 96b4be19 UNCHANGED (story file itself NOT touched this burst, per instruction, to preserve its twice-CLEAN stability). BC-5.39.001 now requires 3 FRESH CONSECUTIVE CLEAN passes from pass-16 onward. Full prior-chain detail: decision-log.md D-996/D-997/D-998 + git show 1199aae3:.factory/STATE.md (pre-D-998 state, v7.53).]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "SHA-PATCH-2026-08-13-D-998 (state-manager; commit 1750bd56; D-chain cite D-967+D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998; trajectory-tail S-21.07 →1→2→0→1 UNCHANGED): Active Branches factory-artifacts row, Session Resume Checkpoint header, and Decisions Log D-998 row SHA-patched 1199aae3->1750bd56 (the D-998 burst commit's own actual HEAD, confirmed via git rev-parse HEAD after push). D-998 substance UNCHANGED by this patch: fresh-context adversary pass-15 executed as a single atomic commit per TD-VSDD-053 -- NOT-CLEAN, 1 MEDIUM finding (F-S2107-P15-001) + 3 non-blocking observations (O-P15-01/02/03); F-S2107-P15-001 CLOSED THIS BURST (STORY-INDEX v4.320->v4.321, E-21 authored-provenance blockquote L721 111->117 pts; TD-VSDD-060 class-complete epic-total aggregation sweep across EVERY epic blockquote in STORY-INDEX, zero further live disagreements). HUMAN PERIMETER DECISION: F-S2107-P15-001 classified IN-PERIMETER -- STREAK EXPLICITLY RESET 1/3 -> 0/3, recorded as human-directed, not an automatic state-manager classification. No gate predicate, GateOutcome semantics, or ADR ratification status changed; ARCH-INDEX untouched. This SHA-patch write is the standard immediate follow-up commit after the D-998 burst commit's push -- permitted per project convention as a SHA-patch, not a Stage-2 chain (TD-VSDD-053). NEXT: dispatch vsdd-factory:adversary fresh-context for pass-16."
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
  264 lines (wc-l post-D-997-S2107-PASS14-RECORD-ONLY-BURST 2026-08-13; pass-14 CLEAN 0 findings+3 obs, RECORD-ONLY — nothing to close; STREAK ADVANCES 0/3→1/3; v7.51→v7.52; parent-commit e5a1702b, own SHA pending push)
  264 lines (wc-l post-SHA-patch 8e5c7344 2026-08-13; Active Branches factory-artifacts e5a1702b→8e5c7344; v7.52→v7.53 UNCHANGED content)
  266 lines (wc-l post-D-998-S2107-PASS15-RECORD-AND-FIX-BURST 2026-08-13; pass-15 NOT-CLEAN 1 MEDIUM+3 obs; F-S2107-P15-001 CLOSED (STORY-INDEX v4.320→v4.321); STREAK RESET 1/3→0/3 human-directed; v7.53→v7.54; parent-commit 1199aae3, own SHA pending push)
  267 lines (wc-l post-SHA-patch 1750bd56 2026-08-13; Active Branches factory-artifacts 1199aae3→1750bd56; v7.54→v7.56 UNCHANGED content)
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
| **Last Updated** | 2026-08-13 — SHA-PATCH-2026-08-13-D-998 (commit `1750bd56`): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-998 row + all pending-SHA-patch cites updated 1199aae3→1750bd56. No content change beyond the SHA-patch itself — D-998's substance (fresh-context adversary pass-15: **NOT-CLEAN, 1 MEDIUM finding** F-S2107-P15-001 + 3 non-blocking observations; F-S2107-P15-001 CLOSED — STORY-INDEX v4.320→v4.321, L721 111→117 pts) is UNCHANGED. HUMAN PERIMETER DECISION: IN-PERIMETER — LOCAL BC-5.39.001 streak (S-21.07 cascade) **RESET 1/3 → 0/3** (human-directed). trajectory-tail (S-21.07) →1→2→0→1. |
| **Current Phase** | **SHA-PATCH-2026-08-13-D-998 (commit `1750bd56`; D-chain cite D-967+D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998; PIPELINE ACTIVE).** S-21.09 remains **MERGED** (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` is **UNFROZEN + sequenced-next**, still **NOT merge-ready** — streak RESET to 0/3, 3 FRESH CONSECUTIVE CLEAN passes required from pass-16 onward. 4-INDEX BC v4.58/VP v2.76/STORY **v4.321**/ARCH v3.58 (STORY UPDATED this burst; others UNCHANGED). policies.yaml v1.4.24 UNCHANGED. SHA-patch follow-up DONE this write. **Next substantive action: dispatch `vsdd-factory:adversary` fresh-context for pass-16** (reads only `adversary-pass-15.md` Part A per the Iron Law). |
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
| D-993-ADR-BODY-RECONCILIATION-BATCH 2026-08-13 (single commit TD-VSDD-053; commit `e85ff8cd`; SHA-patch done) | **COMPLETE** | Small consistency follow-on: ADR-040 v1.15 / ADR-041 v1.2 / ADR-042 v1.4 body reconciliation bundled verbatim (architect); ARCH-INDEX v3.57 (3 row notes); Drift Item `[D-992]` ADR-040 stale-line item RESOLVED. Streak EXPLICITLY UNCHANGED 0/3. STATE.md v7.45. |
| D-994-S2107-PASS11-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; commit `19932766`; SHA-patch done) | **COMPLETE** | Pass-11 NOT-CLEAN (1 MEDIUM F-S2107-P11-001 + 2 obs; **CLIFF** 10→1, deepest reduction of the cascade); F-S2107-P11-001 CLOSED (ADR-040 v1.16, architect); O-P11-01 CLOSED-DEFENSIVELY (ARCH-INDEX reframe); O-P11-02 CODIFIED (lesson + D-994(e)). ARCH-INDEX v3.58. **Streak EXPLICITLY REMAINS 0/3.** STATE.md v7.47. |
| D-995-S2107-PASS12-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; commit `9c132dd2`; SHA-patch done) | **COMPLETE** | Pass-12 NOT-CLEAN (1 MEDIUM F-S2107-P12-001 + 3 obs; locus moved ADR-layer→story-layer); F-S2107-P12-001 CLOSED (story-writer; S-21.07 v1.10, BC-5.39.010 v1.14→v1.18 propagation); O-P12-01/O-P12-02 verification (not findings); O-P12-03 CODIFIED (lesson + D-995(d)). STORY-INDEX v4.319. **Streak EXPLICITLY REMAINS 0/3.** STATE.md v7.49. |
| D-996-S2107-PASS13-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; commit `00cbc4ea`; SHA-patch done) | **COMPLETE** | Pass-13 NOT-CLEAN (2 MEDIUM F-S2107-P13-001+F-S2107-P13-002 + 3 obs; both un-swept siblings of D-995's fix, S-7.01 partial-fix regression); F-S2107-P13-001 CLOSED (story-writer; S-21.07 v1.11, story-wide sibling sweep); F-S2107-P13-002 CLOSED (state-manager; STORY-INDEX v4.320, coverage-blockquote fix + self-discovered BC-4.16.001 sibling, TD-VSDD-060); O-P13-01 CODIFIED (lesson + D-996(e)). **Streak EXPLICITLY REMAINS 0/3; pass-14 adversary NEXT.** STATE.md v7.51. |
| D-997-S2107-PASS14-RECORD-ONLY-BURST 2026-08-13 (single commit TD-VSDD-053; commit `8e5c7344`; SHA-patch done) | **COMPLETE** | Pass-14 **CLEAN** (0 findings + 3 non-blocking obs, none codified); F-S2107-P13-001 + F-S2107-P13-002 independently re-verified RESOLVED by fresh context. RECORD-ONLY burst — no fix owed, nothing to close. **STREAK ADVANCES 0/3 → 1/3; pass-15 adversary NEXT.** STATE.md v7.53. |
| D-998-S2107-PASS15-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; commit `1750bd56`; SHA-patch done) | **COMPLETE** | Pass-15 NOT-CLEAN (1 MEDIUM F-S2107-P15-001 + 3 obs); F-S2107-P15-001 CLOSED (state-manager; STORY-INDEX v4.320→v4.321 — E-21 authored-provenance blockquote L721 111→117 pts; TD-VSDD-060 class-complete epic-total aggregation sweep, zero further live disagreements); O-P15-01 CODIFIED (lesson + D-998(e)). **HUMAN PERIMETER DECISION: IN-PERIMETER — STREAK RESET 1/3→0/3 (human-directed); pass-16 adversary NEXT (3 fresh CONSECUTIVE CLEAN required).** STATE.md v7.56. |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-997 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| SHA-PATCH-2026-08-13-D-997 (state-manager; commit 8e5c7344) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-997 row SHA-patched `e5a1702b`→`8e5c7344`; STATE.md v7.52→v7.53; no content change. |
| D-998-S2107-PASS15-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent-commit 1199aae3; commit 1750bd56) | state-manager | COMPLETE | Pass-15 NOT-CLEAN (1 MEDIUM F-S2107-P15-001 + 3 obs) persisted verbatim; F-S2107-P15-001 CLOSED (STORY-INDEX v4.320→v4.321, L721 111→117 pts, TD-VSDD-060 class-complete epic-total sweep); O-P15-01 CODIFIED. **HUMAN PERIMETER DECISION IN-PERIMETER — STREAK RESET 1/3→0/3.** STATE.md v7.53→v7.54. |
| SHA-PATCH-2026-08-13-D-998 (state-manager; commit 1750bd56) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-998 row SHA-patched `1199aae3`→`1750bd56`; STATE.md v7.54→v7.56; no content change. **NEXT: adversary pass-16 dispatch (fresh-context, reads adversary-pass-15.md Part A only).** |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.58 D-992, UNCHANGED this burst) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX v4.321 D-998 — E-21 authored-provenance blockquote arithmetic fix, no story registered/removed) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-040 v1.16 D-994 pass-11 fix / ADR-041 v1.2 / ADR-042 v1.4 D-993 body reconciliation, UNCHANGED this burst; ADR-043 proposed NOT RATIFIED) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | **108** (STATE.md explicit counter; sprint-state predicate tracked separately per canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (108):** S-21.09 MERGED PR #775 `2e8087af` 2026-08-13 (validate-factory-path-staging WASM artifact restore + registry parity CI check; E-21 W4). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md` (known gap: not appended between S-19.03 and S-21.09; see ledger file note — anchored to a dedicated maintenance sweep).
- **In-Flight (0):** none.
- **E-21:** S-21.07 (W4; **sequenced next**, UNFROZEN; pass-15 **NOT-CLEAN** — 1 MEDIUM (F-S2107-P15-001, CLOSED this burst) + 3 obs, **STREAK RESET 1/3→0/3** (human perimeter decision, IN-PERIMETER; 3 FRESH CONSECUTIVE CLEAN required from pass-16); story spec v1.11 UNCHANGED this burst (fix is STORY-INDEX-only, story file deliberately not touched to preserve its twice-CLEAN stability); branch `96b4be19` pushed (unchanged, code-repo not touched); **NOT merge-ready**; pass-16 adversary dispatch is the pending gate); S-21.09 (**MERGED** PR #775 `2e8087af`); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | **2e8087af** | PR #775 (`feature/S-21.09`) merged 2026-08-13T14:16:26Z; `validate-factory-path-staging.wasm` git-tracked. Pull on next code-worktree resume. |
| factory-artifacts | **1750bd56** | D-998-S2107-PASS15-RECORD-AND-FIX-BURST. SHA-patch done 2026-08-13. |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. **F-001's sole remaining residual (D-992) is BLOCKED-ON this branch merging to `develop`** — routed devops-engineer, anchored Drift Item `[D-969]`. |
| feature/S-21.09 | c20cf2fe | **MERGED** to `develop` via PR #775, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Branch ref retained (standard post-merge retention). LOCAL BC-5.39.001 streak 3/3 RE-CONVERGED (D-988), PRESERVED through D-989 — final state at merge. |
| feature/S-21.07-validate-cross-site-correspondence | **96b4be19** | pass-15 **NOT-CLEAN** (1 MEDIUM, CLOSED, D-998) — **STREAK RESET 0/3** (human perimeter decision, IN-PERIMETER); UNCHANGED this burst (no code-repo commit; fix is STORY-INDEX-only). Pushed; SHA-equal with origin. Still UNFROZEN + sequenced-next, **NOT merge-ready** — convergence depends on **adversary pass-16 (NEXT)**, 3 FRESH CONSECUTIVE CLEAN required. |
| feature/S-21.04 | 323f440f | pass-31 pending; no PR. Pushed; SHA-equal with origin. |
| fix/nested-factory-path-derivation | 9afc3226 | F-S2107-P8-016 + P9-008 CLOSED. Pushed; SHA-equal with origin. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel. Pushed; SHA-equal with origin. |
| fix/fuel-exhaustion-fail-loud | fbb9dcb6 | ABANDONED — orchestrator dispatch error (87 files duplicating unmerged S-21.07). CONFIRMED SUPERSEDED by PR #774 (`62fbcf1a`, D-992 re-verification). Local-only; deliberately NOT pushed. |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-998-S2107-PASS15-RECORD-AND-FIX-BURST COMPLETE (SHA-patch done 1750bd56). S-21.09 **MERGED** to `develop` (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` pass-15 (**NOT-CLEAN**, 1 MEDIUM, D-998): F-S2107-P15-001 CLOSED (STORY-INDEX v4.320→v4.321); SHA `96b4be19` UNCHANGED; story spec v1.11 UNCHANGED. **NOT merge-ready — STREAK RESET 0/3 (human perimeter decision); pass-16 adversary dispatch is the pending gate.** No ADR touched this burst. `develop` **2e8087af**; main 80e5cd7b; `merged_count` **108**; BC v4.58; VP v2.76; STORY **v4.321**; ARCH v3.58 (UNCHANGED); policies.yaml v1.4.24 UNCHANGED; ADR-043 proposed NOT RATIFIED. F-001 redesign RATIFIED (ADR-040 v1.12/v1.13/v1.15/v1.16) — CI wiring still PENDING, BLOCKED-ON `feature/policy15-gate-rust`→`develop`. LOCAL BC-5.39.001 streak (S-21.07 cascade) **0/3 — RESET from 1/3 (human-directed, IN-PERIMETER)**. trajectory-tail (S-21.07) →1→2→0→1. | SHA-patch done 1750bd56 2026-08-13; D-998-S2107-PASS15-RECORD-AND-FIX-BURST 2026-08-13. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range; exhaustive): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range; exhaustive) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-998 (see decision-log.md for full range; exhaustive): this Decisions Log (D-998 live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-998 | D-998-S2107-PASS15-RECORD-AND-FIX-BURST (state-manager; single-commit TD-VSDD-053 2026-08-13; parent-commit 1199aae3; commit 1750bd56; SHA-patch done). Fresh-context adversary pass-15 dispatched against `feature/S-21.07` `96b4be19` (unchanged; story unbuilt) and `factory-artifacts` `1199aae3` (D-997 SHA-patch HEAD, carrying story v1.11, BC-5.39.010 v1.18, BC-INDEX v4.58, STORY-INDEX v4.320 as landed) — **NOT-CLEAN, 1 MEDIUM finding (F-S2107-P15-001) + 3 non-blocking observations**; persisted verbatim to `cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-15.md`. F-S2107-P15-001 (POLICY 5 v1.3.7 sibling-sweep category (i) + TD-VSDD-060): STORY-INDEX §Epic E-21 authored-provenance blockquote tail (L721) stated "111 pts" while the delivery blockquote (L741) and the catalog-row sum (L726-739) both correctly compute "117 pts" for the identical 14-story E-21 set — same-file two-cell disagreement. **CLOSED THIS BURST:** STORY-INDEX.md v4.320→**v4.321** — L721 111→117 pts. TD-VSDD-060 class-complete literal-shell sweep executed across EVERY epic blockquote in STORY-INDEX carrying a stated points/story-count total: only 3 such live terminal-total cells exist file-wide (E-18, E-19, E-21); E-19 and E-21's delivery blockquote were already catalog-correct; E-21's authored-provenance sibling was the sole disagreement, now fixed; E-18's own total (107 pts) also disagrees with its current catalog sum (125 pts) but — per the D-996(d) precedent from one pass earlier in this cascade — is a frozen-historical record of a COMPLETE/merged epic outside this cascade's perimeter, left untouched and recorded transparently as an observation. **HUMAN PERIMETER DECISION this session: F-S2107-P15-001 classified IN-PERIMETER** (an epic aggregation cell, not S-21.07's own datum — S-21.07's own points figure is correct everywhere) — **STREAK EXPLICITLY RESET 1/3 → 0/3, recorded as human-directed, not an automatic state-manager classification.** O-P15-01 [process-gap] CODIFIED (lesson `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` + D-998(e) — a single-cell spot-check is insufficient; every epic blockquote's points/count/wave total must be summed and diffed against catalog rows). O-P15-02 ACCEPTED-OBSERVATION-WITH-RATIONALE (AC-020 Notes illustrative grep missing `-E`, non-load-bearing, NOT fixed this burst to keep the twice-CLEAN story stable). O-P15-03 dispositioned out-of-perimeter non-finding (POLICY 8 EC-mirror is story→BC directional). `policies.yaml` v1.4.24 UNCHANGED. BC-INDEX v4.58 UNCHANGED; VP-INDEX v2.76 UNCHANGED; STORY-INDEX v4.320→**v4.321**; ARCH-INDEX v3.58 UNCHANGED. Trajectory `47→18→25→25→24→20→16→8→10→1→1→2→0→1` (tail `→1→2→0→1`); 14 true adversary reviews, 1 CLEAN verdict (unchanged). **`feature/S-21.07` SHA `96b4be19` UNCHANGED (story file NOT touched, per instruction) — pass-16 adversary NEXT, 3 FRESH CONSECUTIVE CLEAN required.** Full detail: `git -C .factory show 1750bd56:.factory/STATE.md` + `decision-log.md` D-998 + `burst-log.md` `## D-998-S2107-PASS15-RECORD-AND-FIX-BURST`. | pass-15 NOT-CLEAN 1 MEDIUM+3 obs; F-S2107-P15-001 CLOSED (STORY-INDEX v4.321); HUMAN PERIMETER DECISION IN-PERIMETER — STREAK RESET 1/3→0/3 | D-998-S2107-PASS15-RECORD-AND-FIX-BURST | 2026-08-13 |
| D-413..D-998 (see decision-log.md for full range; exhaustive) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-13 |

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
| **[pass-10 carry-over] ADV-BB-P10-MED-001 directory-only `hook-plugins/sub/` staging control** | **OPEN — preserved through D-998; does NOT block anything** | Gate (a)'s ">= 1 component after hook-plugins/" threshold admits directory-only declarations; spurious MISSING report possible. Anchor: next maintenance sweep. |
| **[pass-10 carry-over] ADV-BB-P10-LOW-001/002/003 (NUL/trailing-space names; fail-open arms; `workspace_root()` untested)** | **OPEN — preserved through D-998; does NOT block anything** | Low-severity residuals from the S-21.09 cascade's pass-10; not addressed through the merge or this burst. Anchor: next maintenance sweep. |
| **[D-998] `feature/S-21.07` pass-15 NOT-CLEAN — streak RESET to 0/3, 3 FRESH CONSECUTIVE CLEAN required** | **OPEN — sequenced next, NOT merge-ready** | `feature/S-21.07` SHA `96b4be19` UNCHANGED; story spec v1.11 UNCHANGED (fix is STORY-INDEX-only). HUMAN PERIMETER DECISION classified F-S2107-P15-001 IN-PERIMETER, resetting the streak. **Pass-16 adversary dispatch (fresh-context, reads `adversary-pass-15.md` Part A only) is the pending gate.** |
| **[D-998] E-18 STORY-INDEX delivery-blockquote total (107 pts) disagrees with current catalog sum (125 pts)** | **OPEN — OUT-OF-PERIMETER, does NOT block anything** | Frozen-historical record of a COMPLETE/merged epic, per D-996(d) precedent; left untouched this burst. Anchor: next maintenance sweep or explicit human direction to reconcile. |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67..74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754. |
| **TD-VSDD-061/062/063** | OPEN 2026-05-17/19 | validate-index-cite-refresh large-file fail-open; schema inconsistencies; VP allocation deferred. |
| **TD-VSDD-095..100** | CODIFIED-AND-FORWARDED | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101** | OPEN 2026-05-18 — anchored S-15.15 | VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI; dangling ref confirmed D-972. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | check_index_sites + rows_after_heading advisory-arm defects. |
| **[D-945] VP-102..VP-118 pending allocation** | DEFERRED — anchored `feature/S-21.07` post-convergence | 17 VPs per BC-5.39.010 §VP Anchors. |
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

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-13 — HEAD `1750bd56`; PIPELINE ACTIVE; D-998 pass-15 record+fix COMPLETE; SHA-patch done; pass-16 adversary NEXT)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. S-21.09 is **MERGED** (PR #775, `2e8087af`, UNCHANGED by this burst). The prior burst (D-998) dispatched fresh-context adversary pass-15 against `feature/S-21.07` at `96b4be19` and `factory-artifacts` at `1199aae3` (the D-997 SHA-patch HEAD) — verdict **NOT-CLEAN, 1 MEDIUM finding (F-S2107-P15-001) + 3 non-blocking observations (O-P15-01, O-P15-02, O-P15-03)**. F-S2107-P15-001 was CLOSED same burst. 4-INDEX: BC **v4.58** / VP v2.76 / STORY **v4.321** (UPDATED) / ARCH v3.58 (UNCHANGED). `policies.yaml` **v1.4.24** UNCHANGED. factory-artifacts HEAD `1750bd56` (D-998 burst commit; SHA-patch done this write).

**Last decisions: D-998.** F-S2107-P15-001 (POLICY 5 v1.3.7 sibling-sweep category (i) + TD-VSDD-060): STORY-INDEX §Epic E-21 authored-provenance blockquote tail (L721) stated "111 pts" while the delivery blockquote (L741) and the catalog-row sum (L726-739) both correctly compute "117 pts" for the identical 14-story E-21 set. Fixed: STORY-INDEX.md v4.320→v4.321, L721 111→117. A TD-VSDD-060 class-complete literal-shell sweep was executed across EVERY epic blockquote in STORY-INDEX carrying a stated points/story-count total — only 3 such live cells exist file-wide (E-18, E-19, E-21); E-19 and E-21's delivery blockquote were already correct; E-21's authored-provenance sibling was the sole disagreement, now fixed; E-18's own total (107 pts) also disagrees with its current catalog sum (125 pts) but is a frozen-historical record of a COMPLETE/merged epic outside this cascade's perimeter (per the D-996(d) precedent), left untouched and recorded transparently. **HUMAN PERIMETER DECISION this session: F-S2107-P15-001 classified IN-PERIMETER — LOCAL BC-5.39.001 streak for the S-21.07 cascade EXPLICITLY RESET 1/3 → 0/3, recorded as human-directed, not an automatic state-manager classification.** BC-5.39.001 now requires 3 FRESH CONSECUTIVE CLEAN passes from pass-16 onward. O-P15-01 [process-gap] CODIFIED (lesson `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` + D-998(e)). O-P15-02 ACCEPTED-OBSERVATION-WITH-RATIONALE (not fixed, story kept stable). O-P15-03 dispositioned out-of-perimeter non-finding. The story file itself (`S-21.07-validate-cross-site-correspondence.md`) was explicitly NOT touched this burst, per instruction, to preserve its twice-CLEAN content stability. This SHA-patch write updates the Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-998 row to the D-998 commit's own actual HEAD (`1750bd56`) — no content change beyond the SHA-patch.

### §2 S-21.09 — MERGED (Unrelated to This Burst, Carried Forward)

**MERGED.** PR #775 → `develop`, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Story spec v1.32, impl `c20cf2fe`, LOCAL BC-5.39.001 streak **3/3 RE-CONVERGED (D-988), PRESERVED through D-989** — final state at merge, UNCHANGED by this burst. No further work owed.

### §3 `feature/S-21.07` — Sequenced Next, NOT Merge-Ready, Streak RESET to 0/3, Pass-16 Is the Pending Gate

`feature/S-21.07` (branch `feature/S-21.07-validate-cross-site-correspondence`, SHA **`96b4be19`**, UNCHANGED this burst — no code-repo commit; the fix was STORY-INDEX-only) remains UNFROZEN and sequenced next for E-21 W4. Pass-15 was **NOT-CLEAN** — 1 MEDIUM finding, closed same burst — and the human perimeter decision this session classified it IN-PERIMETER, resetting the LOCAL BC-5.39.001 streak from 1/3 to **0/3**. The branch is **still explicitly NOT merge-ready**: convergence requires 3 FRESH CONSECUTIVE CLEAN adversary verdicts starting from pass-16. **The next substantive action is to dispatch `vsdd-factory:adversary` fresh-context for pass-16**, reading only `adversary-pass-15.md` Part A per the Iron Law, against the state landed at D-998 (S-21.07 v1.11 UNCHANGED, STORY-INDEX v4.321, `feature/S-21.07`'s unchanged `96b4be19`).

### §4 F-001's Residual — the Only Genuinely Open Pass-10-Class Item

F-S2107-P10-001 (originally BLOCKER, vacuous POLICY 15 gate) is **ALREADY-RESOLVED at the design level**: ADR-040 v1.12/v1.13/v1.15/v1.16 RATIFIED/AMENDED, root cause (category error — gate evaluated in the wrong worktree) diagnosed and redesigned, mechanism implemented as `crates/policy15-attestation-gate/` (16 tests, mutation-verified) on branch `feature/policy15-gate-rust` at `d2a3176a` (pushed, no PR). **What remains open is purely operational: the CI job that makes the gate demonstrably RUNNING is not yet wired.** This is Drift Item `[D-969]`, routed to `devops-engineer`, and is **BLOCKED-ON `feature/policy15-gate-rust` merging to `develop`** — note this is a SEPARATE branch from `feature/S-21.07`, with its own independent merge path and no open PR. Do not conflate the two branches' merge readiness. UNCHANGED by this burst (pass-15 did not touch this item).

### §5 ADR-043

v1.5, `status: proposed`, **NOT RATIFIED**. Three fresh-context DO-NOT-RATIFY reviews (4, then 10, then 9 blockers) then amended. POLICY 22 requires human ratification. Reviews persisted as `adv-adr-043-pass-{1,2,3}.md`. UNCHANGED by this burst.

### §6 Blocking Issues

- **C-1 CWE-706** — `binary_allow` basename allow-list escape (structural HIGH / practical LOW). **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.**
- **C-2 CWE-362** — TOCTOU window; ADR-043 threat model boundary unformalized.
- **C-4 CWE-284** — prefix list empty/writable fallthrough; BC amendment pending.
- **C-5 CWE-284** — no per-entry resource limits; anchor S-21.14.
- **POLICY 15 gate — CI-wiring residual only (D-992).** See §4.
- **4 pass-10 carry-over findings** (MED-001, LOW-001/002/003, from the S-21.09 cascade) — anchor: next maintenance sweep; NOT a blocker on anything.
- **`feature/S-21.07` pass-15 NOT-CLEAN, streak RESET to 0/3 (D-998)** — sequenced next; **pass-16 adversary dispatch is the immediate next substantive pipeline action.**
- **E-18 STORY-INDEX total drift (107 vs 125 pts)** — out-of-perimeter, does not block anything; anchor: next maintenance sweep or explicit human direction.

### §7 Infrastructure Blockers

(a) **STATE.md narrative sections** — full-file Write convention continues (`verify-state-timestamp-refresh` guard requires a `timestamp:` advance within EVERY individual Edit/Write call's own diff, confirmed to apply even to small isolated Edits with no other content change).
(b) **`STORY-INDEX.md`** triggers advisory PostToolUse fuel/timeout signals on every edit (confirmed again this burst — advisory only; both edits landed).
(c) **`decision-log.md`/`burst-log.md`/`lessons.md` exhaust WASM validator fuel on every edit** — confirmed again this burst (advisory only; writes land, `block_intent=true` PostToolUse signals do not revert already-landed writes).
(d) **`validate-trajectory-tail-cell-completeness` (D-453(d))**: requires a 4-value trajectory-tail arrow-sequence present in BOTH the frontmatter `current_step` AND the Project Metadata `Last Updated` cell on every STATE.md write. Applied again this burst (`→1→2→0→1` UNCHANGED).
(e) **SHA-patch follow-up — DONE.** Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-998 row's "Full detail" cite updated `1199aae3`→`1750bd56` (actual commit HEAD) in this follow-up write, landed in the immediate follow-up commit after the D-998 burst commit's push.

### §8 Pending Human Decisions

1. **`feature/S-21.07` adversarial correction cascade** — dispatch adversary fresh-context against the state after D-998 (pass-16); this is now the sequenced-next E-21 W4 action. Streak is 0/3 (RESET) — 3 FRESH CONSECUTIVE CLEAN passes needed to converge.
2. **ADR-043 ratification** — v1.5 converged (no BLOCKERs per pass-3); human to decide: ratify v1.5 / request pass-4 / redirect design. UNCHANGED.
3. **S-21.12 blocker B1** — `cargo deny` has 5 advisories (not 2); `deny.toml` read-only; `async-std` no upgrade path; AC-004 unsatisfiable. UNCHANGED.
4. **Four orphan advisories + 18 Dependabot alerts** — scope assignment pending (E-22 or dedicated fix). UNCHANGED.
5. **github-ops push-delegate reliability** — investigate root cause of mid-session push failures (S-15.03 PRIORITY-A or dedicated devops follow-up). UNCHANGED from D-989.
6. **`merged-stories-ledger.md` backfill (S-19.04..S-21.08)** — scope a dedicated maintenance sweep, or accept the gap as permanent-historical. UNCHANGED from D-991.
7. **E-18 STORY-INDEX total drift (107 vs 125 pts)** — scope a dedicated maintenance sweep, or accept as permanent-historical estimate-vs-actual, or explicitly direct a reconciliation edit. UNCHANGED from D-998.

### §9 Follow-up Stories Registered (Unrelated to This Burst, Carried Forward)

- **S-21.14** (W8, 8 pts): release-pipeline weak-predicate sweep across 5 sites + resolver-arm floor + T-017 first-match extractor + artifact-freshness gate.
- **S-21.15** (W8, 5 pts): `compute-input-hash` search-path gap + `traces_to:` bare-filename question.

### §10 Resume Command

`/vsdd-factory:next-step` — **S-21.09 is MERGED and CLOSED** (PR #775, `2e8087af`); no further action owed. `feature/S-21.07`'s pass-15 record+fix burst (D-998) is **COMPLETE** — 1 MEDIUM finding closed, 3 non-blocking observations dispositioned, streak explicitly RESET to 0/3 by human perimeter decision. The pipeline is **ACTIVE**. **The immediate next substantive action is: dispatch `vsdd-factory:adversary` fresh-context for pass-16**, reading only `adversary-pass-15.md` Part A per the Iron Law, against the state landed at D-998 (S-21.07 v1.11 UNCHANGED, STORY-INDEX v4.321, `feature/S-21.07`'s unchanged `96b4be19`). D-998's SHA-patch follow-up is **COMPLETE** — Active Branches `factory-artifacts` row and this checkpoint's header now cite the actual commit HEAD `1750bd56`; no precondition blocks the pass-16 dispatch. **LOCAL BC-5.39.001 streak is 0/3 (RESET) — 3 FRESH CONSECUTIVE CLEAN passes (pass-16, pass-17, pass-18) are required to converge.** Separately, ADR-043 ratification, S-21.12's cargo-deny blocker, the `merged-stories-ledger.md` backfill scope decision, and the E-18 STORY-INDEX total-drift scope decision remain open per §8, unaffected by this burst.
