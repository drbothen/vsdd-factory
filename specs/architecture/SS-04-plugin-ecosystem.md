---
document_type: architecture-section
level: L3
section: "SS-04-plugin-ecosystem"
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

# SS-04: Plugin Ecosystem

## [Section Content]

## Purpose

The Plugin Ecosystem subsystem contains the compiled WASM plugins that provide
actual hook behavior. At v1.0.0-beta.4 this subsystem ships two active crates:
`legacy-bash-adapter` (the universal bridge enabling all 44 existing bash hooks
to run under the dispatcher without modification) and `capture-commit-activity`
(the first native WASM port, currently a 20-LOC stub pending Tier E work).
Four additional Tier E stub crates (`capture-pr-activity`, `block-ai-attribution`)
and four Tier F lifecycle plugin crates (`session-start-telemetry`,
`session-end-telemetry`, `worktree-hooks`, `post-tool-use-failure`) are in the
module inventory but not yet shipped (pending S-3.01–3.03 and S-5.01–5.04
respectively).

The `legacy-bash-adapter` is architecturally significant: it implements the
multi-instance plugin pattern (ADR-012). A single compiled `.wasm` module is
registered as 45 separate entries in `hooks-registry.toml`, each with a distinct
`plugin_config.script_path` pointing to a different bash script. The adapter reads
this config, calls the `exec_subprocess` host function (capability-gated with
`shell_bypass_acknowledged = true`), and returns the bash script's exit code and
output as a `HookResult`. This allows all existing bash hooks to remain on disk
and execute without being ported to Rust, at the cost of requiring bash on the
operator's PATH (which blocks Windows users until native ports land; DRIFT-010).

Future native WASM ports will replace individual legacy-bash-adapter entries with
dedicated plugin crates compiled to `.wasm`. Each native port eliminates one
`exec_subprocess` call, improves cross-platform support, and moves toward the
end-state where all hooks are capability-gated, sandboxed, pure WASM.

## Modules

| Module / File | Responsibility | Status |
|---|---|---|
| `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` | `#[hook]` entry point; reads `plugin_config.script_path`; calls `vsdd::exec_subprocess(["bash", script_path])`; maps exit code / output to `HookResult`; expose `adapter_logic`, `BashOutcome` | shipped |
| `crates/hook-plugins/legacy-bash-adapter/` constants | `MAX_OUTPUT_BYTES = 1 MiB`, `BASH_TIMEOUT_MS = 60_000` | shipped |
| `crates/hook-plugins/capture-commit-activity/src/lib.rs` | Stub: `#[hook] fn on_hook` returning `HookResult::Continue`; 20-LOC placeholder for S-3.01 native port | stub (pending S-3.01) |
| `crates/hook-plugins/capture-pr-activity/src/lib.rs` | Native WASM port for PR activity capture (PostToolUse); emits `pr.opened`, `pr.merged` events | stub (pending S-3.02) |
| `crates/hook-plugins/block-ai-attribution/src/lib.rs` | Native WASM port for AI attribution blocking (PreToolUse); enforces no-AI-attribution commit policy | stub (pending S-3.03) |
| `crates/hook-plugins/session-start-telemetry/src/lib.rs` | SessionStart lifecycle plugin; emits `session.started` with session_id, factory_version, plugin_count, activated_platform, factory_health; runs `factory-health --brief` via `exec_subprocess`; checks tool deps | pending S-5.01 |
| `crates/hook-plugins/session-end-telemetry/src/lib.rs` | SessionEnd lifecycle plugin; emits `session.ended` with session duration and summary telemetry | pending S-5.02 |
| `crates/hook-plugins/worktree-hooks/src/lib.rs` | WorktreeCreate + WorktreeRemove lifecycle plugins; emits `worktree.created` / `worktree.removed` events; single crate covers both event types | pending S-5.03 |
| `crates/hook-plugins/post-tool-use-failure/src/lib.rs` | PostToolUseFailure lifecycle plugin; captures tool failure events with structured error metadata | pending S-5.04 |

