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
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
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

# Behavioral Contract BC-5.31.003: code-delivery: terminal-step

## Description

`cleanup-worktree` is the terminal step of `code-delivery.lobster` § terminal-step — line `430`, source range `430-437` cited as point-in-time evidence pending lobster section-stability verification. Type: agent, agent: devops-engineer, depends_on: `[merge-pr, delivery-human-approval]`. <!-- F-P21-001: lobster-line-class deferred per pass-21 retroactive sweep; line refs preserved as source evidence -->

## Preconditions

1. `merge-pr` has completed.
2. `delivery-human-approval` has been resolved (approved or auto-approved).

## Postconditions

1. Worktree is cleaned up; no orphan branches/working trees remain.
2. Sub-workflow returns to caller.

## Invariants

1. Exactly one step has no successors (`cleanup-worktree`).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Worktree cleanup fails | Escalate per defaults; do not silently ignore |
| EC-002 | Story merged with auto-merge | delivery-human-approval skipped, cleanup still runs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Successful merge + approval | cleanup runs and completes | happy-path |
| Auto-merge path | cleanup runs without human approval gate | edge-case |
| Cleanup fails | escalation event raised | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | DAG has exactly one terminal node | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.001 — identity
- BC-5.31.028 — cleanup-worktree per-step

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 430-437) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative workflow step

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (filesystem) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P21-001: §Description lobster line-refs annotated with carve-out deferral)

**Driver:** F-P21-001 pass-21 retroactive sibling sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited `code-delivery.lobster` terminal step by line number (`line 430`, `lines 430-437`). These are lobster-file references and fall under the lobster-line-class carve-out exception pending lobster section-stability verification.

**Change made:**
- §Description: line refs preserved as point-in-time evidence; `code-delivery.lobster` file name and logical section (terminal-step) made explicit; HTML comment added citing F-P21-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
