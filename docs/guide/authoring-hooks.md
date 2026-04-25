# Authoring vsdd-factory hooks (v1.0)

> **Status:** skeleton — filled in by stories S-1.3, S-1.4, S-2.5, S-3.1, S-3.2, S-3.3.
> Section bodies are deliberately empty until the story that owns each
> section ships. Greppable `TODO(S-X.Y)` markers gate the 1.0.0 release per
> S-5.7 acceptance criteria.

This guide is for developers writing new hooks against the vsdd-factory
v1.0 Factory Plugin Kit. Hooks are WebAssembly modules compiled against
the `vsdd-hook-sdk` crate; the dispatcher loads them at runtime and routes
Claude Code events to them through a small, capability-bounded host
function surface.

## Introduction

<!-- TODO(S-1.3): explain the v1.0 hook model — WASM cdylib, no shell,
     single dispatcher, registry-driven loading, capabilities-by-default-deny.
     Contrast with v0.79.x bash hooks for context. -->

## SDK overview

<!-- TODO(S-1.3): walk the public surface of `vsdd-hook-sdk` — the
     `#[hook]` macro, the `Event` enum, the `HostCtx` handle, the
     `Result` shape returned to the dispatcher. -->

## Plugin structure

<!-- TODO(S-2.5): show the canonical Cargo.toml for a hook crate (cdylib,
     wasm32-wasip1, hook-sdk dep), recommended directory layout, naming
     conventions. -->

## Using vsdd-hook-sdk from crates.io

Once the SDK is published (see [Releasing the SDK](#releasing-the-sdk)),
external authors add the dep with `cargo add` and write a minimal
`#[hook]` plugin:

```bash
cargo new --lib my-hook && cd my-hook
cargo add vsdd-hook-sdk
```

```rust
use vsdd_hook_sdk::{hook, host, HookPayload, HookResult};

#[hook]
pub fn on_hook(payload: HookPayload) -> HookResult {
    host::log_info(&format!("event={}", payload.event_name));
    HookResult::Continue
}
```

Build with `cargo build --target wasm32-wasip1 --release`. The
resulting `.wasm` is a complete WASI command the dispatcher can load
once registered in `hooks-registry.toml`.

## Host functions

<!-- TODO(S-1.4): document the bounded host function surface —
     `exec_subprocess(cmd, args, timeout_ms, max_output_bytes)`,
     `read_file(path, max_bytes, timeout_ms)`, `emit_event(event)` —
     with examples and the timeout/byte-limit caps that hooks must respect. -->

## Capabilities

<!-- TODO(S-1.4, S-1.5): explain the capability declaration in
     hooks-registry.toml — what `exec_subprocess` requires (binary
     allow-list, env allow-list, cwd allow-list, `shell_bypass_acknowledged`),
     what `read_file` requires, and the deny-by-default model. -->

## Testing

<!-- TODO(S-3.1): show the test harness pattern — building the wasm,
     loading it under wasmtime in-process, sending mock events,
     asserting outputs and emitted events. -->

## Deployment

<!-- TODO(S-2.4, S-2.6): describe how a built `.wasm` lands in the
     plugin distribution, how `hooks-registry.toml` references it,
     and how the activation skill picks the right per-platform
     dispatcher binary. -->

## Releasing the SDK

The `vsdd-hook-sdk` and `vsdd-hook-sdk-macros` crates version and
release **independently** of the factory plugin (`plugins/vsdd-factory`)
and the `factory-dispatcher` binary. SDK consumers are external Rust
crates; the factory's own version cadence is irrelevant to them.

### Pre-publish checklist

Run these before every real `cargo publish` of the SDK:

```bash
cargo fmt --check --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build -p vsdd-hook-sdk --example hello-hook --target wasm32-wasip1
cargo publish --dry-run --workspace -p vsdd-hook-sdk-macros -p vsdd-hook-sdk
```

The CI `cargo` job runs the dry-run on every push so publishability
regressions surface immediately.

### Publish order

The macros crate must be on crates.io **before** the parent SDK,
because the SDK's `Cargo.toml` carries a hard `version = "X.Y.Z"`
pin against the macros sibling. Cargo cannot resolve the parent
until the macros version exists in the registry.

```bash
cargo publish -p vsdd-hook-sdk-macros
# wait for the index to propagate (usually <1 min)
cargo publish -p vsdd-hook-sdk
```

### Token setup

The real publish currently runs from a maintainer's workstation
with a personal `cargo login` token. A future improvement is to
add a CI workflow that consumes a `CRATES_IO_TOKEN` GitHub secret
and runs the publish on a tag push; this story (S-2.5) deliberately
stops short of wiring that up.

### Version cadence

- Bug fixes / additive host wrappers → patch bump on the SDK.
- ABI-compatible additions to `HookPayload` / `HookResult` → minor
  bump on the SDK (consumers recompile, no plugin re-link needed).
- ABI breaking changes (`HOST_ABI_VERSION` increments) → major bump
  on **both** crates and on `factory-dispatcher` per the S-5.6
  semver commitment.

Always bump both crates together (lockstep), even if only the
parent changed: it keeps the version pin in `crates/hook-sdk/Cargo.toml`
trivial to maintain and removes a class of "did I bump the right
one?" mistakes.
