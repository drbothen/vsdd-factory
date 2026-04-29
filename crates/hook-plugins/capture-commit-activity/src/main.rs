//! WASI command entry point for capture-commit-activity.
//!
//! The `#[hook]` macro generates a `main()` that reads the payload from
//! stdin, calls `on_hook`, serializes the result to stdout, and exits.
//! We use the SDK's `__internal::run` trampoline here so unit tests in
//! `src/lib.rs` can drive `commit_hook_logic` directly without wasmtime.

use capture_commit_activity::commit_hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    commit_hook_logic(
        payload,
        || match vsdd_hook_sdk::host::exec_subprocess(
            "git",
            &["log", "-1", "--format=%H"],
            &[],
            5000,
            1024,
        ) {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                Ok((result.exit_code, stdout))
            }
            Err(e) => Err(format!("{e:?}")),
        },
        |fields| {
            vsdd_hook_sdk::host::emit_event(
                "commit.made",
                &[
                    ("sha", fields.sha.as_str()),
                    ("branch", fields.branch.as_str()),
                    ("message", fields.message.as_str()),
                    ("author", fields.author.as_str()),
                    ("timestamp", fields.timestamp.as_str()),
                ],
            );
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
