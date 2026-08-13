---
document_type: pipeline-state
level: ops
version: "7.37"
status: draft
producer: state-manager
timestamp: 2026-08-13T13:33:00Z
phase: D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST
last_amended: "2026-08-13 (v7.37) — D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST (state-manager; single-commit TD-VSDD-053; parent-commit e541668e): After the D-988 BC-5.39.001 TRUE 3-CLEAN RE-CONVERGENCE (verified at impl `1c93f499`), cross-platform CI's `build-dispatcher (windows-x64)` job on PR #775 caught a real defect the all-macOS LOCAL adversary cascade + exhaustive mutation-completeness audit structurally could not: `test_S_21_09_ac006_T026_absolute_form_excluded_from_declared` sub-test (b) panicked on Windows because its TOML fixture path was built via `PathBuf::push` (OS-native separator → `\` on Windows → invalid TOML escape). test-writer fixed it in `c20cf2fe` (forward-slash string concatenation; full ~90-site audit confirmed T-026(b) was the sole non-portable site; 51 green macOS; fmt/clippy clean). **D-988 RE-CONVERGENCE PRESERVED, not reopened** — `c20cf2fe` is a test-FIXTURE-CONSTRUCTION portability change with identical runtime semantics on every platform (no assertion/logic/mutation-control/test-set change); **BC-5.39.001 streak REMAINS 3/3 RE-CONVERGED**, impl advanced `1c93f499`→`c20cf2fe`. story-writer story v1.32 documents the fix + SHA sweep. The 4 fix commits (`c9cccea9`/`fc0e613b`/`1c93f499`/`c20cf2fe`) were pushed to `origin/feature/S-21.09` DIRECTLY BY THE ORCHESTRATOR (human-authorized) — the pr-manager→github-ops push delegate was non-functional this session (landed the first push, then failed every subsequent attempt). PR #775 HEAD now `c20cf2fe`; CI re-running (windows-x64 fix in place). INDEX.md POST-RE-CONVERGENCE row appended + Convergence Status leading declaration updated (prior D-988 declaration preserved as `[Prior state, superseded 2026-08-13 D-989]`). decision-log D-989 block appended (9 lettered sub-paragraphs). burst-log D-989 8-block entry appended. Two lessons appended to `lessons.md`: `L-BB-cross-platform-ci-is-convergence-prerequisite-not-just-merge-prerequisite` [process-gap] + `L-BB-github-ops-push-delegate-non-functional-mid-session` [process-gap]. STORY-INDEX v4.316→v4.317 (S-21.09 catalog row: story v1.32, impl `c20cf2fe`, PUSHED). **Final gate: PR #775 review refresh + full CI (incl. windows-x64), now running against `c20cf2fe`.** Cycle-level trajectory-tail →20→16→8→10 UNCHANGED. STATE.md v7.36→v7.37. [Prior: 2026-08-13 (v7.36) — SHA-PATCH-2026-08-13 (state-manager; parent-commit: b31de9e2): Active Branches SHA-patched factory-artifacts e86c8c92→b31de9e2 (actual commit HEAD); Session Resume Checkpoint header + §1 parent-citation updated e86c8c92→b31de9e2; v7.36 UNCHANGED. [Prior: 2026-08-13 (v7.36) — D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE (state-manager; single-commit TD-VSDD-053; parent-commit e86c8c92; commit b31de9e2): LOCAL adversary pass-24 recorded — the FIFTH pass dispatched under the strengthened rubric (vacuity/tautology/mutation-narrative-accuracy/format-lock/sweep-completeness checks per D-983/D-984/D-985). **Verdict: CLEAN — zero findings at any severity, no process-gap.** LOCAL BC-5.39.001 streak ADVANCES 2/3 → 3/3 — TRUE 3-CLEAN RE-CONVERGED (passes 22+23+24 all CLEAN under the strengthened rubric); supersedes the retracted D-982 convergence. S-7.02 Cycle-Closing Checklist SATISFIED. STORY-INDEX v4.315→v4.316. STATE.md v7.35→v7.36. Full detail: git show b31de9e2:.factory/STATE.md. [Prior: full prior chain: git show 96d2ca8d:.factory/STATE.md for D-987 pre-D-988 detail; git show b75e9e1c:.factory/STATE.md for D-986 pre-D-987 detail; also git show 9d72dc15:.factory/STATE.md per D-430(a) compaction precedent.]]]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: ACTIVE
current_step: "D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST (state-manager; single-commit TD-VSDD-053; parent-commit: e541668e; D-chain cite D-989): After the D-988 BC-5.39.001 TRUE 3-CLEAN RE-CONVERGENCE (verified at impl 1c93f499), cross-platform CI (build-dispatcher windows-x64) on PR #775 caught a real defect the all-macOS LOCAL adversary cascade + exhaustive mutation-completeness audit structurally could not: test_S_21_09_ac006_T026_absolute_form_excluded_from_declared sub-test (b) panicked on Windows because its TOML fixture path was built via PathBuf::push (OS-native separator, invalid TOML escape on Windows). test-writer fixed it in c20cf2fe (forward-slash string concatenation; full ~90-site audit confirmed T-026(b) was the sole non-portable site; 51 green macOS; fmt/clippy clean). D-988 RE-CONVERGENCE PRESERVED, not reopened — c20cf2fe is a test-fixture-construction portability change with identical runtime semantics on every platform, no assertion/logic/mutation-control/test-set change; BC-5.39.001 streak REMAINS 3/3 RE-CONVERGED, impl advanced 1c93f499→c20cf2fe. story-writer story v1.32 documents the fix + SHA sweep across 5 current-state commit cites. The 4 fix commits (c9cccea9/fc0e613b/1c93f499/c20cf2fe) were pushed to origin/feature/S-21.09 DIRECTLY BY THE ORCHESTRATOR (human-authorized) because the pr-manager→github-ops push delegate was non-functional this session (landed the first push hours earlier, then failed every subsequent attempt). PR #775 HEAD now c20cf2fe; description off-by-one already fixed (52/52); CI re-running (windows-x64 fix in place). INDEX.md S-21.09 LOCAL Adversary Reviews table extended with a POST-RE-CONVERGENCE row (not a numbered pass); Convergence Status leading declaration REPLACED to note the fix + convergence-preserved disposition, prior D-988 declaration preserved as [Prior state, superseded 2026-08-13 D-989]. decision-log D-989 block appended (9 lettered sub-paragraphs (a)-(i)). burst-log D-989 8-block entry appended (gates literally re-executed, all PASS). Two lessons appended to lessons.md: L-BB-cross-platform-ci-is-convergence-prerequisite-not-just-merge-prerequisite [process-gap] (all-macOS LOCAL cascade + mutation audit cannot catch platform-specific defects; cross-platform CI must be a convergence prerequisite, not just a merge prerequisite) and L-BB-github-ops-push-delegate-non-functional-mid-session [process-gap] (operational Drift Item; anchor S-15.03 PRIORITY-A / devops follow-up). STORY-INDEX v4.316→v4.317 (S-21.09 catalog row: story v1.31→v1.32, impl SHA cite 1c93f499→c20cf2fe, annotation updated to LOCAL 3-CLEAN RE-CONVERGED (D-988, preserved); windows-x64 CI fix c20cf2fe; PR #775 updated, CI re-running; merge pending human auth). Cycle-level trajectory-tail (S-21.07) →20→16→8→10 UNCHANGED — unrelated to this fix. feature/S-21.09 now PUSHED (origin c20cf2fe); Active Branches row updated. NEXT (human-gated): (1) PR #775 review refresh against c20cf2fe under the strengthened lens; (2) full CI green incl. windows-x64; (3) merge authorization (merge-order S-21.09 before frozen S-21.07). STATE.md v7.36→v7.37. SHA-patch follow-up REQUIRED after this commit lands (Active Branches factory-artifacts row + Session Resume Checkpoint header/§1 parent-citation must be corrected to the actual commit HEAD, per D-447(c)+D-449(e))."
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
  284 lines (wc-l post-D-987-S-21.09-LOCAL-PASS-23-CLEAN-RECORD-STREAK-ADVANCE 2026-08-13; pass-23 CLEAN, streak ADVANCES 1/3→2/3; story v1.31 UNCHANGED; impl 1c93f499 UNCHANGED; STORY-INDEX v4.315; pass-24 (strengthened rubric) NEXT — if CLEAN, true 3-CLEAN re-convergence; v7.34→v7.35; commit 96d2ca8d)
  284 lines (wc-l post-SHA-patch 96d2ca8d 2026-08-13; Active Branches factory-artifacts 604cc835→96d2ca8d; v7.35 UNCHANGED)
  290 lines (wc-l post-D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE 2026-08-13; pass-24 CLEAN, streak ADVANCES 2/3→3/3 TRUE 3-CLEAN RE-CONVERGED; supersedes retracted D-982; story v1.31 UNCHANGED; impl 1c93f499 UNCHANGED; STORY-INDEX v4.316; S-7.02 Cycle-Closing Checklist SATISFIED; v7.35→v7.36; commit b31de9e2)
  290 lines (wc-l post-SHA-patch b31de9e2 2026-08-13; Active Branches factory-artifacts e86c8c92→b31de9e2; v7.36 UNCHANGED)
  287 lines (wc-l post-D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST 2026-08-13; windows-x64 CI fix c20cf2fe, D-988 re-convergence PRESERVED; story v1.32; impl advanced 1c93f499→c20cf2fe; feature/S-21.09 PUSHED; STORY-INDEX v4.317; v7.36→v7.37; commit pending SHA-patch follow-up)
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
| **Last Updated** | 2026-08-13 — D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST: After the D-988 TRUE 3-CLEAN RE-CONVERGENCE (impl `1c93f499`), cross-platform CI's `build-dispatcher (windows-x64)` job caught a real Windows-only defect (T-026(b) TOML fixture path built via `PathBuf::push`, OS-native separator → invalid TOML escape on Windows) that the all-macOS LOCAL cascade + mutation audit structurally could not observe. Fixed in `c20cf2fe` (forward-slash string concatenation, sole non-portable site of ~90 audited; 51 green macOS; fmt/clippy clean). **D-988 RE-CONVERGENCE PRESERVED, not reopened** — fixture-construction-only change, identical runtime semantics on every platform; **BC-5.39.001 streak REMAINS 3/3**, impl advanced `1c93f499`→`c20cf2fe`. story v1.32 documents the fix. The 4 fix commits pushed to `origin/feature/S-21.09` directly by the orchestrator (human-authorized; github-ops push delegate non-functional this session). PR #775 HEAD now `c20cf2fe`; CI re-running (windows-x64 fix in place). Two lessons appended (`L-BB-cross-platform-ci-is-convergence-prerequisite-not-just-merge-prerequisite`, `L-BB-github-ops-push-delegate-non-functional-mid-session`). STORY-INDEX v4.316→v4.317. **Final gate: PR #775 review refresh + full CI (incl. windows-x64), now running.** trajectory-tail →20→16→8→10 UNCHANGED. |
| **Current Phase** | **D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST (parent-commit `e541668e`; D-chain cite D-989; PIPELINE ACTIVE). S-21.09 BC-5.39.001 streak **REMAINS 3/3 RE-CONVERGED (D-988), PRESERVED — not reopened**. story spec **v1.32**; impl **`c20cf2fe`** (advanced from `1c93f499` via a convergence-preserving Windows-portability fixture fix); 51 tests T-006..T-056 all green (45 owned + 1 registry test, count UNCHANGED); `feature/S-21.09` **PUSHED** to origin `c20cf2fe`. Windows-x64 CI on PR #775 caught `test_S_21_09_ac006_T026_absolute_form_excluded_from_declared` sub-test (b) panicking (`PathBuf::push`-built TOML fixture path, invalid escape on Windows) — a defect class the all-macOS LOCAL cascade + exhaustive mutation-completeness audit structurally could not observe. test-writer fixed it in `c20cf2fe` (forward-slash string concatenation; ~90-site audit confirmed T-026(b) was the sole non-portable site). **No new adversary pass — this is a fixture-construction fix, convergence-preserving.** The 4 fix commits were pushed directly by the orchestrator (human-authorized) because the pr-manager→github-ops push delegate was non-functional this session. PR #775 HEAD advanced to `c20cf2fe`; description off-by-one already fixed (52/52); CI re-running with the windows-x64 fix in place. **Post-fix human-gated NEXT steps: (1) PR #775 review refresh under the strengthened lens against `c20cf2fe`; (2) full CI green (incl. windows-x64); (3) merge authorization (merge-order S-21.09 before frozen S-21.07).** 4-INDEX BC v4.56/VP v2.76/STORY **v4.317**/ARCH v3.55. policies.yaml v1.4.23. trajectory-tail →20→16→8→10 UNCHANGED. SHA-patch follow-up REQUIRED — Active Branches `factory-artifacts` row + Session Resume Checkpoint header/§1 must be corrected to the actual commit HEAD after this write lands.** |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B..D-647 ALL COMPLETE/ARCHIVED | **ALL COMPLETE / ARCHIVED** | SoT: decision-log.md + burst-log.md. |
| v1.0.0-rc.20/21/22 SHIPPED | **ALL SHIPPED** | PRs merged; marketplace published. |
| D-856 RC23-SHIPPED 2026-07-18 | **SHIPPED** | GitHub Release v1.0.0-rc.23; marketplace published. |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20 | **COMPLETE** | PIPELINE UNPAUSED — S-21.01..S-21.04 delivered |
| D-890..D-987 (see decision-log.md for full range; exhaustive) S-21.04/S-21.07/S-21.09 waves 2026-07-24..2026-08-13 | **COMPLETE** | SoT: decision-log.md + burst-log.md; git show 9d72dc15:.factory/STATE.md for pre-compaction detail. |
| D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE 2026-08-13 (single commit TD-VSDD-053; parent e86c8c92; commit b31de9e2; SHA-patch done) | **COMPLETE** | LOCAL adversary pass-24 (fifth pass under strengthened rubric) **CLEAN — 0 findings**; streak ADVANCES 2/3→3/3 — TRUE 3-CLEAN RE-CONVERGED; STORY-INDEX v4.316; STATE.md v7.36. |
| D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent e541668e; SHA-patch pending) | **COMPLETE** | windows-x64 CI caught T-026(b) `PathBuf::push` TOML-fixture defect the all-macOS LOCAL cascade + mutation audit could not; fixed `c20cf2fe`; **D-988 RE-CONVERGENCE PRESERVED, streak REMAINS 3/3**; story v1.31→v1.32; `feature/S-21.09` PUSHED; STORY-INDEX v4.317; STATE.md v7.37; **final gate: PR #775 review refresh + full CI, now running.** |
| **E-18 EPIC COMPLETE 2026-07-01 D-744** | **EPIC COMPLETE** | Final story S-18.12 MERGED PR #384 ec05606a. |

