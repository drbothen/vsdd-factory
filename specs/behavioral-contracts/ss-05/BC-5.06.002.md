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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:193
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

# Behavioral Contract BC-5.06.002: business-analyst: produces sharded L2 (L2-INDEX + section files), never monolithic

## Description

L2 output MUST be a sharded directory `.factory/specs/domain-spec/` containing
L2-INDEX.md (produced first) plus 9-10 section files (capabilities, entities,
invariants, events, edge-cases, assumptions, risks, failure-modes, differentiators,
optionally event-flow). Each section targets 800-1,200 tokens. Monolithic
`domain-spec-L2.md` is forbidden.

## Preconditions

1. business-analyst dispatched to author L2 specs.

## Postconditions

1. `.factory/specs/domain-spec/L2-INDEX.md` exists.
2. Section files exist with `traces_to: L2-INDEX.md` frontmatter.
3. No monolithic `domain-spec-L2.md` exists.

## Invariants

1. Sharded layout is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single-section L2 needed | Still sharded with INDEX |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Sharded L2 directory with INDEX + sections | Accepted | happy-path |
| Monolithic domain-spec-L2.md | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | L2-INDEX.md and per-section files all present | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/business-analyst.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.001 — composes with (grounding in brief)
- BC-5.06.003 — composes with (all template sections present)

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
| **Path** | `plugins/vsdd-factory/agents/business-analyst.md:30, 144-166` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Sharded L2 Output (DF-021) section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (sharded directory) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
