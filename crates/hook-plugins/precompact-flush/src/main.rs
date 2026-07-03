//! WASI command entry point for precompact-flush.
//!
//! Reads the PreCompact JSON payload from stdin via the SDK's
//! `vsdd_hook_sdk::__internal::run` trampoline, calls `run_plugin`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are caught
//! by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — on_error=continue per BC-7.07.001 INV6 / AC-010).
//!
//! Unit-testable pure logic lives in `src/lib.rs`; this file wires the pure
//! logic to real host function calls via the vsdd-hook-sdk trampoline.
//!
//! # Compliance notes
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced; ADR-025 Decision 1).
//! - `binary_allow = ["git"]` ONLY in hooks-registry.toml — no bash, no shell,
//!   no shell_bypass_acknowledged (ADR-028 §Decision 2 / AC-001 / AC-014).
//! - `async = false` REQUIRED — PreCompact hooks fire synchronously before
//!   compaction (BC-7.07.001 §Precondition 1 + ADR-019).
//! - Uses only host::read_file, host::write_file, host::exec_subprocess (ABI v1).
//! - Block messages use HookResult::Block (block_intent = true; exit code 2).
//! - Crash → on_error=continue fail-open (BC-7.07.001 INV6 / AC-010 / AC-014).
//! - No unwrap() or expect() in non-test code paths (AC-010 / BC-7.07.001 INV6).
//! - Plugin MUST NOT add AI attribution to commit messages (CLAUDE.md directive).
//! - Plugin MUST NOT use --no-verify git flags (CLAUDE.md absolute prohibition).

use precompact_flush::run_plugin;

fn main() {
    vsdd_hook_sdk::__internal::run(run_plugin);
}
