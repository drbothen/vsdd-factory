# AC4 — hooks.json.template SessionEnd entry

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC4 — `hooks.json.template` `SessionEnd` entry is correct  
**BC:** BC-4.05.004  
**GREEN commit:** `3783847`

---

## Entry source

File: `plugins/vsdd-factory/hooks/hooks.json.template`  
Key path: `hooks.SessionEnd[0].hooks[0]`

```json
{
  "type": "command",
  "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
  "timeout": 10000,
  "async": true,
  "once": true
}
```

---

## Postcondition verification (BC-4.05.004)

| Postcondition | Value | Status |
|--------------|-------|--------|
| PC-2: `command` contains `factory-dispatcher` | yes | PASS |
| PC-2: `command` does NOT contain `.wasm` (ADR-011) | confirmed absent | PASS |
| PC-3: `once: true` | `true` | PASS |
| PC-3: `async: true` | `true` | PASS |
| PC-5: `timeout: 10000` | `10000` | PASS |

---

## Task 4 was a no-op

Per story spec Task 4: the `SessionEnd` entry pre-existed in `hooks.json.template`
before S-5.02 began. No template modification was required. The entry was verified
by `test_bc_4_05_004_hooks_json_template_has_session_end` (`integration_test.rs:749`).

```text
test session_end_integration::test_bc_4_05_004_hooks_json_template_has_session_end ... ok
```

---

## Timeout hierarchy

`timeout_ms = 5000` (dispatcher budget, Layer 2) < `timeout = 10000` (harness timeout, Layer 1).

Two-level hierarchy — simpler than SessionStart's three-level (SessionStart also has a
subprocess timeout of 5000ms). SessionEnd has no subprocess, so the hierarchy collapses
to two levels.

---

## Platform variants

The `{{PLATFORM}}` and `{{EXE_SUFFIX}}` placeholders are expanded by the activation skill
into five platform-specific `hooks.json.*` files:

- `hooks.json.darwin-arm64`
- `hooks.json.darwin-x64`
- `hooks.json.linux-arm64`
- `hooks.json.linux-x64`
- `hooks.json.windows-x64`

Since the template was not modified by S-5.02 (Task 4 no-op), no regeneration was needed.
