# Step 3: Red Gate Verification

Run the new tests to verify they fail before implementation (TDD Red phase).

## Inputs

- New test files from Step 2

## Actions

1. Run the new tests -- they MUST fail (Red Gate)
2. If new tests pass before implementation, either:
   - The feature already exists (re-evaluate scope in F1)
   - The tests are vacuously true (fix the tests)
3. Red Gate failure is enforced by `plugins/src/red-gate.ts`
4. Record Red Gate results in `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`

## Outputs

- `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`

## Completion Criteria

- All new tests fail (confirming they test unimplemented behavior)
- No vacuously true tests
- Red Gate log records each test and its failure reason
