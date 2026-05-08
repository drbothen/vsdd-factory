---
document_type: adr
adr_id: ADR-020
status: accepted
accepted_date: 2026-05-08
date: 2026-05-08
version: "1.0"
last_amended: 2026-05-08
cycle: v1.0-feature-plugin-async-semantics-pass-1
subsystems_affected: [SS-01]
supersedes: null
superseded_by: null
---

# ADR-020: Dispatcher Latency Budget Classes

## Context

### Execution model: per-invocation process spawn

Every Claude Code hook event causes the platform to spawn a fresh `factory-dispatcher`
process. On macOS arm64, this is an `execve`-based OS fork + exec. After spawning, the
dispatcher must:

1. Load the Wasmtime runtime
2. Parse `hooks-registry.toml` (~100+ entries)
3. Load and compile WASM modules on first invocation (cold-start)
4. Match the event envelope against registered plugins
5. Partition into sync/async groups (ADR-019)
6. Execute the sync group (WASM invocations, tier ordering per ADR-008)
7. Drain async tasks within `ASYNC_DRAIN_WINDOW_MS` (DI-019)
8. Exit with aggregated verdict

Steps 1–3 are structural overhead that cannot be eliminated under the current
per-invocation spawn model. On macOS arm64, process spawn (fork + exec + dylib load)
alone contributes approximately 200–400ms. WASM cold-start adds 300–600ms. These costs
are incurred on every dispatch event because each invocation spawns a fresh process with
no shared state from the previous invocation.

### Original AC-016 budget: aspirational, not measured

The original AC-016 acceptance criterion in S-15.01 set a budget of **p95 ≤ 500ms**:

> On a representative Edit/Write workload (~30+ matched plugins), the sync_group p95
> latency is measured and must be ≤ 500ms.

This budget was authored before any real measurement of the binary-spawn dispatch path
existed. F5 adversary pass-1 finding F-P1-003 flagged the original latency canary as a
test tautology: the canary body contained `std::hint::black_box(&registry)` which
measured only `Instant::now()` overhead (~42ns), not actual dispatch work. The reported
p95=42ns PASS was fictitious (finding F-P1-009).

### Measured baseline (F5 pass-1 fix burst)

Stage 2 implementer (commit `0d3796e`) replaced the no-op canary with a real
binary-spawn measurement loop. Stage 3 demo-recorder ran the corrected canary and
recorded the following (N=100, macOS arm64, release binary and debug builds):

| Mode | p50 | p95 | p99 | Original Budget | Verdict |
|------|-----|-----|-----|-----------------|---------|
| Release binary | 940ms | **1050ms** | 1111ms | 500ms | FAIL |
| Debug (cargo test) | 919ms | 1076ms | 1656ms | 500ms | FAIL |

Platform: macOS arm64. Sample size: N=100 per mode. Evidence at
`docs/demo-evidence/S-15.01/latency-canary.md` (v1.7).

The dominant cost is OS process spawn + WASM cold-start, not WASM plugin execution
time itself. The T-3h misclassification audit has already been applied (10 telemetry
plugins correctly classified `async = true`); the remaining sync_group latency cannot
be reduced without architectural changes to the spawn model.

The 500ms budget is unattainable under the current per-invocation binary-spawn
architecture on macOS. Setting p95 ≤ 500ms as a hard gate would make every
standard PostToolUse invocation a test failure on the primary development platform.

---

## Decision

### Budget class taxonomy

Dispatcher latency budgets are classified by architectural mode, not by event type.
Three classes are defined:

---

**Class A — Cold-start dispatch (per-invocation binary spawn)**

The operative class under the current architecture.

- **Definition:** Wall-clock time from dispatcher process spawn to dispatcher exit,
  measured on a representative PostToolUse / Write envelope with a full production
  registry (~30+ matched plugins, sync_group contains all non-async validators).
- **Observed p95:** 1050ms (release, macOS arm64, N=100, commit `0d3796e`).
- **Budget: p95 ≤ 1500ms.**
- **Headroom:** 1500ms is 1.43× the observed 1050ms release p95. This provides
  margin for hardware variability (slower CI runners, older MacBook Pro models,
  registry growth as new plugins are added) while remaining close enough to the
  observed baseline to catch meaningful regression.
- **Regression class detected:** A p95 > 1500ms signals either (a) a newly added
  sync plugin with pathological startup time, (b) registry size explosion (e.g., 3×
  growth in matched plugins), or (c) WASM module regression. Values 1050–1500ms are
  accepted as normal Class A operating range.
- **Applies to:** AC-016 in S-15.01, current cycle.

Rationale for 1500ms over alternatives:

- **1200ms (tighter):** 1200ms is only 1.14× headroom above p95. The p99 in release
  mode is 1111ms, meaning ~5% of runs today already approach 1200ms. This provides
  insufficient margin for CI hardware variation (AWS arm64 runners typically perform
  5–15% slower than M-series MacBook for process spawn workloads). Setting 1200ms
  risks false-positive failures on CI.
