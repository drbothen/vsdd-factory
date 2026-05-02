# AC-3: HOST_ABI_VERSION stays at 1

**Criterion:** Both `crates/hook-sdk/src/lib.rs` and `crates/factory-dispatcher/src/lib.rs`
declare `HOST_ABI_VERSION = 1`. No version bump occurs. D-6 Option A (additive extension) applies.

**Trace:** BC-2.01.003 invariant 1 (HOST_ABI_VERSION = 1 in both crates).

---

## grep Evidence

```
$ grep -F 'pub const HOST_ABI_VERSION: u32 = 1;' \
    crates/hook-sdk/src/lib.rs \
    crates/factory-dispatcher/src/lib.rs

crates/factory-dispatcher/src/lib.rs:43:pub const HOST_ABI_VERSION: u32 = 1;
crates/hook-sdk/src/lib.rs:58:pub const HOST_ABI_VERSION: u32 = 1;
```

Both locations return exactly one match. No bump occurred.

---

## Test Verification

From `tests/bc_2_02_011_parity.rs`:

```
test test_BC_2_02_011_invariant_1_dispatcher_host_abi_version_is_1 ... ok
test test_BC_2_02_011_invariant_1_both_crates_source_declare_version_1 ... ok
```

Two independent tests verify the constant at both crate boundaries.

---

## Architectural Decision Record

**AS-DEC:** Additive `write_file` export is ABI-stable. D-6 Option A applies.
`HOST_ABI_VERSION = 1` is unchanged throughout E-8. D-6 Option B (version bump)
is explicitly disallowed for v1.x. Wasmtime silently ignores additional host
exports that a plugin does not import, so existing plugins compiled against
SDK 0.1.x are unaffected.

**Status: PASS**
