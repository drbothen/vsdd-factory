---
name: phase-4-holdout-evaluation
description: Phase 4 entry point — evaluate implementation against hidden acceptance scenarios using a different model family. Delegates to phase sub-workflow.
---

# Phase 4: Holdout Evaluation

Phase entry point for holdout evaluation. Tests implementation against hidden scenarios using information asymmetry (different model family from builder).

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-4-holdout-evaluation.lobster
```

## Steps

| Step | Type | What It Does |
|------|------|-------------|
| Scenario rotation | Agent (orchestrator) | Randomly select 80% of holdout scenarios |
| Holdout evaluation | Skill (`holdout-eval`) | Run evaluation with different model family |

## Work Skill

Direct command: `/vsdd-factory:holdout-eval`

## Prerequisites

- Phase 3 wave complete (all wave stories merged)
- Holdout scenarios exist in `.factory/holdout-scenarios/`

## Gate Criteria

- Evaluator used different model family (GPT-5.4, not Claude)
- Mean satisfaction score >= 0.85
- No must-pass scenario below 0.6
- Satisfaction standard deviation < 0.15
- Scenario rotation applied (80% subset)
