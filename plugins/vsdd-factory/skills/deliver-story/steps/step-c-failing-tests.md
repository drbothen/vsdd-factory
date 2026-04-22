---
name: step-c-failing-tests
description: Dispatch test-writer to write failing tests for each acceptance criterion, then verify the Red Gate.
---

# Step C: Write Failing Tests + Red Gate

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, dispatch discipline, and verification rules.

## Dispatch

**Agent:** `test-writer` as Test Writer (model tier: Standard)

**Task:** "Write failing tests in `.worktrees/STORY-NNN/` for each acceptance criterion / BC. Commit: `test(STORY-NNN): add failing tests for <BC-ref>`."

**Context to pass:** Story file, api-surface.md, test-vectors.md, relevant BC files.

## Red Gate (MANDATORY)

After dispatch returns, **independently** run the test suite and verify:

1. Tests compile
2. All new tests fail
3. Tests fail with **assertion errors**, not build errors
4. The failure messages reference the behavior under test (not "not yet implemented")

If Red Gate fails, dispatch a new test-writer to fix the tests. **Do not proceed to implementation until Red Gate is green** (i.e., tests are correctly red).

Record the Red Gate outcome in `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md` following `${CLAUDE_PLUGIN_ROOT}/templates/red-gate-log-template.md`.

## Exit Condition

Red Gate PASSED — tests compile, all fail with assertion errors, failure messages are meaningful.

## Artifacts

- Test files in `.worktrees/STORY-NNN/`
- `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md` — Red Gate verification log
- Commit on feature branch
