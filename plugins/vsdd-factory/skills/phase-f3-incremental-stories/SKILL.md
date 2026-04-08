---
name: phase-f3-incremental-stories
description: >
  Feature Mode Phase F3: Create new stories for the feature and integrate
  them into the existing dependency graph without cycles.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F3: Incremental Story Creation

## Prerequisites

- Phase F2 Spec Evolution complete and human-approved
- `.factory/phase-f2-spec-evolution/prd-delta.md` exists
- `.factory/stories/` contains existing story specs

## Workflow

### Step 1: Load Existing Story Graph

Read all existing story specs from `.factory/stories/`:
- Build the current dependency graph (story IDs and their depends_on links)
- Identify the highest existing story ID (to continue the sequence)
- Note which stories are completed vs in-progress

### Step 2: Decompose Feature into Stories

Spawn `story-writer` agent to:
- Break the PRD delta into implementable per-file stories (STORY-NNN.md)
- Each story covers one logical unit of work
- Continue the story ID sequence (if last is STORY-005, new ones start at STORY-006)
- Each story must reference:
  - The new/modified behavioral contracts (BC-S.SS.NNN format, DF-020)
  - The verification properties it must uphold (VP-NNN)
  - Acceptance criteria with testable assertions
  - Module criticality from `.factory/specs/module-criticality.md`
  - Implementation strategy: tdd or gene-transfusion

Use `templates/story-template.md` for each story.
Write each story as a separate per-file STORY-NNN.md (not monolithic).

### Step 3: Dependency Graph Extension

For each new story, define dependencies:
- Dependencies on OTHER NEW stories (within this feature)
- Dependencies on EXISTING stories (cross-feature links)
- No story may depend on itself or create a cycle

### Step 4: Cycle Detection

Validate the extended dependency graph:

```
Algorithm:
1. Build adjacency list: existing stories + new stories
2. Run topological sort (Kahn's algorithm)
3. If sort completes: no cycles -- proceed
4. If sort fails: identify the cycle and report to human
```

If cycles are found, restructure the new stories to eliminate them. Common
fixes: merge two stories, split a story differently, remove a dependency.

### Step 5: Conflict Detection

Check for conflicts with in-progress work:
- Do any new stories modify the same files as in-progress stories?
- Do any new stories depend on stories that are not yet complete?
- Are there race conditions in the dependency graph?

Report conflicts to the human with recommended resolution.

### Step 6: Estimation

Spawn `consistency-validator` agent to:
- Estimate story points for each new story (relative to existing stories)
- Identify which stories can be parallelized
- Calculate the critical path through the new story graph
- Estimate total feature effort

### Step 6b: DTU Clone and Gene Transfusion Story Placement

If F2 flagged DTU re-assessment or gene transfusion:
- DTU clone stories placed in Wave 1 (P0 priority) -- external service mocks
  must exist before implementation stories that depend on them
- Gene transfusion stories flagged with `implementation_strategy: gene-transfusion`
  so F4 uses Semport translation instead of "implement from scratch"

### Step 7: Generate Wave Schedule

Generate a wave schedule for the new stories:
- Group stories into waves based on dependency ordering (Kahn's algorithm)
- Typically 1-3 waves for a feature (not 10+ like greenfield)
- Each wave can have stories running in parallel (independent within wave)
- DTU clone stories in Wave 1 if flagged in F2
- Write wave schedule to `.factory/feature/wave-schedule.md`

### Step 7b: Generate Wave Holdout Scenarios

For each wave, generate holdout scenarios:
- Scenarios that test cross-story integration within the wave
- Scenarios that verify regression on existing behavior
- Write to `.factory/feature/wave-holdout-scenarios/`

### Step 8: Write Story Specs

Write each new story as a per-file STORY-NNN.md to `.factory/phase-f3-stories/`
Also update the story index at `.factory/stories/story-index.md` to
include the new stories.

### Step 9: Human Approval Gate

Present to the human:
- New stories with acceptance criteria
- Extended dependency graph (visual)
- Conflict report (if any)
- Effort estimate and critical path

Phase F3 is COMPLETE only when the human explicitly approves the new stories.

## Output Artifacts

- `.factory/phase-f3-stories/STORY-XXX-description.md` (one per new story)
- `.factory/phase-f3-stories/dependency-graph-extended.md`
- `.factory/phase-f3-stories/conflict-report.md` (if conflicts found)
- Updated: `.factory/stories/story-index.md`

## Quality Gate Criteria

- [ ] Story IDs continue the existing sequence (no collisions)
- [ ] Stories are individual per-file STORY-NNN.md (not monolithic)
- [ ] Every new story references BCs (BC-S.SS.NNN format)
- [ ] Every new story has testable acceptance criteria
- [ ] Every new story has verification properties (VP-NNN)
- [ ] Dependency graph has no cycles (topological sort succeeds)
- [ ] Dependency graph extended without modifying existing stories
- [ ] Wave schedule computed for new stories
- [ ] Wave holdout scenarios generated per wave
- [ ] DTU clone stories in Wave 1 if flagged in F2
- [ ] Gene transfusion stories flagged with implementation_strategy
- [ ] Conflicts with in-progress work identified and resolved
- [ ] Effort estimated with critical path identified
- [ ] Human has explicitly approved the new stories
