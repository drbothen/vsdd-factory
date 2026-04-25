//! Validates that the auto-generated `plugins/vsdd-factory/hooks-registry.toml`
//! parses cleanly through the production `Registry::load` codepath. The
//! registry is produced by `scripts/generate-registry-from-hooks-json.sh`
//! (S-2.2); this test is the schema-side gate that the generator
//! output is structurally sound.
//!
//! What this test does NOT cover (deliberate scope cut):
//!   * Whether each entry's `script_path` resolves to an extant `.sh` —
//!     the bats suite at `plugins/vsdd-factory/tests/generate-registry.bats`
//!     owns that check, since it's a filesystem invariant tied to the
//!     plugin layout, not the registry schema.
//!   * Whether the legacy-bash-adapter actually runs the script — that's
//!     the adapter's own integration suite (S-2.1).

use factory_dispatcher::registry::{REGISTRY_SCHEMA_VERSION, Registry};
use std::path::PathBuf;

fn registry_path() -> PathBuf {
    // CARGO_MANIFEST_DIR points at crates/factory-dispatcher; walk two
    // levels up to repo root, then into the plugin tree. Constructing
    // the path this way (rather than hardcoding "../..") makes the test
    // independent of the cwd cargo decides to use.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("plugins")
        .join("vsdd-factory")
        .join("hooks-registry.toml")
}

#[test]
fn loads_generated_registry_from_disk() {
    let path = registry_path();
    assert!(
        path.exists(),
        "expected hooks-registry.toml at {} — run \
         scripts/generate-registry-from-hooks-json.sh",
        path.display()
    );
    let registry = Registry::load(&path).expect("registry must parse cleanly");
    assert_eq!(registry.schema_version, REGISTRY_SCHEMA_VERSION);

    // Sanity bound: the v0.79.x inventory is ~30 hooks (with one
    // duplicate name across matchers). If the count drops below 20
    // something has been silently elided; if it explodes past 100
    // someone has misclassified the source.
    assert!(
        registry.hooks.len() > 20,
        "registry must include the v0.79.x inventory; got {} entries",
        registry.hooks.len()
    );
    assert!(
        registry.hooks.len() < 100,
        "registry hook count is implausibly high ({}); inspect the \
         generator for a duplicated emit",
        registry.hooks.len()
    );
}

#[test]
fn every_entry_routes_through_legacy_bash_adapter() {
    let registry = Registry::load(&registry_path()).unwrap();
    for entry in &registry.hooks {
        let plugin = entry
            .plugin
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        assert_eq!(
            plugin, "legacy-bash-adapter.wasm",
            "entry {:?} routes through {} — every v0.79.x hook should \
             go through the bash adapter at this point in the migration",
            entry.name, plugin
        );
    }
}

#[test]
fn every_entry_carries_a_script_path() {
    // legacy-bash-adapter (S-2.1) demands `plugin_config.script_path`;
    // the generator must emit it for every entry or the adapter will
    // refuse the payload at runtime. Catch that at registry-load time
    // rather than waiting for the first hook fire.
    let registry = Registry::load(&registry_path()).unwrap();
    for entry in &registry.hooks {
        let cfg_json = entry.config_as_json();
        let script_path = cfg_json
            .get("script_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        assert!(
            !script_path.is_empty(),
            "entry {:?} has empty config.script_path — \
             legacy-bash-adapter will refuse this entry",
            entry.name
        );
        assert!(
            script_path.starts_with("hooks/") && script_path.ends_with(".sh"),
            "entry {:?} script_path={:?} should be `hooks/<name>.sh` \
             (relative to ${{CLAUDE_PLUGIN_ROOT}})",
            entry.name,
            script_path
        );
    }
}
