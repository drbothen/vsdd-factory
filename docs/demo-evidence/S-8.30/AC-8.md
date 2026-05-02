# AC-8: All existing event types deserialize without error; SubagentStop fields default to None

**Story:** S-8.30  
**AC:** AC-8 — All 5 event types deserialize cleanly  
**BC clause:** BC-2.02.012 Postcondition 7, Invariant 2  
**Test:** `test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop`

## Cargo test output

```
test payload::tests::test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop ... ok
```

## Event types covered

| event_name | Deserializes OK | agent_type | subagent_name | last_assistant_message | result |
|---|---|---|---|---|---|
| PreToolUse | ok | None | None | None | None |
| PostToolUse | ok | None | None | None | None |
| SessionStart | ok | None | None | None | None |
| SessionEnd | ok | None | None | None | None |
| SubagentStop | ok | Some(...) | None | Some(...) | None |

Note: Stop event type omitted per AC-8 note — unverified in bash hooks and BC-2.02.012 evidence.

## Test source (abridged)

```rust
let non_subagentstop_fixtures = [
    ("PreToolUse",   r#"{"event_name":"PreToolUse","tool_name":"Bash",...}"#),
    ("PostToolUse",  r#"{"event_name":"PostToolUse","tool_name":"Bash",...}"#),
    ("SessionStart", r#"{"event_name":"SessionStart",...}"#),
    ("SessionEnd",   r#"{"event_name":"SessionEnd",...}"#),
];
for (event, json) in &non_subagentstop_fixtures {
    let p: HookPayload = serde_json::from_str(json).unwrap_or_else(|e| panic!(...));
    assert_eq!(p.agent_type, None, "{event}: agent_type must be None");
    assert_eq!(p.subagent_name, None, ...);
    assert_eq!(p.last_assistant_message, None, ...);
    assert_eq!(p.result, None, ...);
}
// SubagentStop with fields: must deserialize without error
let p = fixture(r#"{"event_name":"SubagentStop",...,"agent_type":"product-owner",...}"#);
assert_eq!(p.event_name, "SubagentStop");
assert_eq!(p.agent_type, Some("product-owner".to_string()));
```

**PASS**
