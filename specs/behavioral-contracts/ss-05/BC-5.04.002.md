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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:85
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

# Behavioral Contract BC-5.04.002: adversary: every finding tagged with HIGH/MEDIUM/LOW confidence

## Description

Every adversarial finding MUST carry a confidence tag (HIGH/MEDIUM/LOW). HIGH
requires file path + line number + failure rationale; MEDIUM is pattern-based;
LOW is inferred-from-absence.

## Preconditions

1. adversary producing a finding.

## Postconditions

1. Every finding entry has a `Confidence:` field set to HIGH, MEDIUM, or LOW.
2. HIGH findings include file path + line number + failure rationale.
3. MEDIUM findings are pattern-based with explicit pattern citation.
4. LOW findings are inferred-from-absence with explanation.

## Invariants

1. Confidence is non-optional.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Finding lacks Confidence field | Rejected |
| EC-002 | HIGH finding without line number | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| HIGH finding with full evidence | Accepted | happy-path |
| MEDIUM finding referencing pattern | Accepted | happy-path |
| Finding without Confidence | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every finding has Confidence ∈ {HIGH, MEDIUM, LOW} | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.003 — composes with (mis-anchoring severity)

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
| **Path** | `plugins/vsdd-factory/agents/adversary.md:116-124` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Confidence Levels table

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data classification) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (classifier) |

#### Refactoring Notes

No refactoring needed.
