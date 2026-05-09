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

# Behavioral Contract BC-5.31.005: code-delivery: failure semantics

## Description

`code-delivery.lobster` § workflow-defaults — lines `28-31` cited as point-in-time evidence pending lobster section-stability verification — declare workflow defaults: on_failure=escalate, retries=2, timeout=1h. `red-gate.fail_action: block` (line `80`, cited as point-in-time evidence). <!-- F-P21-001: lobster-line-class deferred per pass-21 retroactive sweep; line refs preserved as source evidence --> Multiple bounded loops: per-story-adversarial-review (max 10, exit on CONVERGENCE_REACHED), storybook-component-tests (max 10, exit on `storybook_tests.all_pass`), pr-review-convergence (max 10, exit on `pr_reviewer.verdict == 'APPROVE'`), wait-for-ci (max 3, exit on `ci.status == 'all_passed'`).

## Preconditions

1. Workflow is invoked with default failure config.
2. Loop steps have `max_iterations` declared.

## Postconditions

1. Any step failure escalates after 2 retries within 1h timeout.
2. red-gate failure blocks downstream execution.
3. Each loop terminates by reaching its exit condition or by exceeding `max_iterations`.

## Invariants

1. No loop runs unbounded (every loop has finite `max_iterations`).
2. Block-action gates prevent downstream execution on failure.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Adversary never converges within 10 iterations | Loop exits at cap, treated per terminal-policy (escalation) |
| EC-002 | red-gate detects compiling+failing tests pass instead | Block downstream, fail workflow |
| EC-003 | wait-for-ci times out after 3 polls | Loop exits, downstream merge gated |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Step transient failure x2 | 2 retries, then succeed | happy-path |
| Step persistent failure | Escalation event after retries | error |
| Adversary CONVERGENCE_REACHED at iter 3 | Loop exits early | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | All loops have bounded iteration counts | manual / lobster-parse |
| VP-002 | block fail_action halts dependent steps | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.001 — identity
- BC-5.31.009 — red-gate
- BC-5.31.011 — per-story-adversarial-review
- BC-5.31.014 — storybook-component-tests
- BC-5.31.021 — pr-review-convergence
- BC-5.31.024 — wait-for-ci

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002 — block semantics

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 28-31, 80) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation / type constraint: declarative `max_iterations`, `fail_action`, `on_failure` keys

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (structural property) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P21-001: §Description lobster line-refs annotated with carve-out deferral)

**Driver:** F-P21-001 pass-21 retroactive sibling sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited `code-delivery.lobster` workflow-defaults and red-gate by line numbers (`lines 28-31`, `line 80`). These are lobster-file references and fall under the lobster-line-class carve-out exception pending lobster section-stability verification.

**Change made:**
- §Description: line refs preserved as point-in-time evidence; `code-delivery.lobster` file name and logical sections (workflow-defaults, red-gate) made explicit; HTML comment added citing F-P21-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
