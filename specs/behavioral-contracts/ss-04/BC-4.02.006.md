---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:480"
subsystem: "SS-04"
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

# Behavioral Contract BC-4.02.006: Adapter's wall-clock cap (BASH_TIMEOUT_MS = 60_000) is a backstop; real per-call deadline = dispatcher's epoch-interruption (default 5_000ms)

## Description

The adapter's 60-second wall-clock cap is intentionally MUCH larger than the dispatcher's per-plugin epoch deadline (default 5_000 ms). The dispatcher will terminate the wasm instance via epoch interrupt long before the adapter's bash subprocess timeout fires. The adapter cap exists as a safety net for the rare case where the dispatcher's epoch deadline fails to fire (e.g., a misconfigured ticker).

## Preconditions

1. A bash hook has hung past the dispatcher's epoch deadline.

## Postconditions

1. The dispatcher's epoch interrupt fires first (default 5_000 ms).
2. `BASH_TIMEOUT_MS = 60_000` is the adapter-side last-resort backstop.

## Invariants

1. Adapter cap >= dispatcher's epoch deadline (defense in depth).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher epoch ticker misconfigured / not running | Adapter cap fires after 60s |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Hung bash hook with default 5s dispatcher epoch | Dispatcher times out via epoch first; adapter cap never fires | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (BASH_TIMEOUT_MS) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:48-54` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `480` |

#### Evidence Types Used

- type constraint (constant + comment)
- documentation (BC-AUDIT-009 epoch-precedence cross-reference)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
