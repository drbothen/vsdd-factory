# Step 2: Decompose Feature into Stories

Spawn `product-owner` agent to break the PRD delta into implementable stories.

## Inputs

- PRD delta from Phase F2
- Existing story graph from Step 1
- Story template at `templates/story-template.md`

## Actions

1. Spawn `product-owner` agent
2. Break the PRD delta into implementable stories
3. Each story covers one logical unit of work
4. Continue the story ID sequence (if last is STORY-005, new ones start at STORY-006)
5. Each story must reference:
   - The new/modified behavioral contracts it satisfies (BC-S.SS.NNN format, DF-020)
   - The verification properties it must uphold (VP-NNN)
   - Module criticality from `.factory/specs/module-criticality.md`
   - Implementation strategy: tdd or gene-transfusion
   - Acceptance criteria with testable assertions

## Outputs

- Draft story specs (one per logical unit)
- Requirement-to-story mapping

## Completion Criteria

- Every new/modified requirement from the PRD delta is covered by at least one story
- Each story has testable acceptance criteria
- Story IDs continue the sequence without gaps or collisions
