# Red Gate Log — S-8.10: host::write_file

**Story:** S-8.10 — SDK extension: host::write_file (D-6 Option A unblocker)
**BC:** BC-2.02.011
**Date:** 2026-05-02
**Test writer:** test-writer agent

## Context

Stub architect (commit `fec9049`) overshot scope by landing the full
`prepare` function + AC-5 dispatcher unit tests (all passing). The Red Gate
pass therefore targets coverage the stub did NOT provide:

1. End-to-end WAT integration tests (full linker → host fn → fs path)
2. BC-2.02.011 invariants and edge cases not covered by unit tests
3. AC-4 SDK unit tests (not written by stub)
4. SDK doctest (AC spec requirement)

## Test Files Added

| File | Tests | Failing |
|------|-------|---------|
| `crates/factory-dispatcher/tests/host_write_file_integration.rs` | 10 | 1 |
| `crates/factory-dispatcher/tests/bc_2_02_011_parity.rs` | 7 | 1 |
| `crates/hook-sdk/tests/host_write_file_sdk_test.rs` | 5 | 0 |
| `crates/hook-sdk/src/host.rs` (doctest) | 2 | 0 |
| **Total** | **24** | **2** |

## Failing Tests (Red Gate)

### 1. `test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt`

**File:** `crates/factory-dispatcher/tests/host_write_file_integration.rs`
**Failure:** `assertion left == right failed: expected CAPABILITY_DENIED (-1), got -99 (INTERNAL_ERROR)`

**Root cause:** `path_allowed` in `write_file.rs` uses `Path::starts_with` without
canonicalizing the path. A traversal path like `/allowed/dir/../../../etc/passwd`
DOES start with `/allowed/dir` at the component level (the `..` is a later
component), passes the allowlist check, and then `std::fs::write` to the
resolved-but-not-canonicalized path fails with an `io::Error` → `INTERNAL_ERROR`.

**BC reference:** BC-2.02.011 EC-001 and invariant 6 (deny-by-default capability model)
require path traversal to return `CAPABILITY_DENIED (-1)`, not `INTERNAL_ERROR (-99)`.

**Fix required:** The implementer must canonicalize the path before the allowlist
check (e.g., using `Path::canonicalize()` or a path-component normalization step
that strips `..` components).

### 2. `test_BC_2_02_011_ac7_changelog_contains_write_file_entry`

**File:** `crates/factory-dispatcher/tests/bc_2_02_011_parity.rs`
**Failure:** `CHANGELOG.md must contain 'host::write_file' entry (AC-7 S-8.10)`

**Root cause:** AC-7 requires adding a CHANGELOG entry under "## Added" with:
`host::write_file — write a file at the given path through the dispatcher's bounded host function.`
The stub architect bumped the SDK version to `0.2.0` but did not add the CHANGELOG
entry.

**BC reference:** BC-2.02.011 postcondition 6 / AC-7 (vsdd-hook-sdk minor version bump
+ CHANGELOG entry).

**Fix required:** Add entry to `CHANGELOG.md` at workspace root under a v0.2.0 section
mentioning `host::write_file`.

## Passing Tests (New Coverage, Not Duplicate)

The following new tests PASS (confirming correct behavior not covered by stub):

- E2E: `write_file` registered in linker (AC-2 / invariant 4)
- E2E: WAT module with `write_file` import instantiates cleanly
- E2E: WAT denied when no capability (tests `func_wrap` → `prepare` path)
- E2E: WAT write succeeds for allowed path (full linker → fs path)
- E2E: WAT `max_bytes` exceeded returns `OUTPUT_TOO_LARGE`
- E2E: `timeout_ms=0` accepted for ABI stability (EC-004)
- E2E: relative path resolves via `plugin_root` through full linker (invariant 3)
- E2E: error codes are stable set (invariant 5)
- EC-007: SDK 0.1.x plugin loads against new dispatcher (ABI backward compat)
- BC invariant 1: `HOST_ABI_VERSION = 1` in both crates (AC-3)
- BC invariant 2: `max_bytes_per_call` cap override rejects oversized content
- BC invariant 2: `max_bytes_per_call = None` uses argument as-is
- Registry: TOML `write_file` capability block parses correctly (EC-008)
- Registry: `write_file` capability rejects unknown fields (deny_unknown_fields)
- SDK AC-4: non-wasm stub returns `Err(CapabilityDenied)` via `from_code(-1)`
- SDK AC-4: `max_bytes=0` and `timeout_ms=0` accepted without panic
- SDK AC-4: empty contents accepted without panic
- SDK AC-4: `HostError` enum covers all write_file outcome codes (invariant 5)
- SDK AC-7: crate version is `0.2.0`
- SDK doctest 1: typical usage shows `Err(CapabilityDenied)` on non-wasm
- SDK doctest 2: max_bytes + timeout_ms mandatory parameters accepted

## ACs / BC Postconditions Covered

| AC | BC Clause | Status |
|----|-----------|--------|
| AC-2 | BC postcondition 1 (allowlist denial via full linker) | Tested |
| AC-2 | BC postcondition 2 (byte cap via full linker) | Tested |
| AC-2 | BC postcondition 3 (successful write via full linker) | Tested |
| AC-3 | BC invariant 1 (HOST_ABI_VERSION = 1) | Tested (passing) |
| AC-4 | BC postcondition 1 (SDK stub → CapabilityDenied) | Tested |
| AC-4 | BC invariant 2 (max_bytes mandatory, no panic) | Tested |
| AC-5 | BC invariant 2 (max_bytes_per_call override) | Tested |
| AC-6 | BC invariant (EC-007 ABI backward compat) | Tested |
| AC-7 | BC postcondition 6 (CHANGELOG entry) | FAILING |
| AC-8 | BC postcondition 6 (HOST_ABI.md — verified by stub) | Pre-existing |
| EC-001 | Path traversal → CAPABILITY_DENIED | FAILING |
| EC-004 | timeout_ms=0 accepted | Tested |
| EC-007 | 0.1.x plugin loads fine | Tested |
| EC-008 | Registry TOML write_file block | Tested |

## Pre-existing Tests

All pre-existing tests still pass. The 5 stub-architect unit tests in
`crates/factory-dispatcher/src/host/write_file.rs` remain green.
