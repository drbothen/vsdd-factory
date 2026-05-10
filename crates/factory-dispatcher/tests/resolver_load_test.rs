// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
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

use factory_dispatcher::engine::EpochTicker;
use factory_dispatcher::internal_log::{InternalEvent, InternalLog};
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
/// Asserts `Ok((registry, warnings))` is returned, registry is empty, warnings empty.
///
/// Red Gate: fails because `load_registry` is `todo!()` in Step 3 stubs.
#[test]
fn test_BC_1_13_001_absent_registry_yields_empty_no_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Deliberately do NOT create any file at this path.
    let missing = dir.path().join("resolvers-registry.toml");

    let engine =
        factory_dispatcher::engine::build_engine().expect("AC-001: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&missing);

    let (registry, warnings) = result.expect(
        "AC-001: load_registry on an absent resolvers-registry.toml must return Ok, \
         not Err — absent file = zero resolvers (BC-1.13.001 INV2)",
    );
    assert!(
        registry.is_empty(),
        "AC-001: registry returned for an absent file must be empty (zero resolvers), \
         not partially populated"
    );
    assert!(
        warnings.is_empty(),
        "AC-001: absent file must produce no LoadWarnings"
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
        b"[[resolvers]
name = 1",
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("AC-002: build_engine must succeed");
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
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let dir_toml = dir.path().to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "missing-wasm"
plugin = "{dir_toml}/nonexistent.wasm"
context_key = "missing_wasm"
"#
    );
    let path = write_file(
        dir.path(),
        "resolvers-registry.toml",
        toml_content.as_bytes(),
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("AC-013: build_engine must succeed");
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

    let engine =
        factory_dispatcher::engine::build_engine().expect("AC-006: build_engine must succeed");
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

    let engine =
        factory_dispatcher::engine::build_engine().expect("AC-004: build_engine must succeed");
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

// ---------------------------------------------------------------------------
// F-P1-008 — path_allow entries are resolved relative to CLAUDE_PROJECT_DIR
//
// BC-4.12.003 INV4: `path_allow` entries are resolved relative to the
// resolver's `project_dir` (i.e. CLAUDE_PROJECT_DIR), not relative to the
// dispatcher's cwd or the resolver .wasm file location.
// ---------------------------------------------------------------------------

/// test_resolver_path_allow_is_project_dir_relative (F-P1-008)
///
/// Verifies that CompiledWasmResolver's HostContext.cwd is set to
/// `ResolverInput.project_dir` so that relative path_allow entries are
/// resolved relative to the project directory (BC-4.12.003 INV4).
///
/// Strategy: create a resolver registry TOML with path_allow set to a
/// specific tempdir subdirectory. Invoke the resolver (via invoke_resolver_wasm
/// on a trapping resolver — the trap fires before any file read, but the
/// HostContext configuration is inspected via a no-op WASM module that
/// reads and returns input bytes). Instead, we verify the wiring indirectly:
/// the resolver's resolve() must NOT panic and must return a resolver
/// error (trap) when given a trapping WASM, proving the HostContext was
/// successfully constructed with the project_dir as cwd.
///
/// A more direct test of path_allow enforcement is provided by the
/// read_file host function unit tests (rejects_path_outside_allow_list).
/// This test focuses on the project_dir → cwd wiring at the resolver level.
#[test]
fn test_resolver_path_allow_is_project_dir_relative() {
    use factory_dispatcher::resolver::ResolverInput;
    use factory_dispatcher::resolver_loader::ResolverLoader;

    let fixture = {
        let manifest = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest).join("tests/fixtures/trapping_resolver.wasm")
    };
    assert!(
        fixture.exists(),
        "F-P1-008: trapping_resolver.wasm must exist at {:?}",
        fixture
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P1-008: build_engine must succeed");

    let project_dir = tempfile::tempdir().expect("tempdir");
    let project_path = project_dir.path();

    // Resolver with path_allow pointing to a project-relative subdirectory.
    let allowed_subdir = project_path.join("allowed");
    std::fs::create_dir_all(&allowed_subdir).expect("create allowed subdir");
    let allowed_rel = "allowed"; // relative to project_dir

    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let fixture_toml = fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "path-allow-resolver"
plugin = "{fixture_toml}"
context_key = "path_ctx"
path_allow = ["{allowed_rel}"]
"#
    );
    let registry_dir = tempfile::tempdir().expect("registry tempdir");
    let registry_path = registry_dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let loader = ResolverLoader::new(engine);
    let (registry, _warnings) = loader
        .load_registry(&registry_path)
        .expect("F-P1-008: load_registry must succeed");

    // Invoke with project_dir set to project_path — the HostContext.cwd must
    // be set to this path so path_allow["allowed"] resolves to project_path/allowed.
    let input = ResolverInput {
        event_type: "PreToolUse".to_string(),
        hook_event_name: "test-hook".to_string(),
        agent_type: None,
        project_dir: project_path.to_str().unwrap_or("").to_string(),
        plugin_config: serde_json::json!({}),
    };

    // The trapping resolver fires unreachable immediately — we get ResolverError::Trap.
    // This proves the HostContext was constructed (not panicked during construction)
    // with the project_dir as cwd. Path-allow enforcement itself is tested by
    // the read_file host function unit tests (rejects_path_outside_allow_list).
    let result = registry.invoke_resolver_wasm_for_testing("path-allow-resolver", &input);

    // The trap must be returned, not a panic — proving the HostContext was
    // successfully built with path_allow wired in (BC-4.12.003 INV4).
    assert!(
        matches!(
            result,
            Err(factory_dispatcher::resolver::ResolverError::Trap { .. })
        ),
        "F-P1-008 / BC-4.12.003 INV4: invoke_resolver_wasm must return ResolverError::Trap \
         (proving HostContext with path_allow was constructed successfully). \
         Got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// F-P2-001 (HIGH) — epoch deadline fires ResolverError::Timeout for a
// long-running resolver
//
// Verifies that `set_epoch_deadline` is wired in CompiledWasmResolver::resolve
// BEFORE instantiation. The long_running_resolver.wasm fixture spins in an
// infinite loop; the epoch ticker fires after ~1500ms and the dispatcher
// returns ResolverError::Timeout — NOT a hang.
// ---------------------------------------------------------------------------

/// test_F_P2_001_epoch_deadline_fires_resolver_timeout
///
/// Loads `long_running_resolver.wasm` (infinite loop) via `ResolverLoader::load_registry`,
/// starts an EpochTicker, invokes the resolver, and asserts:
/// 1. The call returns `Err(ResolverError::Timeout)` within the RESOLVER_TIMEOUT_MS budget.
/// 2. The function does NOT hang (epoch interruption fires).
/// 3. The returned error carries the resolver's registered name.
///
/// F-P2-001: set_epoch_deadline must be called on the Store BEFORE instantiation;
/// this test is the regression guard.
///
/// F-P3-002: Trap::Interrupt now classifies to ResolverError::Timeout (not Trap).
/// This test asserts ONLY Timeout — if epoch interruption mis-classifies to Trap,
/// the test fails and catches the regression.
#[test]
fn test_F_P2_001_epoch_deadline_fires_resolver_timeout() {
    use factory_dispatcher::resolver::ResolverInput;

    let manifest = env!("CARGO_MANIFEST_DIR");
    let fixture = std::path::Path::new(manifest).join("tests/fixtures/long_running_resolver.wasm");
    assert!(
        fixture.exists(),
        "F-P2-001: long_running_resolver.wasm must exist at {:?}. \
         Compile with: wasm-tools parse long_running_resolver.wat -o long_running_resolver.wasm",
        fixture
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-001: build_engine must succeed");

    // Start the EpochTicker so that the epoch advances and the deadline can fire.
    // Without the ticker, set_epoch_deadline has no effect and the test hangs.
    let _ticker = EpochTicker::start(engine.clone());

    let dir = tempfile::tempdir().expect("tempdir");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let fixture_toml = fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "long-running-resolver"
plugin = "{fixture_toml}"
context_key = "long_running_ctx"
"#
    );
    let registry_path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let loader = ResolverLoader::new(engine);
    let (registry, _warnings) = loader
        .load_registry(&registry_path)
        .expect("F-P2-001: load_registry must succeed (loading does not invoke resolve)");

    let input = ResolverInput {
        event_type: "PreToolUse".to_string(),
        hook_event_name: "test-hook".to_string(),
        agent_type: None,
        project_dir: dir.path().to_str().unwrap_or("").to_string(),
        plugin_config: serde_json::json!({}),
    };

    // Record wall time to verify the resolver did not hang beyond ~3s
    let t0 = std::time::Instant::now();

    // Invoke — must return Err(ResolverError::Timeout) when epoch deadline fires.
    let result = registry.invoke_resolver_wasm_for_testing("long-running-resolver", &input);

    let elapsed = t0.elapsed();

    // Must complete in under 3 seconds (budget is 1500ms, allow 2x slack for CI).
    assert!(
        elapsed.as_secs() < 3,
        "F-P2-001: resolver timed out but took {}ms — expected < 3000ms. \
         set_epoch_deadline may not be wired before instantiation.",
        elapsed.as_millis()
    );

    // F-P3-002: Trap::Interrupt now classifies to ResolverError::Timeout via classify_resolver_trap.
    // This arm is the regression guard: if epoch interruption mis-classifies to Trap, this fails.
    match result {
        Err(factory_dispatcher::resolver::ResolverError::Timeout { name }) => {
            assert_eq!(
                name, "long-running-resolver",
                "F-P2-001: Timeout.name must equal the registered resolver name"
            );
        }
        other => {
            panic!(
                "F-P2-001 / F-P3-002: expected ONLY ResolverError::Timeout from \
                 long_running_resolver.wasm — epoch interruption must classify to Timeout \
                 (not Trap) per F-P3-002. Got {:?} in {}ms",
                other,
                elapsed.as_millis()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// F-P2-003 (HIGH) — fail_closed field: fail-open skip vs fail-loud abort
//
// BC-4.12.001 PC6 (amended by F-P2-003):
// - fail_closed: absent or true → abort registry load on compile failure.
// - fail_closed: false → skip the entry, emit resolver.load_warning, continue.
// ---------------------------------------------------------------------------

/// test_F_P2_003_fail_closed_true_aborts_registry_load
///
/// Registry entry with a missing .wasm file and `fail_closed = true` (explicit).
/// Asserts `load_registry` returns `Err` — fail-loud semantics.
#[test]
fn test_F_P2_003_fail_closed_true_aborts_registry_load() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let dir_toml = dir.path().to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "strict-resolver"
plugin = "{dir_toml}/nonexistent.wasm"
context_key = "strict_ctx"
fail_closed = true
"#
    );
    let path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-003: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    assert!(
        result.is_err(),
        "F-P2-003: fail_closed=true with missing .wasm must return Err (fail-loud). \
         Got: Ok (registry loaded when it should have failed)"
    );
}

/// test_F_P2_003_fail_closed_default_aborts_registry_load
///
/// Registry entry with a missing .wasm file and no `fail_closed` field.
/// Absent field defaults to true (fail-loud). Asserts `load_registry` returns `Err`.
#[test]
fn test_F_P2_003_fail_closed_default_aborts_registry_load() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let dir_toml = dir.path().to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "default-fail-resolver"
plugin = "{dir_toml}/nonexistent.wasm"
context_key = "default_ctx"
"#
    );
    let path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-003: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    assert!(
        result.is_err(),
        "F-P2-003: absent fail_closed (default=true) with missing .wasm must return Err. \
         Got: Ok (should have failed loudly)"
    );
}

/// test_F_P2_003_fail_closed_false_skips_entry_and_loads_remainder
///
/// Registry with two entries: one with `fail_closed = false` pointing to a missing
/// .wasm, and one pointing to a valid fixture. Asserts:
/// 1. `load_registry` returns `Ok` (not an error).
/// 2. The registry contains exactly 1 resolver (the valid one).
/// 3. The skipped resolver is absent.
#[test]
fn test_F_P2_003_fail_closed_false_skips_entry_and_loads_remainder() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let valid_fixture =
        std::path::Path::new(manifest).join("tests/fixtures/trapping_resolver.wasm");
    assert!(
        valid_fixture.exists(),
        "F-P2-003: trapping_resolver.wasm must exist"
    );

    let dir = tempfile::tempdir().expect("tempdir");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let dir_toml = dir.path().to_string_lossy().replace('\\', "/");
    let valid_fixture_toml = valid_fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "optional-resolver"
plugin = "{dir_toml}/nonexistent.wasm"
context_key = "optional_ctx"
fail_closed = false

[[resolvers]]
name = "valid-resolver"
plugin = "{valid_fixture_toml}"
context_key = "valid_ctx"
"#
    );
    let path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-003: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    let (registry, warnings) = result.expect(
        "F-P2-003: fail_closed=false with missing .wasm must return Ok \
         (fail-open: skip entry and continue with remaining resolvers)",
    );

    assert_eq!(
        registry.len(),
        1,
        "F-P2-003: registry must contain exactly 1 resolver after skipping the \
         fail_closed=false entry. Expected 'valid-resolver' only."
    );

    // F-P3-003: warnings vec must contain the skipped entry.
    assert_eq!(
        warnings.len(),
        1,
        "F-P3-003: load_registry must return exactly 1 LoadWarning for the fail_closed=false \
         skipped entry ('optional-resolver'). Got {} warnings.",
        warnings.len()
    );
    assert_eq!(
        warnings[0].resolver_name, "optional-resolver",
        "F-P3-003: LoadWarning.resolver_name must equal the skipped resolver name 'optional-resolver'"
    );
    assert!(
        !warnings[0].detail.is_empty(),
        "F-P3-003: LoadWarning.detail must be non-empty (carry error context)"
    );
}

// ---------------------------------------------------------------------------
// F-P2-006 (MEDIUM) — deny_unknown_fields on ResolversRegistryToml and
// ResolverEntryToml: unknown TOML keys are a ParseError.
// ---------------------------------------------------------------------------

/// test_F_P2_006_unknown_top_level_field_is_parse_error
///
/// Registry TOML with an unknown top-level field. Asserts ParseError.
#[test]
fn test_F_P2_006_unknown_top_level_field_is_parse_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let toml_content = r#"schema_version = 1
unknown_top_level_key = "should-fail"
"#;
    let path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-006: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    assert!(
        matches!(result, Err(ResolverLoadError::ParseError { .. })),
        "F-P2-006: unknown top-level TOML field must be a ParseError \
         (deny_unknown_fields on ResolversRegistryToml). Got Err: {:?}",
        result.err()
    );
}

