---
name: analytics-integration
description: >
  Reads product analytics data (if available) to identify feature adoption,
  error patterns, and usage signals. Optional -- only runs if analytics
  sources are configured. Does not implement telemetry in the product.
agents:
  primary: business-analyst
  supporting: [research-agent]
inputs:
  - discovery-config.yaml (analytics configuration)
  - Analytics data files or API endpoints
outputs:
  - .factory/discovery/analytics-digest-YYYY-MM-DD.md
---

# Analytics Integration

Reads product analytics data to identify feature adoption rates, error patterns,
and usage signals that inform what to build next. This skill is entirely
optional -- it only runs when analytics sources are configured. The factory
does not implement telemetry in the product; it reads analytics output that the
product already produces.

## Trigger

- **Scheduled** -- weekly or as configured in `discovery-config.yaml` schedule.analytics_integration
- **Manual** -- human requests "Analyze usage data for [product]"

## Prerequisites

- `discovery-config.yaml` exists with `products[*].analytics.enabled == true`
- Analytics data files exist at configured paths, OR API endpoints are reachable
- `` available for API calls (if API sources configured)

## Supported Analytics Sources

| Source | Access Method | What It Provides |
|--------|-------------|-----------------|
| Analytics export (CSV/JSON) | File read from .factory/analytics/ | Feature usage, user counts, funnels |
| Error tracking export | File read from .factory/analytics/ | Error rates by feature, crash reports |
| API endpoint (configurable) |  HTTP call | Live metrics (if API exists) |

## Analytics Configuration

Configured per-product in `discovery-config.yaml`:

```yaml
products:
  - name: "my-product"
    analytics:
      enabled: true
      sources:
        - type: "export-file"
          path: ".factory/analytics/usage-export.csv"
          format: "csv"
          metrics: ["feature_usage", "daily_active_users", "retention"]

        - type: "export-file"
          path: ".factory/analytics/errors-export.json"
          format: "json"
          metrics: ["error_rate_by_feature", "crash_count"]

        - type: "api"
          endpoint: "https://analytics.internal/api/v1/summary"
          auth: "env:ANALYTICS_API_KEY"
          metrics: ["funnel_completion", "feature_adoption"]
```

## Procedure

### Step 1: Load Analytics Config

Read `discovery-config.yaml` and check if `analytics.enabled == true` for the
target product. If not enabled, exit with no-op.

### Step 2: Collect Data from Sources

For each configured source:

**Export Files (CSV):**
- Read the CSV file from the configured path
- Parse columns based on configured metrics
- Handle missing files gracefully (log warning, skip)

**Export Files (JSON):**
- Read the JSON file from the configured path
- Extract metrics matching the configured keys
- Handle missing files gracefully (log warning, skip)

**API Endpoints:**
- Via : HTTP GET to the configured endpoint
- Include auth header from environment variable (if configured)
- Parse response JSON for configured metrics
- Handle unreachable endpoints gracefully (log warning, skip)

### Step 3: Analyze Feature Adoption

For each feature in the usage data:
- Calculate Daily Active Users (DAU)
- Calculate adoption rate (% of total active users)
- Determine trend (compare to previous period if data available)
- Classify health:
  - **Healthy:** adoption > 20% and stable or growing
  - **Concerning:** adoption declining or < 10% after 30+ days
  - **Unused:** adoption < 2% after 30+ days (candidate for deprecation or redesign)

### Step 4: Analyze Error Hotspots

For each feature with error data:
- Calculate error rate (errors / total usage)
- Determine trend (compare to previous period)
- Classify severity:
  - **HIGH:** error rate > 5% or rising trend
  - **MEDIUM:** error rate 1-5% and stable
  - **LOW:** error rate < 1%

### Step 5: Analyze User Journeys (if funnel data available)

For each funnel step:
- Calculate completion percentage
- Identify largest drop-off points
- Flag steps where completion < 50% as potential UX issues

### Step 6: Extract Key Signals

Synthesize the analysis into actionable signals:
- Features with high adoption but high errors = fix urgently
- Features with low adoption = investigate why (UX? discoverability? need?)
- Rising error trends = potential regression or scaling issue
- Funnel drop-offs = UX improvement opportunities

### Step 7: Produce Analytics Digest

Write `.factory/discovery/analytics-digest-YYYY-MM-DD.md`:

```markdown
---
document_type: analytics-digest
date: YYYY-MM-DD
product: [product-name]
period: [last 7/30 days]
sources_read: [N of M configured]
---

# Analytics Digest: YYYY-MM-DD

## Feature Adoption
| Feature | DAU | Adoption % | Trend | Signal |
|---------|-----|-----------|-------|--------|
| [feature] | [N] | [%] | up/down/flat | [healthy/concerning/unused] |

## Error Hotspots
| Feature | Error Rate | Trend | Severity |
|---------|-----------|-------|---------|
| [feature] | [%] | up/down/flat | HIGH/MED/LOW |

## User Journey Drop-offs
| Funnel Step | Completion % | Drop-off Point |
|------------|-------------|---------------|
| [step] | [%] | [where users leave] |

## Key Signals
- [Signal 1: interpretation and recommended action]
- [Signal 2: interpretation and recommended action]
```

## Graceful Degradation

This skill handles missing or partial data gracefully:

- **No analytics configured:** Exit with no-op, log info message
- **Some sources unavailable:** Process available sources, note gaps in digest
- **Stale data:** Flag data older than configured period as potentially stale
- **Empty data files:** Log warning, skip source, note in digest

## Quality Gate

- [ ] Only runs when analytics.enabled == true (no-op otherwise)
- [ ] Does NOT implement telemetry in the product
- [ ] Reads from configured sources only (no undocumented data access)
- [ ] Feature adoption rates calculated with trends
- [ ] Error rates tracked per feature with severity classification
- [ ] Key signals synthesized (not just raw numbers)
- [ ] Missing/stale data handled gracefully with warnings
- [ ] Digest includes source metadata (which sources were read, data freshness)

## Failure Modes

- If analytics sources unavailable: report which sources could not be reached and skip them
- If all sources unavailable: produce empty digest noting no data available, exit gracefully
- If data format unrecognized: log warning, skip source, note gap in digest
