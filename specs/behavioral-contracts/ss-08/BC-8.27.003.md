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
extracted_from: "plugins/vsdd-factory/rules/worktree-protocol.md"
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
audit_id: BC-AUDIT-2279
section: "Rules: worktree-protocol (branch hierarchy, merge protocol)"
type: rule
---

# Behavioral Contract BC-8.27.003: rules/worktree-protocol.md: worktree creation — `git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN-<desc> develop`

## Description

Always branch from `develop`. Branch name must match pattern `feature/STORY-NNN-<short-description>`. ONE worktree per story — never share worktrees between stories.

## Preconditions

1. The rule from `plugins/vsdd-factory/rules/worktree-protocol.md` is being applied to a code change, commit, or artifact.
2. The artifact under review falls within the scope of the rule ("Used by" field below).

## Postconditions

1. Worktree commands match pattern; branch names match `feature/STORY-NNN-<desc>`.
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
| VP-001 | TBD — promote acceptance criterion to a structural/lint test | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-08 (Templates and Rules) |
| Stories | TBD |
| Audit ID | BC-AUDIT-2279 |
| Section | Rules: worktree-protocol (branch hierarchy, merge protocol) |

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
| **Path** | `plugins/vsdd-factory/rules/worktree-protocol.md` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |

**Source metadata:** `plugins/vsdd-factory/rules/worktree-protocol.md` | **Type:** rule | **Confidence:** HIGH | **Source line(s):** 32–38

**Used by:** code-delivery skill

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

