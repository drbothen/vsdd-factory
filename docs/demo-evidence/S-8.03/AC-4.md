# AC-004 Evidence — Bats Parity Tests (17/17)

**AC:** AC-004 (traces to BC-7.03.082 postcondition 1; BC-2.02.012 postconditions 5 and 6)
**Statement:** Bats parity tests cover: (a) empty result; (b) Status: BLOCKED; (c) ok;
(d) missing `last_assistant_message` with `result` fallback; (e) both absent; (f)
`agent_type` absent with `subagent_name` fallback. Plus AC-007 (malformed JSON), EC-006
(multibyte UTF-8), EC-007 (multiline BLOCKED), parity audit, EC-004b. All 17 pass.

## Recording

- `AC-004-bats-parity-tests.gif` / `.webm` / `.tape`

## Test Suite Output

```
1..17
ok 1  AC-001: hooks-registry.toml entry references native WASM
ok 2  AC-001: no script_path
ok 3  AC-001: no exec_subprocess block
ok 4  AC-001: no [hooks.capabilities] section
ok 5  AC-001 invariant: WASM artifact exists at wasm32-wasip1 target
ok 6  AC-002: track-agent-stop.sh is deleted
ok 7  AC-004(a): empty last_assistant_message => exit_class=empty result_len=0 exit 0
ok 8  AC-004(b): Status: BLOCKED => exit_class=blocked exit 0
ok 9  AC-004(c): non-empty non-BLOCKED => exit_class=ok result_len=4 exit 0
ok 10 AC-004(d): last_assistant_message absent, result present => fallback to result
ok 11 AC-004(e): both absent => exit_class=empty result_len=0 exit 0
ok 12 AC-004(f): agent_type absent, subagent_name present => subagent=subagent_name
ok 13 AC-007: malformed JSON stdin => exit 0, no panic
ok 14 EC-006: U+1F600 emoji => result_len=4 (byte count parity with wc -c)
ok 15 EC-007: BLOCKED on non-first line => exit_class=blocked (multiline regex)
ok 16 parity audit: agent.stop contains hook=track-agent-stop, matcher=SubagentStop
ok 17 EC-004b: both agent_type and subagent_name absent => subagent=unknown
```

## Coverage

| Test Case | AC/EC | Status |
|-----------|-------|--------|
| empty last_assistant_message | AC-004(a) | PASS |
| Status: BLOCKED | AC-004(b) | PASS |
| non-empty non-BLOCKED | AC-004(c) | PASS |
| last_assistant_message absent, result fallback | AC-004(d) / EC-003a | PASS |
| both message fields absent | AC-004(e) / EC-003b | PASS |
| agent_type absent, subagent_name fallback | AC-004(f) / EC-004 | PASS |
| malformed JSON | AC-007 | PASS |
| multibyte UTF-8 emoji byte count | EC-006 | PASS |
| BLOCKED on second line (multiline regex) | EC-007 | PASS |
| full parity field set | AC-003 | PASS |
| both identity fields absent | EC-004b | PASS |

## Result

PASS — 17/17 bats tests pass.
