---
document_type: pipeline-state
level: ops
version: "7.46"
status: draft
producer: state-manager
timestamp: 2026-08-13T20:15:00Z
phase: D-994-S2107-PASS11-RECORD-AND-FIX-BURST
last_amended: "2026-08-13 (v7.46) — D-994-S2107-PASS11-RECORD-AND-FIX-BURST (state-manager; parent-commit 33771c81): fresh-context adversary pass-11 dispatched against feature/S-21.07 96b4be19 and factory-artifacts 33771c81 — NOT-CLEAN, 1 MEDIUM finding (F-S2107-P11-001) + 2 observations (O-P11-01, O-P11-02); pass-10 defect classes F-001..F-009 independently re-verified RESOLVED/REDESIGNED. F-S2107-P11-001 CLOSED THIS BURST (ADR-040 v1.15→v1.16, architect, bundled verbatim — 3 remaining 'Do NOT apply/re-ratification required' sites superseded, ADR-041/042 pattern mirrored, literal-shell sweep confirms zero remaining live directives outside labeled-historical blocks). O-P11-01 CLOSED-DEFENSIVELY (ARCH-INDEX ADR-042 '~415KB' figure reframed as dated measurement-provenance, 2026-08-08 ADR-042 v1.0 empirical burst; not a decaying pin). O-P11-02 [process-gap] CODIFIED (lesson L-BB-adr-reconciliation-sweep-scope-on-ratification + decision-log D-994(e); no policies.yaml text change — routing rationale recorded). ARCH-INDEX v3.57→v3.58. trajectory 47→18→25→25→24→20→16→8→10→1 (tail →16→8→10→1); 10 true adversary reviews, 0 CLEAN verdicts. BC-INDEX v4.58 UNCHANGED; VP-INDEX v2.76 UNCHANGED; STORY-INDEX v4.318 UNCHANGED; policies.yaml v1.4.24 UNCHANGED. LOCAL BC-5.39.001 streak for the S-21.07 cascade EXPLICITLY REMAINS 0/3 — a NOT-CLEAN verdict does not advance the streak and a fix burst is not a clean verdict either. Pass-12 adversary dispatch (fresh-context, reads adversary-pass-11.md Part A only) is the pending gate. SHA-patch follow-up pending after this write. [Prior: 2026-08-13 (v7.45) — SHA-PATCH-2026-08-13-D-993 (state-manager; commit e85ff8cd): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-993 row + all pending-SHA-patch cites updated f672b582→e85ff8cd (the D-993 burst commit's own actual HEAD). No content change beyond the SHA-patch itself — D-993's substance (small consistency follow-on to the D-992 S-21.07 pass-10 fix burst: ADR-040 v1.13→v1.15, ADR-041 v1.1→v1.2, ADR-042 v1.3→v1.4 body reconciliation bundled verbatim; ARCH-INDEX v3.56→v3.57; Drift Item [D-992] RESOLVED; streak EXPLICITLY UNCHANGED 0/3) is UNCHANGED.]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "D-994-S2107-PASS11-RECORD-AND-FIX-BURST (state-manager; parent-commit 33771c81; D-chain cite D-967+D-970+D-992+D-993+D-994; trajectory-tail S-21.07 →16→8→10→1): fresh-context vsdd-factory:adversary pass-11 dispatched against feature/S-21.07-validate-cross-site-correspondence 96b4be19 (unchanged since D-992) and factory-artifacts 33771c81 (D-993 SHA-patch HEAD). Verdict NOT-CLEAN — 1 MEDIUM finding (F-S2107-P11-001) + 2 observations (O-P11-01, O-P11-02); persisted verbatim to cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-11.md per POLICY 22 relay-fidelity. Pass-10 defect classes F-001..F-009 independently re-verified RESOLVED/REDESIGNED by fresh context, confirming the D-992/D-993 fix content held. F-S2107-P11-001 CLOSED THIS BURST: ADR-040 v1.15->v1.16 (architect, already-authored in worktree, bundled verbatim per TD-VSDD-053) -- the 3 remaining 'Do NOT apply until re-ratification / MUST NOT be edited' sites (Status as of v1.3, Status as of v1.4, Proposed policies.yaml Replacement Text preamble) superseded via labeled SUPERSEDED blockquote notes + strikethrough, mirroring the ADR-041/042 pattern; literal-shell sweep confirms zero remaining live directives outside labeled-historical blocks. O-P11-01 CLOSED-DEFENSIVELY: ARCH-INDEX ADR-042 section 1 '~415KB prefix' figure reframed unambiguously as dated measurement-provenance (2026-08-08 ADR-042 v1.0 empirical burst; provenance figure, not a live pointer), closing the intent-ambiguity flag in the compliant direction. O-P11-02 [process-gap] CODIFIED: lesson L-BB-adr-reconciliation-sweep-scope-on-ratification appended to lessons.md plus decision-log D-994(e) -- on ADR ratification/reconciliation the architect MUST sweep EVERY current-tense 'Do NOT apply / MUST NOT edit / re-ratification required / NEEDS-HUMAN / proposed' string in the ENTIRE ADR body, not only the Status paragraph; a partial reconciliation is a POLICY 4 S-7.01 partial-fix regression. Routed to lessons.md + decision-log rather than a new policies.yaml POLICY entry (rationale recorded at D-994(e)); policies.yaml v1.4.24 UNCHANGED. ARCH-INDEX v3.57->v3.58 (ADR-040 row v1.16 note; ADR-042 row provenance reframe). BC-INDEX v4.58 UNCHANGED; VP-INDEX v2.76 UNCHANGED; STORY-INDEX v4.318 UNCHANGED. feature/S-21.07 SHA 96b4be19 UNCHANGED (no code-repo commit this burst -- fix is factory-artifacts-only). STREAK EXPLICITLY REMAINS 0/3 -- pass-11 was NOT-CLEAN and this fix burst is not a clean adversary verdict either; only 3 CONSECUTIVE CLEAN passes advance the streak. This burst is executed as a single atomic commit per TD-VSDD-053, citing this commit's own PARENT SHA (33771c81) per the D-419(b) convention since the commit's own SHA is unknown at write time; a standard SHA-patch follow-up commit (not a Stage-2 chain) will update all self-referential cites to the actual pushed SHA immediately after push. NEXT: SHA-patch follow-up, then dispatch vsdd-factory:adversary fresh-context for pass-12 (reads only adversary-pass-11.md Part A per the Iron Law)."
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
  261 lines (wc-l post-D-993-ADR-BODY-RECONCILIATION-BATCH 2026-08-13; ADR-040 v1.15/ADR-041 v1.2/ADR-042 v1.4 body reconciliation bundled; ARCH-INDEX v3.57; Drift Item [D-992] resolved; streak UNCHANGED 0/3; v7.43→v7.44; commit e85ff8cd)
  261 lines (wc-l post-SHA-patch e85ff8cd 2026-08-13; Active Branches factory-artifacts f672b582→e85ff8cd; v7.44→v7.45 UNCHANGED content)
  259 lines (wc-l post-D-994-S2107-PASS11-RECORD-AND-FIX-BURST 2026-08-13; pass-11 NOT-CLEAN 1 MEDIUM+2 obs, ALL CLOSED same burst; ADR-040 v1.16; ARCH-INDEX v3.58; lesson L-BB-adr-reconciliation-sweep-scope-on-ratification; streak REMAINS 0/3; v7.45→v7.46; commit pending SHA-patch)
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
| **Last Updated** | 2026-08-13 — D-994-S2107-PASS11-RECORD-AND-FIX-BURST (parent-commit `33771c81`; SHA-patch pending): fresh-context adversary pass-11 — NOT-CLEAN, 1 MEDIUM (F-S2107-P11-001) + 2 observations; pass-10 defect classes F-001..F-009 independently re-verified RESOLVED/REDESIGNED. F-S2107-P11-001 CLOSED THIS BURST (ADR-040 v1.16, architect — 3 remaining "Do NOT apply/re-ratification required" sites superseded, mirroring ADR-041/042). O-P11-01 CLOSED-DEFENSIVELY (ARCH-INDEX ADR-042 "~415KB" reframed as dated measurement-provenance). O-P11-02 [process-gap] CODIFIED (lesson + decision-log D-994(e); no policies.yaml change). ARCH-INDEX v3.57→v3.58. LOCAL BC-5.39.001 streak (S-21.07 cascade) **EXPLICITLY REMAINS 0/3**. trajectory-tail (S-21.07) →16→8→10→1. |
| **Current Phase** | **D-994-S2107-PASS11-RECORD-AND-FIX-BURST (parent-commit `33771c81`; D-chain cite D-967+D-970+D-992+D-993+D-994; PIPELINE ACTIVE).** S-21.09 remains **MERGED** (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` is **UNFROZEN + sequenced-next**, still **NOT merge-ready** — pass-11 was NOT-CLEAN and its finding was closed via a fix burst, not a clean adversary verdict; the branch's own convergence still requires 3 CONSECUTIVE CLEAN passes. 4-INDEX BC v4.58/VP v2.76/STORY v4.318/ARCH **v3.58**. policies.yaml v1.4.24 UNCHANGED. **Next substantive action: SHA-patch follow-up, then dispatch `vsdd-factory:adversary` fresh-context for pass-12** (reads only `adversary-pass-11.md` Part A per the Iron Law). |
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
| D-994-S2107-PASS11-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent-commit `33771c81`; commit pending SHA-patch) | **COMPLETE** | Pass-11 NOT-CLEAN (1 MEDIUM F-S2107-P11-001 + 2 obs; **CLIFF** 10→1, deepest reduction of the cascade); F-S2107-P11-001 CLOSED (ADR-040 v1.16, architect); O-P11-01 CLOSED-DEFENSIVELY (ARCH-INDEX reframe); O-P11-02 CODIFIED (lesson + D-994(e)). ARCH-INDEX v3.58. **Streak EXPLICITLY REMAINS 0/3; pass-12 adversary NEXT.** STATE.md v7.46. |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-993 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-993-ADR-BODY-RECONCILIATION-BATCH 2026-08-13 (single commit TD-VSDD-053; parent-commit f672b582; commit e85ff8cd) | state-manager | COMPLETE | Bundled architect ADR-040 v1.13→v1.15 / ADR-041 v1.1→v1.2 / ADR-042 v1.3→v1.4 body edits verbatim; ARCH-INDEX v3.56→v3.57 (3 row notes); Drift Item `[D-992]` RESOLVED. Streak UNCHANGED 0/3. STATE.md v7.43→v7.44. |
| SHA-PATCH-2026-08-13-D-993 (state-manager; commit e85ff8cd) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-993 row SHA-patched `f672b582`→`e85ff8cd`; STATE.md v7.44→v7.45; no content change. |
| D-994-S2107-PASS11-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent-commit 33771c81; commit pending SHA-patch) | state-manager | COMPLETE | Pass-11 NOT-CLEAN (1 MEDIUM + 2 obs) persisted verbatim; F-S2107-P11-001 CLOSED (ADR-040 v1.16 bundled); O-P11-01 CLOSED-DEFENSIVELY (ARCH-INDEX reframe); O-P11-02 CODIFIED (lesson + decision-log); ARCH-INDEX v3.58. Streak REMAINS 0/3. STATE.md v7.45→v7.46. **NEXT: SHA-patch, then adversary pass-12 dispatch (fresh-context, reads adversary-pass-11.md Part A only).** |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.58 D-992, UNCHANGED this burst) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX v4.318 D-991, UNCHANGED this burst) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-040 v1.16 D-994 pass-11 fix / ADR-041 v1.2 / ADR-042 v1.4 D-993 body reconciliation; ADR-043 proposed NOT RATIFIED) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | **108** (STATE.md explicit counter; sprint-state predicate tracked separately per canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (108):** S-21.09 MERGED PR #775 `2e8087af` 2026-08-13 (validate-factory-path-staging WASM artifact restore + registry parity CI check; E-21 W4). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md` (known gap: not appended between S-19.03 and S-21.09; see ledger file note — anchored to a dedicated maintenance sweep).
- **In-Flight (0):** none.
- **E-21:** S-21.07 (W4; **sequenced next**, UNFROZEN; pass-11 NOT-CLEAN — 1 MEDIUM + 2 obs, ALL CLOSED this burst (D-994); branch `96b4be19` pushed; **NOT merge-ready** — requires 3 CONSECUTIVE CLEAN adversary verdicts; pass-12 adversary dispatch is the pending gate); S-21.09 (**MERGED** PR #775 `2e8087af`); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | **2e8087af** | PR #775 (`feature/S-21.09`) merged 2026-08-13T14:16:26Z; `validate-factory-path-staging.wasm` git-tracked. Pull on next code-worktree resume. |
| factory-artifacts | **33771c81** | D-994-S2107-PASS11-RECORD-AND-FIX-BURST parent (this commit's own SHA pending — SHA-patch follow-up will update this row to the actual pushed HEAD). |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. **F-001's sole remaining residual (D-992) is BLOCKED-ON this branch merging to `develop`** — routed devops-engineer, anchored Drift Item `[D-969]`. |
| feature/S-21.09 | c20cf2fe | **MERGED** to `develop` via PR #775, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Branch ref retained (standard post-merge retention). LOCAL BC-5.39.001 streak 3/3 RE-CONVERGED (D-988), PRESERVED through D-989 — final state at merge. |
| feature/S-21.07-validate-cross-site-correspondence | **96b4be19** | pass-11 NOT-CLEAN 1 MEDIUM + 2 obs (D-994) — ALL CLOSED same burst; UNCHANGED this burst (fix is factory-artifacts-only, no code-repo commit). Pushed; SHA-equal with origin. Still UNFROZEN + sequenced-next, **NOT merge-ready** — convergence depends on **adversary pass-12 (NEXT)**. |
| feature/S-21.04 | 323f440f | pass-31 pending; no PR. Pushed; SHA-equal with origin. |
| fix/nested-factory-path-derivation | 9afc3226 | F-S2107-P8-016 + P9-008 CLOSED. Pushed; SHA-equal with origin. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel. Pushed; SHA-equal with origin. |
| fix/fuel-exhaustion-fail-loud | fbb9dcb6 | ABANDONED — orchestrator dispatch error (87 files duplicating unmerged S-21.07). CONFIRMED SUPERSEDED by PR #774 (`62fbcf1a`, D-992 re-verification). Local-only; deliberately NOT pushed. |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-994-S2107-PASS11-RECORD-AND-FIX-BURST COMPLETE (SHA-patch pending). S-21.09 **MERGED** to `develop` (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` pass-11 (1 MEDIUM + 2 obs, D-994): ALL CLOSED same burst; SHA `96b4be19` UNCHANGED; **NOT merge-ready — pass-12 adversary dispatch (fresh-context) is the pending gate.** ADR-040 v1.16 fix bundled at D-994 (architect; 3 remaining live-directive sites superseded). `develop` **2e8087af**; main 80e5cd7b; `merged_count` **108**; BC v4.58; VP v2.76; STORY v4.318; ARCH **v3.58**; policies.yaml v1.4.24 UNCHANGED; ADR-043 proposed NOT RATIFIED. F-001 redesign RATIFIED (ADR-040 v1.12/v1.13/v1.15/v1.16) — CI wiring still PENDING, BLOCKED-ON `feature/policy15-gate-rust`→`develop`. LOCAL BC-5.39.001 streak (S-21.07 cascade) **EXPLICITLY REMAINS 0/3**. trajectory-tail (S-21.07) →16→8→10→1. | SHA-patch pending 2026-08-13; D-994-S2107-PASS11-RECORD-AND-FIX-BURST 2026-08-13. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range; exhaustive): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range; exhaustive) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-994 (see decision-log.md for full range; exhaustive): this Decisions Log (D-994 live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-994 | D-994-S2107-PASS11-RECORD-AND-FIX-BURST (state-manager; single-commit TD-VSDD-053 2026-08-13; parent-commit 33771c81; commit pending SHA-patch). Fresh-context adversary pass-11 dispatched against `feature/S-21.07` `96b4be19` (unchanged) and `factory-artifacts` `33771c81` — **NOT-CLEAN, 1 MEDIUM (F-S2107-P11-001) + 2 observations**; persisted verbatim to `cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-11.md`. Pass-10 defect classes F-001..F-009 independently re-verified RESOLVED/REDESIGNED by fresh context (confirms D-992/D-993 fix content held). F-S2107-P11-001 (POLICY 4 + S-7.01 partial-fix regression: ADR-040 v1.13→v1.15 reconciliation superseded only the trailing §Status paragraph, leaving 3 sibling "Do NOT apply" sites live) **CLOSED THIS BURST** — ADR-040 v1.15→**v1.16** (architect, bundled verbatim; all 3 remaining sites superseded via labeled SUPERSEDED blockquote + strikethrough, mirroring ADR-041/042; literal-shell sweep confirms zero remaining live directives). O-P11-01 (ARCH-INDEX ADR-042 `~415KB` provenance-framing ambiguity) **CLOSED-DEFENSIVELY** — reframed as dated measurement-provenance (2026-08-08 ADR-042 v1.0 empirical burst). O-P11-02 [process-gap] (no reconciliation-sweep convention for "Do NOT apply" ADR sites) **CODIFIED** — lesson `L-BB-adr-reconciliation-sweep-scope-on-ratification` + this D-994(e); routed to lessons.md/decision-log rather than a new `policies.yaml` POLICY entry (rationale recorded). ARCH-INDEX v3.57→**v3.58** (ADR-040 row v1.16 note; ADR-042 row reframe). BC-INDEX v4.58 UNCHANGED; VP-INDEX v2.76 UNCHANGED; STORY-INDEX v4.318 UNCHANGED; policies.yaml v1.4.24 UNCHANGED. Trajectory `47→18→25→25→24→20→16→8→10→1` (tail `→16→8→10→1`); 10 true adversary reviews, 0 CLEAN verdicts. **STREAK EXPLICITLY REMAINS 0/3 — pass-11 NOT-CLEAN + this fix burst neither advance it; pass-12 adversary NEXT.** Full detail: `decision-log.md` D-994 + `burst-log.md` `## D-994-S2107-PASS11-RECORD-AND-FIX-BURST`. | pass-11 NOT-CLEAN 1 MEDIUM+2 obs; F-S2107-P11-001 CLOSED (ADR-040 v1.16); O-P11-01 CLOSED-DEFENSIVELY; O-P11-02 CODIFIED; ARCH-INDEX v3.58; streak REMAINS 0/3 | D-994-S2107-PASS11-RECORD-AND-FIX-BURST | 2026-08-13 |
| D-413..D-994 (see decision-log.md for full range; exhaustive) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-13 |

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
| **[pass-10 carry-over] ADV-BB-P10-MED-001 directory-only `hook-plugins/sub/` staging control** | **OPEN — preserved through D-994; does NOT block anything** | Gate (a)'s ">= 1 component after hook-plugins/" threshold admits directory-only declarations; spurious MISSING report possible. Anchor: next maintenance sweep. |
| **[pass-10 carry-over] ADV-BB-P10-LOW-001/002/003 (NUL/trailing-space names; fail-open arms; `workspace_root()` untested)** | **OPEN — preserved through D-994; does NOT block anything** | Low-severity residuals from the S-21.09 cascade's pass-10; not addressed through the merge or this burst. Anchor: next maintenance sweep. |
| **[D-994] `feature/S-21.07` pass-11 NOT-CLEAN — 1 MEDIUM finding + 2 observations, ALL CLOSED same burst** | **OPEN — sequenced next, NOT merge-ready** | `feature/S-21.07` SHA `96b4be19`; convergence now requires 3 CONSECUTIVE CLEAN adversary verdicts. **Pass-12 adversary dispatch (fresh-context, reads `adversary-pass-11.md` Part A only) is the pending gate.** |

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
| **[D-954] decision-log.md >17,000 lines** | OPEN 2026-08-04 | WASM validators time out on every edit (confirmed again this burst — advisory-only, writes land). |
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
| **[D-992] ADR-040 v1.13 §Implementation-routing "implementer — OUTSTANDING" line stale re F-002** | **RESOLVED D-993** | ADR-040 v1.14 (bundled per architect) corrected the line OUTSTANDING→DONE (commit `96b4be19`); v1.15 further corrected the sibling D-965-annotation item and the routing-list header count. |
| **[D-994] ADR-040 partial-fix reconciliation recurrence risk (POLICY 4 S-7.01)** | **CODIFIED — `L-BB-adr-reconciliation-sweep-scope-on-ratification` anchored S-15.03 PRIORITY-A 2026-08-13** | ADR ratification/reconciliation must sweep the ENTIRE ADR body for live "Do NOT apply" directives, not only the §Status paragraph — nearly recurred within the same cascade that had just done it completely for ADR-041/042. See D-994(e). |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-13 — parent-commit `33771c81`; PIPELINE ACTIVE; D-994 pass-11 record+fix COMPLETE; SHA-patch pending; pass-12 adversary NEXT)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. S-21.09 is **MERGED** (PR #775, `2e8087af`, UNCHANGED by this burst). This burst (D-994) dispatched fresh-context adversary pass-11 against `feature/S-21.07` at `96b4be19` and `factory-artifacts` at `33771c81` (the D-993 SHA-patch HEAD). Verdict: **NOT-CLEAN — 1 MEDIUM finding (F-S2107-P11-001) + 2 observations (O-P11-01, O-P11-02)**, all three CLOSED within this same burst. 4-INDEX: BC **v4.58** / VP v2.76 / STORY v4.318 / ARCH **v3.58**. `policies.yaml` **v1.4.24** UNCHANGED. This commit's own factory-artifacts SHA is not yet known — a SHA-patch follow-up commit is the immediate next write, per the D-419(b) parent-commit-SHA convention.

**Last decisions: D-994.** Pass-11 first independently re-verified that every pass-10 defect class (F-001..F-009) held under fresh-context re-examination — confirming the D-992/D-993 fix content was sound — before surfacing its own single new finding. F-S2107-P11-001 (MEDIUM, POLICY 4 + S-7.01 partial-fix regression) found that the D-992/D-993 ADR-040 body-vs-frontmatter reconciliation had corrected only the trailing §Status paragraph, leaving three sibling "Do NOT apply until re-ratification" sites live and contradicting the ratified/applied reality — while the same reconciliation pass had been complete for sibling ADR-041/042. Architect had already authored the fix (ADR-040 v1.16) in the worktree ahead of this burst; state-manager bundled it verbatim per TD-VSDD-053. Two observations were also resolved: O-P11-01 (ARCH-INDEX `~415KB` intent ambiguity) closed defensively by reframing as dated measurement-provenance; O-P11-02 (process-gap: no reconciliation-sweep convention) closed by codifying a lesson + decision-log obligation. LOCAL BC-5.39.001 streak for the S-21.07 cascade remains **0/3 — EXPLICITLY UNCHANGED**: a NOT-CLEAN verdict does not advance the streak, and neither does this burst's fix (only 3 CONSECUTIVE CLEAN adversary passes advance it).

### §2 S-21.09 — MERGED (Unrelated to This Burst, Carried Forward)

**MERGED.** PR #775 → `develop`, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Story spec v1.32, impl `c20cf2fe`, LOCAL BC-5.39.001 streak **3/3 RE-CONVERGED (D-988), PRESERVED through D-989** — final state at merge, UNCHANGED by this burst. No further work owed.

### §3 `feature/S-21.07` — Sequenced Next, NOT Merge-Ready, Pass-12 Is the Pending Gate

`feature/S-21.07` (branch `feature/S-21.07-validate-cross-site-correspondence`, SHA **`96b4be19`**, UNCHANGED this burst — no code-repo commit; the pass-11 fix was factory-artifacts-only) remains UNFROZEN and sequenced next for E-21 W4. Pass-11's single finding and both observations are **ALL CLOSED** — but the branch is **still explicitly NOT merge-ready**: convergence requires 3 CONSECUTIVE CLEAN adversary verdicts (BC-5.39.001), and pass-11 was NOT-CLEAN. **The next substantive action is to dispatch `vsdd-factory:adversary` fresh-context for pass-12**, reading only `adversary-pass-11.md` Part A per the Iron Law, against the fix content landed at this D-994 burst (ADR-040 v1.16, ARCH-INDEX v3.58, the O-P11-02 lesson). LOCAL BC-5.39.001 streak for this cascade remains **0/3**. Pass-11's cliff reduction (10→1) is the deepest single-pass reduction of the entire cascade — if pass-12 comes back CLEAN, this is streak position 1/3.

### §4 F-001's Residual — the Only Genuinely Open Pass-10-Class Item

F-S2107-P10-001 (originally BLOCKER, vacuous POLICY 15 gate) is **ALREADY-RESOLVED at the design level**: ADR-040 v1.12/v1.13/v1.15/v1.16 RATIFIED/AMENDED, root cause (category error — gate evaluated in the wrong worktree) diagnosed and redesigned, mechanism implemented as `crates/policy15-attestation-gate/` (16 tests, mutation-verified) on branch `feature/policy15-gate-rust` at `d2a3176a` (pushed, no PR). **What remains open is purely operational: the CI job that makes the gate demonstrably RUNNING is not yet wired.** This is Drift Item `[D-969]`, routed to `devops-engineer`, and is **BLOCKED-ON `feature/policy15-gate-rust` merging to `develop`** — note this is a SEPARATE branch from `feature/S-21.07`, with its own independent merge path and no open PR. Do not conflate the two branches' merge readiness.

### §5 ADR-043

v1.5, `status: proposed`, **NOT RATIFIED**. Three fresh-context DO-NOT-RATIFY reviews (4, then 10, then 9 blockers) then amended. POLICY 22 requires human ratification. Reviews persisted as `adv-adr-043-pass-{1,2,3}.md`. UNCHANGED by this burst.

### §6 Blocking Issues

- **C-1 CWE-706** — `binary_allow` basename allow-list escape (structural HIGH / practical LOW). **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.**
- **C-2 CWE-362** — TOCTOU window; ADR-043 threat model boundary unformalized.
- **C-4 CWE-284** — prefix list empty/writable fallthrough; BC amendment pending.
- **C-5 CWE-284** — no per-entry resource limits; anchor S-21.14.
- **POLICY 15 gate — CI-wiring residual only (D-992).** See §4.
- **4 pass-10 carry-over findings** (MED-001, LOW-001/002/003, from the S-21.09 cascade) — anchor: next maintenance sweep; NOT a blocker on anything.
- **`feature/S-21.07` pass-11, 1/1 CLOSED same burst (D-994)** — sequenced next; **pass-12 adversary dispatch is the immediate next substantive pipeline action.**

### §7 Infrastructure Blockers

(a) **STATE.md narrative sections** — full-file Write convention continues (`verify-state-timestamp-refresh` guard requires a `timestamp:` advance within EVERY individual Edit/Write call's own diff, confirmed to apply even to small isolated Edits with no other content change). This burst used a single full-file `Write` carrying both the timestamp advance and all content changes together, per the established remediation.
(b) **`STORY-INDEX.md`** triggers advisory PostToolUse fuel timeouts (not touched this burst).
(c) **`decision-log.md`/`burst-log.md`/`lessons.md` exhaust WASM validator fuel on every edit** — confirmed again this burst (advisory only; writes land, `block_intent=true` PostToolUse signals do not revert already-landed writes).
(d) **`validate-trajectory-tail-cell-completeness` (D-453(d))**: requires a 4-value trajectory-tail arrow-sequence present in BOTH the frontmatter `current_step` AND the Project Metadata `Last Updated` cell on every STATE.md write. Applied again this burst (`→16→8→10→1`, tail advanced from `→20→16→8→10`).
(e) **`ARCH-INDEX.md` WASM validators (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`) timed out on all D-994 row-edit Edits** — `block_intent=true exit_code=2 block_reason="fail-closed: plugin timed out"` (epoch/wall-clock arm, not fuel exhaustion). PostToolUse cannot revert already-landed writes; confirmed via literal-shell grep after each Edit that the intended content landed. Advisory only, consistent with ARCH-INDEX.md's known large-file WASM-timeout profile.
(f) **`lessons.md`/`decision-log.md`/`burst-log.md` `lint-registry-async-invariant` + fuel-timeout advisories** — confirmed again this burst on all three D-994 append writes; content verified landed via literal-shell grep in each case; advisory only.
(g) **SHA-patch follow-up — PENDING.** Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-994 row's "Full detail"/parent-commit cite all currently read `33771c81` (the parent SHA at burst start, per D-419(b) convention since this commit's own SHA is unknown at write time). The immediate next write after this commit's push is the standard SHA-patch follow-up (not a Stage-2 chain, TD-VSDD-053) updating these cites to the actual pushed HEAD.

### §8 Pending Human Decisions

1. **`feature/S-21.07` adversarial correction cascade** — dispatch adversary fresh-context against the state after D-994 (pass-12); this is now the sequenced-next E-21 W4 action.
2. **ADR-043 ratification** — v1.5 converged (no BLOCKERs per pass-3); human to decide: ratify v1.5 / request pass-4 / redirect design. UNCHANGED.
3. **S-21.12 blocker B1** — `cargo deny` has 5 advisories (not 2); `deny.toml` read-only; `async-std` no upgrade path; AC-004 unsatisfiable. UNCHANGED.
4. **Four orphan advisories + 18 Dependabot alerts** — scope assignment pending (E-22 or dedicated fix). UNCHANGED.
5. **github-ops push-delegate reliability** — investigate root cause of mid-session push failures (S-15.03 PRIORITY-A or dedicated devops follow-up). UNCHANGED from D-989.
6. **`merged-stories-ledger.md` backfill (S-19.04..S-21.08)** — scope a dedicated maintenance sweep, or accept the gap as permanent-historical. UNCHANGED from D-991.

### §9 Follow-up Stories Registered (Unrelated to This Burst, Carried Forward)

- **S-21.14** (W8, 8 pts): release-pipeline weak-predicate sweep across 5 sites + resolver-arm floor + T-017 first-match extractor + artifact-freshness gate.
- **S-21.15** (W8, 5 pts): `compute-input-hash` search-path gap + `traces_to:` bare-filename question.

### §10 Resume Command

`/vsdd-factory:next-step` — **S-21.09 is MERGED and CLOSED** (PR #775, `2e8087af`); no further action owed. `feature/S-21.07`'s pass-11 record+fix burst (D-994) is **COMPLETE** — 1 MEDIUM finding and 2 observations, all CLOSED same burst. The pipeline is **ACTIVE**. **The immediate next substantive action is: (1) SHA-patch follow-up** (update Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-994 row from parent-cite `33771c81` to this commit's own actual pushed HEAD), **then (2) dispatch `vsdd-factory:adversary` fresh-context for pass-12**, reading only `adversary-pass-11.md` Part A per the Iron Law, against the fix content landed at D-994 (ADR-040 v1.16, ARCH-INDEX v3.58). Separately, ADR-043 ratification, S-21.12's cargo-deny blocker, and the `merged-stories-ledger.md` backfill scope decision remain open per §8, unaffected by this burst.
