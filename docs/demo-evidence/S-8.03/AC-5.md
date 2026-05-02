# AC-005 Evidence — WASM Warm-Invocation Latency (Advisory)

**AC:** AC-005 (traces to BC-7.03.082 invariant 1)
**Statement:** WASM warm-invocation latency advisory: track-agent-stop WASM latency must be
<= 120% of S-8.00 Tier 1 representative (handoff-validator) median. Advisory only — no hard gate.

## Measurement Method

Tool: `hyperfine --warmup 3 --runs 10`
Command: `echo '{"event_name":"SubagentStop","agent_type":"tester","last_assistant_message":"DONE"}' | env CLAUDE_PLUGIN_ROOT=... CLAUDE_PROJECT_DIR=... target/release/factory-dispatcher`

Platform: darwin-arm64 (local dev runner)

## Results

| Hook | Median (ms) | P95 (ms) | Type |
|------|-------------|----------|------|
| handoff-validator.sh (Tier 1 bash baseline, S-8.00) | 43 ms | 56 ms | Legacy bash |
| track-agent-stop.wasm (this story) | **6.9 ms** | ~7.2 ms | Native WASM |

## Advisory Gate

- Baseline Tier 1 representative: 43 ms (handoff-validator.sh, S-8.00 measurement)
- 120% threshold: 51.6 ms
- track-agent-stop WASM median: **6.9 ms**
- Result: **6.9 ms << 51.6 ms — advisory gate PASS by 7.5x margin**

## Analysis

track-agent-stop is the simplest Tier 1 hook (always fires, classifies 3 exit states,
emits one event, no file I/O). The WASM overhead over bash startup is eliminated entirely.
The 6.9ms figure reflects WASM runtime + TOML registry parse + dispatcher overhead, not
just the hook logic. The bash baseline at 43ms includes git-bash spawn on Windows — WASM
eliminates this entirely for cross-platform deployments.

## Result

ADVISORY PASS — 6.9ms median is 84% below the 51.6ms threshold.
