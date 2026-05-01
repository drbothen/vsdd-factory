---
document_type: story-index
level: ops
version: "1.7"
status: current
producer: state-manager
timestamp: 2026-04-30T23:00:00
phase: 1.8
inputs:
  - .factory/stories/v1.0/EPIC.md
  - .factory/phase-0-ingestion/pass-6-synthesis.md
  - .factory/specs/architecture/ARCH-INDEX.md
traces_to: .factory/specs/domain-spec/capabilities.md
---

# Story Index — vsdd-factory v1.0 (brownfield)

> Auto-generated during Phase 1.8 migration from legacy S-N.M format to canonical
> S-N.MM format. Updated in Phase 2 (story decomposition) with E-7 stories.
> Updated in Wave 11 burst (2026-04-27): S-4.09 and S-4.10 added.
> Updated in Wave 14 burst (2026-04-29): S-5.05 v1.3→v1.4 + S-5.06 v1.3→v1.4 (reality-vs-spec drift fixes). Pass-1 fix burst (2026-04-29): S-5.05 v1.4→v1.5 + S-5.06 v1.4→v1.5 (20 findings closed per pass-1 review tally [10 S-5.05 + 10 S-5.06]; D-144). Pass-2 fix burst (2026-04-29): both v1.5→v1.6 (20 findings closed [12 S-5.05 + 8 S-5.06]; D-145). Pass-3 (2026-04-29): S-5.05 v1.6→v1.7 (1 MED + 5 LOW closed; STORY-INDEX:136 cross-cutting BC-8.31.* count fixed); S-5.06 NITPICK_ONLY (no version bump; clock 1_of_3 per S-7.03 skip-fix strategy); D-146. Pass-4 (2026-04-29): S-5.05 v1.7 NITPICK_ONLY (8 LOW; clock 0_of_3→1_of_3); S-5.06 v1.6 NITPICK_ONLY (0 findings; clock 1_of_3→2_of_3); skip-fix discipline applied per S-7.03; D-147. Pass-5 (2026-04-29): S-5.05 v1.7 NITPICK_ONLY (5 LOW positive; clock 1_of_3→2_of_3); **S-5.06 v1.6→v1.7 CONVERGENCE_REACHED at pass-5** (0 findings; clock 2_of_3→3_of_3 per ADR-013; status draft→ready); D-148.
> Pass-6 (2026-04-29): **S-5.05 v1.7→v1.8 CONVERGENCE_REACHED at pass-6** (0 findings; clock 2_of_3→3_of_3 per ADR-013; status partial→ready; 3 consecutive NITPICK_ONLY: passes 4, 5, 6); D-149.
>
> **Wave 14 spec convergence COMPLETE (both stories ready):** S-5.06 v1.7 ready (5-pass convergence); S-5.05 v1.8 ready (6-pass convergence). Total Wave 14 effort: 11 adversarial passes vs Wave 13's 51 — Wave 13 lesson application up-front compressed convergence ~4×.
> **Wave 14 COMPLETE (2026-04-29):** S-5.05 PR #40 merged at 1e2db47 (5pts; migration guide v0.79.x→v1.0); S-5.06 PR #39 merged at d134648 (2pts; semver commitment doc); 45 of 47 stories merged; D-150.
> **rc.1 v1.0.0-rc.1 SHIPPED 2026-04-30T03:10:59 UTC.** GH pre-release at https://github.com/drbothen/vsdd-factory/releases/tag/v1.0.0-rc.1. Tag at 1485d2e (bot bundle); main HEAD synced. Branch protection toggled per release ritual; remediation tracked as TD-013. 14-day shakedown clock starts. D-152.
> **W-15 entry-point story S-8.00 authored (2026-04-30):** S-8.00-perf-baseline-bc-anchor-verification.md v1.0 status=draft; 444 lines; 9 ACs; 5pts; blocks S-8.01..S-8.09; resolves OQ-1 + OQ-8. E-8 epic section added. STORY-INDEX v1.0 → v1.1 (D-164).
> This index is the authoritative source for story count and status.
> 48 stories across 9 epics (E-0 through E-8).

> **Filename convention:** Stories live at `.factory/stories/S-N.MM-<short-description>.md`. Example: S-1.05 lives at `S-1.05-wasmtime-integration.md`.

