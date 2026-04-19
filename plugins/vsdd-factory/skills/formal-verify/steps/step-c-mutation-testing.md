---
name: step-c-mutation-testing
description: Run mutation testing to verify test suite quality. Target kill rate >= 90%.
---

# Step C: Mutation Testing

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains templates and module criticality rules.

## Procedure

1. Run mutation testing:
```bash
cargo mutants --timeout 60
```

2. Analyze surviving mutants:
   - **Missing test assertions** — tests exist but don't verify the mutated behavior
   - **Dead code** — mutated code is unreachable
   - **Redundant logic** — mutation doesn't change observable behavior

3. For each survivor, recommend a test addition or document why the mutation is benign

**Goal:** Mutation kill rate ≥ 90% (adjusted by module criticality from `.factory/module-criticality.md`).

## Artifacts

- Mutation Survivors section in `.factory/cycles/<current>/formal-verification-report.md`
- Recommended test additions for surviving mutants

## Success Criteria

- Kill rate ≥ 90% overall
- CRITICAL modules: kill rate ≥ 95%
- All surviving mutants analyzed and categorized
