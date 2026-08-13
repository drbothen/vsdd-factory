---
document_type: pipeline-state
level: ops
version: "7.61"
status: draft
producer: state-manager
timestamp: 2026-08-14T00:49:00Z
phase: D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION
last_amended: "2026-08-14 (v7.61) — D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION (state-manager; single-commit TD-VSDD-053; parent-commit c3ef4b53; own SHA pending push): micro-fix burst closing Drift Item [D-1001] — stories/STORY-INDEX.md's own top-level frontmatter version:/last_amended: fields were never bumped past \"4.321\"/D-998 by the D-1000 commit, despite that commit's body-content aggregation-hygiene corrections (L722/L763/L776/L777) landing correctly and every downstream artifact asserting v4.322 ever since. FIX: frontmatter version: \"4.321\"→\"4.322\"; last_amended: new entry prepended (POLICY 1 append-only, prior chain wrapped [Prior: ...]). Zero body content touched — git diff --stat confirms exactly 2 insertions/2 deletions (version: + last_amended: lines only). Verified via literal-shell grep: STORY-INDEX frontmatter v4.322 now parity-matches STATE.md, decision-log.md (5 cites), burst-log.md (2 cites), and INDEX.md. THIS IS A VERSION-PARITY CORRECTION, NOT A STREAK EVENT — BC-5.39.001 LOCAL streak for the S-21.07 cascade EXPLICITLY REMAINS 1/3, UNCHANGED (this was a state-manager bookkeeping discovery, not an adversary-pass finding, and is not in S-21.07's deliverable). Drift Item [D-1001] RESOLVED. Pass-18 adversary dispatch remains the pending gate, unaffected. [Prior: 2026-08-14 (v7.60) — SHA-PATCH-2026-08-14-D-1001 (state-manager; commit c7fa6d93): Active Branches factory-artifacts row + Session Resume Checkpoint header + Decisions Log D-1001 row + all pending-SHA-patch cites updated e811cd15→c7fa6d93 (the D-1001 burst commit's own actual HEAD, confirmed via git rev-parse HEAD after push). No content change beyond the SHA-patch itself — D-1001's substance (fresh-context adversary pass-17: CLEAN, 0 findings at any severity + 2 non-blocking observations; F-S2107-P16-001 closure independently RE-VERIFIED across all five live E-21 aggregation cells; RECORD-ONLY, nothing to close; STREAK ADVANCES 0/3 → 1/3; STORY-INDEX.md frontmatter-version drift DISCOVERED, logged as new Drift Item [D-1001], not fixed) is UNCHANGED.]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION (state-manager; parent-commit c3ef4b53; own SHA pending push; D-chain cite D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998+D-1000+D-1001+D-1002; trajectory-tail S-21.07 →2→0→1→0 UNCHANGED): stories/STORY-INDEX.md frontmatter version: \"4.321\"->\"4.322\" + last_amended: append-only correction, completing D-1000's omitted self-bump; FRONTMATTER-ONLY, zero body content touched (git diff --stat: 2 insertions/2 deletions). Resolves Drift Item [D-1001] -- STORY-INDEX frontmatter now parity-matches STATE.md/decision-log.md/burst-log.md/INDEX.md v4.322 cites (literal-shell grep verified). NOT a streak event -- BC-5.39.001 LOCAL streak for S-21.07 cascade EXPLICITLY UNCHANGED 1/3 (bookkeeping discovery, not an adversary-pass finding, not in S-21.07's deliverable). No gate predicate, GateOutcome semantics, or ADR ratification status changed; feature/S-21.07 SHA 96b4be19 UNCHANGED; ARCH-INDEX untouched. NEXT: push this commit to factory-artifacts, then SHA-patch follow-up (Active Branches + checkpoint + decisions-log), then dispatch vsdd-factory:adversary fresh-context for pass-18 (pending gate, unaffected by this micro-fix)."
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
  272 lines (wc-l post-D-1001-S2107-PASS17-RECORD-ONLY-BURST 2026-08-14; pass-17 CLEAN 0 findings+2 obs, RECORD-ONLY — nothing to close; F-S2107-P16-001 closure independently re-verified; STREAK ADVANCES 0/3→1/3; STORY-INDEX.md frontmatter-version drift DISCOVERED (not fixed, new Drift Item [D-1001]); v7.58→v7.59; parent-commit e811cd15, own SHA pending push)
  271 lines (wc-l post-SHA-patch c7fa6d93 2026-08-14; Active Branches factory-artifacts e811cd15→c7fa6d93; v7.59→v7.60 UNCHANGED content)
  (wc-l post-D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION 2026-08-14; micro-fix burst, NOT a streak event — streak EXPLICITLY UNCHANGED 1/3; Drift Item [D-1001] RESOLVED; STORY-INDEX frontmatter version 4.321→4.322, zero body content touched; v7.60→v7.61; parent-commit c3ef4b53, own SHA pending push)
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
| **Last Updated** | 2026-08-14 — D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION (state-manager; parent-commit `c3ef4b53`; own SHA pending push): `stories/STORY-INDEX.md` frontmatter `version:` corrected `"4.321"`→`"4.322"` + `last_amended:` append-only entry, completing D-1000's omitted self-bump. **FRONTMATTER-ONLY** — zero body content touched (`git diff --stat`: 2 insertions/2 deletions). Resolves Drift Item `[D-1001]` — STORY-INDEX frontmatter now parity-matches STATE.md/decision-log.md/burst-log.md/INDEX.md v4.322 cites (literal-shell grep verified). **NOT a streak event** — this is a state-manager bookkeeping discovery, not an adversary-pass finding, and not in S-21.07's deliverable. LOCAL BC-5.39.001 streak (S-21.07 cascade) **EXPLICITLY REMAINS 1/3, UNCHANGED**. trajectory-tail (S-21.07) →2→0→1→0 UNCHANGED. |
| **Current Phase** | **D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION (parent-commit `c3ef4b53`; own SHA pending push; D-chain cite D-970+D-992+D-993+D-994+D-995+D-996+D-997+D-998+D-1000+D-1001+D-1002; PIPELINE ACTIVE).** S-21.09 remains **MERGED** (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` is **UNFROZEN + sequenced-next**, still **NOT merge-ready** — streak **1/3 (EXPLICITLY UNCHANGED this burst)**, 2 MORE CONSECUTIVE CLEAN passes required (pass-18, pass-19). 4-INDEX BC v4.58/VP v2.76/STORY **v4.322 (frontmatter parity restored)**/ARCH v3.58. policies.yaml v1.4.24 UNCHANGED. Drift Item `[D-1001]` **RESOLVED**. **Next substantive action: push this commit, SHA-patch follow-up, then dispatch `vsdd-factory:adversary` fresh-context for pass-18** (reads only `adversary-pass-17.md` Part A per the Iron Law). |
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
| D-1000-S2107-PASS16-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; commit `ddc07cf5`; SHA-patch done; D-999 sentinel skipped per human directive) | **COMPLETE** | Pass-16 NOT-CLEAN (1 MEDIUM F-S2107-P16-001 + 3 obs); F-S2107-P16-001 CLOSED (state-manager; STORY-INDEX v4.321→v4.322 — the D-998 sweep was FORM-SPECIFIC and missed L722 DAG-header wave-count, L776 E-21 footnote, L763 master-line; human-directed TRUE FILE-WIDE semantic-role sweep executed, additionally fixing L777 E-19 footnote (new finding) and L763's E-18/E-19 terms against their own canonical delivery-blockquote totals; zero remaining live disagreements); O-P16-01 CODIFIED (lesson STRENGTHENED to semantic-role enumeration + D-1000(e)); STATE.md §8 E-18 pending item RESOLVED at master-line layer (deeper catalog-vs-blockquote question remains separately open). **STREAK EXPLICITLY HOLDS 0/3 (second consecutive NOT-CLEAN, does not advance).** STATE.md v7.58. |
| D-1001-S2107-PASS17-RECORD-ONLY-BURST 2026-08-14 (single commit TD-VSDD-053; commit `c7fa6d93`; SHA-patch done) | **COMPLETE** | Pass-17 **CLEAN** (0 findings at any severity + 2 non-blocking obs); F-S2107-P16-001 closure independently RE-VERIFIED across all five live E-21 aggregation cells (zero residual stale figures). RECORD-ONLY burst — no fix owed, nothing to close. O-P17-01/O-P17-02 tracked in §8, not codified. STORY-INDEX.md frontmatter-version/last_amended drift DISCOVERED (D-1000's body content landed but top-level `version:` field never bumped past 4.321) — logged as new Drift Item `[D-1001]`, NOT fixed this burst. **STREAK EXPLICITLY ADVANCES 0/3 → 1/3 (first fresh CLEAN since the pass-15 reset); pass-18 adversary NEXT — 2 more CONSECUTIVE CLEAN required.** STATE.md v7.60. |
| D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION 2026-08-14 (single commit TD-VSDD-053; parent-commit `c3ef4b53`; own SHA pending push) | **COMPLETE** | Micro-fix burst (state-manager bookkeeping discovery, NOT an adversary pass): `stories/STORY-INDEX.md` frontmatter `version:` `"4.321"`→`"4.322"` + `last_amended:` append-only correction, completing D-1000's omitted self-bump. FRONTMATTER-ONLY — zero body content touched (`git diff --stat`: 2 insertions/2 deletions). Verified parity vs STATE.md/decision-log.md/burst-log.md/INDEX.md v4.322 cites (literal-shell grep). Drift Item `[D-1001]` RESOLVED. **NOT A STREAK EVENT — BC-5.39.001 streak EXPLICITLY UNCHANGED 1/3.** STATE.md v7.60→v7.61. |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-1000 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-1000-S2107-PASS16-RECORD-AND-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent-commit 1750bd56; commit ddc07cf5; D-999 skipped) | state-manager | COMPLETE | Pass-16 NOT-CLEAN (1 MEDIUM F-S2107-P16-001 + 3 obs) persisted verbatim; F-S2107-P16-001 CLOSED (STORY-INDEX v4.321→v4.322, file-wide semantic-role aggregation sweep: L722/L763/L776/L777); O-P16-01 CODIFIED (lesson strengthened). **STREAK EXPLICITLY HOLDS 0/3 (second consecutive NOT-CLEAN).** STATE.md v7.56→v7.57. |
| SHA-PATCH-2026-08-13-D-1000 (state-manager; commit ddc07cf5) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-1000 row SHA-patched `1750bd56`→`ddc07cf5`; STATE.md v7.57→v7.58; no content change. |
| D-1001-S2107-PASS17-RECORD-ONLY-BURST 2026-08-14 (single commit TD-VSDD-053; parent-commit e811cd15; commit c7fa6d93) | state-manager | COMPLETE | Pass-17 **CLEAN** (0 findings + 2 obs) persisted verbatim; F-S2107-P16-001 closure independently RE-VERIFIED; O-P17-01/O-P17-02 tracked §8; STORY-INDEX.md frontmatter-version drift DISCOVERED, logged as new Drift Item `[D-1001]`, not fixed. **STREAK ADVANCES 0/3 → 1/3.** STATE.md v7.58→v7.59. |
| SHA-PATCH-2026-08-14-D-1001 (state-manager; commit c7fa6d93) | state-manager | COMPLETE | Active Branches `factory-artifacts` row + Session Resume Checkpoint header + Decisions Log D-1001 row SHA-patched `e811cd15`→`c7fa6d93`; STATE.md v7.59→v7.60; no content change. |
| D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION 2026-08-14 (single commit TD-VSDD-053; parent-commit c3ef4b53; own SHA pending push) | state-manager | COMPLETE | `stories/STORY-INDEX.md` frontmatter `version:` 4.321→4.322 + `last_amended:` append-only correction (frontmatter-only, zero body touched); resolves Drift Item `[D-1001]`; parity verified vs 4 downstream artifacts. **NOT a streak event — streak EXPLICITLY UNCHANGED 1/3.** STATE.md v7.60→v7.61. **NEXT: SHA-patch follow-up, then adversary pass-18 dispatch (fresh-context, reads adversary-pass-17.md Part A only).** |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.58 D-992, UNCHANGED this burst) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX **v4.322** D-1000 body content; frontmatter parity restored D-1002 — Drift Item `[D-1001]` RESOLVED, frontmatter and body now agree) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-040 v1.16 D-994 pass-11 fix / ADR-041 v1.2 / ADR-042 v1.4 D-993 body reconciliation, UNCHANGED this burst; ADR-043 proposed NOT RATIFIED) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | **108** (STATE.md explicit counter; sprint-state predicate tracked separately per canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (108):** S-21.09 MERGED PR #775 `2e8087af` 2026-08-13 (validate-factory-path-staging WASM artifact restore + registry parity CI check; E-21 W4). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md` (known gap: not appended between S-19.03 and S-21.09; see ledger file note — anchored to a dedicated maintenance sweep).
- **In-Flight (0):** none.
- **E-21:** S-21.07 (W4; **sequenced next**, UNFROZEN; pass-17 **CLEAN** — 0 findings + 2 non-blocking obs (O-P17-01/O-P17-02, tracked §8), **STREAK 1/3 (EXPLICITLY UNCHANGED this burst — D-1002 is a version-parity correction, not an adversary-pass finding)** — 2 MORE CONSECUTIVE CLEAN required from pass-18; story spec v1.11 UNCHANGED this burst; branch `96b4be19` pushed (unchanged, code-repo not touched); **NOT merge-ready**; pass-18 adversary dispatch is the pending gate); S-21.09 (**MERGED** PR #775 `2e8087af`); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | **2e8087af** | PR #775 (`feature/S-21.09`) merged 2026-08-13T14:16:26Z; `validate-factory-path-staging.wasm` git-tracked. Pull on next code-worktree resume. |
| factory-artifacts | **c3ef4b53** (parent) | D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION — commit pending push this burst; SHA-patch follow-up will update this row to the actual new HEAD. |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. **F-001's sole remaining residual (D-992) is BLOCKED-ON this branch merging to `develop`** — routed devops-engineer, anchored Drift Item `[D-969]`. |
| feature/S-21.09 | c20cf2fe | **MERGED** to `develop` via PR #775, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Branch ref retained (standard post-merge retention). LOCAL BC-5.39.001 streak 3/3 RE-CONVERGED (D-988), PRESERVED through D-989 — final state at merge. |
| feature/S-21.07-validate-cross-site-correspondence | **96b4be19** | pass-17 **CLEAN** (0 findings, D-1001) — **STREAK 1/3 (EXPLICITLY UNCHANGED by D-1002)**; UNCHANGED this burst (no code-repo commit; unrelated branch). Pushed; SHA-equal with origin. Still UNFROZEN + sequenced-next, **NOT merge-ready** — convergence depends on **adversary pass-18 (NEXT)**, 2 MORE CONSECUTIVE CLEAN required. |
| feature/S-21.04 | 323f440f | pass-31 pending; no PR. Pushed; SHA-equal with origin. |
| fix/nested-factory-path-derivation | 9afc3226 | F-S2107-P8-016 + P9-008 CLOSED. Pushed; SHA-equal with origin. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel. Pushed; SHA-equal with origin. |
| fix/fuel-exhaustion-fail-loud | fbb9dcb6 | ABANDONED — orchestrator dispatch error (87 files duplicating unmerged S-21.07). CONFIRMED SUPERSEDED by PR #774 (`62fbcf1a`, D-992 re-verification). Local-only; deliberately NOT pushed. |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION COMPLETE (commit pending push; SHA-patch follow-up next). S-21.09 **MERGED** to `develop` (PR #775, `2e8087af`, UNCHANGED). `feature/S-21.07` pass-17 (**CLEAN**, 0 findings, D-1001) UNCHANGED by this burst — SHA `96b4be19` UNCHANGED; story spec v1.11 UNCHANGED. **NOT merge-ready — STREAK EXPLICITLY UNCHANGED 1/3 (this burst is a version-parity correction, not an adversary-pass finding); pass-18 adversary dispatch remains the pending gate.** No ADR touched this burst. `develop` **2e8087af**; main 80e5cd7b; `merged_count` **108**; BC v4.58; VP v2.76; STORY **v4.322 (frontmatter parity restored — Drift Item [D-1001] RESOLVED)**; ARCH v3.58 (UNCHANGED); policies.yaml v1.4.24 UNCHANGED; ADR-043 proposed NOT RATIFIED. F-001 redesign RATIFIED (ADR-040 v1.12/v1.13/v1.15/v1.16) — CI wiring still PENDING, BLOCKED-ON `feature/policy15-gate-rust`→`develop`. LOCAL BC-5.39.001 streak (S-21.07 cascade) **1/3 — EXPLICITLY UNCHANGED**. trajectory-tail (S-21.07) →2→0→1→0 UNCHANGED. | commit pending push 2026-08-14; D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION 2026-08-14. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range; exhaustive): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range; exhaustive) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-1002 (see decision-log.md for full range; exhaustive): this Decisions Log (D-1002 live) + decision-log.md SoT. **D-999 is SKIPPED (never allocated) per human directive** — the global D-NNN sequence continues D-998, D-1000, D-1001, D-1002 (no D-999 row exists or ever will).

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-1002 | D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION (state-manager; single-commit TD-VSDD-053 2026-08-14; parent-commit `c3ef4b53`; own SHA pending push). Micro-fix burst closing Drift Item `[D-1001]` (routed at D-1001(h)): `stories/STORY-INDEX.md`'s own top-level frontmatter `version:`/`last_amended:` fields were never bumped past `"4.321"`/D-998 by the D-1000 commit (`ddc07cf5`), despite that commit's body-content aggregation-hygiene corrections (L722/L763/L776/L777) landing correctly and every downstream artifact (STATE.md, decision-log.md, burst-log.md, `cycles/v1.0-brownfield-backfill/INDEX.md`) asserting `STORY-INDEX v4.322` as the D-1000-landed state ever since. **FIX:** frontmatter `version:` `"4.321"`→`"4.322"`; `last_amended:` new entry prepended (POLICY 1 append-only, entire prior chain wrapped `[Prior: ...]`) — **zero body content touched** (`git diff --stat stories/STORY-INDEX.md`: exactly 2 insertions/2 deletions, confirmed via literal-shell `git diff \| grep '^@@'` → `@@ -1,11 +1,11 @@`, spanning only frontmatter lines 1-11). **Verification:** literal-shell grep confirms the corrected frontmatter now parity-matches every downstream cite — STATE.md (`STORY v4.322`/`STORY-INDEX v4.322`), decision-log.md (5 occurrences of `STORY-INDEX v4.322`), burst-log.md (2 occurrences), and `INDEX.md` (multiple occurrences in the D-1000/D-1001 Convergence Status narrative). **THIS IS A VERSION-PARITY CORRECTION, NOT A STREAK EVENT** — a state-manager bookkeeping discovery, not an adversary-pass finding, and not part of `S-21.07`'s deliverable. **BC-5.39.001 LOCAL streak for the S-21.07 cascade EXPLICITLY REMAINS 1/3, UNCHANGED** by this burst — it neither advances nor resets. `feature/S-21.07` SHA `96b4be19` UNCHANGED (unrelated branch, no code-repo commit). BC-INDEX v4.58 UNCHANGED; VP-INDEX v2.76 UNCHANGED; ARCH-INDEX v3.58 UNCHANGED; `policies.yaml` v1.4.24 UNCHANGED (no codification — mechanical parity fix, not a process-gap lesson). **Drift Item `[D-1001]` RESOLVED.** Pass-18 adversary dispatch remains the pending gate, unaffected by this micro-fix. Full detail: `decision-log.md` D-1002 + `burst-log.md` `## D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION`. | STORY-INDEX frontmatter version 4.321→4.322, zero body content touched; resolves Drift Item [D-1001]; parity verified vs 4 downstream artifacts; NOT a streak event, streak EXPLICITLY UNCHANGED 1/3 | D-1002-STORY-INDEX-FRONTMATTER-VERSION-PARITY-CORRECTION | 2026-08-14 |
| D-413..D-1002 (see decision-log.md for full range; exhaustive; D-999 never allocated) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-14 |

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
| **[pass-10 carry-over] ADV-BB-P10-MED-001 directory-only `hook-plugins/sub/` staging control** | **OPEN — preserved through D-1002; does NOT block anything** | Gate (a)'s ">= 1 component after hook-plugins/" threshold admits directory-only declarations; spurious MISSING report possible. Anchor: next maintenance sweep. |
| **[pass-10 carry-over] ADV-BB-P10-LOW-001/002/003 (NUL/trailing-space names; fail-open arms; `workspace_root()` untested)** | **OPEN — preserved through D-1002; does NOT block anything** | Low-severity residuals from the S-21.09 cascade's pass-10; not addressed through the merge or this burst. Anchor: next maintenance sweep. |
| **[D-1001] `feature/S-21.07` pass-17 CLEAN — streak 1/3, 2 MORE CONSECUTIVE CLEAN required** | **OPEN — sequenced next, NOT merge-ready** | `feature/S-21.07` SHA `96b4be19` UNCHANGED; story spec v1.11 UNCHANGED. First fresh CLEAN since the pass-15 human-directed reset. **Pass-18 adversary dispatch (fresh-context, reads `adversary-pass-17.md` Part A only) is the pending gate.** UNCHANGED by D-1002 (unrelated micro-fix). |
| **[D-1000] E-18 STORY-INDEX delivery-blockquote total (107 pts) disagrees with current catalog sum (125 pts)** | **OPEN — OUT-OF-PERIMETER, does NOT block anything; master-line-arithmetic layer RESOLVED D-1000** | Frozen-historical record of a COMPLETE/merged epic, per D-996(d) precedent; the master "Total story points" line (L763) now correctly cites E-18's own canonical 107 (D-1000), closing the narrower master-line-vs-canonical-blockquote disagreement. The deeper question — whether 107 should itself be reconciled against the current 125-pt catalog sum — remains open. Companion item this burst: O-P17-01 (master-total leading-aggregate "533+" stale floor). Anchor: next maintenance sweep or explicit human direction. |

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
| **[D-1000] the D-998 sweep GATE ITSELF was scoped to a named phrasing, not the semantic class — a fifth-generation recurrence one level up (POLICY 5/TD-VSDD-060 companion)** | **CODIFIED — `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` STRENGTHENED (same lesson, D-1000 amendment) anchored S-15.03 PRIORITY-A 2026-08-13** | D-998's sweep operationalized its own obligation via a single grep phrasing (`stories total.`), which structurally could not catch DAG-header, footnote, or master-line cells asserting the identical total under different surface forms — 3 stale E-21 cells and 1 newly-discovered stale E-19 cell survived the "class-complete" sweep that was supposed to catch exactly this. The lesson is now stated by SEMANTIC ROLE (any total-bearing cell, any form) rather than by grep phrasing. See D-1000(e). |
| **[D-1001] STORY-INDEX.md frontmatter version/body-content parity gap (POLICY 8/17 class)** | **RESOLVED D-1002** | D-1000's commit landed body-content corrections (L722/L763/L776/L777) but never advanced the document's own top-level `version:`/`last_amended:` fields past `"4.321"`/D-998. **CLOSED D-1002**: frontmatter `version:` corrected to `"4.322"`, `last_amended:` append-only entry added (POLICY 1), zero body content touched (`git diff --stat`: 2/2), parity verified vs all 4 downstream cites (STATE.md, decision-log.md, burst-log.md, INDEX.md) via literal-shell grep. **NOT a streak event** — bookkeeping discovery, not an adversary-pass finding. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-14 — parent-commit `c3ef4b53`; own SHA pending push; PIPELINE ACTIVE; D-1002 micro-fix burst COMPLETE — Drift Item [D-1001] RESOLVED, streak EXPLICITLY UNCHANGED 1/3; SHA-patch NEXT; pass-18 adversary AFTER THAT)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. S-21.09 is **MERGED** (PR #775, `2e8087af`, UNCHANGED by this burst). This burst (D-1002) is a state-manager bookkeeping micro-fix burst, routed at D-1001(h), closing **Drift Item `[D-1001]`**: `stories/STORY-INDEX.md`'s own top-level frontmatter `version:`/`last_amended:` fields were never bumped past `"4.321"`/D-998 by the D-1000 commit, despite that commit's body-content corrections (L722/L763/L776/L777) landing correctly and every downstream artifact asserting v4.322 ever since. 4-INDEX: BC **v4.58** / VP v2.76 / STORY **v4.322 (frontmatter parity restored)** / ARCH v3.58 (UNCHANGED). `policies.yaml` **v1.4.24** UNCHANGED. factory-artifacts parent-commit `c3ef4b53`, own SHA pending push.

**Last decisions: D-1002.** `stories/STORY-INDEX.md` frontmatter `version:` corrected `"4.321"`→`"4.322"`; `last_amended:` new entry prepended (POLICY 1 append-only, entire prior chain wrapped `[Prior: ...]`). **Zero body content touched** — `git diff --stat stories/STORY-INDEX.md` confirms exactly 2 insertions/2 deletions (the `version:` and `last_amended:` lines only); `git diff` hunk header shows `@@ -1,11 +1,11 @@`, confirming the hunk covers only frontmatter lines 1-11. **Verification (literal-shell grep):** the corrected frontmatter `v4.322` now parity-matches every downstream cite — STATE.md, decision-log.md (5 occurrences), burst-log.md (2 occurrences), and `INDEX.md`. **THIS IS EXPLICITLY NOT A STREAK EVENT** — this burst is a state-manager bookkeeping discovery, not an adversary-pass finding, and it is not in `S-21.07`'s deliverable. **BC-5.39.001 LOCAL streak for the S-21.07 cascade EXPLICITLY REMAINS 1/3, UNCHANGED** — it neither advances nor resets; pass-18 adversary dispatch (fresh-context, reading only `adversary-pass-17.md` Part A per the Iron Law) remains the pending gate, entirely unaffected by this micro-fix. `feature/S-21.07` SHA `96b4be19` UNCHANGED (unrelated branch, no code-repo commit this burst). **Drift Item `[D-1001]` RESOLVED.** No story file, BC-INDEX, VP-INDEX, or ARCH-INDEX touched this burst. Carried forward unaffected: O-P17-01 (STORY-INDEX master-total stale-floor, §8 item 7 below) and O-P17-02 (BC-INDEX epic-column convention, §8 item 8 below) remain open, pending human intent-confirmation; the E-18 STORY-INDEX total-drift question (§8 item 7 companion) also remains open.

### §2 S-21.09 — MERGED (Unrelated to This Burst, Carried Forward)

**MERGED.** PR #775 → `develop`, merge commit `2e8087af`, 2026-08-13T14:16:26Z. Story spec v1.32, impl `c20cf2fe`, LOCAL BC-5.39.001 streak **3/3 RE-CONVERGED (D-988), PRESERVED through D-989** — final state at merge, UNCHANGED by this burst. No further work owed.

### §3 `feature/S-21.07` — Sequenced Next, NOT Merge-Ready, Streak 1/3 (Explicitly Unchanged by D-1002), Pass-18 Is the Pending Gate

`feature/S-21.07` (branch `feature/S-21.07-validate-cross-site-correspondence`, SHA **`96b4be19`**, UNCHANGED this burst — no code-repo commit; unrelated branch) remains UNFROZEN and sequenced next for E-21 W4. Pass-17 was **CLEAN** — 0 findings, 2 non-blocking observations — and the LOCAL BC-5.39.001 streak is **1/3**, EXPLICITLY UNCHANGED by D-1002 (a version-parity correction, not an adversary-pass finding). The branch is **still explicitly NOT merge-ready**: convergence requires 2 MORE CONSECUTIVE CLEAN adversary verdicts (pass-18, pass-19). **The next substantive action is to dispatch `vsdd-factory:adversary` fresh-context for pass-18**, reading only `adversary-pass-17.md` Part A per the Iron Law, against the state landed at D-1001/D-1002 (S-21.07 v1.11 UNCHANGED, STORY-INDEX v4.322 body content AND frontmatter now both correctly parity-matched, `feature/S-21.07`'s unchanged `96b4be19`).

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
- **`feature/S-21.07` pass-17 CLEAN, streak 1/3 (D-1001; EXPLICITLY UNCHANGED by D-1002)** — sequenced next; **pass-18 adversary dispatch is the immediate next substantive pipeline action.**
- **E-18 STORY-INDEX total drift (107 vs 125 pts)** — out-of-perimeter, does not block anything; master-line layer RESOLVED D-1000; deeper catalog-count question remains open; anchor: next maintenance sweep or explicit human direction.

### §7 Infrastructure Blockers

(a) **STATE.md narrative sections** — full-file Write convention continues (`verify-state-timestamp-refresh` guard requires a `timestamp:` advance within EVERY individual Edit/Write call's own diff, confirmed to apply even to small isolated Edits with no other content change; this burst's STATE.md advance required a full-file Write, and a follow-up single-line fix to escape a raw `|` in the Decisions Log D-1002 cell, each with its own fresh timestamp advance).
(b) **`STORY-INDEX.md`** triggers advisory PostToolUse fuel/timeout signals on every edit — touched this burst (frontmatter-only edit); advisory `fail-closed: plugin timed out` signals fired as expected on all three Edit calls, writes landed each time (confirmed via post-write grep), no reversion.
(c) **`decision-log.md`/`burst-log.md`/`lessons.md` exhaust WASM validator fuel on every edit** — confirmed again this burst (advisory only; writes land, `block_intent=true` PostToolUse signals do not revert already-landed writes).
(d) **`validate-trajectory-tail-cell-completeness` (D-453(d))**: requires a 4-value trajectory-tail arrow-sequence present in BOTH the frontmatter `current_step` AND the Project Metadata `Last Updated` cell on every STATE.md write. Applied again this burst (`→2→0→1→0` UNCHANGED).
(e) **`validate-dispatch-advance` bare-sentinel-token scan (ADR-041 §Decision 4)**: no D-999-adjacent narrative drafted this burst — not applicable.
(f) **SHA-patch follow-up — PENDING.** This commit's own SHA is not yet known (pending push). The standard immediate follow-up write will update the Active Branches `factory-artifacts` row, this checkpoint's header, and the Decisions Log D-1002 row's "Full detail" cite from `c3ef4b53` (parent) to the actual new HEAD once `git rev-parse HEAD` is available post-push — per established project convention (TD-VSDD-053 permits this as a SHA-patch, not a Stage-2 chain).
(g) **`validate-table-cell-count` gate**: fired on the initial full-file Write because the D-1002 Decisions Log cell contained a raw `|` inside a backtick code span (`` `git diff | grep '^@@'` ``) — markdown table parsers treat any unescaped `|` as a cell delimiter regardless of backtick nesting. Fixed by escaping to `` \| `` inside the cell; general discipline for future bursts: any literal shell snippet cited inside a table cell that contains a pipe character must be pipe-escaped.
(h) **Large-line frontmatter fields resist the `Read` tool.** `stories/STORY-INDEX.md`'s `last_amended:` field is a single ~228KB line (many nested `[Prior: ...]` wraps) — `Read` cannot load it even with `offset`/`limit` (per-line token cap). This burst's fix used `Bash`/`python3` to inspect exact byte-offsets and construct small, uniquely-matching `old_string`/`new_string` anchors for `Edit`, rather than reading the full line — a pattern worth reusing whenever a future burst must touch another heavily-append-only frontmatter field of this shape.

### §8 Pending Human Decisions

1. **`feature/S-21.07` adversarial correction cascade** — dispatch adversary fresh-context against the state after D-1001/D-1002 (pass-18); this is now the sequenced-next E-21 W4 action. Streak is **1/3 (EXPLICITLY UNCHANGED by D-1002)** — 2 MORE CONSECUTIVE CLEAN passes needed to converge (pass-18, pass-19).
2. **ADR-043 ratification** — v1.5 converged (no BLOCKERs per pass-3); human to decide: ratify v1.5 / request pass-4 / redirect design. UNCHANGED.
3. **S-21.12 blocker B1** — `cargo deny` has 5 advisories (not 2); `deny.toml` read-only; `async-std` no upgrade path; AC-004 unsatisfiable. UNCHANGED.
4. **Four orphan advisories + 18 Dependabot alerts** — scope assignment pending (E-22 or dedicated fix). UNCHANGED.
5. **github-ops push-delegate reliability** — investigate root cause of mid-session push failures (S-15.03 PRIORITY-A or dedicated devops follow-up). UNCHANGED from D-989.
6. **`merged-stories-ledger.md` backfill (S-19.04..S-21.08)** — scope a dedicated maintenance sweep, or accept the gap as permanent-historical. UNCHANGED from D-991.
7. **E-18 STORY-INDEX total drift (107 vs 125 pts)** — **PARTIALLY RESOLVED D-1000**: the master "Total story points" line (L763) now correctly cites E-18's own canonical delivery-blockquote total (107), closing the narrower master-line-vs-canonical-blockquote disagreement this item originally flagged at D-998. The residual, still-open question is deeper: whether 107 (the blockquote, frozen at epic completion) should itself be reconciled against the current 18-row/125-pt catalog sum (which includes two `-prereq` stub rows the original blockquote may not have counted as full stories). **Companion (D-1001, O-P17-01):** the master line's OWN leading aggregate ("Total story points: 533+ across 136 stories", same L763) is itself a stale floor against the sum of its own per-epic terms (~630) — a related but distinct cross-epic hygiene question, also anchored here. Human to decide: scope a dedicated maintenance sweep covering both, accept as permanent-historical estimate-vs-actual, or explicitly direct a reconciliation edit. UNCHANGED by D-1002.
8. **(D-1001) O-P17-02 — BC-INDEX BC-5.39.010 epic-column convention** — row shows `E-12` (the engine-governance epic owning the validate-* hook family) while the anchoring story is `S-21.07` (E-21); matches the entire sibling BC-5.39.003-008 validate-hook cohort, stable across all 16 prior passes. Human to confirm: is this an intentional "owning epic ≠ implementing story's epic" convention (in which case no action needed, disposition this item CLOSED-AS-INTENTIONAL), or should it be corrected to E-21? UNCHANGED by D-1002.
9. ~~**(D-1001) STORY-INDEX.md frontmatter version/body-content parity gap**~~ — **RESOLVED D-1002**: frontmatter `version:` corrected `"4.321"`→`"4.322"` (frontmatter-only, POLICY 1 append-only `last_amended:` entry), parity verified vs all 4 downstream cites via literal-shell grep. No further human decision needed; item retained here for audit-trail continuity.

### §9 Follow-up Stories Registered (Unrelated to This Burst, Carried Forward)

- **S-21.14** (W8, 8 pts): release-pipeline weak-predicate sweep across 5 sites + resolver-arm floor + T-017 first-match extractor + artifact-freshness gate.
- **S-21.15** (W8, 5 pts): `compute-input-hash` search-path gap + `traces_to:` bare-filename question.

### §10 Resume Command

`/vsdd-factory:next-step` — **S-21.09 is MERGED and CLOSED** (PR #775, `2e8087af`); no further action owed. The D-1002 STORY-INDEX frontmatter-version-parity micro-fix burst is **COMPLETE** — Drift Item `[D-1001]` RESOLVED, zero body content touched, parity verified vs 4 downstream artifacts. **THIS WAS NOT A STREAK EVENT** — LOCAL BC-5.39.001 streak for the S-21.07 cascade is **EXPLICITLY UNCHANGED at 1/3**. The pipeline is **ACTIVE**. **The immediate next substantive action is: push this commit to `factory-artifacts`, perform the standard SHA-patch follow-up (Active Branches row + checkpoint header + Decisions Log D-1002 row), then dispatch `vsdd-factory:adversary` fresh-context for pass-18**, reading only `adversary-pass-17.md` Part A per the Iron Law, against the state landed at D-1001/D-1002 (S-21.07 v1.11 UNCHANGED, STORY-INDEX v4.322 body content AND frontmatter both now correctly parity-matched, `feature/S-21.07`'s unchanged `96b4be19`). **LOCAL BC-5.39.001 streak is 1/3 — 2 more CONSECUTIVE CLEAN passes (pass-18, pass-19) are required to converge.** Separately, ADR-043 ratification, S-21.12's cargo-deny blocker, the `merged-stories-ledger.md` backfill scope decision, the (partially-resolved) E-18 STORY-INDEX total-drift scope decision (with its O-P17-01 companion), and the O-P17-02 BC-INDEX epic-column intent-confirmation remain open per §8, unaffected by this burst.