- **2000ms (looser):** 2000ms is nearly 2× the current p95. While this gives maximum
  safety margin, it also masks meaningful performance regressions. A plugin that adds
  400ms of startup cost would pass undetected. 1500ms is the tightest budget that
  comfortably clears the p99 + CI variance band.
- **1500ms chosen:** **Class A budget rationale (refreshed v1.9 release measurement):** Measured p95 = 1161ms; p99 = 1570ms. The 1500ms p95 budget provides 1.29× headroom over measured p95 (339ms margin). p99 of 1570ms is 4.6% over the 1500ms reference (a non-blocking observation; AC-016 enforces p95 only). The budget remains defensible: regressions of >340ms above measured p95 will breach the budget and surface in CI.

**Soft-guard (informational):** Latency canary records p99 alongside p95 (per `latency-canary.md` v1.9). When p99 trends upward past 1800ms across 3 consecutive measurements, this should be treated as a potential class-of-regression signal even if AC-016 (p95-only) continues to PASS. Operators should investigate; not a blocking criterion.

---

**Class B — In-process sync_group dispatch (daemon/persistent-process mode)**

Defined for future architectural work. Not operative in this cycle.

- **Definition:** Time from event receipt to sync_group verdict aggregation, measured
  in a persistent dispatcher process (daemon mode or equivalent) where process spawn
  and WASM cold-start costs are amortized across invocations.
- **Budget: p95 ≤ 50ms** (target; to be validated by measurement when implemented).
- **Status:** DEFERRED. No implementation exists. Daemon mode, persistent process,
  and WASM module caching are out of scope for this cycle. See §Out of Scope below.
- **Note:** The 50ms target reflects the ADR-019 §Consequences estimate of
  "30–100ms for a representative Edit/Write event" under in-process conditions.

---

**Class C — Async drain window**

- **Definition:** Maximum time the dispatcher waits after sync_group completion for
  async tasks to emit terminal events before forcibly terminating them.
- **Budget:** Governed by DI-019. This value is defined there and not restated here
  to preserve DI-019 as the single canonical source.
- **Status:** Operative (implemented in v1.0 of S-15.01). Not changed by this ADR.
- **Relationship to Class A:** Class A wall-clock time includes the drain window.
  Total dispatcher latency upper bound = max(sync_group) + min(max(async_group), drain).

---

### AC-016 revised budget

**AC-016 budget revised: p95 ≤ 1500ms (Class A, macOS arm64).**

The S-15.01 acceptance criterion AC-016 transitions from the aspirational 500ms budget
to the Class A budget of 1500ms. This revision is backed by N=100 real measurements of
the full production dispatch path (commit `0d3796e`). The budget applies to the current
binary-spawn architecture. If the architecture changes to daemon mode (Class B), AC-016
will be re-anchored to the Class B budget at that time.

### Out of scope for this cycle

The following optimizations would reduce per-invocation latency toward Class B levels
but are NOT part of S-15.01:

- Persistent dispatcher daemon (long-lived process receiving events over IPC)
- WASM module pre-compilation cache (AOT cache file reused across invocations)
- Registry parse cache (pre-parsed binary form written to disk between invocations)
- Tokio runtime reuse (eliminating per-spawn runtime init cost)

A follow-up story is required to design and implement these optimizations. Story-writer
will author it. See §Follow-up Story Sketch below.

---

## Consequences

### What this budget allows

| Behavior | Allowed |
|----------|---------|
| p95 up to 1500ms on macOS arm64 with binary-spawn model | YES |
| p95 between 1050ms and 1500ms on CI arm64 runners | YES |
| New sync plugin contributing up to ~450ms marginal startup | YES (at budget margin) |
| AC-016 PASS when measured canary p95 <= 1500ms | YES |

### What this budget forbids

| Behavior | Forbidden |
|----------|-----------|
| p95 > 1500ms (triggers budget exceedance; misclassification audit required) | BLOCKED |
| Reinstating the fictitious 42ns p95 measurement | BLOCKED |
| Interpreting the Class A budget as a Class B performance claim | BLOCKED |
| Treating daemon-mode optimizations as in-scope for S-15.01 | BLOCKED |

### Regression detection

A canary recording p95 > 1500ms triggers: (1) mandatory plugin misclassification
re-audit (T-3h procedure), (2) per-plugin marginal latency profiling to identify the
offending plugin, (3) reclassification to `async = true` if the plugin is
non-blocking, or (4) escalation to architectural optimization track (Class B) if
the reclassification route is exhausted.

### CI implications

The latency canary (`tests/latency_canary.rs`) must be updated to assert p95 ≤ 1500ms
rather than p95 ≤ 500ms. The structural constant in the test file
(`test_BC_1_14_001_ac016_latency_budget_constant_is_500ms`) must be renamed and its
value updated. The demo-recorder will re-record `docs/demo-evidence/S-15.01/latency-canary.md`
with the corrected budget and PASS verdict.

---

## Follow-up Story Sketch

The following describes the intended scope for the dispatcher latency optimization
story. Story-writer will author the full spec.

