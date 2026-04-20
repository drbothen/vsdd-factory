---
name: step-d-adversary-report
description: The adversary writes structured findings to the Phase F5 adversarial review report.
---

# Step D: Adversary Report

> **Shared context:** Read `./_shared-context.md` before executing this step.

The adversary writes structured findings to the report file. Each finding must be actionable.

## Procedure

1. **Write findings** to `.factory/phase-f5-adversarial/adversarial-delta-review.md`

2. **Each finding must include:**
   - **Severity:** CRITICAL / HIGH / MEDIUM / LOW / COSMETIC
   - **Category:** spec-fidelity / regression-risk / convention / security / test-quality
   - **File and line reference** (specific, verifiable)
   - **Description** of the issue (what's wrong, not what to do about it)
   - **Evidence** (quote the code, show the contradiction)
   - **Suggested fix** (optional but recommended)

3. **Follow the template** at `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md`

4. **Include novelty assessment** — are these findings genuinely new, or retreading known issues?

## Artifacts

- `.factory/phase-f5-adversarial/adversarial-delta-review.md`

## Success Criteria

- All findings structured with severity, category, file reference, description
- Findings are actionable (not vague like "consider improving error handling")
- Evidence provided for each finding
- Novelty assessment included
