---
document_type: architecture-section
level: L3
section: "SS-02-hook-sdk"
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

# SS-02: Hook SDK and Plugin ABI

## [Section Content]

## Purpose

The Hook SDK and Plugin ABI subsystem defines the contract between plugin authors
and the dispatcher. It is the only mechanism by which Rust (or any
`wasm32-wasip1`-targeted language) code becomes a `.wasm` plugin loadable by
`factory-dispatcher`. It has two crates: `hook-sdk` (the library plugin authors
add as a dependency) and `hook-sdk-macros` (the proc-macro that generates the WASM
entry point glue).

The SDK encodes the `HOST_ABI_VERSION = 1` constant on both sides of the boundary.
The dispatcher's `setup_linker` registers `vsdd::*` host function imports against
this version; `hook-sdk` declares the matching extern shims. A version mismatch at
load time produces a loud `internal.dispatcher_error` and the plugin is refused —
never silently mis-executed. This is the primary versioning mechanism protecting
operators from ABI drift between SDK and dispatcher (ADR-006, NFR-COMPAT-002).

The `#[hook]` proc-macro emits a `_start` function that the dispatcher calls as the
plugin entry point. It handles stdin deserialization into `HookPayload`, wraps the
user function in a panic boundary, and serializes the `HookResult` back to stdout.
Plugin authors write a single Rust function annotated with `#[hook]`; all FFI,
error handling, and ABI plumbing are invisible to them.

## Modules

| Module / File | Responsibility |
|---|---|
| `crates/hook-sdk/src/lib.rs` | Re-exports: `HookPayload`, `HookResult`, `HOST_ABI_VERSION = 1`, `host::*` shim functions |
| `crates/hook-sdk/src/payload.rs` | `HookPayload` type mirroring dispatcher's envelope; serde deserialization from stdin JSON |
| `crates/hook-sdk/src/result.rs` | `HookResult` enum: `Continue`, `Block { reason }`, `Error { message }`; serialized to stdout JSON |
| `crates/hook-sdk/src/host/mod.rs` | Public host function shim re-exports |
| `crates/hook-sdk/src/host/log.rs` | `vsdd::log(level, msg)` extern shim — calls dispatcher host fn |
| `crates/hook-sdk/src/host/emit_event.rs` | `vsdd::emit_event(type, fields_json)` extern shim |
| `crates/hook-sdk/src/host/context.rs` | `vsdd::session_id()`, `vsdd::dispatcher_trace_id()`, `vsdd::plugin_root()`, `vsdd::plugin_version()`, `vsdd::cwd()` shims |
| `crates/hook-sdk/src/host/env.rs` | `vsdd::env_read(key)` extern shim (allow-list enforced by dispatcher) |
| `crates/hook-sdk/src/host/read_file.rs` | `vsdd::read_file(path)` extern shim (path_allow enforced by dispatcher) |
| `crates/hook-sdk/src/host/exec_subprocess.rs` | `vsdd::exec_subprocess(args_json)` extern shim; returns `SubprocessResult` |
| `crates/hook-sdk-macros/src/lib.rs` | `#[hook]` proc-macro: emits `_start` adapter + JSON stdin deserialize + panic boundary |

## Public Interface

The SDK is published to crates.io as `vsdd-hook-sdk`. Plugin authors declare:

```toml
[dependencies]
vsdd-hook-sdk = "1.0"

[lib]
crate-type = ["cdylib"]
```

And write:

```rust
use vsdd_hook_sdk::{HookPayload, HookResult, hook};

#[hook]
fn on_hook(payload: HookPayload) -> HookResult {
    HookResult::Continue
}
```

The macro expands to a `_start` symbol exported as `wasm32-wasip1` entry point.

**Versioned ABI constants exported:**
- `HOST_ABI_VERSION: u32 = 1` — must match dispatcher's constant; checked at
  plugin load time.
- `HookResult::Continue`, `HookResult::Block { reason: String }`,
  `HookResult::Error { message: String }` — the only valid plugin outputs.

**Host function shims (`vsdd::*` namespace):** `log`, `emit_event`,
`session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`,
`env_read`, `read_file`, `exec_subprocess`.

