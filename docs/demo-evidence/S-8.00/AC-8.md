---
story_id: S-8.00
ac: AC-8
title: All output artifacts committed to canonical paths
---

# AC-8: All output artifacts committed to canonical paths

**Statement:** All output artifacts committed to their canonical paths: `.factory/measurements/E-8-bash-baseline.json` (perf + bundle-size data), `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md` (BC-anchor verification table). No artifacts left uncommitted or under `.scratch/`.

## Evidence

### `ls -la` confirming both canonical artifacts exist

```
$ ls -la .factory/measurements/E-8-bash-baseline.json \
         .factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md

-rw-r--r--@ 1 jmagady  staff   1668 May  2 00:54 .factory/measurements/E-8-bash-baseline.json
-rw-r--r--@ 1 jmagady  staff   5598 May  2 00:58 .factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md
```

### JSON validity check (jq passes)

```bash
$ jq . .factory/measurements/E-8-bash-baseline.json
```

Output: valid JSON — all required schema fields present:
- `measured_at` ✓
- `ci_runner_profile` ✓
- `measurement_method` ✓
- `hooks` (array with 3 entries) ✓
- `tier2_aggregate_projection` ✓
- `bundle_size` ✓

### Artifact inventory

| Artifact | Path | Size | Status |
|----------|------|------|--------|
| Perf + bundle-size baseline | `.factory/measurements/E-8-bash-baseline.json` | 1,668 bytes | Committed |
| BC-anchor verification table | `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md` | 5,598 bytes | Committed |
| E-8 epic (v1.10 fix-burst) | `.factory/stories/epics/E-8-native-wasm-migration.md` | updated | Committed |
| Perf bats harness | `tests/perf/E-8-bash-baseline.bats` | present | Committed |
| Perf tests README | `tests/perf/README.md` | present | Committed |

### No scratch artifacts

No artifacts exist under `.scratch/` or any non-canonical path. The `.factory/measurements/` directory was created as part of AC-8 (per EC-006: directory did not exist pre-S-8.00; created explicitly).

**Result:** AC-8 SATISFIED. Both canonical output artifacts are present at their required paths with correct file sizes. JSON parses cleanly. No uncommitted artifacts remain.
