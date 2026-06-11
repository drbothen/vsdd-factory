# Red Gate Log — S-17.02: verify-factory-lock WASM Guard

**Story:** S-17.02 (verify-factory-lock WASM guard crate + registry entries)
**BC gate:** BC-4.13.001 v1.0
**Date:** 2026-06-11
**Agent:** test-writer

## Red Gate Result: VERIFIED

All 21 Cargo unit tests FAIL. All 9 bats integration tests SKIP.
Zero tests pass before implementation.

## Cargo Unit Tests (21 tests — all FAIL)

Run: `cargo test -p verify-factory-lock`

```
test result: FAILED. 0 passed; 21 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Failure mode

All tests panic with `todo!()` from the stub helper bodies:
- `guard_logic`: `"implement guard_logic decision tree per BC-4.13.001 T-3 specification"`
- `matches_factory_artifacts_push`: `"implement: check command contains 'git', 'push', 'factory-artifacts' in order"`
- `parse_factory_lock`: `"implement: scan frontmatter between --- delimiters for factory_lock block"`
- `parse_iso8601`: `"implement: parse ISO-8601 string via chrono; return MalformedLockBlock on error"`
- `format_time_remaining`: `"implement: compute expires_at - now, round down to minutes, return 'N min remaining'"`
- `build_block_message`: `"implement: format block message with all 5 required fields including /factory-unlock --force"`
- `trim_git_email`: `"implement: trim trailing newline and whitespace from git email output"`

### Full failing test list

| Test Name | BC Clause | Failure |
|-----------|-----------|---------|
| `test_BC_4_13_001_foreign_unexpired_lock_blocks_with_all_five_fields` | PC1 | todo!() in guard_logic |
| `test_BC_4_13_001_expired_lock_returns_continue` | PC2 | todo!() in guard_logic |
| `test_BC_4_13_001_self_held_lock_returns_continue` | PC3 | todo!() in guard_logic |
| `test_BC_4_13_001_malformed_block_returns_continue_with_log_warn` | PC4 | todo!() in guard_logic |
| `test_BC_4_13_001_read_file_host_error_returns_continue` | PC6 | todo!() in guard_logic |
| `test_BC_4_13_001_git_subprocess_failure_returns_continue` | PC7 | todo!() in guard_logic |
| `test_BC_4_13_001_capability_denied_graceful_degrades_to_continue` | Invariant 6 | todo!() in guard_logic |
| `test_BC_4_13_001_bash_factory_artifacts_push_blocked_when_foreign_lock` | T-6 (D9) | todo!() in guard_logic |
| `test_BC_4_13_001_non_push_bash_returns_continue_immediately` | T-7 (D9) / EC-011 | todo!() in guard_logic |
| `test_BC_4_13_001_expires_at_exact_boundary_treated_as_expired` | EC-002 | todo!() in parse_iso8601 |
| `test_BC_4_13_001_push_regex_matches_factory_artifacts_push` | AC-013 / EC-011 | todo!() in matches_factory_artifacts_push |
| `test_BC_4_13_001_push_regex_does_not_match_non_push_command` | AC-013 / EC-011 | todo!() in matches_factory_artifacts_push |
| `test_BC_4_13_001_parse_factory_lock_returns_some_on_valid_block` | PC1/PC4 | todo!() in parse_factory_lock |
| `test_BC_4_13_001_parse_factory_lock_returns_none_on_absent_block` | EC-001 | todo!() in parse_factory_lock |
| `test_BC_4_13_001_parse_factory_lock_errors_on_empty_holder` | EC-004 | todo!() in parse_factory_lock |
| `test_BC_4_13_001_parse_iso8601_succeeds_on_valid_timestamp` | PC1/PC2 | todo!() in parse_iso8601 |
| `test_BC_4_13_001_parse_iso8601_errors_on_invalid_timestamp` | EC-005 | todo!() in parse_iso8601 |
| `test_BC_4_13_001_format_time_remaining_returns_n_min_remaining` | PC1 (field 4) | todo!() in format_time_remaining |
| `test_BC_4_13_001_trim_git_email_strips_trailing_newline` | PC3/PC7 | todo!() in trim_git_email |
| `test_BC_4_13_001_trim_git_email_unchanged_when_no_newline` | PC3/PC7 | todo!() in trim_git_email |
| `test_BC_4_13_001_build_block_message_contains_all_five_fields` | PC1 | todo!() in build_block_message |

## Bats Integration Tests (9 tests — all SKIP)

Run: `bats plugins/vsdd-factory/tests/verify-factory-lock/verify-factory-lock.bats`

```
1..9
ok 1 T-1 ... # skip factory-dispatcher binary not built
ok 2 T-2 ... # skip factory-dispatcher binary not built
... (all 9 skip)
```

### Skip reason

The bats tests require two artifacts produced by the implementer:
1. `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm` (Task T-6)
2. `plugins/vsdd-factory/hooks-registry.toml` entries (Task T-5)

Neither exists yet. Skip (not error) is the correct Red Gate state for
integration tests pending WASM compilation. After implementation, running
`bats` against the compiled plugin will verify T-1..T-9.

## Files Written

| File | Description |
|------|-------------|
| `crates/hook-plugins/verify-factory-lock/src/lib.rs` | `#[cfg(test)] mod tests` with 21 unit tests added to the stub |
| `crates/hook-plugins/verify-factory-lock/Cargo.toml` | Added `serde_json = { workspace = true }` to `[dev-dependencies]` |
| `plugins/vsdd-factory/tests/verify-factory-lock/verify-factory-lock.bats` | 9 bats integration tests (T-1..T-9) |

