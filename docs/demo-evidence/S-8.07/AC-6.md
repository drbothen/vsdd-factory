# AC-006 Evidence: python3 Replaced with serde_yaml 0.9.34

**AC statement:** No reference to `python3` or subprocess invocation in
`crates/hook-plugins/warn-pending-wave-gate/`. The `[hooks.capabilities.exec_subprocess]`
block is removed entirely from the registry entry. The new entry replaces it with
`[hooks.capabilities.read_file] path_allow = [".factory/wave-state.yaml"]`.
No `exec_subprocess`, no `python3`, no `bash` in the hook's capabilities section.

**BC trace:** BC-7.03.091 invariant 2

---

## serde_yaml 0.9.34 Usage

The `lib.rs` uses `serde_yaml::from_str` and `serde_yaml::Value` directly:

```rust
let state: serde_yaml::Value = match serde_yaml::from_str(&yaml_content) {
    Ok(v) => v,
    Err(_) => return HookResult::Continue,
};

let waves_map = match state.get("waves").and_then(|v| v.as_mapping()) {
    Some(m) => m,
    None => return HookResult::Continue,
};

// EC-008: use Value::as_str to avoid panics on non-string gate_status values
if data.get("gate_status").and_then(serde_yaml::Value::as_str) == Some("pending")
```

No `std::process::Command`, no `exec_subprocess`, no python3 subprocess. The
YAML structure (`waves.{name}.gate_status`) is simple enough for `serde_yaml::Value`
(dynamic value API) without strict typed deserialization, which also provides
graceful handling of EC-008 (non-string gate_status values).

## Workspace Pin

`Cargo.toml` (workspace root) declares:
```toml
serde_yaml = "0.9.34"
```

The crate references it as `serde_yaml = { workspace = true }`.

Note: serde_yaml 0.9 was deprecated by dtolnay in 2024 (last release 0.9.34, marked
unmaintained). Decision: pin to 0.9.34 with a tech-debt entry for future migration to
yaml-rust2 or serde_yml. This is documented in the story changelog (v1.1 F-003).

## exec_subprocess Block Removal Verified

Registry entry before (legacy bash adapter pattern):
```toml
[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = [...]
```

Registry entry after (native WASM):
```toml
[hooks.capabilities.read_file]
path_allow = [".factory/wave-state.yaml"]
```

The `exec_subprocess` block is entirely absent from the new entry. No `python3`,
no `bash` reference in any capabilities section of this hook.

## Source Scan Results

Files scanned for subprocess invocation patterns
(`std::process::Command`, `exec_subprocess`, `Command::new("python"`, `bin/emit-event`):

- `src/main.rs` — none found
- `src/lib.rs` — none found (porting-note comment about python3 is documentation, not invocation)
- `Cargo.toml` — no `exec_subprocess` dependency

This is verified by the integration test
`test_BC_7_03_091_ac006_no_python3_subprocess_invocation_in_wasm_crate`
in `tests/integration_test.rs`.

**Result: PASS** — serde_yaml 0.9.34 replaces python3 subprocess; exec_subprocess block removed.
