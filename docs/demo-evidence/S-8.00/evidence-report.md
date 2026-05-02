---
story_id: S-8.00
document_type: evidence-report
version: "1.0"
status: complete
producer: demo-recorder
timestamp: 2026-05-02T00:00:00Z
---

# Evidence Report: S-8.00 — Perf Benchmark Baseline + Tier 1 BC-Anchor Verification

**Story:** S-8.00 — Perf benchmark baseline + Tier 1 BC-anchor verification
**Branch:** `feature/S-8.00-perf-baseline-bc-anchor-verification`
**Product type:** CLI/measurement story — terminal output captures (no VHS or Playwright needed)
**Evidence location:** `docs/demo-evidence/S-8.00/`

## Coverage Summary

| AC | Title | Evidence File | Status |
|----|-------|--------------|--------|
| AC-1 | Bash warm-invocation latency measured | AC-1.md | SATISFIED |
| AC-2 | Tier 2 aggregate projection computed | AC-2.md | SATISFIED |
| AC-3 | E-8 AC-7b ceiling adjusted (fix-burst v1.9 → v1.10) | AC-3.md | SATISFIED |
| AC-4 | BC-anchor verification table for all 9 hooks | AC-4.md | SATISFIED |
| AC-5 | BC-gap remediation (0 of 9 gaps) | AC-5.md | SATISFIED |
| AC-6 | Story point estimates confirmed (0 bumps) | AC-6.md | SATISFIED |
| AC-7 | Bundle-size baseline measured | AC-7.md | SATISFIED |
| AC-8 | All artifacts committed to canonical paths | AC-8.md | SATISFIED |
| AC-9 | E-8 epic fix-burst integrity verified | AC-9.md | SATISFIED |

**Total: 9/9 ACs satisfied.**

---

## AC-1: Bash warm-invocation latency measured

**Evidence:** Bats test output (3/3 PASS) + measured latencies in `.factory/measurements/E-8-bash-baseline.json`.

```
$ bats tests/perf/E-8-bash-baseline.bats
 ✓ BC-perf-baseline AC-1: handoff-validator.sh Tier 1 warm-invocation latency
 ✓ BC-perf-baseline AC-1: validate-bc-title.sh Tier 2 warm-invocation latency
 ✓ BC-perf-baseline AC-1: protect-bc.sh Tier 3 warm-invocation latency

3 tests, 0 failures
```

```json
"hooks": [
  { "name": "handoff-validator.sh",  "tier": 1, "median_ms": 43, "p95_ms": 56 },
  { "name": "validate-bc-title.sh",  "tier": 2, "median_ms": 19, "p95_ms": 21 },
  { "name": "protect-bc.sh",         "tier": 3, "median_ms": 40, "p95_ms": 42 }
]
```

Measurement method: `bats + hyperfine --warmup 3 --runs 10` (hyperfine 1.18.0)

---

## AC-2: Tier 2 aggregate projection computed

**Evidence:** `tier2_aggregate_projection` from `.factory/measurements/E-8-bash-baseline.json`.

```json
"tier2_aggregate_projection": {
  "per_plugin_ms": 19,
  "plugin_count": 23,
  "projected_aggregate_ms": 437,
  "ac7b_ceiling_ms": 200,
  "ac7b_attainable": false
}
```

19ms × 23 plugins = 437ms. The 200ms ceiling is violated; `ac7b_attainable: false`. AC-3 fix-burst pathway triggered.

---

## AC-3: E-8 AC-7b ceiling adjusted via fix-burst

**Evidence:** E-8 epic version 1.10 with v1.10 Changelog entry.

```
version: "1.10"
```

```markdown
### v1.10 (2026-05-02) — S-8.00 AC-3 fix-burst: AC-7b ceiling raised, OQ-8 resolved

- AC-7b ceiling: 200ms → **500ms p95** (accommodates 437ms bash baseline with 15% headroom)
- Goal #6 updated to match new AC-7b ceiling
- R-8.08 re-scored MEDIUM/MEDIUM → **HIGH/HIGH** (warm-pool is now REQUIRED)
- OQ-8 marked RESOLVED
```

---

