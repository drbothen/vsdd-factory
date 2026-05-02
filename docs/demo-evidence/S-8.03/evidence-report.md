# Evidence Report — S-8.03: Native Port: track-agent-stop (SubagentStop)

**Story:** S-8.03 — Native port: track-agent-stop (SubagentStop)
**Branch:** feature/S-8.03-native-port-track-agent-stop
**HEAD:** 8db9723
**Date:** 2026-05-02
**BCs:** BC-7.03.081, BC-7.03.082, BC-2.02.012

## Coverage Map

| AC | Statement | Evidence File | Recording | Result |
|----|-----------|---------------|-----------|--------|
| AC-001 | WASM crate built (wasm32-wasip1); registry updated; event/priority/on_error preserved; [hooks.capabilities] fully removed | AC-1.md | AC-001-wasm-crate-registry-migration.gif/.webm | PASS |
| AC-002 | hooks.json entry absent (positive grep); .sh file absent from repo | AC-2.md | AC-002-bash-deletion.gif/.webm | PASS |
| AC-003 | BC-2.02.012 fallback chains (PC-5/PC-6); byte-count RESULT_LEN; multiline BLOCKED regex; emit agent.stop with 5 fields; exit 0 always | AC-3.md | AC-003-exit-classification.gif/.webm | PASS |
| AC-004 | Bats parity tests 17/17: empty/blocked/ok/missing-field/both-absent/agent-type-fallback/malformed-JSON/multibyte/multiline-BLOCKED | AC-4.md | AC-004-bats-parity-tests.gif/.webm | PASS (17/17) |
| AC-005 | WASM warm-invocation latency advisory <= 120% of S-8.00 Tier 1 representative (handoff-validator) median | AC-5.md | (measurement-only, no VHS) | ADVISORY PASS (6.9ms vs 51.6ms threshold) |
| AC-006 | host::emit_event replaces bin/emit-event; bin/emit-event not removed | AC-6.md | AC-006-host-emit-event.gif/.webm | PASS |
| AC-007 | Malformed/missing JSON stdin -> graceful exit 0; no panic | AC-7.md | AC-007-malformed-json-graceful-exit.gif/.webm | PASS |

## Test Suite Summary

### Bats Integration Tests (17/17)

```
ok 1  AC-001: hooks-registry.toml entry references native WASM
ok 2  AC-001: no script_path
ok 3  AC-001: no exec_subprocess block
ok 4  AC-001: no [hooks.capabilities] section
ok 5  AC-001 invariant: WASM artifact exists at wasm32-wasip1 target
ok 6  AC-002: track-agent-stop.sh is deleted
ok 7  AC-004(a): empty last_assistant_message => exit_class=empty result_len=0
ok 8  AC-004(b): Status: BLOCKED => exit_class=blocked
ok 9  AC-004(c): non-empty non-BLOCKED => exit_class=ok result_len=4
ok 10 AC-004(d): last_assistant_message absent, result present => fallback to result
ok 11 AC-004(e): both absent => exit_class=empty result_len=0
ok 12 AC-004(f): agent_type absent, subagent_name present => subagent=subagent_name
ok 13 AC-007: malformed JSON stdin => exit 0, no panic
ok 14 EC-006: U+1F600 emoji => result_len=4 (byte count, not char count)
ok 15 EC-007: BLOCKED on non-first line => exit_class=blocked (multiline regex)
ok 16 parity audit: hook=track-agent-stop, matcher=SubagentStop, correct fields
ok 17 EC-004b: both agent_type and subagent_name absent => subagent=unknown
```

### Unit Tests (23/23)

```
test result: ok. 23 passed; 0 failed; 0 ignored
```

Tests cover: classify_exit (empty, whitespace-only, blocked variants, ok, mid-text BLOCKED,
multiline BLOCKED, emoji byte count, whitespace exclusion), track_agent_stop_logic (happy
path, exit_class empty/blocked, agent identity PC-5 chain, message content PC-6 chain,
exit-0 invariant on all paths, field count/names, exactly-one-event emission).

## BC Trace Summary

| BC ID | Postcondition/Invariant | Coverage |
|-------|------------------------|----------|
| BC-7.03.081 | Postcondition 1 (identity & registry binding) | AC-001 bats tests 1-5; AC-1.md |
| BC-7.03.081 | Invariant 1 (exit-code semantics / lifecycle) | AC-002 bats test 6; AC-007 bats test 13; AC-2.md, AC-7.md |
| BC-7.03.082 | Postcondition 1 (classify exit_class, emit agent.stop) | AC-003 unit tests; AC-004 bats tests 7-12, 14-17; AC-3.md, AC-4.md |
| BC-7.03.082 | Invariant 1 (warm-invocation latency advisory) | AC-005; AC-5.md |
| BC-2.02.012 | Postcondition 5 (agent identity fallback chain) | bats tests 12, 17; unit tests BC_2_02_012_*; AC-3.md |
| BC-2.02.012 | Postcondition 6 (assistant-message fallback chain) | bats tests 10, 11; unit tests BC_2_02_012_*; AC-3.md |

