//! `hooks-registry.toml` parser and type declarations.
//!
//! The registry is the dispatcher's source of truth for *which* plugins
//! exist, *what* events they react to, and *what capabilities* they
//! hold. S-1.2 defines the types; S-1.5 consumes capabilities during
//! plugin instantiation; S-2.2 auto-generates the file from the existing
//! v0.79.x hooks.json.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Current registry schema version. The loader refuses anything else
/// so an unreleased schema change can't silently mis-parse.
///
/// S-15.01 (AC-001, BC-7.06.001 postcondition 1): bumped to 2 for per-plugin
/// `async` field support and partition semantics (ADR-019). A registry with
/// `schema_version != 2` produces E-REG-001 at load time (fail-closed).
pub const REGISTRY_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("registry file not found: {0}")]
    NotFound(PathBuf),
    #[error("registry read failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("registry parse failed: {0}")]
    Toml(#[from] toml::de::Error),
    #[error(
        "registry schema_version = {got}, dispatcher expects {expected}. \
         Regenerate hooks-registry.toml or upgrade the dispatcher. \
         [E-REG-001]"
    )]
    SchemaVersion { got: u32, expected: u32 },
    #[error("registry entry '{name}': invalid tool regex `{pattern}`: {source}")]
    ToolRegex {
        name: String,
        pattern: String,
        #[source]
        source: regex::Error,
    },
    /// E-REG-002: entry has `on_error = "block"` AND `async = true`.
    /// Enforced at registry-load time; dispatcher refuses to start.
    /// (BC-1.14.001 Invariant 4, BC-7.06.001 Invariant 1)
    #[error(
        "registry entry '{name}': on_error = \"block\" combined with async = true is forbidden \
         (BC-7.06.001 Invariant 1). Classify this plugin async = false or remove on_error = \"block\". \
         [E-REG-002]"
    )]
    AsyncBlockConflict { name: String },
    /// E-REG-003: duplicate (name, event, tool) tuple across [[hooks]] entries.
    /// Enforced at registry-load time; dispatcher refuses to start.
    /// (BC-7.06.001 Invariant 7, F-P8-001)
    #[error(
        "[E-REG-003] Duplicate hook entry: name={name}, event={event}, tool={tool:?} \
         (BC-7.06.001 Invariant 7). Each (name, event, tool) tuple must be unique \
         across all [[hooks]] entries."
    )]
    DuplicateEntry {
        name: String,
        event: String,
        tool: Option<String>,
    },
}

/// Outcome for a plugin that returns `Error` or crashes. `Continue` is
/// the default; operators opt into hard-stop behavior per plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OnError {
    #[default]
    Continue,
    Block,
}

/// Capability declaration for a plugin entry. Deny-by-default — a
/// missing block means the plugin cannot use the corresponding host
/// function at all.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    #[serde(default)]
    pub exec_subprocess: Option<ExecSubprocessCaps>,
    #[serde(default)]
    pub read_file: Option<ReadFileCaps>,
    /// Write-file capability declaration (BC-2.02.011).
    /// Deny-by-default: absence of this block causes every `write_file`
    /// call to return `CAPABILITY_DENIED (-1)`.
    #[serde(default)]
    pub write_file: Option<WriteFileCaps>,
    /// Environment variable names the plugin is allowed to read.
    #[serde(default)]
    pub env_allow: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExecSubprocessCaps {
    /// Binary basename allow-list. The dispatcher resolves each entry
    /// to a full path at registry load time (S-1.5 enforces).
    pub binary_allow: Vec<String>,
    /// Opt-in to invoking shell interpreters (bash/sh/zsh/pwsh). The
    /// field name is deliberate: the operator has acknowledged the
    /// blast-radius increase.
    #[serde(default)]
    pub shell_bypass_acknowledged: Option<String>,
    /// Working-directory allow-list, relative to `CLAUDE_PROJECT_DIR`.
    /// Empty means "no cwd override permitted".
    #[serde(default)]
    pub cwd_allow: Vec<String>,
    /// Environment variable names permitted to be forwarded to the
    /// subprocess.
    #[serde(default)]
    pub env_allow: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReadFileCaps {
    /// Path prefixes the plugin is allowed to read, rooted at
    /// `CLAUDE_PROJECT_DIR` unless absolute.
    pub path_allow: Vec<String>,
}

/// Capability declaration for `host::write_file` (BC-2.02.011).
/// Parallel to [`ReadFileCaps`]. Deny-by-default: absence of this block
/// causes every `vsdd::write_file` host call to return `CAPABILITY_DENIED`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WriteFileCaps {
    /// Path prefixes the plugin is allowed to write, rooted at
    /// `CLAUDE_PROJECT_DIR` unless absolute.
    pub path_allow: Vec<String>,
    /// Optional per-call byte cap. When set, overrides the `max_bytes`
    /// argument if the argument exceeds this value. If `None`, the
    /// `max_bytes` argument is used as-is (BC-2.02.011 invariant 2:
    /// `max_bytes` is mandatory; no opt-out is permitted).
    #[serde(default)]
    pub max_bytes_per_call: Option<u32>,
}

