# Step 2: PRD Delta

Spawn `product-owner` agent to update the PRD with new and modified requirements.

## Inputs

- Delta Analysis requirements list from Step 1
- Current PRD at `.factory/specs/prd.md`

## Actions

1. Spawn `product-owner` agent
2. Append new behavioral contracts (continue BC-S.SS.NNN sequence from existing contracts)
3. Modify existing requirements that need updating (mark with UPDATED tag and record previous version inline)
4. Append new non-functional requirements if applicable
5. Append new edge cases to the Edge Case Catalog
6. Do NOT rewrite or restructure existing unaffected requirements
7. Write the PRD delta to `.factory/phase-f2-spec-evolution/prd-delta.md`
8. Update the main PRD at `.factory/specs/prd.md` with the changes

## Outputs

- `.factory/phase-f2-spec-evolution/prd-delta.md` (isolated delta for review)
- Updated `.factory/specs/prd.md`

## Completion Criteria

- New requirements continue the ID sequence (no gaps, no collisions)
- Modified requirements have UPDATED tag with previous version inline
- Unaffected requirements are untouched
