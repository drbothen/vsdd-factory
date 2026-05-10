# Adversarial Review — S-12.04 Pass 2

**Story:** S-12.04 — WASM Resolver Host ABI & Engine Dispatch  
**Pass:** 2  
**Date:** 2026-05-10  
**Branch SHA at review:** 3ce804bd  
**Classification:** HIGH (BLOCKERS_PRESENT)  
**Findings:** 9 (4 HIGH, 3 MEDIUM, 1 LOW, 1 MEDIUM-unspecified)

---

## Summary

Pass-2 adversarial review of S-12.04 after pass-1 remediation. Four HIGH-severity blockers
remain. The implementation still uses a WASM proxy shim in three acceptance-criteria tests
rather than the real WASM host ABI path. Critical epoch-deadline and context_key wiring is
absent or dead code. `fail_closed` policy is silently dropped.

Process gap noted: the HOST_ABI "Resolver Memory Protocol" section referenced in the story
spec does not exist in the current artifact.

---

## Findings

### F-S12.04-P2-001 — HIGH: missing set_epoch_deadline

**Location:** Engine dispatch path / WASM execution configuration  
**Description:** The `set_epoch_deadline` call is absent from the WASM execution setup. Without
this call the engine has no fuel / epoch bound, meaning a misbehaving or adversarially crafted
resolver module can loop indefinitely. The acceptance criteria (AC-003) require bounded
execution, but the implementation does not configure the deadline before invoking the module.  
**Impact:** Resolver invocations are unbounded — denial of service via infinite loop is trivially
achievable.  
**Required fix:** Call `store.set_epoch_deadline(N)` (where N is derived from the configured
timeout or a safe default) before each resolver `call` invocation. Wire the epoch-incrementing
thread or use Wasmtime's `Config::epoch_interruption`.

---

### F-S12.04-P2-002 — HIGH: context_key parsed but unused during merge

**Location:** Host ABI / context propagation  
**Description:** `context_key` is parsed from the resolver manifest and stored, but is never
consulted during the merge step that assembles the final resolution result. The field was
introduced to scope which context entries a resolver may read; ignoring it during merge means
all context is always visible to all resolvers, violating the isolation guarantee.  
**Impact:** Resolver isolation broken — any resolver can observe context entries it should not
have access to.  
**Required fix:** Thread `context_key` through to the merge logic and filter context entries
accordingly before passing the merged value downstream.

---

### F-S12.04-P2-003 — HIGH: fail_closed silently dropped

**Location:** Resolver error handling / policy enforcement  
**Description:** When a resolver returns an error and the configured policy is `fail_closed`,
the engine swallows the error and proceeds as if the resolver succeeded (or returned empty).
The `fail_closed` branch exists in the match arm but its `Err` path falls through to the
`fail_open` behavior. This is a silent policy violation.  
**Impact:** `fail_closed` configured resolvers silently fail open — security-critical resolution
steps may be bypassed without any observable error.  
**Required fix:** Ensure the `fail_closed` match arm propagates the error upward and halts the
resolution chain. Add a regression test that asserts an error is returned (not swallowed) when
a `fail_closed` resolver returns `Err`.

---

### F-S12.04-P2-004 — HIGH: AC-006/008/009 use proxy not real WASM

**Location:** Acceptance criteria test suite — AC-006, AC-008, AC-009  
**Description:** Tests for AC-006 (host function surface), AC-008 (memory limits), and AC-009
(module linking) instantiate a hand-rolled Rust proxy struct that implements the same trait
interface, rather than loading and executing a compiled `.wasm` binary through the actual
Wasmtime host ABI path. These tests therefore validate the trait contract but not the real
WASM execution boundary.  
**Impact:** The WASM host ABI surface (memory layout, function import names, linear memory
access patterns) is untested. Regressions in the ABI can pass CI undetected.  
**Required fix:** Replace proxy instantiation in AC-006/008/009 with fixtures that load a
minimal compiled `.wasm` module (can be a checked-in test fixture built from WAT source).
The test must exercise the actual `wasmtime::Linker` and `Store` path.

---

### F-S12.04-P2-005 — MEDIUM: AC-011 path_escaping_resolver fixture missing

