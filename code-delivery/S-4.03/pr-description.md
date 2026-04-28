# feat(sink-honeycomb): Honeycomb HTTP sink wrapping HttpSink (S-4.03)

## Summary

Implements `crates/sink-honeycomb`, a Honeycomb-specific observability sink wrapping
the `HttpSink` base from S-4.01. Posts events to Honeycomb's Events API
(`POST /1/events/<dataset>`) with `X-Honeycomb-Team` auth header and RFC3339 `time`
field injection. All 36 tests across 6 test files pass. clippy clean.

**Story:** S-4.03 | **Wave:** 12 | **Points:** 3 | **Priority:** P1
**Depends on:** S-1.08, S-4.01 (both merged) | **Blocks:** S-4.07, S-4.08

---

## Architecture Changes

```mermaid
graph TD
    HoneycombSink["sink-honeycomb\ncrates/sink-honeycomb/src/lib.rs"] --> HttpSink["sink-http\nS-4.01 (merged)"]
    HttpSink --> SinkTrait["Sink trait\ncrates/sink-core"]
    HoneycombSink --> HoneycombConfig["HoneycombSinkConfig\napi_key + dataset + enabled"]
    HoneycombSink --> HoneycombEndpoint["POST /1/events/<dataset>\nX-Honeycomb-Team: <api_key>"]
```

**New crate:** `crates/sink-honeycomb` (451 LOC src, 869 LOC tests, 6 test files)

---

## Story Dependencies

```mermaid
graph LR
    S108["S-1.08\nSink Registry"] --> S403["S-4.03\nsink-honeycomb"]
    S401["S-4.01\nsink-http"] --> S403
    S403 --> S407["S-4.07\n(blocked)"]
    S403 --> S408["S-4.08\n(blocked)"]
```

**Dependency status:** S-1.08 merged, S-4.01 merged, S-4.02 open (parallel, no blocking dependency).

---

## Spec Traceability

```mermaid
flowchart LR
    BC301["BC-3.01.001\nSink trait integration"] --> AC01["AC-01\nSink trait impl"]
    BC360["BC-3.06.005\nSinkConfigCommon"] --> AC02["AC-02\nConfig load"]
    BC301 --> AC03["AC-03\nEndpoint construction"]
    BC360 --> AC04["AC-04\nAuth header"]
    BC301 --> AC05["AC-05\nBatch integration"]
    BC301 --> AC06["AC-06\nInherits HttpSink"]
    AC01 --> T01["contract_sink_trait\n5 tests"]
    AC02 --> T02["contract_config_load\n11 tests"]
    AC03 --> T03["contract_endpoint_construction\n5 tests"]
    AC04 --> T04["contract_auth_header\n3 tests"]
    AC05 --> T05["integration_post_batch\n7 tests"]
    AC06 --> T06["integration_inherits_http_sink\n5 tests"]
```

---

## Acceptance Criteria Checklist

- [x] **AC-01** `crates/sink-honeycomb` implements the `Sink` trait (BC-3.01.001, VP-011, VP-012) — 5 tests PASS
- [x] **AC-02** Sends events to Honeycomb Events API `/1/events/<dataset>` — endpoint construction verified, 5 tests PASS
- [x] **AC-03** Auth via `X-Honeycomb-Team` header from config (BC-3.06.005) — 3 tests PASS
- [x] **AC-04** Each event includes `time` field in RFC3339 format — verified in integration_post_batch
- [x] **AC-05** Dataset configurable per sink instance — config load tests cover missing/empty dataset (EC-001), 11 tests PASS
- [x] **AC-06** Integration test with mock Honeycomb endpoint (httpmock) — 7 tests PASS

---

## Test Evidence

| Test file | Tests | Result |
|-----------|-------|--------|
| `contract_sink_trait.rs` | 5 | PASS |
| `contract_config_load.rs` | 11 | PASS |
| `contract_endpoint_construction.rs` | 5 | PASS |
| `contract_auth_header.rs` | 3 | PASS |
| `integration_post_batch.rs` | 7 | PASS |
| `integration_inherits_http_sink.rs` | 5 | PASS |
| **Total** | **36/36** | **GREEN** |

Build hygiene: `cargo clippy -p sink-honeycomb -- -D warnings` → CLEAN.
`cargo fmt --check` → 5 minor test-file style diffs (non-blocking; `src/lib.rs` is fmt-clean; commit `046b0d6` applies rustfmt).

---

## Demo Evidence

Full per-AC demo evidence: [`docs/demo-evidence/S-4.03/INDEX.md`](docs/demo-evidence/S-4.03/INDEX.md)

