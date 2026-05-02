# E-8 Native WASM Migration — Warm-Invocation Perf Log

Tracks per-story warm-invocation latency for native WASM hook plugins.
Measurement method: `hyperfine --warmup 3 --runs 10` against release-built dispatcher.
Tier 1 results are advisory-only (E-8 AC-7 excludes Tier 1 from the hard 20% gate).

| Story | Hook Plugin | Median (ms) | Stddev (ms) | Gate |
|-------|-------------|-------------|-------------|------|
| S-8.02 | pr-manager-completion-guard | 348.0 | 22.2 | advisory |
