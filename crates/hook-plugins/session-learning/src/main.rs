//! WASI command entry point for session-learning.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_hook`, serializes the result to stdout, and exits.
//! Unit tests and bats integration tests in `tests/integration/hooks/`
//! drive `session_learning_logic` directly without a WASM runtime.
//!
//! Behavioral contracts: BC-7.03.076, BC-7.03.077, BC-7.03.078

use session_learning::session_learning_logic;
use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;

fn on_hook(_payload: HookPayload) -> HookResult {
    // session-learning does NOT parse the stdin envelope (S-8.06 T-3, no stdin parse).
    // The payload has already been read from stdin by __internal::run before this
    // function is called. stdin drain is handled in session_learning_logic.
    session_learning_logic(session_learning::format_utc_now, ".")
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
