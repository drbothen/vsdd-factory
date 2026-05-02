# AC-003 Evidence: Pending Wave Found — emit hook.block + WAVE GATE REMINDER

**AC statement:** When `.factory/wave-state.yaml` exists and at least one wave has
`gate_status: pending`, the WASM plugin calls `host::emit_event` with the canonical
SDK signature and writes the WAVE GATE REMINDER block to stderr. Exits 0.

**BC trace:** BC-7.03.092 postcondition 1

---

## emit_event Call (lib.rs lines 67-77)

```rust
emit(
    "hook.block",
    &[
        ("hook",    "warn-pending-wave-gate"),
        ("matcher", "Stop"),
        ("reason",  "pending_wave_gate_at_session_end"),
        ("severity","warn"),
        ("pending_waves", &comma_joined),
    ],
);
```

`event_type = "hook.block"` is the positional first argument to `host::emit_event`.
The remaining five key-value pairs are the fields slice. This maps the bash `_emit()`
helper pattern (which called `bin/emit-event type=hook.block …`) to the SDK host fn.

`comma_joined` is built as `pending.join(",")` — the string `"W-15"` for a single wave
or `"W-15,W-16"` for multiple waves (EC-004).

## WAVE GATE REMINDER stderr Output (lib.rs lines 81-95)

```rust
let mut msg = String::new();
msg.push('\n');
msg.push_str("WAVE GATE REMINDER:\n");
for wave in &pending {
    msg.push_str(&format!(
        "  - {} gate is pending. Run the gate before starting the next wave.\n",
        wave
    ));
}
msg.push('\n');
msg.push_str("  Invoke /vsdd-factory:wave-gate or update .factory/wave-state.yaml\n");
msg.push_str(
    "  with gate_status: passed (after running checks) or deferred (with rationale).\n",
);
write_stderr(&msg);
```

All `\n` are actual 0x0A newline characters (not literal backslash-n). The format
matches the bash source exactly: blank line before the header, one line per pending
wave with two-space indent and dash prefix, blank line separator, two-line invocation
hint.

## Bats Test Coverage (AC-005 cases a and b)

Test case AC-005(a) — one pending wave (W-15):
- Fixture: `waves.W-15.gate_status: pending`
- Expected: exit 0, output contains `WAVE GATE REMINDER:`, contains
  `  - W-15 gate is pending. Run the gate before starting the next wave.`,
  contains `Invoke /vsdd-factory:wave-gate`
- Result: PASS

Test case AC-005(b) — two pending waves (W-15, W-16):
- Fixture: both `W-15.gate_status: pending` and `W-16.gate_status: pending`
- Expected: exit 0, output contains both `W-15` and `W-16`, header present
- Result: PASS

## Stderr Relay

The dispatcher's `main.rs` relays plugin stderr to process stderr so the WAVE GATE
REMINDER is visible to the user at terminal output:

```rust
for outcome in &summary.per_plugin_results {
    if let PluginResult::Ok { stderr, .. } = &outcome.result
        && !stderr.is_empty()
    {
        eprint!("{stderr}");
    }
}
```

Without this relay, the WASM sandbox captures plugin stderr into `MemoryOutputPipe`
and the REMINDER would only appear in the internal log — invisible to the user. This
is one of the three dispatcher fixes included in the bonus section.

**Result: PASS** — emit_event called with canonical fields, stderr REMINDER matches
bash format, exits 0.
