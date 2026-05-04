---
document_type: architecture-section
level: L3
section: "SS-03-event-emission"
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-05-04T00:00:00
phase: 1.2-rev
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/specs/architecture/SS-03-observability-sinks.md
supersedes: SS-03-observability-sinks.md
traces_to: ARCH-INDEX.md
---

# SS-03: Event Emission (Single-Stream OTel-Aligned)

> **Supersession notice:** This file supersedes `SS-03-observability-sinks.md`
> per ADR-015 (2026-05-04). The two-plane Router/SinkRegistry architecture
> described in the old file is RETIRED. See that file's supersession header
> for the cross-reference. The BC-3.* prefix is retained; BCs that described
> multi-sink fan-out semantics are being revised or withdrawn as Wave 5 lands.

## Purpose

The Event Emission subsystem provides the single-stream, OTel-aligned event
pipeline that delivers all events — dispatcher lifecycle events AND
plugin-emitted domain events — to one physical append file:
`.factory/logs/events-YYYY-MM-DD.jsonl`.

The design is governed by ADR-015 (single-stream OTel-aligned schema) and
supersedes the multi-sink two-plane design from ADR-005. The key design
commitments are:

- **One physical stream**: `FileSink` is the sole writer for all events. The
  old `sink-otel-grpc` crate and `Router`/`SinkRegistry` types are deprecated
  (Wave 1) and retired at Wave 5. Operators who need remote OTel export run
  the OTel Collector's `filelog` receiver pointed at `events-*.jsonl`.
- **Host enrichment, plugin assertion**: The host stamps Resource attributes
  (once per process) and per-event identity fields (once per emission). Plugin
  domain fields are merged after host fields; plugin attempts to supply
  host-owned fields are overridden and logged (ADR-015 D-15.3).
- **Debug file is opt-in**: `dispatcher-internal-YYYY-MM-DD.jsonl` is a
  debug-only stream gated by `VSDD_DEBUG_LOG=1`. It is OFF by default in
  release builds. It serves as the last-resort fallback when the primary
  `FileSink` write fails (ADR-015 D-15.1). The "always-on" guarantee from
  ADR-007 is amended.

## Modules

| Module / File | Responsibility |
|---|---|
| `crates/sink-core/src/lib.rs` | `Sink` trait + `SinkEvent` (field-bag type) + `FileSink`. `Router`, `SinkRegistry`, `DlqWriter` are deprecated in this crate and will be removed at Wave 5. `RoutingFilter` is retained for future use but not invoked in the production path. |
| `crates/sink-file/src/lib.rs` | `FileSink`: direct JSONL append writer for `events-YYYY-MM-DD.jsonl`. Daily rotation by event timestamp. Atomic-append write semantics (see FileSink Write Semantics below). |
| `crates/factory-dispatcher/src/host/emit_event.rs` | Host function exposed to WASM plugins. Stamps all Resource and per-event fields, merges plugin domain fields, delegates to `FileSink::write`. |
| `crates/factory-dispatcher/src/internal_log.rs` | Debug-only stream: `dispatcher-internal-YYYY-MM-DD.jsonl`. Active when `VSDD_DEBUG_LOG=1`. Also serves as the unconditional fallback target when `FileSink::write` fails (see FileSink Write Failure below). 30-day retention. |
| `crates/factory-dispatcher/src/main.rs` | Stamps `dispatcher.started` / `dispatcher.completed` lifecycle events directly to `FileSink`. Invokes `emit_event` host function. |

### Deprecated (Wave 1, retiring Wave 5)

| Module / File | Status |
|---|---|
| `crates/sink-otel-grpc/` | Deprecated Wave 1 (`publish = false`, not called from production path). Physical deletion at Wave 5. |
| `crates/sink-core/src/router.rs` | `Router` and `SinkRegistry` types deprecated in-crate at Wave 1. Removed at Wave 5. |
| `crates/sink-core/src/dlq.rs` | `DlqWriter` deprecated at Wave 1. Removed at Wave 5. |

## OTel-Aligned Event Schema (ADR-015 D-15.2)

### Resource Attributes (per dispatcher process; stamped once at startup)

