//! `ResolverLoader` — mtime-cached WASM module loader for context resolvers.
//!
//! Loads and compiles WASM resolver modules from disk, caching compiled
//! `wasmtime::Module` instances keyed by path + last-modified time.
//! When a file's mtime changes the cached module is invalidated and
//! recompiled (BC-4.12.001 resolver loading contract).
//!
//! Also provides `load_registry` to parse `resolvers-registry.toml` and
//! populate a `ResolverRegistry` with in-memory resolver wrappers that
//! hold the compiled modules and perform real WASM invocation per dispatch.
//!
//! Architecture anchors:
//! - BC-4.12.001 — WASM resolver loading and caching contract
//! - BC-4.12.002 — resolver ABI (packed-i64 return, resolve() signature)
//! - BC-4.12.003 — resolver capability / path_allow enforcement
//! - BC-4.12.004 — resolver crash isolation (trap → ResolverError::Trap)
//! - BC-4.12.005 PC6 — duplicate context_key is a registry-load error
//! - BC-1.13.001 — dispatcher pre-dispatch injection contract (absent-file)
//! - ADR-018 — WASM-plugin Context Resolvers design
//! - S-12.04 — this story

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use serde::Deserialize;
use thiserror::Error;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::p1::{self, WasiP1Ctx};

use crate::engine::timeout_ms_to_epochs;
use crate::host::HostContext;
use crate::registry::Capabilities;
use crate::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, ResolverRegistry,
};
use crate::resolver_classify_trap::classify_resolver_trap;

// ---------------------------------------------------------------------------
// ResolverStoreData — per-invocation store data for WASM resolver modules
// ---------------------------------------------------------------------------

/// Per-invocation store data for resolver WASM instantiation.
///
/// Resolver modules compiled for `wasm32-wasip1` import WASI snapshot
/// preview 1 syscalls from Rust's standard library bootstrap
/// (`environ_get`, `fd_write`, etc.). Without a `WasiP1Ctx` in the store
/// and `p1::add_to_linker_sync` on the linker, instantiation fails with
/// "unknown import wasi_snapshot_preview1::environ_get".
///
/// The WASI context is deliberately restricted:
/// - No filesystem preopens (resolver must use vsdd::read_file for bounded access)
/// - Stdin: empty (resolvers read via host function, not WASI stdin)
/// - Stdout/stderr: sink (resolvers must use vsdd::log, not WASI print)
/// - Environment variables: empty (resolvers use vsdd::read_file, not env)
///
/// This mirrors the pattern in `invoke.rs::StoreData` but with maximum
/// restriction rather than project-directory preopen (BC-4.12.003 INV2).
struct ResolverStoreData {
    host: HostContext,
    wasi: WasiP1Ctx,
}

// ---------------------------------------------------------------------------
// Error type and warning type
// ---------------------------------------------------------------------------

/// A non-fatal warning emitted by `load_registry` when a resolver entry is
/// skipped due to `fail_closed = false` (F-P3-003).
///
/// Returned as part of the `(ResolverRegistry, Vec<LoadWarning>)` tuple so
/// the caller can emit structured telemetry events without coupling
/// `load_registry` to I/O or an InternalLog reference.
///
/// Dual-emission pattern (F-P3-003):
/// - The `eprintln!` in `load_registry` provides startup-visible operator feedback.
/// - The caller emits a structured `resolver.load_warning` InternalLog event
///   (queryable by the observability stack).
#[derive(Debug, Clone)]
pub struct LoadWarning {
    /// The registry name of the skipped resolver.
    pub resolver_name: String,
    /// Human-readable description of why the entry was skipped.
    pub detail: String,
}

/// Errors produced during resolver loading, parsing, or compilation.
///
/// Covers I/O failures, TOML parse errors, and wasmtime compilation errors.
/// `#[non_exhaustive]` allows future variants without breaking downstream
/// match arms (F-P2-006).
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ResolverLoadError {
    /// The `resolvers-registry.toml` file could not be parsed.
    #[error("resolver registry parse error: {detail}")]
    ParseError { detail: String },

    /// An I/O error occurred reading a WASM file or the registry TOML.
    #[error("resolver I/O error: {detail}")]
    IoError { detail: String },

    /// The wasmtime engine failed to compile a WASM module.
    #[error("resolver WASM compilation error: {detail}")]
    CompileError { detail: String },
}

// ---------------------------------------------------------------------------
// TOML deserialization structs
// ---------------------------------------------------------------------------

/// Top-level shape of `resolvers-registry.toml`.
///
/// `deny_unknown_fields` (F-P2-006): unknown TOML keys are a parse error, not
/// silently ignored. This prevents typos in field names from going undetected.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolversRegistryToml {
    schema_version: u32,
    #[serde(default)]
    resolvers: Vec<ResolverEntryToml>,
}

