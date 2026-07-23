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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1075
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

# Behavioral Contract BC-5.09.004: research-agent: every claim cited; never relies on training data alone

## Description

Every claim in a research report MUST cite its source (URL, library doc reference,
or explicitly tagged "training data"). MCP tools are mandatory inputs.

## Preconditions

1. research-agent producing a research report.

## Postconditions

1. Every paragraph or claim has an associated source citation.
2. "Research Methods" section confirms MCP tool usage.

## Invariants

1. Citations are mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Best-effort claim from training data | Tag explicitly as "training data" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Claim with URL citation | Accepted | happy-path |
| Uncited claim | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every claim has a source citation | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/research-agent.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.005 — composes with (verified library versions)
- BC-5.09.006 — composes with (Research Methods section)

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
| **Path** | `plugins/vsdd-factory/agents/research-agent.md:50, 53` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit citation rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (web/MCP) + writes (report) |
| **Global state access** | none |
| **Deterministic** | depends on remote |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
