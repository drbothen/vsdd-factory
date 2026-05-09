---
document_type: adr
adr_id: ADR-019
status: accepted
accepted_date: 2026-05-07
date: 2026-05-07
version: "1.9"
cycle: v1.0-feature-plugin-async-semantics-pass-1
subsystems_affected: [SS-01, SS-07, SS-09]
supersedes: null
superseded_by: null
---

# ADR-019: Plugin Async Semantics Belong at the Registry Layer, Not the Envelope Layer

## Context

### Current execution model

Claude Code reads `hooks.json` (platform-specific variant under `plugins/vsdd-factory/hooks/`)
and fires hook events according to the envelope declarations in that file. Each event can be
declared synchronous or asynchronous via the `"async": true` field on the hook entry.

Today's envelope for PostToolUse (from `hooks.json.darwin-arm64`, confirmed):

```json
"PostToolUse": [{"hooks": [{"type": "command",
  "command": "...factory-dispatcher", "timeout": 10000, "async": true}]}]
```

When `async: true` is set, Claude Code fires the dispatcher in a detached process, never
reads its exit code, and discards any stdout the dispatcher writes. The dispatcher runs
correctly — routing, invoking plugins, and collecting per-plugin exit codes — but its
result is structurally invisible to Claude Code.

Only `PreToolUse` and `PermissionRequest` are synchronous today. All other events —
PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove,
PostToolUseFailure — are declared `async: true` in the current envelope.

### Empirical evidence

Prism project audit, 2026-05-07 (`/Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl`):
- 1,965 `validate-template-compliance` invocations via PostToolUse
- 55 invocations returned `exit_code=2` (block intent) — all were `template_noncompliant`
  findings (S-3.03/04/05 and S-MAINT-001 missing `tdd_mode`)
- Every one of the 55 block decisions was silently discarded
- The user received no notification; Claude Code proceeded in all 55 cases
- The same pattern holds for every other validator plugin with `on_error = "block"` bound
  to PostToolUse or its siblings

### Root cause

`async: true` at the envelope layer is a blunt instrument. One flag silences the dispatcher's
exit code for every plugin matched to that event, regardless of what individual plugins declare
in `hooks-registry.toml`. A plugin that explicitly declares `on_error = "block"` in the
registry cannot enforce that declaration when the envelope is async. The contract between
registry and dispatcher is technically sound; the contract between dispatcher and Claude Code
is broken at the envelope layer.

The dispatcher itself is correct. The fault is one layer above.

### Known constraint (OQ-3)

Claude Code's `async` flag is per-envelope, not per-plugin. There is no mechanism to mark a
single plugin async at the Claude Code side without affecting every other plugin matched to that
event. The only way to achieve per-plugin async granularity is to move async classification
into the registry and have the dispatcher implement the partition at runtime.

---

## Decision

### 1. hooks.json becomes uniformly synchronous

The `"async": true` flag is removed from all event declarations in `hooks.json.template` and
all five platform-specific variants. Every Claude Code hook event — PreToolUse, PostToolUse,
Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove,
PostToolUseFailure, PermissionRequest — is synchronous at the envelope. No per-event
carve-outs.

Note on PermissionRequest: this event was already synchronous prior to ADR-019 (it was never
declared `async: true` in any envelope variant). It is enumerated here for completeness —
ADR-019 imposes no behavior change for PermissionRequest; the decision simply preserves its
existing synchronous status and makes the full event enumeration explicit.

**User explicit decision: every Claude Code hook event must be sync at the envelope. No
per-event carve-outs in hooks.json. Cited verbatim from user decisions, 2026-05-07.**

### 2. Per-plugin `async: bool` field added to hooks-registry.toml (schema v1 → v2)

The registry schema gains a new field in every `[[hooks]]` entry:

```toml
async = false   # default; omit = sync
```

Telemetry plugins — `capture-commit-activity`, `capture-pr-activity`,
`session-start-telemetry`, `session-end-telemetry` — are classified `async = true`.
All validator plugins with `on_error = "block"` remain `async = false` (default; may
be made explicit for clarity).

`registry.rs` bumps `REGISTRY_SCHEMA_VERSION` to 2. The `validate()` function accepts
the new field and the new schema version.

### 3. Dispatcher partitions matched plugins into sync and async groups

In the dispatch loop (`engine.rs` / `run_event`):

