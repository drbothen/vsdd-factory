---
document_type: architecture-section
level: L3
section: "SS-01-hook-dispatcher"
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

# SS-01: Hook Dispatcher Core

## [Section Content]

## Purpose

The Hook Dispatcher Core is the compiled Rust entry point for all Claude Code hook
events. It is a per-invocation short-lived process: Claude Code spawns a fresh
`factory-dispatcher` binary on each hook event, passes the event envelope on stdin
as JSON, and reads the exit code (0 = continue; 2 = block intent). No daemon is
maintained between events.

The subsystem owns all logic between the raw stdin envelope and the per-plugin WASM
invocation boundary: parsing, registry loading, event-to-plugin matching, tier
grouping, parallel-within-tier sequential-between-tier execution, block intent
aggregation, and error handling. It deliberately exits 0 on any registry, payload,
or engine error so that a misconfigured dispatcher never blocks a user's tool call
(ADR-001, NFR-REL-001).

The dispatcher also generates and propagates a `dispatcher_trace_id` (UUID v4) per
invocation, attaches it to every emitted event, and drives the always-on
`dispatcher-internal-YYYY-MM-DD.jsonl` self-telemetry log independently of any
configured external sink (ADR-007, Q6 Option B). This ensures debuggability even
when all sinks are misconfigured or unreachable.

## Modules

| Module / File | Responsibility |
|---|---|
| `crates/factory-dispatcher/src/main.rs` | I/O entry point: reads stdin, drives `engine`, `executor`, `sinks`; sets exit code; emits `dispatcher.started` / `dispatcher.completed` |
| `crates/factory-dispatcher/src/payload.rs` | Parse + validate stdin JSON; accepts both `event_name` and `hook_event_name` field aliases; emits `PayloadError` |
| `crates/factory-dispatcher/src/registry.rs` | Load + validate `hooks-registry.toml` (mtime-cached); resolve relative plugin paths; expose `Registry`, `RegistryEntry`, `Capabilities`, `OnError`, `REGISTRY_SCHEMA_VERSION = 1` |
| `crates/factory-dispatcher/src/routing.rs` | Match plugins by event + tool regex; group matches by priority tier; expose `match_plugins`, `group_by_priority` |
| `crates/factory-dispatcher/src/executor.rs` | Run tiers sequentially; plugins within tier via `tokio::spawn_blocking` in parallel; aggregate `block_intent`; expose `execute_tiers`, `ExecutorInputs`, `PluginOutcome`, `TierExecutionSummary` |
| `crates/factory-dispatcher/src/invoke.rs` | Per-plugin wasmtime Store + WASI ctx + fuel/epoch budgets; classify trap cause; expose `invoke_plugin`, `PluginResult`, `InvokeLimits`, `STDERR_CAP_BYTES = 4096`, `TimeoutCause::{Epoch, Fuel}` |
| `crates/factory-dispatcher/src/engine.rs` | Build wasmtime `Engine`; epoch ticker at 10 ms cadence; expose `build_engine`, `EpochTicker`, `EPOCH_TICK_MS = 10` |
| `crates/factory-dispatcher/src/plugin_loader.rs` | Per-invocation `Module` cache; amortizes WASM compile cost across plugins reusing the same `.wasm`; expose `PluginCache::new`, `get_or_compile` |
| `crates/factory-dispatcher/src/host/` | wasmtime `Linker<HostContext>` setup; all `vsdd::*` imports; capability enforcement; expose `setup_linker`, `HostContext`, `HostCallError`, `codes::*` |
| `crates/factory-dispatcher/src/host/log.rs` | Plugin → dispatcher structured log (level, message); `register` |
| `crates/factory-dispatcher/src/host/emit_event.rs` | Plugin → typed event emission with field bag; `register`, `decode_fields` |
| `crates/factory-dispatcher/src/host/context_fns.rs` | Read-only context accessors: `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`; `register` |
| `crates/factory-dispatcher/src/host/env.rs` | Allow-listed env-var read (per-plugin `Capabilities.env_allow`); `register` |
| `crates/factory-dispatcher/src/host/read_file.rs` | Capability-gated FS read (path_allow prefix match); `register` — NOTE: wired through `Linker<HostContext>` only; StoreData-typed linker path is a stub (DRIFT-001) |
| `crates/factory-dispatcher/src/host/exec_subprocess.rs` | Capability-gated subprocess: `binary_allow`, `shell_bypass_acknowledged`, setuid refusal, bounded time + output; `register`, `decode_args`, `run` |
| `crates/factory-dispatcher/src/host/memory.rs` | Bounds-checked guest memory read/write helpers: `read_wasm_string`, `read_wasm_bytes`, `write_wasm_bytes` |
| `crates/factory-dispatcher/src/internal_log.rs` | Always-on `dispatcher-internal-YYYY-MM-DD.jsonl`; daily rotation; 30-day retention; 17 event-type constants; expose `InternalLog`, `InternalEvent` |
| `crates/factory-dispatcher/src/sinks.rs` | Load `observability-config.toml`; fan-out to enabled drivers; expose `SinkRegistry::load`, `from_config`, `submit_all`, `flush_all`, `shutdown_all` |
| `crates/factory-dispatcher/tests/*.rs` | 7 integration test files covering payload, registry, executor, host fns, E2E pipeline |

