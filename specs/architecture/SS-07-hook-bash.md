---
document_type: architecture-section
level: L3
section: "SS-07-hook-bash"
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

# SS-07: Hook Bash Layer

## [Section Content]

## Purpose

The Hook Bash Layer is the collection of 44 bash scripts that implement the actual
gate, capture, and lifecycle hook behaviors for the VSDD framework. These scripts
are invoked by `legacy-bash-adapter.wasm` (SS-04) via `exec_subprocess` when the
dispatcher routes a hook event to any of the 45 `hooks-registry.toml` entries that
point to this layer.

The bash layer is the current (transitional) implementation home for all hook logic.
Each script is a discrete behavioral contract: PreToolUse scripts can return exit
code 2 to block a tool call; PostToolUse scripts capture events; SubagentStop and
Stop scripts handle session lifecycle. The scripts follow strict conventions
(`#!/bin/bash`, `set -euo pipefail`, jq-with-graceful-fallback for JSON parsing,
`_emit` helper for event emission, `block` helper for blocking output).

SS-07 also owns `hooks-registry.toml` — the routing table that maps hook event
types and tool matchers to the WASM plugins that implement them. As of v1.0.0-beta.4,
all 45 entries route through `legacy-bash-adapter.wasm`. This file is the Subsystem
A-side authority for which bash scripts run on which events.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/hooks-registry.toml` | Routing table; 45 entries; `schema_version = 1`; all entries point to `legacy-bash-adapter.wasm` with `plugin_config.script_path` |
| **PreToolUse gate hooks** | |
| `plugins/vsdd-factory/hooks/block-ai-attribution.sh` | Block commits with AI co-authored-by lines |
| `plugins/vsdd-factory/hooks/protect-secrets.sh` | Block Bash + Read tool calls that risk exposing secrets |
| `plugins/vsdd-factory/hooks/destructive-command-guard.sh` | Block destructive shell commands (rm -rf, git reset --hard on main, etc.) |
| `plugins/vsdd-factory/hooks/factory-branch-guard.sh` | Block writes to main/master without PR flow |
| `plugins/vsdd-factory/hooks/brownfield-discipline.sh` | Enforce brownfield ingest discipline (no spec writes before ingest completes) |
| **PostToolUse capture hooks** | |
| `plugins/vsdd-factory/hooks/capture-commit-activity.sh` | Capture git commit metadata to event stream |
| `plugins/vsdd-factory/hooks/capture-pr-activity.sh` | Capture PR open/close/merge events |
| **PostToolUse validator hooks (24+ validate-* scripts)** | |
| `plugins/vsdd-factory/hooks/validate-*.sh` | Validate artifact structure after Write/Edit tool calls; each validates a specific artifact type or convention |
| **SubagentStop hooks** | |
| `plugins/vsdd-factory/hooks/handoff-validator.sh` | Validate sub-agent handoff completeness before stop |
| `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh` | Block SubagentStop if PR manager has unmerged work |
| `plugins/vsdd-factory/hooks/update-wave-state-on-merge.sh` | Advance wave state in STATE.md when a PR merges |
| **Stop hooks** | |
| `plugins/vsdd-factory/hooks/session-learning.sh` | Capture session learnings to memory on session end |
| `plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh` | Warn user if a wave gate is open at session end |

## Public Interface

**Routing table (`hooks-registry.toml`) schema (version 1):**
```toml
schema_version = 1

[[hooks]]
event = "PreToolUse"
tool_match = "Bash"
plugin = "legacy-bash-adapter.wasm"
priority = 100
on_error = "block"
[hooks.capabilities]
binary_allow = ["bash"]
shell_bypass_acknowledged = true
[hooks.plugin_config]
script_path = "hooks/protect-secrets.sh"
```

**Block protocol:** A PreToolUse bash script exits 2 to block the tool call.
It writes a JSON block message to stdout: `{"block": true, "reason": "..."}`.
Claude Code reads the exit code; the dispatcher reads block intent.

**Event emission:** Scripts call `${CLAUDE_PLUGIN_ROOT}/bin/emit-event <event_type> [<json_fields>]`
to emit structured events to the dispatcher event stream (second-class emission
path; S-3.4 PARTIAL).

**Environment available to bash hooks (per `exec_subprocess` invocation):**
`CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`, `HOOK_EVENT_NAME`, `TOOL_NAME`,
`SESSION_ID` — all passed by the adapter from the `HookPayload` context fields.

## Internal Structure

Bash hook conventions (pass-5-conventions.md §Plugin layer conventions, all 44/44
conform):

1. `#!/bin/bash` + `set -euo pipefail` — strict failure mode.
2. Parse stdin JSON via `jq` with graceful fallback: if `jq` is absent or stdin
   is malformed, the hook exits 0 (non-blocking) rather than erroring.