/// Registry-wide defaults, applied when a per-entry field is missing.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, default)]
pub struct RegistryDefaults {
    pub timeout_ms: u32,
    pub fuel_cap: u64,
    pub on_error: OnError,
    pub priority: u32,
}

impl Default for RegistryDefaults {
    fn default() -> Self {
        Self {
            timeout_ms: 5_000,
            fuel_cap: 10_000_000,
            on_error: OnError::Continue,
            priority: 500,
        }
    }
}

/// A single plugin registration.
///
/// `Eq` was deliberately dropped from the derives when the `config`
/// field landed: `toml::Value` carries `Float`, which is `PartialEq` but
/// not `Eq`. Tests use `assert_eq!`, which only needs `PartialEq`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RegistryEntry {
    /// Stable identifier, e.g. `"capture-commit-activity"`. Unique
    /// within a registry.
    pub name: String,

    /// Claude Code event this entry reacts to, e.g. `"PostToolUse"`.
    pub event: String,

    /// Regex matched against the tool name. `None` (the default) means
    /// "all tools".
    #[serde(default)]
    pub tool: Option<String>,

    /// Relative or absolute path to the plugin `.wasm` file.
    pub plugin: PathBuf,

    /// Ordering tier. Lower fires first. Same-priority plugins execute
    /// in parallel (S-1.6). Default inherited from `RegistryDefaults`.
    #[serde(default)]
    pub priority: Option<u32>,

    /// `false` skips the entry entirely.
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Per-call wall-clock budget. Default inherited from
    /// `RegistryDefaults.timeout_ms`.
    #[serde(default)]
    pub timeout_ms: Option<u32>,

    /// Per-call wasmtime fuel budget. Default inherited from
    /// `RegistryDefaults.fuel_cap`.
    #[serde(default)]
    pub fuel_cap: Option<u64>,

    /// What to do if the plugin errors. Default inherited from
    /// `RegistryDefaults.on_error`.
    #[serde(default)]
    pub on_error: Option<OnError>,

    /// Deny-by-default capability declaration. `None` means the plugin
    /// gets no host-function access beyond the always-on APIs
    /// (`log`, `emit_event`, `session_id`, etc.).
    #[serde(default)]
    pub capabilities: Option<Capabilities>,

    /// Per-plugin configuration. The dispatcher forwards this verbatim
    /// as `plugin_config` on the [`HookPayload`] handed to the plugin.
    /// Schema is plugin-defined; the registry only carries it through.
    /// Default is an empty TOML table — distinguishable from "missing"
    /// by plugin code that wants to require keys (the legacy-bash-
    /// adapter, S-2.1, demands `script_path`).
    ///
    /// [`HookPayload`]: vsdd_hook_sdk::HookPayload
    #[serde(default = "default_config")]
    pub config: toml::Value,

    /// Per-plugin async classification (S-15.01, BC-7.06.001 postcondition 2).
    ///
    /// - `async = true`: plugin is fire-and-forget (async_group). Its verdict
    ///   never affects the dispatcher exit code. Suitable for telemetry-only
    ///   plugins. MUST NOT be combined with `on_error = "block"` (E-REG-002).
    /// - `async = false` (default, including absent field): plugin is in the
    ///   sync_group. The dispatcher awaits its completion; a block verdict gates
    ///   Claude Code. The serde-default semantics (absent = false) ensure all
    ///   existing registry entries are treated as sync-group plugins without
    ///   any TOML file migration (BC-7.06.001 postcondition 3).
    ///
    /// Renamed to `async_flag` in Rust source because `async` is a reserved
    /// keyword. The TOML key remains `async`.
    ///
    /// ASYNC_DRAIN_WINDOW_MS for async group tasks is defined in DI-019 —
    /// cite by reference, do NOT hardcode the value (Decision 4).
    #[serde(default, rename = "async")]
    pub async_flag: bool,
}