/// One `[[resolvers]]` entry in the registry TOML.
///
/// `plugin` (not `path`) matches BC-4.12.001 PC2, BC-1.13.001 EC-003, and
/// the sibling `RegistryEntry.plugin` convention in `registry.rs`.
/// `context_key` is the key under which the resolver's output is written to
/// `plugin_config` (BC-4.12.005 PC6 — uniqueness validated at load time).
///
/// `deny_unknown_fields` (F-P2-006): coordinate with `fail_closed` — both added
/// together so the new field is never treated as unknown.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolverEntryToml {
    /// Registry name — used as the `needs_context` key.
    name: String,
    /// Path to the compiled `.wasm` file.
    plugin: PathBuf,
    /// The `plugin_config` key under which this resolver's output is written.
    /// Validated unique across all entries at load_registry time
    /// (BC-4.12.005 PC6).
    context_key: String,
    /// Declared path-allow list for the `read_file` host function (BC-4.12.003).
    /// Entries are resolved relative to `CLAUDE_PROJECT_DIR` (BC-4.12.003 INV4).
    #[serde(default)]
    path_allow: Vec<String>,
    /// Controls fail behavior when the resolver's `.wasm` fails to load/compile.
    ///
    /// `None` (field absent in TOML) → treated as `true` (fail-loud, default).
    /// `Some(true)` → fail-loud: abort registry load with `Err(ParseError)`.
    /// `Some(false)` → fail-open: skip this entry, emit a `resolver.load_warning`
    ///   to eprintln, and continue loading other entries.
    ///
    /// Document in HOST_ABI.md §Resolver Registry Schema. (F-P2-003)
    #[serde(default)]
    fail_closed: Option<bool>,
}

// ---------------------------------------------------------------------------
// ResolverLoader
// ---------------------------------------------------------------------------

/// Mtime-cached WASM module loader.
///
/// Caches compiled `wasmtime::Module` instances in a `HashMap` keyed by
/// `PathBuf`. On each `get_or_compile` call the file's mtime is compared
/// to the cached entry; a mtime change triggers recompilation
/// (BC-4.12.001 loading contract).
///
/// The loader holds the executor's `Engine` so that compiled modules are
/// bound to the same engine instance used for instantiation.
/// Modules are NOT cross-engine portable (BC-4.12.001 INV3).
///
/// Interior-mutable via `Mutex` so the loader can be shared across
/// threads without `&mut` access. `Arc<ResolverLoader>` is the expected
/// usage pattern.
pub struct ResolverLoader {
    /// The wasmtime Engine used for module compilation AND per-dispatch instantiation.
    ///
    /// Shared with the executor (BC-4.12.001 INV3: resolver modules MUST be
    /// compiled with the same Engine instance used for hook plugins).
    engine: Engine,
    /// Compiled module cache: path → (mtime, compiled module).
    cache: Mutex<HashMap<PathBuf, (SystemTime, Arc<Module>)>>,
}

