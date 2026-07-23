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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1083
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

# Behavioral Contract BC-5.09.005: research-agent: library versions verified against registries, never training data

## Description

Library versions MUST be verified via Context7 (`resolve-library-id` then
`query-docs`) or direct registry lookup. Training data version numbers are
forbidden — they're known stale.

## Preconditions

1. research-agent citing a library version.

## Postconditions

1. Every version number has an accompanying Context7 query or registry URL reference.
2. Report explicitly distinguishes verified vs unverified versions.

## Invariants

1. Versions are registry-verified.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Registry temporarily unavailable | Mark unverified; flag for re-check |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Version with Context7 reference | Accepted | happy-path |
| Version "from memory" without source | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every version has registry reference | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/research-agent.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.004 — composes with (citation rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#research-agent`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/research-agent.md:51, 134` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Context7 / registry rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (web/MCP) |
| **Global state access** | none |
| **Deterministic** | depends on remote |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
