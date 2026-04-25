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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:883
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

# Behavioral Contract BC-5.07.034: performance-engineer: never modifies source code — measurement only

## Description

Despite full profile, the agent MUST limit writes to benchmark code, baseline
files, and reports. It MUST NOT modify implementation source.

## Preconditions

1. performance-engineer dispatched.

## Postconditions

1. Git diff after performance-engineer runs shows changes only in `benches/`,
   `tests/perf/`, or `.factory/cycles/**/hardening/`.
2. Zero changes in `src/`.

## Invariants

1. Performance engineering is measurement-only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Optimization opportunity found | Report; do not implement |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bench run | Diff confined to benches/ | happy-path |
| Attempt to optimize src/foo.rs | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after agent runs has zero src/ entries | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/performance-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.035 — composes with (baseline before changes)

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
| **Path** | `plugins/vsdd-factory/agents/performance-engineer.md:83` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit measurement-only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (benches/) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