fn default_enabled() -> bool {
    true
}

fn default_config() -> toml::Value {
    toml::Value::Table(toml::Table::new())
}

impl RegistryEntry {
    pub fn priority(&self, defaults: &RegistryDefaults) -> u32 {
        self.priority.unwrap_or(defaults.priority)
    }

    pub fn timeout_ms(&self, defaults: &RegistryDefaults) -> u32 {
        self.timeout_ms.unwrap_or(defaults.timeout_ms)
    }

    pub fn fuel_cap(&self, defaults: &RegistryDefaults) -> u64 {
        self.fuel_cap.unwrap_or(defaults.fuel_cap)
    }

    pub fn on_error(&self, defaults: &RegistryDefaults) -> OnError {
        self.on_error.unwrap_or(defaults.on_error)
    }

    /// Convert the registry-side `config` (TOML) into the JSON shape
    /// that lands on `HookPayload.plugin_config`. JSON-incompatible
    /// TOML scalars (datetime, NaN/inf floats) flatten to strings or
    /// null respectively; in practice the registry only carries
    /// strings, ints, bools, arrays, and tables.
    pub fn config_as_json(&self) -> serde_json::Value {
        toml_to_json(&self.config)
    }
}

fn toml_to_json(value: &toml::Value) -> serde_json::Value {
    match value {
        toml::Value::String(s) => serde_json::Value::String(s.clone()),
        toml::Value::Integer(i) => serde_json::Value::Number((*i).into()),
        toml::Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        toml::Value::Boolean(b) => serde_json::Value::Bool(*b),
        toml::Value::Datetime(d) => serde_json::Value::String(d.to_string()),
        toml::Value::Array(arr) => serde_json::Value::Array(arr.iter().map(toml_to_json).collect()),
        toml::Value::Table(tab) => serde_json::Value::Object(
            tab.iter()
                .map(|(k, v)| (k.clone(), toml_to_json(v)))
                .collect(),
        ),
    }
}

/// The whole parsed registry.
///
/// See [`RegistryEntry`] for why `Eq` is not derived.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub schema_version: u32,
    #[serde(default)]
    pub defaults: RegistryDefaults,
    #[serde(default, rename = "hooks")]
    pub hooks: Vec<RegistryEntry>,
}

impl Registry {
    /// Load + validate from a filesystem path.
    ///
    /// Relative `plugin` paths in entries are resolved against the
    /// registry file's parent directory — i.e. plugins live under the
    /// same `${CLAUDE_PLUGIN_ROOT}` as the registry itself. Absolute
    /// `plugin` paths pass through unchanged for tests / packaging
    /// flows that produce them deliberately.
    pub fn load(path: &Path) -> Result<Self, RegistryError> {
        if !path.exists() {
            return Err(RegistryError::NotFound(path.to_path_buf()));
        }
        let text = std::fs::read_to_string(path)?;
        let mut parsed: Self = toml::from_str(&text)?;
        parsed.validate()?;
        if let Some(base) = path.parent() {
            parsed.resolve_plugin_paths(base);
        }
        Ok(parsed)
    }

    /// Resolve every entry's relative `plugin` path against `base`.
    /// Idempotent — absolute paths pass through unchanged.
    pub fn resolve_plugin_paths(&mut self, base: &Path) {
        for entry in &mut self.hooks {
            if entry.plugin.is_relative() {
                entry.plugin = base.join(&entry.plugin);
            }
        }
    }

    /// Parse + validate from a TOML string buffer. Useful for tests.
    ///
    /// Named `parse_str` rather than `from_str` to avoid shadowing the
    /// `std::str::FromStr` convention, which operators (correctly)
    /// expect to be infallible-returning `Result<Self, Self::Err>`
    /// with a single associated error type.
    pub fn parse_str(s: &str) -> Result<Self, RegistryError> {
        let parsed: Self = toml::from_str(s)?;
        parsed.validate()?;
        Ok(parsed)
    }

