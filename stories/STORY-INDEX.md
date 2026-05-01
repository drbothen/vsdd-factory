---
document_type: story-index
level: ops
version: "1.14"
status: current
producer: state-manager
timestamp: 2026-05-01T00:00:00
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
> **E-8 Tier 1 batch authoring (2026-04-30):** 9 Tier 1 hook port stories authored across 2 bursts (S-8.01..S-8.09) at v1.0 status=draft; 22 BCs anchored (BC-7.03.* + BC-7.04.*); 33 base story points; 65 ACs; 3,240 lines; 0 [process-gap] BC-anchor fallbacks. R-8.10 NOT triggered. STORY-INDEX v1.7 → v1.8 (D-171).
> **E-8 Tier 1 batch pass-1 adversarial reviews (2026-04-30):** 9 stories reviewed in parallel; all SUBSTANTIVE; 122 findings total (43H+49M+21L+9NIT); all clocks held 0_of_3. CRITICAL: S-8.04 + S-8.09 BLOCKED on missing host::write_file in SDK (D-6 Option A trigger confirmed). 10 universal systematic patterns identified. STORY-INDEX v1.8 → v1.9 (D-172).
> **E-8 Tier 1 v1.0→v1.1 fix burst (2026-04-30):** 9 parallel story-writer bursts applied pass-1 fixes to all Tier 1 stories; 122 findings closed (43H+49M+21L+9NIT); all stories bumped v1.0→v1.1 status=draft; total 3,248→3,883 lines (+635). Universal patterns applied: wasm32-wasi→wasm32-wasip1; vsdd-hook-sdk=crates/hook-sdk; hooks.json positive-verification; subsystems+=SS-04; CAP-022 stretch; wave:15 [process-gap]; input-hash convention; AC perf Tier 1 exclusion; emit_event signature mapping; read_file capability declarations. CRITICAL BLOCKER: S-8.04 + S-8.09 BLOCKED on host::write_file SDK extension (D-6 Option A; story TBD). Per-story structural changes: S-8.06 AC count 6→4 (over-decomposition fix); S-8.03 +AC-007 (malformed JSON graceful exit); S-8.05 verbatim 3-line bash remediation block; S-8.07 serde_yaml pinned 0.9.34 (deprecated by Dtolnay 2024; TD entry logged). STORY-INDEX v1.9 → v1.10 (D-173).
> **E-8 Tier 1 batch pass-2 adversarial reviews (2026-04-30):** 9 parallel pass-2 reviews; all SUBSTANTIVE; all clocks held 0_of_3; 65 total findings (18H+34M+16L+4NIT). Trajectory: 122 → 65 (-47% decay). Pass-1 verification: 116 of 122 fully closed (95%); 6 had partial residuals re-surfaced as pass-2 findings. Per-story: S-8.01=4 (0H+2M+2L); S-8.02=6 (2H+3M+1L); S-8.03=9 (2H+4M+2L+1NIT); S-8.04=11 (3H+5M+2L+1NIT) D-6 BLOCKER; S-8.05=4 (0H+3M+1L); S-8.06=9 (1H+4M+3L+1NIT); S-8.07=11 (4H+4M+2L+1NIT); S-8.08=9 (2H+5M+2L); S-8.09=9 (4H+4M+1L) D-6 BLOCKER. CRITICAL new discoveries: (1) SS-04 universal mis-anchor across S-8.07/08/09 — ARCH-INDEX SS-04=Plugin Ecosystem; for SDK surface correct is SS-02; (2) S-8.07 vsdd-hook-sdk path wrong (resolves to non-existent crates/crates/hook-sdk); (3) S-8.07 missing workspace Cargo.toml members registration; (4) BC-7.03.071 invariants fabricated again in S-8.09 (regression of F-009); (5) D-6 sibling divergence: S-8.04 declares depends_on S-8.SDK-write-file; S-8.09 orphaned; (6) emit_event signature: universal pattern not pinned across siblings; (7) read_file max_bytes+timeout_ms unspecified. STORY-INDEX v1.10 → v1.11 (D-174).
> **S-8.10 SDK extension authored (2026-05-01):** S-8.10-sdk-extension-write-file.md v1.0 status=draft; 3pts; blocks S-8.04 + S-8.09 (D-6 Option A unblocker); depends_on S-8.00; SS-02; behavioral_contracts=[] pending OQ-1 BC authorship; HOST_ABI_VERSION stays at 1 (AS-DEC). Tier 2/3 stories renumbered S-8.10..S-8.28 → S-8.11..S-8.29. STORY-INDEX v1.11 → v1.12.
> **S-8.10 v1.1 pass-1 fix burst (2026-05-01):** 18 findings from adv-s8.10-p1.md closed (5H+7M+5L+1NIT); input-paths corrected to cycles/ prefix; max_bytes added to signature per BC-2.02.002; FFI input-pointer protocol pinned in AC-1; AC-5 conditional removed (max_bytes mandatory); BC family BC-2.01.005→BC-2.02.011 (BC-2.01.x=HookResult family; BC-2.02.x=host-shim family); WriteFileCaps struct defined; allow_write test helper added; Rule 4 return codes aligned; estimate 3→5 pts; input-hash e441e99.
>
> **E-8 Tier 1 batch pass-5 adversarial reviews (2026-05-01):** 10 parallel pass-5 reviews; 4 stories CONVERGED per ADR-013 (S-8.01 v1.3, S-8.03 v1.2, S-8.07 v1.2, S-8.09 v1.2 all reach 3/3 NITPICK_ONLY). 3 advanced (S-8.04 v1.3 → 1/3, S-8.06 v1.4 → 1/3, S-8.10 v1.1 → 2/3). 3 SUBSTANTIVE require fix burst: S-8.02 v1.3 (1 MED P5-001 HTML comment input-hash 5015917 vs frontmatter 5ae44ad contradiction); S-8.05 v1.4 (2 HIGH+1 MED — F-P5-001 T-5 emit_event partial-fix regression vs AC-008 reframe; F-P5-002 SS-04 "Hook Plugin Layer" mis-anchor vs canonical "Plugin Ecosystem"; F-P5-003 AC-005 sub-case mis-anchor); S-8.08 v1.3 (1 HIGH+1 MED — F-P5-001 bash-parity violation: WASM port adds agent_id+tool_name fields not in bash source — CLOCK RESET 1/3 → 0/3, requires PO adjudication). 5th anti-fabrication HARD GATE on BC-7.03.071 PASSED. STORY-INDEX v1.12 → v1.13 (D-180).
>
> **E-8 Tier 1 pass-5 fix burst + 4 CONVERGED status flips (2026-05-01, D-181):** 7 parallel story-writer dispatches. Fix bursts: S-8.02 v1.3→v1.4 (HTML-comment input-hash mismatch reframed; T-11 added to Tasks block; 1 MED + 1 LOW closed); S-8.05 v1.4→v1.5 (T-5 emit_event partial-fix regression closed [`let _ =` → bare statement]; SS-04 mis-anchor "Hook Plugin Layer" → canonical "Plugin Ecosystem"; AC-005 dual-fallback re-anchored to AC-003; T-3 RESULT casing; 2 HIGH + 1 MED + 1 LOW closed). S-8.08 v1.3→v1.4 (**strict E-8 D-2 parity restored** per option-(a) adjudication: `agent_id` and `tool_name` REMOVED from T-3, Goal, AC-002a; bash source empirically confirmed to emit only `type/hook/matcher/subagent/[story_id]`; AC-002a reframed as parity-audit with negative assertions; SS-06 CAP-022 disclosure reconciled; Token Budget BC-files corrected; 1 HIGH + 1 MED + 2 LOW closed; **TD-015 registered for future per-invocation correlation use case**). 4 CONVERGED status flips: S-8.01 v1.3→v1.4 ready, S-8.03 v1.2→v1.3 ready, S-8.07 v1.2 ready (cleanest — process-event row below), S-8.09 v1.2→v1.3 ready. 3-of-4 status flips forced cosmetic version bumps by validate-changelog-monotonicity.sh hook (process-gap noted in D-181). Pass-6 batch dispatch on 6 non-converged stories (S-8.02/04/05/06/08/10) next. STORY-INDEX v1.13 → v1.14.
> This index is the authoritative source for story count and status.
> 58 stories across 9 epics (E-0 through E-8).

