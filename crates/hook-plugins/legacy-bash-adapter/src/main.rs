//! `legacy-bash-adapter` WASI command entry point.
//!
//! All logic lives in `lib.rs`; this binary is just the `#[hook]`
//! seam that the dispatcher loads at runtime.

use legacy_bash_adapter::{adapter_logic, run_bash_via_host};
use vsdd_hook_sdk::{HookPayload, HookResult, hook};

#[hook]
pub fn on_hook(payload: HookPayload) -> HookResult {
    adapter_logic(payload, run_bash_via_host)
}
