# Verification Coverage Matrix

## Coverage by Module

| Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage Target | VPs |
|--------|------------|-------------|----------|-------------|----------------|-----|
| core-engine | CRITICAL | 2 | 0 | 0 | 95% | VP-001, VP-002 |
| auth-module | HIGH | 0 | 2 | 0 | 90% | VP-003, VP-004 |
| api-gateway | HIGH | 0 | 0 | 2 | 90% | VP-005, VP-006 |
| alert-engine | HIGH | 1 | 0 | 0 | 85% | VP-007 |
| config-loader | HIGH | 0 | 0 | 1 | 85% | VP-008 |
| security-module | CRITICAL | 0 | 0 | 2 | 90% | VP-038 |

## Summary by Method

| Method | Planned Count | P0 | P1 |
|--------|--------------|----|----|
| Kani proofs | 3 | 3 | 0 |
| Proptest properties | 2 | 2 | 0 |
| Fuzz targets | 6 | 5 | 1 |
| **Total VPs** | **11** | **10** | **1** |
