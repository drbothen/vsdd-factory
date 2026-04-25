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

# Behavioral Contract BC-5.32.006: discovery:load-discovery-config

## Description

Step `load-discovery-config` (line 51). Type: agent. Agent: orchestrator. Depends: `[]`. Source 51-60. Loads discovery configuration into workflow state.

## Preconditions

1. Discovery config file is present and readable.

## Postconditions

1. Config object is loaded into workflow context.

## Invariants

1. Step is the workflow root.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Missing config | Step fails |
| EC-002 | Malformed YAML | Step fails |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid config | Loaded | happy-path |
| Bad YAML | Fail | error |
| Missing | Fail | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Step has empty `depends_on` | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.002 — entry-point
- BC-5.32.007 — state-init (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 51-60) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (config) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
