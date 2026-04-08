# Step 8: Adversarial Convergence Report

Write the convergence summary consolidating all review rounds and findings.

## Inputs

- All review rounds (primary + secondary)
- Finding resolution status

## Actions

1. Write convergence summary to `.factory/phase-f5-adversarial/convergence-summary.md`:
   - Number of review rounds (per model)
   - Findings by severity (initial vs final)
   - Novelty score per round
   - Cross-model unique findings (findings from Gemini not found by GPT-5.4, and vice versa)
   - Final verdict: CONVERGED / NOT-CONVERGED

## Outputs

- `.factory/phase-f5-adversarial/convergence-summary.md`

## Completion Criteria

- Primary adversary (GPT-5.4) convergence is reached
- Secondary adversary findings (if any) are additive
- Final verdict is documented
- Regression suite still passes after all fixes
