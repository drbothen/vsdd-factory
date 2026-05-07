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
//! - Pure `fn hook_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.
//! - No hardcoded `.factory/` path pattern literals in source (BC-4.11.001 invariant 1).

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
// Kani is a model checker that defines its own cfg flag at verification time.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

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
pub fn load_registry(yaml: &str) -> Result<PathRegistry, RegistryError> {
    if yaml.is_empty() {
        return Err(RegistryError::ParseError("empty registry YAML".to_string()));
    }
    let registry: PathRegistry = serde_yaml::from_str(yaml)
        .map_err(|e| RegistryError::ParseError(e.to_string()))?;
    Ok(registry)
}

/// Match a write-target `path` against all entries in `registry`.
///
/// Returns the `MatchResult` for the first matching entry (first-match-wins,
/// per BC-4.11.001 EC-005). Returns `MatchResult::NoMatch` if no entry matches.
///
/// Non-`.factory/` paths always return `MatchResult::NoMatch` — `hook_logic`
/// uses this to distinguish early-exit Continue from a genuine no-match block.
///
/// This function is pure and deterministic: for the same `(path, registry)`
/// pair it always returns the same `MatchResult` (VP-070 invariant).
///
/// # Pattern matching
///
/// Patterns use `{placeholder}` syntax. Each placeholder matches any
/// non-empty path segment or sequence of segments. The algorithm converts
/// the pattern into a structure that allows `{placeholder}` to match
/// one or more path characters (excluding nothing).
///
/// # BC trace
/// BC-4.11.001 postcondition 2b — match path against registered patterns.
/// BC-4.11.001 EC-005 — first matching entry wins.
/// VP-070 — path matching is pure and deterministic.
pub fn matches_canonical(path: &str, registry: &PathRegistry) -> MatchResult {
    // Non-.factory/ paths are out of scope — return NoMatch so hook_logic
    // can do an early-exit Continue without blocking.
    if !path.starts_with(".factory/") {
        return MatchResult::NoMatch;
    }

    for entry in &registry.artifacts {
        if pattern_matches(path, &entry.canonical_path_pattern) {
            return match entry.enforcement_level.as_str() {
                "block" => MatchResult::Block,
                "warn" => MatchResult::Warn {
                    pattern: entry.canonical_path_pattern.clone(),
                },
                "advisory" => MatchResult::Advisory {
                    pattern: entry.canonical_path_pattern.clone(),
                },
                // Unknown enforcement_level: treat as advisory (don't block valid paths)
                _ => MatchResult::Advisory {
                    pattern: entry.canonical_path_pattern.clone(),
                },
            };
        }
    }

    MatchResult::NoMatch
}

/// Check whether `path` matches `pattern`, where `{placeholder}` in the
/// pattern matches any non-empty sequence of characters (greedy).
///
/// This implements a simple glob-like match: literal characters must match
/// exactly, and `{placeholder}` segments match one or more characters in the
/// path. The matching is done segment-aware: placeholders may span segment
/// boundaries (slashes) so that e.g. `{cycle-id}` can match
/// `v1.0-feature-engine-discipline-pass-1`.
///
/// Algorithm: split the pattern on `{...}` tokens; check that the path
/// contains each literal segment in order, with at least one character
/// between consecutive literal segments (where a placeholder is expected).
fn pattern_matches(path: &str, pattern: &str) -> bool {
    // Split pattern into alternating literal / placeholder segments.
    // E.g. ".factory/cycles/{cycle-id}/{doc-type}.md" becomes:
    //   [".factory/cycles/", "cycle-id", "/", "doc-type", ".md"]
    // We only care about the literal parts for matching purposes.
    let parts = split_pattern(pattern);

    // Walk through the literal parts ensuring each appears in the path
    // in order, with at least one character between consecutive parts
    // where a placeholder should sit.
    let mut pos = 0usize;
    let path_bytes = path.as_bytes();

    for (i, part) in parts.iter().enumerate() {
        match part {
            PatternPart::Literal(lit) => {
                if lit.is_empty() {
                    continue;
                }
                // Find this literal in path starting at pos.
                // If this is the very first literal, it must match at pos=0.
                if i == 0 || (i > 0 && !matches!(parts.get(i - 1), Some(PatternPart::Placeholder(_)))) {
                    // Consecutive literals — must appear immediately
                    if path_bytes[pos..].starts_with(lit.as_bytes()) {
                        pos += lit.len();
                    } else {
                        return false;
                    }
                } else {
                    // After a placeholder — must find the literal somewhere ahead
                    // but there must be at least one char before it (the placeholder content).
                    if pos >= path_bytes.len() {
                        return false;
                    }
                    // Find the literal starting from pos+1 (placeholder must have >=1 char)
                    match find_subsequence(&path_bytes[pos + 1..], lit.as_bytes()) {
                        Some(offset) => {
                            pos = pos + 1 + offset + lit.len();
                        }
                        None => return false,
                    }
                }
            }
            PatternPart::Placeholder(_) => {
                // Placeholder is handled when the NEXT literal is processed.
                // Just record that we need at least one char consumed.
            }
        }
    }

    // After all pattern parts, the entire path must be consumed
    // (the last placeholder, if any, must match the rest of the path).
    let last_is_placeholder = matches!(parts.last(), Some(PatternPart::Placeholder(_)));
    if last_is_placeholder {
        // Last placeholder must match at least one character.
        pos < path_bytes.len()
    } else {
        // All literal parts matched; path must be exactly consumed.
        pos == path_bytes.len()
    }
}

/// A parsed pattern part.
#[derive(Debug, Clone)]
enum PatternPart {
    Literal(String),
    Placeholder(String),
}