impl ResolverLoader {
    /// Construct an empty loader bound to the given `Engine`.
    ///
    /// `engine` must be the same `Engine` used by the executor for hook
    /// plugin instantiation (BC-4.12.001 INV3). Modules compiled with one
    /// Engine cannot be used with a different Engine instance.
    ///
    /// GREEN-BY-DESIGN: zero branching, no I/O, no non-trivial helpers.
    /// Satisfies BC-5.38.002.
    pub fn new(engine: Engine) -> Self {
        Self {
            engine,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Return a cached compiled module, or compile from disk if uncached
    /// or if the file's mtime has changed since the last load.
    ///
    /// Steps:
    /// 1. Canonicalize path → stable cache key
    /// 2. Read file mtime via `fs::metadata`
    /// 3. Check cache: if (path, mtime) hit → return Arc clone (no recompile)
    /// 4. Miss → compile via `Module::new(&self.engine, bytes)`; update cache
    ///
    /// BC-4.12.001 INV1: compilation is startup-time only (no Store creation
    /// here, just Module compilation). No `resolve()` invocation occurs.
    pub fn get_or_compile(&self, path: &Path) -> Result<Arc<Module>, ResolverLoadError> {
        // Canonicalize for a stable cache key (resolves symlinks, relative segments).
        let canonical = path
            .canonicalize()
            .map_err(|e| ResolverLoadError::IoError {
                detail: format!("cannot canonicalize {}: {e}", path.display()),
            })?;

        // Read current mtime for cache-invalidation check.
        let mtime = std::fs::metadata(&canonical)
            .map_err(|e| ResolverLoadError::IoError {
                detail: format!("cannot stat {}: {e}", canonical.display()),
            })?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH);

        // Cache hit: same path + same mtime → return the cached module.
        {
            let guard = self.cache.lock().expect("resolver module cache poisoned");
            if let Some((cached_mtime, module)) = guard.get(&canonical)
                && *cached_mtime == mtime
            {
                return Ok(Arc::clone(module));
            }
        }

        // Cache miss or mtime change: read bytes and compile.
        // Missing/unreadable file → IoError (F-P1-010: preserved discrimination).
        let bytes = std::fs::read(&canonical).map_err(|e| ResolverLoadError::IoError {
            detail: format!("cannot read {}: {e}", canonical.display()),
        })?;

        // Compile using the loader's shared engine (BC-4.12.001 INV3).
        let module =
            Module::new(&self.engine, &bytes).map_err(|e| ResolverLoadError::CompileError {
                detail: format!("wasmtime compile failed for {}: {e}", canonical.display()),
            })?;

        let arc = Arc::new(module);

        // Store in cache (insert replaces stale entry if mtime changed).
        let mut guard = self.cache.lock().expect("resolver module cache poisoned");
        guard.insert(canonical, (mtime, Arc::clone(&arc)));

        Ok(arc)
    }

    /// Parse `resolvers-registry.toml` at `path` and return a populated
    /// `ResolverRegistry` plus any non-fatal `LoadWarning` entries.
    ///
    /// Rules (BC-1.13.001 + BC-4.12.001):
    /// - Absent file → `Ok((ResolverRegistry::new(), vec![]))` — NOT an error (INV2).
    /// - Malformed TOML → `Err(ResolverLoadError::ParseError)` — fail-loud.
    /// - Unknown schema_version → `Err(ResolverLoadError::ParseError)`.
    /// - Missing `.wasm` file → `Err(IoError)` — distinct from CompileError (F-P1-010).
    /// - Compile failure → `Err(CompileError)` — fail-loud.
    /// - Duplicate `context_key` → `Err(ParseError)` per BC-4.12.005 PC6.
    /// - `fail_closed = false` entry that fails to load → skipped; a `LoadWarning`
    ///   is appended to the warnings vec AND an `eprintln!` is emitted for
    ///   startup-visible operator feedback (F-P3-003 dual-emission pattern).
    ///
    /// The returned registry uses `CompiledWasmResolver` wrappers that
    /// hold the compiled module, context_key, path_allow, and engine.
    /// Real WASM invocation occurs in `CompiledWasmResolver::resolve()`.
    pub fn load_registry(
        &self,
        path: &Path,
    ) -> Result<(ResolverRegistry, Vec<LoadWarning>), ResolverLoadError> {
        // BC-1.13.001 INV2: absent file → empty registry, NOT an error.
        if !path.exists() {
            return Ok((ResolverRegistry::new(), vec![]));
        }

        // Read and parse the TOML.
        let text = std::fs::read_to_string(path).map_err(|e| ResolverLoadError::IoError {
            detail: format!("cannot read {}: {e}", path.display()),
        })?;

        let parsed: ResolversRegistryToml =
            toml::from_str(&text).map_err(|e| ResolverLoadError::ParseError {
                detail: format!("TOML parse error in {}: {e}", path.display()),
            })?;

        // Validate schema_version — only version 1 is supported.
        if parsed.schema_version != 1 {
            return Err(ResolverLoadError::ParseError {
                detail: format!(
                    "unsupported schema_version {} in {} (expected 1)",
                    parsed.schema_version,
                    path.display()
                ),
            });
        }

        let mut registry = ResolverRegistry::new();
        let mut compiled_count = 0usize;
        let mut warnings: Vec<LoadWarning> = Vec::new();
        // BC-4.12.005 PC6: duplicate context_key is a registry-load error.
        let mut seen_context_keys: HashSet<String> = HashSet::new();

        // EC-009: zero [[resolvers]] entries ≡ absent file — valid, no error.
        for entry in parsed.resolvers {
            // F-P4-003: reject empty name / context_key — both are required non-empty identifiers.
            // An empty name would produce an unresolvable needs_context key; an empty context_key
            // would write to an anonymous plugin_config key which is never readable.
            if entry.name.trim().is_empty() {
                return Err(ResolverLoadError::ParseError {
                    detail: format!(
                        "resolver entry in {} has an empty 'name' field; \
                         name must be a non-empty string (BC-4.12.001 PC2)",
                        path.display()
                    ),
                });
            }
            if entry.context_key.trim().is_empty() {
                return Err(ResolverLoadError::ParseError {
                    detail: format!(
                        "resolver entry '{}' in {} has an empty 'context_key' field; \
                         context_key must be a non-empty string (BC-4.12.005 PC6)",
                        entry.name,
                        path.display()
                    ),
                });
            }

            // BC-4.12.005 PC6: validate context_key uniqueness across all entries.
            if !seen_context_keys.insert(entry.context_key.clone()) {
                return Err(ResolverLoadError::ParseError {
                    detail: format!(
                        "duplicate context_key '{}' in {} (resolver '{}') — \
                         each resolver context_key must be unique (BC-4.12.005 PC6)",
                        entry.context_key,
                        path.display(),
                        entry.name
                    ),
                });
            }

            // Compile the module (mtime-cached on subsequent loads).
            // F-P1-010: preserve IoError vs CompileError discrimination.
            // Missing/unreadable files → IoError; wasmtime failure → CompileError.
            // F-P2-003: branch on fail_closed to decide whether load failure is
            // fail-loud (default, true) or fail-open (false = skip + warn).
            let fail_closed = entry.fail_closed.unwrap_or(true);
            let module = match self.get_or_compile(&entry.plugin) {
                Ok(m) => m,
                Err(e) => {
                    let detail = match &e {
                        ResolverLoadError::IoError { detail } => {
                            format!("resolver '{}' — {}", entry.name, detail)
                        }
                        _ => format!("resolver '{}' — {}", entry.name, e),
                    };
                    if fail_closed {
                        // Fail-loud: abort registry load (default behavior).
                        return Err(match e {
                            ResolverLoadError::IoError { .. } => {
                                ResolverLoadError::IoError { detail }
                            }
                            other => other,
                        });
                    } else {
                        // Fail-open: skip entry, emit a resolver.load_warning and continue.
                        // F-P3-003 dual-emission pattern:
                        // 1. eprintln for startup-visible operator feedback (always emitted).
                        // 2. Append LoadWarning for caller to emit a structured InternalLog event.
                        eprintln!(
                            "factory-dispatcher: resolver.load_warning: skipping resolver '{}' \
                             (fail_closed=false) — {detail}",
                            entry.name
                        );
                        warnings.push(LoadWarning {
                            resolver_name: entry.name,
                            detail,
                        });
                        continue;
                    }
                }
            };

            // Wrap the compiled module in a CompiledWasmResolver.
            // Real WASM invocation is performed in CompiledWasmResolver::resolve().
            let wrapper = Box::new(CompiledWasmResolver {
                name: entry.name.clone(),
                context_key: entry.context_key,
                module,
                path_allow: entry.path_allow,
                engine: self.engine.clone(),
            });

            // register() returns Err only on duplicate names — fail-loud per BC-4.12.005 PC6.
            registry
                .register(wrapper)
                .map_err(|e| ResolverLoadError::ParseError {
                    detail: format!("resolver registration failed for '{}': {e}", entry.name),
                })?;

            compiled_count += 1;
        }

        // AC-012 / F-P1-009: dual log path — eprintln for startup visibility +
        // InternalLog event in main.rs (structured, queryable). Both are intentional:
        // - eprintln: operator-visible at startup for interactive debugging.
        // - InternalLog resolver.registry_loaded: queryable by observability stack.
        if compiled_count > 0 {
            eprintln!(
                "factory-dispatcher: Compiled {compiled_count} resolver modules from {}",
                path.display()
            );
        }

        Ok((registry, warnings))
    }
}

// ---------------------------------------------------------------------------
// CompiledWasmResolver — ContextResolver wrapper for a compiled Module
// ---------------------------------------------------------------------------

/// In-memory wrapper around a compiled `wasmtime::Module`.
///
/// Implements `ContextResolver` by performing real wasmtime instantiation
/// and WASM function invocation on each `resolve()` call.
///
/// Per BC-4.12.001 PC2: each invocation creates a fresh `Store<HostContext>`
/// (per-dispatch Store isolation). The `Module` is reused across calls;
/// the `Store` is created, used for one `resolve()` invocation, and dropped.
///
/// Per BC-4.12.003: the `path_allow` list is wired into the `HostContext`
/// capabilities, enforcing deny-by-default filesystem access for the resolver.
/// `path_allow` entries are resolved relative to `CLAUDE_PROJECT_DIR`
/// (BC-4.12.003 INV4).
///
/// Per BC-4.12.004: any wasmtime trap is caught via the `Err` path from
/// `TypedFunc::call`, classified by `classify_resolver_trap`, and returned
/// as `ResolverError::Trap`. The trap NEVER propagates out of `resolve()`.
pub(crate) struct CompiledWasmResolver {
    /// Registry name — must match the `needs_context` key.
    pub(crate) name: String,
    /// The plugin_config key under which this resolver's output is written.
    pub(crate) context_key: String,
    /// Compiled module (mtime-cached from `ResolverLoader::get_or_compile`).
    pub(crate) module: Arc<Module>,
    /// Declared path-allow list for `read_file` capability enforcement.
    /// Entries are project-relative (BC-4.12.003 INV4).
    pub(crate) path_allow: Vec<String>,
    /// The Engine shared with the executor (BC-4.12.001 INV3).
    pub(crate) engine: Engine,
}

impl ContextResolver for CompiledWasmResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn context_key(&self) -> &str {
        &self.context_key
    }

