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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:415
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

# Behavioral Contract BC-5.03.010: demo-recorder: VHS tapes use Wait+Line, not Sleep

## Description

VHS tapes MUST use `Wait+Line /pattern/` for command completion synchronization.
`Sleep` is permitted only for the final 2s frame hold.

## Preconditions

1. demo-recorder authoring a VHS `.tape` file.

## Postconditions

1. `.tape` files contain `Wait+Line` directives for command completion.
2. Only one `Sleep 2s` is allowed (the final frame hold).

## Invariants

1. `Sleep` for command-completion synchronization is forbidden — produces flaky timing.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tape uses `Sleep 5s` between commands | Rejected |
| EC-002 | Tape ends with `Sleep 2s` | Accepted (final hold) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Tape with `Wait+Line /done/` then `Sleep 2s` | Accepted | happy-path |
| Tape with multiple `Sleep` directives | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every .tape file has at most one Sleep directive | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/demo-recorder.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.007 — composes with (VHS for CLI tooling)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#demo-recorder`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/demo-recorder.md:103-104` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit timing-discipline rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (.tape file) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (file authoring) |

#### Refactoring Notes

No refactoring needed.