| Field | Value Source |
|-------|-------------|
| `service.name` | `"vsdd-factory"` (constant) |
| `service.namespace` | basename of `CLAUDE_PROJECT_DIR` |
| `service.instance.id` | UUIDv4 generated at dispatcher startup |
| `service.version` | `env!("CARGO_PKG_VERSION")` |
| `deployment.environment.name` | `"ci"` if `CI=true` else `"local-dev"` |
| `host.name` | `gethostname()` |
| `host.id` | machine-stable ID; cascade: `/etc/machine-id` → macOS IOPlatformUUID → Windows `MachineGuid` registry → SHA-256 of hostname → `"unknown-host"` |
| `os.type` | `"macos"` \| `"linux"` \| `"windows"` |
| `process.pid` | `std::process::id()` |
| `vcs.repository.url.full` | `git remote get-url origin`; fallback to `file://<worktree-path>` |
| `vcs.repository.name` | derived from `vcs.repository.url.full` |
| `vcs.provider.name` | `"github"` \| `"gitlab"` \| `"other"` |
| `vcs.owner.name` | org or user from remote URL; `"unknown"` for local-only |
| `worktree.id` | SHA-256 (hex prefix 12 chars) of absolute worktree path |
| `project.id` | SHA-256 of `vcs.repository.url.full` |
| `schema_url` | `"https://vsdd-factory.dev/schemas/events/v2"` |

No Resource field may be absent or `null`. Every field must have a
deterministic value (possibly a fallback default) before the first event is
emitted. The fallback cascade for `host.id` ends at the literal string
`"unknown-host"`; reaching this terminal default triggers a
`vsdd.internal.host_id_fallback.v1` lifecycle event at startup.

### Per-Event Attributes (host-stamped at emit time; plugin cannot override)

| Field | Value |
|-------|-------|
| `timestamp` | RFC 3339 nanosecond-precision UTC |
| `observed_timestamp` | same as `timestamp` (local machine) |
| `event.name` | reverse-DNS, type-versioned: e.g. `"vsdd.commit.made.v1"` |
| `event.id` | UUIDv4 per emission (idempotency key) |
| `event.category` | `lifecycle` \| `domain` \| `audit` \| `error` \| `unknown` |
| `event.schema_url` | per-event schema URI; informational only — `event.name` `.vN` suffix is the authoritative version signal |
| `event.source` | `"dispatcher"` or `"plugin:<plugin_name>"` |
| `severity_number` | OTel integer (9=INFO, 13=WARN, 17=ERROR) |
| `severity_text` | `"INFO"` \| `"WARN"` \| `"ERROR"` |
| `trace_id` | inherited from `VSDD_TRACE_ID` env if set; else per-invocation UUID |
| `span_id` | per-plugin-invocation UUID |
| `parent_span_id` | dispatcher's span for this invocation |
| `session.id` | from Claude envelope |
| `session.previous_id` | from Claude envelope |
| `project.path` | `CLAUDE_PROJECT_DIR` |
| `project.name` | basename of `CLAUDE_PROJECT_DIR` |
| `vcs.ref.head.name` | current branch name |
| `vcs.ref.head.revision` | current commit SHA |
| `vcs.ref.head.type` | `"branch"` \| `"tag"` \| `"detached"` |
| `hook.tool_name` | from Claude envelope |
| `hook.event_name` | from Claude envelope |
| `plugin.name` | plugin identifier from hooks-registry.toml |
| `plugin.version` | plugin's own Cargo package version (NOT dispatcher's) |
| `plugin.invocation_id` | UUIDv4 per plugin invocation |
| `outcome` | `success` \| `failure` \| `error` \| `timeout` \| `skipped` \| `blocked` |

### Dual-Emit Correlation Fields (Wave 2 migration window only; plugin-asserted)

| Field | Set on | Meaning |
|-------|--------|---------|
| `event.correlation_id` | both halves of a dual-emit pair | shared UUID linking old-name and new-name emissions |
| `event.deprecated_by` | old-name emission only | `event.id` of the corresponding new-name emission |
| `event.replaces_deprecated_alias` | new-name emission only | `event.id` of the corresponding old-name emission |
| `event.host_overrides` | any event where host overrode a plugin-supplied field | array of overridden field names |

