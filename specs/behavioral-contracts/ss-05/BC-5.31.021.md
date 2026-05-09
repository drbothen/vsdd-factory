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

# Behavioral Contract BC-5.31.021: code-delivery:pr-review-convergence

## Description

Step `pr-review-convergence` (line 307; lobster carve-out: stable anchor is step name `pr-review-convergence`, not line number). Type: loop. max_iterations: 10. exit_condition: `pr_reviewer.verdict == 'APPROVE'`. Depends: `[ai-pr-review]`. Condition: `pr_reviewer.verdict == 'REQUEST_CHANGES'`. Source 307-339. Triage → fix → re-review (with same `.factory/**` exclusion). <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `pr-review-convergence` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. ai-pr-review returned REQUEST_CHANGES.
2. PR-reviewer wall (exclude `.factory/**`) is honored on every re-review.

## Postconditions

1. Loop exits at APPROVE or after 10 iterations.
2. Each iteration triages comments, applies fixes, re-runs reviewer.

## Invariants

1. Loop bounded ≤10.
2. Wall enforced on every iteration.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | APPROVE on first run | Loop never enters |
| EC-002 | Persistent disagreement | Cap reached at iter 10 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Single fix needed | Converge in 1-2 iter | happy-path |
| Repeated nits | Cap reached | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Loop terminates ≤ 10 | manual |
| VP-002 | Wall preserved every iteration | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.019 — ai-pr-review

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 307-339) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop with max_iterations
- guard clause: condition expression

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (PR comments + code) |
| **Global state access** | git, PR API |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 307)`) and source range (`Source 307-339.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `pr-review-convergence`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `pr-review-convergence`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
