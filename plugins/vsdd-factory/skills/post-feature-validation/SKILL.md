---
name: post-feature-validation
description: >
  After a feature ships, monitors feedback channels and analytics for signals
  about the feature's reception. Runs at configured intervals (e.g., 7 days,
  30 days, 90 days post-ship). Produces a feature impact report.
agents:
  primary: business-analyst
  supporting: [research-agent]
inputs:
  - Feature description (from the story that shipped)
  - discovery-config.yaml (channels)
  - Usage analytics (if available)
outputs:
  - .factory/discovery/feature-impact-[feature-name]-YYYY-MM-DD.md
---

# Post-Feature Validation

After a feature ships (via Feature Mode Path 3), monitors feedback channels and
analytics for signals about the feature's reception. This closes the feedback
loop: Build -> Deploy -> Customers -> Feedback -> Analysis -> Next Iteration.

Runs at configurable intervals after a feature ships (default: 7, 30, 90 days).
Entirely optional -- no-op if not configured.

## Trigger

- **Scheduled** -- at configured intervals after feature ships (7d, 30d, 90d)
- **Manual** -- human requests "Check how [feature] is doing"

## Prerequisites

- Feature has been released (release step complete in feature.lobster)
- `discovery-config.yaml` exists with channel configuration
- Feature description available from the story/cycle that shipped
- Optional: analytics configured for adoption tracking

## Configuration

Configured in `discovery-config.yaml` or per-product:

```yaml
post_feature_validation:
  enabled: true
  check_intervals: [7, 30, 90]    # Days after ship
  success_criteria:
    adoption_rate: 0.10            # 10% of active users
    positive_ratio: 0.6            # 60% positive feedback
```

## Procedure

### Step 1: Identify Feature Context

Read the feature cycle that shipped:
- Feature name and description from the cycle manifest
- Ship date from the release step
- Target users / use case from the feature brief
- Success criteria from configuration

### Step 2: Calculate Days Since Ship

Determine which check interval this run corresponds to:
- 7-day check: early adoption signal
- 30-day check: sustained adoption
- 90-day check: long-term retention and impact

### Step 3: Check Feedback Channels

Search configured feedback channels for mentions of the shipped feature:

**GitHub Issues:**
```bash
gh issue list --repo org/product --search "[feature name]" \
  --json number,title,body,labels,createdAt \
  --created ">SHIP_DATE"
```
Categorize as positive, negative, or neutral.

**GitHub Discussions:**
Search for discussions mentioning the feature since ship date.

**App Reviews / Review Sites:**
Via research-agent -> Perplexity: "[product] [feature name] review"
Look for mentions of the specific feature in recent reviews.

**Slack/Discord:**
Search configured feedback channels for feature-related messages.

### Step 4: Check Analytics (if available)

If analytics sources are configured:
- Feature-specific DAU (daily active users of the feature)
- Adoption rate: feature users / total active users
- Retention: users still using feature after 7 days
- Error rate for the feature

### Step 5: Assess Impact

Compare actual results against success criteria:

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| Adoption rate | >= configured threshold | [actual %] | PASS/FAIL |
| Positive feedback ratio | >= configured threshold | [actual ratio] | PASS/FAIL |
| Error rate | < 5% | [actual %] | PASS/FAIL |
| Bug reports | decreasing or stable | [trend] | PASS/FAIL |

### Step 6: Determine Verdict

| Verdict | Criteria |
|---------|---------|
| SUCCESS | Adoption meets target AND positive ratio meets target AND no critical bugs |
| PARTIAL | Adoption meets target but feedback mixed, OR good feedback but low adoption |
| MISS | Adoption below target AND/OR negative feedback dominant AND/OR critical bugs |

### Step 7: Generate Recommendations

Based on verdict:

**SUCCESS:**
- Continue monitoring at next interval
- Consider promoting to highlight feature in marketing/docs
- Log success pattern for future feature development

**PARTIAL:**
- Investigate specific issues (low adoption? UX problem? discoverability?)
- Consider iteration: minor improvements to address feedback
- Feed specific pain points back to discovery engine

**MISS:**
- Root cause analysis: was the feature solving the wrong problem?
- Consider: iterate, redesign, or deprecate
- Feed learnings back to discovery engine for future scoring calibration

### Step 8: Produce Feature Impact Report

Write `.factory/discovery/feature-impact-[feature-name]-YYYY-MM-DD.md`:

```markdown
---
document_type: feature-impact
feature: [feature-name]
shipped_date: YYYY-MM-DD
report_date: YYYY-MM-DD
days_since_ship: [N]
check_interval: [7 | 30 | 90]
---

# Feature Impact: [feature-name]

## Adoption
| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Users using feature | [N] | [target] | PASS/WARN/FAIL |
| Adoption rate | [%] | [target] | PASS/WARN/FAIL |
| Retention (still using after 7d) | [%] | [target] | PASS/WARN/FAIL |

## Feedback
| Channel | Positive | Negative | Neutral |
|---------|---------|---------|---------|
| GitHub | [N] | [N] | [N] |
| Reviews | [N] | [N] | [N] |
| Slack | [N] | [N] | [N] |

## Issues
| Issue | Count | Severity | Action |
|-------|-------|---------|--------|
| [issue] | [N] | HIGH/MED/LOW | [fix/monitor/ignore] |

## Verdict: [SUCCESS / PARTIAL / MISS]
[Assessment of whether the feature solved the original problem]

## Recommendations
- [Follow-up action 1]
- [Follow-up action 2]

## Feed Back to Discovery Engine
[Signals to feed into the next synthesis cycle:
 - calibration data for scoring accuracy
 - new pain points or feature requests from feedback
 - evidence for/against related ideas in the backlog]
```

## Feeding Back Into Discovery

The feature impact report feeds back into the discovery engine:

1. **Scoring calibration:** If features with high discovery scores consistently
   MISS, the scoring weights need adjustment
2. **New signals:** Pain points and feature requests from the impact report
   become inputs to the next feedback ingestion cycle
3. **Evidence for backlog items:** If users request feature X after feature Y
   ships, that strengthens the evidence for feature X in the backlog

## Quality Gate

- [ ] Only runs when post_feature_validation.enabled == true
- [ ] Check interval matches configured schedule (7d, 30d, 90d)
- [ ] Feedback channels searched for feature-specific signals
- [ ] Analytics checked for adoption metrics (if available)
- [ ] Verdict is evidence-based (SUCCESS/PARTIAL/MISS with rationale)
- [ ] Recommendations are actionable (not just "do better")
- [ ] Results feed back into discovery engine for next cycle
- [ ] Entirely optional -- no-op if not configured

## Failure Modes

- If feedback channels unavailable: analyze available data and note which channels were unreachable
- If no analytics configured: skip adoption metrics, assess based on qualitative feedback only
- If feature cannot be identified in feedback: produce report noting insufficient signal, recommend manual review
