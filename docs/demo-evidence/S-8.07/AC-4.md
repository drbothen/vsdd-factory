# AC-004 Evidence: Early-Exit Paths Are Silent

**AC statement:** Early-exit paths produce no stderr, no emit_event call, exit 0.
Each path maps to the corresponding bash branch for parity verification.

**BC trace:** BC-7.03.091 postcondition 2

---

## Path (a): wave-state.yaml absent

```rust
// lib.rs lines 31-34
let yaml_content = match read_wave_state() {
    Some(content) => content,
    None => return HookResult::Continue,
};
```

`read_wave_state()` calls `host::read_file(WAVE_STATE_PATH, MAX_BYTES, TIMEOUT_MS)`.
When the file is absent, the dispatcher's `host::read_file::prepare()` encounters a
read error (file does not exist) and returns `codes::INTERNAL_ERROR`. The SDK maps
this to `Err(_)`, which the `match` arm converts to `None`. `HookResult::Continue`
is returned — no stderr, no emit_event.

Bash branch: `[[ ! -f .factory/wave-state.yaml ]] && exit 0`

Bats test AC-005(d): fixture dir has no `.factory/` directory. PASS.

## Path (b): YAML parse fails or `waves` key absent/null

```rust
// lib.rs lines 37-45
let state: serde_yaml::Value = match serde_yaml::from_str(&yaml_content) {
    Ok(v) => v,
    Err(_) => return HookResult::Continue,
};

let waves_map = match state.get("waves").and_then(|v| v.as_mapping()) {
    Some(m) => m,
    None => return HookResult::Continue,
};
```

`serde_yaml::from_str` on malformed YAML returns `Err(_)` → `HookResult::Continue`.
If YAML is valid but `waves` key is absent, `state.get("waves")` returns `None` →
`HookResult::Continue`. If `waves` key exists but value is null or a non-mapping
(EC-007), `.as_mapping()` returns `None` → `HookResult::Continue`.

Bash branch: `if not state or 'waves' not in state: sys.exit(0)`

Bats test AC-005(e): malformed YAML (broken indentation + unclosed bracket). PASS.

## Path (c): No wave has gate_status == "pending"

```rust
// lib.rs lines 50-58
for (name, data) in waves_map {
    if data
        .get("gate_status")
        .and_then(serde_yaml::Value::as_str)
        == Some("pending")
        && let Some(name_str) = name.as_str()
    {
        pending.push(name_str.to_string());
    }
}

// lib.rs lines 62-64
if pending.is_empty() {
    return HookResult::Continue;
}
```

`Value::as_str` is used (not strict typed deserialization) to avoid panics on
non-string `gate_status` values (EC-008). For non-string values, `.as_str()`
returns `None`, so `None == Some("pending")` is false — wave is skipped.

Bash branch: jq `select(.gate_status == "pending")` returning empty.

Bats test AC-005(c): both waves have `gate_status: passed`. PASS.

## EC-009: CapabilityDenied

When the registry is missing the `[hooks.capabilities.read_file]` block, the
dispatcher returns `codes::CAPABILITY_DENIED`. The SDK maps this to `Err(_)`,
which the `match` arm converts to `None`. `HookResult::Continue` is returned — no
stderr, no emit_event. AC-001 verifies the capability block is present, preventing
this path in production.

## Summary Table

| Path | Early-exit condition | Rust branch | bash parity | Bats test |
|------|---------------------|-------------|-------------|-----------|
| (a) | File absent | `read_wave_state()` → None | `[[ ! -f ]] && exit 0` | AC-005(d) PASS |
| (b) | YAML parse error | `serde_yaml::from_str` → Err | `sys.exit(0)` on parse | AC-005(e) PASS |
| (b) | `waves` key absent/null | `.get("waves").as_mapping()` → None | `'waves' not in state` | (b-null) PASS |
| (c) | No pending waves | `pending.is_empty()` | jq empty select | AC-005(c) PASS |
| EC-009 | CapabilityDenied | `Err(_)` → None | N/A (WASM only) | implicit |

**Result: PASS** — all four early-exit paths return `HookResult::Continue` silently.