> **Filename convention:** Stories live at `.factory/stories/S-N.MM-<short-description>.md`. Example: S-1.05 lives at `S-1.05-wasmtime-integration.md`.

> **Cycle field semantics:** Each story's `cycle:` frontmatter records the cycle that ORIGINALLY CREATED that story. For the 22 merged stories migrated in Phase 1.8 (S-N.M -> S-N.MM), `cycle: v1.0.0-greenfield` is preserved as immutable history. The current cycle (`v1.0-brownfield-backfill`) is for backfill of formal specs around already-shipped stories, not for new story creation.

## Status Summary

| Status | Count |
|--------|-------|
| merged | 45 |
| partial | 1 |
| draft | 7 |
| ready | 5 |
| **Total** | **58** |

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

> **E-8 spec CONVERGENCE_REACHED (2026-04-30):** E-8-native-wasm-migration.md v1.7 status=ready; 11 adversarial passes; trajectory 18→7→0→1→0→2→3→1→0→1→0; D-163 sealed. 30 stories planned (S-8.00..S-8.29). story_count: 30 (1 pre-work + 9 Tier 1 + 1 SDK extension + 9 Tier 2 + 10 Tier 3). target_release: v1.1 (Tier 1), v1.2 (Tier 2), v1.3 (Tier 3). Anchors: CAP-002, CAP-008, CAP-013, CAP-022. Tech debt: TD-014.