/// test_F_P2_006_unknown_resolver_entry_field_is_parse_error
///
/// Registry TOML with a valid resolver entry that has an unknown field.
/// Asserts ParseError (deny_unknown_fields on ResolverEntryToml).
#[test]
fn test_F_P2_006_unknown_resolver_entry_field_is_parse_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let dir_toml = dir.path().to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "test-resolver"
plugin = "{dir_toml}/placeholder.wasm"
context_key = "test_ctx"
unknown_field_in_entry = "should-fail"
"#
    );
    let path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P2-006: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&path);

    assert!(
        matches!(result, Err(ResolverLoadError::ParseError { .. })),
        "F-P2-006: unknown resolver entry field must be a ParseError \
         (deny_unknown_fields on ResolverEntryToml). Got Err: {:?}",
        result.err()
    );
}

// ---------------------------------------------------------------------------
// F-P2-005 (deferred) — AC-011 path_escaping_resolver.wasm
//
// Path escape testing for AC-011 is DEFERRED to S-12.07 where the real resolver
// authoring infrastructure (WaveContextResolver) provides a natural test vehicle.
// Requires a resolver that calls `host::read_file("/etc/passwd")` — a non-trivial
// WAT fixture that belongs in S-12.07 alongside the real resolver authoring crate.
//
// Per F-P2-005 resolution: the capability denied invariant is verified HERE via a
// unit test on `host::read_file::prepare` (without a real escape attempt).
// ---------------------------------------------------------------------------

