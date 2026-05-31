---
document_type: cycle-index
cycle: v1.0-brownfield-backfill
status: active
---

# Cycle Index — v1.0-brownfield-backfill (milestone cycle)

Milestone cycle; `per_pass_trajectory` flag deliberately absent (PC3/PC4/PC5
advisory). This fixture isolates EC-022: PC1 (current_step) and PC2 (Last Updated)
each carry BOTH a 4-segment `trajectory-tail →9→9→9→11` marker AND a 5-segment
`Full-cycle trajectory: →9→9→9→9→11` prose string. The inv-4 first-semicolon-segment
marker-prefix scoping MUST count only the 4-segment marker → PASS (BC-5.39.009 v1.9
EC-022; the fix MUST NOT relax inv-4 to LENGTH≥4 to accommodate the prose).

## Adversarial Reviews

| Pass | Date | Findings | Status |
|------|------|----------|--------|
| 1 | 2026-05-25 | 0 | CONVERGED 3/3; trajectory-tail →9→9→9→9 |

## Convergence Status

| Field | Value |
|-------|-------|
| Convergence Status | CONVERGED; trajectory-tail →9→9→9→9 |