    /// Invoke the resolver's WASM `resolve()` export with full Store isolation.
    ///
    /// BC-4.12.001 PC2: fresh `Store<HostContext>` per call; no state carries over.
    /// BC-4.12.003: `path_allow` wired into HostContext.capabilities.read_file.
    /// BC-4.12.004: traps caught and classified; NEVER panic.
    ///
    /// ABI (HOST_ABI.md §Resolver ABI, BC-4.12.002 PC1):
    /// - Serialize `ResolverInput` → JSON bytes
    /// - Copy bytes into WASM memory
    /// - Call `resolve(input_ptr: i32, input_len: i32) -> i64`
    /// - Unpack `i64` as `(output_ptr: i32, output_len: i32)` via packed format:
    ///   `((ptr as i64) << 32) | (len as i64)`
    /// - Copy output bytes from WASM memory
    /// - Deserialize JSON → `ResolverOutput`
    fn resolve(&self, input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError> {
        // F-P3-004: Defensively validate project_dir non-empty before constructing
        // HostContext. BC-4.12.003 INV4 requires project-relative path resolution;
        // an empty project_dir would silently root all path_allow entries at "/"
        // which violates the capability model.
        if input.project_dir.is_empty() {
            return Err(ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: "ResolverInput.project_dir must not be empty \
                         (BC-4.12.003 INV4 requires project-relative path resolution)"
                    .to_string(),
            });
        }

        // Build HostContext with resolver's path_allow wired in (BC-4.12.003 / F-P1-007).
        // The `cwd` is set to the project_dir from the resolver input so that
        // path_allow entries are resolved relative to CLAUDE_PROJECT_DIR (F-P1-008).
        use crate::registry::ReadFileCaps;

        // TODO(F-P4-002): plumb session_id and trace_id into HostContext here.
        // Deferral: ResolverInput does not carry session_id/trace_id fields today.
        // The full plumbing path is: executor.rs::build_plugin_config → ResolverInput →
        // ResolverRegistry::resolve_context_for_entry → CompiledWasmResolver::resolve.
        // Options: (A) extend ResolverInput with session_id + trace_id fields, or
        // (B) add explicit parameters to the resolve() call chain.
        // For now empty strings are safe — resolver WASM does not observe them.
        // Tracking: follow-up story S-12.06 (HOST_ABI maintenance burst, F-P7-002).
        let mut host_ctx = HostContext::new(
            self.name.clone(),
            "0.0.0", // resolver version — not versioned separately from dispatcher
            "",      // session_id: deferred — see TODO(F-P4-002) above
            "",      // trace_id: deferred — see TODO(F-P4-002) above
        );
        // TODO(VP-076-C / F-P4-004): host_ctx.internal_log is None here (default from
        // HostContext::new). This means capability_denied events emitted by
        // HostContext::emit_internal() inside the resolver WASM do NOT flow to
        // VSDD_SINK_FILE. VP-076-C ("audit trail written for all capability denials")
        // is therefore verified structurally (path_allowed() returns false → resolver
        // cannot read the file → VP-076-B holds) rather than via sink event search.
        // Full sink-level verification requires plumbing an InternalLog reference into
        // this HostContext, which in turn requires threading it through the resolver
        // dispatch call chain. Tracked as F-P4-004 Option B deferral (pass-4 cycle).
        host_ctx.cwd = std::path::PathBuf::from(&input.project_dir);
        host_ctx.capabilities = Capabilities {
            read_file: Some(ReadFileCaps {
                path_allow: self.path_allow.clone(),
            }),
            ..Default::default()
        };

        // Build resolver linker against ResolverStoreData.
        //
        // WASI p1 fix (S-12.08 Step 3b): Rust binaries compiled for wasm32-wasip1
        // always import wasi_snapshot_preview1 syscalls (environ_get, fd_write, etc.)
        // from the std bootstrap. Without p1::add_to_linker_sync, instantiation fails
        // with "unknown import wasi_snapshot_preview1::environ_get".
        //
        // We register:
        //   1. vsdd::log + vsdd::read_file (resolver-scoped host functions, BC-4.12.003 INV2).
        //      These are re-registered directly against ResolverStoreData (accessing
        //      store_data.host) — same pattern as invoke.rs::setup_host_on_store_data.
        //   2. wasi_snapshot_preview1 (p1::add_to_linker_sync) so std bootstrap resolves.
        //      The WasiP1Ctx is restricted: no preopens, no stdio — resolvers must use
        //      vsdd::read_file for all I/O (BC-4.12.003 INV2).
        let linker =
            build_resolver_wasi_linker(&self.engine).map_err(|e| ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("resolver linker construction failed: {e}"),
            })?;