<!-- [arch-decision] Decision A (S-5.01 adversarial pass-1, 2026-04-28): Tier F crates follow the v1.0-legacy S-5.1 naming intent and SS-04 per-event dedicated-crate precedent. Crate names: session-start-telemetry (S-5.01), session-end-telemetry (S-5.02), worktree-hooks (S-5.03, covers both WorktreeCreate + WorktreeRemove), post-tool-use-failure (S-5.04). S-5.03 uses a single crate for both worktree events because they share identical plugin_config shape, purity profile, and deployment unit. -->

## Public Interface

Plugins expose no public Rust API. Their external interface is the WASM ABI:

- **Entry point:** `_start` (emitted by `#[hook]` macro; reads stdin, writes stdout).
- **Input:** `HookPayload` JSON on stdin — same envelope the dispatcher parsed from
  Claude Code.
- **Output:** `HookResult` JSON on stdout — `Continue`, `Block`, or `Error`.
- **Host functions used by legacy-bash-adapter:**
  - `vsdd::exec_subprocess` (requires `binary_allow: ["bash"]` +
    `shell_bypass_acknowledged: true` in registry entry capabilities).
  - `vsdd::log` (diagnostic output to dispatcher internal log).
  - `vsdd::emit_event` (event emission, if the bash script's output contains
    structured event fields — forwarded after subprocess completes).
- **`hooks-registry.toml` entries:** 45 entries (all routing through
  `legacy-bash-adapter.wasm` at v1.0.0-beta.4); each entry carries
  `plugin_config.script_path = "hooks/<name>.sh"`.

## Internal Structure

`legacy-bash-adapter` control flow (per invocation):

1. `#[hook]` macro provides `_start`; stdin parsed to `HookPayload`.
2. `adapter_logic` reads `plugin_config.script_path` from the payload's
   per-plugin config block.
3. Calls `vsdd::exec_subprocess(["bash", script_path], env, timeout_ms)`.
4. Captures `SubprocessResult { exit_code, stdout, stderr, truncated }`.
5. Maps: `exit_code == 2` → `HookResult::Block`; `exit_code != 0` → `HookResult::Error`;
   otherwise → `HookResult::Continue`.
6. If stdout contains a JSON `emit_event` directive, forwards it via
   `vsdd::emit_event` (partial — S-3.4 PARTIAL; pass-8-final-synthesis.md §5).

Output truncation: stdout + stderr each capped at `MAX_OUTPUT_BYTES = 1 MiB`.
Bash script timeout: `BASH_TIMEOUT_MS = 60_000` (overridden per registry entry).

`capture-commit-activity` is a stub crate. The `#[hook]` macro is applied to a
`fn on_hook(_: HookPayload) -> HookResult` that returns `Continue`. Real capture
logic is planned for S-3.1 (Tier E). Source:
`crates/hook-plugins/capture-commit-activity/src/lib.rs`.

## Dependencies

**Incoming (consumers of SS-04):**
- SS-01 (Hook Dispatcher Core) — loads `.wasm` binaries via `plugin_loader`;
  invokes `_start`; grants capabilities via `HostContext` during execution.

**Outgoing (SS-04 depends on):**
- SS-02 (Hook SDK and Plugin ABI) — both plugin crates depend on `vsdd-hook-sdk`
  for `HookPayload`, `HookResult`, `#[hook]`, and host function shims.
- SS-07 (Hook Bash Layer) — `legacy-bash-adapter` shells out to the bash hook
  scripts in `plugins/vsdd-factory/hooks/*.sh` via `exec_subprocess`.

## Cross-Cutting

- **Capability requirements:** Every `legacy-bash-adapter` registry entry must
  declare `capabilities.binary_allow = ["bash"]` and
  `capabilities.shell_bypass_acknowledged = true`. Absence of either causes
  capability denial at the dispatcher host fn gate.
