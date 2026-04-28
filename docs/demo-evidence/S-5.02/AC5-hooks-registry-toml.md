# AC5 — hooks-registry.toml SessionEnd entry

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC5 — `hooks-registry.toml` `SessionEnd` entry is correct  
**BC:** BC-4.05.005  
**GREEN commit:** `3783847`

---

## SessionEnd entry

File: `plugins/vsdd-factory/hooks-registry.toml`

```toml
[[hooks]]
name = "session-end-telemetry"
event = "SessionEnd"
plugin = "hook-plugins/session-end-telemetry.wasm"
timeout_ms = 5000
```

---

## Postcondition verification (BC-4.05.005)

| Postcondition | Value | Status |
|--------------|-------|--------|
| PC-1: `name = "session-end-telemetry"` | confirmed | PASS |
| PC-1: `event = "SessionEnd"` | confirmed | PASS |
| PC-2: `plugin` includes `hook-plugins/` prefix | `hook-plugins/session-end-telemetry.wasm` | PASS |
| PC-3: `timeout_ms = 5000` (not `epoch_budget_ms`) | `5000` | PASS |
| PC-4/5: NO `[hooks.capabilities.read_file]` table | absent | PASS |
| PC-4/5: NO `[hooks.capabilities.exec_subprocess]` table | absent | PASS |
| Inv-2: NO `once` field | absent | PASS |
| EC-003: exactly one `SessionEnd` entry | 1 entry | PASS |

---

## Contrast with SessionStart (S-5.01)

SessionEnd has the simplest possible capability profile — zero declared capabilities.
SessionStart (S-5.01) required both `read_file` and `exec_subprocess`:

```toml
# SessionStart entry (S-5.01) — for contrast
[[hooks]]
name = "session-start-telemetry"
event = "SessionStart"
plugin = "hook-plugins/session-start-telemetry.wasm"
timeout_ms = 8000

[hooks.capabilities.read_file]
path_allow = [".claude/settings.local.json"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["factory-health"]
```

Key differences:

| Property | SessionEnd (S-5.02) | SessionStart (S-5.01) |
|----------|--------------------|-----------------------|
| `timeout_ms` | 5000 | 8000 |
| `read_file` capability | absent | present |
| `exec_subprocess` capability | absent | present |
| Capability tables total | 0 | 2 |

The `timeout_ms` difference reflects the absence of subprocess wait time in SessionEnd:
5000ms is sufficient for an envelope-only read + single emit.

---

## Architecture compliance notes

- `timeout_ms` field name (not `epoch_budget_ms`) — F-13 ruling; `RegistryEntry` uses
  `deny_unknown_fields` which would hard-reject any unknown field name
- `hook-plugins/` prefix on `plugin` — required for dispatcher binary resolution at runtime
- No `once` field — `RegistryEntry` schema has none; once-discipline is exclusively Layer 1
  (`hooks.json.template once: true` per BC-4.05.004)

---

## Integration test confirmation

```text
test session_end_integration::test_bc_4_05_005_hooks_registry_toml_has_session_end ... ok
```

Source: `tests/integration_test.rs:816`
