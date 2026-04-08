---
name: competitive-monitoring
description: >
  Monitors competitor activity: new releases, feature announcements, pricing
  changes, funding rounds, acquisitions. Produces a competitive update
  report that feeds into the synthesis layer.
agents:
  primary: research-agent
  supporting: []
inputs:
  - discovery-config.yaml (competitors list)
  - .factory/discovery/competitive-baseline.md (last known state)
outputs:
  - .factory/discovery/competitive-update-YYYY-MM-DD.md
  - .factory/discovery/competitive-baseline.md (updated)
---

# Competitive Monitoring

Monitors competitor activity continuously between discovery runs. Detects
changes in the competitive landscape -- new releases, pricing shifts, funding,
acquisitions, shutdowns, and new entrants -- and produces a structured
competitive update report.

## Trigger

- **Scheduled** -- weekly or as configured in `discovery-config.yaml` schedule.competitive_monitoring
- **Manual** -- human requests "Check competitor activity for [product]"
- **Event-driven** -- triggered by news alerts or discovery engine

## Prerequisites

- `discovery-config.yaml` exists with `products[*].competitors` configured
- `.factory/discovery/competitive-baseline.md` exists (or will be created on first run)
- `research-agent` is available (DF-002) with Perplexity MCP access
- `` available for delegated research calls

## Monitoring Targets

For each competitor in `discovery-config.yaml`:

| Signal | How Detected | Urgency |
|--------|-------------|---------|
| New feature/release | Perplexity: "[competitor] release notes [month year]" | HIGH if overlaps with our roadmap |
| Pricing change | Perplexity: "[competitor] pricing" vs baseline | MEDIUM |
| Funding round | Perplexity: "[competitor] funding" | LOW (info only) |
| Acquisition | Perplexity: "[competitor] acquired" | HIGH (market shift) |
| Shutdown/pivot | Perplexity: "[competitor] shutdown OR pivot" | HIGH (opportunity) |
| New competitor | Perplexity: "[domain] new startup 2026" | MEDIUM |

## Monitoring Workflow

### Step 1: Load Competitor List

Read `discovery-config.yaml` and extract the `competitors` array for the target
product. If no competitors are configured, exit with no-op.

### Step 2: Load Competitive Baseline

Read `.factory/discovery/competitive-baseline.md` for the last known state of
each competitor. If the file does not exist (first run), the monitoring run
establishes the baseline.

```markdown
# Competitive Baseline

## Last Updated: YYYY-MM-DD

### Competitor A
- **Latest Known Release:** vX.Y.Z (YYYY-MM-DD)
- **Key Features:** [list]
- **Pricing:** [current pricing model]
- **Funding Status:** [last known round]
- **Status:** active

### Competitor B
- ...
```

### Step 3: Research Each Competitor

For each competitor, spawn research-agent to run Perplexity queries:

1. **Release check:** "[competitor] release notes [current month] [current year]"
   - Compare against baseline's `Latest Known Release`
   - If new release detected: extract version, date, key features

2. **Pricing check:** "[competitor] pricing [current year]"
   - Compare against baseline's `Pricing`
   - Flag changes in pricing model, tiers, or pricing levels

3. **Funding check:** "[competitor] funding round [current year]"
   - Compare against baseline's `Funding Status`
   - Note amount, investors, valuation if available

4. **Acquisition check:** "[competitor] acquired OR acquisition [current year]"
   - Detect if competitor was acquired or acquired another company

5. **Shutdown/pivot check:** "[competitor] shutdown OR pivot OR closing [current year]"
   - Detect if competitor is shutting down or pivoting strategy

### Step 4: Scan for New Entrants

Research the product's domain for new competitors:
- Perplexity: "[product domain] new startup [current year]"
- Perplexity: "[product domain] new tool launch [current year]"
- Compare against known competitor list

### Step 5: Classify Urgency

For each detected change, assign urgency:

| Urgency | Criteria |
|---------|---------|
| HIGH | Competitor ships feature our customers are requesting; competitor acquired; competitor shuts down (opportunity) |
| MEDIUM | Pricing change; new entrant in our space; funding round (signals acceleration) |
| LOW | Minor release; informational funding news; no direct impact on our roadmap |

### Step 6: Produce Competitive Update

Write `.factory/discovery/competitive-update-YYYY-MM-DD.md`:

```markdown
---
document_type: competitive-update
date: YYYY-MM-DD
product: [product-name]
competitors_monitored: [N]
changes_detected: [N]
---

# Competitive Update: YYYY-MM-DD

## Changes Detected

| Competitor | Change | Date | Urgency | Implication |
|-----------|--------|------|---------|------------|
| [name] | [change] | [date] | HIGH/MED/LOW | [what it means for us] |

## No Change (stable competitors)
[List competitors with no detected changes]

## New Entrants
[Any new competitors detected]
```

### Step 7: Update Baseline

Update `.factory/discovery/competitive-baseline.md` with any newly detected
state changes. Preserve history: append a `## Change Log` section at the bottom
of each competitor entry.

## Quality Gate

- [ ] All configured competitors researched (none skipped without explanation)
- [ ] All findings dated with source URLs
- [ ] Urgency classification applied to every detected change
- [ ] Competitive baseline updated with new state

## Failure Modes

- If a data source is unavailable (Perplexity timeout, API error): flag the source as UNAVAILABLE, continue with remaining sources, note gap in report
- If no changes detected for any competitor: produce a "no change" report (not a silent no-op)
- If a new entrant cannot be verified: mark as UNVERIFIED and include in report with caveat

## Quality Criteria

- [ ] All configured competitors researched
- [ ] Changes compared against baseline (not just raw search results)
- [ ] Urgency classification applied with rationale
- [ ] Implications assessed (not just "competitor shipped X" but "this means Y for us")
- [ ] New entrant scan covers the product domain
- [ ] Baseline updated with detected changes
- [ ] Sources cited for each finding with URLs and dates
- [ ] VERIFIED/UNVERIFIED flags on findings (same as DF-017 research quality controls)
