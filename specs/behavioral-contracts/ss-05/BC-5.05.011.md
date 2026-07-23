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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:605
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

# Behavioral Contract BC-5.05.011: formal-verifier: VP withdrawal requires architect approval

## Description

When a verified VP's proof is found invalid, the formal-verifier produces a
withdrawal document and submits to architect. The VP is marked `status: withdrawn`
only after architect approval. Silent removal is forbidden.

## Preconditions

1. A verified VP's proof is found invalid OR otherwise needs withdrawal.

## Postconditions

1. A withdrawal document is produced.
2. Withdrawal document is submitted to architect for approval.
3. VP is marked `status: withdrawn` only after architect approves.
4. Every withdrawn VP has a corresponding withdrawal document and architect approval annotation.

## Invariants

1. No silent removal of verified VPs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Architect rejects withdrawal | VP stays in current state; document outcome |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Withdrawal with architect approval | VP marked withdrawn | happy-path |
| VP silently deleted | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every withdrawn VP has a withdrawal document and architect-approval annotation | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/formal-verifier.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.006 — composes with (VP-locking 5-step protocol)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#vp-withdrawal`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:41, 85-93` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit VP Withdrawal Initiation section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (withdrawal document) |
| **Global state access** | reads/writes VP status |
| **Deterministic** | no (depends on architect approval) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
