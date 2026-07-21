//! WASI command entry point for verify-state-timestamp-advisory.
//!
//! Reads the PostToolUse JSON payload from stdin via the SDK's `__internal::run`
//! trampoline, calls `on_hook_logic`, and exits.
//!
//! Unit-testable logic lives in `src/lib.rs` (`on_hook_logic`); this file wires
//! the pure logic to the WASI runtime entry point.
//!
//! The `write_advisory` callback is wired as `|s| eprintln!("{s}")` so advisory
//! text is written to the plugin's stderr (captured and surfaced by the dispatcher).

use verify_state_timestamp_advisory::on_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    on_hook_logic(payload, |s| eprintln!("{s}"))
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
