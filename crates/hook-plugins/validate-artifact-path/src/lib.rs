//! validate-artifact-path — PreToolUse WASM hook plugin.
//!
//! Validates that any Write or Edit call targeting `.factory/` matches a
//! registered canonical path pattern in
//! `plugins/vsdd-factory/config/artifact-path-registry.yaml`.
//! Unregistered paths are blocked via `HookResult::block_with_fix`.
//!
//! # Behavioral Contracts
//!
//! - BC-4.11.001: hook consults artifact-path-registry.yaml as single source
//!   of truth; blocks writes whose paths do not match a registered pattern
//!   (immediate block mode per OQ5 resolution).
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - All non-trivial function bodies are `todo!()` per BC-5.38.001 (S-13.01 stub).
//! - Pure `fn hook_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.
//! - No hardcoded `.factory/` path pattern literals in source (BC-4.11.001 invariant 1).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.11.001 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Path to the artifact registry YAML (relative to CLAUDE_PROJECT_DIR).
/// This constant is a registry-access address, NOT a path pattern list.
/// It does NOT violate BC-4.11.001 invariant 1 (no embedded path patterns).
pub const REGISTRY_PATH: &str = "plugins/vsdd-factory/config/artifact-path-registry.yaml";

// ---------------------------------------------------------------------------
// Registry data model (schema per ADR-016)
// ---------------------------------------------------------------------------

/// A single entry in the artifact-path-registry.yaml.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryEntry {
    /// Artifact type identifier (e.g. "behavioral-contract", "adr").
    pub artifact_type: String,
    /// Canonical path pattern with `{placeholder}` expansion markers.
    pub canonical_path_pattern: String,
    /// Human-readable description of this artifact type.
    pub description: String,
    /// Enforcement level: "block", "warn", or "advisory".
    pub enforcement_level: String,
}

/// The full parsed artifact path registry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathRegistry {
    /// Registry schema version. Must be 1.
    pub version: u32,
    /// All registered artifact types.
    pub artifacts: Vec<RegistryEntry>,
}

