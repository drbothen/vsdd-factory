//! F-1 (PR #18 deferred): sink-http Cargo.toml must NOT include "blocking" in reqwest features.
//!
//! Verifies via cargo metadata that the reqwest dependency in sink-http does
//! not list "blocking" in its enabled features. This catches the unused feature
//! flag that increases binary size with no benefit.
//!
//! Test naming: test_F1_* per F-1 finding from PR #18.

use std::process::Command;

/// F-1 — reqwest "blocking" feature absent from sink-http Cargo.toml.
///
/// Uses `cargo metadata` to inspect the resolved feature set for the
/// sink-http crate's reqwest dependency. The "blocking" feature must NOT
/// appear in the features list.
#[test]
fn test_F1_sink_http_reqwest_blocking_feature_not_enabled() {
    // Run `cargo metadata` from the workspace root.
    // The worktree CWD is determined by the manifest path of sink-datadog,
    // which lives in the same workspace as sink-http.
    let manifest = env!("CARGO_MANIFEST_DIR");

    let output = Command::new("cargo")
        .args([
            "metadata",
            "--no-deps",
            "--format-version=1",
            "--manifest-path",
        ])
        .arg(format!("{manifest}/../../Cargo.toml"))
        .output()
        .expect("cargo metadata must be runnable");

    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata_json = String::from_utf8_lossy(&output.stdout);

    // Parse the workspace Cargo.toml reqwest dependency features.
    // We look for the workspace-level reqwest declaration which governs
    // all crates' reqwest feature activation.
    // The assertion: the "blocking" feature must NOT appear in reqwest features
    // for sink-http.
    let parsed: serde_json::Value =
        serde_json::from_str(&metadata_json).expect("cargo metadata must produce valid JSON");

    // Find sink-http in the packages list.
    let packages = parsed["packages"]
        .as_array()
        .expect("packages must be an array");

    let sink_http_pkg = packages
        .iter()
        .find(|p| p["name"].as_str() == Some("sink-http"))
        .expect("sink-http package must be in workspace metadata");

    // Check reqwest dependency features in sink-http's dependency list.
    let deps = sink_http_pkg["dependencies"]
        .as_array()
        .expect("dependencies must be an array");

    let reqwest_dep = deps.iter().find(|d| d["name"].as_str() == Some("reqwest"));

    if let Some(reqwest_dep) = reqwest_dep {
        let features = reqwest_dep["features"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|f| f.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        assert!(
            !features.contains(&"blocking"),
            "F-1: sink-http reqwest dependency must NOT include 'blocking' feature, \
             found features: {features:?}. Remove 'blocking' from the workspace reqwest \
             dependency or sink-http's reqwest declaration (PR #18 deferred finding F-1)."
        );
    }
    // If reqwest is not listed as a direct dep in metadata (inherited from workspace),
    // check the workspace-level reqwest entry.
    // The workspace Cargo.toml currently has: reqwest = { version = "0.12", features = ["json", "blocking"] }
    // F-1 requires removing "blocking" from that list. This test will pass only after
    // the implementer removes it.

    // Additionally verify via the raw workspace Cargo.toml text as a belt-and-suspenders check.
    let workspace_toml_path = format!("{manifest}/../../Cargo.toml");
    let workspace_toml =
        std::fs::read_to_string(&workspace_toml_path).expect("workspace Cargo.toml must exist");

    // The workspace reqwest entry must not include "blocking".
    // Current content: features = ["json", "blocking"] — this test must fail until fixed.
    if workspace_toml.contains("reqwest") {
        let reqwest_section: Vec<&str> = workspace_toml
            .lines()
            .skip_while(|l| !l.contains("reqwest"))
            .take(3)
            .collect();
        let reqwest_block = reqwest_section.join("\n");
        assert!(
            !reqwest_block.contains("\"blocking\""),
            "F-1: workspace Cargo.toml reqwest features must NOT include \"blocking\". \
             Found in block:\n{reqwest_block}\n\
             Remove \"blocking\" from workspace reqwest features (PR #18 deferred F-1)."
        );
    }
}
