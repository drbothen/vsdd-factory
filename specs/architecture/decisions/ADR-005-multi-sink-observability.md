---
document_type: adr
adr_id: ADR-005
status: superseded
date: 2026-04-26
subsystems_affected: [SS-01, SS-03]
supersedes: null
superseded_by: ADR-015
superseded_date: 2026-05-04
---

> **SUPERSEDED.** This ADR was superseded by [ADR-015](./ADR-015-single-stream-otel-schema.md) on 2026-05-04. The two-plane Router/SinkRegistry architecture described here was never fully wired (the integration step at `crates/factory-dispatcher/src/sinks/mod.rs` remained unimplemented for ~6 weeks despite the component crates shipping). ADR-015 replaces it with a single-stream + OTel-aligned schema + producer-side enrichment model. Read ADR-015 for the current architecture; this ADR is preserved for historical context only.

# ADR-005: Multi-Sink Observability Natively in Dispatcher

## Context

Before v1.0, vsdd-factory's observability was hardcoded to a single local JSONL
file: `emit-event` always appended to `.factory/logs/events-*.jsonl`. The local
OTel collector (Grafana/Loki/Prometheus stack) tailed that file and produced
dashboards. This was functional but constrained: operators who wanted to ship
telemetry to Datadog, Honeycomb, or a remote OTel endpoint had to run a local
collector regardless. Operators who wanted zero local disk footprint had no path
at all.

Three distinct operator profiles emerged from early feedback. First, solo developers
who want the existing behavior unchanged — a local file they can tail or open in
the factory dashboard. Second, teams who route production telemetry to an external
backend (Datadog, Honeycomb) and want to bypass the collector entirely to reduce
infrastructure. Third, hybrid operators who want local files for fast debugging
AND a remote backend for long-term retention, running simultaneously.

The v1.0 dispatcher was being designed as a compiled Rust binary that already owns
the event lifecycle — every plugin's `emit_event()` call passes through the
dispatcher before it reaches any storage. This made the dispatcher the natural
integration point for a configurable fan-out model.

## Decision

The dispatcher natively owns multi-sink fan-out. Plugins call `emit_event()` as a
host function; the dispatcher enqueues each event into every enabled sink's bounded
outbound queue; sink worker threads drain their queues independently. Sink types
(`file`, `http`, `otel-grpc`, `datadog`, `honeycomb`) are compiled into the
dispatcher as Rust crates under `crates/sink-*`. Sink instances are declared in
`observability-config.toml`; operators can declare any number of instances of any
type. Failure in one sink does not propagate to others.

## Rationale

Three alternatives were evaluated against the operator profiles described above.

Option A (OTel-only) would require the local OTel collector to remain running even
for operators who only want a Datadog sink. The collector configuration duplicates
routing logic in YAML. Zero-disk mode is impossible because the collector must write
its own buffers. Rejected: adds infrastructure requirements for a use case that
doesn't need them.

Option B (file-only dispatcher, collector handles everything) preserves the current
behavior but imposes the collector on every deployment. Same zero-disk limitation.
Rejected: does not serve the remote-only operator profile.

Option C (dispatcher-owned fan-out, as implemented) makes sink instances first-class
in `observability-config.toml`. The existing `otel-grpc` sink type feeds the existing
Grafana stack with zero behavioral change for operators who already use it. New
operators add a `datadog` or `honeycomb` sink without standing up a collector. The
dispatcher's internal event queue means `emit_event()` never blocks a plugin; sink
slowness or failure is isolated to that sink's worker.

Per-sink resilience (retry with exponential backoff, circuit breaker, bounded drop
queue, dead-letter file) ensures that a failing remote backend does not cause event
loss silently — dropped events land in `dead-letter-<sink>-<date>.jsonl`. The
`internal.sink_health` event emitted every 60 seconds provides observability over
the observability layer itself.

## Consequences

### Positive

- Operators can declare zero, one, or multiple sinks of any type with no dispatcher
  recompile — configuration is the only change surface.
- Zero-disk mode is achievable: disable the `file` sink, enable only remote sinks.
  `dispatcher-internal.jsonl` still exists (ADR-007) but is the dispatcher's
  self-diagnostic log, not user-visible telemetry.
- The existing OTel Grafana stack continues to work via the `otel-grpc` sink type
  with no operator changes.
- Hybrid mode (file + remote) is first-class: both receive every event; one failing
  does not affect the other.
- Dead-letter files prevent silent data loss when a sink is persistently down.

### Negative / Trade-offs

- Adding a new sink *type* (e.g., a Splunk sink) requires a dispatcher release.
  Adding a new sink *instance* is config-only.
- The `crates/sink-*` crate structure adds compilation weight. Currently shipping:
  `sink-core`, `sink-file`, `sink-otel-grpc`.
- Operators must learn `observability-config.toml` syntax; the previous implicit
  file-only behavior was zero-configuration.

### Status as of v1.0.0-beta.5

IN-EFFECT. The multi-sink architecture is wired in `crates/factory-dispatcher/src/sinks/`
with `sink-core`, `sink-file`, and `sink-otel-grpc` crates operational. The
`observability-config.toml` schema is defined and loaded by the dispatcher at startup.
Datadog and Honeycomb sink types are specified in the design but not yet compiled into
the binary as of beta.5 (planned for rc.1 per Phase 4).

## Alternatives Considered

- **OTel-only architecture:** Keep local OTel collector; configure it to route to any
  backend. Rejected: requires running a collector even for simple setups; zero-disk mode
  impossible; config duplication between TOML and YAML.
- **File-only dispatcher, collector handles routing:** Preserves v0.79.x behavior
  exactly. Rejected: same zero-disk limitation; makes remote-only a hard architectural
  block rather than a configuration option.
- **External sidecar process per sink:** Each sink type runs as a separate process;
  dispatcher emits to a local socket. Rejected: adds process management complexity;
  startup cost per tool invocation is too high for the 1–5ms dispatcher startup budget.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 36–38 (problem statement — hardcoded local Grafana), lines 54–58 (multi-sink
  decision), lines 363–395 (failure isolation model), lines 458–473 (ADR-005 section),
  lines 672–688 (rc.1 gate criteria for sink types).
- **Code as-built:** `crates/factory-dispatcher/src/sinks/` (fan-out router),
  `crates/sink-core/`, `crates/sink-file/`, `crates/sink-otel-grpc/`.
- **Config schema:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 286–359 (`observability-config.toml` full example with resilience blocks).
