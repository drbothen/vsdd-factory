---
document_type: architecture-section
level: L3
section: "SS-04-plugin-ecosystem"
version: "1.4"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1.2
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
input-hash: "abdac50"
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
`session-end-telemetry`, `worktree-hooks`, `tool-failure-hooks`) are in the
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
| `crates/hook-plugins/tool-failure-hooks/src/lib.rs` | PostToolUseFailure lifecycle plugin; captures tool failure events with structured error metadata | pending S-5.04 |

<!-- [arch-decision] Decision A (S-5.01 adversarial pass-1, 2026-04-28): Tier F crates follow the v1.0-legacy S-5.1 naming intent and SS-04 per-event dedicated-crate precedent. Crate names: session-start-telemetry (S-5.01), session-end-telemetry (S-5.02), worktree-hooks (S-5.03, covers both WorktreeCreate + WorktreeRemove), tool-failure-hooks (S-5.04). S-5.03 uses a single crate for both worktree events because they share identical plugin_config shape, purity profile, and deployment unit. -->

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
`[hooks.capabilities.read_file]` TOML table pattern (see `plugins/vsdd-factory/hooks-registry.toml::[hooks.capabilities.read_file]`).

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

## E-9: Tier 2 Native WASM Migration (W-16)

**Epic positioning:** E-9 is the Tier 2 native WASM migration epic. It follows E-8
(W-15 Tier 1) which shipped at rc.4 (v1.0.0-rc.4, 2026-04-25). E-9 ports all 23
`validate-*.sh` hooks from bash (routed via `legacy-bash-adapter`) to native WASM
plugins authored in Rust using `vsdd-hook-sdk`.

**Dependency:** E-9 requires E-8 (W-15) Tier 1 closure. E-8 is complete as of rc.4.

**Scope:** ~9 stories total.

| Story | Scope |
|-------|-------|
| S-9.00 | Perf baseline: measure WASM bundle size pre-W-16; set W-16 bundle ceiling |
| SDK-ext | Dispatcher + SDK implementation of `host::run_subprocess` (BC-2.02.013) |
| S-9.01 | Batch 1: 4 pure stdin-parse hooks (demo-evidence-story-scoped, factory-path-root, finding-format, novelty-assessment) |
| S-9.02 | Batch 2: 4 single-file-read frontmatter validators (bc-title, changelog-monotonicity, red-ratio, input-hash) |
| S-9.03 | Batch 3: 3 PR/delivery artifact validators (pr-description-completeness, table-cell-count, pr-merge-prerequisites) |
| S-9.04 | Batch 4: 3 STATE.md + cycle INDEX validators (state-index-status-coherence, state-pin-freshness, state-size) |
| S-9.05 | Batch 5: 3 multi-file cross-document validators (story-bc-sync, count-propagation, index-self-reference) |
| S-9.06 | Batch 6: 3 multi-file + ARCH-INDEX cross-document validators (anchor-capabilities-union, subsystem-names, template-compliance) |
| S-9.07 | Batch 7: 3 highest-complexity validators (vp-consistency, wave-gate-completeness, wave-gate-prerequisite) |

**Port strategy:** rewrite-clean (D-9.1). Idiomatic Rust with `regex`, `serde_json`,
`serde_yaml`. No 1:1 bash-to-Rust translation. See ADR-014.

**Subprocess capability:** `host::run_subprocess` (D-9.2, ADR-014) is used by S-9.07
(`validate-wave-gate-prerequisite` invokes `verify-sha-currency.sh`). The SDK
extension story implements the dispatcher ABI and SDK shim before S-9.01..S-9.07
begin.

**Bundle ceiling:** S-9.00 measures the WASM bundle baseline before any W-16 plugin
lands and sets the W-16-specific 25% growth ceiling (analogous to E-8's R-8.09
25% ceiling on Tier 1 bundle growth). The 23 new plugins are estimated at 4.6–6.9 MB
(per R-W16-003 audit); the S-9.00 baseline determines whether the existing R-8.09
ceiling must be revised or replaced with a per-wave ceiling metric.

**Migration sequence:** After E-9 ships, the 23 `validate-*.sh` legacy-bash-adapter
registry entries are disabled or removed. The `.sh` files remain on disk per
R-W16-001 (bats orphan migration deferred to Phase H). The `legacy-bash-adapter`
continues routing hooks not yet ported (Tier 3, W-17). Phase H (v1.3.0) removes
both the adapter and all `.sh` bash hook files after W-17 completes Tier 3.

**Per-story delivery pattern:** Each story (S-9.01..S-9.07) follows the W-15
per-story-delivery cycle: test-writer RED gate + implementer GREEN + demo-recorder
+ pr-manager 9-step. Each story spec must include:
- Task A: Port hook(s) to Rust WASM crate(s) under `crates/hook-plugins/`.
- Task B: Update `hooks-registry.toml` — add WASM entries; disable legacy-bash-adapter
  entries for ported hooks.
- Task C: Create bats deletion checklist; file TD per hook for bats migration to
  Phase H (TD-020 class problem, per R-W16-004).
- Task D: Create WASM integration tests in `factory-dispatcher/tests/` exercising
  WASM dispatch path end-to-end.

**ADR reference:** ADR-014 (`decisions/ADR-014-tier-2-native-wasm-migration.md`).

---

## Changelog