    fn validate(&self) -> Result<(), RegistryError> {
        if self.schema_version != REGISTRY_SCHEMA_VERSION {
            return Err(RegistryError::SchemaVersion {
                got: self.schema_version,
                expected: REGISTRY_SCHEMA_VERSION,
            });
        }
        for entry in &self.hooks {
            if let Some(pattern) = &entry.tool {
                regex::Regex::new(pattern).map_err(|e| RegistryError::ToolRegex {
                    name: entry.name.clone(),
                    pattern: pattern.clone(),
                    source: e,
                })?;
            }
        }
        // S-15.01 T-3f: check BC-7.06.001 Invariant 1 — on_error=block implies async=false.
        // Any entry with on_error=block AND async=true is E-REG-002 (fail-closed).
        self.validate_async_block_invariant()?;
        // F-P2-011: BC-7.06.001 Invariant 7 — (name, event, tool) tuple must be unique.
        // Two entries MAY share name+event if they bind different tool regex values.
        self.validate_name_event_tool_uniqueness()?;
        Ok(())
    }

    /// Lint invariant (S-15.01 T-3f, BC-7.06.001 Invariant 1, BC-1.14.001 Invariant 4):
    /// No entry may combine `on_error = "block"` with `async = true`.
    ///
    /// This is a hard load-time error (E-REG-002). The dispatcher refuses to start
    /// if any entry violates this invariant. Emits `dispatcher.registry_invalid`.
    ///
    /// ASYNC_DRAIN_WINDOW_MS is defined in DI-019 — cite by reference only.
    fn validate_async_block_invariant(&self) -> Result<(), RegistryError> {
        for entry in &self.hooks {
            let on_error_is_block = entry.on_error == Some(OnError::Block);
            if on_error_is_block && entry.async_flag {
                return Err(RegistryError::AsyncBlockConflict {
                    name: entry.name.clone(),
                });
            }
        }
        Ok(())
    }

