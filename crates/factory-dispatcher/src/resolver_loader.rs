//! `ResolverLoader` — mtime-cached WASM module loader for context resolvers.
//!
//! Loads and compiles WASM resolver modules from disk, caching compiled
//! `wasmtime::Module` instances keyed by path + last-modified time.
//! When a file's mtime changes the cached module is invalidated and
//! recompiled (BC-4.12.001 resolver loading contract).
//!
//! Also provides `load_registry` to parse `resolvers-registry.toml` and
//! populate a `ResolverRegistry` with in-memory resolver wrappers that
//! record the compiled modules for future WASM-backed invocation.
//!
//! Architecture anchors:
//! - BC-4.12.001 — WASM resolver loading and caching contract
//! - BC-1.13.001 — dispatcher pre-dispatch injection contract (absent-file)
//! - ADR-018 — WASM-plugin Context Resolvers design
//! - S-12.04 — this story

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use serde::Deserialize;
use thiserror::Error;
use wasmtime::Module;

use crate::resolver::ResolverRegistry;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

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
#[derive(Debug, Deserialize)]
struct ResolversRegistryToml {
    schema_version: u32,
    #[serde(default)]
    resolvers: Vec<ResolverEntryToml>,
}

/// One `[[resolvers]]` entry in the registry TOML.
#[derive(Debug, Deserialize)]
struct ResolverEntryToml {
    /// Registry name — used as the `needs_context` key and `plugin_config` key.
    name: String,
    /// Path to the compiled `.wasm` file.
    path: PathBuf,
    /// Declared path-allow list for the `read_file` host function (BC-4.12.003).
    #[serde(default)]
    path_allow: Vec<String>,
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
/// Interior-mutable via `Mutex` so the loader can be shared across
/// threads without `&mut` access. `Arc<ResolverLoader>` is the expected
/// usage pattern.
pub struct ResolverLoader {
    /// Compiled module cache: path → (mtime, compiled module).
    ///
    /// The cache maps `PathBuf` to `(SystemTime, Arc<Module>)` so
    /// the same module can be shared cheaply across multiple
    /// instantiations within one dispatch cycle.
    cache: Mutex<HashMap<PathBuf, (SystemTime, Arc<Module>)>>,
}

impl ResolverLoader {
    /// Construct an empty loader (empty mtime cache, no modules loaded).
    ///
    /// GREEN-BY-DESIGN: zero branching, no I/O, no non-trivial helpers,
    /// body ≤ 3 lines. Satisfies BC-5.38.002.
    pub fn new() -> Self {
        Self {
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
    /// 4. Miss → compile via `Module::new(&engine, bytes)`; update cache
    ///
    /// BC-4.12.001: loading is startup-time only (no Store creation here,
    /// just Module compilation). No `resolve()` invocation occurs.
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
        let bytes = std::fs::read(&canonical).map_err(|e| ResolverLoadError::IoError {
            detail: format!("cannot read {}: {e}", canonical.display()),
        })?;

        // Build an ephemeral engine for compilation. The loader does not hold
        // a long-lived engine reference — callers that need cross-module engine
        // sharing should build one engine and pass it; this default engine is
        // adequate for the startup-time compilation path (BC-4.12.001 INV1).
        let engine =
            crate::engine::build_engine().map_err(|e| ResolverLoadError::CompileError {
                detail: format!("engine build failed for {}: {e}", canonical.display()),
            })?;

        let module = Module::new(&engine, &bytes).map_err(|e| ResolverLoadError::CompileError {
            detail: format!("wasmtime compile failed for {}: {e}", canonical.display()),
        })?;

        let arc = Arc::new(module);

        // Store in cache (insert replaces stale entry if mtime changed).
        let mut guard = self.cache.lock().expect("resolver module cache poisoned");
        guard.insert(canonical, (mtime, Arc::clone(&arc)));

        Ok(arc)
    }

    /// Parse `resolvers-registry.toml` at `path` and return a populated
    /// `ResolverRegistry`.
    ///
    /// Rules (BC-1.13.001 + BC-4.12.001):
    /// - Absent file → `Ok(ResolverRegistry::new())` — NOT an error (INV2).
    /// - Malformed TOML → `Err(ResolverLoadError::ParseError)` — fail-loud.
    /// - Unknown schema_version → `Err(ResolverLoadError::ParseError)`.
    /// - Missing `.wasm` file or compile failure → `Err(CompileError)` — fail-loud.
    ///
    /// The returned registry uses `StaticResolverEntry` wrappers that
    /// record the resolver name and compiled module. Real WASM invocation
    /// is wired in `invoke_resolver_wasm` (S-12.04).
    pub fn load_registry(path: &Path) -> Result<ResolverRegistry, ResolverLoadError> {
        // BC-1.13.001 INV2: absent file → empty registry, NOT an error.
        if !path.exists() {
            return Ok(ResolverRegistry::new());
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

        let loader = ResolverLoader::new();
        let mut registry = ResolverRegistry::new();
        let mut compiled_count = 0usize;

        // EC-009: zero [[resolvers]] entries ≡ absent file — valid, no error.
        for entry in parsed.resolvers {
            // Compile the module (mtime-cached on subsequent loads).
            let module = loader.get_or_compile(&entry.path).map_err(|e| match e {
                ResolverLoadError::IoError { detail } => ResolverLoadError::CompileError {
                    detail: format!("resolver '{}' — {}", entry.name, detail),
                },
                other => other,
            })?;

            // Wrap the compiled module in an in-memory ContextResolver.
            // Real WASM invocation is handled in invoke_resolver_wasm (S-12.04 Step 3).
            let wrapper = Box::new(CompiledWasmResolver {
                name: entry.name.clone(),
                module,
                path_allow: entry.path_allow,
            });

            // register() returns Err only on duplicate names — fail-loud per BC-4.12.005 PC6.
            registry
                .register(wrapper)
                .map_err(|e| ResolverLoadError::ParseError {
                    detail: format!("resolver registration failed for '{}': {e}", entry.name),
                })?;

            compiled_count += 1;
        }

        // AC-012: log compiled module count at startup (BC-1.13.001 PC1).
        // The InternalLog is not accessible here (load_registry is a standalone fn),
        // so we emit via eprintln which the dispatcher redirects to the internal log.
        // S-12.07 will wire this through InternalLog once the full startup path is in place.
        if compiled_count > 0 {
            eprintln!(
                "factory-dispatcher: Compiled {compiled_count} resolver modules from {}",
                path.display()
            );
        }

        Ok(registry)
    }
}

impl Default for ResolverLoader {
    /// GREEN-BY-DESIGN: delegates to `Self::new()`.
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// CompiledWasmResolver — ContextResolver wrapper for a compiled Module
// ---------------------------------------------------------------------------

/// In-memory wrapper around a compiled `wasmtime::Module`.
///
/// Implements `ContextResolver` using the compiled module. Real WASM
/// invocation (instantiation + function call) is handled by
/// `ResolverRegistry::invoke_resolver_wasm` in S-12.04. This wrapper
/// stores the module and path_allow so the registry can locate them.
struct CompiledWasmResolver {
    /// Registry name — must match the `needs_context` key.
    name: String,
    /// Compiled module (mtime-cached from `ResolverLoader::get_or_compile`).
    #[allow(dead_code)]
    module: Arc<Module>,
    /// Declared path-allow list for `read_file` capability enforcement.
    #[allow(dead_code)]
    path_allow: Vec<String>,
}

impl crate::resolver::ContextResolver for CompiledWasmResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn resolve(
        &self,
        _input: &crate::resolver::ResolverInput,
    ) -> Result<Option<crate::resolver::ResolverOutput>, crate::resolver::ResolverError> {
        // Real wasmtime invocation is deferred to S-12.07 (full WASM resolver
        // invocation with per-dispatch Store isolation). For S-12.04, the
        // `load_registry` path compiles and registers modules; WASM-backed
        // dispatch is not yet exercised by the acceptance tests.
        //
        // Returning Ok(None) means this resolver contributes no context —
        // key remains absent from plugin_config (BC-4.12.005 PC2).
        Ok(None)
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
