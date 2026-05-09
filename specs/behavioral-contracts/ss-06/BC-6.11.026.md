---
document_type: behavioral-contract
level: L3
version: "1.2"
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
audit_source_id: "BC-AUDIT-448"
ss_section: "Factory operations and dashboards skills"
skill: "maintenance-sweep"
---

# Behavioral Contract BC-6.11.026: maintenance-sweep: Performance regression — >25% triggers PR; 10-25% logs trend; <10% no action

## Description

>25% degradation → open immediate PR (or flag if cause unclear). 10-25% → log with trend data. <10% → no action. Acceptance: `maintenance-sweep/SKILL.md` § "Sweep 5: Performance Regression Detection" — **Action on findings** block.

## Preconditions

1. Performance baseline sweep.

## Postconditions

1. >25% degradation → open immediate PR (or flag if cause unclear). 10-25% → log with trend data. <10% → no action.

## Invariants

1. `maintenance-sweep/SKILL.md` § "Sweep 5: Performance Regression Detection" — **Action on findings** block (three-tier threshold: >25%/10-25%/<10%).

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
| (TBD — to be assigned in Phase 1.6c) | TBD | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | maintenance-sweep |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Factory operations and dashboards skills

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-448 |
| **Source Line(s)** | 99-114 (Sweep 5) |
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

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P20-001: §Description and §Invariants line-range refs migrated to stable section anchor)

**Driver:** F-P20-001 pass-20 extended prose-form sweep — §Description and §Invariants both cited `Threshold table at lines 110-113`, a line-range reference to `maintenance-sweep/SKILL.md` subject to refactor drift.

**Changes made:**
1. §Description: `Acceptance: Threshold table at lines 110-113` → `Acceptance: \`maintenance-sweep/SKILL.md\` § "Sweep 5: Performance Regression Detection" — **Action on findings** block.`
2. §Invariants item 1: `Threshold table at lines 110-113` → `\`maintenance-sweep/SKILL.md\` § "Sweep 5: Performance Regression Detection" — **Action on findings** block (three-tier threshold: >25%/10-25%/<10%).`
3. Frontmatter `version:` bumped `"1.1"` → `"1.2"`.

**Source-of-truth verification (POLICY 4/5):** `grep -n "Action on findings\|Sweep 5" plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md` → `### Sweep 5: Performance Regression Detection` at line 98; `**Action on findings:**` at line 109; three-tier threshold at lines 110-112. Section heading is stable across future SKILL.md edits. Verified lines 110-113 contain the threshold action block (>25%, 10-25%, <10%).
