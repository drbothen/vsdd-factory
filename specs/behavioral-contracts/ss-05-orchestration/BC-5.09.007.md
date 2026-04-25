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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1099
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

# Behavioral Contract BC-5.09.007: research-agent: never overwrites prior research — appends new dated file

## Description

Research output is always appended as a new dated file. RESEARCH-INDEX.md is
updated with a new row. Overwriting prior research is forbidden.

## Preconditions

1. research-agent producing a research report.

## Postconditions

1. Research dir has multiple dated files (one per run).
2. Index shows monotonic date growth.
3. No file overwrites in git history.

## Invariants

1. Research is append-only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Same topic re-researched | New dated file; both kept |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| New file `domain-foo-2026-04-25.md` | Accepted | happy-path |
| Overwrite of prior file | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No file overwrites in git history of research/ | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/research-agent.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.006 — composes with (Research Methods)

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
| **Path** | `plugins/vsdd-factory/agents/research-agent.md:31, 32-33` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit dated-file rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (new file) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