    /// Uniqueness invariant (F-P2-011, BC-7.06.001 Invariant 7):
    /// The (name, event, tool) tuple must be unique across all [[hooks]] entries.
    ///
    /// Two entries MAY share `name` and `event` if they bind to different `tool`
    /// regex values — this permits a single named plugin to enforce against multiple
    /// tool surfaces (e.g. `protect-secrets` on `Bash` and `Read` PreToolUse events).
    /// Two entries with `tool = None` and the same `name`+`event` are duplicates.
    ///
    /// Hard load-time error; dispatcher refuses to start on violation.
    fn validate_name_event_tool_uniqueness(&self) -> Result<(), RegistryError> {
        let mut seen: HashSet<(String, String, Option<String>)> = HashSet::new();
        for entry in &self.hooks {
            let key = (entry.name.clone(), entry.event.clone(), entry.tool.clone());
            if !seen.insert(key) {
                return Err(RegistryError::DuplicateEntry {
                    name: entry.name.clone(),
                    event: entry.event.clone(),
                    tool: entry.tool.clone(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_toml() -> &'static str {
        r#"
schema_version = 2

[[hooks]]
name = "commit"
event = "PostToolUse"
tool = "Bash"
plugin = "plugins/commit.wasm"
"#
    }

    #[test]
    fn parses_minimal_registry() {
        let reg = Registry::parse_str(minimal_toml()).unwrap();
        assert_eq!(reg.schema_version, 2);
        assert_eq!(reg.hooks.len(), 1);
        assert_eq!(reg.hooks[0].name, "commit");
        assert_eq!(reg.hooks[0].event, "PostToolUse");
        assert_eq!(reg.hooks[0].tool.as_deref(), Some("Bash"));
        assert!(reg.hooks[0].enabled);
    }

    #[test]
    fn config_defaults_to_empty_table_when_absent() {
        let reg = Registry::parse_str(minimal_toml()).unwrap();
        assert!(reg.hooks[0].config.is_table());
        assert_eq!(reg.hooks[0].config.as_table().unwrap().len(), 0);
        let as_json = reg.hooks[0].config_as_json();
        assert!(as_json.is_object());
        assert!(as_json.as_object().unwrap().is_empty());
    }

    #[test]
    fn config_block_parses_into_entry() {
        // Real-shape registry as the legacy-bash-adapter operator
        // would write — string + nested table.
        let toml = r#"
schema_version = 2

[[hooks]]
name = "validate-template"
event = "PostToolUse"
plugin = "hook-plugins/legacy-bash-adapter.wasm"

[hooks.config]
script_path = "legacy-hooks/validate-template.sh"
extra = { key = "value" }
"#;
        let reg = Registry::parse_str(toml).unwrap();
        let cfg = reg.hooks[0].config_as_json();
        assert_eq!(
            cfg.get("script_path").and_then(|v| v.as_str()),
            Some("legacy-hooks/validate-template.sh"),
        );
        assert_eq!(
            cfg.get("extra")
                .and_then(|v| v.get("key"))
                .and_then(|v| v.as_str()),
            Some("value"),
        );
    }

    #[test]
    fn defaults_applied_when_missing() {
        let reg = Registry::parse_str(minimal_toml()).unwrap();
        assert_eq!(reg.defaults.timeout_ms, 5_000);
        assert_eq!(reg.defaults.fuel_cap, 10_000_000);
        assert_eq!(reg.defaults.priority, 500);
        assert_eq!(reg.defaults.on_error, OnError::Continue);
        assert_eq!(reg.hooks[0].priority(&reg.defaults), 500);
        assert_eq!(reg.hooks[0].timeout_ms(&reg.defaults), 5_000);
    }

    #[test]
    fn rejects_unknown_schema_version() {
        // schema_version=3 is unknown — dispatcher expects 2 (REGISTRY_SCHEMA_VERSION).
        let toml = r#"
schema_version = 3

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "x.wasm"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::SchemaVersion { got, expected } => {
                assert_eq!(got, 3);
                assert_eq!(expected, 2);
            }
            other => panic!("expected SchemaVersion, got {other:?}"),
        }
    }

    #[test]
    fn rejects_invalid_tool_regex() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "bad"
event = "PreToolUse"
tool = "[unclosed"
plugin = "x.wasm"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::ToolRegex { name, .. } => assert_eq!(name, "bad"),
            other => panic!("expected ToolRegex, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_entry_field() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "typo"
event = "PreToolUse"
plugin = "x.wasm"
priorty = 100
"#;
        assert!(Registry::parse_str(toml).is_err());
    }

    #[test]
    fn rejects_unknown_on_error_value() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "x.wasm"
on_error = "shout"
"#;
        assert!(Registry::parse_str(toml).is_err());
    }

    #[test]
    fn accepts_capabilities_block() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "git"
event = "PostToolUse"
plugin = "git.wasm"

[hooks.capabilities]
env_allow = ["CLAUDE_SESSION_ID"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
cwd_allow = ["."]
env_allow = ["HOME"]

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
"#;
        let reg = Registry::parse_str(toml).unwrap();
        let caps = reg.hooks[0].capabilities.as_ref().unwrap();
        let exec = caps.exec_subprocess.as_ref().unwrap();
        assert_eq!(exec.binary_allow, vec!["git"]);
        assert_eq!(caps.env_allow, vec!["CLAUDE_SESSION_ID"]);
    }

    #[test]
    fn overrides_priority_per_entry() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "fast"
event = "PreToolUse"
plugin = "f.wasm"
priority = 10

[[hooks]]
name = "slow"
event = "PreToolUse"
plugin = "s.wasm"
priority = 900
"#;
        let reg = Registry::parse_str(toml).unwrap();
        assert_eq!(reg.hooks[0].priority(&reg.defaults), 10);
        assert_eq!(reg.hooks[1].priority(&reg.defaults), 900);
    }

    #[test]
    fn load_returns_not_found_for_missing_path() {
        let err = Registry::load(Path::new("/nonexistent/registry.toml")).unwrap_err();
        assert!(matches!(err, RegistryError::NotFound(_)));
    }

    #[test]
    fn load_resolves_relative_plugin_paths_against_registry_dir() {
        // Operators write `plugin = "x.wasm"` in hooks-registry.toml
        // expecting it to resolve under ${CLAUDE_PLUGIN_ROOT}, not cwd.
        // Regression for a smoke-test bug where the dispatcher reported
        // "plugin file not found" for a perfectly valid relative path.
        // The "absolute" fixture uses tempdir() to get a platform-native
        // absolute path (Path::is_absolute is platform-defined: POSIX
        // /paths/ are not absolute on Windows).
        let dir = tempfile::tempdir().unwrap();
        let reg_path = dir.path().join("hooks-registry.toml");
        let abs_dir = tempfile::tempdir().unwrap();
        let abs_plugin = abs_dir.path().join("explicit.wasm");
        assert!(abs_plugin.is_absolute());
        let abs_str = abs_plugin.to_str().unwrap().replace('\\', "/");
        std::fs::write(
            &reg_path,
            format!(
                r#"
schema_version = 2

[[hooks]]
name = "rel"
event = "PreToolUse"
plugin = "rel-plugin.wasm"

[[hooks]]
name = "abs"
event = "PreToolUse"
plugin = "{abs_str}"
"#
            ),
        )
        .unwrap();
        let reg = Registry::load(&reg_path).unwrap();
        assert_eq!(reg.hooks[0].plugin, dir.path().join("rel-plugin.wasm"));
        assert_eq!(reg.hooks[1].plugin, PathBuf::from(&abs_str));
    }

    #[test]
    fn resolve_plugin_paths_is_idempotent_for_absolute_paths() {
        let dir = tempfile::tempdir().unwrap();
        let abs_dir = tempfile::tempdir().unwrap();
        let abs_plugin = abs_dir.path().join("x.wasm");
        assert!(abs_plugin.is_absolute());
        let abs_str = abs_plugin.to_str().unwrap().replace('\\', "/");
        let mut reg = Registry::parse_str(&format!(
            r#"
schema_version = 2

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "{abs_str}"
"#
        ))
        .unwrap();
        let expected = reg.hooks[0].plugin.clone();
        reg.resolve_plugin_paths(dir.path());
        // Absolute path stays absolute.
        assert_eq!(reg.hooks[0].plugin, expected);
        reg.resolve_plugin_paths(dir.path());
        assert_eq!(reg.hooks[0].plugin, expected);
    }
}

// ---------------------------------------------------------------------------
// S-15.01 F4 test additions — VP-078 Harnesses 1 + 4 (registry-side tests)
//
// Harness 1 — lint_invariant: schema_version=2 required; v1 rejected; block+async rejected.
// Harness 4 — serde_default: absent `async` field → false; string `async` → parse error.
//
// These tests exercise Registry::parse_str() directly (no I/O).
// All tests must fail until T-3f is implemented (validate_async_block_invariant is todo!()).
// The schema_version tests fail immediately because REGISTRY_SCHEMA_VERSION = 2 and
// the existing tests use schema_version = 1 — the stub already enforces the version check.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod s15_01_vp078_harness_1_lint_invariant {
    use super::*;

    // -----------------------------------------------------------------------
    // Harness 1a — schema_version = 2 required
    // AC-001: BC-7.06.001 postcondition 1
    // -----------------------------------------------------------------------

    /// VP-078 H1: v1 registry rejected with E-REG-001 (SchemaVersion error).
    ///
    /// RED: validate() returns Err(SchemaVersion{got:1,expected:2}).
    /// Already enforced by REGISTRY_SCHEMA_VERSION = 2 constant in the stub.
    #[test]
    fn test_BC_7_06_001_schema_v1_rejected_with_e_reg_001() {
        let toml = r#"
schema_version = 1

[[hooks]]
name = "some-validator"
event = "PreToolUse"
plugin = "hook-plugins/some-validator.wasm"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::SchemaVersion { got, expected } => {
                assert_eq!(got, 1, "got must be the found version (1)");
                assert_eq!(expected, 2, "expected must be REGISTRY_SCHEMA_VERSION (2)");
            }
            other => panic!(
                "test_BC_7_06_001_schema_v1_rejected: expected SchemaVersion error, got {:?}",
                other
            ),
        }
    }

