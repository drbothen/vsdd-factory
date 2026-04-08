---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary|code-reviewer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1d|2|4
inputs: []
input-hash: "[md5]"
traces_to: prd.md
pass: 1
previous_review: null
---

# Adversarial Review: [Project Name] (Pass [N])

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-P[N-1]-001 | CRITICAL | RESOLVED / PARTIALLY_RESOLVED / UNRESOLVED | [evidence] |

## Part B — New Findings (or all findings for pass 1)

### CRITICAL

#### ADV-[PN]-001: [Finding Title]
- **Severity:** CRITICAL
- **Category:** [contradictions|interface-gaps|security-surface|concurrency|
                 verification-gaps|missing-edge-cases|ambiguous-language|
                 purity-boundary-violations|spec-fidelity|code-quality|
                 coverage-gap|missing-story]
- **Location:** [BC-S.SS.NNN / file:line / VP-NNN]
- **Description:** [what's wrong]
- **Evidence:** [proof of the issue — quote specific lines/sections]
- **Proposed Fix:** [how to fix it]

### HIGH
#### ADV-[PN]-002: ...

### MEDIUM
#### ADV-[PN]-003: ...

### LOW
#### ADV-[PN]-004: ...

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | |
| HIGH | |
| MEDIUM | |
| LOW | |

**Overall Assessment:** [pass / pass-with-findings / block]
**Convergence:** [CONVERGENCE_REACHED / findings remain — iterate]
**Readiness:** [ready for next phase / requires revision]
