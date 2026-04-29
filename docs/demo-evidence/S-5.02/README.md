# Demo Evidence: S-5.02 (SessionEnd hook wiring)

**Story:** S-5.02 — SessionEnd hook wiring  
**Spec convergence:** pass-9 (v2.7), spec sealed  
**GREEN commit:** `3783847`  
**Test status:** 11/11 integration tests GREEN; clippy clean; no workspace regressions  
**VP:** VP-066 — Session-End Plugin Surface Invariant (all BC-4.05.001–005 postconditions)

---

## AC Evidence Files

| File | AC | BCs | One-sentence summary |
|------|----|-----|----------------------|
| [AC1-routing-path.md](AC1-routing-path.md) | AC1 | BC-4.05.004 + BC-4.05.005 | Verifies Layer 1 (`hooks.json.template`) routes `SessionEnd` to `factory-dispatcher` binary and Layer 2 (`hooks-registry.toml`) routes to `session-end-telemetry.wasm`. |
| [AC2-eleven-field-wire-payload.md](AC2-eleven-field-wire-payload.md) | AC2 | BC-4.05.001 | Documents the 11-field wire payload (3 plugin-set + 4 host-enriched + 4 construction-time) with all 7 test variants including EC-001a/b/c, EC-002, EC-003, EC-004. |
| [AC3-no-subprocess.md](AC3-no-subprocess.md) | AC3 | BC-4.05.002 | Shows `CountingMock.invocation_count() == 0` for every `SessionEnd` dispatch; contrasts with SessionStart which calls `exec_subprocess`. |
| [AC4-hooks-json-template.md](AC4-hooks-json-template.md) | AC4 | BC-4.05.004 | Captures the `SessionEnd` entry: `command` = dispatcher binary path, `once: true`, `async: true`, `timeout: 10000`; notes Task 4 was no-op (entry pre-existed). |
| [AC5-hooks-registry-toml.md](AC5-hooks-registry-toml.md) | AC5 | BC-4.05.005 | Captures the new `SessionEnd` entry with zero capability tables and highlights the structural contrast with SessionStart's two-table capability profile. |
| [AC6-vp066-integration-test.md](AC6-vp066-integration-test.md) | AC6 | VP-066 (all 5 BCs) | Full `cargo test -p session-end-telemetry` output; test-to-BC-to-AC coverage map for all 11 tests. |

---

## Architecture Context

S-5.02 wires the second lifecycle event in epic E-5 (FR-046), paired with S-5.01 (SessionStart).
The dual-routing-table architecture (ADR-011) separates:

- **Layer 1** (`hooks.json.template`): Claude Code harness routing — references only the
  dispatcher binary; enforces once-per-session via `once: true`
- **Layer 2** (`hooks-registry.toml`): Dispatcher routing — references only WASM plugin paths;
  declares capability sandbox profile

SessionEnd is simpler than SessionStart:

| Property | SessionEnd (S-5.02) | SessionStart (S-5.01) |
|----------|--------------------|-----------------------|
| Plugin-set fields | 3 (`duration_ms`, `tool_call_count`, `timestamp`) | 6 |
| `exec_subprocess` calls | 0 | 1 |
| Capability tables | 0 | 2 (`read_file` + `exec_subprocess`) |
| `timeout_ms` | 5000 | 8000 |
| Integration tests | 11 | 9 |

Timeout hierarchy (two-level): `timeout_ms = 5000` (dispatcher) < `timeout = 10000` (harness).

S-5.01 lesson applied: no SS-01 BCs, no new host fns, no dispatcher-side dedup.
All routing is Layer 1 once-discipline. Spec converged in 9 passes (vs. 14 for S-5.01).

---

## Verification Property

**VP-066** — Session-End Plugin Surface Invariant  
Proof method: integration  
BCs covered: BC-4.05.001, BC-4.05.002, BC-4.05.003, BC-4.05.004, BC-4.05.005  
Test file: `crates/hook-plugins/session-end-telemetry/tests/integration_test.rs`
