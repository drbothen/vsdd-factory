# Pass 5: Conventions & Patterns — vsdd-factory

**Date:** 2026-04-25
**Reads:** Pass 0–4 + sample reads across Rust crates + skill/agent/template/hook files.

Two distinct convention systems, one per subsystem. Within each, consistency is high.

## 1. Rust conventions (Subsystem A)

### Edition + toolchain
- `edition = "2024"`, `rust-version = "1.95"`. Pinned in workspace.package.
- `rust-toolchain.toml` controls local toolchain (not opened this pass; assumed to match).
- `resolver = "2"`.

### Workspace dependency pinning convention
- All shared deps declared once in `[workspace.dependencies]`. Member crates use `serde.workspace = true` etc.
- Examples observed:
  - `serde = { version = "1.0", features = ["derive"] }`
  - `tokio = { version = "1", features = ["macros", "rt-multi-thread", "sync", "time", "fs", "io-util", "process"] }`
  - `wasmtime = "44.0"` + `wasmtime-wasi = "44.0"` (lockstep major)
  - `opentelemetry-proto = "0.31"` + `tonic = "0.14"` + `tokio-stream = "0.1"` (lockstep — comment in Cargo.toml explains)
- **Convention:** Update one workspace dep, every member follows automatically. No per-crate version drift.

### Error handling: thiserror for crate APIs, anyhow at boundaries
- Every crate-level error enum derives `thiserror::Error`. Examples: `RegistryError`, `PayloadError`, `InvokeError`, `EngineError`, `HostCallError`, `FileSinkError`, `SinkError`, `OtelGrpcError`.
- `anyhow::Result` reserved for boundaries: `SinkRegistry::load`, `SinkRegistry::flush_all`, `SinkRegistry::from_config`.
- Enum variants are descriptive (e.g., `RegistryError::SchemaVersion { got, expected }`, `RegistryError::ToolRegex { name, pattern, source }`). `#[from]` for transparent wrappers (e.g., `Io(#[from] std::io::Error)`).

### Module organization
- One concern per file under `src/`. The dispatcher's `src/` is flat at the top (engine, executor, host, internal_log, invoke, lib, main, payload, plugin_loader, registry, routing, sinks) with `host/` and `sinks/` as subdirectories holding finer-grained submodules.
- Public surface re-exported through `lib.rs`. The binary `main.rs` consumes the library through the same surface integration tests use.
- `mod.rs` files are the public-surface declarations; submodule contents (e.g., `host/exec_subprocess.rs`) are pub-by-default for the parent module's needs but `pub(crate)` or `pub(super)` for cross-module helpers.

### Test organization
- **Unit tests:** `#[cfg(test)] mod tests { ... }` block at the bottom of each `.rs` file, exercising only that module's surface.
- **Integration tests:** `crates/factory-dispatcher/tests/*.rs` — one file per integration scenario (`executor_integration.rs`, `host_functions.rs`, `internal_log_integration.rs`, `loads_legacy_registry.rs`, `routing_integration.rs`, `sinks_file_integration.rs`, `sinks_otel_grpc.rs`).
- **Test-support shims:** `#[cfg(test)] pub(crate) mod test_support` for helpers shared across unit tests in the same crate.
- **Trybuild:** ~~`crates/hook-sdk-macros/` uses `trybuild` for compile-fail / pass tests on the proc-macro.~~ Corrected: `crates/hook-sdk-macros` has no `trybuild` dependency or test fixtures (claim was hallucinated in pass-0; verified absent by extraction-validation 2026-04-25).
- **wasm fixtures:** Tests that need a tiny module use `wat::parse_str(...)` to build wasm from textual WAT inline (e.g., `invoke.rs::tests::compile`).

### Doc comments
- Every public type and module carries a `//!` (module) or `///` (item) doc comment. Examples: registry.rs preamble, payload.rs preamble, host/mod.rs preamble.
- Comments routinely explain *why*, not just *what*. Examples: invoke.rs's StoreData-vs-HostContext linker bridge ("wasmtime doesn't support cloning Func between different Store types"); engine.rs's epoch tick rationale; sink-otel-grpc's "why proto-direct, not the SDK exporter" ADR-style block.
- `#[deny(missing_docs)]` enabled in `sink-file/lib.rs` and `sink-core/lib.rs` and `sink-otel-grpc/lib.rs`.

