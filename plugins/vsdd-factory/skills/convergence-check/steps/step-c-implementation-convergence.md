---
name: step-c-implementation-convergence
description: Verify implementation quality — no spec drift, clean build/lint, no todo/fixme in production code.
---

# Step C: Implementation Convergence

> **Shared context:** Read `./_shared-context.md` before executing this step.

## Procedure

1. Verify all tests green
2. Run spec drift check (from `/spec-drift`)
3. Verify all code review findings addressed
4. Grep for `todo!()`, `unimplemented!()`, `FIXME` in production code
5. Run linter/clippy

## Pass Criteria

- Clean build, clean lint
- No spec drift
- No `todo!()`, `unimplemented!()`, or `FIXME` in production code
- All code review findings addressed

## Output

Update the Implementation row in the convergence report summary table. Write detail section under `## 3. Implementation Convergence`.
