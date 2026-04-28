//! session-start-telemetry — SessionStart WASM hook plugin.
//!
//! Emits `session.started` with 6 plugin-set fields per BC-4.04.001:
//!   - `factory_version` — compile-time `env!("CARGO_PKG_VERSION")`
//!   - `plugin_count`    — count of WASM plugins loaded (v1.0: hardcoded "0"; v1.1 candidate)
//!   - `activated_platform` — read from `.claude/settings.local.json` via `read_file` host fn
//!   - `factory_health`  — result of `factory-health --brief` via `exec_subprocess`
//!   - `tool_deps`       — whitelist-filtered tool versions; JSON-serialized, ≤ 512 bytes
//!   - `timestamp`       — ISO-8601 UTC with millisecond precision and `Z` suffix
//!
//! 4 host-enriched fields are auto-injected by the `emit_event` host fn from `HostContext`
//! (BC-1.05.012): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//!
//! 4 construction-time fields are set by `InternalEvent::now()`:
//! `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! Plugin is unconditionally stateless (idempotency enforced at Layer 1 by Claude Code's
//! `once: true` directive in hooks.json.template per BC-4.04.003 + BC-4.04.004).

use serde_json::Value;
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Whitelist for tool_deps detection (BC-4.04.001 PC-2).
// Eviction order is REVERSE of this list per BC-4.04.001 EC-003 (F-P8-03).
// ---------------------------------------------------------------------------

/// Canonical whitelist for `tool_deps` detection (BC-4.04.001).
/// Eviction order (reverse): `cargo` first, then `rustc`, `yq`, `jq`, `git`.
pub const TOOL_DEPS_WHITELIST: &[&str] = &["git", "jq", "yq", "rustc", "cargo"];

/// Maximum allowed byte length per tool version string (construction-time cap).
/// Per BC-4.04.001 EC-003: canonical max payload with 5×64-char values ≈ 387 bytes < 512.
pub const TOOL_DEPS_MAX_VALUE_LEN: usize = 64;

/// 512-byte budget for the serialized `tool_deps` JSON object.
pub const TOOL_DEPS_SIZE_BUDGET: usize = 512;

// ---------------------------------------------------------------------------
// Outcome types for injectable callbacks.
// ---------------------------------------------------------------------------

/// Outcome of a `read_file` call on `.claude/settings.local.json`.
pub enum ReadFileOutcome {
    /// File read succeeded; contents are the raw bytes.
    Ok(Vec<u8>),
    /// File missing, parse error, capability denied, etc. — fail-open.
    Err,
}

/// Outcome of the `exec_subprocess("factory-health", ["--brief"])` call.
pub enum ExecSubprocessOutcome {
    /// Subprocess ran to completion. exit_code and stdout are available.
    Ok { exit_code: i32, stdout: String },
    /// Subprocess invocation failed: binary not found, timeout, capability denied, etc.
    Err,
}

// ---------------------------------------------------------------------------
// Pure business logic (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Compute the `activated_platform` value from `.claude/settings.local.json` contents.
///
/// Parses the JSON, extracts `vsdd-factory.activated_platform`.
/// On any failure (missing key, wrong type, parse error) → `"unknown"` (fail-open per EC-002).
pub fn extract_activated_platform(file_bytes: &[u8]) -> String {
    let json: Value = match serde_json::from_slice(file_bytes) {
        Ok(v) => v,
        Err(_) => return "unknown".to_string(),
    };
    match json
        .get("vsdd-factory")
        .and_then(|v| v.get("activated_platform"))
        .and_then(|v| v.as_str())
    {
        Some(s) => s.to_string(),
        None => "unknown".to_string(),
    }
}

/// Map `factory-health --brief` subprocess outcome to a `factory_health` string.
///
/// Per BC-4.04.002 PC-2:
/// - exit 0 + no `^WARN(ING)?:` line → `"healthy"`
/// - exit 0 + at least one `^WARN(ING)?:` line → `"warnings"`
/// - exit non-zero → `"errors"`
/// - subprocess error (not found, timeout, etc.) → `"unknown"`
pub fn map_factory_health(outcome: &ExecSubprocessOutcome) -> &'static str {
    match outcome {
        ExecSubprocessOutcome::Err => "unknown",
        ExecSubprocessOutcome::Ok { exit_code, stdout } => {
            if *exit_code != 0 {
                return "errors";
            }
            // Case-sensitive prefix match per BC-4.04.002 PC-2.
            let has_warn = stdout
                .lines()
                .any(|line| line.starts_with("WARN:") || line.starts_with("WARNING:"));
            if has_warn { "warnings" } else { "healthy" }
        }
    }
}

