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
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
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

# Behavioral Contract BC-5.32.010: discovery:customer-feedback-ingestion

## Description

Step `customer-feedback-ingestion` (line 96; lobster carve-out: stable anchor is step name `customer-feedback-ingestion`, not line number). Type: skill. Skill: `skills/customer-feedback-ingestion/SKILL.md`. Depends: `[state-init]`. Condition: `config.products[*].user_channels is configured`. Source 96-103. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `customer-feedback-ingestion` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. state-init completed.
2. At least one product has user_channels configured.

## Postconditions

1. Customer feedback signals ingested and normalized into discovery state.

## Invariants

1. Step skipped when no user_channels configured.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No channels | Skipped |
| EC-002 | Channel API rate limit | Escalate per defaults |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Configured channels | Feedback ingested | happy-path |
| No channels | Skipped | edge-case |
| API outage | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Step gated by user_channels condition | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.012 — state-backup-ingestion

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 96-103) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: config-existence condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | network calls |
| **Global state access** | external feedback channels |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 96)`) and source range (`Source 96-103.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `customer-feedback-ingestion`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `customer-feedback-ingestion`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
