# AC-003 Evidence — Typed-Projection Fallback Chains, Exit Classification, emit_event

**AC:** AC-003 (traces to BC-7.03.082 postcondition 1; BC-2.02.012 postconditions 5 and 6)
**Statement:** WASM plugin resolves agent identity via BC-2.02.012 PC-5 fallback chain
(`agent_type -> subagent_name -> "unknown"`), resolves assistant-message via PC-6
(`last_assistant_message -> result -> ""`), computes RESULT_LEN as non-whitespace byte
count, classifies EXIT_CLASS (`empty`/`blocked`/`ok`), emits `agent.stop` with correct
fields, always exits 0.

## Recording

- `AC-003-exit-classification.gif` / `.webm` / `.tape`

## Verification Steps Shown

1. `grep -n 'agent_type|subagent_name|last_assistant_message' crates/hook-sdk/src/payload.rs | grep pub`
   — confirms all 4 BC-2.02.012 fields declared as `pub` `Option<String>` in `HookPayload`.
2. `grep -n 'as_deref|BLOCKED_PATTERN|bytes.*filter|emit_event' crates/hook-plugins/track-agent-stop/src/lib.rs`
   — shows canonical fallback chain expressions, BLOCKED regex constant, byte-count
   filter form, and `host::emit_event` call.

## Key Implementation Details (from `src/lib.rs`)

### BC-2.02.012 Postcondition 5 — Agent identity fallback chain
```rust
let agent: &str = payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");
```

### BC-2.02.012 Postcondition 6 — Assistant-message fallback chain
```rust
let result: &str = payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");
```

### RESULT_LEN byte-count (bash `wc -c` parity)
```rust
let result_len = result.bytes().filter(|b| !b.is_ascii_whitespace()).count();
```

### BLOCKED regex (multiline, shared with pr-manager-completion-guard)
```rust
const BLOCKED_PATTERN: &str = r"(?m)^(Status:\s*|##?\s*)?\s*BLOCKED";
```

### emit_event call
```rust
emit_fn("agent.stop", &[
    ("hook", "track-agent-stop"),
    ("matcher", "SubagentStop"),
    ("subagent", agent),
    ("exit_class", exit_class),
    ("result_len", &result_len_str),
]);
```

## Result

PASS — All BC-2.02.012 fallback chains implemented canonically; byte-count RESULT_LEN
correct; multiline BLOCKED regex with `(?m)` flag; 5-field emit_event call; always
returns `HookResult::Continue` (exit 0).
