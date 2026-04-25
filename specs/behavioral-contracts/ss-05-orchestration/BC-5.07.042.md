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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1153
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

# Behavioral Contract BC-5.07.042: security-reviewer: supply chain audit ANY finding blocks installation

## Description

During tool installation audits (CVE/NVD/OSV + Perplexity recent compromise +
integrity check), ANY finding regardless of severity blocks the installation.
Human approval is required to override.

## Preconditions

1. security-reviewer running a supply-chain audit.

## Postconditions

1. Every audit report ending with `VERDICT: FINDING -- human approval required`
   blocks the dx-engineer's install step until human override.

## Invariants

1. Supply-chain audits are zero-tolerance.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single LOW finding | Still blocks installation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CLEAN audit | Install proceeds | happy-path |
| Audit with one LOW finding | Install BLOCKED | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Any FINDING verdict blocks install pending human override | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/security-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.013 — composes with (dx-engineer security audit)

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
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:195-198` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Supply Chain Security Audit rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (registries) + writes (audit) |
| **Global state access** | none |
| **Deterministic** | no (depends on registry state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
