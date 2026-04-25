---
document_type: domain-spec-section
level: L2
section: edge-cases
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
input-hash: "08db1f1"
traces_to: L2-INDEX.md
---

# Edge Cases

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §2b + pass-8 drift/NFR/lesson catalog.
> DEC-NNN = domain-level edge cases — unusual but valid inputs or states
> the system must handle correctly.

## Dispatcher Edge Cases

**DEC-001 — Malformed or empty stdin envelope**
The dispatcher receives a stdin JSON that is missing required fields (`event_name`, `session_id`) or is not valid JSON.
Expected behavior: emit `internal.dispatcher_error`, exit 0 (non-blocking). Tool call proceeds.
Enforcement: SS-01 (`payload.rs::PayloadError` path in `main.rs`). Grounded in: pass-8 §NFR-REL-001.

**DEC-002 — Registry file missing or unparseable**
`hooks-registry.toml` does not exist, has a parse error, or has `schema_version != 1`.
Expected behavior: emit `internal.dispatcher_error`, exit 0 (non-blocking). No plugins fire.
Enforcement: SS-01 (`registry.rs::RegistryError` path). Grounded in: DI-014; pass-8 §NFR-REL-001.

**DEC-003 — Plugin `.wasm` file missing or compile-fails**
A registry entry references a `.wasm` path that does not exist or fails to compile under wasmtime.
Expected behavior: `PluginResult::Crashed` for that entry; `plugin.load_failed` emitted (declared, not yet wired — DRIFT-008); remaining entries in the tier continue.
Enforcement: SS-01 (`plugin_loader.rs`). Grounded in: DI-002; DRIFT-008.

**DEC-004 — Plugin exceeds fuel cap**
A plugin consumes more than `fuel_cap` (default 10,000,000) wasmtime fuel units.
Expected behavior: `PluginResult::Timeout { cause: Fuel }` with `plugin.timeout` event. Plugin is terminated; does not block.
Enforcement: SS-01 (`invoke.rs::FUEL`). Grounded in: pass-8 §NFR-PERF.

**DEC-005 — Plugin exceeds epoch deadline**
A plugin runs longer than `timeout_ms` (default 5,000ms) as measured by the epoch ticker (10ms resolution).
Expected behavior: `PluginResult::Timeout { cause: Epoch }` with `plugin.timeout` event.
Enforcement: SS-01 (`engine.rs::EpochTicker`). Grounded in: pass-8 §NFR-PERF.

**DEC-006 — Plugin traps on WASM runtime error**
A plugin causes a wasmtime trap (stack overflow, unreachable instruction, memory fault).
Expected behavior: `PluginResult::Crashed { trap_string }` with `plugin.crashed` event and captured stderr.
Enforcement: SS-01 (`invoke.rs` trap classification). Grounded in: DI-002.

**DEC-007 — `on_error = "block"` in a tier with multiple plugins, some crashing**
A plugin with `on_error = "block"` crashes while siblings complete normally.
Expected behavior: `block_intent = true` is set from the crashed entry's intent; exit code is 2 even though the plugin didn't produce a clean `Block` result. This is a current behavioral ambiguity.
Enforcement: SS-01 (`executor.rs`). Grounded in: DI-003; pass-8 §business rule BR-03.

**DEC-008 — Capability-denied exec_subprocess with shell interpreter**
Plugin attempts `exec_subprocess("bash", [...])` with no `shell_bypass_acknowledged` in its capabilities.
Expected behavior: `codes::CAPABILITY_DENIED` returned; `internal.capability_denied` emitted with `function = "exec_subprocess"` and `reason = "shell interpreter not acknowledged"`.
Enforcement: SS-01 (`host/exec_subprocess.rs::is_shell`). Grounded in: DI-005.

**DEC-009 — Setuid binary in exec_subprocess allow-list on Unix**
A `binary_allow` list includes `/usr/bin/sudo` or another setuid binary.
Expected behavior: refused unconditionally. The `binary_allow` list is not consulted for setuid check.
Enforcement: SS-01 (`host/exec_subprocess.rs::refuse_setuid`). Grounded in: DI-006.

