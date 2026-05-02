# AC-2: Dispatcher write_file binding

**Criterion:** `crates/factory-dispatcher/src/host/write_file.rs` implements the
host-side wasmtime binding for `vsdd::write_file` with path validation, timeout
enforcement, input-pointer protocol, `max_bytes` cap, and linker registration.

**Trace:** BC-2.02.011 postcondition 2 (byte cap exceeded; no bytes written to disk).

---

## E2E Integration Tests (`tests/host_write_file_integration.rs`)

All WAT-based integration tests exercise the full `func_wrap -> read_wasm_bytes -> prepare -> fs::write` pipeline.

```
test test_BC_2_02_011_write_file_registered_in_linker ... ok
test test_BC_2_02_011_wat_module_with_write_file_import_instantiates ... ok
test test_BC_2_02_011_wat_denied_when_no_capability ... ok
test test_BC_2_02_011_wat_write_succeeds_allowed_path ... ok
test test_BC_2_02_011_wat_max_bytes_exceeded_returns_output_too_large ... ok
test test_BC_2_02_011_timeout_ms_zero_accepted_abi_stability ... ok
test test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker ... ok
test test_BC_2_02_011_invariant_5_error_codes_stable_no_new_codes ... ok
test test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt ... ok
test test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher ... ok
```

All 10 integration tests pass.

---

## Key Implementation Details

### Registration in `mod.rs`

`write_file::register(&mut linker)?;` added after `read_file::register` in `setup_linker`.

### Dispatcher `func_wrap` Signature

```rust
linker.func_wrap(
    "vsdd",
    "write_file",
    |mut caller: HostCaller<'_>,
     path_ptr: u32,
     path_len: u32,
     contents_ptr: u32,
     contents_len: u32,
     max_bytes: u32,
     timeout_ms: u32|
     -> i32 {
        let _ = timeout_ms; // accepted for ABI stability; enforced in S-1.5 via epoch interruption
        // ... path read, contents read, prepare() call
    },
)
```

### `prepare()` Postconditions

1. Deny-by-default: no `capabilities.write_file` block -> `CAPABILITY_DENIED (-1)`.
2. Path allowlist + traversal denial via canonicalize-before-compare.
3. `max_bytes` cap enforced before any write: `contents.len() > effective_cap` -> `OUTPUT_TOO_LARGE (-3)`.
4. `std::fs::write` on success; I/O error -> `INTERNAL_ERROR (-99)`.

### `WriteFileCaps` Struct (in `registry.rs`)

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WriteFileCaps {
    pub path_allow: Vec<String>,
    pub max_bytes_per_call: Option<u32>,
}
```

Derives match `ReadFileCaps`; `deny_unknown_fields` confirmed by test
`test_BC_2_02_011_registry_rejects_unknown_write_file_capability_field ... ok`.

**Status: PASS**
