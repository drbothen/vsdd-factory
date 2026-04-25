# Pass 1: Architecture — vsdd-factory

**Date:** 2026-04-25
**Reads:** Pass 0 inventory + Cargo.toml + design docs + entry points
**Critical context:** This project has TWO subsystems sharing one repo. Their boundary is the `factory-dispatcher` binary, which is *built from* Subsystem A (Rust) and *invoked by* Subsystem B (plugin layer) via Claude Code's hooks.json.

## 1. Component Catalog

### Subsystem A — Rust hook dispatcher (compiled artifacts)

| Component | Crate | Responsibility | Public Surface |
|---|---|---|---|
| **dispatcher-binary** | `factory-dispatcher` (bin) | I/O entry point. Reads stdin JSON, drives end-to-end flow, sets exit code. | `main.rs` only |
| **dispatcher-lib** | `factory-dispatcher` (lib) | Reusable internals. Tests + future embedded use cases consume these. | `engine`, `executor`, `host`, `internal_log`, `invoke`, `payload`, `plugin_loader`, `registry`, `routing`, `sinks` |
| **engine** | factory-dispatcher::engine | wasmtime engine builder + epoch ticker (10ms cadence) + `timeout_ms_to_epochs`. | `build_engine`, `EpochTicker`, `EPOCH_TICK_MS = 10` |
| **registry** | factory-dispatcher::registry | Parse + validate `hooks-registry.toml`. Resolve relative plugin paths. | `Registry::load`, `RegistryEntry`, `Capabilities`, `OnError`, `RegistryDefaults`, `REGISTRY_SCHEMA_VERSION = 1` |
| **payload** | factory-dispatcher::payload | Typed projection of stdin envelope. Accepts both `event_name` and `hook_event_name`. | `HookPayload`, `PayloadError` |
| **routing** | factory-dispatcher::routing | Select matching plugins, group by priority. | `match_plugins`, `group_by_priority` |
| **executor** | factory-dispatcher::executor | Run tiers sequentially, plugins-within-tier in parallel via tokio + spawn_blocking. Aggregate `block_intent`. | `execute_tiers`, `ExecutorInputs`, `PluginOutcome`, `TierExecutionSummary` |
| **invoke** | factory-dispatcher::invoke | Per-plugin wasmtime store, WASI ctx, fuel/epoch budgets, classify trap. | `invoke_plugin`, `PluginResult`, `InvokeLimits`, `STDERR_CAP_BYTES = 4096`, `TimeoutCause::{Epoch, Fuel}` |
| **plugin_loader** | factory-dispatcher::plugin_loader | Per-invocation `Module` cache (amortizes compile cost across plugins reusing same .wasm). | `PluginCache::new`, `get_or_compile` |
| **host** | factory-dispatcher::host | wasmtime `Linker<HostContext>` with all `vsdd::*` imports. Capability enforcement. | `setup_linker`, `HostContext`, `HostCallError`, `codes::*` |
| **host::log** | host/log.rs | Plugin → dispatcher structured log (level, message). | `register` |
| **host::emit_event** | host/emit_event.rs | Plugin → typed event emission with field bag. | `register`, `decode_fields` |
| **host::context_fns** | host/context_fns.rs | Read-only context accessors: `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`. | `register` |
| **host::env** | host/env.rs | Allow-listed env-var read (per-plugin `Capabilities.env_allow`). | `register` |
| **host::read_file** | host/read_file.rs | Capability-gated FS read (path_allow prefix match). | `register` |
| **host::exec_subprocess** | host/exec_subprocess.rs | Capability-gated subprocess: binary_allow, shell_bypass_acknowledged, setuid refusal, bounded time + output. | `register`, `decode_args`, `run` |
| **host::memory** | host/memory.rs | Bounds-checked guest memory read/write helpers. | `read_wasm_string`, `read_wasm_bytes`, `write_wasm_bytes` |
| **internal_log** | factory-dispatcher::internal_log | Always-on `dispatcher-internal-YYYY-MM-DD.jsonl`. Daily rotation. 30-day retention. | `InternalLog`, `InternalEvent`, 17 event-type constants |
| **sinks** | factory-dispatcher::sinks | Loader for `observability-config.toml`; fan-out to enabled drivers. | `SinkRegistry::load`, `from_config`, `submit_all`, `flush_all`, `shutdown_all` |
| **sinks::router** | factory-dispatcher::sinks::router | (router module — placeholder for future routing-tag enrichment per S-4.6) | `Router` |

