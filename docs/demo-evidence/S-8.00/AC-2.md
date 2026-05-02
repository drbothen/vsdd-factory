---
story_id: S-8.00
ac: AC-2
title: Tier 2 aggregate latency projection computed
---

# AC-2: Tier 2 aggregate latency projection computed

**Statement:** Per-plugin latency multiplied by 23 (Tier 2 plugin count) to produce aggregate Tier 2 projection. If projection > 200ms, the 10ms/plugin assumption underlying E-8 AC-7b is confirmed violated; adjustment is required.

## Evidence

### Excerpt from `.factory/measurements/E-8-bash-baseline.json`

```json
{
  "tier2_aggregate_projection": {
    "per_plugin_ms": 19,
    "plugin_count": 23,
    "projected_aggregate_ms": 437,
    "ac7b_ceiling_ms": 200,
    "ac7b_attainable": false
  }
}
```

### Interpretation

- **per_plugin_ms = 19**: Measured median for validate-bc-title.sh (Tier 2 representative hook)
- **plugin_count = 23**: 23 Tier 2 PostToolUse:Edit|Write plugins in scope for E-8
- **projected_aggregate_ms = 437**: 19 × 23 = 437ms projected aggregate latency
- **ac7b_ceiling_ms = 200**: Original tentative ceiling from E-8 epic OQ-8
- **ac7b_attainable = false**: 437ms > 200ms — original 10ms/plugin assumption is violated

### Finding

The assumed 10ms/plugin baseline was incorrect. The measured value (19ms/plugin) is 1.9× the estimate. The Tier 2 aggregate projection of 437ms exceeds the 200ms AC-7b ceiling, triggering the AC-3 E-8 epic fix-burst pathway.

**Result:** AC-2 SATISFIED. Aggregate Tier 2 projection computed (23 × 19ms = 437ms). Ceiling violation confirmed; fix-burst pathway triggered.
