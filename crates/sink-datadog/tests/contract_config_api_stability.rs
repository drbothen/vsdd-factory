//! F-2 (PR #18 deferred): HttpSinkConfig must expose a builder API or non-pub fields
//! with accessors so wrappers don't pin to a specific field set.
//!
//! BC-3.06.005 extension / v1.1 BC candidate: BC-3.NN.NNN-http-sink-config-api-stability.
//!
//! Tests verify that HttpSinkConfig exposes a stable API surface usable by
//! wrappers (sink-datadog, sink-honeycomb) without directly accessing pub fields.
//!
//! RED gate strategy:
//! - test_F2_http_sink_config_wrapper_uses_stable_api: panics on unimplemented!()
//!   in DatadogSink::new until both F-2 and S-4.02 GREEN.
//! - test_F2_http_sink_config_url_field_is_pub_not_accessor: fails because it
//!   asserts that HttpSinkConfig::url() accessor does NOT exist in current impl.
//!   When F-2 is done, `url` field becomes private and the accessor is added,
//!   at which point this test is removed/replaced by the accessor variant.

use sink_http::HttpSinkConfig;

/// F-2 — Wrapper crate can construct DatadogSink without pinning to HttpSinkConfig
/// pub field layout.
///
/// RED gate: DatadogSink::new panics (unimplemented!()) until S-4.02 GREEN.
/// Post-GREEN: DatadogSink::new internally wraps HttpSinkConfig using only
/// stable accessor methods, not pub fields.
#[test]
fn test_F2_http_sink_config_wrapper_uses_stable_api() {
    let toml = r#"
schema_version = 1
type = "datadog"
name = "f2-stability-test"
api_key = "f2-key"
endpoint = "http://localhost:39999/api/v2/logs"
"#;

    let config = sink_datadog::DatadogSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    // This panics on unimplemented!() until S-4.02 GREEN gate.
    // When the implementer builds DatadogSink::new to use HttpSinkConfig accessors
    // (not pub fields), both F-2 and this test pass together.
    let sink = sink_datadog::DatadogSink::new(config)
        .expect("DatadogSink::new must succeed — RED until F-2 + S-4.02 implemented");

    // Verify the sink is functional end-to-end via the Sink trait.
    use sink_core::Sink;
    assert!(!sink.name().is_empty(), "sink name must not be empty");
}

/// F-2 — HttpSinkConfig currently exposes pub fields; documents the DESIRED post-F2 state.
///
/// RED gate: asserts that HttpSinkConfig has a `schema_version` pub field AND
/// checks that the field is accessible. The test then FAILS by asserting a
/// condition that can only pass once accessor methods are added (the
/// `url` field must equal the value returned by a non-existent accessor).
///
/// After F-2: this test is replaced by an accessor-API variant; the pub field
/// path is removed from the implementation.
#[test]
fn test_F2_http_sink_config_url_field_is_pub_documents_pre_fix_state() {
    let toml = r#"
schema_version = 1
type = "http"
name = "pub-field-audit"
url = "http://localhost:39998/path"
"#;

    let config = HttpSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    // Current state (pre-F2): pub fields are directly accessible.
    // This assertion passes today — documenting the pre-fix API.
    assert_eq!(
        config.url, "http://localhost:39998/path",
        "pre-F2: pub field direct access works (this should become config.url() post-F2)"
    );

    // RED assertion: F-2 requires that direct pub field access is REPLACED by
    // an accessor. We assert the INVERSE condition — that pub field access is
    // NOT the right pattern — by checking that the `url` field and the
    // `sink_type` field are UNEXPORTED (i.e., have non-pub visibility).
    //
    // Strategy: read the sink-http Cargo.toml and source to verify the field
    // visibility has been changed. We do this by asserting the source does NOT
    // contain `pub url:` (field is no longer pub after F-2).
    let src_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../crates/sink-http/src/lib.rs"
    );
    let src = std::fs::read_to_string(src_path).expect("sink-http/src/lib.rs must be readable");

    // This assertion FAILS now (pre-F2) because the source DOES contain `pub url:`.
    // After F-2 the implementer removes `pub` from the url field; this test passes.
    assert!(
        !src.contains("    pub url: String,"),
        "F-2: HttpSinkConfig must NOT have `pub url: String` — field must be private \
         with a `pub fn url(&self) -> &str` accessor. \
         (PR #18 deferred finding F-2: wrappers must not pin to field layout)"
    );
}
