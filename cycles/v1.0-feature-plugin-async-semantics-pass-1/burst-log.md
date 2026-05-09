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

## Burst: F2 pass-1 fix burst — 19 adversary findings closed (2026-05-07)

| Field | Value |
|-------|-------|
| **Burst date** | 2026-05-07 |
| **Dispatch chain** | orchestrator → adversary → state-manager(persist) → (PO ∥ architect) → architect(followup) → state-manager(close) |
| **Adversary verdict** | SUBSTANTIVE (19 findings: 6 HIGH, 7 MED, 4 LOW, 2 NIT) |
| **ADR-013 clock** | RESET to 0_of_3 |

### Outputs

**New artifacts:**

| File | Producer | Notes |
|------|----------|-------|
| `.factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md` | PO | hooks.json.template envelope-sync invariant; v1.0; addresses F-P1-001 |
| `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` | PO | SS-03 event catalog for 4 async-semantics event types; v1.0; addresses F-P1-008 |
| `.factory/specs/verification-properties/VP-079.md` | architect | Event payload schema conformance; v1.0; addresses BC-3.08.001 VP anchor (was TBD) |

**Amended artifacts:**

| File | Producer | Version | Findings addressed |
|------|----------|---------|-------------------|
| `.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md` | PO | v1.0→v1.1 | F-P1-003/004/009/010/011/013/015 |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` | PO | v1.0→v1.1 | F-P1-005/006/010/016 |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.08.001.md` | PO | v1.0→v1.1 | F-P1-004/011 |
| `.factory/specs/domain-spec/invariants.md` | PO | v1.2→v1.3 | F-P1-003 |
| `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | architect | v1.0→v1.1 | F-P1-018 (placeholder BC IDs resolved; BC-9.01.006 forward ref added by state-manager close) |
| `.factory/specs/architecture/SS-09-config-activation.md` | architect | v1.0→v1.1 | F-P1-002 |
| `.factory/specs/architecture/SS-07-hook-bash.md` | architect | v1.0→v1.1 | F-P1-002 |
| `.factory/specs/verification-properties/VP-077.md` | architect | v1.0→v1.2 | F-P1-007/017/019 |
| `.factory/specs/verification-properties/VP-078.md` | architect | v1.0→v1.3 | F-P1-007/012/016 (Harness 2 rewrite + Harness 3 added in follow-up) |

**Index bumps:**

| Index | Old | New | Notes |
|-------|-----|-----|-------|
| BC-INDEX | v1.19 (1945) | v1.20 (1947) | +2 new, 3 amendments, BC-7.06.001 subsystem reanchored SS-07→SS-01 |
| ARCH-INDEX | v1.10 | v1.11 | ADR-019 v1.1; SS-09 v1.1; SS-07 v1.1 noted |
| VP-INDEX | v1.6 (78) | v1.7 (79) | +1 new VP-079; VP-077 v1.2; VP-078 v1.3 |

### Findings summary (19 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 6 | F-P1-001 (envelope async), F-P1-003 (schema version), F-P1-004 (fail-closed), F-P1-008 (event catalog missing), F-P1-013 (async lifetime), F-P1-018 (placeholder BCs) |
| MED | 7 | F-P1-002, F-P1-005, F-P1-006 (subsystem reanchor), F-P1-007, F-P1-009, F-P1-010, F-P1-011 |
| LOW | 4 | F-P1-012, F-P1-015, F-P1-016, F-P1-017 |
| NIT | 2 | F-P1-019, F-P1-020 |
| SKIP_FIX | 1 | F-P1-014 (H1 length — intentional; no action) |

### Decisions sealed

- **F-P1-004/011 (FAIL-CLOSED):** schema-version mismatch exit code is **exit 2 (fail-closed)** — explicit exception to BC-1.08.001 fail-open. "No silent failures" is the user-stated principle. BC-1.14.001 EC-006, BC-1.08.001, and ADR-019 §Consequences all updated.
- **F-P1-006 (SUBSYSTEM REANCHOR):** BC-7.06.001 primary subsystem reanchored SS-07→SS-01. Runtime enforcement of the `on_error=block ⇒ async=false` invariant belongs to the dispatcher (SS-01), not the bash layer. Artifact frontmatter `subsystem: SS-01` is authoritative. File remains in `ss-07/` per POLICY 1 append-only.
- **F-P1-008 (EVENT CATALOG):** SS-03 event catalog handled via new BC-3.08.001 + VP-079. Four new event types codified: `plugin.async_block_discarded`, `dispatcher.schema_mismatch`, `dispatcher.registry_invalid`, `plugin.timeout` (async path).
- **F-P1-013 (ASYNC LIFETIME):** async plugin lifetime is **best-effort**; truncated telemetry is explicitly acceptable cost. Dispatcher shutdown does not await async group completion. VP-078 Harness 3 validates the positive-classification path. This is not a regression — it is intentional design for the fire-and-forget async group.

### Status

F2 PASS-1 FIX BURST CLOSED. Adversary pass-2 next (ADR-013 clock at 0_of_3 — 3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst: F2 pass-2 fix burst — 19 findings (1 SKIP_FIX) closed (2026-05-07)

| Field | Value |
|-------|-------|
| **Burst date** | 2026-05-07 |
| **Dispatch chain** | orchestrator → adversary → state-manager(persist) → (PO ∥ architect) → state-manager(close + forward-ref) |
| **Adversary verdict** | SUBSTANTIVE (6H/7M/4L/2N; pass-2 returned multiple partial-fix regressions from pass-1: renumbering ripple, sibling-pointer drift, body-vs-postcondition contradictions) |
| **ADR-013 clock** | RESET to 0_of_3 |

### Outputs

**Amended artifacts (PO):**

| File | Old Version | New Version | Findings addressed |
|------|-------------|-------------|-------------------|
| `ss-07/BC-7.06.001.md` | v1.1 | v1.2 | F-P2-001 (Invariant 7: (name,event) tuple uniqueness); F-P2-006 (Invariant 6 expanded: track-agent-start, track-agent-stop, session-learning added; warn-pending-wave-gate/regression-gate kept SYNC); F-P2-013 (Postcondition 3 reworded, no v1 compat framing) |
| `ss-01/BC-1.14.001.md` | v1.1 | v1.2 | F-P2-005 (postconditions renumbered 4,5,6 monotonic); F-P2-009 (Error Paths row for async exit-2 added); F-P2-015 (Precondition 4: BC-7.06.001 pin) |
| `ss-04/BC-4.04.004.md` | v2.0 | v2.1 | F-P2-003 (BC-7.06.001 reference: Postcondition 7 → Invariant 6) |
| `ss-04/BC-4.05.004.md` | v2.0 | v2.1 | F-P2-003 (same fix) |
| `ss-04/BC-4.07.003.md` | v1.2 | v1.3 | F-P2-004 (Description body: "async:true removed" → "async key absent"; matches actual schema where absence = false default) |
| `ss-03/BC-3.08.001.md` | v1.0 | v1.1 | F-P2-010 (Architecture Module: SS-07 → SS-01 reanchor for registry.rs enforcement path) |
| `ss-01/BC-1.08.001.md` | v1.1 | v1.2 | F-P2-017 (Stories field appended for new cycle) |
| `specs/domain-spec/invariants.md` (DI-014) | v1.3 | v1.4 | F-P2-014 (BC range reworded post-reanchor) |

**Amended artifacts (architect):**

| File | Old Version | New Version | Findings addressed |
|------|-------------|-------------|-------------------|
| `decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | v1.2 | v1.3 | F-P2-012 (PermissionRequest enumerated; already covered in v1.2); §Consequences sync: warn-pending-wave-gate/regression-gate SYNC rationale; async list expanded to 9 plugins per BC-7.06.001 Invariant 6; F-P2-011 (6 properties for VP-077 already in v1.2; §Consequences now cross-references Invariant 6) |
| `VP-077.md` | v1.2 | v1.3 | F-P2-002 (VP-078 H3→H4 Harness renumber); F-P2-011 (6 properties for VP-INDEX + ADR-019) |
| `VP-078.md` | v1.3 | v1.4 | F-P2-007 (Harness 2 CLI surface: no CLI flags; stdin envelope + env vars); F-P2-008 (fixture schema fix) |
| `VP-079.md` | v1.0 | v1.1 | F-P2-007 (all 4 scenarios CLI/fixture rewrite); F-P2-008 (fixtures); F-P2-016 (trace_id property relaxed) |
| `VP-INDEX.md` | v1.7 | v1.8 (prior burst) | F-P2-011 (VP-077 title expanded to cite 6 properties) |

**State-manager forward-reference resolution:**

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `VP-077.md` | v1.3 | v1.4 | Forward-ref resolved: BC-7.06.001 Invariant 7 cited for (name,event) tuple uniqueness. Harness 1 kani::assume updated to allow duplicate names across different events. Feasibility row updated. Amendment block added. |
| `VP-INDEX.md` | v1.7 | v1.8 | VP-077 row title updated; changelog entry for forward-ref resolution |
| `BC-INDEX.md` | v1.20 | v1.21 | Changelog entry for 7 BC amendments + 1 DI |
| `ARCH-INDEX.md` | v1.11 | v1.12 | ADR-019 row updated to v1.3; changelog entry |
| `STATE.md` | — | — | current_step, phase, cycle table, burst table, session checkpoint updated |

### Decisions sealed

- **F-P2-006 (ASYNC CLASSIFICATION EXPANSION):** 3 telemetry plugins added to Invariant 6
  async-required list: `track-agent-start`, `track-agent-stop`, `session-learning`. All
  three are telemetry-only and always return Continue. `warn-pending-wave-gate` (Stop) and
  `regression-gate` (PostToolUse) are deliberately classified SYNC with `on_error=continue`
  because they emit human-visible stderr warnings — async classification would silently drop
  warnings at dispatcher process exit. Determined by reading plugin source (`lib.rs`);
  both call `write_stderr`/`eprint!` and always return `HookResult::Continue`. Invariant 6
  now enumerates 9 ASYNC plugins + 2 deliberate SYNC (with rationale).
- **F-P2-007 (DISPATCHER CLI FLAGS DO NOT EXIST):** VP-078 Harness 2 and VP-079 scenarios
  previously scripted flags like `--async` that don't exist in the dispatcher binary.
  Architect rewrote all harnesses to use stdin envelope + env vars (the actual dispatcher
  interface). VP-078 Harness 2 and all VP-079 scenarios rewritten accordingly.
- **F-P2-011 (VP-077 6 PROPERTIES CANONICAL):** VP-077 enumerates 6 properties (totality,
  async-field respect, disjointness, union completeness, exit-code independence, aggregation
  correctness). VP-INDEX row and ADR-019 §Implementation Pointers now match VP-077's
  canonical enumeration. No other VPs added.
- **F-P2-001 (TUPLE UNIQUENESS):** PO assigned BC-7.06.001 Invariant 7 for (name, event)
  tuple uniqueness — not plain plugin-name uniqueness. Duplicate names across different
  events are intentional (worktree-hooks: WorktreeCreate + WorktreeRemove; protect-secrets:
  Bash + Read). Forward reference in VP-077 closed by state-manager.

### Findings summary (19 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 6 | F-P2-001 (uniqueness scope), F-P2-003 (sibling pointer drift), F-P2-004 (body contradiction), F-P2-006 (async list incomplete), F-P2-007 (CLI flags don't exist), F-P2-008 (fixture schema) |
| MED | 7 | F-P2-002 (harness renumber), F-P2-005 (PC renumber), F-P2-009 (Error Paths), F-P2-010 (SS reanchor), F-P2-011 (property count), F-P2-012 (PermissionRequest), F-P2-013 (PC3 framing) |
| LOW | 4 | F-P2-014 (BC range), F-P2-015 (pin), F-P2-016 (trace_id), F-P2-017 (Stories) |
| NIT | 1 | F-P2-018 |
| SKIP_FIX | 1 | F-P2-019 (events-*.jsonl glob form — intentional; no action) |

