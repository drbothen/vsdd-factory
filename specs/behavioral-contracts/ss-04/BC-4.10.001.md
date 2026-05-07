---
document_type: behavioral-contract
level: L3
version: "1.0"
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
bc_id: BC-4.10.001
section: "4.10"
---

# BC-4.10.001: validate-per-story-adversary-convergence WASM hook MUST block wave-gate dispatch when any story lacks convergence clearance

## Description

The `validate-per-story-adversary-convergence` WASM hook fires on `SubagentStop` events
for the wave-gate dispatch step. It reads the `adversary-convergence-state.json` file for
every story in the current wave and blocks dispatch if any story is missing the state file,
has `passes_clean < 3`, or has `last_classification != "NITPICK_ONLY"`. The hook uses the
`HookResult::block_with_fix(hook, reason, recommendation, code)` canonical Why/Fix/Code
block-message pattern (per HOST_ABI.md) and MUST NOT use bare `HookResult::block()` calls.
The hook is implemented as a WASM binary at `plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm`
with `HOST_ABI_VERSION = 1`.

## Preconditions

1. The hook is registered in `hooks-registry.toml` under event `SubagentStop` with the
   agent scope targeting wave-gate dispatch.
2. The cycle directory `.factory/cycles/<cycle-id>/` is accessible via WASI preopened
   project root (readable by `host::read_file`).
3. Each story in the wave has a declared story ID known at wave-gate time.
4. The hook is invoked with a valid `HookPayload` conforming to `HOST_ABI_VERSION = 1`.
5. The hook's pure logic function is exercised by unit tests WITHOUT a WASM runtime
   (injectable-callback pattern per the established pattern in `handoff-validator` and `regression-gate`).

## Postconditions

1. For each story `<story-id>` in the wave, the hook reads:
   `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`
2. If the state file is ABSENT for any story:
   - The hook emits `HookResult::block_with_fix(...)` with:
     - `hook`: `"validate-per-story-adversary-convergence"`
     - `reason`: `"Story <story-id> is missing adversary-convergence-state.json — convergence gate not run"`
     - `recommendation`: `"Run the per-story adversary convergence loop (BC-5.39.001) for story <story-id> before dispatching the wave gate."`
     - `code`: `"CONVERGENCE_STATE_MISSING"`
   - The wave-gate dispatch is BLOCKED.
3. If the state file is PRESENT but `passes_clean < 3` for any story:
   - The hook emits `HookResult::block_with_fix(...)` with:
     - `reason`: `"Story <story-id> has passes_clean=<N> — convergence requires passes_clean >= 3"`
     - `recommendation`: `"Continue adversary review passes for story <story-id> until passes_clean reaches 3."`
     - `code`: `"CONVERGENCE_PASSES_INSUFFICIENT"`
   - The wave-gate dispatch is BLOCKED.
4. If the state file is PRESENT but `last_classification != "NITPICK_ONLY"` for any story:
   - The hook emits `HookResult::block_with_fix(...)` with:
     - `reason`: `"Story <story-id> last adversary pass classified as <classification> — must be NITPICK_ONLY"`
     - `recommendation`: `"Resolve remaining <classification> findings for story <story-id> before dispatching the wave gate."`
     - `code`: `"CONVERGENCE_CLASSIFICATION_INSUFFICIENT"`
   - The wave-gate dispatch is BLOCKED.
5. If ALL stories in the wave have `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`:
   - The hook returns `HookResult::Continue`.
   - The wave-gate dispatch proceeds normally.
6. The deferred_findings arrays from all stories are aggregated and emitted to the wave-gate
   context as a structured log entry (not a block) for wave-gate review.
7. The hook MUST NOT modify the `adversary-convergence-state.json` files — it is read-only.
8. Block messages are emitted using `HookResult::block_with_fix(hook, reason, recommendation, code)`
   per the canonical Why/Fix/Code pattern. Bare `HookResult::block()` is prohibited.

## Invariants

