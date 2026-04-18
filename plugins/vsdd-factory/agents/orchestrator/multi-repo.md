---
name: orchestrator-multi-repo
description: Orchestrator workflow reference for coordinating pipelines across multiple repositories. Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Multi-Repo Orchestration

Reference file for the orchestrator. Load when project_type is multi-repo.

## Invoking multi-repo.lobster

When `project_type: multi-repo` is detected:

1. Parse `project.yaml` to get: repo list, dependency_graph, integration config
2. Route to `multi-repo.lobster` workflow
3. Do NOT invoke greenfield.lobster or brownfield.lobster at the project level --
   multi-repo.lobster invokes those per-repo as sub-workflows with `cwd`

## Coordinating Per-Repo Sub-Workflows

Multi-repo orchestration uses repo-level waves (from dependency_graph):

```
Repo Wave 0: [api-server]          <- primary services, no cross-repo deps
  Per-repo: greenfield.lobster or brownfield.lobster with cwd=./api-server
  Story Wave 0: [story-1, story-2]  <- story-level waves (existing mechanism)
  Story Wave 1: [story-3]

Repo Wave 1: [frontend, sdk-ts]    <- consumer services, depend on wave 0
  Per-repo: greenfield.lobster with cwd=./frontend
  Per-repo: sdk-generation skill for sdk-ts (role: generated)
```

Each repo gets its own `.factory/` directory on a `factory-artifacts` orphan branch.
The `.factory-project/` directory lives in the primary repo on a separate
`factory-project-artifacts` orphan branch (same worktree treatment as `.factory/`).
It coordinates project-level state, unified specs, repo-wave plans, cross-repo
gates, and cost.

## Handling Mixed Mode (Some Repos Brownfield)

When per-repo mode classification detects mixed modes:

1. Brownfield repos run Phase 0 first (all 12 steps)
2. After ALL brownfield Phase 0s complete: run project-level synthesis
   (multi-repo-phase-0-synthesis skill)
3. Human approves unified project context
4. Then proceed with repo-level waves for implementation

Greenfield repos skip Phase 0. Generated repos (SDKs) use sdk-generation skill.
Feature repos (already ingested) skip Phase 0.

## Multi-Repo Feature Mode (Path 3 + 11)

When a feature request affects multiple repos:

1. F1: Cross-repo delta analysis -- identify affected repos, per-repo impact +
   cross-repo impact (API changes propagate to consumers)
2. F2-F3: Per-repo spec evolution + story creation with cross-repo consistency check
3. F4: Per-repo wave loops -- primary service first if contracts change, then consumers.
   Cross-repo wave gate after all repos complete each wave.
4. Cross-repo integration gate (Docker Compose + e2e + holdout + adversary + security)
5. F5-F7: Scoped adversarial + hardening + convergence (per-repo + integration-level)
6. Release: Coordinated MINOR/PATCH release in dependency order

## Multi-Repo Bug Fix (Path 4 + 11)

1. F1: Bug analysis -- if fix touches API contract consumed by other repos,
   flag for cross-repo regression check
2. Fix delivery: Single repo, per Path 4
3. If cross-repo flag: Run consumer repo test suites against fixed API
4. Release: Single-repo PATCH if no contract change, coordinated PATCH if
   consumers need update

## Multi-Repo Maintenance (Path 10 + 11)

1. Per-repo: 9 maintenance sweeps each (parallel across repos)
2. Cross-repo checks: contract drift, integration test suite, dependency
   version consistency across repos
3. Fix PRs per-repo, then cross-repo regression check after fixes

## Per-Repo Mode Classification

For each repo in project.yaml:
- Has existing source code AND no project-context.md -> mode: brownfield
- Has existing source code AND has project-context.md -> mode: feature
- Empty or doesn't exist yet -> mode: greenfield
- Role is "generated" -> mode: generated (use sdk-generation skill)
