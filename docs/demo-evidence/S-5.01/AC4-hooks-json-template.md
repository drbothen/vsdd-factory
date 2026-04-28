# AC4: hooks.json.template Layer 1 routing

**Story:** S-5.01  
**AC:** AC4 — `hooks.json.template` SessionStart entry uses `command` routing to factory-dispatcher binary with `once: true`, `async: true`, `timeout: 10000`; NO `.wasm` filename reference  
**Trace:** BC-4.04.004 (postconditions 1–5, invariant 1)

---

## Evidence 1: Full SessionStart entry

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

---

## Evidence 2: Field-by-field verification against BC-4.04.004 postconditions

| Postcondition | Requirement | Observed Value | Status |
|---------------|-------------|----------------|--------|
| PC-1: SessionStart key present | `hooks.SessionStart` exists in template | Key present with 1 entry | PASS |
| PC-2: command references dispatcher binary | `command` value points to `factory-dispatcher` binary | `...dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` | PASS |
| PC-3: once: true | once-per-session discipline at Layer 1 | `"once": true` | PASS |
| PC-4: async: true | non-blocking execution | `"async": true` | PASS |
| PC-5: timeout: 10000 | harness timeout = 10000ms | `"timeout": 10000` | PASS |
| Invariant 1: NO .wasm filename | command MUST NOT reference `.wasm` files | command value contains no `.wasm` reference | PASS |

---

## Evidence 3: Timeout hierarchy

```
subprocess timeout:    5000ms  (BC-4.04.002 invariant 4)
dispatcher timeout_ms: 8000ms  (hooks-registry.toml — BC-4.04.005)
harness timeout:      10000ms  (hooks.json.template — BC-4.04.004 PC-5)
```

Invariant satisfied: 5000 < 8000 < 10000

---

## Evidence 4: Test passing

Command:
```
cargo test -p session-start-telemetry test_bc_4_04_004 --tests -- --nocapture
```

Output:
```
running 1 test
test session_start_integration::test_bc_4_04_004_hooks_json_template_has_session_start ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

---

**Verdict: PASS**

All 5 BC-4.04.004 postconditions satisfied. ADR-011 invariant 1 verified: no `.wasm` filename appears in the hooks.json.template entry. Layer 1 once-discipline (`once: true`) established at the harness level.
