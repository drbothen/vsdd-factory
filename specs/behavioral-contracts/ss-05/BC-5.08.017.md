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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:945
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

# Behavioral Contract BC-5.08.017: pr-manager: never merges with failing CI checks or unmerged dependency PRs

## Description

Merge (Step 8) executes only after CI green AND all upstream dependency PRs
merged. The pr-manager polls each dependency PR via `gh pr view --json state`
and waits if any is unmerged.

## Preconditions

1. pr-manager reaching Step 8 (execute merge).

## Postconditions

1. Merge timestamp is after CI green timestamp AND after all dependency PRs' merged timestamps.

## Invariants

1. Merge requires both CI green AND dep PRs merged.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dep PR closed without merge | Halt with error |
| EC-002 | CI flaky | Wait for stable green |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CI green + deps merged | Merge proceeds | happy-path |
| Dep PR still open | Merge waits | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Merge timestamp > CI green AND all dep merge timestamps | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.014 — composes with (9-step coordinator)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-manager.md:42, 45, 196-209, 213-225` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-merge-without-CI/deps rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (gh) + writes (merge action) |
| **Global state access** | none |
| **Deterministic** | depends on remote |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
