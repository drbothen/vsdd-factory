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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1007
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

# Behavioral Contract BC-5.08.024: pr-reviewer: demo evidence in `.gif`/`.webm`, not `.txt`

## Description

The pr-reviewer MUST verify each AC has ≥1 `.gif` or `.webm` recording AND both
success and error paths. Plain text demos are flagged BLOCKING.

## Preconditions

1. pr-reviewer evaluating a PR with story scope.

## Postconditions

1. Review reports BLOCKING when any AC has only `.txt` demos or missing recordings.

## Invariants

1. Demo evidence quality is enforced at PR review.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | AC has both .gif and .txt | Pass (gif present); .txt is harmless extra |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Each AC has .gif or .webm | Pass | happy-path |
| AC has only .txt demo | BLOCKING | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Reviews flag BLOCKING for .txt-only or missing demos | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.007 — composes with (demo-recorder VHS/Playwright tooling)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:112-115` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit demo evidence rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (demo files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
