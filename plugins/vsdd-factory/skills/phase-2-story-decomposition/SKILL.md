---
name: phase-2-story-decomposition
description: Phase 2 entry point — decompose PRD into epics, stories, dependency graph, wave schedule, and holdout scenarios. Delegates to phase sub-workflow.
---

# Phase 2: Story Decomposition

Phase entry point for story decomposition. Breaks the PRD and architecture into implementable stories organized by dependency waves.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-2-story-decomposition.lobster
```

## Steps

| Step | File | What It Does |
|------|------|-------------|
| A | `decompose-stories/steps/step-a-define-epics.md` | Group BCs into epics |
| B | `decompose-stories/steps/step-b-create-stories.md` | Break epics into stories |
| C | `decompose-stories/steps/step-c-dependency-graph.md` | Analyze dependencies |
| D | `decompose-stories/steps/step-d-wave-schedule.md` | Assign waves + create index + sprint state |
| E | `decompose-stories/steps/step-e-holdout-scenarios.md` | Create hidden acceptance scenarios |

## Work Skill

Direct command: `/vsdd-factory:decompose-stories`

## Prerequisites

- Phase 1 complete (specs locked, adversarial review passed)
- `.factory/specs/prd.md` with behavioral contracts
- `.factory/specs/architecture/ARCH-INDEX.md`

## Gate Criteria

- Every BC traces to at least one story
- No placeholder acceptance criteria
- Dependency graph has no cycles
- Wave assignments respect dependencies
- At least one holdout scenario per wave
- Input-hash drift check clean
- Human approval
