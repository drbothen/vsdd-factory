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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1305
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

# Behavioral Contract BC-5.10.002: state-manager: never writes spec documents or source code

## Description

The state-manager writes STATE.md, .factory/ directory structure, cycle-manifest.md,
burst-log.md, convergence-trajectory.md, lessons.md, session-checkpoints.md,
blocking-issues-resolved.md, tech-debt-register.md, cost-summary.md. It MUST NOT
write specs, BCs, VPs, source, tests, configs, or review reports.

## Preconditions

1. state-manager dispatched.

## Postconditions

1. Git diff after state-manager runs shows changes only to the documented set of state files.
2. No `.factory/specs/`, `src/`, or review report writes.

## Invariants

1. State-manager owns state, not content.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to update an index | Allowed if it's a state index (STATE.md), not a spec index (BC-INDEX) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Update STATE.md | Accepted | happy-path |
| Edit BC file | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Diff confined to state files; no .factory/specs/ or src/ entries | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/state-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.001 — composes with (git scope)
- BC-5.10.003 — composes with (STATE.md size cap)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#state-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/state-manager.md:37, 181-183, 401` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "What You NEVER Write" section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (state files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
