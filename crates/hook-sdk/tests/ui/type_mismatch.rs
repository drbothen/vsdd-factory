// AC-004 trybuild test: ResolverInput and HookPayload are distinct types with no implicit conversion.
//
// This crate attempts to assign a HookPayload where a ResolverInput is expected.
// Per BC-4.12.002 INV1, there is no From/Into/Deref between the two types —
// the compiler must reject this with a type mismatch error.
//
// The matching .stderr file specifies the expected error.
//
// Traces: AC-004, BC-4.12.002 invariant 1.

use vsdd_hook_sdk::resolver::ResolverInput;
use vsdd_hook_sdk::HookPayload;

fn requires_resolver_input(_: ResolverInput) {}

fn main() {
    let payload: HookPayload = serde_json::from_str(
        r#"{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s","dispatcher_trace_id":"t"}"#,
    )
    .unwrap();
    // BC-4.12.002 INV1: HookPayload cannot be used as ResolverInput.
    // This line must produce a compile-time type mismatch error.
    requires_resolver_input(payload);
}