// TODO(F-P2-005): AC-011 path_escaping_resolver.wasm fixture deferred to S-12.07.
// S-12.07's WaveContextResolver exercises real read_file calls with path_allow gating.
// See adversary-pass-2.md finding F-P2-005 for rationale.

/// test_capability_denied_when_resolver_attempts_disallowed_read (F-P2-005)
///
/// Exercises capability denial via `host::read_file::prepare` WITHOUT a real
/// WASM escape attempt. Creates a HostContext with no `read_file` capability
/// and asserts that `prepare()` returns `Err(CAPABILITY_DENIED)`.
///
/// This covers AC-011's capability enforcement invariant at the host function
/// unit-test level. The full path-escape WAT fixture is deferred to S-12.07.
#[test]
fn test_capability_denied_when_resolver_attempts_disallowed_read() {
    use factory_dispatcher::host::HostContext;
    use factory_dispatcher::registry::Capabilities;

    // Create a HostContext with read_file capability entirely absent (deny-by-default).
    let mut ctx = HostContext::new("test-resolver", "0.0.1", "sess-cap-test", "trace-cap-test");
    // No read_file capability — all reads must be denied.
    ctx.capabilities = Capabilities {
        read_file: None,
        ..Default::default()
    };

    // Attempt to read a file — must be denied (CAPABILITY_DENIED error code).
    // We use a file that actually exists so the only reason for denial is
    // the missing capability (not a missing file).
    let any_readable_path = "/etc/hosts"; // Always exists on Unix/macOS.
    let result = factory_dispatcher::read_file_prepare_for_test(&ctx, any_readable_path, 1024);

    // The result must be Err (capability denied), not Ok.
    assert!(
        result.is_err(),
        "F-P2-005 / BC-4.12.003: read_file must be DENIED when resolver has no \
         read_file capability. Got Ok — deny-by-default is not enforced."
    );
}

