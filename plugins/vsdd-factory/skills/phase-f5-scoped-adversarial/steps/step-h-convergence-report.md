---
name: step-h-convergence-report
description: Write the convergence summary consolidating all review rounds, findings, and verdicts. Check holdout regression if fixes were behavior-changing.
---

# Step H: Convergence Report

> **Shared context:** Read `./_shared-context.md` before executing this step.

Write the convergence summary and check for holdout regressions.

## Procedure

### 1. Write Convergence Summary

Write to `.factory/phase-f5-adversarial/convergence-summary.md`:

- **Number of review rounds** (per model — primary and secondary)
- **Findings by severity** (initial distribution vs final distribution)
- **Novelty score per round** (showing decay curve)
- **Cross-model unique findings** (findings from review-tier not found by primary, and vice versa)
- **Final verdict:** `CONVERGED` or `NOT-CONVERGED`

### 2. Holdout Regression Check (Conditional)

If F5 fixes are **behavior-changing** (not just formatting or naming):
1. Identify holdout scenarios that touch modules affected by the fixes
2. Re-evaluate those scenarios against the current implementation
3. Compare satisfaction scores to Phase 4 baseline
4. If any scenario regressed: route back to Step E for targeted fix

### 3. Final Gate Check

Phase F5 is COMPLETE when:
- Primary adversary convergence reached (novelty < 0.15, min 3 clean passes)
- Secondary adversary findings (if any) are additive and processed
- All CRITICAL and HIGH findings resolved
- Holdout regression check passed (if applicable)
- Regression suite still passes after all fixes

**No human gate** — this is an automated quality gate.

## Artifacts

- `.factory/phase-f5-adversarial/convergence-summary.md`

## Success Criteria

- Convergence summary includes all required sections
- Final verdict is documented
- Holdout regression checked if fixes were behavior-changing
- Regression suite still passes
- Primary adversary convergence confirmed
