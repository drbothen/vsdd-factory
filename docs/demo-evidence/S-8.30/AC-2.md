# AC-2: Backward-compat — PreToolUse envelope deserializes with all 4 new fields = None

**Story:** S-8.30  
**AC:** AC-2 — Backward-compat deserialization  
**BC clause:** BC-2.02.012 Postcondition 7, Invariant 2  
**Test:** `test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none`

## Cargo test output

```
test payload::tests::test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none ... ok
```

## Test source

```rust
/// BC-2.02.012 Postcondition 7 / Invariant 2: PreToolUse envelope
/// (no SubagentStop fields) deserializes with all four new fields = None.
/// Verifies #[serde(default)] backward-compat (AC-2).
#[test]
fn test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none() {
    let p = fixture(
        r#"{
            "event_name": "PreToolUse",
            "tool_name": "Bash",
            "session_id": "s",
            "dispatcher_trace_id": "t",
            "tool_input": {}
        }"#,
    );
    assert_eq!(p.agent_type, None, "agent_type must default to None for PreToolUse");
    assert_eq!(p.subagent_name, None, "subagent_name must default to None for PreToolUse");
    assert_eq!(
        p.last_assistant_message, None,
        "last_assistant_message must default to None for PreToolUse"
    );
    assert_eq!(p.result, None, "result must default to None for PreToolUse");
}
```

**PASS**
