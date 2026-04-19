---
name: step-b-generate-stubs
description: Dispatch test-writer as Stub Architect to create compilable stubs matching the story's file list.
---

# Step B: Generate Stubs

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline, context discipline, and verification rules.

## Dispatch

**Agent:** `test-writer` as Stub Architect (model tier: Fast)

**Task:** "Create compilable stubs in `.worktrees/STORY-NNN/` matching the story's file list. Use `todo!()` or `unimplemented!()` bodies. Commit: `feat(STORY-NNN): add module stubs`."

**Context to pass:** Story file, dependency-graph.md, api-surface.md, relevant BC files.

## Exit Condition

Build/compile passes inside the worktree (e.g., `cargo check`). If it fails, dispatch a new test-writer to fix stubs — do not proceed until clean.

## Artifacts

- Stub source files in `.worktrees/STORY-NNN/`
- Commit on feature branch
