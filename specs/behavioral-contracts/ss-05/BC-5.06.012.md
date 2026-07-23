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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1351
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

# Behavioral Contract BC-5.06.012: story-writer: every AC traces to a BC clause; six context-engineering sections mandatory

## Description

Every AC MUST include `(traces to BC-S.SS.NNN postcondition N)` or
`(... invariant N)` annotation. Every story MUST include all six
context-engineering sections (omitting any degrades downstream agent quality).
Sections marked N/A include explicit "N/A — first story in epic" notes — never omitted.

## Preconditions

1. story-writer authoring a story file.

## Postconditions

1. Every AC line has a `(traces to BC-...)` annotation.
2. Every story file contains all 6 mandatory section headers:
   Token Budget Estimate, Tasks, Previous Story Intelligence, Architecture
   Compliance Rules, Library & Framework Requirements, File Structure Requirements.

## Invariants

1. Six sections are non-optional; N/A is the only allowed empty marker.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | First story in epic (no Previous Story Intelligence) | Section header present with "N/A — first story" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story with all 6 sections + AC traces | Accepted | happy-path |
| Story missing 1 section | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every story file has all 6 section headers | manual |
| VP-TBD | Every AC has a (traces to BC-...) annotation | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/story-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.011 — composes with (one-file-per-story)
- BC-5.06.014 — composes with (BC array propagation)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#story-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/story-writer.md:27, 53, 108-126` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Context-Engineering Sections (ALL MANDATORY)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes |
| **Global state access** | reads BC catalog |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
