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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1145
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

# Behavioral Contract BC-5.07.041: security-reviewer: never dismiss without documented reasoning

## Description

Every dismissed finding (false positive, accepted risk) MUST include an explicit
reasoning line. Silent dismissal is forbidden.

## Preconditions

1. security-reviewer triaging a finding.

## Postconditions

1. Every triage outcome (true positive / false positive / accepted risk) has a
   `reasoning:` field.

## Invariants

1. Silent dismissals are forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Finding is duplicate of prior finding | Dismiss with `reasoning: duplicate of SEC-NNN` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Dismissal with reasoning | Accepted | happy-path |
| Silent dismissal | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every triage outcome has a reasoning field | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/security-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.038 — composes with (CWE/CVE citation)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#security-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:27` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-silent-dismissal rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (triage outcomes) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
