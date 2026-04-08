# Step 1: Establish Regression Baseline

Run the full existing test suite before writing any new code.

## Inputs

- Existing project test suite
- Project language/framework (determines test command)

## Actions

1. Run the FULL existing test suite:
   - Rust: `cargo test --workspace`
   - TypeScript: `npm test`
   - Python: `pytest`
2. Record the result in `.factory/phase-f4-implementation/regression-baseline.md`:
   - Total tests: N
   - Passing: N
   - Failing: 0 (if any fail, STOP -- fix regressions before proceeding)
   - Timestamp
3. This baseline is the contract: after implementation, every one of these tests must still pass

## Outputs

- `.factory/phase-f4-implementation/regression-baseline.md`

## Completion Criteria

- All existing tests pass (zero failures)
- Baseline is recorded with test count and timestamp
- If any tests fail: STOP and fix before proceeding
