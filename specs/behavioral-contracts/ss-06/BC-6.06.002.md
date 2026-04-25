---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L453"
subsystem: SS-06
capability: "TBD"
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

# Behavioral Contract BC-6.06.002: check-input-drift: scan via single binary, not inline shell loops

> Source: `pass-3-deep-skills-batch-1.md` line 453 (was `BC-AUDIT-240`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

check-input-drift: scan via single binary, not inline shell loops. Runs `${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory`. Skill MUST NOT use inline bash loops — Claude Code harness auto-backgrounds multi-line commands and kills before output flushes. Output: `TOTAL=N MATCH=N STALE=N UNCOMPUTED=N NOINPUT=N UPDATED=0 UPDATE_FAILED=0`. Exit 0=clean, 2=drift.

## Preconditions

1. Step 1 of skill

## Postconditions

1. Single-command binary invocation; output format matches; INDEX files are skipped by binary.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD — happy path from skill acceptance | TBD | happy-path |
| TBD — edge case | TBD | edge-case |
| TBD — error case | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Single-command binary invocation; output format matches; INDEX files are skipped by binary." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/check-input-drift/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#check-input-drift-scan-via-single-binary,-not-inline-shell-loops` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/check-input-drift/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 31-58 |

#### Evidence Types Used

- documentation: stated in SKILL.md frontmatter and prose
- inferred: behavior derived from skill acceptance criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — assess once architecture mapping is complete.

#### Source Excerpt (verbatim)

```text
#### BC-AUDIT-240 — check-input-drift: scan via single binary, not inline shell loops

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 31-58
**Trigger:** Step 1 of skill
**Behavior:** Runs `${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory`. Skill MUST NOT use inline bash loops — Claude Code harness auto-backgrounds multi-line commands and kills before output flushes. Output: `TOTAL=N MATCH=N STALE=N UNCOMPUTED=N NOINPUT=N UPDATED=0 UPDATE_FAILED=0`. Exit 0=clean, 2=drift.
**Acceptance:** Single-command binary invocation; output format matches; INDEX files are skipped by binary.
```
