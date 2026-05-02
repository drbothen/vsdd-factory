//! WASI command entry point for handoff-validator.
//!
//! Reads the SubagentStop JSON payload from stdin via the SDK's `__internal::run`
//! trampoline, calls `handoff_validator_main`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are caught
//! by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation per BC-7.03.042 invariant 2 / AC-006 — advisory-only).
//!
//! Unit-testable logic lives in `src/lib.rs` (`handoff_validator_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # AC-007 compliance note
//! `bin/emit-event` calls are replaced by `host::emit_event` here.
//! The `bin/emit-event` binary is NOT removed (E-8 D-10; deferred to S-8.29).

use handoff_validator::handoff_validator_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    handoff_validator_logic(
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
        |line| {
            // Advisory block signal to dispatcher (W-15 gate fix CRIT-PR59-001):
            // dispatcher reads {"outcome":"block",...} on stdout regardless of on_error.
            println!("{}", line);
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