    /// VP-078 H1: missing schema_version field rejected (E-REG-001 boundary).
    ///
    /// RED: TOML without schema_version key — parse fails or schema_version defaults to 0,
    /// which != 2; either way the registry is rejected.
    #[test]
    fn test_BC_7_06_001_missing_schema_version_rejected() {
        let toml = r#"
[[hooks]]
name = "some-validator"
event = "PreToolUse"
plugin = "hook-plugins/some-validator.wasm"
"#;
        // Missing schema_version is either a TOML parse error (deny_unknown_fields) or
        // defaults to 0 then fails version check. Both produce Err.
        let result = Registry::parse_str(toml);
        assert!(
            result.is_err(),
            "test_BC_7_06_001_missing_schema_version_rejected: registry without schema_version must be rejected"
        );
    }

    /// VP-078 H1: schema_version = 2 with valid entries passes.
    ///
    /// RED until T-3f: validate_async_block_invariant() is todo!() — will panic.
    /// After T-3f, this should return Ok.
    #[test]
    fn test_BC_7_06_001_schema_v2_with_valid_entries_passes() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "capture-commit-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-commit-activity.wasm"
async = true
"#;
        // RED: todo!() in validate_async_block_invariant panics before we get Ok.
        let result = Registry::parse_str(toml);
        assert!(
            result.is_ok(),
            "test_BC_7_06_001_schema_v2_with_valid_entries_passes: schema_version=2 with valid entry must pass: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // Harness 1b — on_error=block + async=true is E-REG-002
    // AC-006: BC-7.06.001 invariant 1, BC-1.14.001 invariant 4
    // -----------------------------------------------------------------------

    /// VP-078 H1 / VP-078 Rust unit test: on_error=block AND async=true → E-REG-002.
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics.
    #[test]
    fn test_BC_7_06_001_block_plus_async_true_rejected_e_reg_002() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "bad-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = true
event = "PostToolUse"
priority = 400

[hooks.config]
script_path = "bad.sh"
"#;
        // RED: todo!() in validate_async_block_invariant — panics with "not yet implemented".
        let result = Registry::parse_str(toml);
        assert!(
            result.is_err(),
            "test_BC_7_06_001_block_plus_async_true_rejected_e_reg_002: block+async entry must be rejected (E-REG-002)"
        );
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("on_error")
                || err_str.contains("async")
                || err_str.contains("E-REG-002"),
            "error must name the violating fields or error code: {}",
            err_str
        );
    }

    /// VP-078 H1: on_error=block with async absent (defaults false) → accepted.
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics before returning Ok.
    #[test]
    fn test_BC_7_06_001_block_without_async_accepted() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "valid-blocking-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