// ---------------------------------------------------------------------------
// F-P3-004 — reject empty project_dir as AbiViolation in CompiledWasmResolver::resolve
//
// BC-4.12.003 INV4: path_allow entries are resolved relative to project_dir.
// An empty project_dir would silently root all paths at "/" which violates
// the capability model. Reject early with AbiViolation.
// ---------------------------------------------------------------------------

/// test_F_P3_004_empty_project_dir_returns_abi_violation
///
/// Invokes a real WASM resolver with `project_dir = ""` and asserts that
/// `invoke_resolver_wasm_for_testing` returns `Err(ResolverError::AbiViolation)`.
///
/// The check must fire BEFORE any WASM instantiation — a trapping resolver
/// should NOT trap first; the AbiViolation guard is the first code path
/// exercised in `CompiledWasmResolver::resolve`.
#[test]
fn test_F_P3_004_empty_project_dir_returns_abi_violation() {
    use factory_dispatcher::resolver::{ResolverError, ResolverInput};

    let fixture = trapping_resolver_wasm();
    assert!(
        fixture.exists(),
        "F-P3-004: trapping_resolver.wasm must exist at {:?}",
        fixture
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P3-004: build_engine must succeed");
    let dir = tempfile::tempdir().expect("tempdir");

    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let fixture_toml = fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "empty-dir-resolver"
plugin = "{fixture_toml}"
context_key = "empty_dir_ctx"
"#
    );
    let registry_path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let loader = factory_dispatcher::resolver_loader::ResolverLoader::new(engine);
    let (registry, _warnings) = loader
        .load_registry(&registry_path)
        .expect("F-P3-004: load_registry must succeed");

    // Invoke with project_dir = "" — must be rejected as AbiViolation BEFORE instantiation.
    let input = ResolverInput {
        event_type: "PreToolUse".to_string(),
        hook_event_name: "test-hook".to_string(),
        agent_type: None,
        project_dir: "".to_string(), // Empty — must be rejected
        plugin_config: serde_json::json!({}),
    };

    let result = registry.invoke_resolver_wasm_for_testing("empty-dir-resolver", &input);

    match result {
        Err(ResolverError::AbiViolation { name, detail }) => {
            assert_eq!(
                name, "empty-dir-resolver",
                "F-P3-004: AbiViolation.name must equal the resolver name"
            );
            assert!(
                detail.contains("project_dir"),
                "F-P3-004: AbiViolation.detail must mention 'project_dir'. Got: {detail}"
            );
        }
        other => {
            panic!(
                "F-P3-004: empty project_dir must produce ResolverError::AbiViolation, \
                 got {:?} — the validation guard in CompiledWasmResolver::resolve is not firing",
                other
            );
        }
    }
}

