---
document_type: adr
adr_id: ADR-007
status: accepted
date: 2026-04-26
subsystems_affected: [SS-01, SS-03]
supersedes: null
superseded_by: null
---

# ADR-007: Always-On Dispatcher Self-Telemetry

## Context

The v1.0 multi-sink architecture (ADR-005) made the sink pipeline configurable:
operators can disable the default file sink, enable only remote sinks, or configure
no sinks at all during development. This flexibility introduced a diagnostic gap:
if all configured sinks are misconfigured, down, or not yet set up, there is no
persistent record of dispatcher activity. Debugging "why did this hook not fire?"
or "why did this plugin crash?" becomes impossible without a log.

The problem is circular: when a sink is misconfigured, the `internal.sink_error`
event that would announce the misconfiguration cannot reach any sink — including
the file sink that would normally persist it. The dispatcher needs a diagnostic
channel that is independent of the configurable sink pipeline and cannot be
disabled by sink configuration.

A secondary concern is the operator onboarding experience. The v1.0 activation
flow (ADR-009) writes a platform-specific `hooks.json` and verifies the dispatcher
binary, but a new operator has no configured sinks until they edit
`observability-config.toml`. For those operators, every hook invocation would be
invisible without a sink-independent log.

## Decision

The dispatcher writes a daily-rotated JSONL file at
`.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl` for every dispatcher invocation,
regardless of sink configuration. This file receives all `dispatcher.*` and
`internal.*` lifecycle events (start, plugin loaded, plugin invoked, plugin completed,
plugin timeout, plugin crashed, sink health, capability denied) plus any event emitted
via the `emit_event()` host function. It is always-on: there is no configuration knob
to disable it, and it does not appear in `observability-config.toml`. Writes are
best-effort — I/O errors are swallowed; `write` never panics and never propagates.

## Rationale

The design document explicitly evaluated two options for self-telemetry (Q6 in the
Open Questions section, lines 833–858):

Option A (structured): Route `internal.*` events through a dedicated internal sink
that is always-on and not configurable. This is essentially what was implemented,
but the "dedicated internal sink" framing would place internal events in the same
data flow as user events, sharing queue management and backpressure.

Option B (hardcoded local file): The dispatcher writes its own internal events to a
hardcoded local path unconditionally. This is the chosen approach. The rationale
from the design doc (line 855–858): "This is NOT user-visible observability (doesn't
violate zero-disk remote-only mode for factory-level events); it's a dispatcher-self-
diagnostic trail. Keeps the dispatcher debuggable when all configured sinks are
misconfigured or down."

The key distinction: `dispatcher-internal.jsonl` is not user-visible telemetry. If
an operator configures a remote-only setup, they correctly expect `.factory/logs/events-*.jsonl`
to be empty — and it will be. `dispatcher-internal.jsonl` is a debug artifact for
the dispatcher itself, analogous to a daemon's local process log, not part of the
operator's telemetry pipeline. This distinction means zero-disk remote-only mode is
honored from the operator's perspective.

Daily rotation (`dispatcher-internal-YYYY-MM-DD.jsonl`) with 30-day retention (the
`DEFAULT_RETENTION_DAYS: u32 = 30` constant in `internal_log.rs`) bounds disk usage
automatically. The rotation key is the event timestamp, not `now()`, so tests can
generate events with synthetic timestamps without mocking the system clock.

The "best-effort, never panic" write contract is a deliberate choice for a dispatcher
running on an operator's machine during Claude Code sessions. A dispatcher that panics
while writing its self-diagnostic log would cause the hook invocation to fail with an
opaque error, hiding the original problem it was trying to surface. Swallowing I/O
errors for the self-diagnostic log is the correct trade-off.

## Consequences

### Positive

- Operators can always recover diagnostic information about a hook invocation by
  inspecting `dispatcher-internal-YYYY-MM-DD.jsonl`, even if all configured sinks
  are misconfigured.
- New operators with no `observability-config.toml` have immediate visibility into
  dispatcher activity from the first hook invocation.
- `dispatcher.started`, `plugin.loaded`, `plugin.invoked`, `plugin.completed`,
  `plugin.timeout`, `plugin.crashed`, `internal.sink_error`, and
  `internal.capability_denied` are all auditable without any configuration.
- 30-day retention with daily rotation prevents unbounded disk growth.

### Negative / Trade-offs

- `dispatcher-internal.jsonl` cannot be disabled. Operators who want zero disk
  footprint must accept this one file. (The design doc acknowledges this explicitly
  — it is a deliberate trade-off against debuggability, not an oversight.)
- Best-effort writes mean the self-diagnostic log is not a reliable audit trail
  for security or compliance purposes. It is a debugging aid, not a guarantee.
- The file is in `.factory/logs/`, which is not gitignored by default in all
  projects. Operators must ensure their `.gitignore` excludes this path.

### Status as of v1.0.0-beta.5

IN-EFFECT. `crates/factory-dispatcher/src/internal_log.rs` implements the always-on
log with daily rotation, 30-day retention, and best-effort writes. The file
`dispatcher-internal-YYYY-MM-DD.jsonl` is produced on every dispatcher invocation.
`DEFAULT_RETENTION_DAYS: u32 = 30` is enforced by `InternalLog::prune_old` called
at dispatcher startup.

## Alternatives Considered

- **Configurable self-telemetry sink:** Allow operators to route `internal.*` events
  through `observability-config.toml`. Rejected: the circularity problem means a
  misconfigured sink would suppress the error event announcing its own misconfiguration.
- **Disable when no sinks configured:** If `observability-config.toml` has no enabled
  sinks, emit nothing. Rejected: this is exactly the state where diagnostic information
  is most valuable — the operator doesn't know why nothing is happening.
- **Single non-rotating file:** Write all internal events to a single
  `dispatcher-internal.jsonl` without rotation. Rejected: unbounded growth on
  high-frequency hook deployments; daily rotation bounds disk usage automatically.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 73–75 (always-on self-telemetry decision), lines 126–128 (log file layout),
  lines 393–395 (self-telemetry as last-resort when sinks are down), lines 488–490
  (Q6 resolution summary), lines 833–858 (full Q6 Open Question resolution with
  Option B rationale).
- **Code as-built:** `crates/factory-dispatcher/src/internal_log.rs:1–27` (module
  doc with contract statement), `internal_log.rs:42` (`DEFAULT_RETENTION_DAYS: u32 = 30`),
  `internal_log.rs:48` (`INTERNAL_EVENT_SCHEMA_VERSION: u32 = 1`).