Post-Wave-3 (shim removal): all three correlation fields are absent on all
new events. Consumers that see a dangling cross-reference (the referenced
`event.id` is absent from the same trace scope) MUST treat the event as
non-paired for dedup purposes.

### Event Category Registry (compile-time; not operator-extensible in v1)

| Prefix | category |
|--------|----------|
| `vsdd.dispatcher.*` | `lifecycle` |
| `vsdd.plugin.*` | `lifecycle` |
| `vsdd.internal.*` | `lifecycle` |
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
| *(unrecognized prefix)* | `unknown` |

Unrecognized prefix resolves to `"unknown"` (not `"domain"`). Dashboard
authors MUST alert on `unknown` category events (Wave 3 acceptance criterion).
New prefixes require a dispatcher PR and compile-time registry update.

## FileSink Write Semantics

### Normal Path (ADR-015 D-15.1)

`FileSink::write` appends the serialized event JSON followed by `\n` to
`events-YYYY-MM-DD.jsonl`. Daily rotation is by event timestamp (not wall
clock at open time), matching the same convention as the debug file.

### Partial-Write Recovery (OQ-7 resolution)

JSONL is an append-only format. A crash mid-write leaves a truncated final
line that will fail JSON parsing on replay. `FileSink` uses the
**boundary-marker strategy**: the write sequence is:

1. Serialize the event to a JSON byte buffer (in memory).
2. Write the full buffer as a single `write_all` call (atomic at the kernel
   level for writes under the typical 4 KiB pipe/page boundary; the
   serialized event is expected to be well under 4 KiB in practice).
3. Write the newline `\n` as a second `write_all` call.
4. Call `flush()` (does NOT call `fsync` on the default path; see fsync note).

**Truncation recovery on read:** Consumers MUST skip the final line of any
`events-YYYY-MM-DD.jsonl` file if it fails JSON parsing. The convention is:
the last line of a live file may be truncated if the dispatcher crashed after
the JSON payload was partially written. Consumers treat a parse-error on the
final line as a non-fatal truncation artifact (not a file corruption). All
prior lines are valid JSONL.

**Why not atomic-rename?** Atomic rename (write-to-temp, `rename(2)`) would
provide atomicity at the cost of:
(a) Breaking the `filelog` receiver's checkpoint mechanism — the collector
    tracks inode offsets; rename creates a new inode, invalidating the
    checkpoint.
(b) Preventing daily rotation without a full-file rewrite.
(c) Adding a temp-file management layer that complicates directory scanning.

Boundary-marker (final-line-skip-on-error) is the established JSONL append
pattern used by logrotate, the OTel Collector filelog receiver, and Loki
Promtail. It is simpler, compatible with external consumers, and the truncation
risk is bounded to the final event in a crash scenario.

**fsync policy:** Default path does NOT fsync after each write. For operators
who require sub-crash durability guarantees, `FileSink` exposes a
`sync_on_write: bool` config option. When `true`, `fsync` is called after
each write. This is documented in `observability-config.toml` (see schema
below). Default is `false` (performance path). The risk: an OS crash after
`write_all` but before the OS page-cache flush loses the last event. This is
accepted for the default use case. Operators with compliance requirements set
`sync_on_write = true`.

### Write-Failure Cascade (ADR-015 D-15.1)

When `FileSink::write` returns an error:

1. **Unconditional fallback**: The failed event is written to
   `dispatcher-internal-YYYY-MM-DD.jsonl` regardless of the
   `VSDD_DEBUG_LOG` setting. This preserves the last-resort debuggability
   intent of ADR-007 for actual failure conditions.
2. **stderr warning**: The dispatcher emits a warning of the form:
   `[vsdd-dispatcher] WARN: FileSink write failed for events-YYYY-MM-DD.jsonl
   (<error>); event written to dispatcher-internal-YYYY-MM-DD.jsonl as fallback.`
3. **No silent swallow**: Disk-full, permission errors, and read-only FS
   conditions MUST surface in the process stderr stream.

The debug fallback write is also best-effort (same as the debug file's
existing invariant): a failure to write the fallback is logged to stderr but
does not abort the invocation or change the exit code.