> **Cycle field semantics:** Each story's `cycle:` frontmatter records the cycle that ORIGINALLY CREATED that story. For the 22 merged stories migrated in Phase 1.8 (S-N.M -> S-N.MM), `cycle: v1.0.0-greenfield` is preserved as immutable history. The current cycle (`v1.0-brownfield-backfill`) is for backfill of formal specs around already-shipped stories, not for new story creation.

## Status Summary

| Status | Count |
|--------|-------|
| merged | 45 |
| partial | 1 |
| draft | 1 |
| ready | 1 |
| **Total** | **48** |

## Epic E-0 — Infrastructure Prep (Tier A — all merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | bump-version.sh prerelease support | E-0 | 2 | P0 | -- | merged |
| S-0.02 | Release workflow prerelease handling | E-0 | 2 | P0 | S-0.01 | merged |
| S-0.03 | Activation skill platform detection | E-0 | 3 | P0 | -- | merged |
| S-0.04 | hooks.json.template + CI generation | E-0 | 3 | P0 | -- | merged |
| S-0.05 | Documentation scaffolding | E-0 | 2 | P1 | -- | merged |

## Epic E-1 — Dispatcher Foundation (Tier B.0 + B.x — all merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status | BCs |
|----------|-------|------|--------|----------|------------|--------|-----|
| S-1.01 | Cargo workspace + CI scaffolding | E-1 | 5 | P0 | -- | merged | 0 (pure scaffolding, justified) |
| S-1.02 | factory-dispatcher core (stdin, TOML load, routing) | E-1 | 8 | P0 | S-1.01 | merged | 26 |
| S-1.03 | hook-sdk crate (macro, types, bindings) | E-1 | 8 | P0 | S-1.01 | merged | -- |
| S-1.04 | Host function surface implementation | E-1 | 8 | P0 | S-1.01, S-1.02, S-1.03 | merged | 26 |
| S-1.05 | wasmtime integration + epoch/fuel enforcement | E-1 | 8 | P0 | S-1.01, S-1.02, S-1.04 | merged | 15 |
| S-1.06 | tokio + parallel-within-tier execution | E-1 | 5 | P0 | S-1.01, S-1.02, S-1.04, S-1.05 | merged | 8 |
| S-1.07 | dispatcher-internal.jsonl writer | E-1 | 3 | P0 | S-1.01, S-1.02 | merged | 10 |
| S-1.08 | sink-file driver | E-1 | 5 | P0 | S-1.01, S-1.07 | merged | 23 |
| S-1.09 | sink-otel-grpc driver | E-1 | 5 | P0 | S-1.01, S-1.08 | merged | 15 |

## Epic E-2 — Legacy Adapter and Beta Release (Tier C + D — mostly merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-2.01 | legacy-bash-adapter WASM plugin | E-2 | 5 | P0 | S-1.03, S-1.04, S-1.05, S-1.06 | merged |
| S-2.02 | hooks-registry.toml auto-generation | E-2 | 2 | P0 | S-2.01 | merged |
| S-2.03 | Cross-platform CI matrix build targets | E-2 | 5 | P0 | S-1.01, S-1.02, S-0.04 | merged |
| S-2.04 | Binary commit automation in Release workflow | E-2 | 5 | P0 | S-2.03 | merged |
| S-2.05 | hook-sdk publish to crates.io (0.1.0) | E-2 | 2 | P1 | S-1.03 | partial |
| S-2.06 | Activation skill integrates with real hooks.json variants | E-2 | 3 | P0 | S-0.03, S-0.04, S-2.04 | merged |
| S-2.07 | Regression test suite validation | E-2 | 5 | P0 | S-1.02, S-1.04, S-1.05, S-1.06, S-1.07, S-1.08, S-1.09, S-2.01, S-2.02 | merged |
| S-2.08 | 1.0.0-beta.1 release gate | E-2 | 3 | P0 | all S-0.x + S-1.x + S-2.01..S-2.07 | merged |

## Epic E-3 — WASM Port: High-Value Hooks (Tier E — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status | BCs |
|----------|-------|------|--------|----------|------------|--------|-----|
| S-3.04 | emit_event as host function refactor | E-3 | 3 | P0 | S-1.04 | merged | 8 |
| S-3.01 | Port capture-commit-activity to WASM | E-3 | 5 | P1 | S-2.08, S-3.04 | merged | -- |
| S-3.02 | Port capture-pr-activity to WASM | E-3 | 5 | P1 | S-2.08, S-3.04 | merged | -- |
| S-3.03 | Port block-ai-attribution to WASM | E-3 | 3 | P1 | S-2.08, S-3.04 | merged | -- |

