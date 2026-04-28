# Demo Evidence: S-5.01 (SessionStart hook wiring)

**Story:** S-5.01 — SessionStart hook wiring  
**Spec convergence:** pass-14 (v2.12), D-135, factory-artifacts `aaaf56f`  
**GREEN commit:** `b5c8a66`  
**Test status:** 9/9 integration tests + 14/14 unit tests GREEN; clippy clean; no workspace regressions  
**v1.0 simplifications:** `plugin_count = "0"`, `tool_deps = null` (v1.1 candidates per BC-4.04.001)

---

## AC Evidence Files

| File | AC | One-sentence summary |
|------|----|----------------------|
| [AC1-dispatcher-routing.md](AC1-dispatcher-routing.md) | AC1 | Verifies Layer 1 (hooks.json.template) routes SessionStart to the factory-dispatcher binary and Layer 2 (hooks-registry.toml) routes to session-start-telemetry.wasm via BC-4.04.004 + BC-4.04.005. |
| [AC2-session-started-fields.md](AC2-session-started-fields.md) | AC2 | Documents the 6 plugin-set fields, 4 host-enriched fields, and 4 construction-time fields (14 wire fields total per ADR-011) verified by the BC-4.04.001 integration test. |
| [AC3-factory-health-fail-open.md](AC3-factory-health-fail-open.md) | AC3 | Demonstrates all three BC-4.04.002 paths: healthy success, fail-open on BinaryNotFound/CAPABILITY_DENIED, and fail-open on timeout — session.started always emitted. |
| [AC4-hooks-json-template.md](AC4-hooks-json-template.md) | AC4 | Verifies hooks.json.template has `command` pointing to dispatcher binary (no .wasm reference), `timeout: 10000`, `async: true`, `once: true` per BC-4.04.004 postconditions 1–5. |
| [AC5-hooks-registry-toml.md](AC5-hooks-registry-toml.md) | AC5 | Verifies hooks-registry.toml SessionStart entry has all required fields plus read_file and exec_subprocess capability tables per BC-4.04.005, with no `once` field (deny_unknown_fields). |
| [AC6-integration-test-full-path.md](AC6-integration-test-full-path.md) | AC6 | Full test suite output showing 9 integration tests + 14 unit tests = 23 total GREEN; VP-065 coverage map across all 5 BCs (BC-4.04.001–005). |

---

## Architecture Context

This story implements the first lifecycle event in epic E-5 (FR-046). The dual-routing-table architecture (ADR-011) separates:
- **Layer 1** (`hooks.json.template`): Claude Code harness routing — references only the dispatcher binary; enforces once-per-session via `once: true`
- **Layer 2** (`hooks-registry.toml`): Dispatcher routing — references only WASM plugin paths; provides capability declarations

Timeout hierarchy: `5000ms` (subprocess) < `8000ms` (dispatcher) < `10000ms` (harness).
