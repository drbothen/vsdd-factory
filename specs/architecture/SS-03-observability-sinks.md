---
document_type: architecture-section
level: L3
section: "SS-03-observability-sinks"
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1.2
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
traces_to: ARCH-INDEX.md
---

# SS-03: Observability Sinks

## [Section Content]

## Purpose

The Observability Sinks subsystem provides the multi-sink event fan-out pipeline
that delivers hook events from the dispatcher to operator-configured backends.
It is composed of three crates (`sink-core`, `sink-file`, `sink-otel-grpc`) plus
the dispatcher-resident `internal_log` and `sinks` modules that load, fan-out to,
and shut down sink instances.

The design separates two observability planes. The **always-on plane** is
`dispatcher-internal-YYYY-MM-DD.jsonl`, written by `internal_log.rs` regardless
of any external sink configuration (ADR-007, Q6 Option B). This plane cannot be
disabled and ensures operators always have a local audit trail even if all sinks
are misconfigured or unreachable. The **configurable plane** is driven by
`observability-config.toml`: operators declare any combination of `file`,
`otel-grpc`, `http` (future), `datadog` (future), `honeycomb` (future) sinks.

Each sink is an independent driver implementing the `Sink` trait from `sink-core`.
The `SinkRegistry` in the dispatcher holds a `Vec<Box<dyn Sink>>` and fans out
every event to all enabled sinks via `submit_all`. Sinks are isolated: a failure in
one does not block others (NFR-REL-001 extended to the sink layer).

## Modules

| Module / File | Responsibility |
|---|---|
| `crates/sink-core/src/lib.rs` | `Sink` trait + `SinkEvent` (field-bag event type) + `SinkConfigCommon` + `RoutingFilter` (allow-then-deny) + `SinkError` |
| `crates/sink-file/src/lib.rs` | Default JSONL append driver; daily-rotated filename; mpsc-bounded queue (`DEFAULT_QUEUE_DEPTH = 1000`); dedicated OS thread + `current_thread` tokio runtime; expose `FileSink`, `FileSinkConfig`, `SinkFailure` |
| `crates/sink-otel-grpc/src/lib.rs` | OTLP/gRPC log forwarder; dedicated OS thread + `current_thread` runtime; batch config (`DEFAULT_BATCH_SIZE = 100`); default endpoint `http://localhost:4317`; expose `OtelGrpcSink`, `OtelGrpcConfig` |
| `crates/factory-dispatcher/src/internal_log.rs` | Always-on `dispatcher-internal-YYYY-MM-DD.jsonl`; daily rotation by event timestamp; 30-day retention via `prune_old`; 18 event-type constants; expose `InternalLog`, `InternalEvent`, `INTERNAL_EVENT_SCHEMA_VERSION = 1` |
| `crates/factory-dispatcher/src/sinks.rs` | Load `observability-config.toml`; instantiate sink drivers; `SinkRegistry::load`, `from_config`, `submit_all`, `flush_all`, `shutdown_all`; warn-and-skip on unknown driver type |
| `crates/factory-dispatcher/src/sinks/router.rs` | Per-sink routing-filter dispatch gate AND tag enrichment (wired by S-4.06) |

## Public Interface

**`sink-core` (public crate API):**
- `Sink` trait: `fn submit(&self, event: &SinkEvent) -> Result<(), SinkError>` +
  `fn flush(&self)` + `fn shutdown(self: Box<Self>)`.
- `SinkEvent`: field-bag with `event_type: String`, `timestamp: DateTime<Utc>`,
  `dispatcher_trace_id: Uuid`, `fields: HashMap<String, Value>`.
- `RoutingFilter`: allow-list + deny-list evaluated in order; empty allow = pass all.
- `SinkConfigCommon`: shared fields across all driver config blocks (`enabled`,
  `routing_filter`, `tags`, `schema_version`).

**`observability-config.toml` schema (version 1):**
```toml
schema_version = 1
[sinks.my_file]
type = "file"
enabled = true
path = ".factory/logs/events-{date}.jsonl"

[sinks.my_otel]
type = "otel-grpc"
enabled = true
endpoint = "http://localhost:4317"
batch_size = 100
```

Unknown `type` values are warned to stderr and skipped (graceful degradation).

**Internal log constants (18):** `DISPATCHER_STARTED`, `DISPATCHER_COMPLETED`,
`DISPATCHER_ERROR`, `PLUGIN_INVOKED`, `PLUGIN_COMPLETED`, `PLUGIN_TIMEOUT`,
`PLUGIN_CRASHED`, `PLUGIN_LOADED`, `PLUGIN_LOAD_FAILED`, `DISPATCHER_SHUTTING_DOWN`,
`CAPABILITY_DENIED`, `SINK_ERROR`, `SINK_QUEUE_FULL`, `SINK_CIRCUIT_OPENED`,
`SINK_CIRCUIT_CLOSED`, `EVENT_EMITTED`, `EVENT_EMIT_ERROR`, `EVENT_FILTERED`.

