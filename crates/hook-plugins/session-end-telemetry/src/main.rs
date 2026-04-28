//! WASI command entry point for session-end-telemetry.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_session_end`, serializes the result to stdout, and exits.
//! Unit tests and integration tests in `tests/` drive `session_end_hook_logic`
//! directly without a WASM runtime.

use session_end_telemetry::on_session_end;
use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;

fn on_hook(payload: HookPayload) -> HookResult {
    on_session_end(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
