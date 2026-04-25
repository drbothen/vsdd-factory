# Pass 4: NFR Catalog — vsdd-factory

**Date:** 2026-04-25
**Reads:** Pass 0–3 outputs + dispatcher source + hooks-registry.toml + Cargo.toml + design doc + CHANGELOG.

NFRs grouped by category. Each NFR cites the configuration value or code location that encodes it. Drift between design doc and code-as-built is flagged.

## 1. Performance

### NFR-PERF-001: wasmtime epoch interruption tick = 10ms
- **Source:** `crates/factory-dispatcher/src/engine.rs:22` `pub const EPOCH_TICK_MS: u64 = 10;`.
- **Implication:** Timeout resolution = 10ms. A 5_000ms plugin budget translates to 500 epoch ticks. Sub-tick timeouts get at least 1 tick of grace via `div_ceil` in `timeout_ms_to_epochs`.
- **Cost:** One dedicated OS thread sleeping 10ms then bumping `engine.increment_epoch()`. Negligible CPU.

### NFR-PERF-002: Default per-plugin wall-clock timeout = 5_000ms
- **Source:** `RegistryDefaults::default { timeout_ms: 5_000 }` in `registry.rs`. Override per-entry via `timeout_ms`.
- **Design doc:** Q3 resolution.
- **Override observed:** `convergence-tracker` (10_000ms), `validate-input-hash` (10_000ms), `validate-state-pin-freshness` (10_000ms), `validate-state-index-status-coherence` (10_000ms), `validate-anchor-capabilities-union` (10_000ms), `validate-template-compliance` (10_000ms), `validate-wave-gate-completeness` (10_000ms) — heavier validators get 2× budget.

### NFR-PERF-003: Default per-plugin fuel cap = 10_000_000
- **Source:** `RegistryDefaults::default { fuel_cap: 10_000_000 }`.
- **Implication:** Guards against tight loops between yield points where epoch ticker can't fire (per design doc Q3).

### NFR-PERF-004: Default sink mpsc queue depth = 1000
- **Source:** `sink-file/lib.rs::DEFAULT_QUEUE_DEPTH = 1000`; `sink-otel-grpc/lib.rs::DEFAULT_QUEUE_DEPTH = 1000`.
- **Backpressure:** `try_send` overflow increments `queue_full_count` rather than blocking the producer (the dispatcher hot path stays non-blocking).

### NFR-PERF-005: OTLP/gRPC sink default batch size = 100; default endpoint = `http://localhost:4317`
- **Source:** `sink-otel-grpc/lib.rs::DEFAULT_BATCH_SIZE = 100; DEFAULT_ENDPOINT = "http://localhost:4317"`.
- **Implication:** Co-locates with the bundled local OTel collector documented in `tools/observability/`.

### NFR-PERF-006: Per-invocation Module cache amortizes WASM compile
- **Source:** `crates/factory-dispatcher/src/plugin_loader.rs::PluginCache`. Cold compile = ~ms cost. Cache lives for the dispatcher process (short-lived) so two registry entries pointing at the same .wasm pay compile cost once per dispatch.
- **Hot example:** 45 registry entries all pointing at `legacy-bash-adapter.wasm` → adapter compiled once, instantiated 45× per matching dispatch.

### NFR-PERF-007: tokio runtime is single-threaded current_thread (avoids thread pool overhead)
- **Source:** `main.rs:40` `#[tokio::main(flavor = "current_thread")]`.
- **Rationale (commented):** "wasmtime invocations are wrapped in `spawn_blocking` inside the executor, and the dispatcher's own fan-in work is trivial. Single-thread keeps startup cost low and avoids surprising thread pools in a short-lived process."

### NFR-PERF-008: Engine + ticker rebuilt per invocation
- **Source:** `main.rs:109–122` builds engine + ticker per `run`.
- **Design rationale (commented):** "Keeping the engine short-lived adds a bit of cold-start cost but sidesteps any global state concerns for the short-lived dispatcher process."

### NFR-PERF-009: Release profile favors size + LTO
- **Source:** `Cargo.toml` `[profile.release] opt-level = 3, lto = "thin", codegen-units = 1, strip = "symbols"`.
- **Implication:** Larger compile time; smaller, faster binaries committed to repo (5 platforms × ~few MB).