3. `_emit` helper function: wraps `bin/emit-event` call with error suppression
   so emission failure never blocks the hook's primary behavior.
4. `block` helper function: emits block JSON to stdout and exits 2.
5. Exit 0 on any error that is not an intentional block — hooks must never
   accidentally block the user due to their own internal failures.

The 24+ `validate-*` hooks share a common pattern: they read the last-written
file path from the tool response on stdin, check its structure against expected
conventions, and either pass (exit 0) or emit a WARNING event (exit 0 — they
never block; validation failure is advisory).

`hooks-registry.toml` is generated from `hooks.json` by
`scripts/generate-registry-from-hooks-json.sh` (SS-09). The TOML file has a
"DO NOT HAND-EDIT" header during the migration window; edits to the routing table
must go through `hooks.json` + regeneration (DRIFT-004).

## Dependencies

**Incoming (consumers of SS-07):**
- SS-04 (Plugin Ecosystem) — `legacy-bash-adapter.wasm` invokes every script here
  via `exec_subprocess`. The adapter reads `plugin_config.script_path` from each
  registry entry.
- SS-05 (Pipeline Orchestration) — workflows gate sub-agent file writes through
  the bash validator hooks; SubagentStop hooks enforce handoff quality.

**Outgoing (SS-07 depends on):**
- SS-10 (CLI Tools and Bin) — scripts call `bin/emit-event`, `bin/wave-state`,
  and other bin tools for event emission and state management.

## Cross-Cutting

- **Secrets protection:** `protect-secrets.sh` gates every Bash and Read tool
  call. It checks for common secret patterns in the tool input and blocks if
  found. This is the primary secrets-at-rest protection layer (ARCH-INDEX.md
  Cross-Cutting Concerns table).
- **Non-blocking contract:** Every hook exits 0 on internal errors. Only
  intentional blocks (content policy, gate failures) exit 2. This enforces
  NFR-REL-001 at the bash layer.
- **Windows incompatibility:** All 44 scripts require `bash`. DRIFT-010. Native
  WASM ports will progressively replace individual scripts, eliminating the
  Windows bash dependency per hook.
- **Dual routing tables:** `hooks.json` (SS-09 / legacy) and `hooks-registry.toml`
  (this subsystem) coexist. `hooks-registry.toml` is generated from `hooks.json`;
  do not hand-edit during migration (DRIFT-004, ADR-011).
- **jq dependency:** All scripts that parse JSON depend on `jq` being installed.
  Scripts use graceful fallback (exit 0 if jq absent) so the hook degrades
  silently rather than blocking on misconfigured machines.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-07/`
(target prefix BC-7; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: PreToolUse gate hooks — block-ai-attribution,
protect-secrets, destructive-command-guard, factory-branch-guard,
brownfield-discipline (BC-7.001–BC-7.050), PostToolUse capture hooks —
capture-commit-activity, capture-pr-activity (BC-7.051–BC-7.070), validate-*
family (24+ scripts, BC-7.071–BC-7.130), SubagentStop hooks — handoff-validator,
pr-manager-completion-guard, update-wave-state-on-merge (BC-7.131–BC-7.155),
Stop hooks — session-learning, warn-pending-wave-gate (BC-7.156–BC-7.176).

Note: per L-P0-003, the 24+ `validate-*` hooks currently have no formal BCs.
Phase 1 backfill must create BC-7.NNN entries per validator treating the script
body as the authoritative source.

## ADRs

- ADR-011: Dual hooks.json + hooks-registry.toml during migration — `decisions/ADR-011-dual-hook-routing-tables.md`
- ADR-012: Legacy-bash-adapter as universal current router — `decisions/ADR-012-legacy-bash-adapter-router.md`

## Drift / Known Issues

- **DRIFT-004 (P1 — medium):** Two parallel routing tables — `hooks.json` (legacy
  v0.79.x) and `hooks-registry.toml` (generated v1.0). Source-of-truth ambiguity
  during migration window. Re-running the generation script on a hand-edited TOML
  reverts changes silently. Planned-fix at 1.0 (L-P0-002).
- **DRIFT-010 (P0 for Windows / P1 overall):** All 44 scripts require `bash`.
  Windows operators need git-bash. Native WASM ports (Tier E) will reduce this
  requirement progressively.
- **L-P0-003 (debt):** 24+ `validate-*` hooks have no formal BCs. Phase 1
  backfill creates them; until then, the script body is the only spec.
- **S-3.4 PARTIAL:** Scripts emit events via `bin/emit-event` shell tool rather
  than directly through the `vsdd::emit_event` host fn path. Two emission paths
  write to different internal structures.
