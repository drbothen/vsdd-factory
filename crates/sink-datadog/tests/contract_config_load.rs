//! AC-3 (api_key required), schema_version, type validation.
//!
//! BC-3.06.005 postcondition 1: SinkConfigCommon defaults govern config parsing.
//! EC-001: missing api_key is a hard error at config load time.

use sink_datadog::DatadogSinkConfig;

/// BC-3.06.005 — valid config with api_key + optional endpoint + tags parses correctly.
///
/// Exercises: DatadogSinkConfig::from_toml with full stanza.
#[test]
fn test_BC_3_06_005_valid_config_with_api_key_and_endpoint_parses() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "prod-datadog"
api_key = "dd-key-xyz"
endpoint = "https://http-intake.logs.datadoghq.eu/api/v2/logs"

[tags]
env = "prod"
team = "factory"
"#;

    let result = DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must not error on valid stanza")
        .expect("must return Some for type=datadog");

    assert_eq!(result.api_key, "dd-key-xyz");
    assert_eq!(
        result.endpoint.as_deref(),
        Some("https://http-intake.logs.datadoghq.eu/api/v2/logs")
    );
    assert_eq!(result.common.name, "prod-datadog");
    assert!(result.common.tags.contains_key("env"));
    assert!(result.common.tags.contains_key("team"));
}

/// BC-3.06.005 — minimal config (no endpoint, no tags) parses correctly.
///
/// Exercises: DatadogSinkConfig::from_toml with minimal required fields only.
#[test]
fn test_BC_3_06_005_minimal_config_parses() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "minimal-sink"
api_key = "some-key"
"#;

    let result = DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    assert_eq!(result.api_key, "some-key");
    assert!(result.endpoint.is_none(), "endpoint must be None when absent");
    assert!(result.common.enabled, "enabled must default to true");
    assert!(result.common.tags.is_empty());
}

/// EC-001 — missing api_key is a hard error at config load time.
///
/// Exercises: DatadogSinkConfig::from_toml without api_key field.
/// BC-3.06.005 extension: required Datadog-specific field missing -> Err.
#[test]
fn test_BC_3_06_005_missing_api_key_is_hard_error() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "no-key-sink"
"#;

    let result = DatadogSinkConfig::from_toml(toml);

    assert!(
        result.is_err(),
        "missing api_key must return Err — got Ok instead"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.to_lowercase().contains("api_key")
            || err_msg.to_lowercase().contains("api key")
            || err_msg.to_lowercase().contains("missing"),
        "error message must mention api_key or missing, got: {err_msg}"
    );
}

/// EC-001 — empty api_key string is a hard error at config load time.
#[test]
fn test_BC_3_06_005_empty_api_key_is_hard_error() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "empty-key-sink"
api_key = ""
"#;

    let result = DatadogSinkConfig::from_toml(toml);

    assert!(
        result.is_err(),
        "empty api_key must return Err — got Ok instead"
    );
}

/// schema_version != 1 is a hard error (consistent with sink-http BC-3.01.003 pattern).
#[test]
fn test_BC_3_06_005_schema_version_mismatch_is_hard_error() {
    let toml = r#"
schema_version = 2
type = "datadog"
name = "future-sink"
api_key = "some-key"
"#;

    let result = DatadogSinkConfig::from_toml(toml);

    assert!(
        result.is_err(),
        "schema_version != 1 must return Err — got Ok instead"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("schema_version"),
        "error message must mention 'schema_version', got: {err_msg}"
    );
}

/// Unknown sink type warns and returns Ok(None) — consistent with sink-http BC-3.01.002.
#[test]
fn test_BC_3_06_005_unknown_sink_type_returns_none() {
    let toml = r#"
schema_version = 1
type = "not-datadog"
name = "mystery-sink"
api_key = "some-key"
"#;

    let result = DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must not return Err for unknown type");

    assert!(
        result.is_none(),
        "unknown sink type must return Ok(None) so the rest of config loads"
    );
}
