## tests/perf/

This directory contains the performance baseline timing harness for E-8 (Native WASM Migration). It measures warm-invocation wall-clock latency for representative bash hooks using `hyperfine --warmup 3 --runs 10`, establishing the pre-migration baseline against which WASM port stories (S-8.01..S-8.09) verify their E-8 AC-7b compliance (p95 latency <= 200ms for 23 Tier 2 plugins in aggregate).

**To run:** `bats tests/perf/` (requires bats-core >= 1.10 and hyperfine >= 1.18).

**Results are persisted to:** `.factory/measurements/E-8-bash-baseline.json`
