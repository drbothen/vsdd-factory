---
story_id: S-9.00
ac: AC-5
title: Script reproduces per-plugin sizes (anti-tautology gate)
---

# AC-5: Script reproduces per-plugin sizes byte-for-byte (anti-tautology gate)

**Statement:** Running `measure-bundle-sizes.sh` twice against the same fixed bundle dir produces identical per-plugin sizes and total (±0 bytes). This prevents hand-written numbers (POLICY 11 — no test tautologies).

## Command

```bash
# Run script twice, compare per_plugin maps and all_hook_plugins_wasm_bytes
RUN1=$(bash .factory/measurements/measure-bundle-sizes.sh \
  plugins/vsdd-factory/hook-plugins/ 2>/dev/null)
RUN2=$(bash .factory/measurements/measure-bundle-sizes.sh \
  plugins/vsdd-factory/hook-plugins/ 2>/dev/null)

echo "Run 1 all_hook_plugins_wasm_bytes: $(echo "$RUN1" | jq '.all_hook_plugins_wasm_bytes')"
echo "Run 2 all_hook_plugins_wasm_bytes: $(echo "$RUN2" | jq '.all_hook_plugins_wasm_bytes')"

diff <(echo "$RUN1" | jq -cS '.per_plugin') \
     <(echo "$RUN2" | jq -cS '.per_plugin') \
  && echo "per_plugin maps: IDENTICAL (diff is empty)" \
  || echo "DIVERGENCE DETECTED"
```

## Output

```
Run 1 all_hook_plugins_wasm_bytes: 8704199
Run 2 all_hook_plugins_wasm_bytes: 8704199
per_plugin maps: IDENTICAL (diff is empty)
```

## Independent Verification (Anti-Tautology)

The bats test AC-5 computes expected values INDEPENDENTLY using `wc -c` and never reads `perf-baseline-w16.md`. Each frozen plugin is measured fresh and compared to the script's reported value:

```
block-ai-attribution:        176647  ← independent wc -c  =  reported 176647  MATCH
capture-commit-activity:     170580  ← independent wc -c  =  reported 170580  MATCH
capture-pr-activity:         173713  ← independent wc -c  =  reported 173713  MATCH
handoff-validator:           163030  ← independent wc -c  =  reported 163030  MATCH
legacy-bash-adapter:         172860  ← independent wc -c  =  reported 172860  MATCH
pr-manager-completion-guard: 1216002 ← independent wc -c  =  reported 1216002 MATCH
regression-gate:             227618  ← independent wc -c  =  reported 227618  MATCH
session-end-telemetry:       205160  ← independent wc -c  =  reported 205160  MATCH
session-learning:            115968  ← independent wc -c  =  reported 115968  MATCH
session-start-telemetry:     216506  ← independent wc -c  =  reported 216506  MATCH
tool-failure-hooks:          156345  ← independent wc -c  =  reported 156345  MATCH
track-agent-start:           1205443 ← independent wc -c  =  reported 1205443 MATCH
track-agent-stop:            1204506 ← independent wc -c  =  reported 1204506 MATCH
update-wave-state-on-merge:  1455326 ← independent wc -c  =  reported 1455326 MATCH
validate-pr-review-posted:   1213652 ← independent wc -c  =  reported 1213652 MATCH
warn-pending-wave-gate:      319924  ← independent wc -c  =  reported 319924  MATCH
worktree-hooks:              155866  ← independent wc -c  =  reported 155866  MATCH
```

All 17 plugins: independent measurement matches script output exactly (±0 bytes).

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 5 S-9.00 AC-5: script per-plugin byte counts match independent wc -c measurements
```

## Verdict

PASS — Two consecutive script runs produce identical `all_hook_plugins_wasm_bytes` (8,704,199) and `per_plugin` maps. All 17 frozen plugins verified against independent `wc -c` measurements with 0-byte divergence. No hand-written numbers. POLICY 11 anti-tautology satisfied. Bats test AC-5 passes.