| Version | Date | Change |
|---------|------|--------|
| 1.4 | 2026-05-12 | F2 block-ai-attribution message-file arm: added capability declaration pattern section for bounded I/O PostToolUse verification flows (BC-7.03.094, BC-7.03.095; E-16 stories S-16.01/S-16.02). |
| 1.3 | 2026-05-08 | TD-VSDD-091 Chunk 6 — migrated 1 body cite: `hooks-registry.toml:470` → `plugins/vsdd-factory/hooks-registry.toml::[hooks.capabilities.read_file]`. |
| 1.2 | 2026-05-03 | ADR-014: added E-9 epic positioning section — Tier 2 native WASM migration, ~9 stories, bundle ceiling, migration sequence, per-story delivery pattern. |
| 1.1 | 2026-04-29 | ADV-S5.04-P06 HIGH-P06-001: crate name sync to canonical `tool-failure-hooks` (matches ARCH-INDEX line 77, S-5.04 target_module, BC-4.08.001/003, VP-068, PRD line 455). Three references updated: Purpose prose, Modules table row, Decision A comment. |
| 1.0 | 2026-04-25 | Initial version |

---

## Amendment 2026-05-12 (v1.3 → v1.4 — F2 block-ai-attribution message-file arm)

_Added in v1.4. Authoritative behavioral contracts: BC-7.03.094 (PostToolUse arm), BC-7.03.095 (PreToolUse -F arm). Architectural decisions: ADR-002 (WASM capability model), ADR-014 (Tier-2 native WASM migration)._

### Capability Declaration Pattern: Bounded I/O for PostToolUse Verification Flows

The `block-ai-attribution` plugin (`crates/hook-plugins/block-ai-attribution/`)
is the first plugin in the ecosystem to combine:

1. **PreToolUse blocking** with `on_error = "block"`, and
2. **PostToolUse verification** with `on_error = "continue"`, and
3. Both `read_file` and `exec_subprocess` capabilities declared in a single plugin's
   registry entries.

This combination establishes a reference pattern for future plugins that need bounded
I/O to perform verification in a PostToolUse corrective-signal flow.

#### exec_subprocess capability scope for git operations

When a plugin needs to invoke `git` for a read-only query (e.g., `git log -1
--format=%B HEAD` to read the HEAD commit message), the registry entry's capability
block should follow the narrow pattern:

```toml
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
shell_bypass_acknowledged = false
timeout_ms = 1000
```

Rules:
- `binary_allow` must be a tight allowlist — list only the specific binary required.
  For trivial git reads, `["git"]` is the full allowlist.
- `shell_bypass_acknowledged = false` is correct when `binary_allow` is narrow and
  no shell metacharacter expansion is needed. Set `true` only when a bash script is
  the subprocess (legacy-bash-adapter pattern).
- `timeout_ms = 1000` for trivial read-only git operations (Class A per ADR-020).
  Longer timeouts (2000–5000 ms) are appropriate for write operations or operations
  on large repos with expected I/O latency.

This pattern operationalizes ADR-002's capability gating model: deny-by-default with
an explicit narrow allowlist, not a broad wildcard.

#### read_file capability scope: prefer narrow path_allow

For the PreToolUse -F arm, the `read_file` capability uses a narrow `path_allow`
list rather than a broad glob. The principle (OQ-F1-002, resolved in F1 delta
analysis): **security-relevant plugins with `on_error = "block"` must not have
broad filesystem read access.**

Recommended pattern for commit-message-file reading:

```toml
[hooks.capabilities.read_file]
path_allow = [
  "**/.git/COMMIT_EDITMSG",
  "/tmp/**",
  "/var/folders/**",
  "<project-root>/**",
]
max_bytes = 65536
```

The four entries in `path_allow` cover all realistic commit message file locations:
- `**/.git/COMMIT_EDITMSG` — git's own commit message temp file.
- `/tmp/**` — POSIX temp directory (editors like Vim, Emacs, nano).
- `/var/folders/**` — macOS-specific temp directories (`mktemp` output).
- `<project-root>/**` — project-tree message files (e.g., `./msg.txt`).

Reads outside the allowlist are denied by the dispatcher capability gate. Per
BC-7.03.095 INV-1, a denied read yields `HookResult::Continue` — not Block, not
Error. This invariant ensures that an operator running `git commit -F /etc/passwd`
never receives a spurious block; the plugin cannot read that path and passes
through cleanly.

#### Contrast with broad-glob pattern (anti-pattern for security-relevant plugins)

Using a wildcard `path_allow` (or omitting `path_allow`) grants the plugin read
access to any file on the filesystem. This is appropriate for utility plugins with
`on_error = "continue"` that need flexible file access. It is not appropriate for
security-relevant gate plugins.

| Plugin class | on_error | path_allow recommendation |
|---|---|---|
| Security gate (block-ai-attribution, protect-secrets, etc.) | `block` | Narrow — enumerate specific paths/globs |
| Telemetry / capture | `continue` | Moderate — scope to project tree |
| Utility validator | `continue` | Moderate to broad — depends on what files the plugin needs to read |

#### Summary: block-ai-attribution capability surface after E-16

After S-16.01 and S-16.02 ship, the `block-ai-attribution` plugin has two registry
entries with the following capability surfaces:

| Entry | Event | Capabilities | on_error |
|---|---|---|---|
| PreToolUse | PreToolUse / Bash | `read_file` (narrow path_allow, max_bytes=65536) | `block` |
| PostToolUse | PostToolUse / Bash | `exec_subprocess` (binary_allow=["git"], shell_bypass=false, timeout=1000ms) | `continue` |

Both entries cite ADR-002 (WASM plugin ABI and capability model) as the authoritative
architectural decision. ADR-014 (Tier-2 native WASM migration) classifies
`block-ai-attribution` as an already-shipped Tier-2 plugin; no migration
considerations arise from adding capabilities to an existing entry.
