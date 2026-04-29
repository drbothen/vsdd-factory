//! WASI command entry point for tool-failure-hooks.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_post_tool_use_failure`, serializes the result to stdout, and exits.
//! Unit tests and integration tests in `tests/` drive `tool_failure_hook_logic`
//! directly without a WASM runtime.

use tool_failure_hooks::on_post_tool_use_failure;
use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;

fn on_hook(payload: HookPayload) -> HookResult {
    on_post_tool_use_failure(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