## Exit Classification Evidence

| Input | RESULT_LEN | EXIT_CLASS | Evidence |
|-------|-----------|------------|---------|
| `""` (empty string) | 0 | empty | bats test 7; unit test classify_exit_empty_string |
| `"   \t\n"` (whitespace only) | 0 | empty | unit test classify_exit_whitespace_only |
| `"Status: BLOCKED — missing context"` | >0 | blocked | bats test 8; unit test classify_exit_blocked_status_prefix |
| `"BLOCKED"` (bare at line start) | 7 | blocked | unit test classify_exit_blocked_bare |
| `"## BLOCKED"` | >0 | blocked | unit test classify_exit_blocked_h2_prefix |
| `"result is BLOCKED by policy"` (mid-text) | >0 | ok | unit test classify_exit_blocked_mid_text_is_ok (EC-002) |
| `"first line\nBLOCKED\nmore"` (second line) | >0 | blocked | bats test 15; unit test multiline_second_line (EC-007) |
| `"DONE"` | 4 | ok | bats test 9; unit test classify_exit_ok |
| U+1F600 emoji (4 bytes) | 4 | ok | bats test 14; unit test result_len_byte_count (EC-006) |

## BC-2.02.012 Fallback Chain Evidence

### Postcondition 5 — Agent Identity

| agent_type | subagent_name | Resolved subagent | Evidence |
|-----------|---------------|-------------------|---------|
| "pr-reviewer" | (absent) | "pr-reviewer" | bats test 9; unit test agent_type_used_when_present |
| (absent) | "story-writer" | "story-writer" | bats test 12 (EC-004) |
| (absent) | (absent) | "unknown" | bats test 17; unit test subagent_unknown_when_both_absent (EC-004b) |

### Postcondition 6 — Assistant-Message Content

| last_assistant_message | result | Resolved content | Evidence |
|----------------------|--------|-----------------|---------|
| "DONE primary" | "result field" | "DONE primary" (PC-6: lam wins) | unit test lam_used_when_present |
| (absent) | "DONE via result" | "DONE via result" (PC-6: fallback) | bats test 10 (EC-003a) |
| (absent) | (absent) | "" -> RESULT_LEN=0, EXIT_CLASS=empty | bats test 11 (EC-003b) |

## Performance Advisory (AC-005)

| Metric | Value |
|--------|-------|
| S-8.00 Tier 1 baseline (handoff-validator.sh median) | 43 ms |
| Advisory threshold (120% of baseline) | 51.6 ms |
| track-agent-stop.wasm measured median | **6.9 ms** |
| Margin under threshold | 84% faster |
| Advisory gate | PASS |

Measurement: `hyperfine --warmup 3 --runs 10`, darwin-arm64, release build.

## File Inventory

### VHS Recordings (6 ACs)

| File | Size | AC |
|------|------|----|
| AC-001-wasm-crate-registry-migration.gif | 372K | AC-001 |
| AC-001-wasm-crate-registry-migration.webm | 288K | AC-001 |
| AC-001-wasm-crate-registry-migration.tape | 796B | AC-001 |
| AC-002-bash-deletion.gif | 134K | AC-002 |
| AC-002-bash-deletion.webm | 125K | AC-002 |
| AC-002-bash-deletion.tape | 759B | AC-002 |
| AC-003-exit-classification.gif | 236K | AC-003 |
| AC-003-exit-classification.webm | 247K | AC-003 |
| AC-003-exit-classification.tape | 790B | AC-003 |
| AC-004-bats-parity-tests.gif | 234K | AC-004 |
| AC-004-bats-parity-tests.webm | 298K | AC-004 |
| AC-004-bats-parity-tests.tape | 590B | AC-004 |
| AC-006-host-emit-event.gif | 157K | AC-006 |
| AC-006-host-emit-event.webm | 154K | AC-006 |
| AC-006-host-emit-event.tape | 826B | AC-006 |
| AC-007-malformed-json-graceful-exit.gif | 107K | AC-007 |
| AC-007-malformed-json-graceful-exit.webm | 108K | AC-007 |
| AC-007-malformed-json-graceful-exit.tape | 857B | AC-007 |

### Per-AC Evidence Markdown

| File | AC |
|------|----|
| AC-1.md | AC-001 |
| AC-2.md | AC-002 |
| AC-3.md | AC-003 |
| AC-4.md | AC-004 |
| AC-5.md | AC-005 (advisory measurement) |
| AC-6.md | AC-006 |
| AC-7.md | AC-007 |

## Summary

7 acceptance criteria recorded. 6 VHS recordings (GIF + WebM + tape each) covering AC-001
through AC-004, AC-006, AC-007. AC-005 (performance advisory) covered by measurement-only
evidence in AC-5.md. 17/17 bats integration tests pass. 23/23 unit tests pass.

All acceptance criteria have explicit recording or measurement evidence linked to a specific
AC identifier. BC-7.03.081, BC-7.03.082, and BC-2.02.012 postconditions/invariants are
fully covered by test and recording evidence.