event = "PostToolUse"
priority = 400

[hooks.config]
script_path = "valid.sh"
"#;
        // async absent → default false → invariant satisfied → Ok.
        // RED: todo!() in validate_async_block_invariant panics.
        let result = Registry::parse_str(toml);
        assert!(
            result.is_ok(),
            "test_BC_7_06_001_block_without_async_accepted: block without async must be accepted: {:?}",
            result
        );
    }

    /// VP-078 H1: async=true with on_error=continue (not block) → accepted.
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics.
    #[test]
    fn test_BC_7_06_001_async_true_with_continue_accepted() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "telemetry-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "telemetry.sh"
"#;
        // RED: todo!() in validate_async_block_invariant panics.
        let result = Registry::parse_str(toml);
        assert!(
            result.is_ok(),
            "test_BC_7_06_001_async_true_with_continue_accepted: async=true with on_error=continue must be accepted: {:?}",
            result
        );
    }
}

#[cfg(test)]
mod s15_01_vp078_harness_4_serde_default {
    use super::*;

    // -----------------------------------------------------------------------
    // Harness 4 — serde-default semantics (VP-078 H4, AC-002)
    // BC-7.06.001 postconditions 2 + 3
    // VP-077 Property #2 delegates field-absence testing to VP-078 H4.
    // -----------------------------------------------------------------------

    /// VP-078 H4a: explicit async=true → async_flag = true.
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics before we can assert.
    #[test]
    fn test_BC_7_06_001_async_explicit_true_parsed_as_true() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "telemetry-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "telemetry.sh"
"#;
        // RED: todo!() in validate_async_block_invariant panics.
        let registry = Registry::parse_str(toml).expect("valid toml must parse");
        let entry = &registry.hooks[0];
        assert!(
            entry.async_flag,
            "test_BC_7_06_001_async_explicit_true_parsed_as_true: explicit async=true must parse as true"
        );
    }

    /// VP-078 H4b: explicit async=false → async_flag = false.
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics.
    #[test]
    fn test_BC_7_06_001_async_explicit_false_parsed_as_false() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "blocking-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = false
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "blocking.sh"
"#;
        // RED: todo!() in validate_async_block_invariant panics.
        let registry = Registry::parse_str(toml).expect("valid toml must parse");
        let entry = &registry.hooks[0];
        assert!(
            !entry.async_flag,
            "test_BC_7_06_001_async_explicit_false_parsed_as_false: explicit async=false must parse as false"
        );
    }

    /// VP-078 H4c: async field absent → async_flag = false (serde-default).
    ///
    /// RED: validate_async_block_invariant() is todo!() — panics.
    #[test]
    fn test_BC_7_06_001_async_absent_defaults_to_false() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "blocking-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "blocking.sh"
