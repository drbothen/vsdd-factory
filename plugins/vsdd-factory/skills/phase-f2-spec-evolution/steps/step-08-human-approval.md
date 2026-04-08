# Step 8: Human Approval Gate

Present the spec delta package to the human for final approval.

## Inputs

- PRD delta (new and modified requirements)
- Architecture delta (if any)
- Verification property extensions
- Adversarial review results
- Spec version bump and changelog entry

## Actions

1. Present the complete spec delta package to the human
2. Highlight: new requirements, modified requirements, architecture changes, version bump
3. Include adversarial review summary (findings and resolution status)
4. Wait for explicit human approval

## Outputs

- Human approval (explicit "approved" or revision request)

## Completion Criteria

- Human has explicitly approved the spec delta
- Phase F2 is COMPLETE only when approval is granted
- If rejected: iterate on specific sections per human feedback
