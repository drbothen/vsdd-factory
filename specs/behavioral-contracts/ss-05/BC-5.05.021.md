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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1405
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

# Behavioral Contract BC-5.05.021: technical-writer: gaps in source documentation explicitly listed

## Description

When source documentation is missing (no doc comments, no schemas), the
technical-writer MUST list these gaps in the output rather than fabricating
descriptions or skipping silently.

## Preconditions

1. technical-writer dispatched.
2. Some source files lack doc comments / schemas.

## Postconditions

1. Documentation output includes a "Documentation Gaps" section.
2. The section enumerates modules/files lacking doc comments.

## Invariants

1. Silent skipping of gaps is forbidden.
2. Fabricating descriptions is forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Entire module has no doc comments | List the module in gaps |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run with partially documented source | "Documentation Gaps" section populated | happy-path |
| Run with no gaps | "Documentation Gaps" section empty/omitted | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Output explicitly lists gaps when source documentation is missing | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/technical-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.019 — composes with (current-code-only docs)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#technical-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/technical-writer.md:50, 64-65` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Failure & Escalation section

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
