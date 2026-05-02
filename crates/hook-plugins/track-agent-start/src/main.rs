//! WASI command entry point for track-agent-start.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_agent_start`, serializes the result to stdout, and exits.
//! Unit tests and integration tests drive `track_agent_start_logic`
//! directly without a WASM runtime.

use track_agent_start::on_agent_start;
use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;

fn on_hook(payload: HookPayload) -> HookResult {
    on_agent_start(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
