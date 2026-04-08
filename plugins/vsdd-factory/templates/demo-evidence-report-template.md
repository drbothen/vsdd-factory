---
document_type: demo-evidence-report
product: "{product_name}"
pipeline_run: "{timestamp}"
demo_type: "{cli | web-ui | web-api | library | composite}"
recording_tool: "{vhs | playwright | curl | composite}"
status: pending
---

# Demo Evidence Report

## Product: {product_name}
## Pipeline Run: {timestamp}
## Demo Type: {demo_type}

---

## Per-AC Demo Recordings

| AC | Story | Description | Recording | Format | Duration | Size | Status |
|----|-------|-------------|-----------|--------|----------|------|--------|
| {ac_id} | {story_id} | {ac_description} | [{filename}]({path}) | {webm/gif} | {duration} | {size} | {recorded/failed/skipped} |

---

## Full User Journey Demo

| Demo | Description | Recording | Duration | Size |
|------|-------------|-----------|----------|------|
| Happy path | Complete user workflow from start to finish | [{filename}]({path}) | {duration} | {size} |

---

## Holdout Scenario Demos

| Scenario | Category | Satisfaction | Recording | Duration |
|----------|----------|-------------|-----------|----------|
| {hs_id} | {category} | {score} | [{filename}]({path}) | {duration} |

---

## Visual Review Summary

| Demo | AC | Visual Satisfaction | Findings | Regression? |
|------|-----|-------------------|----------|-------------|
| {filename} | {ac_id} | {score} | {findings} | {N/A | Yes | No} |

---

## Regression Comparison (Feature Mode)

| Feature | Previous Demo | Current Demo | Visual Diff |
|---------|--------------|-------------|-------------|
| {feature} | [{prev}]({prev_path}) | [{curr}]({curr_path}) | {same/changed} |

---

## Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | {version} | {installed/missing} |
| Playwright | {version} | {installed/missing} |
| ffmpeg | {version} | {installed/missing} |
| asciinema | {version} | {installed/missing/optional} |

---

## PR Embedding Snippet

```markdown
{auto-generated markdown snippet for PR description}
```

---

## Notes

- WebM is the primary format (best compression, GitHub supports playback)
- GIF fallback for inline embedding in PR descriptions and READMEs
- Playwright traces (.zip) available for interactive debugging
- All recordings optimized with ffmpeg (target: <2MB WebM, <5MB GIF)
