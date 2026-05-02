---
story_id: S-8.00
ac: AC-6
title: Per-hook story point estimates for S-8.01..S-8.09 confirmed
---

# AC-6: Per-hook story point estimates for S-8.01..S-8.09 confirmed

**Statement:** Per-hook story point estimates for S-8.01..S-8.09 confirmed or bumped per BC-gap audit + complexity findings. If any hook required a new BC (AC-5), its port story's estimate is bumped by +1 pt. If regression-gate's BC-anchor cannot be confirmed pending OQ-6 resolution, flag as deferred S-8.09 sub-task and bump S-8.09.

## Evidence

### Story point confirmation from BC-anchor audit

Since AC-5 found **0 gaps** (no new BCs drafted), **no +1pt bumps are required** for any story.

```markdown
## Notes (from E-8-bc-anchor-table.md)

### Story Point Implications (AC-6)

Since 0 hooks required new BCs, no S-8.01..S-8.09 story estimates need +1pt bump
for BC-creation overhead. S-8.09 retains its existing estimate noting the OQ-6
deferred sub-task.
```

### Current E-8 epic Stories table (relevant rows, unchanged from v1.9)

```
| S-8.01 | Native port: handoff-validator (SubagentStop)                  | 4 | S-8.00 | draft |
| S-8.02 | Native port: pr-manager-completion-guard (SubagentStop)        | 5 | S-8.00 | draft |
| S-8.03 | Native port: track-agent-stop (SubagentStop)                   | 3 | S-8.00 | draft |
| S-8.04 | Native port: update-wave-state-on-merge (SubagentStop)          | 4 | S-8.00 | draft |
| S-8.05 | Native port: validate-pr-review-posted (SubagentStop)           | 3 | S-8.00 | draft |
| S-8.06 | Native port: session-learning (Stop)                           | 3 | S-8.00 | draft |
| S-8.07 | Native port: warn-pending-wave-gate (Stop)                     | 3 | S-8.00 | draft |
| S-8.08 | Native port: track-agent-start (PreToolUse:Agent)              | 3 | S-8.00 | draft |
| S-8.09 | Native port: regression-gate + legacy-bash-adapter retirement prep | 5 | S-8.01..S-8.08 | draft |
```

**Note on S-8.09:** S-8.09 retains its 5-point estimate. The OQ-6 deferred sub-task (subprocess capability profile audit) is flagged as an open entry in the BC-anchor table's Action-Needed column for regression-gate, not a point bump. The story spec for S-8.09 must include OQ-6 in assumption_validations when authored.

### Summary

- **Stories requiring estimate bumps:** 0
- **Story point changes committed:** None (no E-8 epic D-8 update required)
- **OQ-6 flag for S-8.09:** Noted in BC-anchor table; S-8.09 story spec must reference OQ-6

**Result:** AC-6 SATISFIED. 0 story point bumps required because 0 new BCs were drafted. S-8.01..S-8.09 estimates remain at their v1.9 values. S-8.09 OQ-6 deferred sub-task noted.
