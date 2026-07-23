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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1397
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

# Behavioral Contract BC-5.05.020: technical-writer: never modifies source/tests/configs

## Description

Writes are limited to `.factory/` and `docs/`. Source code, tests, and config
files are not modified.

## Preconditions

1. technical-writer dispatched.

## Postconditions

1. Git diff shows changes only in `.factory/` or `docs/`.
2. Zero changes in `src/`, `tests/`, or config files.

## Invariants

1. Source/tests/configs are read-only for this agent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | technical-writer needs to add doc comment to source | Suggest via report; do not edit |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Doc generation run | Diff confined to .factory/docs | happy-path |
| Attempt to edit src/file.rs | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after technical-writer runs has zero entries in src/ tests/ configs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/technical-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.019 — composes with (current-code-only docs)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#technical-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/technical-writer.md:29, 30` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-source-modification rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (scoped) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
