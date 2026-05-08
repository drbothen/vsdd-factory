---
document_type: epic
epic_id: "E-15"
version: "1.0"
status: draft
title: "Plugin Async Semantics — Registry-Layer Partition (single-shot delivery)"
prd_capabilities: [CAP-002, CAP-003, CAP-008]
subsystems_affected: [SS-01, SS-07, SS-09]
target_release: "v1.1"
story_count: 1
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: F3
cycle: v1.0-feature-plugin-async-semantics-pass-1
depends_on: []
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.08.001.md
  - .factory/specs/verification-properties/VP-077.md
  - .factory/specs/verification-properties/VP-078.md
  - .factory/specs/verification-properties/VP-079.md
  - .factory/specs/domain-spec/invariants.md
input-hash: "[to-be-computed-by-state-manager]"
---

# Epic E-15: Plugin Async Semantics — Registry-Layer Partition (single-shot delivery)

## Description

Move plugin async classification from the Claude Code envelope layer (`hooks.json`) to
the registry layer (`hooks-registry.toml`), closing the silent-block-bleed bug observed
in the prism audit of 2026-05-07. A single story delivers all four interlocking changes
atomically: registry schema v2 (per-plugin `async: bool` field), dispatcher partition
logic (sync group awaited + async group fire-and-forget), envelope synchronization
(remove `async: true` from PostToolUse and siblings in `hooks.json.template`), and a
CI lint invariant enforcing `on_error = "block"` implies `async = false`. The ADR-019
user decision of no phased rollout and no backwards compatibility makes this a
clean, single-commit behavioral change.

## Trigger / Motivation

**Prism project audit, 2026-05-07.**

Log source: `/Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl`
(212,398 lines, 21,721 unique dispatcher trace IDs, 92,385 total plugin invocations).

| Plugin | Invocations | exit_code=2 (block) | Block rate |
|--------|-------------|---------------------|------------|
| validate-template-compliance | 1,965 | **55** | 2.8% |
| validate-table-cell-count | ~1,011 | 200 | 19.8% |
| validate-changelog-monotonicity | ~1,011 | 169 | 16.7% |
| validate-state-size | ~1,011 | 111 | 11.0% |
| validate-input-hash | ~1,011 | 14 | 1.4% |

Every one of the 55 validate-template-compliance block decisions was silently discarded.
The user received no notification in any of the 55 cases; Claude Code proceeded.
Root cause: `PostToolUse` is declared `async: true` in `hooks.json.darwin-arm64`,
making the dispatcher's exit code invisible to Claude Code regardless of what individual
plugins with `on_error = "block"` return. (Verbatim from F1 §1.)

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-002 | Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins | P0 |
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-003 | Stream observability events to multiple configurable sinks | P1 |

## Capability Anchor Justification

