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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:315
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

# Behavioral Contract BC-5.05.008: consistency-validator: index-first validation discipline

## Description

The validator MUST load ARCH-INDEX, BC-INDEX, VP-INDEX, STORY-INDEX, L2-INDEX,
UX-INDEX first; detail files only when an index-level issue is detected. It
MUST NOT load `src/` (Phase 6 scope) or holdout-scenarios/evaluations.

## Preconditions

1. consistency-validator dispatched.

## Postconditions

1. Tool-call audit shows index files loaded before any detail files.
2. No Read on `src/` or `.factory/holdout-scenarios/evaluations/`.

## Invariants

1. Index-first discipline preserves context budget.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Index missing | Halt with error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Validator session | Indexes loaded before details | happy-path |
| Read on src/ during validation | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows index-before-detail Read order | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/consistency-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.007 — composes with (80-criteria coverage)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#consistency-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/consistency-validator.md:84-94` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Context Discipline section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