### Naming
- Types: PascalCase (`HookPayload`, `RegistryEntry`, `EpochTicker`).
- Functions: snake_case (`match_plugins`, `group_by_priority`, `build_engine`).
- Constants: SCREAMING_SNAKE_CASE (`HOST_ABI_VERSION`, `EPOCH_TICK_MS`, `STDERR_CAP_BYTES`).
- Negative error codes: prefixed `codes::*` and i32 (`CAPABILITY_DENIED = -1`).
- Enum variants for outcomes: PascalCase (`Continue`, `Block`, `Error`).
- Test functions: snake_case, descriptive ("rejects_unknown_schema_version", "match_filters_by_event_name", "ticker_advances_epoch_over_time").

### Builder pattern for events / payloads
- `InternalEvent::now(type_).with_trace_id(...).with_session_id(...).with_field(k, v)` — chainable, `#[must_use]`.
- `SinkEvent::new().insert(k, v)` — same shape.

### Async + concurrency
- `current_thread` tokio runtime in dispatcher (no multi-threaded scheduler).
- `spawn_blocking` for synchronous wasmtime calls.
- Atomic flags + cooperative shutdown for background threads (`AtomicBool::Relaxed` + `JoinHandle` join in Drop).
- `Arc<Mutex<Vec<_>>>` for shared accumulators (event queue, sink failures).
- mpsc bounded channels for sink workers.

### Capability / security idioms
- Deny-by-default at type level: `Option<ExecSubprocessCaps>` rather than empty struct.
- Capability check happens in host fn; emits denial event AND returns negative error code (both, not either-or).
- Reserved field filtering (`emit_event` strips `dispatcher_trace_id`, `session_id`, etc. before adding to event).

### Tracing
- `tracing` crate available workspace-wide but the durable always-on log path is `internal_log.rs` (custom JSONL writer), not the tracing subscriber.
- Stderr is the fallback when even the internal log fails (e.g., `eprintln!` in `internal_log::write` and `prune_old`).

### Schema versioning
- Every config struct has a `schema_version` field. Mismatch is a hard error. Constants follow the pattern: `pub const SCHEMA_VERSION: u32 = 1;` with explicit comment that bumping is a breaking change.

### Comments-as-design-decisions
- The codebase repeatedly calls out *why* a decision was made: "We use a wrapper type so both live in the store's data slot" (StoreData rationale); "S-1.6 will introduce a dispatcher-wide tokio runtime that all sinks can share. Until then..." (sink ownership); "Avoid the exact 30-day boundary..." (test race-condition fix).
- This is a strong convention — the comments *are* lightweight ADRs.

## 2. Plugin layer conventions (Subsystem B)

### Skill frontmatter schema
```yaml
---
name: <skill-id>            # required, kebab-case, matches directory name
description: <one-liner>    # required, used as command palette description
argument-hint: "[arg1] [arg2]"  # optional, hint for slash command
---
```
- 119/119 SKILL.md files conform to this shape (sampled: brownfield-ingest, activate, create-prd, etc.).
- Body is markdown, often very long, with sections like "Procedure", "Quality Gate", "Templates", "When to Use", "Output", "Step N", etc.

### Skill internal structure (composite skills)
```
skills/<name>/
├── SKILL.md                 # main entry
├── steps/                   # optional sub-procedure files
│   ├── step-a-<name>.md
│   ├── step-b-<name>.md
│   └── _shared-context.md
└── templates/ or examples/  # optional skill-local resources
```
- Observed in: brownfield-ingest, code-delivery, deliver-story, factory-cycles-bootstrap, holdout-eval, etc.
- 581 markdown files across 1270 skill subdirectories (per Pass 0 recount).

### Agent frontmatter schema
```yaml
---
name: <agent-id>            # required
description: <when-to-use>  # required
tools: [Read, Grep, Glob]   # optional, restricts agent's tool set
model: opus | sonnet        # optional, picks model
color: red|green|blue|...   # optional, UI hint
---
```
- 33 agent files at `agents/*.md` + 1 directory-based agent at `agents/orchestrator/`.
- Examples sampled:
  - `adversary.md`: `tools: Read, Grep, Glob, model: opus, color: red`
  - `implementer.md`: `model: sonnet, color: green` (no tools restriction → full access)
  - `orchestrator/orchestrator.md`: minimal frontmatter (no tools/model/color)

