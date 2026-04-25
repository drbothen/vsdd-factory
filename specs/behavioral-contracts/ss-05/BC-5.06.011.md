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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1343
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

# Behavioral Contract BC-5.06.011: story-writer: one file per story, never monolithic

## Description

Stories MUST be written one-per-file as `STORY-NNN-[short].md` under
`.factory/stories/stories/`. A monolithic `stories.md` is forbidden.
STORY-INDEX.md aggregates references.

## Preconditions

1. story-writer dispatched.

## Postconditions

1. `.factory/stories/stories/` contains individual story files.
2. No monolithic `stories.md` exists.
3. STORY-INDEX.md lists all stories.

## Invariants

1. One-file-per-story is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single-story project | Still one file under stories/ + STORY-INDEX |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Per-story files + STORY-INDEX | Accepted | happy-path |
| Monolithic stories.md | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | `.factory/stories/stories/` has individual files; no monolithic | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/story-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.012 — composes with (AC traceability)
- BC-5.06.013 — composes with (story sizing)

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
| **Path** | `plugins/vsdd-factory/agents/story-writer.md:26, 41-49, 501` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-monolithic rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (per-story files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
