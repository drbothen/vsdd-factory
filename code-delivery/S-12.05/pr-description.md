## Summary

S-12.05 adds the hook-sdk authoring surface for WASM-plugin context resolvers per BC-4.12.002 v1.2. New public types (`ResolverInput`, `ResolverOutput`), constant (`RESOLVER_ABI_VERSION = 1`), and proc-macro (`#[resolver]`) — all gated behind the `resolver-authoring` feature flag.

This is the author-side counterpart to S-12.03 (the registry side). Together with S-12.04 (WASM loading) and S-12.07 (concrete WaveContextResolver), this completes the platform that closes F-P2-001 in S-12.08.

## Behavioral Contracts

- **BC-4.12.002 v1.2** (PRIMARY): resolver ABI + payload schema; PC1 packed-i64 ABI, PC2/PC3 type shapes, PC4 RESOLVER_ABI_VERSION = 1, PC5 #[resolver] macro, PC8 feature gate, INV1 distinct types, INV2 independent versioning

## Acceptance Criteria

10 ACs (AC-001..AC-010) all verified by tests. See docs/demo-evidence/S-12.05/README.md for AC-to-test mapping + demo evidence.

## Test Coverage

- crates/hook-sdk/tests/resolver_types_test.rs — type-level tests + trybuild fixtures (16 tests)
- crates/hook-sdk/tests/wasm32_resolver_export_integration.rs — wasm32-wasip1 cross-compile + wasmparser export verification (#[ignore]'d for default CI)
- crates/hook-sdk-macros/src/resolver_macro.rs — proc-macro implementation
- crates/hook-sdk/examples/wasm_resolver_export/ — example crate validating the public surface

## Adversarial Convergence (BC-5.39.001)

| Pass | Classification | Findings |
|------|---------------|----------|
| 1 | HIGH | 7 |
| 2 | HIGH | 8 |
| 3 | LOW | 4 |
| 4 | MEDIUM | 5 (regression — partial-fix sweep) |
| 5 | NITPICK_ONLY | 2 |
| 6 | NITPICK_ONLY | 2 |
| 7 | NITPICK_ONLY | 1 |

**3 consecutive NITPICK_ONLY achieved. Story converged 2026-05-10.**

## SemVer Impact

- crates/hook-sdk 0.2.0 → 0.3.0 (minor — adds public types/macro behind feature flag)
- crates/hook-sdk-macros 0.1.0 → 0.2.0 (minor — adds public proc-macro)

CHANGELOG entries at crates/hook-sdk/CHANGELOG.md.

## Architecture Anchors

- BC-4.12.002 v1.2 (resolver ABI)
- ADR-018 (WASM-plugin Context Resolvers — Design and Layering)
- HOST_ABI.md §SDK Authoring Surface (resolver-authoring feature)

## Demo Evidence

docs/demo-evidence/S-12.05/ — 10 demo files covering all 10 ACs.

## Security Review

Reviewed: no injection vectors, no auth surface, no OWASP top-10 exposure. WASM proc-macro emits extern "C" exports only; no host-side parsing of untrusted input in this crate. Classification: LOW RISK.

## Risk Assessment

- Blast radius: crates/hook-sdk, crates/hook-sdk-macros only; feature-flagged (resolver-authoring). Existing hook-sdk consumers unaffected unless they opt in.
- Performance impact: None at runtime (types + macro); macro expansion is compile-time only.
- Rollback: Remove feature flag from Cargo.toml; no database migrations.

## Test plan

- [x] Cargo workspace tests pass (cargo test --workspace --all-targets)
- [x] Cargo fmt --check passes
- [x] Cargo clippy --workspace -- -D warnings passes
- [x] Bats suite passes (plugins/vsdd-factory/tests)
- [x] Adversarial convergence: 3 consecutive NITPICK_ONLY passes (BC-5.39.001)
- [x] Demo evidence captured for all 10 ACs (POLICY 10)
- [x] CHANGELOG entries added
- [x] Cargo.toml versions bumped (SemVer minor)
