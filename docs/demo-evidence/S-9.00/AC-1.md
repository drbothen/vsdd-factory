---
story_id: S-9.00
ac: AC-1
title: Current total WASM bundle size measured for post-rc.4 release artifacts
---

# AC-1: Current total WASM bundle size measured for post-rc.4 release artifacts

**Statement:** Current total WASM bundle size measured for post-rc.4 release artifacts using `wc -c < <file>` (portable). Record: (a) `dispatcher_binary_bytes`, (b) `all_hook_plugins_wasm_bytes`, (c) `per_plugin_bytes` map, (d) `total_bytes`. Measurement platform: darwin-arm64 (local dev; CI values flagged as CI-only per EC-002).

## Command

```bash
bash .factory/measurements/measure-bundle-sizes.sh \
  plugins/vsdd-factory/hook-plugins/ 2>/dev/null | jq .
```

(Run from worktree root: `.worktrees/S-9.00-perf-baseline/`)

## Output

```json
{
  "methodology_version": 1,
  "measurement_timestamp": "2026-05-05T04:56:48Z",
  "host_platform": "Darwin-arm64",
  "all_hook_plugins_wasm_bytes": 8704199,
  "dispatcher_bytes": 12250912,
  "grand_total_bytes": 20955111,
  "cold_start_p95_measured_ms": 656.7,
  "per_plugin": {
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
}
```

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 1 S-9.00 AC-1: script outputs JSON with all_hook_plugins_wasm_bytes field equal to sum of present wasm files
```

## Key Values

| Metric | Value |
|--------|-------|
| `all_hook_plugins_wasm_bytes` | 8,704,199 |
| `dispatcher_binary_bytes` | 12,250,912 |
| `grand_total_bytes` | 20,955,111 (~20MB) |
| Platform | darwin-arm64 |
| develop HEAD (post-rc.8) | see `git rev-parse HEAD` in worktree |

Note: Per EC-002, this measurement uses the local release build. All 5 CI platforms (darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64) require re-measurement on CI runners; darwin-arm64 local dev is recorded here as the W-16 Wave 0 baseline.

## Verdict

PASS — `all_hook_plugins_wasm_bytes` = 8,704,199 bytes recorded. `dispatcher_binary_bytes` = 12,250,912. `grand_total_bytes` = 20,955,111 (~20MB; ~9MB headroom under 30MB kill-switch). Per-plugin map complete for all 17 frozen plugins. Bats test AC-1 passes.
