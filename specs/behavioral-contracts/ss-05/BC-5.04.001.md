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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:77
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

# Behavioral Contract BC-5.04.001: adversary: cannot see prior adversarial reviews (information wall)

## Description

The adversary MUST start each pass without access to prior pass findings. Read-only
tools are enforced (Read, Grep, Glob). Every pass derives novelty independently —
each review is fresh.

## Preconditions

1. adversary dispatched for an adversarial review pass.

## Postconditions

1. Tool-call audit shows zero Read calls against
   `.factory/cycles/*/adversarial-reviews/pass-*.md` for any prior pass.
2. Every finding has fresh-context justification.
3. Read-only tools are enforced (Read, Grep, Glob); Write/Edit/Bash/exec/process denied.

## Invariants

1. The information wall is structural (enforced by `context: { exclude }` and tool denial).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to Read prior pass file | Read denied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pass-2 dispatch | Zero Reads on `pass-1.md` | happy-path |
| Attempt to Read prior review | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows zero Reads on prior adversarial-reviews/pass-*.md | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.010 — composes with (information-asymmetry walls)
- BC-5.04.006 — composes with (adversary read-only)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#adversary`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/adversary.md:21-25, 187-189` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed inputs) |
| **Thread safety** | unknown |
| **Overall classification** | pure (read-only review) |

#### Refactoring Notes

No refactoring needed.
