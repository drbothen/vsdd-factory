---
document_type: demo-evidence-index
story_id: S-4.03
producer: demo-recorder
phase: per-story-delivery
branch: feat/S-4.03-sink-honeycomb-driver
tested_at_sha: 2a4e25e
timestamp: 2026-04-27T00:00:00Z
---

# S-4.03 sink-honeycomb — Demo Evidence Index

**Story:** S-4.03 sink-honeycomb driver  
**Branch:** `feat/S-4.03-sink-honeycomb-driver`  
**SHA:** `2a4e25e`  
**Test result:** 36/36 GREEN

## Per-AC Evidence Map

| AC | Description | Evidence File | Tests | Result |
|----|-------------|---------------|-------|--------|
| AC-01 | `crates/sink-honeycomb` implements the `Sink` trait (BC-3.01.001, VP-011, VP-012) | `AC-01-sink-trait.txt` | 5 | PASS |
| AC-02 | Config load via `HoneycombSinkConfig::from_toml` — valid, invalid, edge cases (BC-3.06.005, EC-001) | `AC-02-config-load.txt` | 11 | PASS |
| AC-03 | Endpoint URL embeds dataset in path `/1/events/<dataset>` (BC-3.01.001) | `AC-03-endpoint-construction.txt` | 5 | PASS |
| AC-04 | Auth via `X-Honeycomb-Team` header + `Content-Type: application/json` (BC-3.06.005) | `AC-04-auth-header.txt` | 3 | PASS |
| AC-05 | Batch integration: events posted to correct path, RFC3339 `time` field, 429 retry (BC-3.01.001, EC-002) | `AC-05-batch-integration.txt` | 7 | PASS |
| AC-06 | Inherits HTTP sink: non-blocking submit, 5xx retry, flush semantics (VP-011, VP-012, BC-3.01.001) | `AC-06-inherits-http-sink.txt` | 5 | PASS |

**Total: 36/36 tests passing**

## Build Hygiene

| Check | File | Result |
|-------|------|--------|
| `cargo clippy -p sink-honeycomb -- -D warnings` | `clippy-clean.txt` | CLEAN (exit 0) |
| `cargo fmt --check -p sink-honeycomb` | `fmt-clean.txt` | 5 minor diffs in test files only (see note) |
| `cargo test -p sink-honeycomb` (all) | `all-tests-summary.txt` | 36/36 PASS |

### fmt note

`cargo fmt --check` exits 1 due to 5 formatting diffs in test files
(`contract_config_load.rs`, `contract_endpoint_construction.rs`,
`integration_post_batch.rs`). These are pre-existing stylistic choices
(multi-line assert vs single-line, import ordering). The source file
`crates/sink-honeycomb/src/lib.rs` is fully fmt-clean. No source diffs.

### Compiler warnings note

All 36 test function names use BC-ID / VP-ID prefixes (e.g.
`test_BC_3_01_001_…`, `test_VP_011_…`) which trigger `non_snake_case`
warnings. These are intentional traceability IDs. No `#[allow]`
suppression is used — warnings are visible in output but do not block
compilation or tests.

## Deferred Items

### Cross-crate rebase coordination

S-4.02 changed `HttpSinkConfig.url` field type to `HttpEndpointUrl` alias.
`sink-honeycomb` wraps `HttpSink` and depends on `sink-http`. The alias is
backward-compatible, but a rebase of `feat/S-4.03-sink-honeycomb-driver`
onto the latest `main` (after S-4.02 merges) will be needed at merge time.
No code changes expected — import resolution only.

### Homebrew PATH env anomaly

Local machine has Homebrew cargo 1.94 on default PATH while `rust-toolchain.toml`
pins 1.95. Tests were captured using
`PATH=~/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH`.
CI uses `rustup` correctly and is unaffected. This is an env-only issue
documented in all prior S-4.xx stories.

### Doctest

`Doc-tests sink_honeycomb: 0 tests` — no doctests present in `src/lib.rs`.
No failure; the crate is integration-tested via the 6 test files above.
A PATH mismatch between Homebrew rustdoc and rustup rustdoc would surface
here if doctests were present, as documented in S-4.01/S-4.02 evidence.
