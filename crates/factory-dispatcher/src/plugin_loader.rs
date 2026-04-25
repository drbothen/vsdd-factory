//! Cached wasmtime module loader keyed by `(path, mtime)`.
//!
//! Compiling a wasm module with wasmtime costs single-digit ms per
//! module; over a long-running session that adds up. The cache keeps
//! compiled `Module` values hot across invocations. Invalidation is
//! mtime-driven: if the file on disk changes, the next `get_or_compile`
//! recompiles automatically (operator-friendly — no dispatcher restart
//! needed after a plugin rebuild).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use thiserror::Error;
use wasmtime::{Engine, Module};

#[derive(Debug, Error)]
pub enum PluginLoadError {
    #[error("plugin file not found: {0}")]
    NotFound(PathBuf),
    #[error("plugin file io error for {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("wasmtime module compile failed for {path}: {source}")]
    Compile {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
}

#[derive(Debug, Clone)]
struct CacheKey {
    path: PathBuf,
    mtime: SystemTime,
}

/// Thread-safe module cache. One per `wasmtime::Engine`.
pub struct PluginCache {
    engine: Engine,
    entries: Mutex<HashMap<PathBuf, (SystemTime, Module)>>,
}

impl PluginCache {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Look up a module by path. If the file has been modified since
    /// we last compiled it, recompile. If it hasn't been seen yet,
    /// compile on first use.
    pub fn get_or_compile(&self, path: &Path) -> Result<Module, PluginLoadError> {
        let key = probe(path)?;

        {
            let entries = self.entries.lock().expect("plugin cache poisoned");
            if let Some((mtime, module)) = entries.get(&key.path)
                && *mtime == key.mtime
            {
                return Ok(module.clone());
            }
        }

        let bytes = std::fs::read(&key.path).map_err(|e| PluginLoadError::Io {
            path: key.path.clone(),
            source: e,
        })?;
        let module =
            Module::from_binary(&self.engine, &bytes).map_err(|e| PluginLoadError::Compile {
                path: key.path.clone(),
                source: anyhow::Error::from(e),
            })?;

        let mut entries = self.entries.lock().expect("plugin cache poisoned");
        entries.insert(key.path.clone(), (key.mtime, module.clone()));
        Ok(module)
    }

    /// Number of cached modules. Useful for tests.
    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.entries.lock().map(|e| e.len()).unwrap_or(0)
    }
}

fn probe(path: &Path) -> Result<CacheKey, PluginLoadError> {
    if !path.exists() {
        return Err(PluginLoadError::NotFound(path.to_path_buf()));
    }
    let meta = std::fs::metadata(path).map_err(|e| PluginLoadError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    Ok(CacheKey {
        path: path.to_path_buf(),
        mtime,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;
    use std::time::Duration;

    /// A minimal valid WASI preview-1 command. Empty `_start`.
    fn minimal_wat() -> Vec<u8> {
        let wat = r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")))
        "#;
        wat::parse_str(wat).expect("WAT fixture should parse")
    }

    fn write_wasm_to(path: &Path) {
        std::fs::write(path, minimal_wat()).unwrap();
    }

    #[test]
    fn not_found_error_for_missing_path() {
        let engine = build_engine().unwrap();
        let cache = PluginCache::new(engine);
        let err = cache
            .get_or_compile(Path::new("/nonexistent/ghost.wasm"))
            .unwrap_err();
        assert!(matches!(err, PluginLoadError::NotFound(_)));
    }

    #[test]
    fn compiles_on_first_use_and_caches() {
        let engine = build_engine().unwrap();
        let cache = PluginCache::new(engine);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.wasm");
        write_wasm_to(&path);

        cache.get_or_compile(&path).unwrap();
        assert_eq!(cache.size(), 1);
        cache.get_or_compile(&path).unwrap();
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn invalidates_on_mtime_change() {
        let engine = build_engine().unwrap();
        let cache = PluginCache::new(engine);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("b.wasm");
        write_wasm_to(&path);

        cache.get_or_compile(&path).unwrap();
        assert_eq!(cache.size(), 1);

        // Bump mtime by touching the file with a later timestamp.
        std::thread::sleep(Duration::from_millis(15));
        let later = std::time::SystemTime::now() + Duration::from_secs(5);
        filetime::set_file_mtime(&path, filetime::FileTime::from_system_time(later)).unwrap();

        cache.get_or_compile(&path).unwrap();
        // Same path → entry replaced in place.
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn corrupt_bytes_produce_compile_error() {
        let engine = build_engine().unwrap();
        let cache = PluginCache::new(engine);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.wasm");
        std::fs::write(&path, b"not a wasm module").unwrap();
        let err = cache.get_or_compile(&path).unwrap_err();
        assert!(matches!(err, PluginLoadError::Compile { .. }));
    }
}
