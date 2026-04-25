---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:72"
subsystem: "SS-01"
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

# Behavioral Contract BC-1.02.002: HookPayload accepts both `event_name` and `hook_event_name`

## Description

The dispatcher's `HookPayload` accepts a real Claude Code harness envelope using either `event_name` or `hook_event_name` (alias). The canonical name remains `event_name`. This was pinned by v1.0.0-beta.2 dogfood regression after the harness shape mismatch was discovered.

## Preconditions

1. JSON envelope uses `hook_event_name` (real harness) OR `event_name` (canonical).

## Postconditions

1. Either alias parses cleanly into `HookPayload`.
2. The parsed payload's `event_name` field is populated regardless of which alias was used.

## Invariants

1. The harness envelope key alias is permanent (cannot be removed without a breaking version event).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Both keys present | Behavior TBD (not pinned by source; canonical takes precedence by serde alias semantics) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Envelope with `hook_event_name = "PreToolUse"` | Parsed with `event_name = "PreToolUse"` | happy-path |
| Envelope with `event_name = "PreToolUse"` | Parsed with `event_name = "PreToolUse"` | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/payload.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `payload.rs::tests::accepts_hook_event_name_alias_from_real_harness` |
| **Confidence** | HIGH (regression test explicitly cited in CHANGELOG) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `72` |

#### Evidence Types Used

- assertion (regression test pinned by v1.0.0-beta.2)

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
