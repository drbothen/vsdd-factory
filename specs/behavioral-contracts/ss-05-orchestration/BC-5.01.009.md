---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:327
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

# Behavioral Contract BC-5.01.009: `agent` steps with `model_tier:` override the default agent model assignment

## Description

`agent` steps may declare `model_tier:` to override the default model assigned
to the agent. Observed tier keys: `adversary` (used for adversary dispatch, e.g.,
GPT-5.4 for fresh-eyes review) and `review` (used for code-reviewer / visual-reviewer,
e.g., Gemini 3.1 Pro for secondary review).

## Preconditions

1. Agent dispatch needs a different/stronger model than the agent's default.

## Postconditions

1. `model_tier:` is set on the step.
2. The orchestrator routes the dispatch to the model registered for that tier.
3. Recognized tier keys include at least `adversary` and `review`.

## Invariants

1. `model_tier` is opt-in; absence falls back to the agent's default model assignment.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `model_tier` references unknown tier key | TBD — error or fall back to default |
| EC-002 | Tier registry maps the same tier to a different model in another env | Dispatch uses the env-specific registry |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:269` (`model_tier: adversary`) | Adversary dispatch on adversary-tier model | happy-path |
| `greenfield.lobster:1113` (`model_tier: review`) | code-reviewer dispatch on review-tier model | happy-path |
| `greenfield.lobster:1289` (visual-reviewer review tier) | visual-reviewer dispatch on review-tier model | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every `model_tier` value resolves in the tier registry | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator agent dispatcher, tier registry |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy: agent)
- BC-5.04.001 — composes with (adversary information wall)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#model-tiers`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:269, 435, 822, 1070, 1113, 1289` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit `model_tier:` annotations across 6 dispatch sites

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data) |
| **Global state access** | reads tier registry |
| **Deterministic** | yes (given fixed registry) |
| **Thread safety** | unknown |
| **Overall classification** | pure (with controlled registry read) |

#### Refactoring Notes

No refactoring needed.
