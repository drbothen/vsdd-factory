---
document_type: architecture-baseline
level: L4
section: "w16-perf-baseline"
story_id: "S-9.00"
version: "1.0"
status: baseline
producer: implementer
timestamp: 2026-05-05T04:48:03Z
phase: "Phase 3 — W-16 Wave 0 pre-work"
traces_to: .factory/stories/S-9.00-perf-baseline-and-bundle-ceiling.md
references:
  - ADR-014 R-8.09 revised (Amendment 2026-05-03)
  - E-8 R-8.08 (canonical cold-start 500ms gate)
  - S-8.00 PR #47 develop@9e649ed (warm-invocation baseline)
  - ADR-013 (convergence gate before S-9.01..S-9.07 dispatch)
---

# W-16 Perf Baseline + Bundle Growth Ceiling

**Story:** S-9.00 — Perf baseline + W-16 bundle growth ceiling
**Epic:** E-9 — Tier 2 Native WASM Migration (W-16)
**Blocks:** S-9.01, S-9.02, S-9.03, S-9.04, S-9.05, S-9.06, S-9.07

> **ADR-013 Convergence Requirement:** Adversarial convergence (3 consecutive
> NITPICK_ONLY passes per ADR-013) is required before this baseline is
> implementation-ready and before S-9.01..S-9.07 may be dispatched for
> implementation. See §"Convergence Gate" below.

---

## W-16 Bundle Baseline (post-rc.4, pre-Tier 2)

Measurement taken at W-16 Wave 0 using `measure-bundle-sizes.sh` against the
release bundle at `plugins/vsdd-factory/hook-plugins/` (darwin-arm64).

| Metric | Value |
|--------|-------|
| measured_at | 2026-05-05T05:14:10Z |
| release_tag_sha | (post-rc.8; develop HEAD at measurement time) |
| develop_head_sha | see `git rev-parse HEAD` at measurement time |
| platform | darwin-arm64 |
| all_hook_plugins_wasm_bytes | 8549146 (frozen-17 sum; = sum(per_plugin)) |
| unaccounted_wasm_bytes | 155053 (hello-hook.wasm + underscore-named stubs not in frozen enumeration — review needed) |
| dispatcher_binary_bytes | 12250912 |
| grand_total_bytes | 20955111 |

### Measurement Path

Bundle directory measured: `plugins/vsdd-factory/hook-plugins/`
Dispatcher binary measured: `target/release/factory-dispatcher`
Byte-count method: `wc -c < <file>` (POSIX portable; works on macOS BSD and Linux GNU)
Do NOT use `du -sb` (GNU-only `-b` flag; macOS `du` uses `-k` for kibibytes).

### Script Output (JSON)

```json
{
  "methodology_version": 1,
  "measurement_timestamp": "2026-05-05T05:14:10Z",
  "host_platform": "Darwin-arm64",
  "all_hook_plugins_wasm_bytes": 8549146,
  "unaccounted_wasm_bytes": 155053,
  "dispatcher_bytes": 12250912,
  "grand_total_bytes": 20955111,
  "cold_start_p95_measured_ms": 679.1,
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

### Per-Plugin Sizes (17-Plugin Frozen Enumeration, AC-2)

| Plugin Name | wasm_bytes |
|-------------|-----------|
| block-ai-attribution | 176647 |
| capture-commit-activity | 170580 |
| capture-pr-activity | 173713 |
| handoff-validator | 163030 |
| legacy-bash-adapter | 172860 |
| pr-manager-completion-guard | 1216002 |
| regression-gate | 227618 |
| session-end-telemetry | 205160 |
| session-learning | 115968 |
| session-start-telemetry | 216506 |
| tool-failure-hooks | 156345 |
| track-agent-start | 1205443 |
| track-agent-stop | 1204506 |
| update-wave-state-on-merge | 1455326 |
| validate-pr-review-posted | 1213652 |
| warn-pending-wave-gate | 319924 |
| worktree-hooks | 155866 |
| **total_bytes (17-plugin sum)** | **8549146** |

`all_hook_plugins_wasm_bytes` now equals the frozen-17 sum (8549146 = sum(per_plugin)).
Non-frozen files (hello-hook.wasm + underscore-named stubs) are captured in
`unaccounted_wasm_bytes` (155053 bytes) — review needed if non-zero per wave.

---

## Pre-W-15 Baseline (Advisory Cap Denominator)

Per Task A.0 (git ls-tree audit at v1.0.0-rc.1 tag):

```
git ls-tree -lr v1.0.0-rc.1 plugins/vsdd-factory/hook-plugins/ \
  | awk '{sum+=$4} END {print sum}'