        // Restricted WASI context: no preopens, no stdin, stdout/stderr sink.
        // Resolvers must not access the filesystem directly — vsdd::read_file is the
        // bounded mechanism (BC-4.12.003 INV2). Env vars are empty for the same reason.
        let wasi_ctx = WasiCtxBuilder::new().build_p1();

        let store_data = ResolverStoreData {
            host: host_ctx,
            wasi: wasi_ctx,
        };

        // Create a fresh Store per invocation (BC-4.12.001 PC2 isolation).
        // Fuel enforcement: set a generous fuel budget; timeout via epoch interruption
        // (same engine configuration as hooks — epoch_interruption + consume_fuel).
        let mut store: Store<ResolverStoreData> = Store::new(&self.engine, store_data);
        // Epoch deadline: 25% of the 6000ms hook budget (1500ms per F1-amendment §S-12.04 sketch).
        // Enforced by the shared EpochTicker (same pattern as invoke.rs:174).
        // TODO: derive from HOOK_TIMEOUT_MS * 0.25 once HOOK_TIMEOUT_MS is constant-exported.
        const RESOLVER_TIMEOUT_MS: u64 = 1500;
        store.set_epoch_deadline(timeout_ms_to_epochs(RESOLVER_TIMEOUT_MS));
        // Fuel cap: 1 billion instructions (same default as hook plugins).
        // ResolverError::Timeout is returned on exhaustion.
        if let Err(e) = store.set_fuel(1_000_000_000) {
            return Err(ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("failed to set fuel on resolver store: {e}"),
            });
        }

        // Instantiate the compiled module against the resolver linker.
        let instance = linker.instantiate(&mut store, &self.module).map_err(|e| {
            ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("resolver instantiation failed: {e}"),
            }
        })?;

        // Get the `resolve` export (BC-4.12.002 PC1 signature: (i32, i32) -> i64).
        let resolve_fn = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, "resolve")
            .map_err(|e| ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("resolver does not export 'resolve(i32,i32)->i64': {e}"),
            })?;

        // Get the exported memory for reading/writing.
        let memory = instance.get_memory(&mut store, "memory").ok_or_else(|| {
            ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: "resolver does not export 'memory'".to_string(),
            }
        })?;

        // Serialize ResolverInput to JSON bytes.
        let input_bytes = serde_json::to_vec(input).map_err(|e| ResolverError::AbiViolation {
            name: self.name.clone(),
            detail: format!("failed to serialize ResolverInput: {e}"),
        })?;

        // Allocate space in WASM memory for the input.
        // The trapping_resolver.wasm fixture has 1 page (65536 bytes). We write
        // the input at the start of the memory (offset 0). This is safe for
        // the test fixture since it traps before reading memory at all.
        //
        // For real resolver modules that use the SDK macro, the macro-generated
        // shim manages its own memory; the dispatcher writes input at offset 0
        // per BC-4.12.002 PC1 packed-i64 ABI (see HOST_ABI.md §Resolver ABI Types).
        let input_ptr: i32 = 0;
        let input_len = input_bytes.len() as i32;

        // Bounds-check: ensure the input fits in memory.
        let mem_size = memory.data_size(&store);
        if input_bytes.len() > mem_size {
            return Err(ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!(
                    "ResolverInput ({} bytes) exceeds WASM memory ({mem_size} bytes)",
                    input_bytes.len()
                ),
            });
        }

        // Write the serialized ResolverInput into WASM memory.
        memory
            .write(&mut store, input_ptr as usize, &input_bytes)
            .map_err(|e| ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("failed to write ResolverInput to WASM memory: {e}"),
            })?;

        // Call the resolve function. Trap isolation: errors here are either
        // wasmtime::Trap (WASM execution trap / epoch interrupt) or other anyhow errors.
        // BC-4.12.004: traps MUST NOT propagate; classify_resolver_trap maps them to
        // ResolverError::Trap (fault) or ResolverError::Timeout (Interrupt / F-P3-002).
        let packed_result = resolve_fn
            .call(&mut store, (input_ptr, input_len))
            .map_err(|e| {
                // Check for fuel exhaustion (ResolverError::Timeout).
                if e.to_string().contains("all fuel consumed") {
                    return ResolverError::Timeout {
                        name: self.name.clone(),
                    };
                }
                // Downcast to wasmtime::Trap for classification.
                if let Some(trap) = e.downcast_ref::<wasmtime::Trap>() {
                    return classify_resolver_trap(&self.name, *trap);
                }
                // Other errors (ABI violation, link errors, etc.).
                ResolverError::Trap {
                    name: self.name.clone(),
                    detail: format!("{e}"),
                }
            })?;

        // Unpack packed i64 → (output_ptr: usize, output_len: usize).
        // HOST_ABI.md: `((ptr as i64) << 32) | (len as i64)`.
        //
        // F-P2-007: cast via u32 first (not i32) to eliminate sign-extension.
        // A `ptr` with bit 31 set would be misinterpreted as a large negative
        // i32 (e.g., 0x8000_0000 → -2147483648) when cast directly i64→i32.
        // Via u32 the value is correctly a large positive usize for >2GB memories.
        let output_ptr = ((packed_result >> 32) & 0xFFFF_FFFF) as u32 as usize;
        let output_len = (packed_result & 0xFFFF_FFFF) as u32 as usize;

        // A zero-length response means the resolver produced no output (Ok(None)).
        // F-P2-008: (0, 0) packed return convention — Ok(None) shortcut.
        if output_len == 0 {
            return Ok(None);
        }

        // Bounds-check the output region.
        let out_start = output_ptr;
        let out_end =
            out_start
                .checked_add(output_len)
                .ok_or_else(|| ResolverError::AbiViolation {
                    name: self.name.clone(),
                    detail: format!("output ptr+len overflow: ptr={output_ptr} len={output_len}"),
                })?;
        let mem_data = memory.data(&store);
        if out_end > mem_data.len() {
            return Err(ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!(
                    "output out-of-bounds: ptr={output_ptr} len={output_len} \
                     memory_size={}",
                    mem_data.len()
                ),
            });
        }

        // Copy output bytes out of WASM memory.
        let output_bytes = mem_data[out_start..out_end].to_vec();

        // Deserialize JSON → ResolverOutput.
        let output: ResolverOutput =
            serde_json::from_slice(&output_bytes).map_err(|e| ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("failed to deserialize ResolverOutput JSON: {e}"),
            })?;

        if output.value.is_none() {
            Ok(None)
        } else {
            Ok(Some(output))
        }
    }
}