### Subsystem A — supporting crates (linked into dispatcher)
| Crate | Role | Key types |
|---|---|---|
| **sink-core** | Trait + shared types every sink driver implements. Field-bag `SinkEvent`, allow-then-deny `RoutingFilter`. | `Sink` trait, `SinkEvent`, `SinkConfigCommon`, `RoutingFilter`, `SinkError` |
| **sink-file** | Default JSONL append driver. Daily-rotated, mpsc-bounded queue. | `FileSink`, `FileSinkConfig`, `DEFAULT_QUEUE_DEPTH = 1000`, `SinkFailure` |
| **sink-otel-grpc** | OTLP/gRPC log forwarder. Owns its own current_thread tokio runtime on a dedicated OS thread. | `OtelGrpcSink`, `OtelGrpcConfig`, `DEFAULT_ENDPOINT = http://localhost:4317`, `DEFAULT_BATCH_SIZE = 100` |
| **hook-sdk** | Plugin-author API (cargo dep). `HookPayload`, `HookResult`, host fn shims. | `HookPayload`, `HookResult::{Continue, Block, Error}`, `HOST_ABI_VERSION = 1`, `host::*` |
| **hook-sdk-macros** | proc-macro crate that emits the `_start` adapter + JSON deserialize + panic boundary. | `#[hook]` |
| **hook-plugins/legacy-bash-adapter** | WASM plugin that reads `plugin_config.script_path` and shells out to bash. Multi-instance: one registry entry per bash hook. | `adapter_logic`, `BashOutcome`, `MAX_OUTPUT_BYTES = 1MiB`, `BASH_TIMEOUT_MS = 60_000` |
| **hook-plugins/capture-commit-activity** | First native WASM port (S-3.1 in flight). | `#[hook] on_hook` |

### Subsystem B — VSDD plugin layer (orchestration framework)

