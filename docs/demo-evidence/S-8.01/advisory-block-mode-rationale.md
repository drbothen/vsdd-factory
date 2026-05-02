# Advisory: Block-Mode Interpretation for handoff-validator

**Story:** S-8.01 — Native port: handoff-validator (SubagentStop)
**Type:** Architecture advisory / wave-gate flag
**Audience:** Wave-gate reviewer, S-8.10 SDK extension author, E-8 epic owner

## The Current Behavior

handoff-validator always returns `HookResult::Continue` (exit 0), even when
it detects an empty or truncated subagent result. When a warning condition is
detected, it:

1. Calls `host::emit_event("hook.block", [...])` — emits a `hook.block` event
   to the dispatcher log with `severity=warn`.
2. Writes a human-readable warning to stderr.
3. Returns `HookResult::Continue` (exit 0).

The hooks-registry.toml entry for handoff-validator has `on_error = "block"`.

## The Naming Tension

There is a semantic tension between these two facts:

| Element | Value | Meaning |
|---------|-------|---------|
| `host::emit_event` type | `hook.block` | Event *named* "hook.block" — logged as an advisory warning |
| Registry `on_error` | `"block"` | Dispatcher crash-handler behavior — blocks SubagentStop if the *plugin itself* panics/crashes |
| Hook return value | `HookResult::Continue` | Plugin says "continue processing" — does not actually block |

The term "block" appears in two distinct contexts, each with different semantics:

1. **`hook.block` event type** — This is the event name emitted via
   `host::emit_event`. It is a log entry. The dispatcher records it, but
   currently does not gate the pipeline based on it. It is an advisory signal.

2. **`on_error = "block"` in the registry** — This governs what the
   *dispatcher* does if the plugin crashes (panics, WASM trap, timeout).
   It means: if handoff-validator fails to run at all, treat that as a
   SubagentStop blocker. This is the dispatcher's crash-handler behavior.

handoff-validator's design intent is: always advisory. The hook warns operators
about potentially empty subagent output but never hard-blocks the SubagentStop
lifecycle event. A subagent that produces a 5-character result is suspicious,
but it should not be prevented from completing.

## What Is NOT Happening

There is no `HookResult::Block` variant in the current `vsdd_hook_sdk`. The
hook cannot actually block the SubagentStop event based on its own logic. The
closest analog would be returning `HookResult::Error` with a blocking message,
but even that behavior depends on the dispatcher's interpretation of error
results.

The current implementation correctly represents the design intent: emit a
machine-readable warning event, write a human-readable stderr message, and
allow the pipeline to continue.

## Wave-Gate Review Question

**Should `hook.block` events (severity=warn) be interpreted as pipeline gates?**

Two options:

**Option A (current behavior — advisory only):** `hook.block` events are
informational. The dispatcher logs them but does not gate the pipeline. The
operator sees the warning in the events log but the SubagentStop proceeds.
`on_error = "block"` remains a crash-handler-only setting. No SDK extension
needed.

**Option B (real block semantics):** Introduce a `HookResult::Block` variant
to the `vsdd_hook_sdk` (a new SDK extension story, similar in scope to
S-8.10's `host::write_file`). When a plugin returns `HookResult::Block`, the
dispatcher halts the lifecycle event and surfaces the block reason to the
operator. This would make handoff-validator a genuine enforcement gate rather
than an advisory.

## Current Factory Position

The implementation follows Option A. This is consistent with the bash
`handoff-validator.sh` behavior: it always exited 0, even when warning. The
bash script had no mechanism to block SubagentStop.

The `hook.block` event name is inherited from the bash era (the bash script
called `bin/emit-event hook.block`). It is a naming artifact, not an
enforcement mechanism.

## Recommended Wave-Gate Disposition

Flag this advisory for W-15 wave-gate review. The question is whether
the intended long-term design for handoff-validator (and similar "warning
gate" hooks) is:

- **Permanently advisory** (Option A): the hook warns, humans decide. No SDK
  extension needed. The `hook.block` event name may be confusing; consider
  renaming to `hook.warn` in a future story.

- **Eventual enforcement** (Option B): queue an SDK extension story (post-S-8.01)
  to add `HookResult::Block` and then update handoff-validator to use it. The
  `hook.block` event name aligns with this future state.

This advisory does not block S-8.01 convergence. The current behavior matches
the bash parity requirement (E-8 D-2). Any change to block semantics is a
new capability addition and belongs in a separate story.

## References

- `crates/hook-plugins/handoff-validator/src/lib.rs` — line 111-181
  (`handoff_validator_logic` always returns `HookResult::Continue`)
- `plugins/vsdd-factory/hooks-registry.toml` — handoff-validator entry with
  `on_error = "block"`
- Story S-8.01 AC-003/AC-004 — canonical warning path implementation
- E-8 epic D-2 Option C — behavior parity is the sole requirement for port stories
- vsdd_hook_sdk `HookResult` enum — current variants: `Continue`, `Error`
  (no `Block` variant exists as of S-8.10)