| Story ID | Title | Epic | Points | Priority | Depends On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|------------|--------|--------|-----|
| S-8.00 | Perf benchmark baseline + Tier 1 BC-anchor verification (W-15 pre-work) | E-8 | 5 | P2 | -- | S-8.01..S-8.09 | ready | [] ([process-gap] under D-2 Option C; v1.1 candidates: BC-7.00.001, BC-7.00.002) |
| S-8.01 | Native port: handoff-validator | E-8 | 4 | P2 | S-8.00 | S-8.09 | ready | BC-7.03.042, BC-7.03.043, BC-7.03.044 |
| S-8.02 | Native port: pr-manager-completion-guard | E-8 | 5 | P2 | S-8.00 | S-8.09 | draft | BC-7.03.045, BC-7.03.046, BC-7.03.047, BC-7.03.048 |
| S-8.03 | Native port: track-agent-stop | E-8 | 3 | P2 | S-8.00 | S-8.09 | ready | BC-7.03.081, BC-7.03.082 |
| S-8.04 | Native port: update-wave-state-on-merge | E-8 | 4 | P2 | S-8.00 | S-8.09 | draft | BC-7.03.083, BC-7.03.084, BC-7.03.085, BC-7.03.086 |
| S-8.05 | Native port: validate-pr-review-posted | E-8 | 3 | P2 | S-8.00 | S-8.09 | draft | BC-7.04.040, BC-7.04.041, BC-7.04.042, BC-7.04.043, BC-7.04.044 |
| S-8.06 | Native port: session-learning | E-8 | 3 | P2 | S-8.00 | S-8.09 | draft | BC-7.03.076, BC-7.03.077, BC-7.03.078 |
| S-8.07 | Native port: warn-pending-wave-gate | E-8 | 3 | P2 | S-8.00 | S-8.09 | ready | BC-7.03.091, BC-7.03.092 |
| S-8.08 | Native port: track-agent-start | E-8 | 3 | P2 | S-8.00 | S-8.09 | draft | BC-7.03.079, BC-7.03.080 |
| S-8.09 | Native port: regression-gate + adapter retirement prep | E-8 | 5 | P2 | S-8.00, S-8.01, S-8.02, S-8.03, S-8.04, S-8.05, S-8.06, S-8.07, S-8.08 | S-8.10..S-8.29 | ready | BC-7.03.071, BC-7.03.072, BC-7.03.073, BC-7.03.074, BC-7.03.075 |
| S-8.10 | SDK extension: host::write_file (D-6 Option A unblocker) | E-8 | 5 | P2 | S-8.00 | S-8.04, S-8.09 | draft | [] (OQ-1: BC-2.02.011 pending PO authorship) |

