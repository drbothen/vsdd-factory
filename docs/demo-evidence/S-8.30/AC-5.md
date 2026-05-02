# AC-5: Round-trip preservation

**Story:** S-8.30  
**AC:** AC-5 — Serde round-trip preservation  
**BC clause:** BC-2.02.012 Invariant 4  
**Test:** `test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields`

## Cargo test output

```
test payload::tests::test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields ... ok
```

## Test source

```rust
#[test]
fn test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields() {
    let original = fixture(
        r#"{
            "event_name": "SubagentStop",
            "session_id": "s",
            "dispatcher_trace_id": "t",
            "agent_type": "product-owner",
            "subagent_name": "po-fallback",
            "last_assistant_message": "Done.",
            "result": "Complete."
        }"#,
    );
    let json = serde_json::to_string(&original).expect("serialize");
    let round: HookPayload = serde_json::from_str(&json).expect("round-trip");

    assert_eq!(round.agent_type, original.agent_type, "agent_type must survive round-trip");
    assert_eq!(round.subagent_name, original.subagent_name, "subagent_name must survive round-trip");
    assert_eq!(
        round.last_assistant_message, original.last_assistant_message,
        "last_assistant_message must survive round-trip"
    );
    assert_eq!(round.result, original.result, "result must survive round-trip");
}
```

**PASS**
