# AC-4: jq-`//` parity — JSON null deserializes to None; fallback chains match bash

**Story:** S-8.30  
**AC:** AC-4 — JSON null semantics and canonical fallback chains  
**BC clause:** BC-2.02.012 Postconditions 3, 5, 6; Invariant 3; EC-001, EC-003  
**Tests:** `test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains`,
`test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults`

## Cargo test output

```
test payload::tests::test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains ... ok
test payload::tests::test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults ... ok
```

## Test source (JSON null + fallback chains)

```rust
#[test]
fn test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains() {
    let p = fixture(
        r#"{
            "event_name": "SubagentStop",
            "session_id": "s",
            "dispatcher_trace_id": "t",
            "agent_type": null,
            "subagent_name": "pr-reviewer",
            "last_assistant_message": null,
            "result": "actual-result"
        }"#,
    );
    // EC-001 / Invariant 3: JSON null → None
    assert_eq!(p.agent_type, None, "JSON null agent_type must deserialize to None");
    assert_eq!(p.subagent_name, Some("pr-reviewer".to_string()));
    assert_eq!(p.last_assistant_message, None, "JSON null last_assistant_message must be None");
    assert_eq!(p.result, Some("actual-result".to_string()));

    // Postcondition 5: canonical agent identity fallback chain
    let identity = p.agent_type.as_deref().or(p.subagent_name.as_deref()).unwrap_or("unknown");
    assert_eq!(identity, "pr-reviewer");

    // Postcondition 6: canonical assistant-message fallback chain
    let msg = p.last_assistant_message.as_deref().or(p.result.as_deref()).unwrap_or("");
    assert_eq!(msg, "actual-result");
}
```

## Canonical fallback chains (normative)

Agent identity (mirrors `jq -r '.agent_type // .subagent_name // "unknown"'`):
```rust
payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown")
```

Assistant message (mirrors `jq -r '.last_assistant_message // .result // empty'`):
```rust
payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("")
```

**PASS**