### NFR-PERF-010: STDERR_CAP_BYTES = 4 KiB per plugin event
- **Source:** `invoke.rs::STDERR_CAP_BYTES = 4096`.
- **Rationale:** Bounded per-event payload size (operators see this in `plugin.completed`/`plugin.timeout`/`plugin.crashed`).

### NFR-PERF-011: legacy-bash-adapter caps combined output at 1 MiB
- **Source:** `legacy-bash-adapter/src/lib.rs::MAX_OUTPUT_BYTES = 1024 * 1024`.

### NFR-PERF-012: legacy-bash-adapter wall-clock backstop = 60_000ms
- **Source:** `legacy-bash-adapter/src/lib.rs::BASH_TIMEOUT_MS = 60_000`.
- **Rationale (commented):** "Picked higher than the dispatcher's per-hook `timeout_ms` ceiling so the wasmtime epoch interrupt is the source of truth — the bash timeout is a backstop for the rare case where the dispatcher's epoch deadline didn't fire."

### NFR-PERF-013: Bash hook timeout in `hooks.json` = 5–10 seconds per script
- **Source:** `plugins/vsdd-factory/hooks/hooks.json` per-hook `timeout` fields. Most hooks 5s; validate-* heavy ones 10s; track-agent-* 5s.
- **Note:** Bash-side timeout (Claude Code-enforced) is in seconds; dispatcher-side is in ms.

### NFR-PERF-014: Dispatcher-managed hooks.json template uses 10_000ms timeout
- **Source:** `plugins/vsdd-factory/hooks/hooks.json.template` — every hook entry has `"timeout": 10000`.
- **Note:** This is post-dispatcher migration; legacy hooks.json is shorter per-script.

### NFR-PERF-015: Async hook events use `"async": true` in hooks.json template
- **Source:** `hooks.json.template` for PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd. `PreToolUse` and `PermissionRequest` stay sync.
- **Rationale:** Per design Q3 — non-blocking events get `async` so CC doesn't hang on a misbehaving dispatcher; blocking events stay sync so `exit 2` still blocks tool calls.

### NFR-PERF-016: SessionStart/SessionEnd use `"once": true`
- **Source:** `hooks.json.template` SessionStart and SessionEnd entries.

## 2. Security

### NFR-SEC-001: Capability deny-by-default
- **Source:** `host/mod.rs` + `Capabilities::default()` (all fields None / empty).
- **Implication:** A plugin with no `[hooks.capabilities]` block has no access to `read_file`, `exec_subprocess`, or `vsdd::env`. Always-allowed surface is restricted to logging, event emission, context getters.

### NFR-SEC-002: exec_subprocess shell-bypass requires explicit acknowledgement string
- **Source:** `Capabilities::ExecSubprocessCaps::shell_bypass_acknowledged: Option<String>`. SHELL_NAMES = bash, sh, zsh, pwsh, fish, csh, tcsh, ksh.
- **Currently in use:** Every `legacy-bash-adapter` registry entry has `shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"` (45 entries).

### NFR-SEC-003: Setuid/setgid binaries refused categorically (Unix)
- **Source:** `host/exec_subprocess.rs::refuse_setuid`. No allow-list override. Per design Q4.

### NFR-SEC-004: Bounded host fns by API contract — no unbounded overloads
- **Source:** Design doc + every cap-gated host fn. `exec_subprocess` requires `timeout_ms` + `max_output_bytes`; `read_file` requires `max_bytes` + `timeout_ms` (per design; SDK signature).
- **Rationale (design):** "every host function with blocking or resource-unbounded potential takes explicit size and time bounds at the API level — no unbounded overloads."

### NFR-SEC-005: env_allow restricts which env vars a plugin can read
- **Source:** `Capabilities::env_allow: Vec<String>`. Default = empty (deny-all).
- **Common allow-list across legacy adapter entries:** `["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]`.

### NFR-SEC-006: env passthrough to subprocess separately gated
- **Source:** `ExecSubprocessCaps::env_allow` (separate from `Capabilities::env_allow`).
- **Implication:** A plugin can read `CLAUDE_SESSION_ID` via `vsdd::env` (top-level env_allow) without that var being forwarded to spawned subprocesses (subprocess env_allow). Layered defense.

