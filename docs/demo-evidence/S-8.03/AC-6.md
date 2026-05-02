# AC-006 Evidence — host::emit_event Replaces bin/emit-event

**AC:** AC-006 (traces to BC-7.03.082 postcondition 1)
**Statement:** `bin/emit-event` calls replaced with `host::emit_event` native host function
calls. No reference to `bin/emit-event` remains in `crates/hook-plugins/track-agent-stop/`.
`bin/emit-event` binary is NOT removed (E-8 D-10 — deferred to S-8.29).

## Recording

- `AC-006-host-emit-event.gif` / `.webm` / `.tape`

## Verification Steps Shown

1. `grep -n 'emit_event|bin/emit-event' crates/hook-plugins/track-agent-stop/src/lib.rs`
   — shows `vsdd_hook_sdk::host::emit_event` used; no `bin/emit-event` reference.
2. `grep -rn 'bin/emit-event' crates/hook-plugins/track-agent-stop/ 2>&1 || echo 'CONFIRMED: no bin/emit-event references'`
   — grep returns non-zero (no matches); fallback confirms absence.
3. `ls -lh target/wasm32-wasip1/release/track-agent-stop.wasm` — WASM artifact exists.

## Implementation (from `src/lib.rs`)

```rust
pub fn on_agent_stop(payload: HookPayload) -> HookResult {
    track_agent_stop_logic(payload, |event_type, fields| {
        // Silently swallow emit_event errors (best-effort, on_error=continue).
        let _ = vsdd_hook_sdk::host::emit_event(event_type, fields);
    })
}
```

`bin/emit-event` binary at `bin/emit-event` remains on disk (not deleted) per E-8 D-10.
Only the reference in this crate is removed.

## Result

PASS — `host::emit_event` used; no `bin/emit-event` reference in crate; binary not removed.
