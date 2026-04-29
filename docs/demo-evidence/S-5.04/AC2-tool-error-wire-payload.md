# AC2: tool.error Wire Payload

**AC:** AC2 — `tool.error` emitted with exactly 10 fields; 2 plugin-set; 4 host-enriched; 4 construction-time; RESERVED_FIELDS not set by plugin.
**BC:** BC-4.08.001 Postconditions 1–2
**VP-068 test:** `test_bc_4_08_001_tool_error_emitted_with_required_fields` (test 1)

## 10-Field Wire Layout

```
tool.error wire payload (10 fields total)
├── Plugin-set (2) — set by tool-failure-hooks.wasm via emit_event call
│   ├── tool_name       "Bash"                    (from envelope tool_input.tool_name)
│   └── error_message   "command exited with..."  (from envelope tool_input.error_message)
│
├── Host-enriched (4) — injected by emit_event host fn from HostContext (BC-1.05.012)
│   ├── dispatcher_trace_id   "<uuid>"           (from envelope; MUST NOT be set by plugin)
│   ├── session_id            "<uuid>"           (from envelope; MUST NOT be set by plugin — BC-1.05.012)
│   ├── plugin_name           "tool-failure-hooks"
│   └── plugin_version        "1.0.0-rc.1"
│
└── Construction-time (4) — set by dispatcher InternalEvent::now() (opaque from spec layer)
    ├── type            "tool.error"
    ├── ts              "2026-04-28T..."   (ISO 8601 timestamp)
    ├── ts_epoch        1745808000         (Unix epoch seconds)
    └── schema_version  1
```

## Plugin Source

File: `crates/hook-plugins/tool-failure-hooks/src/lib.rs`, lines 79–82:

```rust
// Emit exactly once with the 2 plugin-set fields; RESERVED_FIELDS are NOT set here.
emit("tool.error", &[("tool_name", tool_name), ("error_message", error_message)]);

HookResult::Continue
```

The plugin calls `emit` with exactly 2 fields. The remaining 8 fields (4 host-enriched + 4 construction-time) are injected by the dispatcher, not the plugin.

## RESERVED_FIELDS Discipline

The 8 RESERVED_FIELDS the plugin MUST NOT set (split into 2 opaque groups per BC-4.08.001):

| Group | Fields |
|-------|--------|
| Host-enriched (4) | `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version` |
| Construction-time (4) | `ts`, `ts_epoch`, `schema_version`, `type` |

`session_id` is specifically host-enriched per BC-1.05.012. If a plugin attempted to set it, the `emit_event` host fn would silently drop the plugin-supplied value and use the envelope value instead.

## Test Assertion (VP-068 Test 1)

The integration test builds the full 10-field payload via `dispatch_and_capture` (which simulates host enrichment) and asserts:

```rust
// Exactly one tool.error event emitted
assert_eq!(tool_errors.len(), 1, "BC-4.08.001 PC-1: ...");

// Plugin-set field values
assert_eq!(event["tool_name"].as_str(), Some("Bash"), "...");
assert_eq!(event["error_message"].as_str(), Some("command exited with status 1"), "...");

// Host-enriched fields present and non-empty
assert!(event["dispatcher_trace_id"].is_string() && !event["dispatcher_trace_id"].as_str().unwrap().is_empty(), "...");
assert!(event["session_id"].is_string() && !event["session_id"].as_str().unwrap().is_empty(), "...");
assert!(event["plugin_name"].is_string() && ..., "...");
assert!(event["plugin_version"].is_string() && ..., "...");

// Construction-time fields present
assert!(event.get("ts").is_some(), "...");
assert!(event.get("ts_epoch").is_some(), "...");
assert!(event.get("schema_version").is_some(), "...");
assert_eq!(event.get("type").and_then(|v| v.as_str()), Some("tool.error"), "...");

// Exactly 10 total fields
let field_count = event.as_object().map(|m| m.len()).unwrap_or(0);
assert_eq!(field_count, 10, "...");

// Exactly 2 plugin-set fields
assert_eq!(plugin_set_fields.len(), 2, "...");
```

All assertions pass in commit `81e9fc4`.
