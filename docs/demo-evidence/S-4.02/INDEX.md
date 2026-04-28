---
document_type: demo-evidence-index
story_id: S-4.02
producer: demo-recorder
phase: per-story-delivery
branch: feat/S-4.02-sink-datadog-driver
tested_at_sha: 95b9ad0
timestamp: 2026-04-27T00:00:00
---

# S-4.02 Demo Evidence Index — sink-datadog driver

**Branch:** `feat/S-4.02-sink-datadog-driver`
**SHA:** `95b9ad0`
**Date:** 2026-04-27
**Result:** 24/24 sink-datadog GREEN + 10/10 sink-http upstream UNCHANGED

## Per-AC Coverage

| File | AC / Finding | Tests | Result |
|------|-------------|-------|--------|
| AC-01-sink-trait.txt | Sink trait implementation (BC-3.01.001) | 2 | PASS |
| AC-02-config-load.txt | Config load — API key required, schema_version gate, unknown type (BC-3.06.005) | 6 | PASS |
| AC-03-endpoint-default.txt | Regional endpoints — default us1, explicit override, constant accessible (BC-3.06.005) | 3 | PASS |
| AC-04-headers.txt | DD-API-KEY header on POST; absent key does not match auth mock (BC-3.06.005) | 2 | PASS |
| AC-05-batch-integration.txt | POST JSON array to /api/v2/logs; Datadog schema fields; 5MB constant; VP-011 non-block; VP-012 isolation | 5 | PASS |
| AC-06-inherits-http-sink.txt | Mock endpoint integration; VP-011 non-block; VP-012 retry-then-success; VP-012 exhausted retries | 3 | PASS |
| F-1-blocking-feature-removed.txt | PR #18 F-1: reqwest "blocking" feature removed from workspace Cargo.toml | 1 | PASS |
| F-2-config-api-stability.txt | PR #18 F-2: HttpSinkConfig builder/type-alias stable API; wrapper does not pin to field layout | 2 | PASS |

**Total sink-datadog: 24/24**

## Cumulative Evidence

| File | Description | Result |
|------|-------------|--------|
| all-tests-summary.txt | `cargo test -p sink-datadog` — all 8 test suites | 24/24 PASS |
| sink-http-still-passing.txt | `cargo test -p sink-http` — upstream regression check | 10/10 PASS |
| clippy-clean.txt | `cargo clippy -p sink-datadog -p sink-http -- -D warnings` | CLEAN |
| fmt-clean.txt | `cargo fmt -p sink-datadog -p sink-http -- --check` | CLEAN |

## F-1 Closure Detail

PR #18 deferred LOW finding F-1 is closed. The workspace `Cargo.toml` reqwest entry
changed from `features = ["json", "blocking"]` to `features = ["json"]`. The
`blocking` feature was unused — no test or source file calls reqwest's blocking API.
Both `cargo build -p sink-http` and `cargo build -p sink-datadog` succeed after removal.
Contract test `contract_no_blocking_feature` verifies the feature is absent at compile time.

## F-2 Closure Detail

PR #18 deferred LOW finding F-2 is closed via a type-alias approach. `HttpSinkConfig`
retains `pub url: HttpEndpointUrl` (where `HttpEndpointUrl = String`) for backward
compatibility with existing direct field reads, while adding `HttpSinkConfig::builder()`
and accessor methods (`url()`, `queue_depth()`, `extra_headers()`). Wrapper sinks
(sink-datadog, sink-honeycomb) use `builder()` exclusively — they never pin to the raw
`String` type. Contract test `contract_config_api_stability` verifies the builder
pattern compiles and functions correctly.

## Cross-Crate Impact Note

S-4.03 (sink-honeycomb, parallel worktree `/private/tmp/vsdd-S-4.03`) will encounter
the F-2 API change on merge since it also wraps `HttpSinkConfig`. The pr-manager
handles rebase coordination. No action required from this story.

## Build Hygiene

- `cargo clippy -p sink-datadog -p sink-http -- -D warnings`: clean (exit 0)
- `cargo fmt -p sink-datadog -p sink-http -- --check`: clean (exit 0)
- Non-snake-case lint warnings on BC/VP/F-prefixed test function names are expected
  (factory convention for BC/VP traceability). These are warnings only, not errors,
  and do not appear under `-D warnings` for clippy (only cargo test compilation).

## Deferred Items

None. All 9 ACs (including F-1 and F-2 deferred findings) are closed in this story.
The v1.1 BC candidates listed in the story spec (BC-3.NN.NNN-*) remain as future work.
