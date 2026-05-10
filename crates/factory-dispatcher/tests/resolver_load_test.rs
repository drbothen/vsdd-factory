//! `ResolverLoader` loading-lifecycle tests (S-12.04 AC-001, AC-002, AC-003).
//!
//! These tests exercise `ResolverLoader::get_or_compile` and
//! `ResolverLoader::load_registry` with real tempfiles. They are stubbed
//! with `todo!()` bodies — Red Gate per BC-5.38.001.
//!
//! WASM test fixtures:
//! - `fixtures/trapping_resolver.wasm` — deferred; generated via wat2wasm
//!   by the implementer during Step 3 (or provided by S-12.07). Tests
//!   reference the path but will fail at fixture-not-found before reaching
//!   the todo!() — acceptable for Red Gate (BC-5.38.001).
//!
//! Architecture anchors:
//! - BC-4.12.001 — WASM resolver loading and caching contract
//! - S-12.04 AC-001 (deterministic load), AC-002 (no invoke on load),
//!   AC-003 (mtime-triggered recompilation)

#[allow(unused_imports)]
use factory_dispatcher::resolver_loader::ResolverLoader;

/// AC-001: Loading the same WASM path twice returns byte-identical compiled
/// modules (deterministic load — no randomness in wasmtime module bytes).
///
/// BC-5.38.005 self-check: "If I include a real implementation here, will
/// the test pass trivially without implementer work?" — Yes (todo!() is the
/// only acceptable body; the real assertion requires load + hash comparison).
/// Body: todo!() per BC-5.38.001.
#[test]
fn test_load_is_deterministic() {
    todo!("S-12.04 Step 2 test authoring")
}

/// AC-002: `get_or_compile` caches the module but does NOT invoke
/// `resolve()` on load — loading is side-effect-free with respect to
/// resolver execution (BC-4.12.001 INV3).
///
/// BC-5.38.005 self-check: real body would require a side-effect-tracking
/// WASM fixture and assertion logic. todo!() per BC-5.38.001.
#[test]
fn test_load_does_not_invoke_resolve() {
    todo!()
}

/// AC-003: When a WASM file's mtime advances the loader discards the
/// cached `Arc<Module>` and recompiles from the new bytes.
///
/// BC-5.38.005 self-check: real body requires tempfile write + mtime
/// manipulation (via `filetime`) + two `get_or_compile` calls +
/// pointer-inequality assertion. todo!() per BC-5.38.001.
#[test]
fn test_mtime_change_triggers_recompilation() {
    todo!()
}
