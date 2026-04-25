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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1121
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

# Behavioral Contract BC-5.07.038: security-reviewer: cite CWE/CVE for every finding

## Description

Every SEC-NNN finding MUST include a specific CWE number (and OWASP category if
applicable). Generic descriptions without CWE are forbidden.

## Preconditions

1. security-reviewer producing a finding.

## Postconditions

1. Every SEC-NNN entry has a non-empty `CWE:` field matching `CWE-\d+`.

## Invariants

1. CWE/CVE citation is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Vulnerability without an exact CWE match | Pick closest CWE; document selection rationale |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| SEC-001 with CWE-79 | Accepted | happy-path |
| SEC-001 without CWE | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every SEC-NNN has CWE field matching CWE-\d+ | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/security-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.039 — composes with (4-tier severity)

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
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:28, 73-85` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit CWE/CVE rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (findings) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
