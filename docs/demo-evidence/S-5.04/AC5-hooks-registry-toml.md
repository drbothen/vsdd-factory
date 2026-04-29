# AC5: hooks-registry.toml Entry

**AC:** AC5 — `[[hooks]]` entry with correct field values; NO capability tables; NO `once` field; exactly one entry.
**BC:** BC-4.08.003 Postconditions 1–7
**VP-068 test:** `test_bc_4_08_003_hooks_registry_toml_post_tool_use_failure_entry` (test 9)

## Registry Entry

**Command:**
```
grep -B 1 -A 6 'name = "tool-failure-hooks"' plugins/vsdd-factory/hooks-registry.toml
```

**Output** (`plugins/vsdd-factory/hooks-registry.toml`, lines 53–57):
```toml
[[hooks]]
name = "tool-failure-hooks"
event = "PostToolUseFailure"
plugin = "hook-plugins/tool-failure-hooks.wasm"
timeout_ms = 5000
```

## BC-4.08.003 Postcondition Checklist

| Postcondition | Value | Status |
|---------------|-------|--------|
| PC-1: `name = "tool-failure-hooks"` | required field; no default in RegistryEntry | PASS |
| PC-2: `event = "PostToolUseFailure"` | routing key for dispatcher | PASS |
| PC-2: `plugin = "hook-plugins/tool-failure-hooks.wasm"` | includes `hook-plugins/` prefix per Invariant 3 | PASS |
| PC-3: `timeout_ms = 5000` | field name `timeout_ms` (not `timeout`, not `epoch_budget_ms`); `deny_unknown_fields` rejects wrong name | PASS |
| PC-4/PC-5: NO capability tables | entry has no `[hooks.capabilities.*]` | PASS |
| No `once` field | RegistryEntry has no such field; `deny_unknown_fields` would reject it | PASS |
| Exactly one entry | only one entry with `event = "PostToolUseFailure"` | PASS |

## Contrast: S-5.01 SessionStart Has Capability Tables

**Command:**
```
grep -B 1 -A 12 'name = "session-start-telemetry"' plugins/vsdd-factory/hooks-registry.toml
```

**Output** (`hooks-registry.toml`, lines 15–25):
```toml
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

`session-start-telemetry` (S-5.01) reads `.claude/settings.local.json` and execs `factory-health`. It declares 2 capability tables. `tool-failure-hooks` (S-5.04) declares ZERO capability tables — all data comes from the incoming envelope, no filesystem access or subprocess needed.

## Zero-Capability Rationale (Option A)

`tool-failure-hooks` is an emit-only plugin. The entire payload (tool name + error message) is delivered in the envelope's `tool_input` field. No file reads are needed to enrich the event, and no subprocess is needed to report the failure. This is the Option A scoping decision: minimum capability footprint for the deny-by-default sandbox.

Siblings `session-end-telemetry` (S-5.02), `worktree-hooks` (S-5.03), and `tool-failure-hooks` (S-5.04) all use Option A zero-capability scoping. Only `session-start-telemetry` (S-5.01) needs capabilities for the factory-health subprocess call.

## Field Name Discipline

`timeout_ms` is the canonical RegistryEntry field name. Cargo `deny_unknown_fields` on the production struct rejects:
- `timeout` — used in Layer 1 (hooks.json) but not Layer 2 (registry)
- `epoch_budget_ms` — a common mismatch from S-5.01 pass-1

The integration test uses `toml::Value` parsing and asserts `entry["timeout_ms"].as_integer() == Some(5000)`.
