# Step 4: Adversary Report

The adversary writes structured findings to a report file.

## Inputs

- Review findings from Step 3

## Actions

1. Write findings to `.factory/phase-f5-adversarial/adversarial-delta-review.md`
2. Each finding must include:
   - Severity: CRITICAL / HIGH / MEDIUM / LOW / COSMETIC
   - Category: spec-fidelity / regression-risk / convention / security / test-quality
   - File and line reference
   - Description of the issue
   - Suggested fix (optional)

## Outputs

- `.factory/phase-f5-adversarial/adversarial-delta-review.md`

## Completion Criteria

- All findings are structured with severity, category, file reference, description
- Findings are actionable (not vague)
