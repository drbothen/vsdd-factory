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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1497
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

# Behavioral Contract BC-5.03.014: ux-designer: WCAG 2.1 AA documented per screen

## Description

Every screen file MUST document accessibility requirements at WCAG 2.1 AA minimum
(color contrast, keyboard navigation, ARIA, focus order).

## Preconditions

1. ux-designer is authoring a screen file.

## Postconditions

1. Every screen file has an `## Accessibility` section.
2. The section lists applicable WCAG 2.1 AA criteria.
3. Coverage includes color contrast, keyboard navigation, ARIA, focus order.

## Invariants

1. WCAG 2.1 AA is the documented minimum bar.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Screen lacks Accessibility section | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Screen with full Accessibility section | Accepted | happy-path |
| Screen missing accessibility | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every screen file has an Accessibility section | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/ux-designer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.001 — composes with (accessibility-auditor WCAG citation)
- BC-5.03.013 — composes with (sharded UX)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#ux-designer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/ux-designer.md:86, 117` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit WCAG 2.1 AA rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (screen files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
