---
document_type: architecture-section
level: L3
section: "SS-10-cli-tools"
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

# SS-10: CLI Tools and Bin

## [Section Content]

## Purpose

The CLI Tools and Bin subsystem provides 12 project-local executable tools that
support observability, state management, workflow introspection, and event
infrastructure across the VSDD pipeline. These tools are the bash-layer primitives
that skills, hooks, and sub-agents call to interact with the factory's runtime
state. They are invokable directly from the command line and are referenced via
`${CLAUDE_PLUGIN_ROOT}/bin/<tool>` within bash hooks.

The subsystem also owns the `commands/` directory (110 slash-command binding files),
which provides the `/vsdd-factory:<name>` surface for every skill in SS-06. Command
files are thin: they name a skill and optionally describe the invocation argument.
They are the discovery layer — the mechanism by which users and Claude Code know
what skills are available without reading the `skills/` directory.

Together, the 12 bin tools and 110 command bindings form the CLI surface of
Subsystem B. The bin tools are infrastructure primitives; the commands are the
user-facing API. This subsystem is classified as supporting infrastructure rather
than a behavioral core — its contracts are well-bounded and largely operational.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/bin/emit-event` | Normalize and emit structured events to the dispatcher event stream; bash-side second-class emission path |
| `plugins/vsdd-factory/bin/wave-state` | Read/write wave gate state in `.factory/STATE.md`; used by wave-gate skill and SS-07 hooks |
| `plugins/vsdd-factory/bin/factory-dashboard` | Render factory pipeline status dashboard from STATE.md and event logs |
| `plugins/vsdd-factory/bin/factory-obs` | Observability query CLI: tail/filter `.factory/logs/*.jsonl` event files |
| `plugins/vsdd-factory/bin/factory-query` | Query event history with field filters and time range |
| `plugins/vsdd-factory/bin/factory-replay` | Replay recorded event sequences for debugging |
| `plugins/vsdd-factory/bin/factory-report` | Generate summary reports from event history (story throughput, gate pass rates) |
| `plugins/vsdd-factory/bin/factory-sla` | Check SLA compliance metrics from event history |
| `plugins/vsdd-factory/bin/lobster-parse` | Parse and validate `.lobster` workflow files; report DAG structure and step count |
| `plugins/vsdd-factory/bin/multi-repo-scan` | Scan multiple repos for factory state; used by multi-repo workflow |
| `plugins/vsdd-factory/bin/research-cache` | Manage the research cache (`.factory/research-cache/`); prune stale entries |
| `plugins/vsdd-factory/bin/compute-input-hash` | Compute SHA-256 input hash for drift detection; update `input-hash` frontmatter field |
| `plugins/vsdd-factory/commands/*.md` (110 files) | Slash-command binding files: `/vsdd-factory:<name>` → skill invocation |

## Public Interface

**`emit-event` CLI:**
```
bin/emit-event <event_type> [<json_fields>]
```
Normalizes the event, adds timestamp and session context, writes to the event
stream (currently via file append to `.factory/logs/events-<date>.jsonl`; partial
S-3.4 — does not route through dispatcher host fn).

**`wave-state` CLI:**
```
bin/wave-state get <gate_id>
bin/wave-state set <gate_id> <status>   # open | passed | failed
bin/wave-state list
```
Reads/writes STATE.md's wave gate section. Used by `wave-gate` skill and
`update-wave-state-on-merge.sh` hook.

**`factory-dashboard` CLI:**
```
bin/factory-dashboard [--project <dir>]
```
Renders a terminal dashboard of current phase, open wave gates, recent events,
and story queue from STATE.md + event logs.

**`lobster-parse` CLI:**
```
bin/lobster-parse <workflow.lobster>
```
Validates Lobster YAML schema, reports step count, dependency graph edges, and
any circular dependency errors.

**`compute-input-hash` CLI:**
```
bin/compute-input-hash <spec_file> [--update]
```
Computes SHA-256 of the spec file's declared `inputs:` list; with `--update`,
writes the hash into the file's `input-hash:` frontmatter field for drift
detection.

**Slash command binding format (`commands/<name>.md`):**
```markdown
---
name: deliver-story
skill: deliver-story
argument-hint: "<story-id>"
---
Invoke the deliver-story skill for the given story ID.
```

## Internal Structure

All 12 bin tools are standalone bash scripts (or lightweight Python scripts) with
no shared library. They follow the same conventions as bash hooks (SS-07):
`#!/bin/bash`, `set -euo pipefail`, graceful jq fallback for JSON parsing.

The `emit-event` tool is the most heavily used. It accepts an event type and
optional JSON field bag, adds metadata (`timestamp`, `session_id` from env,
`cwd`), and appends the JSON line to `.factory/logs/events-<date>.jsonl`. This
is the second-class emission path: bash hooks call it; native WASM plugins will
eventually call `vsdd::emit_event` host fn directly (S-3.4 PARTIAL).

`wave-state` reads and writes STATE.md using `yq` or inline bash `sed`/`awk`.
STATE.md is the single shared mutable state file for the pipeline; concurrent
writes are not protected (single-threaded assumption: only one Claude Code session
modifies STATE.md at a time in normal usage).

`factory-obs`, `factory-query`, `factory-replay`, `factory-report`, `factory-sla`
form the observability bin family. They all read `.factory/logs/*.jsonl` files.
`factory-obs` is a real-time tail with event-type filtering. `factory-query`
accepts structured filter expressions. `factory-replay` is used for debugging by
re-feeding recorded events through the dashboard rendering path.

110 command files are 1:1 with skills (110 of 119 skills have commands; 9 skills
are accessible by direct agent invocation only). Command files contain frontmatter
only plus a short description; the Claude Code harness reads them to populate the
`/vsdd-factory:*` autocomplete list.

## Dependencies

**Incoming (consumers of SS-10):**
- SS-06 (Skill Catalog) — skills call `bin/emit-event`, `bin/wave-state`,
  `bin/factory-dashboard`, and `bin/compute-input-hash` as part of their
  execution steps.
- SS-07 (Hook Bash Layer) — bash hooks call `bin/emit-event` for event emission
  and `bin/wave-state` for gate bookkeeping.
- SS-05 (Pipeline Orchestration) — orchestrator reads `wave-state` output;
  sub-agents use `factory-dashboard` for progress reporting.

**Outgoing (SS-10 depends on):**
- SS-05 (Pipeline Orchestration) — `wave-state` and `lobster-parse` read/write
  STATE.md and workflow files that are SS-05 artifacts. This is a read-only
  dependency: bin tools consume but do not define the STATE.md schema.

## Cross-Cutting

- **`emit-event` as emission bridge:** All bash-side events pass through this
  tool. It is the single normalization point for event type names, timestamps,
  and field schemas in the bash emission path. Keeping it consistent with the
  dispatcher's internal event schema (SS-01/SS-03) is a manual convention, not
  enforced by tooling (S-3.4 PARTIAL).
- **STATE.md single-writer assumption:** `wave-state` does not use file locking.
  Concurrent writes from multiple Claude Code sessions would corrupt STATE.md.
  This is a known limitation documented in STATE.md conventions; multi-session
  scenarios are not a supported use case for v1.0.
- **`compute-input-hash` and drift detection:** Spec files with `inputs:` + `input-hash:`
  frontmatter fields enable the `check-input-drift` skill to detect when upstream
  inputs have changed since the spec was last written. `compute-input-hash --update`
  is called by spec-writing skills after producing output.
- **Observability of the observability tools:** The bin tools do not emit events
  about their own execution (no self-telemetry). If `emit-event` fails, the
  failure is silent (error suppressed via `|| true` in bash hooks per
  SS-07 `_emit` helper convention).

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-10/`
(target prefix BC-10; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: `emit-event` normalization and emission contracts
(BC-10.001–BC-10.020), `wave-state` read/write and STATE.md mutation contracts
(BC-10.021–BC-10.040), `factory-obs/query/replay/report/sla` observability tool
contracts (BC-10.041–BC-10.090), `lobster-parse` validation contracts
(BC-10.091–BC-10.100), `multi-repo-scan` and `research-cache` operational
contracts (BC-10.101–BC-10.115), `compute-input-hash` drift detection contracts
(BC-10.116–BC-10.125), slash-command binding format contracts
(BC-10.126–BC-10.143).

## ADRs

No SS-10-specific ADRs. Bin tools and commands follow the Subsystem B conventions
established in the plugin layer. See ADR-012 for the legacy-bash-adapter pattern
that governs `emit-event`'s role as the current emission bridge.

## Drift / Known Issues

- **S-3.4 PARTIAL:** `emit-event` is a bash shell tool, not a typed Rust binary
  calling the `vsdd::emit_event` host fn. Two emission paths exist with different
  normalization. Reconciliation is planned under S-3.4 (Tier E).
- **L-P2-002 (debt):** 110 slash commands are a flat directory with no navigable
  index. There is no machine-readable catalog cross-referencing commands to skills,
  argument schemas, or BC IDs. A catalog index would improve discoverability.
- **L-P3-005 (known divergence):** Design ambition for `bin/emit-event` is a
  typed Rust binary calling the SDK host-fn shim directly. Current implementation
  is bash. This remains the planned end-state for S-3.4 but is not a regression.
- **STATE.md single-writer limitation:** No file locking on `wave-state` writes.
  Not a bug in single-session use; documented as an architectural constraint for
  v1.0.
