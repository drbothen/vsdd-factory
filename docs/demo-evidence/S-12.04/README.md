# Demo Evidence: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation

**Story:** S-12.04  
**Branch:** feature/S-12.04-wasm-resolver-loading  
**Recorded:** 2026-05-10  
**Status:** CONVERGED (passes_clean=3, 11 adversary passes)

## AC-to-File Mapping

| File | AC | Test Name | Result |
|------|----|-----------|--------|
| 01-absent-registry.txt | AC-001 | test_BC_1_13_001_absent_registry_yields_empty_no_error | PASS |
| 02-malformed-toml.txt | AC-002 | test_BC_1_13_001_malformed_toml_returns_parse_error | PASS |
| 03-mtime-cache-hit.txt | AC-003 | test_BC_4_12_001_load_is_deterministic | PASS |
| 04-mtime-recompilation.txt | AC-004 | test_BC_4_12_001_mtime_change_triggers_recompilation | PASS |
| 05-load-does-not-invoke-resolve.txt | AC-005 | test_BC_4_12_001_load_does_not_invoke_resolve | PASS |
| 06-failed-compile-fail-loud.txt | AC-006 | test_BC_4_12_001_failed_compile_returns_compile_error + test_F_P1_001 | PASS |
| 07-kani-totality-byte-iter.txt | AC-007 | test_classify_resolver_trap_total_byte_iter (unit); kani blocked by toolchain | PASS (unit); BLOCKED (kani) |
| 08-linker-capabilities.txt | AC-008 | test_BC_4_12_003_resolver_linker_excludes_write_file_exec_emit | PASS |
| 09-trapping-resolver-isolation.txt | AC-009 | test_BC_4_12_004_trapping_resolver_does_not_abort_dispatch | PASS |
| 10-resolver-error-event.txt | AC-010 | test_BC_4_12_004_trapping_resolver_emits_resolver_error_event | PASS |
| 11-capability-denied-deferred.txt | AC-011 | test_capability_denied_when_resolver_attempts_disallowed_read (unit); VP-076 full integration deferred to S-12.07 | PARTIAL |
| 12-startup-log.txt | AC-012 | test_resolver_path_allow_is_project_dir_relative (--nocapture shows startup log) | PASS |
| 13-missing-wasm-error.txt | AC-013 | test_BC_4_12_001_failed_compile_returns_compile_error | PASS |
| 14-full-test-suite.txt | regression | cargo test -p factory-dispatcher (all 28 binaries) | PASS (0 failures) |
| 15-host-abi-resolver-events.txt | reference | HOST_ABI.md resolver event table excerpts | N/A |

## AC-007 Kani Note

Kani 0.67.0 requires rustc 1.93.0-nightly; this workspace pins stable 1.95+. The toolchain gap is documented in `resolver_classify_trap.rs` lines 80-88. Three harnesses are authored under `#[cfg(kani)]` and compile cleanly under `cargo check`. The exhaustive byte-iter unit test (`test_classify_resolver_trap_total_byte_iter`) covers the same totality property over all 256 possible `u8` values.

## AC-011 Deferral Note

Per story spec AC-011 and F-P2-005: the full VP-076 integration test using `path_escaping_resolver.wasm` (reads `/etc/passwd`) is deferred to S-12.07. The `path_allow` enforcement unit test in S-12.04 confirms the mechanism works. The `path_escaping_resolver.wasm` fixture is committed in this story.

## Test Coverage Summary

- resolver_load_test.rs: 18 tests (0 failures)
- resolver_error_isolation_test.rs: 4 tests (0 failures)
- host::linker_capability_tests: 2 tests (0 failures)
- resolver_classify_trap::tests: 3 tests (0 failures)
- Total factory-dispatcher suite: 0 failures across all 28 test binaries