> S-8.00 v1.5 (status=**ready**, **CONVERGENCE_REACHED** at adversarial pass-6 per ADR-013 2026-04-30). 512 lines; 9 ACs; 5pts; depends_on=[]; blocks S-8.01..S-8.09. Two-responsibility scope: (A) perf benchmark baseline resolving OQ-8 (~10ms/plugin warm-invocation); (B) BC-anchor verification table for 9 Tier 1 hooks (handoff-validator, pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge, validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start, regression-gate) per D-2 Option C. behavioral_contracts=[] intentional ([process-gap] disclosure). subsystems=[SS-01, SS-07]. Adversarial pass-1 closed (14 v1.1); pass-2 closed (8 v1.2); pass-3 closed (6 v1.3); pass-4 NITPICK_ONLY (3 closed v1.4 + clock 0/3→1/3); pass-5 NITPICK_ONLY (1 NIT SKIP_FIX + clock 1/3→2/3); **pass-6 NITPICK_ONLY (2 NIT SKIP_FIX + clock 2/3→3/3 = CONVERGENCE_REACHED)**. Trajectory 14→8→6→3→1→2 over 6 passes (86% decay; healthy late-convergence shape). D-164 + D-165 + D-166 + D-167 + D-168 + D-169 + D-170 sealed. Ready for per-story-delivery cycle.

> S-8.01 v1.4 (status=**ready**, **CONVERGENCE_REACHED** 2026-05-01, D-181 status flip). Pass-1: 14 findings closed (4H+6M+3L+1NIT); 344→380 lines (+36). Pass-2: 4 findings (0H+2M+2L); 380→419 lines (+39). Pass-3: 7 findings (1H+3M+2L+1NIT); input-hash bcf4e0a → 7b31f6f. Pass-4 NITPICK_ONLY (clock 1/3). Pass-5 NITPICK_ONLY (clock 2/3). Pass-5 verdict: 1 NIT (clock 2/3→3/3 = CONVERGENCE_REACHED per ADR-013). Status flip burst v1.3→v1.4 (validate-changelog-monotonicity.sh forced version bump; process-gap per D-181). input-hash updated post-burst.

> S-8.02 v1.4 (status=draft, pass-5 fix burst applied 2026-05-01, D-181). Pass-1: 13 findings closed (4H+5M+3L+1NIT); 322→383 lines (+61). Pass-2: 6 findings (2H+3M+1L); 383→427 lines (+44). Pass-3: NITPICK_ONLY (clock 1/3 held). Pass-4: 4 findings (1H+1M+2L); 427→436 lines (+9); input-hash e441e99 → 5ae44ad. Pass-5 SUBSTANTIVE (1 MED HTML-comment input-hash 5015917 vs frontmatter 5ae44ad; 1 LOW T-11 asymmetry). Pass-5 fix burst v1.3→v1.4: F-S802-P5-001 [MED] HTML comment input-hash mismatch reframed; T-11 added to Tasks block to track BC-7.03.045 amendment obligation; 1 MED + 1 LOW closed. Clock: HELD (pass-6 next). adv-s8.02-p5.md persisted.

