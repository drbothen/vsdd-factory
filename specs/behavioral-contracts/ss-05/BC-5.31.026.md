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

# Behavioral Contract BC-5.31.026: code-delivery:merge-pr

## Description

Step `merge-pr` (line 407; lobster carve-out: stable anchor is step name `merge-pr`, not line number). Type: agent. Agent: pr-manager. Depends: `[dependency-merge-check]`. Source 407-416. Reads `.factory/merge-config.yaml` for autonomy level (Level 3 = label only; 3.5 = auto-merge low risk; 4 = auto-merge with squash). <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `merge-pr` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. dependency-merge-check passed.
2. `.factory/merge-config.yaml` is readable and well-formed.

## Postconditions

1. Action taken matches autonomy level: label-only at L3, auto-merge low risk at L3.5, auto-merge squash at L4.

## Invariants

1. Autonomy level reading is mandatory; never auto-merges without explicit config.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Level 3 | Apply label, do not merge |
| EC-002 | Level 3.5 + low risk | Auto-merge |
| EC-003 | Level 4 | Auto-merge with squash |
| EC-004 | Missing config | Fail |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| L3 | Label only | happy-path |
| L3.5 + risk metric low | Auto-merge | edge-case |
| Missing config | Fail | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Action matches autonomy level table | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.025 — dependency-merge-check
- BC-5.31.027 — delivery-human-approval (downstream conditional)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`
- `architecture/ss-09-config-activation.md#merge-config`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 407-416) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: behavior comment + config reference

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads config + network (PR merge API) |
| **Global state access** | filesystem + git host |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 407)`) and source range (`Source 407-416.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `merge-pr`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `merge-pr`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
