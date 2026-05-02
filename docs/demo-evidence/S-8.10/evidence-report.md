# Demo Evidence Report — S-8.10: SDK extension: host::write_file

**Story:** S-8.10 — SDK extension: host::write_file (D-6 Option A unblocker)
**Branch:** feature/S-8.10-sdk-extension-write-file
**Date recorded:** 2026-05-02
**BC anchor:** BC-2.02.011 — host::write_file: bounded write capability with allowlist enforcement
**Toolchain:** rustc 1.95.0 (59807616e 2026-04-14)

---

## Coverage Summary

| AC | Description | Evidence File | Status |
|----|-------------|---------------|--------|
| AC-1 | SDK `write_file` wrapper signature + FFI declaration + doctest | [AC-1.md](AC-1.md) | PASS |
| AC-2 | Dispatcher binding — linker registration + E2E integration tests | [AC-2.md](AC-2.md) | PASS |
| AC-3 | `HOST_ABI_VERSION` stays at 1 in both crates | [AC-3.md](AC-3.md) | PASS |
| AC-4 | SDK unit tests — stub chain, param acceptance, error mapping | [AC-4.md](AC-4.md) | PASS |
| AC-5 | Dispatcher unit tests — 5 capability scenarios in `prepare()` | [AC-5.md](AC-5.md) | PASS |
| AC-6 | Legacy plugin regression — `host_functions.rs` + EC-007 compat | [AC-6.md](AC-6.md) | PASS |
| AC-7 | SDK version bumped to 0.2.0; CHANGELOG entry added | [AC-7.md](AC-7.md) | PASS |
| AC-8 | `HOST_ABI.md` catalog updated with full `write_file` entry | [AC-8.md](AC-8.md) | PASS |
| BONUS | `read_file` path-traversal fix + regression test | [BONUS-path-traversal-security-fix.md](BONUS-path-traversal-security-fix.md) | FIXED |

All 8 acceptance criteria: **PASS**. Zero regressions.

---

## AC-1: SDK API

`pub fn write_file(path: &str, contents: &[u8], max_bytes: u32, timeout_ms: u32) -> Result<(), HostError>`
added to `crates/hook-sdk/src/host.rs`. Uses input-pointer protocol (guest-owned bytes passed
to dispatcher via `(contents_ptr, contents_len)`). Non-wasm stub returns `-1`
(`HostError::CapabilityDenied`). Two doctests pass on non-wasm target.

See [AC-1.md](AC-1.md) for full signature, FFI declaration, and doctest examples.

---

## AC-2: Dispatcher Binding

`crates/factory-dispatcher/src/host/write_file.rs` (272 lines) registered as
`write_file::register(&mut linker)?` in `setup_linker`. E2E integration via
`tests/host_write_file_integration.rs` (10 WAT-based tests, all pass).

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

See [AC-2.md](AC-2.md) for full test list and implementation details.

---

## AC-3: HOST_ABI_VERSION Unchanged

```
grep -F 'pub const HOST_ABI_VERSION: u32 = 1;' \
    crates/hook-sdk/src/lib.rs \
    crates/factory-dispatcher/src/lib.rs

crates/factory-dispatcher/src/lib.rs:43:pub const HOST_ABI_VERSION: u32 = 1;
crates/hook-sdk/src/lib.rs:58:pub const HOST_ABI_VERSION: u32 = 1;
```

```
test test_BC_2_02_011_invariant_1_dispatcher_host_abi_version_is_1 ... ok
test test_BC_2_02_011_invariant_1_both_crates_source_declare_version_1 ... ok
```

See [AC-3.md](AC-3.md) for AS-DEC rationale.

---

## AC-4: SDK Unit Tests

5 integration tests in `tests/host_write_file_sdk_test.rs` + 2 doctests pass:

```
test test_BC_2_02_011_sdk_stub_returns_capability_denied ... ok
test test_BC_2_02_011_sdk_max_bytes_and_timeout_accepted_without_panic ... ok
test test_BC_2_02_011_sdk_empty_contents_accepted_without_panic ... ok
test test_BC_2_02_011_sdk_error_enum_covers_all_write_file_outcomes ... ok
test test_BC_2_02_011_sdk_version_bumped_to_0_2_0 ... ok
test crates/hook-sdk/src/host.rs - host::write_file (line 236) ... ok
test crates/hook-sdk/src/host.rs - host::write_file (line 246) ... ok
```

See [AC-4.md](AC-4.md) for conversion chain verification.

---

## AC-5: Dispatcher Capability Scenarios

6 unit tests in `host::write_file::tests` (calling `prepare()` directly):

```
test host::write_file::tests::denies_when_no_capability_block ... ok
test host::write_file::tests::writes_allowed_file ... ok
test host::write_file::tests::rejects_path_outside_allow_list ... ok
test host::write_file::tests::rejects_content_exceeding_max_bytes ... ok
test host::write_file::tests::writes_empty_contents_creates_file ... ok
test host::write_file::tests::rejects_missing_parent_directory ... ok
```

See [AC-5.md](AC-5.md) for scenario mapping and key assertions.

---

## AC-6: Legacy Plugin Regression

`tests/host_functions.rs` passes unchanged. EC-007 (SDK 0.1.x plugin loads against
new dispatcher) confirmed:

```
test setup_linker_registers_every_vsdd_import ... ok
test wat_module_importing_host_functions_instantiates ... ok
test test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher ... ok
```

Total dispatcher test result: 110 lib unit tests + all integration suites — 0 failures.

See [AC-6.md](AC-6.md) for complete test suite breakdown.

---

## AC-7: Version Bump and CHANGELOG

`crates/hook-sdk/Cargo.toml`: `version = "0.2.0"` (bumped from 0.1.0).

CHANGELOG.md `[0.2.0]` entry:
- **Added:** `host::write_file` SDK API with `WriteFileCaps` capability schema.
- **Security:** path-traversal fix in `path_allowed()` for both `read_file` and `write_file`.

`hook-sdk-macros` stays at `0.1.0` (no new macro surface added).

See [AC-7.md](AC-7.md) for full CHANGELOG excerpt.

---

## AC-8: HOST_ABI.md Catalog

`crates/hook-sdk/HOST_ABI.md` section added:
`write_file(path_ptr, path_len, contents_ptr, contents_len, max_bytes, timeout_ms) -> i32`

Includes: full parameter table, return codes (-1 through -99), input-pointer protocol note,
timeout/epoch semantics, `max_bytes` byte-cap semantics, safety policy, deny-by-default.

See [AC-8.md](AC-8.md) for full excerpt and coverage checklist.

---

## Bonus: Path-Traversal Security Fix

Out-of-scope security improvement: `path_allowed()` in `read_file.rs` had the same
`../` bypass vulnerability as the new `write_file.rs`. Both are now fixed by
canonicalization-before-compare. Regression test confirms protection:

```
test test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt ... ok
```

See [BONUS-path-traversal-security-fix.md](BONUS-path-traversal-security-fix.md) for
full vulnerability description, fix implementation, and E2E regression test.

---

## Commits on Branch

| Hash | Description |
|------|-------------|
| `fec9049` | Stub Architect (host::write_file SDK + dispatcher binding + 5 unit tests) |
| `1c520e4` | Red Gate (E2E + BC parity + doctest tests) |
| `66678fb` | GREEN security fix (canonicalize paths in read+write_file allowlist check) |
| `47be403` | GREEN docs (CHANGELOG.md v0.2.0 entry) |

All `cargo build/test/clippy` clean on rustc 1.95.0.
