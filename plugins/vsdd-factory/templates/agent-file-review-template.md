---
document_type: agent-file-review
level: ops
version: "1.0"
status: draft
producer: orchestrator
timestamp: YYYY-MM-DDTHH:MM:SS
phase: ops
inputs: [agents/]
traces_to: ""
---

# Agent File Review: [agent-name]

**File:** [path]
**Words:** [count] ([PASS/WARN/FAIL])
**Overall:** [PASS / WARN / FAIL] ([N] issues found)

## Results

| # | Check | Result | Details |
|---|-------|--------|---------|
| 1 | Token budget | PASS/WARN/FAIL | [word count] |
| 2 | Global header | PASS/FAIL | Present / Missing |
| 3 | Constraints in first 20% | PASS/WARN | NEVER/ALWAYS in first N lines |
| 4 | Tool Access section | PASS/FAIL | Present / Missing |
| 5 | Failure & Escalation | PASS/FAIL | Present / Missing |
| 6 | Remember section | PASS/FAIL | Present / Missing |
| 7 | AGENT-SOUL reference | PASS/FAIL | Present / Missing |

## Recommendations

1. [Specific actionable recommendation]
2. [Specific actionable recommendation]

---

# Agent File Review — Batch Summary

| Agent | Words | FAIL | WARN | PASS | Top Issue |
|-------|-------|------|------|------|-----------|
| [agent-name] | [count] | [N] | [N] | [N] | [description or —] |

**Agents needing attention (sorted by FAIL count):**

1. [agent] — [N] FAILs: [list]
