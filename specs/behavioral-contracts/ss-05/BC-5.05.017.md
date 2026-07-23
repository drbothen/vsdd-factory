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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1275
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

# Behavioral Contract BC-5.05.017: spec-steward: locked VP enforcement (immutable after lock)

## Description

VP files with `verification_lock: true` MUST NOT be edited. Any changes require
the VP withdrawal process. Locked VPs are flagged for re-assessment if their
source BC has a MAJOR version bump.

## Preconditions

1. A VP file has `verification_lock: true`.

## Postconditions

1. Git history on factory-artifacts shows zero edits to the VP file after lock date
   (except via formal withdrawal documents).
2. Major version bump of the source BC flags the VP for re-assessment.

## Invariants

1. Locked VPs are append-only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Source BC has MAJOR bump | Flag locked VP for re-assessment |
| EC-002 | Locked VP needs typo fix | Use withdrawal process |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Edit to locked VP | Audit failure | error |
| Withdrawal of locked VP | Allowed via formal process | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git history shows zero post-lock edits to locked VPs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-steward.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.006 — composes with (architect VP-locking protocol)
- BC-5.05.011 — composes with (VP withdrawal process)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#vp-locking`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/spec-steward.md:102-110` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit L4 Immutability Enforcement section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (git history) |
| **Global state access** | reads VP lock state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
