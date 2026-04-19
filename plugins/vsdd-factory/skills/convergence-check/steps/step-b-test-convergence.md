---
name: step-b-test-convergence
description: Verify test suite quality — mutation kill rate >= 90%, coverage >= 85%.
---

# Step B: Test Convergence

> **Shared context:** Read `./_shared-context.md` before executing this step.

## Procedure

1. Run test suite: `cargo test --release 2>&1`
2. Read mutation testing results from formal verification report
3. Check coverage metrics

## Pass Criteria

- Mutation kill rate ≥ 90%
- Code coverage ≥ 85%
- All tests pass

## Output

Update the Tests row in the convergence report summary table. Write detail section under `## 2. Test Convergence`.
