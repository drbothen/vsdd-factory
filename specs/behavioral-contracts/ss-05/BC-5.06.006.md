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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1029
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

# Behavioral Contract BC-5.06.006: product-owner: BC H1 heading is title source of truth

## Description

The BC file's H1 (`# BC-S.SS.NNN: <title>`) is the single source of truth.
BC-INDEX, PRD section 2/5 tables, and story body BC tables MUST use the H1
verbatim. Title enrichment (e.g., "(Fail-Closed)") moves INTO the H1 — not
left as index-only context. Title drift is HIGH severity.

## Preconditions

1. product-owner authoring a BC file or its references in indexes/stories.

## Postconditions

1. BC H1 is the authoritative title.
2. Sampled BC H1 vs BC-INDEX title column shows zero mismatches.

## Invariants

1. No drift between H1 and index/story references.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to add disambiguator | Add it INTO the H1 (and propagate) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| BC H1 == BC-INDEX title | Accepted | happy-path |
| BC H1 != BC-INDEX title | HIGH severity drift | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every BC's H1 == BC-INDEX title across all references | manual |

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
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:97-106` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "BC H1 Title Authority" rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (sync) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
