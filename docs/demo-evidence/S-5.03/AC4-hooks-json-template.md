# AC4 — hooks.json.template entries (`once` key absent)

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC4 — Template contains WorktreeCreate + WorktreeRemove keys; `once` key completely absent
**BCs:** BC-4.07.003
**GREEN commit:** `8336cd0`

---

## What is verified

`plugins/vsdd-factory/hooks/hooks.json.template` contains both `WorktreeCreate` and `WorktreeRemove` top-level keys, each routing to the dispatcher binary with the correct properties and — critically — no `once` key.

---

## Actual template content

```json
"WorktreeCreate": [
  {
    "hooks": [
      {
        "type": "command",
        "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
        "timeout": 10000,
        "async": true
      }
    ]
  }
],
"WorktreeRemove": [
  {
    "hooks": [
      {
        "type": "command",
        "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
        "timeout": 10000,
        "async": true
      }
    ]
  }
]
```

File: `plugins/vsdd-factory/hooks/hooks.json.template`

---

## BC-4.07.003 Invariant 1: `once` key MUST be absent

The `once` key is **completely absent** from both entries — not `"once": false`, not `"once": true`. The key must not exist at all.

**Reason:** Worktree events can re-fire on Claude Code reconnect (EC-001). A present `once` key (regardless of value) risks future Claude Code parser changes misinterpreting the intent. Defensive omission is the safe choice.

**Contrast with session events:**

| Event | `once` in hooks.json.template | Rationale |
|-------|-------------------------------|-----------|
| `SessionStart` | `"once": true` | Fires exactly once per session |
| `SessionEnd` | `"once": true` | Fires exactly once per session |
| `WorktreeCreate` | **absent** | Can re-fire on reconnect (EC-001) |
| `WorktreeRemove` | **absent** | Can re-fire on reconnect (EC-001) |

---

## Per-platform variants

The template is materialized to 5 per-platform variant files (SUFFIX-based naming per S-5.01 lesson 5):

```
plugins/vsdd-factory/hooks/hooks.json.darwin-arm64
plugins/vsdd-factory/hooks/hooks.json.darwin-x64
plugins/vsdd-factory/hooks/hooks.json.linux-arm64
plugins/vsdd-factory/hooks/hooks.json.linux-x64
plugins/vsdd-factory/hooks/hooks.json.windows-x64
```

Each contains the same `WorktreeCreate` and `WorktreeRemove` entries with the `{{PLATFORM}}` and `{{EXE_SUFFIX}}` placeholders resolved for the target platform.

---

## BC-4.07.003 checklist

| Invariant | Status |
|-----------|--------|
| `WorktreeCreate` key present | confirmed |
| `WorktreeRemove` key present | confirmed |
| `command` contains `factory-dispatcher` (Layer 1 → dispatcher) | confirmed |
| `command` does NOT contain `.wasm` (ADR-011 layer separation) | confirmed |
| `once` key absent (not false, not true — completely absent) | confirmed |
| `async: true` | confirmed |
| `timeout: 10000` (harness outer bound) | confirmed |

---

## Integration test assertion

```text
test worktree_integration::test_bc_4_07_003_hooks_json_template_has_worktree_create_and_remove ... ok
```

Key assertion in test (line ~858):
```rust
assert!(
    entry.get("once").is_none(),
    "BC-4.07.003 Invariant 1: {event_name} entry must NOT have a 'once' key ..."
);
```
