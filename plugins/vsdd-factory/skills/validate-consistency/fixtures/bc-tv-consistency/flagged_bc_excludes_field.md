---
document_type: behavioral-contract
level: ops
bc_id: "BC-FIXTURE-FLAGGED.001"
version: "1.0"
status: fixture
producer: validate-consistency
target_module: crates/sink-core/src/entry.rs
phase: skill-fixtures
---

# BC-FIXTURE-FLAGGED.001 — LogEntry omits trace_id on retry exhaustion

> **Fixture only.** Not registered in BC-INDEX. Pairs with
> `flagged_emitter_serializes_field.rs` to exercise Check 9's positive path.

## Description

When a `LogEntry` is emitted as part of a retry-exhausted DLQ payload, the
`trace_id` field is intentionally omitted because the upstream tracer is
no longer available and the field would carry stale data.

## Postconditions

1. The serialized `LogEntry` JSON for retry-exhaustion entries does not contain a `trace_id` key.
2. For non-DLQ entries, `trace_id` may be present.

## Canonical Test Vectors

| Struct | Scenario | trace_id in Entry? | level in Entry? | message in Entry? |
|--------|----------|-------------------:|----------------:|------------------:|
| LogEntry | normal info log | Yes | Yes | Yes |
| LogEntry | retry-exhaustion DLQ entry | **No** | Yes | Yes |
| LogEntry | queue-overflow DLQ entry | **No** | Yes | Yes |

The `No` answers in the `trace_id in Entry?` column are what Check 9
cross-references against `crates/sink-core/src/entry.rs::LogEntry`.

## Architecture Module

`crates/sink-core/src/entry.rs`