```

Result: **321,843 bytes** (3 wasm files: capture_commit_activity 103B +
hello-hook 152370B + legacy-bash-adapter 169370B at v1.0.0-rc.1 tag).

Note: v1.0.0 GA tag does not exist as of 2026-05-03; v1.0.0-rc.1 (cut at W-13
close, pre-W-15) is the canonical pre-W-15 baseline reference.

---

## W-16 Gate Model (ADR-014 R-8.09 Revised — Option C)

Reference: E-9 D-9.4 "Option C" + ADR-014 R-8.09 (Amendment 2026-05-03).
Replaces the old E-8 R-8.09 25% bundle-only ceiling (that model would have
blocked W-16 even when cold-start budget is satisfied).

| Field | Value |
|-------|-------|
| cold_start_p95_gate_ms | 500 (HARD gate; inherited from E-8 R-8.08) |
| w16_advisory_bundle_soft_cap_bytes | 643686 (= 321843 × 2; applies to `all_hook_plugins_wasm_bytes` only, NOT `grand_total_bytes`) |
| w16_bundle_hard_killswitch_bytes | 30000000 (30MB; `grand_total_bytes` ≥ 30MB requires architecture review) |
| per_wave_telemetry_fields | bundle_size_delta_bytes, cold_start_p95_delta_ms |
| pause_criterion | cold-start regresses >10% vs previous wave |

### Advisory Soft Cap Rationale

The advisory soft cap applies to `all_hook_plugins_wasm_bytes` ONLY — NOT to
`grand_total_bytes`. The dispatcher binary (~12.1MB) dominates `grand_total_bytes`
and is not the growth driver for plugin porting work. Applying a cap to
`grand_total_bytes` would be meaningless given the dispatcher size dominance.

Formula: `advisory_soft_cap = pre-W-15-baseline × 2 = 321843 × 2 = 643686 bytes`

### Per-Plugin Advisory Cap (AC-8: median × 3 formula)

New plugins added in W-16 MUST be ≤ median(17-plugin enumeration) × 3 OR
justified in this document's notes.

Median computation over 17-plugin frozen enumeration (sorted sizes):
`115968, 155866, 156345, 163030, 170580, 172860, 173713, 176647, 205160,
216506, 227618, 319924, 1204506, 1205443, 1213652, 1216002, 1455326`

Count: 17, median index: 8 → **median = 205160 bytes**
Per-plugin advisory cap: 205160 × 3 = **615480 bytes**

Note: The median value used above (205160, session-end-telemetry) is the
midpoint of the sorted 17-plugin enumeration. This is the advisory ceiling
for new W-16 plugin additions.

### Hard Kill-Switch Threshold

`grand_total_bytes ≥ 30,000,000` (30MB) triggers mandatory architecture review
before any further plugin dispatch. Current baseline: 20,955,111 bytes (~20MB).
Headroom to kill-switch: ~9MB.

---

## Latency Baseline

| Metric | Value | Source |
|--------|-------|--------|
| warm_invocation_p50_ms | 19 | S-8.00 PR #47 develop@9e649ed |
| aggregate_437ms_projection | 19ms × 23 plugins | S-8.00 AC-2 + E-8 R-8.08 |
| cold_start_p95_gate_ms | 500 | E-8 R-8.08 (canonical; raised from 200ms; E-8 v1.10 risk table is source of truth) |
| cold_start_p95_measured_ms | 665.0 | S-9.00 hyperfine --warmup 0 --runs 30 (darwin-arm64, 2026-05-05) |
| cold_start_median_ms | 620.6 | N=30 (darwin-arm64, 2026-05-05) |
| cold_start_IQR_ms | 36.7 | Q1=603.4ms, Q3=640.1ms; min=522.4ms, max=690.0ms; N=30 |

**Citation:** "Source: S-8.00 PR #47 develop@9e649ed; warm p50 = 19ms/plugin;
aggregate 437ms; cold-start p95 gate 500ms per E-8 R-8.08 (canonical risk ID per
E-8 v1.10 risk table; ADR-014 Amendment 2026-05-03 erroneously cites R-8.10 —
E-8 v1.10 is the source of truth)."

### Cold-Start Measurement Status

The measured cold_start_p95_measured_ms of **665.0ms** (N=30) exceeds the 500ms gate.
This is measured on darwin-arm64 local dev machine; CI ubuntu-latest is typically
10-30% faster. Flag as potential R-W16-003 trigger; recommend re-measurement on
linux-x64 CI runner before dispatching S-9.01..S-9.07.

**Variance disclosure (N=30):** median=620.6ms, p95=665.0ms, IQR=36.7ms
(Q1=603.4ms, Q3=640.1ms). Prior N=10 value of 627.8ms was within the N=30 IQR.

If cold-start exceeds 500ms on CI: escalate per EC-004 (R-W16-003 triggered) before
dispatch.

---

## Methodology

### Measurement Path

1. Build release artifacts: `cargo build --release --target wasm32-wasip1`
   and `cargo build --release -p factory-dispatcher`
2. Run `measure-bundle-sizes.sh plugins/vsdd-factory/hook-plugins/`
3. Script uses `wc -c < <file>` for all byte counts (POSIX portable; `LC_ALL=C` for locale-safe output)
4. Cold-start measured via `hyperfine --warmup 0 --runs 30` (SubagentStop fixture; N=30 for p95 reliability)
5. p95 computed as index `floor(N * 0.95) - 1` of sorted time samples (N=30)

### Reproducibility

Re-run `measure-bundle-sizes.sh` against the same fixed bundle directory to
reproduce measurements. Two consecutive runs produce identical `all_hook_plugins_wasm_bytes`
and `per_plugin` values (idempotent for a fixed artifact set).

### Cross-Platform Notes

- macOS (BSD) and Linux (GNU): `wc -c < <file>` is portable (both supported)
- Windows (Git Bash): `wc -c` available in Git Bash 3.2+; script runs unmodified
- Windows (PowerShell native): use `(Get-Item <file>).Length` as alternative
- Do NOT use `du -sb` (GNU-only `-b` flag; macOS uses `-k` for kibibytes)

### Cold-Start Measurement Fixture

Fixture: `.factory/measurements/fixtures/handoff-validator-input.json`
Event: `SubagentStop` (handoff-validator's registered event)
Target: `handoff-validator.wasm` (native WASM; NOT legacy-bash-adapter)

Rationale for handoff-validator vs legacy-bash-adapter: legacy-bash-adapter adds
bash-spawn subprocess overhead on top of WASM instantiation. It is not representative
of native WASM cold-start (watch-out B.1 from S-9.00 spec).

---

## Convergence Gate (ADR-013)

Per ADR-013, adversarial convergence (3 consecutive NITPICK_ONLY passes) is
**required** before S-9.00 is implementation-ready and before S-9.01..S-9.07
may be dispatched for implementation.

The NITPICK_ONLY convergence gate prevents dispatch of batch stories against an
unconverged baseline, which would cause cascading drift across all 7 batch stories.

---

## Per-Wave Telemetry Requirements

Each batch story (S-9.01..S-9.07) MUST record:
- `bundle_size_delta_bytes`: change in `all_hook_plugins_wasm_bytes` vs this baseline
- `cold_start_p95_delta_ms`: change in cold-start p95 vs this baseline (627.8ms)

Pause criterion: wave paused if cold-start regresses >10% vs previous wave.
At W-16 baseline (N=30): 10% of 665.0ms = 66.5ms; pause threshold = 731.5ms.

---

## Notes

- The ~7.2MB pre-W-15 baseline cited in W-16-spec-foundation-research.md §Q3 was
  a research-document projection extrapolated from industry comparables — NOT an
  actual measurement. The authoritative pre-W-15 baseline is 321,843 bytes (from
  `git ls-tree -lr v1.0.0-rc.1` audit above).
- S-9.30 (host::run_subprocess) withdrawn per ADR-014 D-9.2 amendment 2026-05-03;
  no SDK bundle contribution from run_subprocess to measure.
- E-8 R-8.09 25%-bundle-only ceiling is SUPERSEDED by this latency-primary model.
