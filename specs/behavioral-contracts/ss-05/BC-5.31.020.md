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

# Behavioral Contract BC-5.31.020: code-delivery:security-review

## Description

Step `security-review` (line 289; lobster carve-out: stable anchor is step name `security-review`, not line number). Type: agent. Agent: security-reviewer. Depends: `[create-pr]`. Condition: `module_criticality in ['CRITICAL', 'HIGH']`. Source 289-301. Information-asymmetry wall: excludes implementer notes. Max 3 security review cycles. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `security-review` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. module_criticality is CRITICAL or HIGH.
2. PR is open.

## Postconditions

1. Security verdict produced (or step skipped for low-criticality).
2. Implementer notes are not visible to security-reviewer.

## Invariants

1. Step is skipped for low-criticality modules.
2. Cycle count ≤ 3.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | module_criticality=LOW | Skipped |
| EC-002 | Reviewer attempts wall breach | Denied |
| EC-003 | 3 cycles without resolution | Escalation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CRITICAL module | Review run | happy-path |
| LOW module | Skipped | edge-case |
| Persistent finding | Escalate at cycle 3 | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Implementer notes never appear in reviewer context | manual |
| VP-002 | Max cycles ≤ 3 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.018 — create-pr
- BC-5.31.021 — pr-review-convergence (parallel)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 289-301; lobster path carve-out: line range is unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: criticality condition
- type constraint: max cycles

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered context) |
| **Global state access** | reads filtered filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 289)`) and source range (`Source 289-301.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `security-review`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `security-review`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster path lines 289-301

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
