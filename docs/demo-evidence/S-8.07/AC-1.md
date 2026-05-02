# AC-001 Evidence: WASM Crate Exists and Builds for wasm32-wasip1

**AC statement:** WASM crate `crates/hook-plugins/warn-pending-wave-gate/` exists with
`Cargo.toml` targeting `wasm32-wasip1`, implements the `vsdd_hook_sdk` hook interface,
and builds successfully. Registry entry updated with native plugin path, `read_file`
capability declared, `script_path`/`shell_bypass_acknowledged` removed. Event binding
preserved: `event = "Stop"`, `priority = 920`, `on_error = "continue"`.

**BC trace:** BC-7.03.091 postcondition 1 (identity & registry binding)

---

## Crate Scaffold Verified

File layout at `crates/hook-plugins/warn-pending-wave-gate/`:

```
Cargo.toml
src/
  lib.rs     — hook logic (testable without WASM runtime)
  main.rs    — WASI command entry point, calls lib
tests/
  integration_test.rs
```

`Cargo.toml` declares:
- `[[bin]] name = "warn-pending-wave-gate" path = "src/main.rs"` — WASI command entry point
- `[lib] path = "src/lib.rs"` — testable logic layer
- Dependencies: `vsdd-hook-sdk = { path = "../../hook-sdk" }`, `serde_yaml = { workspace = true }`, `serde = { workspace = true }`
- No `python3`, no `exec_subprocess`, no `legacy-bash-adapter` dependency

The crate is registered in the workspace root `Cargo.toml` members array.

## Registry Entry Verified

`plugins/vsdd-factory/hooks-registry.toml` (lines ~864-876):

```toml
[[hooks]]
name = "warn-pending-wave-gate"
event = "Stop"
plugin = "hook-plugins/warn-pending-wave-gate.wasm"
priority = 920
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory/wave-state.yaml"]
```

Confirmed absent from new entry:
- `script_path` — removed
- `shell_bypass_acknowledged` — removed
- `[hooks.capabilities.exec_subprocess]` — removed entirely

## WASM Build Artifact

The `.wasm` artifact is compiled from `src/main.rs` (direct `fn main()` entry point,
no `#[hook]` macro — per capture-commit-activity sibling pattern). The binary receives
the Stop hook JSON envelope on stdin via `vsdd_hook_sdk::__internal::run(on_hook)`.

**Result: PASS** — Crate exists, builds for wasm32-wasip1, registry entry is correct.
