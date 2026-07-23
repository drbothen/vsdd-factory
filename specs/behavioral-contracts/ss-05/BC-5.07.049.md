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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1459
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

# Behavioral Contract BC-5.07.049: test-writer: uses canonical test vectors from BCs when available

## Description

When a BC includes a Canonical Test Vectors table, the test-writer MUST use
those exact inputs/outputs as parameterized test cases — not invent new inputs.

## Preconditions

1. BC includes a Canonical Test Vectors table.

## Postconditions

1. Tests for that BC use input strings matching the BC's table verbatim.

## Invariants

1. Canonical vectors are the source of truth when present.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC has no canonical vectors | test-writer composes representative inputs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| BC with happy-path/edge/error vectors | Test parameterizes over them | happy-path |
| Test invents inputs ignoring BC vectors | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tests use verbatim canonical inputs from BCs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.045 — composes with (BC-NNN test naming)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#test-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:172-186` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Canonical Test Vectors usage rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (BC) + writes (test) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (test authoring) |

#### Refactoring Notes

No refactoring needed.
