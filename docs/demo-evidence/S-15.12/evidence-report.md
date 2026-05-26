# Demo Evidence: S-15.12 validate-closes-completeness WASM Hook Phase 1

**Story:** S-15.12  
**BC:** BC-5.39.007 v1.5  
**Evidence type:** Test execution (WASM hook — no UI; evidence is bats/cargo test runs)

## AC Coverage

| AC | Verification Method | Result |
|----|---------------------|--------|
| AC-1 | `fail-lessons-missing-closes.bats` | PASS (hook blocks as expected) |
| AC-2 | `fail-lessons-empty-closes.bats` | PASS (hook blocks as expected) |
| AC-3 | `fail-lessons-wrong-closes-format.bats` | PASS (hook blocks as expected) |
| AC-4 | `pass-lessons-valid-closes.bats` | PASS (hook continues as expected) |
| AC-5 | `fail-forbidden-annotation.bats` | PASS (hook blocks as expected) |
| AC-6 | `fail-forbidden-shorthand.bats` | PASS (hook blocks as expected) |
| AC-7 | `fail-state-umbrella-no-flag.bats` | PASS (hook blocks as expected) |
| AC-8 | `pass-state-umbrella-with-flag.bats` | PASS (hook continues as expected) |
| AC-9 | `fail-index-umbrella-no-flag.bats` | PASS (hook blocks as expected) |
| AC-10 | `fail-decisionlog-umbrella-no-flag.bats` | PASS (hook blocks as expected) |
| AC-11 | `pass-decisionlog-umbrella-exhaustive.bats` | PASS (hook continues as expected) |
| AC-12 | `fail-malformed-cite.bats` | PASS (hook blocks as expected) |
| AC-13 | `pass-exemption-declared.bats` | PASS (hook continues as expected) |
| AC-14 | `fail-exemption-not-declared.bats` | PASS (hook blocks as expected) |
| AC-15 | `fail-multiple-violations.bats` | PASS (single BlockWithFix with all violations) |
| AC-16 | `fail-open-unreadable.bats` | PASS (Continue + log_warn, no block) |
| AC-17 | `pass-xstate-md-not-target.bats` | PASS (path-component-strict guard works) |
| AC-18 | `pass-phase1-advisory-only.bats` | PASS (D-999 cite → Continue + log_warn) |
| AC-19 | `cargo build --release --target wasm32-wasip1` | PASS (zero warnings) |
| AC-20 | `integration-production-registry.bats` | PASS (registry shape confirmed) |
| AC-21 | Pre-flight 4-gate | PASS (fmt + clippy + bats + cargo all clean) |
| AC-22 | `pass-empty-file.bats` | PASS (Continue + advisory log) |

## Test Summary

- **36/36 bats integration tests PASS**
- **43/43 cargo unit tests PASS**
- `cargo fmt --check --all` CLEAN
- `cargo clippy --workspace --all-targets -- -D warnings` CLEAN

## LOCAL Adversary Cascade

**Result: CONVERGED 3/3 (8 passes; trajectory 4→2→0→0→1→0→0→0)**

Per BC-5.39.001 3-CLEAN convergence protocol. All findings resolved before PR dispatch.
