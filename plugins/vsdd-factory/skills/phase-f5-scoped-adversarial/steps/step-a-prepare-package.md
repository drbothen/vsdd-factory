---
name: step-a-prepare-package
description: Build the scoped review package for the adversary — delta files only, with information asymmetry walls enforced.
---

# Step A: Prepare Review Package

> **Shared context:** Read `./_shared-context.md` before executing this step.

Build the scoped review package for the adversary. Include ONLY delta files and necessary context. Enforce information asymmetry walls.

## Procedure

1. **Read the affected files list:** `.factory/phase-f1-delta-analysis/affected-files.txt`

2. **Build the review package:**

   **Changed files** (from affected-files.txt):
   - New source files (full content)
   - Modified source files (provide full file, not just diff — the adversary needs context)
   - New test files (full content)

   **Context files** (read-only reference):
   - Relevant spec sections (PRD delta, architecture delta)
   - Story specs for the implemented stories
   - Existing conventions documentation (FACTORY.md, coding rules)

3. **Enforce exclusion list** (DF-025 information asymmetry walls):
   - Unchanged source files (unless direct dependents of changed code)
   - Previous adversarial review reports
   - Implementation notes or rationale
   - Phase F4 TDD logs and implementer session notes
   - `.factory/semport/**` (gene transfusion history, DF-028)
   - `.factory/cycles/**/implementation/red-gate-log*`, `implementer-notes*`

4. **Verify package completeness:**
   - All files from affected-files.txt are included
   - Modified files are provided in full (not diffs)
   - No excluded files are accidentally included

## Artifacts

- Review package (in-memory) — file list and content for adversary context

## Success Criteria

- Package contains only delta files + necessary context
- No unchanged files included (except direct dependents)
- No previous adversarial reports included (fresh perspective)
- No implementation rationale or TDD logs included
- Modified files provided in full, not as diffs
