---
story_id: S-9.00
ac: AC-2
title: Per-plugin bundle-size data captured in JSON format
---

# AC-2: Per-plugin bundle-size data captured in JSON format

**Statement:** For each of the 17 enumerated plugins (13 Tier 1 W-15 + 4 pre-W-15 native WASM), record `wasm_bytes` in baseline data as a JSON array with `total_bytes` aggregate. Frozen enumeration must not be a directory glob.

## Command

```bash
bash .factory/measurements/measure-bundle-sizes.sh \
  plugins/vsdd-factory/hook-plugins/ 2>/dev/null | jq '.per_plugin'
```

## Output

```json
{
  "block-ai-attribution": 176647,
  "capture-commit-activity": 170580,
  "capture-pr-activity": 173713,
  "handoff-validator": 163030,
  "legacy-bash-adapter": 172860,
  "pr-manager-completion-guard": 1216002,
  "regression-gate": 227618,
  "session-end-telemetry": 205160,
  "session-learning": 115968,
  "session-start-telemetry": 216506,
  "tool-failure-hooks": 156345,
  "track-agent-start": 1205443,
  "track-agent-stop": 1204506,
  "update-wave-state-on-merge": 1455326,
  "validate-pr-review-posted": 1213652,
  "warn-pending-wave-gate": 319924,
  "worktree-hooks": 155866
}
```

## 17-Plugin Frozen Enumeration Coverage

| Plugin | wasm_bytes | Category |
|--------|-----------|----------|
| block-ai-attribution | 176,647 | Tier 1 W-15 |
| capture-commit-activity | 170,580 | Tier 1 W-15 |
| capture-pr-activity | 173,713 | Tier 1 W-15 |
| handoff-validator | 163,030 | Tier 1 W-15 |
| legacy-bash-adapter | 172,860 | Tier 1 W-15 |
| pr-manager-completion-guard | 1,216,002 | Tier 1 W-15 |
| regression-gate | 227,618 | Tier 1 W-15 |
| session-learning | 115,968 | Tier 1 W-15 |
| track-agent-start | 1,205,443 | Tier 1 W-15 |
| track-agent-stop | 1,204,506 | Tier 1 W-15 |
| update-wave-state-on-merge | 1,455,326 | Tier 1 W-15 |
| validate-pr-review-posted | 1,213,652 | Tier 1 W-15 (S-8.05) |
| warn-pending-wave-gate | 319,924 | Tier 1 W-15 |
| session-end-telemetry | 205,160 | pre-W-15 native WASM |
| session-start-telemetry | 216,506 | pre-W-15 native WASM |
| tool-failure-hooks | 156,345 | pre-W-15 native WASM |
| worktree-hooks | 155,866 | pre-W-15 native WASM |
| **total_bytes (17-plugin frozen sum)** | **8,549,146** | |

Note: `all_hook_plugins_wasm_bytes` = 8,549,146 equals the frozen-17 sum (pass-1 fix-burst corrected semantics). Non-frozen files (hello-hook.wasm and underscore-named stubs totalling 155,053 bytes) are captured in `unaccounted_wasm_bytes`.

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 2 S-9.00 AC-2: JSON per_plugin map contains all 17 frozen-enumeration plugin keys
```

## Verdict

PASS — All 17 frozen-enumeration plugins present with byte counts in JSON output. Enumeration is hard-coded (not a directory glob, per spec). Both 13 Tier 1 W-15 and 4 pre-W-15 plugins captured. Frozen-17 sum = 8,549,146 bytes. Bats test AC-2 passes.
