---
document_type: module-criticality
level: ops
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: []
input-hash: "[md5]"
traces_to: ""
---

# Module Criticality Classification: [Product Name]

## Tier Definitions

| Tier | Mutation Kill Rate Target | Description | Examples |
|------|--------------------------|-------------|----------|
| **CRITICAL** | >= 95% | Core business logic, security boundaries, data integrity | Authentication, payment processing, state machines |
| **HIGH** | >= 90% | Important functionality with significant user impact | API handlers, validation, data transformation |
| **MEDIUM** | >= 80% | Supporting functionality, utilities | Logging, formatting, configuration parsing |
| **LOW** | >= 70% | Infrastructure, glue code, generated code | Build scripts, boilerplate, wrappers |

## Module Classification

| Module | Path | Tier | Rationale | Kill Rate Target | VP Count |
|--------|------|------|-----------|-----------------|----------|
| [module name] | [file/module path] | CRITICAL | [why this tier] | >= 95% | [n] |
| [module name] | [file/module path] | HIGH | [why this tier] | >= 90% | [n] |
| [module name] | [file/module path] | MEDIUM | [why this tier] | >= 80% | [n] |
| [module name] | [file/module path] | LOW | [why this tier] | >= 70% | [n] |

## Classification Summary

| Tier | Module Count | Percentage |
|------|-------------|------------|
| CRITICAL | [n] | [pct]% |
| HIGH | [n] | [pct]% |
| MEDIUM | [n] | [pct]% |
| LOW | [n] | [pct]% |
| **Total** | **[n]** | **100%** |