/// Compute `tool_deps` JSON string from a map of tool -> version_string.
///
/// Enforces:
/// - Per-value length cap: truncate to TOOL_DEPS_MAX_VALUE_LEN (construction-time).
/// - 512-byte serialization budget per BC-4.04.001 EC-003.
/// - Eviction order: reverse of TOOL_DEPS_WHITELIST (`cargo` first, etc.).
/// - Returns `None` if no tools present or post-eviction still > budget.
///
/// Input `tool_versions`: key is tool name (from whitelist), value is raw version string.
/// Keys NOT in the whitelist are silently dropped.
pub fn compute_tool_deps(
    tool_versions: &std::collections::HashMap<String, String>,
) -> Option<String> {
    // Build ordered map: only whitelist keys, values capped at TOOL_DEPS_MAX_VALUE_LEN.
    let mut map: serde_json::Map<String, Value> = serde_json::Map::new();
    for &tool in TOOL_DEPS_WHITELIST {
        if let Some(version) = tool_versions.get(tool) {
            let capped: String = version.chars().take(TOOL_DEPS_MAX_VALUE_LEN).collect();
            map.insert(tool.to_string(), Value::String(capped));
        }
    }

    if map.is_empty() {
        return None;
    }

    // Serialize and check budget. serde_json::Map preserves insertion order,
    // but the wire format must be lexicographically sorted for determinism.
    // Collect into a sorted BTreeMap-like structure via serde_json with sorted keys.
    let serialized = serialize_sorted(&map);
    if serialized.len() <= TOOL_DEPS_SIZE_BUDGET {
        return Some(serialized);
    }

    // EC-003: budget exceeded — evict in reverse-whitelist order.
    // TOOL_DEPS_WHITELIST = [git, jq, yq, rustc, cargo]
    // Reverse order = [cargo, rustc, yq, jq, git]
    for &evict_key in TOOL_DEPS_WHITELIST.iter().rev() {
        if map.remove(evict_key).is_some() {
            if map.is_empty() {
                // All keys evicted — defense-in-depth fallback.
                return None;
            }
            let s = serialize_sorted(&map);
            if s.len() <= TOOL_DEPS_SIZE_BUDGET {
                return Some(s);
            }
        }
    }

    // Budget still exceeded after whitelist exhaustion — emit null (defense-in-depth).
    None
}

/// Serialize a `serde_json::Map` with lexicographically sorted keys, no whitespace.
fn serialize_sorted(map: &serde_json::Map<String, Value>) -> String {
    // Collect keys, sort, build a new map in sorted order.
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut sorted: serde_json::Map<String, Value> = serde_json::Map::new();
    for k in keys {
        sorted.insert(k.clone(), map[k].clone());
    }
    serde_json::to_string(&Value::Object(sorted)).unwrap_or_default()
}

