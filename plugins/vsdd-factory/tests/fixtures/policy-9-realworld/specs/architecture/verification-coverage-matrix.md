# Verification Coverage Matrix

## Coverage by Module

| Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage Target | VPs |
|--------|------------|-------------|----------|-------------|----------------|-----|
| core-engine | CRITICAL | 3 | 0 | 0 | 95% | VP-001, VP-002, VP-003 |
| api-gateway | HIGH | 0 | 1 | 1 | 90% | VP-004 (proptest); VP-006 (fuzz) |
| query-engine | CRITICAL | 0 | 1 | 0 | 90% | VP-005 |
| config-loader | HIGH | 0 | 0 | 1 | 85% | VP-007 |
| auth-module | HIGH | 0 | 0 | 0 | 85% | VP-008 (integration test) |

## Summary by Method

| Method | Planned Count | P0 | P1 |
|--------|--------------|----|----|
| Kani proofs | 3 | 3 | 0 |
| Proptest properties | 2 | 2 | 0 |
| Fuzz targets | 2 | 1 | 1 |
| Integration test VPs | 1 | 1 | 0 |
| **Total VPs** | **8** | **7** | **1** |
