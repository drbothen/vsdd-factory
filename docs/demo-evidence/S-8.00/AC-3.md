---
story_id: S-8.00
ac: AC-3
title: E-8 AC-7b 200ms ceiling adjusted via fix-burst
---

# AC-3: E-8 AC-7b 200ms p95 ceiling adjusted (fix-burst E-8 v1.9 → v1.10)

**Statement:** E-8 epic AC-7b 200ms p95 ceiling either confirmed attainable or proposed-adjustment committed as fix-burst on E-8. The fix-burst must update E-8 AC-7b, Goal #6, and R-8.08 fields.

## Evidence

### E-8 epic frontmatter confirms v1.10

```
---
document_type: epic
epic_id: "E-8"
version: "1.10"
title: "Native WASM Migration Completion"
status: ready
```

### v1.10 Changelog entry (`.factory/stories/epics/E-8-native-wasm-migration.md`)

```markdown
### v1.10 (2026-05-02) — S-8.00 AC-3 fix-burst: AC-7b ceiling raised, OQ-8 resolved

S-8.00 perf baseline measurement (darwin-arm64 local runner, hyperfine --warmup 3 --runs 10):
- handoff-validator.sh (Tier 1): median 43ms, p95 56ms
- validate-bc-title.sh (Tier 2 representative): median 19ms, p95 21ms
- protect-bc.sh (Tier 3): median 40ms, p95 42ms

Tier 2 aggregate projection: 19ms/plugin × 23 plugins = **437ms** — exceeds the 200ms AC-7b ceiling.
The 10ms/plugin assumption (OQ-8) was incorrect; measured value is 1.9× the estimate.

**Fix-burst changes (AC-3/AC-9):**
- AC-7b ceiling: 200ms → **500ms p95** (accommodates 437ms bash baseline with 15% headroom; warm-pool mitigation required)
- Goal #6 updated to match new AC-7b ceiling
- R-8.08 re-scored MEDIUM/MEDIUM → **HIGH/HIGH** (risk confirmed; warm-pool is now REQUIRED not optional)
- OQ-8 marked RESOLVED (S-8.00 measurement closes the open question)
- input-hash: unchanged (no S-8.00 story-spec changes needed)

BC-anchor audit outcome (AC-4/AC-5): 0 of 9 Tier 1 hooks have Gap-Found = Y. All 9 hooks
have pre-existing BC-7.03/BC-7.04 coverage. No new BCs drafted. No S-8.0N story point bumps
required for BC overhead.
```

### Updated E-8 AC-7b line (D-12 acceptance criteria table)

```
| AC-7b | Aggregate PostToolUse:Edit|Write latency (sum of all 23 plugins) ≤ 500ms p95
         (ceiling raised from 200ms to 500ms by S-8.00 fix-burst v1.10; S-8.00 measured
         19ms/plugin bash median × 23 = 437ms projected aggregate; warm-pool + compile-cache
         mitigations required; OQ-8 resolved) |
```

### Updated Goal #6

```
6. Aggregate PostToolUse:Edit|Write latency (sum of all 23 plugins) ≤ 500ms p95
   (per AC-7b; ceiling raised from 200ms to 500ms by S-8.00 fix-burst v1.10
   after measuring 19ms/plugin bash median × 23 = 437ms projected aggregate;
   warm-pool + compile-cache mitigations remain required to hit sub-500ms target);
```

**Result:** AC-3 SATISFIED. E-8 epic updated from v1.9 to v1.10. AC-7b ceiling raised 200ms → 500ms. Goal #6 and R-8.08 updated. OQ-8 marked RESOLVED.
