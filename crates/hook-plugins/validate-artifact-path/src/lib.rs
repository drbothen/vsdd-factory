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
    let registry: PathRegistry =
        serde_yaml::from_str(yaml).map_err(|e| RegistryError::ParseError(e.to_string()))?;
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
/// Patterns use `{placeholder}` syntax. Each placeholder matches a single
/// non-empty path segment — one or more characters that do NOT contain `/`.
/// Multi-segment spanning across `/` is prohibited per BC-4.11.001 invariant
/// 6 (amended v1.1, NC-1, F5 pass-1 fix burst 2026-05-07).
///
/// # BC trace
/// BC-4.11.001 postcondition 2b — match path against registered patterns.
/// BC-4.11.001 EC-005 — first matching entry wins.
/// VP-070 — path matching is pure and deterministic.
pub fn matches_canonical(path: &str, registry: &PathRegistry) -> MatchResult {
    // Non-.factory/ paths are out of scope — return NoMatch so hook_logic
    // can do an early-exit Continue without blocking.
    // Accept both relative (`.factory/...`) and absolute (`/path/to/.factory/...`) forms.
    // Leading-slash discipline (`/.factory/`) prevents false positives on `prefix.factory/...`.
    // F-P18-001: sibling fix propagated from validate-stable-anchors cc5a016b.
    let normalized: &str = if path.starts_with(".factory/") {
        // Already relative — use as-is.
        path
    } else if let Some(idx) = path.find("/.factory/") {
        // Absolute path: strip the prefix up to and including the leading slash,
        // leaving the `.factory/...` relative form for pattern matching.
        &path[idx + 1..]
    } else {
        // Not a .factory/ path — out of scope.
        return MatchResult::NoMatch;
    };

    for entry in &registry.artifacts {
        if pattern_matches(normalized, &entry.canonical_path_pattern) {
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
/// pattern matches any single path segment (one or more characters that do
/// NOT contain `/`).
///
/// This implements a simple glob-like match: literal characters must match
/// exactly, and `{placeholder}` segments match exactly one non-empty path
/// segment (no `/` in matched content). For example, `{cycle-id}` matches
/// `v1.0-feature-engine-discipline-pass-1` but NOT `v1.0/sub`.
///
/// Algorithm: split the pattern on `{...}` tokens; check that the path
/// contains each literal segment in order, with at least one non-slash
/// character between consecutive literal segments (where a placeholder
/// is expected).
///
/// # BC trace
/// BC-4.11.001 invariant 6 (v1.1): `{placeholder}` matches any non-empty
/// sequence of characters that does NOT contain `/` (single path segment).
/// Amended 2026-05-07 per NC-1 / F5 pass-1 fix burst B2 (F-HIGH-6).
fn pattern_matches(path: &str, pattern: &str) -> bool {
    // Split pattern into alternating literal / placeholder segments.
    // E.g. ".factory/cycles/{cycle-id}/{doc-type}.md" becomes:
    //   [".factory/cycles/", "cycle-id", "/", "doc-type", ".md"]
    // We only care about the literal parts for matching purposes.
    let parts = split_pattern(pattern);

    // Walk through the literal parts ensuring each appears in the path
    // in order, with at least one non-slash character between consecutive
    // parts where a placeholder should sit.
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
                if i == 0
                    || (i > 0 && !matches!(parts.get(i - 1), Some(PatternPart::Placeholder(_))))
                {
                    // Consecutive literals — must appear immediately
                    if path_bytes[pos..].starts_with(lit.as_bytes()) {
                        pos += lit.len();
                    } else {
                        return false;
                    }
                } else {
                    // After a placeholder — the placeholder content must be:
                    //   (a) at least one character (non-empty), and
                    //   (b) contain no '/' (single path segment, per invariant 6 v1.1).
                    //
                    // Find the next literal anywhere ahead, then verify that the
                    // placeholder content (bytes from pos+1 to literal_start) has no '/'.
                    if pos >= path_bytes.len() {
                        return false;
                    }
                    // The placeholder content starts at pos+1 (must have >=1 char before literal).
                    let search_start = pos + 1;
                    match find_subsequence(&path_bytes[search_start..], lit.as_bytes()) {
                        Some(offset) => {
                            // offset is relative to search_start; literal_start is absolute.
                            let literal_start = search_start + offset;
                            // Placeholder content: path[pos+1 .. literal_start]
                            // Must not contain '/' (single-segment invariant).
                            let placeholder_content = &path_bytes[search_start..literal_start];
                            if placeholder_content.contains(&b'/') {
                                return false;
                            }
                            pos = literal_start + lit.len();
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
    // (the last placeholder, if any, must match the rest of the path,
    // and that remainder must contain no '/' per single-segment rule).
    let last_is_placeholder = matches!(parts.last(), Some(PatternPart::Placeholder(_)));
    if last_is_placeholder {
        // Last placeholder: must match at least one character and no '/'.
        pos < path_bytes.len() && !path_bytes[pos..].contains(&b'/')
    } else {
        // All literal parts matched; path must be exactly consumed.
        pos == path_bytes.len()
    }
}

/// A parsed pattern part.
#[derive(Debug, Clone)]
enum PatternPart {
    Literal(String),
    #[allow(dead_code)] // placeholder name captured for diagnostics but not currently read
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
            // EC-006: file_path absent — log_warn (level 3) and emit hook.warn event
            // (F-MED-4 fix: emit_event for observability, mirrors MatchResult::Warn path).
            (callbacks.log)(
                3,
                "[validate-artifact-path] WARN: file_path absent from tool_input payload — graceful degrade",
            );
            (callbacks.emit_event)(
                "hook.warn",
                &[
                    ("hook", "validate-artifact-path"),
                    ("code", "EC_006_FILE_PATH_ABSENT"),
                    ("enforcement_level", "warn"),
                ],
            );
            return HookResult::Continue;
        }
    };

    // BC-4.11.001 PC7 / EC-004: non-.factory/ path → early-exit Continue.
    // No registry lookup performed.
    // Accept both relative (`.factory/...`) and absolute (`/path/to/.factory/...`) forms.
    // Leading-slash discipline (`/.factory/`) prevents false positives on `prefix.factory/...`.
    // F-P18-001: sibling fix propagated from validate-stable-anchors cc5a016b.
    let is_factory_path =
        file_path.starts_with(".factory/") || file_path.contains("/.factory/");
    if !is_factory_path {
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
                format!(
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
                0..=2 => host::log_info(msg),
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

// ---------------------------------------------------------------------------
// Unit tests — see src/tests.rs (extracted to keep lib.rs free of
// hardcoded path pattern literals per VP-072 / BC-4.11.001 invariant 1).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Kani proof harnesses — VP-070 (pure function, deterministic path matching)
// These run under `cargo kani` only (not `cargo test`).
// Patterns use a generic test form to avoid embedding registry paths in source
// (VP-072 / BC-4.11.001 invariant 1: no hardcoded .factory/ path pattern lists).
// ---------------------------------------------------------------------------

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    // Generic test pattern — uses .factory/ prefix with placeholder segments
    // without encoding a specific canonical registry path. This tests the
    // pattern-matching algorithm itself, not a specific registry entry.
    const KANI_TEST_PATTERN: &str = ".factory/test/{seg-a}/{seg-b}.md";

    /// VP-070 Proof 1: match_path is deterministic.
    /// Same (path, registry) always yields same MatchResult.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_match_path_is_deterministic() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);

        let entry = RegistryEntry {
            artifact_type: "test-artifact".to_string(),
            canonical_path_pattern: KANI_TEST_PATTERN.to_string(),
            description: "Kani test artifact".to_string(),
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
            artifact_type: "test-artifact".to_string(),
            canonical_path_pattern: KANI_TEST_PATTERN.to_string(),
            description: "Kani test artifact".to_string(),
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
            artifact_type: "test-artifact".to_string(),
            canonical_path_pattern: KANI_TEST_PATTERN.to_string(),
            description: "Kani test artifact".to_string(),
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
            artifact_type: "test-artifact".to_string(),
            canonical_path_pattern: KANI_TEST_PATTERN.to_string(),
            description: "Kani test artifact".to_string(),
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