"#;
        // RED: todo!() in validate_async_block_invariant panics.
        let registry = Registry::parse_str(toml).expect("valid toml must parse");
        let entry = &registry.hooks[0];
        assert!(
            !entry.async_flag,
            "test_BC_7_06_001_async_absent_defaults_to_false: absent async field must default to false (serde default — AC-002, DI-019 cite-by-reference)"
        );
    }

    /// VP-078 H4d: async = "true" (string, not bool) → parse error.
    ///
    /// TOML does not allow string where bool is expected.
    /// RED: this may fail at toml parse before reaching validate_async_block_invariant.
    #[test]
    fn test_BC_7_06_001_async_string_value_is_parse_error() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "bad-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = "true"
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "bad.sh"
"#;
        // String where bool expected → TOML type mismatch → parse error.
        let result = Registry::parse_str(toml);
        assert!(
            result.is_err(),
            "test_BC_7_06_001_async_string_value_is_parse_error: async field with string value must produce a parse error (AC-002, BC-7.06.001 PC3)"
        );
    }
}

// ---------------------------------------------------------------------------
// F-P2-011 — BC-7.06.001 Invariant 7: (name, event, tool) tuple uniqueness
// ---------------------------------------------------------------------------

#[cfg(test)]
mod f_p2_011_name_event_tool_uniqueness {
    use super::*;

    /// Identical (name, event, tool) tuples must be rejected with DuplicateEntry.
    #[test]
    fn test_validate_rejects_duplicate_name_event_tool_tuple() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "protect-secrets"
event = "PreToolUse"
tool = "Bash"
plugin = "hook-plugins/protect-secrets.wasm"
on_error = "block"

[[hooks]]
name = "protect-secrets"
event = "PreToolUse"
tool = "Bash"
plugin = "hook-plugins/protect-secrets.wasm"
on_error = "block"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::DuplicateEntry { name, event, tool } => {
                assert_eq!(name, "protect-secrets");
                assert_eq!(event, "PreToolUse");
                assert_eq!(tool.as_deref(), Some("Bash"));
            }
            other => panic!(
                "test_validate_rejects_duplicate_name_event_tool_tuple: expected DuplicateEntry, got {:?}",
                other
            ),
        }
    }

    /// Same name+event but different tool values must be accepted (BC-7.06.001 Invariant 7).
    /// This is the protect-secrets pattern: two tool surfaces for one named plugin.
    #[test]
    fn test_validate_accepts_same_name_event_different_tool() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "protect-secrets"
event = "PreToolUse"
tool = "Bash"
plugin = "hook-plugins/protect-secrets.wasm"
on_error = "block"

[[hooks]]
name = "protect-secrets"
event = "PreToolUse"
tool = "Read"
plugin = "hook-plugins/protect-secrets.wasm"
on_error = "block"
"#;
        let result = Registry::parse_str(toml);
        assert!(
            result.is_ok(),
            "test_validate_accepts_same_name_event_different_tool: same name+event with different tool must be accepted: {:?}",
            result
        );
    }

    /// F-P6-002 / BC-7.06.001 v1.7 Invariant 7 (F-P3-003 amendment):
    /// String equality, not regex equivalence — `tool='^Bash$'` and `tool='Bash'` are
    /// DISTINCT entries because the uniqueness key is the raw string value, not the set of
    /// tool surfaces the pattern matches. Two entries that happen to match the same tool
    /// surface via different regex strings are NOT duplicates.
    #[test]
    fn test_validate_treats_regex_variants_as_distinct_per_v1_5_amendment() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "regex-test"
event = "PreToolUse"
tool = "^Bash$"
on_error = "continue"
plugin = "hook-plugins/regex-test.wasm"

[[hooks]]
name = "regex-test"
event = "PreToolUse"
tool = "Bash"
on_error = "continue"
plugin = "hook-plugins/regex-test.wasm"
"#;

        let result = Registry::parse_str(toml);
        assert!(
            result.is_ok(),
            "BC-7.06.001 v1.7 Invariant 7: tool='^Bash$' and tool='Bash' MUST be DISTINCT entries \
(raw-string equality, not regex equivalence). Got: {:?}",
            result.err()
        );
    }

    /// Two entries with tool = None (absent) and matching name+event must be rejected.
    #[test]
    fn test_validate_treats_two_none_tools_as_duplicate() {
        let toml = r#"
schema_version = 2

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "hook-plugins/x.wasm"

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "hook-plugins/x.wasm"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::DuplicateEntry { name, event, tool } => {
                assert_eq!(name, "x");
                assert_eq!(event, "PreToolUse");
                assert!(tool.is_none(), "tool must be None for absent tool field");
            }
            other => panic!(
                "test_validate_treats_two_none_tools_as_duplicate: expected DuplicateEntry, got {:?}",
                other
            ),
        }
    }
}
