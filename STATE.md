---
document_type: pipeline-state
level: ops
version: "2.0"
status: draft
producer: state-manager
timestamp: 2026-06-11T00:00:00Z
phase: D-546-S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-11
last_amended: 2026-06-11 (v2.96) — D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT: 3 bin/ helpers (factory-lock-status.sh + factory-lock-acquire-precheck.sh + factory-unlock-decide.sh) + 3 bats; SKILL.md thin orchestrators; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; issue #170 Wave 3 delivery-prep complete. [Prior: 2026-06-11 (v2.95) — D-545 S-17.02 DELIVERED/MERGED: PR #182 squash-merged df4f26b8; CI 13/13 bats green; trend 1H+2M+4L→1M→0→0→0 3-CLEAN; pr-reviewer APPROVE; develop c64b46d2→df4f26b8; BC-4.13.001 POL-14 draft→active; issue #170 partial-close (S-17.03 remains); STORY-INDEX v3.89→v3.90; BC-INDEX v2.69→v2.70; ARCH-INDEX v2.19→v2.20 (ADR-025 v1.3 env_allow footgun). [Prior: 2026-06-11 (v2.94) — D-544 S-17.01 DELIVERED/MERGED: PR #181 squash-merged c64b46d2; CI 22/22 bats green; trend 9→3→0→0→0 3-CLEAN; BC-5.40.001 POL-14 draft→active; issue #170 REOPENED; STORY-INDEX v3.87→v3.88; BC-INDEX v2.66→v2.67. [Prior: 2026-06-10 (v2.93) — D-543 S-17.01 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT. [Prior: 2026-06-10 (v2.92) — D-542 STORY-DECOMPOSITION FOR ISSUE-170 FACTORY LOCK: epic E-17 + 3 stories S-17.01/02/03. [Prior: 2026-06-10 (v2.91) — D-541 BC-AUTHORING: 3 BCs draft; BC-INDEX v2.65→v2.66. [Prior: 2026-06-10 (v2.90) — D-540 ADR-025 ADOPTED FOR ISSUE-170; ARCH-INDEX v2.18→v2.19.]]]]]]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
current_step: "D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT 2026-06-11 — delivery-prep applied S-17.01 precedent (D-543): 3 new bin/ helpers extracted (factory-lock-status.sh shared three-state display, factory-lock-acquire-precheck.sh, factory-unlock-decide.sh) + 3 bats; SKILL.md files become thin orchestrators delegating STATE.md write to state-manager (single-writer) via S-17.01 helpers; reuses factory-lock-write.sh+factory-cas-push.sh (S-17.01 D-543); BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; 4-index: BC-INDEX v2.70 UNCHANGED STORY-INDEX v3.90→v3.91 VP-INDEX v2.06 UNCHANGED ARCH-INDEX v2.20 UNCHANGED; trajectory-tail →9→9→9→11; maintain all 5 BC-5.39.006 v1.7 PCs per TD-VSDD-097-EXT; D-chain cite D-545 per D-419(b); parent-commit e9a22a0b per D-419(b). SIZE BUDGET: 413L (wc-l; -2 under soft 415; margin 500-413=87 from hard cap; D-446(c))"
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

  D-430(a) compaction (D-538 burst 2026-06-10):
  Decisions Log D-527+D-528 (2 rows, ~8 lines) archived to decision-log.md SoT per D-430(a).
  All content preserved via decision-log.md SoT (rows present from prior entry).

  Line-growth tracker (D-532 onward; D-520..D-531 archived per D-430(a) 2026-06-10 compaction):
  Pre-D-520 tracker entries (D-504..D-519) preserved at: git show 688dd1c2:.factory/STATE.md.
  D-520..D-531 tracker entries (12 entries, all OVER soft-target) archived per D-430(a) 2026-06-10; preserved at: git show c62c2c03:.factory/STATE.md lines 36-51.
  D-532..D-535 tracker entries (4 entries; all UNDER soft-target) archived per D-430(a) D-542 burst; preserved at: git show ba471c58:.factory/STATE.md lines 43-46.
  D-536..D-538 (archived per D-430(a) D-543 burst): D-536 416L +1 over; D-537 421L +6 over; D-538 408L -7 under (D-430(a) compaction §1-§12 refresh).
  D-539-ISSUE-169-176-PR-180-MERGED-2026-06-10 401 lines (wc-l; D-430(a) D-529+D-530+D-531 archived; 14 UNDER soft; D-446(c)).
  D-539-ISSUE-169-176-PR-180-MERGED-2026-06-10 401 lines (wc-l; D-430(a) D-529+D-530+D-531 archived; 14 UNDER soft; D-446(c)).
  D-540-ADR-025-ADOPTED-ISSUE-170-DESIGN-2026-06-10 409 lines (wc-l; ADR-025 v1.2; ARCH-INDEX v2.19; 6 UNDER soft; D-446(c)).
  D-541-BC-AUTHORING-ISSUE-170-3-BCS-AUTHORED-2026-06-10 415 lines (wc-l; 3 BCs + CAP-031 + BC-INDEX v2.66; AT soft-target; D-446(c)).
  D-542-STORY-DECOMPOSITION-ISSUE-170-3-STORIES-E17-AUTHORED-2026-06-10 415 lines (wc-l; epic E-17 + S-17.01/02/03; STORY-INDEX v3.85; D-430(a) compaction: D-510+D-522+D-525+D-526+D-532..D-535 archived; §1-§4-§5-§8-§9-§10-§11-§12 Session Resume refresh; margin 500-415=85 from hard cap; margin 415-415=AT soft-target; D-446(c) dual-margin form).
  D-543-S-17.01-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-10 418 lines (wc-l; STORY-INDEX v3.86; D-543 row + §1/§3/§4/§5/§8/§9/§11/§12 refresh; D-430(a) D-536..D-538 banner archived; +3 over soft-target; margin 500-418=82 from hard cap; D-446(c) dual-margin form).
  D-544-S-17.01-DELIVERED-MERGED-2026-06-11 430 lines (wc-l; D-544 row + BC-5.40.001 POL-14 active + §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; STORY-INDEX v3.88; BC-INDEX v2.67; develop c64b46d2; +15 over soft-target; margin 500-430=70 from hard cap; D-446(c) dual-margin form).
  D-545-S-17.02-DELIVERED-MERGED-2026-06-11 409 lines (wc-l; D-545 row + BC-4.13.001 POL-14 active + ADR-025 v1.3 + §1/§3/§4/§8/§9/§10/§11/§12 refresh; STORY-INDEX v3.90; BC-INDEX v2.70; ARCH-INDEX v2.20; develop df4f26b8; D-430(a) compaction Phase Progress + Decisions Log D-532..D-543 archived; -6 under soft 415; margin 500-409=91 from hard cap; D-446(c) dual-margin form).
  D-546-S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-11 413 lines (wc-l; D-546 row + STORY-INDEX v3.91 + §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; BC-INDEX v2.70 UNCHANGED; ARCH-INDEX v2.20 UNCHANGED; -2 under soft 415; margin 500-413=87 from hard cap; D-446(c) dual-margin form).
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
| **Last Updated** | 2026-06-11 — D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT: 3 bin/ helpers (factory-lock-status.sh + factory-lock-acquire-precheck.sh + factory-unlock-decide.sh) + 3 bats; SKILL.md thin orchestrators; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; trajectory-tail →9→9→9→11. |
| **Current Phase** | D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT 2026-06-11 — S-17.03 v1.0→v1.1 executable-helper model applied (S-17.01 D-543 precedent); 3 bin/ helpers + 3 bats; BC-6.23.001 UNCHANGED; 14 ACs preserved; issue #170 Wave 3 delivery-prep complete. Next: S-17.03 test-writer Red Gate on feature/S-17.03-factory-lock-skills. |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B, Waves 1-11, S-7.03, beta.5-7, W-14, W-15 | **COMPLETE** | `cycles/v1.0-brownfield-backfill/phase-progress-archive.md` |
| Phase D-1..D-4, Waves 12-16, E-9 v1.7 sweep | **COMPLETE** | `cycles/v1.0-brownfield-backfill/` |
| Releases rc.11..rc.18, F3/F4 E-12, S-12.03..S-12.08 | **ARCHIVED 2026-06-01 per D-430(a)** | Full rows: `git show aa1f05c9:.factory/STATE.md` lines 80-93. |
| F5 passes 3-8 cycle-level adversary + fix bursts | **COMPLETE** | Trajectory 11→9→8→7→5; F5 pass-8 verdict MEDIUM; ARCH-INDEX v1.45, D-381. |
| F5 passes 9-17 adversary + fix bursts | **ARCHIVED 2026-06-08 per D-430(a)** | 20 rows archived; trajectory pass-9→17: HIGH→MEDIUM→MEDIUM→MEDIUM→HIGH→MEDIUM→MEDIUM×3; D-382..D-392 codified; L-EDP1-007/009 captured. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 85-106. |
| D-343..D-523 (E-10 pass-9..14, M3 cascade, S-15.03 PRIORITY-A waves, rc.19, S-15.17 cascade) | **ARCHIVED 2026-06-10 per D-430(a)** | 22 rows archived; all COMPLETE/SEALED/SHIPPED; Full rows: `git show c62c2c03:.factory/STATE.md` lines 82-108. Summary: E-10 pass-9..14 SEALED D-471; M3 BC 11-pass CONVERGED D-497; S-15.03 PRIORITY-A COMPLETE D-508 (all 11 stories + 40pts); rc.19 SHIPPED d15152af; S-15.17 cascade 9-pass SEALED D-522; 7/7 uncertainties CONFIRMED D-523. |
| Release v1.0.0-rc.20 | **SHIPPED 2026-06-01** at 2a191314 | PR #166 --merge e00ab1ab; tag e9e38286; run 26738809372 all 6 jobs PASS; GitHub Release prerelease; marketplace PR #12 squash-merged 862e660d; S-15.17 hook + MCP fleet-sweep reach operator cache; plugin count 52→53; develop sync 9ed17b1d→474a2731 |
| POST-RC.20 MAINTENANCE SWEEP | **COMPLETE 2026-06-01** D-529 | td-74 worktree+branch removed; Dependabot: #3+#156+#157 MERGED, #152/#125/#2+#167 closed-redundant; develop 474a2731→b21fd358; zero open PRs |
| E-10 pass-16 adversary + fix-burst PR #168 | **COMPLETE 2026-06-01** D-530 | verdict LOW (0C+0H+0M+3L); trend 22→…→8→3; F-PASS16-002 CI-count-floor FIXED PR #168 82163b7f (derived from crate count); F-PASS16-001+003 ACCEPTED-AT-FLOOR per D-471. |
| E-10 adversarial cascade | **SEALED 2026-06-01 at pass-16 (D-531)** | verdict LOW; 16-pass trend 22→11→16→16→12→2→1→4→5→4→6→7→5→8→8→3; asymptotic-acceptance per D-471/D-386 Option C; S-7.02 SATISFIED; resumption gate = engine-surface material change |
| D-526 S-15.17 SHIPPED — PR #164 9ed17b1d | **SHIPPED 2026-05-31** | validate-trajectory-tail-cell-completeness WASM hook; priority 158; BC-5.39.009 POL-14 draft→active; ADV-EDP1-P75-HIGH-002 CLOSED; BC-INDEX v2.65; STORY-INDEX v3.84; develop HEAD 9ed17b1d |
| D-532..D-543 (2026-06-08..2026-06-11) | **ARCHIVED 2026-06-11 per D-430(a)** | D-532 SESSION-END DURABILITY; D-535 #128 MERGED f6ce4b7c; D-537 #130 MERGED 89fbe2d6 ADR-024 v1.2; D-538 SESSION-END; D-539 #169+#176 MERGED 0f4793f1; D-540 ADR-025 ADOPTED; D-541 3-BCS-AUTHORED BC-INDEX v2.66; D-542 E-17+S-17.01/02/03 STORY-INDEX v3.85; D-543 S-17.01 v1.1 helpers; D-544 S-17.01 MERGED c64b46d2 BC-5.40.001 ACTIVE. Full rows: decision-log.md SoT. |
| D-544 S-17.01 DELIVERED/MERGED 2026-06-11 | **DELIVERED/MERGED** | PR #181 squash-merged c64b46d2; 22/22 bats green; 3-CLEAN trend 9→3→0→0→0; BC-5.40.001 POL-14 active; issue #170 REOPENED; E-17 1/3 |
| D-545 S-17.02 DELIVERED/MERGED 2026-06-11 | **DELIVERED/MERGED** | PR #182 squash-merged df4f26b8; 13/13 bats green; 3-CLEAN trend 1H+2M+4L→1M→0→0→0; BC-4.13.001 POL-14 active; ADR-025 v1.3 env_allow footgun; issue #170 partial-close; E-17 2/3; S-17.03 Wave 3 next |