## Public Interface

The dispatcher binary is invoked by Claude Code via `hooks.json` (written by the
activate skill). Its external API is entirely I/O-level:

- **stdin:** JSON hook envelope. Fields: `event_name` OR `hook_event_name` (both
  accepted via serde alias), `tool_name`, `session_id`, `tool_input`,
  `tool_response`, optional `cwd`.
- **stdout:** Not consumed by Claude Code at this boundary; plugins write their
  `HookResult` JSON on their own stdout which the dispatcher reads via WASM memory.
- **exit code:** 0 = continue; 2 = block (one or more plugins set block intent).
- **`hooks-registry.toml`:** Configuration file consumed at startup. Schema version
  REGISTRY_SCHEMA_VERSION = 1; mismatch = hard error.
- **`observability-config.toml`:** Sink configuration consumed at startup.
  Schema version 1; mismatch = hard error.

Internal (crate-level) public surface: exposed via `factory-dispatcher` lib target
for use by integration tests.

## Internal Structure

Control flow per invocation (pass-1-architecture.md, lines 94-101):

1. `main.rs`: parse stdin → `HookPayload`; assign `dispatcher_trace_id`; emit
   `dispatcher.started` to internal log.
2. `registry.rs`: load + validate `hooks-registry.toml`; mtime-cache invalidates
   on file change.
3. `routing.rs`: `match_plugins(event, tool)` → `group_by_priority` → ordered vec
   of tiers.
4. `engine.rs` + `plugin_loader.rs`: build wasmtime engine + epoch ticker; compile
   or retrieve cached WASM `Module`.
5. `executor.rs`: `execute_tiers` — for each tier: `spawn_blocking` all plugins in
   parallel; collect `PluginOutcome`; detect block intent.
6. `invoke.rs`: per-plugin Store + WASI ctx; fuel cap (default 10M) + epoch budget
   (default 5 000 ms); invoke `_start`; classify trap as `Epoch`, `Fuel`, or crash.
7. `host/`: capability enforcement per host fn call; all denials emit
   `internal.capability_denied` AND return `codes::CAPABILITY_DENIED = -1`.
8. `internal_log.rs` + `sinks.rs`: emit events to always-on log AND fan-out to
   configured sinks; `flush_all`; `shutdown_all`; exit.

Key Rust types: `HookPayload`, `Registry`, `RegistryEntry`, `Capabilities`,
`HostContext`, `InvokeResult`, `PluginOutcome`, `TierExecutionSummary`,
`InternalLog`, `InternalEvent`, `SinkRegistry`.

## Dependencies

**Incoming (consumers of SS-01):**
- SS-09 (Configuration and Activation) — generates and places `hooks.json` variants
  that invoke the dispatcher binary; generates `hooks-registry.toml` from
  `hooks.json`.
- Claude Code harness — spawns the binary per hook event.

**Outgoing (SS-01 depends on):**
- SS-02 (Hook SDK and Plugin ABI) — linker expects plugins compiled against
  `HOST_ABI_VERSION = 1`; `vsdd::*` imports registered by `setup_linker`.
- SS-03 (Observability Sinks) — `sinks.rs` loads `SinkRegistry`; `submit_all`
  fans events to sink-file, sink-otel-grpc, and future drivers.
- SS-04 (Plugin Ecosystem) — loads `.wasm` plugin binaries from disk; invokes
  `_start` via wasmtime; plugins may call `exec_subprocess` to reach SS-07.

## Cross-Cutting

