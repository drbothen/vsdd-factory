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