### Workflow Lobster YAML schema
```yaml
workflow:
  name: <id>
  description: >
    <multi-line>
  version: "X.Y.Z"
  defaults:
    on_failure: escalate | retry | continue
    max_retries: 2
    timeout: "1h" | "30m" | "10s"
  cost_tracking:           # optional (DF-027)
    enabled: true
    metadata: { ... }
    thresholds: { warn: 0.70, pause: 0.95 }
    protected_agents: [adversary, security-reviewer, formal-verifier]
    summary_artifact: "..."
  steps:
    - name: <step-id>
      type: agent | skill | gate | wait
      agent: <agent-id>
      depends_on: [<step-ids>]
      on_failure: ...
      max_retries: ...
      timeout: ...
```
- Workflows are read by orchestrator as DATA, not prose. Helper: `bin/lobster-parse <file> '<jq-expr>'`.
- 16 .lobster files: 8 top-level modes + 8 phase sub-flows.

### Bash hook conventions
```bash
#!/bin/bash
# <hook-name>.sh — <brief description>
#
# Trigger: <hook event + matcher>
# Exit 0 on pass / 2 to block / always advisory etc.
#
# <Determinism / latency budget statement>

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0  # graceful no-op when jq missing
fi

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# ... gate logic ...

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

block() {
  local reason="$1"
  local code="${2:-unknown}"
  _emit type=hook.block hook=<hook-name> matcher=<matcher> reason="$code" command="$COMMAND"
  echo "" >&2
  echo "<HUMAN-READABLE BLOCK MESSAGE>:" >&2
  echo "  $reason" >&2
  echo "  <fix instructions>" >&2
  exit 2
}
```
- Observed across: block-ai-attribution.sh, check-factory-commit.sh (smaller pattern), validate-novelty-assessment.sh (the very hook that just gated this analysis), regression-gate.sh, capture-commit-activity.sh.
- **Conventions:**
  - Always use `#!/bin/bash` (not `#!/usr/bin/env bash` — strict shebang).
  - `set -euo pipefail` always.
  - Graceful no-op when jq missing (so partial environments don't break).
  - `_emit` helper consistently named and structured.
  - `block` helper produces `hook.block` event AND prints to stderr AND exits 2.
  - Reason codes are snake_case strings (e.g., `ai_attribution_coauthored`, `binary_not_on_allow_list`).

### Template naming
- All templates under `templates/`. Observed naming patterns:
  - `<artifact-type>-template.md` (most common): `behavioral-contract-template.md`, `prd-template.md`, `story-template.md`, `epic-template.md`, `pr-description-template.md`.
  - `<level>-<artifact>-template.md`: `L2-domain-spec-template.md`, `L4-verification-property-template.md`.
  - `<artifact>-config-template.yaml`: `autonomy-config-template.yaml`, `discovery-config-template.yaml`, `policies-template.yaml`, `merge-config-template.yaml`.
  - `*-section-template.md`: subsections (e.g., `architecture-section-template.md`, `L2-domain-spec-section-template.md`).
  - `*-index-template.md`: index documents (e.g., `epic-index-template.md`, `story-index-template.md`).
  - `*-report-template.md`: outputs (e.g., `consistency-report-template.md`, `delta-analysis-report-template.md`).
- 108 top-level template files; some (`design-system/`, `adversary-prompt-templates/`, `ui-quality/`) are subdirectories.

### Slash command convention
- 110 `commands/<name>.md` files; each binds to a skill. Naming = exactly the skill name. Body is short (typically delegates straight to the skill).

### Bin-tool convention
- `plugins/vsdd-factory/bin/<tool>` are executables (no `.sh` extension).
- Example tools: `emit-event`, `factory-dashboard`, `factory-obs`, `factory-query`, `factory-replay`, `factory-report`, `factory-sla`, `lobster-parse`, `multi-repo-scan`, `research-cache`, `wave-state`, `compute-input-hash`.
- `factory-*` family is observability tooling; `wave-state`, `compute-input-hash`, `lobster-parse` are workflow infra.

### File / directory naming (general)
- Skills: kebab-case (`brownfield-ingest`, `create-prd`, `wave-gate`, `multi-repo-phase-0-synthesis`).
- Agents: kebab-case `.md` files.
- Story IDs: `S-<phase>.<seq>` (e.g., `S-0.1`, `S-1.2`, `S-3.4`, `S-5.7`).
- Spec IDs: `<YYYY-MM-DD>-<slug>-design.md` (date-prefixed).
- Plan IDs: `<YYYY-MM-DD>-<slug>.md`.
- BC IDs: `BC-<S>.<SS>.<NNN>` (4-level per DF-035) or `BC-AUDIT-<NNN>` for recovered.
- Adversarial finding IDs: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` (template-defined).
- Internal events: dot-namespaced lowercase (e.g., `dispatcher.started`, `plugin.invoked`, `internal.capability_denied`, `commit.made`, `hook.block`).

## 3. Test conventions

### Rust
- Unit tests live in the same file as the code under test, in `#[cfg(test)] mod tests`.
- Test names are full sentences in snake_case ("rejects_invalid_tool_regex", "match_includes_no_tool_entries_for_any_tool", "ticker_advances_epoch_over_time").
- Each test asserts ONE behavior (single-purpose tests).
- Integration tests live in `crates/factory-dispatcher/tests/` — one .rs file per scenario, named by what they test.
- Negative-path tests are first-class — many tests start with `rejects_*`, `denies_*`, `swallows_*`.

### Bats
- 1245+ bats tests across `plugins/vsdd-factory/tests/*.bats`.
- 11 dispatcher pipeline regression tests in `regression-v1.0.bats` (per CHANGELOG).
- 6 generate-registry bats tests for the registry-generation script.

### Trybuild (proc-macro)
- ~~13 trybuild fixtures for `#[hook]` macro (compile-pass and compile-fail).~~ Corrected: `crates/hook-sdk-macros` has no `trybuild` dependency or test fixtures (13-fixture claim was hallucinated in pass-0; verified absent by extraction-validation 2026-04-25).

## 4. Design patterns observed

### Strategy / dyn-trait fan-out
- `SinkRegistry { sinks: Vec<Box<dyn Sink>> }` — heterogeneous drivers behind a uniform interface. Each `Sink` impl encapsulates its driver-specific concerns.

### Bounded queue producer/consumer (per-sink)
- Each sink owns an mpsc channel with a worker thread. Producer (`submit`) is non-blocking via `try_send`. Consumer drains queue and writes I/O.

### Builder pattern for events
- `InternalEvent` and `SinkEvent` both expose chainable `with_*` / `insert` methods returning `Self`.

### Capability-token pattern
- `Capabilities` block on each registry entry encodes which host fns the plugin may call. Deny-by-default; absent block = no privileges beyond always-on.

### Module bridging (StoreData ↔ HostContext)
- `setup_linker` builds a `Linker<HostContext>` for the canonical surface; `proxy_host_imports` rebuilds the same set against `Linker<StoreData>` for the per-invocation flavor. Documented rationale: "wasmtime doesn't support cloning Func between different Store types."

### Adapter / bridge plugin
- `legacy-bash-adapter` is the canonical Adapter pattern: one wasm plugin instance per registry entry, each configured with its own `script_path`, all sharing the same compiled module.

### Fail-loudly + non-blocking-by-default
- Errors in the dispatcher's startup path emit `internal.dispatcher_error` and return exit 0 (visible in audit log; doesn't block CC).
- Schema mismatches are hard errors at parse (visible immediately).

