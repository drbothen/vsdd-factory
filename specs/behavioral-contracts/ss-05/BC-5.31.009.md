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

# Behavioral Contract BC-5.31.009: code-delivery:red-gate

## Description

Step `red-gate` (line 73; lobster carve-out: stable anchor is step name `red-gate`, not line number). Type: gate. Depends: `[write-tests]`. fail_action: block. Source 73-80. Verifies tests compile AND all tests fail before implementation may proceed. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `red-gate` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. write-tests has completed.
2. Build/test runner is available.

## Postconditions

1. If pass condition met (compile-OK + all-fail), gate passes and downstream proceeds.
2. Otherwise, gate blocks downstream (fail_action: block).

## Invariants

1. Gate semantics are pure: same input → same gate decision.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tests fail to compile | Gate blocks |
| EC-002 | Some tests pass | Gate blocks (Red required) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All tests fail, compile OK | Gate passes | happy-path |
| Compile error | Gate blocks | error |
| Mixed pass/fail | Gate blocks | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Gate blocks unless tests compile and all fail | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.008 — write-tests (depends on)
- BC-5.31.010 — implement (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 73-80; lobster path carve-out: line range is unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: gate type with fail_action: block
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (test runner output) |
| **Global state access** | reads test results |
| **Deterministic** | yes (given test outcomes) |
| **Thread safety** | N/A |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 73)`) and source range (`Source 73-80.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `red-gate`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `red-gate`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster path lines 73-80

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
