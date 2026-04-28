# AC5: hooks-registry.toml Layer 2 routing

**Story:** S-5.01  
**AC:** AC5 — `hooks-registry.toml` has SessionStart entry with all required fields and capability tables  
**Trace:** BC-4.04.005 (postconditions 1–6, invariants 1–3, invariant 6)

---

## Evidence 1: Full SessionStart entry

Command:
```
awk '/name = "session-start-telemetry"/,/^$/' plugins/vsdd-factory/hooks-registry.toml | head -20
```

Output (from `hooks-registry.toml` lines 15–25):
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

---

## Evidence 2: Field-by-field verification against BC-4.04.005 postconditions

| Postcondition | Requirement | Observed Value | Status |
|---------------|-------------|----------------|--------|
| PC-1: name | `name = "session-start-telemetry"` | `name = "session-start-telemetry"` | PASS |
| PC-2: event | `event = "SessionStart"` | `event = "SessionStart"` | PASS |
| PC-3: plugin | `plugin = "hook-plugins/session-start-telemetry.wasm"` | `plugin = "hook-plugins/session-start-telemetry.wasm"` | PASS |
| PC-4: capability tables | `[hooks.capabilities.read_file]` + `[hooks.capabilities.exec_subprocess]` | Both tables present | PASS |
| PC-5: path_allow | `path_allow = [".claude/settings.local.json"]` | `path_allow = [".claude/settings.local.json"]` | PASS |
| PC-6: timeout_ms | `timeout_ms = 8000` | `timeout_ms = 8000` | PASS |

---

## Evidence 3: Invariant verification

| Invariant | Requirement | Observed | Status |
|-----------|-------------|----------|--------|
| Invariant 1: No `once` field | RegistryEntry schema has no `once` field; `deny_unknown_fields` would reject it | No `once` field present | PASS |
| Invariant 2: TOML table form for capabilities | `[hooks.capabilities.read_file]` not array `capabilities = [...]` | TOML table form used | PASS |
| Invariant 3: capability tables required | `read_file` + `exec_subprocess` tables declared | Both present | PASS |
| Invariant 5: timeout_ms >= subprocess timeout | `timeout_ms = 8000` >= `5000ms` subprocess | 8000 > 5000 | PASS |
| Invariant 6: name unique stable identifier | `name = "session-start-telemetry"` matches plugin crate name | Unique in registry | PASS |

---

## Evidence 4: Once-discipline architecture note

`hooks-registry.toml` (Layer 2) deliberately has NO `once` field. Once-per-session discipline is enforced entirely at Layer 1 (`hooks.json.template` `once: true`). This is BC-4.04.004 invariant 1 / BC-4.04.005 invariant 2 design: the dispatcher Layer 2 registry does not need to know about per-session deduplication.

---

## Evidence 5: Test passing

Command:
```
cargo test -p session-start-telemetry test_bc_4_04_005 --tests -- --nocapture
```

Output:
```
running 1 test
test session_start_integration::test_bc_4_04_005_hooks_registry_toml_has_session_start ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.01s
```

---

**Verdict: PASS**

All 6 BC-4.04.005 postconditions and all applicable invariants satisfied. Entry was added by direct edit (generator retired per F-17). Capability tables use TOML table form as required. `deny_unknown_fields` enforcement means no `once` field is permissible.
