---
document_type: convergence-trajectory
level: ops
version: "1.0"
status: in-progress
producer: state-manager
timestamp: YYYY-MM-DDTHH:MM:SS
cycle: "[cycle-name]"
inputs: [adversarial-reviews/]
input-hash: "[md5]"
traces_to: STATE.md
---

# Convergence Trajectory — [cycle-name]

## Finding Progression

| Pass | Date | Total | CRIT | HIGH | MED | LOW | Novelty | Score | Counter | Verdict |
|------|------|-------|------|------|-----|-----|---------|-------|---------|---------|
| 1 | YYYY-MM-DD | | | | | | HIGH | | 0/3 | FINDINGS_REMAIN |
| 2 | YYYY-MM-DD | | | | | | | | 0/3 | FINDINGS_REMAIN |

## Trajectory Shorthand

`[total-1]→[total-2]→[total-3]→...`

## Per-Pass Details

### Pass 1 (YYYY-MM-DD)

**Findings:** [N] ([N CRIT, N HIGH, N MED, N LOW])
**Novelty:** [HIGH / MEDIUM / LOW]
**Convergence counter:** [N of 3]

[Summary of key findings and their categories.]

---

<!-- Repeat for each pass. Maintain chronological order. -->

## Frontmatter Fields (extracted from STATE.md)

<!-- When compacting STATE.md, adversary_pass_* frontmatter fields are
     converted to rows in the Finding Progression table above.
     Original field format: adversary_pass_N_findings: "description"
     Original field format: adversary_pass_N_date: "YYYY-MM-DD" -->