## AC-4: BC-anchor verification table for all 9 Tier 1 hooks

**Evidence:** Full audit table at `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md`.

| Hook | BC IDs | Gap-Found |
|------|--------|-----------|
| handoff-validator.sh | BC-7.03.042/043/044 | N |
| pr-manager-completion-guard.sh | BC-7.03.045/046/047/048 | N |
| track-agent-stop.sh | BC-7.03.081/082 | N |
| update-wave-state-on-merge.sh | BC-7.03.083/084/085/086 | N |
| validate-pr-review-posted.sh | BC-7.04.040/041/042/043/044 | N |
| session-learning.sh | BC-7.03.076/077/078 | N |
| warn-pending-wave-gate.sh | BC-7.03.091/092 | N |
| track-agent-start.sh | BC-7.03.079/080 | N |
| regression-gate.sh | BC-7.01.003, BC-7.03.071..075 | N (OQ-6 deferred) |

All 9 hooks present. Schema (Hook, BC ID(s), BC Title(s), Spec-Current Y/N, Gap-Found Y/N, Action-Needed) confirmed.

---

## AC-5: BC-gap remediation

**Evidence:** Audit summary from `E-8-bc-anchor-table.md`.

```
- Hooks with Gap-Found = Y: 0
- AC-5 >5-gap threshold triggered: No (0 of 9)
- New BCs drafted: 0
- Story point bumps triggered: 0
```

0 of 9 gaps found. The AC-5 majority threshold (>5 of 9 = ~56%) was not triggered. No OQ-9 filing required. No W-16 deferral evaluation needed.

---

## AC-6: Story point estimates confirmed

**Evidence:** BC-anchor table notes section + E-8 epic Stories table unchanged from v1.9.

Since 0 hooks required new BCs, no +1pt BC-creation overhead applies to any S-8.01..S-8.09 story. All 9 port stories retain their original estimates (4/5/3/4/3/3/3/3/5 pts). S-8.09 notes the OQ-6 deferred sub-task without a point bump.

---

## AC-7: Bundle-size baseline measured

**Evidence:** `bundle_size` section of `.factory/measurements/E-8-bash-baseline.json`.

```json
"bundle_size": {
  "measured_at": "2026-05-02T00:00:00Z",
  "legacy_bash_adapter_wasm_bytes": 169370,
  "all_hook_plugins_wasm_bytes": 321843,
  "dispatcher_binary_bytes": 12150752
}
```

R-8.09 baseline: 25% headroom = ~2.9 MB additional before size-optimization review triggers.

---

## AC-8: All artifacts committed to canonical paths

**Evidence:** `ls -la` output for both canonical paths.

```
-rw-r--r--  1668 bytes  .factory/measurements/E-8-bash-baseline.json
-rw-r--r--  5598 bytes  .factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md
```

JSON schema validation: `jq . .factory/measurements/E-8-bash-baseline.json` passes. All required fields present. `.factory/measurements/` directory created explicitly (per EC-006 — did not exist pre-S-8.00).

---

## AC-9: E-8 epic fix-burst integrity verified

**Evidence:** E-8 epic `version: "1.10"` + v1.10 Changelog entry + updated AC-7b + updated R-8.08 + RESOLVED OQ-8.

```yaml
version: "1.10"
```

All four fix-burst fields updated:
- AC-7b: 200ms → 500ms p95
- Goal #6: updated to 500ms language
- R-8.08: re-scored MEDIUM/MEDIUM → HIGH/HIGH
- OQ-8: RESOLVED by S-8.00 (2026-05-02)

---

## Artifact Inventory

| Artifact | Path | Committed |
|----------|------|-----------|
| Perf + bundle-size baseline | `.factory/measurements/E-8-bash-baseline.json` | Yes |
| BC-anchor verification table | `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md` | Yes |
| E-8 epic (v1.10) | `.factory/stories/epics/E-8-native-wasm-migration.md` | Yes |
| Perf bats harness | `tests/perf/E-8-bash-baseline.bats` | Yes |
| Perf tests README | `tests/perf/README.md` | Yes |
| Demo evidence (this dir) | `docs/demo-evidence/S-8.00/` | Yes |
