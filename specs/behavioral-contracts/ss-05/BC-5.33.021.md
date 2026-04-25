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

# Behavioral Contract BC-5.33.021: maintenance:spec-coherence

## Description

Step `spec-coherence` (line 168). Type: agent. Agent: consistency-validator. Depends: `[state-init]`. Source 168-189. Runs 33 spec-coherence checks per DF-030 (criteria 1-23 existing + 24-33 lifecycle).

## Preconditions

1. state-init completed.
2. Spec corpus is present.

## Postconditions

1. 33 coherence checks executed; findings recorded.

## Invariants

1. Exactly 33 checks (criteria 1-23 + 24-33).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All coherent | Empty findings |
| EC-002 | Missing artifact | Findings raise issue |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Coherent corpus | Empty findings | happy-path |
| Drift | Findings | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | 33 distinct checks invoked | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.022 — state-backup-sweep-7

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 168-189) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit count of 33 checks per DF-030

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (specs) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.
