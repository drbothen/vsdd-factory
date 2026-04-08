---
name: adversarial-review
description: Launch a fresh-context adversarial review of specs or implementation. Uses the adversary agent with information asymmetry to find gaps, contradictions, and missing edge cases. Minimum 2 passes to convergence.
argument-hint: "[specs|implementation]"
disable-model-invocation: true
context: fork
agent: adversary
---

# Adversarial Review

Launch the adversary agent to review specs or implementation with fresh context.

## Templates

Read and follow the output format in:
- `.claude/templates/adversarial-review-template.md` — review document structure
- `.claude/templates/adversarial-review-index-template.md` — review index
- `.claude/templates/adversarial-finding-template.md` — individual finding format

## Target

Parse `$ARGUMENTS` to determine review target:
- `specs` — review all documents in `.factory/specs/`
- `implementation` — review source code against specs
- If no argument, default to `specs`

## For Spec Review

Read all spec documents:
1. `.factory/specs/product-brief.md`
2. `.factory/specs/domain-spec/L2-INDEX.md` → read index, then all sections (if exists)
3. `.factory/specs/prd.md`
4. `.factory/specs/prd-supplements/*`
5. `.factory/specs/behavioral-contracts/*`
6. `.factory/specs/verification-properties/*`
7. `.factory/specs/architecture/*`

Attack with the adversary protocol. Write findings to `.factory/cycles/<current>/adversarial-reviews/`.

## For Implementation Review

Read specs first, then review source code against them. Focus on spec drift and silent failures.

## Pass Management

- Each review is a numbered pass (ADV-P1, ADV-P2, etc.)
- After each pass, assess novelty decay
- When novelty is LOW (findings are refinements, not gaps), report convergence
- Minimum 2 passes, maximum 5 before escalating
