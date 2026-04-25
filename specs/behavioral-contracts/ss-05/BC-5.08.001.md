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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:345
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

# Behavioral Contract BC-5.08.001: data-engineer: every migration has both up and down scripts

## Description

Every migration script MUST be reversible — for each `up.sql` (or equivalent),
a corresponding `down.sql` is required. Schemas without rollback are rejected.

## Preconditions

1. data-engineer authoring a migration.

## Postconditions

1. `migrations/` directory: every NNN-up.sql has a matching NNN-down.sql.
2. CI lint hook enforces parity.

## Invariants

1. Schema changes are reversible.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | DROP COLUMN is destructive | Down script may need data restoration plan |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 001-up.sql + 001-down.sql | Accepted | happy-path |
| Up script without down | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every up.sql has matching down.sql in migrations/ | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/data-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.004 — composes with (schema BC-NNN tracing)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#data-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/data-engineer.md:31, 47, 95` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit reversible-migration rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (migrations) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
