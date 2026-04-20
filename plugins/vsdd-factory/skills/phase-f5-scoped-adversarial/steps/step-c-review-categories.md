---
name: step-c-review-categories
description: The adversary evaluates the delta across five dimensions — spec fidelity, regression risk, convention adherence, security, and test quality.
---

# Step C: Adversary Review Categories

> **Shared context:** Read `./_shared-context.md` before executing this step.

The adversary evaluates the delta across these five dimensions. Each finding must be tied to a specific file and line.

## Review Dimensions

### C1. Spec Fidelity

- Does every new requirement from the PRD delta have corresponding implementation?
- Do modified requirements have updated implementation?
- Are acceptance criteria from story specs met?
- Do new BCs have matching postcondition enforcement in code?

### C2. Regression Risk

- Could any change to an existing file break existing behavior?
- Are there implicit dependencies that the tests might not cover?
- Are error paths in modified code still correct?
- Did any shared state mutation change semantics for callers?

### C3. Convention Adherence

- Does new code follow the same naming patterns as existing code?
- Are error types structured consistently with existing error types?
- Is the module structure consistent with the architecture?
- Do new files follow the established directory layout?

### C4. Security Review

- Are there new trust boundaries introduced?
- Is input validation present at new entry points?
- Are error messages sanitized (no internal details leaked)?
- Are authentication/authorization checks present where required?

### C5. Test Quality

- Are boundary cases tested?
- Are negative cases tested (invalid input, error paths)?
- Could any test pass vacuously (silent failure)?
- Do tests verify behavior, not implementation details?

## Artifacts

- Findings per category (fed into Step D report)

## Success Criteria

- All five categories evaluated
- Each finding tied to a specific file and line
- No category skipped (even if clean — report "no findings" explicitly)
