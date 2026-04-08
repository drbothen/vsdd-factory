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

## Dependency Analysis

| # | Service | Integration Type | Usage Scope | Fidelity | DTU? | Points | Justification |
|---|---------|-----------------|-------------|----------|------|--------|---------------|
| 1 | | | | | | | |

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
