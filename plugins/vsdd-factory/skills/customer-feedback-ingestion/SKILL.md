---
name: customer-feedback-ingestion
description: >
  Ingests customer feedback from configured channels (GitHub issues, support
  tickets, app reviews, Slack/Discord). Categorizes, deduplicates, and
  produces a structured feedback digest. Does not interact with customers.
agents:
  primary: business-analyst
  supporting: [research-agent]
inputs:
  - discovery-config.yaml (channel configuration)
  - .factory/discovery/feedback-state.yaml (last ingestion timestamps)
outputs:
  - .factory/discovery/feedback-digest-YYYY-MM-DD.md
  - .factory/discovery/feedback-state.yaml (updated)
---

# Customer Feedback Ingestion

Ingests customer feedback from configured channels, categorizes it, deduplicates
similar items, and produces a structured feedback digest. This is a read-only
process -- the factory does not interact with customers directly.

## Trigger

- **Scheduled** -- daily or as configured in `discovery-config.yaml` schedule.feedback_ingestion
- **Manual** -- human requests "Ingest customer feedback for [product]"
- **Event-driven** -- triggered by high-volume feedback spikes (if monitoring configured)

## Prerequisites

- `discovery-config.yaml` exists with `products[*].user_channels` configured
- `.factory/discovery/feedback-state.yaml` exists (or will be created on first run)
- `research-agent` is available (DF-002) for Perplexity-based review searches
- `` available for GitHub API, Slack API access

## Supported Channels

