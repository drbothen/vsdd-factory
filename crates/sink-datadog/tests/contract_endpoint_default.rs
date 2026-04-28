//! AC-4 (regional endpoints) + default endpoint.
//!
//! Verifies that when endpoint is absent in config, DatadogSink posts to
//! DEFAULT_DATADOG_ENDPOINT (https://http-intake.logs.datadoghq.com/api/v2/logs).
//!
//! v1.1 BC candidate: BC-3.NN.NNN-datadog-regional-endpoint.

use sink_datadog::{DEFAULT_DATADOG_ENDPOINT, DatadogSinkConfig};

/// When endpoint absent, effective_endpoint returns the us1 default.
///
/// Exercises: DatadogSinkConfig::effective_endpoint with no endpoint set.
#[test]
fn test_BC_3_06_005_default_endpoint_is_us1_when_absent() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "default-endpoint-sink"
api_key = "test-key"
"#;

    let config = DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    assert!(
        config.endpoint.is_none(),
        "endpoint field must be None when not specified"
    );

    let effective = config.effective_endpoint();
    assert_eq!(
        effective, DEFAULT_DATADOG_ENDPOINT,
        "effective_endpoint must return the us1 default when endpoint is absent"
    );
    assert!(
        effective.contains("/api/v2/logs"),
        "default endpoint must include /api/v2/logs path"
    );
}

/// When endpoint is specified, effective_endpoint returns the caller-supplied URL.
///
/// Exercises: DatadogSinkConfig::effective_endpoint with eu1 endpoint.
#[test]
fn test_BC_3_06_005_explicit_endpoint_overrides_default() {
    let eu1 = "https://http-intake.logs.datadoghq.eu/api/v2/logs";
    let toml = format!(
        r#"
schema_version = 1
type = "datadog"
name = "eu1-sink"
api_key = "test-key"
endpoint = "{eu1}"
"#
    );

    let config = DatadogSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    assert_eq!(
        config.effective_endpoint(),
        eu1,
        "effective_endpoint must return the explicitly-set eu1 URL"
    );
}

/// DEFAULT_DATADOG_ENDPOINT constant matches the Datadog documented us1 URL AND
/// effective_endpoint returns it when no endpoint is configured.
///
/// The constant value can be verified at compile time; the accessor requires implementation.
/// This test exercises both so it fails on the unimplemented!() accessor.
#[test]
fn test_BC_3_06_005_default_endpoint_constant_value_and_accessible_via_config() {
    // Constant-level check (spec compliance).
    assert_eq!(
        DEFAULT_DATADOG_ENDPOINT, "https://http-intake.logs.datadoghq.com/api/v2/logs",
        "DEFAULT_DATADOG_ENDPOINT must equal the documented Datadog us1 logs intake URL"
    );

    // Runtime check: the config accessor must return this constant.
    // This panics on unimplemented!() until GREEN.
    let toml = r#"
schema_version = 1
type = "datadog"
name = "constant-check-sink"
api_key = "test-key"
"#;
    let config = DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    assert_eq!(
        config.effective_endpoint(),
        DEFAULT_DATADOG_ENDPOINT,
        "effective_endpoint must return DEFAULT_DATADOG_ENDPOINT when endpoint is absent"
    );
}
