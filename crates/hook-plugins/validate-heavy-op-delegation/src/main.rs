//! WASI command entry point for validate-heavy-op-delegation.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — `on_error = "continue"` per BC-4.15.001 PC-C).
//!
//! Unit-testable logic lives in `src/lib.rs` (`evaluate_patterns`,
//! `truncate_command_preview`, `build_recommendation_message`, and
//! `on_pre_tool_use`); this file wires the pure logic to real host function
//! calls via the SDK trampoline.
//!
//! # Compliance notes (BC-4.15.001)
//! - `on_error = "continue"` in the registry: WASM crash → fail-open Continue.
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Host-call surface (INV1 audit): on an advisory match exactly two calls are
//!   made — `eprintln!` writes the stderr nudge to WASM stdio (flows through to
//!   the dispatcher process stderr; PC-B-B1), and
//!   `host::emit_event("plugin.log", ...)` writes the structured
//!   DelegationRecommended record to the dispatcher internal JSONL log
//!   (PC-B-B2; `emit_event` is defined in `crates/hook-sdk/src/host.rs`).
//!   On no-match both calls are skipped entirely. There is NO `host::read_file`,
//!   NO subprocess invocation, NO network call — pure-parse (INV1).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - Never blocks: block_intent is ALWAYS false (INV2).

use validate_heavy_op_delegation::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
