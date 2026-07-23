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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1367
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

# Behavioral Contract BC-5.06.014: story-writer: BC array changes propagate to body and ACs in same atomic commit

## Description

Frontmatter `bcs:` array changes MUST propagate atomically to the body BC table,
AC traces, Token Budget subtable, and any BC-count derivations. Pre-commit
verification reads the story and confirms each BC in the final array appears in
body table AND in at least one AC trace. Failure = HIGH severity blocker.

## Preconditions

1. story-writer modifying a story's `bcs:` frontmatter array.

## Postconditions

1. Body BC table updated to match the new array.
2. AC traces updated.
3. Token Budget subtable updated.
4. Any other BC-count derivations updated.
5. Validate hook (`validate-story-bc-sync.sh`) catches drift.

## Invariants

1. BC array changes are atomic across body and ACs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC removed from array but still referenced in body | Validate hook FAIL |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story commit with synced BC arrays | Validate hook PASS | happy-path |
| Story commit with desynced BC | Validate hook FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | validate-story-bc-sync.sh PASS on every story commit | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/story-writer.md`, validate-story-bc-sync.sh |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.012 — composes with (AC traceability)
- BC-5.06.009 — composes with (anchor-back rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#story-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/story-writer.md:469-487` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit BC Array Propagation Policy

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
