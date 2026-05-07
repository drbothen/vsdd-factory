---
document_type: adr
adr_id: ADR-019
status: accepted
accepted_date: 2026-05-07
date: 2026-05-07
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
PostToolUseFailure — is synchronous at the envelope. No per-event carve-outs.

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

`capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`, and
`session-end-telemetry` MUST be classified `async = true` to preserve the current
latency profile. They have no block intent; they must not gate the user.

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

- **BC-1.NN.001** (SS-01): Dispatcher partition contract — sync group runs await-all
  with tier ordering per ADR-008; async group fire-and-forget; `dispatcher_exit` is
  the sync group aggregate and is independent of async group exit codes.
- **BC-7.NN.001** (SS-07): Per-plugin `async: bool` declaration semantics in
  hooks-registry.toml v2 schema; CI lint invariant `on_error = "block"` ⇒ `async = false`.

Do not author these BCs here — PO is doing that in parallel.

Verification properties produced alongside this ADR:

- **VP-077**: Dispatcher partition correctness (Kani-provable) — partition function
  totality, set disjointness, union completeness, exit code independence
- **VP-078**: CI lint invariant — `on_error = "block"` implies `async = false` (integration)

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