### NFR-SEC-007: cwd restricted via cwd_allow
- **Source:** `ExecSubprocessCaps::cwd_allow: Vec<String>`. Empty = no override permitted.

### NFR-SEC-008: Binary path resolution at load time prevents PATH hijack
- **Source:** Design Q4 + `exec_subprocess.rs::binary_allowed`. Allow-list checks both full path and basename.

### NFR-SEC-009: Capability denial emits durable audit event
- **Source:** Every cap-gated host fn calls `host.emit_internal(host.denial_event(...))` on denial → `internal.capability_denied` event with function, reason, command/variable/path.
- **Persistence:** Always lands in `dispatcher-internal-YYYY-MM-DD.jsonl` regardless of sink config.

### NFR-SEC-010: protect-secrets bash hook on Bash + Read tools
- **Source:** `plugins/vsdd-factory/hooks/protect-secrets.sh`. Registered on PreToolUse:Bash and PreToolUse:Read.

### NFR-SEC-011: protect-bc, protect-vp prevent edits to BC / verification-property files
- **Source:** Hooks on PreToolUse:Edit|Write.

### NFR-SEC-012: destructive-command-guard blocks dangerous bash invocations on PreToolUse:Bash
- **Source:** `hooks/destructive-command-guard.sh`.

### NFR-SEC-013: factory-branch-guard / brownfield-discipline / red-gate bash hooks
- **Source:** PreToolUse gates ensuring users cannot edit factory artifacts in disallowed states.

### NFR-SEC-014: WASI preview-1 sandbox; preview-2 explicitly out of scope (ADR-003)
- **Source:** Design doc ADR-003.
- **Implication:** No `wasi-http`, no `wasi-sockets`, no `wasi-command` for v1.0. Network access requires `exec_subprocess` (curl/wget) — gated by binary_allow.

### NFR-SEC-015: Plugin runs in isolated wasmtime Store with bounded epoch + fuel
- **Source:** `invoke.rs::invoke_plugin` builds Store + sets epoch deadline + fuel cap per call.
- **Implication:** Plugin failure in one Store does not affect siblings.

### NFR-SEC-016: Adversary SHA-currency gate for dispatched adversary subagents
- **Source:** `validate-wave-gate-prerequisite.sh` branches on subagent type; calls `verify-sha-currency.sh` when present.
- **Rationale:** CHANGELOG v1.0.0-beta.4: "Without this, the adversary could dispatch against a dirty factory-artifacts state and report stale-cite drift as 'false positive' findings."

### NFR-SEC-017: No auth on dispatcher input boundary (Claude Code is the trust boundary)
- **Source:** No auth code in dispatcher; stdin trusted.
- **Future sink auth (deferred):** Datadog/Honeycomb sinks (S-4.x) will carry per-driver auth headers. Not yet shipped.

### NFR-SEC-018: hooks.json gitignored; per-platform variants committed
- **Source:** Design doc Phase 0 step 3 + S-0.4 + activate skill behavior. Per-machine state stays out of source control.

## 3. Observability

### NFR-OBS-001: Always-on dispatcher self-telemetry at fixed path
- **Source:** `crates/factory-dispatcher/src/internal_log.rs`. Path: `${CLAUDE_PROJECT_DIR}/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl`. Independent of `observability-config.toml`. Per design Q6 Option B.

### NFR-OBS-002: 30-day retention default
- **Source:** `internal_log.rs::DEFAULT_RETENTION_DAYS = 30`. `prune_old(30)` called at dispatcher start.

### NFR-OBS-003: Daily rotation by event timestamp (not "now")
- **Source:** `internal_log.rs::date_stamp` derives from `event.ts[..10]`.
- **Rationale (commented):** "tests can write events dated in the past/future without reaching for a mocked clock."

### NFR-OBS-004: 17 internal event-type constants enumerated
- **Source:** `internal_log.rs:57–71`. `dispatcher.{started,shutting_down}`, `plugin.{loaded,load_failed,invoked,completed,timeout,crashed}`, `internal.{capability_denied,host_function_panic,sink_error,sink_queue_full,sink_circuit_opened,sink_circuit_closed,dispatcher_error}`, plus `plugin.log` host shim and arbitrary plugin-emitted types.