// ---------------------------------------------------------------------------
// Resolver WASI linker helper
// ---------------------------------------------------------------------------

/// Build a `Linker<ResolverStoreData>` that wires both the resolver-scoped
/// vsdd host functions AND WASI snapshot preview 1 syscalls.
///
/// Host functions registered:
/// - `vsdd::log`     — diagnostic logging (BC-4.12.003 PC2)
/// - `vsdd::read_file` — bounded file access (BC-4.12.003 PC2)
///
/// Intentionally excluded (BC-4.12.003 INV2 — resolver sandbox):
/// - `vsdd::write_file`, `vsdd::exec_subprocess`, `vsdd::emit_event`
///
/// WASI p1 syscalls are required because Rust binaries compiled for
/// `wasm32-wasip1` always import `wasi_snapshot_preview1` during std
/// bootstrap (environ_get, fd_write, clock_time_get, etc.).
/// Without this, resolver instantiation fails with
/// "unknown import wasi_snapshot_preview1::environ_get".
///
/// The WasiCtx supplied to the store is restricted (no preopens, no stdio)
/// so WASI calls succeed at the linker level but produce no side effects
/// (BC-4.12.003 INV2: resolver must use vsdd::read_file for all I/O).
fn build_resolver_wasi_linker(engine: &Engine) -> Result<Linker<ResolverStoreData>, String> {
    use crate::host::codes;
    use crate::internal_log::InternalEvent;
    use serde_json::Value;
    use wasmtime::Caller;

    let mut linker: Linker<ResolverStoreData> = Linker::new(engine);

    // vsdd::log — proxy through store_data.host
    linker
        .func_wrap(
            "vsdd",
            "log",
            |mut caller: Caller<'_, ResolverStoreData>, level: u32, msg_ptr: u32, msg_len: u32| {
                // Read the message string from WASM memory.
                let msg = match read_wasm_str_rsd(&mut caller, msg_ptr, msg_len) {
                    Ok(s) => s,
                    Err(_) => return,
                };
                let level_str = match level {
                    0 => "trace",
                    1 => "debug",
                    2 => "info",
                    3 => "warn",
                    4 => "error",
                    _ => "info",
                };
                let host = &caller.data().host;
                let ev = InternalEvent::now("plugin.log")
                    .with_trace_id(&host.dispatcher_trace_id)
                    .with_session_id(&host.session_id)
                    .with_plugin_name(&host.plugin_name)
                    .with_plugin_version(&host.plugin_version)
                    .with_field("level", Value::String(level_str.to_string()))
                    .with_field("message", Value::String(msg));
                host.emit_internal(ev);
            },
        )
        .map_err(|e| e.to_string())?;

    // vsdd::read_file — mirrors the memory-growth protocol from invoke.rs::setup_host_on_store_data.
    //
    // The SDK's `read_file` wrapper initializes `out_ptr = 0` and passes &mut out_ptr
    // to the host. The host must:
    //   1. Read and capability-check the file (prepare())
    //   2. Grow WASM linear memory to hold the bytes
    //   3. Write the file bytes at the newly allocated address (current_bytes)
    //   4. Write (write_offset, len) back into the guest-provided out_ptr_out / out_len_out
    //
    // Writing bytes at address 0 (the old host/read_file.rs pattern) doesn't work because
    // read_owned_bytes(0, len) → Vec::new() in the SDK (ptr == 0 guard).
    // The memory-growth pattern (used by invoke.rs) is the correct implementation.
    linker
        .func_wrap(
            "vsdd",
            "read_file",
            |mut caller: Caller<'_, ResolverStoreData>,
             path_ptr: u32,
             path_len: u32,
             max_bytes: u32,
             _timeout_ms: u32,
             out_ptr_out: u32,
             out_len_out: u32|
             -> i32 {
                let path = match read_wasm_str_rsd(&mut caller, path_ptr, path_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };

                // Capability check + file read (host-side logic, no WASM memory).
                let body = {
                    let ctx = caller.data().host.clone();
                    match crate::host::read_file::prepare(&ctx, &path, max_bytes) {
                        Ok((bytes, _)) => bytes,
                        Err(code) => return code,
                    }
                };

                if body.is_empty() {
                    // Empty file: write ptr=0, len=0.
                    let _ = write_wasm_u32_rsd(&mut caller, out_ptr_out, 0);
                    let _ = write_wasm_u32_rsd(&mut caller, out_len_out, 0);
                    return codes::OK;
                }

                // Grow WASM memory to hold the file bytes and write at the new end.
                // This matches invoke.rs's read_file pattern (memory-growth protocol).
                let memory = match get_memory_rsd(&mut caller) {
                    Ok(m) => m,
                    Err(_) => return codes::INTERNAL_ERROR,
                };
                let current_bytes = memory.data_size(&caller);
                let pages_needed = body.len().div_ceil(65536) as u64;
                if memory.grow(&mut caller, pages_needed).is_err() {
                    return codes::INTERNAL_ERROR;
                }

                let write_offset = current_bytes as u32;

                if write_wasm_bytes_rsd(&mut caller, write_offset, body.len() as u32, &body)
                    .is_err()
                {
                    return codes::INTERNAL_ERROR;
                }

                // Return (ptr, len) to the guest via the out-params.
                if write_wasm_u32_rsd(&mut caller, out_ptr_out, write_offset).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                if write_wasm_u32_rsd(&mut caller, out_len_out, body.len() as u32).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                codes::OK
            },
        )
        .map_err(|e| e.to_string())?;

    // WASI snapshot preview 1 — required by Rust std bootstrap on wasm32-wasip1.
    // The accessor extracts &mut WasiP1Ctx from ResolverStoreData.
    // The WasiCtx is restricted (built with WasiCtxBuilder::new().build_p1() —
    // no preopens, no env, no stdio) so WASI calls succeed without side effects.
    p1::add_to_linker_sync(&mut linker, |d: &mut ResolverStoreData| &mut d.wasi)
        .map_err(|e| e.to_string())?;

    Ok(linker)
}

