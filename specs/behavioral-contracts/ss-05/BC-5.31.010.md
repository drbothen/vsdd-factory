---
document_type: behavioral-contract
level: L3
version: "1.3"
last_amended: 2026-05-08
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
subsystem: "SS-05"
capability: "CAP-TBD"
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

# Behavioral Contract BC-5.31.010: code-delivery:implement

## Description

Step `implement` (line 86; lobster carve-out: stable anchor is step name `implement`, not line number). Type: agent. Agent: implementer. Depends: `[red-gate]`. Source 86-95. Behavior: TDD inner loop or gene-transfusion (Semport translation) per `implementation_strategy` input. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `implement` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. red-gate passed.
2. `implementation_strategy` input is one of the supported values (e.g. `tdd-inner-loop`, `semport-gene-transfusion`).

## Postconditions

1. Tests written in write-tests now pass.
2. Implementation source is committed in worktree.

## Invariants

1. Strategy selection is honored (no silent fallback).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | implementation_strategy=semport-gene-transfusion | Use Semport translation path |
| EC-002 | Tests fail after implementation | Step fails or loops per agent contract |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TDD strategy | Tests passing | happy-path |
| Semport strategy | Translated impl, tests passing | edge-case |
| Unknown strategy | Step fails / escalates | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | At step exit, all story tests pass | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.009 — red-gate (depends on)
- BC-5.31.011 — per-story-adversarial-review (downstream)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 86-95; lobster path carve-out: line range is unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative step + behavior

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (source files) |
| **Global state access** | reads filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 86)`) and source range (`Source 86-95.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `implement`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `implement`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster path lines 86-95

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