## `observability-config.toml` Schema (OQ-1 resolution)

The multi-sink stanza model is removed. The v2 schema configures only the
single `FileSink` path, retention, debug-log gate, and optional sync mode.
Operators who need remote OTel export configure the OTel Collector externally.

```toml
schema_version = 2

# Path template for the primary events file.
# Supports {date} → YYYY-MM-DD, {project} → basename of CLAUDE_PROJECT_DIR.
# Default: ".factory/logs/events-{date}.jsonl"
events_file = ".factory/logs/events-{date}.jsonl"

# Retention for events-*.jsonl files (days). Default: 90.
retention_days = 90

# Retention for dispatcher-internal-*.jsonl files (days). Default: 30.
debug_log_retention_days = 30

# Gate the dispatcher-internal-*.jsonl debug stream.
# Overridden by VSDD_DEBUG_LOG=1 env var (env var takes precedence).
# Default: false.
debug_log_enabled = false

# Call fsync after each FileSink write. Increases durability at the cost
# of ~0.5–5ms per write on spinning disk; negligible on NVMe.
# Default: false.
sync_on_write = false
```

**Schema versioning:** `schema_version = 2` is required; the old v1 schema
(multi-sink stanzas) hard-errors at load time with a migration hint directing
operators to remove multi-sink blocks. Unknown top-level keys are warned and
skipped (graceful degradation, consistent with the old behavior for unknown
sink types). The `VSDD_DEBUG_LOG=1` environment variable ALWAYS overrides
`debug_log_enabled`; the env var takes precedence.

**OQ-1 scope decision:** The v2 schema deliberately omits an OTel Collector
endpoint field. Operators who want the dispatcher to push directly to an
OTel endpoint (bypassing the file → Collector path) should configure the
OTel Collector's `otlpexporter` in the Collector's own config. Adding a
`[otel_collector]` stanza back into `observability-config.toml` is deferred
to a future ADR. The rationale: the `FileSink` → Collector file-tail pattern
is strictly more reliable (events survive Collector downtime), and introducing
a direct-push option before it is needed adds maintenance surface. Track as
future-work if operators request it.

## Persistent Deprecation Registry (OQ-9)

The one-time `vsdd.internal.event_name_deprecated.v1` announcement at Wave 3
closure exists only in the rotating JSONL stream (90-day default retention).
Operators who arrive post-Wave-3 (new install, restored backup, cloned
dashboard template) have no queryable record of which event namespaces were
deprecated and when.

**OQ-9 resolution: DEFERRED to v1.1.**

Rationale for deferral:
- The primary audience for the deprecation registry is dashboard authors
  migrating Grafana queries. Wave 3's pre-shim-removal sub-task mandates an
  operator audit before shim removal; the one-time announcement covers the
  migration window.
- A persistent registry artifact (e.g., `.factory/logs/deprecations.json`
  or a dedicated `vsdd.factory/deprecations` JSONL stream) introduces an
  additional write surface, a new schema, and new consumer tooling. Adding
  this to Wave 3 increases Wave 3 scope beyond the dashboard migration work.
- The risk is bounded: post-Wave-3 dashboard silence from stale old-name
  queries is visible as zero-row panels (not corrupt data). The operator
  diagnoses by checking event names against the ADR-015 deprecation map.

**v1.1 scope (deferred):** A persistent `deprecations.jsonl` sidecar file in
`.factory/logs/`, written atomically at each Wave N shim-removal close,
containing a JSON record per deprecated namespace with fields:
`deprecated_event_name`, `canonical_event_name`, `wave_removed`, `date_removed`.
This file does not rotate and is not subject to retention pruning.
Consumer tooling (e.g., `factory-query deprecations list`) reads it for
operator onboarding. This work is tracked as a backlog item for v1.1.

## Dependencies

**Incoming (consumers of SS-03):**
- SS-01 (Hook Dispatcher Core) — `host::emit_event` calls `FileSink::write`
  for all plugin-emitted events; `main.rs` calls `FileSink::write` for
  lifecycle events.
- Operators — read `events-*.jsonl` directly or via OTel Collector filelog
  receiver.