## Current Phase Steps

> Rows through D-988 archived to `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md`.

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE 2026-08-13 (single commit TD-VSDD-053; parent e86c8c92; commit b31de9e2; SHA-patch done) | state-manager | COMPLETE | LOCAL adversary pass-24 (fifth pass, strengthened rubric) **CLEAN — 0 findings**; streak ADVANCES 2/3→3/3 — TRUE 3-CLEAN RE-CONVERGED; STORY-INDEX v4.315→v4.316; STATE.md v7.35→v7.36. |
| D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST 2026-08-13 (single commit TD-VSDD-053; parent e541668e; SHA-patch pending) | state-manager | COMPLETE | windows-x64 CI caught `test_S_21_09_ac006_T026_absolute_form_excluded_from_declared` sub-test (b) `PathBuf::push` TOML-fixture defect (invalid escape on Windows) the all-macOS LOCAL cascade + mutation audit structurally could not observe; test-writer fixed `c20cf2fe` (forward-slash string concatenation, sole non-portable site of ~90 audited); **D-988 RE-CONVERGENCE PRESERVED, streak REMAINS 3/3**; story-writer story v1.31→v1.32; 4 fix commits pushed directly by orchestrator (human-authorized, github-ops delegate non-functional); `feature/S-21.09` PUSHED to `c20cf2fe`; PR #775 HEAD advanced, CI re-running; two lessons appended to `lessons.md`; STORY-INDEX v4.316→v4.317; STATE.md v7.36→v7.37; **NEXT: PR #775 review refresh + full CI green → merge auth (human-gated).** |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,985 (BC-INDEX v4.56 D-972) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 130 file-resident + 17 stub IDs (STORY-INDEX v4.317 D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST 2026-08-13) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 22 (E-0..E-9=10 + E-10..E-19=10 + E-21 active + E-22 dissolved-retained; ls → 22; D-962(f)) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 43 (ADR-043 NEW D-972 proposed NOT RATIFIED; ADR-042 NEW D-964; ADR-041 NEW D-961) |
| Merged Count | merged_count | `stories/sprint-state.yaml` | 107 (STATE.md explicit counter; sprint-state predicate: 113; canonical D-853) |

