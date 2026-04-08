---
document_type: holdout-evaluation-report
level: ops
version: "1.0"
status: draft
producer: holdout-evaluator
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 3.5|4
inputs: []
input-hash: "[md5]"
traces_to: ""
total_scenarios: 0
must_pass_scenarios: 0
must_pass_rate: 0.0
mean_satisfaction: 0.0
satisfaction_std_dev: 0.0
---

# Holdout Evaluation Report: [Project Name]

## Overall Metrics
| Metric | Value | Gate Threshold |
|--------|-------|---------------|
| Total scenarios | | |
| Must-pass scenarios | | |
| Mean satisfaction score | | >= 0.85 |
| Satisfaction std deviation | | < 0.15 |
| Must-pass minimum score | | >= 0.6 |
| Verdict | PASS / FAIL | |

## Per-Scenario Scores

| Scenario | Category | Priority | Satisfaction | Confidence | Revalidated? | Run Details |
|----------|----------|----------|-------------|------------|-------------|-------------|
| HS-001 | | must-pass / should-pass | 0.000-1.000 | 0.00-1.00 | yes / carried-forward | |

## Low-Satisfaction Scenarios (score < 0.85)
### HS-NNN: [title]
- **Score:** [N]
- **Root cause:** [why it failed]
- **Fix applied:** [what was done]
- **Revalidation result:** [new score]

## Evidence Summary
- Test files executed: [list]
- Command: [exact command run]
- Result: [pass/fail counts]

## Final Verdict
[PASS / FAIL]