**Location:** Acceptance criteria test — AC-011  
**Description:** AC-011 is intended to verify that a resolver module attempting path traversal
(e.g., `../../etc/passwd` style access) is rejected. The test references a
`path_escaping_resolver` fixture but no such `.wasm` fixture exists in the repository. The
test is either skipped or panics at fixture load time.  
**Impact:** Path-traversal rejection is untested in CI.  
**Required fix:** Create the `path_escaping_resolver.wat` / `.wasm` fixture that attempts an
out-of-sandbox path access and check it into the test fixtures directory.

---

### F-S12.04-P2-006 — MEDIUM: no deny_unknown_fields on TOML structs

**Location:** Resolver manifest deserialization  
**Description:** The `ResolverManifest` and related TOML-deserialized structs do not use
`#[serde(deny_unknown_fields)]`. Unknown fields silently succeed deserialization, making it
impossible to detect typos in user-supplied manifests (e.g., `failt_closed` instead of
`fail_closed`).  
**Impact:** Misconfigured manifests silently load with wrong policy — operational risk and
difficult-to-diagnose production bugs.  
**Required fix:** Add `#[serde(deny_unknown_fields)]` to all manifest structs. Update tests to
assert that unknown fields produce a deserialization error.

---

### F-S12.04-P2-007 — MEDIUM: i32 unpack of packed u32 values

**Location:** Host ABI / WASM function return value unpacking  
**Description:** The host ABI decodes packed return values (offset, length pairs) using `i32`
Wasmtime value extraction. WASM linear memory addresses are `u32` and packed pairs frequently
exceed `i32::MAX` when the upper half encodes the length. Extracting as `i32` silently
sign-extends, producing negative or incorrect offsets for memory regions above 2 GiB (or for
large length values).  
**Impact:** Memory reads from resolver output corrupt data for any module that uses the upper
address space or encodes lengths > 2^31.  
**Required fix:** Extract packed values using `u32` (via `as u32` cast from the WASM `i32`
representation, which is the correct ABI idiom). Document the packing convention explicitly.

---

### F-S12.04-P2-008 — MEDIUM: (0,0) packed return undocumented

**Location:** HOST_ABI spec / resolver return convention  
**Description:** The convention that a resolver signals "no result" by returning the packed
value `(offset=0, length=0)` is implemented in the engine but not documented in the story
spec or in any in-code doc comment. Resolver authors implementing the ABI from the spec alone
will not know this sentinel exists and may interpret a zero-zero return as an error.  
**Impact:** Third-party resolver authors will implement the ABI incorrectly, producing silent
empty results instead of explicit errors.  
**Required fix:** Document the `(0,0)` sentinel in the HOST_ABI spec section and in the
relevant `invoke_resolver_wasm` doc comment.

---

### F-S12.04-P2-009 — LOW: invoke_resolver_wasm dual API surface

**Location:** `invoke_resolver_wasm` function  
**Description:** `invoke_resolver_wasm` exposes two calling conventions: one that takes a
pre-instantiated `Instance` and one that takes a raw module path and instantiates internally.
Both paths exist in the same function via an enum parameter. This dual surface complicates
testing, makes error paths harder to reason about, and leaks instantiation concerns into the
call site.  
**Impact:** Low — no correctness issue, but increases maintenance surface and makes the API
harder to evolve.  
**Suggested fix:** Split into `invoke_resolver_instance` (takes `Instance`) and
`load_and_invoke_resolver` (takes path). Deprecate or gate the dual-mode entry point.

---

## Process Gap

The story spec references a HOST_ABI section titled "Resolver Memory Protocol" that describes
the linear memory handshake, pointer packing convention, and memory limit enforcement. This
section does not exist in the current artifact. Findings F-S12.04-P2-007 and F-S12.04-P2-008
are direct consequences of the missing documentation — the implementation diverged from an
undocumented intent.

**Required fix (process):** Author the "Resolver Memory Protocol" section in the HOST_ABI
spec before Pass 3. This section must cover: memory layout, `(offset, length)` packing as
`u32`, the `(0,0)` no-result sentinel, memory limit enforcement hook points, and
out-of-bounds access rejection.

---

## Pass Verdict

**BLOCKERS_PRESENT — normalized to HIGH**

All four HIGH findings (F-001 through F-004) are blockers. Pass-3 may proceed only after
remediation of F-001, F-002, F-003, F-004, and F-007 (i32/u32 ABI correctness). MEDIUM
findings F-005, F-006, F-008 should be resolved in the same remediation burst. F-009 (LOW)
may be deferred.
