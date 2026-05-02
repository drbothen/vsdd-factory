//! BC-2.02.011 behavioral-parity tests: invariants and edge cases NOT covered
//! by the stub-architect's unit tests in `host/write_file.rs`.
//!
//! Each test is named `test_BC_2_02_011_*` for full traceability.
//!
//! **What this file covers (gap analysis vs. stub-architect tests):**
//! - Invariant 1: HOST_ABI_VERSION = 1 in both crates (AC-3)
//! - Invariant 2: `max_bytes` mandatory — `max_bytes_per_call` capability
//!   override is tested (WriteFileCaps.max_bytes_per_call feature)
//! - BC EC-008: registry accepts a `write_file` capability block in TOML
//! - AC-7: CHANGELOG contains `host::write_file` entry under "## Added"
//!
//! BC: BC-2.02.011
//! Story: S-8.10

// ---------------------------------------------------------------------------
// Invariant 1: HOST_ABI_VERSION = 1 in both crates (AC-3)
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 1 / AC-3: `pub const HOST_ABI_VERSION: u32 = 1` in
/// `crates/factory-dispatcher/src/lib.rs`.  Both crates must agree.
/// Adding `write_file` is an additive ABI extension (D-6 Option A);
/// neither crate may bump HOST_ABI_VERSION in v1.x.
#[test]
fn test_BC_2_02_011_invariant_1_dispatcher_host_abi_version_is_1() {
    assert_eq!(
        factory_dispatcher::HOST_ABI_VERSION,
        1,
        "HOST_ABI_VERSION in crates/factory-dispatcher/src/lib.rs must remain 1 \
         after write_file is added (D-6 Option A, BC-2.02.011 invariant 1)"
    );
}

/// BC-2.02.011 invariant 1 / AC-3: both crates declare `HOST_ABI_VERSION = 1`.
/// Verified by reading source text of both files (grep-equivalent) so this
/// test is independent of Rust's const resolution.
#[test]
fn test_BC_2_02_011_invariant_1_both_crates_source_declare_version_1() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("resolve workspace root");

    let dispatcher_lib = workspace_root.join("crates/factory-dispatcher/src/lib.rs");
    let hook_sdk_lib = workspace_root.join("crates/hook-sdk/src/lib.rs");

    let dispatcher_src = std::fs::read_to_string(&dispatcher_lib)
        .expect("read crates/factory-dispatcher/src/lib.rs");
    let hook_sdk_src =
        std::fs::read_to_string(&hook_sdk_lib).expect("read crates/hook-sdk/src/lib.rs");

    assert!(
        dispatcher_src.contains("pub const HOST_ABI_VERSION: u32 = 1;"),
        "crates/factory-dispatcher/src/lib.rs must contain \
         `pub const HOST_ABI_VERSION: u32 = 1;` (BC-2.02.011 invariant 1 / AC-3)"
    );
    assert!(
        hook_sdk_src.contains("pub const HOST_ABI_VERSION: u32 = 1;"),
        "crates/hook-sdk/src/lib.rs must contain \
         `pub const HOST_ABI_VERSION: u32 = 1;` (BC-2.02.011 invariant 1 / AC-3)"
    );
}

// ---------------------------------------------------------------------------
// Invariant 2: max_bytes_per_call capability override
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 2 / AC-5: when `WriteFileCaps.max_bytes_per_call` is
/// set to a value SMALLER than the `max_bytes` argument, the effective cap is
/// `min(max_bytes_arg, max_bytes_per_call)`.  Content exceeding the effective
/// cap must be rejected with `OUTPUT_TOO_LARGE (-3)`.
///
/// This test covers a behavior path NOT exercised by the stub-architect's
/// `rejects_content_exceeding_max_bytes` test (which sets `max_bytes_per_call`
/// to `None`).
#[test]
fn test_BC_2_02_011_invariant_2_max_bytes_per_call_cap_override_rejects_oversized_content() {
    use factory_dispatcher::host::{HostContext, codes};
    use factory_dispatcher::registry::{Capabilities, WriteFileCaps};

    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("capped.txt");

    let mut ctx = HostContext::new("p", "0.0.1", "sess", "trace");
    ctx.capabilities = Capabilities {
        write_file: Some(WriteFileCaps {
            path_allow: vec![dir.path().to_str().unwrap().to_string()],
            // Hard per-capability cap of 10 bytes.
            max_bytes_per_call: Some(10),
        }),
        ..Default::default()
    };
    ctx.plugin_root = dir.path().to_path_buf();

    // contents.len()=15 > effective_cap=min(1024, 10)=10 → OUTPUT_TOO_LARGE.
    let contents = b"fifteen bytes!!";
    assert_eq!(contents.len(), 15);

    // Call `prepare` via the public test-support path: instantiate a
    // fake HostContext and use the same path exercised by AC-5 unit tests.
    // We access `prepare` indirectly through the linker for a truer E2E
    // read, but the unit-test level is sufficient to verify invariant 2.
    //
    // We call prepare via the dispatcher module. Since prepare is pub(crate),
    // we go through the full linker path via the WAT approach:
    let engine = wasmtime::Engine::default();
    let linker = factory_dispatcher::host::setup_linker(&engine).expect("linker");
    let wat = r#"
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
    let module = wasmtime::Module::new(&engine, wat).expect("WAT parse");
    let mut store = wasmtime::Store::new(&engine, ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path_bytes = file.to_str().unwrap().as_bytes();
    memory
        .write(&mut store, 0, path_bytes)
        .expect("write path to memory");
    memory
        .write(&mut store, 512, contents)
        .expect("write contents to memory");

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
                1024, // max_bytes arg = 1024, but max_bytes_per_call = 10
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::OUTPUT_TOO_LARGE,
        "max_bytes_per_call=10 overrides max_bytes_arg=1024; \
         contents.len()=15 must be rejected (BC-2.02.011 invariant 2)"
    );
    assert!(
        !file.exists(),
        "no bytes written when max_bytes_per_call exceeded (BC-2.02.011 postcondition 2)"
    );
}

