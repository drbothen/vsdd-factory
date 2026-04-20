---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary|code-reviewer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1d|2|5
inputs: []
input-hash: "[md5]"
traces_to: prd.md
pass: 1
previous_review: null
---

# Adversarial Review: [Project Name] (Pass [N])

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix from `.factory/current-cycle` (e.g., `P1CONV`, `P3PATCH`)
  - If no current-cycle file exists, omit the cycle segment (falls back to `ADV-P<PASS>-<SEV>-<SEQ>`)
- `<PASS>`: Two-digit pass number (e.g., `P01`, `P24`)
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `ADV-P1CONV-P03-CRIT-001`, `ADV-P3PATCH-P24-HIGH-002`, `ADV-P01-MED-003` (no cycle)

The cycle prefix prevents ID collisions when multiple convergence cycles coexist in the same project.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-<CYCLE>-P[N-1]-[SEV]-001 | CRITICAL | RESOLVED / PARTIALLY_RESOLVED / UNRESOLVED | [evidence] |

## Part B — New Findings (or all findings for pass 1)

### CRITICAL

#### ADV-<CYCLE>-P[N]-CRIT-001: [Finding Title]
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
#### ADV-<CYCLE>-P[N]-HIGH-001: ...

### MEDIUM
#### ADV-<CYCLE>-P[N]-MED-001: ...

### LOW
#### ADV-<CYCLE>-P[N]-LOW-001: ...

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
