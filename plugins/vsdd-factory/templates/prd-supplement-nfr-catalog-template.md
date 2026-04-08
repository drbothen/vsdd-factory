---
document_type: prd-supplement-nfr-catalog
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [prd.md, domain-spec/L2-INDEX.md]
input-hash: "[md5]"
traces_to: prd.md
---

# Non-Functional Requirements Catalog: [Product Name]

> PRD supplement — extracted from PRD Section 4.
> Referenced by: architect, performance-engineer, formal-verifier.

## NFR Registry

| ID | Category | Requirement | Target | Validation Method | Priority | Risk Source |
|----|----------|-------------|--------|------------------|----------|-------------|
| NFR-001 | Performance | | | benchmark / load-test / proof | P0 | R-NNN or N/A |
| NFR-002 | Security | | | scan / audit / pentest | P0 | R-NNN or N/A |
| NFR-003 | Reliability | | | chaos / fuzz / soak | P1 | R-NNN or N/A |
| NFR-004 | Scalability | | | benchmark / projection | P1 | R-NNN or N/A |

## NFR Categories

| Category | Description | Validation Agent |
|----------|-------------|-----------------|
| Performance | Throughput, latency, memory | performance-engineer |
| Security | Auth, encryption, injection | security-reviewer |
| Reliability | Uptime, recovery, data integrity | formal-verifier |
| Scalability | Growth, concurrency, resource | performance-engineer |
| Maintainability | Complexity, coupling, coverage | code-reviewer |
| Accessibility | WCAG compliance | accessibility-auditor |

## NFR-to-Module Mapping

| NFR ID | Affected Modules | Architectural Impact |
|--------|-----------------|---------------------|
| NFR-001 | [modules] | [constraints on design] |
