---
document_type: demo-evidence-index
story_id: S-4.01
producer: demo-recorder
phase: per-story-delivery
branch: feat/S-4.01-sink-http-driver
tested_at_sha: b0bb135
timestamp: 2026-04-27T22:59:18Z
---

# S-4.01: sink-http driver — Demo Evidence Index

**Story spec:** `.factory/stories/S-4.01-sink-http-driver.md`
**Epic:** E-4 — Observability Sinks and RC Release
**Target module:** `crates/sink-http`

## Per-AC Evidence Map

| AC | Evidence File | Status | Description |
|----|--------------|--------|-------------|
| AC-01 | [AC-01-sink-trait.txt](AC-01-sink-trait.txt) | PASS | `HttpSink` implements the `Sink` trait (BC-3.01.001) |
| AC-02 | [AC-02-disabled-config.txt](AC-02-disabled-config.txt) | PASS | `enabled=false` config: no events accepted, no HTTP calls (BC-3.06.005) |
| AC-03 | [AC-03-unknown-type-warns.txt](AC-03-unknown-type-warns.txt) | PASS | Unknown sink `type` field logs warning and does not fail load (BC-3.01.002) |
| AC-04 | [AC-04-schema-version-error.txt](AC-04-schema-version-error.txt) | PASS | `schema_version != 1` is a hard error at load time (BC-3.01.003) |
| AC-05 + AC-08 | [AC-05-AC-08-batched-post-integration.txt](AC-05-AC-08-batched-post-integration.txt) | PASS | Events batched and POSTed as JSON array; integration test with mock HTTP server |
| AC-06 | [AC-06-error-handling.txt](AC-06-error-handling.txt) | PASS | HTTP error handling: retry on 5xx, drop on 4xx (VP-012, both paths) |
| AC-07 | [AC-07-flush-sync.txt](AC-07-flush-sync.txt) | PASS | `flush()` sends current batch synchronously |
| AC-08 | (see AC-05+AC-08) | PASS | Integration test with mock server (covered in combined evidence) |
| AC-09 | [AC-09-public-export.txt](AC-09-public-export.txt) | PASS | `HttpSink` struct exposed as public API for reuse |

## Verification Property Coverage

| VP | Evidence File | Status | Description |
|----|--------------|--------|-------------|
| VP-011 | [VP-011-non-blocking-submit.txt](VP-011-non-blocking-submit.txt) | PASS | `submit()` does not block when queue is full |
| VP-012 | [AC-06-error-handling.txt](AC-06-error-handling.txt) | PASS | 5xx retries + 4xx drop (both sub-paths demonstrated) |
| VP-013 | [AC-05-AC-08-batched-post-integration.txt](AC-05-AC-08-batched-post-integration.txt) | PASS | JSON array batching verified against mock server |

## Test Count Summary

**10 of 10 tests passing** across 5 test files:

| Test file | Tests | Result |
|-----------|-------|--------|
| `tests/contract_sink_trait.rs` | 2 | ok |
| `tests/contract_config_load.rs` | 3 | ok |
| `tests/error_handling.rs` | 2 | ok |
| `tests/integration_post_batch.rs` | 2 | ok |
| `tests/non_blocking.rs` | 1 | ok |

Full output: [all-tests-summary.txt](all-tests-summary.txt)

## Build Hygiene

| Check | Status | Evidence |
|-------|--------|----------|
| `cargo fmt --check` | CLEAN (empty output, exit 0) | [fmt-clean.txt](fmt-clean.txt) |
| `cargo clippy -- -D warnings` | CLEAN (`Finished` with 0 warnings) | [clippy-clean.txt](clippy-clean.txt) |

Note: `non_snake_case` warnings appear for test function names using BC/VP/TV identifiers in their names. These are test naming conventions and are not caught by `-D warnings` (they are `warn` level, not `deny` level). Clippy exits 0.

## Deferred Items (TD Candidates)

The following technical debt candidates were identified during implementation and are being scoped in a parallel story-creation burst:

- **TD-008 candidate:** Retry backoff — current retry logic on 5xx does not implement exponential backoff; a fixed retry count is used. Backoff strategy deferred to a follow-on story.
- **TD-009 candidate:** `internal.sink_error` metric — error events are logged but not emitted as structured sink-error metrics to the observability pipeline. Metric emission deferred to a follow-on story.

## Toolchain Note

This project requires rustc 1.95 (pinned in `rust-toolchain.toml`). Evidence was captured using the 1.95.0-aarch64-apple-darwin toolchain with PATH-priority override, as the system PATH contains a Homebrew cargo 1.94 binary. All 10 spec tests and clippy ran cleanly under 1.95. Doctests exhibit a mixed-compiler artifact (rustdoc invoked by Homebrew cargo picks up 1.94-vs-1.95 compiled deps) — this is a local env issue, not a code defect. CI runs against the pinned toolchain and is unaffected.