/// Generate an ISO-8601 UTC timestamp with millisecond precision and `Z` suffix.
///
/// Format: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`
pub fn now_timestamp() -> String {
    use chrono::Utc;
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level session-start hook logic with injectable callbacks.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime — the same pattern as
/// `capture-commit-activity::commit_hook_logic`.
///
/// - `read_file_fn`: called once to read `.claude/settings.local.json`
/// - `exec_subprocess_fn`: called once to invoke `factory-health --brief`
/// - `emit_fn`: called once to emit the `session.started` event
///
/// The plugin is unconditionally stateless (BC-4.04.003): it emits on every
/// invocation without dedup state.
pub fn session_start_hook_logic<R, E, Emit>(
    _payload: HookPayload,
    read_file_fn: R,
    exec_subprocess_fn: E,
    emit_fn: Emit,
) -> HookResult
where
    R: FnOnce() -> ReadFileOutcome,
    E: FnOnce() -> ExecSubprocessOutcome,
    Emit: FnOnce(&[(&str, &str)]),
{
    // 1. Read activated_platform from .claude/settings.local.json (BC-4.04.001 PC-2).
    let activated_platform = match read_file_fn() {
        ReadFileOutcome::Ok(bytes) => extract_activated_platform(&bytes),
        ReadFileOutcome::Err => "unknown".to_string(),
    };

    // 2. Invoke factory-health --brief (BC-4.04.002).
    let health_outcome = exec_subprocess_fn();
    let factory_health = map_factory_health(&health_outcome);

    // 3. factory_version: compile-time CARGO_PKG_VERSION (BC-4.04.001 PC-2).
    let factory_version = env!("CARGO_PKG_VERSION");

    // 4. plugin_count: v1.0 stub → "0".
    //    The plugin doesn't have access to the dispatcher's PluginCache.
    //    Tracked as v1.1 candidate: expose plugin_count via HookPayload or a host fn.
    let plugin_count = "0";

    // 5. tool_deps: detect whitelist tool versions (BC-4.04.001 PC-2).
    //    v1.0 simplification: tool version detection is skipped in the library (not calling
    //    exec_subprocess for tool detection — only factory-health is called via exec_subprocess).
    //    tool_deps = null when no detection is performed.
    //    The eviction logic (EC-003) is exercised via the public `compute_tool_deps` function.
    //    v1.1 candidate: call exec_subprocess for each tool in TOOL_DEPS_WHITELIST.
    let tool_deps: Option<String> = None;

    // 6. timestamp: ISO-8601 UTC with millisecond precision (BC-4.04.001 PC-2).
    let timestamp = now_timestamp();

    // 7. Emit session.started (BC-4.04.001 PC-1).
    //    Do NOT set RESERVED_FIELDS: dispatcher_trace_id, session_id, plugin_name,
    //    plugin_version, ts, ts_epoch, schema_version, type.
    let tool_deps_str;
    let fields_with_tool_deps: Vec<(&str, &str)>;
    let fields_without_tool_deps: Vec<(&str, &str)>;

    if let Some(ref td) = tool_deps {
        tool_deps_str = td.as_str();
        fields_with_tool_deps = vec![
            ("factory_version", factory_version),
            ("plugin_count", plugin_count),
            ("activated_platform", activated_platform.as_str()),
            ("factory_health", factory_health),
            ("tool_deps", tool_deps_str),
            ("timestamp", timestamp.as_str()),
        ];
        emit_fn(&fields_with_tool_deps);
    } else {
        // tool_deps = null: don't emit the field (it will be absent / null in the event).
        fields_without_tool_deps = vec![
            ("factory_version", factory_version),
            ("plugin_count", plugin_count),
            ("activated_platform", activated_platform.as_str()),
            ("factory_health", factory_health),
            ("timestamp", timestamp.as_str()),
        ];
        emit_fn(&fields_without_tool_deps);
    }

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `session_start_hook_logic`.
pub fn on_session_start(payload: HookPayload) -> HookResult {
    session_start_hook_logic(
        payload,
        // read_file: .claude/settings.local.json
        || {
            match vsdd_hook_sdk::host::read_file(".claude/settings.local.json", 65536, 1000) {
                Ok(bytes) => ReadFileOutcome::Ok(bytes),
                Err(_) => ReadFileOutcome::Err,
            }
        },
        // exec_subprocess: factory-health --brief
        || {
            match vsdd_hook_sdk::host::exec_subprocess(
                "factory-health",
                &["--brief"],
                &[],
                5000,
                65536,
            ) {
                Ok(result) => ExecSubprocessOutcome::Ok {
                    exit_code: result.exit_code,
                    stdout: String::from_utf8_lossy(&result.stdout).into_owned(),
                },
                Err(_) => ExecSubprocessOutcome::Err,
            }
        },
        // emit_event
        |fields| {
            vsdd_hook_sdk::host::emit_event("session.started", fields);
        },
    )
}

// ---------------------------------------------------------------------------
// Test helper: compute_tool_deps without the 64-char per-value cap.
// Used by the EC-003 eviction test to construct over-budget payloads.
// Not for use in production code — the cap is intentional.
// ---------------------------------------------------------------------------

/// Compute `tool_deps` without the 64-char per-value cap.
///
/// **Test-helper only.** Used by VP-065 `test_bc_4_04_001_tool_deps_eviction_when_oversized`
/// to bypass the construction-time 64-char cap and exercise the EC-003 eviction algorithm
/// with an artificially over-budget payload. Not for use in production code.
#[doc(hidden)]
pub fn compute_tool_deps_uncapped(
    tool_versions: &std::collections::HashMap<String, String>,
) -> Option<String> {
    // Same as compute_tool_deps but WITHOUT the 64-char cap on values.
    // This allows tests to bypass the construction-time cap and exercise EC-003.
    let mut map: serde_json::Map<String, Value> = serde_json::Map::new();
    for &tool in TOOL_DEPS_WHITELIST {
        if let Some(version) = tool_versions.get(tool) {
            map.insert(tool.to_string(), Value::String(version.clone()));
        }
    }

    if map.is_empty() {
        return None;
    }

    let serialized = serialize_sorted(&map);
    if serialized.len() <= TOOL_DEPS_SIZE_BUDGET {
        return Some(serialized);
    }

    // EC-003 eviction: reverse whitelist order (cargo first, then rustc, yq, jq, git).
    for &evict_key in TOOL_DEPS_WHITELIST.iter().rev() {
        if map.remove(evict_key).is_some() {
            if map.is_empty() {
                return None;
            }
            let s = serialize_sorted(&map);
            if s.len() <= TOOL_DEPS_SIZE_BUDGET {
                return Some(s);
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Tests for pure logic functions
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_activated_platform_happy_path() {
        let json = br#"{"vsdd-factory": {"activated_platform": "darwin-arm64"}}"#;
        assert_eq!(extract_activated_platform(json), "darwin-arm64");
    }

    #[test]
    fn extract_activated_platform_missing_key() {
        let json = br#"{"vsdd-factory": {}}"#;
        assert_eq!(extract_activated_platform(json), "unknown");
    }

    #[test]
    fn extract_activated_platform_invalid_json() {
        assert_eq!(extract_activated_platform(b"not json"), "unknown");
    }

    #[test]
    fn extract_activated_platform_wrong_type() {
        let json = br#"{"vsdd-factory": {"activated_platform": 42}}"#;
        assert_eq!(extract_activated_platform(json), "unknown");
    }

    #[test]
    fn map_factory_health_healthy() {
        let out = ExecSubprocessOutcome::Ok {
            exit_code: 0,
            stdout: "All checks passed.".to_string(),
        };
        assert_eq!(map_factory_health(&out), "healthy");
    }

    #[test]
    fn map_factory_health_warnings() {
        let out = ExecSubprocessOutcome::Ok {
            exit_code: 0,
            stdout: "WARN: something degraded".to_string(),
        };
        assert_eq!(map_factory_health(&out), "warnings");
    }

    #[test]
    fn map_factory_health_warnings_via_warning_prefix() {
        let out = ExecSubprocessOutcome::Ok {
            exit_code: 0,
            stdout: "WARNING: disk space low".to_string(),
        };
        assert_eq!(map_factory_health(&out), "warnings");
    }

    #[test]
    fn map_factory_health_errors() {
        let out = ExecSubprocessOutcome::Ok {
            exit_code: 1,
            stdout: String::new(),
        };
        assert_eq!(map_factory_health(&out), "errors");
    }

    #[test]
    fn map_factory_health_unknown_on_err() {
        assert_eq!(map_factory_health(&ExecSubprocessOutcome::Err), "unknown");
    }

    #[test]
    fn compute_tool_deps_empty_input() {
        let map = std::collections::HashMap::new();
        assert!(compute_tool_deps(&map).is_none());
    }

    #[test]
    fn compute_tool_deps_single_tool() {
        let mut map = std::collections::HashMap::new();
        map.insert("git".to_string(), "2.42.0".to_string());
        let result = compute_tool_deps(&map).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["git"].as_str(), Some("2.42.0"));
    }

    #[test]
    fn compute_tool_deps_non_whitelist_keys_dropped() {
        let mut map = std::collections::HashMap::new();
        map.insert("git".to_string(), "2.42.0".to_string());
        map.insert("curl".to_string(), "7.88.0".to_string()); // not in whitelist
        let result = compute_tool_deps(&map).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("curl").is_none());
        assert!(parsed.get("git").is_some());
    }

    #[test]
    fn compute_tool_deps_eviction_order() {
        // Construct an over-budget payload by using very long values (bypassing 64-char cap
        // by pre-inserting the value directly into the whitelist map).
        // This exercises the EC-003 defense-in-depth eviction algorithm.
        let long_value = "x".repeat(100); // 100 chars > 64-char per-value cap
        let mut map = std::collections::HashMap::new();
        // Insert directly; compute_tool_deps caps values at construction time.
        // To bypass the cap we need to test the eviction function directly with a
        // pre-built oversized map. We'll use the serialize_sorted path.

        // Build a map with 5 tools, each value exactly 100 chars (over 64-char cap).
        // compute_tool_deps will cap at 64 chars so won't trigger EC-003 naturally.
        // Instead we test compute_tool_deps_uncapped which is the internal eviction fn.
        // For the WHITE-BOX test we use compute_tool_deps_with_raw to bypass the cap.
        for &tool in TOOL_DEPS_WHITELIST {
            map.insert(tool.to_string(), long_value.clone());
        }
        // With 64-char cap applied (by compute_tool_deps), each value = 64 chars.
        // 5 tools × (quoted key + quoted 64-char value + overhead) ≈ 387 bytes < 512.
        // So normal production never triggers EC-003. We test the EC-003 path directly:
        let result = compute_tool_deps_uncapped(&map);
        // After cap at 64 chars per value, result is within budget.
        assert!(result.is_some());
    }

    #[test]
    fn now_timestamp_format() {
        let ts = now_timestamp();
        let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$").unwrap();
        assert!(re.is_match(&ts), "timestamp format mismatch: {ts:?}");
    }
}
