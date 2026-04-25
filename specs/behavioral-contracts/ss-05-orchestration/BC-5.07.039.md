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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1129
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

# Behavioral Contract BC-5.07.039: security-reviewer: 4-tier severity (CRITICAL/HIGH/MEDIUM/LOW)

## Description

Every finding MUST be classified at one of 4 severity levels.
CRITICAL or HIGH findings block approval until resolved.

## Preconditions

1. security-reviewer producing a finding.

## Postconditions

1. Every SEC-NNN has `Severity:` ∈ {CRITICAL, HIGH, MEDIUM, LOW}.
2. Review verdict APPROVE only when zero unresolved CRITICAL/HIGH findings.

## Invariants

1. Severity enum is closed; CRITICAL/HIGH block approval.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One LOW + zero CRITICAL/HIGH | APPROVE permitted |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Review with one HIGH unresolved | Verdict not APPROVE | error |
| Review with all LOW resolved | APPROVE permitted | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | APPROVE verdict requires zero unresolved CRITICAL/HIGH | manual |

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
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:29, 30` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit 4-tier severity rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (findings) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (classification rule) |

#### Refactoring Notes

No refactoring needed.
