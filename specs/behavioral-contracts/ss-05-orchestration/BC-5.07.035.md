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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:891
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

# Behavioral Contract BC-5.07.035: performance-engineer: capture baseline BEFORE changes

## Description

Before evaluating any change, the agent MUST capture a baseline measurement
(mean, median, p95, p99, throughput, memory) and write it to
`performance-baseline.md`. Without a baseline, regression detection is impossible.

## Preconditions

1. performance-engineer about to evaluate a change.

## Postconditions

1. `.factory/cycles/**/hardening/performance-baseline.md` exists.
2. The baseline file pre-dates any post-change measurement.

## Invariants

1. No baseline → no regression detection.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Re-running on same change | Re-use prior baseline |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Baseline captured before change | Accepted | happy-path |
| Skip baseline | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | performance-baseline.md exists pre-change | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/performance-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.036 — composes with (numerical thresholds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#performance-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/performance-engineer.md:86, 105-114, 148` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Baseline Measurement rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (baseline.md) |
| **Global state access** | none |
| **Deterministic** | depends on system state |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
