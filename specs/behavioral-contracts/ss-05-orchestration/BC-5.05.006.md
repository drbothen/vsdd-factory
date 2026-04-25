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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:171
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

# Behavioral Contract BC-5.05.006: architect: VP-locking is 5-step protocol, after which VP is immutable

## Description

The architect specifies VP authoring; formal-verifier owns the lock protocol:
(1) write proof harness, (2) run to completion, (3) on success set
`verification_lock: true` + `proof_completed_date` + `proof_file_hash`,
(4) create git tag `vp-verified-VP-NNN-YYYY-MM-DD`, (5) VP-NNN.md is immutable
thereafter. Issues require withdrawal, not editing. Architect approves withdrawals.

## Preconditions

1. A VP has a completed proof harness.

## Postconditions

1. `verification_lock: true` set.
2. `proof_completed_date` and `proof_file_hash` populated.
3. Git tag `vp-verified-VP-NNN-YYYY-MM-DD` exists on factory-artifacts branch.
4. VP-NNN.md is immutable thereafter (no body edits).

## Invariants

1. Locked VP is append-only; modifications require withdrawal.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Locked VP has issue requiring fix | Submit withdrawal; architect approves |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Successful proof completion | Lock + tag created | happy-path |
| Edit to locked VP | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git log on factory-artifacts shows zero edits to locked VP files after lock date | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/formal-verifier.md`, `plugins/vsdd-factory/agents/architect.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.011 — composes with (VP withdrawal requires architect approval)
- BC-5.05.017 — composes with (spec-steward locked VP enforcement)

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
| **Path** | `plugins/vsdd-factory/agents/architect.md:104-112`; `plugins/vsdd-factory/agents/formal-verifier.md:71-83` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit VP Locking Protocol

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (VP file + git tag) |
| **Global state access** | reads/writes git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