## Current Phase Steps

> **Rows before pass-57 archived to** `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` per STATE.md content-routing rules (keep last 5 only).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| F5 passes 18-60 fix bursts (archived) | state-manager | ARCHIVED | See `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`. Passes 57-59: D-437..D-439 (META-LEVEL-12/13/14 CANDIDATES; trajectory →8→8→9); pass-60: D-440 META-LEVEL-15 CONFIRMED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,958 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 80 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 105 file-resident + 15 stub IDs (STORY-INDEX v3.86) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 17 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 24 |

## Story Status

105 file-resident + 15 unauthored stub IDs = 120 stories registered.

- **Merged (76):** Includes all prior + S-15.04 (PR #142 fdc7da16) + S-15.05 (PR #143 224fa184) + S-15.08 (PR #144 c62f952c) + S-15.07 (PR #145 6fe7de4c) + S-15.11 (PR #146 6e0d5407) + S-15.09 (PR #147 6e2d7805) + S-15.14 (PR #148 6d2ba5ad) + S-15.16-Part-B (PR #153 c1c81603) + S-15.10 (PR #154 a36ab711) + S-15.12 (PR #155 fba7e1cd) + S-15.15 (PR #158 24cc2ba6) + S-15.13 (PR #159 ced39c82) + S-15.17 (PR #164 9ed17b1d) + **S-17.01 (PR #181 c64b46d2)** + **S-17.02 (PR #182 df4f26b8)**. Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **Draft (30 file-resident):** S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); S-17.03 (BC-6.23.001; W3); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Unauthored stub IDs (15):** S-9.01..S-9.07 (W-16); S-11.01..S-11.08 (E-11 W-17 Tier 3)
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 2a191314 | rc.20 SHIPPED 2026-06-01; bot binary commit on top of --merge from develop; prior: 43afbfa7 (rc.19 2026-05-28) |
| develop | df4f26b8 | D-545 PR #182 S-17.02 SQUASH-MERGED 2026-06-11; verify-factory-lock WASM guard DELIVERED; prior: c64b46d2 (D-544 PR #181 S-17.01 2026-06-11) |
| factory-artifacts | `171a9413` | D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-11 (prior: `735b9168` D-545; prior-prior: `37414e5a` D-544-sha-patch) |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; annotated tag object; GitHub Release prerelease; marketplace PR drbothen/claude-mp #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28; GitHub Release prerelease 2026-05-28T15:10:56Z; marketplace PR #11 squash-merged |
| v1.0.0-rc.18 (tag) | 666d689f | SHIPPED 2026-05-13 PR #135 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready (S-16.01 5pts PostToolUse HEAD verify, S-16.02 3pts PreToolUse -F arm); E-16 under SS-07/SS-04; milestone v1.0.0-rc.17; BC-7.03.094/095/001, VP-080, ARCH SS-07 v1.3/SS-04 v1.4 registered |
| v1.0-brownfield-backfill | brownfield | **D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT 2026-06-11** | S-15.03 PRIORITY-A COMPLETE D-508; rc.20 SHIPPED D-528; #128+#130+#169+#176+#170-S17.01+#170-S17.02 MERGED; D-544 S-17.01 MERGED c64b46d2 BC-5.40.001 ACTIVE; D-545 S-17.02 MERGED df4f26b8; BC-4.13.001 ACTIVE; ADR-025 v1.3; **D-546 S-17.03 v1.1 executable-helper refinement (3 helpers+3 bats); STORY-INDEX v3.91**; BC-INDEX v2.70; ARCH-INDEX v2.20; issue #170 partial-close; S-17.03 Red Gate next; requires rc release. |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11 (tick-up from 35-consecutive 9s; 14-day pause cost); 4 structural ACCEPTED-AT-FLOOR per D-386 Option C extension; S-15.17 anchors HIGH-002 cure; L-EDP1-067 captured; BC-INDEX v2.53; STORY-INDEX v3.71. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-540: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md` (factory-artifacts 7b83ef58)
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` <!-- D-452(e) umbrella-range-auto-advance; D-511..D-546 per-burst D-range advances archived to decision-log.md; D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT 2026-06-11 D-range→D-546 -->

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-546 | S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-11 — delivery-prep applied S-17.01 precedent: 3 bin/ helpers (factory-lock-status.sh shared three-state display, factory-lock-acquire-precheck.sh, factory-unlock-decide.sh) + 3 bats; SKILL.md thin orchestrators delegating STATE.md write to state-manager via S-17.01 helpers; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; D-chain cite D-545; parent-commit e9a22a0b. | S-17.03 v1.1 executable-helper model: 3 helpers + 3 bats; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91 | story-refinement | 2026-06-11 |
| D-545 | S-17.02 DELIVERED/MERGED 2026-06-11 — PR #182 squash-merged df4f26b8; CI 13/13 bats green; trend 1H+2M+4L→1M→0→0→0 3-CLEAN; pr-reviewer APPROVE; feature DELETED+VERIFIED; develop c64b46d2→df4f26b8; BC-4.13.001 POL-14 draft→active; ADR-025 v1.2→v1.3 (env_allow footgun — 3rd silent-no-op vector); issue #170 partial-close (S-17.03 remains); STORY-INDEX v3.89→v3.90; BC-INDEX v2.69→v2.70; ARCH-INDEX v2.19→v2.20; D-chain cite D-544; parent-commit 37414e5a. | S-17.02 MERGED PR #182 df4f26b8; BC-4.13.001 POL-14 active; ADR-025 v1.3; develop df4f26b8; E-17 2/3; issue #170 partial-close | story-merge-closure | 2026-06-11 |
| D-544 | S-17.01 DELIVERED/MERGED 2026-06-11 — PR #181 squash-merged c64b46d2; CI 22/22 bats green; trend 9→3→0→0→0 3-CLEAN; pr-reviewer APPROVE; feature DELETED+VERIFIED; develop 0f4793f1→c64b46d2; BC-5.40.001 POL-14 draft→active; issue #170 REOPENED (S-17.02 Wave 2 next); STORY-INDEX v3.87→v3.88; BC-INDEX v2.66→v2.67; D-chain cite D-543; parent-commit b84a6886. | S-17.01 MERGED; BC-5.40.001 POL-14 active; develop c64b46d2; E-17 1/3; issue #170 REOPENED | story-merge-closure | 2026-06-11 |
| D-543 | S-17.01 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-10 — delivery-prep Red-Gate-feasibility defect: v1.0 tested prose (SKILL.md + state-manager.md) with Rust-style test names + no host module; refined per L-issue-169-176-worktree-identity(b): factory-lock-write.sh (D3) + factory-cas-push.sh (D6) under plugins/vsdd-factory/bin/; factory-lock-write.bats + factory-cas-push.bats under tests/; all 10 ACs + BC-5.40.001 PC/EC traces UNCHANGED; STORY-INDEX v3.85→v3.86; D-chain cite D-542; parent-commit 0601fdb1. | S-17.01 v1.1 executable-helper model: factory-lock-write.sh+factory-cas-push.sh+bats; STORY-INDEX v3.85→v3.86; BC-5.40.001 UNCHANGED | story-refinement | 2026-06-10 |
| D-542 | STORY-DECOMPOSITION FOR ISSUE-170 FACTORY LOCK 2026-06-10 — epic E-17 (Factory State Durability and Concurrency) + 3 stories: S-17.01 (factory_lock schema+CAS; 5pts; BC-5.40.001; W1; SS-05; acyclic), S-17.02 (verify-factory-lock WASM guard; 8pts; BC-4.13.001; W2; SS-04; deps [S-17.01]), S-17.03 (/factory-lock+/factory-unlock+factory-health; 8pts; BC-6.23.001; W3; SS-06; deps [S-17.01, S-17.02]); 21pts/39ACs; STORY-INDEX v3.84→v3.85; D-chain cite D-541; parent-commit ba471c58. | epic E-17 + 3 stories S-17.01/02/03 (21pts; acyclic); STORY-INDEX v3.84→v3.85 | story-decomposition | 2026-06-10 |
| D-541 | BC-AUTHORING FOR ISSUE-170 FACTORY LOCK/LEASE 2026-06-10 — 3 BCs authored draft per ADR-025 v1.2: BC-4.13.001 (verify-factory-lock WASM guard, SS-04, 8PCs, 15ECs, 10TVs); BC-5.40.001 (factory_lock schema+TTL+CAS, SS-05, 6PCs, 9ECs); BC-6.23.001 (/factory-lock+/factory-unlock+factory-health, SS-06, 8PCs, 10ECs, 10TVs); CAP-031 registered capabilities.md v1.3; BC-INDEX v2.65→v2.66; SS-04 39→40 SS-05 656→657 SS-06 586→587 total_bcs 1955→1958; VP IDs TBD (TD-VSDD-063); POLICY 8 propagation deferred to implementing-story; 4-index BC bumped VP/STORY/ARCH UNCHANGED; D-chain cite D-540; parent-commit c7277468. | bc-authoring | 2026-06-10 |
| D-540 | ADR-025 ADOPTED FOR ISSUE-170 FACTORY LOCK/LEASE DESIGN 2026-06-10 — ADR-025 v1.2 ACCEPTED: local native-WASM PreToolUse guard verify-factory-lock as primary enforcement (frontmatter factory_lock block, git-email identity, block-mutations/allow-reads, TTL 45min mid-burst-renewed + audited force-unlock break-glass, fail-open-on-crash); --force-with-lease push-CAS complementary mitigation; git-ref refs/factory-lock CAS deferred; NO dispatcher-binary/host-ABI change (host_abi=1 unchanged); research-verified APPROVE-WITH-FIXES all 5 fixes landed; 9 deliverables enumerated; ARCH-INDEX v2.18→v2.19; 4-index BC/VP/STORY UNCHANGED; human-approved for implementation; D-chain cite D-539 per D-419(b); parent-commit ba6844c1 per D-419(b). | adr-adoption | 2026-06-10 |
| D-539 | ISSUE-169+176 WORKTREE-IDENTITY COUPLE MERGED 2026-06-10 — PR #180 "fix(adversary): worktree-identity engine fix — eliminate phantom findings (#169 + #176)" SQUASH-MERGED 0f4793f1; CI run 27309724791 11/11 GREEN (cargo-host ubuntu+macos, 5× build-dispatcher, validate, SAST, platforms-drift); issues #169+#176 AUTO-CLOSED; feature/issue-169-176-worktree-identity DELETED+VERIFIED (git ls-remote --exit-code exit 2); develop 89fbe2d6→0f4793f1; POL-14 no-op; 4-index ALL UNCHANGED (BC-INDEX v2.65, VP-INDEX v2.06, STORY-INDEX v3.84, ARCH-INDEX v2.18); requires rc release for operator cache (agents/adversary.md + skills/adversarial-review/SKILL.md + skills/deliver-story/steps/shared-context+step-d5-adversary-convergence + NEW bin/resolve-worktree-identity.sh + 2 NEW bats test files); multi-family 3-CLEAN at LOCAL SHA 5ea02ecf (Gemini cross-family 7-iter ~20 defects + Claude canonical 3-pass CRITICAL CWD-relative repo-root bug caught); lesson L-issue-169-176-worktree-identity codified; D-538 checkpoint archived to session-checkpoints.md; D-chain cite D-538 per D-419(b); parent-commit 9eb53aab per D-419(b). | issue-169-176-merge-closure | 2026-06-10 |
| D-532..D-538 archived | **COMPACTED 2026-06-11 per D-430(a)** | D-532 SESSION-END DURABILITY 2026-06-08; D-533 ISSUE-VALIDATION 18-issues 2026-06-09; D-534 #128 in-flight; D-535 #128 MERGED f6ce4b7c 2026-06-09; D-536 ADR-024 ADOPTED ARCH-INDEX v2.17; D-537 #130 MERGED 89fbe2d6 ADR-024 v1.2 ARCH-INDEX v2.18 2026-06-10; D-538 SESSION-END DURABILITY 2026-06-10. Full rows: decision-log.md SoT. |
| D-529+D-530+D-531 archived | **COMPACTED 2026-06-10 per D-430(a)** | D-529 POST-RC.20 MAINT SWEEP COMPLETE 2026-06-01; D-530 E-10 PASS-16 COMPLETE 2026-06-01 (verdict LOW; F-PASS16-002 FIXED PR #168 82163b7f); D-531 E-10 CASCADE SEALED 2026-06-01 (asymptotic-acceptance per D-471+D-386 Option C; S-7.02 SATISFIED). Full rows in decision-log.md SoT. |
| D-527+D-528 archived | **COMPACTED 2026-06-10 per D-430(a)** | D-527 SESSION-END DURABILITY BURST 2026-05-31; D-528 v1.0.0-rc.20 SHIPPED 2026-06-01 (PR #166 --merge e00ab1ab; tag e9e38286; main 2a191314; marketplace #12; plugin 52→53). Full rows in decision-log.md SoT. |
| D-499..D-509 archived | **COMPACTED 2026-06-08 per D-430(a)** | 11 rows archived. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 249-259. |
| D-413..D-498 archived | **COMPACTED 2026-05-27 per D-430(a)** | 36 rows archived. Full content: decision-log.md (F5 + brownfield). Pre-compaction state: `git show 20cb8e1c:.factory/STATE.md`. |
| D-510+D-522+D-525+D-526 archived | **COMPACTED 2026-06-10 per D-430(a) D-542 burst** | 4 rows archived to decision-log.md SoT (D-510 F5 pass-75; D-522 S-15.17 SEALED; D-525 ADR-023; D-526 S-15.17 SHIPPED). Full rows in decision-log.md SoT. |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfusion Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers on active stories. -->

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67** 4 timing-flaky e2e tests | **RESOLVED 2026-05-15 PR #143 + RECURRENCE RESOLVED 2026-05-31 PR #165 f34b7567** | F-P3-008 pattern fully resolved across all known bats suites. |
| **TD #68/69/70/71/72/74** | **ALL RESOLVED** 2026-05-13/14/15 | See PRs #114/#116/#117/#140/#138/#139/#141. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log have `host::read_file(...65536...)` callsites against files >64KiB → silent fail-open. RECOMMENDED ACTION: follow-up story targeting both crates to raise max_bytes to 524288 + add oversize regression tests. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml entry. Target: S-15.03 PRIORITY-A automation wave (CI lint). |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 2026-05-17/18 | 6-class META-LEVEL perimeter; TDD micro-commit + registry-priority + compaction-burst-sibling-sweep + own-burst-log-structural-integrity + dim2-pc-must-read-production disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` env-var skips production STATE.md bats test in CI. Structural fix options: (a) mount factory worktree; (b) capability-check skip; (c) local-only harness. |
| **S-15.17-CR-001** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` uses `has_trajectory_tail` on full table rows (advisory-arm only; unreachable in production). Revisit if INDEX.md layout changes. |
| **S-15.17-CR-002** | ACCEPTED-DEFERRED 2026-05-31 | `rows_after_heading` duplicate-heading `continue` branch does not reset `seen_separator` (advisory-arm only; impossible in production). Revisit if INDEX.md gains duplicate headings. |
| **test_F_P2_001 timing flake** | OPEN 2026-06-08 | darwin-x64 test_F_P2_001 observed at 3761ms vs 3000ms threshold; same class as TD #67/F-P3-008 (wall-clock assertion); PR #165 fixed TC-9 sibling only; this test not yet de-flaked. Candidate de-flake follow-up story (same strategy: event-observation structural rewrite). D-532 capture. |
| **RUSTSEC-2026-0149** | OPEN 2026-06-11 — wasmtime-wasi HIGH | `cargo audit` on PR #182 CI: RUSTSEC-2026-0149 wasmtime-wasi HIGH advisory (path_open TRUNCATE bypass; CVE pending). Pre-existing; not introduced by S-17.02. Fix: wasmtime >= 44.0.2 required; awaiting upstream compatibility validation. Anchor: next rc release cycle or explicit cargo-audit remediation story. |
| **#170 partial-close — S-17.03 remains** | OPEN 2026-06-11 | Issue #170 partially closed (S-17.01 W1 + S-17.02 W2 merged). S-17.03 (W3 — /factory-lock+/factory-unlock skills + /factory-health + /factory-worktree-health lock status) remains draft. Issue #170 stays open until S-17.03 PR merges. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | validate-trajectory-tail-cell-completeness src: `extract_per_pass_trajectory_flag`/`check_state_md_with_flag` function doc-comment headers still describe old extraction approach (hook shipped green + correct, but header comment stale). Cosmetic cleanup on next spec-touch of S-15.17 or next adversary sweep. D-532 capture. |
| **F-P3-007 / F-P4-001 / F-P4-002** | OPEN-DEFERRED 2026-05-17 | STATE.md `phase:` field cap; story v1.1 PC numbering; BC v1.2 changelog phrasing. Anchor: next BC-5.39.006 amendment. |
| **L-EDP1-067-CANDIDATE-INV-015** | FORWARDED-TO-SK-MCP-001-APPENDIX-D 2026-05-18 | Adversary-fresh-context-must-grep-canonical-source. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-11 — D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT; 3 bin/ helpers + 3 bats; BC-6.23.001 UNCHANGED; STORY-INDEX v3.91; S-17.03 Red Gate next)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION ON A DIFFERENT MACHINE**
> Read this section alone to resume the orchestrator after full CLEAR, new session, or new machine. All context needed is here.
> Assumes ZERO prior context. Every decision, directive, and anchor is stated explicitly below.

### §1. Where We Are

**FIVE ISSUES DELIVERED THIS CYCLE (plus 3 design gates). rc.20 SHIPPED (D-528). E-10 CASCADE SEALED (D-531). D-535 #128 MERGED. D-537 #130 MERGED. D-539 #169+#176 MERGED. D-540 ADR-025 ADOPTED. D-541 3-BCS-AUTHORED. D-542 E-17+S-17.01/02/03 AUTHORED. D-543 S-17.01 v1.1 HELPERS. D-544 S-17.01 DELIVERED/MERGED — PR #181 c64b46d2; BC-5.40.001 POL-14 active; E-17 1/3. D-545 S-17.02 DELIVERED/MERGED — PR #182 squash-merged df4f26b8; CI 13/13 bats green; trend 1H+2M+4L→1M→0→0→0 3-CLEAN; BC-4.13.001 POL-14 active; ADR-025 v1.3 env_allow footgun; issue #170 partial-close; develop df4f26b8; E-17 2/3. D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT — 3 bin/ helpers (factory-lock-status.sh + factory-lock-acquire-precheck.sh + factory-unlock-decide.sh) + 3 bats; SKILL.md thin orchestrators; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.91; S-17.03 ready for Red Gate. RC RELEASE PENDING for operator cache (#128+#130+#169+#176+#170-S17.01+#170-S17.02).**

- **D-528..D-533 (2026-06-01/09; archived):** rc.20 SHIPPED tag e9e38286 main 2a191314; issue-validation 18 issues 17 actionable. Full rows: decision-log.md SoT.
- **D-535 (2026-06-09):** ISSUE-128 PR-178 MERGED f6ce4b7c. feature/issue-128 DELETED+VERIFIED. develop→f6ce4b7c. #128 DELIVERED.
- **D-537 (2026-06-10):** ISSUE-130 PR-179 MERGED 89fbe2d6. ADR-024 v1.2. ARCH-INDEX v2.18. #130 DELIVERED. Requires rc release.
- **D-539 (2026-06-10):** ISSUE-169+176 PR-180 MERGED 0f4793f1. CI 11/11 GREEN. issues AUTO-CLOSED. feature DELETED+VERIFIED. Requires rc release. Multi-family 3-CLEAN SHA 5ea02ecf.
- **D-540 (2026-06-10):** ADR-025 ADOPTED. 9 deliverables enumerated. ARCH-INDEX v2.19. Implementation-ready.
- **D-545 (2026-06-11):** S-17.02 DELIVERED/MERGED. PR #182 squash-merged df4f26b8. CI 13/13 bats green. Trend 1H+2M+4L→1M→0→0→0 3-CLEAN. BC-4.13.001 POL-14 draft→active. ADR-025 v1.3 env_allow footgun enumerated. STORY-INDEX v3.89→v3.90. BC-INDEX v2.69→v2.70. ARCH-INDEX v2.19→v2.20. develop df4f26b8. E-17 2/3 stories merged. issue #170 partial-close.
- **D-544 (2026-06-11):** S-17.01 DELIVERED/MERGED. PR #181 squash-merged c64b46d2. CI 22/22 bats green. BC-5.40.001 POL-14 active. E-17 1/3. issue #170 REOPENED.
- **D-540..D-543 (2026-06-10):** ADR-025 ADOPTED; 3-BCs-AUTHORED; E-17+stories-AUTHORED; S-17.01-v1.1-helpers. (Decision-log.md SoT for details.)
- **develop HEAD:** `df4f26b8`. **main HEAD:** `2a191314` (rc.20 bot binary commit 2026-06-01).
- **D-range:** D-001..D-545.
- **4-index (post-D-546):** BC-INDEX v2.70, VP-INDEX v2.06, STORY-INDEX v3.91, ARCH-INDEX v2.20 (STORY-INDEX bumped D-546; others UNCHANGED).
- **BC content:** BC-5.39.005 v1.3 ACTIVE + BC-5.39.006 v1.7 ACTIVE + BC-5.39.007 v1.6 ACTIVE + BC-5.39.008 v1.5 ACTIVE + BC-5.39.009 v1.9 ACTIVE + BC-7.04.051 v1.1 ACTIVE + **BC-4.13.001 v1.3 ACTIVE** + **BC-5.40.001 v1.1 ACTIVE** + BC-6.23.001 v1.0 DRAFT.
- **policies.yaml v1.3.6:** SEALED — no further cures.

**D-545 S-17.02 DELIVERED/MERGED 2026-06-11 — verify-factory-lock WASM guard (D1+D2) + env_allow footgun fixed + boundary semantics corrected. BC-4.13.001 v1.3 ACTIVE. ADR-025 v1.3. ARCH-INDEX v2.20. develop df4f26b8. issue #170 partial-close (S-17.03 W3 remains). #128+#130+#169+#176+#170-S17.01+#170-S17.02 DELIVERED/MERGED; requires rc release for operator cache. Next: S-17.03 test-writer Red Gate E-17 Wave 3 OR rc release OR #129 canonical-principle.**

### §2. Operating Mode

- vsdd-factory brownfield-onboarding; cycle `v1.0-brownfield-backfill`; self-referential.
- **E-10 SEALED D-471** (2026-05-14). **E-10 CASCADE FULLY SEALED D-531 (2026-06-01; pass-16 asymptotic-acceptance; resumption gate = engine-surface material change).**
- **F5 PAUSED D-386 Option C** (2026-05-13). **Do NOT resume without explicit human direction.**
- **S-15.14 SEALED D-477** (2026-05-18; LOCAL cascade 11 passes asymptotic; M3 gate 3c satisfied).
- **S-15.17 SHIPPED D-526** (2026-05-31; PR #164 9ed17b1d; BC-5.39.009 ACTIVE; ADV-EDP1-P75-HIGH-002 CLOSED).
- **S-15.03 PRIORITY-A COMPLETE D-508** (2026-05-27; Wave 4 COMPLETE; 3M3c COMPLETE; all 11 stories merged).
- **RC.20 SHIPPED D-528** (2026-06-01; run 26738809372; tag e9e38286; main 2a191314; marketplace #12).
- **D-532..D-539 (2026-06-08..2026-06-10):** SESSION-END bursts + ISSUE-128/130/169+176 DELIVERED/MERGED. Full rows: decision-log.md SoT.

### §3. User Directives (Carry Across CLEAR)

ALL of these are ACTIVE and MANDATORY on every dispatch:

- **2026-05-18 M3 path chosen:** Forward path = M3 (5 stories + ADR-021/022 already ACCEPTED 2026-05-15). M3 now COMPLETE. TD-VSDD-101 resolved in-story S-15.15.
- **TD-VSDD-097-EXT:** ALL orchestrator dispatch templates for `current_step:` writes MUST satisfy ALL 5 BC-5.39.006 v1.7 PCs (PC2+PC3+PC4+PC5+PC6) simultaneously.
- **TD-VSDD-099:** Every burst-log entry MUST include all 4 Dim blocks (Dim-2+Dim-5+Dim-6+Dim-7); Dim-6 MUST contain literal-shell count with captured stdout.
- **TD-VSDD-100:** Dim-2 PC attestations MUST read production artifact (`grep ^current_step: .factory/STATE.md`); synthetic echo/printf strings FORBIDDEN.
- **POLICY 14 5-leg quintuple parity MANDATORY** same-burst on all BC/VP/story/epic version bumps (D-490): (1) version: frontmatter, (2) body Changelog row, (3) modified[] array, (4) last_amended: text-prefix, (5) upstream-index body-table cells.
- **Verification_step 7** literal-shell 4-index self-application gate MANDATORY (D-494).
- **INV-019 cure (a)/(b)/(c) MANDATORY** in ALL BC changelog rows AND persisted adversary reports (D-489 + D-493).
- **INV-020 / POLICY 14:** Cross-BC parity sweep required whenever ANY BC in a group (BC-5.39.006/007/008) is modified (D-491).
- **Adversary MUST grep `origin/develop` or `factory-artifacts` for literal-shell evidence** (NOT stale local main); per L-EDP1-067-CANDIDATE / D-482.
- **Cure-extension parsimony (D-497):** When META-LEVEL recurrence is structurally same class as prior INV, EXTEND the existing cure rather than introduce new INV-N abstraction.
- **POLICY 8 v1.3 EC-mirror routing-rule (D-517):** When a story adds an Edge Case (EC) row that references a BC anchor, orchestrator MUST dispatch same-burst PO mirror.
- **POLICY 8 bidirectional parity + audit-block-exclusion (D-515+D-516):** After any PC insertion/deletion/renumbering in BC, story-writer MUST run literal-shell bidirectional AC↔PC parity check with captured stdout per POLICY 15.
- **POLICY 5 v1.3.1 SDK-grounding stable-anchor sub-clause (D-517):** Every BC narrative claim about external artifact MUST have literal-shell grep stdout at §SDK Grounding Evidence using stable anchors. NO grep -n line numbers.
- **POLICY 5 v1.3.3/v1.3.4/v1.3.5/v1.3.6 sibling-sweep mandates (D-518/519/520/521):** Sibling-sweep required with captured-stdout proof; historical-by-construction enumeration (5 forms only); HEAD-reproducibility mandate; snapshot-rescue detection.
- **Last Updated cell trajectory-tail marker requirement (PC2):** Last Updated cell MUST include `trajectory-tail →N→N→N→N` marker (BC-5.39.009 v1.8 PC2 compliance).
- **D-537 [process-gap] spec-drift routing obligation (D-539 carry):** When an implementer TDD fix changes behavior an accepted ADR specifies verbatim, the fix-burst MUST route an architect ADR amendment in the SAME burst per CLAUDE.md Architectural Authority §12 (spec wins). Codified in ADR-024 v1.2 Process note; L-issue-130-3pass-convergence lesson (b).
- **D-539 multi-family adversary obligation:** For issues involving both prompt-contract discipline and shell-logic correctness, both a cross-family adversary pass AND a same-family Claude adversary pass are required before declaring convergence. Per L-issue-169-176-worktree-identity (a).
- **D-540 carry:** ADR-025 v1.2 ACCEPTED. Issue #170 factory lock/lease design-codified. 9 deliverables enumerated (see D-540 in decision-log.md SoT). Implementation next: test-writer Red Gate on feature/issue-170-factory-locklease.
- **D-546 carry:** S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT COMPLETE. 3 bin/ helpers (factory-lock-status.sh + factory-lock-acquire-precheck.sh + factory-unlock-decide.sh) + 3 bats added. SKILL.md files are thin orchestrators delegating STATE.md write to state-manager via S-17.01 helpers (factory-lock-write.sh). BC-6.23.001 UNCHANGED. 14 ACs preserved. STORY-INDEX v3.91. BC-INDEX v2.70 UNCHANGED. ARCH-INDEX v2.20 UNCHANGED. NEXT: S-17.03 test-writer Red Gate on feature/S-17.03-factory-lock-skills (E-17 Wave 3).
- **D-545 carry:** S-17.02 DELIVERED/MERGED. PR #182 squash-merged df4f26b8. BC-4.13.001 v1.3 ACTIVE (POL-14). ADR-025 v1.3 (env_allow footgun — 3rd silent-no-op vector). Issue #170 partial-close — S-17.03 (/factory-lock+/factory-unlock skills; BC-6.23.001) remains draft. STORY-INDEX v3.90; BC-INDEX v2.70; ARCH-INDEX v2.20. RC RELEASE PENDING (#128+#130+#169+#176+#170-S17.01+#170-S17.02).
- **D-544 carry (superseded):** S-17.01 DELIVERED/MERGED c64b46d2; BC-5.40.001 ACTIVE; E-17 1/3 (now 2/3 per D-545).
- **D-541 carry:** BC-6.23.001 remains draft (S-17.03 not yet merged); VP IDs TBD per TD-VSDD-063; POLICY 8 propagation deferred to S-17.03.

### §4. Tier-A Completed Log

All S-15.03 PRIORITY-A items SHIPPED. Key entries (most recent first):
- **D-546 (2026-06-11):** S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT. 3 bin/ helpers (factory-lock-status.sh + factory-lock-acquire-precheck.sh + factory-unlock-decide.sh) + 3 bats. SKILL.md thin orchestrators. BC-6.23.001 UNCHANGED. 14 ACs preserved. STORY-INDEX v3.90→v3.91. S-17.03 ready for Red Gate.
- **D-545 (2026-06-11):** S-17.02 DELIVERED/MERGED. PR #182 squash-merged df4f26b8. 13/13 bats green. 23 unit tests. Trend 1H+2M+4L→1M→0→0→0 3-CLEAN. BC-4.13.001 POL-14 active. ADR-025 v1.3 env_allow footgun. issue #170 partial-close. STORY-INDEX v3.89→v3.90. BC-INDEX v2.69→v2.70. ARCH-INDEX v2.19→v2.20. develop df4f26b8. E-17 2/3 stories merged.
- **D-544 (2026-06-11):** S-17.01 DELIVERED/MERGED. PR #181 squash-merged c64b46d2. 22/22 bats green. BC-5.40.001 POL-14 active. E-17 1/3.
- **D-540..D-543 (2026-06-10):** ADR-025 v1.2 ADOPTED; 3-BCs-AUTHORED (BC-INDEX v2.66); E-17+S-17.01/02/03 AUTHORED; S-17.01-v1.1-helpers. Full rows: decision-log.md SoT.
- **D-532..D-539 (2026-06-08..2026-06-10):** SESSION-END bursts; #128+#130+#169+#176 DELIVERED/MERGED; ADR-024 v1.2; ARCH-INDEX v2.18. Full rows: decision-log.md SoT.
- **D-531 (2026-06-01):** E-10 CASCADE SEALED. Pass-16 asymptotic-acceptance per D-471+D-386 Option C.
- **D-528 (2026-06-01):** RC.20 SHIPPED. Plugin 52→53. Marketplace #12.
- **D-526 (2026-05-31):** S-15.17 SHIPPED PR #164 9ed17b1d. BC-5.39.009 ACTIVE. BC-INDEX v2.65. STORY-INDEX v3.84.
- **D-508 (2026-05-27):** S-15.03 PRIORITY-A COMPLETE. All 11 stories. 40pts.

**Current Active:** D-546 COMPLETE. Issues #128, #130, #169, #176 all DELIVERED/MERGED. Issue #170 S-17.01+S-17.02 DELIVERED/MERGED (D-544+D-545; S-17.03 delivery-prep COMPLETE D-546). BC-5.40.001+BC-4.13.001 ACTIVE. BC-6.23.001 DRAFT. Requires rc release for operator cache. Next = S-17.03 test-writer Red Gate E-17 Wave 3 OR rc release OR #129 canonical-principle.

Prior Tier-A (pre-session, all COMPLETE): TD #71/72/70/74 (PRs #138–141) + S-15.04/05/08/07/11/09/14 (PRs #142–148) + 3M3a D-497 + D-498/504/507 durability + D-499/500/501 3M3b/r/remove-uncertainty + D-502/503/505/506/508 Waves 1-4 + D-509 E-10 pass-15 + D-510 F5 pass-75 + D-511 banner remediation + D-523/525/529/530/536 prior milestones.

### §5. Cumulative Codifications
- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-546 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Key: D-497 BC cascade CONVERGED; D-508 Wave 4 + S-15.03 PRIORITY-A COMPLETE; D-510 F5 pass-75 + META-LEVEL-30; D-512 rc.19 SHIPPED; D-526 S-15.17 SHIPPED PR #164 9ed17b1d; D-528 RC.20 SHIPPED 2026-06-01; D-531 E-10 CASCADE SEALED 2026-06-01; D-535 ISSUE-128 PR-178 MERGED 2026-06-09; D-537 ISSUE-130 PR-179 MERGED 2026-06-10 (ADR-024 v1.2; ARCH-INDEX v2.18); D-539 ISSUE-169+176 PR-180 MERGED 2026-06-10; D-540 ADR-025 ADOPTED 2026-06-10 (ARCH-INDEX v2.19); D-541 BC-AUTHORING COMPLETE 2026-06-10 (BC-INDEX v2.66); D-542 E-17+S-17.01/02/03 AUTHORED 2026-06-10; D-544 S-17.01 DELIVERED/MERGED 2026-06-11 (BC-5.40.001 ACTIVE; develop c64b46d2; E-17 1/3); D-545 S-17.02 DELIVERED/MERGED 2026-06-11 (BC-4.13.001 ACTIVE; ADR-025 v1.3; develop df4f26b8; E-17 2/3); **D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-11 — 3 bin/ helpers + 3 bats; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; S-17.03 ready for Red Gate**.

### §6. Cumulative Lessons
- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade-CONVERGED + L-session-2026-05-20-resume-CONVERGENCE + L-E10-pass15-automation-wave-effectiveness + L-banner-format-drift + L-rc19-pre-release-validation-banner-format-drift + L-S-15.17-BC-authoring-clean-propagation + L-S-15.17-SP1..SP7 lessons + L-S-15.17-SP8-META-36-snapshot-rescue + L-S-15.17-SP8-TD-VSDD-059-paper-fix-detection + L-S-15.17-SP9-META-37-asymptotic-acceptance-SEAL + L-S-15.17-cascade-9-pass-SEAL-precedent + L-S-15.17-remove-uncertainty-clean-result + L-F-P3-008-wallclock-deflake-structural-recurrence + L-session-2026-05-31-fabricated-SHA-discipline + L-session-2026-06-01-rc20-clean-ship + L-session-2026-06-01-dependabot-sweep + L-E10-pass16-derived-ci-count + L-E10-cascade-SEAL-16-pass + L-session-2026-06-08-session-end-durability + L-issue-128-cross-family-adversary + L-issue-128-PR-178-merged + L-issue-130-3pass-convergence + L-session-2026-06-10-issue-128-130-delivered-durability + **L-issue-169-176-worktree-identity: (a) multi-family adversary diversity is load-bearing — one family catches CRITICAL the other misses; (b) extract mechanical logic to tested bash helper vs inline markdown; (c) run bats from mount-less feature worktree to reproduce #169 phantom-failure class; (d) fix dogfooded its own discipline — adversary reviewed with embedded worktree-identity tuple** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope (Cumulative)
11-story wave S-15.06..S-15.16. **ALL SHIPPED:** M1 + M2 + M3 Wave 1+2+3+4. **S-15.03 PRIORITY-A COMPLETE. All 11 stories. 40pts M3 total.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.70 | D-545 BC-4.13.001 POL-14 draft→active (PR #182 S-17.02 merged); body row draft→active + v1.0→v1.1→v1.2→v1.3; total_bcs 1958 UNCHANGED |
| VP-INDEX | v2.06 | UNCHANGED (18 VPs pending architect per TD-VSDD-063) |
| STORY-INDEX | v3.91 | D-546: S-17.03 v1.0→v1.1 executable-helper refinement; 3 bin/ helpers + 3 bats; BC-6.23.001 UNCHANGED; 14 ACs preserved |
| ARCH-INDEX | v2.20 | D-545 ADR-025 v1.2→v1.3 (env_allow footgun — 3rd silent-no-op vector enumerated; ADR count 25) |

### §9. Critical Anchors

- **factory-artifacts HEAD:** `171a9413` D-546 S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-11 (prior: `735b9168` D-545; prior-prior: `37414e5a` D-544-sha-patch)
- **develop HEAD:** `df4f26b8` (D-545 PR #182 S-17.02 squash-merge 2026-06-11; prior: `c64b46d2` D-544 PR #181 S-17.01 2026-06-11)
- **main HEAD:** `2a191314` (rc.20 bot binary commit 2026-06-01; prior: `43afbfa7` rc.19 2026-05-28)
- **v1.0.0-rc.20 tag:** `e9e38286` (annotated tag object; GitHub Release prerelease 2026-06-01; marketplace PR #12 squash-merged 862e660d)
- **v1.0.0-rc.19 tag:** `d15152af` (GitHub Release 2026-05-28T15:10:56Z)
- D-526: PR #164 merge commit `9ed17b1d`; PR #165 merge commit `f34b7567`; PR #163 merge commit `766ab7bc`
- D-530: PR #168 merge commit `82163b7f`
- **BC-5.39.009:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md` v1.9 ACTIVE (POL-14 promoted on PR #164 merge; BC-INDEX v2.65; 13 PCs; hooks-registry priority 158)
- **S-15.17 story:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md` v1.11 (MERGED PR #164 9ed17b1d; STORY-INDEX v3.85)
- **policies.yaml:** `.factory/policies.yaml` (v1.3.6 — POLICY 5 v1.3.6 SEALED; no further cures)
- D-535: PR #178 squash-merged `f6ce4b7c` 2026-06-09; feature/issue-128-verify-branch-deletion DELETED+VERIFIED
- D-537: PR #179 squash-merged `89fbe2d6` 2026-06-10; feature/issue-130-dispatcher-log-shadow DELETED+VERIFIED; ADR-024 v1.2 amended
- D-539: PR #180 squash-merged `0f4793f1` 2026-06-10; feature/issue-169-176-worktree-identity DELETED+VERIFIED; LOCAL 3-CLEAN SHA `5ea02ecf`
- **ADR-024 v1.2:** `.factory/specs/architecture/decisions/ADR-024.md` — ARCH-INDEX v2.18; Decision 3 bounded char-safe N=4096 + Decision 4 lexical-normalization guard + [process-gap] spec-drift routing Process note
- **ADR-025 v1.3:** `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — ARCH-INDEX v2.20; env_allow=["HOME","GIT_CONFIG_GLOBAL","XDG_CONFIG_HOME"] footgun (3rd silent-no-op vector) enumerated in Accepted Tradeoffs; D-545
- Verify on resume: `git rev-parse --short origin/develop` → expect `df4f26b8`
- **BC-5.40.001:** `.factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md` v1.1 ACTIVE (POL-14 auto-promoted on PR #181 merge; BC-INDEX v2.70; factory_lock schema+TTL+CAS; SS-05)
- **BC-4.13.001:** `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` v1.3 ACTIVE (POL-14 auto-promoted on PR #182 merge; BC-INDEX v2.70; verify-factory-lock WASM guard; SS-04; env_allow PC7+Inv5+EC-016; boundary now>=expires_at)
- **S-17.01 story:** `.factory/stories/S-17.01-factory-lock-schema-cas-push.md` v1.4 MERGED PR #181 c64b46d2 (STORY-INDEX v3.90; E-17 W1)
- **S-17.02 story:** `.factory/stories/S-17.02-verify-factory-lock-wasm-guard.md` v1.5 MERGED PR #182 df4f26b8 (STORY-INDEX v3.90; E-17 W2; D-545 codified)
- **S-17.03 story:** `.factory/stories/S-17.03-factory-lock-unlock-skills-health.md` v1.1 (draft; BC-6.23.001; W3; STORY-INDEX v3.91; 3 bin/ helpers + 3 bats added; test-writer Red Gate next)

### §10. PR Status

- **0 open source PRs; issue #170 PARTIALLY-DELIVERED (S-17.01+S-17.02 MERGED; S-17.03 remains); rc release PENDING for #128+#130+#169+#176+#170-S17.01+#170-S17.02 operator reach.**
- **MERGED (D-545 issue #170 S-17.02):** PR #182 feature/S-17.02-verify-factory-lock-wasm-guard `df4f26b8` 2026-06-11; branch deleted+verified; verify-factory-lock WASM guard; 23 unit + 13 bats; 3-CLEAN 1H+2M+4L→1M→0→0→0; BC-4.13.001 POL-14 active; ADR-025 v1.3. REQUIRES rc release for operator cache.
- **MERGED (D-544 issue #170 S-17.01):** PR #181 feature/S-17.01-factory-lock-schema-cas-push `c64b46d2` 2026-06-11; branch deleted+verified; factory_lock schema+CAS push; 22/22 bats; 3-CLEAN 9→3→0→0→0; BC-5.40.001 POL-14 active. REQUIRES rc release for operator cache.
- **MERGED (D-539 issues #169+#176):** PR #180 `0f4793f1` 2026-06-10; REQUIRES rc release for operator cache.
- **MERGED (D-537 issue #130):** PR #179 `89fbe2d6` 2026-06-10; ADR-024 v1.2; REQUIRES rc release for operator cache.
- **MERGED (D-535 issue #128):** PR #178 `f6ce4b7c` 2026-06-09; ships in next rc release.
- **MERGED (D-530 E-10 pass-16):** PR #168 `82163b7f` 2026-06-01.
- **MERGED (rc.20 bundle):** PR #166 `e00ab1ab`; PR #163 `766ab7bc`; PR #164 `9ed17b1d`; PR #165 `f34b7567`.
- **Marketplace:** drbothen/claude-mp PR #11 (rc.19) + PR #12 (rc.20) both squash-merged.
- **Next source PR:** test-writer Red Gate feature branch S-17.03 for #170 (D-546 S-17.03 v1.1 delivery-prep COMPLETE; BC-6.23.001 draft; feature/S-17.03-factory-lock-skills; E-17 Wave 3; 3 bin/ helpers+3 bats ready); OR rc release (ships #128+#130+#169+#176+#170-S17.01+#170-S17.02 to operator cache — #130+#169+#176+S17.01+S17.02 code+hooks REQUIRE it); OR issue #129 canonical-principle.

### §11. Post-CLEAR Resume Checklist (zero-context)

1. **Verify worktree state (TWO worktrees only):** Main repo: `git rev-parse --abbrev-ref HEAD` → expect `develop` (HEAD `df4f26b8`). Factory: `git -C .factory log -1` + `git -C .factory status` (expect clean; branch factory-artifacts). No feature branches open. Main repo + .factory are the only worktrees.
2. **Read this checkpoint** (entire §1-§12).
3. **Verify PC4 (trajectory-tail segment LENGTH=4):** `grep "^current_step:" .factory/STATE.md | grep -oE "trajectory-tail [→0-9]+"` → expect `trajectory-tail →9→9→9→11`.
4. **Verify develop HEAD:** `git rev-parse --short origin/develop` → expect `df4f26b8`.
5. **E-10 CASCADE SEALED (D-531 2026-06-01).** Pass-16 asymptotic-acceptance per D-471+D-386 Option C. 16-pass trend ends at 3 (LOW). S-7.02 SATISFIED. Resumption gate = engine-surface material change. Do NOT resume E-10 without engine-surface material change.
6. **BC-5.39.009 v1.9 STATUS:** ACTIVE. BC-INDEX v2.65. hooks-registry priority 158. IN OPERATOR CACHE rc.20.
7. **S-15.17 v1.11 STATUS:** MERGED PR #164. STORY-INDEX v3.84.
8. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume F5 without explicit human direction.
9. **RC.20 SHIPPED D-528 (2026-06-01):** run 26738809372 all 6 PASS; tag e9e38286; main 2a191314; marketplace #12; plugin 52→53; operator cache updated. NEXT RC RELEASE PENDING: #128 + #130 (crates+hooks) + #169+#176 (agents/skills/bin/bats) + **#170-S17.01 (factory-lock-write.sh+factory-cas-push.sh; SKILL.md+state-manager.md)** + **#170-S17.02 (verify-factory-lock WASM crate + hooks-registry entry)** — all require rc.21+ for operator cache reach.
10. **ALL dispatches carry these non-negotiables:** TD-VSDD-097-EXT (all 5 BC-5.39.006 PCs in current_step:) + TD-VSDD-099 (4 Dim blocks in burst-log) + TD-VSDD-100 (production artifact read, no synthetic echo) + POLICY 14 5-leg quintuple parity + verification_step 7 4-index gate + INV-019 cure (a)/(b)/(c) in changelog rows + adversary must grep origin/factory-artifacts (not stale local main) + D-449(a) literal-shell all Dim-2 gates (no pseudocode) + POLICY 8 v1.3 bidirectional AC↔PC parity + audit-block-exclusion + EC-mirror routing-rule + POLICY 5 v1.3.1 SDK-grounding stable-anchor mandate + POLICY 5 v1.3.4 literal-shell VERIFICATION GATE + POLICY 5 v1.3.5 historical-by-construction enumeration (5 forms only) + POLICY 5 v1.3.6 HEAD-reproducibility + structural-form-only + snapshot-rescue detection + D-537/D-539 [process-gap] spec-drift routing: implementer TDD fix changing ADR-specified behavior MUST route architect ADR amendment in same burst + D-539 multi-family adversary obligation: prompt-contract + shell-logic issues require cross-family AND same-family Claude adversary passes.
11. **All caught up.** Next decision is D-547. #128 D-535. #130 D-537. #169+#176 D-539. D-540 ADR-025 ADOPTED. D-541 3-BCs-AUTHORED. D-542 E-17+stories. D-544 S-17.01 MERGED. D-545 S-17.02 DELIVERED/MERGED. **D-546 S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-11 — 3 bin/ helpers + 3 bats; SKILL.md thin orchestrators; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.91.** All PRs require rc release for operator-level cache. Options: (a) S-17.03 test-writer Red Gate on feature/S-17.03-factory-lock-skills (RECOMMENDED — E-17 Wave 3); (b) rc release to ship #128+#130+#169+#176+#170-S17.01+#170-S17.02 to operators; (c) #129 canonical-principle; F5 pass-76 (PAUSED, needs human); forward proposals (UNI-PLUG-001+SK-MCP-001).

### §12. Pending Work Items — Strict Engine-Discipline Ordering (refreshed 2026-06-11 post-D-545)

| Step | Item | Tier | Gate | Status / Scope |
|------|------|------|------|---------------|
| ~~1~~ | ~~TD #74 dispatch-package cargo-audit~~ | ~~A~~ | ~~—~~ | **SHIPPED 2026-05-15** |
| ~~2~~ | ~~TD #66/67 cleanup~~ | ~~A~~ | ~~(1) done~~ | **COMPLETE 2026-05-15** |
| ~~3~~ | ~~S-15.03 PRIORITY-A lint-hook automation~~ | ~~D~~ | ~~(2) done~~ | **COMPLETE 2026-05-27 D-508 — ALL 11 STORIES SHIPPED** |
| ~~3M3c~~ | ~~M3 per-story-delivery~~ | ~~D~~ | ~~(D-501) done~~ | **COMPLETE — Wave 1+2+3+4 SHIPPED D-508; 40pts; S-15.03 PRIORITY-A COMPLETE** |
| ~~4~~ | ~~E-10 resumption (pass-15+)~~ | ~~gated~~ | ~~(3) COMPLETE~~ | **E-10 pass-15 COMPLETE D-509 — PR #160 4b68ab83** |
| ~~5a~~ | ~~F5 pass-75 fix-burst~~ | ~~gated~~ | ~~done~~ | **F5 PASS-75 COMPLETE D-510 — META-LEVEL-30 CANDIDATE-CONFIRMED** |
| ~~rc.19~~ | ~~v1.0.0-rc.19 release~~ | ~~release~~ | ~~done~~ | **SHIPPED D-512 2026-05-28 — run 26581752361; tag d15152af; marketplace PR #11** |
| ~~5a-prime~~ | ~~BC-5.39.009 v1.0 AUTHORED + S-15.17 v1.1 PROPAGATED~~ | ~~A~~ | ~~—~~ | **COMPLETE D-513 2026-05-28** |
| ~~5b-cascade~~ | ~~S-15.17 adversarial cascade (passes 1-9)~~ | ~~gated~~ | ~~BC-5.39.001 (D-386 Option C SEAL)~~ | **SEALED D-522 2026-05-29** |
| ~~5b-remove-uncertainty~~ | ~~S-15.17 remove-uncertainty sweep~~ | ~~gated~~ | ~~D-522 SEAL~~ | **COMPLETE D-523 2026-05-30** |
| ~~5b-impl~~ | ~~S-15.17 per-story-delivery~~ | ~~ACTIVE NEXT~~ | ~~D-523 CLEAN~~ | **SHIPPED D-526 2026-05-31** — PR #164 9ed17b1d; BC-5.39.009 ACTIVE |
| ~~rc.20~~ | ~~v1.0.0-rc.20 release~~ | ~~release~~ | ~~done~~ | **SHIPPED D-528 2026-06-01 — run 26738809372; tag e9e38286; main 2a191314; marketplace PR #12** |
| ~~MAINT~~ | ~~POST-RC.20 MAINTENANCE SWEEP~~ | ~~READY~~ | ~~—~~ | **COMPLETE D-529 2026-06-01** — td-74 removed; #3+#156+#157 MERGED; zero open PRs; develop b21fd358. |
| ~~4-next~~ | ~~E-10 pass-16~~ | ~~gated→READY~~ | ~~S-15.03 PRIORITY-A COMPLETE (D-508)~~ | **COMPLETE D-530 2026-06-01** — verdict LOW (0C+0H+0M+3L); F-PASS16-002 FIXED PR #168 82163b7f. |
| ~~4-seal~~ | ~~E-10 SEAL-vs-pass-17 decision~~ | ~~PENDING~~ | ~~human direction~~ | **SEALED D-531 2026-06-01** — E-10 cascade SEALED pass-16 asymptotic-acceptance; S-7.02 SATISFIED. |
| ~~SESSION-END~~ | ~~D-532 SESSION-END DURABILITY BURST~~ | ~~durability~~ | ~~—~~ | **COMPLETE 2026-06-08** — D-430(a) compaction; §1-§12 refresh; 2 follow-up candidates captured. |
| ~~#128~~ | ~~Issue #128 pr-manager branch-deletion verify~~ | ~~bug~~ | ~~—~~ | **DELIVERED/MERGED — PR #178 squash-merged f6ce4b7c 2026-06-09** (TDD 45/45; Gemini 3-pass 6→4→4; branch DELETED+VERIFIED) |
| ~~#130~~ | ~~Issue #130 dispatcher `.factory/.factory/` log shadow~~ | ~~bug~~ | ~~—~~ | **DELIVERED/MERGED — PR #179 squash-merged 89fbe2d6 2026-06-10** (7-level worktree-aware log-dir; 3-pass adversary CLEAN; ADR-024 v1.2; ARCH-INDEX v2.18; requires rc release for operator cache) |
| ~~#169+#176~~ | ~~Issues #169+#176 worktree-identity couple~~ | ~~bug~~ | ~~—~~ | **DELIVERED/MERGED — PR #180 squash-merged 0f4793f1 2026-06-10** (worktree-identity preflight + resolve-worktree-identity.sh + 28 bats tests; multi-family 3-CLEAN SHA 5ea02ecf; CI 11/11 GREEN; requires rc release for operator cache) |
| **CR-DEFERRED** | **2 S-15.17 code-reviewer suggestion-level findings** | **deferred** | — | ACCEPTED-DEFERRED (see Drift Items S-15.17-CR-001 + S-15.17-CR-002). Advisory-arm only; no production reachability. |
| **FLAKE-001** | **test_F_P2_001 timing flake de-flake** | **candidate** | human-authorize | darwin-x64 3761ms vs 3000ms; same class as TD #67/F-P3-008; event-observation rewrite candidate. D-532 capture. |
| **COSMETIC-001** | **O-PASS16-002 stale header doc-comment** | **cosmetic** | on next spec-touch | validate-trajectory-tail-cell-completeness extract/check function headers stale. Cleanup on next S-15.17 spec-touch. D-532 capture. |
| ~~**#170-S17.01**~~ | ~~**Issue #170 S-17.01 factory_lock schema+CAS push**~~ | ~~**implementation**~~ | ~~D-543 S-17.01 v1.1 ready~~ | **DELIVERED/MERGED — PR #181 squash-merged c64b46d2 2026-06-11** (22/22 bats; 3-CLEAN; BC-5.40.001 ACTIVE) |
| ~~**#170-S17.02**~~ | ~~**Issue #170 S-17.02 verify-factory-lock WASM guard**~~ | ~~**implementation**~~ | ~~D-544 S-17.01 MERGED~~ | **DELIVERED/MERGED — PR #182 squash-merged df4f26b8 2026-06-11** (23 unit + 13 bats; 3-CLEAN; BC-4.13.001 ACTIVE; ADR-025 v1.3 env_allow footgun; issue #170 partial-close) |
| **#170-S17.03** | **Issue #170 S-17.03 /factory-lock+/factory-unlock+/factory-health skills** | **implementation** | D-545 S-17.02 MERGED | ADR-025 v1.3 ACCEPTED; BC-6.23.001 v1.0 DRAFT; S-17.03 **v1.1** draft (D-546 executable-helper refinement COMPLETE: 3 bin/ helpers + 3 bats); **test-writer Red Gate on feature/S-17.03-factory-lock-skills NEXT** (E-17 Wave 3) |
| **5c** | **F5 pass-76** | **gated** | EXPLICIT human direction required | PAUSED per D-386 Option C. Pass-75 trajectory →9→9→9→11. Do NOT resume without explicit human direction. |
| **6** | **UNI-PLUG-001 implementation** | **forward** | human-authorize | **PROPOSAL REVIEW-READY** |
| **7** | **SK-MCP-001 implementation** | **forward** | (6) Tier 1 done | **PROPOSAL REVIEW-READY** |

### Validated GitHub-Issue Backlog (2026-06-09 sweep — D-533; updated 2026-06-10 D-539)

Research cached at `.factory/research/issues/issue-<N>.md`; full triage at `.factory/research/issues/INDEX.md`.

| Cluster | Issues | Verdict |
|---------|--------|---------|
| Bug: PR-lifecycle | ~~#128 pr-manager branch-deletion verify~~ | **DELIVERED/MERGED — PR #178 squash-merged f6ce4b7c 2026-06-09** |
| Bug: dispatcher | ~~#130 `.factory/.factory/` log shadow~~ | **DELIVERED/MERGED — PR #179 squash-merged 89fbe2d6 2026-06-10** (requires rc release for operator cache) |
| Worktree-identity (fix together) | ~~#169 stale-spec sub-agents + #176 adv-review preflight~~ | **DELIVERED/MERGED — PR #180 squash-merged 0f4793f1 2026-06-10** (requires rc release for operator cache) |
| State durability/concurrency (#170→#173→#171) | **#170 factory lock/lease** (D-544 S-17.01 MERGED c64b46d2; D-545 S-17.02 MERGED df4f26b8; S-17.03 skills REMAIN; issue #170 partial-close; S-17.03 test-writer Red Gate NEXT) + #173 wave-checkpoint + #171 deferred-revalidate | VALID-NEW×3 (#170 S-17.03 next) |
| Runtime enforcement (#162 umbrella + #133/#177) | #162 orchestrator enforcement + #133 intra-phase adversary + #177 hollow-demo | VALID-NEW/PARTIAL |
| Consistency/citation (ship together) | #151 drift checker + #131 URL/path coherence | VALID-PARTIAL/NEW |
| Pre-Phase-3 gate | #150 uncertainty-removal/self-containment | VALID-PARTIAL |
| Canonicalization | #129 canonical-principle in shipped plugin | VALID-NEW |
| Demo-evidence routing | #172 route demo evidence → factory-artifacts | VALID-NEW |
| Doc governance | #174 CLAUDE.md health-check | VALID-NEW |
| Activate | #175 version-drift block hook | VALID-PARTIAL |
| **ALREADY-DONE** | **#149 OTEL telemetry — recommend GitHub close** | ALREADY-DONE |

**RECOMMENDED ACTIVE NEXT:** (a) **#170 S-17.03 test-writer Red Gate** (D-546 S-17.03 v1.1 delivery-prep COMPLETE; BC-6.23.001 draft; feature/S-17.03-factory-lock-skills; E-17 Wave 3 skills crate; 3 bin/ helpers + 3 bats ready); (b) **rc release** to ship #128+#130+#169+#176+#170-S17.01+#170-S17.02 to operator cache (#130+#169+#176+S17.01+S17.02 code+hooks REQUIRE it); (c) **#129 canonical-principle** (VALID-NEW; ship-ready); (d) **#173 wave-checkpoint** (state-durability chain next after #170); F5 pass-76 (PAUSED; needs explicit human direction); UNI-PLUG-001/SK-MCP-001 REVIEW-READY.

**Track-independent:** E-9 W-16 Tier 2 + E-11 W-17 Tier 3 + verify-git-push.sh + S-10.08 + S-11.00.

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-545 S-17.02-DELIVERED-MERGED-2026-06-11) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
