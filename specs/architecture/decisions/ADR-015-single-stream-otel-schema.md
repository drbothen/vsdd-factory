---
document_type: adr
adr_id: ADR-015
status: proposed
date: 2026-05-04
subsystems_affected: [SS-01, SS-03]
supersedes: [ADR-005]
superseded_by: null
amends: [ADR-007]
---

# ADR-015: Single-Stream Event Emission with OTel-Aligned Schema

Supersedes ADR-005. Amends ADR-007.

## Context

### The integration gap that was never closed

ADR-005 (accepted 2026-04-26) designed a multi-sink fan-out architecture with a
`Router`/`SinkRegistry` dispatch layer. The component crates shipped:
`sink-core`, `sink-file`, `sink-otel-grpc`, `Router`, `RoutingFilter`, and
`SinkRegistry` all exist and compile. The wiring never landed.
`crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15 contain the open
integration TODO stating that `main.rs` wiring is "tracked separately." Story
S-3.04 (AC-001) was marked shipped in v1.0.0-beta.4 without this wire having
been made.

The practical consequence: every event emitted by a WASM-native plugin via
`host::emit_event` lands in `dispatcher-internal-YYYY-MM-DD.jsonl` alongside
`dispatcher.started` / `plugin.invoked` lifecycle noise. Every downstream
consumer (Grafana dashboards, `factory-query`, `factory-replay`, `factory-sla`,
`factory-report`, `factory-dashboard`) reads `events-YYYY-MM-DD.jsonl`, which
contains none of these events. Commit 818fb95 (2026-05-03) deleted the
bash-side `bin/emit-event` that had previously written to `events-*.jsonl`
directly, fully exposing the gap.

### Why the two-plane design did not ship

The Router/SinkRegistry architecture added substantial complexity: a
configurable fan-out layer, per-sink bounded queues, worker threads per sink,
dead-letter files, retry/circuit-breaker logic, and the
`observability-config.toml` schema. This complexity accumulated over six weeks
without producing the one behavioral outcome that matters: plugin-emitted events
appearing in `events-*.jsonl`.

The barrier was not technical difficulty in any single piece. It was the number
of independently-testable components that all had to be integrated in sequence
before any end-to-end signal reached the consumers. The complexity exceeded the
incremental value at each integration step.

### Industry consensus contradicts the two-plane design

External research confirms: OTel Logs Data Model, CloudEvents, Kubernetes Events,
Backstage EventsService, Tekton, Spin, and Cloudflare Workers all use a single
logical event stream with consumer-side filtering. Producer-side fanout into
physically separate streams by event family is a pattern that predates structured
observability tooling. Modern observability backends (Grafana, Datadog, Honeycomb)
discriminate events by attribute, not by file path.

### Field schema is critically incomplete

Audit of current emitted fields revealed the following gaps:

**Identification fields absent entirely:**
- `service.namespace` / project name — multi-project users cannot drill down
- `worktree.id` — concurrent worktrees of the same repo collapse in dashboards
- `service.instance.id` — concurrent dispatcher processes on one machine are
  indistinguishable

**Identity fields present but wrong:**
- `plugin_version` is always the dispatcher's version (`env!("CARGO_PKG_VERSION")`
  at `main.rs:143`), not the plugin's actual version. Dashboard slicing by
  plugin_version is meaningless.

**Causal chain fields absent:**
- No `trace_id`, `span_id`, or `parent_span_id` propagation across
  `exec_subprocess` boundaries.

**Naming inconsistencies causing silent dashboard failures:**
- `pr.opened` queried by Grafana but `pr.created` emitted by plugin — panel
  shows zero forever.
- `open_to_merge_seconds` queried but never emitted.

**Structural gaps:**
- `schema_version = 1` stamped on every event but never inspected by any
  consumer. Schema drift is silent.
- `HookResult::Block` plugins emit no event — block path has no audit trail.
- Bash hooks via `bin/emit-event` carry no `dispatcher_trace_id`,
  `plugin_name`, or `plugin_version`. Cross-side joins lose every bash event.
- `event.category` absent — consumers must grep event name prefixes to
  discriminate lifecycle from domain from audit events.

## Decision

### D-15.1 — Single physical stream for all events

All events — WASM-plugin-emitted domain events AND dispatcher lifecycle events —
are written to one physical file: `.factory/logs/events-YYYY-MM-DD.jsonl`.

The `dispatcher-internal-YYYY-MM-DD.jsonl` file (ADR-007) is repurposed as an
opt-in debug stream. It is gated by the `VSDD_DEBUG_LOG=1` environment variable
and is off by default in release builds. ADR-007's "always-on" guarantee is
amended: the debug file is active only when `VSDD_DEBUG_LOG=1` is set.

**FileSink write-failure semantics (resolves OQ-4):** Because `FileSink` is the
sole writer after D-15.1, the DLQ fallback model from the multi-sink era no
longer applies. When `FileSink::write` returns an error (disk full, permissions
failure, or any I/O error), the dispatcher MUST:

1. Attempt to write the failed event to `dispatcher-internal-YYYY-MM-DD.jsonl`
   unconditionally — regardless of whether `VSDD_DEBUG_LOG=1` is set. This
   preserves the last-resort debuggability intent of ADR-007.
2. Emit a `stderr` warning of the form:
   `[vsdd-dispatcher] WARN: FileSink write failed for events-YYYY-MM-DD.jsonl
   (<error>); event written to dispatcher-internal-YYYY-MM-DD.jsonl as fallback.`
3. NOT silently swallow the failure. A disk-full condition must be visible to
   the operator in the process's stderr stream.

This replaces the old "all sinks failed" clause (which was incoherent after
`Router`/`SinkRegistry` retirement) with a single, unambiguous rule: primary
sink failure triggers unconditional fallback to the debug file plus a stderr
warning. The `DlqWriter` in `sink-core` is retired; the debug file IS the DLQ.

**Implementation path:** `host::emit_event` in `main.rs` calls `FileSink::write`
directly on the single events file. The `sink-core` trait and `sink-file` driver
are KEPT — `FileSink` becomes the direct writer for `events-*.jsonl`. The
`Router`, `SinkRegistry`, and `sink-otel-grpc` crates are retired.

**Retirement semantics:** Retired crates (`Router`, `SinkRegistry`,
`sink-otel-grpc`, `DlqWriter` from `sink-core`) remain in the workspace through
Wave 5 of the migration. They are excluded from `default-members` in the root
`Cargo.toml` and marked `publish = false` immediately in Wave 1. They are NOT
called from any production code path after Wave 1. Physical removal from the
repository happens only in Wave 5. This policy preserves `git bisect` / rollback
options and avoids forcing a workspace restructure mid-migration. "Retired" means
excluded and uncalled, not deleted.

**Consumer fan-out:** Downstream OTel collector receives all events via its
existing filelog receiver pointed at `events-*.jsonl`. Consumers that need only
a subset of events apply consumer-side filters on `event.category` or
`event.name` prefix. This is the same architecture used by Loki, Grafana Tempo,
and the OTel Collector routing connector.

**`observability-config.toml`:** Retained for configuring the file sink path
template, retention policy, and the `VSDD_DEBUG_LOG` gate. The multi-sink
stanza model is removed. Operators who need remote export configure the OTel
Collector as the second hop, not the dispatcher.

### D-15.2 — OTel-aligned schema with explicit enrichment contract

Every event carries OTel-canonical fields. The host stamps two categories of
fields; plugins stamp a third.

**Resource attributes** (per dispatcher process, stable across all events in
one invocation):

| Field | Value |
|-------|-------|
| `service.name` | `"vsdd-factory"` |
| `service.namespace` | basename of `CLAUDE_PROJECT_DIR` |
| `service.instance.id` | UUIDv4 generated at dispatcher startup |
| `service.version` | `env!("CARGO_PKG_VERSION")` |
| `deployment.environment.name` | `"ci"` if `CI=true` else `"local-dev"` |
| `host.name` | `gethostname()` |
| `host.id` | machine-stable ID (macOS IOKit serial or `/etc/machine-id`) |
| `os.type` | `"macos"` \| `"linux"` \| `"windows"` |
| `process.pid` | `std::process::id()` |
| `vcs.repository.url.full` | canonical remote URL (clash-resistant) |
| `vcs.repository.name` | repo basename |
| `vcs.provider.name` | `"github"` \| `"gitlab"` \| `"other"` |
| `vcs.owner.name` | org or user from remote URL |
| `worktree.id` | absolute worktree path hash (SHA-256, hex prefix 12 chars) |
| `schema_url` | `"https://vsdd-factory.dev/schemas/events/v2"` |

**Per-event attributes** (host-stamped at emit time; plugin cannot override):

| Field | Value |
|-------|-------|
| `timestamp` | RFC 3339 nanosecond-precision UTC |
| `observed_timestamp` | same as timestamp (local machine) |
| `event.name` | reverse-DNS, type-versioned: `"vsdd.commit.made.v1"` |
| `event.id` | UUIDv4 per emission (idempotency key) |
| `event.category` | `lifecycle` \| `domain` \| `audit` \| `error` |
| `event.source` | `"dispatcher"` or `"plugin:<plugin_name>"` |
| `severity_number` | OTel severity integer (9=INFO, 13=WARN, 17=ERROR) |
| `severity_text` | `"INFO"` \| `"WARN"` \| `"ERROR"` |
| `trace_id` | inherited from `VSDD_TRACE_ID` env if set; else per-invocation UUID |
| `span_id` | per-plugin-invocation UUID |
| `parent_span_id` | dispatcher's span for this invocation |
| `session.id` | from Claude envelope |
| `session.previous_id` | from Claude envelope (for session chain) |
| `project.id` | SHA-256 of `vcs.repository.url.full` |
| `project.path` | `CLAUDE_PROJECT_DIR` |
| `project.name` | basename of `CLAUDE_PROJECT_DIR` |
| `vcs.ref.head.name` | branch name |
| `vcs.ref.head.revision` | current commit SHA |
| `vcs.ref.head.type` | `"branch"` \| `"tag"` \| `"detached"` |
| `hook.tool_name` | auto-derived from Claude envelope |
| `hook.event_name` | auto-derived from Claude envelope |
| `plugin.name` | plugin identifier from hooks-registry.toml |
| `plugin.version` | the plugin's own Cargo package version (NOT dispatcher's) |
| `plugin.invocation_id` | UUIDv4 per plugin invocation |
| `outcome` | canonical enum: `success` \| `failure` \| `error` \| `timeout` \| `skipped` \| `blocked` |

**Plugin-asserted domain fields** (plugin declares; host does not override):

Plugins assert only `event.name` plus event-family domain fields
(`commit.*`, `pr.*`, `session.*`, `agent.*`, `worktree.*`, `hook.*`,
`tool.*`). Domain field schemas follow OTel semantic conventions and
CloudEvents semconv where applicable.

**`event.category` taxonomy registry** (centralized in dispatcher):

The host derives `event.category` from a static registry table mapping
`event.name` prefix to category. Plugin authors do not set this field.

**Registry ownership decision (D-15.2.a):** The registry is maintained in
dispatcher source code (compile-time stable). Operator-extensible config-file
registration (e.g., `event-category-registry.toml`) is explicitly deferred to
a future ADR. Rationale: compile-time stability prevents operator misconfiguration
from silently mis-routing security or audit events; new vsdd-factory event families
require a dispatcher release by design, since they must also update field schemas
and SDK documentation. The maintenance coupling is accepted. Plugin authors who
need a new prefix must file a dispatcher PR; this ensures the registry remains
the authoritative and auditable source.

**Unrecognized prefix default (D-15.2.b):** An `event.name` whose prefix is NOT
in the registry resolves to `event.category = "unknown"` — not `"domain"`. This
allows dashboards to alert on uncategorized events rather than silently absorbing
them into domain aggregates. Security and audit events emitted under an
unregistered prefix will surface in an `unknown` category alert, not pollute
domain dashboards.

| Prefix | category |
|--------|----------|
| `vsdd.dispatcher.*` | `lifecycle` |
| `vsdd.plugin.*` | `lifecycle` |
| `vsdd.commit.*` | `domain` |
| `vsdd.pr.*` | `domain` |
| `vsdd.session.*` | `domain` |
| `vsdd.agent.*` | `domain` |
| `vsdd.worktree.*` | `domain` |
| `vsdd.hook.*` | `domain` |
| `vsdd.tool.*` | `domain` |
| `vsdd.block.*` | `audit` |
| `vsdd.capability.denied.*` | `audit` |
| `vsdd.error.*` | `error` |
| `vsdd.internal.*` | `lifecycle` |
| *(unrecognized prefix)* | `unknown` |

**Audit category integrity note:** The `audit` category provides event
classification, not tamper-evidence. Events in the `audit` category within
`events-*.jsonl` identify that actions occurred; they do not guarantee the
events have not been modified or deleted after the fact. A tamper-evident audit
layer (WORM file, hash-chained append, or similar) is future work. This is an
accepted limitation for vsdd-factory's current threat model.

### D-15.3 — Producer-side enrichment contract

**Host enriches; plugin asserts:**
- The host stamps all Resource attributes at startup.
- The host stamps all per-event identity, causal, and tool-context fields at
  `emit_event` time, before the plugin's domain fields are merged.
- Plugin-supplied values for Resource or host-stamped fields are silently
  dropped. The host-stamped value wins unconditionally.

**Block path audit trail:**
- When a plugin returns `HookResult::Block`, the dispatcher emits a
  `vsdd.block.plugin_blocked.v1` event with `outcome=blocked`,
  `plugin.name`, and `hook.tool_name` before exiting. Block path now has an
  audit trail.

**Bash hook parity (legacy-bash-adapter):**
- `bin/emit-event` is enhanced to add all Resource fields and host-stamped
  per-event fields before writing to `events-*.jsonl`. Alternatively, bash
  hooks that emit events are routed through the dispatcher's
  `host::emit_event` path (preferred for the native WASM port track).
- Until `bin/emit-event` is enhanced, bash-sourced events carry a
  `event.source = "bash-adapter"` marker so dashboards can identify thin events.

**Schema versioning:**
- Breaking schema changes bump both the `schema_url` URI version (`/v2` →
  `/v3`) AND the `event.name` suffix (`.v1` → `.v2`).
- Consumers route by `event.name` suffix; old consumers remain compatible with
  old event names during a migration window.
- `schema_version` field in JSONL lines is removed; `schema_url` in each
  Resource object is the authoritative version signal.

## Rationale

### Why single stream is correct for this scale

At vsdd-factory's event volume (tens to hundreds of events per Claude Code
session, burst peaks of 1–5k events/day on active multi-project setups),
file I/O is not the bottleneck. A single append-only JSONL file handles
10k events/minute without measurable overhead. The OTel Collector's filelog
receiver handles fan-out, filtering, batching, and remote export in a
separate process, which is the correct architectural separation: the dispatcher
stays fast and simple; the collector handles reliability.

At the theoretical scale where single-stream I/O becomes a bottleneck (tens of
millions of events/day), the right architectural move is to emit directly to a
local OTel Collector over loopback UDP/gRPC — not to add per-family files.
That migration is additive: replace `FileSink` with an `OtlpSink` pointing to
localhost. The event schema developed in D-15.2 is already OTel-native, making
this a configuration change, not a schema migration.

### Why Router/SinkRegistry complexity is not worth recovering

The six-week integration gap is empirical evidence that the two-plane design's
complexity exceeded its incremental value. The Router/SinkRegistry produced
correct component-level tests but no end-to-end signal because each integration
step required the next one to be complete before any output was observable.
Direct `FileSink` write in `host::emit_event` produces observable output from
the first line of integration code. Operational simplicity has compounding value
for a tool that runs on every Claude Code tool invocation.

### Why sink-otel-grpc is retired, not kept

`sink-otel-grpc` duplicates the OTel Collector's log export functionality. The
OTel Collector is a mature, maintained, battle-tested project that handles
retries, circuit breaking, batching, header injection, TLS, and routing. The
dispatcher's `sink-otel-grpc` crate, by contrast, is a thin gRPC wrapper with
no retry logic and no tests for failure modes. Keeping both in parallel means
two implementations of the same function with divergent maintenance burden.
Operators who want OTel export run the Collector (already a first-class option
in vsdd-factory's `factory-obs` stack) and point it at `events-*.jsonl`.

The failure mode for OTel-Collector-based export: if the collector is
misconfigured or down, events still land in `events-*.jsonl`. Local dashboards
work. Remote export fails visibly (collector logs errors). This is strictly
better than the prior sink-otel-grpc failure mode, where a failed gRPC
connection silently dropped events with no local fallback.

## Consequences

### Positive

- **Dashboard parity with reality, immediately.** `events-*.jsonl` will contain
  every plugin-emitted event from the first story that implements D-15.1.
- **Project and worktree drill-down.** `service.namespace`, `worktree.id`, and
  `project.id` make multi-project dashboards possible.
- **Causal chain.** `trace_id` / `span_id` / `parent_span_id` propagation
  enables root-cause queries across plugin boundaries.
- **OTel-native schema.** Switching the export backend from Grafana to Datadog
  or Honeycomb is a collector configuration change, not a schema migration.
- **Audit trail for blocks.** `vsdd.block.plugin_blocked.v1` closes the silent
  block-path gap.
- **Accurate plugin version.** Dashboards slicing by `plugin.version` now
  reflect the plugin's actual released version.
- **Consumer-side filters are simpler.** Consumers filter on `event.category`
  to separate domain events from lifecycle noise, rather than reading different
  files.

### Negative / Trade-offs

- **Every existing Grafana query needs updating.** Queries using the old field
  names (`pr.opened` → `pr.created`, etc.) and old dashboard patterns (reading
  only `events-*.jsonl` with no category filter) must be rewritten.
- **Every plugin's `emit_event` calls need updating.** Plugins that currently
  stamp Resource-level fields themselves will see those fields silently dropped
  and replaced by host-stamped values. Plugins that stamp incorrect
  `event.name` prefixes will get the wrong `event.category`.
- **`bin/emit-event` parity work.** Until bash hooks are ported to native WASM,
  `bin/emit-event` must be enhanced to add the full Resource + per-event field
  set. Thin bash events remain identifiable via `event.source = "bash-adapter"`.
- **ADR-005 subsystem contracts are superseded.** SS-03 must be rewritten to
  reflect the simplified sink model. BC-3.* contracts covering multi-sink
  fan-out, DLQ, and sink health events are withdrawn or revised.
- **`sink-otel-grpc` crate retires.** Any operator who configured `type =
  "otel-grpc"` in `observability-config.toml` must migrate to an OTel Collector
  filelog receiver pointed at `events-*.jsonl`.
- **Lifecycle events now appear in `events-*.jsonl`.** Dashboard authors must
  add `event.category = "domain"` filters where they previously assumed the
  file contained only domain events. (Previously they assumed this incorrectly;
  now the assumption must be made explicit.)
- **ADR-007 always-on guarantee is weakened.** The debug file is no longer
  always-on in release builds. Operators who relied on `dispatcher-internal-*.jsonl`
  being present without configuration must set `VSDD_DEBUG_LOG=1`.

### Status

PROPOSED. Not yet in effect. Implementation begins with the migration plan below.

## Alternatives Considered

### Option A: Producer-side dual-write (both files, filter by type prefix)

Write every event to both `events-*.jsonl` and `dispatcher-internal-*.jsonl`,
filtering by event name prefix at write time to determine which file(s) receive
the event. Domain events go to `events-*.jsonl`; lifecycle events go to both.

Rejected for three reasons. First, partial-write hazard: a crash between the
two writes leaves the files inconsistent with no recovery path. Second, two
independent retention timelines diverge: events-*.jsonl has a 90-day retention
default; dispatcher-internal-*.jsonl has a 30-day retention. An event's
presence in one file does not guarantee its presence in the other. Third, every
cited industry reference — OTel, CloudEvents, K8s Events — uses single stream
with attribute discrimination, not multi-stream with producer-side fanout. Dual
write is a pattern that emerges when organizations retrofit structure onto an
existing unstructured log system. Building it in from the start is unnecessary.

### Option B: Wire the original Router/SinkRegistry as designed (ADR-005)

Complete the integration TODO in `sinks/mod.rs` lines 11–15 by wiring
`Router::submit` from `main.rs` after each plugin invocation.

Rejected. Six weeks of "almost done" is empirical evidence that the design's
incremental integration complexity exceeded its value. The Router/SinkRegistry
provides configurable fan-out, per-sink queues, circuit breaking, and DLQ
semantics — capabilities that are also provided by the OTel Collector, which
is already part of the vsdd-factory observability stack, is battle-tested, and
requires zero dispatcher maintenance. Wiring the Router would permanently add
complexity to the dispatcher's hot path (every plugin invocation) to replicate
functionality already available outside the dispatcher.

### Option C: Two host imports (`emit_lifecycle` vs `emit_domain`)

Expose two host functions in the plugin SDK: `host::emit_lifecycle` for
dispatcher-internal events, and `host::emit_event` for plugin domain events.
Route each to its respective file.

Rejected. This has the same partial-write risks as Option A. It doubles the
host-import surface complexity. It does not solve the field schema gaps
identified in the Context section. Plugins that accidentally call the wrong
import would silently route events to the wrong file with no observable error.

## Migration Plan

Stories are ordered to minimize dashboard downtime. Each story should be a
distinct wave-scope item.

**Wave 0 (prerequisite): Document the current broken state**
- Audit every Grafana panel query and record the exact field name assumed.
- Snapshot the current `events-*.jsonl` field inventory.
- This is read-only; no code changes.

**Wave 1: Implement host-side enrichment + single-stream write**
- Modify `main.rs`: at `emit_event` call site, stamp Resource attributes and
  per-event fields before delegating to `FileSink::write`.
- Retire `Router`, `SinkRegistry`, `sink-otel-grpc` from the integration path
  (leave crates on disk until post-migration cleanup; do not call them).
- Gate `dispatcher-internal-*.jsonl` writes on `VSDD_DEBUG_LOG=1`.
- All plugins now emit to `events-*.jsonl`. Field values are enriched but
  schema may not yet match D-15.2 fully (plugin field names are in-flight).

**Wave 2: Plugin schema migration**
- Update each native WASM plugin's `emit_event` call to assert only
  `event.name` + domain fields. Remove any plugin-side Resource field stamps.
- Fix all `event.name` values to reverse-DNS format with `.v1` suffix.
- Update `pr.created` event name (was `pr.opened` in plugin, `pr.opened` in
  Grafana — both need fixing but in coordination).

**Wave 3: Grafana dashboard rewrite**
- Rewrite all queries against new field names.
- Add `event.category = "domain"` filters where needed.
- Validate `pr.created`, `open_to_merge_seconds`, and other broken queries.

**Wave 4: Bash hook parity**
- Enhance `bin/emit-event` to add Resource + per-event fields.
- Mark bash-sourced events with `event.source = "bash-adapter"` until ported.

**Wave 5: SS-03 spec update + crate cleanup**
- Retire `sink-otel-grpc` crate.
- Rewrite SS-03-observability-sinks.md to reflect the simplified model.
- Withdraw or revise BC-3.* contracts that described multi-sink fan-out semantics.

## Adversarial Pressure Points

### Scale: What if volume grows to 10k events/min?

At 10k events/min, each event averaging 2 KB, throughput is ~330 KB/s. A
modern NVMe drive sustains 500+ MB/s sequential write. File I/O is not a
bottleneck at this scale. Daily file rotation bounds file size at ~475 MB/day,
within normal log management range.

If volume grows to the range where append-file I/O is genuinely problematic
(hundreds of millions of events/day), the correct migration is to replace
`FileSink` with an `OtlpSink` that writes to a local OTel Collector over
loopback. The D-15.2 schema is already OTel-native. This migration requires
no consumer changes — the Collector can write `events-*.jsonl` as it does today
while simultaneously exporting to a high-throughput backend.

### Tamper-evident audit log for compliance

Single-stream does not prevent a tamper-evident audit log; it defers the
mechanism to a separate layer. Two options are compatible with D-15.1:

1. The OTel Collector's `fileexporter` can write a separate audit stream
   filtered on `event.category = "audit"` with append-only POSIX permissions.
2. A future `audit-sink` crate can be wired directly at the `FileSink` write
   point for events where `event.category = "audit"`, writing to a write-once
   (or WORM-equivalent) file with a separate retention policy.

Neither option is required for vsdd-factory's current threat model. The ADR
does not preclude adding either.

### Guaranteed delivery for a specific consumer (WAL semantics)

The OTel Collector's `fileexporter` with `max_backoff` and `retry_on_failure`
provides durable buffering to disk before export. For a true WAL-semantics
requirement (no event loss, even if the collector crashes mid-export), the OTel
Collector's `fileexporter` writes to its own journal before acking upstream.
This is an OTel Collector configuration concern, not a dispatcher concern.

Single-stream is compatible with WAL semantics at the collector layer. The
dispatcher's write to `events-*.jsonl` is the first durable record;
the collector consumes from there with its own delivery guarantees.

For requirements that demand sub-millisecond guarantee at the dispatcher level
(e.g., financial compliance), the dispatcher `emit_event` write to `FileSink`
must use `fsync` after each write. This is a performance trade-off
(`fsync` adds 0.5–5ms per write on spinning disk; negligible on NVMe). It
is not architecturally incompatible with D-15.1 — it is a `FileSink` config
option.

### Why retire sink-otel-grpc? What is the OTel-Collector-equivalent failure mode?

`sink-otel-grpc` failure mode: the gRPC connection to the OTel endpoint fails.
Events are queued in the sink's bounded in-memory queue. If the queue fills
(bounded), subsequent events are silently dropped (with a DLQ file as fallback).
The dispatcher's hot path is unaffected but remote observability is lost
without an operator alert.

OTel Collector filelog receiver failure mode: the Collector process is down. The
`events-*.jsonl` file accumulates events. When the Collector restarts, it reads
from its last checkpoint (if configured) and catches up. No events are lost. The
Collector's health is observable via its own `/metrics` endpoint.

The Collector failure mode is strictly better: events survive Collector downtime
with automatic catch-up. The `sink-otel-grpc` failure mode requires the
dispatcher to be alive and connected at the moment of each event. The Collector
decouples the two concerns.

### Upgrade path: atomic cutover or migration window?

Migration window, not atomic cutover. During Wave 1, `events-*.jsonl` begins
receiving WASM-plugin events for the first time. Grafana queries that were
already broken (because events-*.jsonl was empty) start receiving data, but
with old field names. This is a strict improvement; no panel gets worse.

Wave 2 (plugin schema migration) changes field names. Grafana queries that
target old field names will stop matching. Wave 2 and Wave 3 must ship in the
same release to avoid dashboard regression. Coordination is achieved by
completing Wave 2 plugin changes and Wave 3 query rewrites in the same story
batch.

Bash events (Wave 4) continue using the thin format until `bin/emit-event` is
updated. `event.source = "bash-adapter"` allows dashboards to identify and
optionally exclude thin events during the migration window.

There is no "flag day" cutover. Each wave is independently observable and
reversible (WASM plugins can be rolled back independently; Grafana queries can
be reverted). The migration is safe to run incrementally across multiple sprints.

### Plugin authors targeting v1.0: how to manage the breaking ABI change?

The `emit_event` host function signature does not change. The breaking change is
in the field contract: previously, plugin-stamped fields were passed through
to the event. After D-15.1, host-stamped fields silently win.

For plugin authors:
1. Resource-level fields (`service.name`, `service.namespace`, etc.) stamped
   by plugins will be silently dropped. Most existing plugins do not stamp
   these fields. No action required for those plugins.
2. `plugin_version` stamps in existing plugins are wrong (they stamp the
   dispatcher version from the environment). The host now stamps the correct
   plugin version. Existing plugins that stamp a wrong `plugin_version` will
   have that value dropped. This is a correction, not a regression.
3. `event.name` values that do not conform to the reverse-DNS + `.vN` format
   will still be written, but `event.category` will default to `"domain"` for
   unrecognized prefixes. Plugins should migrate their event names in Wave 2.

The SDK changelog for the wave containing D-15.2 must document:
- Fields the host now stamps (plugin stamps are dropped)
- Required `event.name` format for correct `event.category` derivation
- Canonical `outcome` enum values

`HOST_ABI_VERSION` stays at 1. The `emit_event` signature is unchanged.
Breaking changes are in the behavioral contract of which fields the host stamps,
not in the ABI. This follows the precedent of ADR-006 (additive-only ABI
extensions do not bump `HOST_ABI_VERSION`).

## Open Questions (escalate to SS-XX-level decisions)

**OQ-1 (SS-03): `observability-config.toml` schema for D-15.1**
What is the minimal `observability-config.toml` schema post-migration? The
multi-sink stanza model is removed. What configuration does the file retain?
(File sink path template, retention days, `VSDD_DEBUG_LOG` default, OTel
Collector endpoint for operators who skip the Collector entirely?) The SS-03
spec rewrite must answer this.

**OQ-2 (SS-01): `host::emit_event` enrichment implementation scope for Wave 1**
Which of the D-15.2 Resource fields can be reliably computed at dispatcher
startup on all three platforms (macOS, Linux, Windows)? `host.id` on Windows
has no `/etc/machine-id` equivalent; `vcs.repository.url.full` requires a
`git remote get-url` call that may fail in detached worktrees. The Wave 1
implementation story must define the fallback values for each field when
computation fails.

**OQ-3 (SS-01): VSDD_TRACE_ID propagation via `exec_subprocess`**
D-15.2 specifies `trace_id` propagation via the `VSDD_TRACE_ID` environment
variable across `exec_subprocess` boundaries. The `exec_subprocess` capability
currently strips env vars not in `env_allowlist`. Who adds `VSDD_TRACE_ID` to
the universal `env_allowlist`, and how is the `span_id` / `parent_span_id`
chain maintained across subprocess hops? This needs a BC in SS-01.

**OQ-4 (SS-03): DLQ semantics after sink-otel-grpc retirement**
The `DlqWriter` in `sink-core` was designed for the multi-sink model. After
D-15.1, `FileSink` is the only writer. If `FileSink::write` fails (disk full,
permissions error), where does the DLQ event go? The current DLQ writes to
another file, which fails the same way. The SS-03 spec must define the failure
mode for `FileSink` write errors post-migration (options: swallow and emit to
stderr; write to a tmpfs-backed emergency file; write to the debug stream if
`VSDD_DEBUG_LOG=1` is set).

**OQ-5 (SS-03): Grafana dashboard migration scope and ownership**
The Wave 3 migration touches every Grafana panel query. Who authors the
migration? Are the dashboard JSON files stored in `.factory/`? Is there a
dashboard-as-code workflow that makes the migration auditable? This must be
answered before Wave 3 is scheduled.

**OQ-6 (SS-02): Plugin SDK changelog and migration guidance scope**
The SDK changelog for the D-15.2 wave must cover which fields the host stamps,
required event name format, and the `outcome` enum. Is this a minor release
of the SDK (semver patch) or a major release (semver major given the silent
field-drop behavior)? The SDK versioning policy does not currently address this
case.

## Architect Notes (for adversarial review awareness)

Two points where I have reservations about the brief that should be on the
adversarial review radar:

**1. The ADR-007 amendment weakens a safety guarantee.** The brief asks to
gate `dispatcher-internal-*.jsonl` on `VSDD_DEBUG_LOG=1`. ADR-007's "always-on"
guarantee exists precisely because misconfigured operators lose visibility when
the file isn't present. This ADR amends it to "always-on when the debug env
var is set, or when all sinks have failed." The last-resort fallback preserves
the original motivation, but it introduces a new failure mode: an operator who
doesn't know about `VSDD_DEBUG_LOG` and has a failing sink may not discover
the fallback file. The adversarial review should probe whether the last-resort
fallback is observable enough (e.g., should the dispatcher print a stderr
warning when it falls back to the debug file?).

**2. The `event.category` registry table is a new centralization point.**
Moving category derivation to a host-side registry means the registry must be
updated every time a new `event.name` prefix is introduced. If a plugin uses
a prefix not in the registry, it gets `event.category = "domain"` by default.
This is a safe default but it means new event families are silently
mis-categorized until the registry is updated. The adversarial review should
probe whether the registry should be in a config file (operator-extensible)
or stay in dispatcher source (compile-time stable but requires dispatcher
release for new prefixes).

## Source / Origin

- **Integration gap:** `crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15
  (open TODO confirming Router::submit never wired from main.rs).
- **Plugin version bug:** `crates/factory-dispatcher/src/main.rs:143`
  (`plugin_version = env!("CARGO_PKG_VERSION")` stamps dispatcher version).
- **Commit exposing the gap:** 818fb95 (2026-05-03) — deleted bash `bin/emit-event`
  bash duplicates; WASM events exclusively in dispatcher-internal-*.jsonl thereafter.
- **Industry consensus references:** OTel Logs Data Model, CloudEvents 1.0 spec,
  Kubernetes Events API, Backstage EventsService, Tekton PipelineRun events,
  Spin (Fermyon), Cloudflare Workers Analytics Engine — all single-stream with
  attribute discrimination.
- **Prior ADRs superseded/amended:** ADR-005 (multi-sink), ADR-007 (always-on
  self-telemetry).

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial draft. D-15.1 single-stream, D-15.2 OTel schema, D-15.3 enrichment contract. |