> S-8.03 v1.3 (status=**ready**, **CONVERGENCE_REACHED** 2026-05-01, D-181 status flip). Pass-1: 13 findings closed (4H+5M+3L+1NIT); 295→356 lines (+61). Pass-2: 9 findings (2H+4M+2L+1NIT). Pass-3 fix burst + NITPICK_ONLY (clock 0/3→1/3). Pass-4 NITPICK_ONLY (clock 1/3→2/3). Pass-5 NITPICK_ONLY (clock 2/3→3/3 = CONVERGENCE_REACHED). Status flip burst v1.2→v1.3 (validate-changelog-monotonicity.sh forced version bump; process-gap per D-181). input-hash updated post-burst.

> S-8.04 v1.3 (status=draft, pass-4 fix burst applied 2026-05-01). **HIGHEST RISK — D-6 BLOCKER (host::write_file SDK extension required; S-8.10 must merge first).** Pass-1: 17 findings closed (7H+6M+3L+1NIT); pass-2: 13 findings closed (v1.2); pass-3: 4 findings adv-only (0H+1M+3L) not yet applied; pass-4: 6 findings (2H+1M+3L) all closed in v1.3 burst. Key v1.3 fixes: SS-04 "Wave State" fabricated name corrected to "Plugin Ecosystem" (ARCH-INDEX:77 confirmed, POLICY 6 HIGH); write_file 4-param signature `(path, contents, max_bytes, timeout_ms)` propagated from S-8.10 v1.1 AC-1 across all 4 call sites (HIGH); BC-7.03.083 stale line-range 877-894→942-948 follow-up tracked via T-9.5 process-gap (MED); vsdd-hook-sdk version gate reworded to "asserted post-S-8.10 merge — STOP if 0.1.x" (LOW); AC-006 fixture key gate:~→gate_status:~ (LOW); T-1.5 serde_yaml wording "last release; dtolnay deprecated at this version per 2024 announcement" (LOW). adv-s8.04-p4.md closed.

> S-8.05 v1.5 (status=draft, pass-5 fix burst applied 2026-05-01, D-181). Pass-1: 12 findings closed (4H+5M+2L+1NIT); 338→395 lines (+57). Pass-2: 4 findings (0H+3M+1L); all closed in v1.2. Pass-3: 5 findings (2H+1M+2L); all closed in v1.3. Pass-4: 4 findings (2H+0M+2L); all closed in v1.4. Pass-5 SUBSTANTIVE (2H+1M+1L): F-P5-001 T-5 emit_event partial-fix regression (let _ = → bare statement); F-P5-002 SS-04 "Hook Plugin Layer" → "Plugin Ecosystem" (POLICY 6); F-P5-003 AC-005 dual-fallback mis-anchor → AC-003. Pass-5 fix burst v1.4→v1.5: T-5 bare-statement form corrected; SS-04 canonical name fixed; AC-005 sub-case re-anchored to AC-003; T-3 RESULT casing; 2 HIGH + 1 MED + 1 LOW closed. Clock RESET 0/3 (pass-6 next). adv-s8.05-p5.md persisted.

> S-8.06 v1.1 (status=draft, pass-1 fix burst applied 2026-04-30; pass-2 SUBSTANTIVE 2026-04-30). Pass-1: 11 findings closed (4H+5M+1L+1NIT); 354→419 lines (+65). Pass-2: 9 findings (1H+4M+3L+1NIT); clock held 0_of_3. Key pass-2 findings: BC-7.03.076 gate regressed from "includes" to "may include" — soft gate leaves self-reference loop open (HIGH, pass-1 regression); AC numbering gap (AC-001/003/004/005, no AC-002 after 6→4 collapse) (MED); BC-7.03.076 trace content fabricated (MED); EC-005 "large" envelope not quantified in bytes (MED); wave [provisional] vs [process-gap] inconsistency (MED). Pass-2 fix burst pending. adv-s8.06-p2.md persisted. D-174.