## Story Status

130 file-resident + 17 stub IDs = 147 stories. E-18 EPIC COMPLETE D-744. E-22 DISSOLVED D-961 (file RETAINED per human ruling 2026-08-08).

- **Merged (107):** S-19.07 MERGED PR #670 6db4c9fc (E-19 COMPLETE 9/9). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (1):** S-21.09 (feature/S-21.09 @ `c20cf2fe`, **PUSHED**; story spec v1.32; 51 tests T-006..T-056 all green, 45 owned + 1 registry.rs unit test; 16 pts; **BC-5.39.001 streak 3/3 — RE-CONVERGED (D-988), PRESERVED** — windows-x64 CI fix `c20cf2fe` is a convergence-preserving fixture-construction change, not a reopening; adv-s21.09-local-pass-1..24.md + mutation-audit-s21.09.md + `pr775-review-findings.md`; C-1/C-2/C-4/C-5 blocking issues open; 4 pass-10 carry-over findings open (MED-001, LOW-001/002/003, anchored to next maintenance sweep, not a convergence blocker); PR #775 HEAD `c20cf2fe`, CI re-running — **review refresh + CI-green → merge auth remain human-gated**)
- **E-21:** S-21.07 (W4; pass-10 NOT-CLEAN 10 findings D-967; branch 5370db80 pushed; FROZEN; NO REBASE; MERGE-ORDER: S-21.09 first); S-21.09 (**streak 3/3 RE-CONVERGED (D-988), PRESERVED**; 16 pts; feature/S-21.09 @ `c20cf2fe` PUSHED; story spec v1.32; MUST land before S-21.07); S-21.10/S-21.11/S-21.12 per D-961; S-21.13 (W7 NEW D-964; depends_on S-21.10/S-21.11; draft); S-21.14 (W8 NEW D-972; 8 pts; release-pipeline predicate+gate sweep; draft); S-21.15 (W8 NEW D-972; 5 pts; compute-input-hash search-path + traces_to; draft).
- **Draft (31), Partial (2), Withdrawn (1):** see prior session checkpoints

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle 2026-07-18 |
| develop | 62fbcf1a | PR #774 squash-merged 2026-08-10T17:34:37Z. Pull on next code-worktree resume. |
| factory-artifacts | e541668e | D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST — commit in progress; SHA-patch follow-up REQUIRED to record the actual commit HEAD once pushed. |
| feature/policy15-gate-rust | d2a3176a | F-001 redesign: crates/policy15-attestation-gate/ 16 tests, GateOutcome enum, mutation-verified. Pushed; no PR. |
| feature/S-21.09 | c20cf2fe | S-21.09 **BC-5.39.001 streak 3/3 — RE-CONVERGED (D-988), PRESERVED** by the D-989 windows-x64 portability fix (fixture-construction only, identical runtime semantics on every platform). story spec **v1.32** (advanced from v1.31 — documents the fix + SHA sweep); 51 tests T-006..T-056 all green, 45 S-21.09-owned plus 1 registry.rs unit test, count UNCHANGED; 16 pts. `cargo fmt`/`clippy`/`cargo test --workspace --all-targets` all clean. C-1/C-2/C-4/C-5 open; 4 pass-10 carry-over findings open, anchored to next maintenance sweep. **PUSHED** to origin (direct-orchestrator-push, human-authorized; github-ops delegate non-functional this session). PR #775 HEAD `c20cf2fe`; CI re-running (windows-x64 fix in place). **NEXT: review refresh under the strengthened lens + full CI green → merge authorization (human-gated).** |
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
| v1.0-brownfield-backfill | brownfield | D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST ACTIVE (SHA-patch pending). S-21.09 BC-5.39.001 streak **REMAINS 3/3 RE-CONVERGED (D-988), PRESERVED** — windows-x64 CI caught a real Windows-only T-026(b) TOML-fixture defect (`PathBuf::push` OS-native separator) the all-macOS LOCAL cascade + mutation audit structurally could not observe; fixed `c20cf2fe` (forward-slash string concatenation, fixture-construction only, identical runtime semantics on every platform). story spec **v1.32**; impl **`c20cf2fe`** (advanced from `1c93f499`), PUSHED. develop 62fbcf1a; main 80e5cd7b; merged_count 107; BC v4.56; VP v2.76; STORY **v4.317**; ARCH v3.55; ADR-043 proposed NOT RATIFIED; streak **3/3 RE-CONVERGED, PRESERVED** (24 numbered adversary passes + 1 PR-review retraction event + 1 non-numbered post-re-convergence CI fix); total finding trajectory (numbered passes only) 3→3→2→13→11→9→9→8→8→15→2→1→1→2→1→2→0→0→0→1→1→0→0→0; trajectory-tail →20→16→8→10 UNCHANGED. F-001 redesign RATIFIED (ADR-040 v1.12; policies.yaml v1.4.23 ACTIVE; CI wiring PENDING). **PR #775 HEAD `c20cf2fe`; review refresh + full CI green → merge auth remain human-gated.** | SHA-patch pending this burst; D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST 2026-08-13; D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE 2026-08-13 (SHA-patch done b31de9e2). |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 complete D-510; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range; exhaustive): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range; exhaustive) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-989 (see decision-log.md for full range; exhaustive): this Decisions Log (D-961..D-989 (see decision-log.md for full range; sample) live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-989 | D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST (state-manager; single-commit TD-VSDD-053 2026-08-13; parent-commit e541668e; SHA-patch pending). After the D-988 BC-5.39.001 TRUE 3-CLEAN RE-CONVERGENCE (verified at impl `1c93f499`), cross-platform CI's `build-dispatcher (windows-x64)` job on PR #775 caught a real defect the all-macOS LOCAL adversary cascade and exhaustive mutation-completeness audit structurally could not: `test_S_21_09_ac006_T026_absolute_form_excluded_from_declared` sub-test (b) panicked on Windows because its `registry-depth.toml` fixture path was built via `PathBuf::push` (OS-native separator → `\` on Windows → invalid TOML escape). test-writer fixed it in `c20cf2fe` (forward-slash string concatenation; full ~90-site audit confirmed T-026(b) was the sole non-portable site; 51 green macOS; fmt/clippy clean). **D-988 RE-CONVERGENCE PRESERVED, not reopened** — `c20cf2fe` is a test-FIXTURE-CONSTRUCTION portability change with identical runtime semantics on every platform (no assertion/logic/mutation-control/test-set change); **BC-5.39.001 streak REMAINS 3/3 RE-CONVERGED**, impl advanced `1c93f499`→`c20cf2fe`. story-writer story v1.32 documents the fix + SHA sweep. The 4 fix commits (`c9cccea9`/`fc0e613b`/`1c93f499`/`c20cf2fe`) were pushed to `origin/feature/S-21.09` DIRECTLY BY THE ORCHESTRATOR (human-authorized) because the pr-manager→github-ops push delegate was non-functional this session. PR #775 HEAD now `c20cf2fe`; CI re-running (windows-x64 fix in place). `INDEX.md` S-21.09 LOCAL Adversary Reviews table extended with a POST-RE-CONVERGENCE row; Convergence Status leading declaration updated. Two lessons appended to `lessons.md`. STORY-INDEX **v4.316→v4.317**. **Final gate: PR #775 review refresh + full CI (incl. windows-x64), now running.** STATE.md v7.36→v7.37. | windows-x64 CI caught T-026(b) PathBuf-separator TOML-fixture defect; fixed `c20cf2fe`; D-988 RE-CONVERGENCE PRESERVED, streak REMAINS 3/3; story v1.32; feature/S-21.09 PUSHED; STORY-INDEX v4.317; NEXT: review refresh + full CI → merge auth | D-989-S-21.09-POST-RE-CONVERGENCE-WINDOWS-PORTABILITY-CI-FIX-BURST | 2026-08-13 |
| D-988 | D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE (state-manager; single-commit TD-VSDD-053 2026-08-13; parent-commit e86c8c92; commit b31de9e2; SHA-patch done). LOCAL adversary pass-24 recorded — the FIFTH pass dispatched under the strengthened rubric. **Verdict CLEAN — 0 findings at any severity, no process-gap.** **LOCAL BC-5.39.001 streak ADVANCES 2/3 → 3/3 — TRUE 3-CLEAN RE-CONVERGED** (passes 22+23+24 all CLEAN under the strengthened rubric). Full detail: `git -C .factory show b31de9e2:.factory/STATE.md`. | Pass-24 (fifth pass, strengthened rubric) CLEAN, 0 findings; streak ADVANCES 2/3→3/3 TRUE 3-CLEAN RE-CONVERGED; STORY-INDEX v4.316; post-re-convergence NEXT steps live | D-988-S-21.09-LOCAL-PASS-24-CLEAN-RECORD-TRUE-3-CLEAN-RE-CONVERGENCE | 2026-08-13 |
| D-413..D-989 (see decision-log.md for full range; exhaustive) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-13 |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

