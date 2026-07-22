---
document_type: domain-spec-section
level: L2
section: core-domain-model
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
input-hash: "bdf3ae7"
traces_to: L2-INDEX.md
---

# Core Domain Model

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §2a Structural Extraction.

## Half A — Dispatcher Runtime Entities (Subsystem A, SS-01 through SS-04)

### Aggregate: Registry
Root aggregate for hook routing. Parsed from `hooks-registry.toml`.
- Fields: `schema_version: u32`, `defaults: RegistryDefaults`, `hooks: Vec<RegistryEntry>`
- Identity: the TOML file path; keyed by `schema_version == 1` (REGISTRY_SCHEMA_VERSION)
- Invariant: `schema_version` mismatch is a hard error. See DI-001.

### Entity: RegistryEntry ("Hook" in dispatcher context)
- Fields: `name`, `event`, `tool` (regex), `plugin` (path), `priority` (default 500), `enabled`, `timeout_ms` (default 5000), `fuel_cap` (default 10M), `on_error`, `capabilities`, `config`
- Identity: `name` is unique within the registry
- Owns: 0..1 `Capabilities`; references one `.wasm` file

### Value Object: Capabilities (deny-by-default)
Scoped to parent RegistryEntry. Sub-objects:
- `exec_subprocess`: `binary_allow`, `shell_bypass_acknowledged`, `cwd_allow`, `env_allow`
- `read_file`: `path_allow`
- `env_allow: Vec<String>`

### Entity: HookPayload
- Fields: `event_name` (alias `hook_event_name`), `tool_name`, `session_id`, `tool_input`, `tool_response`, `dispatcher_trace_id`, `plugin_config`
- Invariants: `event_name` non-empty; `session_id` non-empty; `dispatcher_trace_id` is UUID v4 per invocation

### Transient Aggregates (per-dispatch lifecycle)
- **Tier**: priority cohort of matched RegistryEntries for parallel execution
- **PluginResult**: `Ok | Timeout | Crashed` — stderr truncated to 4 KiB
- **PluginOutcome**: wraps PluginResult with `plugin_name`, `plugin_version`, `on_error`
- **TierExecutionSummary**: `per_plugin_results`, `total_elapsed_ms`, `block_intent`, `exit_code`
- **HostContext**: per-invocation context: capabilities, session_id, trace_id, events accumulator

### Entity: HookResult (plugin-emitted, SDK-defined)
- Sum type: `Continue` (exit 0) | `Block { reason }` (exit 2) | `Error { message }` (exit 1)

### Entity: InternalEvent
- Fields: `type_`, `ts`, `ts_epoch`, `schema_version (=1)`, `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `fields`
- 17 event-type constants (see domain-events.md for DE-NNN catalog)

### Entity: InternalLog (writer)
- Daily-rotated `dispatcher-internal-YYYY-MM-DD.jsonl`. 30-day retention. Always-on.

### Sink Entities (SS-03)
- **Sink** (trait): `name`, `accepts`, `submit` (non-blocking), `flush`, `shutdown`
- **SinkConfigCommon**: `name`, `enabled`, `routing_filter`, `tags`
- **RoutingFilter**: allow-then-deny on `event_types_allow` / `event_types_deny`
- **FileSink**: daily-rotated JSONL with mpsc-bounded queue (default depth 1000)
- **OtelGrpcSink**: OTLP/gRPC with dedicated OS thread + `current_thread` tokio
- **SinkRegistry**: fan-out holder (`Vec<Box<dyn Sink>>`); unknown driver types warn-and-skip
- **SinkFailure**: `path`, `reason`, `ts` — recorded in `Mutex<Vec<SinkFailure>>` per driver

### Plugin Execution Entities (SS-01, SS-02)
- **Engine + EpochTicker**: wasmtime Engine with epoch_interruption + consume_fuel; 10ms tick
- **PluginCache**: per-invocation Module cache keyed by absolute plugin path
- **SinkEvent**: flat `Map<String,Value>` projection of InternalEvent for sink fan-out

## Half B — Orchestration Framework Entities (Subsystem B, SS-05 through SS-10)

### Entity: Agent
- Frontmatter: `name`, `description`, optional `tools`, `model`, `color`
- 34 identities: orchestrator + 33 specialist sub-agents
- Orchestrator is the only agent set as default after activation

### Entity: Skill
- Format: `skills/<name>/SKILL.md` with frontmatter `name`, `description`, optional `argument-hint`
- 119 skills covering the full SDLC pipeline
- Invokable as `/vsdd-factory:<name>`

### Entity: Workflow (Lobster YAML)
- Format: `.lobster` files; schema: `workflow: { name, steps: [...] }`
- Step shape: `{ name, type, agent, depends_on?, on_failure?, max_retries?, timeout? }`
- 8 mode workflows + 8 phase sub-flows = 16 total

### Entity: Story
- Identity: `S-<phase>.<seq>` (e.g., S-1.2)
- Tier mapping: S-0.x=A, S-1.x=B, S-2.x=C/D, S-3.x/4.x=E, S-4.8=F, S-5.x=G, S-5.7=H

### Entity: BehavioralContract (BC)
- Identity: `BC-S.SS.NNN` (formal) or `BC-AUDIT-NNN` (recovered/draft)
- 1,851 BCs cataloged across 10 subsystems

### Other Half B Entities
- **Plugin** (marketplace): `plugin.json` — name, version, author, homepage
- **Marketplace**: `.claude-plugin/marketplace.json` — repo hosts its own marketplace
- **Template**: 108+ markdown/YAML skeletons referenced by skills
- **WorkflowState**: `.factory/STATE.md` — phase, wave, mode, blockers
- **HookScript** (bash): 44 `.sh` scripts — gates (PreToolUse), capture, validators, lifecycle
- **Epic**: `.factory/stories/v1.0/EPIC.md` — 41 stories with tier dependencies
- **SpecDoc**: `.factory/specs/<date>-<slug>-design.md` (strongly date-stamped)

## State Machines

Three state machines govern the domain lifecycle (from pass-2-domain-model.md §State machines):

1. **Plugin invocation outcome**: Loading → Compiling/Instantiating → Running → {OkExit0, OkExitN, TimeoutEpoch, TimeoutFuel, Crashed}
2. **Build → release → activate → dispatch**: Develop → CI_Test → Tag → ReleaseBuild → BinaryBundle → BotCommit → Marketplace → Install → Activate → Dispatch (recurring)
3. **Story lifecycle**: Defined → Ready → InProgress → RedGate → GreenGate → Refactor → Review → Merged → Demoed
