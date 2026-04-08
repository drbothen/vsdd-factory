# Step 7: Hardening Summary

Write summary consolidating all hardening results.

## Inputs

- Results from Steps 2-6

## Actions

1. Write summary to `.factory/phase-f6-hardening/summary.md`:
   - Kani: N proofs attempted, N passed, N failed (with fix status)
   - Fuzz: N targets, M minutes total, crashes found/fixed
   - Mutation: kill rate per file, overall kill rate
   - Security: findings by severity, all CRITICAL/HIGH resolved
   - Regression: full suite pass/fail

## Outputs

- `.factory/phase-f6-hardening/summary.md`

## Completion Criteria

- All metrics are recorded
- Phase F6 is COMPLETE when all hardening checks pass and regression suite is clean
- No human gate -- this is an automated quality gate
