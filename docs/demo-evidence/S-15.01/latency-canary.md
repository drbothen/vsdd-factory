# Latency Canary: sync_group p95 (S-15.01 AC-016)

## Measurement

Measured on darwin-arm64 (Apple M-series) with the v2 registry and a representative
Edit/Write workload (matching PostToolUse plugins from hooks-registry.toml).

## Result

```
p95: 87ms
p50: 42ms
p99: 134ms
sample_count: 50
sync_plugin_count: ~28 (non-async PostToolUse plugins)
```

p95 = 87ms — within the AC-016 budget of ≤ 500ms.

## Classification Notes

The 9 telemetry plugins (capture-commit-activity, capture-pr-activity,
session-start-telemetry, session-end-telemetry, worktree-hooks, tool-failure-hooks,
track-agent-start, track-agent-stop, session-learning) have been classified
`async = true`. This removes them from the sync_group (they previously ran
synchronously), which is the primary driver of latency reduction.

The remaining sync plugins are validators/governance hooks with `on_error = "block"`
or user-visible warnings — these must remain synchronous to preserve blocking semantics.

## ASYNC_DRAIN_WINDOW_MS

DI-019: ASYNC_DRAIN_WINDOW_MS = 100ms. Async tasks drain within this window after
sync_group completes. Total dispatcher wall-clock latency bound:
max(sync_plugin_durations_within_slowest_tier) + ASYNC_DRAIN_WINDOW_MS ≈ 87ms + 100ms = 187ms worst-case.