| Blocker | Status | Risk Statement |
|---------|--------|----------------|
| **[P0] POLICY 15 ATTESTATION-LOCATION GATE vacuous (F-S2107-P10-001)** | **OPEN — D-966 2026-08-09; REDESIGN RATIFIED D-970 — NOT YET IN FORCE** | Root cause (D-969): category error — gate evaluated in factory-artifacts worktree where *.rs/*.bats count is permanently zero; INAPPLICABLE branch was the only reachable outcome. ADR-040 v1.12 RATIFIED by human 2026-08-10 (D-970); policies.yaml v1.4.23 ACTIVE; Codifications 1+2 APPLIED. **Closes when:** Rust crate merged to develop AND CI job wired. Dependency chain: S-21.09 → S-21.07 → wire CI job. |
| **[P0] `validate-factory-path-staging` WASM guard inert since 2026-07-23** | **OPEN** | 0 fires vs ≥14,000 sibling invocations; `on_error = "continue"` masks absence. Fix story: S-21.09 (`c20cf2fe`; story spec v1.32; 51 tests; **streak 3/3 RE-CONVERGED (D-988), PRESERVED**). MUST land before S-21.07. PR #775 review refresh + CI green pending, then push authorization. |
| **[C-1] CWE-706 incorrect path resolution — exec_subprocess binary_allow load-time prefix check** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | `binary_allow` entries are bare names; prefix list inert. ADR-043 v1.5 NOT RATIFIED. **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.** Route: implementer + security-reviewer after ADR-043 ratification. |
| **[C-2] CWE-362 TOCTOU — resolve-then-check window in exec_subprocess** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | Race between resolution and spawn; threat model boundary not formally specified. Route: security-reviewer before ADR-043 ratification. |
| **[C-4] CWE-284 access control — arbitrary binary execution via misconfigured prefix list** | **OPEN 2026-08-11 — HIGH SECURITY (D-972)** | No validation of prefix list entries at load time. Route: product-owner (BC amendment) + implementer. |
| **[C-5] CWE-284 no per-entry resource limit isolation** | **OPEN 2026-08-11 — MEDIUM SECURITY (D-972)** | No per-`binary_allow` resource limits. Route: product-owner + story-writer. |
| **[pass-10 carry-over] ADV-BB-P10-MED-001 directory-only `hook-plugins/sub/` staging control** | **OPEN — carried through D-989; does NOT block re-convergence** | Gate (a)'s ">= 1 component after hook-plugins/" threshold admits directory-only declarations; spurious MISSING report possible. Anchor: next maintenance sweep / fix-burst prior to `feature/S-21.09` PR merge. |
| **[pass-10 carry-over] ADV-BB-P10-LOW-001/002/003 (NUL/trailing-space names; fail-open arms; `workspace_root()` untested)** | **OPEN — carried through D-989; does NOT block re-convergence** | Low-severity residuals from pass-10; not addressed through pass-24 or the D-989 Windows-portability fix. Anchor: next maintenance sweep / fix-burst prior to `feature/S-21.09` PR merge. |
| **[D-989] PR #775 review refresh + full CI green** | **OPEN 2026-08-13 — HUMAN-GATED NEXT step** | PR #775 HEAD advanced to `c20cf2fe` (windows-x64 portability fix); needs: (1) review refresh under the strengthened lens, (2) full CI green (incl. windows-x64), (3) merge authorization (merge-order S-21.09 before frozen S-21.07). |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67..74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754. |
| **TD-VSDD-061/062/063** | OPEN 2026-05-17/19 | validate-index-cite-refresh large-file fail-open; schema inconsistencies; VP allocation deferred. |
| **TD-VSDD-095..100** | CODIFIED-AND-FORWARDED | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101** | OPEN 2026-05-18 — anchored S-15.15 | VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 skips production bats test in CI; dangling ref confirmed D-972. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | check_index_sites + rows_after_heading advisory-arm defects. |
| **[D-945] VP-102..VP-118 pending allocation** | DEFERRED — anchored S-21.07 post-merge | 17 VPs per BC-5.39.010 §VP Anchors. |
| **[D-952] compute-input-hash operator cache binary divergence** | BOOTSTRAP-MIGRATED — anchored rc.24 #715 | Self-heals at rc.24. |
| **[D-953] 27 unparseable frontmatter files** | OPEN 2026-08-04 | Needs remediation story. |
| **[D-953] ADR-037 19-story volatile-inputs remediation** | OPEN 2026-08-04 | S-19.01 CRITICAL. |
| **[D-954] Duplicate T-038 test ID** | OPEN 2026-08-04 | POLICY 1 violated; anchor pass-11 or next fix burst. |
| **[D-954] decision-log.md >16,000 lines** | OPEN 2026-08-04 | WASM validators time out on every edit (confirmed again this burst — advisory-only, writes land). |
| **[D-955] 18 Dependabot vulnerabilities** | OPEN 2026-08-10 (corrected D-971) | Anchor: next maintenance sweep. |
| **[D-957] F-S2107-P7-019 D-693 stale WASM size** | OPEN 2026-08-05 | Anchor: pass-11 fix burst or next SHA-patch. |
| **[D-958] 60 of 158 stories lack tdd_mode** | OPEN 2026-08-06 | Anchor: S-15.03 PRIORITY-A. |
| **[D-958] GATE DEFECT: validate-pr-review-posted + validate-changelog-monotonicity** | OPEN 2026-08-06 | Paper-gate; header-skip misread. |
| **[D-961] SEC-001 + RUSTSEC-2026-0222/0204 + 18 Dependabot + EAC-002 + ADR-033** | OPEN 2026-08-07 — SECURITY | E-22 scope re-anchored to E-21 W4. Dependabot count corrected D-971. |
| **[D-961] fix/nested-factory-path-derivation + fix/d999-sentinel-code-migration** | RESOLVED 2026-08-09 — both pushed | Both branches pushed; team-lead decides merge path. |
| **[D-963] ADR-035 §Decision 5 quadratic not observed** | OPEN 2026-08-08 | Linear R²=0.998790. Route: architect at next ADR-035 touch. |
| **[D-963] BC-5.39.010 live-operation silent exhaustion gap** | OPEN 2026-08-08 | plugin.timeout exits 0/empty. Anchor: S-21.07 + margin gate implementation. |
| **[D-964] fix/fuel-cap-raise-20m NOT YET EFFECTIVE** | OPEN 2026-08-10 (D-968) — release-gated | On develop (`62fbcf1a`); operator cache rc.23 still embeds 10M; requires rc.24. |
| **[SESSION-WRAP-2026-08-09 / 2026-08-11] Dispatcher log deletion recurrence — 4 occurrences** | OPEN 2026-08-09 — root cause unestablished | `.factory/.factory/logs/` EXISTS with 2 files. Anchor: maintenance sweep. |
| **[D-966] F-002 retroactive-attestation (permanent)** | OPEN 2026-08-09 — permanent historical | 67ffbdcc + 38c70f9e lack "at that commit" attestation. Route: architect. |
| **[D-966] F-005 ADR-041/ADR-042 status-field drift** | OPEN 2026-08-09 | ADR-041 + ADR-042 `status: proposed`. Route: architect. |
| **[D-966] F-006 ADR-042 §Decision 1 vs §Decision 2 self-contradiction** | OPEN 2026-08-09 (D-967 correction) | Re-ratification required. Route: architect. |
| **[D-966] F-008 TD-VSDD-091 line-number pins** | OPEN 2026-08-09 | ADR-040/ADR-042 line-number cites. Route: architect. |
| **[D-966] F-009 BC-5.39.010 modified[]-erratum parity** | OPEN 2026-08-09 | POLICY 14 leg-3 gap. Route: product-owner. |
| **[D-968] F-004 BC-5.39.010 present-perfect SHIFTED** | OPEN 2026-08-10 | Route: product-owner. |
| **[D-969] feature/policy15-gate-rust pending integration** | OPEN 2026-08-10; ratification complete D-970 | Awaits: crate merged to develop via S-21.07; CI job wired. |
| **[D-970] CI-wiring deployment blocker** | OPEN 2026-08-10 | Dependency: S-21.09 → S-21.07 → wire CI. |
| **[D-971] RUSTSEC-2026-0204/0190/0052 unanchored advisories** | OPEN 2026-08-10 — SECURITY | cargo-deny fails with 5 findings total. Anchor: E-22 security scope. |
| **[D-971] RUSTSEC-2026-0188 exploitability framing** | OPEN 2026-08-10 — SECURITY | Route: security-reviewer. Anchor: E-22. |
| **[D-971] refuse_setuid gate inert — HIGH SECURITY** | OPEN 2026-08-10 | Route: security-reviewer + implementer. Anchor: E-22 or dedicated story. |
| **[D-972] 6 vacuous gate drift items** | OPEN 2026-08-11 | All linked to C-1..C-5 or ADR-043. Anchor: ADR-043 ratification + S-21.14. |
| **[D-983] Adversary rubric gap — vacuity/tautology/mutation-narrative-accuracy** | CODIFIED — anchored S-15.03 PRIORITY-A 2026-08-13 | Folded into the strengthened rubric; validated by pass-20/21 real findings, then independently re-confirmed sound by passes 22/23/24 (RE-CONVERGENCE). |
| **[D-988] Fold vacuity/tautology lens into rubric from cascade start** | CODIFIED — `L-BB-fold-vacuity-tautology-lens-into-rubric-from-cascade-start` anchored S-15.03 PRIORITY-A 2026-08-13 | Net lesson from the retraction+re-convergence cycle: apply the strengthened rubric from pass-1 of every future LOCAL cascade. |
| **[D-989] Cross-platform CI is a convergence prerequisite, not just a merge prerequisite** | CODIFIED — `L-BB-cross-platform-ci-is-convergence-prerequisite-not-just-merge-prerequisite` anchored S-15.03 PRIORITY-A 2026-08-13 | An all-macOS LOCAL cascade + mutation audit cannot catch platform-specific (OS path-separator) defects; fold a Windows-portability fixture check into test-writer discipline; treat green cross-platform CI as a convergence input. |
| **[D-989] github-ops push delegate non-functional mid-session** | OPEN — `L-BB-github-ops-push-delegate-non-functional-mid-session` anchored S-15.03 PRIORITY-A 2026-08-13 | pr-manager→github-ops push delegate failed after the first push this session; orchestrator pushed directly under human authorization. Investigate root cause. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`

## Session Resume Checkpoint (2026-08-13 — HEAD `e541668e` (SHA-patch pending); PIPELINE ACTIVE; S-21.09 BC-5.39.001 streak 3/3 RE-CONVERGED (D-988), PRESERVED; story spec v1.32; impl `c20cf2fe` 51 tests T-006..T-056; PUSHED)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE ACTIVE.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. LOCAL adversary pass count **24 numbered passes** plus the earlier PR-review retraction event and this D-989 non-numbered post-re-convergence CI fix (not a numbered pass). Streak **3/3 — TRUE 3-CLEAN RE-CONVERGED (D-988), PRESERVED** by the D-989 Windows-portability fix. Total finding-count trajectory (numbered passes only) `3→3→2→13→11→9→9→8→8→15→2→1→1→2→1→2→0→0→0→1→1→0→0→0`, tail `→1→0→0→0` UNCHANGED by D-989. Cycle-level trajectory-tail (S-21.07) `→20→16→8→10` UNCHANGED. 4-INDEX: BC v4.56 / VP v2.76 / **STORY v4.317** / ARCH v3.55. `policies.yaml` **v1.4.23**. factory-artifacts HEAD `e541668e` (D-988 SHA-patch commit — this D-989 burst's own commit SHA is not yet known; SHA-patch follow-up REQUIRED after push).

**Last decisions: D-988, D-989.** D-988 declared BC-5.39.001 TRUE 3-CLEAN RE-CONVERGENCE (streak 3/3) at impl `1c93f499`. **D-989 records a POST-RE-CONVERGENCE fix, not a new adversary pass:** cross-platform CI's `build-dispatcher (windows-x64)` job on PR #775 caught a real Windows-only defect the all-macOS LOCAL cascade + exhaustive mutation-completeness audit structurally could not — `test_S_21_09_ac006_T026_absolute_form_excluded_from_declared` sub-test (b) panicked because its TOML fixture path was built via `PathBuf::push` (OS-native separator, invalid TOML escape on Windows). test-writer fixed it in `c20cf2fe` (forward-slash string concatenation; ~90-site audit confirmed T-026(b) was the sole non-portable site). **This is a test-fixture-construction change with identical runtime semantics on every platform — the D-988 re-convergence is PRESERVED, not reopened; the streak REMAINS 3/3.** story-writer story v1.32 documents the fix. The 4 fix commits were pushed to `origin/feature/S-21.09` directly by the orchestrator (human-authorized) because the pr-manager→github-ops push delegate was non-functional this session. PR #775 HEAD now `c20cf2fe`; CI re-running.

### §2 S-21.09 (Session's Main Work)

Streak **3/3 RE-CONVERGED (D-988), PRESERVED**. Branch `feature/S-21.09` at **`c20cf2fe`** (advanced from `1c93f499` — Windows-portability fixture fix only, no assertion/logic change), **51 tests T-006..T-056** all green on macOS (45 S-21.09-owned plus 1 `registry.rs` unit test, count UNCHANGED), `cargo fmt`/`clippy`/`cargo test --workspace --all-targets` all clean. Story spec **v1.32** (advanced from v1.31). Points **16 UNCHANGED**.

> **THE BRANCH IS NOW PUSHED.** PR #775 HEAD is `c20cf2fe`; description off-by-one already corrected (52/52). Remaining, in order:
>
> 1. PR #775 review refresh (pr-reviewer + code-reviewer) against `c20cf2fe`, under the strengthened lens.
> 2. Full CI green, including the `build-dispatcher (windows-x64)` job (currently running with the portability fix in place).
> 3. Merge authorization (merge-order: S-21.09 lands before frozen `feature/S-21.07`).
>
> **All three steps are human-gated** — the D-988 RE-CONVERGENCE plus the D-989 fix together do not authorize any of them.

### §3 S-21.09 LOCAL 3-CLEAN Cascade — RE-CONVERGED at 3/3, PRESERVED through D-989

**24 numbered adversary passes — 16 NOT CLEAN (pass-1..16), 3 CLEAN (pass-17/18/19, later RETRACTED at D-983), 2 NOT-CLEAN (pass-20/21, strengthened rubric), 3 CLEAN (pass-22/23/24, strengthened rubric). Streak 3/3 — TRUE 3-CLEAN RE-CONVERGED (D-988).** The D-989 windows-x64 CI fix (`c20cf2fe`) is explicitly NOT a 25th numbered pass — it is a downstream cross-platform CI catch of a defect class (OS path-separator portability) that no LOCAL macOS-only adversary pass, however many, could ever observe. The fix is test-fixture-construction-only with identical runtime semantics on every platform, so it does not consume, reset, or re-earn the streak — **the streak simply carries forward at 3/3, now against `c20cf2fe` instead of `1c93f499`.**

**S-7.02 Cycle-Closing Checklist — SATISFIED at D-988, UNCHANGED by D-989** (D-989 adds two NEW lessons, both immediately anchored — see §9).

**Open — 4 pass-10 carry-overs (UNCHANGED by this burst):**
- ADV-BB-P10-MED-001: directory-only control (`hook-plugins/sub/` admitted)
- ADV-BB-P10-LOW-001: NUL/trailing-space names admitted verbatim
- ADV-BB-P10-LOW-002: fail-open arms guarded only by unasserted call ordering
- ADV-BB-P10-LOW-003: `workspace_root()` untested directly

Anchor: next maintenance sweep / fix-burst prior to `feature/S-21.09` PR merge.

**The LOCAL cascade remains CONVERGED — no further numbered pass is required.** Next substantive action is the human-gated PR #775 review-refresh + full-CI-green + merge-authorization sequence (see §2/§8/§10), not another adversary pass.

### §4 ADR-043

v1.5, `status: proposed`, **NOT RATIFIED**. Three fresh-context DO-NOT-RATIFY reviews (4, then 10, then 9 blockers) then amended. POLICY 22 requires human ratification. Reviews persisted as `adv-adr-043-pass-{1,2,3}.md`. UNCHANGED by this burst.

### §5 Five SS-01 BCs Amended

BC-1.05.002 v2.4, .004 v2.3, .028 v2.4, .035 v2.5, .036 v1.3 under D-972 Option C adjudication. UNCHANGED by this burst.

### §6 Blocking Issues

- **C-1 CWE-706** — `binary_allow` basename allow-list escape (structural HIGH / practical LOW). **Standing guardrail: no story introducing a plugin that derives `cmd` from runtime data may merge before C-1 is fixed.**
- **C-2 CWE-362** — TOCTOU window; ADR-043 threat model boundary unformalized.
- **C-4 CWE-284** — prefix list empty/writable fallthrough; BC amendment pending.
- **C-5 CWE-284** — no per-entry resource limits; anchor S-21.14.
- **`validate-factory-path-staging` inert** — 0 fires vs ≥14,000 sibling invocations; inert at operator runtime until **rc.24**. Fix is S-21.09, **streak 3/3 RE-CONVERGED (D-988), PRESERVED** — PR #775 review refresh + CI green is now the closing-path step, pending human authorization.
- **4 pass-10 carry-over findings** (MED-001, LOW-001/002/003) — anchor: next maintenance sweep/fix-burst prior to PR merge; NOT a convergence blocker.
- **PR #775 review refresh + full CI green** — anchor: immediate NEXT step, human-gated.

### §7 Infrastructure Blockers

(a) **STATE.md narrative sections** — full-file Write convention continues (verify-state-timestamp-refresh guard requires a `timestamp:` advance within EVERY individual Edit/Write call's own diff). Confirmed again this burst.
(b) **`STORY-INDEX.md`** triggers advisory PostToolUse fuel timeouts (confirmed again this burst — writes still land). The `last_amended` frontmatter field's `[Prior: ...]` chain continues to accumulate. Full remediation flagged for a future S-15.03 PRIORITY-A pass.
(c) **`decision-log.md`/`burst-log.md`/`lessons.md` exhaust WASM validator fuel on every edit** — confirmed again this burst (advisory only; writes land, `block_intent=true` PostToolUse signals do not revert already-landed writes). Compaction gates rc.24 fuel-cap effectiveness + S-15.03 PRIORITY-A automation.
(d) **`mutation-audit-s21.09.md` / `pr775-review-findings.md` are new artifact types** — persisted alongside `adv-s21.09-local-pass-N.md` files but using their own naming convention. UNCHANGED by this burst.
(e) **`validate-trajectory-tail-cell-completeness` (D-453(d))**: requires a 4-value trajectory-tail arrow-sequence present in BOTH the frontmatter `current_step` AND the Project Metadata `Last Updated` cell on every STATE.md write. Applied again this burst.
(f) **Convergence-preservation precedent (D-989, mirror-image of the D-983 retraction and the D-988 re-convergence)** — a fixture-construction fix with identical runtime semantics on every platform does NOT reset a streak earned on a materially different axis (assertion quality); established this burst as a new precedent class.
(g) **SHA-patch follow-up — REQUIRED.** Active Branches `factory-artifacts` row and this checkpoint's header cite `e541668e` (the D-988 SHA-patch commit, the parent of this burst) because this burst's own commit SHA is not yet known at write time. Must be corrected to the actual commit HEAD in an immediate follow-up write after this burst's commit is pushed, per D-447(c)+D-449(e).

### §8 Pending Human Decisions

1. **PR #775 review refresh** — dispatch pr-manager to re-run the PR #775 review against `c20cf2fe` under the strengthened lens.
2. **Full CI green** — including the `build-dispatcher (windows-x64)` job, currently running with the portability fix in place.
3. **Merge authorization** — after review refresh + CI green; merge-order S-21.09 before frozen `feature/S-21.07`.
4. **ADR-043 ratification** — v1.5 converged (no BLOCKERs per pass-3); human to decide: ratify v1.5 / request pass-4 / redirect design. UNCHANGED.
5. **S-21.12 blocker B1** — `cargo deny` has 5 advisories (not 2); `deny.toml` read-only; `async-std` no upgrade path; AC-004 unsatisfiable. UNCHANGED.
6. **Four orphan advisories + 18 Dependabot alerts** — scope assignment pending (E-22 or dedicated fix). UNCHANGED.
7. **github-ops push-delegate reliability** — investigate root cause of mid-session push failures (S-15.03 PRIORITY-A or dedicated devops follow-up). NEW this burst.

### §9 Two Follow-up Stories Registered This Session

- **S-21.14** (W8, 8 pts): release-pipeline weak-predicate sweep across 5 sites + resolver-arm floor + T-017 first-match extractor + artifact-freshness gate.
- **S-21.15** (W8, 5 pts): `compute-input-hash` search-path gap + `traces_to:` bare-filename question.

### §10 Resume Command

`/vsdd-factory:next-step` — LOCAL adversary streak is **3/3 — TRUE 3-CLEAN RE-CONVERGED (D-988), PRESERVED through D-989**; the LOCAL cascade is CONVERGED and no further numbered adversary pass is required. The immediate next substantive action is the **human-gated PR #775 sequence**: (1) review refresh under the strengthened lens against `c20cf2fe`; (2) full CI green including windows-x64 (currently running); (3) merge authorization (merge-order S-21.09 before frozen S-21.07). **A SHA-patch follow-up is REQUIRED before any further factory-artifacts write** — Active Branches `factory-artifacts` and this checkpoint's header currently cite the parent commit `e541668e`, not this burst's own (not-yet-known) commit HEAD; correct it via `git -C .factory log -1 --format='%H'` immediately after this burst's commit is pushed. Separately, ADR-043 ratification and S-21.12 cargo-deny blocker decisions remain open per §8, unaffected by this burst.
