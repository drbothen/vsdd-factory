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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:93
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

# Behavioral Contract BC-5.04.003: adversary: mis-anchoring always blocks convergence

## Description

Findings about semantic anchor mismatches (capability anchors, subsystem IDs,
VP anchor stories, BC cross-references, module names, file paths) MUST be
classified as CRITICAL/HIGH/MEDIUM/LOW per the severity table — never as
"Observation" or deferred. They block convergence.

## Preconditions

1. adversary detects a mis-anchor finding.

## Postconditions

1. Finding is classified at one of CRITICAL/HIGH/MEDIUM/LOW severity.
2. Finding appears under Critical Findings or Important Findings section.
3. Finding never appears under Observations.
4. Convergence is blocked while the mis-anchor remains unresolved.

## Invariants

1. Mis-anchoring NEVER as Observation; NEVER deferred post-v1.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Trivially small anchor mismatch | Still blocks (LOW severity) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Mis-anchor finding tagged HIGH | Blocks convergence | happy-path |
| Mis-anchor classified as Observation | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No mis-anchor finding has severity Observation or status deferred | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.002 — composes with (confidence tagging)
- BC-5.05.010 — composes with (consistency-validator mis-anchoring rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#adversary`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/adversary.md:99-114` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Semantic Anchoring Audit section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (classification) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
