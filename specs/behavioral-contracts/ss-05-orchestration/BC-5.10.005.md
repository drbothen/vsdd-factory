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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1329
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

# Behavioral Contract BC-5.10.005: state-manager: wave-gate remediation uses Single Canonical SHA + Two-Commit Protocol

## Description

When committing a burst that updates STATE.md + SESSION-HANDOFF.md +
wave-state.yaml together, the state-manager MUST use the
`vsdd-factory:state-burst` skill which enforces: past-tense
"REMEDIATED — Awaiting Pass N+1" voice; literal `15fa97e6` placeholder in
Stage 1; max 2 commits; post-push hook verification; cross-document field sync.

## Preconditions

1. state-manager committing a wave-gate remediation burst.

## Postconditions

1. Wave-gate burst on factory-artifacts shows ≤2 commits.
2. HEAD and HEAD^ do not both contain `backfill`.
3. verify-sha-currency hook reports PASS post-push.

## Invariants

1. Anti-patterns (narrative voice, intermediate SHAs cited, 3rd commit, skipped hook,
   one-doc-only updates) have caused 6+ consecutive defect recurrences and are forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need a 3rd commit to fix typo | Forbidden — incorporate into the existing 2 commits |
| EC-002 | Document desync mid-burst | Halt; sync all sibling docs in same burst |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 2-commit burst with all sibling docs synced | Hook PASS | happy-path |
| Burst with intermediate SHA cited | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | verify-sha-currency hook PASS post-push | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/state-manager.md`, state-burst skill |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.003 — composes with (STATE.md size cap)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#wave-gate-remediation`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/state-manager.md:148-180` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Wave-gate remediation bursts rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (multiple state docs) + git ops |
| **Global state access** | reads/writes git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
