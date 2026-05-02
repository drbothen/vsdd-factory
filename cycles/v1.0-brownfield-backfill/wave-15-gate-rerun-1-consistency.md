---
document_type: consistency-review
wave: 15
gate_run: rerun-1
producer: consistency-validator
date: 2026-05-02
branch_under_review: 1ab1d6f
verdict: FINDINGS
input_hash: rerun-1-1ab1d6f
---

# W-15 Wave Gate Re-run #1 — Consistency Findings

**Branch:** develop @ 1ab1d6f

## Verdict: FINDINGS

### CRIT-CONS-001: update-wave-state-on-merge unreachable in production

**File:** `crates/hook-plugins/update-wave-state-on-merge/src/lib.rs:310-331`

`wave_state_hook_logic` reads agent identity and result via `payload.tool_input.get("agent_type")` / `payload.tool_input.get("last_assistant_message")`. The dispatcher's SubagentStop envelope sends `tool_input: null`. The plugin always sees `agent_type = "unknown"` and exits at `is_pr_manager_agent` guard. Merge-tracking logic never fires in production. Test fixtures masked this by populating both `tool_input` and the top-level typed fields.

**Fix:** Replace with BC-2.02.012 typed projection: `payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")` and similar for result via `payload.last_assistant_message.as_deref().or(payload.result.as_deref()).unwrap_or("")`.

### LOW-CONS-002: track-agent-stop:65 doc comment inversion

**File:** `crates/hook-plugins/track-agent-stop/src/lib.rs:65,184`

Doc comment says `c.is_whitespace()` where impl uses `!c.is_whitespace()`. Line 184 says "byte-filter" (stale wording from pre-chars refactor).

**Recommended fix:** Correct doc comment to `!c.is_whitespace()` and update stale "byte-filter" reference to "char-filter".
