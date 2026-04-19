---
document_type: story-index
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [prd.md, behavioral-contracts/, architecture/module-decomposition.md]
traces_to: prd.md
---

# Story Index

> Auto-generated index of all stories. STORY-INDEX.md is the authoritative
> source for story count and status. All other documents cite this index.

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| STORY-001 | [title] | EPIC-001 | [1-13] | P0/P1/P2 | -- | draft |
| STORY-002 | [title] | EPIC-001 | [1-13] | P0/P1/P2 | STORY-001 | draft |

**Status values:** draft, ready, in-progress, merged, blocked

**Rules:**
- Every story must have a unique sequential ID (append-only numbering, Policy 1)
- Points must be 1-13 (no story exceeds 13 points)
- Dependencies must be acyclic (topological sort)
- P0 stories must not depend on P1/P2 stories
