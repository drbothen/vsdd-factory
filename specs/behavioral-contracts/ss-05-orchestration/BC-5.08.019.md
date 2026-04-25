---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:967
subsystem: SS-05
capability: CAP-TBD
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

# Behavioral Contract BC-5.08.019: pr-reviewer: cannot see .factory/ artifacts (information wall)

## Description

The pr-reviewer MUST review the PR purely as a human reviewer would — diff,
description, demo evidence, CI test results. It MUST NOT load any file under
`.factory/**`. Information needed from behind the wall must be derived
independently from the PR.

## Preconditions

1. pr-reviewer dispatched.

## Postconditions

1. Tool-call audit shows zero Read calls against `.factory/**`.
2. Allowed: PR diff, story spec, and architecture/api-surface.md (for API contract verification).

## Invariants

1. pr-reviewer reviews from external perspective.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Review needs implementer notes | Derive from PR diff + spec |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| pr-reviewer session | Zero Reads on .factory/** | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.020 — composes with (gh pr review)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:38, 87-103` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
