---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "40a6fb6"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
origin: greenfield
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.10.002
section: "4.10"
---

# BC-4.10.002: validate-per-story-adversary-convergence WASM hook MUST gracefully degrade (exit 0) when invoked outside wave-gate context or when cycle directory is absent

## Description

The `validate-per-story-adversary-convergence` WASM hook may fire on `SubagentStop` events
in contexts other than wave-gate dispatch (e.g., during normal per-story subagent stops,
outside any active cycle, or in a fresh repository with no `.factory/cycles/` directory).
In all such cases, the hook MUST return `HookResult::Continue` without blocking or erroring.
This is the graceful degrade path. The pattern follows `validate-wave-gate-prerequisite.sh`
lines 64–70 and the `regression-gate` BC-7.03.074 precedent.

## Preconditions

1. The hook fires on a `SubagentStop` event.
2. One or more of the following is true:
   a. The hook cannot determine the current cycle-id from the payload or environment.
   b. The `.factory/cycles/` directory does not exist.
   c. The current cycle directory `.factory/cycles/<cycle-id>/` does not exist.
   d. The payload does not contain a wave-gate dispatch indicator (i.e., the hook cannot
      confirm it is executing in a wave-gate context).
   e. The hook is invoked outside any active vsdd-factory factory cycle.

## Postconditions

1. The hook returns `HookResult::Continue` (exit 0 in WASM terms).
2. The hook MUST NOT emit a block message in any graceful-degrade case.
3. The hook logs a single advisory message via `host::log_info(...)`:
   `"validate-per-story-adversary-convergence: graceful degrade — invoked outside wave-gate context or cycle directory absent; returning Continue"`
   Note: HOST_ABI v1 does not expose a `log_debug` endpoint; `log_info` is the
   lowest-severity level available. The `log_debug` symbol referenced in early drafts
   is absent from the SDK.
4. The hook does NOT write to stderr in the graceful-degrade path. Advisory logging only.
5. The hook does NOT emit any `hook.block` or `hook.warn` events in the graceful-degrade path.
6. The hook's execution time in the graceful-degrade path MUST be under 50ms (fast-path exit
   before any file I/O or state file parsing).

## Invariants

1. The graceful-degrade path MUST be exercised by at least one dedicated unit test
   (injectable-callback pattern). The test must assert `HookResult::Continue` is returned
   and no block event is emitted.
2. The graceful-degrade check MUST occur BEFORE any attempt to read state files. If cycle
   context cannot be determined, the hook exits immediately without attempting file reads.
3. A missing `.factory/cycles/` directory is never treated as an error. It is a valid
   operational state (e.g., a repository without factory cycles configured).
4. The graceful-degrade path applies to ALL cases where the wave-gate context cannot be
   confirmed — not just to missing directories. The hook errs on the side of Continue
   rather than blocking on uncertainty.
5. The block path (BC-4.10.001) and the graceful-degrade path (this BC) are mutually
   exclusive: if the hook can confirm wave-gate context AND can read state files, BC-4.10.001
   applies. If context is ambiguous or absent, this BC applies.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Hook fires on a normal per-story `SubagentStop` (not wave-gate) | Graceful degrade: return Continue, log advisory. |
| EC-002 | Fresh repository — `.factory/cycles/` does not exist | Graceful degrade: return Continue. No error. |
| EC-003 | Cycle directory exists but has no story subdirectories | Graceful degrade: no stories to check → Continue (equivalent to BC-4.10.001 EC-001 empty-wave case). |
| EC-004 | Payload missing `subagent_name` and `agent_type` fields | Cannot determine context. Graceful degrade: return Continue. Log advisory. |
| EC-005 | Hook invoked by an ad-hoc SubagentStop from a non-factory tool | Cannot confirm factory context. Graceful degrade: return Continue. |
| EC-006 | Cycle directory path is readable but cycle-id cannot be extracted from payload | Graceful degrade: return Continue. Do not attempt to scan all cycle directories. |

## Canonical Test Vectors

| Invocation Context | Cycle Dir Present | Expected Result |
|-------------------|------------------|----------------|
| Wave-gate SubagentStop | Yes, with story state files | BC-4.10.001 applies |
| Per-story SubagentStop (not wave-gate) | Yes | Graceful degrade: Continue |
| Any SubagentStop | `.factory/cycles/` absent | Graceful degrade: Continue |
| Any SubagentStop | Cycle dir absent | Graceful degrade: Continue |
| SubagentStop with missing agent_type field | Yes | Graceful degrade: Continue |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (unit-test) | hook_logic returns Continue when cycle directory is absent | Rust unit test (injectable callbacks; read_file callback returns Err for cycles path) |
| (unit-test) | hook_logic returns Continue when agent_type field is absent | Rust unit test |
| (unit-test) | hook_logic does not call read_file before context check | Rust unit test (assert callback not invoked) |
| (proptest) | hook_logic never panics on any HookPayload input shape | proptest property test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the graceful-degrade behavior of the `validate-per-story-adversary-convergence` WASM plugin. It specifies the `HookResult::Continue` return path in the plugin's pure `fn hook_logic(...)` function, which is authored using the `vsdd-hook-sdk` crate's `HookResult` type. This is a first-party WASM plugin contract (CAP-009 surface), not a workflow-level contract. |
| L2 Domain Invariants | none |
| Architecture Module | crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs — graceful-degrade branch in `fn hook_logic(...)` |
| Stories | S-12.02 |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-4.10.001 — sibling (block path for the same hook; mutually exclusive with this BC's graceful-degrade path)
- BC-7.03.074 — pattern reference (regression-gate graceful degrade; established pattern for `HookResult::Continue` on missing context)
- BC-5.39.001 — indirect (the loop this hook enforces at wave-gate; graceful degrade does not affect per-story loop)

## Architecture Anchors

- `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs` — graceful-degrade branch; unit test for this path inline
- `plugins/vsdd-factory/hooks/validate-wave-gate-prerequisite.sh` lines 64–70 — reference implementation pattern for graceful degrade

## Story Anchor

Story B — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

(proptest — no panic on arbitrary payload; unit-test — Continue on missing context)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). Pattern follows validate-wave-gate-prerequisite.sh lines 64–70 and regression-gate BC-7.03.074 as specified in the F2 dispatch. |
| 1.1 | 2026-05-07 | PC3 amendment (architect; F-HIGH-4, F5 pass-1 fix burst B2): `host::log_debug` changed to `host::log_info`. HOST_ABI v1 does not expose a `log_debug` endpoint; the implementation correctly maps `log_debug` → `host::log_info` (the lowest-severity level available). Spec amended to match implementation. |
| 1.2 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: "Story B" placeholder → S-12.02. BC-INDEX was updated in fix-burst-39 (v1.55) to replace TBD; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
