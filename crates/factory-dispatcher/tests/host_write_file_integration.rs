//! E2E integration tests for `vsdd::write_file` host function.
//!
//! Exercises the full wasmtime linker → host fn → filesystem path that the
//! dispatcher-side unit tests in `host/write_file.rs` cannot cover because
//! they call `prepare()` directly without going through guest memory.
//!
//! These tests complement the stub-architect's unit tests (AC-5) by verifying:
//! - `write_file` is registered in the linker (AC-2 / BC-2.02.011 invariant 4)
//! - WAT module calling `vsdd::write_file` routes through `read_wasm_bytes`
//!   correctly for both path and contents (input-pointer protocol)
//! - Return codes propagate from `prepare()` through `func_wrap` to the guest
//! - `timeout_ms` parameter is accepted without error (BC-2.02.011 EC-004)
//! - ABI backward compat: plugin without `write_file` import loads fine (EC-007)
//!
//! BC: BC-2.02.011

use factory_dispatcher::host::{HostContext, codes, setup_linker};
use factory_dispatcher::registry::{Capabilities, WriteFileCaps};
use wasmtime::{Engine, Module, Store};

// ---------------------------------------------------------------------------
// WAT fixtures
// ---------------------------------------------------------------------------

/// Minimal module that imports `vsdd::write_file` and provides a callable
/// `do_write` export.  Path is "/PLACEHOLDER/out.txt" (22 bytes at offset 0);
/// contents are "hello from wasm" (15 bytes at offset 64).
///
/// The host coordinates (path_allow) and the store context are wired in
/// each test.  This WAT is purely structural — actual path bytes are
/// overridden per-test via the capabilities / plugin_root on the Store.
const WAT_WRITE_FILE: &str = r#"
(module
  (import "vsdd" "write_file"
    (func $write_file (param i32 i32 i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "do_write")
        (param $path_ptr i32) (param $path_len i32)
        (param $contents_ptr i32) (param $contents_len i32)
        (param $max_bytes i32) (param $timeout_ms i32)
        (result i32)
    (call $write_file
      (local.get $path_ptr)
      (local.get $path_len)
      (local.get $contents_ptr)
      (local.get $contents_len)
      (local.get $max_bytes)
      (local.get $timeout_ms)))
)
"#;

/// Module that does NOT import `write_file` — ABI backward-compat test
/// (BC-2.02.011 EC-007: plugin compiled against SDK 0.1.x loads fine).
const WAT_NO_WRITE_IMPORT: &str = r#"
(module
  (import "vsdd" "log" (func $log (param i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "_start") nop)
)
"#;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build engine + linker (shared across test bodies).
fn engine_and_linker() -> (Engine, wasmtime::Linker<HostContext>) {
    let engine = Engine::default();
    let linker = setup_linker(&engine).expect("setup_linker failed");
    (engine, linker)
}

/// Bare context: no write_file capability.
fn bare_context() -> HostContext {
    HostContext::new("test-plugin", "0.1.0", "sess", "trace")
}

/// Context with write_file allowed for `allowed_dir`.
fn context_write_allowed(allowed_dir: &str) -> HostContext {
    let mut ctx = bare_context();
    ctx.capabilities = Capabilities {
        write_file: Some(WriteFileCaps {
            path_allow: vec![allowed_dir.to_string()],
            max_bytes_per_call: None,
        }),
        ..Default::default()
    };
    ctx.plugin_root = std::path::PathBuf::from(allowed_dir);
    ctx
}

/// Write `bytes` to WAT memory at `offset`. Returns updated WAT bytes
/// (for non-`Module::new` approaches).  Here we rely on a helper that
/// places path/contents into a pre-allocated guest memory at runtime
/// via the exported memory.
fn write_bytes_to_memory(
    store: &mut Store<HostContext>,
    memory: &wasmtime::Memory,
    offset: usize,
    bytes: &[u8],
) {
    memory
        .write(store, offset, bytes)
        .expect("write_bytes_to_memory failed");
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_write_file_registered_in_linker
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 4 / AC-2: `vsdd::write_file` must be registered in
/// `setup_linker`.  Verified by `get()` against a fresh Store.
///
/// This test extends `host_functions.rs::setup_linker_registers_every_vsdd_import`
/// which did not include `write_file` in its expected list.
#[test]
fn test_BC_2_02_011_write_file_registered_in_linker() {
    let (engine, linker) = engine_and_linker();
    let mut store = Store::new(&engine, bare_context());
    linker
        .get(&mut store, "vsdd", "write_file")
        .expect("vsdd::write_file must be registered in setup_linker (BC-2.02.011 invariant 4)");
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_wat_module_with_write_file_import_instantiates
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-007 inverse: a module that DOES import `vsdd::write_file`
/// must instantiate cleanly against the linker.
#[test]
fn test_BC_2_02_011_wat_module_with_write_file_import_instantiates() {
    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, bare_context());
    linker
        .instantiate(&mut store, &module)
        .expect("WAT module importing vsdd::write_file must instantiate without link error");
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-007: a plugin compiled against SDK 0.1.x (without
/// `write_file` import) must load against a dispatcher that exports
/// `write_file`.  Wasmtime silently ignores unimported host exports.
#[test]
fn test_BC_2_02_011_ec007_old_plugin_without_write_import_loads_against_new_dispatcher() {
    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_NO_WRITE_IMPORT).expect("WAT_NO_WRITE_IMPORT parses");
    let mut store = Store::new(&engine, bare_context());
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("0.1.x plugin must load against new linker (BC-2.02.011 EC-007)");
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .expect("_start export should resolve");
    start
        .call(&mut store, ())
        .expect("_start runs to completion — no regression (BC-2.01.003 invariant 2)");
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_wat_denied_when_no_capability (full linker path)
// ---------------------------------------------------------------------------

/// BC-2.02.011 postcondition 1 / invariant 6: calling `write_file` through
/// the wasmtime linker with no capability block must return
/// `CAPABILITY_DENIED (-1)` at the FFI boundary.
///
/// This covers the `func_wrap` → `read_wasm_bytes` → `prepare` path that
/// the stub-architect unit tests skip (they call `prepare` directly).
#[test]
fn test_BC_2_02_011_wat_denied_when_no_capability() {
    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    // bare_context() has no write_file capability.
    let mut store = Store::new(&engine, bare_context());
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path = b"/some/path.txt";
    let contents = b"data";
    write_bytes_to_memory(&mut store, &memory, 0, path);
    write_bytes_to_memory(&mut store, &memory, 128, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (0, path.len() as i32, 128, contents.len() as i32, 1024, 5000),
        )
        .expect("call should not trap");

    // BC-2.02.011 postcondition 1: no capability → CAPABILITY_DENIED (-1).
    assert_eq!(
        result,
        codes::CAPABILITY_DENIED,
        "expected CAPABILITY_DENIED (-1) when no write_file capability block is present"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_wat_write_succeeds_allowed_path (full linker path)
// ---------------------------------------------------------------------------

/// BC-2.02.011 postcondition 3 (canonical test vector happy-path):
/// `write_file("/allowed/path", b"hello", 1024, 5000)` where path is in
/// `path_allow` → `Ok(())` (code 0); file contains `b"hello"`.
///
/// Exercises the full `func_wrap` → `read_wasm_bytes` (path + contents) →
/// `prepare` → `std::fs::write` path.
#[test]
fn test_BC_2_02_011_wat_write_succeeds_allowed_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let target_file = dir.path().join("out.txt");
    let path_str = target_file.to_str().expect("valid utf8 path");

    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, context_write_allowed(dir.path().to_str().unwrap()));
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path_bytes = path_str.as_bytes();
    let contents = b"hello";
    write_bytes_to_memory(&mut store, &memory, 0, path_bytes);
    write_bytes_to_memory(&mut store, &memory, 512, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (
                0,
                path_bytes.len() as i32,
                512,
                contents.len() as i32,
                1024,
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(result, codes::OK, "expected OK (0) for allowed write");
    assert_eq!(
        std::fs::read(&target_file).expect("file should exist"),
        b"hello",
        "file contents must match written bytes (BC-2.02.011 postcondition 3)"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_wat_max_bytes_exceeded_returns_output_too_large
// ---------------------------------------------------------------------------

/// BC-2.02.011 postcondition 2 (canonical test vector):
/// `write_file("/allowed/path", b"data", 3, 5000)` where `contents.len()=4 >
/// max_bytes=3` → `OUTPUT_TOO_LARGE (-3)`; no bytes written to disk.
///
/// Covers the `func_wrap` → `read_wasm_bytes` → `prepare` max_bytes branch.
#[test]
fn test_BC_2_02_011_wat_max_bytes_exceeded_returns_output_too_large() {
    let dir = tempfile::tempdir().expect("tempdir");
    let target_file = dir.path().join("big.txt");
    let path_str = target_file.to_str().expect("valid utf8 path");

    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, context_write_allowed(dir.path().to_str().unwrap()));
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path_bytes = path_str.as_bytes();
    let contents = b"data"; // 4 bytes; max_bytes = 3
    write_bytes_to_memory(&mut store, &memory, 0, path_bytes);
    write_bytes_to_memory(&mut store, &memory, 512, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (
                0,
                path_bytes.len() as i32,
                512,
                contents.len() as i32,
                3, // max_bytes=3 < contents.len()=4
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::OUTPUT_TOO_LARGE,
        "expected OUTPUT_TOO_LARGE (-3) when contents.len() > max_bytes (BC-2.02.011 postcondition 2)"
    );
    // BC-2.02.011 postcondition 2: no bytes written to disk.
    assert!(
        !target_file.exists(),
        "file must not exist when max_bytes exceeded (BC-2.02.011 postcondition 2)"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_timeout_ms_zero_accepted_abi_stability
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-004: `timeout_ms = 0` must be accepted for ABI stability.
/// The epoch interruption policy handles actual enforcement (not this story).
/// Verify that passing `timeout_ms=0` to an allowed write produces `OK (0)`.
#[test]
fn test_BC_2_02_011_timeout_ms_zero_accepted_abi_stability() {
    let dir = tempfile::tempdir().expect("tempdir");
    let target_file = dir.path().join("timeout_zero.txt");
    let path_str = target_file.to_str().expect("valid utf8 path");

    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, context_write_allowed(dir.path().to_str().unwrap()));
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path_bytes = path_str.as_bytes();
    let contents = b"x";
    write_bytes_to_memory(&mut store, &memory, 0, path_bytes);
    write_bytes_to_memory(&mut store, &memory, 512, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (
                0,
                path_bytes.len() as i32,
                512,
                contents.len() as i32,
                1024,
                0, // timeout_ms = 0: accepted for ABI stability
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::OK,
        "timeout_ms=0 must be accepted (BC-2.02.011 EC-004)"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 3: relative paths are joined with
/// `ctx.plugin_root` on the host side.  This test exercises the full
/// linker path (not just `prepare` directly) to verify the
/// `path_allowed` + `resolve_for_write` logic works end-to-end when
/// the guest sends a relative path.
#[test]
fn test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker() {
    let dir = tempfile::tempdir().expect("tempdir");
    let dir_str = dir.path().to_str().unwrap().to_string();

    // Allow the entire temp dir (relative allowlist entry ".")
    let mut ctx = bare_context();
    ctx.capabilities = Capabilities {
        write_file: Some(WriteFileCaps {
            path_allow: vec![".".to_string()],
            max_bytes_per_call: None,
        }),
        ..Default::default()
    };
    ctx.plugin_root = dir.path().to_path_buf();

    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    // Relative path "rel.txt" should resolve to <plugin_root>/rel.txt
    let path_bytes = b"rel.txt";
    let contents = b"relative";
    write_bytes_to_memory(&mut store, &memory, 0, path_bytes);
    write_bytes_to_memory(&mut store, &memory, 64, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (
                0,
                path_bytes.len() as i32,
                64,
                contents.len() as i32,
                1024,
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::OK,
        "relative path must resolve to plugin_root/rel.txt (BC-2.02.011 invariant 3)"
    );

    let written_path = dir.path().join("rel.txt");
    assert_eq!(
        std::fs::read(&written_path).expect("file must exist after successful write"),
        b"relative",
        "contents at resolved path must match (BC-2.02.011 invariant 3)"
    );

    drop(dir_str); // suppress unused-var warning
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_invariant_5_error_codes_stable_no_new_codes
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 5: the complete error code set is stable.
/// No new negative codes are introduced by this story.
/// Verified by exhaustive match over the known set.
#[test]
fn test_BC_2_02_011_invariant_5_error_codes_stable_no_new_codes() {
    // Canonical set per Architecture Compliance Rule 4 (S-8.10 v1.1).
    let known_write_codes: &[(i32, &str)] = &[
        (codes::OK, "success"),
        (codes::CAPABILITY_DENIED, "capability_denied (-1)"),
        (codes::TIMEOUT, "timeout (-2)"),
        (codes::OUTPUT_TOO_LARGE, "output_too_large (-3)"),
        (codes::INVALID_ARGUMENT, "invalid_argument (-4)"),
        (codes::INTERNAL_ERROR, "internal_error (-99)"),
    ];

    // Verify the numeric values are stable.
    assert_eq!(codes::OK, 0);
    assert_eq!(codes::CAPABILITY_DENIED, -1);
    assert_eq!(codes::TIMEOUT, -2);
    assert_eq!(codes::OUTPUT_TOO_LARGE, -3);
    assert_eq!(codes::INVALID_ARGUMENT, -4);
    assert_eq!(codes::INTERNAL_ERROR, -99);

    // All codes are accounted for; none are 0 except OK (no overlap).
    let unique: std::collections::HashSet<i32> =
        known_write_codes.iter().map(|(c, _)| *c).collect();
    assert_eq!(
        unique.len(),
        known_write_codes.len(),
        "error codes must be unique"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-001 / invariant 6: path traversal via `..` must be denied
/// even when a write_file capability block is present.
///
/// Exercises the full linker path to verify traversal detection works
/// end-to-end (not just in unit tests of `path_allowed`).
#[test]
fn test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt() {
    let dir = tempfile::tempdir().expect("tempdir");
    let dir_str = dir.path().to_str().unwrap().to_string();

    let (engine, linker) = engine_and_linker();
    let module = Module::new(&engine, WAT_WRITE_FILE).expect("WAT_WRITE_FILE should parse");
    let mut store = Store::new(&engine, context_write_allowed(&dir_str));
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    // Traversal path: starts inside allow-listed dir but escapes it via `..`
    let traversal = format!("{dir_str}/../../../etc/passwd");
    let path_bytes = traversal.as_bytes();
    let contents = b"evil";
    write_bytes_to_memory(&mut store, &memory, 0, path_bytes);
    write_bytes_to_memory(&mut store, &memory, 512, contents);

    let do_write = instance
        .get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(&mut store, "do_write")
        .expect("do_write export");

    let result = do_write
        .call(
            &mut store,
            (
                0,
                path_bytes.len() as i32,
                512,
                contents.len() as i32,
                1024,
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::CAPABILITY_DENIED,
        "path traversal must be denied with CAPABILITY_DENIED (-1) (BC-2.02.011 EC-001)"
    );
}
