# Step 7: Secondary Adversarial Pass (Gemini)

Optionally spawn a secondary adversary for cognitive diversity on the delta.

## Inputs

- Delta files (same as primary review package)
- Primary adversary convergence status

## Actions

1. After GPT-5.4 convergence, optionally spawn secondary review using `review/primary` (Gemini 3.1 Pro)
2. Recommended for: security-critical delta, large delta spanning many files, maximum cognitive diversity
3. NOT recommended for: trivial bug fixes or cosmetic changes
4. Write secondary findings to `.factory/phase-f5-adversarial/gemini-review.md`
5. Any new CRITICAL/HIGH findings route through Step 5/6 fix cycle

## Outputs

- `.factory/phase-f5-adversarial/gemini-review.md` (if secondary pass was run)

## Completion Criteria

- Decision to run or skip secondary pass is documented with rationale
- If run: findings are processed through the same triage/fix cycle
