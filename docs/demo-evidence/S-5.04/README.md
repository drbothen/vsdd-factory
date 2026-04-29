# S-5.04: PostToolUseFailure Hook Wiring — Demo Evidence

**Story:** S-5.04 | **Branch:** feat/s-5.04-tool-failure-hooks | **GREEN commit:** `81e9fc4`
**VP:** VP-068 (9/9 tests pass) | **BCs:** BC-4.08.001, BC-4.08.002, BC-4.08.003

## Coverage Summary

| File | AC | BCs | VP-068 Tests |
|------|----|-----|-------------|
| [AC1-routing-path.md](AC1-routing-path.md) | AC1 | BC-4.08.002, BC-4.08.003 | tests 7, 8, 9 |
| [AC2-tool-error-wire-payload.md](AC2-tool-error-wire-payload.md) | AC2 | BC-4.08.001 PC-1/PC-2 | test 1 |
| [AC3-edge-cases.md](AC3-edge-cases.md) | AC3 | BC-4.08.001 EC-001/EC-002/EC-003/EC-004, BC-4.08.003 PC-4/PC-5 | tests 2, 3, 4, 5, 6 |
| [AC4-hooks-json-template.md](AC4-hooks-json-template.md) | AC4 | BC-4.08.002 PC-1–PC-6, Invariant 1, Invariant 5 | tests 7, 8 |
| [AC5-hooks-registry-toml.md](AC5-hooks-registry-toml.md) | AC5 | BC-4.08.003 PC-1–PC-7 | test 9 |
| [AC6-vp068-integration-test.md](AC6-vp068-integration-test.md) | AC6 | All BC-4.08.* | all 9 tests |

## Key Design Decisions

- **Zero-capability plugin (Option A):** No `read_file`, no `exec_subprocess` — all data from envelope.
- **`once` key ABSENT:** PostToolUseFailure fires per-failure; defensive omission mirrors S-5.03 worktree pattern.
- **10-field wire payload:** 2 plugin-set + 4 host-enriched + 4 construction-time. Plugin sets only `tool_name` and `error_message`.
- **session_id RESERVED:** Host-enriched per BC-1.05.012; plugin MUST NOT set it.
- **Platform variants regenerated:** All 5 `hooks.json.*` files updated via `scripts/generate-hooks-json.sh`.

## Lessons Applied from S-5.01/S-5.02/S-5.03

| Lesson | Source | Applied |
|--------|--------|---------|
| 4+4 opaque RESERVED_FIELDS grouping | S-5.02 pass-2 reversal | Yes — from pass-1 of S-5.04 |
| truncation limit 1000 chars (not 2000) | S-5.04 ADV pass-1 CRIT-003 | Yes |
| `once` ABSENT (not `once:false`) | S-5.03 pass-7 | Yes |
| Platform variant regeneration mandatory | S-5.03 PR-cycle-1 | Yes — Task 3b |
| `tool.error` (not `tool.failed`) | S-5.03 pass-7 PRD propagation | Yes |
