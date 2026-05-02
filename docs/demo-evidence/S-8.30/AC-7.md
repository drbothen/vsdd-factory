# AC-7: HOST_ABI_VERSION constant remains at 1 in both crates

**Story:** S-8.30  
**AC:** AC-7 — HOST_ABI_VERSION = 1 invariant  
**BC clause:** BC-2.02.012 Invariant 1, BC-2.01.003 Invariant 1  
**Test:** `test_BC_2_02_012_invariant1_host_abi_version_remains_one`

## grep verification

```
crates/hook-sdk/src/lib.rs:58:pub const HOST_ABI_VERSION: u32 = 1;
crates/factory-dispatcher/src/lib.rs:43:pub const HOST_ABI_VERSION: u32 = 1;
```

Both lines confirm `HOST_ABI_VERSION: u32 = 1`. No bump occurred.

## Test output

```
test payload::tests::test_BC_2_02_012_invariant1_host_abi_version_remains_one ... ok
```

## Test source

```rust
#[test]
fn test_BC_2_02_012_invariant1_host_abi_version_remains_one() {
    assert_eq!(
        crate::HOST_ABI_VERSION,
        1,
        "HOST_ABI_VERSION must remain 1 (BC-2.02.012 Invariant 1, D-6 Option A)"
    );
}
```

## AS-DEC recorded in PR

"Additive HookPayload SubagentStop fields extension; D-6 Option A applies;
HOST_ABI_VERSION = 1 unchanged (BC-2.02.012 Invariant 1, BC-2.01.003)."

**PASS**
