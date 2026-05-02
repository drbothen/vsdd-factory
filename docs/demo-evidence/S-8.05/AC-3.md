# AC-003: Agent scoping; BC-2.02.012 typed projection fallback chains

**BC traces:** BC-7.04.041 postcondition 1 (agent scope); BC-2.02.012 postcondition 5 (canonical agent identity fallback chain)
**Status:** PASS

## What was verified

### Agent scoping (BC-7.04.041)

Hook exits 0 immediately (no checks run) when the resolved agent identifier does not
contain `pr-reviewer`, `pr_reviewer`, or `pr-review-triage`. Matching is substring
containment — dotted variants (e.g., `pr.reviewer`) are not canonical and do not match.

### BC-2.02.012 Postcondition 5 — canonical agent identity fallback chain

```rust
let agent: &str = payload
    .agent_type
    .as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");
```

This mirrors `validate-pr-review-posted.sh:21`:
`AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')`.
serde_json's `Option<String>` + `#[serde(default)]` deserialization provides jq `//` null-as-advance semantics for free (BC-2.02.012 Invariant 3).

### BC-2.02.012 Postcondition 6 — canonical assistant-message fallback chain

```rust
let result: &str = payload
    .last_assistant_message
    .as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");
```

### Agent dual-fallback sub-cases (bats cases g.1 and g.2)

- **(g.1)** `agent_type` present, `subagent_name` absent: primary arm of fallback chain used. Bats test ok 13 confirms.
- **(g.2)** `agent_type` absent, `subagent_name = "pr-reviewer"` present: fallback arm exercised; resolved value still applies containment checks. Bats test ok 14 confirms.

No `envelope.get(...)` calls anywhere in the crate — strict BC-2.02.012 Invariant 5 compliance.

## Recording

[AC-003-agent-scoping-bc2-02-012.gif](AC-003-agent-scoping-bc2-02-012.gif)