**Outgoing (SS-03 depends on):**
- No external crate dependencies beyond `serde_json`, `uuid`, `chrono` (all
  already in the workspace). The `sink-otel-grpc` external dependency on
  `opentelemetry`/`tonic` is retired with that crate.

## Cross-Cutting

- **Single-stream guarantee:** All events — lifecycle and domain — land in
  `events-YYYY-MM-DD.jsonl`. Consumers must filter on `event.category` to
  separate domain events from lifecycle noise.
- **Host override visibility:** D-15.3 dual-channel approach: inline
  `event.host_overrides` on the domain event + `vsdd.internal.host_field_override.v1`
  lifecycle event carrying `affected.plugin.name`. Both channels are emitted
  when the host overrides a plugin-supplied field (rate-limited to one
  `vsdd.internal.host_field_override.v1` per unique `(plugin.name, field_name)`
  pair per dispatcher invocation).
- **Debug file is NOT always-on in production:** ADR-007's always-on guarantee
  is amended. The debug file is active only when `VSDD_DEBUG_LOG=1` is set
  (or `debug_log_enabled = true` in config). It is unconditionally activated
  as a fallback target when `FileSink::write` fails.
- **Block path audit trail:** `vsdd.block.plugin_blocked.v1` event is emitted
  with `outcome=blocked`, `plugin.name`, and `hook.tool_name` when a plugin
  returns `HookResult::Block`.
- **`schema_version` field removed from JSONL lines:** Per ADR-015 D-15.3,
  the `.vN` suffix of `event.name` is the per-family version signal. The
  flat `schema_version` field in each JSONL line is retired.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-03/`
(prefix BC-3; 51 existing BCs).

**BCs under revision due to ADR-015:** Multi-sink fan-out BCs
(BC-3.01.001–BC-3.01.009) and router/filter BCs are being revised or
withdrawn as Wave 5 lands. The BC-3 prefix is retained; new BCs will be
added in the BC-3.03.* and BC-3.04.* clusters for FileSink write semantics
and the v2 config schema.

High-level BC groupings (post-ADR-015):
- FileSink write and partial-write semantics (BC-3.03.*)
- Write-failure cascade and debug-file fallback (BC-3.04.*)
- `observability-config.toml` v2 schema validation (BC-3.05.*)
- Resource attribute startup enrichment (BC-3.06.*)
- Per-event host-stamp contract (BC-3.07.*)
- Debug-file opt-in gating (BC-3.08.*)

## ADRs

- ADR-015: Single-stream event emission with OTel-aligned schema —
  `decisions/ADR-015-single-stream-otel-schema.md` (SUPERSEDES ADR-005)
- ADR-007: Always-on dispatcher self-telemetry —
  `decisions/ADR-007-always-on-telemetry.md` (AMENDED by ADR-015 D-15.1)
- ADR-005: Multi-sink observability — `decisions/ADR-005-multi-sink-observability.md`
  (SUPERSEDED by ADR-015; `Router`/`SinkRegistry`/`sink-otel-grpc` retired)

## Drift / Known Issues

- **DRIFT-002 (resolved by ADR-015):** `internal.sink_*` event constants
  (`SINK_ERROR`, `SINK_QUEUE_FULL`, etc.) declared in `internal_log.rs` but
  never emitted. With the multi-sink fan-out retired, these constants are
  obsolete. They will be removed at Wave 5 along with the multi-sink types.
- **DRIFT-003 (obsolete):** Dedicated OS thread per sink driver. Moot after
  `sink-otel-grpc` retirement and `SinkRegistry` deprecation. Wave 5 cleanup.
- **DRIFT-005 (closed):** `sink-http`, `sink-datadog`, `sink-honeycomb`
  drivers planned but not implemented. ADR-015 retires the multi-sink model;
  these planned drivers are cancelled. Operators needing these backends use
  the OTel Collector.
- **TD-015-a (open):** `publish = false` on retired crates does not prevent
  intra-workspace coupling. A CI check using `cargo metadata` to reject new
  workspace-internal dependencies on retired crates is needed before Wave 5
  deletion. Tracked in technical debt register.
