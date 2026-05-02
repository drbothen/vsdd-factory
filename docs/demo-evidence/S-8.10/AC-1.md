# AC-1: SDK write_file wrapper

**Criterion:** `host::write_file(path, contents, max_bytes, timeout_ms) -> Result<(), HostError>`
exists in `crates/hook-sdk/src/host.rs` and round-trips through the dispatcher's
`vsdd::write_file` host import using the input-pointer protocol.

**Trace:** BC-2.02.011 postcondition 1 (allowlist denial and capability gating).

---

## Function Signature

File: `crates/hook-sdk/src/host.rs` (after `read_file` at line 187)

```rust
pub fn write_file(
    path: &str,
    contents: &[u8],
    max_bytes: u32,
    timeout_ms: u32,
) -> Result<(), HostError>
```

Both `max_bytes` and `timeout_ms` are mandatory parameters per BC-2.02.002 (byte cap on all bounded calls).

---

## FFI Declaration (`crates/hook-sdk/src/ffi.rs`)

### wasm32 target (`extern "C"` block — input-pointer protocol)

```rust
pub safe fn write_file(
    path_ptr: *const u8,
    path_len: u32,
    contents_ptr: *const u8,
    contents_len: u32,
    max_bytes: u32,
    timeout_ms: u32,
) -> i32;
```

Parameters: path as `(path_ptr, path_len)` UTF-8 bytes; contents as
`(contents_ptr, contents_len)` guest-owned bytes (the dispatcher copies via
`read_wasm_bytes`); `max_bytes` hard byte cap; `timeout_ms` epoch budget.
Returns `0` on success or a negative error code.

### Non-wasm stub (`host_stubs` module)

```rust
/// Non-wasm stub for `write_file` (BC-2.02.011). Always returns `-1`
/// (capability denied) because no dispatcher is present on non-wasm targets.
pub fn write_file(
    _path_ptr: *const u8,
    _path_len: u32,
    _contents_ptr: *const u8,
    _contents_len: u32,
    _max_bytes: u32,
    _timeout_ms: u32,
) -> i32 {
    -1
}
```

---

## SDK Wrapper Body

```rust
pub fn write_file(
    path: &str,
    contents: &[u8],
    max_bytes: u32,
    timeout_ms: u32,
) -> Result<(), HostError> {
    let path_bytes = path.as_bytes();
    let code = ffi::write_file(
        path_bytes.as_ptr(),
        path_bytes.len() as u32,
        contents.as_ptr(),
        contents.len() as u32,
        max_bytes,
        timeout_ms,
    );
    if code < 0 {
        return Err(HostError::from_code(code));
    }
    Ok(())
}
```

---

## Doctest Examples (both pass — see AC-4)

```rust
// Non-wasm (doc-test) target: stub returns Err(CapabilityDenied).
let result = write_file(".factory/STATE.md", b"updated state", 65536, 5000);
assert_eq!(result, Err(HostError::CapabilityDenied));
```

```rust
// max_bytes cap is enforced by the dispatcher; timeout_ms by epoch interruption.
// On non-wasm stub both are accepted as parameters without panic.
let _ = write_file("/tmp/out.txt", b"", 0, 0);
```

---

## Protocol Note

This uses the **input-pointer protocol** — the SDK passes guest-owned bytes
`(contents_ptr, contents_len)` to the dispatcher, which copies them out via
`read_wasm_bytes`. This differs from `read_file`'s output-pointer protocol
(caller provides buffer; host writes into it). Only the error-code mapping
(`HostError::from_code`) is shared with `read_file`.

**Status: PASS**
