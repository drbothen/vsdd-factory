---
name: step-d-implement
description: Dispatch implementer to write minimum code to make each failing test pass via TDD micro-commits.
---

# Step D: TDD Implementation

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, dispatch discipline, context discipline, model selection, and verification rules.

## Dispatch

**Agent:** `implementer` (model tier: Standard for S/M stories, Capable for L/XL stories)

**Task:** "Implement in `.worktrees/STORY-NNN/` via TDD. For each failing test, write the minimum code to make it pass. Micro-commit per test: `feat(STORY-NNN): implement <behavior>`. Do not write code not covered by a test."

If the story has `implementation_strategy: gene-transfusion`, include in the task: "Read `.factory/semport/<module>/<module>-target-design.md` and the reference source files listed in the story. Use the translation strategy. Mark uncertain translations `// SEMPORT-REVIEW`."

**Context to pass:** Story file, module-decomposition.md, dependency-graph.md, api-surface.md, relevant BC files.

## Exit Condition

- All tests green
- Clippy/linter clean
- Formatter check clean (e.g., `cargo +nightly fmt --all --check`)
- Zero `todo!()` / `unimplemented!()` in production code

**Verify independently** — run the test suite and linter yourself.

## Artifacts

- Implementation source files in `.worktrees/STORY-NNN/`
- Micro-commits on feature branch
