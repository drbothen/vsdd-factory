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

# Behavioral Contract BC-5.32.027: discovery:discovery-review

## Description

Step `discovery-review` (line 337; lobster carve-out: stable anchor is step name `discovery-review`, not line number). Type: human-approval. Timeout: 72h. Depends: `[generate-report]`. Source 337-349. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `discovery-review` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. generate-report completed and report is human-readable.

## Postconditions

1. Human review decision recorded (approve / defer / reject) within 72h.
2. On timeout, escalation per workflow defaults.

## Invariants

1. Human approval is bounded by 72h.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Timeout | Escalate |
| EC-002 | Reject all | Routing-step handles rejection |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Approve some | Decision captured | happy-path |
| Defer | Decision captured | edge-case |
| Timeout | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Bounded by 72h | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.024 — generate-report
- BC-5.32.028 — route-approved-ideas

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 337-349) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: human-approval + timeout

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (approval store) |
| **Global state access** | external (human input) |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | opaque |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 337)`) and source range (`Source 337-349.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `discovery-review`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `discovery-review`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
