# Step 5: Run New Tests (Green Gate)

Run the new test suite to verify all new tests pass.

## Inputs

- New tests from Step 2
- Implementation from Step 4

## Actions

1. Run the new test suite
2. All new tests must pass
3. If any fail, iterate on the implementation until they pass

## Outputs

- Green Gate result (all new tests passing)

## Completion Criteria

- Every new test passes
- No test was weakened or removed to achieve green (tests are immutable after Red Gate)
