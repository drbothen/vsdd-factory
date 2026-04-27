//! WASI command entry point for capture-commit-activity.
//!
//! The `#[hook]` macro generates a `main()` that reads the payload from
//! stdin, calls `on_hook`, serializes the result to stdout, and exits.
//! We use the SDK's `__internal::run` trampoline here so unit tests in
//! `src/lib.rs` can drive `commit_hook_logic` directly without wasmtime.

use vsdd_hook_sdk::{HookPayload, HookResult, host};
use capture_commit_activity::{GitLogOutcome, call_git_log, commit_hook_logic, is_git_commit_command};

fn on_hook(payload: HookPayload) -> HookResult {
    unimplemented!("on_hook: implementer wires commit_hook_logic to host fn surface here")
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
