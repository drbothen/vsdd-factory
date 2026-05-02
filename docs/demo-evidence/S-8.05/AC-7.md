# AC-007: Bats parity tests — 14/14 pass

**BC traces:** BC-7.04.040 + BC-7.04.042/043/044 postcondition 1; BC-2.02.012 postconditions 5 and 6
**Status:** PASS

## Test suite

File: `tests/integration/E-8-hook-plugins/validate-pr-review-posted.bats`

Tests route through `factory-dispatcher` release build with `CLAUDE_PLUGIN_ROOT` pointing
at `plugins/vsdd-factory`. The dispatcher provides all WASM host functions including
`host::emit_event`. Bare `wasmtime` is NOT used — the dispatcher is the correct host.

Advisory block-mode: hook emits `hook.block` + writes stderr, then exits 0.
The dispatcher-internal log at `$WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl`
is grepped to assert `hook.block` event presence or absence.

## Test cases (14/14)

| # | Case | Description | Result |
|---|------|-------------|--------|
| 1 | AC-001 | hooks-registry.toml references native WASM, not legacy-bash-adapter | ok |
| 2 | AC-001 | hooks-registry.toml has no script_path | ok |
| 3 | AC-001 | hooks-registry.toml has no exec_subprocess block | ok |
| 4 | AC-002 | hooks.json does not contain validate-pr-review-posted entry | ok |
| 5 | AC-002 | validate-pr-review-posted.sh is deleted | ok |
| 6 | (a) | All checks pass → exit 0, no hook.block event | ok |
| 7 | (b) | pr-review.md not written → exit 0, check1 error in stderr | ok |
| 8 | (c) | gh pr comment used → exit 0, check2 error in stderr | ok |
| 9 | (d) | No formal review posted → exit 0, check3a error in stderr | ok |
| 10 | (e) | gh pr review with no verdict → exit 0, check3b error in stderr | ok |
| 11 | (f) | Multiple checks fail → exit 0, all accumulated errors in stderr | ok |
| 12 | (g) | Non-pr-reviewer agent → exit 0, no hook.block event | ok |
| 13 | (g.1) | agent_type=pr-reviewer (primary BC-2.02.012 chain arm) → checks applied | ok |
| 14 | (g.2) | subagent_name=pr-reviewer (fallback BC-2.02.012 chain arm) → checks applied | ok |

## Unit tests (14/14)

```
cargo test -p validate-pr-review-posted
test result: ok. 14 passed; 0 failed; 0 ignored
```

Tests cover: BC-7.04.041 non-pr-reviewer exit, BC-2.02.012 primary/fallback arms, pr-review-triage
matching, BC-7.04.042 check1 failure/pass, BC-7.04.043 check2 detection, BC-7.04.044 check3a/3b,
all-pass no output, multiple-failure accumulation, remediation block presence, advisory block-mode
(always HookResult::Continue).

## Recording

[AC-007-bats-parity-tests.gif](AC-007-bats-parity-tests.gif)
