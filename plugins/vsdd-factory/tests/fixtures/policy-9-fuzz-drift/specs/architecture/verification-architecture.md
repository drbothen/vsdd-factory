# Verification Architecture

## Provable Properties Catalog

| VP | Description | Module | Tool | Phase |
|----|-------------|--------|------|-------|
| VP-001 | Tenant isolation | core-engine | Kani | P0 |
| VP-002 | Auth enforcement | core-engine | Kani | P0 |
| VP-003 | Input bounds | core-engine | Kani | P0 |
| VP-004 | Rate limiting | api-gateway | Proptest | P0 |
| VP-005 | Query injection | query-engine | Proptest | P0 |
| VP-006 | Payload fuzz | api-gateway | Fuzz | P0 |
| VP-007 | Config fuzz | config-loader | Fuzz | P1 |
| VP-008 | E2E auth flow | auth-module | Integration test | P0 |

## P0 Properties

VP-001, VP-002, VP-003, VP-004, VP-005, VP-006, VP-008