```
matched = match_plugins(registry, payload)

sync_group  = [p for p in matched if !p.async]   // async=false or unset
async_group = [p for p in matched if  p.async]   // async=true

// Sync group: run tiers per ADR-008, await all, aggregate exit code
sync_results = run_tiers(sync_group)
dispatcher_exit = if any_block_intent(sync_results) { 2 } else { 0 }

// Async group: spawn detached tasks, fire-and-forget
// Exit codes NEVER influence dispatcher_exit
for p in async_group { spawn_detached(p) }

return dispatcher_exit   // Claude Code reads this
```

ADR-008 (parallel-within-tier, sequential-between-tier) continues to govern sync group
execution unchanged. Async group plugins are excluded from the tier ordering model
entirely — they are unordered and fire-and-forget. This is an annotation to ADR-008,
not a supersession.

### 4. CI lint invariant: `on_error = "block"` implies `async = false`

A CI lint (bats test or pre-commit hook) scans `hooks-registry.toml` and asserts that no
entry has both `on_error = "block"` and `async = true`. This invariant is also enforced
at parse time inside the dispatcher. Defense in depth: parse-time, pre-commit, and CI.

### 5. No backwards compatibility — hard schema v2 requirement

**User explicit decision: no backwards compatibility. v2 dispatcher hard-errors on v1
registry. Single hard cut. No migration tooling. No coordinated announcement. Cited
verbatim from user decisions, 2026-05-07.**

A product installing a v2 dispatcher with a v1 registry receives a hard schema-version
error at startup. There is no compat shim. No downgrade path. Products must update
`schema_version = 2` in `hooks-registry.toml` before using the new dispatcher.

### 6. No phased rollout — single story delivery

**User explicit decision: no phased rollout. All changes ship together in one story:
schema v2 + dispatcher partition + plugin classification + envelope flip + CI lint.
Latency canary is a local benchmark in the story's acceptance criteria, not a release
gate. Cited verbatim from user decisions, 2026-05-07.**

The four-phase migration sketch in F1 (Phases A/B/C/D) is superseded by this decision.
All components ship atomically in a single story.

---

## Consequences

### User-visible behavior change

Validators with `on_error = "block"` now actually block. Edits and Writes can fail
synchronously with a `block_with_fix` message surfaced to the user. This is the intended
behavior. Prior to this change, all 55 block decisions in the prism audit were silently
discarded.

### Latency on PostToolUse

Converting PostToolUse to synchronous means Claude Code blocks for the duration of the
sync group's execution. Current prism median: ~29ms per validator plugin; observed max:
~409ms. With priority-tier parallelism (ADR-008), the expected PostToolUse overhead is
bounded by the slowest plugin in the slowest tier, not the sum. Estimated peak: 30–100ms
for a representative Edit/Write event.

Mitigation: plugin classification audit is part of the implementation story; a local
latency benchmark is a story acceptance criterion. Misclassification risk (heavy plugin
tagged sync) is caught in code review for the `async = false` case; the CI lint catches
the converse case (`on_error = "block"` tagged `async = true`).

### Telemetry plugins must be classified async

`capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`,
`session-end-telemetry`, `track-agent-start`, `track-agent-stop`, and `session-learning`
MUST be classified `async = true` to preserve the current latency profile. They have no
block intent and always return `Continue`; they must not gate the user.

Plugins with `on_error = 'continue'` that emit user-visible stderr warnings
(e.g., `warn-pending-wave-gate`, `regression-gate`) are deliberately classified SYNC
despite being non-blocking, so that the dispatcher's parent process delivers their output
reliably before exit. Telemetry-only continue plugins (`track-agent-start`,
`track-agent-stop`, `session-learning`, `capture-commit-activity`,
`capture-pr-activity`, `session-start-telemetry`, `session-end-telemetry`,
`worktree-hooks`, `tool-failure-hooks`) are classified ASYNC. See BC-7.06.001 Invariant 6
for the canonical async-required list.

### Async-task drain window

After `sync_group` completes, the dispatcher waits up to `ASYNC_DRAIN_WINDOW_MS`
(governed by DI-019) for async tasks to emit terminal events before
exit. See BC-1.14.001 PC4 for the dispatch-loop partition contract; DI-019 for the
canonical constant value; VP-079 for the schema-conformance verification. This allows
`plugin.timeout` and `plugin.async_block_discarded` events to reach FileSink before
the dispatcher process terminates.

Total dispatcher latency (from event receipt to dispatcher exit) is bounded by:

```
latency = max(sync_plugin_durations) + min(max(async_plugin_durations), ASYNC_DRAIN_WINDOW_MS)
```