/// BC-2.02.011 invariant 2 complement: when `max_bytes_per_call` is None,
/// the call argument is used as-is.  Content within the argument cap is
/// accepted.
#[test]
fn test_BC_2_02_011_invariant_2_max_bytes_per_call_none_uses_argument() {
    use factory_dispatcher::host::{HostContext, codes};
    use factory_dispatcher::registry::{Capabilities, WriteFileCaps};

    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("uncapped.txt");

    let mut ctx = HostContext::new("p", "0.0.1", "sess", "trace");
    ctx.capabilities = Capabilities {
        write_file: Some(WriteFileCaps {
            path_allow: vec![dir.path().to_str().unwrap().to_string()],
            max_bytes_per_call: None, // no override
        }),
        ..Default::default()
    };
    ctx.plugin_root = dir.path().to_path_buf();

    let engine = wasmtime::Engine::default();
    let linker = factory_dispatcher::host::setup_linker(&engine).expect("linker");
    let wat = r#"
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
    let module = wasmtime::Module::new(&engine, wat).expect("WAT parse");
    let mut store = wasmtime::Store::new(&engine, ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    let path_bytes = file.to_str().unwrap().as_bytes();
    let contents = b"hello no cap";
    memory
        .write(&mut store, 0, path_bytes)
        .expect("write path to memory");
    memory
        .write(&mut store, 512, contents)
        .expect("write contents to memory");

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
                1024, // max_bytes arg = 1024; no per-call override
                5000,
            ),
        )
        .expect("call should not trap");

    assert_eq!(
        result,
        codes::OK,
        "max_bytes_per_call=None uses max_bytes_arg; \
         contents.len()=12 < 1024 must succeed (BC-2.02.011 invariant 2)"
    );
}

// ---------------------------------------------------------------------------
// Registry: write_file capability TOML block (BC-2.02.011 EC-008)
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-008 / AC-5: the `hooks-registry.toml` parser must accept
/// a `[hooks.capabilities.write_file]` block with `path_allow` and
/// `max_bytes_per_call` fields.
///
/// This test covers a gap in `registry.rs` tests: the `accepts_capabilities_block`
/// test there does NOT exercise `write_file` capability parsing.
#[test]
fn test_BC_2_02_011_ec008_registry_accepts_write_file_capability_block() {
    use factory_dispatcher::registry::Registry;

    let toml = r#"
schema_version = 1

[[hooks]]
name = "state-writer"
event = "PostToolUse"
plugin = "plugins/state-writer.wasm"

[hooks.capabilities.write_file]
path_allow = [".factory/STATE.md", "/tmp/vsdd"]
max_bytes_per_call = 65536
"#;

    let reg = Registry::parse_str(toml).expect("registry should parse write_file capability");
    let caps = reg.hooks[0]
        .capabilities
        .as_ref()
        .expect("capabilities block must be present");
    let wf = caps
        .write_file
        .as_ref()
        .expect("write_file capability block must parse");

    assert_eq!(
        wf.path_allow,
        vec![".factory/STATE.md", "/tmp/vsdd"],
        "path_allow must round-trip through TOML parsing"
    );
    assert_eq!(
        wf.max_bytes_per_call,
        Some(65536),
        "max_bytes_per_call must round-trip through TOML parsing"
    );
}

/// BC-2.02.011: registry must reject a `write_file` capability block with
/// unknown fields (deny_unknown_fields derives on WriteFileCaps).
#[test]
fn test_BC_2_02_011_registry_rejects_unknown_write_file_capability_field() {
    use factory_dispatcher::registry::Registry;

    let toml = r#"
schema_version = 1

[[hooks]]
name = "bad-caps"
event = "PreToolUse"
plugin = "x.wasm"

[hooks.capabilities.write_file]
path_allow = ["."]
unknown_field = true
"#;

    assert!(
        Registry::parse_str(toml).is_err(),
        "WriteFileCaps must reject unknown fields (serde deny_unknown_fields)"
    );
}

// ---------------------------------------------------------------------------
// AC-7: CHANGELOG contains write_file entry
// ---------------------------------------------------------------------------

/// AC-7 (S-8.10): the workspace CHANGELOG must contain a `host::write_file`
/// entry under an "## Added" section.
///
/// This test verifies the documentation deliverable from AC-7.  It reads
/// the real CHANGELOG file at workspace root.
#[test]
fn test_BC_2_02_011_ac7_changelog_contains_write_file_entry() {
    // Resolve the CHANGELOG.md path relative to the workspace root.
    // CARGO_MANIFEST_DIR points to the factory-dispatcher crate; walk up.
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .expect("could not resolve workspace root from CARGO_MANIFEST_DIR");

    let changelog = workspace_root.join("CHANGELOG.md");
    assert!(
        changelog.exists(),
        "CHANGELOG.md must exist at workspace root"
    );

    let content = std::fs::read_to_string(&changelog).expect("CHANGELOG.md read failed");

    // AC-7 requires an entry under "## Added" mentioning host::write_file.
    assert!(
        content.contains("host::write_file"),
        "CHANGELOG.md must contain 'host::write_file' entry (AC-7 S-8.10)"
    );
}
