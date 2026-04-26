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

## ADRs

- ADR-002: WASM (wasmtime) plugin ABI — `decisions/ADR-002-wasm-plugin-abi.md`
- ADR-003: WASI preview 1 for v1.0; preview 2 deferred — `decisions/ADR-003-wasi-preview1.md`
- ADR-006: HOST_ABI_VERSION as separate semver constant — `decisions/ADR-006-host-abi-version.md`
- ADR-010: StoreData-typed linker for host functions — `decisions/ADR-010-storedata-linker.md`

## Drift / Known Issues

- **DRIFT-001 (P1 — medium):** `read_file` shim in the SDK calls a host fn that
  is a stub on the dispatcher side (invoke.rs StoreData-typed linker). The shim
  itself is correct; the dispatcher does not execute it. Must-fix before rc.1.
- **L-P1-007 (debt):** `codes::*` constants duplicated between `factory-dispatcher::host`
  and `hook-sdk::host::HostError::from_code`. No drift today; candidate for
  extraction to a shared `host-codes` crate.
- **L-P1-002 (debt):** `#[deny(missing_docs)]` not yet applied to `hook-sdk`.
  Docs are present in practice but not attribute-enforced.
