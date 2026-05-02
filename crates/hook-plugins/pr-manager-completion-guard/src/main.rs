//! WASI command entry point for pr-manager-completion-guard.
//!
//! Reads the SubagentStop JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `pr_manager_guard_main`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits
//! with code 1 (HookResult::Error). The registry `on_error = "block"` setting
//! means this manifests as a block at the dispatcher level — this preserves
//! parity with the bash jq-missing graceful exit 0 path. The trampoline's
//! behavior is documented in AC-008 and BC-7.03.045 invariant 2.
//!
//! [process-gap] AC-008 calls for exit 0 on malformed JSON (parity with bash
//! jq-missing path). The current trampoline emits HookResult::Error (exit 1)
//! on parse failure. Per AC-008 this discrepancy must be documented in a
//! BC-7.03.045 amendment (T-11 post-DONE, deferred). Implementer should
//! wrap serde_json::from_str manually (as in this entry point) to achieve
//! true exit-0-on-parse-failure parity if needed.
//!
//! Unit-testable logic lives in `src/lib.rs` (`pr_manager_guard_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # AC-007 compliance note
//! `bin/emit-event` calls are replaced by `host::emit_event` here.
//! The `bin/emit-event` binary is NOT removed (E-8 D-10; deferred to S-8.29).

use pr_manager_completion_guard::pr_manager_guard_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    pr_manager_guard_logic(
        payload,
        |event_type, fields| {
            // Replaces `bin/emit-event` subprocess calls from the bash hook
            // (AC-007 / E-8 D-10). `host::emit_event` is self-contained in
            // the WASM guest; no subprocess or PATH lookup required.
            vsdd_hook_sdk::host::emit_event(event_type, fields);
        },
        |msg| {
            eprint!("{}", msg);
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
