# vsdd-hook-sdk-macros

Procedural macro internals for [`vsdd-hook-sdk`][sdk]. **Do not depend
on this crate directly** — `vsdd-hook-sdk` re-exports everything you
need (`#[hook]`, `HookPayload`, `HookResult`, `host::*`) and pins the
matching macro version itself.

## What lives here

- The `#[hook]` attribute macro that wraps a
  `fn(HookPayload) -> HookResult` in a WASI command entry point with
  stdin parsing, panic boundary, and result framing.

That's the entire surface. The crate exists as a separate publication
unit only because Cargo requires `proc-macro = true` libraries to be
their own crate; it is not intended to be a public API.

## Use `vsdd-hook-sdk` instead

```toml
[dependencies]
vsdd-hook-sdk = "0.1"
```

```rust
use vsdd_hook_sdk::{hook, host, HookPayload, HookResult};

#[hook]
pub fn on_hook(payload: HookPayload) -> HookResult {
    host::log_info(&format!("event: {}", payload.event_name));
    HookResult::Continue
}
```

## ABI documentation

Cross-language host ABI: see the parent crate's
[`HOST_ABI.md`](https://github.com/drbothen/vsdd-factory/blob/main/crates/hook-sdk/HOST_ABI.md).

## License

MIT — see [`LICENSE`](https://github.com/drbothen/vsdd-factory/blob/main/LICENSE).

[sdk]: https://crates.io/crates/vsdd-hook-sdk