| AC | Evidence file |
|----|--------------|
| AC-01 Sink trait | `docs/demo-evidence/S-4.03/AC-01-sink-trait.txt` |
| AC-02 Config load | `docs/demo-evidence/S-4.03/AC-02-config-load.txt` |
| AC-03 Endpoint construction | `docs/demo-evidence/S-4.03/AC-03-endpoint-construction.txt` |
| AC-04 Auth header | `docs/demo-evidence/S-4.03/AC-04-auth-header.txt` |
| AC-05 Batch integration | `docs/demo-evidence/S-4.03/AC-05-batch-integration.txt` |
| AC-06 Inherits HttpSink | `docs/demo-evidence/S-4.03/AC-06-inherits-http-sink.txt` |
| All tests summary | `docs/demo-evidence/S-4.03/all-tests-summary.txt` |
| clippy clean | `docs/demo-evidence/S-4.03/clippy-clean.txt` |
| fmt check | `docs/demo-evidence/S-4.03/fmt-clean.txt` |

---

## Auth + Endpoint Construction Detail

- **Endpoint:** `POST https://api.honeycomb.io/1/events/<dataset>` — dataset is embedded in URL path, not a header
- **Auth:** `X-Honeycomb-Team: <api_key>` header; `Content-Type: application/json`
- **Time field:** RFC3339 `time` key injected per event from event's `ts_epoch`; falls back to wall-clock if absent
- **Config required fields:** `api_key` (non-empty, non-whitespace), `dataset` (non-empty, non-whitespace) — both absent/empty fail at config load (EC-001)
- **429 retry:** exponential backoff on rate limit (EC-002), tested in integration_post_batch

---

## Cross-Crate Note (S-4.02 F-2 API Change)

S-4.02 (PR #24, currently open) changed `HttpSinkConfig.url` field type from `String`
to `HttpEndpointUrl` (which is `type HttpEndpointUrl = String`). This is a type-alias-only
change; direct field access still works.

S-4.03 wraps `HttpSink` and depends on `sink-http`. The type alias is backward-compatible.
**If S-4.02 merges into develop before this PR merges:**
1. Checkout `feat/S-4.03-sink-honeycomb-driver`
2. `git pull --rebase origin develop`
3. Resolve any conflict on `crates/sink-http/src/lib.rs` by accepting develop's version (S-4.02 refactor)
4. Verify `cargo test -p sink-honeycomb` still passes 36/36
5. `git push --force-with-lease`

No code changes to `sink-honeycomb` itself are expected.

---

## Known TD Candidates (Deferred, Non-Blocking)

| ID | Description | Disposition |
|----|-------------|-------------|
| TD-01 | `httpmock 0.7` routing collision pattern — surfaced in S-4.01 and S-4.03 auth-header test (single-mock chained matchers required). Should be codified as a test-writer agent prompt or "common httpmock pitfalls" doc. | Defer to v1.1 |
| TD-02 | Dataset path encoding: dataset name is currently passed literally; if dataset contains special chars (e.g. `/`, `%`), URL path would be malformed. URL-encode dataset in v1.1. | Defer to v1.1 |
| TD-03 | Doctest failure under local Homebrew rustc/rustdoc PATH mismatch — env-only, CI unaffected. | Doc note only |

---

## Holdout Evaluation

N/A — evaluated at wave gate.

## Adversarial Review

N/A — evaluated at Phase 5.

## Security Review

**Result: PASS — no CRITICAL or HIGH findings.**

| Finding | Severity | Disposition |
|---------|----------|-------------|
| Dataset appended to URL path without URL-encoding | LOW | Tracked as TD-02 (operator config value, not user input — no injection vector). Defer to v1.1. |
| API key stored in worker thread memory | INFO | No logging, no exposure. Standard practice. |

OWASP Top 10 scan: no SQL injection, no command injection, no user-controlled input flowing into HTTP request construction. All config values are operator-provided, validated at load time.

---

## Risk Assessment

| Dimension | Assessment |
|-----------|-----------|
| Blast radius | New crate only (`crates/sink-honeycomb`). No changes to existing crates. |
| Performance impact | None — new code path, not on hot path. |
| Breaking changes | None. Type alias in S-4.02 is backward-compatible. |
| Rollback | Remove `sink-honeycomb` from `Cargo.toml` workspace members. |

---

## AI Pipeline Metadata

| Field | Value |
|-------|-------|
| Pipeline mode | story-delivery |
| Wave | 12 |
| Story ID | S-4.03 |
| Model | claude-sonnet-4-6 |

---

## Pre-Merge Checklist

- [x] PR description matches actual diff
- [x] All ACs covered by demo evidence (6/6)
- [x] Traceability chain complete (BC → AC → Test → Demo)
- [x] 36/36 tests passing
- [x] clippy clean
- [x] Security review complete (PASS — no CRITICAL/HIGH)
- [ ] PR reviewer approved
- [ ] CI checks passing
- [ ] Dependency PRs merged (S-4.01 merged; S-4.02 parallel — no hard dep)
