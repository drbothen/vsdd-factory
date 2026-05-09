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

# Behavioral Contract BC-5.31.014: code-delivery:storybook-component-tests

## Description

Step `storybook-component-tests` (line 176; lobster carve-out: stable anchor is step name `storybook-component-tests`, not line number). Type: loop. max_iterations: 10. exit_condition: `storybook_tests.all_pass`. Depends: `[storybook-story-generation]`. Same UI/storybook condition. Source 176-200. Self-healing loop: run-story-tests → fix-component (if failures or a11y violations). <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `storybook-component-tests` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. Storybook stories exist (storybook-story-generation completed).
2. UI/storybook conditions still hold.

## Postconditions

1. Loop exits when all storybook tests pass (incl. a11y) or after 10 iterations.

## Invariants

1. Loop is bounded.
2. Each iteration makes a fix attempt only when failures or a11y violations are present.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All pass on first run | Loop exits at iter 1 |
| EC-002 | Persistent failure | Loop terminates at iter 10; escalates |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Healthy components | First-iter exit | happy-path |
| A11y violation only | Fix loop converges | edge-case |
| Unfixable | Terminate at cap | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Loop terminates ≤10 iterations | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.013 — storybook-story-generation
- BC-5.31.015 — per-story-ui-quality-gate (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 176-200) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop + exit_condition
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (story tests) |
| **Global state access** | reads filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 176)`) and source range (`Source 176-200.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `storybook-component-tests`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `storybook-component-tests`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.
