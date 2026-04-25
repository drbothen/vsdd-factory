# vsdd-hook-sdk

Author vsdd-factory v1.0 hook plugins as WebAssembly modules. Provides:

- The `#[hook]` proc-macro — turns a `fn(HookPayload) -> HookResult` into
  a complete WASI command entry point with stdin parsing, panic
  boundary, and result framing.
- `HookPayload` / `HookResult` — typed projections of the dispatcher's
  envelope.
- Ergonomic wrappers (`vsdd_hook_sdk::host::*`) over the `vsdd` host
  module: `log`, `emit_event`, `read_file`, `exec_subprocess`,
  `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`,
  `cwd`, `env`.
- `HOST_ABI_VERSION` — the ABI version this SDK speaks.

## Quick start

```rust
use vsdd_hook_sdk::{hook, host, HookPayload, HookResult};

#[hook]
pub fn on_hook(payload: HookPayload) -> HookResult {
    host::log_info(&format!("event: {}", payload.event_name));
    HookResult::Continue
}
```

Build:

```bash
cargo build --target wasm32-wasip1 --release
```

The `.wasm` file in `target/wasm32-wasip1/release/` is a valid plugin
the dispatcher can load by registering it in `hooks-registry.toml`.

## Status

Pre-publish. Will move to crates.io as part of S-2.5 alongside the
1.0.0-beta.1 milestone. The current `0.0.x` line tracks v1.0
implementation work — APIs may shift before the first published
release.

## ABI documentation

See [`HOST_ABI.md`](HOST_ABI.md) for the cross-language ABI contract.

## License

MIT — see [`../../LICENSE`](../../LICENSE).
