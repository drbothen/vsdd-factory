---
document_type: behavioral-contract
level: L3
version: "1.1"
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

# Behavioral Contract BC-5.31.019: code-delivery:ai-pr-review

## Description

Step `ai-pr-review` (line 271). Type: agent. Agent: pr-reviewer. Depends: `[create-pr]`. Source 271-284. Information-asymmetry wall: `context.exclude: [".factory/**"]` — pr-reviewer sees only PR diff.

## Preconditions

1. PR is open.
2. `.factory/**` is excluded from reviewer context.

## Postconditions

1. AI PR review verdict is set to APPROVE / REQUEST_CHANGES / etc.
2. Review comments authored on PR.

## Invariants

1. pr-reviewer never reads any path under `.factory/**`.
2. Wall is enforced per-step (other agents may see `.factory/**`).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Reviewer attempts to read excluded path | Tool denial / runtime guard |
| EC-002 | Empty diff | Reviewer returns trivial APPROVE or fails per agent |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Standard PR | Verdict produced | happy-path |
| Diff-only context | Reviewer sees only diff | edge-case |
| Wall breach attempt | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | `.factory/**` paths never appear in reviewer's effective context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.018 — create-pr
- BC-5.31.021 — pr-review-convergence (downstream)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 271-284) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: context.exclude
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (diff only) |
| **Global state access** | reads filtered filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
