# AC1 — Full routing path wired

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC1 — Full Layer 1 → dispatcher → Layer 2 → plugin wiring path  
**BCs:** BC-4.05.004 + BC-4.05.005  
**GREEN commit:** `3783847`

---

## What is verified

`SessionEnd` travels through two routing layers (ADR-011 dual-hook-routing-tables):

| Layer | File | Routes to |
|-------|------|-----------|
| Layer 1 (harness) | `plugins/vsdd-factory/hooks/hooks.json.template` | `factory-dispatcher` binary |
| Layer 2 (dispatcher) | `plugins/vsdd-factory/hooks-registry.toml` | `session-end-telemetry.wasm` |

---

## Layer 1 — hooks.json.template (BC-4.05.004)

File: `plugins/vsdd-factory/hooks/hooks.json.template`  
Key: `hooks.SessionEnd[0].hooks[0]`

```json
{
  "type": "command",
  "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
  "timeout": 10000,
  "async": true,
  "once": true
}
```

Invariants confirmed:
- `command` contains `factory-dispatcher` — routes to the dispatcher binary
- `command` does NOT contain `.wasm` — no ADR-011 layer violation
- `once: true` — Layer 1 once-discipline (not the plugin's concern)
- `async: true` — non-blocking harness invocation
- `timeout: 10000` — harness timeout (outer bound of timeout hierarchy)

---

## Layer 2 — hooks-registry.toml (BC-4.05.005)

File: `plugins/vsdd-factory/hooks-registry.toml`

```toml
[[hooks]]
name = "session-end-telemetry"
event = "SessionEnd"
plugin = "hook-plugins/session-end-telemetry.wasm"
timeout_ms = 5000
```

Invariants confirmed:
- `plugin` path includes `hook-plugins/` prefix — required for dispatcher binary resolution
- `timeout_ms` field name (not `epoch_budget_ms`) — matches `RegistryEntry` schema
- No `[hooks.capabilities.*]` tables — zero-capability sandbox profile
- No `once` field — `RegistryEntry` has no such field; `deny_unknown_fields` would reject it

---

## Integration test confirmation

Both routing entries are verified by tests in the VP-066 harness:

```
test session_end_integration::test_bc_4_05_004_hooks_json_template_has_session_end ... ok
test session_end_integration::test_bc_4_05_005_hooks_registry_toml_has_session_end ... ok
```

Sources:
- `crates/hook-plugins/session-end-telemetry/tests/integration_test.rs:749`  (BC-4.05.004 test)
- `crates/hook-plugins/session-end-telemetry/tests/integration_test.rs:816`  (BC-4.05.005 test)
