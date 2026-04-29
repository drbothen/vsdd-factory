# AC1: Dispatcher receives SessionStart via Layer 1 + Layer 2

**Story:** S-5.01  
**AC:** AC1 — Dispatcher receives `SessionStart` event via `hooks.json.template` (Layer 1) routing to dispatcher binary, then routes to `session-start-telemetry.wasm` via `hooks-registry.toml` (Layer 2)  
**Trace:** BC-4.04.004 (postconditions 1–4) + BC-4.04.005 (postconditions 1–6)

---

## Evidence 1: hooks.json.template SessionStart entry (Layer 1)

Command:
```
cat plugins/vsdd-factory/hooks/hooks.json.template | python3 -c "import sys,json; d=json.load(sys.stdin); print(json.dumps(d['hooks']['SessionStart'][0], indent=2))"
```

Output:
```json
{
  "hooks": [
    {
      "type": "command",
      "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
      "timeout": 10000,
      "async": true,
      "once": true
    }
  ]
}
```

Verified:
- `command` routes to `factory-dispatcher` binary (NOT a `.wasm` filename) — satisfies BC-4.04.004 invariant 1
- `timeout: 10000` — satisfies BC-4.04.004 postcondition 5 + timeout hierarchy (10000 > 8000 > 5000)
- `async: true` — satisfies BC-4.04.004 postcondition 4
- `once: true` — satisfies BC-4.04.004 postcondition 3; Layer 1 once-per-session discipline

---

## Evidence 2: hooks-registry.toml SessionStart entry (Layer 2)

Command:
```
grep -A 12 'name = "session-start-telemetry"' plugins/vsdd-factory/hooks-registry.toml
```

Output:
```
name = "session-start-telemetry"
event = "SessionStart"
plugin = "hook-plugins/session-start-telemetry.wasm"
timeout_ms = 8000

[hooks.capabilities.read_file]
path_allow = [".claude/settings.local.json"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["factory-health"]
```

Verified:
- `event = "SessionStart"` — routes to correct event
- `plugin = "hook-plugins/session-start-telemetry.wasm"` — Layer 2 WASM reference (ADR-011 separation)
- `timeout_ms = 8000` — satisfies BC-4.04.005 postcondition 6 + invariant 5 (8000 > 5000 subprocess)
- Capability tables present for `read_file` and `exec_subprocess`
- No `once` field (RegistryEntry schema has none; deny_unknown_fields would reject it per BC-4.04.005 invariant 2)

---

## Evidence 3: Tests passing

Command:
```
cargo test -p session-start-telemetry test_bc_4_04_004 --tests -- --nocapture
cargo test -p session-start-telemetry test_bc_4_04_005 --tests -- --nocapture
```

Output (test_bc_4_04_004):
```
running 1 test
test session_start_integration::test_bc_4_04_004_hooks_json_template_has_session_start ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

Output (test_bc_4_04_005):
```
running 1 test
test session_start_integration::test_bc_4_04_005_hooks_registry_toml_has_session_start ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.01s
```

---

**Verdict: PASS**

Both routing layers verified structurally (file inspection) and dynamically (integration tests). ADR-011 Layer 1/Layer 2 separation holds: `command` in hooks.json.template references only the dispatcher binary; `plugin` in hooks-registry.toml references only the `.wasm` file.
