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
