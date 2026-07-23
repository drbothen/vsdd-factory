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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:391
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

# Behavioral Contract BC-5.03.007: demo-recorder: VHS for CLI, Playwright for web — never plain text captures

## Description

The agent MUST use VHS (`.gif` + `.webm` + `.tape`) for CLI products and
Playwright (`.webm` + `.spec.ts`) for web products. Plain `.txt` captures or
`cargo test` stdout dumps are forbidden as demo evidence.

## Preconditions

1. demo-recorder dispatched.

## Postconditions

1. Every AC has at least one `.gif` or `.webm` artifact.
2. Zero `.txt` files in `docs/demo-evidence/<STORY-ID>/`.

## Invariants

1. Plain text is never demo evidence.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tooling unavailable | Halt with installation request |
| EC-002 | Mixed CLI + web product | Use both VHS and Playwright as appropriate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CLI app demo | `.gif` + `.webm` + `.tape` | happy-path |
| Web app demo | `.webm` + `.spec.ts` | happy-path |
| `.txt` demo attempt | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Zero .txt files in demo-evidence directories | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/demo-recorder.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.006 — composes with (output destination)
- BC-5.03.010 — composes with (VHS Wait+Line discipline)

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
| **Path** | `plugins/vsdd-factory/agents/demo-recorder.md:36, 119-120` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit tooling rules

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (tool execution + recording artifacts) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed UI/CLI) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
