---
document_type: burst-log
cycle: v1.0-feature-plugin-async-semantics-pass-1
producer: state-manager
version: "1.0"
last_updated: 2026-05-07
---

# Burst Log — v1.0-feature-plugin-async-semantics-pass-1

Plugin async semantics: partition belongs at the registry layer; defeats silent-block
bleed observed in the prism audit (2026-05-07, 55 silent blocks from
validate-template-compliance).

---

## Burst 1 — F1 delta analysis authored + cycle registered + tech-debt expansion

**Date:** 2026-05-07
**Dispatchers:** orchestrator → architect → state-manager
**Phase:** F1 COMPLETE → human-review-gate

### Outputs

| File | Author | Notes |
|------|--------|-------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` | architect | F1 delta analysis: 4 stories sketched (dispatcher partition, plugin classification, envelope flip, CI lint invariant). 1 ADR + 2 new BCs + 2 new VPs proposed for F2. |
| `.factory/STATE.md` | state-manager | Cycle registration: current_cycle flipped, Active Cycles table updated (plugin-async-semantics added; engine-discipline-pass-1 → PAUSED). Phase Progress + Session Checkpoint updated. |
| `.factory/tech-debt-register.md` | state-manager | TD-027 authored (Stop-hook async-block surfacing; medium severity, S-M effort, v1.1 target). Scope statement broadened from defect-only to general deferred-work inbox. |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | state-manager | This file (cycle burst log initialized). |

### Decisions sealed

None. F1 is exploratory; F2 will produce seal-able specs once human approval gate clears.

### Open questions carried forward

- OQ-1: Stop-hook (or SubagentStop) turn-end summary for async-block decisions — now tracked as TD-027.
- OQ-2: Plugin classification taxonomy (deterministic validators vs advisory/telemetry) — architect's F1 analysis outlines; product-owner + architect to finalize in F2.
- OQ-3: Envelope flip scope — which plugins get `on_error: "block"` → sync reclassification vs which legitimately stay async.

### Status

F1 COMPLETE. Awaiting human review gate before F2 spec evolution.

---

## Burst 2 — F2 spec evolution: BCs + ADR + VPs authored in parallel

**Date:** 2026-05-07
**Dispatchers:** orchestrator → (product-owner ∥ architect) → state-manager
**Phase:** F2 SPECS AUTHORED → adversarial convergence pending

### Outputs

#### New files

| File | Author | Notes |
|------|--------|-------|
| `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | architect | Accepted. Subsystems: SS-01, SS-07, SS-09. Supersedes: null. Single hard cut, no backcompat, no phased rollout per user decisions 2026-05-07. |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md` | product-owner | Dispatcher partition contract (SS-01). sync_group awaits verdict; async_group fire-and-forget. |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` | product-owner | hooks-registry.toml schema_version 2 — per-plugin `async: bool` field + CI lint invariant `on_error=block ⇒ async=false`. |
| `.factory/specs/verification-properties/VP-077.md` | architect | Dispatcher partition correctness — Kani proof. Partition totality, disjointness, union completeness, exit-code independence. |
| `.factory/specs/verification-properties/VP-078.md` | architect | CI lint invariant `on_error=block ⇒ async=false` — integration/bats proof. |

#### Amended files (version bumps)

| File | Old Version | New Version | Notes |
|------|-------------|-------------|-------|
| `ss-01/BC-1.01.001.md` | v1.0 | v1.1 | REGISTRY_SCHEMA_VERSION 1→2 hard gate |
| `ss-01/BC-1.01.007.md` | v1.0 | v1.1 | Minimal registry fixture v2; async=false default |
| `ss-01/BC-1.08.002.md` | v1.0 | v1.1 | Exit code 2 semantics scoped to sync group only |
| `ss-04/BC-4.04.004.md` | v1.0 | v2.0 | SessionStart async:true removed (envelope now sync) |
| `ss-04/BC-4.05.004.md` | v1.0 | v2.0 | SessionEnd async:true removed (envelope now sync) |
| `ss-04/BC-4.07.003.md` | v1.1 | v1.2 | Worktree* async:true removed (envelope now sync) |
| `ss-04/BC-4.08.002.md` | v1.2 | v1.3 | PostToolUseFailure async:true removed (envelope now sync) |
| `specs/verification-properties/VP-001.md` | v1.0 | v1.1 | Tier execution semantics scoped to sync group |
| `specs/verification-properties/VP-002.md` | v1.0 | v1.1 | Sibling-isolation semantics scoped to sync group |

