# Verification Coverage Matrix

## VP-to-Module Table

| Module | Kani | Proptest | Fuzz | VPs |
|--------|------|----------|------|-----|
| core-engine | 2 | 0 | 0 | VP-001, VP-002 |
| auth-module | 0 | 2 | 0 | VP-003, VP-004 |
| api-gateway | 0 | 0 | 2 | VP-005, VP-006 |
| alert-engine | 1 | 0 | 0 | VP-007 |
| config-loader | 0 | 0 | 1 | VP-008 |
| security-module | 0 | 0 | 2 | VP-038 |
| **Totals** | **3** | **2** | **6** | **11** |
