---
name: step-a-create-worktree
description: Dispatch devops-engineer to create an isolated worktree for the story implementation.
---

# Step A: Create Worktree

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline, model selection, and verification rules.

## Dispatch

**Agent:** `devops-engineer` (model tier: Fast)

**Task:** "Create worktree `.worktrees/STORY-NNN/` on branch `feature/STORY-NNN-<desc>` from `develop`."

**Context to pass:** Worktree protocol rules only.

## Exit Condition

`git worktree list` shows the new worktree on the correct branch. **Verify before proceeding.**

## Artifacts

- `.worktrees/STORY-NNN/` — isolated worktree directory
- `feature/STORY-NNN-<desc>` — feature branch
