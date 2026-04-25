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
| S-1.01 | [title] | E-1 | [1-13] | P0/P1/P2 | -- | draft |
| S-1.02 | [title] | E-1 | [1-13] | P0/P1/P2 | S-1.01 | draft |

**Status values:** draft, ready, in-progress, merged, blocked

**ID format:**
- Stories: `S-N.MM` where `N` is the section/epic grouping (single digit) and `MM` is the zero-padded story number within that section. Examples: `S-1.01`, `S-3.15`.
- Epics: `E-N` where `N` matches the section number used by stories. Epic `E-3` directly contains all `S-3.*` stories.
- Story `N` (section/epic) and BC `S` (subsystem number in BC-S.SS.NNN) are intentionally different hierarchies — a story can implement BCs from multiple subsystems.

**Rules:**
- Every story must have a unique ID within its section (append-only numbering, Policy 1)
- Points must be 1-13 (no story exceeds 13 points)
- Dependencies must be acyclic (topological sort)
- P0 stories must not depend on P1/P2 stories
