---
document_type: session-review
date: YYYY-MM-DD
run_id: [unique run ID from STATE.md]
path: [1-11]
path_name: [greenfield | brownfield | feature | bug-fix | planning | discovery | maintenance | multi-repo]
product: [product name]
duration: [total wall clock time]
total_cost: [$X.XX]
stories_delivered: [N]
---

# Session Review: [product] — [path_name] — [date]

## Executive Summary
[2-3 sentences: what went well, what didn't, top recommendation]

## Run Overview
| Metric | Value | Benchmark | Status |
|--------|-------|-----------|--------|
| Total cost | $X.XX | $Y.YY (avg for this path) | / / |
| Duration | Xh Ym | Yh Zm (avg) | / / |
| Stories delivered | N | — | — |
| Adversarial rounds | N (avg per phase) | M (avg) | / / |
| PR review rounds | N (avg per story) | M (avg) | / / |
| Gate failures | N | M (avg) | / / |
| Human interventions | N | M (avg) | / / |
| Holdout satisfaction | 0.XX | >=0.85 | / / |
| Mutation kill rate | XX% | >=90% | / / |

---

## 1. Cost Analysis
[Findings + recommendations]

## 2. Timing Analysis
[Findings + recommendations]

## 3. Convergence Analysis
[Findings + recommendations]

## 4. Agent Behavior Analysis
[Findings + recommendations]

## 5. Gate Outcome Analysis
[Findings + recommendations]

## 6. Wall Integrity Analysis
[Findings + recommendations]

## 7. Quality Signal Analysis
[Findings + recommendations]

## 8. Pattern Detection
[Findings from cross-run comparison]

---

## Improvement Proposals

### Proposal 1: [Title]
- **Category:** [cost | timing | convergence | agent | gate | wall | quality | pattern | workflow | template]
- **Priority:** [HIGH | MEDIUM | LOW]
- **Evidence:** [specific data from this run]
- **Recommendation:** [specific change]
- **Affected files:** [which factory files would change]
- **Risk:** [what could go wrong if implemented]

### Proposal 2: [Title]
...

---

## Metrics for Next Run
[Specific things to measure in the next run to validate improvements]
