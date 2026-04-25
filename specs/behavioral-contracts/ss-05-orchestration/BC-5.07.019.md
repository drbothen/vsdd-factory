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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:589
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

# Behavioral Contract BC-5.07.019: formal-verifier: never marks VP verified without running proof to completion

## Description

Setting `verification_lock: true` and `status: verified` on a VP requires that
the proof harness ran to completion (e.g., `cargo kani --harness` exited 0).
Speculative verification is forbidden.

## Preconditions

1. formal-verifier processing a VP.

## Postconditions

1. Each verified VP file has matching evidence in `kani-results/` showing successful completion.
2. Speculative verification is rejected.

## Invariants

1. Verification is empirical, not assumed.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Proof harness times out | VP not marked verified |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cargo kani --harness` exit 0 + result file | VP can be verified | happy-path |
| Verification without proof completion | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every verified VP has matching evidence in kani-results/ | manual |

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

- `architecture/SS-05-orchestration.md#formal-verifier`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:39, 248` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "run to completion" rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (proof execution and result) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed harness) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
