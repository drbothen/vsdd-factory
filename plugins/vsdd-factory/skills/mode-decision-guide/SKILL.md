---
name: mode-decision-guide
description: >
  Decision guide for choosing between Greenfield Mode, Brownfield Mode,
  and Feature Mode. Referenced by the Orchestrator during mode detection.
---

# Mode Decision Guide

## Quick Decision Table

| Scenario | Mode | Why |
|----------|------|-----|
| New product from scratch | Greenfield (Phases 1-7) | No existing artifacts to preserve |
| Adding a feature to shipped product | Feature Mode (Phases F1-F7) | Scoped delta, regression protection |
| Major architectural rework | Greenfield (Phases 1-7) | Too many changes to scope as delta |
| Bug fix | Feature Mode (minimal) | Scoped to affected module |
| Performance optimization | Feature Mode | Scoped to hot paths |
| Migration (database, framework) | Greenfield (Phases 1-7) | Cross-cutting, affects everything |
| Existing codebase, first VSDD run | Brownfield (Phase 0) + Greenfield | Need to ingest first, then build |
| Adding to factory-built product | Feature Mode (Phases F1-F7) | Full VSDD artifacts already exist |
| Refactoring without behavior change | Feature Mode (minimal) | Tests protect behavior, verify no regression |
| Multi-repo project | Multi-repo | Coordinates per-repo pipelines via project.yaml |
| Scheduled quality audit | Maintenance | 10 sweep types run on schedule |
| Research new opportunities | Discovery | Autonomous idea evaluation and brief creation |

## Decision Flowchart

```
START: What are you building?
|
+-- "I have a product brief and no existing code"
|   +-> GREENFIELD MODE (Phases 1-7)
|
+-- "I have an existing codebase I want to extend"
|   +-> Has the factory ingested this codebase before?
|       +-- NO -> BROWNFIELD MODE first (Phase 0)
|       |         Then: Greenfield (Phases 1-7) for first feature
|       |         Then: Feature Mode for subsequent features
|       +-- YES -> FEATURE MODE (Phases F1-F7)
|
+-- "I need to add a feature to something the factory already built"
|   +-> FEATURE MODE (Phases F1-F7)
|
+-- "I need a major rework / migration / architecture change"
|   +-> GREENFIELD MODE (Phases 1-7)
|       Even on existing code -- the delta is too large to scope
|
+-- "I just need a bug fix"
|   +-> FEATURE MODE (minimal)
|       Phase F1 (delta analysis) -> F4 (fix) -> F5 (review) -> F7 (converge)
|       Skip F2 (spec unchanged), F3 (no new stories), F6 (hardening optional)
|
+-- "This project spans multiple repos"
|   +-> MULTI-REPO MODE
|       Reads project.yaml, computes repo-level waves, runs per-repo pipelines
|
+-- "I want the system to find new opportunities"
|   +-> DISCOVERY MODE
|       Autonomous research, idea scoring, brief generation
|
+-- "I want to run quality sweeps"
    +-> MAINTENANCE MODE
        10 sweep types, opens fix PRs automatically
```

**Note:** Planning is not a standalone mode — it runs automatically as the front-end of greenfield and brownfield modes. It detects existing artifacts, validates quality, and routes to the correct mode.

**Note:** You don't need to choose multi-repo upfront. Start with greenfield — during Phase 1 architecture, the architect analyzes deployment topology. If the product requires multiple independent services (different stacks, independent release cycles, service boundaries), the architect sets `deployment_topology: multi-service` in ARCH-INDEX.md. The orchestrator detects this after Phase 1 and transitions to multi-repo mode automatically (with human confirmation).

## Quantitative Heuristics

When the mode is not obvious, use these heuristics:

### Feature Mode Threshold

Feature Mode is appropriate when:
- **Files changed:** < 30% of total source files
- **Architecture components affected:** < 50% of components
- **New modules:** fewer new modules than existing modules
- **Dependency depth:** changes do not cascade beyond 2 levels in the dependency graph

### Greenfield Threshold

Switch to Greenfield when:
- **Files changed:** >= 30% of total source files
- **Architecture components affected:** >= 50% of components
- **Breaking changes:** existing interfaces change shape, not just content
- **Dependency depth:** changes cascade through 3+ levels

### Bug Fix (Minimal Feature Mode)

For simple bug fixes, skip non-essential phases:
- **Run:** F1 (scope the fix) -> F4 (implement fix with TDD) -> F5 (review) -> F7 (converge)
- **Skip:** F2 (spec unchanged by a bug fix), F3 (no new stories needed)
- **Optional:** F6 (run hardening only if the bug was in a security-critical module)

## Cross-Cutting Infrastructure (applies to ALL modes)

Regardless of which mode is selected, the following infrastructure from
DF-020-028 applies. See docs/factory-paths.md Cross-Cutting Infrastructure
section for the full baseline:

- **Agent tiers** (T1/T2/T3) with least-privilege tooling
- **Per-story delivery** (worktree -> PR -> merge) via code-delivery.lobster
- **Information asymmetry walls** (8 agents with enforced context exclusions)
- **Security review touchpoints** (4 proactive review points)
- **Cost monitoring** (continuous, tiered response)
- **Notifications** (configurable channel)
- **Artifact backup** (.factory/ on factory-artifacts branch)
- **Crash recovery** (phase-level resume)
- **4-level spec hierarchy** (BC-S.SS.NNN format)
- **Artifact sharding** (index + detail file pattern)
- **Fix PR delivery** (FIX-P[N]-NNN)
- **CI/CD** (ci.yml, release.yml, security.yml)
- **Release** (semver -> CHANGELOG -> tag -> publish)

## Brownfield + Feature Mode Combined Workflow

The typical long-term workflow for an existing codebase:

```
Day 1:     Brownfield Mode (Env → Repo Verify → Phase 0) -> Project Context
Day 2-3:   Greenfield Mode (Phases 1-7 + Release) -> First feature shipped
Day 4+:    Feature Mode (Env → F1-F7 + Release) -> Incremental features
Periodic:  Greenfield Mode -> Major reworks, migrations, architecture changes
Ongoing:   Maintenance Sweep (9 sweeps weekly) -> Quality assurance
           Discovery (scheduled) -> New feature/product ideas
```

1. **First engagement:** Brownfield Mode (Phase 0) to ingest the existing codebase
2. **First feature:** Greenfield (Phases 1-7) to establish VSDD baseline
3. **Subsequent features:** Feature Mode (Phases F1-F7) for incremental additions
4. **Major reworks:** Greenfield when the delta is too large to scope
5. **Continuous:** Feature Mode becomes the steady-state operation
6. **Background:** Maintenance sweeps run weekly (9 sweep types, DF-029)
7. **Proactive:** Discovery engine researches opportunities (DF-034)

## Mode Override

The human can always override auto-detection:
- "Run greenfield" -> Greenfield regardless of existing code
- "Run brownfield" -> Brownfield regardless of state
- "Run feature mode" -> Feature Mode regardless of delta size

The Orchestrator respects explicit human mode selection over auto-detection.

## Applicability

Reference document -- no quality gate. Consumed by orchestrator during mode detection.
