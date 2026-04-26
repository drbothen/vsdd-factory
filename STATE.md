---
document_type: pipeline-state
level: ops
version: "2.0"
status: draft
producer: state-manager
timestamp: 2026-04-25T00:00:00Z
phase: 1d-converged
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "Phase 1d CONVERGED. Next: release/v1.0.0-beta.5"
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET: Keep this file under 200 lines.
  Historical content belongs in cycle files, NOT here.
  Run /vsdd-factory:compact-state if this file grows past 200 lines.
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
| **Last Updated** | 2026-04-25 |
| **Current Phase** | 1d-converged |
| **Current Cycle** | v1.0-brownfield-backfill |

## Current Cycle: v1.0-brownfield-backfill

**Mode:** brownfield-onboarding — formal VSDD backfill for v1.0 work that shipped as 1.0.0-beta.4  
**Cycle pointer:** `.factory/cycles/v1.0-brownfield-backfill/INDEX.md`

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phase 0 — Brownfield Ingest | COMPLETE | 1,851 BCs in pass-3-* + pass-8-final-synthesis.md |
| Phase 1.1 — Architecture Index + ADRs | PARTIAL | ARCH-INDEX (10 SS-NN), 3 of 13 ADRs (10 deferred) |
| Phase 1.2 — Sharded Architecture | COMPLETE | 10 SS-NN-\<name\>.md files |
| Phase 1.3 — L2 Domain Spec | COMPLETE | 8 sharded files (28 CAPs, 17 DIs, 22 DEs, 18 DECs, 35 entities) |
| Phase 1.4 — BC Migration | COMPLETE | 1,851 BC-S.SS.NNN files in 10 ss-NN/ shards + BC-INDEX.md |
| Phase 1.5 — Formal PRD | COMPLETE | 40 FRs, 76 NFRs, 100% BC traceability |
| Phase 1.6a — DTU Assessment | COMPLETE | DTU_REQUIRED: false |
| Phase 1.6b — Verification Properties | COMPLETE | 57 VPs (all draft, VP-001..VP-057) |
| Phase 1.7 — Extraction Validation R2 | in-progress | Migration fidelity check |
| Phase 1.8 — Story Migration | COMPLETE | 41 stories S-N.MM, 6 epics E-0..E-5 |
| Phase 1d — Adversarial Spec Review | COMPLETE | 6 passes, converged at pass 6 (3 consecutive NITPICK: passes 4-5-6) |
| Phase 2 — Story Decomposition | not-started | Awaiting Phase 1d completion |

## Current Phase Steps

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| Phase 1d adversarial pass 6 | adversarial-reviewer | complete | pass-6.md — CONVERGENCE_REACHED |
| Phase 1d convergence commit | state-manager | complete | factory-artifacts updated |
| Next: release/v1.0.0-beta.5 | release-agent | pending | bundle ADR template + canonicalization |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN (one-per-file) | `specs/behavioral-contracts/ss-NN/` | 1,851 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 57 |
| Capability | CAP-NNN | `specs/domain-spec/capabilities.md` | 28 |
| Domain Invariant | DI-NNN | `specs/domain-spec/invariants.md` | 17 |
| Domain Event | DE-NNN | `specs/domain-spec/domain-events.md` | 22 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 41 |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 6 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 3 of 13 |

## Subsystem Distribution

| SS-ID | Name | BC Prefix | BCs |
|-------|------|-----------|-----|
| SS-01 | Hook Dispatcher Core | BC-1 | 99 |
| SS-02 | Hook SDK and Plugin ABI | BC-2 | 22 |
| SS-03 | Observability Sinks | BC-3 | 49 |
| SS-04 | Plugin Ecosystem | BC-4 | 13 |
| SS-05 | Pipeline Orchestration | BC-5 | 627 |
| SS-06 | Skill Catalog | BC-6 | 571 |
| SS-07 | Hook Bash Layer | BC-7 | 192 |
| SS-08 | Templates and Rules | BC-8 | 215 |
| SS-09 | Configuration and Activation | BC-9 | 5 |
| SS-10 | CLI Tools and Bin | BC-10 | 58 |
| **Total** | | | **1,851** |

