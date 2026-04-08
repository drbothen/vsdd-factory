# Step 6: Adversarial Spec Review (Scoped)

Spawn `adversary` agent to review only the spec delta, not the full spec.

## Inputs

- PRD delta from Step 2
- Architecture delta from Step 3 (if applicable)
- Verification property extensions from Step 4
- Surrounding context for coherence (existing PRD sections adjacent to changes)

## Actions

1. Spawn `adversary` agent (GPT-5.4, fresh context)
2. Provide ONLY the delta documents and enough surrounding context for coherence
3. The adversary does NOT review unchanged spec sections
4. Adversary reviews for: completeness, consistency, testability, ambiguity, contradictions
5. Write review to `.factory/phase-f2-spec-evolution/adversarial-spec-delta-review.md`

## Outputs

- `.factory/phase-f2-spec-evolution/adversarial-spec-delta-review.md`

## Completion Criteria

- Review is scoped to delta only (not full spec)
- Fresh context used (no carryover from previous conversations)
- Findings are categorized by severity
