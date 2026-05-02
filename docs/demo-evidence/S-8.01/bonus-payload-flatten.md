# Bonus: HookPayload `#[serde(flatten)]` Fix in factory-dispatcher

**File:** `crates/factory-dispatcher/src/payload.rs`
**Severity:** Critical workspace-shared fix
**Impact:** S-8.30 typed projection fields (and all future event-specific fields)
now flow through the dispatcher to plugins without silent data loss.

## Problem

The `factory-dispatcher`'s `HookPayload` struct (the pre-dispatch envelope
shape) previously modeled only a fixed set of fields:
`event_name`, `tool_name`, `session_id`, `tool_input`, `tool_response`.

Any field not explicitly named was silently dropped during serde
deserialization. For SubagentStop events, this meant:
- `agent_type` — dropped
- `subagent_name` — dropped
- `last_assistant_message` — dropped
- `result` — dropped

These are exactly the four BC-2.02.012 typed projection fields that
S-8.30 added to `vsdd_hook_sdk::HookPayload`. Without the flatten fix,
the dispatcher was stripping them before the payload ever reached the plugin.
The handoff-validator WASM plugin would have received a payload where all
four fields resolved to `None`, making it impossible to:

1. Identify the subagent (fallback chain always returned `"unknown"`)
2. Evaluate the assistant message (fallback chain always returned `""`)
3. Trigger the `subagent_empty_result` warning for the right semantic reason

In other words: every SubagentStop would have produced an empty-result
warning regardless of actual content — silently broken behavior.

## Fix

`crates/factory-dispatcher/src/payload.rs` — added `#[serde(flatten)]`
extra field to the dispatcher's `HookPayload`:

```rust
/// Pass-through fields not explicitly modeled above. This captures
/// event-specific fields (e.g. SubagentStop's `agent_type`,
/// `subagent_name`, `last_assistant_message`, `result` per
/// BC-2.02.012) so they are forwarded to plugins unchanged.
/// The dispatcher only needs `event_name` and `tool_name` for
/// routing; all other fields are plugin-owned. Using `flatten`
/// ensures unknown fields survive the parse→serialize round-trip.
#[serde(flatten)]
pub extra: std::collections::HashMap<String, serde_json::Value>,
```

The dispatcher now:
1. Deserializes the raw stdin JSON into `HookPayload` with `extra` capturing
   all fields not explicitly modeled.
2. Serializes `HookPayload` back to JSON for the plugin's stdin, including
   all `extra` fields.
3. The plugin's `vsdd_hook_sdk::HookPayload` receives the full envelope,
   including `agent_type`, `subagent_name`, `last_assistant_message`, `result`.

## Why This Is Critical for S-8.30

S-8.30 added the four BC-2.02.012 fields to `vsdd_hook_sdk::HookPayload`.
But those fields are only useful if the dispatcher passes them through.
The flatten fix is the "pipe" that connects Claude Code's envelope to the
plugin's typed projection. Without it:

- S-8.30's typed fields exist in the SDK struct but are always `None` at runtime.
- handoff-validator (S-8.01) can never read real agent identity or messages.
- All future E-8 Tier 1 WASM plugins that consume SubagentStop fields face
  the same silent breakage.

## Scope

This fix is in `crates/factory-dispatcher/` (SS-01 Hook Dispatcher Core),
not in `crates/hook-plugins/handoff-validator/` (SS-04). It is a
workspace-shared fix: all plugins across all event types benefit from the
flatten round-trip preservation. It is not scoped to SubagentStop or
handoff-validator alone.

## Verification

The bats integration tests (AC-005, 7/7 pass) provide end-to-end verification:
they pipe SubagentStop JSON through the real dispatcher to the real
handoff-validator.wasm plugin and assert that `agent_type` reaches the plugin
(e.g., `assert_plugin_stderr_contains "empty result"` works because the
plugin correctly reads the empty `last_assistant_message` from the payload).
If the flatten fix were absent, case (c) and case (e) would falsely emit
warnings (empty fallback), and cases (a)/(f) would still warn but for the
wrong reason.

## Wave-Gate Consideration

This fix was discovered during S-8.01 implementation (T-0 STOP CHECK /
T-3 integration). It should be explicitly noted in the S-8.01 PR description
and flagged for wave-gate review (W-15) as a cross-subsystem fix (SS-01
fixing a gap that unblocks SS-04 + SS-02 consumers). The fix is included in
commit `6f2e0e7`.
