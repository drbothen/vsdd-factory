# AC-6: HOST_ABI.md updated with SubagentStop envelope schema

**Story:** S-8.30  
**AC:** AC-6 — HOST_ABI.md SubagentStop documentation  
**BC clause:** BC-2.02.012 Postconditions 5-6  
**Source:** `crates/hook-sdk/HOST_ABI.md` lines 64-123  
**Tests:** 5 file-reading tests in `payload.rs` test module (tests 16-20)

## HOST_ABI.md SubagentStop section excerpt

```markdown
### SubagentStop stdin envelope

When Claude Code fires a `SubagentStop` event the dispatcher writes the
following shape to plugin stdin.  The four SubagentStop-specific fields
(`agent_type`, `subagent_name`, `last_assistant_message`, `result`) are
**top-level** — not nested inside `tool_input` or `tool_response`.

{
  "event_name": "SubagentStop",
  "session_id": "abc-123",
  "dispatcher_trace_id": "uuidv4",
  "tool_name": "",
  "tool_input": null,
  "tool_response": null,
  "agent_type": "product-owner",
  "subagent_name": "product-owner-fallback",
  "last_assistant_message": "Wrote the story spec and committed.",
  "result": "fallback-result-string"
}

Field presence semantics

| Field                   | Type in HookPayload | Present on       | Absent behaviour            |
|-------------------------|---------------------|------------------|-----------------------------|
| agent_type              | Option<String>      | SubagentStop only| None (via #[serde(default)])|
| subagent_name           | Option<String>      | SubagentStop only| None                        |
| last_assistant_message  | Option<String>      | SubagentStop only| None                        |
| result                  | Option<String>      | SubagentStop only| None                        |

JSON null deserializes to None for all four fields — identical to jq's
// null-advance semantics.

Canonical fallback chains (BC-2.02.012 Postconditions 5-6)

// Agent identity — mirrors: jq -r '.agent_type // .subagent_name // "unknown"'
let agent = payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");

// Assistant message — mirrors: jq -r '.last_assistant_message // .result // empty'
let message = payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");

BC reference: BC-2.02.012 — "HookPayload SubagentStop fields: top-level
envelope schema for agent_type, subagent_name, last_assistant_message,
result."

ABI version: These fields are an additive HookPayload extension
under D-6 Option A and D-183.  HOST_ABI_VERSION remains 1.
```

## AC-6 checklist (from story spec)

| Requirement | Status |
|---|---|
| (a) Field names and types documented | PASS — table at HOST_ABI.md:88-93 |
| (b) Presence semantics: SubagentStop only | PASS — "Present only on SubagentStop envelopes" text + table |
| (c) Canonical fallback chains from PC-5 and PC-6 | PASS — both chains present at lines 101-110 |
| (d) Example SubagentStop envelope JSON | PASS — JSON block at lines 72-84 |
| (e) Cross-reference to BC-2.02.012 | PASS — "BC reference: BC-2.02.012" at line 117 |

## Test evidence (file-reading tests)

```
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_section ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_documents_subagentstop_presence_semantics ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_agent_identity_fallback_chain ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_assistant_message_fallback_chain ... ok
test payload::tests::test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_example_json ... ok
```

**PASS**
