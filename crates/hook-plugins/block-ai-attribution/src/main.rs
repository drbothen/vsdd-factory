//! WASI command entry point for block-ai-attribution.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's `__internal::run`
//! trampoline, calls `on_hook_logic`, and exits.
//!
//! Unit-testable logic lives in `src/lib.rs` (`on_hook_logic`); this file
//! wires the pure logic to the WASI runtime entry point.
//!
//! # rc.7 migration note
//! Previously this entry point lived inside `lib.rs` as a `wasm_entry` module
//! using the `#[hook]` macro with `crate-type = ["cdylib", "rlib"]`. That
//! produced an underscored output filename (`block_ai_attribution.wasm`)
//! because Cargo's default `[lib]` name converts hyphens to underscores. The
//! registry expects the hyphenated form (`block-ai-attribution.wasm`), so
//! the dispatcher silently failed to load this plugin (`plugin file not
//! found` crash, every PreToolUse). Switching to the `[[bin]]` pattern with
//! an explicit hyphenated bin name produces the correct filename.

use block_ai_attribution::on_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    on_hook_logic(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