- **Sandboxing:** Each plugin invocation runs in an isolated wasmtime Store with
  per-plugin fuel (default 10M) and epoch budget (default 5 000 ms). The
  `legacy-bash-adapter` itself is sandboxed; the bash subprocess it spawns is
  not further sandboxed beyond the OS process boundary.
- **Windows compatibility:** DRIFT-010. All 44 hooks route through
  `legacy-bash-adapter` which requires `bash` on PATH. Windows users must have
  git-bash installed. Native WASM ports (S-3.1/3.2/3.3) will eliminate this
  requirement per hook as they land (Tier E, rc.1 target).
- **Error handling:** `BashOutcome` captures subprocess result; adapter maps to
  typed `HookResult`. Bash script stderr is captured (up to 4 KiB from dispatcher
  `STDERR_CAP_BYTES`) and forwarded on `HookResult::Error`.
- **Observability:** `vsdd::log` calls from the adapter appear in
  `dispatcher-internal.jsonl` under the plugin's event entries.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-04/`
(target prefix BC-4; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: `legacy-bash-adapter` config read and script dispatch
(BC-4.001–BC-4.010), exit-code-to-HookResult mapping (BC-4.011–BC-4.015),
output truncation invariants (BC-4.016–BC-4.020), capability denial propagation
(BC-4.021–BC-4.025), `capture-commit-activity` stub contract (BC-4.026–BC-4.030),
session-start-telemetry plugin family (BC-4.04.001–BC-4.04.005).

**Session-start-telemetry plugin pattern (pass-4 architectural note):**
The `session-start-telemetry` plugin reads `activated_platform` from
`.claude/settings.local.json` using the canonical `read_file` host fn pattern —
declaring `[hooks.capabilities.read_file]` with `path_allow = [".claude/settings.local.json"]`
in its `hooks-registry.toml` entry per BC-4.04.005. No new host function was needed.
BC-1.10.001 ("vsdd::activated_platform() host fn") was created in pass-2 and retired in
pass-4 as over-engineering: the production `read_file` host fn
(`crates/factory-dispatcher/src/host/read_file.rs`) with `ReadFileCaps.path_allow`
enforcement already provides controlled sandboxed file access via the established
`[hooks.capabilities.read_file]` TOML table pattern (see `hooks-registry.toml:470`).

Once-per-session discipline for the `SessionStart` entry is enforced at Layer 1 by
Claude Code's `once: true` directive in `hooks.json.template` (BC-4.04.004 invariant 1).
The dispatcher does not enforce per-event dedup at Layer 2 for this entry; BC-1.10.002
was retired in pass-4. The plugin itself is unconditionally stateless (BC-4.04.003
pass-4 revision).

## ADRs

- ADR-002: WASM (wasmtime) plugin ABI — `decisions/ADR-002-wasm-plugin-abi.md`
- ADR-003: WASI preview 1 for v1.0 — `decisions/ADR-003-wasi-preview1.md`
- ADR-012: Legacy-bash-adapter as universal current router — `decisions/ADR-012-legacy-bash-adapter-router.md`

## Drift / Known Issues

- **DRIFT-010 (P0 for Windows / P1 overall):** All 44 hooks require `bash`.
  Native ports (S-3.1 `capture-commit-activity`, S-3.2 `capture-pr-activity`,
  S-3.3 `block-ai-attribution`) are stubs. Planned for Tier E (rc.1).
- **S-3.1 NOT SHIPPED:** `capture-commit-activity` native WASM port is a 20-LOC
  stub in `crates/hook-plugins/capture-commit-activity/src/lib.rs`.
- **S-3.2, S-3.3 NOT SHIPPED:** No crates yet for `capture-pr-activity` or
  `block-ai-attribution` native ports.
- **S-3.4 PARTIAL:** `emit_event` forwarding from bash stdout is partially
  implemented. Bash hooks call `bin/emit-event` shell tool rather than emitting
  directly through the host fn path. Reconciliation pending S-3.4 completion.
