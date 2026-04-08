# Step 4: Affected Artifact Mapping

Spawn `business-analyst` agent to map the feature request to existing PRD requirements, stories, tests, and verification properties.

## Inputs

- Feature request from Step 1
- All existing artifacts from Step 2
- Component impact map from Step 3

## Actions

1. Spawn `business-analyst` agent with feature request and existing artifact index
2. Map feature request to existing PRD requirements (new vs modified vs unchanged)
3. Identify which existing stories are in the "regression risk zone" (stories whose implementation touches modules being modified)
4. List existing tests that cover the affected modules
5. Identify which verification properties need extension

## Outputs

- Requirement mapping: which existing FRs/NFRs are affected, which are new
- Story risk zone: existing stories touching modified modules
- Test inventory: existing tests covering affected code
- Verification gap list: which VPs need new proofs

## Completion Criteria

- Every existing requirement is classified as affected or unaffected
- Regression risk zone stories are enumerated
- Test coverage of affected modules is known
