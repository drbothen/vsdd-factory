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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:991
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

# Behavioral Contract BC-5.08.022: pr-reviewer: 3-tier severity classification (BLOCKING / WARNING / NIT)

## Description

Every finding MUST be tagged `[BLOCKING]`, `[SUGGESTION]`, or `[NIT]` at the
start of the inline comment. BLOCKING findings prevent APPROVE.

## Preconditions

1. pr-reviewer producing a finding.

## Postconditions

1. Every inline comment begins with one of the three severity tags.
2. APPROVE verdict requires zero `[BLOCKING]` findings.

## Invariants

1. Severity tags are mandatory and exact.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Mid-severity finding | Use [SUGGESTION] |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `[BLOCKING] Bug X` | Accepted | happy-path |
| `Bug X (severe)` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every inline comment starts with [BLOCKING]/[SUGGESTION]/[NIT] | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.020 — composes with (gh pr review)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:39, 159-163` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit severity tag rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (review body) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (tagging) |

#### Refactoring Notes

No refactoring needed.
