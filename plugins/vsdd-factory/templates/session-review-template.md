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

## 9. Governance Policy Audit
Review whether any drift patterns observed in this run warrant a new governance policy or strengthen an existing one.

**Existing policies to verify enforcement:**
- `append_only_numbering` — Were any IDs renumbered, reused, or filename slugs changed?
- `lift_invariants_to_bcs` — Are there orphan domain invariants with no enforcing BC?
- `state_manager_runs_last` — Did state-manager always commit last in every burst?
- `semantic_anchoring_integrity` — Were any anchors syntactically valid but semantically wrong?
- `creators_justify_anchors` — Did creator agents justify every anchor choice against source-of-truth?
- `architecture_is_subsystem_name_source_of_truth` — Did any subsystem references use non-registry names?
- `bc_h1_is_title_source_of_truth` — Did any BC references use stale or enriched-only titles?
- `bc_array_changes_propagate_to_body_and_acs` — Did any story frontmatter `bcs:` changes fail to propagate to body BC tables and ACs?
- `vp_index_is_vp_catalog_source_of_truth` — Did any VP-INDEX changes fail to propagate to verification-architecture.md or verification-coverage-matrix.md?

**New policy candidates:** If a specific class of drift recurred across 3+ bursts or 2+ adversarial passes, it is a policy candidate. Document: the drift pattern, incident examples, which agents failed to enforce, and the proposed policy statement.

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
