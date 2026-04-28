# AC2: session.started emitted with 6 plugin-set fields

**Story:** S-5.01  
**AC:** AC2 — `session.started` event emitted with 6 plugin-set required fields  
**Trace:** BC-4.04.001 (postconditions 1–2, invariant 1) + BC-1.05.012 (enrichment + RESERVED_FIELDS filter)

---

## Evidence 1: Integration test

Command:
```
cargo test -p session-start-telemetry test_bc_4_04_001_session_started_emitted_with_required_fields --tests -- --nocapture
```

Output:
```
running 1 test
test session_start_integration::test_bc_4_04_001_session_started_emitted_with_required_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

---

## Evidence 2: Field provenance (14 wire fields total per ADR-011)

### 6 Plugin-set fields (set by session-start-telemetry plugin)

| Field | BC Reference | v1.0 Value / Format |
|-------|-------------|---------------------|
| `factory_version` | BC-4.04.001 Postcondition 2 | Cargo.toml version of the plugin crate |
| `plugin_count` | BC-4.04.001 Postcondition 2 | `"0"` (v1.0 simplification; v1.1 candidate) |
| `activated_platform` | BC-4.04.001 Postcondition 2 | Read from `.claude/settings.local.json` key `vsdd-factory.activated_platform` via `read_file` host fn; read failures → `"unknown"` (fail-open per EC-003) |
| `factory_health` | BC-4.04.001 Postcondition 2 | Output of `exec_subprocess("factory-health", ["--brief"])`; failure → `"unknown"` (fail-open per BC-4.04.002) |
| `tool_deps` | BC-4.04.001 Postcondition 2 | `null` (v1.0 simplification; v1.1 candidate) |
| `timestamp` | BC-4.04.001 Postcondition 2 | Plugin-generated ISO-8601 with milliseconds matching `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$` |

### 4 Host-enriched fields (auto-injected by emit_event host fn from HostContext per BC-1.05.012)

| Field | Injected By |
|-------|-------------|
| `dispatcher_trace_id` | `emit_event` host fn — from dispatcher `HostContext` |
| `session_id` | `emit_event` host fn — from dispatcher `HostContext` |
| `plugin_name` | `emit_event` host fn — from dispatcher `HostContext` |
| `plugin_version` | `emit_event` host fn — from dispatcher `HostContext` |

### 4 Construction-time fields (set by `InternalEvent::now()`)

| Field | Set By |
|-------|--------|
| `ts` | `InternalEvent::now()` |
| `ts_epoch` | `InternalEvent::now()` |
| `schema_version` | `InternalEvent::now()` |
| `type` | `InternalEvent::now()` |

---

## Evidence 3: RESERVED_FIELDS invariant

The plugin does NOT attempt to set any of the 8 `RESERVED_FIELDS`:
`dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`.  
Any plugin attempt to set these is silently dropped by `emit_event.rs` (BC-1.05.012 filter).

---

## Evidence 4: v1.0 simplifications

Per implementer notes and BC-4.04.001:
- `plugin_count = "0"` — dynamic hook registry enumeration deferred to v1.1
- `tool_deps = null` — tool dependency scanning deferred to v1.1

Both are spec-compliant v1.0 values; marked as v1.1 candidates in the behavioral contract.

---

**Verdict: PASS**

Integration test `test_bc_4_04_001_session_started_emitted_with_required_fields` verifies all 6 plugin-set fields are present and correctly typed. Field provenance documented per ADR-011 4+4+6 = 14 wire fields.
