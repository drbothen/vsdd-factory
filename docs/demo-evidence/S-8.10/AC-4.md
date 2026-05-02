# AC-4: SDK unit tests

**Criterion:** `crates/hook-sdk/src/host.rs` unit tests (non-wasm target) cover:
(a) stub returns `-1` -> `HostError::CapabilityDenied`; (b) params accepted without panic;
(c) `HostError::from_code` mapping unchanged.

**Trace:** BC-2.02.011 postcondition 3 (successful write path exercised via unit test).

---

## Test Results (`cargo test --package vsdd-hook-sdk`)

```
test host::tests::host_error_code_mapping ... ok
test host::tests::decode_subprocess_result_rejects_truncated ... ok
test host::tests::encode_args_round_trip ... ok
test host::tests::encode_fields_uses_length_prefix ... ok
test host::tests::log_levels_are_stable ... ok

Running tests/host_write_file_sdk_test.rs
test test_BC_2_02_011_sdk_stub_returns_capability_denied ... ok
test test_BC_2_02_011_sdk_max_bytes_and_timeout_accepted_without_panic ... ok
test test_BC_2_02_011_sdk_empty_contents_accepted_without_panic ... ok
test test_BC_2_02_011_sdk_error_enum_covers_all_write_file_outcomes ... ok
test test_BC_2_02_011_sdk_version_bumped_to_0_2_0 ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Doctest Results

```
Doc-tests vsdd_hook_sdk

test crates/hook-sdk/src/host.rs - host::write_file (line 236) ... ok
test crates/hook-sdk/src/host.rs - host::write_file (line 246) ... ok

test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

Both write_file doctests pass:
- Line 236: `write_file(".factory/STATE.md", b"updated state", 65536, 5000)` -> `Err(CapabilityDenied)`
- Line 246: `write_file("/tmp/out.txt", b"", 0, 0)` -> accepted without panic

---

## Conversion Chain Verification

`test_BC_2_02_011_sdk_stub_returns_capability_denied` confirms:

1. Non-wasm stub `ffi::write_file(...)` returns `-1`.
2. SDK wrapper calls `HostError::from_code(-1)`.
3. `HostError::from_code(-1)` maps to `HostError::CapabilityDenied`.
4. Wrapper returns `Err(HostError::CapabilityDenied)`.

`test_BC_2_02_011_sdk_error_enum_covers_all_write_file_outcomes` verifies no
new error codes were added: the full set remains
`{CapabilityDenied, Timeout, OutputTooLarge, InvalidArgument, Other(-99)}`.

**Status: PASS**
