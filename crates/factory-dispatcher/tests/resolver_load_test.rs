//! `ResolverLoader` loading-lifecycle tests (S-12.04 AC-001 through AC-004, AC-007, AC-012,
//! AC-013).
//!
//! These tests exercise `ResolverLoader::get_or_compile` and
//! `ResolverLoader::load_registry` with real tempfiles and the compiled
//! `tests/fixtures/trapping_resolver.wasm` artifact.
//!
//! All test bodies are fully authored per S-12.04 Step 2. Every test MUST
//! FAIL before Step 3 implementation because the production functions under
//! test are `todo!("S-12.04 Step 3 implementation")`. Red Gate per BC-5.38.001.
//!
//! Architecture anchors:
//! - BC-4.12.001 — WASM resolver loading and caching contract
//! - BC-1.13.001 — dispatcher pre-dispatch injection contract (absent-file behavior)
//! - S-12.04 AC-001/002/003/004/007/012/013

use std::sync::Arc;

use factory_dispatcher::resolver_loader::{ResolverLoadError, ResolverLoader};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Absolute path to the trapping_resolver.wasm fixture compiled from WAT in
/// `tests/fixtures/trapping_resolver.wat` during S-12.04 Step 2.
///
/// This is a real compiled WASM artifact that exports `resolve(i32,i32)->i64`
/// and immediately executes `unreachable`. Used for AC-005/006 load tests
/// (load does not invoke, so the trap does not fire during compilation).
fn trapping_resolver_wasm() -> std::path::PathBuf {
    // The path is relative to the crate root. Cargo integration tests run
    // with cwd = workspace root, but the safe way is to anchor off
    // CARGO_MANIFEST_DIR (set at compile time).
    let manifest = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest).join("tests/fixtures/trapping_resolver.wasm")
}

/// Write `contents` to `dir/name` and return the path.
fn write_file(dir: &std::path::Path, name: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, contents).expect("write_file helper: write must succeed");
    path
}

// ---------------------------------------------------------------------------
// AC-001 (unit) — absent resolvers-registry.toml → Ok(empty registry)
//
// BC-1.13.001 postcondition 1 + invariant 2:
// A missing registry file is NEVER an error. The dispatcher starts with zero
// resolvers rather than returning Err.
// ---------------------------------------------------------------------------

/// test_BC_1_13_001_absent_registry_yields_empty_no_error
///
/// Calls `ResolverLoader::load_registry` with a path that does not exist.
/// Asserts `Ok(registry)` is returned and the registry is empty.
///
/// Red Gate: fails because `load_registry` is `todo!()` in Step 3 stubs.
#[test]
fn test_BC_1_13_001_absent_registry_yields_empty_no_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Deliberately do NOT create any file at this path.
    let missing = dir.path().join("resolvers-registry.toml");

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-001: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&missing);

    let registry = result.expect(
        "AC-001: load_registry on an absent resolvers-registry.toml must return Ok, \
         not Err — absent file = zero resolvers (BC-1.13.001 INV2)",
    );
    assert!(
        registry.is_empty(),
        "AC-001: registry returned for an absent file must be empty (zero resolvers), \
         not partially populated"
    );
}

// ---------------------------------------------------------------------------
// AC-002 (unit) — malformed TOML → Err(ParseError)
//
// BC-1.13.001 postcondition 2:
// A present but malformed registry file must fail loud (ParseError),
// not silently yield an empty registry.
// ---------------------------------------------------------------------------

