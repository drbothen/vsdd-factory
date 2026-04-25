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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:751
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

# Behavioral Contract BC-5.07.033: implementer: status reporting in {DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED}

## Description

Final report uses one of four canonical statuses: DONE / DONE_WITH_CONCERNS /
NEEDS_CONTEXT / BLOCKED. Each status implies a specific dispatcher action.

## Preconditions

1. implementer concluding a story.

## Postconditions

1. Final agent message includes one of the four exact tokens.

## Invariants

1. Status enum is closed.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Status unclear (e.g., "looks good") | Map to DONE or DONE_WITH_CONCERNS |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story complete | `DONE` | happy-path |
| Need clarification | `NEEDS_CONTEXT` | edge-case |
| Blocker | `BLOCKED` | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Final message contains one of four tokens | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.032 — composes with (HALT conditions)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#implementer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/implementer.md:226-235` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Reporting status table

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (status string) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
