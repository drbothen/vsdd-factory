---
document_type: domain-spec-section
level: L2
section: glossary
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
input-hash: "7a51864"
traces_to: L2-INDEX.md
---

# Glossary

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §3 Ubiquitous language glossary (22 canonical terms).
> These definitions are authoritative for all agents reading this spec.
> Use these exact terms in BCs, stories, PRDs, and architecture docs.

## Subsystem A — Dispatcher Runtime Terms

**Dispatcher**
The compiled Rust binary `factory-dispatcher` that Claude Code invokes per hook event. Reads stdin JSON, loads `hooks-registry.toml`, routes to WASM plugins, writes telemetry, exits 0 or 2.

**Plugin** (Half A context)
A WASM module loaded by the dispatcher. Subject of a `RegistryEntry`. Built with the Hook SDK. Runs in a sandboxed wasmtime Store with bounded fuel and epoch timeout.

**Hook** (Half A context)
Synonymous with "registry entry" — the routing record in `hooks-registry.toml` that pairs an event type + tool regex with a plugin path and capability grant.

**Tier**
A priority cohort of registry entries that execute in parallel within a single dispatch. Tiers are ordered by ascending `priority` integer. Between tiers, execution is sequential. The primitive for expressing plugin ordering dependencies.

**HOST_ABI_VERSION**
The integer constant (currently `1`) shared between the dispatcher and the Hook SDK. A mismatch causes a hard plugin load failure. Breaking host ABI requires a major version bump on both dispatcher and SDK.

**dispatcher_trace_id**
A UUID v4 generated once per stdin envelope. Injected into the payload and carried by every emitted event. The correlation key for reconstructing all activity from a single hook invocation.

**block_intent**
Boolean aggregate across all plugins in a tier. Set to `true` if any plugin produced a `Block` result or crashed under `on_error = "block"`. When `true`, the dispatcher exits with code 2, signaling Claude Code to block the tool call.

**legacy-bash-adapter**
The migration-window WASM plugin that shells out to existing bash hook scripts via `exec_subprocess`. All 44 current hook scripts run through this adapter. Multi-instance: one registry entry per bash hook, each with its own `script_path` in `plugin_config`.

**shell_bypass_acknowledged**
A required justification string in the `exec_subprocess` capability entry when the target binary is a shell interpreter. Empty string or absence = refusal.

## Subsystem B — Orchestration Framework Terms

**Activate**
Per-project opt-in performed by `/vsdd-factory:activate`. Detects platform, writes `hooks.json.<platform>` over `hooks.json` (gitignored), verifies the dispatcher binary, and sets the orchestrator as the default agent. Required after every fresh install.

**Orchestrator**
The main-thread Claude Code agent set as default after activation. Reads `.lobster` workflow files as data and dispatches specialist sub-agents through pipeline phases. Does NOT produce artifacts directly.

**Skill**
A named procedure under `skills/<name>/SKILL.md`. Invokable as `/vsdd-factory:<name>`. Skills are the atomic units of orchestration behavior — each declares its steps, inputs, and outputs.

**Agent**
A specialist sub-persona under `agents/<name>.md` with a frontmatter-declared `name`, `description`, optional `tools`, and optional `model`. Spawned by the orchestrator via the Agent tool.

**Workflow**
A `.lobster` YAML file with `steps` declarations. The orchestrator reads workflow files as data (not prose) and drives agent dispatch accordingly. 8 mode workflows + 8 phase sub-flows.

**Story**
A decomposed unit of work identified `S-<phase>.<seq>` (e.g., S-1.2). Grounded in BCs. Tracked through the story lifecycle state machine: Defined → Ready → InProgress → RedGate → GreenGate → Refactor → Review → Merged → Demoed.

**Wave**
A grouped delivery batch of stories sharing the same dependency tier. Stories within a wave may execute in parallel; waves execute sequentially. Wave completion requires passing the wave gate.

**BC (Behavioral Contract)**
A formal, testable assertion about system behavior. Identified `BC-S.SS.NNN` (4-level hierarchy) once formally assigned, or `BC-AUDIT-NNN` for recovered/draft contracts. Every acceptance criterion in a story traces to one or more BCs.

**Tier (story tier)**
A dependency batch in the EPIC (A through H for v1.0). Tier A stories must ship before Tier B begins. Not to be confused with "plugin priority tier" in the dispatcher — both use the word but in different bounded contexts.

**Brownfield ingest**
The Phase 0 ingestion procedure that analyzes an existing codebase through multiple structured passes until novelty reaches NITPICK. Produces entity catalog, BC catalog, NFR catalog, and conventions.

**Semport**
"Semantic port" — the translation procedure for porting a component from one language/framework to another while preserving behavioral contracts. vsdd-factory's `semport-analyze` skill implements this.

**VSDD**
Verified Spec-Driven Development — the methodology this product implements. Key properties: brief → spec → PRD → architecture → stories → TDD delivery → adversarial review → formal verification → convergence.

**Wave gate**
The quality gate at the end of each wave. Validates story completeness (BC coverage, test coverage, demo evidence). Blocks the next wave until all criteria pass.

**Convergence**
The state reached when iterative deepening produces no new SUBSTANTIVE findings — only NITPICK-level observations. Phase 0 ingestion and adversarial review both use convergence as their terminal condition.

**Strict-binary novelty**
The assessment policy for convergence: every finding is either SUBSTANTIVE (blocks convergence, requires another pass) or NITPICK (does not block). No gradient or partial credit. Prevents premature convergence.

## Cross-Context Disambiguation

The term "Hook" is overloaded across the two subsystems:
- **Half A**: Hook = a `RegistryEntry` in `hooks-registry.toml` (the routing record)
- **Half B**: Hook = a bash script in `plugins/vsdd-factory/hooks/*.sh` (the behavior script)

The term "Plugin" is also overloaded:
- **Half A**: Plugin = a `.wasm` module loaded and sandboxed by the dispatcher
- **Half B**: Plugin = the Claude Code marketplace plugin package (`plugin.json`)

When a document uses these terms without qualification, context determines meaning. In cross-subsystem discussions, qualify: "WASM plugin" vs "marketplace plugin", "registry hook" vs "bash hook".