## Sink Edge Cases

**DEC-010 — Sink queue full at high event rate**
A FileSink or OtelGrpcSink mpsc channel reaches `queue_depth` (default 1000) while the worker thread is lagging.
Expected behavior: `try_send` fails; event dropped; `SinkFailure` recorded in driver's `Mutex<Vec<SinkFailure>>`. No `internal.sink_queue_full` event emitted until S-4.4 lands (DRIFT-002).
Enforcement: SS-03. Grounded in: DI-011; DRIFT-002.

**DEC-011 — OTel gRPC endpoint unreachable**
`sink-otel-grpc` attempts to send a batch to `http://localhost:4317` and the connection is refused.
Expected behavior: batch is dropped or retried per current implementation (S-4.4 retry not yet shipped). No `internal.sink_error` event yet (DRIFT-002). Dispatcher continues.
Enforcement: SS-03 (`sink-otel-grpc::send`). Grounded in: DRIFT-002; DRIFT-005.

**DEC-012 — Unknown sink driver type in observability config**
`observability-config.toml` specifies `type = "datadog"` (not yet shipped).
Expected behavior: warn to stderr, skip that sink entry, load remaining sinks. Not a hard error.
Enforcement: SS-03 (`sinks::from_config::warn-and-skip`). Grounded in: DI-013; DRIFT-005.

## Configuration and Activation Edge Cases

**DEC-013 — Activation on an unsupported platform**
`/vsdd-factory:activate` is run on a platform not in the supported set (e.g., `linux-arm32`).
Expected behavior: the detect-platform.sh script fails with an explicit error; `hooks.json` is not written. The operator receives a clear error message.
Enforcement: SS-09 (`skills/activate/detect-platform.sh`). Grounded in: CAP-007.

**DEC-014 — Re-activation after platform binary update**
An operator upgrades the plugin and re-runs activate on the same machine.
Expected behavior: activate skill overwrites `hooks.json` with the new platform variant and verifies the updated binary. Previous hooks.json is silently replaced.
Enforcement: SS-09 (activate skill idempotency). Grounded in: DI-015.

**DEC-015 — Dual hook-routing table drift (DRIFT-004)**
`hooks.json` and `hooks-registry.toml` are out of sync (e.g., a hook was manually added to `hooks-registry.toml` but `generate-registry-from-hooks-json.sh` was not re-run).
Expected behavior: the dispatcher uses only `hooks-registry.toml`; `hooks.json` is used only by the Claude Code harness to invoke the dispatcher. Out-of-sync TOML entries may silently not fire.
Enforcement: SS-09 (generate script + documentation). Grounded in: DRIFT-004; pass-8 §L-P0-002.

## Orchestration Edge Cases

**DEC-016 — Brownfield ingestion of a self-referential repo**
The brownfield-ingest skill is run against vsdd-factory itself. Both subsystems must be analyzed without the skill confusing the engine (analyzer) with the product (the framework being analyzed).
Expected behavior: the skill treats both subsystems as first-class equals, producing two entity halves.
Enforcement: SS-06 (brownfield-ingest SKILL.md). Grounded in: pass-8 §L-P1-003.

**DEC-017 — Story delivery interrupted mid-wave (crash recovery)**
The implementer agent crashes between RedGate and GreenGate for a story.
Expected behavior: `.factory/STATE.md` records the last committed wave state; the orchestrator can re-dispatch the story from the beginning of its lifecycle state (InProgress), not from Defined.
Enforcement: SS-05 (crash recovery path). Grounded in: CAP-012; pass-8 §NFR-REL.

**DEC-018 — Adversary SHA-currency gate with stale commit**
The adversary's context window references an older commit SHA than `HEAD` because it was spawned before recent commits.
Expected behavior: `verify-sha-currency.sh` (opt-in, must be installed by operator) blocks the adversary until context is refreshed.
Enforcement: SS-06 (adversary skill + verify-sha-currency.sh template). Grounded in: DRIFT-009; pass-8 §ADR-013.
