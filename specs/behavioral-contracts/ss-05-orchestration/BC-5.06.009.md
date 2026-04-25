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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1053
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

# Behavioral Contract BC-5.06.009: product-owner: same-burst anchor-back when creating BCs

## Description

Creating new BC files MUST be accompanied in the same burst by: finding stories
whose scope touches the new BC's domain, updating those stories' BC tables, and
updating BC-INDEX. Deferring to a follow-up burst causes empty BC tables — caught
by adversary.

## Preconditions

1. product-owner creating one or more new BC files.

## Postconditions

1. Burst log shows new BC files + updated story BC tables + BC-INDEX update in a single commit.
2. No deferral to follow-up bursts.

## Invariants

1. Anchor-back is same-burst, not optional follow-up.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No existing stories touch the new BC | Document in burst that no story BC tables required updates |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| New BC with same-burst anchor-back | Accepted | happy-path |
| New BC with deferred anchor-back | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Burst commits show co-located BC files + story-table edits + BC-INDEX update | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/product-owner.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.008 — composes with (invariant lifting)
- BC-5.06.014 — composes with (story-writer BC array propagation)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#anchor-back-rule`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:327-335` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Anchor-Back Rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (multiple files in same burst) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