> S-8.07 v1.2 (status=**ready**, **CONVERGENCE_REACHED** 2026-05-01, D-181 status flip). Pass-1: 14 findings closed (6H+5M+2L+1NIT); 358→433 lines (+75). Pass-2: 11 findings (4H+4M+2L+1NIT). Pass-3 fix burst (v1.2). Pass-4 NITPICK_ONLY (clock 0/3→1/3). Pass-5 NITPICK_ONLY (clock 1/3→2/3). Pass-5 verdict: 3 LOW re-flags (clock 2/3→3/3 = CONVERGENCE_REACHED). Status flip via cleanest path: process-event row with -- version cell appended BELOW latest versioned row (no frontmatter bump needed; S-8.07 stays at v1.2). input-hash updated post-burst.

> S-8.08 v1.4 (status=draft, **CLOCK RESET** 0/3, pass-5 fix burst applied 2026-05-01, D-181). Pass-1: 12 findings closed (4H+5M+2L+1NIT); 343→385 lines (+42). Pass-2: 9 findings (2H+5M+2L). Pass-3 fix burst (v1.3); NITPICK_ONLY clock 0/3→1/3. Pass-5 (fresh-context) found F-S808-P5-001 HIGH bash-parity violation: agent_id+tool_name present in WASM spec but NOT emitted by bash track-agent-start.sh; CLOCK RESET 1/3→0/3; adjudication escalated to PO. D-181 sealed option (a): STRICT E-8 D-2 PARITY — agent_id and tool_name REMOVED from T-3 emit_event call, Goal section, AC-002a. AC-002a reframed as parity-audit with negative assertions. SS-06 CAP-022 disclosure reconciled; Token Budget BC-files corrected. Pass-5 fix burst v1.3→v1.4: 1 HIGH + 1 MED + 2 LOW closed. TD-015 registered for future per-invocation correlation epic. Pass-6 next (full re-review of parity restoration). adv-s8.08-p5.md persisted.

> S-8.09 v1.3 (status=**ready**, **CONVERGENCE_REACHED** 2026-05-01, D-181 status flip). **D-6 BLOCKER (host::write_file SDK extension required for implementation; spec converged independently).** Pass-1: 16 findings closed (6H+7M+2L+1NIT); 542→647 lines (+105). Pass-2: 9 findings (4H+4M+1L). Pass-3 fix burst (v1.2). Pass-4 NITPICK_ONLY (clock 0/3→1/3). Pass-5 NITPICK_ONLY (4 LOW positive confirmations; clock 1/3→2/3). 5th BC-7.03.071 anti-fabrication HARD GATE PASSED. Pass-5 verdict: CONVERGENCE_REACHED (clock 2/3→3/3 per ADR-013). Status flip burst v1.2→v1.3 (validate-changelog-monotonicity.sh forced version bump; process-gap per D-181). input-hash updated post-burst.

---

**Draft story policy:** Stories with `status: draft` MAY have empty
`behavioral_contracts: []` arrays. BC anchoring is deferred to the elaboration phase
(when status transitions to `ready`). **Source:** Phase 1d pass 3 F-035.

> Stories with `status: merged` that pre-date BC anchoring (e.g., Tier A/B/C/D
> stories migrated from S-N.M legacy format in Phase 1.8) MAY also have empty
> `behavioral_contracts: []`. BC backfill for these merged stories is tracked
> under TD-001 (BC-level CAP/DI/Stories anchoring incomplete).

**Status values:** draft, ready, in-progress, merged, partial, blocked

**Total story points:** 255 across 58 stories (190 E-0..E-5 + 3 E-6 + 21 E-7 + 41 E-8*)

> \*E-8 in progress — only S-8.00 + 9 Tier 1 stories authored at 38pts; ~85 additional pts pending S-8.11..S-8.29 (Tier 2 + Tier 3).

**Rules:**
- Every story has a unique sequential ID (zero-padded: S-N.MM)
- Points are 1-13 (no story exceeds 13 points)
- Dependencies are acyclic (topological sort valid)
- P0 stories do not depend on P1/P2 stories