## Internal Structure

Two-crate split (pass-1-architecture.md, lines 40-43):

- `hook-sdk`: pure library; no proc-macro magic. Can be used without `#[hook]`
  for advanced plugins that manage their own `_start`. Exposes typed wrappers
  around raw `extern "C"` shims so plugin authors never write unsafe FFI.
- `hook-sdk-macros`: proc-macro crate only. Depends on `hook-sdk` to validate
  the user function signature against `HookPayload` → `HookResult` at compile
  time. The macro emits: (1) stdin read loop, (2) JSON deserialize into
  `HookPayload`, (3) panic catch boundary around the user fn, (4) JSON serialize
  `HookResult` to stdout.

All host functions follow the same pattern: the shim is `extern "C"` + unsafe;
the public SDK wrapper is safe Rust with typed return values. `SubprocessResult`
is the SDK-side typed envelope for `exec_subprocess` output (exit code, stdout
bytes, stderr bytes, truncation flag).

Note: `codes::*` error codes are defined in `factory-dispatcher::host` and
mirrored in `hook-sdk::host::HostError::from_code`. These must stay in sync
(L-P1-007 — candidate for extraction to a `host-codes` crate in a future cycle).

## Dependencies

**Incoming (consumers of SS-02):**
- SS-01 (Hook Dispatcher Core) — registers `vsdd::*` linker imports; checks
  `HOST_ABI_VERSION` at plugin load; invokes `_start` entry point.
- SS-04 (Plugin Ecosystem) — all WASM plugins are built using `hook-sdk` and
  `hook-sdk-macros`.

**Outgoing (SS-02 depends on):**
- None within the factory subsystem. `hook-sdk` depends only on `serde`,
  `serde_json`, and `wasm32-wasip1` standard library. It is a leaf dependency.

## Cross-Cutting

- **ABI versioning:** `HOST_ABI_VERSION = 1` is a hard breaking-change gate.
  Any host fn signature change or new required import must bump both the
  dispatcher constant and the SDK constant, triggering a semver major bump on
  both (NFR-COMPAT-002, ADR-006).
- **Panic boundary:** The `#[hook]` macro wraps the user fn in `std::panic::catch_unwind`.
  A panicking plugin returns `HookResult::Error` rather than crashing the WASM
  guest and producing an opaque trap.
- **No network access:** Plugins compiled with this SDK have no direct network
  path. `exec_subprocess` (cap-gated) is the only egress until WASI preview-2
  (ADR-003).
- **`#[deny(missing_docs)]`:** Not yet enforced on `hook-sdk` itself; only
  `sink-*` crates enforce it (L-P1-002 — planned fix).
- **Error handling:** `thiserror` for `HostError`; plugin-facing panics caught
  by macro boundary; `SubprocessResult` carries typed error fields rather than
  raw i32.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-02/`
(target prefix BC-2; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: `HookPayload` deserialization invariants (BC-2.001–BC-2.005),
`HookResult` serialization invariants (BC-2.006–BC-2.010), `#[hook]` macro
expansion correctness (BC-2.011–BC-2.015), host function shim ABI alignment
(BC-2.016–BC-2.020), panic boundary and error promotion (BC-2.021–BC-2.025).

### BC-2.02.x Host-Shim ABI Family

The `BC-2.02.NNN` family covers the SDK host-function ABI surface.

