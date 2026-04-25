---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
subsystem: "SS-05"
capability: "CAP-TBD"
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

# Behavioral Contract BC-5.32.007: discovery:state-init

## Description

Step `state-init` (line 62). Type: agent. Agent: state-manager. Depends: `[load-discovery-config]`. Source 62-69. Initializes the run's STATE.md with the 5 tracked fields.

## Preconditions

1. load-discovery-config completed.

## Postconditions

1. STATE.md initialized with the 5 discovery fields.
2. Subsequent ingestion streams may write to state-backup hooks.

## Invariants

1. State init is exactly-once per run.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Existing state from prior aborted run | Reset/merge per state-manager contract |
| EC-002 | Disk write fail | Escalate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Fresh run | State initialized | happy-path |
| Resume | Existing state honored | edge-case |
| Disk error | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | All 5 STATE.md fields present after step | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.006 — load-discovery-config

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 62-69) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (state) |
| **Global state access** | state files |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