### Status

F2 PASS-2 FIX BURST CLOSED. Adversary pass-3 next (ADR-013 clock at 0_of_3 — 3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst: F2 pass-3 fix burst (initial + user-correction) — 7 findings closed (2026-05-07)

| Field | Value |
|-------|-------|
| **Burst date** | 2026-05-07 |
| **Dispatch chain** | orchestrator → adversary → (PO ∥ architect, initial fix burst) → user-review → (PO ∥ architect, user-correction round) → state-manager(close) |
| **Adversary verdict** | SUBSTANTIVE (4H/3M/0L/0N — 7 findings) |
| **ADR-013 clock** | RESET to 0_of_3 |
| **User steering** | "fix it the most correct and right way" — user chose architecturally correct paths over expedient ones on Q2 and Q3 |

### Decisions sealed

| Decision | Outcome |
|----------|---------|
| **Q1 user-confirmed: drain window kept** | ASYNC_DRAIN_WINDOW_MS = 100ms drain window retained (Position A). Pass-3 finding that the drain window might cause observable delay was reviewed and accepted as intentional design. |
| **Q2 user-corrected: ARCH-INDEX BC re-tally** | BC counts in ARCH-INDEX re-tallied to authoritative BC frontmatter `subsystem:` field (not directory location). Obsolete directory-based footnote removed. Net: SS-01 116→117 (+BC-7.06.001), SS-05 648→652 (+BC-8.29.001/002/003 + BC-8.30.002), SS-07 197→196, SS-08 218→214. Total 1,947 unchanged. |
| **Q3 user-corrected: ASYNC_DRAIN_WINDOW_MS → DI-019** | Constant lifted from BC-1.14.001 inline "Constant Definitions" table to DI-019 domain invariant. Domain layer is the canonical owner. BC-1.14.001 PC4 cites DI-019 by reference; value removed from BC body. |
| **F-P3-001: VP-078 Harness 3 expansion** | Harness 3 plugin list expanded six→nine plugins: added track-agent-start, track-agent-stop, session-learning per BC-7.06.001 Invariant 6 v1.2. VP-078 v1.4→v1.5. |
| **F-P3-002 / F-P3-007: VP-079 Scenarios 1+4 fixture fix** | Sync plugin added to Scenarios 1 and 4 to hold dispatcher alive during async drain window. Scenario 5 added (drain-window truncation negative case). VP-079 v1.1→v1.2. BC-1.14.001 PC4 updated with bounded ASYNC_DRAIN_WINDOW_MS. |
| **F-P3-003: POLICY 7 — 6 BC-INDEX H1 syncs** | BC-1.08.002, BC-1.01.007, BC-1.14.001 titles updated in BC-INDEX to match H1s byte-for-byte. BC-4.04.004/4.05.004/4.07.003/4.08.002 confirmed already matching. |
| **F-P3-004: VP-079 type drift** | VP-079 type corrected invariant→postcondition in VP-INDEX Full Index table. |
| **F-P3-005: ARCH-INDEX BC re-tally** | Covered by Q2 user-correction above. |
| **F-P3-006: SS-09/SS-07 stale text** | SS-09-config-activation.md v1.1→v1.2 (stale async/schema_v1 body text replaced in-place). SS-07-hook-bash.md v1.1→v1.2 (stale schema_v1 body text replaced in-place). |

### Outputs

**New artifacts:**

| File | Producer | Notes |
|------|----------|-------|
| `.factory/specs/domain-spec/invariants.md` (DI-019) | PO (user-correction) | DI-019 authored: ASYNC_DRAIN_WINDOW_MS=100ms; SS-01 enforcement; BC range BC-1.14.001 + BC-3.08.001. invariants.md v1.4→v1.5. |

**Amended artifacts:**

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `ss-01/BC-1.14.001.md` | v1.2 | v1.4 | PC4 updated with bounded drain window (v1.3 initial; v1.4 user-correction: cite DI-019, value removed from inline constant table) |
| `ss-03/BC-3.08.001.md` | v1.1 | v1.2 | DI-019 traceability cite added (plugin.timeout + plugin.async_block_discarded emitted within DI-019 drain window) |
| `specs/verification-properties/VP-078.md` | v1.4 | v1.5 | Harness 3 list six→nine plugins per BC-7.06.001 Invariant 6 v1.2 |
| `specs/verification-properties/VP-079.md` | v1.1 | v1.3 | v1.2: Property 5 + Scenarios 1/4/5 + drain-window; v1.3 (user-correction): DI-NN→DI-019 placeholder resolved throughout |
| `decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | v1.3 | v1.5 | v1.4: drain window in §Consequences; v1.5: DI-NN→DI-019 placeholder resolved |
| `specs/architecture/SS-09-config-activation.md` | v1.1 | v1.2 | Stale async/schema_v1 body text replaced in-place (F-P3-006) |
| `specs/architecture/SS-07-hook-bash.md` | v1.1 | v1.2 | Stale schema_v1 body text replaced in-place (F-P3-006) |

**Index bumps:**

| Index | Old | New | Notes |
|-------|-----|-----|-------|
| BC-INDEX | v1.21 | v1.22 | BC-1.14.001 title synced to H1; BC-1.08.002 title synced; BC-1.01.007 title synced; 3 other titles confirmed; changelog entry for pass-3 close |
| ARCH-INDEX | v1.12 → v1.13 (BC re-tally) | v1.14 (close) | BC counts by authoritative subsystem; ADR-019 v1.5; SS-09 v1.2; SS-07 v1.2 noted |
| VP-INDEX | v1.9 | v1.10 | VP-079 DI-019 placeholder resolved; VP-078 confirmed v1.5; VP-077 confirmed v1.4 |
| STATE.md | — | — | current_step, phase, cycle table, burst row, session checkpoint updated |

### Findings summary (7 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 4 | F-P3-002 (VP-079 Scenarios 1+4 untestable — sync plugin missing), F-P3-003 (POLICY 7 H1 sync missing), F-P3-005 (ARCH-INDEX BC re-tally wrong), F-P3-007 (VP-079 Scenario 4 no sync plugin) |
| MED | 3 | F-P3-001 (VP-078 Harness 3 incomplete list), F-P3-004 (VP-079 type drift), F-P3-006 (SS-09/SS-07 stale body text) |

### Status

F2 PASS-3 FIX BURST CLOSED. ADR-013 clock at 0_of_3. Adversary pass-4 next (3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst: F2 pass-4 fix burst close — 2026-05-07

**Role:** state-manager (closing burst)
**Pass result:** SUBSTANTIVE (6 findings; clock 0_of_3)
**Agents:** PO fix burst (BC-1.14.001 v1.5) + architect fix burst (ADR-019 v1.6; VP-077 v1.5; VP-078 v1.6; VP-INDEX v1.11) + state-manager close (BC-INDEX re-tally; BC-7.06.001 listing unification; VP-INDEX v1.12; ARCH-INDEX v1.15; STATE.md)

### Files modified

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `ss-01/BC-1.14.001.md` | v1.4 | v1.5 | PO: inline 100ms literals removed; PC4/EC-011/Traceability cite DI-019 by reference only (§Constant Reference rule) |
| `decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | v1.5 | v1.6 | Architect: §Consequences line 215 `+ 100ms` → `+ ASYNC_DRAIN_WINDOW_MS` symbolic constant (F-P4-004) |
| `specs/verification-properties/VP-077.md` | v1.4 | v1.5 | Architect: Traceability domain_invariants explanatory note added (F-P4-006) |
| `specs/verification-properties/VP-078.md` | v1.5 | v1.6 | Architect: frontmatter scope expanded SS-07 → SS-07, SS-01 (F-P4-003) |
| `specs/verification-properties/VP-INDEX.md` | v1.10 | v1.12 | Architect: VP-079 Domain Invariant column '—'→'DI-017, DI-019' (v1.11; F-P4-002). State-manager: Traceability summary 17→18 active invariants (DI-019; DI-018 deferred) (v1.12) |
| `specs/behavioral-contracts/BC-INDEX.md` | v1.22 | v1.23 | State-manager: Summary table SS-01 116→117, SS-05 648→652, SS-07 197→196, SS-08 218→214; BC-7.06.001 listing moved SS-07→SS-01 section; SS-01 header count updated; explanatory comment added (F-P4-001 HIGH) |
| `specs/architecture/ARCH-INDEX.md` | v1.14 | v1.15 | State-manager: ADR-019 row updated to v1.6; changelog entry added |

### Index bumps

| Index | Old | New | Notes |
|-------|-----|-----|-------|
| BC-INDEX | v1.22 | v1.23 | Re-tally complete; BC-7.06.001 listing convention unified to authoritative-frontmatter |
| ARCH-INDEX | v1.14 | v1.15 | ADR-019 v1.6 noted |
| VP-INDEX | v1.10 | v1.12 | v1.11 (architect): VP-079 DI column; v1.12 (state-manager close): DI-019 traceability summary |
| STATE.md | — | — | current_step, phase, cycle table, burst row, session checkpoint, index versions updated |

### Findings summary (6 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 1 | F-P4-001 (BC-INDEX vs ARCH-INDEX subsystem-count divergence; BC-7.06.001 convention mismatch) |
| MED | 2 | F-P4-002 (VP-INDEX VP-079 Domain Invariant column stale), F-P4-003 (VP-078 scope missing SS-01) |
| LOW | 2 | F-P4-004 (ADR-019 inline 100ms literal on line 215), F-P4-005 (BC-1.14.001 inline 100ms literals) |
| NIT | 1 | F-P4-006 (VP-077 domain_invariants traceability note absent) |

### BC-INDEX convention unification note

BC-7.06.001 listing moved from SS-07 section to SS-01 section in BC-INDEX (F-P4-001). This unifies the index to the authoritative-frontmatter convention: BCs whose frontmatter `subsystem:` differs from their directory are listed under their authoritative subsystem. The BC-8.29.001/002/003 + BC-8.30.002 rows (ss-08/ directory, SS-05 authoritative) established this convention; BC-7.06.001 (ss-07/ directory, SS-01 authoritative per F-P1-006 reanchor) now follows the same pattern. Filename slug immutable per POLICY 1.

### Defensive sweep results

Count-propagation sweep post-re-tally: BC-INDEX Summary table (SS-01 117, SS-05 652, SS-07 196, SS-08 214), ARCH-INDEX Subsystem Registry (already updated in pass-3 to authoritative counts), VP-INDEX (no BC counts), STATE.md (no per-subsystem BC count table). No stale counts found in BC-INDEX section headers (SS-07 header has no count annotation; SS-01 header updated from 116 to 117).

### Status

F2 PASS-4 FIX BURST CLOSED. ADR-013 clock at 0_of_3. Adversary pass-5 next (3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst: F2 pass-5 fix burst close — 2026-05-07

**Role:** state-manager (closing burst — all 3 findings are index/cross-reference syncs)
**Pass result:** SUBSTANTIVE (3 findings: 1 HIGH, 0 MED, 2 LOW, 0 NIT; clock 0_of_3)
**Trajectory:** 19→19→7→6→3 (improving)

### Files modified

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `specs/behavioral-contracts/BC-INDEX.md` | v1.23 | v1.24 | 4 BC-INDEX H1 syncs (POLICY 7 strict): BC-4.04.004/4.05.004 rows `; synchronous at envelope` → `and synchronous envelope`; BC-4.07.003/4.08.002 rows `synchronous at envelope` → `synchronous envelope`. Changelog entry added. |
| `specs/architecture/ARCH-INDEX.md` | v1.15 | v1.16 | BC-INDEX version cite v1.22→v1.24 (line 116). ADR-019 row updated to v1.7. Changelog entry added. |
| `specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | v1.6 | v1.7 | §References table: VP-079 row added for parity with §Implementation Pointers. Amendment block v1.6→v1.7 appended. |
| `cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-5.md` | new | v1.0 | Pass-5 findings persisted. |
| `STATE.md` | — | — | current_step, phase, cycle table, session checkpoint, index versions updated. |

### Findings summary (3 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 1 | F-P5-001 (POLICY 7: 4-BC sibling H1↔BC-INDEX title drift — pass-3 claim "confirmed matching" was incorrect) |
| MED | 0 | — |
| LOW | 2 | F-P5-002 (ARCH-INDEX stale BC-INDEX version cite v1.22), F-P5-003 (ADR-019 §References missing VP-079) |

### Decisions sealed

None. All 3 findings were unambiguous index/cross-reference syncs with no policy interpretation required.

## Lessons

Close-burst verification claims must be byte-for-byte grep, not visual inspection. F-P5-001 was a 4-BC drift that pass-3 claimed to fix but didn't (the verification was incomplete). The pass-3 changelog entry read `BC-4.04.004/4.05.004/4.07.003/4.08.002 confirmed matching` — this was written without running `grep` to compare H1 strings against BC-INDEX row text. Going forward: any close-burst that claims "N BC H1↔INDEX title syncs confirmed" must include a `grep` verification extracting both strings side-by-side before writing the changelog claim.

### Status

F2 PASS-5 FIX BURST CLOSED. ADR-013 clock at 0_of_3. Adversary pass-6 next (3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst: F2 pass-6 fix burst close — 2026-05-07

**Role:** state-manager (closing burst)
**Pass result:** SUBSTANTIVE (5 findings: 2H/2M/1L/0N; clock 0_of_3)
**Trajectory:** 19→19→7→6→3→5
**Agents:** PO fix burst (BC-3.08.001 v1.3; VP-078 v1.7; VP-079 v1.4) + architect fix burst (ADR-019 v1.8) + state-manager close (BC-INDEX v1.25; ARCH-INDEX v1.17; VP-INDEX v1.13; STATE.md)
**Verification discipline:** Byte-for-byte grep applied throughout (pass-5 lesson). All site counts verified before and after edits.

### Files modified

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `ss-03/BC-3.08.001.md` | v1.2 | v1.3 | PO: inline `100 ms` literal removed from Traceability section; cites DI-019 by reference (F-P6-003 MED; sibling-fix to F-P4-005) |
| `specs/verification-properties/VP-078.md` | v1.6 | v1.7 | PO: 8 events=[...] → event="..." sites (F-P6-001 HIGH, singular schema-correct); 7 script="X.sh" → plugin=adapter + [hooks.config] script_path in Rust unit tests (F-P6-002 HIGH, pass-2 missed Rust block); bats Harness 2 fixture TOML reordered top-level before [hooks.config] sub-table (F-P6-004 MED) |
| `specs/verification-properties/VP-079.md` | v1.3 | v1.4 | PO: 8 events=[...] → event="..." sites (F-P6-001 HIGH) |
| `decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` | v1.7 | v1.8 | Architect: §Consequences inline 100ms parenthetical removed; cites DI-019 by reference (F-P6-005 LOW) |
| `specs/behavioral-contracts/BC-INDEX.md` | v1.24 | v1.25 | State-manager: changelog entry for BC-3.08.001 v1.3 (no count change; no row text change needed — H1 unchanged) |
| `specs/architecture/ARCH-INDEX.md` | v1.16 | v1.17 | State-manager: ADR-019 row updated to v1.8; BC-INDEX version cite v1.24→v1.25; changelog entry added |
| `specs/verification-properties/VP-INDEX.md` | v1.12 | v1.13 | State-manager: VP-078/VP-079 row notes updated; changelog entry for pass-6 |
| `STATE.md` | — | — | current_step, Last Updated, Current Phase, Active Cycles table, Current Phase Steps (pass-6 row added), Session Checkpoint, ACTIVE STEP, INDEX versions updated |

### Index bumps

| Index | Old | New | Notes |
|-------|-----|-----|-------|
| BC-INDEX | v1.24 | v1.25 | BC-3.08.001 v1.3 noted; no count change |
| ARCH-INDEX | v1.16 | v1.17 | ADR-019 v1.8; BC-INDEX cite v1.25 |
| VP-INDEX | v1.12 | v1.13 | VP-078 v1.7; VP-079 v1.4 noted |

### Findings summary (5 total)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 2 | F-P6-001 (16 events=[...]→event="..." harness defects — partial-fix regression of F-P2-008); F-P6-002 (7 VP-078 Rust unit tests script="X.sh" — pass-2 missed Rust block; now closed) |
| MED | 2 | F-P6-003 (BC-3.08.001 inline `100 ms` literal in Traceability); F-P6-004 (VP-078 bats Harness 2 TOML scoping — top-level before sub-table) |
| LOW | 1 | F-P6-005 (ADR-019 §Consequences parenthetical inline 100ms) |

### Byte-for-byte verification log

Applied per pass-5 lesson (close-burst claims must be grep-verified, not visual):

- `events = [` outside CHANGELOG narrative: zero hits in VP-078, VP-079 after fix
- `script = "` in VP-078 outside CHANGELOG narrative: zero hits after fix
- `BC-3.08.001` version frontmatter: confirmed v1.3 post-amendment
- `ADR-019` version frontmatter: confirmed v1.8 post-amendment
- BC-INDEX v1.25 frontmatter: confirmed
- ARCH-INDEX v1.17 frontmatter: confirmed
- VP-INDEX v1.13 frontmatter: confirmed
- BC-INDEX row for BC-3.08.001: H1 title unchanged (no literal in H1 — literal was in Traceability body section only); row text confirmed matching H1 byte-for-byte

### Status

F2 PASS-6 FIX BURST CLOSED. ADR-013 clock at 0_of_3. Adversary pass-7 next (3 consecutive NITPICK_ONLY required before F3 story decomposition).

---

## Burst 8 — F2 pass-7 fix burst close

**Date:** 2026-05-07
**Phase:** F2 PASS-7 FIX BURST CLOSED
**Agents:** PO fix burst (VP-079 v1.5; BC-9.01.006 v1.1; BC-1.14.001 v1.6) + state-manager close (BC-INDEX v1.26; VP-INDEX v1.14; STATE.md)
**Verification discipline:** Byte-for-byte grep applied throughout. All site counts verified before and after edits.

**Convergence milestone:** FIRST PASS WITHOUT HIGH FINDINGS. Trajectory: 19→19→7→6→3→5→4.

### Files modified

| File | Old Version | New Version | Change |
|------|-------------|-------------|--------|
| `specs/verification-properties/VP-079.md` | v1.4 | v1.5 | PO: F-P7-001 — 9 inline `100ms` literals replaced with symbolic ASYNC_DRAIN_WINDOW_MS / DI-019 citations across Property 5, Scenario 1/4/5 comments, False-Positive table, Feasibility Assessment, Traceability; F-P7-003 — stale "BC-1.14.001 v1.4 traceability" cite at Property 5 → "BC-1.14.001 PC4 traceability" (stable semantic anchor) |
| `specs/behavioral-contracts/ss-09/BC-9.01.006.md` | v1.0 | v1.1 | PO: F-P7-002 — inputs frontmatter path `.factory/specs/architecture/ADR-019.md` corrected to canonical `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` (stale path never existed); cosmetic only |
| `specs/behavioral-contracts/ss-01/BC-1.14.001.md` | v1.5 | v1.6 | PO: F-P7-004 — redundant `(per DI-019;` parenthetical removed from L2 Domain Invariants DI-019 cell in Traceability; cosmetic only |
| `specs/behavioral-contracts/BC-INDEX.md` | v1.25 | v1.26 | State-manager: changelog entry for BC-9.01.006 v1.1 + BC-1.14.001 v1.6 (no count change; H1 titles unchanged — no row text update needed) |
| `specs/verification-properties/VP-INDEX.md` | v1.13 | v1.14 | State-manager: VP-079 row description updated to v1.5 summary; changelog entry for pass-7 |
| `STATE.md` | — | — | current_step, Last Updated, Current Phase, Active Cycles table, Phase Progress row, Current Phase Steps (pass-7 row added), Session Checkpoint, ACTIVE STEP, INDEX versions updated |

### Index bumps

| Index | Old | New | Notes |
|-------|-----|-----|-------|
| BC-INDEX | v1.25 | v1.26 | BC-9.01.006 v1.1 + BC-1.14.001 v1.6 noted; no count change |
| ARCH-INDEX | v1.17 | v1.17 | No change — ADR-019 not touched in pass-7 |
| VP-INDEX | v1.13 | v1.14 | VP-079 v1.5 row updated; changelog entry added |

### Findings summary (4 total — FIRST PASS WITHOUT HIGH)

| Severity | Count | Notable |
|----------|-------|---------|
| HIGH | 0 | First pass without HIGH findings — convergence milestone |
| MED | 2 | F-P7-001 (VP-079 9 inline 100ms literals — DI-019-canonical-home principle not previously applied to VP-079); F-P7-002 (BC-9.01.006 stale ADR-019 inputs path) |
| LOW | 0 | — |
| NIT | 2 | F-P7-003 (VP-079 stale v1.4 version-anchored cite → PC4 stable anchor); F-P7-004 (BC-1.14.001 redundant parenthetical — cosmetic) |

### Byte-for-byte verification log

Applied per pass-5 lesson (close-burst claims must be grep-verified, not visual):

- `100ms` in VP-079 live body (excluding CHANGELOG/Amendment sections): zero hits confirmed
- `ASYNC_DRAIN_WINDOW_MS` present at all 9 former literal sites in VP-079: confirmed
- `BC-1.14.001 v1.4` version-anchored cite in VP-079 live body: zero hits (replaced with PC4 anchor)
- `ADR-019.md` stale path in BC-9.01.006 inputs frontmatter: zero hits (corrected to canonical decisions/ path)
- `decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` file exists at cited path: confirmed
- `(per DI-019;` redundant pattern in BC-1.14.001 live body: zero hits (removed)
- BC-9.01.006 version frontmatter: confirmed v1.1 post-amendment
- BC-1.14.001 version frontmatter: confirmed v1.6 post-amendment
- VP-079 version frontmatter: confirmed v1.5 post-amendment
- BC-INDEX v1.26 frontmatter: confirmed
- VP-INDEX v1.14 frontmatter: confirmed
- BC-INDEX rows for BC-9.01.006 + BC-1.14.001: H1 titles unchanged (path fix in frontmatter only; parenthetical in body Traceability section only); row texts confirmed matching H1 byte-for-byte

### Status

F2 PASS-7 FIX BURST CLOSED. ADR-013 clock at 0_of_3. Adversary pass-8 next (3 consecutive NITPICK_ONLY required before F3 story decomposition). First pass without HIGH findings — DI-019-canonical-home principle now propagated to all 4 citing artifacts (BC-1.14.001, BC-3.08.001, ADR-019, VP-079).

---

## Burst: STATE.md compaction archive — F2 spec evolution + passes 1–6 fix bursts (archived 2026-05-07 for F3 burst row)

The following step rows were in STATE.md before F3 compaction and are archived here to keep STATE.md under the 200-line budget:

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| **F2 spec evolution — plugin-async-semantics-pass-1** | product-owner + architect + state-manager | **COMPLETE** | ADR-019 accepted (async semantics at registry layer; hard cut, no backcompat). BC-1.14.001 (dispatcher partition contract). BC-7.06.001 (registry schema v2 + CI lint invariant). VP-077 (partition correctness, Kani). VP-078 (CI lint invariant, integration). 7 BCs amended (envelope sync; schema v2 gates). 2 VPs amended (scope to sync group). BC-INDEX 1.18→1.19; ARCH-INDEX 1.9→1.10; VP-INDEX 1.5→1.6. Adversarial convergence next (≥3 NITPICK_ONLY). |
| **F2 pass-1 fix burst close — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 19 adversary findings addressed across 4 specialist bursts (PO ∥ architect → architect-followup → state-manager-close). New: BC-9.01.006 (SS-09), BC-3.08.001 (SS-03), VP-079. Amended: BC-1.14.001 v1.1, BC-7.06.001 v1.1 (subsystem SS-01), BC-1.08.001 v1.1, DI-014 v1.3, ADR-019 v1.1, SS-09 v1.1, SS-07 v1.1, VP-077 v1.2, VP-078 v1.3. INDEX bumps: BC-INDEX v1.20 (1947 total), ARCH-INDEX v1.11, VP-INDEX v1.7 (79 total). Sealed: schema-mismatch fail-CLOSED; BC-7.06.001 primary SS-01; async lifetime best-effort. Adversary pass-2 next. |
| **F2 pass-2 fix burst close — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 19 adversary findings addressed (1 SKIP_FIX F-P2-019). PO: BC-7.06.001 v1.2 (Invariant 7 tuple-unique; Invariant 6 → 9 plugins; PC3 reword), BC-1.14.001 v1.2 (PCs renumbered; Error Paths; PC4 pin), BC-4.04.004 v2.1 + BC-4.05.004 v2.1 (PC7→Inv6 ref), BC-4.07.003 v1.3 (body fix), BC-3.08.001 v1.1 (SS-07→SS-01), BC-1.08.001 v1.2 (Stories). DI-014 v1.4 (BC range reword). Architect: ADR-019 v1.3 (§Consequences sync; SYNC/ASYNC rationale). State-manager: VP-077 v1.4 (Invariant 7 forward-ref). INDEX bumps: BC-INDEX v1.21, ARCH-INDEX v1.12, VP-INDEX v1.8. Sealed decisions: F-P2-006 (9 plugins ASYNC; warn-pending-wave-gate/regression-gate SYNC); F-P2-007 (no CLI flags; stdin envelope + env vars); F-P2-011 (VP-077 6 properties canonical). Pass-3 next. |
| **F2 pass-3 fix burst close + user-correction — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 7 findings + 2 user-correction revisions. New: DI-019 (ASYNC_DRAIN_WINDOW_MS=100ms; SS-01 enforcement). User-correction Q2: ARCH-INDEX BC re-tally to authoritative frontmatter subsystem (SS-01 +1, SS-05 +4, SS-07 −1, SS-08 −4; total 1,947 unchanged). User-correction Q3: ASYNC_DRAIN_WINDOW_MS lifted from BC-1.14.001 inline to DI-019 domain invariant. DI-NN placeholder resolved to DI-019 in VP-079 + ADR-019. BC-1.14.001 v1.4, BC-3.08.001 v1.2, VP-078 v1.5, VP-079 v1.3, ADR-019 v1.5, SS-09 v1.2, SS-07 v1.2. 6 BC-INDEX H1 syncs (POLICY 7). INDEX bumps: BC-INDEX v1.22, ARCH-INDEX v1.14, VP-INDEX v1.10. ADR-013 clock at 0_of_3. Pass-4 next. |
| **F2 pass-4 fix burst close — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 6 findings closed (F-P4-001 HIGH BC-INDEX re-tally; F-P4-002/003 VP-INDEX propagation; F-P4-004/005 symbolic constants; F-P4-006 documentation note). BC-INDEX re-tallied: SS-01 116→117, SS-05 648→652, SS-07 197→196, SS-08 218→214 (total 1947 unchanged). BC-7.06.001 listing unified SS-07→SS-01 section (authoritative-frontmatter convention; filename slug retained ss-07/ POLICY 1). BC-1.14.001 v1.4→v1.5 (inline 100ms literals removed). ADR-019 v1.5→v1.6 (symbolic ASYNC_DRAIN_WINDOW_MS). VP-077 v1.4→v1.5; VP-078 v1.5→v1.6. VP-INDEX v1.11→v1.12 (DI-019 traceability updated). BC-INDEX v1.22→v1.23; ARCH-INDEX v1.14→v1.15. ADR-013 clock at 0_of_3. Pass-5 next. |
| **F2 pass-5 fix burst close — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 3 findings closed. F-P5-001 HIGH POLICY 7: 4-BC sibling H1↔BC-INDEX drift (BC-4.04.004/4.05.004/4.07.003/4.08.002). BC-INDEX rows synced to H1s byte-for-byte (`synchronous envelope`; `and synchronous` for two of four). Pass-3 "confirmed matching" claim was incorrect — byte-for-byte grep not performed. F-P5-002 LOW: ARCH-INDEX BC-INDEX version cite v1.22→v1.24. F-P5-003 LOW: ADR-019 §References VP-079 row added; ADR-019 v1.6→v1.7. BC-INDEX v1.23→v1.24; ARCH-INDEX v1.15→v1.16. ADR-013 clock at 0_of_3. Pass-6 next. |
| **F2 pass-6 fix burst close — plugin-async-semantics-pass-1** | state-manager | **COMPLETE** | 5 findings closed. F-P6-001 HIGH: 16 events=[...] → event="..." sites (VP-078 8 + VP-079 8). F-P6-002 HIGH: 7 VP-078 Rust unit tests script="X.sh" → plugin=adapter + [hooks.config] script_path. F-P6-003 MED: BC-3.08.001 inline `100 ms` removed; cites DI-019 (v1.2→v1.3). F-P6-004 MED: VP-078 bats Harness 2 TOML reordered. F-P6-005 LOW: ADR-019 §Consequences 100ms parenthetical removed; cites DI-019 (v1.7→v1.8). Byte-for-byte grep verification applied. BC-INDEX v1.25; ARCH-INDEX v1.17; VP-INDEX v1.13. ADR-013 clock at 0_of_3. Pass-7 next. |

---

## Burst 9 — F2 pass-8 close — first NITPICK_ONLY of cycle; clock advances 0→1_of_3

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager

### Summary

Pass-8 returned NITPICK_ONLY (first such pass for this cycle). ADR-013 clock advances 0→1_of_3. Trajectory: 19→19→7→6→3→5→4→1 (NIT).

### Findings addressed

- **NIT-P8-001** (ARCH-INDEX BC-INDEX version-cite drift): ARCH-INDEX line 120 `per BC-INDEX v1.25` refreshed to `per BC-INDEX v1.26`. Recurring drift — 3rd refresh of this cite (pass-5 v1.22→v1.24; pass-6 v1.24→v1.25; pass-8 v1.25→v1.26). Adversary findings persisted to `adversary-pass-8.md`.

### Decision

ARCH-INDEX BC-INDEX cite refreshed; future close-burst protocol must include ARCH-INDEX cite refresh whenever BC-INDEX version bumps (process enhancement noted in ARCH-INDEX changelog v1.18).

### Files touched

- `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-8.md` — created (pass-8 findings)
- `.factory/specs/architecture/ARCH-INDEX.md` — v1.17→v1.18 (NIT-P8-001 fix + changelog entry)
- `.factory/STATE.md` — current_step, last_amended, phase progress, concurrent cycles, session checkpoint, INDEX versions

### Index versions at close

BC-INDEX: v1.26 (no change) | ARCH-INDEX: v1.17→v1.18 | VP-INDEX: v1.14 (no change)

### Status

F2 PASS-8 CLOSED. ADR-013 clock 1_of_3. Pass-9 next. Need 2 more consecutive NITPICK_ONLY for CONVERGENCE_REACHED.

---

## Burst 10 — F2 pass-9 close — NITPICK_ONLY; clock advances 1→2_of_3; both NITs SKIP_FIX per ADR-013

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager

### Summary

Pass-9 returned NITPICK_ONLY (second consecutive NITPICK_ONLY pass for this cycle). ADR-013 clock advances 1→2_of_3. Trajectory: 19→19→7→6→3→5→4→1→2 (NIT). One more NITPICK_ONLY = CONVERGENCE_REACHED.

### Findings — both SKIP_FIX per ADR-013

| ID | File | Severity | Decision | Rationale |
|----|------|----------|----------|-----------|
| NIT-P9-001 | ADR-019.md line 173 | NIT | SKIP_FIX | "30-100ms" numeric coincidence with DI-019 value (cosmetic; latency estimate range, not the drain constant; no defect) |
| NIT-P9-002 | VP-079.md line ~421 | NIT | SKIP_FIX | Scenario 5 `timeout_ms=200` implicit DI-019 dependency (cosmetic hardening for future-proofing; no defect today) |

### Files touched

- `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-9.md` — created (pass-9 findings persisted)
- `.factory/STATE.md` — current_step, last_amended, phase progress row, concurrent cycles row, current phase steps (pass-9 row added, D-362 row archived here), session checkpoint updated

### No spec file modifications

Both NITs are SKIP_FIX. No spec files modified. No index bumps.

### Archived step row (from STATE.md Current Phase Steps)

The following step row was archived from STATE.md to this burst-log to maintain the 200-line budget:

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| **D-362 F2-amendment integration burst** | state-manager | **COMPLETE** | 6 new BCs (BC-1.13.001 SS-01; BC-4.12.001-005 SS-04). ADR-018. 4 new VPs (VP-073-076). PRD 1.1→1.2 (FR-048). F-P2-002 fix (BC-4.10.001 v1.1 + BC-5.39.001 v1.1). BC-INDEX 1.17→1.18 (total_bcs 1937→1943; SS-01 114→115, SS-04 34→39). ARCH-INDEX 1.8→1.9 (ADR-018 added). VP-INDEX 1.4→1.5 (total_vps 72→76). Next: F3-amendment story authoring (S-12.03-S-12.08 under E-12). |

### Convergence status

ADR-013 clock 2_of_3. One more NITPICK_ONLY pass required for CONVERGENCE_REACHED.

### Status

F2 PASS-9 CLOSED. ADR-013 clock 2_of_3. Pass-10 next.

---

## Burst 11 — F2 CONVERGENCE close — pass-10 NITPICK_ONLY; clock 2→3_of_3; CONVERGENCE_REACHED

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager (closing burst)
**Phase:** F2 CONVERGED

### Summary

Pass-10 returned NITPICK_ONLY (third consecutive for this cycle). ADR-013 clock advances 2→3_of_3. Three consecutive NITPICK_ONLY passes = CONVERGENCE_REACHED per ADR-013.

Trajectory across all 10 passes + 7 fix bursts: **19 → 19 → 7 → 6 → 3 → 5 → 4 → 1 → 2 → 1**.

### NIT-P10-001 — Fix applied

- **File:** BC-3.08.001.md line 196 (Traceability §L2 Domain Invariants cell)
- **Fix:** Redundant `(per DI-019)` parenthetical removed. Cell already began with `DI-019 —` prefix. Sibling fix to F-P7-004 (which fixed the same pattern in BC-1.14.001 v1.5→v1.6 during pass-7).
- **Version bump:** BC-3.08.001 v1.3 → v1.4. Amendment section added.

### Files touched

| File | Change |
|------|--------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-10.md` | Created — pass-10 findings persisted (verdict: NITPICK_ONLY; clock 3_of_3; CONVERGENCE_REACHED) |
| `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` | v1.3→v1.4 — NIT-P10-001 cleanup; amendment section added |
| `.factory/specs/behavioral-contracts/BC-INDEX.md` | v1.26→v1.27 — BC-3.08.001 v1.4 noted; changelog entry added |
| `.factory/specs/architecture/ARCH-INDEX.md` | v1.18→v1.19 — BC-INDEX cite refreshed v1.26→v1.27; changelog entry added |
| `.factory/STATE.md` | current_step, last_amended, phase progress, concurrent cycles, current phase steps, session checkpoint, index versions updated |

### Index versions at F2 convergence close

| Index | Version |
|-------|---------|
| BC-INDEX | v1.27 |
| ARCH-INDEX | v1.19 |
| VP-INDEX | v1.14 (no change) |

### Final F2 Statistics

| Metric | Value |
|--------|-------|
| Total adversary passes | 10 |
| SUBSTANTIVE passes | 7 (passes 1–7) |
| NITPICK_ONLY passes | 3 (passes 8, 9, 10 — final ADR-013 clock chain) |
| Fix bursts | 7 (pass-1, pass-2, pass-3, pass-3-user-correction, pass-4, pass-5, pass-6, pass-7 — 7 distinct fix bursts) |
| Finding trajectory | 19→19→7→6→3→5→4→1→2→1 |
| Final ADR-013 clock | 3_of_3 (CONVERGENCE_REACHED) |
| New BCs | 5 (BC-1.14.001, BC-7.06.001, BC-9.01.006, BC-3.08.001, BC-1.08.001 amendment exception) |
| New ADR | 1 (ADR-019) |
| New VPs | 3 (VP-077, VP-078, VP-079) |
| New DI | 1 (DI-019) |
| Amended BCs | 9 (BC-1.01.001, BC-1.01.007, BC-1.08.001, BC-1.08.002, BC-4.04.004, BC-4.05.004, BC-4.07.003, BC-4.08.002, plus retroactive cite updates) |
| Amended VPs | 2 (VP-001, VP-002) |
| Amended SS docs | 2 (SS-09, SS-07) |

### User-locked decisions sealed

1. Every Claude Code hook event sync at envelope (no carve-outs) — all 4 envelope BCs amended
2. No backwards compatibility — v2 dispatcher hard-errors on v1 registry (ADR-019; BC-1.08.001 fail-closed exception)
3. No phased rollout — single story consolidating all changes (ADR-019 §6)
4. ASYNC_DRAIN_WINDOW_MS = 100ms via DI-019 (lifted from BC to domain layer; propagated to all 4 citing artifacts)

### Lessons captured

| Lesson | Manifestation | Codification |
|--------|--------------|--------------|
| Byte-for-byte grep verification beats visual inspection | Pass-3 claimed "confirmed matching" without grep; pass-5 found 4-BC H1↔BC-INDEX drift | S-7.01 discipline applied from pass-5 onward |
| Sibling-fix propagation: amendments to one artifact must scan every citing sibling | F-P7-004 fixed BC-1.14.001 parenthetical but missed BC-3.08.001 sibling — caught at pass-10 | NIT-P10-001 |
| User-correction principle: choose architecturally correct over expedient | Q2 (ARCH-INDEX re-tally to authoritative subsystem); Q3 (DI-019 lift from BC to domain layer) | ADR-019 + DI-019 in spec package |
| Recurring NIT codification trigger at 3+ recurrences | ARCH-INDEX BC-INDEX cite drift refreshed 4 times (passes 5/6/8/10) | Burst-close protocol: any BC-INDEX version bump triggers ARCH-INDEX cite refresh |

### Next phase

F3 story decomposition (1 consolidated story per ADR-019 §6; covers schema v2 + dispatcher partition + plugin classification + envelope flip + CI lint). Gated behind human approval.

### Status

F2 CONVERGED. Human approval gate pending. F3 story decomposition queued.

---

## Burst 12 — F3 story decomposition — E-15 epic + S-15.01 story authored (2026-05-07)

**Date:** 2026-05-07
**Dispatchers:** orchestrator → product-owner (E-15 epic) → story-writer (S-15.01 story) → state-manager (close)
**Phase:** F3 STORIES AUTHORED → adversarial convergence pending

### Summary

Human approval gate cleared. F3 story decomposition executed per ADR-019 §6 user single-shot decision: 1 consolidated story covering all 5 implementation tracks.

Trajectory to here: F2 CONVERGENCE_REACHED at pass-10 (commit 3568657). ADR-013 clock 3_of_3. F3 now opens adversarial convergence gate (≥3 NITPICK_ONLY per ADR-013 before F4).

### Outputs

| File | Producer | Notes |
|------|----------|-------|
| `.factory/stories/epics/E-15-plugin-async-semantics.md` | product-owner | E-15 epic (draft, v1.0, 200L; 1 story per ADR-019 §6 single-shot; E-12/13/14 already taken by engine-discipline cycle; E-15 is next free per POLICY 1 append-only) |
| `.factory/stories/S-15.01-plugin-async-semantics.md` | story-writer | S-15.01 story (draft, v1.0, 765L; XL/13 points; 17 ACs; tdd_mode: strict) |

### Index updates

| Index | Old | New | Change |
|-------|-----|-----|--------|
| STORY-INDEX | v2.30 | v2.31 | E-15 epic row + S-15.01 story row appended; total_stories 90→91 |
| BC-INDEX | v1.27 | v1.28 | Stories field updated for 12 BCs (5 primary + 7 secondary) to include S-15.01 |
| VP-INDEX | v1.14 | v1.15 | Story Anchors section: VP-077/078/079 anchored to S-15.01 |

### Story details — S-15.01

| Field | Value |
|-------|-------|
| **Story ID** | S-15.01 |
| **Title** | Plugin async semantics — full implementation (schema v2 + dispatcher partition + classification + envelope flip + CI lint) |
| **Epic** | E-15 |
| **Points** | 13 (XL) |
| **Priority** | P1 |
| **tdd_mode** | strict |
| **ACs** | 17 |
| **Primary BCs** | BC-1.14.001, BC-7.06.001, BC-9.01.006, BC-3.08.001, BC-1.08.001 |
| **Secondary BCs** | BC-1.01.001, BC-1.01.007, BC-1.08.002, BC-4.04.004, BC-4.05.004, BC-4.07.003, BC-4.08.002 |
| **VPs** | VP-077, VP-078, VP-079, VP-001 (amended), VP-002 (amended) |
| **DIs** | DI-014, DI-019 |
| **Subsystems** | SS-01, SS-04, SS-07, SS-09 |
| **File list** | 25 paths (registry.rs schema v2, dispatcher partition, hooks-registry.toml, 5 platform hooks.json variants, pre-commit lint hook, demo evidence dir) |

### Implementation tracks consolidated (per ADR-019 §6)

| Track | Scope |
|-------|-------|
| Schema v2 migration | `hooks-registry.toml` schema_version 2; per-plugin `async: bool` field; `REGISTRY_SCHEMA_VERSION` constant |
| Dispatcher partition runtime | `sync_group` / `async_group` split; `ASYNC_DRAIN_WINDOW_MS` fire-and-forget drain |
| Plugin classification | 9 telemetry plugins → `async = true`; classify in production `hooks-registry.toml` |
| hooks.json envelope flip | 5 platform hooks.json variants: `async: true` removed from all event entries |
| CI lint invariant | 3 enforcement layers: pre-commit hook + load-time validation + CI gate |

### Decisions sealed

- E-15 ID assigned (E-12/13/14 already taken by engine-discipline cycle; E-15 is next free per POLICY 1 append-only)
- S-15.01 consolidates all 5 implementation tracks per ADR-019 §6 user single-shot decision
- POLICY 8 BC propagation verified: 5 primary BCs in frontmatter, body BC table, 17 ACs (every AC has at least one BC trace), Token Budget subtable

### Status

F3 STORIES AUTHORED. Awaiting adversarial convergence (≥3 NITPICK_ONLY per ADR-013) before F4 TDD implementation.

---

## Burst 2 — F3 Pass-1 Fix Burst (2026-05-07)

**Agents:** story-writer + state-manager
**Trigger:** F3 adversary pass-1 returned SUBSTANTIVE on S-15.01 (9 findings: 4 HIGH, 2 MED, 2 LOW, 1 NIT)
**Story version:** S-15.01 v1.0 → v1.1

### Findings addressed

| ID | Severity | Finding | Resolution |
|----|----------|---------|------------|
| F-P1-001 | HIGH | BC body titles not byte-for-byte synced to BC H1s (POLICY 4/6 violation) | BC-4.04/05/07.003 body titles corrected to match H1s exactly: SessionStart, SessionEnd, WorktreeCreate/Remove — not PostToolUse/Stop/SubagentStop |
| F-P1-002 | HIGH | Secondary BC versions incorrect (POLICY 8 violation) | BC-4.04/05.004 v1.1→v2.1; BC-4.07.003/4.08.002 v1.1→v1.3 |
| F-P1-003 | HIGH | subsystems frontmatter array missing SS-03 (anchor for BC-3.08.001) | SS-03 added to subsystems array in S-15.01 frontmatter |
| F-P1-004 | HIGH | AC-010 stated "all 10" but only 9 events; PermissionRequest no-op not clarified (POLICY 7 violation) | AC-010 reworded "all 9"; explicit PermissionRequest no-op clarification added |
| F-P1-005 | MED | SS-03 and SS-04 anchor justification blocks missing | Anchor justification blocks added for SS-03 (BC-3.08.001) and SS-04 (BC-9.01.006) |
| F-P1-006 | MED | VP-077 property-to-harness mapping table missing | 6-property × 4-harness mapping table added to VP-077 section |
| F-P1-007 | LOW | event_catalog.rs listed as NEW instead of host/emit_event.rs amend per ARCH-INDEX | Corrected to host/emit_event.rs amend per ARCH-INDEX authoritative event-emission location |
| F-P1-008 | LOW | Pre-commit hook described as traditional git hook; S-13.01 precedent (Claude Code PostToolUse) not cited | Clarified as Claude Code PostToolUse mechanism despite "pre-commit" naming; S-13.01 precedent cited |
| F-P1-009 | NIT | Token Budget table missing VP-001/VP-002 amendment costs | Token Budget table updated with VP-001 and VP-002 amendment costs |

### Verification discipline applied

Byte-for-byte grep verification applied per pass-5/6 cycle lessons (TD-VSDD-058/059 pre-commit checklists). Each BC title correction verified via grep against BC H1 source files. Secondary BC versions verified against BC frontmatter. SS-03 anchor verified in ARCH-INDEX.

### Artifacts touched

- `.factory/stories/S-15.01-plugin-async-semantics.md` — v1.0 → v1.1 (body: BC title corrections, AC-010 reword, SS-03/SS-04 anchors, VP-077 table, host/emit_event.rs correction, pre-commit mechanism clarification, Token Budget table update)
- `.factory/stories/STORY-INDEX.md` — v2.31 → v2.32 (S-15.01 row updated to v1.1; last_amended updated)
- `.factory/STATE.md` — current_step, phase progress, concurrent cycles, session checkpoint updated
- `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` — this entry appended

### Status

F3 PASS-1 FIX BURST CLOSED. ADR-013 clock 0_of_3. Awaiting F3 adversary pass-2.

---

## Burst 3 — F3 Pass-2 Fix Burst: Option C → Option A (WASM-migration directive) (2026-05-07)

**Agents:** story-writer + architect (investigation) + state-manager
**Trigger:** F3 adversary pass-2 returned SUBSTANTIVE; architect investigated F-P2-001 (F-P2-001-mechanism-investigation.md). User intervened: "we are migrating to WASM, any new plugins need to use WASM." Option C (bash via legacy-bash-adapter) invalidated.
**Story version:** S-15.01 v1.1 → v1.3

### Findings addressed

| ID | Severity | Finding | Resolution |
|----|----------|---------|------------|
| F-P2-001 | HIGH | CI lint mechanism underspecified (bash vs WASM unclear) | Option A: native WASM plugin (Rust crate at `crates/hook-plugins/lint-registry-async-invariant/`, `.wasm` artifact, registered in `hooks-registry.toml` with `plugin=` field, NOT `legacy-bash-adapter`). S-15.01 v1.1→v1.2. |
| F-P2-002 | MED | File list stale after mechanism change | 30-path file list updated in S-15.01 tasks. v1.2→v1.3 (editorial). |
| F-P2-003 | LOW | BC-7.06.001 PostToolUse Edit|Write wording imprecise | Postcondition 7 + Invariant 5 Layer 1: "Pre-commit hook fails commit" → "Claude Code PostToolUse Edit|Write hook blocks tool call before edit lands". BC-7.06.001 v1.2→v1.3. |

### Artifacts touched

- `.factory/stories/S-15.01-plugin-async-semantics.md` — v1.1 → v1.3
- `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` — v1.2 → v1.3
- `.factory/stories/STORY-INDEX.md` — v2.32 → v2.33 (S-15.01 row updated to v1.3)
- `.factory/specs/behavioral-contracts/BC-INDEX.md` — v1.28 → v1.29 (BC-7.06.001 v1.3 changelog entry)
- `.factory/STATE.md` — current_step, phase progress, concurrent cycles, session checkpoint updated
- `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` — this entry appended
- `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F-P2-001-mechanism-investigation.md` — architect investigation report (untracked → committed)

### Side flag

BC-9.01.006 line 61 has similar pre-commit wording — clarification candidate but not blocking (noted in commit message).

### Status

F3 PASS-2 FIX BURST CLOSED. ADR-013 clock 0_of_3. Awaiting F3 adversary pass-3.

---

## Burst 4 — WASM-Rule Retroactive Audit Fix Burst (2026-05-07)

**Date:** 2026-05-07
**Dispatchers:** orchestrator → architect (audit) → state-manager
**Phase:** F3 ADV CONVERGENCE — inter-pass maintenance

**Trigger:** User directive: "make sure we keep that rule for everything we have done" (WASM-migration rule). Architect conducted retroactive audit of all session work against BC-7.06.001 v1.3 PostToolUse Edit|Write wording. Mirror of BC-7.06.001 v1.3 fix applied session-wide.

**Audit result:** 2 HIGH violations + 1 MEDIUM test-fixture question found.
- ADR-019 verified clean (its "pre-commit" is generic/conceptual — no fix needed).

### Findings addressed

| Severity | Artifact | Finding | Resolution |
|----------|----------|---------|------------|
| HIGH | BC-9.01.006 | Error Paths row 1 + EC-002 still used "Pre-commit hook fails" wording from v1.1; Layer 1 did not reflect PostToolUse Edit|Write semantics | v1.1→v1.2: Error Paths row 1 + EC-002 reworded to "Claude Code PostToolUse Edit|Write hook blocks tool call"; Layer 1 wording aligned |
| HIGH | VP-078 | Line 50-51 said "bash script or bats test" (should be "native WASM plugin"); lines 300+441 said "pre-commit" (should be "PostToolUse Edit|Write") | v1.7→v1.8: 3 wording fixes applied |
| MEDIUM | VP-078 + VP-079 | Test harness uses legacy-bash-adapter for fixture purposes — could be misread as WASM-rule violation | Test-fixture annotations added to both VPs explaining legacy-bash-adapter usage is transitional test infrastructure, not a new bash plugin |

### Artifacts touched

| File | Change |
|------|--------|
| `.factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md` | v1.1 → v1.2 |
| `.factory/specs/verification-properties/VP-078.md` | v1.7 → v1.8 |
| `.factory/specs/verification-properties/VP-079.md` | v1.5 → v1.6 |
| `.factory/specs/behavioral-contracts/BC-INDEX.md` | v1.29 → v1.30 |
| `.factory/specs/verification-properties/VP-INDEX.md` | v1.15 → v1.16 |
| `.factory/STATE.md` | current_step, last_updated, concurrent cycle row, session checkpoint, index versions updated |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | this entry appended |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/wasm-rule-audit.md` | untracked audit report committed |

### Notes

- F2 convergence not impacted — these are informational v-bumps; findings did not exist during F2 passes (wording was introduced in BC-7.06.001 v1.3, which was fixed in Burst 3)
- ADR-019 "pre-commit" text is generic/conceptual; verified clean, no change
- F3 adversary pass-3 proceeds with WASM-rule conformance now verified across all session artifacts

### Status

WASM-RULE AUDIT FIX BURST CLOSED. ADR-013 clock 0_of_3. Awaiting F3 adversary pass-3.

---

## Burst 5 — F3 Pass-3 NITPICK_ONLY Close Burst (2026-05-07)

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager
**Phase:** F3 ADV CONVERGENCE — NITPICK_ONLY pass-3 closed
**Story version:** S-15.01 v1.3 → v1.4

### Summary

Pass-3 returned NITPICK_ONLY (first such pass in F3). ADR-013 clock advances 0→1_of_3. F3 trajectory: 9→3→3(NIT). Two more NITPICK_ONLY = CONVERGENCE_REACHED.

Pass-3 adversary findings persisted at `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-3.md`.

### Findings addressed

| ID | Severity | Finding | Resolution |
|----|----------|---------|------------|
| NIT-P3-001 | NIT | Body BC table cites stale versions: BC-7.06.001 v1.2, BC-9.01.006 v1.1 | Updated to v1.3 and v1.2 respectively (reflect WASM-rule audit fix burst versions) |
| NIT-P3-002 | NIT | References table cites stale VP versions: VP-078 v1.7, VP-079 v1.5 | Updated to v1.8 and v1.6 respectively (reflect WASM-rule audit fix burst versions) |
| NIT-P3-003 | NIT | Observation: pass-2 fix burst bundled title sync (F-P2-003) but not version sync — sibling-fix gap | Lesson captured (see below); NIT-P3-001 fix closes the gap |

### Lesson captured

**NIT-P3-003:** Version sync should ride alongside title sync in the same fix burst. When a BC title is updated (e.g., F-P2-003 synced BC-7.06.001 title byte-for-byte), the cited version in every story BC table referencing that BC should be refreshed in the same burst. Separating title sync from version sync creates a sibling-fix gap that surfaces as a NIT in the next adversary pass.

### Verification

- `BC-7.06.001 | v1.2` in body BC table: zero hits (replaced with v1.3)
- `BC-9.01.006 | v1.1` in body BC table: zero hits (replaced with v1.2)
- `VP-078 v1.7` in References table: zero hits (replaced with v1.8)
- `VP-079 v1.5` in References table: zero hits (replaced with v1.6)
- S-15.01 frontmatter version: v1.4 confirmed

### Artifacts touched

| File | Change |
|------|--------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-3.md` | Created — pass-3 findings persisted (NITPICK_ONLY; clock 1_of_3) |
| `.factory/stories/S-15.01-plugin-async-semantics.md` | v1.3 → v1.4 (4 version label refreshes + amendment section) |
| `.factory/stories/STORY-INDEX.md` | v2.33 → v2.34 (S-15.01 row updated to v1.4; clock 1_of_3) |
| `.factory/STATE.md` | current_step, last_amended, phase progress, concurrent cycles, current phase steps, session checkpoint, index versions updated |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | this entry appended |

### Status

F3 PASS-3 CLOSED. ADR-013 clock 1_of_3. Awaiting F3 adversary pass-4. Two more NITPICK_ONLY = CONVERGENCE_REACHED.

---

## Burst 7 — F3 pass-4 NITPICK_ONLY close burst

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager
**Phase:** F3 ADV CONVERGENCE — NITPICK_ONLY pass-4 closed
**Story version:** S-15.01 v1.4 → v1.5

### Summary

Pass-4 returned NITPICK_ONLY (second consecutive). ADR-013 clock advances 1→2_of_3. F3 trajectory: 9→3→3→1(NIT). ONE MORE NITPICK_ONLY = CONVERGENCE_REACHED. Pass-5 is the potential convergence pass.

Pass-4 adversary findings persisted at `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-4.md`.

### Findings addressed

| ID | Severity | Finding | Resolution |
|----|----------|---------|------------|
| NIT-P4-001 | NIT | References table BC version labels stale: BC-7.06.001 v1.2, BC-9.01.006 v1.1 | Updated to v1.3 and v1.2 respectively. Sibling completion of pass-3 NIT-P3-001 (which fixed body BC table but not References table). |

### Verification

- `BC-7.06.001 v1.2` in References table: zero hits (replaced with v1.3)
- `BC-9.01.006 v1.1` in References table: zero hits (replaced with v1.2)
- S-15.01 frontmatter version: v1.5 confirmed
- ADR-013 clock notation: 2_of_3

### Artifacts touched

| File | Change |
|------|--------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-4.md` | Created — pass-4 findings persisted (NITPICK_ONLY; clock 2_of_3) |
| `.factory/stories/S-15.01-plugin-async-semantics.md` | v1.4 → v1.5 (References table BC labels refreshed + changelog row + amendment section) |
| `.factory/stories/STORY-INDEX.md` | v2.34 → v2.35 (S-15.01 row updated to v1.5; clock 2_of_3) |
| `.factory/STATE.md` | current_step, last_amended, phase progress, concurrent cycles, current phase steps, session checkpoint, index versions updated |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | this entry appended |

### Status

F3 PASS-4 CLOSED. ADR-013 clock 2_of_3. Awaiting F3 adversary pass-5 (potential convergence pass). ONE MORE NITPICK_ONLY = CONVERGENCE_REACHED.

---

## Burst 8 — F3 CONVERGENCE close — pass-5 NITPICK_ONLY NIT-0; clock 2→3_of_3; CONVERGENCE_REACHED

**Date:** 2026-05-07
**Dispatchers:** orchestrator → state-manager
**Phase:** F3 CONVERGED
**Story version:** S-15.01 v1.5 → v1.6

### Summary

Pass-5 returned NITPICK_ONLY with zero findings. ADR-013 clock advances 2→3_of_3. Three consecutive NITPICK_ONLY passes (pass-3 NIT-1, pass-4 NIT-1, pass-5 NIT-0) per ADR-013 = CONVERGENCE_REACHED. S-15.01 status flipped draft → ready. F4 TDD implementation dispatch next.

F3 trajectory: **9 → 3 → 3 → 1 → 0** (5 passes, 4 fix bursts).

### Final F3 statistics

| Metric | Value |
|--------|-------|
| Total adversary passes | 5 |
| SUBSTANTIVE passes | 2 (passes 1 and 2) |
| NITPICK_ONLY passes | 3 (passes 3, 4, 5 — final ADR-013 clock chain) |
| Fix bursts (substantive) | 2 (pass-1, pass-2) |
| Fix bursts (NIT-only) | 2 (pass-3, pass-4) |
| Total fix bursts | 4 |
| Finding trajectory | 9 → 3 → 3 → 1 → 0 |
| Final ADR-013 clock | 3_of_3 (CONVERGENCE_REACHED) |
| WASM-rule conformance | VERIFIED (native WASM crate; legacy-bash-adapter mentions are explicit negations) |
| File count | 30 paths (Rust crate, .wasm artifact, hooks-registry.toml with plugin= field per S-13.01 precedent) |
| Primary BCs | BC-1.14.001, BC-7.06.001, BC-9.01.006, BC-3.08.001, BC-1.08.001 |
| VPs | VP-077, VP-078, VP-079 |
| DIs | DI-014, DI-019 |

### Lessons captured

| Lesson | Manifestation |
|--------|--------------|
| Architect's "matches existing precedent" reasoning gap | User intervention "we are migrating to WASM" mid-pass-2 fix burst surfaced that the architect's Option C recommendation cited 6 existing bash hooks as precedents — which are themselves under migration (E-8/E-9/E-11). Architects should verify whether cited precedents are themselves migrating before recommending adoption. |
| Sibling-fix discipline: version sync alongside title sync | NIT-P3-003 confirmed by NIT-P4-001. Pass-2 F-P2-003 fixed BC title sync but did not refresh cited versions; pass-3 addressed body table (NIT-P3-001) but missed References table, requiring pass-4 NIT-P4-001. Protocol: any title sync must co-locate version sync across all citing tables in the same fix burst. |
| Byte-for-byte grep > visual inspection | Carried from F2 pass-5/6 lessons. All close-burst verification in F3 used grep to confirm zero residual occurrences of stale values. |

### Artifacts touched

| File | Change |
|------|--------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-5.md` | Created — pass-5 findings persisted (NITPICK_ONLY; clock 3_of_3; CONVERGENCE_REACHED) |
| `.factory/stories/S-15.01-plugin-async-semantics.md` | v1.5 → v1.6 (status draft→ready; amendment section added) |
| `.factory/stories/STORY-INDEX.md` | v2.35 → v2.36 (S-15.01 row: status draft→ready, v1.6; update notice added) |
| `.factory/STATE.md` | current_step, last_amended, phase progress, concurrent cycles, current phase steps (F3 convergence row added), story status (Ready 0→1, Draft 24→23), session checkpoint, ACTIVE STEP, index versions updated |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | this entry appended |

### Status

F3 CONVERGED. S-15.01 v1.6 status: ready. ADR-013 clock 3_of_3. F4 TDD implementation dispatch pending (stub-architect → test-writer → implementer → demo-recorder → pr-manager). Engine-discipline-pass-1 cycle remains paused at F3-pending; will resume after S-15.01 ships.

---

## Burst 13 — F4-handoff document authored for post-context-compaction dispatch

**Date:** 2026-05-07
**Dispatcher:** state-manager
**Phase:** F3 CONVERGED → F4 pre-dispatch

### Outputs

| File | Notes |
|------|-------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F4-handoff.md` | Created — comprehensive F4 dispatch reference. Captures cycle status, S-15.01 v1.6 story summary, 5 user-locked decisions (verbatim), full Option A dispatch chain (stub-architect → test-writer → implementer T-3a..T-3i → demo-recorder → pr-manager), 5 lessons, critical path references, WASM precedent (validate-artifact-path). |
| `.factory/STATE.md` | Session Resume Checkpoint updated: F4-handoff.md reference added; ACTIVE STEP now directs reader to handoff doc; "go F4 Option A" trigger documented. |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | This entry appended. |

### Status

F4-handoff.md on disk. User intends to /compact, then issue "go F4 Option A". All dispatch context is now on disk; no live conversation memory required.

---

## Burst 14 — F5 fix-burst-2 Stage 1: spec amendments (VP/BC/DI/ADR/stories)

**Date:** 2026-05-08
**Dispatcher:** state-manager
**Phase:** F5 fix-burst-2 Stage 1

### Findings addressed this burst

- F-P2-002 [H] VP-079 Scenario 6 SITES short fn names stale — VP-079 v1.7→v1.8 (stable fn name citations)
- F-P2-004 [M] BC-1.14.001 PC4 vs PC6 contradiction — BC-1.14.001 v1.8→v1.9 (PC4 amended)
- F-P2-007 [M] BC version citations stale across S-15.01 — addressed via version-label sweep (S-15.01 v1.8→v1.9)
- F-P2-009 [M] VP-077 H5 harness name incorrect — VP-077 v1.7→v1.9 (H5 name corrected; doc clarification; kani::assume tuple)
- F-P2-010 [M] VP-077 doc clarification needed — included in v1.9 amendment
- F-P2-011 [M] Kani harness name drift + USER-APPROVED Path A: Invariant 7 → (name, event, tool) tuple — BC-7.06.001 v1.3→v1.4; VP-077 kani::assume aligned
- F-P2-015 [NIT] BC-3.08.001 frontmatter hygiene — BC-3.08.001 v1.5→v1.6

### Spec files amended

| File | Change |
|------|--------|
| `.factory/specs/verification-properties/VP-079.md` | v1.7→v1.8: F-P2-002 SITES + stable fn name citations |
| `.factory/specs/verification-properties/VP-077.md` | v1.7→v1.9: F-P2-009 H5 name + F-P2-010 doc clarify + F-P2-011 sibling kani::assume tuple |
| `.factory/specs/domain-spec/invariants.md` | v1.6→v1.7: DI-019 v1.1→v1.2 §Debug-build env-var clause |
| `.factory/specs/architecture/decisions/ADR-020-dispatcher-latency-budget-classes.md` | typo fix line 261, last_amended bumped |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md` | v1.8→v1.9: F-P2-004 PC4 amend |
| `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` | v1.5→v1.6: F-P2-015 frontmatter hygiene |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` | v1.3→v1.4: F-P2-011 USER-APPROVED Path A Invariant 7 → (name, event, tool) tuple |
| `.factory/stories/S-15.01-plugin-async-semantics.md` | v1.8→v1.9: version-label sweep across 6+ sites |
| `.factory/stories/S-15.02-dispatcher-cold-start-optimization.md` | v1.0→v1.1: References table version-label sweep |

### Index updates

| Index | Before | After |
|-------|--------|-------|
| BC-INDEX | v1.32 | v1.33 |
| VP-INDEX | v1.17 | v1.18 |
| STORY-INDEX | v2.38 | v2.39 |
| ARCH-INDEX | v1.20 | v1.21 |

### Status

Stage 1 complete. All spec amendments committed to factory-artifacts.
Stage 2 next: code/test/demo on branch fix/S-15.01-F5-convergence (long-lived; no PR until ADR-013 clock = 3_of_3).
F-P2-001 [H] (ac017_demo_evidence.rs still cites 500ms) and F-P2-003 [H] (latency-canary.md re-record) are Stage 2 scope — code/test/demo files, not touched in Stage 1.

---

## F5 pass-18 adversary review — 2026-05-08

**Verdict:** HIGH (1H/3M/3L; 4 process-gap findings)
**ADR-013 clock:** 0_of_3 (HIGH resets)

### Findings

| ID | Severity | Summary |
|----|----------|---------|
| F-P18-001 | H | Sibling-hook propagation gap — validate-artifact-path had identical absolute-path bug as cc5a016b in validate-stable-anchors |
| F-P18-002 | M | Prose-form line references in BC-1.05.035/036 + BC-2.02.011 not migrated by sweep |
| F-P18-003 | M | BC-INDEX + VP-INDEX missing aggregated changelog entry for 6-chunk mass sweep |
| F-P18-004 | M | TD-031 register not updated with cc5a016b fix or test count change |
| (process-gap) | L×3 | Sibling-hook discipline, prose-form sweep discipline, index aggregation discipline, TD register currency discipline |

**Trajectory:** →(pass-18 HIGH)

---

## F5 fix-burst-17 — 2026-05-08

Three sub-bursts. State-manager ran last per POLICY 3.

### Sub-burst 1: validate-artifact-path sibling absolute-path fix (8b4f697f)

**Agent:** implementer + test-writer
**Branch:** fix/S-15.01-F5-convergence

- `is_spec_target` in validate-artifact-path had identical absolute-path false-negative as cc5a016b in validate-stable-anchors
- Fixed `matches_canonical` + `hook_logic` for absolute-path payloads
- 4 absolute-path tests added to validate-artifact-path; 54/54 → 58/58
- Both WASM artifacts rebuilt
- F-P18-001 closed

### Sub-burst 2: prose-form stable-anchor migration (fadafca5, factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| BC-1.05.035 | v1.1 → v1.2 | Prose "at line NNN" → stable symbol anchors |
| BC-1.05.036 | v1.1 → v1.2 | Prose "at line NNN" → stable symbol anchors |
| BC-2.02.011 | v1.1 → v1.2 | Prose "at line NNN" → stable symbol anchors |

- F-P18-002 closed

### Sub-burst 3: indexes + TD register + state + lessons (this burst, factory-artifacts)

**Agent:** state-manager

| Index | Before | After |
|-------|--------|-------|
| BC-INDEX | v1.41 | v1.42 |
| VP-INDEX | v1.27 | v1.28 |

- BC-INDEX: aggregated changelog entry for 6-chunk TD-VSDD-091 sweep + F-P18-002 follow-up
- VP-INDEX: aggregated changelog entry; VP-077 row updated to v1.11
- TD-031: cc5a016b + 8b4f697f recorded; test counts 58→62 (validate-stable-anchors) + 54→58 (validate-artifact-path); Kani harness deferral noted
- STATE.md: pass-18 + fix-burst-17 progress; current_step updated
- lessons.md: 4 process-gap lessons codified [F-P18-001..004]
- F-P18-003 + F-P18-004 closed

---

## F5 pass-19 adversary review — 2026-05-08

**Verdict:** HIGH (F-P19-001..F-P19-005)
**ADR-013 clock:** 0_of_3 (HIGH resets)

### Findings

| ID | Severity | Summary |
|----|----------|---------|
| F-P19-001 | — | Corpus-wide prose-form sweep — 18 grep matches; 6 body refs in 4 files not migrated (BC-7.03.009, open-questions.md, prd.md, ADR-019); L-P18-002 codified but retroactive sweep not run |
| F-P19-002 | — | Kani Proof 2 assumption stale — `kani::assume(!path.starts_with(".factory/"))` in validate-artifact-path excludes relative paths only; absolute-path payloads bypass assumption; 3 locations (lib.rs:593, kani_path_matching.rs:271, VP-070.md:103) |
| F-P19-003 | — | BC-4.11.001 missing explicit absolute-path semantics — absolute paths to .factory/ excluded by validate-artifact-path but not formalized in BC; Invariant 8 (Path Form Invariance) needed |
| F-P19-004 | — | (process-gap) Lesson backfill discipline — codification without retroactive sweep (cf. L-P19-001 codified this burst) |
| F-P19-005 | — | (process-gap) Kani harness sync policy — non-Kani fix changed behavior without updating assumption (cf. L-P19-002 codified this burst) |

**Trajectory:** →(pass-19 HIGH)

---

## F5 fix-burst-18 — 2026-05-08

Three sub-bursts. State-manager ran last per POLICY 3.

### Sub-burst 1: F-P19-002 Kani harness fix (026272ae fix branch + e23a82ab factory-artifacts)

**Agent:** implementer
**Branch:** fix/S-15.01-F5-convergence + factory-artifacts

- VP-070 Proof 2 assumption tightened to exclude both relative and absolute .factory/ paths
- 3 locations updated: lib.rs:593, kani_path_matching.rs:271, VP-070.md:103
- VP-070 v1.1 → v1.2
- Kani CLI version mismatch (cargo kani 0.67.0 → rustc 1.93.0-nightly < workspace 1.95) defers actual proof execution to CI
- F-P19-002 closed

### Sub-burst 2: F-P19-001 corpus-wide prose-form sweep (91bb304c factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| BC-7.03.009 | v1.2 → v1.3 | Prose-form line refs migrated to stable symbol anchors |
| open-questions.md | v1.1 → v1.2 | Prose-form line refs migrated |
| prd.md | v1.2 → v1.3 | Prose-form line refs migrated |
| ADR-019 | v1.8 → v1.9 | Prose-form line refs migrated |

- 18 grep matches total; 6 body refs migrated; 12 historical refs left as record
- F-P19-001 closed

### Sub-burst 3: F-P19-003 BC-4.11.001 amendment (ce848f24 factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| BC-4.11.001 | v1.1 → v1.2 | New Invariant 8 (Path Form Invariance) + cross-refs in Postconditions 2 and 7 |

- F-P19-003 closed

### Sub-burst 4: indexes + TD register + state + lessons (this burst, factory-artifacts)

**Agent:** state-manager

| Index | Before | After |
|-------|--------|-------|
| BC-INDEX | v1.42 | v1.43 |
| VP-INDEX | v1.28 | v1.29 |

- BC-INDEX: BC-4.11.001 v1.2 + BC-7.03.009 v1.3 recorded; F-P19-001/003 refs
- VP-INDEX: VP-070 v1.2 row description updated; F-P19-002 refs
- TD-031: 026272ae + 91bb304c + ce848f24 recorded; Kani proof execution pending CI
- STATE.md: pass-19 + fix-burst-18 progress; current_step updated; ADR-013 0_of_3; pass-20 next
- lessons.md: 2 process-gap lessons codified [L-P19-001 + L-P19-002]
- F-P19-004 + F-P19-005 closed

---

## Fix-burst-19 — pass-20 HIGH verdict + sub-burst 1 + sub-burst 2 close

**Date:** 2026-05-08
**Dispatchers:** orchestrator → spec-writer → state-manager (POLICY 3 run-last)
**Phase:** F5 ADVERSARIAL — pass-20

### Pass-20 Verdict

**Verdict: HIGH** — F-P20-001 (extended prose-form line-ref sweep across 4 files), F-P20-002 (ARCH-INDEX BC-INDEX cite stale 10 versions; v1.33 vs current v1.43), F-P20-003 (L-P19-002 Kani VP audit disposition incomplete), F-P20-004 (lobster file line-ref class undocumented), F-P20-005 (BC-7.06.001 v1.9 §Fail-Closed Symmetry wording ambiguous). ADR-013 clock: 0_of_3 (RESET — HIGH).

### Sub-burst 1: F-P20-001/002/005 close (98864e19 factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| ARCH-INDEX | v1.22 → v1.23 | BC-INDEX cite refreshed v1.33 → v1.43 (F-P20-002); extended prose-form sweep |
| BC-1.14.001 | v1.9 → v1.10 | Extended prose sweep: plural/range line-ref forms migrated |
| BC-5.34.003 | v1.1 → v1.2 | Extended prose sweep: lobster carve-out documented inline (F-P20-001) |
| BC-6.11.026 | v1.1 → v1.2 | Extended prose sweep: plural/range line-ref forms migrated |
| BC-7.06.001 | v1.9 → v1.10 | §Fail-Closed Symmetry reword (F-P20-005) |

- F-P20-001 closed (extended prose sweep across 4 files; lobster carve-out documented inline)
- F-P20-002 closed (ARCH-INDEX BC-INDEX cite refreshed v1.33 → v1.43; ARCH-INDEX v1.22 → v1.23)
- F-P20-005 closed (BC-7.06.001 v1.9 → v1.10 §Fail-Closed Symmetry reword)

**Lobster-line-class deferral documented:** ~20 BC files contain `(lines NNN-MMM)` style refs to `.lobster` workflow files (e.g., BC-5.32.032:98 to `discovery.lobster (lines 47-57)`, BC-5.34.003:43 to `multi-repo.lobster (lines 575-731)`, etc.). These are deferred as a separate class from BC-source-code line refs because lobster files have stable section structure (each `step:` block has stable `name:` field that could serve as anchor). Drift risk LOW. Future sweep would migrate to `<file>::<step-name>` form. No TD entry required at this time.

### Sub-burst 2: F-P20-003/004 close + lessons + indexes + state (this burst, factory-artifacts)

**Agent:** state-manager (POLICY 3 run-last)

| File | Change |
|------|--------|
| lessons.md | L-P19-002 disposition amended with VP-071/VP-077 audit results (F-P20-003) |
| lessons.md | L-P20-001 codified: literal-vs-class grep discipline in retroactive-sweep |
| lessons.md | L-P20-002 codified: index-of-indexes cite-refresh discipline |
| burst-log.md | F-P20-004 lobster deferral documented (this note) |
| BC-INDEX | v1.43 → v1.44 (BC-1.14.001 v1.10 + BC-5.34.003 v1.2 + BC-6.11.026 v1.2 + BC-7.06.001 v1.10) |
| ARCH-INDEX | v1.23 → v1.24 (BC-INDEX cite refreshed v1.43→v1.44; L-P20-002 same-burst discipline) | <!-- retroactive correction: O-P21-002 — row was missing from original burst-log entry -->
| STORY-INDEX | v2.49 → v2.50 (S-15.03 stub filed) |
| S-15.03 | New story stub filed (draft; hook-based enforcement for index cite-refresh + lesson sweep) |
| STATE.md | fix-burst-19 closed; ADR-013 0_of_3; pass-21 next |

- F-P20-003 closed (L-P19-002 disposition amended — all 3 active Kani VPs audited clean)
- F-P20-004 closed (lobster deferral documented above and in lessons)
- L-P20-001 codified [codified]
- L-P20-002 codified [codified]
- S-15.03 stub filed (follow-up story for hook-based enforcement)

---

## Fix-burst-20 — pass-21 HIGH verdict + sub-burst 1 + sub-burst 2 close

**Date:** 2026-05-08
**Dispatchers:** orchestrator → spec-writer → state-manager (POLICY 3 run-last)
**Phase:** F5 ADVERSARIAL — pass-21

### Pass-21 Verdict

**Verdict: HIGH** — F-P21-001 (BC-5.34.004 lobster carve-out missing — sibling to BC-5.34.003 fix in fix-burst-19 sub-burst 1), F-P21-002 (L-P19-002 disposition fabricated symbol `passes_clean_to_close` — actual fn is `hook_result_for`), F-P21-003 (S-15.03 epic anchor wrong — E-15 Plugin Async Semantics is unrelated; correct anchor E-12 Engine Governance). ADR-013 clock: 0_of_3 (RESET — HIGH).

### Sub-burst 1: F-P21-001 + F-P21-002 close (823468ce factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| BC-5.34.004 | v1.1 → v1.2 | F-P21-001 primary: lobster carve-out HTML comment added |
| BC-5.30.004 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.31.002 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.31.003 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.31.005 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.32.005 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.33.005 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.33.031 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.34.005 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| BC-5.35.005 | v? → v1.2 | Retroactive lobster-class sweep: carve-out HTML comment added |
| lessons.md | — | L-P19-002 disposition corrected: fabricated symbol `passes_clean_to_close` → real production fn `hook_result_for` (F-P21-002) |

- F-P21-001 closed (BC-5.34.004 + 9 siblings retroactive lobster carve-out sweep; 10 BCs total)
- F-P21-002 closed (L-P19-002 disposition fabricated symbol corrected to `hook_result_for`)

### Sub-burst 2: F-P21-003 + indexes + lessons + state (this burst, factory-artifacts)

**Agent:** state-manager (POLICY 3 run-last)

| File | Version | Change |
|------|---------|--------|
| S-15.03 | v1.0 → v1.1 | Re-anchored E-15 → E-12; subsystems [SS-04] → [SS-01, SS-04]; §Anchor Justification + §Amendment added (F-P21-003) |
| E-12 epic | v1.1 → v1.2 | S-15.03 row added; story_count 8→9 |
| BC-INDEX | v1.44 → v1.45 | 10 BC bumps from sub-burst 1 recorded |
| ARCH-INDEX | v1.24 → v1.25 | BC-INDEX cite refreshed v1.44→v1.45 (L-P20-002 same-burst discipline) |
| STORY-INDEX | v2.50 → v2.51 | S-15.03 moved E-15 table → E-12 table |
| lessons.md | — | L-P21-001 codified (POLICY 4 audit-trail integrity — symbols must be grep-verifiable) |
| lessons.md | — | L-P21-002 codified (story epic anchor must match epic title and subsystems_affected) |
| burst-log.md | — | fix-burst-19 sub-burst 2 retroactive correction (ARCH-INDEX v1.24 row added; O-P21-002) |
| STATE.md | — | fix-burst-20 closed; ADR-013 0_of_3; pass-22 next; strategic note added |

- F-P21-003 closed (S-15.03 re-anchored E-15 → E-12; §Anchor Justification + §Amendment per POLICY 5)
- L-P21-001 codified [codified]
- L-P21-002 codified [codified]
- burst-log retroactive correction for fix-burst-19 sub-burst 2 (ARCH-INDEX v1.24 row was missing)

---

## Fix-burst-21 — pass-22 HIGH verdict + sub-bursts 1–4 close

**Date:** 2026-05-08
**Dispatchers:** orchestrator → spec-writer → state-manager (POLICY 3 run-last)
**Phase:** F5 ADVERSARIAL — pass-22

### Pass-22 Verdict

**Verdict: HIGH** — F-P22-001 (corpus-wide lobster-line-cite sweep — 88 BCs in ss-05 missing carve-out; broadest single-class sweep yet, applying L-P19-001 + L-P20-001 at full semantic scope), F-P22-002 (BC-1.14.001 v1.10 fabricated symbols: RegistryEntry.async_flag, executor.rs::execute_tiers, executor.rs::spawn_async_plugin; L-P21-001 retroactive sweep found 7 additional fabrications: BC-1.07.005, BC-1.07.006, edge-cases.md, domain-events.md, VP-016, VP-043). F-P22-003/004/005 E-12 v1.0→v1.3 fixes + L-P21-002 retroactive sweep on 9 stories under E-12. ADR-013 clock: 0_of_3 (RESET — HIGH). Strategic note: 5 consecutive HIGH passes (P18–P22).

### Sub-burst 1: F-P22-001 — 88 BCs ss-05 lobster-line-cite sweep (87dd64aa, factory-artifacts)

**Agent:** spec-writer

| File | Change |
|------|--------|
| 88 BCs in ss-05/ | v1.1 → v1.2 — lobster-line-cite carve-out HTML comment added per L-P19-001+L-P20-001 FULL semantic scope |

- F-P22-001 closed (88 BCs swept — broadest single-burst lobster sweep in F5 cycle)

### Sub-burst 2: F-P22-002 + L-P21-001 retroactive sweep (2ea5ee5a, factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| BC-1.14.001 | v1.10 → v1.11 | F-P22-002: fabricated symbols removed (RegistryEntry.async_flag, executor.rs::execute_tiers, executor.rs::spawn_async_plugin) |
| BC-1.07.005 | v1.0 → v1.1 | L-P21-001 retroactive: fabricated symbol fix |
| BC-1.07.006 | v1.0 → v1.1 | L-P21-001 retroactive: fabricated symbol fix |
| edge-cases.md | v? → v? | L-P21-001 retroactive: fabricated symbol fix |
| domain-events.md | v? → v? | L-P21-001 retroactive: fabricated symbol fix |
| VP-016 | v1.0 → v1.1 | L-P21-001 retroactive: fabricated symbol fix |
| VP-043 | v1.0 → v1.1 | L-P21-001 retroactive: fabricated symbol fix |

- F-P22-002 closed (BC-1.14.001 cycle-anchor fabricated symbols corrected)
- L-P21-001 retroactive: 7 additional fabricated symbols fixed across 7 files

### Sub-burst 3: F-P22-003/004/005 + L-P21-002 retroactive sweep (56f0b883, factory-artifacts)

**Agent:** spec-writer

| File | Version | Change |
|------|---------|--------|
| E-12 epic | v1.0 → v1.3 | F-P22-003/004/005: corrections closed; L-P21-002 retroactive sweep on 9 stories under E-12 |
| 9 stories under E-12 | various | L-P21-002 retroactive: epic anchor + subsystems alignment verified/corrected |

- F-P22-003/004/005 closed (E-12 v1.3)
- L-P21-002 retroactive sweep on 9 E-12 stories complete

### Sub-burst 4: indexes + lessons + state (this burst, factory-artifacts)

**Agent:** state-manager (POLICY 3 run-last)

| File | Version | Change |
|------|---------|--------|
| BC-INDEX | v1.45 → v1.46 | 88 ss-05 BCs + BC-1.14.001 v1.11 + BC-1.07.005/006 v1.1 recorded |
| VP-INDEX | v1.29 → v1.30 | VP-016 v1.1 + VP-043 v1.1 (L-P21-001 retroactive) |
| ARCH-INDEX | v1.25 → v1.26 | BC-INDEX cite refreshed v1.45→v1.46 (L-P20-002 same-burst discipline) |
| STORY-INDEX | v2.51 → v2.52 | E-12 v1.3 amendment recorded |
| tech-debt-register.md | — | TD-031 updated: fix-burst-21 commits recorded (87dd64aa, 2ea5ee5a, 56f0b883) |
| lessons.md | — | L-P22-001 codified: 5-pass HIGH streak — prose-only structurally non-convergent; MUST implement S-15.03 |
| STATE.md | — | fix-burst-21 closed; ADR-013 0_of_3; pass-23 next; HALT-CONSIDERATION strategic note |
| burst-log.md | — | fix-burst-21 entries (this entry) |

- L-P22-001 codified [codified]
- Fix-burst-21 COMPLETE — largest fix-burst in F5 cycle (91 BC bumps + 7 fabrication corrections + E-12 v1.3)
