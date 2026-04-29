# AC3: Edge Cases

**AC:** AC3 — Zero capabilities; no subprocess; no file reads; edge-case field handling (EC-001–EC-004).
**BCs:** BC-4.08.001 Invariants 1–2, EC-001–EC-004; BC-4.08.003 Postconditions 4–5
**VP-068 tests:** tests 2, 3, 4, 5, 6

## EC-002: tool_name Absent → "unknown" Sentinel (Test 2)

When `tool_name` is missing from the `PostToolUseFailure` envelope, the plugin emits `tool_name = "unknown"` — not an empty string. "unknown" is the explicit sentinel for the absent-tool-name case.

**Plugin logic** (`src/lib.rs`, lines 56–65):
```rust
let tool_name_raw = ctx
    .tool_input
    .get("tool_name")
    .and_then(|v| v.as_str())
    .unwrap_or("");
let tool_name = if tool_name_raw.is_empty() {
    "unknown"
} else {
    tool_name_raw
};
```

**Test assertion** (`test_bc_4_08_001_missing_tool_name_emits_unknown_sentinel`):
```rust
// tool_name absent from envelope — EC-002 path
let payload = make_tool_failure_payload("sess-no-tool-name", "trace-no-tool-name",
    None,                    // tool_name absent
    Some("some error occurred"),
);
assert_eq!(tool_errors[0]["tool_name"].as_str(), Some("unknown"), "...");
// error_message still emitted normally
assert_eq!(tool_errors[0]["error_message"].as_str(), Some("some error occurred"), "...");
```

Result: PASS.

## EC-001: error_message Truncated at 1000 Chars (Test 3)

Input of 1500 chars is truncated to exactly 1000 chars before emit. Truncation is silent — no warning emitted. The truncation limit is 1000 chars (reverted from 2000 in ADV-S5.04-P01 CRIT-003).

**Plugin logic** (`src/lib.rs`, lines 67–77):
```rust
let error_message_raw = ctx
    .tool_input
    .get("error_message")
    .and_then(|v| v.as_str())
    .unwrap_or("");
let error_message = if error_message_raw.len() > 1000 {
    &error_message_raw[..1000]
} else {
    error_message_raw
};
```

**Test assertion** (`test_bc_4_08_001_error_message_truncated_at_1000_chars`):
```rust
let long_error = "E".repeat(1500);
// ...
assert_eq!(emitted_msg.len(), 1000, "... got {} chars", emitted_msg.len());
assert_eq!(emitted_msg, &long_error[..1000], "truncated must be first 1000 chars");
```

Result: PASS.

## EC-001 Boundary: error_message Exactly 1000 Chars, No Truncation (Test 4)

A 1000-char error message must be emitted verbatim — truncation triggers only when `len() > 1000`, not `>= 1000`.

**Test assertion** (`test_bc_4_08_001_error_message_exactly_1000_chars_no_truncation`):
```rust
let exact_error = "X".repeat(1000);
// ...
assert_eq!(emitted_msg.len(), 1000, "... must NOT be truncated; got {} chars", emitted_msg.len());
assert_eq!(emitted_msg, &exact_error, "1000-char input must be emitted verbatim");
```

Result: PASS.

## EC-003: error_message Absent → Empty String (Test 5)

When `error_message` is missing from the envelope, the plugin emits `error_message = ""` (empty string). This is distinct from EC-002: the absent-field defaults are asymmetric ("unknown" for `tool_name`, `""` for `error_message`).

**Test assertion** (`test_bc_4_08_001_missing_error_message_emits_empty_string`):
```rust
let payload = make_tool_failure_payload("sess-no-error-msg", "trace-no-error-msg",
    Some("Write"),
    None,   // error_message absent — EC-003 path
);
assert_eq!(tool_errors[0]["error_message"].as_str(), Some(""), "...");
assert_eq!(tool_errors[0]["tool_name"].as_str(), Some("Write"), "...");
```

Result: PASS.

## EC-004: Capability Scope — ZERO Declared (Test 6)

The plugin declares ZERO capabilities in `hooks-registry.toml`. The `CountingMock` confirms `exec_subprocess` and `read_file` are never invoked.

**Test assertion** (`test_bc_4_08_001_no_subprocess_no_read_file_invoked`):
```rust
let exec_mock = CountingMock::new();
let read_mock = CountingMock::new();
// ... dispatch ...
assert_eq!(exec_mock.invocation_count(), 0, "BC-4.08.001 Invariant 1: exec_subprocess count must be 0");
assert_eq!(read_mock.invocation_count(), 0, "BC-4.08.001 Invariant 2: read_file count must be 0");
```

`CountingMock` is a zero-incrementing stub — because `tool_failure_hook_logic` never calls `exec_subprocess` or `read_file`, the counts remain at 0. All data comes exclusively from the incoming envelope's `tool_input` fields.

Result: PASS.

The `hooks-registry.toml` entry has no capability tables (confirmed in AC5).
