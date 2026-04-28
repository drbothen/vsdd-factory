//! S-4.07 end-to-end observability integration tests.
//!
//! Exercises the full Router → SinkRegistry → individual sink driver path
//! under realistic configurations. Each sub-module maps to one or more ACs
//! from S-4.07 and traces to specific behavioral contracts.
//!
//! Entry point: `Router::submit()` (unless noted in SUT boundary exceptions).
//!
//! ## Sub-modules
//! - `harness`      — shared mock server helpers (OTLP gRPC, httpmock, temp dirs)
//! - `zero_disk`    — AC-1: otel-grpc only, no file sink (BC-3.03.007, BC-3.03.001)
//! - `hybrid`       — AC-2: file + otel-grpc fan-out (BC-3.05.002, BC-3.05.003)
//! - `routing`      — AC-3: multi-sink routing filter (BC-3.02.009, BC-3.01.004)
//! - `dlq`          — AC-4: DLQ on 5xx retry exhaustion (BC-3.07.003, BC-3.07.004)
//! - `circuit_breaker` — AC-5: circuit breaker opens after N failures
//! - `lifecycle`    — AC-6/7/8: tag enrichment, disabled sink, shutdown drain
//! - `config_load`  — AC-10/15: unknown sink type + config load integration
//! - `otlp_mapping` — AC-11: OTLP LogRecord field mapping
//! - `internal_sink_error` — AC-12: cross-sink internal.sink_error emission
//! - `datadog`      — AC-13: DatadogSink mock HTTP integration
//! - `honeycomb`    — AC-14: HoneycombSink mock HTTP integration

pub mod harness;

pub mod zero_disk;
pub mod hybrid;
pub mod routing;
pub mod dlq;
pub mod circuit_breaker;
pub mod lifecycle;
pub mod config_load;
pub mod otlp_mapping;
pub mod internal_sink_error;
pub mod datadog;
pub mod honeycomb;
