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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:407
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

# Behavioral Contract BC-5.03.009: demo-recorder: every recording links to a specific AC via AC-NNN naming

## Description

Filenames MUST follow `AC-NNN-[description].{gif,webm,tape}` (CLI) or
`FLOW-NNN-[description].{webm,spec.ts}` (web). evidence-report.md cross-references
each recording to its AC.

## Preconditions

1. demo-recorder produces a recording.

## Postconditions

1. All recording filenames match the regex `^(AC|FLOW)-\d+-[a-z-]+\.(gif|webm|tape|spec\.ts)$`.
2. evidence-report.md table maps each filename to an AC.

## Invariants

1. Recording naming is canonical and parseable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Filename `demo.gif` (no AC prefix) | Rejected |
| EC-002 | Filename `AC-1-success.gif` (single-digit AC) | Accepted (regex matches) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `AC-001-login-success.gif` | Accepted | happy-path |
| `FLOW-002-checkout.webm` | Accepted | happy-path |
| `demo.txt` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every recording filename matches the canonical regex | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/demo-recorder.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.006 — composes with (output destination)
- BC-5.03.008 — composes with (success/error path)

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
| **Path** | `plugins/vsdd-factory/agents/demo-recorder.md:75, 81` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit naming convention

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (recordings) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
