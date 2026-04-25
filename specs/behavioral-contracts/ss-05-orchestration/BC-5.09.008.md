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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1107
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

# Behavioral Contract BC-5.09.008: research-agent: no source code modification, no Bash

## Description

Research output is markdown only. Bash, exec, process are denied. Writes limited
to `.factory/planning/`, `.factory/specs/research/`, or
`.factory/specs/domain-research.md`.

## Preconditions

1. research-agent dispatched.

## Postconditions

1. Tool profile excludes Bash.
2. Git diff shows zero changes outside research output paths.

## Invariants

1. Research is markdown-only; no shell.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to scan local code | Use Read/Grep/Glob, not Bash |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Research session | Tools = {Read, Write, Edit, Glob, Grep, WebSearch, WebFetch, MCP} | happy-path |
| Attempt to use Bash | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool profile excludes Bash; diff confined to research paths | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/research-agent.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.007 — composes with (append-only research files)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#research-agent`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/research-agent.md:49, 144-148` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Tool Access denial of Bash

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (web/MCP) + writes (markdown) |
| **Global state access** | none |
| **Deterministic** | depends on remote |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
