# AC-005 Evidence: Bats Parity Tests Pass (5/5)

**AC statement:** Bats parity tests at
`tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats` cover all five
scenarios via the production dispatcher path. All tests pass.

**BC trace:** BC-7.03.092 postcondition 1 + BC-7.03.091 postcondition 1

---

## Test File Location

`tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats`

Note: the story specified `tests/integration/hooks/` but the implementation placed
tests in `tests/integration/E-8-hook-plugins/` (consistent with the E-8 integration
test directory convention established by sibling stories).

## Dispatcher Invocation Path

Tests invoke the dispatcher via the production path (AC-005 requirement; direct
`.wasm` invocation via wasmtime is NOT used):

```bash
run bash -c "cd '$FIXTURE_DIR' && echo '$STOP_STDIN' | '$DISPATCHER_BIN' 2>&1"
```

Stop hook stdin envelope:
```json
{"hook_event_name": "Stop", "session_id": "test-session-001",
 "transcript_path": "/tmp/test-transcript.jsonl"}
```

The test helper `cd`s to `FIXTURE_DIR` (a `mktemp -d` temporary directory) and
exports `CLAUDE_PROJECT_DIR=$FIXTURE_DIR` so the dispatcher resolves
`.factory/wave-state.yaml` relative to the fixture directory. This is the correct
path for the cwd path-resolution fix (dispatcher bonus fix #2).

## Test Results (5/5 PASS)

### AC-005(a): One pending wave — exit 0 + WAVE GATE REMINDER

- Fixture: `waves.W-15.gate_status: pending`
- Assertions:
  - `[ "$status" -eq 0 ]` — PASS
  - `[[ "$output" == *"WAVE GATE REMINDER:"* ]]` — PASS
  - `[[ "$output" == *"  - W-15 gate is pending. Run the gate before starting the next wave."* ]]` — PASS
  - `[[ "$output" == *"Invoke /vsdd-factory:wave-gate"* ]]` — PASS

### AC-005(b): Two pending waves — exit 0 + both names in REMINDER

- Fixture: `W-15.gate_status: pending`, `W-16.gate_status: pending`
- Assertions:
  - `[ "$status" -eq 0 ]` — PASS
  - `[[ "$output" == *"W-15"* ]]` — PASS
  - `[[ "$output" == *"W-16"* ]]` — PASS
  - `[[ "$output" == *"WAVE GATE REMINDER:"* ]]` — PASS

### AC-005(c): All waves passed — exit 0 + no REMINDER

- Fixture: `W-14.gate_status: passed`, `W-15.gate_status: passed`
- Assertions:
  - `[ "$status" -eq 0 ]` — PASS
  - `[[ "$output" != *"WAVE GATE REMINDER:"* ]]` — PASS
  - `[[ "$output" != *"gate is pending"* ]]` — PASS

### AC-005(d): wave-state.yaml absent — exit 0 + no output

- Fixture: empty fixture directory (no `.factory/` created)
- Assertions:
  - `[ "$status" -eq 0 ]` — PASS
  - `[[ "$output" != *"WAVE GATE REMINDER:"* ]]` — PASS
  - `[[ "$output" != *"gate is pending"* ]]` — PASS

### AC-005(e): Malformed YAML — exit 0 + no output (graceful parse error)

- Fixture: YAML with broken indentation and unclosed bracket
- Assertions:
  - `[ "$status" -eq 0 ]` — PASS
  - `[[ "$output" != *"WAVE GATE REMINDER:"* ]]` — PASS
  - `[[ "$output" != *"gate is pending"* ]]` — PASS

## Performance Note

Per E-8 AC-7 Tier 1 exemption: no performance regression ceiling is enforced. No
S-8.00 baseline exists for warn-pending-wave-gate. Performance is informational only.
No perf test case was included in the bats suite.

## Commit Reference

Commit `216f05e` message:
> All 5 bats parity tests pass (AC-005 cases a-e). All cargo tests pass (0 failures).
> Clippy clean (-D warnings). BC-7.03.091/092 satisfied.

**Result: PASS — 5/5 bats parity tests pass via production dispatcher path.**
