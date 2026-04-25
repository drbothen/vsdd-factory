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
extracted_from: "plugins/vsdd-factory/rules/bash.md"
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
audit_id: BC-AUDIT-2203
section: "Rules: bash (no stderr suppression, no eval, dependency checks)"
type: rule
---

# Behavioral Contract BC-8.20.003: rules/bash.md: justfile recipes MUST guard optional tools with `command -v` check

## Description

Every recipe that depends on an optional tool must check availability before running. Pattern: `set -euo pipefail; if ! command -v tool-name &>/dev/null; then echo "tool-name not installed. Run 'just setup' or 'cargo install tool-name --locked'"; exit 1; fi; tool-name actual-command`.

## Preconditions

1. The rule from `plugins/vsdd-factory/rules/bash.md` is being applied to a code change, commit, or artifact.
2. The artifact under review falls within the scope of the rule ("Used by" field below).

## Postconditions

1. Each recipe with external tool has the guard.
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
| Audit ID | BC-AUDIT-2203 |
| Section | Rules: bash (no stderr suppression, no eval, dependency checks) |

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
| **Path** | `plugins/vsdd-factory/rules/bash.md` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |

**Source metadata:** `plugins/vsdd-factory/rules/bash.md` | **Type:** rule | **Confidence:** HIGH | **Source line(s):** 47–60

**Used by:** Project justfile recipes

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

