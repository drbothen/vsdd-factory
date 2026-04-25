//! `hooks-registry.toml` parser and type declarations.
//!
//! The registry is the dispatcher's source of truth for *which* plugins
//! exist, *what* events they react to, and *what capabilities* they
//! hold. S-1.2 defines the types; S-1.5 consumes capabilities during
//! plugin instantiation; S-2.2 auto-generates the file from the existing
//! v0.79.x hooks.json.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Current registry schema version. The loader refuses anything else
/// so an unreleased schema change can't silently mis-parse.
pub const REGISTRY_SCHEMA_VERSION: u32 = 1;

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
         Regenerate hooks-registry.toml or upgrade the dispatcher."
    )]
    SchemaVersion { got: u32, expected: u32 },
    #[error("registry entry '{name}': invalid tool regex `{pattern}`: {source}")]
    ToolRegex {
        name: String,
        pattern: String,
        #[source]
        source: regex::Error,
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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
}

fn default_enabled() -> bool {
    true
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
}

/// The whole parsed registry.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_toml() -> &'static str {
        r#"
schema_version = 1

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
        assert_eq!(reg.schema_version, 1);
        assert_eq!(reg.hooks.len(), 1);
        assert_eq!(reg.hooks[0].name, "commit");
        assert_eq!(reg.hooks[0].event, "PostToolUse");
        assert_eq!(reg.hooks[0].tool.as_deref(), Some("Bash"));
        assert!(reg.hooks[0].enabled);
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
        let toml = r#"
schema_version = 2

[[hooks]]
name = "x"
event = "PreToolUse"
plugin = "x.wasm"
"#;
        let err = Registry::parse_str(toml).unwrap_err();
        match err {
            RegistryError::SchemaVersion { got, expected } => {
                assert_eq!(got, 2);
                assert_eq!(expected, 1);
            }
            other => panic!("expected SchemaVersion, got {other:?}"),
        }
    }

    #[test]
    fn rejects_invalid_tool_regex() {
        let toml = r#"
schema_version = 1

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
schema_version = 1

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
schema_version = 1

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
schema_version = 1

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
schema_version = 1

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
schema_version = 1

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
schema_version = 1

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
