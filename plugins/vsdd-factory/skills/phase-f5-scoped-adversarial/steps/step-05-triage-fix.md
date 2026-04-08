# Step 5: Triage and Fix

Route adversary findings to responsible agents and fix blocking issues.

## Inputs

- Adversary report from Step 4

## Actions

1. Route findings by severity:
   - CRITICAL / HIGH: must be fixed before proceeding
   - MEDIUM: should be fixed, Orchestrator decides
   - LOW / COSMETIC: documented but not blocking
2. For each fix:
   - Agent makes the fix
   - Re-run relevant tests (new + regression)
   - Verify the fix addresses the finding

## Outputs

- Fixed code (for CRITICAL/HIGH findings)
- Updated test results
- Finding resolution status

## Completion Criteria

- All CRITICAL and HIGH findings are resolved
- Tests still pass after fixes
- Resolution status documented for each finding
