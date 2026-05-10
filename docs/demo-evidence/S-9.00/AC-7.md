---
story_id: S-9.00
ac: AC-7
title: Cold-start p95 measured and recorded (WARNING — gate exceedance flagged)
---

# AC-7: S-8.00 latency baseline re-confirmed + cold-start p95 measured

**Statement:** S-8.00 warm-invocation baseline (19ms/plugin) re-confirmed without re-measurement. Cold-start p95 measured once using `hyperfine --warmup 0` via `handoff-validator` (SubagentStop fixture). Value recorded in `perf-baseline-w16.md`. Gate: ≤ 500ms (E-8 R-8.08). AC passes if value is RECORDED — the gate exceedance is flagged for CI re-measurement, not a blocker for this AC.

**Note (adversary pass-1 fix):** Methodology updated from N=10 to N=30 for p95 reliability. The N=10 value (627.8ms) was within the N=30 IQR.

**Note (adversary pass-3 fix):** p95 formula corrected to NIST nearest-rank `ceil(0.95*N)-1` (methodology_version 2). Prior methodology_version 1 reported 665.0ms (floor-index, index 27). Methodology_version 2 reports 642.6ms (ceil-index, index 28). 642.6ms is the canonical baseline for S-9.01..S-9.07 delta comparisons.

---

## WARNING: COLD-START GATE EXCEEDANCE

> The measured cold_start_p95_measured_ms of **642.6ms** (N=30, methodology_version 2, NIST nearest-rank p95) **exceeds the 500ms gate** (E-8 R-8.08).
>
> This DOES NOT fail AC-7 — the AC's pass criterion is "value is recorded in baseline doc" (analogous to S-8.00's approach of recording a violation and triggering a fix-burst pathway). However, this is a potential **R-W16-003 trigger** requiring CI re-measurement before S-9.01..S-9.07 may be dispatched.
>
> The darwin-arm64 local dev machine may run 10-30% slower than CI ubuntu-latest. Recommend re-measurement on linux-x64 CI runner before dispatch.

---

## Command

```bash
# Verify fixture targets handoff-validator (SubagentStop, not legacy-bash-adapter)
jq '{hook_event_name, session_id}' \
  .factory/measurements/fixtures/handoff-validator-input.json

# Verify cold-start value recorded in baseline doc
grep "cold_start_p95_measured_ms" .factory/architecture/perf-baseline-w16.md
grep "500" .factory/architecture/perf-baseline-w16.md | head -5
```

## Output

```json
{
  "hook_event_name": "SubagentStop",
  "session_id": "test-session-s-9.00-perf"
}
```

```
| cold_start_p95_measured_ms | 642.6 | S-9.00 hyperfine --warmup 0 --runs 30 (darwin-arm64, 2026-05-05; methodology_version 2) |
| cold_start_p95_gate_ms | 500 (HARD gate; inherited from E-8 R-8.08) |
```

## Fresh Measurement (adversary pass-3 evidence re-run)

Live script output (2026-05-05, N=30, methodology_version 2): `cold_start_p95_measured_ms` = 642.6ms (NIST nearest-rank p95; exceeds 500ms gate).

## Fixture Verification

| Property | Expected | Actual |
|----------|----------|--------|
| `hook_event_name` | `SubagentStop` | `SubagentStop` |
| References `legacy-bash-adapter` | Must NOT | Not present (verified) |
| Valid JSON | Yes | Yes |

## Latency Values Recorded in `perf-baseline-w16.md`

| Metric | Value | Source |
|--------|-------|--------|
| `warm_invocation_p50_ms` | 19 | S-8.00 PR #47 develop@9e649ed |
| `aggregate_437ms_projection` | 19ms × 23 plugins = 437ms | S-8.00 AC-2 + E-8 R-8.08 |
| `cold_start_p95_gate_ms` | 500 | E-8 R-8.08 (canonical; raised from 200ms) |
| `cold_start_p95_measured_ms` | **642.6** (N=30, methodology_version 2) | S-9.00 hyperfine --warmup 0 --runs 30 (darwin-arm64) |

Note: ADR-014 Amendment 2026-05-03 erroneously cites "R-8.10"; E-8 v1.10 risk table is the source of truth — **R-8.08** is the canonical ID.

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 7 S-9.00 AC-7: cold-start baseline measured via handoff-validator and recorded in perf-baseline-w16.md
```

## Verdict

PASS (with mandatory flag) — Cold-start p95 value (642.6ms, N=30, methodology_version 2) is recorded in `perf-baseline-w16.md`. Fixture targets handoff-validator/SubagentStop (not legacy-bash-adapter). 500ms gate cited. Per-wave telemetry delta reference = 642.6ms. Bats test AC-7 passes.

**FLAG: 642.6ms > 500ms target. Potential R-W16-003 trigger. Recommend CI re-measurement on linux-x64 runner before dispatching S-9.01..S-9.07. If CI cold-start also exceeds 500ms, escalate per EC-004.**
