# Step 1: Prepare Review Package

Build the scoped review package for the Adversary, including only delta files.

## Inputs

- `.factory/phase-f1-delta-analysis/affected-files.txt`
- `.factory/phase-f4-implementation/summary.md`
- Delta spec documents from Phase F2

## Actions

1. Build the scoped review package:
   - **Changed files**: new source files, modified source files (full file), new test files
   - **Context files** (read-only): relevant spec sections, story specs, coding conventions
   - **Explicitly excluded**: unchanged source files, previous adversarial reports, implementation rationale
2. Provide full files for modified sources (not just diffs) -- the Adversary needs context

## Outputs

- Review package assembled for adversary consumption

## Completion Criteria

- Package contains only delta files + necessary context
- No unchanged files are included (except direct dependents)
- No previous adversarial reports are included (fresh perspective)
