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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1389
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

# Behavioral Contract BC-5.05.019: technical-writer: documents only current code, never aspirational

## Description

Documentation reflects what the code currently does. Planned features, roadmap
items, or TODO behaviors are forbidden in documentation output.

## Preconditions

1. technical-writer dispatched.

## Postconditions

1. Generated docs match current type signatures and runtime behavior.
2. No "Coming soon" / "Planned" sections.

## Invariants

1. Aspirational documentation is forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Code has TODO comment | Don't document the TODO behavior |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Doc reflects current API | Accepted | happy-path |
| Doc with "Coming soon" section | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Docs contain no aspirational language | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/technical-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.020 — composes with (no source modification)
- BC-5.05.021 — composes with (gaps explicitly listed)

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
| **Path** | `plugins/vsdd-factory/agents/technical-writer.md:27, 47, 68` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit anti-aspirational rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (source) + writes (docs) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
