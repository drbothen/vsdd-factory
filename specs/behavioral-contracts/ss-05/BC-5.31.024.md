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

# Behavioral Contract BC-5.31.024: code-delivery:wait-for-ci

## Description

Step `wait-for-ci` (line 367; lobster carve-out: stable anchor is step name `wait-for-ci`, not line number). Type: loop. max_iterations: 3. exit_condition: `ci.status == 'all_passed'`. Depends: `[pr-review-convergence, brownfield-full-regression]`. Source 367-389. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `wait-for-ci` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. PR has been pushed and CI has been triggered.
2. CI status API is reachable.

## Postconditions

1. Loop exits when CI reports all_passed or after 3 polls.
2. Failure path escalates per defaults.

## Invariants

1. Loop bounded ≤ 3 polls.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | CI flaky failure | Cap may be hit |
| EC-002 | CI offline | Step fails |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CI passes immediately | Exit at iter 1 | happy-path |
| CI failure | Cap, escalate | error |
| Slow CI | Cap | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Bounded ≤ 3 iterations | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.021 — pr-review-convergence
- BC-5.31.025 — dependency-merge-check (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 367-389) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop with max_iterations + exit_condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | network calls (CI API) |
| **Global state access** | reads CI host |
| **Deterministic** | yes (given CI state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 367)`) and source range (`Source 367-389.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `wait-for-ci`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `wait-for-ci`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
