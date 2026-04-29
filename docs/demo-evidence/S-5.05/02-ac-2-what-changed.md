# AC-2 — "What changed" section

**AC statement:** Section "What changed" populated (dispatcher binary,
hooks.json variants, activation requirement).

**Evidence type:** file snippet

## Section heading

```markdown
## What changed
```

## Content snippet (lines 10-31)

```markdown
v1.0 replaces the hook-by-hook bash dispatch model with a single Rust
dispatcher binary (`factory-dispatcher`) that reads a registry file and
routes events to WASM plugins. Key changes:

- **Rust dispatcher binary** replaces the per-hook bash dispatch chain.
  One process starts at session open, receives all Claude Code hook events,
  and routes them to the correct WASM plugin entry points.
- **WASM plugins** replace bash scripts as the hook execution unit.
  Plugins run in a capability-sandboxed WASM runtime (Wasmtime). Bash hooks
  from v0.79.x are not deleted — they are wrapped by `legacy-bash-adapter.wasm`,
  which invokes them via `exec_subprocess`, preserving existing behavior.
- **Activation step required.** After installing or updating the plugin, you
  must run `/vsdd-factory:activate` once to register the dispatcher with
  Claude Code's hook system. Without activation, hooks do not fire.
- **Observability config moves** from hardcoded file output to
  `observability-config.toml`. The default behavior (writing JSONL to
  `.factory/logs/`) matches v0.79.x with no config changes required.
- **Per-platform binaries** are committed to the plugin package.
  The dispatcher is a compiled Rust binary; the correct binary for your
  OS/arch is selected at activation time.
```

## Commentary

Covers all three required elements: Rust dispatcher binary (replacing bash
dispatch chain), hooks.json variants (WASM plugins + legacy-bash-adapter),
and the activation requirement (explicit `/vsdd-factory:activate` step).
