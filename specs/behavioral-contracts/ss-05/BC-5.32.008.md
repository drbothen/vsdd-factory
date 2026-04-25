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

# Behavioral Contract BC-5.32.008: discovery:feature-research

## Description

Step `feature-research` (line 76). Type: skill. Skill: `skills/discovery-engine/SKILL.md`. Depends: `[state-init]`. Condition: `config.feature_discovery.enabled == true`. Source 76-82.

## Preconditions

1. state-init completed.
2. feature_discovery is enabled in config.

## Postconditions

1. Feature research findings written to discovery output.

## Invariants

1. Step skipped when feature_discovery is disabled.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Feature discovery disabled | Skipped |
| EC-002 | Skill unavailable | Fail |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Enabled | Findings produced | happy-path |
| Disabled | Skipped | edge-case |
| Skill error | Fail | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Step gated by config flag | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.009 — state-backup-feature-research

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 76-82) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes + network |
| **Global state access** | filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
