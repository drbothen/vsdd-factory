---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: 2026-06-14
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - crates/factory-dispatcher/src/invoke.rs
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.15.001: Dispatcher routes PreCompact and PostCompact harness events to registered plugins (harness >= v2.1.105)

## Description

The vsdd-factory dispatcher binary (`factory-dispatcher`) must pass `PreCompact` and `PostCompact` events emitted by the Claude Code harness (v2.1.105+) through to registered WASM and shell plugins. Without this routing, the `precompact-flush.sh` and `postcompact-reanchor.sh` hooks (S-18.04, S-18.05) would be silently ignored. This BC specifies the dispatcher-side routing obligation; it does not prescribe the hooks themselves (those are specified in BC-7.07.001 and BC-7.07.002).

## Preconditions

1. The Claude Code harness version is >= v2.1.105 (confirmed in production: v2.1.177 per F1 delta analysis).
2. `hooks-registry.toml` contains one or more `[[hooks]]` entries with `event = "PreCompact"` or `event = "PostCompact"`.
3. The dispatcher binary is the vsdd-factory factory-dispatcher (source: `crates/factory-dispatcher/`).
4. The dispatcher `invoke.rs` (or equivalent dispatch routing module) enumerates supported event types.

## Postconditions

1. **PreCompact routing**: When the harness fires a `PreCompact` event, `factory-dispatcher` invokes all plugins registered under `event = "PreCompact"` in `hooks-registry.toml`, in priority order, with the standard plugin invocation protocol (payload delivery, response collection, block-intent propagation).

2. **PostCompact routing**: When the harness fires a `PostCompact` event, `factory-dispatcher` invokes all plugins registered under `event = "PostCompact"` in `hooks-registry.toml`, in priority order. PostCompact is advisory-only at the harness level; the dispatcher propagates exit codes but the harness does not honour block-intent on PostCompact regardless.

3. **No-op when no plugins registered**: If no `[[hooks]]` entry declares `event = "PreCompact"` or `event = "PostCompact"`, the dispatcher returns without error when these events are received. Zero-plugin handling is identical to all other event types.

4. **Block-intent propagation on PreCompact**: The dispatcher propagates `block_intent = true` (exit 2) from PreCompact sync plugins to the harness. This allows `precompact-flush.sh` to block compaction on flush failure per ADR-026 Decision 6.

5. **On_error semantics**: `on_error = "continue"` on a PreCompact plugin means a plugin crash does not block compaction. `on_error = "block"` on a PreCompact plugin means a crash IS treated as a block. ADR-026 Decision 6 mandates `on_error = "continue"` for `precompact-flush.sh`.

6. **Release requirement**: Changes to `crates/factory-dispatcher/src/` require a release cut to propagate to the operator-level cache at `~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`. Develop-branch edits do not affect the cached dispatcher binary (per CLAUDE.md self-referential note).

## Invariants

1. **PreCompact and PostCompact are first-class event types**: They must be enumerated alongside existing event types (PreToolUse, PostToolUse, SubagentStop, etc.) in the dispatcher's event-type enum or match arms. An unknown-event fallback that silently discards these events is a specification violation.

2. **Async classification applies normally**: PreCompact and PostCompact plugins are subject to the same `async`/sync classification as all other plugins (BC-7.06.001 / ADR-019). `on_error = "block"` implies `async = false` as invariant.

