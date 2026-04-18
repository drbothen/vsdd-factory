---
document_type: dtu-assessment
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: [architecture/api-surface.md, architecture/module-decomposition.md]
traces_to: architecture/ARCH-INDEX.md
---

# DTU Assessment: [Product Name]

## Summary

| Metric | Value |
|--------|-------|
| External dependencies identified | N |
| DTU clones recommended | N |
| Total clone story points | N |
| Estimated Wave 1 capacity needed | N points |

## Integration Surface Inventory (MANDATORY — all categories required)

Categorized by data flow direction and business criticality. Every category must be assessed — if no services exist for a category, state "None identified" with rationale.

### Inbound Data Sources (External → Product)

Systems your product reads from: APIs polled, feeds consumed, webhooks received, sensor data ingested.

*Fidelity signal: how critical is data freshness and completeness?*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

### Outbound Operations (Product → External)

Systems your product writes to or triggers: notifications, ticketing, payments, command execution.

*Fidelity signal: how critical is delivery guarantee?*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

### Identity & Access (Bidirectional — auth flow)

Authentication, authorization, secrets management, credential stores.

*Fidelity signal: always high — security boundary. Typically L3+.*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

### Persistence & State (Product ↔ Storage)

External databases, caches, object stores, message queues, distributed state.

*Fidelity signal: how critical is consistency?*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

### Observability & Export (Product → Monitoring)

Systems your product emits data to: logging aggregators, metrics platforms, tracing, analytics.

*Fidelity signal: can tests run without it? Often L1 or skip.*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

### Enrichment & Lookup (External → Product, on-demand)

External data that augments your product's decisions but isn't the primary data source.

*Fidelity signal: how stale can the data be?*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| | | | | | |

*If none: "None identified — rationale: ..."*

## Dependency Summary

| # | Service | Category | Fidelity | DTU? | Points | Justification |
|---|---------|----------|----------|------|--------|---------------|
| 1 | | | | | | |

## Services NOT Requiring DTU

| # | Service | Reason |
|---|---------|--------|
| 1 | | Simple static endpoint / mock sufficient / no integration tests needed |

## DTU Architecture

### Docker Compose Structure

| Clone | Port | Fidelity | Depends On |
|-------|------|----------|-----------|
| [service]-clone | 8080 | L3 | -- |

### Environment Variable Overrides

| Variable | Production Value | DTU Value |
|----------|-----------------|-----------|
| SERVICE_API_URL | https://api.service.com | http://localhost:8080 |

## Clone Development Approach

Each DTU clone will be developed as a VSDD story with:
- Behavioral contracts derived from API documentation
- Contract tests verifying clone matches API spec
- Implementation via TDD (Express/FastAPI HTTP server)
- Docker packaging for portability
- Demo recording showing clone API behavior

Clones are scheduled in Wave 1 of the wave schedule to ensure they
are available before any product story that depends on them.
