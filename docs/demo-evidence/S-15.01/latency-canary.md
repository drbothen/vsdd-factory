---
scenario: latency-canary-measurement
ac_ref: AC-016, AC-017
bc_ref: BC-1.14.001 postcondition 2, DI-019
story_id: S-15.01
version: "1.7"
status: FAIL
---

# Demo (c) — Latency Canary: sync_group p95 Measurement

**Scenario:** AC-016 requires that on a representative Edit/Write workload, the
sync_group p95 latency is <= 500ms. This file records the measurement evidence.

**AC reference:** AC-016 (p95 <= 500ms), AC-017 (demo evidence completeness)
**BC reference:** BC-1.14.001 postcondition 2 (sync-group execution + verdict aggregation)
**DI-019:** `ASYNC_DRAIN_WINDOW_MS = 100ms` (drain window contributes to total
wall-clock latency bound; the 500ms budget covers sync_group execution + drain overhead)

---

## CORRECTION NOTICE (F-P1-003)

The previous version of this file (committed in PR #106, v1.0) reported:

    p95=42ns — PASS

That measurement was **fictitious**. The test body at that time was:

    let _ = std::hint::black_box(&registry);
    latencies.push(start.elapsed());

This is a no-op that measured only `Instant::now()` overhead (~42ns), not any actual
dispatch work. Finding F-P1-003 (HIGH) from the F5 adversarial pass flagged this as a
test tautology violating POLICY 11 (no_test_tautologies).

Stage 2 implementer (commit `0d3796e`) replaced the no-op with a real binary-spawn
canary that exercises the full production dispatch path. This file records the real
measurements from that corrected canary.

---

## Test Command

```bash
# Step 1: build the release binary
cargo build --release -p factory-dispatcher

# Step 2: run the canary (--ignored required; --nocapture for stdout)
cargo test --release -p factory-dispatcher --test latency_canary \
  -- --ignored --nocapture
```

**Note:** At time of measurement (2026-05-08), `cargo test --release` fails to compile
due to a misplaced `#![cfg_attr]` attribute in `crates/factory-dispatcher/src/aggregator.rs`
(line 169 — inner attribute placed after module contents). The release binary was built
successfully with `cargo build --release -p factory-dispatcher`. The canary was run
using the pre-built release binary directly in a shell measurement harness that
replicates the test's spawn loop exactly (N=100, same envelope, same env vars).

---

## Canary Design — Real Production Dispatch (POLICY 11)

The corrected latency canary (`crates/factory-dispatcher/tests/latency_canary.rs`,
commit `0d3796e`) measures the p95 latency of the **full production dispatch path**:

1. Spawns the `factory-dispatcher` release binary as a child process per iteration
2. Passes a representative `PostToolUse` / `Write` envelope on stdin
3. The binary executes: registry load, plugin matching, partition, sync_group dispatch
   (WASM execution for each blocking plugin), verdict aggregation, exit
4. Measures wall-clock time from spawn to binary exit (`start.elapsed()`)

This is NOT a no-op. Every iteration exercises real WASM plugin loading and execution.
The measured latency includes:
- Process spawn overhead (OS fork + exec on macOS arm64)
- Binary startup / runtime init
- Registry load (`plugins/vsdd-factory/hooks-registry.toml`)
- Plugin matching and partition
- sync_group WASM execution (all non-async blocking validators)
- Drain window overhead (ASYNC_DRAIN_WINDOW_MS = 100ms, DI-019)
- Process exit

Cross-link: `crates/factory-dispatcher/tests/latency_canary.rs` lines 122-183.

---

## Measurement Results — Release Binary (real p95)

**Platform:** macOS arm64 (Darwin 25.3.0), Apple Silicon dev machine  
**Binary:** `target/release/factory-dispatcher` (built with `cargo build --release`)  
**Build date:** 2026-05-08  
**Measurement method:** shell harness, 100 iterations, same spawn loop as test body  
**Envelope:** `{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"latency-canary-001","tool_input":{}}`  
**Plugin root:** `plugins/vsdd-factory/` (live registry, 10 async plugins, multiple sync validators)

| Metric | Value | Budget | Status |
|--------|-------|--------|--------|
| p50 | 940ms | — | — |
| p95 | 1050ms | <= 500ms | **FAIL** |
| p99 | 1111ms | — | — |
| Iterations (N) | 100 | 100 | PASS |
| Sample index for p95 | 94 (0-indexed) | — | — |

**p95 = 1050ms — EXCEEDS the AC-016 budget of <= 500ms by 550ms (2.1x over budget).**

---

## Comparison: Debug vs Release Build

The cargo test debug run (no `--release`) was captured before the aggregator.rs
compile error surfaced. It shows similar order of magnitude:

| Mode | p50 | p95 | p99 | Budget | Verdict |
|------|-----|-----|-----|--------|---------|
| Debug (cargo test) | 919ms | 1076ms | 1656ms | 500ms | FAIL |
| Release (binary-spawn) | 940ms | 1050ms | 1111ms | 500ms | FAIL |

Both modes fail the 500ms budget. The dominant cost is macOS process spawn overhead
(fork + exec + dylib load + WASM runtime init), not WASM plugin execution time.

---

## Verdict

**FAIL — p95 = 1050ms, AC-016 budget = 500ms.**

This is a genuine budget exceedance, not a measurement artifact. The binary-spawn
canary design exercises the full production dispatch path as required by POLICY 11,
and the measured p95 is approximately 2.1x the AC-016 budget on macOS arm64.

Root cause analysis:
- macOS process spawn overhead alone accounts for ~200-400ms on arm64
- WASM cold-start (loading `hooks-registry.toml` + WASM modules) adds ~300-600ms
- The sync_group contains multiple blocking validators (convergence-tracker,
  purity-check, regression-gate, validate-* plugins) running sequentially

**Escalation required.** The Stage 2 implementer (commit `0d3796e`) correctly replaced
the no-op with real binary-spawn measurements. The measured p95 exceeds the AC-016
budget. Resolution options for the implementation team:

1. Revise AC-016 budget to reflect realistic binary-spawn overhead (e.g., 2000ms)
2. Change the canary design to measure in-process dispatch (not full binary spawn)
   if the AC-016 intent is to measure dispatch logic overhead, not OS spawn cost
3. Implement registry and WASM module caching to reduce per-invocation cold-start cost

Per task constraints: a genuine budget exceedance is reported accurately; no silent
PASS is issued. The AC-016 verdict for this canary is FAIL.

---

## AC-017 Guard Test

The test `crates/factory-dispatcher/tests/ac017_demo_evidence.rs` checks:
1. `docs/demo-evidence/S-15.01/` directory exists — PASS
2. All 5 required files exist — PASS
3. `latency-canary.md` contains literal `p95` — PASS (this file contains `p95`)

Run:
```bash
cargo test -p factory-dispatcher --test ac017_demo_evidence
```

---

## Structural Tests (always-on, non-ignored)

```bash
cargo test -p factory-dispatcher -- latency_budget_constant latency_canary_sample
```

Expected output:
```
test test_BC_1_14_001_ac016_latency_budget_constant_is_500ms ... ok
test test_BC_1_14_001_ac016_canary_sample_size_is_100 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; ...
```

---

## Test File Cross-Links

- `crates/factory-dispatcher/tests/latency_canary.rs` — full canary harness;
  lines 122-183 contain the binary-spawn measurement loop (real production dispatch,
  not a `std::hint::black_box` no-op)
- `crates/factory-dispatcher/tests/ac017_demo_evidence.rs` — `test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value`

---

## Registry Context

The live `plugins/vsdd-factory/hooks-registry.toml` contains:
- 10 plugins marked `async = true` (fire-and-forget, not in sync_group)
- Multiple sync validators in the sync_group for `PostToolUse` events:
  `convergence-tracker`, `purity-check`, `regression-gate`, `validate-anchor-capabilities-union`,
  `validate-bc-title`, `validate-changelog-monotonicity`, `validate-demo-evidence-story-scoped`,
  `validate-factory-path-root`, `validate-finding-format`, `validate-index-self-reference`,
  `validate-input-hash`, `validate-novelty-assessment`, `validate-pr-description-completeness`,
  and others

The T-3h misclassification audit (referenced in `latency_canary.rs` docstring) has been
applied: 10 telemetry plugins are correctly classified `async = true`. The remaining
sync_group latency is dominated by OS process spawn cost in the binary-spawn canary design.
