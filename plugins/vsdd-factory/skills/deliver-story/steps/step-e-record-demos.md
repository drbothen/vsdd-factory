---
name: step-e-record-demos
description: Dispatch demo-recorder to capture per-AC demo artifacts covering success and error paths.
---

# Step E: Record Demos

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline and verification rules.

## Dispatch

**Agent:** `demo-recorder` (model tier: Fast)

**Task:** "Record per-AC demos in `.worktrees/STORY-NNN/docs/demo-evidence/<STORY-ID>/`. Use VHS for CLI or Playwright for web. Capture both success and error paths. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`."

**Context to pass:** Story file, acceptance criteria extract only.

## Exit Condition

Every acceptance criterion has at least one demo artifact referenced in the evidence report.

## Artifacts

- Demo recordings in `.worktrees/STORY-NNN/docs/demo-evidence/<STORY-ID>/`
- `docs/demo-evidence/<STORY-ID>/evidence-report.md` — evidence index
