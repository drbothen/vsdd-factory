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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1589
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

# Behavioral Contract BC-5.03.018: visual-reviewer: distinguishes intentional changes from regressions

## Description

In Feature Mode, the visual-reviewer compares baseline vs current recordings and
labels each visual difference as intentional (new feature element, updated UI) or
regression (layout shift, missing element, broken formatting). Regressions are
reported with timestamps; intentional changes are noted but not flagged.

## Preconditions

1. visual-reviewer running in Feature Mode.
2. Both baseline and current recordings exist.

## Postconditions

1. Feature Mode visual-review.md has rows distinguishing `Regression?` = Yes/No.
2. Intentional changes are noted but not flagged.
3. Regressions are flagged with timestamps for triage.

## Invariants

1. Feature Mode requires baseline comparison.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No baseline recording | TBD — fall back to first-time review |
| EC-002 | Mixed intentional + regression in one demo | Both rows produced |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| New button added (intentional) | `Regression?: No (new feature)` | happy-path |
| Misaligned layout regression | `Regression?: Yes` with timestamp | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every Feature Mode visual diff is labeled Yes/No | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/visual-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.016 — composes with (4-dimensional scoring)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#visual-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/visual-reviewer.md:138-148, 162` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Visual Regression Detection section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (baseline + current recordings) + writes (review report) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed recordings) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