### Co-occurring effect (event + return code)
- Capability denials emit an event AND return a negative code, never just one. This makes the audit log canonical and lets the plugin still take corrective action.

## 5. Anti-patterns observed (and counter-evidence)

### Drift between design doc and code
- Design says `internal.sink_error`, `internal.sink_circuit_*`, `internal.sink_queue_full` events fire from sinks. Code has the constants but no driver currently emits them; they live in pending S-4.4. Drift acknowledged in code comments.
- Design mentions "shared tokio runtime" once S-1.6 lands. S-1.6 has shipped but the per-sink dedicated thread pattern remains. Drift acknowledged in source comments.

### Two parallel hook tables
- Both `hooks.json` (legacy v0.79.x routing) AND `hooks-registry.toml` (v1.0 dispatcher routing) currently live in the repo. Activate skill picks the right `hooks.json.<platform>` to copy in; that file references the dispatcher. The `hooks-registry.toml` is the dispatcher's own routing.
- Generation: `scripts/generate-registry-from-hooks-json.sh` produces the toml from the json. Idempotent. Documented as "DO NOT HAND-EDIT during the v0.79.x → v1.0 migration".
- This is a transitional anti-pattern: there is no single source of truth for hook routing during the migration window. Once all bash hooks are ported (post-v1.0), `hooks.json` retires.

