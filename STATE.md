---
document_type: pipeline-state
level: ops
version: "2.0"
status: draft
producer: state-manager
timestamp: 2026-04-26T12:00:00Z
phase: post-beta-5-shipped
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "E-7 + S-6.01 spec converged + GREEN done. Next: PR both branches → develop → cut beta.6 release."
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
| **Last Updated** | 2026-04-26 |
| **Current Phase** | post-beta-5-shipped |
| **Current Cycle** | v1.0-brownfield-backfill |

## Current Cycle: v1.0-brownfield-backfill

**Mode:** brownfield-onboarding — formal VSDD backfill for v1.0 work that shipped as 1.0.0-beta.4  
**Cycle pointer:** `.factory/cycles/v1.0-brownfield-backfill/INDEX.md`

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phase 0 — Brownfield Ingest | COMPLETE | initial BC migration in pass-3-* + pass-8-final-synthesis.md |
| Phase 1.1 — Architecture Index + ADRs | COMPLETE | ARCH-INDEX (10 SS-NN) + 13 of 13 ADRs (ADR-001..013) |
| Phase 1.2 — Sharded Architecture | COMPLETE | 10 SS-NN-\<name\>.md files |
| Phase 1.3 — L2 Domain Spec | COMPLETE | 8 sharded files (28 CAPs, 17 DIs, 22 DEs, 18 DECs, 35 entities) |
| Phase 1.4 — BC Migration | COMPLETE | 1,878 BC-S.SS.NNN files in 10 ss-NN/ shards + BC-INDEX.md (current) |
| Phase 1.5 — Formal PRD | COMPLETE | 42 FRs (FR-041, FR-042 added), 76 NFRs, 100% BC traceability |
| Phase 1.6a — DTU Assessment | COMPLETE | DTU_REQUIRED: false |
| Phase 1.6b — Verification Properties | COMPLETE | 62 VPs (all draft, VP-001..VP-062; +2 for E-7) |
| Phase 1.7 — Extraction Validation R2 | in-progress | Migration fidelity check |
| Phase 1.8 — Story Migration | COMPLETE | 41 stories S-N.MM, 6 epics E-0..E-5 |
| Phase 1d — Adversarial Spec Review | COMPLETE | 6 passes, converged at pass 6 (3 consecutive NITPICK: passes 4-5-6) |
| Release v1.0.0-beta.5 | COMPLETE | PR #5 merged 2001b97; tag 0a95c8c; bot bundle f1ec5bf; 5 plugins · 110 skills |
| Phase 2 — Story Decomposition | not-started | Unblocked; 44 stories (41 migrated + 3 new E-6/E-7) ready for dependency graph + wave schedule |
| S-6.01 spec convergence (sub-cycle) | COMPLETE | 8 passes, 19→0 trajectory, CONVERGENCE_REACHED at pass-8 |
| E-7 Process Codification spec foundation | COMPLETE | E-7 epic + S-7.01/S-7.02 (status=ready) + 15 BCs + 2 VPs + FR-042 |
| E-7 spec convergence (sub-cycle) | COMPLETE | 7 passes, 12→5→1→2→2→0→0 trajectory, CONVERGENCE_REACHED at pass-7 |
| E-7 GREEN implementation (S-7.01 + S-7.02) | COMPLETE | feat/codify-lessons commit 5b9e4fb; 16/16 bats tests green; 10 plugin source files |
| S-6.01 GREEN implementation | COMPLETE | feat/create-adr-skill commit 5f0b0fa; 25/25 bats tests green; SKILL.md + commands + driver |

## Current Phase Steps

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| PR feat/create-adr-skill → develop | pr-manager | pending | TBD |
| PR feat/codify-lessons → develop | pr-manager | pending | TBD |
| Cut release/v1.0.0-beta.6 after both PRs merge | devops-engineer | pending | TBD |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN (one-per-file) | `specs/behavioral-contracts/ss-NN/` | 1,878 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 62 |
| Capability | CAP-NNN | `specs/domain-spec/capabilities.md` | 28 |
| Domain Invariant | DI-NNN | `specs/domain-spec/invariants.md` | 17 |
| Domain Event | DE-NNN | `specs/domain-spec/domain-events.md` | 22 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 44 |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 8 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 13 |

## Subsystem Distribution

| SS-ID | Name | BC Prefix | BCs |
|-------|------|-----------|-----|
| SS-01 | Hook Dispatcher Core | BC-1 | 99 |
| SS-02 | Hook SDK and Plugin ABI | BC-2 | 22 |
| SS-03 | Observability Sinks | BC-3 | 49 |
| SS-04 | Plugin Ecosystem | BC-4 | 13 |
| SS-05 | Pipeline Orchestration | BC-5 | 636 |
| SS-06 | Skill Catalog | BC-6 | 583 |
| SS-07 | Hook Bash Layer | BC-7 | 196 |
| SS-08 | Templates and Rules | BC-8 | 217 |
| SS-09 | Configuration and Activation | BC-9 | 5 |
| SS-10 | CLI Tools and Bin | BC-10 | 58 |
| **Total** | | | **1,878** |

## Story Status (44 total)

