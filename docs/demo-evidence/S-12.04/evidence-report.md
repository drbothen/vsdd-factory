# Evidence Report: S-12.04

**Story:** S-12.04 — WASM Resolver Loading, Lifecycle, and Error Isolation  
**Branch:** feature/S-12.04-wasm-resolver-loading  
**Recorded:** 2026-05-10  
**Recorder:** demo-recorder (Step 5 post-convergence)

## Coverage

| AC | Title | Demo File | Status |
|----|-------|-----------|--------|
| AC-001 | Absent registry = empty, no error | 01-absent-registry.txt | PASS |
| AC-002 | Malformed TOML = ParseError + fail-loud | 02-malformed-toml.txt | PASS |
| AC-003 | Module compiled once; cache hit | 03-mtime-cache-hit.txt | PASS |
| AC-004 | mtime change triggers recompile | 04-mtime-recompilation.txt | PASS |
| AC-005 | load_does_not_invoke_resolve | 05-load-does-not-invoke-resolve.txt | PASS |
| AC-006 | Failed compile = fail-loud CompileError | 06-failed-compile-fail-loud.txt | PASS |
| AC-007 | classify_resolver_trap totality (kani + byte-iter unit) | 07-kani-totality-byte-iter.txt | PASS (unit); kani blocked toolchain gap |
| AC-008 | resolver linker excludes write/exec/emit | 08-linker-capabilities.txt | PASS |
| AC-009 | Trapping resolver does not abort dispatch | 09-trapping-resolver-isolation.txt | PASS |
| AC-010 | resolver.error event with structured fields | 10-resolver-error-event.txt | PASS |
| AC-011 | capability_denied (deferred per F-P2-005 to S-12.07) | 11-capability-denied-deferred.txt | PARTIAL |
| AC-012 | Startup log "Compiled N resolver modules" | 12-startup-log.txt | PASS |
| AC-013 | Missing .wasm path = CompileError/IoError | 13-missing-wasm-error.txt | PASS |

## Supplementary Evidence

| File | Content |
|------|---------|
| 14-full-test-suite.txt | Full cargo test -p factory-dispatcher (0 failures, 28 binaries) |
| 15-host-abi-resolver-events.txt | HOST_ABI.md resolver event table excerpts |

## Notable Observations

1. **Startup log confirmed live** (AC-012): `--nocapture` output shows `factory-dispatcher: Compiled 1 resolver modules from <path>` emitted by the dispatcher when resolvers load successfully.

2. **Kani toolchain gap** (AC-007): kani 0.67.0 requires nightly 1.93; workspace pins stable 1.95. Harnesses compile under `#[cfg(kani)]` and are authored per VP-074 spec. Exhaustive byte-iter unit test covers the same totality property.

3. **trapping_resolver.wasm fixture** in use: error-isolation tests load the real WASM fixture at `tests/fixtures/trapping_resolver.wasm` and confirm `unreachable` traps are caught, classified as `ResolverError::Trap`, emitted as `resolver.error`, and the dispatcher continues.

4. **AC-011 partial coverage**: path_allow unit enforcement tested. VP-076 full integration (live WASM dispatching to `/etc/passwd`) scoped to S-12.07.

5. **Zero regressions**: All 28 test binaries pass. Pre-existing factory-dispatcher tests unaffected.