### NFR-OBS-005: Internal event schema version = 1
- **Source:** `INTERNAL_EVENT_SCHEMA_VERSION: u32 = 1`. Bumped only on non-backwards-compat shape change.

### NFR-OBS-006: Trace correlation via dispatcher_trace_id (UUID v4 per dispatch)
- **Source:** `main.rs::run` calls `new_trace_id()`. Injected into payload + every emitted event.
- **Implication:** Full causal chain reconstructible from sink data.

### NFR-OBS-007: Multi-sink fan-out
- **Source:** `crates/factory-dispatcher/src/sinks/mod.rs::SinkRegistry`. Each enabled sink whose `accepts(event)` returns true gets a clone via `submit_all`.
- **Currently shipped:** `file`, `otel-grpc`. Pending: `http`, `datadog`, `honeycomb` (Tier E S-4.x).

### NFR-OBS-008: Per-sink RoutingFilter (allow-then-deny)
- **Source:** `sink-core/lib.rs::RoutingFilter`. Allow non-empty = whitelist; deny applied after.

### NFR-OBS-009: Per-sink static tags (e.g., `env:prod`, `team:factory`)
- **Source:** `SinkConfigCommon::tags: BTreeMap<String, String>`. Reserved field collision (`type`, `ts`, etc.) not overwritten.

### NFR-OBS-010: Stderr capture on lifecycle events
- **Source:** CHANGELOG v1.0.0-beta.4: every `plugin.{completed,crashed,timeout}` event carries truncated stderr.
- **Cap:** STDERR_CAP_BYTES = 4096 with explicit `(stderr truncated)` marker.

### NFR-OBS-011: dispatcher.started includes platform, dispatcher_version, host_abi_version, pid, registry_path, loaded_plugin_count
- **Source:** `main.rs:73–86`. First event emitted on every dispatch.

### NFR-OBS-012: file sink path placeholders `{date}`, `{name}`, `{project}`
- **Source:** `sink-file/lib.rs::resolve_path_template`.

### NFR-OBS-013: file sink writes failures to `Mutex<Vec<SinkFailure>>` (pending S-4.4 wiring to internal.sink_error)
- **Source:** `sink-file/lib.rs::SinkFailure`. Drift from design: design says `internal.sink_error` events fire; current state stops at the failure-vector. The wiring lives in S-4.4.

### NFR-OBS-014: Bash hooks emit events via `bin/emit-event` helper
- **Source:** `plugins/vsdd-factory/bin/emit-event` invocation pattern across hook scripts (e.g., block-ai-attribution.sh `_emit` function).
- **Files:** events written to `.factory/logs/factory-events-*.jsonl` (per CHANGELOG).

### NFR-OBS-015: Local OTel collector configuration shipped under `tools/observability/`
- **Source:** `plugins/vsdd-factory/tools/observability/otel-collector-config.yaml` (per `sink-otel-grpc/lib.rs` doc comment).

### NFR-OBS-016: tracing crate available across the workspace
- **Source:** Cargo.toml `[workspace.dependencies] tracing = "0.1", tracing-subscriber = "0.3"`. Used for in-process structured logs (in addition to internal_log JSONL).

## 4. Reliability

### NFR-REL-001: All registry/payload/engine errors are non-blocking (exit 0)
- **Source:** `main.rs::run` error branches.
- **Implication:** A misconfigured registry never blocks the user's tool call; Claude Code stays usable. Errors land in internal log.

### NFR-REL-002: `on_error` policy per registry entry
- **Source:** `OnError::Continue` (default) | `OnError::Block`. Per-entry override.
- **Pattern observed:** Every legacy-adapter entry is `on_error = "continue"`. None of the 45 registry entries currently uses `on_error = "block"`.

### NFR-REL-003: Plugin failure isolated to that plugin
- **Source:** Each plugin runs in own wasmtime Store. Tokio task panic isolation.
- **Implication:** One plugin crashing does not affect siblings in the same tier.

