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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1091
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

# Behavioral Contract BC-5.09.006: research-agent: mandatory Research Methods section per report

## Description

Every research report MUST conclude with a `## Research Methods` table showing
tool name, query count, and purpose, plus total MCP tool calls and training-data
reliance level (low/medium/high). The section is non-negotiable.

## Preconditions

1. research-agent finalizing a report.

## Postconditions

1. Every file under `.factory/specs/research/` ends with a `## Research Methods` section.
2. The section follows the documented schema (tool, count, purpose, total, reliance).

## Invariants

1. Research Methods is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single-tool research | Still include the section |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Report with Research Methods section | Accepted | happy-path |
| Report missing the section | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every research file has Research Methods section | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/research-agent.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.004 — composes with (citation rule)

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
| **Path** | `plugins/vsdd-factory/agents/research-agent.md:65-89` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Research Methods (MANDATORY)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
