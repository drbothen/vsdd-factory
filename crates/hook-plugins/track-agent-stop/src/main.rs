//! WASI command entry point for track-agent-stop.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_agent_stop`, serializes the result to stdout, and exits.
//! Unit tests and integration tests drive `track_agent_stop_logic`
//! directly without a WASM runtime.

use track_agent_stop::on_agent_stop;
use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;

fn on_hook(payload: HookPayload) -> HookResult {
    on_agent_stop(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
