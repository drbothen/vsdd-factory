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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:521
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

# Behavioral Contract BC-5.07.012: dx-engineer: blocks pipeline when any of 3 model families unreachable

## Description

Pre-pipeline preflight MUST verify Claude Sonnet, Claude Opus, and adversary
model + review-tier model are all reachable. ANY unreachable model BLOCKS the
pipeline. Silent fallback to a different model is forbidden — human approval
required for substitution.

## Preconditions

1. dx-engineer running pre-pipeline preflight.

## Postconditions

1. Preflight report has 4 model rows all `healthy`.
2. If any `UNAVAILABLE`, pipeline state shows BLOCKED with notification to human.

## Invariants

1. No silent model fallback.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Adversary model temporarily unavailable | Pipeline BLOCKED; human approves substitution |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All 4 models healthy | Pipeline proceeds | happy-path |
| Opus unavailable | Pipeline BLOCKED | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Preflight report has all 4 model rows; any UNAVAILABLE → BLOCKED | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dx-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.011 — composes with (no key value logging)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dx-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/dx-engineer.md:41, 156-181` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit LLM Availability Check section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (model APIs) + writes (preflight report) |
| **Global state access** | none |
| **Deterministic** | no (depends on remote API state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