| BC ID | Contract | Story | Status |
|-------|----------|-------|--------|
| BC-2.02.001 | Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private (`mod ffi;`) | S-1.03 | active |
| BC-2.02.002 | `host::read_file` shim surfaces typed `Result<Vec<u8>, HostError>` | S-1.03 | active |
| BC-2.02.003 | `host::log` maps `LogLevel` discriminants to dispatcher wire codes | S-1.03 | active |
| BC-2.02.004 | `host::emit_event` serializes fields as length-prefixed UTF-8 key/value sequence | S-1.03 | active |
| BC-2.02.005 | `host::exec_subprocess` returns typed `SubprocessResult` with exit code and truncation flag | S-1.03 | active |
| BC-2.02.006 | `host::session_id` re-calls with larger buffer when host returns required capacity | S-1.03 | active |
| BC-2.02.007 | `host::dispatcher_trace_id` same re-call-on-overflow contract as session_id | S-1.03 | active |
| BC-2.02.008 | `host::plugin_root` same re-call-on-overflow contract | S-1.03 | active |
| BC-2.02.009 | `host::plugin_version` same re-call-on-overflow contract | S-1.03 | active |
| BC-2.02.010 | `LogLevel` discriminants 0..=4 are pinned (Trace=0 ... Error=4) | S-1.03 | active |
| BC-2.02.011 | `host::write_file` shim: write content to an allow-listed path; returns `Ok(())` on success, `Err(HostError)` on failure | S-8.10 | draft (PO authoring) |
| BC-2.02.012 | `HookPayload` SubagentStop top-level fields: `agent_type`, `subagent_name`, `last_assistant_message`, `result` — all `Option<String>` with `#[serde(default)]`; absent or JSON-null deserializes to `None` | S-8.30 + S-8.01 + S-8.03 reopens | draft (PO authoring) |

Note: BC-2.02.011 and BC-2.02.012 are in-flight (D-183 decision). When PO files them, the ARCH-INDEX SS-02 BC count will update from 22 to 24.

## HookPayload Schema Evolution

### Additive-ABI Extension Pattern

When Claude Code's hook envelope schema gains new top-level fields for a given event
type (e.g. SubagentStop carries `agent_type` and `last_assistant_message` at the
top level of the JSON payload), `HookPayload` extends additively: new fields are
added as `#[serde(default)] Option<String>`. This pattern has been sanctioned twice:

| Decision | Story | Extension | HOST_ABI_VERSION |
|----------|-------|-----------|-----------------|
| D-6 Option A | S-8.10 | `host::write_file` — additive host function; anchors BC-2.02.011 | Stays at 1 |
| D-183 | S-8.30 | `HookPayload` SubagentStop fields — additive struct fields; anchors BC-2.02.012 | Stays at 1 |

**General rule:** Any future Claude Code envelope schema addition follows this pattern
unless the change is backward-incompatible (which would require a `HOST_ABI_VERSION`
bump and is out of scope for additive evolution per ADR-006).

### Why `#[serde(default)] Option<String>`

- `#[serde(default)]` means the field is absent from the JSON for event types that
  do not carry it — `PreToolUse`, `PostToolUse`, `SessionStart`, etc. — and serde
  deserializes it as `None` without error. Existing event types are unaffected.
- JSON `null` also deserializes to `None` via serde's standard `Option<T>`
  implementation. This provides free jq-`//`-equivalent fallback semantics (see
  jq-`//` Parity Convention section below).
- `HOST_ABI_VERSION = 1` stays stable because the ABI between the dispatcher and
  WASM plugins is the stdin/stdout JSON wire format, and adding optional top-level
  fields is backward-compatible by definition.

### SubagentStop Envelope Schema

Claude Code's `SubagentStop` event delivers these top-level JSON fields. The four
bash hooks that consume them are the canonical reference for field names and fallback
chains:

| Field | Type | Bash fallback chain | Source |
|-------|------|---------------------|--------|
| `agent_type` | `Option<String>` | `.agent_type \|\| .subagent_name \|\| "unknown"` | handoff-validator.sh:27, pr-manager-completion-guard.sh:25, validate-pr-review-posted.sh:21, track-agent-stop.sh:22 |
| `subagent_name` | `Option<String>` | second fallback for agent identity when `agent_type` absent | same hooks |
| `last_assistant_message` | `Option<String>` | `.last_assistant_message \|\| .result \|\| ...` | handoff-validator.sh:28, pr-manager-completion-guard.sh:26, validate-pr-review-posted.sh:22, track-agent-stop.sh:23 |
| `result` | `Option<String>` | second fallback for message content | same hooks |

**Note on `output`:** `handoff-validator.sh:28` uses a third fallback `.output` —
this is story-specific (S-8.01) to handle historical format variations. It is NOT
part of the canonical SubagentStop envelope. The WASM port (BC-2.02.012) does not
need to model `output` as a separate field; hook-level logic handles the third
fallback via `Option` chaining on `last_assistant_message` → `result`.

