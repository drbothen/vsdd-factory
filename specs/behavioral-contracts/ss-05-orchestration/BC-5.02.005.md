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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:797
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

# Behavioral Contract BC-5.02.005: orchestrator: state-manager runs LAST in every burst

## Description

In every multi-agent burst, state-manager is the FINAL dispatch — after all
other agents have completed. State-manager must not write citations
(STORY-INDEX, BC-INDEX) until story-writer/product-owner have finalized their
version bumps. Running state-manager early causes version-race regressions.

## Preconditions

1. A multi-agent burst is active.

## Postconditions

1. state-manager is dispatched after all other agents in the burst have returned DONE.
2. state-manager never runs concurrently with story-writer or product-owner.

## Invariants

1. The state-manager-LAST ordering prevents version-race regressions.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One agent in the burst BLOCKED | Halt burst; state-manager not dispatched |
| EC-002 | Single-agent burst | state-manager runs after that agent (still last) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Multi-agent burst log | state-manager dispatched after all others returned DONE | happy-path |
| state-manager dispatched concurrently with story-writer | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Burst log shows state-manager as last dispatch in every burst | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.001 — composes with (state-manager git scope)
- BC-5.06.005 — composes with (product-owner BC numbering)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#burst-ordering`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:130` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit ordering rule in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads burst state |
| **Global state access** | reads/writes burst sequence |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