/// Error type for registry parse failures.
#[derive(Debug)]
pub enum RegistryError {
    /// YAML parse error.
    ParseError(String),
    /// Missing required field.
    MissingField(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::ParseError(msg) => write!(f, "registry parse error: {}", msg),
            RegistryError::MissingField(field) => {
                write!(f, "registry missing required field: {}", field)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Path match result
// ---------------------------------------------------------------------------

/// Result of matching a write-target path against the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult {
    /// Path matches a registry entry with `enforcement_level: block`.
    /// Write proceeds (the path is valid and canonical).
    Block,
    /// Path matches a registry entry with `enforcement_level: warn`.
    /// Write proceeds after emitting a warn event.
    Warn { pattern: String },
    /// Path matches a registry entry with `enforcement_level: advisory`.
    /// Write proceeds after logging debug.
    Advisory { pattern: String },
    /// Path does not match any registry entry.
    /// Write must be blocked (ARTIFACT_PATH_UNREGISTERED).
    NoMatch,
}

// ---------------------------------------------------------------------------
// Pure core functions (injectable-callback pattern)
// ---------------------------------------------------------------------------

/// Load and parse the artifact path registry from a YAML string.
///
/// Returns `Ok(PathRegistry)` on success, `Err(RegistryError)` on parse failure.
/// Never panics on any input (VP-069 proptest invariant).
///
/// # BC trace
/// BC-4.11.001 postcondition 1 — registry is read, never embedded as literals.
/// VP-069 Part A — registry-load never panics on malformed YAML.
///
/// # S-13.01 Step 4 implementer
/// Implement YAML parsing using `serde_yaml`. If binary > 500 KB after linking,
/// switch to minimal YAML-subset parser per F1 OQ-1 (story dev notes).
pub fn load_registry(_yaml: &str) -> Result<PathRegistry, RegistryError> {
    todo!("S-13.01 Step 4 implementer — see BC-4.11.001 PC1; VP-069 harness; parse yaml str into PathRegistry using serde_yaml")
}

/// Match a write-target `path` against all entries in `registry`.
///
/// Returns the `MatchResult` for the first matching entry (first-match-wins,
/// per BC-4.11.001 EC-005). Returns `MatchResult::NoMatch` if no entry matches.
///
/// This function is pure and deterministic: for the same `(path, registry)`
/// pair it always returns the same `MatchResult` (VP-070 invariant).
///
/// # BC trace
/// BC-4.11.001 postcondition 2b — match path against registered patterns.
/// BC-4.11.001 EC-005 — first matching entry wins.
/// VP-070 — path matching is pure and deterministic.
///
/// # S-13.01 Step 4 implementer
/// Implement `canonical_path_pattern` matching with `{placeholder}` expansion
/// per BC-4.11.001 invariant 6. Pattern semantics are defined in F4 implementation scope.
pub fn matches_canonical(_path: &str, _registry: &PathRegistry) -> MatchResult {
    todo!("S-13.01 Step 4 implementer — see BC-4.11.001 PC2b, EC-005; VP-070 kani harness; first-match-wins pattern matching with placeholder expansion")
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `hook_logic` for testability.
/// In production (main.rs), these are wired to host fns.
pub struct HookCallbacks<R, E, L>
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    /// Read a file by path; returns `Ok(contents)` or `Err(message)` if absent/error.
    pub read_file: R,
    /// Emit an event (type, fields).
    pub emit_event: E,
    /// Log a message at the given level (0=trace, 1=debug, 2=info, 3=warn, 4=error).
    pub log: L,
}

/// Core validate-artifact-path hook logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Reads `tool_input.file_path` from `payload`. For paths under `.factory/`,
/// loads the registry and matches the path. Returns `HookResult::Continue` for
/// non-`.factory/` paths (early-exit). Returns `HookResult::block_with_fix` for
/// unregistered `.factory/` paths.
///
/// Graceful degrade: registry absent or malformed → `HookResult::Continue` + log.
///
/// # BC traces
/// - BC-4.11.001 PC2-9: core enforcement logic
/// - BC-4.11.001 EC-001: registry absent → Continue + log_debug
/// - BC-4.11.001 EC-002: registry malformed → Continue + log_error
/// - BC-4.11.001 EC-004/PC7: non-.factory/ path → early-exit Continue
/// - BC-4.11.001 EC-006: file_path absent → Continue + log_warn
///
/// # S-13.01 Step 4 implementer — see BC-4.11.001 full postcondition/edge-case spec
pub fn hook_logic<R, E, L>(
    _payload: HookPayload,
    _callbacks: HookCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    todo!("S-13.01 Step 4 implementer — see BC-4.11.001 PC2-9, EC-001/002/004/005/006; wire load_registry + matches_canonical + block_with_fix; all branches must be covered by unit tests")
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`.
///
/// # S-13.01 Step 4 implementer
/// Wire `host::read_file`, `host::emit_event`, `host::log_*` to
/// `hook_logic`'s injectable callbacks. See `regression_gate::on_post_tool_use`
/// in `regression-gate/src/lib.rs` for the canonical wiring pattern.
pub fn on_pre_tool_use(_payload: HookPayload) -> HookResult {
    todo!("S-13.01 Step 4 implementer — wire host::read_file, host::emit_event, host::log_* to hook_logic callbacks; see regression-gate on_post_tool_use for canonical pattern")
}

// ---------------------------------------------------------------------------
// Unit tests (all non-trivial test bodies are todo!() — red by design)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    fn make_payload(tool_name: &str, file_path: Option<&str>) -> HookPayload {
        let mut v = json!({
            "event_name": "PreToolUse",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_name": tool_name,
            "tool_input": {}
        });
        if let Some(path) = file_path {
            v["tool_input"]["file_path"] = json!(path);
        }
        serde_json::from_value(v).expect("fixture must deserialize")
    }

    fn run_logic(
        payload: HookPayload,
        registry_content: Option<&str>,
    ) -> (HookResult, Option<String>, Option<(String, Vec<(String, String)>)>) {
        let mut emitted: Option<(String, Vec<(String, String)>)> = None;
        let mut logged: Option<String> = None;
        let registry_content = registry_content.map(|s| s.to_string());

        let result = hook_logic(
            payload,
            HookCallbacks {
                read_file: move |_path| {
                    registry_content
                        .clone()
                        .ok_or_else(|| "registry absent".to_string())
                },
                emit_event: |event_type, fields| {
                    emitted = Some((
                        event_type.to_string(),
                        fields
                            .iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                    ));
                },
                log: |_level, msg| {
                    logged = Some(msg.to_string());
                },
            },
        );

        (result, logged, emitted)
    }

    // -----------------------------------------------------------------------
    // AC-003 (BC-4.11.001 PC6): unregistered .factory/ path blocked
    // -----------------------------------------------------------------------

    #[test]
    fn test_unregistered_path_blocked() {
        todo!("S-13.01 Step 3 test-writer: inject payload with file_path '.factory/feature-deltas/F1-delta.md' (no registry match); assert HookResult::block_with_fix with code ARTIFACT_PATH_UNREGISTERED")
    }

    // -----------------------------------------------------------------------
    // AC-004 (BC-4.11.001 PC7): non-.factory/ path → early-exit Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_non_factory_path_no_registry_lookup() {
        todo!("S-13.01 Step 3 test-writer: inject payload with file_path 'src/lib.rs'; assert HookResult::Continue returned and read_file callback NOT invoked")
    }

    // -----------------------------------------------------------------------
    // AC-005 (BC-4.11.001 PC3/4/5): enforcement_level per entry
    // -----------------------------------------------------------------------

    #[test]
    fn test_enforcement_level_block_entry() {
        todo!("S-13.01 Step 3 test-writer: registry entry enforcement_level=block; path matches; assert HookResult::Continue (write proceeds)")
    }

    #[test]
    fn test_enforcement_level_warn_entry() {
        todo!("S-13.01 Step 3 test-writer: registry entry enforcement_level=warn; path matches; assert HookResult::Continue + hook.warn event + stderr written")
    }

    #[test]
    fn test_enforcement_level_advisory_entry() {
        todo!("S-13.01 Step 3 test-writer: registry entry enforcement_level=advisory; path matches; assert HookResult::Continue + host::log_debug called; no stderr")
    }

    // -----------------------------------------------------------------------
    // AC-006 (BC-4.11.001 EC-001/002): graceful degrade
    // -----------------------------------------------------------------------

    #[test]
    fn test_graceful_degrade_absent_registry() {
        todo!("S-13.01 Step 3 test-writer: read_file callback returns Err; assert HookResult::Continue + log_debug message")
    }

    #[test]
    fn test_graceful_degrade_malformed_registry() {
        todo!("S-13.01 Step 3 test-writer: read_file callback returns malformed YAML string; assert HookResult::Continue + log_error message")
    }

    // -----------------------------------------------------------------------
    // BC-4.11.001 EC-006: file_path absent from payload
    // -----------------------------------------------------------------------

    #[test]
    fn test_graceful_degrade_missing_file_path() {
        todo!("S-13.01 Step 3 test-writer: payload has no file_path field; assert HookResult::Continue + log_warn")
    }

    // -----------------------------------------------------------------------
    // VP-069 proptest: registry load never panics on any byte sequence
    // -----------------------------------------------------------------------

    // Proptest harnesses are defined here for test-writer in Step 3.
    // The harness structure below is intentionally left as a skeleton;
    // Step 3 fills in property assertions.

    #[cfg(test)]
    mod proptests {
        use super::*;

        #[test]
        fn prop_registry_parse_never_panics() {
            todo!("S-13.01 Step 3 test-writer: proptest harness — VP-069 Part A; arbitrary byte sequences; assert load_registry never panics (minimum 200 trials)")
        }

        #[test]
        fn prop_malformed_registry_produces_continue() {
            todo!("S-13.01 Step 3 test-writer: proptest harness — VP-069 Part B; malformed YAML input; assert hook_logic returns Continue")
        }

        #[test]
        fn prop_empty_registry_continues_for_all_paths() {
            todo!("S-13.01 Step 3 test-writer: proptest harness — VP-069 Part C; empty artifact list; all .factory/ paths return block_with_fix; non-.factory/ returns Continue")
        }
    }

    // -----------------------------------------------------------------------
    // VP-070 kani harness: path matching is pure and deterministic
    // -----------------------------------------------------------------------

    // Kani proof harnesses. Step 3 test-writer adds #[cfg(kani)] attribute
    // and kani::proof annotation.

    #[test]
    fn proof_match_path_is_deterministic() {
        todo!("S-13.01 Step 3 test-writer: kani harness — VP-070 Proof 1; same (path, registry) always returns same MatchResult; unwind 16")
    }

    #[test]
    fn proof_non_factory_path_always_allow() {
        todo!("S-13.01 Step 3 test-writer: kani harness — VP-070 Proof 2; any path not starting with '.factory/' returns MatchResult::NoMatch or early-exit Continue")
    }

    #[test]
    fn proof_empty_path_is_allow() {
        todo!("S-13.01 Step 3 test-writer: kani harness — VP-070 Proof 3; empty path returns Allow (Continue)")
    }

    #[test]
    fn proof_block_only_on_factory_path_with_block_level() {
        todo!("S-13.01 Step 3 test-writer: kani harness — VP-070 Proof 4; advisory-only registry never produces block_with_fix result")
    }
}