**Canonical contract:** BC-2.02.012 defines the typed projection. Stories S-8.30,
S-8.01 (reopen), and S-8.03 (reopen) trace to BC-2.02.012.

### jq-`//` Parity Convention

Bash hooks written against the Claude Code SubagentStop event use jq's `//` null-
coalescing operator:

```bash
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // empty')
```

The WASM port preserves identical semantics via `Option::as_deref()` chaining:

```rust
// Per BC-2.02.012 Postcondition 5 (canonical agent identity fallback chain):
let agent: &str = payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");

// Per BC-2.02.012 Postcondition 6 (canonical assistant-message fallback chain):
let result: &str = payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");
```

**These chains are NORMATIVE per BC-2.02.012 Invariant 5. Story specs (S-8.01,
S-8.02, S-8.03, S-8.05, S-8.30) MUST use these expressions verbatim. Deviations
require explicit divergence rationale.**

The borrowing form (`as_deref()` → `&str`) is required — not the consuming form
(`Option::or` returning `Option<String>`). When a bash hook uses
`jq -r '.fieldA // .fieldB // "default"'`, the WASM translation is always
`payload.field_a.as_deref().or(payload.field_b.as_deref()).unwrap_or("default")`.
No bespoke parsing logic is needed.

## ADRs

- ADR-002: WASM (wasmtime) plugin ABI — `decisions/ADR-002-wasm-plugin-abi.md`
- ADR-003: WASI preview 1 for v1.0; preview 2 deferred — `decisions/ADR-003-wasi-preview1.md`
- ADR-006: HOST_ABI_VERSION as separate semver constant — `decisions/ADR-006-host-abi-version.md`
- ADR-010: StoreData-typed linker for host functions — `decisions/ADR-010-storedata-linker.md`

### Decision Log (in-spec)

**D-183 (2026-05-01) — HookPayload additive extension for SubagentStop top-level fields**

Context: S-8.01 (handoff-validator WASM port) and S-8.03 (pr-manager-completion-guard
WASM port) both reached CONVERGENCE_REACHED before HookPayload carried the
SubagentStop envelope's `agent_type` and `last_assistant_message` as typed fields.
The bash hooks read these fields via jq at the top level of stdin, but HookPayload
only had `tool_input: serde_json::Value`. S-8.30 was opened to fix this gap (authored 2026-05-01, D-183 Phase B; originally referred to as S-8.11 before POLICY 1 assigned the canonical ID).

Decision: Extend HookPayload with four `#[serde(default)] Option<String>` fields
(`agent_type`, `subagent_name`, `last_assistant_message`, `result`) mirroring the
SubagentStop envelope. This is the second additive-ABI extension under E-8 (the first
was D-6 Option A — `host::write_file` in S-8.10).

Outcome: HOST_ABI_VERSION stays at 1. BC-2.02.012 is the canonical contract.
S-8.01 and S-8.03 reopen to update T-3 task bindings to reference specific
`payload.<field_name>` access patterns.

Sanctioned precedent for all future envelope schema additions: follow this pattern
unless the change requires a breaking ABI constraint.

## Drift / Known Issues

- **DRIFT-001 (P1 — medium):** `read_file` shim in the SDK calls a host fn that
  is a stub on the dispatcher side (invoke.rs StoreData-typed linker). The shim
  itself is correct; the dispatcher does not execute it. Must-fix before rc.1.
- **L-P1-007 (debt):** `codes::*` constants duplicated between `factory-dispatcher::host`
  and `hook-sdk::host::HostError::from_code`. No drift today; candidate for
  extraction to a shared `host-codes` crate.
- **L-P1-002 (debt):** `#[deny(missing_docs)]` not yet applied to `hook-sdk`.
  Docs are present in practice but not attribute-enforced.

## Process Gaps

**[process-gap-D-183-A]** SDK-surface story specs MUST verify typed-projection binding
against `HookPayload` struct fields, not just describe semantics in prose.