- **Merged (22):** All Tier A (5), Tier B.0 (1), Tier B.x (8), most Tier C (6 of 7), Tier D (1)
- **Partial (4):** S-2.05 (cargo publish dry-run), S-3.04 (host fn done, bash not retired), S-4.06 (RoutingFilter parsed not wired), S-5.05 (skeleton)
- **Draft / Not Shipped (15):** All Tier E except partials, all Tier F/G/H
- **Ready (3):** S-6.01 (create-adr skill), S-7.01 (agent prompt discipline), S-7.02 (defensive sweep + hook + meta-rule)

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

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | f1ec5bf | bot binary bundle on top of 2001b97 (PR #5 merge) |
| develop | faa4aac | PR #4 squash merge content |
| factory-artifacts | c50bb0f | 10 ADRs commit |
| v1.0.0-beta.5 (tag) | 0a95c8c | SHIPPED 2026-04-26; GitHub Release published |

## Decisions Log

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-001 | 10-subsystem layout (SS-01..SS-10) | Natural split: Rust compiled (SS-01..04) vs VSDD framework (SS-05..10) | 1.1 | 2026-04-25 | architect |
| D-002 | BC-S.SS.NNN one-per-file sharding | Enables granular traceability and diff-friendly git history | 1.4 | 2026-04-25 | architect |
| D-003 | DTU not required | All external services are HTTP APIs with stable public contracts; no clone needed | 1.6a | 2026-04-25 | architect |
| D-004 | v1.0.0-beta.5 release scope | ADR template + identifier canonicalization phase 1 shipped; phase 2 (test fixtures, workflows, agents) deferred to beta.6 | release | 2026-04-26 | orchestrator |
| D-005 | Add create-adr skill to v1.0.x roadmap | ADR is the only major artifact without a dedicated authoring skill (compare create-prd, create-story, create-architecture, create-domain-spec); 10-ADR backfill exposed pain points (manual ID allocation, ARCH-INDEX drift, no supersession patcher) | post-1.1 | 2026-04-26 | orchestrator + user |
| D-006 | Spec-first authoring discipline restored after S-6.01 gap caught | Story scaffolded without BCs initially; user caught the gap; full upstream artifacts (BCs/VPs/FR/epic) backfilled before TDD continued | 1.5 | 2026-04-26 | orchestrator + user |
| D-007 | Hook validate-novelty-assessment.sh tightened to anchor on cycles/<key>/adversarial-reviews/ directory; ADR-* explicitly skipped | False-positive on ADR-013 (filename contains 'adversarial-review'); fix lands in plugin source for next release | post-adv-pass-1 | 2026-04-26 | orchestrator |
| D-008 | Codify spec-first-then-TDD discipline + defensive-sweep pattern as plugin source rules | User caught "no BCs/no E-6 epic" gap; F-027 (incomplete defensive sweep) caused 2 wasted passes; lessons should land in agent prompts and consistency-validator | post-1.5 | 2026-04-26 | orchestrator + user |
| D-009 | E-7 Process Codification — codify lessons learned from S-6.01 sub-cycle as plugin source rules | Self-referential dogfooding — vsdd-factory uses its own VSDD process to improve itself; lessons table from D-008 driven into prompt/rule/hook deliverables | post-1.5 | 2026-04-26 | orchestrator + user |
| D-010 | E-7 process codification + S-6.01 create-adr skill → bundle into beta.6 release | Both branches ready (specs converged, GREEN tests pass). Bundling reduces release overhead; both deliver self-improvement value (E-7 codifies lessons; S-6.01 closes per-artifact create-* skill gap) | pre-release | 2026-04-26 | orchestrator + user |


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
| **Date** | 2026-04-26 |
| **Position** | E-7 CONVERGED (7 passes, 12→0) + GREEN done (5b9e4fb). S-6.01 GREEN done (5f0b0fa). D-010 logged. |
| **Convergence counters** | E-7: 12→5→1→2→2→0→0 CONVERGENCE_REACHED pass-7. S-6.01: 19→4→2→1→1→0→0→0 CONVERGENCE_REACHED pass-8. |
| **Next action** | PR feat/create-adr-skill → develop. PR feat/codify-lessons → develop. Cut release/v1.0.0-beta.6. |
| **Branches** | feat/create-adr-skill (5f0b0fa); feat/codify-lessons (5b9e4fb). Both GREEN. |

**Lessons Learned (S-6.01 sub-cycle):** All 8 lessons codified into plugin source via E-7 (commit 5b9e4fb). Lessons #1-4, #7-8 → agent prompts + hooks + rules. Lessons #5-6 shipped previously. All CLOSED.

## Release Ladder

| Tag | Date | Highlights |
|-----|------|-----------|
| v1.0.0-beta.1 | 2026-04-22 | Initial beta |
| v1.0.0-beta.2 | 2026-04-23 | (per CHANGELOG) |
| v1.0.0-beta.3 | 2026-04-25 | hook tool_response shape fix |
| v1.0.0-beta.4 | 2026-04-25 | cache fix + stderr capture + SHA-currency gate |
| v1.0.0-beta.5 | 2026-04-26 | ADR template + identifier canonicalization phase 1 |

## Historical Content
Historical detail (burst-log, convergence-trajectory, session-checkpoints, lessons, resolved-blockers) lives in `cycles/v1.0-brownfield-backfill/`.
