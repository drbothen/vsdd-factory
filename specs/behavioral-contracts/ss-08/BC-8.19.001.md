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
extracted_from: "plugins/vsdd-factory/rules/_index.md"
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
audit_id: BC-AUDIT-2200
section: "Rules index (_index.md include order)"
type: rule
---

# Behavioral Contract BC-8.19.001: rules/_index.md: rule include-order via @-references

## Description

Defines Claude Code rule import order. `@git-commits.md`, `@rust.md`, `@bash.md`, `@story-completeness.md`, `@factory-protocol.md`, `@spec-format.md`, `@worktree-protocol.md` (7 entries). Order is the explicit precedence: git-commits first, worktree-protocol last.

## Preconditions

1. The rule from `plugins/vsdd-factory/rules/_index.md` is being applied to a code change, commit, or artifact.
2. The artifact under review falls within the scope of the rule ("Used by" field below).

## Postconditions

1. All 7 `@<filename>` entries present in this exact order.
2. Artifact MUST satisfy the rule mandate; violations are blocked or flagged.

## Invariants

1. Rule applies uniformly to all in-scope artifacts; no silent exceptions.
2. Rule MUST be enforceable mechanically (lint, CI, structural test) where stated.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — derive from source file edge cases | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Artifact compliant with rule | Lint/check passes | happy-path |
| Artifact violating rule | Lint/check fails with diagnostic | error |
| Artifact with documented exemption | Check passes (TBD) | edge-case |

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
| Audit ID | BC-AUDIT-2200 |
| Section | Rules index (_index.md include order) |

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
| **Path** | `plugins/vsdd-factory/rules/_index.md` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |

**Source metadata:** `plugins/vsdd-factory/rules/_index.md` | **Type:** rule | **Confidence:** HIGH | **Source line(s):** 1–9

**Used by:** Claude Code rule loader

#### Evidence Types Used

- **documentation**: MUST/SHALL mandate stated in rules file
- **assertion**: where indicated, rule is enforced by a structural or CI test

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (when rule is mechanically enforced via a checker) |
| **Global state access** | reads repository state |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure when rule check is deterministic; documentation otherwise |

#### Refactoring Notes

Where rule has documented enforcement (CI hook, structural test), promote to VP. Where rule is documentation-only, note as advisory in the lifecycle table.

