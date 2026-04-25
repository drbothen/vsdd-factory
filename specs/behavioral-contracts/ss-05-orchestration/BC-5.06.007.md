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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1037
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

# Behavioral Contract BC-5.06.007: product-owner: append-only IDs and slugs

## Description

When a BC is removed, refactored, or replaced, the old ID stays in indexes with
`status: retired` or `removed`. New artifacts get new IDs — never reuse.
Filename slugs are immutable across title changes. Use `replaced_by:` to link
old→new.

## Preconditions

1. product-owner removing/refactoring/replacing a BC.

## Postconditions

1. No BC ID appears as `active` after being marked `retired` in BC-INDEX history.
2. Filenames preserve original slugs even when titles change.
3. `replaced_by:` links old → new artifacts.

## Invariants

1. ID space is monotonically growing.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Title change after retirement | Slug remains the same |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Retiring BC-5.01.001 | Status set to retired in index | happy-path |
| Reusing retired ID | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No retired ID is reused | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/product-owner.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.018 — composes with (spec-steward append-only IDs)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#append-only-ids`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:169-178` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Append-Only ID and Slug Protection rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (indexes) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (governance check) |

#### Refactoring Notes

No refactoring needed.
