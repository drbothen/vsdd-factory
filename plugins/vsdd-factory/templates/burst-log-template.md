---
document_type: burst-log
level: ops
version: "1.0"
status: in-progress
producer: state-manager
timestamp: YYYY-MM-DDTHH:MM:SS
cycle: "[cycle-name]"
inputs: [STATE.md]
input-hash: "[md5]"
traces_to: STATE.md
---

# Burst Log — [cycle-name]

## Burst 1 (YYYY-MM-DD)

**Agents dispatched:** [agent-1, agent-2, ...]
**Files touched:** [list of files modified/created]
**Versions bumped:** [artifact-name vX.Y → vX.Z, ...]

### Summary

[Brief narrative of what this burst accomplished — findings closed, artifacts updated, decisions made.]

### Details

| Agent | Task | Output |
|-------|------|--------|
| [agent-id] | [task description] | [artifact path or outcome] |

---

<!-- Repeat for each burst. Maintain chronological order. -->
