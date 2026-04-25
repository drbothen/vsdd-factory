---
document_type: domain-spec-section
level: L2
section: business-rules
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

# Business Rules

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §2b Behavioral Extraction — Business rules.
> These 17 cross-cutting invariants are domain rules that must always hold.
> Each traces to one or more DI-NNN entries in invariants.md.

## Dispatcher Execution Rules

**BR-01 — Parallel within tier, sequential between tiers**
Plugins in the same priority tier run as parallel tokio tasks; tiers execute one at a time in ascending priority order. This is the primitive for expressing ordering dependencies.
Source: design doc Q3 resolution; `executor.rs`. Traces to: DI-001.

**BR-02 — Plugin failure does not block sibling plugins**
Each plugin runs in its own wasmtime Store with tokio panic isolation. A crash or timeout in one plugin does not prevent other plugins in the same tier from completing.
Source: design doc; `executor.rs`. Traces to: DI-002.

**BR-03 — `on_error = "block"` records intent without aborting the tier**
When a plugin's outcome is Block, `block_intent = true` is recorded and the final exit code becomes 2, but remaining plugins in the tier still run to completion.
Source: design doc Q3; `executor.rs::plugin_requests_block`. Traces to: DI-003.

**BR-04 — Capability denial returns negative code AND emits denial event**
Every capability-gated host function performs both actions atomically: returns `codes::CAPABILITY_DENIED (-1)` to the plugin AND emits `internal.capability_denied` to the internal log. Neither action alone is sufficient.
Source: design doc; `host/exec_subprocess.rs::emit_denial`. Traces to: DI-004.

**BR-05 — Shell interpreters require explicit `shell_bypass_acknowledged`**
`exec_subprocess` refuses bash, sh, zsh, pwsh, fish, csh, tcsh, ksh unless the capability entry contains a non-empty `shell_bypass_acknowledged` justification string.
Source: design doc Q4; `exec_subprocess.rs::is_shell + SHELL_NAMES`. Traces to: DI-005.

**BR-06 — Setuid/setgid binaries are refused categorically on Unix**
Regardless of the binary_allow list, any target binary with setuid or setgid bits set is refused with `codes::CAPABILITY_DENIED`. No override mechanism exists.
Source: design doc Q4; `exec_subprocess.rs::refuse_setuid`. Traces to: DI-006.

## Observability Rules

**BR-07 — Always-on dispatcher self-telemetry**
`dispatcher-internal-YYYY-MM-DD.jsonl` is written regardless of `observability-config.toml` contents. If all configured sinks are misconfigured or down, the dispatcher remains debuggable.
Source: design doc Q6 Option B; `internal_log.rs`. Traces to: DI-007.

**BR-08 — Daily rotation by event timestamp, not wall-clock "now"**
The JSONL filename date is derived from `event.ts[..10]`, not `SystemTime::now()`. This ensures test-injected timestamps produce deterministic file names.
Source: `internal_log.rs::date_stamp`. Traces to: DI-008.

**BR-09 — 30-day retention enforced at dispatcher start**
`prune_old(30)` is called during dispatcher startup. Log files older than 30 days are deleted regardless of sink config.
Source: `internal_log.rs::DEFAULT_RETENTION_DAYS`. Traces to: DI-009.

**BR-10 — stderr truncated to 4 KiB with explicit marker**
Plugin stderr is capped at `STDERR_CAP_BYTES = 4096` bytes. If truncated, the string `(stderr truncated)` is appended so consumers can detect the truncation boundary.
Source: `invoke.rs::STDERR_CAP_BYTES`. Traces to: DI-010.

## Sink Rules

**BR-11 — Sink submit must not block**
Every sink driver uses a bounded mpsc channel + `try_send`. A full queue causes the event to be dropped (recorded as SinkFailure), not a blocked caller. The dispatcher must never block on I/O.
Source: sink-core Sink trait doc; all drivers. Traces to: DI-011.

**BR-12 — Sink failure is isolated per driver**
A failed or misconfigured sink does not affect other sinks. Failures are recorded in the driver's `Mutex<Vec<SinkFailure>>`. S-4.4 will wire these to `internal.sink_error` events.
Source: design doc; `sinks/mod.rs`. Traces to: DI-012.

**BR-13 — Unknown sink driver type is warned and skipped**
If `observability-config.toml` references a driver type not yet implemented (e.g., `datadog`, `honeycomb`, `http`), the dispatcher logs a warning to stderr and continues. This is not a hard error.
Source: `sinks::from_config`. Traces to: DI-013.

## Configuration Rules

**BR-14 — Schema-version mismatch is a hard error**
`hooks-registry.toml`, `observability-config.toml`, and `InternalEvent` all carry `schema_version = 1`. Any mismatch causes the dispatcher to refuse to load and emit `internal.dispatcher_error`.
Source: `registry.rs::validate`; `sinks::from_config`; `internal_log.rs`. Traces to: DI-014.

**BR-15 — Activation is required after every fresh install**
The dispatcher binary path and `hooks.json` contents are platform-specific and gitignored. The `/vsdd-factory:activate` skill is the only mechanism that writes the correct values for the operator's host.
Source: CHANGELOG migration notes; `skills/activate/SKILL.md`. Traces to: DI-015.

## Plugin Isolation Rules

**BR-16 — Per-plugin `plugin_config` is spliced per invocation**
Even when multiple registry entries reference the same `.wasm` file (multi-instance pattern), each entry injects only its own `config` block as `plugin_config` into the payload. No cross-entry config leakage.
Source: `executor.rs`. Traces to: DI-016.

**BR-17 — `dispatcher_trace_id` propagates to every emitted event**
A UUID v4 generated once per stdin envelope is injected into the payload and carried by every subsequent internal event. The full causal chain for a single hook invocation is reconstructible by filtering on this ID.
Source: `main.rs` (inject) + `internal_log` + `emit_event` host fn. Traces to: DI-017.
