# Step 2: Spawn Primary Adversary

Spawn the primary adversary agent with fresh context and review package.

## Inputs

- Review package from Step 1

## Actions

1. Spawn `adversary` agent with:
   - Model: GPT-5.4 (must be different model family from Claude builders)
   - Context: ONLY the review package from Step 1 (fresh context, no prior conversation)
   - Instructions: review the delta across all categories in Step 3

## Outputs

- Adversary agent spawned with scoped context

## Completion Criteria

- Adversary is a different model family from the builder
- Fresh context (no carryover from previous conversations)
- Only review package provided (not full codebase)