3. **Harness-version precondition is non-blocking at dispatcher level**: The dispatcher does not check the harness version at runtime. The precondition (harness >= v2.1.105) is documented for operators and CI; the dispatcher routes the event if present regardless. On pre-v2.1.105 harnesses, PreCompact fires as notification-only; the dispatcher's routing is correct and the block-intent is simply not honoured by the older harness.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `event = "PreCompact"` entry exists but harness is pre-v2.1.105 | Dispatcher routes the event normally; harness ignores exit-2 block-intent. Flush runs but cannot prevent compaction. No dispatcher error. |
| EC-002 | `event = "PostCompact"` plugin returns exit 2 | Dispatcher records exit code; does NOT propagate block-intent to harness (PostCompact is advisory-only). Advisory finding logged. |
| EC-003 | Zero plugins registered under `event = "PreCompact"` at install time | Dispatcher receives event, matches zero plugins, returns exit 0 with zero plugins run. No error. |
| EC-004 | PreCompact plugin crashes (on_error = "continue") | Dispatcher treats crash as advisory; exits 0. Compaction proceeds unblocked. No session wedge. |
| EC-005 | Multiple plugins registered under `event = "PreCompact"` | Dispatcher invokes in priority order; applies same block-intent aggregation as other sync events (any exit-2 → overall block). |
| EC-006 | S-18.00 inspection reveals PreCompact/PostCompact already supported | Story S-18.00 becomes no-op verification story; this BC is trivially satisfied; story confirms routing via bats test. |
| EC-007 | S-18.00 inspection reveals routing absent | Story S-18.00 adds routing support; RC cut required before S-18.04/18.05 can be integrated. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Dispatcher receives `PreCompact` event; one plugin registered; plugin exits 0 | `plugins_run=1`, `block_intent=false`, harness proceeds with compaction | happy-path |
| Dispatcher receives `PreCompact` event; one plugin registered; plugin exits 2 | `plugins_run=1`, `block_intent=true`, compaction blocked | flush-failure-block |
| Dispatcher receives `PostCompact` event; one plugin registered; plugin exits 0 | `plugins_run=1`, advisory response only; no block propagated | happy-path-post |
| Dispatcher receives `PreCompact` event; zero plugins registered | `plugins_run=0`, `block_intent=false`, exit 0 | no-op |
| Dispatcher receives `PreCompact` event; `on_error = "continue"` plugin crashes | `block_intent=false`, exit 0; advisory log entry | crash-fail-open |
| bats test: `echo '{"event":"PreCompact",...}' | factory-dispatcher` | dispatcher routes to registered PreCompact plugin; exit code matches plugin exit code | integration |

## Related BCs

- BC-7.07.001 — depends on: precompact-flush.sh is the canonical PreCompact plugin whose routing this BC enables
- BC-7.07.002 — depends on: postcompact-reanchor.sh is the canonical PostCompact plugin whose routing this BC enables
- BC-7.06.001 — sibling: async/on_error classification rules apply equally to PreCompact/PostCompact entries
- BC-1.14.001 — sibling: dispatcher partition logic that separates async vs sync groups; PreCompact plugins follow the same partition rule

## Architecture Anchors

- `crates/factory-dispatcher/src/invoke.rs` (or equivalent dispatch routing) — must enumerate `PreCompact` and `PostCompact` alongside existing event variants
- `crates/factory-dispatcher/src/registry.rs` — `RegistryEntry.event` field must parse `"PreCompact"` and `"PostCompact"` without `RegistryError::UnknownEvent`
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]] event = "PreCompact"` entry for precompact-flush.sh; `event = "PostCompact"` entry for postcompact-reanchor.sh

## Story Anchor

S-18.00 (verification/addition of PreCompact/PostCompact dispatcher routing — wave-1 prerequisite for E-18)

## VP Anchors

TBD-VP — no VP explicitly assigned at F2 for this dispatcher routing BC; story-writer and test-writer assign integration test VP at F3.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| TBD-VP | Dispatcher routes PreCompact event to registered shell plugin and propagates exit-2 block-intent | integration bats test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the dispatcher-side routing obligation that enables the PreCompact flush (precompact-flush.sh) and PostCompact re-anchor (postcompact-reanchor.sh) hooks to receive harness events; without this routing, E-18 Parts B are non-functional |
| L2 Domain Invariants | TBD-DI — no existing domain invariant directly covers PreCompact/PostCompact event routing; new invariant candidate flagged for business-analyst |
| Architecture Module | SS-01 (Hook Dispatcher Core) — runtime routing is in `crates/factory-dispatcher/src/invoke.rs` |
| ADR | ADR-026 v1.0 Decision 11 — S-18.00 dispatcher routing verification/addition |
| Stories | S-18.00 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
