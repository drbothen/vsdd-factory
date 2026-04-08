# Step 1: Determine Hardening Scope

Read the delta analysis and build the hardening scope for each tool.

## Inputs

- `.factory/phase-f1-delta-analysis/affected-files.txt`
- `.factory/phase-f5-adversarial/convergence-summary.md`

## Actions

1. Read the delta analysis and build the hardening scope:

   | Tool | Scope | Rationale |
   |------|-------|-----------|
   | Kani proofs | New/modified modules only | Prove properties of new code |
   | Fuzz testing | New code paths only | Find crashes in new input handling |
   | Mutation testing | Changed files only | Verify test quality for new code |
   | Semgrep | Changed files + new files | Security patterns on delta |
   | Regression tests | Full existing test suite | Protect existing behavior |
   | cargo audit / npm audit | Full dependency tree | New deps may introduce vulns |

## Outputs

- Hardening scope document (which tools run on which files)

## Completion Criteria

- Every hardening tool has a defined scope
- Scope is justified per tool
