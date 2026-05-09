---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/multi-repo.lobster"
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

# Behavioral Contract BC-5.34.003: multi-repo: terminal-step (primary track)

## Description

Terminal step `process-review-decisions` (line 555). Agent: state-manager. Depends `[session-review-approval]` on the primary track.

## Preconditions

1. session-review-approval resolved on the primary track.

## Postconditions

1. Decisions committed to durable state for the multi-repo project.

## Invariants

1. Single terminal node on the primary track. (Three alternative sub-mode trees `feature_mode`, `bugfix_mode`, `maintenance_mode` defined in `multi-repo.lobster` § sub-mode workflow trees — line range `575-731` cited intentionally as point-in-time evidence pending lobster section-stability verification.) <!-- F-P20-001: lobster-line-class deferred per pass-20 carve-out; line range preserved as source evidence -->

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty | No-op |
| EC-002 | Persistence error | Escalate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Decisions | Persisted | happy-path |
| Empty | No-op | edge-case |
| Error | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Single primary-track terminal | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#multi-repo-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` (line 555) |
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

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P20-001: §Invariants lobster line-range annotated with carve-out deferral)

**Driver:** F-P20-001 pass-20 extended prose-form sweep — §Invariants cited `lines 575-731` referencing `multi-repo.lobster` sub-mode workflow trees. This is a lobster-file reference and falls under the pass-20 lobster-line-class carve-out exception (separate migration class pending lobster section-stability verification).

**Change made:**
- §Invariants item 1: line range `575-731` preserved as point-in-time evidence; file name `multi-repo.lobster` and logical section description (sub-mode workflow trees: `feature_mode`, `bugfix_mode`, `maintenance_mode`) added for reader navigability; HTML comment added citing F-P20-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`; `last_amended:` unchanged (2026-04-25, original extraction date — no structural change to behavioral content).

**Source-of-truth verification (POLICY 4/5):** `grep -n "feature_mode\|bugfix_mode\|maintenance_mode" plugins/vsdd-factory/workflows/multi-repo.lobster` → lines 575, 654, 696 confirmed. Line range 575-731 spans all three sub-mode tree definitions.
