//! `ResolverLoader` — mtime-cached WASM module loader for context resolvers.
//!
//! Loads and compiles WASM resolver modules from disk, caching compiled
//! `wasmtime::Module` instances keyed by path + last-modified time.
//! When a file's mtime changes the cached module is invalidated and
//! recompiled (BC-4.12.001 resolver loading contract).
//!
//! Also provides `load_registry` to parse `resolvers-registry.toml` and
//! populate a `ResolverRegistry` with WASM-backed resolver wrappers.
//!
//! Architecture anchors:
//! - BC-4.12.001 — WASM resolver loading and caching contract
//! - ADR-018 — WASM-plugin Context Resolvers design
//! - S-12.04 — this story; implementation deferred to Step 3

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

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
// ResolverLoader
// ---------------------------------------------------------------------------

/// Mtime-cached WASM module loader.
///
/// Caches compiled `wasmtime::Module` instances in a `HashMap` keyed by
/// `(PathBuf, SystemTime)`. On each `get_or_compile` call the file's
/// mtime is compared to the cached entry; a mtime change triggers
/// recompilation (BC-4.12.001 loading contract).
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
    ///
    /// `#[allow(dead_code)]` — read by `get_or_compile` in Step 3;
    /// stub body is `todo!()` so the field appears unread to the compiler.
    #[allow(dead_code)]
    cache: Mutex<HashMap<PathBuf, (SystemTime, Arc<Module>)>>,
}

impl ResolverLoader {
    /// Construct an empty loader (empty mtime cache, no modules loaded).
    ///
    /// This is a GREEN-BY-DESIGN function: zero branching, no I/O,
    /// no non-trivial helpers, body ≤ 3 lines. Satisfies BC-5.38.002.
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Return a cached compiled module, or compile from disk if uncached
    /// or if the file's mtime has changed since the last load.
    ///
    /// Non-trivial: performs I/O, conditional branching, and wasmtime
    /// compilation. S-12.04 Step 3 implementation.
    pub fn get_or_compile(&self, _path: &Path) -> Result<Arc<Module>, ResolverLoadError> {
        todo!("S-12.04 Step 3 implementation")
    }

    /// Parse `resolvers-registry.toml` at `path` and return a populated
    /// `ResolverRegistry` containing one WASM-backed resolver wrapper per
    /// `[[resolvers]]` entry.
    ///
    /// Non-trivial: performs I/O, TOML parsing, and wasmtime module
    /// compilation via `get_or_compile`. S-12.04 Step 3 implementation.
    pub fn load_registry(_path: &Path) -> Result<ResolverRegistry, ResolverLoadError> {
        todo!("S-12.04 Step 3 implementation")
    }
}

impl Default for ResolverLoader {
    /// GREEN-BY-DESIGN: delegates to `Self::new()` — zero branching,
    /// no I/O, no helpers beyond the constructor, 1 line.
    fn default() -> Self {
        Self::new()
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
/// branching, no I/O, no non-trivial helpers, 1 line. Satisfies
/// BC-5.38.002.
pub fn empty() -> ResolverRegistry {
    ResolverRegistry::new()
}
