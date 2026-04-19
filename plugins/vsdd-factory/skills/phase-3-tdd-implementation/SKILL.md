---
name: phase-3-tdd-implementation
description: Phase 3 entry point — per-story TDD delivery through specialist subagents. Delegates to phase sub-workflow.
---

# Phase 3: TDD Implementation

Phase entry point for implementation. Delivers stories through specialist subagents (test-writer, implementer, demo-recorder, pr-manager) with Red Gate discipline.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-3-tdd-implementation.lobster
```

This workflow runs once per story. The orchestrator invokes it for each story in wave order.

## Steps

| Step | File | What It Does |
|------|------|-------------|
| A | `deliver-story/steps/step-a-create-worktree.md` | Create isolated worktree |
| B | `deliver-story/steps/step-b-generate-stubs.md` | Compilable stubs |
| C | `deliver-story/steps/step-c-failing-tests.md` | Failing tests + Red Gate |
| D | `deliver-story/steps/step-d-implement.md` | TDD micro-commits |
| E | `deliver-story/steps/step-e-record-demos.md` | Per-AC demo artifacts |
| F | `deliver-story/steps/step-f-pr-lifecycle.md` | Push + PR + merge |
| G | `deliver-story/steps/step-g-cleanup.md` | Worktree cleanup + state update |

## Work Skill

Direct command: `/vsdd-factory:deliver-story STORY-NNN`

## Prerequisites

- Phase 2 complete (stories approved)
- Story status is `ready` in STORY-INDEX
- All dependency stories completed

## Gate Criteria

- Red Gate passed (tests failed before implementation)
- All tests pass
- Demo evidence covers all acceptance criteria
- PR merged
- Worktree cleaned up
- Input-hash drift check clean
