---
document_type: epic
epic_id: "E-4"
version: "1.0"
prd_capabilities: []
status: draft
story_count: 8
---

# Epic E-4: Observability Sinks and RC Release

## Description

Eight stories (Tier E parallel + Tier F gate) that implement the full observability
sink suite: HTTP, Datadog, Honeycomb drivers; retry/circuit-breaker resilience;
dead-letter queue; routing filters. S-4.07 (integration tests) gates S-4.08 (rc.1
release gate). Milestone: `1.0.0-rc.1`. Subsystems: SS-01, SS-03, SS-10.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | Generic HTTP sink for custom backends | P1 |
| (pre-CAP) | Datadog Logs Intake sink | P1 |
| (pre-CAP) | Honeycomb Events API sink | P1 |
| (pre-CAP) | Exponential backoff + circuit breaker for all HTTP sinks | P1 |
| (pre-CAP) | Dead-letter queue for dropped events | P1 |
| (pre-CAP) | Per-sink routing filters + static tag enrichment | P1 |
| (pre-CAP) | End-to-end observability integration tests | P0 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | sink-http POSTs event batches to configured endpoint | Integration test (mock server) | 200, 429, 5xx responses |
| EAC-002 | sink-datadog sends to Datadog Logs API with DD-API-KEY auth | Integration test (mock) | Auth header, 5MB split, regional endpoint |
| EAC-003 | sink-honeycomb sends to Honeycomb Events API with time field | Integration test (mock) | Required fields, dataset routing |
| EAC-004 | Retry with exponential backoff; circuit opens after N failures | Integration test | 5xx server → retries → circuit open → internal event |
| EAC-005 | Events exhausting retries written to DLQ file | Integration test | 5xx always → DLQ populated |
| EAC-006 | Routing filters apply; events reach correct sinks | Integration test | 2 sinks with different event-type filters |
| EAC-007 | All 5 E2E observability scenarios green in CI | CI run | zero-disk, hybrid, multi-sink, DLQ, circuit-breaker |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-4.01 | sink-http driver | 5 | S-1.08 | draft |
| S-4.02 | sink-datadog driver | 5 | S-1.08, S-4.01 | draft |
| S-4.03 | sink-honeycomb driver | 3 | S-1.08, S-4.01 | draft |
| S-4.04 | Per-sink retry + circuit breaker | 8 | S-1.08, S-4.01 | draft |
| S-4.05 | Dead letter queue implementation | 3 | S-4.04 | draft |
| S-4.06 | Per-sink routing filters + tag enrichment | 3 | S-1.08 | partial |
| S-4.07 | End-to-end observability integration tests | 8 | S-3.01..S-3.04, S-4.01..S-4.06 | draft |
| S-4.08 | 1.0.0-rc.1 release gate | 3 | S-4.07 + 2-week shakedown | draft |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| Datadog Logs Intake API | HTTP endpoint for integration test mock | Available |
| Honeycomb Events API | HTTP endpoint for integration test mock | Available |
| beta shakedown | 2+ weeks of beta.1 stability | In progress |
