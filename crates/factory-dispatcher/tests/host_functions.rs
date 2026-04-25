//! Integration test: `setup_linker` registers every host import against a
//! real wasmtime [`Engine`] without error, and a minimal WAT module that
//! imports a couple of those functions instantiates cleanly.
//!
//! Running the plugin to completion (calling `_start`, observing
//! `emit_event` on a real guest) is S-1.5 / S-1.6 territory — it needs
//! the wasmtime scheduler, fuel budget, and store lifecycle that those
//! stories implement. This test verifies the link surface alone.

use factory_dispatcher::host::{HostContext, setup_linker};
use wasmtime::{Engine, Module, Store};

const WAT_SMOKE: &str = r#"
(module
  (import "vsdd" "log" (func $log (param i32 i32 i32)))
  (import "vsdd" "emit_event" (func $emit (param i32 i32 i32 i32)))
  (import "vsdd" "session_id" (func $sid (param i32 i32) (result i32)))
  (import "vsdd" "env" (func $env (param i32 i32 i32 i32) (result i32)))
  (import "vsdd" "read_file" (func $rf (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "vsdd" "exec_subprocess"
    (func $exec (param i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "_start")
    nop))
"#;

#[test]
fn setup_linker_registers_every_vsdd_import() {
    let engine = Engine::default();
    let linker = setup_linker(&engine).expect("linker setup failed");

    // Every fn we expect:
    let expected = [
        "log",
        "emit_event",
        "read_file",
        "exec_subprocess",
        "session_id",
        "dispatcher_trace_id",
        "plugin_root",
        "plugin_version",
        "cwd",
        "env",
    ];
    // The wasmtime API doesn't expose a direct lookup on Linker, but
    // `get` against a fresh Store resolves the Extern if it's been
    // defined. We use a context-less empty Store for lookup only.
    let mut store = Store::new(&engine, HostContext::new("p", "0.0.1", "sess", "trace"));
    for name in expected {
        linker
            .get(&mut store, "vsdd", name)
            .unwrap_or_else(|_| panic!("vsdd::{name} was not registered"));
    }
}

#[test]
fn wat_module_importing_host_functions_instantiates() {
    let engine = Engine::default();
    let linker = setup_linker(&engine).expect("linker setup failed");
    let module = Module::new(&engine, WAT_SMOKE).expect("wat should parse");

    let mut store = Store::new(
        &engine,
        HostContext::new("hello-hook", "0.0.1", "sess", "trace"),
    );
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instance should link cleanly against the linker");

    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .expect("_start export should resolve");
    start
        .call(&mut store, ())
        .expect("_start runs to completion with no host traps");
}
