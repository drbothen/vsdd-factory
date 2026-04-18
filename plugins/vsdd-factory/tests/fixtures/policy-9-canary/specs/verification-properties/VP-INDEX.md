# VP-INDEX

| VP | Description | Module | Tool | Phase | Status | Story |
|----|-------------|--------|------|-------|--------|-------|
| VP-001 | Input validation | core-engine | kani | P0 | draft | S-001 |
| VP-002 | Output encoding | core-engine | kani | P0 | draft | S-002 |
| VP-003 | Auth token lifecycle | auth-module | proptest | P0 | draft | S-003 |
| VP-004 | Session expiry | auth-module | proptest | P1 | draft | S-004 |
| VP-005 | Rate limiting | api-gateway | fuzz | P0 | draft | S-005 |
| VP-006 | Payload parsing | api-gateway | fuzz | P1 | draft | S-006 |
| VP-007 | Alert creation | alert-engine | kani | P1 | draft | S-007 |
| VP-008 | Config parsing | config-loader | fuzz | P0 | draft | S-008 |
| VP-009 | Credential rotation | security-module | fuzz | P0 | draft | S-009 |
| VP-010 | Log sanitization | security-module | fuzz | P1 | draft | S-010 |
| VP-038 | Threat feed parsing | security-module | fuzz | P1 | draft | S-038 |

## Summary

| Tool | Count |
|------|-------|
| Kani | 3 |
| Proptest | 2 |
| Fuzz | 6 |
| **Total** | **11** |
