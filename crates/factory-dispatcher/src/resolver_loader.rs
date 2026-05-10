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
use wasmtime::{Engine, Module, Store};

use crate::engine::timeout_ms_to_epochs;
use crate::host::HostContext;
use crate::registry::Capabilities;
use crate::resolver::{
    ContextResolver, ResolverError, ResolverInput, ResolverOutput, ResolverRegistry,
};
use crate::resolver_classify_trap::classify_resolver_trap;

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

        let mut host_ctx = HostContext::new(
            self.name.clone(),
            "0.0.0", // resolver version — not versioned separately from dispatcher
            "",      // session_id: available in ResolverInput.project_dir context
            "",      // trace_id: not available at this layer; propagated via InternalLog
        );
        host_ctx.cwd = std::path::PathBuf::from(&input.project_dir);
        host_ctx.capabilities = Capabilities {
            read_file: Some(ReadFileCaps {
                path_allow: self.path_allow.clone(),
            }),
            ..Default::default()
        };

        // Build resolver linker (read_file + log only; no write/exec/emit per BC-4.12.003 INV2).
        // F-P1-012: resolver_linker returns Result; propagate registration errors via map_err.
        let linker = crate::host::resolver_linker(&self.engine).map_err(|e| {
            ResolverError::AbiViolation {
                name: self.name.clone(),
                detail: format!("resolver linker construction failed: {e}"),
            }
        })?;

        // Create a fresh Store per invocation (BC-4.12.001 PC2 isolation).
        // Fuel enforcement: set a generous fuel budget; timeout via epoch interruption
        // (same engine configuration as hooks — epoch_interruption + consume_fuel).
        let mut store: Store<HostContext> = Store::new(&self.engine, host_ctx);
        // Epoch deadline: 25% of the 6000ms hook budget (1500ms per F1-amendment §S-12.04 sketch).
        // Enforced by the shared EpochTicker (same pattern as invoke.rs:174).
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
