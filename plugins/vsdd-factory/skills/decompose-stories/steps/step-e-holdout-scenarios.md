---
name: step-e-holdout-scenarios
description: Create hidden acceptance scenarios for holdout evaluation. Derived from BCs but phrased from a black-box perspective.
---

# Step E: Create Holdout Scenarios

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the holdout scenario template.

Create hidden acceptance scenarios that the holdout evaluator will use to independently validate implementation quality.

## Procedure

1. For each wave, create scenarios in `.factory/holdout-scenarios/wave-scenarios/`
2. Derive scenarios from BCs but phrase differently (black-box perspective)
3. Focus on:
   - Critical-path behaviors
   - Edge cases
   - Cross-subsystem interactions
   - Error handling paths
4. Write `.factory/holdout-scenarios/HS-INDEX.md` following `${CLAUDE_PLUGIN_ROOT}/templates/holdout-scenario-template.md`

## Information Asymmetry

Holdout scenarios are **hidden** from the implementer. They test whether the implementation satisfies the spirit of the BCs, not just the letter. Scenarios should be phrased in a way that doesn't directly mirror the BC's acceptance criteria.

## Artifacts

- `.factory/holdout-scenarios/wave-scenarios/*.md` — per-wave scenario files
- `.factory/holdout-scenarios/HS-INDEX.md` — scenario index

## Success Criteria

- At least one holdout scenario per wave
- Critical-path BCs have corresponding holdout scenarios
- Scenarios are phrased from black-box user perspective
- HS-INDEX is complete and matches scenario files