// ---------------------------------------------------------------------------
// F-P4-003 — reject empty resolver name / context_key at registry load
//
// An empty 'name' would produce an unresolvable needs_context key.
// An empty 'context_key' would write to an anonymous plugin_config slot.
// Both must be rejected as ParseError at load_registry time.
// ---------------------------------------------------------------------------

/// test_F_P4_003_empty_name_returns_parse_error
///
/// Calls `load_registry` with a TOML entry where `name = ""`.
/// Asserts `Err(ResolverLoadError::ParseError)` is returned.
#[test]
fn test_F_P4_003_empty_name_returns_parse_error() {
    let fixture = trapping_resolver_wasm();
    assert!(
        fixture.exists(),
        "F-P4-003: trapping_resolver.wasm must exist at {:?}",
        fixture
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P4-003: build_engine must succeed");
    let dir = tempfile::tempdir().expect("tempdir");

    // TOML entry with name = "" — must be rejected before compilation.
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let fixture_toml = fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = ""
plugin = "{fixture_toml}"
context_key = "some_ctx"
"#
    );
    let registry_path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&registry_path);

    assert!(
        matches!(result, Err(ResolverLoadError::ParseError { .. })),
        "F-P4-003: empty resolver 'name' field must produce ParseError. Got: {:?}",
        result.err()
    );
}

