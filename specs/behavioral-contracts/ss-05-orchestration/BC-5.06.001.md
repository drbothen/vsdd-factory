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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:185
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

# Behavioral Contract BC-5.06.001: business-analyst: never invents capabilities — must ground in product brief

## Description

Every CAP-NNN, DI-NNN, R-NNN, ASM-NNN MUST be traceable to a specific section/line
of the product brief or domain research. When the brief is ambiguous, the agent
stops and asks the human — it never guesses.

## Preconditions

1. business-analyst dispatched to author L2 capabilities/invariants/risks/assumptions.

## Postconditions

1. Every CAP-NNN entry has `traces_to: product-brief.md§<section>` annotation.
2. No capability lacks a brief grounding.
3. Ambiguity halts the agent for human input.

## Invariants

1. Capabilities never invented; always grounded.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Brief is ambiguous on a needed capability | Halt; ask human |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Capability traced to brief | Accepted | happy-path |
| Capability without brief reference | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every CAP-NNN has traces_to populated | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/business-analyst.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.002 — composes with (sharded L2 output)
- BC-5.06.004 — composes with (ASM/R validation methods)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#business-analyst`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/business-analyst.md:27-29, 45` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit grounding rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (brief) + writes (L2 specs) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
