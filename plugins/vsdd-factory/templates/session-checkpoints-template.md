---
document_type: session-checkpoints
level: ops
version: "1.0"
status: archive
producer: state-manager
timestamp: YYYY-MM-DDTHH:MM:SS
cycle: "[cycle-name]"
inputs: [STATE.md]
input-hash: "[md5]"
traces_to: STATE.md
---

# Session Checkpoints — [cycle-name]

<!-- Archived session resume checkpoints extracted from STATE.md.
     Only the LATEST checkpoint lives in STATE.md.
     Prior checkpoints are archived here for historical reference. -->

## Session Resume Checkpoint (YYYY-MM-DD) — [position label]

### Spec Versions

| Artifact | Version |
|----------|---------|
| [artifact-name] | [version] |

### State

| Field | Value |
|-------|-------|
| **Date** | YYYY-MM-DD |
| **Position** | [phase, step, what was next] |
| **Convergence counter** | [N of 3] |
| **Next step** | [what was planned next] |

### Resume Prompt

```
[The resume prompt that was in STATE.md at this checkpoint]
```

---

<!-- Repeat for each archived checkpoint. Maintain chronological order. -->