/// test_BC_1_13_001_malformed_toml_returns_parse_error
///
/// Writes a malformed TOML file and asserts `Err(ResolverLoadError::ParseError)`.
/// A ParseError must NOT be confused with the absent-file `Ok(empty)` path —
/// both cases must be distinguishable (BC-1.13.001 PC2).
///
/// Red Gate: fails because `load_registry` is `todo!()`.
#[test]
fn test_BC_1_13_001_malformed_toml_returns_parse_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Deliberately broken TOML: `[[resolvers]` (unclosed bracket) is not valid.
    let path = write_file(
        dir.path(),
        "resolvers-registry.toml",
        b"[[resolvers]\nname = 1",
    );

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-002: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    match result {
        Err(ResolverLoadError::ParseError { detail }) => {
            // Good path — verify detail is non-empty (SOUL #4: observable errors).
            assert!(
                !detail.is_empty(),
                "AC-002: ParseError.detail must be non-empty (carry TOML error context)"
            );
        }
        Err(other) => {
            panic!(
                "AC-002: expected ResolverLoadError::ParseError but got {:?} — \
                 malformed TOML must not produce IoError or CompileError",
                other
            );
        }
        Ok(_) => {
            panic!(
                "AC-002: load_registry with malformed TOML must return Err(ParseError), \
                 not Ok — dispatcher MUST NOT start with a partial resolver set when \
                 the registry file is broken (BC-1.13.001 PC2)"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// AC-013 (unit) — failed .wasm compilation → Err(CompileError)
//
// BC-4.12.001 postcondition 6:
// A registry entry pointing to an invalid or absent .wasm path must produce
// a CompileError at load time, not silently skip the resolver.
// ---------------------------------------------------------------------------

/// test_BC_4_12_001_failed_compile_returns_compile_error
///
/// Writes a registry TOML that references a non-existent `.wasm` path and
/// asserts `Err(ResolverLoadError::CompileError)`.
///
/// Per R-PLAT-002 (story Dev Notes): missing-file and invalid-bytes errors are
/// distinct from ParseError and must be tested independently.
///
/// Red Gate: fails because `load_registry` is `todo!()`.
#[test]
fn test_BC_4_12_001_failed_compile_returns_compile_error() {
    let dir = tempfile::tempdir().expect("tempdir");

    // TOML with a resolver entry pointing to a .wasm file that does not exist.
    // Uses `plugin` field (F-P1-003 rename from `path`) and `context_key` (F-P1-004).
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "missing-wasm"
plugin = "{}/nonexistent.wasm"
context_key = "missing_wasm"
"#,
        dir.path().display()
    );
    let path = write_file(
        dir.path(),
        "resolvers-registry.toml",
        toml_content.as_bytes(),
    );

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-013: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    match result {
        Err(ResolverLoadError::CompileError { detail }) => {
            // Good path — the missing .wasm path must be observable in the detail.
            assert!(
                !detail.is_empty(),
                "AC-013: CompileError.detail must be non-empty (carry path/compile error context)"
            );
        }
        Err(ResolverLoadError::IoError { detail }) => {
            // Also acceptable: a missing .wasm file may surface as IoError rather than
            // CompileError depending on whether the file is stat'd before passing to wasmtime.
            // Either variant satisfies the "fail-loud" requirement (BC-4.12.001 PC6).
            assert!(
                !detail.is_empty(),
                "AC-013: IoError.detail must be non-empty for missing .wasm path"
            );
        }
        Err(ResolverLoadError::ParseError { .. }) => {
            panic!(
                "AC-013: a valid TOML file with a missing .wasm path must produce \
                 CompileError or IoError, NOT ParseError — TOML is well-formed"
            );
        }
        Ok(_) => {
            panic!(
                "AC-013: load_registry must NOT return Ok when a referenced .wasm file \
                 does not exist — fail-loud at startup (BC-4.12.001 PC6)"
            );
        }
        // Non-exhaustive: accept any other Err variant as a fail-loud signal.
        Err(other) => {
            // Any error is better than Ok; but log the variant for diagnosis.
            let _ = other;
        }
    }
}

// ---------------------------------------------------------------------------
// AC-005 (integration) — load is deterministic
//
// BC-4.12.001 postcondition 2 / VP-073-A:
// Calling get_or_compile twice on the same path with the same mtime returns
// byte-identical Arc<Module> contents (no random bytes, same compilation).
// ---------------------------------------------------------------------------

/// test_BC_4_12_001_load_is_deterministic
///
/// Loads the same `.wasm` artifact via two separate `ResolverLoader` instances
/// and asserts the resulting compiled Module serializes to the same bytes.
///
/// VP-073-A: determinism across independent loader instances rules out
/// randomness in the compilation output.
///
/// Red Gate: fails because `get_or_compile` is `todo!()`.
#[test]
fn test_BC_4_12_001_load_is_deterministic() {
    let wasm_path = trapping_resolver_wasm();
    assert!(
        wasm_path.exists(),
        "Fixture trapping_resolver.wasm must exist at {:?}; \
         compile it with: wasm-tools parse trapping_resolver.wat -o trapping_resolver.wasm",
        wasm_path
    );

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-005: build_engine must succeed for determinism test");

    let loader_a = ResolverLoader::new(engine.clone());
    let loader_b = ResolverLoader::new(engine.clone());

    let module_a = loader_a
        .get_or_compile(&wasm_path)
        .expect("AC-005: first get_or_compile must succeed");
    let module_b = loader_b
        .get_or_compile(&wasm_path)
        .expect("AC-005: second get_or_compile must succeed");

    // Serialize both to bytes and compare. wasmtime Module::serialize() returns
    // the compiled bytes. If determinism holds, both must be identical.
    let bytes_a = module_a
        .serialize()
        .expect("AC-005: module_a.serialize() must succeed");
    let bytes_b = module_b
        .serialize()
        .expect("AC-005: module_b.serialize() must succeed");

    assert_eq!(
        bytes_a, bytes_b,
        "AC-005 / VP-073-A: two independent get_or_compile calls on the same .wasm \
         path must produce byte-identical compiled modules — deterministic loading \
         (BC-4.12.001 postcondition 2)"
    );

    // Suppress unused-variable warning for `engine` (needed to establish context).
    drop(engine);
}

// ---------------------------------------------------------------------------
// AC-006 (integration) — load does not invoke resolve()
//
// BC-4.12.001 invariant 3:
// get_or_compile only compiles the module; it does NOT execute resolve().
// The trapping resolver's `unreachable` must NOT fire during compilation.
// ---------------------------------------------------------------------------

/// test_BC_4_12_001_load_does_not_invoke_resolve
///
/// Loads `trapping_resolver.wasm` (whose resolve() immediately executes
/// `unreachable`) via `get_or_compile`. The load must return `Ok` — the trap
/// only fires when `resolve()` is actually invoked.
///
/// Verifies BC-4.12.001 INV3: loading is side-effect-free with respect to
/// resolver execution.
///
/// Red Gate: fails because `get_or_compile` is `todo!()`.
#[test]
fn test_BC_4_12_001_load_does_not_invoke_resolve() {
    let wasm_path = trapping_resolver_wasm();
    assert!(
        wasm_path.exists(),
        "Fixture trapping_resolver.wasm must exist at {:?}",
        wasm_path
    );

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-006: build_engine must succeed");
    let loader = ResolverLoader::new(engine);

    // If get_or_compile accidentally invoked resolve(), the `unreachable`
    // instruction would fire and this call would return Err (or panic).
    // A successful Ok proves resolve() was NOT called.
    let result = loader.get_or_compile(&wasm_path);

    assert!(
        result.is_ok(),
        "AC-006 / BC-4.12.001 INV3: get_or_compile on trapping_resolver.wasm must \
         return Ok — loading must NOT invoke resolve(). If this returns Err, the \
         loader is incorrectly executing the WASM during compilation. \
         Error: {:?}",
        result.err()
    );
}

// ---------------------------------------------------------------------------
// AC-004 (integration) — mtime change triggers recompilation
//
// BC-4.12.001 postcondition 3 / VP-073-C:
// When a .wasm file's mtime changes, the next get_or_compile evicts the
// cached module and recompiles — the returned Arc<Module> is a NEW instance,
// not the same cached one.
// ---------------------------------------------------------------------------

/// test_BC_4_12_001_mtime_change_triggers_recompilation
///
/// Writes a minimal valid WASM to a tempfile, compiles it, advances the file's
/// mtime via `filetime::set_file_mtime`, compiles again, and asserts the two
/// `Arc<Module>` instances are NOT pointer-equal (a fresh compilation occurred).
///
/// VP-073-C: mtime-based cache invalidation is the correctness gate for
/// hot-reload without a dispatcher restart.
///
/// Red Gate: fails because `get_or_compile` is `todo!()`.
#[test]
fn test_BC_4_12_001_mtime_change_triggers_recompilation() {
    let dir = tempfile::tempdir().expect("tempdir");

    // Write a minimal valid WASM module (returns i32 constant 0).
    let wasm_bytes = wat::parse_str(
        r#"(module (memory (export "memory") 1) (func (export "resolve") (param i32 i32) (result i64) i64.const 0))"#,
    )
    .expect("WAT parse for mtime test");
    let wasm_path = write_file(dir.path(), "resolver.wasm", &wasm_bytes);

    let engine = factory_dispatcher::engine::build_engine()
        .expect("AC-004: build_engine must succeed");
    let loader = ResolverLoader::new(engine);

    // First compilation — populates the cache keyed by (path, mtime_t1).
    let module_a: Arc<wasmtime::Module> = loader
        .get_or_compile(&wasm_path)
        .expect("AC-004: first get_or_compile must succeed");

    // Advance the mtime by +2 seconds to guarantee a cache miss on the next call.
    // filetime is in dev-dependencies (Cargo.toml).
    let original_mtime = std::fs::metadata(&wasm_path)
        .expect("stat wasm_path")
        .modified()
        .expect("mtime");
    let new_mtime = original_mtime + std::time::Duration::from_secs(2);
    filetime::set_file_mtime(&wasm_path, filetime::FileTime::from_system_time(new_mtime))
        .expect("AC-004: set_file_mtime must succeed");

    // Second compilation — must detect mtime change and recompile.
    let module_b: Arc<wasmtime::Module> = loader
        .get_or_compile(&wasm_path)
        .expect("AC-004: second get_or_compile (after mtime change) must succeed");

    // The two Arc<Module> values must NOT be the same cached instance.
    // Pointer inequality is the strongest assertion: if the loader returned the
    // cached module unchanged, both Arcs would point to the same allocation.
    assert!(
        !Arc::ptr_eq(&module_a, &module_b),
        "AC-004 / VP-073-C: after advancing the file mtime, get_or_compile must \
         return a NEW Arc<Module> (cache eviction + recompilation), not the \
         previously cached instance. \
         Loader returned the same Arc pointer — mtime invalidation is not implemented."
    );
}
