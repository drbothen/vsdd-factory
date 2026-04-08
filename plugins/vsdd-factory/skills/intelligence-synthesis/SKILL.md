---
name: intelligence-synthesis
description: >
  Correlates signals across market research, customer feedback, and usage
  analytics to produce scored insights. Clusters related signals into
  themes, calculates evidence strength, and feeds into DF-017's scoring
  framework.
agents:
  primary: business-analyst
  supporting: [research-agent]
inputs:
  - .factory/discovery/feedback-digest-*.md (latest)
  - .factory/discovery/competitive-update-*.md (latest)
  - .factory/discovery/analytics-digest-*.md (latest, if exists)
  - .factory/discovery/product-research-*.md (latest DF-017 scan)
outputs:
  - .factory/discovery/insights-YYYY-MM-DD.md
---

# Intelligence Synthesis

Correlates signals across all intelligence sources -- market research, customer
feedback, competitive monitoring, and usage analytics -- to produce scored
insights. This is the layer that turns raw signals into actionable intelligence
by extracting themes, calculating evidence strength, and formatting insights
for DF-017's scoring framework.

## Trigger

- **Scheduled** -- weekly or as configured in `discovery-config.yaml` schedule.full_synthesis
- **Manual** -- human requests "Synthesize intelligence for [product]"
- **Automatic** -- runs after all ingestion steps complete in discovery.lobster

## Prerequisites

- At least one input source must exist (market research at minimum)
- Business-analyst agent available
- Previous ingestion steps completed (or their outputs exist from prior runs)

## Input Sources

| Source | File Pattern | Required? | Producer |
|--------|-------------|-----------|---------|
| Market Research | `.factory/discovery/feature-research-*-YYYY-MM-DD.md` | YES | DF-017 discovery-engine |
| Customer Feedback | `.factory/discovery/feedback-digest-YYYY-MM-DD.md` | NO | customer-feedback-ingestion |
| Competitive Update | `.factory/discovery/competitive-update-YYYY-MM-DD.md` | NO | competitive-monitoring |
| Analytics Digest | `.factory/discovery/analytics-digest-YYYY-MM-DD.md` | NO | analytics-integration |

The synthesis layer works with partial data. If only market research exists,
it still produces insights (with lower evidence strength scores). As more
sources come online, evidence strength increases.

## Synthesis Workflow

### Step 1: Collect Latest Inputs

Read the most recent file matching each input pattern. Track which sources
are available for evidence strength scoring.

```yaml
sources_available:
  market_research: true    # always required
  customer_feedback: true  # if digest exists
  competitive_intel: true  # if update exists
  usage_analytics: false   # if digest exists
source_count: 3            # out of 4 possible
```

### Step 2: Extract Signals

From each source, extract individual signals:

**From Market Research:**
- Competitive features shipped
- Technology opportunities
- Industry trend shifts
- User signal patterns

**From Customer Feedback:**
- Top feature requests (by frequency)
- Top pain points (by severity)
- Emerging signals (new this period)

**From Competitive Update:**
- Competitor releases overlapping our roadmap
- Pricing changes affecting our positioning
- New entrants in our space

**From Analytics:**
- Features with declining adoption
- Error hotspots
- Funnel drop-off points
- Unused features

### Step 3: Theme Extraction

Cluster related signals across sources into themes:

1. **Semantic clustering:** Group signals that refer to the same concept
   - Example: "dark mode" request (GitHub) + competitor ships dark mode
     (competitive) + 40% users on dark OS themes (analytics) = one theme

2. **Cross-source correlation:** A signal appearing in multiple sources
   is stronger than a single-source signal

3. **Theme naming:** Each cluster gets a descriptive theme title

4. **Signal mapping:** Each theme lists which signals from which sources
   contribute to it

### Step 4: Evidence Strength Scoring

Score each theme's evidence strength based on source diversity and quality:

| Evidence Level | Sources | Score Range |
|---------------|---------|-------------|
| Market scan only | Perplexity research | 0.3 - 0.5 |
| Market + one customer signal | Research + feedback OR analytics | 0.5 - 0.6 |
| Market + multiple customer signals | Research + feedback + analytics | 0.6 - 0.8 |
| Market + customer + competitive | Research + feedback + competitive | 0.7 - 0.8 |
| All three customer sources | Feedback + competitive + analytics | 0.8 - 0.9 |
| All sources + revenue impact | All + payment/revenue data | 0.9 - 1.0 |

Within each range, score higher when:
- More independent signals confirm the theme
- Signals are recent (within 30 days)
- Signals come from diverse channel types (not just one GitHub repo)

### Step 5: Priority Signal Analysis

