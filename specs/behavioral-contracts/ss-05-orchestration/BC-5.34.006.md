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
extracted_from: "plugins/vsdd-factory/workflows/multi-repo.lobster"
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

# Behavioral Contract BC-5.34.006: multi-repo: cross-repo information asymmetry walls

## Description

Source 39-51, 391-455. Walls extending single-repo walls to multi-repo:
- holdout-evaluator excludes ALL repos' source/specs/impl notes + `.factory-project/phase-0-synthesis/**`
- pr-reviewer excludes `.factory-project/**` and `**/.factory/**`
- adversary excludes `**/.factory/cycles/**/implementation/**`, `**/.factory/semport/**`, `**/.factory/phase-5-adversarial/**`
- security-reviewer excludes implementer reasoning + spec rationale

## Preconditions

1. Cross-repo agents are invoked in the multi-repo workflow.

## Postconditions

1. Each agent operates with a strictly filtered context per the documented exclusion lists.

## Invariants

1. The exclusion lists are exact:
   - holdout-evaluator: all repos' source + specs + impl notes + `.factory-project/phase-0-synthesis/**`
   - pr-reviewer: `.factory-project/**` and `**/.factory/**`
   - adversary: 3 patterns (implementation, semport, phase-5-adversarial)
   - security-reviewer: implementer reasoning + spec rationale

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wall breach attempt | Denied at tool layer |
| EC-002 | Project root has factory-project but no factory dirs | pr-reviewer wall still applies to project root |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Cross-repo adversary invocation | Excluded paths absent | happy-path |
| Wall breach attempt | Denied | error |
| Standard pr-review | Sees only diff + project README | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Each agent's effective context excludes the documented patterns | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.001 — identity

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
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` (lines 39-51, 391-455) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: context.exclude declarations

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered) |
| **Global state access** | reads filtered filesystem |
| **Deterministic** | yes (given inputs) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
