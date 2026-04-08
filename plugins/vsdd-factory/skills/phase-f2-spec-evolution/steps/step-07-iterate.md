# Step 7: Iterate on Findings

Fix legitimate issues found by the adversarial spec review and re-review.

## Inputs

- Adversarial review findings from Step 6
- Delta documents (PRD, architecture, verification)

## Actions

1. If the Adversary finds legitimate issues in the spec delta:
   - Route findings to the responsible agent (product-owner, architect, or formal-verifier)
   - Agent fixes the delta
   - Re-run adversarial review on the updated delta (fresh context)
   - Repeat until findings are cosmetic only
2. If no legitimate issues found, proceed to Step 8

## Outputs

- Updated delta documents (if fixes were needed)
- Updated adversarial review (if re-review was needed)

## Completion Criteria

- All CRITICAL and HIGH findings are resolved
- Remaining findings are COSMETIC or LOW severity
- No more than 10 iteration rounds (escalate to human if not converging)