### NFR-REL-004: Sink failure isolated to that sink
- **Source:** Per-sink driver owns its own `Mutex<Vec<SinkFailure>>`; submit returns immediately regardless of failure state.

### NFR-REL-005: Internal log writes are best-effort; never panic; never propagate
- **Source:** `internal_log.rs::write_inner` errors swallowed and printed to stderr only.

### NFR-REL-006: Internal log auto-creates missing parent directories
- **Source:** `internal_log.rs::write_inner` calls `fs::create_dir_all` first.

### NFR-REL-007: prune_old skips non-matching files (won't delete unrelated files even if log dir misconfigured)
- **Source:** `internal_log.rs::prune_old_inner` checks prefix + suffix.

### NFR-REL-008: Retry / circuit breaker / dead letter queue PENDING (S-4.4, S-4.5)
- **Source:** Design doc resilience config block (`[sinks.<name>.retries|circuit_breaker|queue|dead_letter]`). Stories S-4.4 + S-4.5.
- **Status:** NOT YET IMPLEMENTED. Internal log constants `internal.sink_circuit_opened|closed`, `internal.sink_queue_full` declared but never emitted.
- **DRIFT:** Design doc treats these as v1.0-rc.1 features; currently shipped 1.0.0-beta.4 has the stub but no driver-side enforcement.

### NFR-REL-009: Cooperative shutdown for epoch ticker
- **Source:** `engine.rs::EpochTicker::shutdown` sets atomic flag; thread joins.

### NFR-REL-010: Ticker shutdown is idempotent
- **Source:** Test `ticker_shutdown_is_idempotent`.

### NFR-REL-011: Atomic write of binaries + plugin.json + marketplace.json by release bot commit
- **Source:** CHANGELOG v1.0.0-beta.4 fix. Eliminates the `version=X with X-1 binaries` cache-staleness race.

### NFR-REL-012: Crash recovery in feature workflow
- **Source:** `workflows/feature.lobster` — orchestrator reconstructs from STATE.md, git worktrees, open PRs, factory-artifacts branch, regression baseline SHA.

### NFR-REL-013: Tier execution awaits all in-tier plugins before next tier
- **Source:** `executor.rs::execute_tiers` for-loop + per-tier await.

### NFR-REL-014: spawn_blocking JoinError treated as a plugin crash (dispatcher stays up)
- **Source:** `executor.rs::execute_tier` JoinError fallback constructs `PluginResult::Crashed`.

### NFR-REL-015: Generated registry's idempotence prevents drift between hooks.json and hooks-registry.toml
- **Source:** `scripts/generate-registry-from-hooks-json.sh` documented as idempotent. Re-running on a hand-edited registry reverts changes silently.

## 5. Scalability

### NFR-SCALE-001: Parallel within priority tier; sequential between tiers
- **Source:** Design Q3 + `executor.rs`.
- **Implication:** Wall-clock budget per dispatch ≈ sum of (max-tier-elapsed-ms) + tier-switch overhead, NOT sum of all per-plugin timeouts.

### NFR-SCALE-002: Dispatcher process is single-shot, short-lived
- **Source:** `main.rs` exits per hook event.
- **Implication:** No global state; trivial horizontal isolation between concurrent dispatcher invocations.

### NFR-SCALE-003: Per-sink dedicated OS thread + bounded queue
- **Source:** `sink-file` and `sink-otel-grpc` each spawn their own current_thread tokio runtime on a dedicated thread.
- **Rationale (commented):** "S-1.6 will introduce a dispatcher-wide tokio runtime that all sinks can share. Until then ... fully self-contained, no runtime leakage into the dispatcher."
- **Future:** S-1.6 promised a shared runtime; current state has per-sink threads. **DRIFT:** Sink module says "swap to a shared `Handle` as a one-line edit once S-1.6 lands"; S-1.6 has shipped (`Tier B.x` complete) but the shared-runtime conversion has not.

### NFR-SCALE-004: Binary distribution sized at ~3GB ceiling over first 3 years
- **Source:** Design doc Open Question 1 ("Binary commit strategy"). Estimates ~3GB ceiling.
- **Mitigation if hit:** "migration to an orphan branch (pattern similar to `factory-artifacts`) is mechanical."