| Component | Path | Responsibility |
|---|---|---|
| **plugin manifest** | `plugins/vsdd-factory/.claude-plugin/plugin.json` | Declares plugin name, version, author. Read by Claude Code's plugin loader. |
| **marketplace manifest** | `.claude-plugin/marketplace.json` | Declares this repo as a marketplace with one plugin entry. |
| **orchestrator** | `agents/orchestrator/orchestrator.md` | Main-thread agent set as default after `/vsdd-factory:activate`. Reads `.lobster` workflow files and dispatches sub-agents. Does NOT produce artifacts itself. |
| **agents** | `agents/*.md` (33 specialist + orchestrator) | Sub-personas with frontmatter `name:`, `description:`, optional `tools:`/`model:`/`color:`. Spawned by orchestrator via Agent tool. |
| **skills** | `skills/*/SKILL.md` (119 procedures) | Named procedures. Invokable as `/vsdd-factory:<skill-name>`. Many compose templates and other skills. |
| **hooks (bash)** | `hooks/*.sh` (44 scripts) | Each is a discrete behavioral contract. PreToolUse: gates (block-ai-attribution, protect-secrets, destructive-command-guard, factory-branch-guard, brownfield-discipline). PostToolUse: capture (capture-commit-activity, capture-pr-activity) + validators (validate-* family). SubagentStop: handoff-validator, pr-manager-completion-guard, update-wave-state-on-merge. Stop: session-learning, warn-pending-wave-gate. |
| **hooks (registry)** | `hooks-registry.toml` (45 entries) | Routing table consumed by `factory-dispatcher`. As of v1.0.0-beta.4, every entry routes through `legacy-bash-adapter.wasm`. |
| **hooks.json (legacy)** | `hooks/hooks.json` | Active v0.79.x-style routing table. Coexists with hooks-registry.toml — TODO: drift between the two is structural (see Pass 6). |
| **hooks.json.template** | `hooks/hooks.json.template` | Source of truth for new (dispatcher-style) hooks.json. CI generates 5 platform-specific variants from this. Activate skill copies the right one over `hooks.json` at activation time. |
| **dispatcher binary set** | `hooks/dispatcher/bin/{darwin-arm64,darwin-x64,linux-x64,linux-arm64,windows-x64}/factory-dispatcher[.exe]` | Per-platform compiled binaries. Committed to repo by release CI. |
| **workflows** | `workflows/*.lobster` (8 modes) + `workflows/phases/*.lobster` (8 phase sub-flows) | Lobster YAML data files declaring steps with `name`, `type`, `agent`, `depends_on`, `on_failure`, `max_retries`, `timeout`. Read by orchestrator as data, not prose. |
| **templates** | `templates/*` (108 files + subdirs) | Output-shape templates referenced by skills. E.g., `recovered-architecture-template.md`, `behavioral-contract-template.md`, `pr-description-template.md`. |
| **rules** | `rules/*` | Cross-cutting policies (10+ rule files). |
| **commands** | `commands/*.md` (110 files) | Slash-command bindings (`/vsdd-factory:<name>` → invokes a skill or kicks off a flow). |
| **bin** | `bin/*` (12 helpers) | Project-local executables: `emit-event`, `factory-dashboard`, `factory-obs`, `factory-query`, `factory-replay`, `factory-report`, `factory-sla`, `lobster-parse`, `multi-repo-scan`, `research-cache`, `wave-state`, `compute-input-hash`. |
| **tools/observability** | `tools/observability/` | OTel collector configs, Grafana dashboards, etc. Not source-of-truth for Rust dispatcher's sink config. |
| **tests** | `tests/*.bats` + helpers (71 files) | Bash-side regression suite (1245+ baseline bats per CHANGELOG; 11 dispatcher-pipeline regression tests in `regression-v1.0.bats`). |

## 2. Layer Structure