## Internal Structure

Two-plane architecture (pass-1-architecture.md, lines 127-135):

**Plane 1 — always-on internal log:**
`InternalLog` holds a `BufWriter<File>` opened once per invocation. Each
`InternalEvent` is built via the builder pattern (`InternalEvent::now(type_).with_*(...)`)
and serialized as a single JSONL line. `prune_old` runs at open time and removes
files older than 30 days. Daily rotation is by event timestamp (not wall clock at
open time), so events near midnight always land in the correct date file.

**Plane 2 — configurable sinks:**
`SinkRegistry` is a `Vec<Box<dyn Sink>>`. `from_config` deserializes
`observability-config.toml`, instantiates each enabled driver. `submit_all` is a
simple loop with individual error capture (failure in one sink does not propagate).
Each sink driver owns a dedicated OS thread and a `current_thread` tokio runtime
(DRIFT-003 — planned migration to shared dispatcher runtime post-1.0).

`sink-file` uses an mpsc channel with `DEFAULT_QUEUE_DEPTH = 1000` and
`try_send` (non-blocking; drops when full — failure recorded in `SinkFailure`
but not yet emitted as event; DRIFT-002). `sink-otel-grpc` batches events up to
`DEFAULT_BATCH_SIZE = 100` before flushing gRPC stream.

## Dependencies

**Incoming (consumers of SS-03):**
- SS-01 (Hook Dispatcher Core) — `internal_log.rs` and `sinks.rs` are resident
  in the dispatcher; SS-01 calls `SinkRegistry::submit_all` on every event.
- Operators — configure sinks via `observability-config.toml`; read output from
  `.factory/logs/` or configured remote endpoint.

**Outgoing (SS-03 depends on):**
- External: `opentelemetry` + `opentelemetry-otlp` + `tonic` at lockstep version
  `0.31` (NFR-MAINT-003). No internal subsystem dependencies.

## Cross-Cutting

- **Always-on guarantee:** `dispatcher-internal-YYYY-MM-DD.jsonl` is written
  before any sink is loaded; failure to write it is logged to stderr but does not
  abort the invocation.
- **Isolation:** Sink driver failures are caught per-sink; `submit_all` continues
  to remaining sinks. `SinkFailure` is accumulated in a `Mutex<Vec<SinkFailure>>`
  per sink but not yet surfaced as events (DRIFT-002 — must-fix before rc.1).
- **Schema versioning:** `schema_version = 1` in `observability-config.toml`;
  mismatch = hard error at `from_config` (NFR-MAINT-004).
- **`#[deny(missing_docs)]`:** Enforced on `sink-core`, `sink-file`,
  `sink-otel-grpc`. Not enforced on the dispatcher-resident `internal_log` and
  `sinks` modules (covered by SS-01's missing-docs gap, L-P1-002).
- **Trace correlation:** All configurable-plane events carry `dispatcher_trace_id`
  from the parent invocation (passed through `SinkEvent`).

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-03/`
(target prefix BC-3; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: always-on internal log invariants (BC-3.001–BC-3.015),
daily rotation and 30-day retention (BC-3.016–BC-3.020), configurable sink load
and graceful degradation (BC-3.021–BC-3.030), `sink-file` queue and rotation
(BC-3.031–BC-3.038), `sink-otel-grpc` batch and endpoint (BC-3.039–BC-3.045),
`RoutingFilter` allow-then-deny semantics (BC-3.046–BC-3.050).

## ADRs

- ADR-005: Multi-sink observability natively in dispatcher — `decisions/ADR-005-multi-sink-observability.md`
- ADR-007: Always-on dispatcher self-telemetry — `decisions/ADR-007-always-on-telemetry.md`

## Drift / Known Issues

- **DRIFT-002 (P1 — medium):** `internal.sink_*` event constants (`SINK_ERROR`,
  `SINK_QUEUE_FULL`, `SINK_CIRCUIT_OPENED`, `SINK_CIRCUIT_CLOSED`) declared in
  `internal_log.rs:67-70` but never emitted. `SinkFailure` is recorded per-sink
  but never converted to events. Operators have no audit trail of sink degradation.
  Must-fix before rc.1 (S-4.4).
- **DRIFT-003 (P2 — low):** `sink-file` and `sink-otel-grpc` each own a dedicated
  OS thread + `current_thread` tokio runtime. S-1.6 shipped the shared dispatcher
  runtime but the swap has not been made. Planned post-1.0.
- **DRIFT-005 (P2 — low):** `sink-http`, `sink-datadog`, `sink-honeycomb` drivers
  declared in design but not implemented. Unknown driver type currently warns and
  skips. Planned for rc.1 (Tier E: S-4.1, S-4.2, S-4.3).
- **S-4.4 NOT SHIPPED:** Per-sink retry + circuit breaker. Constants declared,
  logic absent. Pending Tier E.
- **S-4.5 NOT SHIPPED:** Dead-letter queue. Design has `[sinks.*.dead_letter]`
  block; no implementation.