### NFR-SCALE-005: Per-event payload bounded
- **Source:** STDERR_CAP_BYTES = 4096; sink event field bag (no nested structures expected); legacy adapter MAX_OUTPUT_BYTES = 1MiB.

### NFR-SCALE-006: Multi-instance sinks supported (e.g., two Datadog sinks for two orgs)
- **Source:** Design doc + array-of-tables `[[sinks]]`. Each stanza is a separate driver instance.

## 6. Maintainability

### NFR-MAINT-001: Workspace dependency pinning
- **Source:** `[workspace.dependencies]` block in root Cargo.toml. Member crates use `serde.workspace = true` etc. so updates land in one place.

### NFR-MAINT-002: opentelemetry-* crates pinned in lockstep at 0.31
- **Source:** Cargo.toml comment: "The opentelemetry crate ecosystem moves in lockstep at 0.31 as of April 2026; pin all four together so a cargo-update on one doesn't break the others."

### NFR-MAINT-003: Edition 2024 + rust-version 1.95
- **Source:** `[workspace.package] edition = "2024", rust-version = "1.95"`. `rust-toolchain.toml` pins.

### NFR-MAINT-004: Schema versioning at every config boundary
- **Source:** `REGISTRY_SCHEMA_VERSION = 1`, `observability-config schema_version = 1`, `INTERNAL_EVENT_SCHEMA_VERSION = 1`, `HOST_ABI_VERSION = 1`.

### NFR-MAINT-005: deny_unknown_fields on all serde-derived configs
- **Source:** `Registry`, `RegistryEntry`, `Capabilities`, `RegistryDefaults`, `ExecSubprocessCaps`, `ReadFileCaps` all carry `#[serde(deny_unknown_fields)]`.
- **Implication:** Typos fail loudly at parse time.

### NFR-MAINT-006: thiserror for crate-level errors; anyhow at boundaries
- **Source:** Pattern across all Rust crates. Convention documented (implicitly) by widespread use.

### NFR-MAINT-007: Test coverage observed at 180 Rust tests + ~1262 bats
- **Source:** Recount. CHANGELOG v1.0.0-beta.1 lists "87 dispatcher lib + 14 adapter + 18 host integration + 20 sdk + 17 sink + 13 macros + 11 regression-v1.0 bats + 6 generate-registry bats + 1245 baseline bats."

### NFR-MAINT-008: Feature flags / gating absent — no `#[cfg(feature = ...)]` guards
- **Source:** Sampling across crates. Simplifies build matrix.

## 7. Compatibility

### NFR-COMPAT-001: 5-platform support: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64
- **Source:** `ci/platforms.yaml` (per CHANGELOG v1.0.0-beta.1).

### NFR-COMPAT-002: HOST_ABI_VERSION = 1 frozen at v1.0
- **Source:** Design semver promise + lib.rs constants. Breaking ABI = major bump on dispatcher AND SDK.

### NFR-COMPAT-003: SDK accepts both `event_name` and `hook_event_name` envelope spellings
- **Source:** payload.rs serde alias.
- **Note:** SDK's plugin-facing payload uses canonical `event_name` only.

### NFR-COMPAT-004: Activation skill warns on cross-host re-activation drift
- **Source:** activate skill step 4. Persisted `activated_platform` mismatch warning.

### NFR-COMPAT-005: Windows native-WASM hooks work without git-bash
- **Source:** Design doc + CHANGELOG. Windows operators get native support for the 4 ported hooks; remaining 26 hooks need git-bash.

## 8. Auditability

### NFR-AUDIT-001: Every capability denial recorded with function + reason + details
- **Source:** `host/exec_subprocess.rs::emit_denial` and analogues across host fns.

### NFR-AUDIT-002: dispatcher_trace_id propagated through every event
- **Source:** main.rs + executor + emit_event host fn.

### NFR-AUDIT-003: factory-events-*.jsonl + dispatcher-internal-*.jsonl as durable audit trail
- **Source:** CHANGELOG references.

### NFR-AUDIT-004: bin/emit-event normalizes bash-side event emission
- **Source:** Used across all 44 bash hooks consistently.

## 9. Configuration values index (canonical)

