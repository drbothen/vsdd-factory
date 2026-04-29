# AC3: Factory-health subprocess fail-open

**Story:** S-5.01  
**AC:** AC3 — Factory-health brief triggered on session start; plugin emits `session.started` with `factory_health = "unknown"` if factory-health fails, times out, or returns CAPABILITY_DENIED  
**Trace:** BC-4.04.002 (postconditions 1–3, invariant 1)

---

## Evidence 1: All three BC-4.04.002 tests

Command:
```
cargo test -p session-start-telemetry test_bc_4_04_002 --tests -- --nocapture
```

Output:
```
running 3 tests
test session_start_integration::test_bc_4_04_002_factory_health_fail_open ... ok
test session_start_integration::test_bc_4_04_002_factory_health_timeout ... ok
test session_start_integration::test_bc_4_04_002_factory_health_healthy_on_success ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
```

---

## Evidence 2: Fail-open semantics

The three tests cover the full BC-4.04.002 postcondition surface:

| Test | Scenario | Expected factory_health | session.started emitted? |
|------|----------|------------------------|--------------------------|
| `test_bc_4_04_002_factory_health_healthy_on_success` | subprocess exits 0 with "OK" stdout | `"healthy"` | yes |
| `test_bc_4_04_002_factory_health_fail_open` | BinaryNotFound / CAPABILITY_DENIED / non-zero exit | `"unknown"` | yes (fail-open) |
| `test_bc_4_04_002_factory_health_timeout` | subprocess exceeds 5000ms timeout | `"unknown"` | yes (fail-open) |

**Fail-open invariant (BC-4.04.002 invariant 1):** `session.started` is ALWAYS emitted regardless of factory-health outcome. A CountingMock in the integration tests asserts `invocation_count == 1` even in error paths.

**Timeout hierarchy verified:**
- subprocess timeout: 5000ms
- dispatcher timeout_ms: 8000ms (hooks-registry.toml)
- harness timeout: 10000ms (hooks.json.template)
- Invariant: 5000 < 8000 < 10000 (BC-4.04.004 Invariant 3 + BC-4.04.005 Invariant 5)

---

**Verdict: PASS**

All three factory-health paths verified: healthy success, fail-open on BinaryNotFound/CAPABILITY_DENIED, fail-open on timeout. In all cases `session.started` is emitted exactly once.
