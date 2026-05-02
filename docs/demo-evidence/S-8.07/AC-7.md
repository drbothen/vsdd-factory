# AC-007 Evidence: bin/emit-event Replaced with host::emit_event

**AC statement:** `bin/emit-event` calls from the bash `_emit()` function are replaced
with `host::emit_event` Rust calls in the WASM crate. No reference to `bin/emit-event`
remains in `crates/hook-plugins/warn-pending-wave-gate/`. `bin/emit-event` binary is
NOT removed (E-8 D-10 — deferred to S-8.29).

**BC trace:** BC-7.03.092 postcondition 1 (emit_event host fn)

---

## host::emit_event Call (lib.rs lines 67-77)

The `emit` closure in `main.rs` wires directly to the SDK host function:

```rust
// main.rs lines 33-35
|event_type, fields| {
    vsdd_hook_sdk::host::emit_event(event_type, fields);
},
```

The closure matches the `emit: impl FnOnce(&str, &[(&str, &str)])` parameter
in `warn_pending_wave_gate_logic`. `host::emit_event` takes `(event_type: &str,
fields: &[(&str, &str)])` — event_type is positional (NOT a KV field). This maps
the bash `_emit()` helper's `type=hook.block key=value ...` calling convention to
the SDK's positional API.

SDK semantics: `host::emit_event` returns `()` (not `Result`). If the underlying
sink has an outage, the SDK swallows it. This matches bash `|| true` semantics
(EC-006 / best-effort emission).

## bin/emit-event Binary Preservation

`bin/emit-event` is NOT removed per E-8 D-10:
> bin/emit-event binary must NOT be removed until S-8.29

The binary at `bin/emit-event` is untouched by this story. Deferred to S-8.29
(universal decommission of bin/emit-event after all hook plugins are ported).

## Source Scan Results

Files scanned for `bin/emit-event` references:

- `src/main.rs` — none found
- `src/lib.rs` — none found
- `Cargo.toml` — none found

This is verified by the integration test
`test_BC_7_03_092_ac007_no_bin_emit_event_reference_in_wasm_crate`
in `tests/integration_test.rs`.

## Bash Source Mapping

The bash `_emit()` helper (original source):
```bash
_emit() {
    "${SCRIPT_DIR}/../bin/emit-event" "$@" || true
}

_emit type=hook.block hook=warn-pending-wave-gate matcher=Stop \
      reason=pending_wave_gate_at_session_end severity=warn \
      pending_waves="$pending_waves_str"
```

Maps to the WASM Rust call:
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

`type=hook.block` → positional `"hook.block"`. All other KVs → fields slice.
The `|| true` no-fail semantics → SDK's `()` return type.

**Result: PASS** — host::emit_event replaces bin/emit-event calls; binary not removed.
