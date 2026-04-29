//! S-4.07 end-to-end observability integration test root.
//!
//! This file is the integration test binary entry point. It declares the
//! `integration_tests` module (from the sub-directory) and re-exports all
//! test sub-modules so `cargo test` discovers them under this binary.
//!
//! Each sub-module corresponds to one or more ACs from S-4.07:
//! - harness      — shared mock server helpers
//! - zero_disk    — AC-1 (BC-3.03.007, BC-3.03.001)
//! - hybrid       — AC-2 (BC-3.05.002, BC-3.05.003)
//! - routing      — AC-3 (BC-3.02.009, BC-3.01.004)
//! - dlq          — AC-4 (BC-3.07.003, BC-3.07.004)
//! - circuit_breaker — AC-5 (v1.1 BC candidates)
//! - lifecycle    — AC-6/7/8 (BC-3.02.010/011/012/015)
//! - config_load  — AC-10/15 (BC-3.01.002, BC-3.05.001)
//! - otlp_mapping — AC-11 (BC-3.05.003)
//! - internal_sink_error — AC-12 (BC-3.07.002)
//! - datadog      — AC-13 (v1.1 BC candidate)
//! - honeycomb    — AC-14 (v1.1 BC candidate)

mod integration_tests;
