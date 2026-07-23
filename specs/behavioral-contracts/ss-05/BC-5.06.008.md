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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1045
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

# Behavioral Contract BC-5.06.008: product-owner: every domain invariant lifted to a BC

## Description

For every DI-NNN in `domain-spec/invariants.md`, the product-owner MUST identify
or create a BC that enforces it and cite the DI-NNN in the BC's Traceability
"L2 Invariants" field. Bidirectional check: invariant Scope column lists
enforcer BCs; those BCs cite back. Orphan invariants are forbidden.

## Preconditions

1. A domain invariant DI-NNN exists.

## Postconditions

1. At least one BC enforces each DI-NNN.
2. The enforcing BC cites DI-NNN in its L2 Invariants field.
3. The invariant's Scope column lists enforcer BCs.

## Invariants

1. No orphan invariants.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | DI requires multiple BCs to enforce | All listed in Scope |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| DI-001 with enforcing BC | Accepted | happy-path |
| DI-001 with no enforcer | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every DI-NNN appears in at least one BC's L2 Invariants field | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/product-owner.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.005 — composes with (BC numbering)
- BC-5.06.009 — composes with (anchor-back rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#product-owner`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:180-189` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Invariant Lifting Obligation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