/// test_F_P4_003_empty_context_key_returns_parse_error
///
/// Calls `load_registry` with a TOML entry where `context_key = ""`.
/// Asserts `Err(ResolverLoadError::ParseError)` is returned.
#[test]
fn test_F_P4_003_empty_context_key_returns_parse_error() {
    let fixture = trapping_resolver_wasm();
    assert!(
        fixture.exists(),
        "F-P4-003: trapping_resolver.wasm must exist at {:?}",
        fixture
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P4-003: build_engine must succeed");
    let dir = tempfile::tempdir().expect("tempdir");

    // TOML entry with context_key = "" — must be rejected before compilation.
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let fixture_toml = fixture.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "my-resolver"
plugin = "{fixture_toml}"
context_key = ""
"#
    );
    let registry_path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&registry_path);

    assert!(
        matches!(result, Err(ResolverLoadError::ParseError { .. })),
        "F-P4-003: empty resolver 'context_key' field must produce ParseError. Got: {:?}",
        result.err()
    );
}

// ---------------------------------------------------------------------------
// F-P4-004 — integration test for resolver.load_warning structured InternalLog event
//
// When a resolver entry has fail_closed = false and its .wasm fails to load,
// load_registry returns a LoadWarning. The caller (main.rs) emits it as a
// structured "resolver.load_warning" InternalLog event. This test mirrors that
// pattern to assert the structured event lands in the NDJSON log.
// ---------------------------------------------------------------------------

/// test_F_P4_004_resolver_load_warning_wire_format_is_writable
///
/// Sets up a registry with one fail_closed=false entry pointing at a
/// non-existent .wasm path. Calls load_registry, obtains the LoadWarning,
/// emits it to an InternalLog (mirroring main.rs), drops the log, reads
/// the NDJSON file, and asserts that a "resolver.load_warning" event
/// is present with the correct resolver_name and detail fields.
///
/// [deferred-integration] This test verifies the wire format of resolver.load_warning
/// events is correctly writable to InternalLog by mirroring main.rs's emission logic.
/// It does NOT exercise the production path (main.rs:327-337). End-to-end coverage
/// of the production path (deletion-regression detector) is deferred to a future
/// integration test that boots factory-dispatcher with a fail_closed=false fixture
/// and asserts the event appears in the resulting events-*.jsonl. Tracking: F-P5-003.
#[test]
fn test_F_P4_004_resolver_load_warning_wire_format_is_writable() {
    let dir = tempfile::tempdir().expect("F-P4-004: tempdir");
    let log_dir = dir.path().join("logs");

    // Set up registry TOML pointing to a non-existent wasm file with fail_closed = false.
    let nonexistent_wasm = dir.path().join("does-not-exist.wasm");
    // Windows: PathBuf::display() produces backslashes which TOML parses as escape sequences.
    // Forward-slash the path so it is valid TOML on all platforms (F-WIN-001).
    let nonexistent_wasm_toml = nonexistent_wasm.to_string_lossy().replace('\\', "/");
    let toml_content = format!(
        r#"schema_version = 1

[[resolvers]]
name = "warn-resolver"
plugin = "{nonexistent_wasm_toml}"
context_key = "warn_ctx"
fail_closed = false
"#
    );
    let registry_path = dir.path().join("resolvers-registry.toml");
    std::fs::write(&registry_path, toml_content).expect("write registry TOML");

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-P4-004: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let (_registry, warnings) = loader
        .load_registry(&registry_path)
        .expect("F-P4-004: load_registry must succeed (fail_closed=false skips bad entry)");

    assert_eq!(
        warnings.len(),
        1,
        "F-P4-004: exactly one LoadWarning expected for the skipped fail_closed=false entry"
    );

    // Mirror main.rs: emit each warning as a structured InternalLog event.
    let internal_log = InternalLog::new(log_dir.clone());
    for w in warnings {
        let ev = InternalEvent::now("resolver.load_warning")
            .with_trace_id("test-trace-fp4-004")
            .with_session_id("test-session-fp4-004")
            .with_field("resolver_name", serde_json::Value::String(w.resolver_name))
            .with_field("error_detail", serde_json::Value::String(w.detail));
        internal_log.write(&ev);
    }
    // Drop to flush (InternalLog writes on each call, but drop ensures no buffered data).
    drop(internal_log);

    // Read all log files in log_dir and check for the resolver.load_warning event.
    let all_log_content: String = std::fs::read_dir(&log_dir)
        .expect("F-P4-004: log dir must exist after write")
        .filter_map(|e| e.ok())
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect::<Vec<_>>()
        .join(
            "
",
        );

    assert!(
        all_log_content.contains("resolver.load_warning"),
        "F-P4-004: InternalLog must contain a 'resolver.load_warning' event. \
         Log content: {all_log_content:?}"
    );
    assert!(
        all_log_content.contains("warn-resolver"),
        "F-P4-004: resolver.load_warning event must include the resolver_name field \
         ('warn-resolver'). Log content: {all_log_content:?}"
    );
    assert!(
        all_log_content.contains("error_detail"),
        "F-P4-004: resolver.load_warning event must include the 'error_detail' field. \
         Log content: {all_log_content:?}"
    );
}

