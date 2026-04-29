//! WASI command entry point for worktree-hooks.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_worktree_event`, serializes the result to stdout, and exits.
//! Unit tests and integration tests in `tests/` drive `worktree_hook_logic`
//! directly without a WASM runtime.

use vsdd_hook_sdk::HookPayload;
use vsdd_hook_sdk::HookResult;
use worktree_hooks::on_worktree_event;

fn on_hook(payload: HookPayload) -> HookResult {
    on_worktree_event(payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
