---
document_type: behavioral-contract
level: L3
version: "1.2"
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

# Behavioral Contract BC-5.31.007: code-delivery:generate-stubs

## Description

Step `generate-stubs` (line 51; lobster carve-out: stable anchor is step name `generate-stubs`, not line number). Type: agent. Agent: test-writer. Depends: `[create-worktree]`. Source 51-59. Behavior: creates compilable stubs (build passes) without tests or implementation. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `generate-stubs` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. Worktree exists and is on the correct branch.
2. Story spec is available to test-writer.

## Postconditions

1. Stubs exist for the story's surface area.
2. Project build succeeds against the stubs.
3. No tests have been written yet, no implementation logic added.

## Invariants

1. Build is green after stubs are produced.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Stubs cause build break | Step fails before downstream |
| EC-002 | Existing stubs detected | Step augments without duplicating |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| New story, no existing stubs | Stubs created, build green | happy-path |
| Partial existing stubs | Stubs completed | edge-case |
| Build break | Step fails | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Build passes after generate-stubs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.006 — create-worktree (depends on)
- BC-5.31.008 — write-tests (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 51-59) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative step + behavior comment

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (source files) |
| **Global state access** | reads filesystem |
| **Deterministic** | no (depends on agent reasoning) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 51)`) and source range (`Source 51-59.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `generate-stubs`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `generate-stubs`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
