//! BC-3.NN.NNN-honeycomb-dataset-url-routing — Endpoint URL is built from
//! base + dataset name embedded in the path.
//!
//! AC: Sends events to Honeycomb Events API (`/1/events/<dataset>`).
//! Traces to: BC-3.01.001 postcondition 1 + v1.1 BC candidate
//! BC-3.NN.NNN-honeycomb-dataset-url-routing.

use sink_honeycomb::{HONEYCOMB_BASE_URL, HoneycombSinkConfig};

fn valid_config(dataset: &str) -> HoneycombSinkConfig {
    let toml_src = format!(
        r#"
        type = "honeycomb"
        name = "url-test"
        api_key = "test-key-abc"
        dataset = "{dataset}"
        "#
    );
    HoneycombSinkConfig::from_toml(&toml_src)
        .expect("no error")
        .expect("Some")
}

#[test]
fn test_BC_3_01_001_endpoint_url_includes_dataset_in_path() {
    // AC: endpoint URL = https://api.honeycomb.io/1/events/<dataset>
    let config = valid_config("factory-events");
    let url = config.endpoint_url();
    assert_eq!(
        url, "https://api.honeycomb.io/1/events/factory-events",
        "endpoint URL must embed dataset in path"
    );
}

#[test]
fn test_BC_3_01_001_endpoint_url_uses_honeycomb_base() {
    // Base URL constant is correct.
    assert_eq!(HONEYCOMB_BASE_URL, "https://api.honeycomb.io/1/events");

    let config = valid_config("my-dataset");
    let url = config.endpoint_url();
    assert!(
        url.starts_with(HONEYCOMB_BASE_URL),
        "endpoint URL must start with HONEYCOMB_BASE_URL"
    );
}

#[test]
fn test_BC_3_01_001_endpoint_url_different_datasets_produce_different_urls() {
    let url_a = valid_config("dataset-alpha").endpoint_url();
    let url_b = valid_config("dataset-beta").endpoint_url();
    assert_ne!(url_a, url_b, "different datasets must yield different URLs");
    assert!(url_a.ends_with("/dataset-alpha"));
    assert!(url_b.ends_with("/dataset-beta"));
}

#[test]
fn test_BC_3_01_001_endpoint_url_dataset_with_hyphens() {
    // Dataset names with hyphens are common in Honeycomb.
    let config = valid_config("factory-prod-events");
    let url = config.endpoint_url();
    assert_eq!(url, "https://api.honeycomb.io/1/events/factory-prod-events");
}

#[test]
fn test_BC_3_01_001_endpoint_url_no_trailing_slash_in_base() {
    // The base URL must not result in double-slash when joined.
    let config = valid_config("ds");
    let url = config.endpoint_url();
    assert!(
        !url.contains("//1/events"),
        "URL must not contain double-slash: {url}"
    );
}
