---
name: phase-5-adversarial-refinement
description: Phase 5 entry point — multi-model adversarial review loop until finding novelty decays to zero. Delegates to phase sub-workflow.
---

# Phase 5: Adversarial Refinement

Phase entry point for adversarial refinement. Fresh-context review by a different model family, iterated until convergence.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-5-adversarial-refinement.lobster
```

## Steps

| Step | Type | What It Does |
|------|------|-------------|
| Adversarial review loop | Loop (adversary agent) | Fresh-context review + triage-and-fix, iterated until CONVERGENCE_REACHED |
| Secondary review | Agent (code-reviewer) | Optional Gemini secondary pass |

## Work Skill

Direct command: `/vsdd-factory:adversarial-review implementation`

## Prerequisites

- Phase 3 implementation complete (all wave stories merged)
- Phase 4 holdout evaluation passed

## Gate Criteria

- Finding novelty decayed to zero (CONVERGENCE_REACHED)
- All findings addressed or accepted
- Minimum 3 clean passes
