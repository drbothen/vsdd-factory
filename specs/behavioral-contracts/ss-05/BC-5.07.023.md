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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:629
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

# Behavioral Contract BC-5.07.023: formal-verifier: purity boundary audit catches I/O in pure-core

## Description

For each module classified as pure core in `purity-boundary-map.md`, the
formal-verifier verifies no I/O operations, no global mutable state, pure-only
dependencies, and no implicit state. Side effects in pure core flag for refactoring.

## Preconditions

1. formal-verifier running purity audit against modules.

## Postconditions

1. `purity-audit.md` lists every pure-core module with PASS/FAIL status.
2. Failures route back to architect for refactoring.

## Invariants

1. Pure-core modules are I/O-free, dep-pure, state-pure.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pure-core module reads a constant config | Reading const literals doesn't break purity |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pure-core module with no I/O | PASS | happy-path |
| Pure-core module with `println!` | FAIL; route to architect | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | purity-audit.md has PASS/FAIL row for every pure-core module | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/formal-verifier.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.001 — composes with (architect purity boundary classification)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#formal-verifier`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:167-178` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Purity Boundary Audit section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
