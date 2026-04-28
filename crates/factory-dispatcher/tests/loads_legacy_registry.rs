//! Validates that `plugins/vsdd-factory/hooks-registry.toml` parses cleanly
//! through the production `Registry::load` codepath.
//!
//! The registry is now a permanent, human-edited file that contains both
//! native WASM hook entries (capture-commit-activity, capture-pr-activity,
//! block-ai-attribution — ported in S-3.01/S-3.02/S-3.03) and legacy-bash-
//! adapter entries for hooks that have not yet been ported. The v0.79.x →
//! v1.0 migration that required all entries to route through the bash adapter
//! is complete; the generator script is retired.
//!
//! What this test does NOT cover (deliberate scope cut):
//!   * Whether each bash-adapter entry's `script_path` resolves to an extant
//!     `.sh` — the bats suite at `plugins/vsdd-factory/tests/` owns that.
//!   * Whether the legacy-bash-adapter actually runs the script — that's the
//!     adapter's own integration suite (S-2.1).

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