## Story Status (41 total)

- **Merged (22):** All Tier A (5), Tier B.0 (1), Tier B.x (8), most Tier C (6 of 7), Tier D (1)
- **Partial (4):** S-2.05 (cargo publish dry-run), S-3.04 (host fn done, bash not retired), S-4.06 (RoutingFilter parsed not wired), S-5.05 (skeleton)
- **Draft / Not Shipped (15):** All Tier E except partials, all Tier F/G/H

## Drift Items (open)

| ID | Description | Severity | Disposition |
|----|-------------|----------|-------------|
| DRIFT-001 | read_file host fn stub returns CAPABILITY_DENIED unconditionally | MEDIUM | L-P0-001 fix in beta.5 |
| DRIFT-002 | sink.* internal events declared but never emitted | MEDIUM | tied to S-4.04 retry/breaker |
| DRIFT-003 | Per-sink dedicated threads despite S-1.06 shared-runtime intent | MEDIUM | re-design at rc.1 |
| DRIFT-004 | hooks.json + hooks-registry.toml dual routing tables | MEDIUM-HIGH | L-P0-002 cutover before rc.1 |
| DRIFT-005 | HTTP/Datadog/Honeycomb sinks declared but not implemented | MEDIUM | Tier E (S-4.01..S-4.03) |
| DRIFT-006 | Phase 5 events not wired (SessionStart/End) | MEDIUM | Tier G (S-5.01, S-5.02) |
| DRIFT-007 | DISPATCHER_SHUTTING_DOWN constant declared, never emitted | LOW | Tier G fixup |
| DRIFT-008 | plugin.loaded/load_failed events not wired | LOW | dispatcher cleanup |
| DRIFT-009 | verify-sha-currency.sh is template, not registered hook | RESOLVED | CONV-ABS-1 closed |
| DRIFT-010 | 26 unported bash hooks block Windows native | MEDIUM | Tier E (S-3.01..S-3.04) |

## Active Branches

| Branch | SHA | Notes |
|--------|-----|-------|
| main | 1907d8f | last shipped: v1.0.0-beta.4 |
| develop | 40b4592 | ADR template fix; tracks ahead of main |
| feat/canonical-identifiers | open | PR #4 — plugin canonicalization phase 1, 26 files |
| factory-artifacts | f6c36d1 | Phase 1 backfill committed |

## Decisions Log

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-001 | 10-subsystem layout (SS-01..SS-10) | Natural split: Rust compiled (SS-01..04) vs VSDD framework (SS-05..10) | 1.1 | 2026-04-25 | architect |
| D-002 | BC-S.SS.NNN one-per-file sharding | Enables granular traceability and diff-friendly git history | 1.4 | 2026-04-25 | architect |
| D-003 | DTU not required | All external services are HTTP APIs with stable public contracts; no clone needed | 1.6a | 2026-04-25 | architect |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfusion Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers. -->

## Session Resume Checkpoint

| Field | Value |
|-------|-------|
| **Date** | 2026-04-25 |
| **Position** | Phase 1d CONVERGED (6 passes, 3 consecutive NITPICK). Specs are stable. |
| **Convergence counter** | 3 of 3 (passes 4, 5, 6 all NITPICK) — CONVERGENCE_REACHED |
| **Next action** | release/v1.0.0-beta.5 — bundle ADR template + feat/canonical-identifiers (PR #4) |
| **After release** | Phase 2 story dependency graph + wave schedule based on 41 migrated stories |
| **ADR backlog** | 10 deferred ADRs (ADR-004..ADR-013 stubs exist; full write-up after PR #4 lands) |

## Historical Content

| Content | Location |
|---------|----------|
| Burst history | `cycles/v1.0-brownfield-backfill/burst-log.md` |
| Convergence trajectory | `cycles/v1.0-brownfield-backfill/convergence-trajectory.md` |
| Session checkpoints | `cycles/v1.0-brownfield-backfill/session-checkpoints.md` |
| Lessons learned | `cycles/v1.0-brownfield-backfill/lessons.md` |
| Resolved blockers | `cycles/v1.0-brownfield-backfill/blocking-issues-resolved.md` |
