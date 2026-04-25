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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:621
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

# Behavioral Contract BC-5.07.022: formal-verifier: fuzz targets run ≥5 minutes with no crashes

## Description

Fuzz targets MUST run for at least 5 minutes (`-max_total_time=300`) per target
with zero crashes for Phase 5 to converge.

## Preconditions

1. formal-verifier running fuzz tests in Phase 5.

## Postconditions

1. `fuzz-results/` reports each target ran ≥300 seconds.
2. Zero crash artifacts produced.

## Invariants

1. The 5-minute / 0-crash bound is mandatory for Phase 5 convergence.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Crash found at 4 minutes | Fail immediately; do not converge |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 5-minute fuzz run, zero crashes | Phase 5 may converge | happy-path |
| Crash at any point | Phase 5 blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every fuzz-results report shows ≥300s runtime + zero crashes | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/formal-verifier.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.021 — composes with (mutation kill rate)

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
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:60, 132-138, 226-228` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit cargo fuzz max_total_time=300 + Convergence Criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (fuzz-results) |
| **Global state access** | none |
| **Deterministic** | no (random fuzz inputs) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
