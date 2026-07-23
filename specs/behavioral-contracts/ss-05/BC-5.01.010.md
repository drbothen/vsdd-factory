---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
last_amended: 2026-05-08
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md § "BC-AUDIT-117"
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

# Behavioral Contract BC-5.01.010: `agent` steps declare `context: { include: [...], exclude: [...] }` to enforce information-asymmetry walls

## Description

`agent` steps include a `context:` block with `include:` and `exclude:` glob
lists to enforce information-asymmetry walls between sub-agents. Exclude-list
comments often use a `▓ WALL:` prefix to mark deliberate boundaries (e.g.,
adversary cannot see prior reviews; pr-reviewer cannot see `.factory/`;
holdout-evaluator cannot see source).

## Preconditions

1. A sub-agent should NOT have visibility into certain files (information asymmetry).

## Postconditions

1. `context:` block is declared with `include:` and `exclude:` glob lists.
2. The orchestrator filters file access so the dispatched agent sees only
   files matching `include` and not matching `exclude`.
3. Exclude-list comments may use `▓ WALL:` prefix to mark deliberate
   information-asymmetry boundaries.

## Invariants

1. The walls are load-bearing — used 12+ times in greenfield alone.
2. Common walls include prior adversarial reviews, implementer notes, holdout
   scenarios, semport history, and factory cycles.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to Read a path matching `exclude` | Read denied |
| EC-002 | `include`/`exclude` overlap | TBD — exclude wins (typical) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster::spawn-adversary-spec-review` (adversary excludes `.factory/holdout-scenarios/**`) | Adversary cannot read holdout scenarios | happy-path |
| `greenfield.lobster::ai-pr-review` (PR reviewer excludes `.factory/**`) | PR reviewer cannot read .factory artifacts | happy-path |
| `greenfield.lobster::spawn-wave-adversary` (wave adversary excludes prior reviews) | Adversary cannot read prior pass findings | happy-path |
| `greenfield.lobster::adversary-code-review` (phase-5 adversary excludes implementer notes, prior reviews, semport) | Adversary cannot read inherited reasoning | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows zero Read on excluded paths for the dispatched agent | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator context-filter logic |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy: agent)
- BC-5.04.001 — composes with (adversary cannot see prior reviews)
- BC-5.07.024 — composes with (holdout-evaluator information wall)
- BC-5.08.019 — composes with (pr-reviewer cannot see .factory/)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster::spawn-adversary-spec-review`, `greenfield.lobster::ai-pr-review`, `greenfield.lobster::spawn-wave-adversary`, `greenfield.lobster::adversary-code-review`, `greenfield.lobster::phase-5-gemini-review` |
| **Confidence** | high (load-bearing pattern, used 12+ times in greenfield alone) |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit `context: { include, exclude }` blocks
- documentation: `▓ WALL:` prefix comments mark deliberate boundaries

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (file allow/deny enforcement) |
| **Global state access** | reads workspace metadata |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | mixed |

#### Refactoring Notes

The schema is pure data; the enforcement layer that denies Reads is effectful and lives in the dispatcher.

## Changelog

- v1.2 (2026-05-08): TD-VSDD-091 stable-anchor migration sweep (Chunk 3) — 5 cites migrated. `extracted_from` line cite and `.lobster:NNN` cites in Canonical Test Vectors and Source Evidence replaced with stable step-name anchors (`§ "BC-AUDIT-117"`; 5 context-wall step-name anchors in greenfield.lobster: `spawn-adversary-spec-review`, `ai-pr-review`, `spawn-wave-adversary`, `adversary-code-review`, `phase-5-gemini-review`).
