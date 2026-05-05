---
story_id: S-9.00
ac: AC-3
title: W-16 latency-primary and advisory bundle ceiling established
---

# AC-3: W-16 latency-primary and advisory bundle ceiling established (ADR-014 R-8.09 revised)

**Statement:** (a) Primary gate (HARD): cold-start p95 ≤ 500ms (E-8 R-8.08). (b) Advisory soft cap: target ceiling = 2 × v1.0.0-rc.1 `all_hook_plugins_wasm_bytes`. (c) Hard kill-switch: `grand_total_bytes` ≥ 30MB. (d) Per-wave telemetry fields. (e) Pause criterion. All committed to `perf-baseline-w16.md`.

## Command

```bash
# Verify JSON output has distinct advisory cap fields
bash .factory/measurements/measure-bundle-sizes.sh \
  plugins/vsdd-factory/hook-plugins/ 2>/dev/null | \
  jq '{all_hook_plugins_wasm_bytes, unaccounted_wasm_bytes, dispatcher_bytes, grand_total_bytes}'
```

## Output

```json
{
  "all_hook_plugins_wasm_bytes": 8549146,
  "unaccounted_wasm_bytes": 155053,
  "dispatcher_bytes": 12250912,
  "grand_total_bytes": 20955111
}
```

## Gate Model Values (from `.factory/architecture/perf-baseline-w16.md`)

| Field | Value | Status |
|-------|-------|--------|
| `cold_start_p95_gate_ms` | 500 (HARD; E-8 R-8.08) | Inherited — see AC-7 for measured value |
| `w16_advisory_bundle_soft_cap_bytes` | 643,686 (= 321,843 × 2) | Established |
| `w16_bundle_hard_killswitch_bytes` | 30,000,000 (30MB) | Established |
| `per_wave_telemetry_fields` | `bundle_size_delta_bytes, cold_start_p95_delta_ms` | Documented |
| `pause_criterion` | cold-start regresses >10% vs previous wave | Documented |
| Current `all_hook_plugins_wasm_bytes` | 8,549,146 (frozen-17 sum) | Well above advisory cap (by design — advisory cap was pre-W-15 baseline) |
| Current `unaccounted_wasm_bytes` | 155,053 | Non-frozen files (hello-hook.wasm + stubs); review per wave |
| Current `grand_total_bytes` | 20,955,111 (~20MB) | ~9MB headroom under 30MB kill-switch |

### Advisory Soft Cap Rationale

- Pre-W-15 baseline (v1.0.0-rc.1 via `git ls-tree -lr`): **321,843 bytes** (3 WASM files)
- Advisory soft cap formula: 321,843 × 2 = **643,686 bytes**
- Current `all_hook_plugins_wasm_bytes` = 8,549,146 — exceeds advisory cap because W-15 already ported 17 plugins. The advisory cap model applies to future growth, not to block the already-shipped W-15 work.
- Cap applies to `all_hook_plugins_wasm_bytes` ONLY; `grand_total_bytes` is not capped (dispatcher binary dominates at ~12MB).

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 3 S-9.00 AC-3: JSON has distinct all_hook_plugins_wasm_bytes, grand_total_bytes, and dispatcher_bytes fields
```

## Verdict

PASS — JSON output has distinct `all_hook_plugins_wasm_bytes`, `unaccounted_wasm_bytes`, `grand_total_bytes`, and `dispatcher_bytes` fields. Grand total equality check: 8,549,146 + 155,053 + 12,250,912 = 20,955,111 (grand_total_bytes verified). Gate model committed to `perf-baseline-w16.md` with all required fields. Bats test AC-3 passes.
