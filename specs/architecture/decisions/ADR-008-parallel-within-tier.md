---
document_type: adr
adr_id: ADR-008
status: accepted
date: 2026-04-26
subsystems_affected: [SS-01]
supersedes: null
superseded_by: null
---

# ADR-008: Parallel-Within-Tier, Sequential-Between-Tier Execution

## Context

The v1.0 dispatcher can route multiple WASM plugins to the same Claude Code hook
event. For example, a `PostToolUse:Bash` event might match `capture-commit-activity`
(priority 100), `block-ai-attribution` (priority 50), and `legacy-bash-adapter`
(priority 900). The dispatcher must execute all matching plugins and aggregate their
results into a single exit code for Claude Code.

Three fundamental execution strategies were available:

1. Sequential: run all plugins in series, priority order. Simple but additive latency:
   `sum(plugin_times)` becomes the wall-clock cost of the hook event.
2. Parallel (all): run all plugins concurrently regardless of priority. Minimal
   wall-clock but provides no ordering guarantee — a plugin that must run before
   another cannot express that constraint.
3. Tiered: group plugins by priority tier; run all plugins within a tier in parallel;
   complete all tiers before starting the next tier.

The v0.79.x hook system had no concept of parallelism — hooks were bash scripts
invoked serially by the Claude Code harness's matcher system. Introducing
parallelism in v1.0 required an explicit model that operators could reason about
when configuring `hooks-registry.toml`.

## Decision

Plugins within the same priority tier run concurrently as tokio tasks; each task
wraps the synchronous wasmtime invocation in `spawn_blocking` so the async runtime
is not blocked. Priority tiers execute sequentially: all plugins in tier N complete
(or time out) before the dispatcher starts tier N+1. A plugin's `on_error = "block"`
verdict records a block intent but does not abort the current tier — remaining
plugins in that tier continue to execute. The aggregate exit code is `2` if any
block intent was recorded across all tiers, `0` otherwise.

The `executor.rs` module documents this contract in its module-level comment:
"Within a tier: plugins run concurrently via tokio tasks... Between tiers: the
dispatcher awaits every task before the next tier begins, preserving priority
ordering."

## Rationale

The parallel-within-tier model is the result of Open Question Q3 (design doc lines
743–790). It was chosen over the alternatives for three reasons:

First, it addresses the primary latency concern without sacrificing ordering guarantees.
A `PostToolUse` event with 3 independent plugins at different priorities runs the
first-priority plugins in parallel, completing in `max(P1 plugins)` not `sum(P1 plugins)`.
Only when all P1 plugins complete does the dispatcher advance to P2. This matches
Claude Code's own model: Claude Code runs all hooks for a given matcher in parallel
within a tier.

Second, priority tiers are the correct primitive for expressing ordering dependencies.
Operators who need plugin A to complete before plugin B set different priority values;
the tier model honors that. Operators who don't care about ordering leave plugins at
the same default priority and get parallel execution automatically.

Third, the `block` verdict non-abort-within-tier rule avoids a subtle race condition.
If a `block` verdict aborted the current tier immediately, a plugin at the same
priority that was already running would be interrupted mid-execution — potentially
leaving observability state inconsistent (an `emit_event` call in flight). Completing
the tier before applying the aggregate block decision is cleaner and more auditable.

The `spawn_blocking` wrapper for each wasmtime invocation is a deliberate async
architecture choice: wasmtime module execution is synchronous (it's not async-aware
WASM), and blocking the tokio runtime thread would limit parallelism. `spawn_blocking`
pushes each invocation onto the blocking thread pool, freeing the async executor for
I/O work (sink drain, internal log writes).

## Consequences

### Positive

- Hook invocation wall-clock cost scales as `sum(max(tier_latency))` rather than
  `sum(all_plugin_latency)`, often an order-of-magnitude improvement on events with
  multiple independent hooks.
- Priority tiers provide an explicit, declarative mechanism for ordering — no need
  for plugin-level synchronization primitives.
- `block` verdicts are collected from all plugins in a tier before Claude Code
  receives the exit code, ensuring complete audit coverage even when blocking.

### Negative / Trade-offs

- Parallel execution within a tier means event ordering in the sink pipeline is
  non-deterministic within a tier. Events from plugins in the same tier may arrive
  out-of-insertion order. The `dispatcher_trace_id` and per-event timestamps are the
  reconstruction primitive, not order of arrival.
- `spawn_blocking` consumes tokio blocking threads. High fan-out events (many plugins
  at the same priority) on low-core machines may see thread pool pressure. In practice,
  v1.0 has 2–5 plugins per event on most factories.
- The semantics of "complete the tier even if a block is requested" may be surprising
  to operators who expect a `block` to immediately halt processing. The design
  doc captures this as the explicit Q3 resolution.

### Status as of v1.0.0-beta.5

IN-EFFECT. `crates/factory-dispatcher/src/executor.rs` implements tiered parallel
execution with `spawn_blocking` per plugin and sequential tier advancement. The
`TierExecutionSummary` struct accumulates per-plugin outcomes; `exit_code` is `2`
iff any `block` intent was recorded. The `routing.rs` module's `group_by_priority`
function produces the ordered tier vec consumed by the executor.

## Alternatives Considered

- **Sequential execution:** Run plugins in priority order, one at a time. Rejected:
  wall-clock cost is additive; `sum(timeouts)` is pathological for events with
  multiple hooks. The design doc (line 765) notes this explicitly.
- **Fully parallel (ignore priority):** Run all plugins concurrently regardless of
  priority. Rejected: provides no mechanism for a plugin to express that it must
  run after another; ordering dependencies cannot be configured.
- **Parallel with abort-on-block:** When a `block` verdict arrives, cancel all
  remaining tasks in the tier. Rejected: mid-execution cancellation of wasmtime
  stores leaves observability state inconsistent; block decisions should be applied
  after complete tier execution for audit integrity.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 143–149 (control flow diagram showing parallel tokio tasks per tier),
  lines 478–483 (parallel-within-tier captured as Q3 ADR decision),
  lines 743–790 (full Q3 resolution with alternatives and rationale for non-abort).
- **Code as-built:** `crates/factory-dispatcher/src/executor.rs:1–18` (module doc
  with explicit parallel-within-tier contract statement).
- **Code as-built:** `crates/factory-dispatcher/src/routing.rs` (`group_by_priority`
  function producing ordered tier vec).
