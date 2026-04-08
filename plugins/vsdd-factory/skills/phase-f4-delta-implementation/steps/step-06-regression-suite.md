# Step 6: Run Regression Suite

Run the full existing test suite to detect any regressions from the implementation.

## Inputs

- Regression baseline from Step 1
- Full test suite (existing + new)

## Actions

1. Run the FULL existing test suite (same command as Step 1)
2. Compare against the baseline from Step 1
3. Every previously-passing test must still pass
4. If ANY regression is detected:
   - Log the regression to `.factory/phase-f4-implementation/regression-log.md`
   - Identify which implementation change caused the regression
   - Fix the implementation (not the test)
   - Re-run until regression suite is clean
5. The `plugins/src/regression-gate.ts` hook automates regression detection

## Outputs

- Regression suite result (pass/fail)
- `.factory/phase-f4-implementation/regression-log.md` (if regressions found)

## Completion Criteria

- Every previously-passing test still passes
- Zero regressions detected
- If regressions were found and fixed, the fix is documented in the log
