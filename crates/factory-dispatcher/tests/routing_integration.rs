//! End-to-end routing integration test: construct a realistic registry
//! in a temp dir, parse it from disk (not a string), match against a
//! payload, group into tiers, and verify the output order.
//!
//! Distinct from the unit tests in `routing.rs` — those work with
//! in-memory registries; this one exercises `Registry::load` against a
//! real filesystem path, which is the codepath `main.rs` uses.

use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::registry::Registry;
use factory_dispatcher::routing::{group_by_priority, match_plugins};
use std::fs;

const REGISTRY: &str = r#"
schema_version = 1

[defaults]
timeout_ms = 3000
priority = 500

[[hooks]]
name = "brownfield-discipline"
event = "PreToolUse"
tool = "Edit|Write"
plugin = "plugins/brownfield-discipline.wasm"
priority = 10

[[hooks]]
name = "destructive-guard"
event = "PreToolUse"
tool = "^Bash$"
plugin = "plugins/destructive-guard.wasm"
priority = 10

[[hooks]]
name = "factory-branch-guard"
event = "PreToolUse"
tool = "Edit|Write"
plugin = "plugins/factory-branch-guard.wasm"
priority = 20

[[hooks]]
name = "validate-vp-consistency"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "plugins/validate-vp-consistency.wasm"
priority = 100

[[hooks]]
name = "disabled-entry"
event = "PreToolUse"
plugin = "plugins/x.wasm"
enabled = false
"#;

fn payload(event: &str, tool: &str) -> HookPayload {
    HookPayload {
        event_name: event.to_string(),
        tool_name: tool.to_string(),
        session_id: "sess-int".to_string(),
        tool_input: serde_json::Value::Null,
        tool_response: None,
    }
}

#[test]
fn load_from_disk_and_route_pretooluse_edit() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hooks-registry.toml");
    fs::write(&path, REGISTRY).unwrap();

    let reg = Registry::load(&path).unwrap();
    assert_eq!(reg.defaults.timeout_ms, 3000);

    let matched = match_plugins(&reg, &payload("PreToolUse", "Edit"));
    let tiers = group_by_priority(&reg, matched);

    // Expected tiers for Edit on PreToolUse:
    //   10 → brownfield-discipline
    //   20 → factory-branch-guard
    assert_eq!(tiers.len(), 2);
    assert_eq!(tiers[0][0].name, "brownfield-discipline");
    assert_eq!(tiers[1][0].name, "factory-branch-guard");
}

#[test]
fn load_from_disk_and_route_pretooluse_bash() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hooks-registry.toml");
    fs::write(&path, REGISTRY).unwrap();

    let reg = Registry::load(&path).unwrap();
    let matched = match_plugins(&reg, &payload("PreToolUse", "Bash"));
    let tiers = group_by_priority(&reg, matched);

    // Bash-only matches destructive-guard; nothing else matches Bash.
    assert_eq!(tiers.len(), 1);
    assert_eq!(tiers[0].len(), 1);
    assert_eq!(tiers[0][0].name, "destructive-guard");
}

#[test]
fn disabled_entries_do_not_match() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hooks-registry.toml");
    fs::write(&path, REGISTRY).unwrap();
    let reg = Registry::load(&path).unwrap();

    let matched = match_plugins(&reg, &payload("PreToolUse", "AnyTool"));
    assert!(matched.iter().all(|e| e.name != "disabled-entry"));
}

#[test]
fn missing_registry_surfaces_not_found_error() {
    let missing = std::path::PathBuf::from("/nonexistent/hooks-registry.toml");
    let err = Registry::load(&missing).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("not found"), "expected 'not found' in: {msg}");
}
