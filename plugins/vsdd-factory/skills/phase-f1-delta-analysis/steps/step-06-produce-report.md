# Step 6: Produce Delta Analysis Report

Write the Delta Analysis Report consolidating all findings from Steps 3-5.

## Inputs

- Component impact map from Step 3
- Affected artifact mapping from Step 4
- Regression risk assessment from Step 5

## Actions

1. Write the Delta Analysis Report to `.factory/phase-f1-delta-analysis/delta-analysis.md` using `templates/delta-analysis-report-template.md`
2. The report must include:
   - Feature name and brief link
   - Impact assessment table (PRD, Architecture, UX, Stories, Tests, Verification)
   - Files likely changed (with change type: new, modified)
   - Files NOT changed (regression baseline)
   - Risk assessment (regression, architecture, security)
   - Recommended scope for subsequent phases
3. Write machine-readable file list to `.factory/phase-f1-delta-analysis/affected-files.txt`
4. For multi-repo projects: write `.factory/phase-f1-delta-analysis/affected-repos.txt`

## Outputs

- `.factory/phase-f1-delta-analysis/delta-analysis.md`
- `.factory/phase-f1-delta-analysis/affected-files.txt`
- `.factory/phase-f1-delta-analysis/affected-repos.txt` (multi-repo only)

## Completion Criteria

- Report follows the template structure completely
- All sections are populated (no empty placeholders)
- Affected files list is machine-readable (one path per line)
- Regression baseline files are explicitly listed