| Channel | Access Method | What It Captures |
|---------|-------------|-----------------|
| GitHub Issues | `gh` CLI / GitHub API | Feature requests, bug reports, discussions |
| GitHub Discussions | `gh` CLI / GitHub API | Community questions, feature ideas |
| Slack/Discord | Webhook archive or API | Customer messages in feedback channels |
| App Store Reviews | Perplexity search (public) | iOS/Android app reviews |
| G2/Capterra Reviews | Perplexity search (public) | Enterprise software reviews |
| Support Tickets | API integration (configurable) | Customer support issues |
| NPS/Survey Results | File import (.factory/surveys/*.csv) | Structured survey responses |

## Channel Configuration

Channels are configured per-product in `discovery-config.yaml`:

```yaml
products:
  - name: "my-product"
    user_channels:
      - type: "github-issues"
        repo: "org/product"
        labels_feature: ["feature-request", "enhancement"]
        labels_bug: ["bug", "defect"]
        since: "last-ingestion"

      - type: "github-discussions"
        repo: "org/product"
        categories: ["Ideas", "Feature Requests"]
        since: "last-ingestion"

      - type: "slack-channel"
        workspace: "company"
        channel: "#product-feedback"
        since: "last-ingestion"

      - type: "app-reviews"
        search_terms: ["my-product"]
        platforms: ["ios", "android"]
        since: "30d"

      - type: "review-sites"
        search_terms: ["my-product"]
        sites: ["g2.com", "capterra.com"]
        since: "30d"

      - type: "survey-import"
        path: ".factory/surveys/"
        format: "csv"
```

## Ingestion Workflow

### Step 1: Read Channel Config

Read `discovery-config.yaml` and load the `user_channels` array for the target
product. If no channels are configured, exit with no-op.

### Step 2: Load Ingestion State

Read `.factory/discovery/feedback-state.yaml` for last ingestion timestamps
per channel. If the file does not exist (first run), initialize with empty state.

```yaml
# .factory/discovery/feedback-state.yaml
last_run: YYYY-MM-DDTHH:MM:SSZ
channels:
  github-issues:
    last_ingested: YYYY-MM-DDTHH:MM:SSZ
    items_total: N
  github-discussions:
    last_ingested: YYYY-MM-DDTHH:MM:SSZ
    items_total: N
  # ... per channel
```

### Step 3: Fetch New Items Per Channel

For each configured channel:

**GitHub Issues:**
```bash
gh issue list --repo org/product --label feature-request --state open \
  --json number,title,body,labels,createdAt,author,comments \
  --limit 100
```
Filter to items created/updated since `last_ingested`.

**GitHub Discussions:**
```bash
gh api graphql -f query='{ repository(owner:"org", name:"product") {
  discussions(first:50, categoryId:"...") { nodes { title body createdAt } }
}}'
```

**Slack/Discord:**
- Via : fetch messages from configured channel since last ingestion
- Parse for feature requests, complaints, praise

**App Store Reviews:**
- Via research-agent -> Perplexity: "[product name] app reviews [platform] [month year]"
- Extract individual reviews with ratings and text

**Review Sites (G2/Capterra):**
- Via research-agent -> Perplexity: "[product name] reviews [site] [month year]"
- Extract review summaries with ratings

**Survey Import:**
- Read CSV files from `.factory/surveys/` directory
- Expected columns: `date`, `respondent_id`, `type` (NPS/CSAT/custom), `score`, `comment`
- Process new files not previously ingested

### Step 4: Categorize Each Item

Assign each ingested item to one category:

| Category | Signal Words / Patterns | Priority |
|----------|------------------------|----------|
| Feature Request | "would be great if", "please add", "feature request", enhancement labels | HIGH |
| Bug Report | "broken", "doesn't work", "error", "crash", bug labels | HIGH |
| Pain Point | "frustrated", "difficult", "can't figure out", "workaround" | MEDIUM |
| Praise | "love", "great", "amazing", "thank you", 4-5 star reviews | LOW |
| Question | "how do I", "is it possible", "documentation", "?" | MEDIUM |

Categorization uses the business-analyst agent's judgment, not just keyword matching.
Context and tone matter.

### Step 5: Deduplicate

Cluster similar items to avoid inflated counts:

1. Group items by category
2. Within each category, compute semantic similarity
3. Cluster items with >0.80 similarity into a single entry
4. Track count per cluster (frequency signal)
5. Preserve the most detailed/articulate item as the cluster representative

### Step 6: Produce Feedback Digest

Write `.factory/discovery/feedback-digest-YYYY-MM-DD.md`:

```markdown
---
document_type: feedback-digest
date: YYYY-MM-DD
product: [product-name]
channels_ingested: [list]
total_items: [N]
new_since_last: [N]
---

# Feedback Digest: YYYY-MM-DD

## Summary
| Category | Count | Trend vs Last Period |
|----------|-------|---------------------|
| Feature Requests | [N] | up/down/flat |
| Bug Reports | [N] | up/down/flat |
| Pain Points | [N] | up/down/flat |
| Praise | [N] | up/down/flat |
| Questions | [N] | up/down/flat |

## Top Feature Requests (by frequency)
| Rank | Request | Sources | Count | First Seen | Trend |
|------|---------|---------|-------|-----------|-------|
| 1 | [request] | GH, Slack, G2 | [N] | [date] | up |

## Top Pain Points
| Rank | Pain | Sources | Count | Severity |
|------|------|---------|-------|---------|
| 1 | [pain] | [sources] | [N] | HIGH/MED/LOW |

## Emerging Signals (new this period)
[Items not seen in any previous digest]

## Raw Items
[Appendix: all ingested items with source, date, category, text]
```

### Step 7: Update State

Update `.factory/discovery/feedback-state.yaml` with:
- Current timestamp per channel
- Cumulative item counts
- Run metadata

## Quality Gate

- [ ] All configured channels scanned (failures logged, not blocking)
- [ ] Feedback items categorized with rationale (not keyword-only)
- [ ] Deduplication applied -- similar items clustered, no inflated counts
- [ ] Feedback digest written to `.factory/discovery/feedback-digest-YYYY-MM-DD.md`

## Quality Criteria

- [ ] All configured channels attempted (failures logged, not blocking)
- [ ] Items categorized with rationale (not just keyword matching)
- [ ] Deduplication clusters similar items (no inflated counts)
- [ ] Frequency counts are accurate (cluster size, not raw count)
- [ ] Trends compare against previous digest (if available)
- [ ] State file updated (no re-processing on next run)
- [ ] Raw items preserved in appendix for traceability
