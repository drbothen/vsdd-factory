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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:261
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

# Behavioral Contract BC-5.07.005: codebase-analyzer: 6-pass protocol with per-pass output files

## Description

The codebase-analyzer MUST execute 6 passes (0=Inventory, 1=Architecture,
2=Domain Model, 3=Behavioral Contracts, 4=NFRs, 5=Conventions, 6=Synthesis) in
order, writing each pass to its own file before proceeding. Holding analysis
only in conversation context is forbidden. Default rules about "do not create
documentation" do NOT apply — file writing IS the task.

## Preconditions

1. codebase-analyzer dispatched.

## Postconditions

1. Pass files `pass-0-inventory.md` through `pass-6-synthesis.md` exist after the
   analyzer completes.
2. Each pass file is written before proceeding to the next pass.
3. Output destination is `.factory/semport/<project>/` or `.factory/phase-0-ingestion/`.

## Invariants

1. The 6-pass order is canonical.
2. Conversation-only output is forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pass interrupted | Resume from checkpoint (per BC-5.07.009) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Full 6-pass run | All 7 files present (0-6) | happy-path |
| Skip pass 3 | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All 7 pass files present after analyzer completes | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/codebase-analyzer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.006 — composes with (no inline output on Write denial)
- BC-5.07.009 — composes with (state checkpoint at end of every pass)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#codebase-analyzer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/codebase-analyzer.md:104-117, 118-306` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Pass 0–6 spec + writing-files-is-the-task rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (source) + writes (pass files) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed source) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