S-8.01 and S-8.03 each reached CONVERGENCE_REACHED (pass-5 and pass-3 respectively)
while their T-3 task descriptions said vague "parse JSON from stdin" without
specifying which `HookPayload` fields carry the SubagentStop envelope's `agent_type`
and `last_assistant_message` — because those fields did not exist in `HookPayload`
at the time. The structural gap was discovered during D-183 triage.

**Going forward:** Pass-1 adversary on any SDK-surface story MUST audit T-N task
code snippets against the `HookPayload` struct definition in
`crates/hook-sdk/src/payload.rs`. Absence of a specific `payload.<field_name>`
access pattern in task code snippets is a HIGH finding when the story's bash
reference hook reads top-level envelope fields via jq. Stories may not cite
`tool_input` as the source for SubagentStop fields unless the field genuinely
lives in `tool_input` (it does not for SubagentStop).

Concretely: before any SDK-surface story reaches CONVERGENCE_REACHED, confirm that
every T-N task that accesses envelope data explicitly references the `HookPayload`
field (or explicitly cites the BC that defines the projection). A BC citation without
a field-level binding is insufficient — the BC must exist and enumerate the specific
fields by name.

## host::run_subprocess (BC-2.02.013)

Added in W-16 (ADR-014). This is the third additive host function extension after
D-6 Option A (`host::write_file`, S-8.10) and D-183 (HookPayload SubagentStop
fields, S-8.30). `HOST_ABI_VERSION` stays at 1.

### Function Signature

```rust
/// Execute a subprocess with capability-gated binary + arg + env allow-list.
/// Returns `Ok(SubprocessResult)` on completion (any exit code).
/// Returns `Err(HostError::CapabilityDenied)` if binary or args violate the allow-list.
/// Returns `Err(HostError::Timeout)` if the process exceeds `caps.max_timeout_ms`.
pub fn run_subprocess(spec: &SubprocessSpec) -> Result<SubprocessResult, HostError>;
```

### SubprocessSpec

```rust
pub struct SubprocessSpec {
    /// Absolute or relative path to the binary to execute (no shell expansion).
    pub binary: String,
    /// Arguments to pass directly to the binary (no shell interpretation).
    pub args: Vec<String>,
    /// Environment variables to set. Variables not in `SubprocessCaps.env_allowlist`
    /// are stripped before exec regardless of what is set here.
    pub env: HashMap<String, String>,
    /// Wall-clock timeout in milliseconds. Capped at `caps.max_timeout_ms`.
    pub timeout_ms: u64,
    /// Working directory for the subprocess, relative to CLAUDE_PROJECT_DIR.
    /// `None` means inherit dispatcher working directory.
    pub working_dir: Option<PathBuf>,
}
```

### SubprocessResult

```rust
pub struct SubprocessResult {
    /// Process exit code (0 = success by convention; negative = signal termination).
    pub exit_code: i32,
    /// Raw stdout bytes, capped at `SubprocessCaps.max_stdout_bytes`.
    pub stdout: Vec<u8>,
    /// Raw stderr bytes, capped at `SubprocessCaps.max_stderr_bytes`.
    pub stderr: Vec<u8>,
    /// Actual wall-clock duration of subprocess execution in milliseconds.
    pub duration_ms: u64,
    /// `true` if stdout or stderr was truncated due to byte caps.
    pub truncated: bool,
}
```

### SubprocessCaps Schema

Declared in `hooks-registry.toml` under `[hooks.capabilities.run_subprocess]`:

```rust
pub struct SubprocessCaps {
    /// Glob patterns for permitted binary paths.
    /// The dispatcher resolves `SubprocessSpec.binary` to an absolute path before
    /// matching. Path traversal (`../`) in `binary` is rejected before glob matching.
    /// Example: ["/usr/bin/git", "*/hooks/verify-sha-currency.sh"]
    pub binary_allowlist: Vec<String>,

    /// Glob patterns for permitted argument strings.
    /// Each element of `SubprocessSpec.args` must match at least one pattern.
    /// Example: ["--version", "show*", "HEAD:*", "--project-root*"]
    pub arg_allowlist: Vec<String>,

    /// Environment variable names the subprocess may inherit.
    /// All other env vars are removed from the subprocess environment before exec.
    /// An empty list means the subprocess inherits zero env vars.
    pub env_allowlist: Vec<String>,

    /// Maximum bytes captured from stdout. Excess bytes are discarded and
    /// `SubprocessResult.truncated` is set to `true`. Default: 1_048_576 (1 MiB).
    pub max_stdout_bytes: u64,

    /// Maximum bytes captured from stderr. Excess bytes are discarded and
    /// `SubprocessResult.truncated` is set to `true`. Default: 262_144 (256 KiB).
    pub max_stderr_bytes: u64,

    /// Maximum wall-clock milliseconds allowed for subprocess execution.
    /// The process is killed (SIGKILL) if this limit is exceeded.
    /// Default: 30_000 (30 seconds). Hard cap enforced by dispatcher.
    pub max_timeout_ms: u64,
}
```

**Security boundaries (enforced by dispatcher, not the plugin):**
- NO shell interpretation. `binary` is exec'd directly via `execvp`-equivalent.
  Shell interpreter binaries (`bash`, `sh`, `zsh`, `fish`) are not implicitly
  permitted; they must appear explicitly in `binary_allowlist` with justification.
- NO path traversal. `../` sequences in `binary` are rejected before allow-list
  matching with `HostError::CapabilityDenied`.
- Env filtering is unconditional. Even if the plugin passes env vars in
  `SubprocessSpec.env`, vars not in `env_allowlist` are stripped.
- `max_stdout_bytes` and `max_stderr_bytes` default to 1 MiB and 256 KiB
  respectively if not declared. Dispatcher enforces the cap; plugins cannot
  exceed it by passing a larger value in `SubprocessSpec.timeout_ms`.

### Module Entry

Add to the Modules table:

| Module / File | Responsibility |
|---|---|
| `crates/hook-sdk/src/host/run_subprocess.rs` | `vsdd::run_subprocess(spec_json)` extern shim; returns `SubprocessResult` |

### Behavioral Contract

BC-2.02.013 (authored by PO): `host::run_subprocess` invariants — binary allow-list
enforcement, arg allow-list enforcement, env stripping, timeout enforcement,
`SubprocessResult` typed contract, `HOST_ABI_VERSION = 1` stability.

---

## HookPayload Schema Evolution (updated)

### Additive-ABI Extension Pattern (updated for W-16)

| Decision | Story | Extension | HOST_ABI_VERSION |
|----------|-------|-----------|-----------------|
| D-6 Option A | S-8.10 | `host::write_file` — additive host function; anchors BC-2.02.011 | Stays at 1 |
| D-183 | S-8.30 | `HookPayload` SubagentStop fields — additive struct fields; anchors BC-2.02.012 | Stays at 1 |
| D-9.2 (ADR-014) | SDK-ext (W-16) | `host::run_subprocess` — additive host function; anchors BC-2.02.013 | Stays at 1 |

---

## Schema Evolution Table (BC-2.02.x family)

| BC ID | Contract | Introduced | HOST_ABI_VERSION |
|-------|----------|------------|-----------------|
| BC-2.02.011 | `host::write_file` shim: write to allow-listed path | S-8.10 | 1 |
| BC-2.02.012 | `HookPayload` SubagentStop fields (`agent_type`, etc.) | S-8.30 | 1 |
| BC-2.02.013 | `host::run_subprocess` — SubprocessCaps enforcement, SubprocessResult contract | SDK-ext (W-16) | 1 |

---

## Change Log

| Date | Change |
|------|--------|
| 2026-05-03 | ADR-014 D-9.2: added `host::run_subprocess` function signature, `SubprocessSpec`, `SubprocessResult`, `SubprocessCaps` schema (6 fields), security boundaries, module entry, BC-2.02.013 anchor, and Schema Evolution table row. HOST_ABI_VERSION stays at 1. |
| 2026-05-01 | F-S830-P1-004 fix: fallback-chain example aligned with BC-2.02.012 canonical (`as_deref()` borrowing chain returning `&str`); architecture doc no longer diverges from BC. Both agent identity and assistant-message chains updated. Prose translation pattern updated from consuming to borrowing form. |