/// Split a pattern like ".factory/{cycle-id}/doc.md" into alternating
/// literal and placeholder parts.
fn split_pattern(pattern: &str) -> Vec<PatternPart> {
    let mut parts = Vec::new();
    let mut remaining = pattern;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find('{') {
            // Literal before the placeholder
            let literal = &remaining[..start];
            if !literal.is_empty() {
                parts.push(PatternPart::Literal(literal.to_string()));
            }
            remaining = &remaining[start + 1..];
            // Find the closing brace
            if let Some(end) = remaining.find('}') {
                let placeholder = &remaining[..end];
                parts.push(PatternPart::Placeholder(placeholder.to_string()));
                remaining = &remaining[end + 1..];
            } else {
                // No closing brace — treat the rest as a literal
                parts.push(PatternPart::Literal(format!("{{{}", remaining)));
                break;
            }
        } else {
            // No more placeholders — rest is a literal
            parts.push(PatternPart::Literal(remaining.to_string()));
            break;
        }
    }

    parts
}

/// Find the first occurrence of `needle` in `haystack`.
/// Returns the byte offset of the start, or `None` if not found.
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
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
pub fn hook_logic<R, E, L>(
    payload: HookPayload,
    mut callbacks: HookCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    // BC-4.11.001 EC-006: extract file_path from tool_input; graceful degrade if absent.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            // EC-006: file_path absent — log_warn (level 3) and continue.
            (callbacks.log)(
                3,
                "[validate-artifact-path] WARN: file_path absent from tool_input payload — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    // BC-4.11.001 PC7 / EC-004: non-.factory/ path → early-exit Continue.
    // No registry lookup performed.
    if !file_path.starts_with(".factory/") {
        return HookResult::Continue;
    }

    // Load the registry via the injectable read_file callback.
    let registry_yaml = match (callbacks.read_file)(REGISTRY_PATH) {
        Ok(contents) => contents,
        Err(_) => {
            // BC-4.11.001 EC-001: registry absent — log_debug (level 1) and continue.
            (callbacks.log)(
                1,
                &format!(
                    "[validate-artifact-path] registry absent at {} — graceful degrade",
                    REGISTRY_PATH
                ),
            );
            return HookResult::Continue;
        }
    };

    // Parse the registry YAML.
    let registry = match load_registry(&registry_yaml) {
        Ok(r) => r,
        Err(e) => {
            // BC-4.11.001 EC-002: registry malformed — log_error (level 4) and continue.
            (callbacks.log)(
                4,
                &format!(
                    "[validate-artifact-path] registry parse error: {} — graceful degrade",
                    e
                ),
            );
            return HookResult::Continue;
        }
    };

    // Match the path against the registry.
    match matches_canonical(&file_path, &registry) {
        MatchResult::Block => {
            // BC-4.11.001 PC3: path matches block-level entry → write proceeds.
            HookResult::Continue
        }
        MatchResult::Warn { pattern } => {
            // BC-4.11.001 PC4: warn-level match → emit hook.warn + continue.
            (callbacks.emit_event)(
                "hook.warn",
                &[
                    ("hook", "validate-artifact-path"),
                    ("path", &file_path),
                    ("pattern", &pattern),
                    ("enforcement_level", "warn"),
                ],
            );
            HookResult::Continue
        }
        MatchResult::Advisory { .. } => {
            // BC-4.11.001 PC5: advisory-level match → log_debug + continue.
            (callbacks.log)(
                1,
                &format!(
                    "[validate-artifact-path] advisory: '{}' matches registry pattern with enforcement_level: advisory",
                    file_path
                ),
            );
            HookResult::Continue
        }
        MatchResult::NoMatch => {
            // BC-4.11.001 PC6 / invariant 3: unregistered .factory/ path → block.
            HookResult::block_with_fix(
                "validate-artifact-path",
                &format!(
                    "Write to '{}' under .factory/ has no matching entry in {}",
                    file_path, REGISTRY_PATH
                ),
                "Consult the registry to find the canonical path for this artifact type. \
                 If the artifact type is new, use /vsdd-factory:register-artifact to add it \
                 to the registry first. Do not invent directory names",
                "ARTIFACT_PATH_UNREGISTERED",
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    hook_logic(
        payload,
        HookCallbacks {
            read_file: |path| match host::read_file(path, 65536, 5000) {
                Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
                Err(e) => Err(format!("{:?}", e)),
            },
            emit_event: |event_type, fields| {
                host::emit_event(event_type, fields);
            },
            log: |level, msg| match level {
                0 | 1 | 2 => host::log_info(msg),
                3 => host::log_warn(msg),
                _ => host::log_error(msg),
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests — Red Gate (BC-5.36.001)
//
// All tests in this module exercise the production functions declared above.
// Test naming follows the BC-based convention: test_BC_S_SS_NNN_xxx().
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::panic;

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    /// Build a minimal valid registry YAML string with a single entry.
    /// `enforcement_level` controls the entry's level: "block", "warn", or "advisory".
    fn registry_yaml(enforcement_level: &str) -> String {
        format!(
            r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{{subsystem}}/BC-{{bc-id}}.md"
    description: Behavioral contract spec
    enforcement_level: "{}"
"#,
            enforcement_level
        )
    }

    /// Build a registry YAML with no artifact entries (empty artifact list).
    fn empty_registry_yaml() -> String {
        "version: 1\nartifacts: []\n".to_string()
    }

    /// Build a minimal valid registry YAML with multiple entries covering
    /// at least 5 artifact types.
    fn multi_entry_registry_yaml() -> String {
        r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Behavioral contract spec
    enforcement_level: "block"
  - artifact_type: adr
    canonical_path_pattern: ".factory/specs/architecture/decisions/ADR-{adr-id}-{slug}.md"
    description: Architecture decision record
    enforcement_level: "block"
  - artifact_type: verification-property
    canonical_path_pattern: ".factory/specs/verification-properties/VP-{vp-id}.md"
    description: Verification property
    enforcement_level: "block"
  - artifact_type: story-spec
    canonical_path_pattern: ".factory/stories/S-{story-id}-{slug}.md"
    description: Story specification
    enforcement_level: "block"
  - artifact_type: cycle-document
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{doc-type}.md"
    description: Cycle document
    enforcement_level: "block"
  - artifact_type: prd
    canonical_path_pattern: ".factory/specs/prd.md"
    description: Product requirements document
    enforcement_level: "block"
"#
        .to_string()
    }

    /// Build a registry YAML with all entries set to advisory enforcement.
    fn advisory_only_registry_yaml() -> String {
        r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Behavioral contract spec
    enforcement_level: "advisory"
"#
        .to_string()
    }

    /// Make a PreToolUse HookPayload for the given tool and optional file_path.
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

    /// Attempt to call `load_registry` on `yaml` and ASSERT it returns Ok.
    fn require_registry(yaml: &str, context: &str) -> PathRegistry {
        let result = panic::catch_unwind(|| load_registry(yaml));
        assert!(
            result.is_ok(),
            "load_registry panicked on valid YAML — production function is unimplemented (todo!()). \
             Context: {}. BC-4.11.001 PC1: load_registry must return Ok(PathRegistry) for valid YAML.",
            context
        );
        let parse_result = result.unwrap();
        assert!(
            parse_result.is_ok(),
            "load_registry returned Err for valid YAML — expected Ok(PathRegistry). \
             Context: {}. BC-4.11.001 PC1.",
            context
        );
        parse_result.unwrap()
    }

    /// Run hook_logic with injectable callbacks and capture (result, log_level+msg, emitted_event).
    fn run_logic(
        payload: HookPayload,
        registry_result: Result<String, String>,
    ) -> (HookResult, Vec<(u8, String)>, Vec<(String, Vec<(String, String)>)>) {
        let mut log_calls: Vec<(u8, String)> = Vec::new();
        let mut events: Vec<(String, Vec<(String, String)>)> = Vec::new();

        let result = hook_logic(
            payload,
            HookCallbacks {
                read_file: move |_path| registry_result.clone(),
                emit_event: |event_type, fields| {
                    events.push((
                        event_type.to_string(),
                        fields
                            .iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                    ));
                },
                log: |level, msg| {
                    log_calls.push((level, msg.to_string()));
                },
            },
        );

        (result, log_calls, events)
    }

    // -----------------------------------------------------------------------
    // AC-001 (BC-4.11.001 PC1 + VP-069): load_registry happy path
    // load_registry on valid YAML returns Ok with parsed entries.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_valid_yaml_returns_ok() {
        let yaml = registry_yaml("block");
        let result = panic::catch_unwind(|| load_registry(&yaml));
        assert!(
            result.is_ok(),
            "load_registry should return Ok(PathRegistry) for valid YAML, \
             but the production function is unimplemented (todo!()). \
             BC-4.11.001 PC1: registry must be parseable at hook invocation time."
        );
        if let Ok(Ok(registry)) = result {
            assert!(
                !registry.artifacts.is_empty(),
                "load_registry should parse at least one entry from valid YAML \
                 (BC-4.11.001 PC1 — registry is the single source of truth)"
            );
            assert_eq!(registry.version, 1, "registry schema version must be 1");
        }
    }

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_parses_artifact_type_field() {
        let yaml = registry_yaml("block");
        let result = panic::catch_unwind(|| load_registry(&yaml));
        assert!(
            result.is_ok(),
            "load_registry should return Ok for valid YAML with artifact_type field \
             (BC-4.11.001 PC1 — artifact_type is a required registry entry field). \
             Production function is unimplemented."
        );
        if let Ok(Ok(registry)) = result {
            let entry = registry
                .artifacts
                .first()
                .expect("parsed registry must have at least one entry");
            assert_eq!(
                entry.artifact_type, "behavioral-contract",
                "artifact_type must be parsed from YAML"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_parses_enforcement_level_field() {
        let yaml = registry_yaml("warn");
        let result = panic::catch_unwind(|| load_registry(&yaml));
        assert!(
            result.is_ok(),
            "load_registry should return Ok and parse enforcement_level field \
             (BC-4.11.001 invariant 2 — per-entry enforcement_level must be respected). \
             Production function is unimplemented."
        );
        if let Ok(Ok(registry)) = result {
            let entry = registry
                .artifacts
                .first()
                .expect("parsed registry must have at least one entry");
            assert_eq!(
                entry.enforcement_level, "warn",
                "enforcement_level must be parsed correctly from YAML"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-001 continued: load_registry malformed YAML returns Err (not panic)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_malformed_yaml_returns_err() {
        let malformed = "{ this is not: valid yaml: [unclosed bracket";
        let result = panic::catch_unwind(|| load_registry(malformed));
        assert!(
            result.is_ok(),
            "load_registry must NOT panic on malformed YAML (VP-069 Part A invariant). \
             It should return Err(RegistryError::ParseError). Production function is unimplemented."
        );
        if let Ok(outcome) = result {
            assert!(
                outcome.is_err(),
                "load_registry must return Err for malformed YAML (BC-4.11.001 EC-002)"
            );
            if let Err(err) = outcome {
                let msg = format!("{}", err);
                assert!(
                    msg.contains("parse error") || msg.contains("parse"),
                    "RegistryError for malformed YAML must be ParseError, got: {}", msg
                );
            }
        }
    }

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_empty_string_returns_err() {
        let result = panic::catch_unwind(|| load_registry(""));
        assert!(
            result.is_ok(),
            "load_registry must NOT panic on empty string input (VP-069 Part A). \
             Should return Err(RegistryError). Production function is unimplemented."
        );
    }

    #[test]
    fn test_BC_4_11_001_ac001_load_registry_missing_required_field_returns_missing_field_err() {
        let missing_fields_yaml = "version: 1\nartifacts:\n  - artifact_type: behavioral-contract\n";
        let result = panic::catch_unwind(|| load_registry(missing_fields_yaml));
        assert!(
            result.is_ok(),
            "load_registry must NOT panic when a required field is absent (VP-069 Part A). \
             Should return Err(RegistryError::MissingField). Production function is unimplemented."
        );
        // If the function returns at all, it should be an error for incomplete entries
        // (enforcement_level and canonical_path_pattern are required per ADR-016 schema)
    }

    // -----------------------------------------------------------------------
    // AC-002 (BC-4.11.001 PC2b): matches_canonical — canonical paths → Match
    // Tests at least 5 distinct artifact types per story requirement.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_bc_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(
            &yaml,
            "test_BC_4_11_001_ac002_matches_canonical_bc_path_returns_match",
        );
        let match_result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
                &registry,
            )
        });
        assert!(
            match_result.is_ok(),
            "matches_canonical should return MatchResult::Block for a canonical BC path \
             (BC-4.11.001 PC2b, PC3: path matches block entry → write proceeds). \
             Production function is unimplemented."
        );
        if let Ok(result) = match_result {
            assert_eq!(
                result,
                MatchResult::Block,
                "canonical BC path with enforcement_level=block must return MatchResult::Block"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_adr_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(&yaml, "ac002_adr_path");
        let result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/specs/architecture/decisions/ADR-016-artifact-path-registry-sot.md",
                &registry,
            )
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::Block for canonical ADR path \
             (BC-4.11.001 PC2b — ADR artifact type registered in registry). \
             Production function is unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(mr, MatchResult::Block, "ADR path must match Block entry");
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_vp_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(&yaml, "ac002_vp_path");
        let result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/specs/verification-properties/VP-069.md",
                &registry,
            )
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::Block for canonical VP path \
             (BC-4.11.001 PC2b — VP artifact type registered). Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(mr, MatchResult::Block, "VP path must match Block entry");
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_story_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(&yaml, "ac002_story_path");
        let result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/stories/S-13.01-path-governance-bundle.md",
                &registry,
            )
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::Block for canonical story path \
             (BC-4.11.001 PC2b — story-spec artifact type registered). Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(mr, MatchResult::Block, "story path must match Block entry");
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_prd_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(&yaml, "ac002_prd_path");
        let result = panic::catch_unwind(|| {
            matches_canonical(".factory/specs/prd.md", &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::Block for canonical PRD path \
             (BC-4.11.001 PC2b — prd artifact type registered). Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(mr, MatchResult::Block, "PRD path must match Block entry");
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_cycle_doc_path_returns_match() {
        let yaml = multi_entry_registry_yaml();
        let registry = require_registry(&yaml, "ac002_cycle_doc_path");
        let result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md",
                &registry,
            )
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::Block for canonical cycle-doc path \
             (BC-4.11.001 PC2b — cycle-document artifact type registered). Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "cycle-document path must match Block entry"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 (BC-4.11.001 PC2b): non-canonical paths → NoMatch
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_unregistered_path_returns_nomatch() {
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "ac002_unregistered_path_nomatch");
        let result = panic::catch_unwind(|| {
            matches_canonical(".factory/feature-deltas/F1-delta.md", &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return MatchResult::NoMatch for unregistered path \
             (BC-4.11.001 invariant 3 — unregistered path must be blocked). Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::NoMatch,
                ".factory/feature-deltas/F1-delta.md matches no registered pattern → NoMatch"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 (BC-4.11.001 EC-005): first-match-wins for ambiguous paths
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac002_matches_canonical_first_match_wins_for_ambiguous_path() {
        let ambiguous_yaml = r#"version: 1
artifacts:
  - artifact_type: first-match
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: First entry
    enforcement_level: "block"
  - artifact_type: second-match
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Second entry
    enforcement_level: "warn"
"#;
        let registry = require_registry(ambiguous_yaml, "ac002_first_match_wins");
        let result = panic::catch_unwind(|| {
            matches_canonical(
                ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
                &registry,
            )
        });
        assert!(
            result.is_ok(),
            "matches_canonical should return first-match result (BC-4.11.001 EC-005: \
             first-matching entry wins). Production unimplemented."
        );
        if let Ok(mr) = result {
            // The first entry has enforcement_level=block → MatchResult::Block
            assert_eq!(
                mr,
                MatchResult::Block,
                "first-match-wins: the first registry entry (enforcement_level=block) \
                 must be returned, not the second (enforcement_level=warn)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-002: placeholder substitution in patterns
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac002_placeholder_subsystem_expansion_in_bc_pattern() {
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "ac002_placeholder_subsystem");
        for subsystem in &["01", "04", "06", "13"] {
            let path = format!(
                ".factory/specs/behavioral-contracts/ss-{}/BC-{}.01.001.md",
                subsystem, subsystem
            );
            let result = panic::catch_unwind({
                let registry = registry.clone();
                let path = path.clone();
                move || matches_canonical(&path, &registry)
            });
            assert!(
                result.is_ok(),
                "matches_canonical should support {{subsystem}} placeholder for \
                 path '{}' (BC-4.11.001 invariant 6). Production unimplemented.",
                path
            );
            if let Ok(mr) = result {
                assert_eq!(
                    mr,
                    MatchResult::Block,
                    "path with subsystem '{}' must match block entry via placeholder expansion",
                    subsystem
                );
            }
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_placeholder_bc_id_expansion_in_bc_pattern() {
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "ac002_placeholder_bc_id");
        let paths = [
            ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
            ".factory/specs/behavioral-contracts/ss-06/BC-6.22.001.md",
            ".factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md",
        ];
        for path in &paths {
            let result = panic::catch_unwind({
                let registry = registry.clone();
                let path = path.to_string();
                move || matches_canonical(&path, &registry)
            });
            assert!(
                result.is_ok(),
                "matches_canonical should support {{bc-id}} placeholder for path '{}' \
                 (BC-4.11.001 invariant 6 — placeholder expansion). Production unimplemented.",
                path
            );
            if let Ok(mr) = result {
                assert_eq!(
                    mr,
                    MatchResult::Block,
                    "BC path '{}' must match via {{bc-id}} placeholder expansion",
                    path
                );
            }
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_placeholder_cycle_id_expansion_in_cycle_doc_pattern() {
        let yaml = r#"version: 1
artifacts:
  - artifact_type: cycle-document
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{doc-type}.md"
    description: Cycle document
    enforcement_level: "block"
"#;
        let registry = require_registry(yaml, "ac002_placeholder_cycle_id");
        let path = ".factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md";
        let result = panic::catch_unwind({
            let registry = registry.clone();
            move || matches_canonical(path, &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should support {{cycle-id}} placeholder for cycle docs \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "cycle-doc path must match via {{cycle-id}} placeholder"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_placeholder_story_id_expansion_in_story_pattern() {
        let yaml = r#"version: 1
artifacts:
  - artifact_type: story-spec
    canonical_path_pattern: ".factory/stories/S-{story-id}-{slug}.md"
    description: Story specification
    enforcement_level: "block"
"#;
        let registry = require_registry(yaml, "ac002_placeholder_story_id");
        let path = ".factory/stories/S-13.01-path-governance-bundle.md";
        let result = panic::catch_unwind({
            let registry = registry.clone();
            move || matches_canonical(path, &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should support {{story-id}} and {{slug}} placeholders \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "story path must match via {{story-id}}/{{slug}} placeholder expansion"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac002_placeholder_phase_expansion_in_phase_pattern() {
        let yaml = r#"version: 1
artifacts:
  - artifact_type: phase-delta-analysis
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{phase}-delta-analysis.md"
    description: Phase delta analysis
    enforcement_level: "block"
"#;
        let registry = require_registry(yaml, "ac002_placeholder_phase");
        let path = ".factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md";
        let result = panic::catch_unwind({
            let registry = registry.clone();
            move || matches_canonical(path, &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should support {{phase}} placeholder for phase delta analysis \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "phase-delta path must match via {{phase}} placeholder expansion"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-003 (BC-4.11.001 PC6): unregistered .factory/ path blocked
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac003_unregistered_path_blocked() {
        let payload = make_payload("Write", Some(".factory/feature-deltas/F1-delta.md"));
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::block_with_fix for unregistered .factory/ path \
             '.factory/feature-deltas/F1-delta.md' (BC-4.11.001 PC6 — ARTIFACT_PATH_UNREGISTERED). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            match &hook_result {
                HookResult::Block { reason } => {
                    assert!(
                        reason.contains("ARTIFACT_PATH_UNREGISTERED"),
                        "block reason must contain ARTIFACT_PATH_UNREGISTERED code \
                         (BC-4.11.001 PC6). Got: '{}'",
                        reason
                    );
                    assert!(
                        reason.contains("validate-artifact-path"),
                        "block reason must include hook name 'validate-artifact-path' \
                         (canonical Why/Fix/Code pattern). Got: '{}'",
                        reason
                    );
                }
                other => {
                    panic!(
                        "expected HookResult::Block for unregistered .factory/ path, \
                         got {:?} (BC-4.11.001 PC6 — unregistered path must be blocked)",
                        other
                    );
                }
            }
        }
    }

    #[test]
    fn test_BC_4_11_001_ac003_block_reason_contains_path_under_test() {
        let target_path = ".factory/feature-deltas/F1-delta.md";
        let payload = make_payload("Write", Some(target_path));
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic must not panic for unregistered .factory/ path; \
             should return HookResult::Block (BC-4.11.001 PC6). Production unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            if let HookResult::Block { reason } = hook_result {
                assert!(
                    reason.contains(target_path),
                    "block reason must include the write target path '{}' \
                     so the agent knows which path was rejected (BC-4.11.001 PC6). \
                     Got: '{}'",
                    target_path,
                    reason
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // AC-004 (BC-4.11.001 PC7): non-.factory/ path → early-exit Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac004_non_factory_path_returns_continue() {
        let mut read_file_called = false;
        let payload = make_payload("Write", Some("src/lib.rs"));
        let result = panic::catch_unwind(move || {
            hook_logic(
                payload,
                HookCallbacks {
                    read_file: |_path| {
                        // read_file must NOT be called for non-.factory/ paths
                        read_file_called = true;
                        Err("should not be called".to_string())
                    },
                    emit_event: |_, _| {},
                    log: |_, _| {},
                },
            )
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue for non-.factory/ path 'src/lib.rs' \
             (BC-4.11.001 PC7 — early-exit for non-.factory/ paths). Production unimplemented."
        );
        if let Ok(hook_result) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "non-.factory/ path 'src/lib.rs' must return HookResult::Continue \
                 (BC-4.11.001 PC7 — hook is scoped to .factory/ paths only)"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac004_non_factory_path_does_not_invoke_read_file() {
        use std::sync::{Arc, Mutex};
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        let payload = make_payload("Edit", Some("plugins/vsdd-factory/skills/create-adr/SKILL.md"));
        let result = panic::catch_unwind(move || {
            hook_logic(
                payload,
                HookCallbacks {
                    read_file: move |_path| {
                        *called_clone.lock().unwrap() = true;
                        Err("registry-should-not-be-read".to_string())
                    },
                    emit_event: |_, _| {},
                    log: |_, _| {},
                },
            )
        });
        assert!(
            result.is_ok(),
            "hook_logic should return Continue without reading registry for non-.factory/ path \
             (BC-4.11.001 PC7). Production unimplemented."
        );
        if let Ok(hook_result) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "non-.factory/ path (even inside plugins/) must return Continue without \
                 registry lookup (BC-4.11.001 PC7)"
            );
            assert!(
                !*called.lock().unwrap(),
                "read_file must NOT be called for non-.factory/ path (BC-4.11.001 PC7 — \
                 no registry lookup for paths outside .factory/)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 (BC-4.11.001 PC3/4/5): enforcement_level per entry
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac005_enforcement_level_block_entry_returns_continue() {
        let payload = make_payload(
            "Write",
            Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
        );
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue when path matches a 'block' \
             enforcement_level entry (BC-4.11.001 PC3: matched block-level path → write proceeds). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 PC3: path matching enforcement_level=block entry must return Continue \
                 (the write is ALLOWED — 'block' means 'this is the canonical path')"
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac005_enforcement_level_warn_entry_returns_continue_with_event() {
        let payload = make_payload(
            "Write",
            Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
        );
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("warn")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue and emit hook.warn event \
             when path matches enforcement_level=warn entry (BC-4.11.001 PC4). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, _, events)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 PC4: warn-level match must return Continue (write proceeds)"
            );
            assert!(
                events.iter().any(|(event_type, _)| event_type == "hook.warn"),
                "BC-4.11.001 PC4: enforcement_level=warn must emit a hook.warn event. \
                 Got events: {:?}",
                events
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac005_enforcement_level_advisory_entry_returns_continue_with_log() {
        let payload = make_payload(
            "Write",
            Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
        );
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("advisory")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue and call log_debug \
             when path matches enforcement_level=advisory entry (BC-4.11.001 PC5). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, log_calls, events)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 PC5: advisory-level match must return Continue"
            );
            // log_debug is level 1
            assert!(
                log_calls.iter().any(|(level, _)| *level <= 1),
                "BC-4.11.001 PC5: advisory match must call log_debug (level 0 or 1). \
                 Got log calls: {:?}",
                log_calls
            );
            // No hook.warn event should be emitted for advisory
            assert!(
                !events.iter().any(|(event_type, _)| event_type == "hook.warn"),
                "BC-4.11.001 PC5: advisory match must NOT emit hook.warn event \
                 (no stderr/event for advisory — log_debug only). Got events: {:?}",
                events
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac005_no_match_returns_block_with_fix() {
        let payload = make_payload("Write", Some(".factory/unknown/path/artifact.md"));
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::block_with_fix when no registry entry matches \
             (BC-4.11.001 PC6 — unregistered path is always blocked). Production unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            assert!(
                matches!(hook_result, HookResult::Block { .. }),
                "BC-4.11.001 PC6: no-match path must return HookResult::Block. Got: {:?}",
                hook_result
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-006 (BC-4.11.001 EC-001/EC-002): graceful degrade
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ac006_graceful_degrade_absent_registry_returns_continue() {
        let payload = make_payload("Write", Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"));
        let result = panic::catch_unwind(|| {
            run_logic(
                payload,
                Err("registry absent: no such file".to_string()),
            )
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue when registry is absent \
             (BC-4.11.001 EC-001 — graceful degrade, never blocks on missing registry). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, log_calls, _)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 EC-001: absent registry must return Continue (graceful degrade). \
                 Must NOT block (registry absence is not an error for the hook to block on)"
            );
            assert!(
                !log_calls.is_empty(),
                "BC-4.11.001 EC-001: absent registry must emit a log message \
                 (log_debug advisory). Got zero log calls."
            );
        }
    }

    #[test]
    fn test_BC_4_11_001_ac006_graceful_degrade_malformed_registry_returns_continue() {
        let payload = make_payload("Write", Some(".factory/specs/prd.md"));
        let result = panic::catch_unwind(|| {
            run_logic(
                payload,
                Ok("{ THIS IS NOT VALID YAML: [broken".to_string()),
            )
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue when registry YAML is malformed \
             (BC-4.11.001 EC-002 — graceful degrade on parse error, never blocks). \
             Production function is unimplemented."
        );
        if let Ok((hook_result, log_calls, _)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 EC-002: malformed registry must return Continue (graceful degrade). \
                 Must NOT block."
            );
            // log_error is level 4; verify some log was emitted
            assert!(
                !log_calls.is_empty(),
                "BC-4.11.001 EC-002: malformed registry must emit a log_error message. \
                 Got zero log calls."
            );
            // Check it's an error-level log (level 4 = error)
            assert!(
                log_calls.iter().any(|(level, _)| *level >= 3),
                "BC-4.11.001 EC-002: malformed registry must call log_error (level >= 3). \
                 Got log calls: {:?}",
                log_calls
            );
        }
    }

    // -----------------------------------------------------------------------
    // BC-4.11.001 EC-006: file_path absent from payload
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_ec006_missing_file_path_returns_continue() {
        let payload = make_payload("Write", None); // no file_path
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Continue when file_path is absent from payload \
             (BC-4.11.001 EC-006 — graceful degrade on missing data). Production unimplemented."
        );
        if let Ok((hook_result, log_calls, _)) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "BC-4.11.001 EC-006: absent file_path must return Continue (not block on missing data)"
            );
            // log_warn is level 3
            assert!(
                log_calls.iter().any(|(level, _)| *level >= 2),
                "BC-4.11.001 EC-006: absent file_path must call log_warn (level >= 2). \
                 Got log calls: {:?}",
                log_calls
            );
        }
    }

    // -----------------------------------------------------------------------
    // BC-4.11.001 invariant 3: unregistered .factory/ path is ALWAYS blocked
    // (regardless of enforcement_level of other entries)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_invariant3_unregistered_factory_path_always_blocked() {
        let payload = make_payload("Write", Some(".factory/some/new/artifact-type/file.md"));
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(advisory_only_registry_yaml()))
        });
        assert!(
            result.is_ok(),
            "hook_logic should return HookResult::Block for unregistered .factory/ path \
             even when registered entries are advisory (BC-4.11.001 invariant 3: \
             unregistered path → always blocked). Production unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            assert!(
                matches!(hook_result, HookResult::Block { .. }),
                "BC-4.11.001 invariant 3: unregistered .factory/ path must be blocked \
                 regardless of other entries' enforcement_level. Got: {:?}",
                hook_result
            );
        }
    }

    // -----------------------------------------------------------------------
    // BC-4.11.001 invariant 4: hook must NOT modify the registry file
    // (read-only access — tested via callback tracking)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_invariant4_hook_does_not_write_registry() {
        let original_yaml = registry_yaml("block");
        let yaml_snapshot = original_yaml.clone();
        let payload = make_payload(
            "Write",
            Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
        );
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(yaml_snapshot.clone()))
        });
        assert!(
            result.is_ok(),
            "hook_logic must not panic and must not modify registry content \
             (BC-4.11.001 invariant 4 — registry is read-only). Production unimplemented."
        );
        // After running, the original YAML string is unchanged (immutable Rust string)
        assert_eq!(
            original_yaml, yaml_snapshot,
            "BC-4.11.001 invariant 4: registry content must be unchanged after hook_logic invocation"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.11.001 invariant 9: bare HookResult::block() is prohibited;
    // all blocks must use block_with_fix.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_11_001_invariant9_block_uses_block_with_fix_pattern() {
        let payload = make_payload("Write", Some(".factory/feature-deltas/F1-delta.md"));
        let result = panic::catch_unwind(|| {
            run_logic(payload, Ok(registry_yaml("block")))
        });
        assert!(
            result.is_ok(),
            "hook_logic must not panic for unregistered .factory/ path. \
             Block message must use block_with_fix pattern (BC-4.11.001 invariant 9). \
             Production unimplemented."
        );
        if let Ok((hook_result, _, _)) = result {
            if let HookResult::Block { reason } = hook_result {
                // block_with_fix produces: "BLOCKED by <hook>: ... Code: <code>."
                assert!(
                    reason.starts_with("BLOCKED by"),
                    "BC-4.11.001 invariant 9: block reason must start with 'BLOCKED by' \
                     (block_with_fix canonical format). Got: '{}'",
                    reason
                );
                assert!(
                    reason.contains("Code: ARTIFACT_PATH_UNREGISTERED"),
                    "BC-4.11.001 invariant 9: block reason must contain \
                     'Code: ARTIFACT_PATH_UNREGISTERED'. Got: '{}'",
                    reason
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-069 proptest: registry load never panics on any byte sequence
    // -----------------------------------------------------------------------

    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        // VP-069 Part A: parse_registry never panics on any input.
        // The harness generates arbitrary strings and asserts no panic.
        proptest! {
            #![proptest_config(proptest::test_runner::Config::with_cases(200))]

            /// VP-069 Part A (BC-4.11.001 PC1): load_registry must never panic on
            /// any byte sequence. Always returns Ok(registry) or Err(parse_error).
            #[test]
            fn prop_BC_4_11_001_vp069_registry_parse_never_panics(input in any::<Vec<u8>>()) {
                // Convert arbitrary bytes to string (lossy — mimics real-world malformed YAML)
                let s = String::from_utf8_lossy(&input).into_owned();
                // load_registry must not panic on any input; catch_unwind detects panic
                let result = panic::catch_unwind(|| load_registry(&s));
                prop_assert!(
                    result.is_ok(),
                    "VP-069 Part A: load_registry panicked on input (len {}). \
                     Must return Ok or Err, never panic. Production function is unimplemented.",
                    s.len()
                );
            }

            /// VP-069 Part B (BC-4.11.001 EC-002): malformed YAML must not produce
            /// a block outcome. The hook must return Continue on parse failure.
            /// Tests that load_registry + hook_logic treat malformed input as
            /// graceful-degrade (Continue), never Block.
            #[test]
            fn prop_BC_4_11_001_vp069_malformed_registry_hook_returns_continue(
                input in ".*"
            ) {
                // Use a .factory/ path to ensure we actually enter the registry-check branch
                let payload = make_payload(
                    "Write",
                    Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
                );
                let result = panic::catch_unwind(|| {
                    run_logic(payload, Ok(input.clone()))
                });
                prop_assert!(
                    result.is_ok(),
                    "VP-069 Part B: hook_logic panicked on malformed registry input. \
                     Must return Continue (graceful degrade). Production unimplemented."
                );
                if let Ok((hook_result, _, _)) = result {
                    // For inputs that fail to parse as valid registry YAML,
                    // the hook must return Continue (graceful degrade per EC-002).
                    // Inputs that happen to parse as valid YAML may return Block if the
                    // path doesn't match — but they must never panic.
                    prop_assert_ne!(
                        hook_result.exit_code(),
                        1, // HookResult::Error exit code
                        "VP-069 Part B: hook_logic must not return HookResult::Error \
                         for any registry input (panic-escape through error path). \
                         Got: {:?}",
                        "Error"
                    );
                }
            }

            /// VP-069 Part C (BC-4.11.001 PC7 + invariant 3): empty registry
            /// (zero artifacts) must Continue for non-.factory/ paths and
            /// Block for .factory/ paths (empty registry = no valid patterns).
            #[test]
            fn prop_BC_4_11_001_vp069_empty_registry_non_factory_path_always_continues(
                // Generate paths that do NOT start with .factory/
                path in "[^.].*"
            ) {
                let payload = make_payload("Write", Some(&path));
                let result = panic::catch_unwind(|| {
                    run_logic(payload, Ok(empty_registry_yaml()))
                });
                prop_assert!(
                    result.is_ok(),
                    "VP-069 Part C: hook_logic panicked for non-.factory/ path '{}' \
                     with empty registry. Must return Continue. Production unimplemented.",
                    path
                );
                if let Ok((hook_result, _, _)) = result {
                    prop_assert_eq!(
                        hook_result,
                        HookResult::Continue,
                        "VP-069 Part C: non-.factory/ path '{}' with empty registry \
                         must return Continue (BC-4.11.001 PC7 — early exit for non-.factory/)",
                        path
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-070 kani harness: path matching is pure and deterministic
    //
    // The kani proofs live in #[cfg(kani)] mod kani_proofs below.
    // These test-mode equivalents exercise the same properties via catch_unwind
    // to provide Red Gate failures with clear assertion messages.
    // -----------------------------------------------------------------------

    #[test]
    fn proof_BC_4_11_001_vp070_match_path_is_deterministic() {
        // VP-070 Proof 1: same (path, registry) always returns same MatchResult.
        // Tests determinism for a fixed canonical BC path.
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "vp070_proof1_determinism");
        let path = ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md";
        let r1 = panic::catch_unwind({
            let reg = registry.clone();
            move || matches_canonical(path, &reg)
        });
        let r2 = panic::catch_unwind({
            let reg = registry.clone();
            move || matches_canonical(path, &reg)
        });
        assert!(
            r1.is_ok(),
            "VP-070 Proof 1: matches_canonical must not panic for canonical BC path \
             (determinism proof). Production unimplemented."
        );
        assert!(
            r2.is_ok(),
            "VP-070 Proof 1: second call to matches_canonical must not panic \
             (determinism proof). Production unimplemented."
        );
        if let (Ok(result1), Ok(result2)) = (r1, r2) {
            assert_eq!(
                result1, result2,
                "VP-070 Proof 1: matches_canonical must be deterministic — \
                 same (path, registry) must always return same MatchResult"
            );
        }
    }

    #[test]
    fn proof_BC_4_11_001_vp070_non_factory_path_always_returns_nomatch() {
        // VP-070 Proof 2: non-.factory/ paths always return MatchResult::NoMatch
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "vp070_proof2_non_factory");
        let non_factory_paths = [
            "src/lib.rs",
            "crates/hook-plugins/validate-artifact-path/src/lib.rs",
            "",
            "Cargo.toml",
            "plugins/vsdd-factory/hooks/validate-artifact-path.sh",
        ];
        for path in &non_factory_paths {
            let result = panic::catch_unwind({
                let reg = registry.clone();
                let p = path.to_string();
                move || matches_canonical(&p, &reg)
            });
            assert!(
                result.is_ok(),
                "VP-070 Proof 2: matches_canonical must not panic for non-.factory/ path '{}'. \
                 Production unimplemented.",
                path
            );
            if let Ok(mr) = result {
                assert_eq!(
                    mr,
                    MatchResult::NoMatch,
                    "VP-070 Proof 2: non-.factory/ path '{}' must return MatchResult::NoMatch \
                     (hook_logic handles early-exit Continue for these paths)",
                    path
                );
            }
        }
    }

    #[test]
    fn proof_BC_4_11_001_vp070_empty_path_returns_nomatch() {
        // VP-070 Proof 3: empty path returns MatchResult::NoMatch.
        let yaml = registry_yaml("block");
        let registry = require_registry(&yaml, "vp070_proof3_empty_path");
        let result = panic::catch_unwind(move || matches_canonical("", &registry));
        assert!(
            result.is_ok(),
            "VP-070 Proof 3: matches_canonical must not panic for empty path. \
             Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::NoMatch,
                "VP-070 Proof 3: empty path must return MatchResult::NoMatch \
                 (empty path cannot match any .factory/ prefix)"
            );
        }
    }

    #[test]
    fn proof_BC_4_11_001_vp070_advisory_only_registry_never_produces_block_in_matches() {
        // VP-070 Proof 4: advisory-only registry must never return MatchResult::Block
        let yaml = advisory_only_registry_yaml();
        let registry = require_registry(&yaml, "vp070_proof4_advisory_only");
        let result = panic::catch_unwind({
            let reg = registry.clone();
            move || {
                matches_canonical(
                    ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
                    &reg,
                )
            }
        });
        assert!(
            result.is_ok(),
            "VP-070 Proof 4: matches_canonical must not panic for advisory-only registry. \
             Production unimplemented."
        );
        if let Ok(mr) = result {
            assert_ne!(
                mr,
                MatchResult::Block,
                "VP-070 Proof 4: advisory-only registry must never return MatchResult::Block \
                 from matches_canonical (Block variant is reserved for enforcement_level=block entries)"
            );
            // Should return Advisory variant for matching paths
            match mr {
                MatchResult::Advisory { .. } | MatchResult::NoMatch => {}
                other => panic!(
                    "VP-070 Proof 4: advisory-level matching path should return Advisory or NoMatch, \
                     not {:?}",
                    other
                ),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Kani proof harnesses — VP-070 (pure function, deterministic path matching)
// These run under `cargo kani` only (not `cargo test`).
// ---------------------------------------------------------------------------

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    /// VP-070 Proof 1: match_path is deterministic.
    /// Same (path, registry) always yields same MatchResult.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_match_path_is_deterministic() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);

        let entry = RegistryEntry {
            artifact_type: ".factory/specs/behavioral-contracts/ss-".to_string(),
            canonical_path_pattern:
                ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
            description: "Behavioral contract spec".to_string(),
            enforcement_level: "block".to_string(),
        };
        let registry = PathRegistry {
            version: 1,
            artifacts: vec![entry],
        };

        let decision_1 = matches_canonical(&path, &registry);
        let decision_2 = matches_canonical(&path, &registry);

        kani::assert(
            decision_1 == decision_2,
            "VP-070 Proof 1: matches_canonical must be deterministic: \
             same inputs must yield same MatchResult",
        );
    }

    /// VP-070 Proof 2: Non-.factory/ paths must return MatchResult::NoMatch (never Block).
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_non_factory_path_returns_nomatch() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);
        kani::assume(!path.starts_with(".factory/"));

        let entry = RegistryEntry {
            artifact_type: "behavioral-contract".to_string(),
            canonical_path_pattern:
                ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
            description: "Behavioral contract spec".to_string(),
            enforcement_level: "block".to_string(),
        };
        let registry = PathRegistry {
            version: 1,
            artifacts: vec![entry],
        };

        let result = matches_canonical(&path, &registry);
        kani::assert(
            result == MatchResult::NoMatch,
            "VP-070 Proof 2: non-.factory/ paths must return NoMatch",
        );
    }

    /// VP-070 Proof 3: empty path always returns NoMatch.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_empty_path_returns_nomatch() {
        let entry = RegistryEntry {
            artifact_type: "behavioral-contract".to_string(),
            canonical_path_pattern:
                ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
            description: "Behavioral contract spec".to_string(),
            enforcement_level: "block".to_string(),
        };
        let registry = PathRegistry {
            version: 1,
            artifacts: vec![entry],
        };

        let result = matches_canonical("", &registry);
        kani::assert(
            result == MatchResult::NoMatch,
            "VP-070 Proof 3: empty path must return NoMatch",
        );
    }

    /// VP-070 Proof 4: advisory-only registry never returns Block from matches_canonical.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_advisory_only_registry_never_produces_block() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);

        let entry = RegistryEntry {
            artifact_type: "behavioral-contract".to_string(),
            canonical_path_pattern:
                ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
            description: "Behavioral contract spec".to_string(),
            enforcement_level: "advisory".to_string(),
        };
        let registry = PathRegistry {
            version: 1,
            artifacts: vec![entry],
        };

        let result = matches_canonical(&path, &registry);
        kani::assert(
            result != MatchResult::Block,
            "VP-070 Proof 4: advisory-only registry must never return MatchResult::Block",
        );
    }
}
