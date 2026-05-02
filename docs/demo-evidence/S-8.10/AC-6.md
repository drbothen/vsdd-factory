# AC-6: Legacy plugin regression preserved

**Criterion:** `cargo test --workspace` passes; existing hook-plugin crates compile
and pass tests. `crates/factory-dispatcher/tests/host_functions.rs` passes unchanged.
EC-007: plugin without `write_file` import loads against new dispatcher.

**Trace:** BC-2.01.003 invariant 2 (ABI backward compatibility).

---

## host_functions.rs Regression Tests

```
Running tests/host_functions.rs
test setup_linker_registers_every_vsdd_import ... ok
test wat_module_importing_host_functions_instantiates ... ok
test result: ok. 2 passed; 0 failed
```

Both pre-existing `host_functions.rs` tests pass unchanged, confirming no
regression in the linker registration path.

---

## EC-007: Backward Compatibility (SDK 0.1.x Plugin)

```
test test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher ... ok
```

A WAT module that imports only `vsdd::log` (no `write_file`) — representing a
plugin compiled against SDK 0.1.x — instantiates and runs `_start` successfully
against a dispatcher that exports `write_file`. Wasmtime silently ignores the
additional host export.

Test implementation:

```rust
const WAT_NO_WRITE_IMPORT: &str = r#"
(module
  (import "vsdd" "log" (func $log (param i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "_start") nop)
)
"#;

fn test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher() {
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("0.1.x plugin must load against new linker (BC-2.02.011 EC-007)");
    start
        .call(&mut store, ())
        .expect("_start runs to completion — no regression (BC-2.01.003 invariant 2)");
}
```

---

## Full Dispatcher Test Suite Summary

```
test result: ok. 110 passed; 0 failed; 0 ignored  (lib unit tests)
test result: ok. 7 passed; 0 failed   (bc_2_02_011_parity.rs)
test result: ok. 10 passed; 0 failed  (host_write_file_integration.rs)
test result: ok. 2 passed; 0 failed   (host_functions.rs)
test result: ok. 23 passed; 0 failed  (bc_9_01_rc1_release_gate_test.rs)
test result: ok. 5 passed; 0 failed   (executor_integration.rs)
test result: ok. 40 passed; 0 failed  (s4_07_integration.rs)
test result: ok. 1 passed; 0 failed   (loads_legacy_registry.rs)
```

0 regressions across the full workspace.

**Status: PASS**
