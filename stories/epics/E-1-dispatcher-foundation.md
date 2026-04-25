---
document_type: epic
epic_id: "E-1"
version: "1.0"
prd_capabilities: []
status: closed
story_count: 9
---

# Epic E-1: Dispatcher Foundation

## Description

Nine stories (1 blocker + 8 parallel) that build the `factory-dispatcher` Rust binary,
the `vsdd-hook-sdk` crate, and the first two sink drivers. S-1.01 (Cargo workspace
scaffolding) is the single blocker; all others fan out behind it. Milestone:
`1.0.0-beta.1`. Subsystems: SS-01 (Hook Dispatcher Core), SS-02 (Hook SDK),
SS-03 (Observability Sinks).

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | Rust dispatcher binary parsing stdin JSON + routing plugins | P0 |
| (pre-CAP) | WASM plugin execution via wasmtime with epoch/fuel limits | P0 |
| (pre-CAP) | Hook SDK proc-macro for ergonomic plugin authoring | P0 |
| (pre-CAP) | Host function surface (10 functions) with capability policy | P0 |
| (pre-CAP) | Parallel-within-tier plugin execution via tokio | P0 |
| (pre-CAP) | Always-on dispatcher-internal.jsonl telemetry | P0 |
| (pre-CAP) | File sink (JSONL append, daily rotation) | P0 |
| (pre-CAP) | OTel gRPC sink (OTLP/gRPC batch export) | P0 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | Cargo workspace builds clean on darwin-arm64 + linux-x64 | CI cargo-check job | All crates compile; fmt + clippy green |
| EAC-002 | Dispatcher routes events to correct plugins by event + tool regex | Unit tests in routing.rs | 15+ routing permutations |
| EAC-003 | Plugin invocation with epoch/fuel limits; timeout produces PluginResult::Timeout | Integration tests | hanging plugin, fuel-exhausted, crashed, normal |
| EAC-004 | #[hook] macro compiles + runs on wasm32-wasip1 | Smoke test in CI | hello-hook example plugin |
| EAC-005 | All 10 host functions registered and enforcing capability policy | Unit + integration tests | Each denied path tested |
| EAC-006 | Plugins within same tier run concurrently; tiers run sequentially | Integration test | 3 tiers x 2 plugins; verify order + concurrency |
| EAC-007 | dispatcher-internal.jsonl written on every invocation | Integration test | All 17+ event types appear |
| EAC-008 | File sink appends JSONL; daily rotation works | Integration test (1000 events) | All events present; valid JSON per line |
| EAC-009 | OTel gRPC sink delivers batches to mock OTLP receiver | Integration test | Batch delivery verified |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-1.01 | Cargo workspace + CI scaffolding | 5 | — | merged |
| S-1.02 | factory-dispatcher core (stdin, TOML load, routing) | 8 | S-1.01 | merged |
| S-1.03 | hook-sdk crate (macro, types, bindings) | 8 | S-1.01 | merged |
| S-1.04 | Host function surface implementation | 8 | S-1.01, S-1.02, S-1.03 | merged |
| S-1.05 | wasmtime integration + epoch/fuel enforcement | 8 | S-1.01, S-1.02, S-1.04 | merged |
| S-1.06 | tokio + parallel-within-tier execution | 5 | S-1.01, S-1.02, S-1.04, S-1.05 | merged |
| S-1.07 | dispatcher-internal.jsonl writer | 3 | S-1.01, S-1.02 | merged |
| S-1.08 | sink-file driver | 5 | S-1.01, S-1.07 | merged |
| S-1.09 | sink-otel-grpc driver | 5 | S-1.01, S-1.08 | merged |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| wasmtime | WASM runtime with epoch + fuel | Available (workspace-pinned) |
| tonic/prost | gRPC client for OTLP | Available (workspace-pinned) |
| GitHub Actions | Rust CI matrix | Available |
