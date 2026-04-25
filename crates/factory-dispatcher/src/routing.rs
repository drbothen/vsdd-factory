//! Routing: given a parsed [`Registry`] and an incoming [`HookPayload`],
//! decide *which* plugins run and *in what order*.
//!
//! Outputs are handed to the execution layer (S-1.5 / S-1.6). This
//! module is deliberately state-free and easy to unit-test.

use crate::payload::HookPayload;
use crate::registry::{Registry, RegistryEntry};

/// Placeholder for the real `PluginResult` the execution layer will
/// return. Kept here so the routing contract can be expressed before
/// S-1.5 lands; will move when execution replaces the stub.
#[derive(Debug, Clone)]
pub struct PluginResultStub {
    pub plugin_name: String,
    pub outcome: PluginOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginOutcome {
    Continue,
    Block { reason: String },
    Error { message: String },
}

/// Select registry entries that apply to this payload.
///
/// An entry matches when:
/// 1. `entry.enabled` is true.
/// 2. `entry.event == payload.event_name`.
/// 3. Either `entry.tool` is `None`, or its compiled regex matches
///    `payload.tool_name`.
///
/// Invalid regexes are caught at registry load time ([`Registry::validate`]),
/// so this function compiles them on the hot path without falling back
/// to error surface area.
pub fn match_plugins<'a>(registry: &'a Registry, payload: &HookPayload) -> Vec<&'a RegistryEntry> {
    registry
        .hooks
        .iter()
        .filter(|e| e.enabled)
        .filter(|e| e.event == payload.event_name)
        .filter(|e| tool_matches(e, &payload.tool_name))
        .collect()
}

fn tool_matches(entry: &RegistryEntry, tool_name: &str) -> bool {
    match &entry.tool {
        None => true,
        Some(pattern) => match regex::Regex::new(pattern) {
            Ok(re) => re.is_match(tool_name),
            // Registry validation catches invalid patterns, so reaching
            // this branch implies registry-loader drift — treat as
            // non-match rather than panic so production stays up.
            Err(_) => false,
        },
    }
}

