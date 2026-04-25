---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:147
subsystem: SS-05
capability: CAP-TBD
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-5.05.003: architect: ARCH-INDEX must declare deployment_topology

## Description

`ARCH-INDEX.md` MUST include a frontmatter `deployment_topology` field set to
`single-service` or `multi-service`. If `multi-service`, `system-overview.md`
MUST include a Service Boundaries section listing each service, tech stack,
role, and dependencies.

## Preconditions

1. architect authoring or updating ARCH-INDEX.md.

## Postconditions

1. `ARCH-INDEX.md` parses with `deployment_topology in {single-service, multi-service}`.
2. Multi-service architectures have `## Service Boundaries` heading in system-overview.md.

## Invariants

1. The deployment_topology field is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Architecture is hybrid (mostly single-service with one sidecar) | TBD — choose dominant topology |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| ARCH-INDEX with `deployment_topology: single-service` | Accepted | happy-path |
| ARCH-INDEX missing field | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | ARCH-INDEX has deployment_topology in the closed enum | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/architect.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.001 — composes with (purity boundary)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#architect`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/architect.md:43, 130-181` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Deployment Topology section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (ARCH-INDEX) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