// ---------------------------------------------------------------------------
// ResolverStoreData memory helpers
// ---------------------------------------------------------------------------

fn get_memory_rsd(
    caller: &mut wasmtime::Caller<'_, ResolverStoreData>,
) -> Result<wasmtime::Memory, crate::host::HostCallError> {
    caller
        .get_export("memory")
        .and_then(|e| e.into_memory())
        .ok_or(crate::host::HostCallError::MissingMemory)
}

fn read_wasm_str_rsd(
    caller: &mut wasmtime::Caller<'_, ResolverStoreData>,
    ptr: u32,
    len: u32,
) -> Result<String, crate::host::HostCallError> {
    let memory = get_memory_rsd(caller)?;
    let data = memory.data(caller);
    let start = ptr as usize;
    let end = start
        .checked_add(len as usize)
        .ok_or(crate::host::HostCallError::MemoryOverflow)?;
    if end > data.len() {
        return Err(crate::host::HostCallError::OutOfBounds {
            ptr,
            len,
            memory_size: data.len(),
        });
    }
    String::from_utf8(data[start..end].to_vec())
        .map_err(|_| crate::host::HostCallError::InvalidUtf8)
}

fn write_wasm_bytes_rsd(
    caller: &mut wasmtime::Caller<'_, ResolverStoreData>,
    out_ptr: u32,
    out_cap: u32,
    bytes: &[u8],
) -> Result<u32, crate::host::HostCallError> {
    let needed = bytes.len() as u32;
    if needed > out_cap {
        return Ok(needed);
    }
    let memory = get_memory_rsd(caller)?;
    let start = out_ptr as usize;
    let end = start
        .checked_add(bytes.len())
        .ok_or(crate::host::HostCallError::MemoryOverflow)?;
    let data_len = memory.data(&mut *caller).len();
    if end > data_len {
        return Err(crate::host::HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        });
    }
    memory
        .write(caller, start, bytes)
        .map_err(|_| crate::host::HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        })?;
    Ok(needed)
}