/// Group matched entries into priority tiers, ascending. Within a tier
/// entries keep their registry order. Execution runs tiers
/// sequentially and entries within a tier in parallel (S-1.6).
pub fn group_by_priority<'a>(
    registry: &'a Registry,
    matched: Vec<&'a RegistryEntry>,
) -> Vec<Vec<&'a RegistryEntry>> {
    // Stable sort on (priority, original index) keeps registry order
    // within a priority tier.
    let defaults = &registry.defaults;
    let mut indexed: Vec<(usize, u32, &RegistryEntry)> = matched
        .into_iter()
        .enumerate()
        .map(|(i, e)| (i, e.priority(defaults), e))
        .collect();
    indexed.sort_by_key(|(i, p, _)| (*p, *i));

    let mut tiers: Vec<Vec<&RegistryEntry>> = Vec::new();
    let mut current_priority: Option<u32> = None;
    for (_, p, entry) in indexed {
        if Some(p) != current_priority {
            tiers.push(Vec::new());
            current_priority = Some(p);
        }
        tiers.last_mut().unwrap().push(entry);
    }
    tiers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload::HookPayload;
    use crate::registry::Registry;

    fn registry_fixture() -> Registry {
        let toml = r#"
schema_version = 1

[[hooks]]
name = "fast-pre"
event = "PreToolUse"
plugin = "a.wasm"
priority = 10

[[hooks]]
name = "bash-only"
event = "PreToolUse"
tool = "^Bash$"
plugin = "b.wasm"
priority = 100

[[hooks]]
name = "edit-or-write"
event = "PreToolUse"
tool = "Edit|Write"
plugin = "c.wasm"
priority = 100

[[hooks]]
name = "disabled"
event = "PreToolUse"
plugin = "d.wasm"
enabled = false

[[hooks]]
name = "post"
event = "PostToolUse"
plugin = "e.wasm"
priority = 50

[[hooks]]
name = "all-tools"
event = "PreToolUse"
plugin = "f.wasm"
priority = 200
"#;
        Registry::parse_str(toml).unwrap()
    }

    fn payload(event: &str, tool: &str) -> HookPayload {
        HookPayload {
            event_name: event.to_string(),
            tool_name: tool.to_string(),
            session_id: "s".to_string(),
            tool_input: serde_json::Value::Null,
            tool_response: None,
        }
    }

    #[test]
    fn match_filters_by_event_name() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("PostToolUse", "Bash"));
        let names: Vec<_> = matched.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["post"]);
    }

    #[test]
    fn match_skips_disabled_entries() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("PreToolUse", "Bash"));
        assert!(matched.iter().all(|e| e.name != "disabled"));
    }

    #[test]
    fn match_includes_no_tool_entries_for_any_tool() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("PreToolUse", "WeirdTool"));
        let names: Vec<_> = matched.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"fast-pre"));
        assert!(names.contains(&"all-tools"));
        assert!(!names.contains(&"bash-only"));
    }

    #[test]
    fn match_respects_tool_regex_anchoring() {
        let reg = registry_fixture();
        // "^Bash$" does not match "BashTool".
        let matched = match_plugins(&reg, &payload("PreToolUse", "BashTool"));
        let names: Vec<_> = matched.iter().map(|e| e.name.as_str()).collect();
        assert!(!names.contains(&"bash-only"));
    }

    #[test]
    fn match_regex_alternation() {
        let reg = registry_fixture();
        let m = match_plugins(&reg, &payload("PreToolUse", "Edit"));
        assert!(m.iter().any(|e| e.name == "edit-or-write"));

        let m = match_plugins(&reg, &payload("PreToolUse", "Write"));
        assert!(m.iter().any(|e| e.name == "edit-or-write"));

        let m = match_plugins(&reg, &payload("PreToolUse", "Bash"));
        assert!(!m.iter().any(|e| e.name == "edit-or-write"));
    }

    #[test]
    fn group_orders_tiers_ascending() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("PreToolUse", "Bash"));
        let tiers = group_by_priority(&reg, matched);
        // Expected tiers for Bash on PreToolUse:
        //   priority 10:  fast-pre
        //   priority 100: bash-only
        //   priority 200: all-tools
        assert_eq!(tiers.len(), 3);
        assert_eq!(tiers[0][0].name, "fast-pre");
        assert_eq!(tiers[1][0].name, "bash-only");
        assert_eq!(tiers[2][0].name, "all-tools");
    }

    #[test]
    fn group_keeps_registry_order_within_tier() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("PreToolUse", "Edit"));
        // Edit matches: fast-pre(10), edit-or-write(100), all-tools(200)
        let tiers = group_by_priority(&reg, matched);
        assert_eq!(tiers.len(), 3);
        assert_eq!(tiers[1].len(), 1);
        assert_eq!(tiers[1][0].name, "edit-or-write");
    }

    #[test]
    fn group_packs_multiple_entries_at_same_priority() {
        let toml = r#"
schema_version = 1

[[hooks]]
name = "a"
event = "PreToolUse"
plugin = "a.wasm"
priority = 100

[[hooks]]
name = "b"
event = "PreToolUse"
plugin = "b.wasm"
priority = 100

[[hooks]]
name = "c"
event = "PreToolUse"
plugin = "c.wasm"
priority = 50
"#;
        let reg = Registry::parse_str(toml).unwrap();
        let m = match_plugins(&reg, &payload("PreToolUse", ""));
        let tiers = group_by_priority(&reg, m);
        assert_eq!(tiers.len(), 2);
        assert_eq!(tiers[0].len(), 1);
        assert_eq!(tiers[0][0].name, "c");
        assert_eq!(tiers[1].len(), 2);
        assert_eq!(tiers[1][0].name, "a");
        assert_eq!(tiers[1][1].name, "b");
    }

    #[test]
    fn group_returns_empty_for_no_matches() {
        let reg = registry_fixture();
        let matched = match_plugins(&reg, &payload("SessionStart", ""));
        let tiers = group_by_priority(&reg, matched);
        assert!(tiers.is_empty());
    }
}