| Knob | Default | Source | Override path |
|---|---|---|---|
| EPOCH_TICK_MS | 10 | engine.rs | (compile-time const) |
| timeout_ms | 5_000 | RegistryDefaults | per-entry `timeout_ms` |
| fuel_cap | 10_000_000 | RegistryDefaults | per-entry `fuel_cap` |
| on_error | Continue | RegistryDefaults | per-entry `on_error` |
| priority | 500 | RegistryDefaults | per-entry `priority` |
| STDERR_CAP_BYTES | 4096 | invoke.rs | (compile-time const) |
| DEFAULT_RETENTION_DAYS | 30 | internal_log.rs | (compile-time const) |
| DEFAULT_QUEUE_DEPTH (file) | 1000 | sink-file | per-sink `queue_depth` |
| DEFAULT_QUEUE_DEPTH (otel) | 1000 | sink-otel-grpc | per-sink `queue_depth` |
| DEFAULT_BATCH_SIZE (otel) | 100 | sink-otel-grpc | per-sink `batch_size` |
| DEFAULT_ENDPOINT (otel) | http://localhost:4317 | sink-otel-grpc | per-sink `endpoint` |
| MAX_OUTPUT_BYTES (legacy adapter) | 1 MiB | legacy-bash-adapter | (compile-time const) |
| BASH_TIMEOUT_MS | 60_000 | legacy-bash-adapter | (compile-time const) |
| Per-platform hook timeout (template) | 10_000 ms | hooks.json.template | regenerate template |
| Per-bash-hook timeout (legacy hooks.json) | 5–10 s | hooks/hooks.json | per-hook |
| HOST_ABI_VERSION | 1 | factory-dispatcher::lib.rs + hook-sdk::lib.rs | (major-version event) |
| REGISTRY_SCHEMA_VERSION | 1 | registry.rs | (major schema event) |
| INTERNAL_EVENT_SCHEMA_VERSION | 1 | internal_log.rs | (major schema event) |
| Cost monitoring warn / pause threshold | 0.70 / 0.95 | workflows/feature.lobster | per-workflow |

## 10. Missing NFRs (declared in design but not yet shipped)

| Missing NFR | Where declared | Currently |
|---|---|---|
| Retry with exp backoff + jitter | design `[sinks.*.retries]` block | not implemented (S-4.4) |
| Circuit breaker | design `[sinks.*.circuit_breaker]` block | not implemented (S-4.4) |
| Dead letter queue | design `[sinks.*.dead_letter]` block | not implemented (S-4.5) |
| Sink batching | design `[sinks.*.batch]` block | sink-otel-grpc has it; sink-file does not need it |
| HTTP / Datadog / Honeycomb sinks | design + Tier E stories | not implemented |
| read_file via StoreData-typed linker | design + host/read_file.rs | StoreData wrapper returns CAPABILITY_DENIED stub (drift) |
| `internal.sink_error` event emission | declared constant + design | constant defined but not emitted (drift) |
| `internal.sink_circuit_*` events | declared constants + design | constants defined but never emitted (drift) |
| Plugin signing / marketplace discovery | design non-goals | explicitly out of scope for v1.0 |
| seccomp / AppArmor / Landlock | design non-goals | explicitly out of scope (v2+) |
| WASI preview-2 | design non-goals | explicitly v2.0 |

## State Checkpoint
```yaml
pass: 4
status: complete
nfrs_cataloged: 76
categories: 8 (Performance, Security, Observability, Reliability, Scalability, Maintainability, Compatibility, Auditability)
config_values_indexed: 19
missing_nfrs_flagged: 11
timestamp: 2026-04-25
next_pass: 5
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **Novelty score** | SUBSTANTIVE — first NFR catalog for this repo |
| **Trajectory** | First sweep. 76 NFRs across 8 categories with citation per item. 19 canonical config knobs indexed. 11 design-declared NFRs flagged as not-yet-shipped. No prior pass-4 artifact existed. |
| **Verdict** | FINDINGS_REMAIN — protect-secrets details, validate-* hook NFRs, tools/observability collector config not deeply analyzed. CONVERGENCE_REACHED is not declarable until Phase B convergence loops complete. |
