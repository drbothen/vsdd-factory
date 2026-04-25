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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:155
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

# Behavioral Contract BC-5.05.004: architect: DTU assessment is mandatory and covers all 6 categories

## Description

After producing api-surface.md, the architect MUST produce
`.factory/specs/dtu-assessment.md` covering 6 integration-surface categories
(inbound data sources, outbound operations, identity & access, persistence & state,
observability & export, enrichment & lookup). Categories with no services MUST
state "None identified — rationale: …" — not omit the category.

## Preconditions

1. api-surface.md has been authored.

## Postconditions

1. `.factory/specs/dtu-assessment.md` exists.
2. The file contains all 6 category headings.
3. Each category has at least one service entry OR an explicit "None identified" rationale.
4. If no external dependencies exist, file sets `DTU_REQUIRED: false`.

## Invariants

1. All 6 categories are present — none omitted silently.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No external dependencies at all | `DTU_REQUIRED: false` |
| EC-002 | One category has no services | "None identified — rationale: ..." |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| dtu-assessment.md with all 6 categories | Accepted | happy-path |
| dtu-assessment.md missing a category | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | dtu-assessment.md has 6 category headings | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/architect.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.003 — composes with (deployment topology)

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
| **Path** | `plugins/vsdd-factory/agents/architect.md:182-227` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit DTU Assessment section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (dtu-assessment.md) |
| **Global state access** | reads api-surface.md |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
