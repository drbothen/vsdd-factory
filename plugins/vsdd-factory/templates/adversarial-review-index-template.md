---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: "in-review|resolved"
producer: adversary
timestamp: YYYY-MM-DDTHH:MM:SS
phase: "1d|2|4"
pass: 1
inputs: [prd.md, "architecture/", "behavioral-contracts/"]
traces_to: prd.md
total_findings: 0
severity_distribution: { CRIT: 0, HIGH: 0, MED: 0, LOW: 0 }
---

# Adversarial Review -- Pass [N]

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|-----------|--------|
| ADV-P[N]-001 | [severity] | [category] | [title] | open | -- | [IDs] |

## Dependency Graph

```text
[Finding dependency DAG. Example:]
ADV-P[N]-001 --blocks--> ADV-P[N]-005
ADV-P[N]-003 --blocks--> ADV-P[N]-012
[All other findings are independent]
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|------------|------------------------|
| spec-gap | [IDs] | [Yes / Yes after X resolved] |
| consistency | [IDs] | [Yes / No -- explain] |
| completeness | [IDs] | [Yes / dependency note] |
| edge-case | [IDs] | [Yes] |
| security | [IDs] | [Yes / No -- explain] |
| performance | [IDs] | [Yes] |
