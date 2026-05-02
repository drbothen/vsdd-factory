# Bonus: S-8.04 as First Consumer of S-8.10 host::write_file

## Context

S-8.10 landed the `host::write_file` SDK extension (D-6 Option A unblocker).
S-8.04 is the **first production consumer** of that capability. This document traces
the capability-gated write path from the SDK surface down to the registry enforcement.

---

## Call Chain

### 1. SDK surface (crates/hook-sdk/src/host.rs)

```rust
pub fn write_file(
    path: &str,
    contents: &[u8],
    max_bytes: u32,
    timeout_ms: u32,
) -> Result<(), HostError>
```

4-param form per S-8.10 v1.1 AC-1. `max_bytes` caps byte size; `timeout_ms` sets
epoch budget per BC-2.02.002.

### 2. S-8.04 call site (crates/hook-plugins/update-wave-state-on-merge/src/main.rs)

```rust
vsdd_hook_sdk::host::write_file(
    ".factory/wave-state.yaml",
    &bytes,
    65536,    // max_bytes matching read_file cap
    10000,    // timeout_ms matching registry entry
)
```

The path `.factory/wave-state.yaml` is relative. The dispatcher resolves it against
`CLAUDE_PROJECT_DIR` (the `cwd` stored in `StoreData`).

### 3. Dispatcher capability enforcement (crates/factory-dispatcher/src/invoke.rs)

The `write_file` host function in `invoke.rs` enforces the path allowlist:

```rust
let caps = match &host.capabilities.write_file {
    Some(c) => c.clone(),
    None => return codes::CAPABILITY_DENIED,
};
// Resolve relative path against cwd
let resolved = cwd.join(&path);
// Check against path_allow list
let allowed = caps.path_allow.iter().any(|pref| {
    let pref_path = cwd.join(pref);
    let canon_resolved = resolved.canonicalize().unwrap_or_else(|_| resolved.clone());
    let canon_pref   = pref_path.canonicalize().unwrap_or_else(|_| pref_path.clone());
    canon_resolved.starts_with(&canon_pref)
});
if !allowed {
    return codes::CAPABILITY_DENIED;
}
```

Deny-by-default: absent capability block → `CAPABILITY_DENIED`. Present but path
not in `path_allow` → `CAPABILITY_DENIED`.

### 4. Registry capability block (plugins/vsdd-factory/hooks-registry.toml)

```toml
[hooks.capabilities.write_file]
path_allow = [".factory/wave-state.yaml"]
max_bytes_per_call = 65536
```

Only `.factory/wave-state.yaml` is allowed. Any other path (e.g., `../.factory/something`)
is denied by the canonicalize-before-compare check (path-traversal fix from S-8.10 BONUS).

---

## Capability-Gated Path Diagram

```
update-wave-state-on-merge.wasm
  │  calls vsdd_hook_sdk::host::write_file(".factory/wave-state.yaml", ...)
  ▼
factory-dispatcher invoke.rs
  │  reads StoreData.capabilities.write_file
  │  path_allow check: ".factory/wave-state.yaml" matches → allowed
  ▼
std::fs::write(resolved_absolute_path, contents)
  ▼
.factory/wave-state.yaml (updated YAML on disk)
```

If the capability block is absent from the registry → `CapabilityDenied` → hook emits
`hook.error` event and logs to stderr, then exits 0 (advisory; never blocks pr-manager).

---

## Standalone Feature Flag (bats parity testing)

The crate's `standalone` feature (enabled by default in debug builds) substitutes
`std::fs` for `vsdd` host functions, enabling `wasmtime run` testing without the
dispatcher. The production WASM is built with `--no-default-features`.

This is a novel pattern for the E-8 migration: previous Tier 1 hooks (S-8.01, S-8.03)
did not need file I/O and had no analogous dual-mode feature. S-8.04 establishes the
pattern for future file-I/O hooks in the WASM migration.

---

## write_file vs read_file Protocol Differences

| Property | read_file | write_file |
|----------|-----------|------------|
| ABI pattern | output-pointer (host writes to guest buffer) | input-pointer (guest passes byte slice to host) |
| Max size param | `max_bytes` caps host read | `max_bytes` caps content written |
| Memory management | host grows WASM memory 1 page; writes content there | host reads content from existing guest memory |
| SDK version | 0.1.0 (original) | 0.2.0 (S-8.10 addition) |
| First consumer | S-8.04 (also read_file) | **S-8.04** (first consumer) |
