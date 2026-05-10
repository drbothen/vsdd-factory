# Demo Evidence — S-12.05: hook-sdk Resolver-Authoring Extensions

**Branch:** `feature/S-12.05-hook-sdk-resolver` at `5acd1f4a`
**Status:** CONVERGED (3 consecutive NITPICK_ONLY adversarial passes)
**Story spec:** `.factory/stories/S-12.05-hook-sdk-resolver-extensions.md`
**Behavioral contract:** BC-4.12.002 (Resolver ABI uses distinct types, versioned independently)
**Verification property:** VP-075 (serde round-trip determinism)

All demos were produced by running real `cargo` commands against the feature branch.
No output was manufactured.

## AC-to-Demo Mapping

| AC | Description | Demo File | Result |
|----|-------------|-----------|--------|
| AC-001 | `RESOLVER_ABI_VERSION = 1` constant defined | `01-abi-version-test.txt` | PASS |
| AC-002 | `ResolverInput` type shape and serde round-trip | `02-trybuild-type-mismatch.txt` | PASS |
| AC-003 | `ResolverOutput` type: value=None serializes as null | `02-trybuild-type-mismatch.txt` | PASS |
| AC-004 | `ResolverInput`/`ResolverOutput` distinct from `HookPayload`/`HookResult` | `02-trybuild-type-mismatch.txt` | PASS |
| AC-005 | `#[resolver]` macro generates correct WASM `resolve` export | `05-wasm32-export-verification.txt` | PASS |
| AC-006 | `#[resolver]` rejects wrong signatures (wrong_sig + async) | `03-trybuild-wrong-sig.txt`, `04-trybuild-async-resolver.txt` | PASS |
| AC-007 | `resolver-authoring` feature flag gates all resolver types | `06-feature-gate.txt` | PASS |
| AC-008 | `RESOLVER_ABI_VERSION` independently defined from `HOST_ABI_VERSION` | `07-host-abi-independence.txt` | PASS |
| AC-009 | Existing hook authoring surface unchanged, no regression | `09-full-test-suite.txt` | PASS |
| AC-010 | `ResolverInput`/`ResolverOutput` serde round-trips deterministic (proptest 100 trials) | `09-full-test-suite.txt` | PASS |
| Implicit | `hook-sdk` bumped to 0.3.0, `hook-sdk-macros` to 0.2.0 | `08-changelog-and-versions.txt` | CONFIRMED |

## Demo Files

| File | Content |
|------|---------|
| `01-abi-version-test.txt` | `cargo test tests::test_BC_4_12_002_resolver_abi_version_is_1` — PASS |
| `02-trybuild-type-mismatch.txt` | Serde round-trip tests (AC-002, AC-003) + type distinctness trybuild (AC-004) — PASS |
| `03-trybuild-wrong-sig.txt` | `#[resolver]` rejects wrong return type (AC-006 path 1) — PASS |
| `04-trybuild-async-resolver.txt` | `#[resolver]` rejects async fn (AC-006 path 2) — PASS |
| `05-wasm32-export-verification.txt` | trybuild positive compile + wasmparser WASM export verification (AC-005) — PASS |
| `06-feature-gate.txt` | `cargo build -p vsdd-hook-sdk` (no feature, zero errors) + feature-gate structural test (AC-007) — PASS |
| `07-host-abi-independence.txt` | Grep showing distinct declaration sites + unit test (AC-008) — PASS |
| `08-changelog-and-versions.txt` | `git diff develop...HEAD` version bumps + CHANGELOG.md entry — CONFIRMED |
| `09-full-test-suite.txt` | `cargo test --workspace --features resolver-authoring` — 0 failures (AC-009, AC-010) |

## Reproduction

Any operator can reproduce all demos by running the commands listed in each file
against the `feature/S-12.05-hook-sdk-resolver` branch.

Prerequisites:
- Rust toolchain (see `rust-toolchain.toml`)
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1` (required for AC-005 WASM test)
