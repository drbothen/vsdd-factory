---
document_type: story-index
level: ops
version: "1.0"
status: current
---

# STORY-INDEX (sprint-state-format fixture)
#
# Used by: test_sprint_state_status_matches_story_index (AC-003 / BC-5.41.004 PC2 + INV-2)
# This minimal STORY-INDEX has 3 non-retired stories whose canonical statuses match
# fixture-migrated.yaml. A round-trip check verifies each stories[*].status against
# the corresponding catalog row. The fixture also contains one status that matches
# the 8-value enum (S-1.04 ready) to exercise EC-007 non-trigger path.
#
# Story topology for wave ordering:
#   S-1.01: no deps       → wave 1 — merged (terminal)
#   S-1.02: depends_on S-1.01 → wave 2 — draft (next-wave selector)
#   S-1.03: depends_on S-1.01 → wave 2 — ready (non-terminal, active-but-not-next-wave)
#
# EC-007 test vector: if this STORY-INDEX had a story with status 'completed' or
# 'pending', the producer must hard-abort with UnknownStatusToken. We omit that
# here since this fixture represents a VALID index (EC-007 is tested via inline
# data in test_sprint_state_status_matches_story_index).

## Epic E-1 — Test Epic

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-1.01 | Root story no deps | E-1 | 2 | P0 | [] | [S-1.02,S-1.03] | merged | [] |
| S-1.02 | Second story depends on S-1.01 | E-1 | 3 | P1 | [S-1.01] | [] | draft | [] |
| S-1.03 | Third story depends on S-1.01 | E-1 | 3 | P1 | [S-1.01] | [] | ready | [] |
