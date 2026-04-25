---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:400"
subsystem: "SS-01"
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

# Behavioral Contract BC-1.07.002: `commit.made` events fire reliably on real Claude Code git commit

## Description

When an operator runs a `git commit` invocation in a real Claude Code session with vsdd-factory beta.3+ activated, a `commit.made` event lands in `factory-events-*.jsonl`. CHANGELOG-pinned by v1.0.0-beta.3 confirmation against four real commits (4fd662ab, 400fedb5, 7617214d, 3fe36e4b).

## Preconditions

1. Operator runs git commit in a real CC session.
2. vsdd-factory beta.3+ is activated.

## Postconditions

1. `commit.made` event lands in `factory-events-*.jsonl`.

## Invariants

1. The dispatcher pipeline emits `commit.made` for every successful real-harness git commit.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Real CC session git commit | `commit.made` event in factory-events-*.jsonl | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 + SS-04 + SS-07 — dispatcher pipeline + capture-commit-activity hook |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | CHANGELOG v1.0.0-beta.3 with explicit commit shas (4fd662ab, 400fedb5, 7617214d, 3fe36e4b) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `400` |

#### Evidence Types Used

- documentation (CHANGELOG with provenance)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
