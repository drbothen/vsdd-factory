//! WASI command entry point for validate-pr-review-posted.
//!
//! Thin trampoline: reads payload from stdin via the SDK's `__internal::run`
//! helper, calls `on_hook`, and exits. Mirroring the `capture-commit-activity`
//! pattern (src/main.rs:42-44).

use validate_pr_review_posted::validate_pr_review_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    validate_pr_review_logic(payload, |agent| {
        vsdd_hook_sdk::host::emit_event(
            "hook.block",
            &[
                ("hook", "validate-pr-review-posted"),
                ("matcher", "SubagentStop"),
                ("reason", "pr_review_not_posted"),
                ("subagent", agent),
            ],
        );
    })
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
