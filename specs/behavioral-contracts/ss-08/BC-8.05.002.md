---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4-b-bcs-agent-10
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-deep-templates-tools-rules.md
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/templates/holdout-scenario-index-template.md"
subsystem: SS-08
capability: ""
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
audit_id: BC-AUDIT-1848
section: "Holdout-evaluation templates"
type: template
---

# Behavioral Contract BC-8.05.002: holdout-scenario-index-template: HS-INDEX scenario catalog

## Description

Frontmatter `document_type: holdout-scenario-index`, `level: ops`. Required: `## Scenario Catalog`, `## Category Distribution`, `## Wave Holdout Scenarios (cycle-scoped)`.

## Preconditions

1. Template file at `plugins/vsdd-factory/templates/holdout-scenario-index-template.md` is being authored or validated.
2. Authoring agent or validation tool is reading the template definition.

## Postconditions

1. All 3 `##` headings present.
2. Template-conforming document parses cleanly and is consumable by skills listed in "Used by".

## Invariants

1. Frontmatter and required section headings remain stable across artifact instances.
2. Template identity (document_type, level) MUST NOT change without versioning.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — derive from source file edge cases | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Document conforming to template | Validation passes | happy-path |
| Document missing a required section | Validation fails with clear error | error |
| Document with optional/recommended sections only | Validation passes | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — promote acceptance criterion to a structural/lint test | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-08 (Templates and Rules) |
| Stories | TBD |
| Audit ID | BC-AUDIT-1848 |
| Section | Holdout-evaluation templates |

## Related BCs (Recommended)

- TBD — populate during cross-pass synthesis

## Architecture Anchors (Recommended)

- `architecture/SS-08-templates-rules.md` — TBD

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/templates/holdout-scenario-index-template.md` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |

**Source metadata:** `plugins/vsdd-factory/templates/holdout-scenario-index-template.md` | **Type:** template | **Confidence:** HIGH | **Source line(s):** 1–3

**Used by:** decompose-stories (state-manager bookkeeping; no direct skill cross-walk)

#### Evidence Types Used

- **documentation**: structural requirements declared in template file (frontmatter + section headings)
- **type constraint**: enforced via document_type/level keys in YAML frontmatter

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (template is a static schema) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure (schema definition) |

#### Refactoring Notes

No refactoring needed — template is a passive schema. Verification can be promoted from "document parses" to "all structural fields type-checked."