For each theme, assess priority dimensions:

| Dimension | Question | Weight |
|-----------|---------|--------|
| Pain frequency | How many independent reports? | HIGH |
| Revenue signal | Does this affect paying customers? | HIGH |
| Competitive urgency | Did a competitor just ship this? | HIGH |
| Trend direction | Is the signal growing or fading? | MEDIUM |
| User impact | How many users affected (from analytics)? | MEDIUM |
| Effort estimate | Quick win or major investment? | LOW (informational) |

### Step 6: Score Insights on 7 Dimensions

For each theme that merits an insight, apply DF-017's scoring framework
with the new evidence_strength dimension:

| Dimension | Weight | Description |
|-----------|--------|------------|
| Value | 0.25 | How much value does this create for users? |
| Feasibility | 0.15 | Can we build this with current tech/team? |
| Alignment | 0.15 | Does this fit our product vision? |
| Novelty | 0.10 | Is this a new capability (not incremental)? |
| Time-Criticality | 0.10 | Is there a window of opportunity? |
| Effort | 0.10 | How much work to implement? (inverse) |
| Evidence Strength | 0.15 | How many independent sources confirm this? |

### Step 7: Determine Recommended Action

Based on composite scores and evidence strength:

| Composite | Evidence Strength | Recommended Action |
|-----------|------------------|-------------------|
| >= 0.7 | >= 0.6 | Brief -> Planning (auto-generate brief) |
| 0.5 - 0.7 | any | Backlog (resurface in next report) |
| any | 0.4 - 0.6 | Backlog (may promote as evidence grows) |
| < 0.5 | < 0.4 | Registry (log for future, re-evaluate) |
| any (competitive HIGH) | >= 0.7 | URGENT -> Immediate human notification |

### Step 8: Produce Insights Report

Write `.factory/discovery/insights-YYYY-MM-DD.md`:

```markdown
---
document_type: intelligence-insights
date: YYYY-MM-DD
product: [product-name]
insights_count: [N]
sources_available: [N of 4]
---

# Intelligence Insights: YYYY-MM-DD

## Source Coverage
| Source | Available | Freshness |
|--------|----------|-----------|
| Market Research | YES/NO | [date] |
| Customer Feedback | YES/NO | [date] |
| Competitive Intel | YES/NO | [date] |
| Usage Analytics | YES/NO | [date] |

## Insight 1: [Theme Title]

### Signal Correlation
| Source | Signal | Date | Strength |
|--------|--------|------|----------|
| Customer Feedback | [signal] | [date] | strong/moderate/weak |
| Competitive | [signal] | [date] | strong/moderate/weak |
| Analytics | [signal] | [date] | strong/moderate/weak |
| Market Research | [signal] | [date] | strong/moderate/weak |

### Scores
| Dimension | Score | Rationale |
|-----------|-------|-----------|
| Value | [0.XX] | [why] |
| Feasibility | [0.XX] | [why] |
| Alignment | [0.XX] | [why] |
| Novelty | [0.XX] | [why] |
| Time-Criticality | [0.XX] | [why] |
| Effort | [0.XX] | [why] |
| Evidence Strength | [0.XX] | [source count + quality] |
| **Composite** | **[0.XX]** | |

### Recommended Action
[Brief -> Planning | Backlog | Registry | Monitor | URGENT]

## Insight 2: [Theme Title]
...
```

## Working with Partial Data

The synthesis layer degrades gracefully:

- **Market research only:** Insights generated with evidence_strength 0.3-0.5.
  All themes noted as "unvalidated by customer data."
- **Market + feedback:** Evidence strength up to 0.6-0.7. Customer feedback
  confirms or contradicts market research findings.
- **Market + competitive:** Evidence strength 0.5-0.7. Competitive pressure
  validates urgency but lacks customer confirmation.
- **All sources:** Full evidence scoring up to 0.9-1.0.

## Quality Gate

- [ ] All available signal sources collected and correlated
- [ ] Evidence strength scored for every theme based on source diversity
- [ ] Themes clustered from cross-source signals (not just listed per-source)
- [ ] Insights report written to `.factory/discovery/insights-YYYY-MM-DD.md`

## Quality Criteria

- [ ] All available sources collected and referenced
- [ ] Theme extraction clusters related signals (not just lists)
- [ ] Evidence strength scored based on source count and quality
- [ ] Each insight scored on 7 dimensions (6 existing + evidence_strength)
- [ ] Recommended actions match the routing rules in discovery-engine SKILL.md
- [ ] Works with partial data (some sources may not be configured)
- [ ] Cross-source correlations identified and documented
- [ ] URGENT insights flagged for immediate notification
