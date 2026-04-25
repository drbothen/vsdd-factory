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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1543
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

# Behavioral Contract BC-5.09.018: validate-extraction: never modifies source code

## Description

Despite Bash access, source code is read-only — Bash is for inspection (find,
wc, grep, ls), never modification.

## Preconditions

1. validate-extraction dispatched.

## Postconditions

1. Git diff after validate-extraction runs shows changes only in
   `.factory/phase-0-ingestion/` validation reports.
2. Zero source modifications.

## Invariants

1. Source is read-only for this agent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to fix typo in source | Document; do not patch |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Validation run | Diff confined to validation reports | happy-path |
| Attempt to edit src/ | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after validate-extraction has no source changes | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.014 — composes with (phase split)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#validate-extraction`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:141, 147-150` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Bash-for-inspection rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (via Bash) + writes (reports) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
