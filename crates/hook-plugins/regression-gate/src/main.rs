//! WASI command entry point for regression-gate.
//!
//! Reads the PostToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_post_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — advisory hook, on_error=continue).
//!
//! Unit-testable logic lives in `src/lib.rs` (`regression_gate_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-7.03.071..075)
//! - `bin/emit-event` calls replaced by `host::emit_event` (AC-008).
//!   `bin/emit-event` binary is NOT removed (E-8 D-10; deferred to S-8.29).
//! - No jq subprocess (AC-009 confirmed by OQ-6 audit: binary_allow=[]).
//! - No legacy-bash-adapter dependency (forbidden per E-8 D-10).
//! - HOST_ABI_VERSION = 1 unchanged.

use regression_gate::on_post_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_post_tool_use);
}
