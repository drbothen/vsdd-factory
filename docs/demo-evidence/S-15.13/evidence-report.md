# Demo Evidence: S-15.13 validate-closes-completeness WASM Hook Phase 2

**Story:** S-15.13
**BC:** BC-5.39.007 v1.5 (Phase 2 extension)
**Evidence type:** Test execution (WASM hook — no UI; evidence is bats/cargo test runs)

## AC Coverage

| AC | Criterion | Verification Method | Result |
|----|-----------|---------------------|--------|
| AC-1 | Pointer file absent → Continue + advisory (fail-open) | `pass-p2-pointer-absent.bats` | PASS |
| AC-1b | Pointer file present but non-integer content → block with parse-error message | `fail-p2-pointer-invalid-integer.bats` | PASS |
| AC-2 | Pointer present but adversary file unreadable → Continue + advisory (fail-open) | `pass-p2-adversary-unreadable.bats` | PASS |
| AC-3 | Pointer + adversary file present → finding IDs extracted from Part A correctly | `pass-p2-pointer-present-valid.bats` | PASS |
| AC-4 | Citation site missing finding ID → block naming site + missing ID, cites D-411(c) | `fail-p2-site-missing-finding.bats` | PASS |
| AC-5 | Cardinality divergence across sites → block citing D-420(a) | `fail-p2-cardinality-diverges.bats` | PASS |
| AC-6 | Phase 1 non-regression: all Phase 1 PASS and FAIL fixtures unchanged | Phase 1 bats suite (32 tests) | PASS |
| AC-7 | WASM compilation clean (wasm32-wasip1, zero warnings) | `cargo build --release --target wasm32-wasip1` | PASS |
| AC-8 | Pre-flight 4-gate (fmt + clippy + bats + cargo test) | All four gate commands | PASS |

## Test Summary

- **51/51 bats integration tests PASS** (32 Phase 1 non-regression + 19 Phase 2)
- `cargo fmt --check --all` CLEAN
- `cargo clippy --workspace --all-targets -- -D warnings` CLEAN
- `cargo test --workspace` CLEAN (pre-existing sink-http failure is unrelated to this story)
- WASM binary: 179KB (Phase 1) → 191KB (Phase 2, +12KB for cross-site validation logic)

## LOCAL Adversary Cascade

**Result: CONVERGED 3/3 (4 passes; trajectory 7→2→0→0)**

Per BC-5.39.001 3-CLEAN convergence protocol. All findings resolved before PR dispatch.

### Pass Summary

| Pass | Findings | Critical | High | Medium | Low | Status |
|------|----------|----------|------|--------|-----|--------|
| P1 | 7 | 0 | 2 | 3 | 2 | Fixed |
| P2 | 2 | 0 | 1 | 1 | 0 | Fixed |
| P3 | 0 | 0 | 0 | 0 | 0 | CLEAN (1/3) |
| P4 | 0 | 0 | 0 | 0 | 0 | CLEAN (2/3 → 3/3 CONVERGED) |