```
┌────────────────────────────────────────────────────────────────────┐
│  USER LAYER                                                         │
│  Claude Code session — types `/vsdd-factory:*` slash commands,      │
│  drives orchestrator, runs tools (Bash, Edit, Write, Agent, ...)    │
└────────────────────────────────────────────────────────────────────┘
                  │                         │
                  ▼                         ▼
┌────────────────────────────────────────────────────────────────────┐
│  ORCHESTRATION LAYER (Subsystem B — markdown + bash + lobster)      │
│  agents/* + skills/* + workflows/* + templates/*                    │
│  Reads .factory/STATE.md, dispatches sub-agents per workflow steps. │
│  No code here — pure declarative + procedural-prose.                │
└────────────────────────────────────────────────────────────────────┘
                  │ (Tool invocations: Bash, Edit, Write, Agent)
                  ▼
┌────────────────────────────────────────────────────────────────────┐
│  CLAUDE CODE HOOK BUS                                                │
│  hooks.json (per-platform) declares: on PreToolUse/PostToolUse/...  │
│  spawn factory-dispatcher with stdin = JSON envelope                │
└────────────────────────────────────────────────────────────────────┘
                  │ (stdin JSON)
                  ▼
┌────────────────────────────────────────────────────────────────────┐
│  DISPATCH LAYER (Subsystem A — Rust binary, WASI host)              │
│  factory-dispatcher main.rs                                         │
│  ├── parse stdin (payload.rs)                                       │
│  ├── load registry (registry.rs, mtime-cached)                      │
│  ├── match_plugins + group_by_priority (routing.rs)                 │
│  ├── build engine + ticker + cache (engine.rs, plugin_loader.rs)    │
│  ├── execute_tiers (executor.rs) — sequential tiers, parallel within│
│  │     └── invoke_plugin (invoke.rs) — wasmtime + WASI + host fns   │
│  └── exit 0 (continue) | 2 (block_intent)                           │
└────────────────────────────────────────────────────────────────────┘
                  │ (linker imports = host fns)
                  ▼
┌────────────────────────────────────────────────────────────────────┐
│  PLUGIN LAYER (WASM, sandboxed by wasmtime)                          │
│  legacy-bash-adapter.wasm + (future) capture-commit-activity.wasm + │
│  block-ai-attribution.wasm + capture-pr-activity.wasm + …            │
│  Built from crates/hook-plugins/* via vsdd-hook-sdk.                 │
└────────────────────────────────────────────────────────────────────┘
                  │
        ┌─────────┴────────────┬───────────────┐
        ▼                      ▼               ▼
┌──────────────┐    ┌──────────────────┐  ┌────────────┐
│ exec_sub-    │    │  emit_event,     │  │  log,      │
│ process      │    │  read_file,      │  │  context   │
│ (cap-gated)  │    │  env (cap-gated) │  │  getters   │
└──────────────┘    └──────────────────┘  └────────────┘
        │                  │                    │
        │                  ▼                    ▼
        │         ┌──────────────────────────────────┐
        │         │  TELEMETRY LAYER                 │
        │         │  internal_log → dispatcher-      │
        │         │    internal-YYYY-MM-DD.jsonl     │
        │         │    (always-on, 30-day retention) │
        │         │  sinks → SinkRegistry            │
        │         │    ├── sink-file (JSONL)         │
        │         │    └── sink-otel-grpc            │
        │         │    (config: observability-       │
        │         │     config.toml)                 │
        │         └──────────────────────────────────┘
        ▼
┌────────────────────────────────────────────────────────────────────┐
│  LEGACY BRIDGE                                                      │
│  legacy-bash-adapter.wasm → exec_subprocess("bash", [<script>])    │
│  → 44 bash hooks under plugins/vsdd-factory/hooks/*.sh             │
│  Bash hooks call ${CLAUDE_PLUGIN_ROOT}/bin/emit-event for events.  │
└────────────────────────────────────────────────────────────────────┘
```

**Dependency direction:** strictly down-and-out. The plugin layer never imports the dispatcher; the dispatcher never imports the plugin layer (only loads `.wasm` files from disk). `sink-core` is the only shared abstraction between the dispatcher and the sink drivers.

## 3. Deployment Topology

