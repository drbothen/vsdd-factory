# AC-3: SubagentStop envelope with all 4 fields present deserializes correctly

**Story:** S-8.30  
**AC:** AC-3 — SubagentStop happy-path deserialization  
**BC clause:** BC-2.02.012 Postconditions 1-4  
**Test:** `test_BC_2_02_012_subagentstop_all_four_fields_populated`

## Cargo test output

```
test payload::tests::test_BC_2_02_012_subagentstop_all_four_fields_populated ... ok
test payload::tests::test_BC_2_02_012_subagentstop_fallback_fields_populated ... ok
```

## Test source

```rust
#[test]
fn test_BC_2_02_012_subagentstop_all_four_fields_populated() {
    let p = fixture(
        r#"{
            "event_name": "SubagentStop",
            "session_id": "s",
            "dispatcher_trace_id": "t",
            "agent_type": "pr-reviewer",
            "subagent_name": "pr-reviewer-fallback",
            "last_assistant_message": "wrote pr-review.md and posted gh pr review --approve",
            "result": "fallback-result"
        }"#,
    );
    // Postcondition 1
    assert_eq!(p.agent_type, Some("pr-reviewer".to_string()));
    // Postcondition 2
    assert_eq!(p.subagent_name, Some("pr-reviewer-fallback".to_string()));
    // Postcondition 3
    assert_eq!(
        p.last_assistant_message,
        Some("wrote pr-review.md and posted gh pr review --approve".to_string())
    );
    // Postcondition 4
    assert_eq!(p.result, Some("fallback-result".to_string()));
}
```

**PASS**
