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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:899
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

# Behavioral Contract BC-5.07.036: performance-engineer: numerical thresholds only, never qualitative

## Description

The agent uses NFR-NNN numerical targets as acceptance criteria. Regression
thresholds: latency p99 increase >10% = WARNING, >25% = CRITICAL; throughput
decrease >10% = WARNING; memory increase >20% = WARNING. Qualitative assessments
are forbidden.

## Preconditions

1. performance-engineer reporting on a change.

## Postconditions

1. performance-report.md uses absolute numbers and explicit deltas.
2. No "fast"/"slow"/"good" qualitative verdicts.

## Invariants

1. All assessments are numerical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | p99 latency +9.5% increase | OK (below WARNING threshold) |
| EC-002 | p99 latency +30% increase | CRITICAL |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Report with p99 +12% (WARNING) | Accepted | happy-path |
| Report saying "looks fast" | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | performance-report.md contains only numerical assessments | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/performance-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.037 — composes with (NFR compliance row)

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
| **Path** | `plugins/vsdd-factory/agents/performance-engineer.md:85, 116-122` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit numerical-only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (numerical evaluation) |

#### Refactoring Notes

No refactoring needed.
