---
document_type: epic-index
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [prd.md, behavioral-contracts/]
traces_to: prd.md
---

# Epic Index

> Epics group related stories for wave scheduling and dependency management.
> Epic IDs use `E-N` format (single digit, append-only, never reused).
> Epic `E-N` directly contains all stories with `story_id: S-N.*` (i.e.,
> story section number matches epic number — `E-3` contains all `S-3.*`).
> Filename: `E-N-<short-description>.md` under `.factory/stories/epics/`.

| Epic ID | Title | Stories | Priority | Status |
|---------|-------|---------|----------|--------|
| E-1 | [title] | S-1.01, S-1.02 | P0/P1/P2 | active / complete |

## Epic-to-Capability Mapping

| Epic ID | Capabilities (CAP-NNN) | Subsystems (SS-NN) |
|---------|----------------------|-------------------|
| E-1 | CAP-001, CAP-002 | SS-01 |