## Epic E-4 — Observability Sinks and RC Release (Tier E + F — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status | BCs |
|----------|-------|------|--------|----------|------------|--------|-----|
| S-4.01 | sink-http driver | E-4 | 5 | P1 | S-1.08 | merged | 4 |
| S-4.02 | sink-datadog driver | E-4 | 5 | P1 | S-1.08, S-4.01 | merged | 2 (+ 2 deferred LOW findings F-1/F-2 from PR #18; v1.3) |
| S-4.03 | sink-honeycomb driver | E-4 | 3 | P1 | S-1.08, S-4.01 | merged | 2 |
| S-4.04 | Per-sink retry + circuit breaker | E-4 | 8 | P1 | S-1.08, S-4.01 | merged | 1 (v1.1 BC creation dep note) |
| S-4.05 | Dead letter queue implementation | E-4 | 3 | P1 | S-4.04 | merged | 2 (+ v1.1 candidates; CONVERGENCE_REACHED pass-48; v1.45; commit ac22a3d; PR #29 merged a84a5f5 on develop 2026-04-28) |
| S-4.06 | Per-sink routing filters + tag enrichment | E-4 | 3 | P1 | S-1.08 | merged | 6 (BC-3.04.003, BC-3.04.004, BC-3.06.007 added; 5 lifecycle updated; PR #30 merged 6ef564c on develop 2026-04-28) |
| S-4.07 | End-to-end observability integration tests | E-4 | 13 | P1 | S-3.01..S-3.04, S-4.01..S-4.06, S-4.10 | merged | 16 (PR #31 merged 1d4edb7 on develop 2026-04-28; spec v1.11 4c0050c; 8 adversarial passes; 40/40 tests in 5.09s) |
| S-4.08 | 1.0.0-rc.1 release gate | E-4 | 5 | P0 | S-0.01, S-0.02, S-3.01..S-3.04, S-4.01..S-4.07, S-4.09, S-4.10, S-5.05 + 2-week shakedown | merged | 5 (PR #32 merged d7eae89 on develop 2026-04-28; spec v1.16 62f7297; 17-pass spec convergence; 6 testable-now ACs RED→GREEN; 11 deferred-to-shakedown; D-133) |
| S-4.09 | sink-http retry backoff with jitter | E-4 | 3 | P1 | S-4.01 | merged | 1 |
| S-4.10 | internal.sink_error event emission (cross-sink) | E-4 | 5 | P1 | S-4.01 | merged | 1 |

## Epic E-5 — New Hook Events and 1.0.0 Release (Tier G + H — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status | Version |
|----------|-------|------|--------|----------|------------|--------|---------|
| S-5.01 | SessionStart hook wiring | E-5 | 3 | P1 | S-4.08 | merged | 2.12 (PR #35 merged 0257f03 2026-04-28; D-136) |
| S-5.02 | SessionEnd hook wiring | E-5 | 3 | P1 | S-4.08 | merged | 2.8 (PR #36 merged edef7da 2026-04-28; D-138) |
| S-5.03 | WorktreeCreate / WorktreeRemove hook wiring | E-5 | 5 | P1 | S-4.08 | merged | 2.5 (PR #37 merged 93b298f 2026-04-29; D-140) |
| S-5.04 | PostToolUseFailure hook wiring | E-5 | 3 | P1 | S-4.08 | merged | 2.6 (PR #38 merged e90faab 2026-04-29; D-142; DRIFT-006 FULLY CLOSED; Wave 13 COMPLETE) |
| S-5.05 | Migration guide (0.79.x → 1.0) | E-5 | 5 | P1 | — | merged | v1.8 (PR #40 1e2db47 2026-04-29) |
| S-5.06 | Semver commitment documentation | E-5 | 2 | P1 | S-4.08 | merged | v1.7 (PR #39 d134648 2026-04-29) |
| S-5.07 | 1.0.0 release gate | E-5 | 3 | P0 | S-0.01, S-0.02, S-5.01..S-5.06 + 1-week shakedown | draft | -- |

## Epic E-6 — VSDD Self-Improvement / Tooling Backlog (open)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-6.01 | Add create-adr skill for ADR authoring | E-6 | 3 | P1 | -- | merged |

## Epic E-7 — Process Codification (open)

| Story ID | Title | Epic | Points | Priority | Depends On | Status | Version |
|----------|-------|------|--------|----------|------------|--------|---------|
| S-7.01 | Agent prompt updates for spec/anchor/adversary discipline | E-7 | 5 | P1 | -- | merged | -- |
| S-7.02 | State-manager defensive sweep + count-propagation hook + meta-rule | E-7 | 8 | P1 | -- | merged | -- |
| S-7.03 | TDD Discipline Hardening — Stub-as-Implementation Anti-Pattern Prevention | E-7 | 8 | P1 | -- | merged | 2.2 |

> **S-7.03 delivery:** PR #13 merged to develop at 4db2340 on 2026-04-26. 18/18 bats tests GREEN. Worktree feat/tdd-discipline-hardening (9b1624b → 121d24c, 9 commits). Spec convergence: 17 adversarial passes.

> **S-4.05 spec CONVERGENCE_REACHED at pass-48 (2026-04-28).** v1.45, commit ac22a3d on factory-artifacts. 48 adversarial passes — longest run in project history (eclipses S-7.03's 17-pass record). Trajectory: 11→5→8→8→8→3→0→3→5→1→2→1→2→0→2→2→0→1→4→2→2→2→2→1→1HIGH→4→5→6→2→7→6→8→8→6→5→4→5→4→3→7→7→7→8→5→5→3→3LOW→6LOW→0. 6 carry-forward LOWs (F-4601..F-4603, F-4701..F-4703) non-blocking per ADR-013. Status: draft → ready. (D-129)

> **Wave 1 SS-01 re-anchor CONVERGED 3-of-3 at pass-6 (2026-04-26).** 7 stories anchored to SS-01 BCs: S-1.01 (0/justified), S-1.02 (26), S-1.04 (26), S-1.05 (15), S-1.06 (8), S-1.07 (10), S-3.04 (8). 93 unique SS-01 BCs anchored (of 99); 4 deferred to Wave 3 (BC-1.07.003-006); 10 v1.1 BC candidates logged. Trajectory: 10→4→3→1→0→0.

> **Wave 2 SS-03 sinks re-anchor CONVERGED 3-of-3 at pass-13 (2026-04-27).** 9 stories anchored to SS-03 BCs: S-1.08 (23), S-1.09 (15), S-4.01 (4), S-4.02 (2), S-4.03 (2), S-4.04 (1 + v1.1 BC creation dep note), S-4.05 (2 + v1.1 candidates), S-4.06 (6), S-4.07 (16, +BC-3.07.002 added 2026-04-27; BC-3.01.001+BC-3.03.002 removed pass-3 2026-04-28). ~37 unique SS-03 BCs anchored; PRD FR-044 added (per-sink resilience); 32 v1.1 BC candidates logged. Trajectory: 11→1→3→0→1→0→1→2→0→1→0→0→0 (13 passes; 4 reset events).

> **Wave 7 SS-10 re-anchor** (2026-04-27): 3 stories anchored — S-0.02, S-4.08, S-5.07 — to existing SS-09 BCs (BC-9.01.001, BC-9.01.003) per Wave 3 F-007 / Wave 5 F-002 / Wave 6 F-005 sanctioned-template-anchor pattern (BC-subsystem SS-09 ≠ story.subsystems[] SS-10; SS-10 is target_module surface). 11 v1.1 BC candidates registered (BC-10.13.001-011). S-0.02 blocks {S-2.08, S-4.08, S-5.07}; S-4.08 + S-5.07 gained S-0.02 dep.

> **Wave 8 SS-08 re-anchor** (2026-04-27): 3 docs-stories anchored — S-0.05, S-5.05, S-5.06 — to existing SS-08 methodology BCs (BC-8.22.001 Conventional Commits, BC-8.26.001 story-completeness 14-check audit, BC-8.26.006 user-facing-docs deliverable) per Wave 7 F-204 cross-wave-complementary methodology-anchor pattern (BC-subsystem SS-08 = story.subsystems[] SS-08; BCs are methodology contracts not directly exercised by ACs). 6 v1.1 BC candidates registered (BC-8.31.003-008; BC-8.31.001-002 gap-numbered) for docs-content-specific contracts. S-0.05 deliberately excludes BC-8.26.006 (skeleton-only stories are not complete deliverables).

> **Wave 9 SS-01 straggler re-anchor** (2026-04-27): 1 story anchored — S-2.07 (regression-test-validation) — to existing SS-01 BCs (BC-1.07.001/002, BC-1.08.001/002) + VP-043 + CAP-002 with Stretch-Anchor Disclosure for SS-04+SS-07 cross-subsystem regression coverage. CONVERGED at pass-4 (3_of_3 NITPICK_ONLY); 4 passes total (smallest baseline + fastest convergence of 9 waves). TD #105 closed (S-2.07 depends_on includes S-1.09). **41 of 41 cumulative stories re-anchored** — v1.0-brownfield-backfill re-anchor phase COMPLETE.

> **S-4.05 DLQ delivered: PR #29 merged to develop at a84a5f5 (2026-04-28).** 18/18 tests GREEN; 0 regressions; 12/12 ACs verified. Spec v1.45, 48 adversarial passes (longest in project history). STORY-INDEX merged 35→36; ready 4→3. Wave 12 remaining: S-4.06 (3pts), S-4.07 (13pts critical-path), S-4.08 (5pts) — 23/28 pts remaining.

> **S-4.06 routing filters + tag enrichment delivered: PR #30 merged to develop at 6ef564c (2026-04-28).** 9/9 ACs verified; 264 workspace tests pass; 0 regressions. Spec v1.10, 10 adversarial passes (CONVERGED d7b29dc). 2 code-review cycles (cycle 1 fixed sink-http+honeycomb missing routing_filter/tags; cycle 2 APPROVE). 3 new BCs (BC-3.04.003, BC-3.04.004, BC-3.06.007); 5 BCs lifecycle updated. STORY-INDEX merged 36→37; ready 3→2. Wave 12 remaining: S-4.07 (13pts critical-path), S-4.08 (5pts) — 18/28 pts remaining.

> **S-4.07 E2E observability integration tests delivered: PR #31 merged to develop at 1d4edb7 (2026-04-28).** 16/16 ACs verified; 40/40 integration tests pass in 5.09s; 0 regressions. Spec v1.11 at 4c0050c, 8 adversarial passes (CONVERGED). 1 code-review cycle (immediate APPROVE; 0 blocking). rc.1 critical-path unblocked. STORY-INDEX merged 37→38; ready 2→1. Wave 12 remaining: S-4.08 (5pts rc.1 gate) — 23/28 pts done.

> **S-4.08 rc.1 release gate delivered: PR #32 merged to develop at d7eae89 (2026-04-28).** 6 testable-now ACs RED→GREEN; 17 regression-guards GREEN; 11 deferred-to-shakedown. Spec v1.16 at 62f7297, 17 adversarial passes (CONVERGED). 1 code-review cycle (immediate APPROVE; 0 blocking). STORY-INDEX merged 38→39; ready 1→0. **Wave 12 COMPLETE: 28/28 pts shipped.** (D-133)

> **Wave 12 CLOSED (2026-04-28) — 28/28 pts, 4 stories, 4 PRs, 83 total adversarial spec passes:** S-4.05 DLQ (PR #29 a84a5f5; 5pts; 48-pass convergence — longest in project history); S-4.06 routing+enrich (PR #30 6ef564c; 3pts+5pts; 10-pass convergence); S-4.07 E2E obs tests (PR #31 1d4edb7; 13pts; 8-pass convergence; rc.1 critical-path); S-4.08 rc.1 release gate (PR #32 d7eae89; 5pts; 17-pass convergence). ~165+ findings closed across ~50 fix bursts. 4 PRs merged in single session — zero rollbacks. 11 S-4.08 ACs deferred to shakedown (RC Engineer executes at rc.1 cut time).

> **Wave 11 SS-03 fully closed at develop@ccf34e6 (2026-04-27).** PRs merged this session: #18 (S-4.01), #20 (S-3.01), #21 (S-3.02), S-3.03 ports (4229648), #22 (Semgrep SAST), #23 (S-4.04 retry+CB), #24 (S-4.02 datadog), #25 (S-4.03 honeycomb), #26 (docs), #27 (S-4.09 backoff), #28 (S-4.10 cross-sink emission). 9 stories shipped + 1 docs + 1 SAST = 11 PRs. STORY-INDEX merged 33 → 35. Wave 12 also fully closed (S-4.02/03/04). S-4.07 (E2E integration) now waits only on S-4.05 + S-4.06 spec convergence + impl. Worktrees cleaned up: /private/tmp/vsdd-S-3.01, S-3.02, S-3.03 (Wave 11 first batch); /private/tmp/vsdd-S-4.02, S-4.03, S-4.04 (Wave 12); /private/tmp/vsdd-S-4.09, S-4.10 (Wave 11 close). Local repo now shows only develop + .factory/ + harness-managed agent worktrees.

## Epic E-8 — Native WASM Migration Completion (W-15 pre-work — draft)

> **E-8 spec CONVERGENCE_REACHED (2026-04-30):** E-8-native-wasm-migration.md v1.7 status=ready; 11 adversarial passes; trajectory 18→7→0→1→0→2→3→1→0→1→0; D-163 sealed. 29 stories planned (S-8.00..S-8.28). story_count: 29 (1 pre-work + 9 Tier 1 + 11 Tier 2 + 8 Tier 3). target_release: v1.1 (Tier 1), v1.2 (Tier 2), v1.3 (Tier 3). Anchors: CAP-002, CAP-008, CAP-013, CAP-022. Tech debt: TD-014.

| Story ID | Title | Epic | Points | Priority | Depends On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|------------|--------|--------|-----|
| S-8.00 | Perf benchmark baseline + Tier 1 BC-anchor verification (W-15 pre-work) | E-8 | 5 | P2 | -- | S-8.01..S-8.09 | ready | [] ([process-gap] under D-2 Option C; v1.1 candidates: BC-7.00.001, BC-7.00.002) |

> S-8.00 v1.5 (status=**ready**, **CONVERGENCE_REACHED** at adversarial pass-6 per ADR-013 2026-04-30). 512 lines; 9 ACs; 5pts; depends_on=[]; blocks S-8.01..S-8.09. Two-responsibility scope: (A) perf benchmark baseline resolving OQ-8 (~10ms/plugin warm-invocation); (B) BC-anchor verification table for 9 Tier 1 hooks (handoff-validator, pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge, validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start, regression-gate) per D-2 Option C. behavioral_contracts=[] intentional ([process-gap] disclosure). subsystems=[SS-01, SS-07]. Adversarial pass-1 closed (14 v1.1); pass-2 closed (8 v1.2); pass-3 closed (6 v1.3); pass-4 NITPICK_ONLY (3 closed v1.4 + clock 0/3→1/3); pass-5 NITPICK_ONLY (1 NIT SKIP_FIX + clock 1/3→2/3); **pass-6 NITPICK_ONLY (2 NIT SKIP_FIX + clock 2/3→3/3 = CONVERGENCE_REACHED)**. Trajectory 14→8→6→3→1→2 over 6 passes (86% decay; healthy late-convergence shape). D-164 + D-165 + D-166 + D-167 + D-168 + D-169 + D-170 sealed. Ready for per-story-delivery cycle.

---

**Draft story policy:** Stories with `status: draft` MAY have empty
`behavioral_contracts: []` arrays. BC anchoring is deferred to the elaboration phase
(when status transitions to `ready`). **Source:** Phase 1d pass 3 F-035.

> Stories with `status: merged` that pre-date BC anchoring (e.g., Tier A/B/C/D
> stories migrated from S-N.M legacy format in Phase 1.8) MAY also have empty
> `behavioral_contracts: []`. BC backfill for these merged stories is tracked
> under TD-001 (BC-level CAP/DI/Stories anchoring incomplete).

**Status values:** draft, ready, in-progress, merged, partial, blocked

**Total story points:** 219 across 48 stories (190 E-0..E-5 + 3 E-6 + 21 E-7 + 5 E-8*)

> \*E-8 in progress — only S-8.00 authored at 5pts; ~118 additional pts pending S-8.01..S-8.28.

**Rules:**
- Every story has a unique sequential ID (zero-padded: S-N.MM)
- Points are 1-13 (no story exceeds 13 points)
- Dependencies are acyclic (topological sort valid)
- P0 stories do not depend on P1/P2 stories
