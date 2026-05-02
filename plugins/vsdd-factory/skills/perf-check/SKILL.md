---
name: perf-check
description: Run performance validation — benchmark regression checks, resource profiling, and budget compliance. Ensures no performance regression between implementations.

allowed-tools: Read, Write, Bash, Glob, Grep
---

# Performance Check

Validate performance characteristics against budgets and detect regressions.

## Checks

### 1. Benchmark Suite

Run the project's benchmark suite (if it exists):

```bash
cargo bench
```

Or with criterion for detailed comparison:

```bash
cargo bench -- --baseline main
```

**Look for:**
- Regressions > 10% vs baseline
- New benchmarks needed for added functionality
- P50, P95, P99 latency distributions

### 2. Binary Size

```bash
cargo build --release
ls -lh target/release/<binary>
```

Track binary size over time. Flag if it grows > 20% in a single wave.

### 3. Startup Time

```bash
time target/release/<binary> --help
hyperfine 'target/release/<binary> --help'
```

Budget: CLI startup < 100ms.

### 4. Memory Profiling

For long-running processes:

```bash
# Peak memory usage
/usr/bin/time -l target/release/<binary> <workload>

# Heap profiling (if DHAT available)
cargo run --release --features dhat-heap -- <workload>
```

**Look for:**
- Unbounded memory growth (leaks)
- Excessive allocations
- Large resident set size for small workloads

### 5. Compile Time

```bash
cargo clean && time cargo build --release 2>&1
cargo clean && time cargo build 2>&1
```

Track debug and release build times. Flag if > 30% regression.

### 6. Test Suite Performance

```bash
time cargo test --release 2>&1
```

Track test suite execution time. Flag if > 50% regression.

## Budgets

Define in `.factory/specs/prd-supplements/performance-budgets.md` (if exists):

| Metric | Budget | Measured |
|--------|--------|---------|
| CLI startup | < 100ms | ... |
| Binary size (release) | < 50MB | ... |
| Debug build time | < 60s | ... |
| Test suite time | < 120s | ... |

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/performance-report-template.md` for the performance report format.

## Output

Write to `.factory/cycles/<current>/performance-report.md`:

```markdown
# Performance Report

## Summary
| Metric | Budget | Measured | Status |
|--------|--------|---------|--------|
| ... | ... | ... | ✅/⚠️/❌ |

## Benchmark Results
<Detailed results>

## Regressions Detected
<Any regressions with analysis>

## Recommendations
<Optimization suggestions if budgets are exceeded>

## Gate: PASS | WARN | FAIL
```

If no benchmarks exist yet, report that and recommend creating them.
