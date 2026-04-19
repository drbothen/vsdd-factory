---
name: phase-7-convergence
description: Phase 7 entry point — 7-dimensional convergence assessment determining release readiness. Delegates to phase sub-workflow.
---

# Phase 7: Convergence

Phase entry point for convergence assessment. Evaluates all 7 dimensions to determine if the project is ready for release.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-7-convergence.lobster
```

## Steps

| Step | File | What It Does |
|------|------|-------------|
| A | `convergence-check/steps/step-a-spec-convergence.md` | Adversarial novelty assessment |
| B | `convergence-check/steps/step-b-test-convergence.md` | Mutation kill rate + coverage |
| C | `convergence-check/steps/step-c-implementation-convergence.md` | Spec drift + code quality |
| D | `convergence-check/steps/step-d-verification-convergence.md` | Formal verification results |
| E | `convergence-check/steps/step-e-visual-convergence.md` | Demo evidence coverage |
| F | `convergence-check/steps/step-f-performance-convergence.md` | Performance budgets |
| G | `convergence-check/steps/step-g-documentation-convergence.md` | Documentation accuracy |

## Work Skill

Direct command: `/vsdd-factory:convergence-check`

## Prerequisites

- Phase 6 formal hardening passed
- All prior phases complete

## Gate Criteria

- All 7 dimensions CONVERGED
- Traceability matrix generated
- Convergence report generated
- Input-hash drift check clean
- Demo recordings verified by visual-reviewer
- Human approval

## Outcome

- **CONVERGED**: Ready for release → proceed to `/vsdd-factory:release`
- **NOT CONVERGED**: List remaining items with severity → loop back to Phase 3