Async tasks not complete by drain expiry are forcibly terminated and emit no event
(truncation). The drain window is a best-effort guarantee: it bounds the async-event
capture window without allowing runaway async plugins to delay the user's tool call
response beyond `max(sync_plugin_durations) + ASYNC_DRAIN_WINDOW_MS`.

### Schema v2 hard-error on v1 registry — deliberate

No downgrade. No compat shim. See user explicit decision above.

### ADR-008 annotation

Async group plugins are excluded from the tier ordering model. ADR-008's parallel-within-
tier invariant applies only to the sync group. No supersession; annotation only.

---

## Alternatives Considered

### (a) Keep envelope async; add a Stop hook that surfaces block decisions post-hoc

Rejected as the primary fix for this cycle. A post-hoc Stop hook can report that async
block decisions were made but cannot actually block the user — the tool use has already
completed. This does not fix the root cause. Tracked separately as TD-027 for
residual-bleed coverage after this cycle.

### (b) Per-event async config in hooks.json (e.g., `"async": ["telemetry"]` whitelist)

Rejected. This duplicates registry routing logic into the envelope, creates a second
source of truth for plugin classification, and ties async semantics to event types rather
than individual plugin intent. The registry is the correct location for per-plugin routing
decisions.

### (c) Phased rollout (Phases A→B→C→D as sketched in F1)

Rejected per user explicit decision. Single user, no migration risk, single-story delivery
simplifies the model and eliminates the partial-state window between Phase A and Phase C.

### (d) Per-plugin async override at the hooks.json level

Technically impossible. Claude Code's `async` flag is per-envelope, not per-plugin
(confirmed: Claude Code hook API does not expose per-plugin async configuration). This
is the architectural constraint that makes the registry-layer approach the only viable
path to per-plugin granularity (OQ-3 from F1 delta analysis).

---

## Implementation Pointers

The following BCs codify the partition contract (PO authors these in parallel):

- **BC-1.14.001** (SS-01): Dispatcher partition contract — sync group runs await-all
  with tier ordering per ADR-008; async group fire-and-forget; `dispatcher_exit` is
  the sync group aggregate and is independent of async group exit codes.
- **BC-7.06.001** (SS-07 filed; authoritative subsystem SS-01 per F-P1-006 reanchoring): Per-plugin `async: bool` declaration semantics in
  hooks-registry.toml v2 schema; CI lint invariant `on_error = "block"` ⇒ `async = false`.
- **BC-9.01.006** (SS-09): hooks.json.template envelope uniformly synchronous — every event entry has the `async` key absent or `async: false`; no entry has `async: true`. Addresses F-P1-001.

Do not author these BCs here — PO authored BC-1.14.001, BC-7.06.001, and BC-9.01.006.

Verification properties produced alongside this ADR:

- **VP-077**: Dispatcher partition correctness (Kani-provable) — partition function
  totality, async-field respect, disjointness, union completeness, exit-code independence
  from async group, aggregation correctness (6 properties; VP-077 §Property Statement
  is canonical)
- **VP-078**: CI lint invariant — `on_error = "block"` implies `async = false` (integration)
- **VP-079**: Async-semantics event payload schema conformance — each of the four event
  types introduced by this ADR conforms to BC-3.08.001 schema (integration)

---

## Subsystem Assignments

**SS-01 (Hook Dispatcher Core):** Referencing SS-01 because the partition logic lives
in `crates/factory-dispatcher/src/` — `registry.rs` (schema bump), `routing.rs` or
`engine.rs` (partition implementation), and the dispatch loop. The `dispatcher_exit`
aggregation invariant is an SS-01 behavioral contract.

**SS-07 (Hook Bash Layer):** Referencing SS-07 because `hooks-registry.toml` is the
SS-07 configuration artifact (per Subsystem Registry). The per-plugin `async` field and
the CI lint invariant are SS-07 obligations.

**SS-09 (Configuration and Activation):** Referencing SS-09 because `hooks.json.template`
and the five platform-specific variants are SS-09 artifacts (per Subsystem Registry,
`plugins/vsdd-factory/hooks/hooks.json*`). The envelope synchronization change lives here.
BC-9.01.006 (authored by PO in F2 pass-1 fix burst) codifies the envelope-sync invariant
for this subsystem: every event entry must have `async` absent or `async: false`; any
entry with `async: true` is a hard violation caught by CI lint and VP-079.

---

## References

