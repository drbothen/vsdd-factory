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
extracted_from: "plugins/vsdd-factory/workflows/maintenance.lobster"
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

# Behavioral Contract BC-5.33.025: maintenance:accessibility-regression

## Description

Step `accessibility-regression` (line 220; lobster carve-out: stable anchor is step name `accessibility-regression`, not line number). Type: agent. Agent: accessibility-auditor. Depends: `[state-init]`. Condition: `state.has_ui == true`. Source 220-234. Information-asymmetry wall: excludes `.factory/specs/architecture/**`. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `accessibility-regression` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. state-init completed.
2. Project has UI.

## Postconditions

1. Accessibility regression findings recorded.
2. Architecture specs not visible to auditor.

## Invariants

1. Skipped when no UI present.
2. Wall enforced: `.factory/specs/architecture/**` excluded from auditor context.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No UI | Skipped |
| EC-002 | Wall breach | Denied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| UI a11y issue | Findings | happy-path |
| No UI | Skipped | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Architecture specs never appear in auditor context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.026 — state-backup-sweep-9

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 220-234) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: condition + context.exclude

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered) |
| **Global state access** | reads filtered |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 220)`) and source range (`Source 220-234.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `accessibility-regression`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `accessibility-regression`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