fn write_wasm_u32_rsd(
    caller: &mut wasmtime::Caller<'_, ResolverStoreData>,
    out_ptr: u32,
    value: u32,
) -> Result<(), crate::host::HostCallError> {
    let bytes = value.to_le_bytes();
    write_wasm_bytes_rsd(caller, out_ptr, bytes.len() as u32, &bytes)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Standalone constructor
// ---------------------------------------------------------------------------

/// Return an empty `ResolverRegistry` with no resolvers registered.
///
/// Used at startup when `resolvers-registry.toml` is absent (BC-1.13.001
/// INV2: absent registry = zero resolvers, not an error).
///
/// GREEN-BY-DESIGN: delegates to `ResolverRegistry::new()` — zero
/// branching, no I/O, no non-trivial helpers, 1 line. Satisfies BC-5.38.002.
pub fn empty() -> ResolverRegistry {
    ResolverRegistry::new()
}

// ---------------------------------------------------------------------------
// Integration tests — resolver WASI linker + WaveContextResolver
// ---------------------------------------------------------------------------

#[cfg(test)]
mod wasi_integration_tests {
    use super::*;
    use crate::engine::build_engine;
    use crate::resolver::ResolverInput;
    use tempfile::TempDir;

    /// Helper: set up a factory directory with test fixtures.
    fn make_factory_root(passes_clean_s001: u32, passes_clean_s002: u32) -> TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory = dir.path().join(".factory");
        std::fs::create_dir_all(&factory).unwrap();

        // STATE.md
        std::fs::write(
            factory.join("STATE.md"),
            "---\ncurrent_cycle: test-cycle-001\n---\n",
        )
        .unwrap();

        // wave-state.yaml
        std::fs::write(
            factory.join("wave-state.yaml"),
            "waves:\n  - wave: test-wave-001\n    stories:\n      - S-FAKE-001\n      - S-FAKE-002\n",
        )
        .unwrap();

        // Convergence states
        let cycles = factory.join("cycles/test-cycle-001");
        for (story, passes) in &[
            ("S-FAKE-001", passes_clean_s001),
            ("S-FAKE-002", passes_clean_s002),
        ] {
            let story_dir = cycles.join(story);
            std::fs::create_dir_all(&story_dir).unwrap();
            let state = format!(
                r#"{{"passes_clean":{passes},"last_classification":"NITPICK_ONLY","last_finding_count":0,"last_timestamp":"2026-05-10T00:00:00Z","deferred_findings":[]}}"#
            );
            std::fs::write(story_dir.join("adversary-convergence-state.json"), &state).unwrap();
        }

        dir
    }

    /// Test that CompiledWasmResolver::resolve() with the production
    /// vsdd-context-resolvers.wasm correctly injects wave_context when the
    /// factory root has wave-state.yaml and STATE.md.
    ///
    /// This test validates the full resolver pipeline:
    ///   1. WASI linker construction (build_resolver_wasi_linker)
    ///   2. WASM module instantiation with ResolverStoreData
    ///   3. read_file host function with memory-growth protocol
    ///   4. WaveContextResolver logic (wave_state.yaml + STATE.md parsing)
    ///   5. Packed-i64 return → Ok(Some(ResolverOutput))
    ///
    /// If this test passes, the bats integration tests should also pass.
    #[test]
    fn test_wave_context_resolver_wasm_injects_context() {
        // Path to the production WASM artifact (built by cargo build).
        // We resolve relative to CARGO_MANIFEST_DIR.
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent() // crates/
            .unwrap()
            .parent() // workspace root
            .unwrap();
        let wasm_path =
            workspace_root.join("plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm");

        if !wasm_path.exists() {
            eprintln!(
                "SKIP: vsdd-context-resolvers.wasm not found at {}",
                wasm_path.display()
            );
            return; // Skip if WASM not built yet
        }

        let factory_root = make_factory_root(1, 3);
        let project_dir = factory_root.path().to_str().unwrap().to_string();

        let engine = build_engine().expect("build_engine");
        let loader = ResolverLoader::new(engine.clone());
        let module = loader
            .get_or_compile(&wasm_path)
            .expect("should compile vsdd-context-resolvers.wasm");

        let resolver = CompiledWasmResolver {
            name: "wave_context".to_string(),
            context_key: "wave_context".to_string(),
            module,
            path_allow: vec![".factory/".to_string()],
            engine,
        };

        let input = ResolverInput {
            event_type: "SubagentStop".to_string(),
            hook_event_name: "validate-per-story-adversary-convergence".to_string(),
            agent_type: Some("wave-gate-dispatch".to_string()),
            project_dir: project_dir.clone(),
            plugin_config: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = resolver.resolve(&input);

        match result {
            Ok(Some(output)) => {
                eprintln!("Output value: {:?}", output.value);
                assert!(
                    output.value.is_some(),
                    "resolver should inject wave_context"
                );
                let value = output.value.unwrap();
                assert!(
                    value.get("stories").is_some(),
                    "wave_context should have 'stories' key"
                );
            }
            Ok(None) => {
                panic!(
                    "resolver returned Ok(None) — WASM ran but produced no output. \
                     Check read_file host function and path_allow enforcement. \
                     project_dir={}",
                    project_dir
                );
            }
            Err(e) => {
                panic!("resolver returned error: {:?}", e);
            }
        }
    }
}
