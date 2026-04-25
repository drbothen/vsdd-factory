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
extracted_from: "plugins/vsdd-factory/workflows/maintenance.lobster"
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

# Behavioral Contract BC-5.33.008: maintenance:dependency-audit-scan

## Description

Step `dependency-audit-scan` (line 50). Type: agent. Agent: dx-engineer. Depends: `[state-init]`. Timeout: 15m. Source 50-62.

## Preconditions

1. state-init completed.

## Postconditions

1. Dependency vulnerability scan output produced.

## Invariants

1. Bounded by 15m timeout.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tool unavailable | Skip per defaults |
| EC-002 | Timeout | Skip per defaults |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Standard | Scan output | happy-path |
| Slow scan | Timeout | edge-case |
| Tool error | Skip | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Bounded by 15m | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.009 — dependency-audit-analysis

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 50-62) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: timeout

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | filesystem |
| **Deterministic** | yes (given lockfile) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