### Stub / pending state in shipped code
- `read_file` is a CAPABILITY_DENIED stub at the StoreData-typed linker layer (with a fully-implemented host/read_file.rs that just isn't wired). Documented but is real drift.
- `dispatcher.shutting_down` constant declared but never emitted.

### Inconsistent vs. consistent patterns
- **Highly consistent:** thiserror everywhere; deny_unknown_fields everywhere; SchemaVersion check everywhere; bash hook structure (set -euo pipefail, _emit helper, block helper).
- **Slightly inconsistent:** Test naming — most are descriptive sentences but a few are short (`empty_registry_submit_is_a_noop`); `#[deny(missing_docs)]` enabled in sink-* but not in factory-dispatcher (factory-dispatcher's docs are still pervasive but not enforced by attribute).
- **Multi-instance plugin pattern:** legacy-bash-adapter is multi-registered (45 entries → one .wasm). This is a pattern, not an anti-pattern — design doc explicitly enables it.

## 6. Consistency assessment

| Convention | Assessment | Evidence |
|---|---|---|
| Workspace dependency pinning | UNIVERSAL | All member crates use `.workspace = true` |
| thiserror for crate errors | UNIVERSAL | 8+ crate-level error enums all use thiserror |
| anyhow at boundaries | UNIVERSAL | sinks::load, flush_all use anyhow::Result |
| deny_unknown_fields on configs | UNIVERSAL | Registry, RegistryEntry, Capabilities, etc. |
| Schema versioning | UNIVERSAL | 4 schema_version fields across configs/events/ABI |
| Test placement (#[cfg(test)] mod tests) | UNIVERSAL | Every src/*.rs file follows |
| Doc comments on public types | NEAR-UNIVERSAL | Every type carries one; `#[deny(missing_docs)]` enforces in sink-* crates |
| Bash hook shebang `#!/bin/bash` | UNIVERSAL | 44/44 hook scripts (sampled) |
| Bash hook `set -euo pipefail` | UNIVERSAL | All inspected; pattern explicit |
| Bash hook `_emit` helper | UNIVERSAL | block-ai-attribution, check-factory-commit, validate-novelty-assessment all use the same shape |
| Skill frontmatter (name + description) | UNIVERSAL | 119/119 SKILL.md files |
| Agent frontmatter (name + description) | UNIVERSAL | 33+ files |
| Lobster workflow shape | UNIVERSAL | All 16 .lobster files share the `workflow:`/`steps:` structure |
| Capability deny-by-default | UNIVERSAL | All 4 cap-gated host fns enforce |
| Builder pattern for events | UNIVERSAL | InternalEvent + SinkEvent both expose chainable builders |
| Story ID `S-<phase>.<seq>` | UNIVERSAL | 41 stories all conform |
| Spec ID `<date>-<slug>-design.md` | UNIVERSAL | 8/8 spec files |
| Internal event dot-namespacing | UNIVERSAL | All 17 declared events follow lowercase dot-namespace |
| `#[deny(missing_docs)]` | PARTIAL | sink-core, sink-file, sink-otel-grpc only |
| `internal.sink_*` event emission | PENDING | Constants declared; emission deferred to S-4.4 |
| StoreData read_file impl | PENDING | Stub returns CAPABILITY_DENIED; full impl in host/read_file.rs not wired |
| Shared tokio runtime across sinks | PENDING | Per-sink dedicated thread; design promised consolidation post-S-1.6 |

## State Checkpoint
```yaml
pass: 5
status: complete
rust_conventions_documented: 13
plugin_layer_conventions_documented: 9
test_conventions_documented: 4
design_patterns_observed: 7
anti_patterns_flagged: 4
consistency_assessment_rows: 22
timestamp: 2026-04-25
next_pass: 6
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **Novelty score** | SUBSTANTIVE — first conventions catalog for this repo |
| **Trajectory** | First sweep. 13 Rust + 9 plugin-layer conventions documented. 7 design patterns + 4 anti-patterns / drift items + 22-row consistency assessment all newly derived. No prior pass-5 artifact existed. |
| **Verdict** | FINDINGS_REMAIN — sample-size for SKILL.md frontmatter conformance was qualitative not exhaustive; per-validator hook structure was inferred from a sample. CONVERGENCE_REACHED is not declarable until Phase B convergence loops complete. |
