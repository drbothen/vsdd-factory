# AC2 — 11-field wire payload

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC2 — `session.ended` emitted with correct 11-field wire payload  
**BC:** BC-4.05.001  
**GREEN commit:** `3783847`

---

## Field inventory (3 + 4 + 4 = 11)

| Group | Field | Source | Wire type |
|-------|-------|--------|-----------|
| Plugin-set | `duration_ms` | Computed from envelope `session_start_ts` | string |
| Plugin-set | `tool_call_count` | Envelope `tool_call_count` | string |
| Plugin-set | `timestamp` | Plugin emission time (ISO-8601 UTC ms Z) | string |
| Host-enriched | `dispatcher_trace_id` | `HostContext` via `emit_event` | string |
| Host-enriched | `session_id` | `HostContext` via `emit_event` | string |
| Host-enriched | `plugin_name` | `HostContext` via `emit_event` | string |
| Host-enriched | `plugin_version` | `HostContext` via `emit_event` | string |
| Construction-time | `type` | `InternalEvent::now()` | string (`"session.ended"`) |
| Construction-time | `ts` | `InternalEvent::now()` | string |
| Construction-time | `ts_epoch` | `InternalEvent::now()` | integer |
| Construction-time | `schema_version` | `InternalEvent::now()` | integer |

Plugin must NOT set RESERVED_FIELDS (8 total: 4 host-enriched + 4 construction-time).  
Source: `crates/hook-plugins/session-end-telemetry/src/lib.rs:99–101`

---

## Happy-path test output (EC-000)

```text
test session_end_integration::test_bc_4_05_001_session_ended_emitted_with_required_fields ... ok
```

Test confirms: exactly one `session.ended` emitted; `duration_ms` is a positive integer string
(fixture: `session_start_ts` = 1 minute ago); `tool_call_count = "42"` (fixture: 42);
`timestamp` matches `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`.  
Source: `tests/integration_test.rs:269`

---

## Edge-case test results

### EC-001a — `session_start_ts` absent → `duration_ms = "0"`

```text
test session_end_integration::test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration ... ok
```

`tool_call_count` present (7) → must NOT default to "0"; confirmed by test at line 397.

### EC-001b — `session_start_ts` in the future → `duration_ms = "0"` (clock-skew clamp)

```text
test session_end_integration::test_bc_4_05_001_future_session_start_ts_emits_zero_duration ... ok
```

Negative elapsed duration clamped to "0". Source: `src/lib.rs:70–73`.

### EC-001c — `session_start_ts` present but unparseable string → `duration_ms = "0"`

```text
test session_end_integration::test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration ... ok
```

Fixture: `"garbage-not-iso8601"`. RFC-3339 parse failure returns "0". Source: `src/lib.rs:64–67`.

### EC-002 — `tool_call_count` absent → `tool_call_count = "0"`

```text
test session_end_integration::test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count ... ok
```

`session_start_ts` present → `duration_ms` is still positive.

### EC-003 — both absent → both `"0"`

```text
test session_end_integration::test_bc_4_05_001_both_missing_emit_zero_defaults ... ok
```

`timestamp` remains well-formed (plugin emission time, independent of envelope).

### EC-004 — empty `session_id` → `"unknown"` sentinel

```text
test session_end_integration::test_bc_4_05_001_missing_session_id_emits_unknown ... ok
```

BC-1.02.005 lifecycle-tolerance: `session_id = ""` → host sets `"unknown"`.

---

## Plugin emit call site

```rust
// src/lib.rs:142–146
emit_fn(&[
    ("duration_ms", duration_ms.as_str()),
    ("tool_call_count", tool_call_count.as_str()),
    ("timestamp", timestamp.as_str()),
]);
```

Exactly 3 plugin-set fields passed. Host fn injects the remaining 8.
