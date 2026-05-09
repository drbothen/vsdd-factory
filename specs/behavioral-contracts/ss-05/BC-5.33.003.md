---
document_type: behavioral-contract
level: L3
version: "1.2"
last_amended: 2026-05-08
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

# Behavioral Contract BC-5.33.003: maintenance: terminal-step

## Description

Terminal step `process-review-decisions` (line 408; lobster carve-out: stable anchor is step name `process-review-decisions`, not line number). Agent: state-manager. Depends `[session-review-approval]`.

## Preconditions

1. session-review-approval resolved.

## Postconditions

1. Review decisions persisted.

## Invariants

1. Single terminal node.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No decisions | No-op |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Decisions present | Committed | happy-path |
| Empty | No-op | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Single-terminal invariant | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.001 — identity

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
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 408; lobster path carve-out: line range is unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | state files |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — desc step `process-review-decisions` (line 408); lobster path lines 408

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
