---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "codebase-analyzer"
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: TBD
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md"
subsystem: "SS-06"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
audit_source_id: "BC-AUDIT-536"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f2-spec-evolution"
---

# Behavioral Contract BC-6.17.009: phase-f2-spec-evolution: PRD delta appends new BCs continuing BC-S.SS.NNN sequence; modified BCs marked UPDATED with previous version inline

## Description

product-owner appends new BCs using BC-S.SS.NNN format (DF-020 4-level hierarchy). Continues numbering. Modifies existing BCs with UPDATED tag and previous version inline. Does NOT rewrite or restructure unaffected requirements. Writes `.factory/phase-f2-spec-evolution/prd-delta.md`. Acceptance: Append-only with UPDATED tag and inline previous version.

## Preconditions

1. Step 2 runs.

## Postconditions

1. product-owner appends new BCs using BC-S.SS.NNN format (DF-020 4-level hierarchy). Continues numbering. Modifies existing BCs with UPDATED tag and previous version inline. Does NOT rewrite or restructure unaffected requirements. Writes `.factory/phase-f2-spec-evolution/prd-delta.md`.

## Invariants

1. Append-only with UPDATED tag and inline previous version.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | TBD | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | phase-f2-spec-evolution |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Feature-mode phase skills (f1-f7)

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-536 |
| **Source Line(s)** | 30-43 (Step 2: PRD Delta), 165-167 (Quality Gate) |
| **Source File** | `.factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md` |

#### Evidence Types Used

- **documentation**: extracted from SKILL.md frontmatter and Quality Gate sections

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (skill orchestrates filesystem and agent dispatch) |
| **Global state access** | reads global (STATE.md, .factory/ tree) |
| **Deterministic** | no -- depends on agent execution and human approval |
| **Thread safety** | not thread-safe |
| **Overall classification** | effectful shell |

#### Refactoring Notes

This BC describes a skill-level workflow contract. The acceptance criteria
encode the Quality Gate checks performed by the skill; these can be lifted
into automated assertions where the skill's underlying procedure is
deterministic. Adversarial and human-gated steps are explicitly opaque.