## BC Coverage

| BC Clause | Test(s) Covering It |
|-----------|---------------------|
| PC1 (ForeignLockHeld block + 5 fields) | `test_BC_4_13_001_foreign_unexpired_lock_blocks_with_all_five_fields`, `test_BC_4_13_001_bash_factory_artifacts_push_blocked_when_foreign_lock`, `test_BC_4_13_001_build_block_message_contains_all_five_fields` |
| PC2 (LockExpired pass) | `test_BC_4_13_001_expired_lock_returns_continue` |
| PC3 (self-held pass) | `test_BC_4_13_001_self_held_lock_returns_continue` |
| PC4 (malformed fail-open + log_warn) | `test_BC_4_13_001_malformed_block_returns_continue_with_log_warn`, `test_BC_4_13_001_parse_factory_lock_errors_on_empty_holder` |
| PC6 (read failure fail-open) | `test_BC_4_13_001_read_file_host_error_returns_continue` |
| PC7 (identity resolution fail-open) | `test_BC_4_13_001_git_subprocess_failure_returns_continue` |
| Invariant 6 (CapabilityDenied degrade) | `test_BC_4_13_001_capability_denied_graceful_degrades_to_continue` |
| EC-001 (absent block) | `test_BC_4_13_001_parse_factory_lock_returns_none_on_absent_block` |
| EC-002 (boundary = expired) | `test_BC_4_13_001_expires_at_exact_boundary_treated_as_expired` |
| EC-004 (empty holder malformed) | `test_BC_4_13_001_parse_factory_lock_errors_on_empty_holder` |
| EC-005 (invalid ISO-8601) | `test_BC_4_13_001_parse_iso8601_errors_on_invalid_timestamp` |
| EC-011 (non-push Bash no-op) | `test_BC_4_13_001_non_push_bash_returns_continue_immediately` |
| T-6 D9 (Bash push arm) | `test_BC_4_13_001_bash_factory_artifacts_push_blocked_when_foreign_lock` |
| T-7 D9 (non-push Bash) | `test_BC_4_13_001_non_push_bash_returns_continue_immediately` |

## Instructions for Implementer

Make each test pass, one at a time, with minimum code. Suggested order:

1. `trim_git_email` (trivial string operation)
2. `matches_factory_artifacts_push` (substring check for three fragments in order)
3. `parse_iso8601` (chrono parse)
4. `parse_factory_lock` (line-by-line frontmatter scan — Architecture Compliance Rule 4)
5. `format_time_remaining` (duration math + format)
6. `build_block_message` (string format with 5 fields)
7. `guard_logic` (decision tree — all helpers must pass first)

Do NOT weaken any test assertion. Do NOT skip any test. Add `serde_json` to
`[dependencies]` (not just `[dev-dependencies]`) if the WASM entry point needs it.
