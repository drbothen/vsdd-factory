---
document_type: behavioral-contract
level: ops
bc_id: "BC-FIXTURE-CLEAN.001"
version: "1.0"
status: fixture
producer: validate-consistency
target_module: crates/sink-core/src/entry.rs
phase: skill-fixtures
---

# BC-FIXTURE-CLEAN.001 — LogEntry omits trace_id on retry exhaustion (clean)

> **Fixture only.** Not registered in BC-INDEX. Pairs with
> `clean_emitter_uses_option_skip.rs` — same BC shape as the flagged
> fixture, but the paired emitter honors the exclusion.

## Description

When a `LogEntry` is emitted as part of a retry-exhausted DLQ payload, the
`trace_id` field is intentionally omitted.

## Postconditions

1. The serialized `LogEntry` JSON for retry-exhaustion entries does not contain a `trace_id` key.
2. For non-DLQ entries, `trace_id` may be present.

## Canonical Test Vectors

| Struct | Scenario | trace_id in Entry? | level in Entry? | message in Entry? |
|--------|----------|-------------------:|----------------:|------------------:|
| LogEntry | normal info log | Yes | Yes | Yes |
| LogEntry | retry-exhaustion DLQ entry | **No** | Yes | Yes |
| LogEntry | queue-overflow DLQ entry | **No** | Yes | Yes |

## Architecture Module

`crates/sink-core/src/entry.rs`
