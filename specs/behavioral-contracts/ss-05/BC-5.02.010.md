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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:837
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

# Behavioral Contract BC-5.02.010: orchestrator: 3-clean-passes minimum for adversarial convergence

## Description

Adversarial convergence at Phase 1d, Phase 2, Phase 3 (per-story and per-wave),
and Phase 5 each REQUIRE at least 3 consecutive NITPICK-novelty passes before
declaring convergence.

## Preconditions

1. An adversarial review loop is active at one of the named phases.

## Postconditions

1. Convergence is declared only after ≥3 consecutive NITPICK-novelty passes.
2. Each convergence-trajectory.md shows ≥3 consecutive nitpick-novelty passes
   for every relevant phase.

## Invariants

1. The 3-clean-passes minimum is universal across the named phases.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pass 3 finds new SUBSTANTIVE finding | Counter resets; another 3 clean passes required |
| EC-002 | After 10 passes still SUBSTANTIVE | Escalate to human (per BC-5.04.004) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Phase 1d adversary loop | ≥3 consecutive NITPICK passes before convergence | happy-path |
| Pass 3 SUBSTANTIVE finding | Counter resets | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | convergence-trajectory.md shows ≥3 NITPICK trailing passes for every converged phase | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.004 — composes with (adversary 3-clean / 10-max bounds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#convergence-bounds`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:113-117` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit MANDATORY STEPS section in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (convergence trajectory) |
| **Global state access** | reads pass-novelty history |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (counter logic) |

#### Refactoring Notes

No refactoring needed.
