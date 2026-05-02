//! WASI command entry point for validate-pr-review-posted.
//!
//! Thin trampoline: reads payload from stdin via the SDK's `__internal::run`
//! helper, calls `on_hook`, and exits. Mirroring the `capture-commit-activity`
//! pattern (src/main.rs:42-44).
//!
//! Advisory block-mode: emit `hook.block` warning event + write to stderr,
//! then return `HookResult::Continue` (exit 0). The `on_error=continue` registry
//! setting governs dispatcher crash semantics; the hook itself never exits 2.

use validate_pr_review_posted::validate_pr_review_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    validate_pr_review_logic(
        payload,
        |agent| {
            // AC-008: bare statement form — host::emit_event returns ().
            // No Result to discard; `let _ =` is NOT acceptable.
            vsdd_hook_sdk::host::emit_event(
                "hook.block",
                &[
                    ("hook", "validate-pr-review-posted"),
                    ("matcher", "SubagentStop"),
                    ("reason", "pr_review_not_posted"),
                    ("subagent", agent),
                ],
            );
        },
        |msg| {
            eprint!("{}", msg);
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