| Artifact | Path |
|----------|------|
| F1 delta analysis | `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` |
| Prism dispatcher audit log | `/Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl` |
| Envelope file (darwin-arm64) | `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks/hooks.json.darwin-arm64` |
| Registry schema | `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml` |
| Dispatcher registry parser | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/registry.rs` |
| Dispatcher engine | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/engine.rs` |
| ADR-008 (parallel-within-tier) | `decisions/ADR-008-parallel-within-tier.md` |
| ADR-011 (dual routing tables) | `decisions/ADR-011-dual-hook-routing-tables.md` |
| VP-077 | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-077.md` |
| VP-078 | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-078.md` |
| VP-079 | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-079.md` |

---

## Amendment 2026-05-08: v1.8 → v1.9 (F-P19-001 corpus-wide L-P18-002 sweep)

- **Amendment date:** 2026-05-08
- **Reason:** F-P19-001 corpus-wide L-P18-002 prose-form sweep identified 2 surviving prose-form
  line references in the `v1.5 → v1.6` amendment body (refs to "line 209" and "line 209"
  again for the total-latency formula and the closing sentence respectively).
- **Changes:**
  - `v1.5 → v1.6` amendment body: both `on line 209` references replaced with stable section
    anchors identifying the `§Consequences` "Total dispatcher latency" code block and the
    "Async-task drain window" paragraph closing sentence.
- **No decision changes:** All §Decision policy text is unchanged. Amendment narrative only.

## Amendment 2026-05-07: v1.0 → v1.1 (F2 pass-1 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** Adversary pass-1 finding F-P1-018 identified that §Implementation Pointers used placeholder BC IDs (`BC-1.NN.001`, `BC-7.NN.001`) from the pre-burst draft; the actual authored BCs are `BC-1.14.001` and `BC-7.06.001`.
- **Changes:**
  - §Implementation Pointers line 1: `BC-1.NN.001` → `BC-1.14.001`
  - §Implementation Pointers line 2: `BC-7.NN.001` → `BC-7.06.001`
- **Forward reference (RESOLVED by state-manager close-burst 2026-05-07):** PO assigned BC-9.01.006 (SS-09) for the hooks.json.template envelope-sync invariant (F-P1-001). BC-9.01.006 has been added to §Implementation Pointers and §Subsystem Assignments under SS-09. Forward reference closed.
- **No decision changes:** All §Decision entries are unchanged. This amendment corrects stale text only.

## Amendment 2026-05-07: v1.7 → v1.8 (F2 pass-6 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** Adversary pass-6 finding F-P5-005. §Consequences "Async-task drain window"
  paragraph retained `(governed by DI-019; default 100ms)` — an inline literal alongside the
  canonical reference. BC-1.14.001 v1.5 removed this exact pattern for consistency (DI-019 is
  the single canonical home for the constant value). The parenthetical `default 100ms` in the
  ADR §Consequences section is referenceable from VP-079 fixtures; a literal that drifts from
  DI-019 would propagate silently. Enforcing consistency: single canonical home wins.
- **Changes:**
  - §Consequences "Async-task drain window" (sentence starting "After `sync_group` completes"):
    `(governed by DI-019; default 100ms)` → `(governed by DI-019)`. The `default 100ms` detail
    belongs exclusively in DI-019.
- **No decision changes:** All §Decision policy text is unchanged. This is a constant-reference
  consistency correction only.

## Amendment 2026-05-07: v1.6 → v1.7 (F2 pass-5 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** Adversary pass-5 finding F-P5-003. §References table listed VP-077 and VP-078
  but omitted VP-079, despite VP-079 being cited in §Implementation Pointers (line 278).
  Parity gap between §References and §Implementation Pointers.
- **Changes:**
  - §References table: added VP-079 row (`/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-079.md`).
- **No decision changes:** All §Decision policy text is unchanged. This amendment restores
  §References parity with §Implementation Pointers.

## Amendment 2026-05-07: v1.5 → v1.6 (F2 pass-4 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** Adversary pass-4 finding F-P4-004. §Consequences "Async-task drain window"
  inline bound sentence (final line of the paragraph) inlined the literal `100ms` value
  despite the DI-019 lift principle (v1.4→v1.5 amendment) which elevated this constant
  from inline to canonical domain invariant. The earlier total-latency formula in the
  `§Consequences` "Total dispatcher latency" code block already uses the symbolic
  `ASYNC_DRAIN_WINDOW_MS` form; the closing sentence of the "Async-task drain window"
  paragraph must match.
- **Changes:**
  - §Consequences "Async-task drain window" (paragraph closing sentence): replaced
    `+ 100ms` → `+ ASYNC_DRAIN_WINDOW_MS` to be consistent with the "Total dispatcher
    latency" formula in `§Consequences` and with the DI-019 symbolic convention.
- **No decision changes:** All §Decision policy text is unchanged. This is a symbolic
  consistency correction only — the constant value remains 100ms per DI-019.

## Amendment 2026-05-07: v1.4 → v1.5 (F2 pass-3 fix burst revision)

- **Amendment date:** 2026-05-07
- **Reason:** User revised Q3 decision toward most-correct: `ASYNC_DRAIN_WINDOW_MS` lifted to
  domain invariant DI-019 (PO authoring in `domain-spec/invariants.md`). ADR-019 §Consequences
  drain window subsection must cite DI-019 as canonical source rather than inlining the
  constant value via BC-1.14.001 v1.3.
- **Changes:**
  - §Consequences "Async-task drain window": sentence "per BC-1.14.001 v1.3" replaced with
    "governed by DI-019; default 100ms". Added cross-reference triptych:
    "BC-1.14.001 PC4 for partition contract; DI-019 for canonical constant value; VP-079 for
    verification."
- **No decision changes:** All §Decision policy text is unchanged. This amendment elevates
  the constant's canonical home from an inline BC postcondition to a domain invariant.
- **Forward reference (open):** DI-019 ID is a placeholder pending PO authoring of the
  invariant in `domain-spec/invariants.md`. State-manager resolves to actual ID on close.

## Amendment 2026-05-07: v1.3 → v1.4 (F2 pass-3 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** PO amended BC-1.14.001 to v1.3, adding a bounded `ASYNC_DRAIN_WINDOW_MS`
  (default 100ms) to the dispatch loop contract. This resolves adversary findings
  F-P3-002 and F-P3-007 (VP-079 Scenarios 1+4 structurally untestable under the original
  "exit immediately after sync_group" PC4 semantics). ADR-019 §Consequences must reflect
  the updated latency budget.
- **Changes:**
  - §Consequences: added "Async-task drain window" subsection documenting
    `ASYNC_DRAIN_WINDOW_MS = 100ms` (default), the total-latency formula
    `max(sync) + min(max(async), drain)`, and the truncation behavior for async tasks
    that exceed the drain window.
- **No decision changes:** All §Decision policy text is unchanged. The drain window is
  an addendum to the existing PC4 semantics, not a reversal of the fire-and-forget
  model for `dispatcher_exit` determination.

## Amendment 2026-05-07: v1.2 → v1.3 (F2 pass-2 fix burst close)

- **Amendment date:** 2026-05-07
- **Reason:** PO's F-P2-006 fix expanded BC-7.06.001 Invariant 6 from 6 to 9 async-required
  plugins. ADR-019 §Consequences "Telemetry plugins must be classified async" required sync
  to reflect the expanded list and to document the deliberate SYNC classification rationale
  for `warn-pending-wave-gate` and `regression-gate`.
- **Changes:**
  - §Consequences "Telemetry plugins must be classified async": expanded telemetry plugin
    list to include `track-agent-start`, `track-agent-stop`, `session-learning`; added
    clarification paragraph: `warn-pending-wave-gate` and `regression-gate` are deliberately
    SYNC (on_error=continue) because they emit user-visible stderr warnings that must be
    delivered reliably before dispatcher exit; telemetry-only plugins are ASYNC. Canonical
    list reference: BC-7.06.001 Invariant 6.
- **No decision changes:** All §Decision policy text is unchanged. This amendment corrects
  §Consequences prose to match the expanded Invariant 6 classification list.

## Amendment 2026-05-07: v1.1 → v1.2 (F2 pass-2 fix burst)

- **Amendment date:** 2026-05-07
- **Reason:** Adversary pass-2 findings F-P2-011 and F-P2-012.
- **Changes:**
  - §Decision 1 event enumeration: added `PermissionRequest` to the ten-event list with
    an explanatory note that PermissionRequest was already synchronous prior to ADR-019
    and this ADR imposes no behavior change for it (F-P2-012). BC-1.14.001 Precondition 2
    and BC-9.01.006 Postcondition 2 enumerate 10 events including PermissionRequest; the
    ADR's decision enumeration now matches.
  - §Implementation Pointers VP-077 entry: expanded from 4 properties to the canonical 6
    (totality, async-field respect, disjointness, union completeness, exit-code independence,
    aggregation correctness) per VP-077 §Property Statement (F-P2-011).
  - §Implementation Pointers: added VP-079 entry (was produced alongside this ADR but not
    previously listed here).
- **No decision changes:** All §Decision policy text is unchanged. This amendment adds
  enumeration completeness and cross-reference accuracy only.
