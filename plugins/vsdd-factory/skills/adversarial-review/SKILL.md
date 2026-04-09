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

## The Iron Law

> **NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST**

Violating the letter of the rule is violating the spirit of the rule. Fresh context means the adversary has not seen prior review passes, the author's explanations, or the orchestrator's summary. Loading any of those contaminates the asymmetry the pattern depends on.

## Announce at Start

Before any other action, say verbatim:

> I'm using the adversarial-review skill to launch a fresh-context adversary pass on <target>.

Then create TodoWrite entries: one per planned pass (minimum 2).

## Red Flags

| Thought | Reality |
|---|---|
| "I already reviewed this, I can skip the adversary pass" | Self-review is not adversarial review. Dispatch. |
| "The spec is obviously correct, one pass is enough" | Minimum is 2. The rule exists because round 1 systematically misses things. |
| "Let me summarize the prior pass for the adversary to save tokens" | That destroys fresh context. Dispatch with only the target artifact. |
| "The adversary found nothing, let's call it done" | Zero findings after a short prompt is a prompt bug, not convergence. Re-dispatch with sharper scope. |
| "This finding isn't really critical, I'll downgrade it" | Severity is the adversary's call, not the orchestrator's. Record as-is. |
| "The same finding keeps appearing, the adversary is stuck" | It keeps appearing because it isn't fixed. Fix it, then re-run. |
| "Novelty is LOW after one pass, we've converged" | Minimum 2 passes. No exceptions. |
| "Let me tell the adversary what the prior reviewer found" | Information asymmetry is the mechanism. Do not leak prior findings. |


## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md` — review document structure
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-index-template.md` — review index
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md` — individual finding format

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
