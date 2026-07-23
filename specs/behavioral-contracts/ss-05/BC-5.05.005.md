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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:163
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

# Behavioral Contract BC-5.05.005: architect: VP-INDEX changes propagate in same burst to verification-architecture.md and verification-coverage-matrix.md

## Description

Any change to VP-INDEX (addition, retirement, module reassignment, tool change,
phase reassignment, count change) MUST propagate within the same burst to
`verification-architecture.md` Provable Properties Catalog + P0/P1 lists AND
`verification-coverage-matrix.md` VP-to-Module table + Totals row. Arithmetic
invariant: VP-INDEX total = sum of per-tool counts = VP row count.

## Preconditions

1. architect changes VP-INDEX.

## Postconditions

1. `verification-architecture.md` updated in same burst.
2. `verification-coverage-matrix.md` updated in same burst.
3. After the burst, `validate-vp-consistency.sh` reports PASS.

## Invariants

1. VP-INDEX total = sum of per-tool counts = VP row count.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | VP added without propagation | Validation hook fails post-burst |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Burst with VP added + propagation | validate-vp-consistency.sh PASS | happy-path |
| Burst with VP added without propagation | Hook FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | validate-vp-consistency.sh reports PASS after every architect burst | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/architect.md`, `validate-vp-consistency.sh` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.002 — composes with (VP proof harness rule)
- BC-5.05.006 — composes with (VP-locking protocol)

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
| **Path** | `plugins/vsdd-factory/agents/architect.md:342-352` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit VP-INDEX Propagation Obligation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (multiple files in same burst) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
