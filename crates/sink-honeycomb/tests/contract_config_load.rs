//! BC-3.06.005 — Config loading: api_key + dataset required; missing either is hard error.
//!
//! AC: Auth via `X-Honeycomb-Team` header from config (api_key required).
//! AC: Dataset configurable per sink instance (dataset required).
//! EC-001: Dataset name missing → Fail at config load.
//! Traces to: BC-3.06.005 postcondition 1 (SinkConfigCommon governs config parsing).

use sink_honeycomb::HoneycombSinkConfig;

// ---------------------------------------------------------------------------
// Happy-path: valid config returns Some(config)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_3_06_005_valid_config_returns_some() {
    let toml_src = r#"
        type = "honeycomb"
        name = "prod-hc"
        api_key = "hcaik_abc123"
        dataset = "factory-events"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    let config = result
        .expect("from_toml must not error on valid config")
        .expect("must return Some for type=honeycomb");
    assert_eq!(config.sink_type, "honeycomb");
    assert_eq!(config.common.name, "prod-hc");
    assert_eq!(config.api_key.as_deref(), Some("hcaik_abc123"));
    assert_eq!(config.dataset.as_deref(), Some("factory-events"));
}

#[test]
fn test_BC_3_06_005_enabled_defaults_to_true() {
    // BC-3.06.005: SinkConfigCommon.enabled defaults true when omitted.
    let toml_src = r#"
        type = "honeycomb"
        name = "defaults-sink"
        api_key = "key123"
        dataset = "ds"
    "#;
    let config = HoneycombSinkConfig::from_toml(toml_src)
        .expect("no error")
        .expect("Some");
    assert!(config.common.enabled, "enabled must default to true");
}

#[test]
fn test_BC_3_06_005_enabled_false_parsed_correctly() {
    let toml_src = r#"
        type = "honeycomb"
        name = "disabled-sink"
        api_key = "key123"
        dataset = "ds"
        enabled = false
    "#;
    let config = HoneycombSinkConfig::from_toml(toml_src)
        .expect("no error")
        .expect("Some");
    assert!(!config.common.enabled);
}

// ---------------------------------------------------------------------------
// EC-001: missing dataset → hard error
// ---------------------------------------------------------------------------

#[test]
fn test_BC_3_06_005_rejects_missing_dataset_ec001() {
    // EC-001: Dataset name missing → Fail at config load.
    let toml_src = r#"
        type = "honeycomb"
        name = "no-dataset"
        api_key = "hcaik_abc123"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "missing dataset must be a hard error (EC-001)"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.to_lowercase().contains("dataset"),
        "error message must mention 'dataset', got: {err}"
    );
}

#[test]
fn test_BC_3_06_005_rejects_empty_dataset_ec001() {
    // EC-001: empty dataset string is equally invalid.
    let toml_src = r#"
        type = "honeycomb"
        name = "empty-dataset"
        api_key = "hcaik_abc123"
        dataset = ""
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "empty dataset must be a hard error (EC-001)"
    );
}

// ---------------------------------------------------------------------------
// Missing / empty api_key → hard error (BC-3.NN.NNN-honeycomb-api-key-required)
// ---------------------------------------------------------------------------

#[test]
fn test_BC_3_06_005_rejects_missing_api_key() {
    let toml_src = r#"
        type = "honeycomb"
        name = "no-key"
        dataset = "factory-events"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "missing api_key must be a hard error"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.to_lowercase().contains("api_key") || err.to_lowercase().contains("api key"),
        "error message must mention api_key, got: {err}"
    );
}

#[test]
fn test_BC_3_06_005_rejects_empty_api_key() {
    let toml_src = r#"
        type = "honeycomb"
        name = "empty-key"
        api_key = ""
        dataset = "factory-events"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "empty api_key must be a hard error"
    );
}

// ---------------------------------------------------------------------------
// Wrong type → Ok(None), not an error
// ---------------------------------------------------------------------------

#[test]
fn test_BC_3_06_005_unknown_type_returns_none() {
    let toml_src = r#"
        type = "datadog"
        name = "wrong-type"
        api_key = "key"
        dataset = "ds"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(result.is_ok(), "wrong type must not error — only warn");
    assert!(
        result.unwrap().is_none(),
        "wrong type must return None"
    );
}

#[test]
fn test_BC_3_06_005_http_type_returns_none() {
    // Honeycomb config parser rejects "http" type as not its own.
    let toml_src = r#"
        type = "http"
        name = "http-type"
        api_key = "key"
        dataset = "ds"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// ---------------------------------------------------------------------------
// Boundary: whitespace-only api_key / dataset
// ---------------------------------------------------------------------------

#[test]
fn test_BC_3_06_005_rejects_whitespace_only_api_key() {
    let toml_src = r#"
        type = "honeycomb"
        name = "ws-key"
        api_key = "   "
        dataset = "factory-events"
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "whitespace-only api_key must be a hard error"
    );
}

#[test]
fn test_BC_3_06_005_rejects_whitespace_only_dataset() {
    let toml_src = r#"
        type = "honeycomb"
        name = "ws-ds"
        api_key = "real-key"
        dataset = "   "
    "#;
    let result = HoneycombSinkConfig::from_toml(toml_src);
    assert!(
        result.is_err(),
        "whitespace-only dataset must be a hard error (EC-001)"
    );
}