**Primary anchor: CAP-002** ("Hook Claude Code tool calls and session/worktree lifecycle
events with sandboxed WASM plugins") per `domain-spec/capabilities.md` §CAP-002. E-15
fixes the contract between dispatcher and Claude Code at every PostToolUse, Stop,
SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, and
PostToolUseFailure event — the full lifecycle event set that CAP-002 governs. Without
this fix, every plugin with `on_error = "block"` bound to these events is structurally
inert: CAP-002's "block a tool call (exit 2)" outcome is unreachable for the affected
event types.

**CAP-008 anchor:** The gate-tool-calls capability is directly impaired by the bug.
PreToolUse was already synchronous; the remaining blocking-plugin surface (PostToolUse
validators) gains equivalent block enforcement through this epic. CAP-008 requires
that block-mode plugins actually block — E-15 closes the gap for the PostToolUse
event class.

**CAP-003 anchor:** Four new event types introduced by BC-3.08.001 (`plugin.async_start`,
`plugin.async_complete`, `plugin.async_timeout`, `plugin.async_block_discarded`) enter
the observability stream through this change. These events are SS-03 artifacts that
make async-group behavior visible in `events-YYYY-MM-DD.jsonl`.

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-15.01 | Plugin async semantics — full implementation (schema v2 + dispatcher partition + plugin classification + envelope flip + CI lint) | TBD | none | draft |

**Story count: 1** (per ADR-019 §Decision 6 — no phased rollout; single-shot delivery).

## Problem Statement

### The silent-block-bleed bug

`hooks-registry.toml` entries declare `on_error = "block"` per plugin. The dispatcher
reads these declarations, routes events, collects exit codes, and computes the correct
block verdict. But `hooks.json.darwin-arm64` wraps every PostToolUse call with
`"async": true`. Claude Code fires the dispatcher as a detached process, never reads
its exit code, and discards stdout before the `block_with_fix` message can reach the
user. The registry contract is honored by the dispatcher. The block verdict is computed
correctly. It is simply never delivered.

### The architectural mismatch

Async classification belongs at the plugin level (per plugin's operational semantics),
not at the event-envelope level (per event type). An envelope-level `async: true` flag
is a blunt instrument: one flag silences every plugin matched to that event, regardless
of individual `on_error` declarations. The registry is the correct location for async
classification — it has per-plugin resolution, while the Claude Code envelope does not
(OQ-3 confirmed: Claude Code's `async` flag is per-envelope, not per-plugin; no
mechanism exists to express per-plugin async at the envelope layer).

ADR-019 formalizes this principle: the envelope becomes uniformly synchronous;
per-plugin async classification moves into `hooks-registry.toml`; the dispatcher
partitions matched plugins at runtime into sync (await-all) and async (fire-and-forget)
groups.

## Goals

1. Validators with `on_error = "block"` actually block — the 55 silent discards from
   the prism audit become 55 delivered `block_with_fix` messages.
2. Telemetry plugins (`capture-commit-activity`, `capture-pr-activity`,
   `session-start-telemetry`, `session-end-telemetry`, `track-agent-start`,
   `track-agent-stop`, `session-learning`) remain async — no user-visible latency
   impact for non-blocking observability work.
3. CI lint (VP-078) catches future regressions: no plugin may have both
   `on_error = "block"` and `async = true` in `hooks-registry.toml`.
4. Kani-provable partition function (VP-077): `partition_plugins()` is pure, total,
   and produces disjoint sync/async groups covering all matched plugins.
5. Hooks.json envelope synchronous for all events (BC-9.01.006): no entry carries
   `async: true` in any platform variant.

## Non-Goals

- Porting `verify-git-push.sh` to native WASM — stays bash per E-8 D-1.
- Changing any individual plugin's `on_error` semantics — only `async` classification
  is new; existing `on_error = "block"` / `on_error = "continue"` decisions are
  preserved verbatim.
- A backwards-compatible migration path — per ADR-019 §Decision 5 (user explicit
  decision): v2 dispatcher hard-errors on v1 registry; no compat shim; no migration
  tooling.
- Phased delivery (Phases A/B/C/D from F1 §12) — per ADR-019 §Decision 6: all
  components ship atomically in S-15.01.
- Downstream product announcement or coordinated rollout — deliberate non-goal per
  user decision; downstream products must upgrade registry to schema_version = 2.

## Decisions

### D-15.1: Single-story delivery (ADR-019 §Decision 6)

All four changes — schema v2, dispatcher partition, plugin classification, envelope
flip, CI lint — ship atomically in S-15.01. The four-phase migration sketch in F1 §12
is superseded. A local latency canary (S-15.01 AC) substitutes for a release-gate
canary.

### D-15.2: No backwards compatibility (ADR-019 §Decision 5)

A v2 dispatcher receiving a v1 registry emits a hard schema-version error at startup.
No downgrade path. No compat shim. Downstream products add `schema_version = 2` to
`hooks-registry.toml` before deploying the new dispatcher binary.

### D-15.3: Hard schema cut in BC-7.06.001

BC-7.06.001 Postcondition 4: dispatcher hard-errors on schema_version != 2 (no compat
reading of v1 registries). This is the normative implementation of D-15.2 at the
behavioral-contract level.

### D-15.4: Latency canary as story AC, not release gate (ADR-019 §Consequences)

P95 latency of the sync_group on a representative Edit/Write workload must be measured
as part of S-15.01 acceptance criteria. Threshold: sync_group p95 < 500ms (from F1
R-1; estimated peak 30–100ms with tier-parallelism per ADR-008, observed prism max
409ms). Evidence stored at `docs/demo-evidence/S-15.01/`.

### D-15.5: ASYNC_DRAIN_WINDOW_MS governed by DI-019

After sync_group completes, the dispatcher waits up to `ASYNC_DRAIN_WINDOW_MS`
(canonical value: DI-019) for async tasks to emit terminal events before exit. This
bounds async-event capture without allowing runaway async plugins to delay the user.
Truncated async tasks emit no terminal event. See BC-1.14.001 PC4.

## Risks

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---------|-------------|-----------|--------|------------|
| R-E15-001 | Latency regression on PostToolUse sync conversion — sync_group p95 exceeds 500ms if plugins are mis-classified async=false when they should be async=true | MED | HIGH | S-15.01 includes a complete plugin classification audit for all plugins in hooks-registry.toml; latency canary (S-15.01 AC-2) gates merge; VP-078 CI lint prevents future sync-tagged blocking regressions |
| R-E15-002 | Telemetry truncation — async tasks not complete within ASYNC_DRAIN_WINDOW_MS are silently terminated; telemetry events lost | LOW | MED | Acceptable per BC-1.14.001 PC4 + DI-019 (drain window is deliberate best-effort bound); VP-079 fault injection verifies truncation behavior is clean (no panic, no data corruption) |
| R-E15-003 | No backcompat — v2 dispatcher + v1 registry causes hard startup error in downstream products | HIGH | MED | Deliberate per ADR-019 §Decision 5 (user explicit non-mitigation); release notes must document registry upgrade requirement |
| R-E15-004 | Scope conflict with E-9 (W-16) and E-11 (W-17) registry migrations — both modify hooks-registry.toml plugin entries; the schema_version bump in S-15.01 must be coordinated | MED | MED | S-15.01 does not modify plugin behavior entries; only adds `schema_version = 2` and `async = true` for telemetry plugins. E-9/E-11 add new plugin entries without touching schema_version. No hard sequencing constraint; story merges require conflict resolution on `hooks-registry.toml` |

## Acceptance Criteria

| AC | Statement |
|----|-----------|
| AC-1 | All 5 new behavioral contracts implemented: BC-1.14.001 (dispatcher partition), BC-7.06.001 (registry schema v2 + CI lint), BC-9.01.006 (envelope sync), BC-3.08.001 (4 new async event types), BC-1.08.001 v1.2 (fail-closed exception) |
| AC-2 | Local latency canary passes: sync_group p95 < 500ms on representative Edit/Write workload; measurement evidence at `docs/demo-evidence/S-15.01/latency-canary.md` |
| AC-3 | All 9 amended BCs' invariants honored — verify via diff that amended BC postconditions are implemented in the new dispatch loop code |
| AC-4 | VP-078 lint integration green: 3 harnesses pass (pre-commit hook, CI bats test, dispatcher parse-time check); no entry in `hooks-registry.toml` has both `on_error = "block"` and `async = true` |
| AC-5 | VP-079 fault injection green: all 5 scenarios pass (drain window expiry + truncation, async block discarded event, sync group exit independence, schema conformance for all 4 new event types, async plugin crash isolation) |
| AC-6 | VP-077 Kani proof passes on `partition_plugins()`: 6 properties verified (totality, async-field respect, disjointness, union completeness, exit-code independence from async group, aggregation correctness) |
| AC-7 | `hooks.json.template` and all 5 platform-specific variants contain no `async: true` entries (BC-9.01.006 invariant); verified by grep in CI |
| AC-8 | Demo evidence at `docs/demo-evidence/S-15.01/` includes: (a) before/after dispatcher-internal log showing a previously-silent block_with_fix now surfaced; (b) latency canary output; (c) VP-077 Kani proof run transcript; (d) VP-078 lint pass; (e) VP-079 fault injection pass |

## Open Questions

No open questions from F2 convergence remain unresolved. OQ-1 through OQ-5 from F1
were resolved as follows:

- OQ-1 (async block discarded): resolved — emit `plugin.async_block_discarded` event
  (BC-3.08.001 §event-type-4); no user-facing notification.
- OQ-2 (sync parallelism cap): resolved in ADR-008 scope; tier-parallel model unchanged;
  no pool cap introduced in this cycle.
- OQ-3 (per-plugin async at envelope): resolved — architecturally impossible per Claude
  Code hook API; documented as known constraint in ADR-019 §Context.
- OQ-4 (coordinated release): resolved — hard startup error per D-15.2; no activation
  skill change; release notes carry the migration requirement.
- OQ-5 (canonical test for synchronous envelope): resolved — VP-079 Scenario 4 tests
  drain window; grep for `async: true` absence is the static invariant (BC-9.01.006).

## Architecture Mapping

References: ADR-019 §Subsystem Assignments.

| Component | Subsystem | Module |
|-----------|-----------|--------|
| `registry.rs` — `REGISTRY_SCHEMA_VERSION` bump to 2; `async` field on `RegistryEntry`; `validate()` rejects schema != 2 | SS-01 | `crates/factory-dispatcher/src/registry.rs` |
| `routing.rs` or `engine.rs` — `partition_plugins(matched, registry)` pure function | SS-01 | `crates/factory-dispatcher/src/{routing,engine}.rs` |
| Dispatch loop (`run_event`) — sync_group await-all; async_group spawn-detached; drain window | SS-01 | `crates/factory-dispatcher/src/engine.rs` |
| `hooks-registry.toml` — `schema_version = 2`; `async = true` for telemetry plugin set per BC-7.06.001 Invariant 6 | SS-07 | `plugins/vsdd-factory/hooks-registry.toml` |
| CI lint bats test / pre-commit hook — `on_error = "block"` implies `async = false` | SS-07 | `plugins/vsdd-factory/hooks/*.sh` or CI pipeline |
| `hooks.json.template` + 5 platform variants — remove `async: true` from all entries | SS-09 | `plugins/vsdd-factory/hooks/hooks.json*` |

## Dependency Graph

```
S-15.01 (plugin async semantics — full implementation)
    no hard upstream blocking dependencies
    (E-9/E-11 run concurrently; hooks-registry.toml conflicts resolved at merge)
    ↓
AC-2 latency canary + AC-6 Kani proof + AC-4/AC-5 VP-078/VP-079 green
    ↓
E-15 DONE
```

E-9 (W-16) closure is NOT a hard prerequisite. E-9 adds new plugin entries to
`hooks-registry.toml` without modifying `schema_version`. S-15.01 bumps
`schema_version = 2` and adds `async = true` to telemetry plugins. These are
non-overlapping changes; any merge conflict is mechanical (TOML section ordering).
E-11 (W-17) similarly adds native WASM entries; same non-overlapping analysis applies.

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| 1.0 | 2026-05-07 | product-owner | Initial authoring. Single story (S-15.01). Epic ID E-15 (E-12 through E-14 occupied by engine-discipline-pass-1 cycle). CAP anchors: CAP-002 (primary), CAP-008, CAP-003. Subsystems: SS-01, SS-07, SS-09. No dependency on E-9/E-11. ADR-019 v1.8 is the authoritative scope source. |
