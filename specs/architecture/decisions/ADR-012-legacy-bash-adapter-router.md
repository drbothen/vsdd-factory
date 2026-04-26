---
document_type: adr
adr_id: ADR-012
status: accepted
date: 2026-04-26
subsystems_affected: [SS-04, SS-07]
supersedes: null
superseded_by: null
---

# ADR-012: Legacy-Bash-Adapter as Universal Current Router

## Context

At the time v1.0 was designed, vsdd-factory had approximately 44 bash hooks covering
commit tracking, PR management, template compliance validation, branch protection,
session learning, and dozens of other factory behaviors. These hooks represented
years of accumulated factory logic and were verified by a 1177+ test bats suite.

The v1.0 WASM plugin system (ADR-002) replaced the direct bash invocation model:
hooks would now be `.wasm` modules executed by the dispatcher. However, porting 44
bash hooks to native WASM was not feasible within the v1.0 beta timeline. Each port
requires: authoring a Rust plugin, testing against the bats suite, validating the
`exec_subprocess` capability envelope if the hook calls external tools, and releasing
a new plugin binary.

The design needed a bridge that would make all 44 existing bash hooks function
correctly under the v1.0 WASM dispatcher without requiring any modifications to
the bash scripts themselves, while preserving the regression test coverage.

## Decision

A single WASM plugin, `legacy-bash-adapter`, serves as the universal current router
for all unported bash hooks. Each bash hook gets one entry in `hooks-registry.toml`
pointing to the same `legacy-bash-adapter.wasm` plugin, with a `[hooks.config]`
block specifying the `script_path` to invoke. The adapter reads `script_path` from
the registry-supplied plugin config, resolves it under `${CLAUDE_PLUGIN_ROOT}`,
re-serializes the original `HookPayload` (with `plugin_config` stripped — bash hooks
predate that field), pipes it to `bash <script>` via the `exec_subprocess` host
function, and maps bash exit codes to `HookResult` values: `0` → `Continue`,
`2` → `Block`, other → `Error`.

The adapter requires explicit capability escalation in `hooks-registry.toml`:
`exec_subprocess` with a binary allow-list (`["bash"]`), `shell_bypass_acknowledged`
with justification string, an env allow-list, a cwd allow-list, and explicit
`default_timeout_ms` and `max_output_bytes` bounds. Shell interpreter use is not
silent; it must be declared with a reason string per the exec_subprocess capability
design (ADR per Q4 resolution).

## Rationale

The adapter-per-hook alternative (one WASM plugin per bash hook, each wrapping a
single script) was evaluated and rejected. It would produce 44 WASM modules that all
contained identical logic (shell out to bash, pipe payload, map exit code) differing
only in the script path. This creates maintenance overhead — any fix to the shelling-out
logic would require rebuilding and committing 44 modules. It also bloats
`hooks-registry.toml` with redundant capability blocks.

The single-adapter approach is strictly simpler: one WASM binary, one capability
block template (instantiated per-hook in the registry), one place to fix if the
adapter logic changes. The per-hook `script_path` in `[hooks.config]` carries the
only per-hook variation.

The design doc (line 59–63) frames this explicitly: "A single WASM plugin that shells
out to existing bash scripts (via host-granted `exec_subprocess` capability with
explicit `shell_bypass_acknowledged`) keeps the 30 current hooks working on
macOS/Linux while they're ported individually." (The count rose to 44 as more hooks
were catalogued before implementation.)

The `adapter_logic<F>` function in `lib.rs` separates the core adapter logic from
the `#[hook]` entry point, enabling unit tests to drive the adapter through synthetic
payloads and a mocked subprocess runner without standing up wasmtime. This was a
deliberate testability design decision captured in the `lib.rs` module comment.

The adapter is explicitly temporary. Post-1.0, individual hook ports to native WASM
are driven by observed latency telemetry from the dispatcher's `plugin.completed`
events. As each hook is ported, its `hooks-registry.toml` entry is updated to point
to the native plugin instead of `legacy-bash-adapter.wasm`. When the last bash hook
is ported, the adapter can be retired.

## Consequences

### Positive

- All 44 bash hooks function correctly under the v1.0 dispatcher without any
  modification to the bash scripts themselves.
- The full 1177+ test bats suite continues to provide regression coverage through
  the adapter.
- The `exec_subprocess` capability model (deny-by-default, explicit allow-list) is
  applied consistently to all bash invocations through the adapter; there are no
  uncontrolled subprocess calls.
- Windows support for native-WASM-ported hooks is not blocked by the adapter;
  as each hook is ported, Windows coverage automatically extends to that hook.

### Negative / Trade-offs

- All adapter-routed hooks incur subprocess overhead: the WASM module instantiation
  cost plus the `fork+exec` cost of launching `bash`. This is higher latency than a
  native WASM hook. The magnitude is acceptable for most hooks; high-frequency hooks
  are prioritized for native porting.
- The adapter requires `shell_bypass_acknowledged` in every registry entry, which
  produces a large capability block in `hooks-registry.toml`. This is intentional
  (explicit consent for shell use) but visually noisy.
- Bash hooks routed through the adapter will never run on Windows without native
  porting. DRIFT-010 ("26 unported bash hooks block Windows native") tracks this
  as a known limitation through v1.0.

### Status as of v1.0.0-beta.5

IN-EFFECT. `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` implements the
adapter with `adapter_logic<F>` for testability. The compiled `legacy-bash-adapter.wasm`
is committed to `plugins/vsdd-factory/hook-plugins/`. All bash hooks currently in
scope are routed through this adapter in `hooks-registry.toml`. DRIFT-010 tracks
the 26 unported hooks as a planned future work item.

## Alternatives Considered

- **One WASM plugin per bash hook:** 44 modules all containing identical shelling-out
  logic, differing only in script path. Rejected: maintenance overhead; 44
  capability blocks in `hooks-registry.toml`; no reduction in subprocess overhead.
- **Keep bash scripts as direct hooks.json entries (bypass the WASM layer):**
  Register bash scripts directly in `hooks.json` entries pointing to the dispatcher.
  Rejected: this collapses the dispatcher routing table back to the v0.79.x model;
  the dispatcher would need to special-case bash invocations; the capability
  enforcement model would not apply.
- **Port all bash hooks to native WASM before v1.0:** Deferred past v1.0 scope.
  Rejected as a v1.0 gate criterion: 44 native ports was estimated at multiple
  sprints; the adapter achieves functional parity immediately.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 59–63 (legacy-bash-adapter decision), lines 251–283 (hooks-registry.toml
  example showing adapter capability block with `shell_bypass_acknowledged`),
  lines 545–554 (Phase 2 deliverables for legacy-bash-adapter).
- **Code as-built:** `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:1–50`
  (module doc with flow description and `adapter_logic<F>` testability rationale).
- **State tracking:** `.factory/STATE.md` line 125 (DRIFT-010 — "26 unported bash
  hooks block Windows native | MEDIUM | Tier E").
