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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1489
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

# Behavioral Contract BC-5.03.013: ux-designer: sharded UX (UX-INDEX + screen + flow files), never monolithic

## Description

UX output is sharded: `UX-INDEX.md` first, then `screens/SCR-NNN-[name].md` per
screen, `flows/FLOW-NNN-[name].md` per flow. Each screen/flow targets 800-1,200
tokens. Monolithic `ux-spec.md` is forbidden.

## Preconditions

1. ux-designer dispatched.

## Postconditions

1. `.factory/specs/ux-spec/UX-INDEX.md` exists.
2. `screens/` and `flows/` subdirectories contain individual files.
3. No monolithic `ux-spec.md`.
4. Each screen/flow file targets 800-1,200 tokens.

## Invariants

1. Sharded layout is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single huge ux-spec.md | Rejected |
| EC-002 | UX-INDEX missing | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Sharded UX directory | Accepted | happy-path |
| Monolithic ux-spec.md | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | UX-INDEX.md and per-screen/per-flow files all present | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/ux-designer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.011 — composes with (screen traces to PRD)
- BC-5.03.012 — composes with (flow success/error)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#ux-designer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/ux-designer.md:97, 178-199` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Sharded UX Output (DF-021)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (sharded UX directory) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