- **Observability:** Every invocation emits `dispatcher.started`, per-plugin
  `plugin.invoked`/`plugin.completed`/`plugin.timeout`/`plugin.crashed`, and
  `dispatcher.completed` to `dispatcher-internal-YYYY-MM-DD.jsonl` (always-on,
  independent of sink config; ADR-007). All events carry `dispatcher_trace_id`.
- **Security:** Deny-by-default capability model. `exec_subprocess` requires
  `binary_allow` + `shell_bypass_acknowledged`. `read_file` requires `path_allow`.
  `env` read requires `env_allow`. Denials emit audit event and return -1.
  setuid subprocess refused unconditionally.
- **Sandboxing:** Per-plugin isolated wasmtime Store; epoch interruption (10 ms
  tick); fuel metering (default 10 000 000 ops); WASI preview-1 with stdin/stdout/
  stderr only (no preopens unless `read_file` capability granted).
- **Error handling:** `thiserror` enums per crate (`RegistryError`, `PayloadError`,
  `InvokeError`, `EngineError`, `HostCallError`). Dispatcher main swallows
  registry/payload/engine errors → logs `internal.dispatcher_error` → exits 0
  (non-blocking contract; ADR-001, NFR-REL-001).
- **Schema versioning:** `REGISTRY_SCHEMA_VERSION = 1`; `INTERNAL_EVENT_SCHEMA_VERSION = 1`;
  mismatches produce hard errors at load time (NFR-MAINT-004).

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-01/`
(target prefix BC-1; 106 BCs per ARCH-INDEX Subsystem Registry).

High-level BC groupings: payload parsing and alias tolerance (BC-1.001–BC-1.010),
registry load and schema validation (BC-1.011–BC-1.025), plugin routing and tier
grouping (BC-1.026–BC-1.040), executor parallel-within-tier semantics
(BC-1.041–BC-1.055), per-plugin invoke lifecycle (BC-1.056–BC-1.068), host
function capability enforcement (BC-1.069–BC-1.075), internal log emission
(BC-1.076–BC-1.080), non-blocking error handling invariants (BC-1.081–BC-1.086),
ADR-015 OTel emission contracts (BC-1.11.001–BC-1.11.003: VSDD_TRACE_ID injection,
FileSink partial-write recovery, atomic dual-emit host helper).

## ADRs

- ADR-001: Rust for the dispatcher — `decisions/ADR-001-rust-dispatcher.md`
- ADR-004: TOML for all configuration files — `decisions/ADR-004-toml-config.md`
- ADR-005: Multi-sink observability natively in dispatcher — `decisions/ADR-005-multi-sink-observability.md` (SUPERSEDED by ADR-015)
- ADR-006: HOST_ABI_VERSION as separate semver constant — `decisions/ADR-006-host-abi-version.md`
- ADR-007: Always-on dispatcher self-telemetry — `decisions/ADR-007-always-on-telemetry.md` (AMENDED by ADR-015 D-15.1)
- ADR-008: Parallel-within-tier, sequential-between-tier execution — `decisions/ADR-008-parallel-within-tier.md`
- ADR-010: StoreData-typed linker for host functions (invoke.rs pattern) — `decisions/ADR-010-storedata-linker.md`
- ADR-015: Single-stream OTel-aligned event emission — `decisions/ADR-015-single-stream-otel-schema.md` (affects SS-01 host enrichment, exec_subprocess trace injection, and emit_pair host helper)

## Drift / Known Issues

- **DRIFT-001 (P1 — medium):** `read_file` host fn at the StoreData-typed linker
  (`invoke.rs:447-474`) is a CAPABILITY_DENIED stub. The full impl in
  `host/read_file.rs` is registered against `Linker<HostContext>` but not wired
  through the invoke path. Must-fix before rc.1.
- **DRIFT-002 (P1 — medium):** `internal.sink_*` event constants declared in
  `internal_log.rs:67-70` but never emitted. SinkFailure accumulated in mutex
  but never converted to events. Must-fix before rc.1 (S-4.4).
- **DRIFT-007 (P3 — cosmetic):** `dispatcher.shutting_down` constant defined at
  `internal_log.rs:58` but no emit in any exit path. Acceptable for a short-lived
  process; remove or wire before 1.0.
- **DRIFT-008 (P3 — cosmetic):** `plugin.loaded` / `plugin.load_failed` constants
  declared but never emitted from `plugin_loader.rs`. 1-line fix.