**Goal:** Reduce Class A (per-invocation) dispatch latency toward Class B (in-process)
levels by eliminating the dominant structural costs: OS fork + exec + dylib load and
WASM cold-start. Primary mechanism is a persistent dispatcher daemon that receives
hook events over a Unix domain socket (or stdin-based IPC) and processes them in a
long-lived Tokio runtime. Secondary mechanism is WASM module AOT pre-compilation
cache: modules are compiled to native code on first load and the compiled form is
written to a cache file (e.g., `$HOME/.cache/factory-dispatcher/wasm/`), with cache
invalidation on module file hash change. Together these two changes should reduce
per-invocation latency from ~940ms (p50) to the 30–100ms range estimated by ADR-019
§Consequences.

**Out of scope for the optimization story:** any changes to WASM plugin ABI (ADR-002),
registry schema (currently v2 per ADR-019), or hooks.json envelope format (ADR-019 §3).
The optimization is a runtime-infrastructure change only. The Class B budget (p95 ≤ 50ms)
will be set as the acceptance criterion and validated by measurement on macOS arm64 once
the daemon implementation is complete.

---

## Cross-References

| Reference | Role |
|-----------|------|
| F5 pass-1 finding F-P1-003 | Root cause — AC-016 canary was a no-op; real measurement required |
| F5 pass-1 finding F-P1-009 | Evidence — demo file `latency-canary.md` reported false PASS on 42ns measurement |
| ADR-019 (plugin async semantics at registry layer) | Defines sync/async group partition; Class C drain window; per-plugin async classification model |
| DI-019 | Canonical source for `ASYNC_DRAIN_WINDOW_MS` (Class C budget constant) |
| S-15.01 AC-016 | Acceptance criterion revised from p95 ≤ 500ms to p95 ≤ 1500ms by this ADR |
| `docs/demo-evidence/S-15.01/latency-canary.md` v1.7 | Measurement evidence: N=100, macOS arm64, p95=1050ms release |

---

## Alternatives Considered

### (a) 1200ms budget (tighter headroom)

Rejected. The release p99 today is 1111ms. Setting 1500ms ensures p99 clears the
budget even on a typical canary run. 1200ms does not clear today's p99 and would
produce false-positive CI failures on arm64 runners that are 5–15% slower than
development hardware.

### (b) 2000ms budget (maximum safety margin)

Rejected. 2000ms is nearly 2× the current p95. A newly added sync plugin that
introduces 400ms of regression would pass undetected. 1500ms detects regressions
that add ≥450ms and is the tightest budget that comfortably clears the observed p99
plus hardware variance band.

### (c) In-process canary (measure dispatch logic only, not spawn cost)

Rejected as the fix for this cycle. Changing the canary design to an in-process
measurement would require daemon-mode infrastructure that is out of scope for S-15.01.
The binary-spawn canary is the correct POLICY 11-compliant test of the production
path as deployed today. An in-process canary would validate the wrong abstraction level
for the current architecture.

### (d) Keep 500ms budget, require spawn-cost optimization before PASS

Rejected. Optimization work is multi-week; it would block S-15.01 delivery and is a
separate architectural concern. The correct sequencing is: acknowledge measured reality
now (Class A budget), ship S-15.01 with a PASS, and track optimization separately.

---

## Subsystem Assignments

**SS-01 (Hook Dispatcher Core):** Referencing SS-01 because the dispatch loop, WASM
execution, and exit-code aggregation all live in `crates/factory-dispatcher/src/`.
The Class A latency budget governs the end-to-end SS-01 execution time per invocation.
The Class B target is a future SS-01 architectural goal (daemon mode).

---

## Changelog

### v1.0 rationale clarification — 2026-05-08 (F5 fix-burst-3; F-P3-006)

Addresses adversary pass-3 finding **F-P3-006**: the rationale paragraph for the 1500ms budget choice cited "Clears p99 by 35%" which was based on the initial v1.7 debug measurement (p99 = 1111ms release). The v1.9 release measurement records p99 = 1570ms, which is 4.6% over 1500ms, making the 35% claim incorrect.

**Changes:**
- Replaced the stale "Clears p99 by 35%" sentence with a refreshed rationale citing v1.9 measured values: p95 = 1161ms, p99 = 1570ms, 1.29× headroom over p95 (339ms margin), p99 non-blocking observation noted.
- Added §Soft-guard (informational) paragraph: p99 > 1800ms trending across 3 consecutive measurements should trigger investigation even if AC-016 (p95-only) continues to PASS.
- `last_amended` confirmed 2026-05-08.
- No version bump (v1.0 stands; this is a rationale clarification, not a decision change).

### v1.0 — 2026-05-08 (initial; F5 pass-1 fix-burst origin)

- ADR authored in response to F5 pass-1 findings F-P1-003 (no-op canary) and
  F-P1-009 (false PASS demo evidence).
- User approved Path A (acknowledge operational reality) per F-P1-003 escalation.
- Class A budget set at p95 ≤ 1500ms, backed by N=100 measurement on macOS arm64.
- Class B (daemon mode) defined as deferred future target.
- Class C (drain window) delegates to DI-019.
- AC-016 revised from aspirational 500ms to evidence-backed 1500ms.
