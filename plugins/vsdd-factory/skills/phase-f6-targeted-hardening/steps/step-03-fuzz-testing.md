# Step 3: Fuzz Testing

Write and run fuzz targets for new public interfaces and input handling code.

## Inputs

- New/modified source modules with public interfaces
- Hardening scope from Step 1

## Actions

1. Spawn `formal-verifier` agent
2. Write fuzz targets for new public interfaces and input handling code
3. Run fuzzing for at least 5 minutes per target: `cargo fuzz run <target> -- -max_total_time=300`
4. Report any crashes found
5. For each crash: create a regression test, fix the code, re-fuzz
6. If project does not support fuzzing: skip with documented justification

## Outputs

- `.factory/phase-f6-hardening/fuzz-results.md`

## Completion Criteria

- Fuzz targets cover all new public interfaces
- 5 minutes minimum per target
- All crashes found are fixed and regression-tested
