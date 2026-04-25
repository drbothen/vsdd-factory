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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:735
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

# Behavioral Contract BC-5.07.031: implementer: respects purity boundary map

## Description

For every module in `purity-boundary-map.md` marked pure core, the implementer
MUST keep functions side-effect-free. Effectful operations (I/O, DB, network)
live in the Effectful Shell.

## Preconditions

1. implementer adding code to a pure-core module.

## Postconditions

1. Formal-verifier's purity audit (BC-5.07.023) passes after implementation.

## Invariants

1. Pure core stays pure throughout implementation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pure-core module needs to log | Move log to effectful shell |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pure-core impl with no I/O | Purity audit PASS | happy-path |
| Pure-core impl with println! | Purity audit FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Pure-core modules pass formal-verifier purity audit post-impl | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.001 — composes with (architect purity classification)
- BC-5.07.023 — composes with (formal-verifier purity audit)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#implementer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/implementer.md:99-101` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Purity Boundary Map respect rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (source) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
