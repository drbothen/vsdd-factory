# VP-INDEX

| ID | Property | Module | Method | Priority | Status | Anchor Story |
|----|----------|--------|--------|----------|--------|-------------|
| VP-001 | Tenant isolation | core-engine | Kani | P0 | draft | S-001 |
| VP-002 | Auth enforcement | core-engine | Kani | P0 | draft | S-002 |
| VP-003 | Input bounds | core-engine | Kani | P0 | draft | S-003 |
| VP-004 | Rate limiting | api-gateway | Proptest | P0 | draft | S-004 |
| VP-005 | Query injection | query-engine | Proptest | P0 | draft | S-005 |
| VP-006 | Payload fuzz | api-gateway | Fuzz | P0 | draft | S-006 |
| VP-007 | Config fuzz | config-loader | Fuzz | P1 | draft | S-007 |
| VP-008 | E2E auth flow | auth-module | Integration test | P0 | draft | S-008 |

## Summary by Method

| Method | Count | P0 | P1 |
|--------|-------|----|----|
| Kani | 3 | 3 | 0 |
| Proptest | 2 | 2 | 0 |
| Fuzz | 2 | 1 | 1 |
| Integration test | 1 | 1 | 0 |
| **Total** | **8** | **7** | **1** |
