# Step 6: Re-Review (if needed)

If CRITICAL or HIGH findings were fixed, re-review with fresh adversary context.

## Inputs

- Fixed files from Step 5
- Original finding list

## Actions

1. If CRITICAL or HIGH findings were fixed:
   - Spawn Adversary again with fresh context
   - Include only the FIXED files (not the full delta again)
   - Iterate until no CRITICAL or HIGH findings remain
2. Convergence criterion: findings are all MEDIUM or below, and novelty score < 0.15

## Outputs

- `.factory/phase-f5-adversarial/round-N-review.md` (one per re-review round)

## Completion Criteria

- No CRITICAL or HIGH findings remain
- Novelty score < 0.15 (findings are cosmetic or repeated)
- Fresh context used for each re-review round
