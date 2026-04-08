# Step 4: Mutation Testing

Run mutation testing scoped to changed files to verify test quality.

## Inputs

- Changed file list from `.factory/phase-f1-delta-analysis/affected-files.txt`
- Test suite (new + existing)

## Actions

1. Run mutation testing scoped to changed files:
   - Rust: `cargo mutants --file <changed-file-1> --file <changed-file-2> ...`
2. Target: mutation kill rate >= 90% for changed files (>= 95% for security-critical modules)
3. If kill rate is below threshold:
   - Identify surviving mutants
   - Write additional tests to kill them
   - Re-run mutation testing

## Outputs

- `.factory/phase-f6-hardening/mutation-results.md`

## Completion Criteria

- Mutation kill rate >= 90% on changed files
- >= 95% on security-critical modules
- Surviving mutants documented with justification if not killed