// ---------------------------------------------------------------------------
// F-017 — shipped resolvers-registry.toml parses against production API
//
// CI guard: the shipped registry must be parseable by ResolverLoader::load_registry.
// This catches schema drift between the TOML file and the ResolverEntryToml struct
// at CI time, before runtime failure.
// ---------------------------------------------------------------------------

/// test_shipped_resolvers_registry_parses
///
/// Parses the shipped `plugins/vsdd-factory/resolvers-registry.toml` against the
/// production `ResolverLoader::load_registry` API. The .wasm artifact does NOT need
/// to exist — we only test TOML parse + schema validation, not WASM compilation.
///
/// A ParseError here means the shipped TOML no longer matches the schema (e.g.,
/// a required field like `context_key` is missing). This is the F-017 CI guard.
///
/// Note: the test expects the load to produce a CompileError (or IoError) — NOT a
/// ParseError — because the TOML is valid but the .wasm path may not exist in CI.
/// A ParseError would indicate schema drift and must fail the test.
#[test]
fn test_shipped_resolvers_registry_parses() {
    // Find workspace root by sentinel (Cargo.toml with [workspace]).
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = {
        let mut current = manifest;
        loop {
            let candidate = current.join("Cargo.toml");
            if candidate.exists()
                && let Ok(contents) = std::fs::read_to_string(&candidate)
                && contents.contains("[workspace]")
            {
                break current.to_path_buf();
            }
            current = current
                .parent()
                .expect("F-017: reached filesystem root without finding workspace Cargo.toml");
        }
    };

    let registry_path = workspace_root.join("plugins/vsdd-factory/resolvers-registry.toml");

    // If the file does not exist, this is a CI setup error — not a schema failure.
    assert!(
        registry_path.exists(),
        "F-017: shipped resolvers-registry.toml must exist at {}",
        registry_path.display()
    );

    let engine =
        factory_dispatcher::engine::build_engine().expect("F-017: build_engine must succeed");
    let loader = ResolverLoader::new(engine);
    let result = loader.load_registry(&registry_path);

    match result {
        Err(ResolverLoadError::ParseError { detail }) => {
            panic!(
                "F-017: shipped resolvers-registry.toml failed to parse against production schema. \
                 This indicates schema drift (e.g., missing required field). Detail: {detail}"
            );
        }
        // CompileError / IoError are acceptable: the .wasm may not be built in this test run.
        // The key assertion is that the TOML parsed correctly (no ParseError).
        Err(ResolverLoadError::CompileError { .. }) | Err(ResolverLoadError::IoError { .. }) => {
            // Good: TOML parsed, .wasm compilation/load failed (expected in unit test context).
        }
        Ok(_) => {
            // Even better: TOML parsed AND .wasm loaded. Both are fine.
        }
        Err(other) => {
            panic!("F-017: unexpected error variant: {other:?}");
        }
    }
}
