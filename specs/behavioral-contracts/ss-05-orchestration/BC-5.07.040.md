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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1137
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

# Behavioral Contract BC-5.07.040: security-reviewer: cannot see implementer reasoning (information wall)

## Description

Per-story PR review: cannot read implementer notes/logs. Wave integration review:
cannot read per-story PR reviews. The reviewer must form judgments from first
principles, not inherit reasoning.

## Preconditions

1. security-reviewer dispatched (per-story or wave integration).

## Postconditions

1. Tool-call audit shows zero Read against
   `.factory/cycles/**/implementation/implementer-notes*`
   or `.factory/code-delivery/*/review-findings.md`.

## Invariants

1. Independent judgment — no inherited reasoning.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Reviewer needs context | Derive from PR diff and spec only |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| security-reviewer session | Zero Reads on forbidden paths | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/security-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.041 — composes with (no dismissal without reasoning)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#security-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:156-167, 169-178` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Per-Story PR Review Wall + Wave Integration Wall

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
