# Affected Files — validation-integrity-layer1 (F1 Delta)
<!-- Machine-readable section: STATUS  path -->

```
NEW       crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs
NEW       crates/hook-plugins/validate-unvalidated-mutation-marker/Cargo.toml
NEW       crates/factory-dispatcher/src/indeterminate_marker.rs
NEW       plugins/vsdd-factory/tests/validate_indeterminate_marker.bats

MODIFIED  crates/factory-dispatcher/src/executor.rs
MODIFIED  crates/factory-dispatcher/src/invoke.rs
MODIFIED  crates/factory-dispatcher/Cargo.toml
MODIFIED  Cargo.toml
MODIFIED  plugins/vsdd-factory/hooks-registry.toml

DEPENDENT crates/factory-dispatcher/src/engine.rs
DEPENDENT crates/factory-dispatcher/src/main.rs
DEPENDENT crates/factory-dispatcher/src/aggregator.rs
DEPENDENT crates/factory-dispatcher/src/routing.rs
```

## Regression Baseline (Must Stay Green)

All files in `crates/` NOT listed above are in the regression baseline.
All files in `plugins/vsdd-factory/` NOT listed above are in the regression baseline.

Key regression-baseline test files:
- `crates/factory-dispatcher/src/registry.rs` (test mod `s21_10_bc_1_01_016_failure_policy`)
- `plugins/vsdd-factory/tests/` (all existing `.bats` files)
- `crates/hook-plugins/*/src/lib.rs` (all existing plugin crates)
