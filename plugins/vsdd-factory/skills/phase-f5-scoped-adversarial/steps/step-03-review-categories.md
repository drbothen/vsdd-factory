# Step 3: Adversary Review Categories

The adversary evaluates the delta across five dimensions.

## Inputs

- Review package provided to adversary

## Actions

The Adversary evaluates:

**3a. Spec Fidelity**
- Does every new requirement from the PRD delta have corresponding implementation?
- Do modified requirements have updated implementation?
- Are acceptance criteria from story specs met?

**3b. Regression Risk**
- Could any change to an existing file break existing behavior?
- Are there implicit dependencies that the tests might not cover?
- Are error paths in modified code still correct?

**3c. Convention Adherence**
- Does new code follow the same naming patterns as existing code?
- Are error types structured consistently with existing error types?
- Is the module structure consistent with the architecture?

**3d. Security Review**
- Are there new trust boundaries introduced?
- Is input validation present at new entry points?
- Are error messages sanitized (no internal details leaked)?

**3e. Test Quality**
- Are boundary cases tested?
- Are negative cases tested (invalid input, error paths)?
- Could any test pass vacuously (silent failure)?

## Outputs

- Findings per category (fed into Step 4 report)

## Completion Criteria

- All five categories are evaluated
- Each finding is tied to a specific file and line