- **Single-process per hook event.** Each Claude Code hook event spawns a fresh `factory-dispatcher` process. Dispatcher loads registry, runs plugins, exits. No long-running daemon. Cold-start cost ≤ ~1-5ms per design (wasmtime engine cached per invocation, not globally).
- **Per-platform binary.** 5 OS+arch combinations: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64. Committed to the repo by CI on each release. No runtime dep (Rust binary is statically linked).
- **Multiple WASM plugins per dispatch.** Each plugin invocation gets its own wasmtime `Store` with bounded fuel + epoch budget; same `Engine` reused across plugins in one dispatch. Plugins within a priority tier run in parallel tokio tasks (`spawn_blocking`); between tiers run sequentially.
- **Co-located telemetry.**
  - `dispatcher-internal-YYYY-MM-DD.jsonl` always-on, written to `${CLAUDE_PROJECT_DIR}/.factory/logs/`. Independent of `observability-config.toml` (Q6 Option B).
  - `events-<date>.jsonl` (and analogous per-sink files) written by the file sink driver if configured.
  - OTLP/gRPC sink talks to `http://localhost:4317` by default (the project's local Grafana/Loki stack documented in `tools/observability/`).
- **Async vs sync hook events.** `PostToolUse`, `Stop`, `SubagentStop`, `SessionStart`, `SessionEnd` use `"async": true` in `hooks.json.template` so Claude Code doesn't wait. `PreToolUse` and `PermissionRequest` are sync (so `exit 2` can block).

## 4. Cross-cutting Concerns

| Concern | Implementation | Where |
|---|---|---|
| **Logging** | `tracing = 0.1` + `tracing-subscriber = 0.3` for Rust internal log emission. `dispatcher-internal.jsonl` is the durable always-on log. Bash hooks log via `bin/emit-event`. | crates/factory-dispatcher/src/internal_log.rs; plugins/vsdd-factory/bin/emit-event |
| **Auth** | None at the dispatcher boundary (Claude Code is the trust boundary). Sinks may carry per-driver auth (Datadog API keys, Honeycomb keys per design — not yet shipped). | (deferred to S-4.x) |
| **Error handling** | `thiserror` for crate-level error enums (`RegistryError`, `PayloadError`, `InvokeError`, `EngineError`, `HostCallError`, `FileSinkError`, `SinkError`); `anyhow` at boundaries (sinks::load returns `anyhow::Result`). Dispatcher main *swallows* registry/payload/engine errors → logs `internal.dispatcher_error` → exits 0 (non-blocking by design). | All Rust crates; main.rs `emit_dispatcher_error` |
| **Capability enforcement** | Deny-by-default. Each capability-bearing host fn (`exec_subprocess`, `read_file`, `env`) checks `HostContext.capabilities.*` before performing the action. Denied calls return `codes::CAPABILITY_DENIED = -1` AND emit `internal.capability_denied`. | crates/factory-dispatcher/src/host/*.rs |
| **Sandboxing** | wasmtime per-plugin Store with: epoch interruption (10ms tick), fuel metering (default 10M), per-call wall-clock cap (default 5_000ms), WASI preview-1 with stdin/stdout/stderr only (no preopens unless `read_file` capability). | crates/factory-dispatcher/src/{engine,invoke}.rs |
| **Secrets** | `protect-secrets.sh` bash hook (PreToolUse on Bash and Read). Capability allow-list for env_allow does NOT include common secret env vars by default. | plugins/vsdd-factory/hooks/protect-secrets.sh |
| **Schema versioning** | `schema_version` field in `hooks-registry.toml` (REGISTRY_SCHEMA_VERSION = 1), `observability-config.toml` (1), `InternalEvent.schema_version` (INTERNAL_EVENT_SCHEMA_VERSION = 1). Mismatch = hard error. | registry.rs::validate, sinks::mod.rs::from_config, internal_log.rs |
| **Trace correlation** | `dispatcher_trace_id` (UUID v4) generated per stdin envelope; injected into outgoing payload JSON; carried on every emitted event (internal log + sinks). | main.rs::run + executor.rs::emit_invoked + emit_event host fn |
| **Test scaffolding** | `tempfile`, `filetime`, `wat` (parse WAT to wasm in tests). Unit tests live in `#[cfg(test)] mod tests` blocks; integration tests in `crates/factory-dispatcher/tests/*.rs` (7 files). bash bats suite for end-to-end. | crates/*/Cargo.toml [dev-dependencies] |

## 5. Data Flow Diagram

```mermaid
sequenceDiagram
    autonumber
    participant CC as Claude Code
    participant FD as factory-dispatcher
    participant Reg as hooks-registry.toml
    participant Eng as wasmtime Engine + Ticker
    participant Plg as Plugin (WASM)
    participant Bash as bash hook script
    participant IL as dispatcher-internal.jsonl
    participant Snk as SinkRegistry (file + otel-grpc)

    CC->>FD: stdin = JSON hook envelope (event_name | hook_event_name, tool_name, session_id, tool_input, tool_response)
    FD->>FD: parse + validate (HookPayload)
    FD->>FD: assign dispatcher_trace_id (UUID v4)
    FD->>IL: dispatcher.started
    FD->>Reg: load + validate (schema_version=1)
    FD->>FD: match_plugins(event, tool) + group_by_priority
    loop For each priority tier (sequential)
        par Plugins in tier (parallel via tokio spawn_blocking)
            FD->>Eng: build Store, set epoch deadline, set fuel
            FD->>Plg: invoke _start with payload+plugin_config on stdin
            FD->>IL: plugin.invoked
            alt Plugin uses exec_subprocess
                Plg->>FD: host call (cap-gated; binary_allow, shell_bypass_acknowledged)
                FD->>Bash: execute bash (legacy-bash-adapter path)
                Bash-->>FD: exit_code, stdout, stderr (truncated)
                FD-->>Plg: SubprocessResult envelope
            end
            Plg->>FD: emit_event(type, fields) host call
            FD->>IL: <event_type>
            FD->>Snk: submit_all (fan-out to file + otel-grpc)
            Plg-->>FD: HookResult JSON on stdout (Continue|Block|Error) + exit_code
            FD->>IL: plugin.completed | plugin.timeout | plugin.crashed (with stderr capture)
        end
    end
    FD->>FD: aggregate block_intent
    FD-->>CC: exit code 0 (continue) | 2 (block)
```

## 6. Inter-subsystem Coupling

The two subsystems communicate via **four contracts**, all of which are co-versioned at the plugin version (1.0.0-beta.4):

| Contract | Subsystem A side | Subsystem B side | Versioning |
|---|---|---|---|
| **stdin envelope** | `factory-dispatcher` parses `HookPayload` (`payload.rs`) — accepts `event_name` (canonical) and `hook_event_name` (Claude Code harness alias) | Claude Code itself populates this; bash hooks consume the same shape via `bin/emit-event` and via stdin in legacy mode | Schema version: `HOST_ABI_VERSION = 1` (dispatcher constant) |
| **hooks-registry.toml** | dispatcher loads + validates `schema_version = 1` | plugin layer's `scripts/generate-registry-from-hooks-json.sh` produces it from `hooks.json` | `REGISTRY_SCHEMA_VERSION = 1` |
| **observability-config.toml** | dispatcher loads via `SinkRegistry::load` | plugin layer ships safe defaults; operators override locally | `schema_version = 1` |
| **plugin ABI** | dispatcher's `setup_linker` registers `vsdd::*` host imports; `HOST_ABI_VERSION = 1` | `vsdd-hook-sdk` declares the same `HOST_ABI_VERSION = 1`; `#[hook]` macro binds against it | Mismatch = plugin refused to load with loud `internal.dispatcher_error` |

The **activate skill** (`/vsdd-factory:activate`) is the runtime glue:
1. Detects host platform (darwin-arm64 / darwin-x64 / linux-x64 / linux-arm64 / windows-x64).
2. Copies `hooks/hooks.json.<platform>` over `hooks.json` (the canonical, gitignored file).
3. Verifies the dispatcher binary at `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]` is present + executable.
4. Persists `activated_platform` in `.claude/settings.local.json`.

This is the *only* mechanism that wires Subsystem A artifacts (the binaries) into Subsystem B's runtime configuration on a particular operator's machine. The activation must happen post-install on every fresh install (per CHANGELOG migration notes).

## 7. State Checkpoint
```yaml
pass: 1
status: complete
files_scanned: ~50 (Rust top-level + plugin layer manifests + workflow samples + design doc deep-read)
key_modules_documented: 23 Rust components + 14 plugin-layer components
deployment_topology: per-hook short-lived process; per-platform committed binaries; co-located telemetry
inter_subsystem_contracts: 4 (stdin envelope, hooks-registry.toml, observability-config.toml, plugin ABI)
timestamp: 2026-04-25
next_pass: 2
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **Novelty score** | SUBSTANTIVE — first architectural mapping for this repo |
| **Trajectory** | First sweep across both subsystems. Component catalog (37 entries), layer diagram, deployment topology, and 4 inter-subsystem contracts all newly derived from source. No prior pass-1 artifact existed. |
| **Verdict** | FINDINGS_REMAIN — passes 2–6 plus deepening rounds remain. CONVERGENCE_REACHED is not declarable until Phase B convergence loops complete. |
