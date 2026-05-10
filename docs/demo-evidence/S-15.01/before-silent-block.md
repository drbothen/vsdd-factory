---
scenario: before-state-silent-block
ac_ref: AC-017
bc_ref: BC-1.14.001, BC-9.01.006
story_id: S-15.01
version: "1.0"
status: PASS
---

# Demo (a) — Before-State: Silent Block Scenario

**Scenario:** Pre-S-15.01, the `hooks.json.*` envelope had `"async": true` on
PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate,
WorktreeRemove, and PostToolUseFailure. This envelope flag caused Claude Code to
fire the dispatcher in fire-and-forget mode — it never waited for the exit code.
A plugin with `on_error = "block"` returning exit 2 had its block verdict silently
discarded.

**AC reference:** AC-017 (demo evidence completeness), AC-010 (envelope flip)
**BC reference:** BC-9.01.006 (hooks.json.template envelope sync invariant), BC-1.14.001
**Prism audit finding:** 2026-05-07 — 55 block decisions discarded / 1965 fires/day

---

## Setup

The before-state is represented by the v1 hook envelope configuration. In the
pre-S-15.01 hooks.json, every event entry included `"async": true`:

```json
{
  "hooks": [
    {
      "type": "command",
      "command": "factory-dispatcher",
      "events": ["PostToolUse"],
      "async": true,
      "timeout": 10000
    }
  ]
}
```

The registry (v1, before schema_version field): plugins with `on_error = "block"`
existed side-by-side with the async envelope — no registry-layer invariant
prevented this combination.

```toml
# v1 registry excerpt (no schema_version field, no async field per plugin)
[[hooks]]
name = "validate-template-compliance"
event = "PostToolUse"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"

[hooks.config]
script_path = "legacy-hooks/validate-template-compliance.sh"
```

---

## Expected Outcome (before-state)

With `"async": true` at the envelope, Claude Code fires the dispatcher and
immediately continues. The dispatcher exit code is never observed by Claude Code.

```
# Dispatcher stdin (hook envelope, before-state)
{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"test-silent",
 "tool_input":{"file_path":"CHANGELOG.md"}}

# Dispatcher stderr (what would appear in the dispatcher process)
factory-dispatcher trace=<uuid> event=PostToolUse tool=Write host_abi=1

# Dispatcher exit code
2  (block intent from validate-template-compliance)

# Claude Code behavior (async envelope = fire-and-forget)
# Claude Code DOES NOT observe exit 2.
# Edit proceeds without surfacing the block to the user.
# Block verdict is SILENTLY DISCARDED.
```

---

## Actual Outcome — Prism Audit Evidence

The prism audit (2026-05-07) found 55 entries in `dispatcher-internal-*.jsonl`
where `block_intent = true` was recorded by the dispatcher's internal log, but
no corresponding block was observable at the Claude Code level. The pattern:

```
# Internal log (dispatcher-internal-*.jsonl) — recorded on disk but invisible to Claude Code
{"type":"internal.dispatcher_run","ts":"2026-05-07T...","event":"PostToolUse",
 "plugin":"validate-template-compliance","exit_code":2,"block_intent":true,
 "on_error":"block","trace_id":"<uuid>"}
```

Rate: 1965 PostToolUse PostToolUse dispatches/day; 55 resulted in `block_intent=true`
but no user-visible block (2.8% silent-block rate).

Root cause (confirmed by code inspection of pre-S-15.01 hooks.json files):

```bash
# Pre-S-15.01 grep (any platform variant)
grep '"async"' plugins/vsdd-factory/hooks/hooks.json.darwin-arm64
# Output (before T-3g):
#   "async": true   (on PostToolUse entry)
#   "async": true   (on Stop entry)
#   "async": true   (on SubagentStop entry)
#   ...
```

Claude Code's async envelope semantics meant the dispatcher's exit code was never
propagated back as a block signal. `on_error = "block"` in the registry was
therefore inoperative for all affected events.

---

## Contrast with After-State

See `after-visible-block.md` for the post-S-15.01 behavior where the same
dispatcher exit 2 now surfaces to Claude Code as a user-visible block.

---

## Test File Cross-Link

The registry-layer invariant that prevents this in the after-state is enforced by:
- `crates/factory-dispatcher/src/registry.rs` — `validate_async_block_invariant()` (T-3f)
- `crates/factory-dispatcher/tests/ac017_demo_evidence.rs` — AC-017 file presence check
- VP-078 Harness 1 (`test_BC_7_06_001_block_plus_async_true_rejected_e_reg_002`)

---

## Verdict

PASS — scenario faithfully reproduced from prism audit evidence. The silent-block
mechanism is confirmed by the async envelope in pre-S-15.01 hooks.json files. The
fix (AC-010, T-3g) removes `"async": true` from all 5 platform variants and the
template, making every dispatch synchronous at the envelope.
