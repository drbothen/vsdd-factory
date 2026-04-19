---
name: step-g-cleanup
description: Dispatch devops-engineer to remove the worktree and local branch, then update sprint state.
---

# Step G: Cleanup + State Update

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline and verification rules.

## Sub-step G.1: Worktree Cleanup

**Agent:** `devops-engineer` (model tier: Fast)

**Task:** "Remove worktree `.worktrees/STORY-NNN/` and delete local branch `feature/STORY-NNN-<desc>`."

**Exit condition:** `git worktree list` no longer shows the worktree; `git branch --list 'feature/STORY-NNN-*'` returns empty for this story.

## Sub-step G.2: State Update

Update `.factory/stories/sprint-state.yaml`: story status → `completed`.
Update `.factory/stories/STORY-INDEX.md`: status column for this story.
Commit to `factory-artifacts` branch: `factory(phase-3): STORY-NNN delivered`.

## Artifacts

- Worktree removed
- Local branch deleted
- `sprint-state.yaml` updated
- `STORY-INDEX.md` updated
- Commit on factory-artifacts

## After Delivery

Tell the user:

```
Story STORY-NNN delivered:
  Red Gate:       PASSED (see .factory/stories/red-gate-log.md)
  Implementation: <N> micro-commits
  Demos:          <N> artifacts in docs/demo-evidence/
  PR:             #<N> merged to develop
  Worktree:       cleaned up
  State:          sprint-state.yaml updated

Next: /wave-gate wave-N when all wave stories are complete.
```