1. The hook fires exclusively on `SubagentStop` events targeting wave-gate dispatch. It MUST
   NOT fire on per-story `SubagentStop` events (which trigger BC-5.39.001's convergence loop).
   The event filter in `hooks-registry.toml` MUST scope the hook to wave-gate only.
2. `HOST_ABI_VERSION = 1` — no new host functions are required. The hook uses only
   `host::read_file`, `host::log_*`, and `host::emit_event` (already present in ABI v1).
3. The hook's pure `fn hook_logic(...)` function takes all host I/O as injectable closures
   and contains all decision logic. The WASM `main.rs` entry wires real host functions.
   Unit tests MUST exercise `hook_logic` directly without WASM runtime.
4. The hook is read-only with respect to the `.factory/` directory. It never writes files.
5. The convergence criterion checked by the hook is identical to BC-5.39.001 Postcondition 5:
   `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`. No looser threshold is acceptable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wave has 0 stories (edge: empty wave) | Hook returns `HookResult::Continue` (vacuously all stories cleared). Log warning: "Wave has zero stories." |
| EC-002 | State file exists but is malformed JSON | Hook returns `HookResult::block_with_fix(...)` with `code: "CONVERGENCE_STATE_MALFORMED"`. Treat malformed as non-cleared. |
| EC-003 | State file exists, `passes_clean >= 3`, but `last_classification` field is missing | Treat as non-cleared. Block with `code: "CONVERGENCE_STATE_SCHEMA_INVALID"`. |
| EC-004 | State file exists, passes_clean = 3, last_classification = "NITPICK_ONLY", but deferred_findings is non-empty | Allowed. Deferred findings do not block per BC-5.39.002. Hook returns Continue. Emits aggregated deferred log. |
| EC-005 | Multiple stories fail convergence check | Block on the FIRST non-cleared story found. Emit a single block message identifying the first failure. |
| EC-006 | Cycle directory absent (`<cycle-id>` directory does not exist) | Hook gracefully returns `HookResult::Continue` per BC-4.10.002 (invoked outside wave-gate context). |

## Canonical Test Vectors

| Stories in Wave | State Files | Expected Result |
|----------------|-------------|----------------|
| [S-A] | S-A: {passes_clean: 3, last_classification: "NITPICK_ONLY"} | Continue |
| [S-A, S-B] | S-A: cleared; S-B: {passes_clean: 2} | BLOCK — S-B passes_clean < 3; code: CONVERGENCE_PASSES_INSUFFICIENT |
| [S-A] | S-A file absent | BLOCK — state file missing; code: CONVERGENCE_STATE_MISSING |
| [S-A] | S-A: {passes_clean: 3, last_classification: "HIGH"} | BLOCK — classification != NITPICK_ONLY; code: CONVERGENCE_CLASSIFICATION_INSUFFICIENT |
| [S-A] | S-A: {passes_clean: 3, last_classification: "NITPICK_ONLY", deferred_findings: [{...}]} | Continue (deferred does not block) |
| [S-A] | S-A: malformed JSON | BLOCK — code: CONVERGENCE_STATE_MALFORMED |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-071 | advisory-block output always emitted on non-cleared gate | kani (pure logic branch coverage) |
| (unit-test) | hook_logic returns Continue when all stories pass convergence | Rust unit test (injectable callbacks; no WASM runtime) |
| (unit-test) | hook_logic blocks on missing state file | Rust unit test |
| (unit-test) | hook_logic blocks on passes_clean < 3 | Rust unit test |
| (unit-test) | hook_logic blocks on last_classification != NITPICK_ONLY | Rust unit test |
| (unit-test) | hook_logic blocks (not panics) on malformed JSON | Rust unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs a new WASM plugin (`validate-per-story-adversary-convergence.wasm`) authored using the `vsdd-hook-sdk` crate. CAP-009 defines the SDK as the interface through which plugin authors implement hook behavior using the `#[hook]` macro, `HookPayload`, `HookResult`, and `vsdd::*` host function bindings. This BC specifies the behavioral contract of a first-party WASM plugin that uses exactly those SDK surfaces to enforce the per-story convergence gate at wave-gate dispatch. |
| Secondary Capability Reference | CAP-005 ("Run adversarial review with information asymmetry") per capabilities.md §CAP-005 — the gate enforced by this hook is the mechanical lock on CAP-005's per-story adversary workflow. CAP-005 is a secondary reference because this BC governs the WASM implementation (CAP-009 surface), not the adversary workflow contract (BC-5.39.001). |
| L2 Domain Invariants | none |
| Architecture Module | crates/hook-plugins/validate-per-story-adversary-convergence/ (Rust crate); plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm (build output); hooks-registry.toml (registration) |
| Stories | Story B (v1.0-feature-engine-discipline-pass-1 F3 decomposition) |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-5.39.001 — depends on (defines the convergence criterion `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` that this hook enforces mechanically)
- BC-5.39.002 — depends on (defines the `deferred_findings` schema that this hook aggregates)
- BC-4.10.002 — sibling (graceful degrade path for the same hook)
- BC-2.02.012 — depends on (two-stage agent identity fallback chain; mirrors handoff-validator pattern)

## Architecture Anchors

- `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs` — pure `fn hook_logic(...)` with injectable callbacks; unit tests inline
- `crates/hook-plugins/validate-per-story-adversary-convergence/src/main.rs` — WASM entry point wiring real host fns
- `plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm` — build output
- `hooks-registry.toml` — event: SubagentStop; scoped to wave-gate dispatch

## Story Anchor

Story B — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

- VP-071 — adversary convergence gate enforcement

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). D-337 constraint applied: WASM-only (no Bash hook). HOST_ABI_VERSION = 1 confirmed by F1 architect. |
