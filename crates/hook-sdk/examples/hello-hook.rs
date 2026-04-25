//! `hello-hook` — minimal vsdd-factory hook plugin.
//!
//! Build with:
//!
//! ```bash
//! cargo build --example hello-hook --target wasm32-wasip1 --release
//! ```
//!
//! The output `target/wasm32-wasip1/release/examples/hello_hook.wasm`
//! is a valid WASI command and can be loaded by `factory-dispatcher`
//! once S-1.5 lands.
//!
//! What it does: logs the event/tool combination through the dispatcher
//! internal log, emits a `hello.fired` event, and always returns
//! `Continue`.

use vsdd_hook_sdk::{HookPayload, HookResult, hook, host};

#[hook]
pub fn on_hook(payload: HookPayload) -> HookResult {
    host::log_info(&format!(
        "hello-hook: event={} tool={} session={}",
        payload.event_name, payload.tool_name, payload.session_id,
    ));

    host::emit_event(
        "hello.fired",
        &[
            ("event_name", payload.event_name.as_str()),
            ("tool_name", payload.tool_name.as_str()),
            ("session_id", payload.session_id.as_str()),
            ("dispatcher_trace_id", payload.dispatcher_trace_id.as_str()),
        ],
    );

    HookResult::Continue
}
