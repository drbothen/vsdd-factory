//! WASI command entry point for capture-pr-activity.
//!
//! Reads the PostToolUse JSON payload from stdin via the SDK's `__internal::run`
//! trampoline, calls `dispatch`, and exits.
//!
//! Unit-testable logic lives in `src/lib.rs` (`dispatch` and helpers); this
//! file wires the pure logic to the WASI runtime entry point.
//!
//! # rc.7 migration note
//! Previously this crate built as `crate-type = ["cdylib", "rlib"]` with no
//! WASI entry point, producing `capture_pr_activity.wasm` (underscores) that
//! the dispatcher could not load (registry expects `capture-pr-activity.wasm`
//! with hyphens). Result: every PostToolUse fired a `plugin file not found`
//! crash. Switching to the `[[bin]]` pattern with an explicit hyphenated bin
//! name produces the correct filename and gives the lib a real WASI entry.

use capture_pr_activity::dispatch;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    dispatch(&payload)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
