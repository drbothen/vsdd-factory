# Step 6: Estimation

Spawn `consistency-validator` agent to estimate effort and identify parallelization.

## Inputs

- New story specs from Step 2
- Extended dependency graph from Step 3
- Existing story estimates for calibration

## Actions

1. Spawn `consistency-validator` agent
2. Estimate story points for each new story (relative to existing stories)
3. Identify which stories can be parallelized (no dependency between them)
4. Calculate the critical path through the new story graph
5. Estimate total feature effort

## Outputs

- Story point estimates per new story
- Parallelization opportunities
- Critical path identification
- Total feature effort estimate

## Completion Criteria

- Every new story has a point estimate
- Critical path is identified with total estimate
- Parallelizable stories are grouped
