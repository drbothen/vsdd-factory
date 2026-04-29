---
story: S-5.04
wave: 16
phase: red-gate
timestamp: 2026-04-28T00:00:00Z
agent: test-writer
status: RED_GATE_VERIFIED
---

# Red Gate Log — S-5.04: PostToolUseFailure hook wiring

## Summary

9 integration tests written covering all 3 behavioral contracts (BC-4.08.001–003) per VP-068.
6 tests FAIL (RED gate verified). 3 tests pass (AC4/AC5 file-system tests — template entry,
platform variants, and registry entry are added as part of the RED gate scaffold).

## Test Files Created

| File | Action | Test count |
|------|--------|-----------|
| `crates/hook-plugins/tool-failure-hooks/Cargo.toml` | created | crate manifest |
| `crates/hook-plugins/tool-failure-hooks/src/lib.rs` | created | plugin skeleton (unimplemented! stub) |
| `crates/hook-plugins/tool-failure-hooks/src/main.rs` | created | WASI entry point |
| `crates/hook-plugins/tool-failure-hooks/tests/integration_test.rs` | created | 9 integration tests |
| `Cargo.toml` (workspace root) | modified | added tool-failure-hooks to members (alphabetical, before worktree-hooks) |
| `plugins/vsdd-factory/hooks/hooks.json.template` | modified | added PostToolUseFailure entry (once ABSENT; async:true; timeout:10000) |
| `plugins/vsdd-factory/hooks/hooks.json.darwin-arm64` | regenerated | via scripts/generate-hooks-json.sh |
| `plugins/vsdd-factory/hooks/hooks.json.darwin-x64` | regenerated | via scripts/generate-hooks-json.sh |
| `plugins/vsdd-factory/hooks/hooks.json.linux-arm64` | regenerated | via scripts/generate-hooks-json.sh |
| `plugins/vsdd-factory/hooks/hooks.json.linux-x64` | regenerated | via scripts/generate-hooks-json.sh |
| `plugins/vsdd-factory/hooks/hooks.json.windows-x64` | regenerated | via scripts/generate-hooks-json.sh |
| `plugins/vsdd-factory/hooks-registry.toml` | modified | added PostToolUseFailure [[hooks]] entry |

## Test Count by BC

| BC | Test | Status |
|----|------|--------|
| BC-4.08.001 (happy path, 10-field wire) | `test_bc_4_08_001_tool_error_emitted_with_required_fields` | FAIL (unimplemented!) |
| BC-4.08.001 EC-002 (missing tool_name → "unknown") | `test_bc_4_08_001_missing_tool_name_emits_unknown_sentinel` | FAIL (unimplemented!) |
| BC-4.08.001 EC-001 (error_message truncated at 1000) | `test_bc_4_08_001_error_message_truncated_at_1000_chars` | FAIL (unimplemented!) |
| BC-4.08.001 EC-001 boundary (exactly 1000, no truncation) | `test_bc_4_08_001_error_message_exactly_1000_chars_no_truncation` | FAIL (unimplemented!) |
| BC-4.08.001 EC-003 (missing error_message → "") | `test_bc_4_08_001_missing_error_message_emits_empty_string` | FAIL (unimplemented!) |
| BC-4.08.001 Invariants 1-2 (no subprocess, no read_file) | `test_bc_4_08_001_no_subprocess_no_read_file_invoked` | FAIL (unimplemented!) |
| BC-4.08.002 (hooks.json.template entry) | `test_bc_4_08_002_hooks_json_template_post_tool_use_failure_entry` | PASS (entry added in scaffold) |
| BC-4.08.002 Invariant 5 (platform variants in sync) | `test_bc_4_08_002b_platform_variants_in_sync` | PASS (variants regenerated in scaffold) |
| BC-4.08.003 (hooks-registry.toml entry) | `test_bc_4_08_003_hooks_registry_toml_post_tool_use_failure_entry` | PASS (entry added in scaffold) |

## Build Result

```
cargo build -p tool-failure-hooks  →  SUCCESS (0 errors, 2 unused-variable warnings from unimplemented! stub)
```

## Test Failure Summary

6 of 9 tests FAIL (all plugin logic tests panic at unimplemented! stub).

All 6 tests panic at `src/lib.rs:55` with:
```
not implemented: S-5.04 GREEN
```

## Pre-Existing Tests: PASS

Tests 7-9 pass because the scaffold adds the template + registry entries + regenerates platform
variants as part of the RED gate commit. Per S-5.02 pattern, AC4/AC5 file-system tests passing
at RED time is correct and does not weaken the RED gate.

## RED Gate Verdict

RED gate VERIFIED: 6/9 tests fail. All failures are deterministic `unimplemented!()` panics.
The crate compiles cleanly. The RED gate is valid.

## Handoff to Implementer

Implement `tool_failure_hook_logic` in `src/lib.rs`:

1. Extract `tool_name` from `payload.tool_input["tool_name"]`; if absent or empty → "unknown" (EC-002)
2. Extract `error_message` from `payload.tool_input["error_message"]`; if absent → ""; if > 1000 chars → truncate to 1000 (EC-001)
3. Call `emit("tool.error", &[("tool_name", ...), ("error_message", ...)])` exactly once
4. Return `HookResult::Continue`
5. MUST NOT set any RESERVED_FIELDS