#### Index files updated

| File | Change |
|------|--------|
| `specs/behavioral-contracts/BC-INDEX.md` | v1.18→v1.19; total_bcs 1943→1945; +2 new rows (BC-1.14.001, BC-7.06.001); 7 amended BC titles updated; SS-01 115→116, SS-07 196→197 |
| `specs/architecture/ARCH-INDEX.md` | v1.9→v1.10; +ADR-019 row; SS-01/SS-07/SS-09 affected |
| `specs/verification-properties/VP-INDEX.md` | v1.5→v1.6; total_vps 76→78; +VP-077/078 rows; VP-001/VP-002 versions noted |
| `STATE.md` | current_step updated; Concurrent Cycles table updated; compaction: D-337..D-362 steps archived here |

### STATE.md compaction (archived from Current Phase Steps)

The following step rows were in STATE.md before compaction and are now archived here to keep STATE.md under the 200-line budget:

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| **D-337 state-manager seal — pass-8 fix-cycle** | state-manager | COMPLETE | BC-INDEX 1.13→1.14 (16 BC version pins); STORY-INDEX 2.22→2.23 (S-10.05 1.4→1.5); E-10-pass-8.md created; STATE.md + lessons.md sealed. Pass-9 is next dispatch (PAUSED). |
| **D-340 F2 spec evolution — engine-discipline-pass-1** | product-owner + architect + state-manager | COMPLETE | 6 BCs (BC-5.39.001/002 SS-05; BC-4.10.001/002 + BC-4.11.001 SS-04; BC-6.22.001 SS-06). ADR-016 + ADR-017. VP-069..072. PRD 1.0→1.1 (FR-047). BC-INDEX 1.14→1.15; ARCH-INDEX 1.7→1.8; VP-INDEX 1.0→1.1. total_bcs 1931→1937. current_cycle flipped. Next: F3 (3 stories: C path governance, A workflow+agent docs, B WASM hook). |
| **E-11 authoring + indexing burst (orphan-hook anchor) — D-11.7** | product-owner (E-11 epic) + state-manager (indexing) | COMPLETE | E-11 epic v1.0/draft authored (491L; 8 stories S-11.01..S-11.08; target v1.3); collision resolved: S-11.00 already registered as verify-sha-currency.sh stub (D-297); E-11 stories renumbered +1 (S-11.01..S-11.08); STORY-INDEX 2.23→2.24 (8 new rows + 8 pointer updates S-8.20–S-8.27 re-pointed to E-11); E-11 frontmatter story_count corrected 7→8. |
| **E-11 epic v1.1 amendment — sync body to STORY-INDEX renumber** | product-owner (epic body) + state-manager (commit) | COMPLETE (25b3c20) | E-11 epic body v1.0→v1.1: all live story-pointer refs renumbered S-11.00..S-11.07 → S-11.01..S-11.08 to match STORY-INDEX (14bb9c4). CHANGELOG v1.1 entry + narrative appended. Verification: zero live S-11.00 refs; dependency graph topology preserved. No semantic changes. |
| **D-349 F3 story decomposition — v1.0-feature-engine-discipline-pass-1** | product-owner (epics E-12/E-13) + story-writer (S-13.01/S-12.01/S-12.02) + state-manager (indexing + commit) | COMPLETE | E-12 Engine Governance (S-12.01/S-12.02) + E-13 Artifact Integrity (S-13.01). 38 ACs total. Linear: S-13.01→S-12.01→S-12.02. All tdd_mode strict. STORY-INDEX 2.24→2.25. D-345..D-348 logged. OQ-9 surfaced (VP-071 vs BC-4.10.001 discrepancy; pre-F4 gate). |
| **D-350 S-13.01 merged — state update post-merge** | state-manager | COMPLETE | S-13.01 (Path Governance Bundle, E-13) merged to develop at 2c97cb0 (PR #97, 2026-05-07). validate-artifact-path WASM hook live in block mode. sprint-state.yaml S-13.01 → completed; STORY-INDEX 2.25→2.26 (S-13.01 draft→completed); STATE.md F4 IN PROGRESS; decision-log D-350+D-351 appended. S-12.01 + S-12.02 unblocked. |
| **D-352..D-355 F4 closeout — S-12.01 + S-12.02 merged** | state-manager | COMPLETE | S-12.01 merged at 2e9b670 (PR #98, 2026-05-07): 31/31 bats, CLEAN security, 1-cycle convergence. S-12.02 merged at e2fd3d4 (PR #99, 2026-05-07): 148KB WASM, SubagentStop priority 960, 30/30 cargo + 11+1skip bats, conflict resolution at 7100431. sprint-state.yaml S-12.01+S-12.02 → completed; STORY-INDEX 2.26→2.27; STATE.md F4 COMPLETE; decision-log D-352..D-355 appended. Next: F5. |
| **D-356 F5 pass-1 — adversarial review persisted** | state-manager | COMPLETE | Classification: CRITICAL. 29 findings (4C/14H/6M/5L). adv-cycle-pass-1.md persisted (65KB, 704L). 2 [process-gap] observations surfaced. INDEX.md + decision-log updated. Next: route findings via fix-pr-delivery; pass-2 after remediation. |
| **D-357 F5 pass-1 B1 spec amendments** | state-manager | COMPLETE | VP-071 v1.1→v1.2 (BlockWithFix→Block; F-CRIT-3/F-HIGH-5/F-MED-7). BC-4.11.001 v1.0→v1.1 (NC-1 single-segment semantics). 6 BC input-hashes → 40a6fb6 (F-LOW-5). ADR-017 slug fixed in S-12.01, S-12.02, E-12 (F-CRIT-2). BC-INDEX 1.15→1.16; VP-INDEX 1.2→1.3. B1 source fix PR in flight. |
| **D-358 F5 pass-1 B2 spec amendments** | state-manager | COMPLETE | BC-4.10.002 v1.0→v1.1 (PC3 log_debug→log_info; F-HIGH-4). VP-070 v1.0→v1.1 (match_path→matches_canonical, BC-4.11.001 resolved, MatchResult/PathRegistry types corrected; F-HIGH-10). S-13.01 terminology (parse_registry→load_registry, match_path→matches_canonical; F-HIGH-9). S-12.02 block_with_fix throughout (F-HIGH-12). BC-INDEX 1.16→1.17; VP-INDEX 1.3→1.4. B2 source fix PR in flight. |
| **D-359 F5 B6 process-gap stories + PG-2 backfill** | state-manager | COMPLETE | E-14 Engine Discipline Pass-2 authored (5 stories: S-14.01 P0, S-14.02..S-14.04 P1, S-14.03 P2). PG-2 inline backfill: adversary-convergence-state.json created for S-13.01/S-12.01/S-12.02 with bootstrap_annotation (exception_type: cycle_self_introduction). STORY-INDEX 2.27→2.28 (84 stories, 14 epics). F7 CONVERGENCE_STATE_MISSING risk cleared. B3+B4 source PRs in flight (#103, #104). |

### Decisions sealed

- ADR-019 accepted (async semantics at registry layer; hard cut, no backcompat, no phased rollout per user decisions 2026-05-07)
- BC-1.14.001 ratified pending adversarial convergence (dispatcher partition contract)
- BC-7.06.001 ratified pending adversarial convergence (registry schema v2 + CI lint)
- Envelope flip: all Claude Code hook events synchronous at the envelope layer (no per-event carve-outs)
- Plugin classification: telemetry plugins (`capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`, `session-end-telemetry`) → `async = true`; all block-capable validators → `async = false` (default)

### Open questions carried forward

- OQ-3 (closed): Per-plugin async at hooks.json envelope layer is technically impossible (confirmed: Claude Code `async` is per-envelope). Registry-layer approach is the only viable path. ADR-019 Decision sealed.
- OQ-1 (TD-027): Stop-hook async-block summary for residual-bleed after this cycle — still tracked as TD-027.

### Status

F2 SPECS AUTHORED. Adversarial spec convergence next (≥3 NITPICK_ONLY passes per ADR-013) before F3 story decomposition.

---
