---
name: step-decomposition
description: Standard for VSDD pipeline phase numbering, step naming, workflow structure, and phase entry-point skills.
---

# VSDD Workflow Standard

This document defines the canonical structure for VSDD pipeline phases, step decomposition, naming conventions, and workflow wiring. All phases MUST follow this standard.

---

## Phase Numbering

The VSDD pipeline has 8 phases, numbered 0-7:

| Phase | Name | Work Skill | Entry-Point Skill |
|-------|------|-----------|-------------------|
| 0 | Codebase Ingestion | `brownfield-ingest` | `phase-0-codebase-ingestion` |
| 1 | Spec Crystallization | `create-brief`, `create-domain-spec`, `create-prd`, `create-architecture` | `phase-1-spec-crystallization` |
| 2 | Story Decomposition | `decompose-stories` | `phase-2-story-decomposition` |
| 3 | TDD Implementation | `deliver-story` | `phase-3-tdd-implementation` |
| 4 | Holdout Evaluation | `holdout-eval` | `phase-4-holdout-evaluation` |
| 5 | Adversarial Refinement | `adversarial-review` | `phase-5-adversarial-refinement` |
| 6 | Formal Hardening | `formal-verify` | `phase-6-formal-hardening` |
| 7 | Convergence | `convergence-check` | `phase-7-convergence` |

**Rules:**
- Phase numbers are sequential integers 0-7. No fractional phases (no "3.5").
- Feature-mode phases use the `f` prefix: `phase-f1` through `phase-f7`.
- Every phase has exactly two skill entry points: a **work skill** (functional name, command target) and a **phase entry-point skill** (phase-numbered, lobster target).

---

## Three-Layer Architecture

```
Top-level lobster (greenfield.lobster, brownfield.lobster, etc.)
  → calls phase entry-point skill (skills/phase-N-<name>/SKILL.md)
    → references phase sub-workflow (workflows/phases/phase-N-<name>.lobster)
      → calls step files (skills/<work-skill>/steps/step-<letter>-<name>.md)
        → does the actual work
```

### Layer 1: Top-Level Workflow (lobster)

Files: `workflows/greenfield.lobster`, `workflows/brownfield.lobster`, etc.

These are mode workflows that string phases together. They reference phase entry-point skills:

```yaml
- name: phase-1-spec-crystallization
  type: skill
  skill: "skills/phase-1-spec-crystallization/SKILL.md"
```

### Layer 2: Phase Entry-Point Skill

Files: `skills/phase-N-<name>/SKILL.md`

Thin orchestration wrapper that:
1. Describes the phase purpose, prerequisites, and gate criteria
2. References the phase sub-workflow lobster file as the canonical step sequence
3. Lists the work skill(s) and step files this phase delegates to

These are NOT command targets — they have no corresponding `/vsdd-factory:` command. They exist solely as lobster reference points.

### Layer 3: Phase Sub-Workflow (lobster)

Files: `workflows/phases/phase-N-<name>.lobster`

Step-level orchestration with state-manager backups between each step. References step files from the work skill's `steps/` directory.

### Layer 4: Work Skills + Step Files

Files: `skills/<work-skill>/SKILL.md` (command target) and `skills/<work-skill>/steps/*.md` (step files)

The work skill is the monolithic command entry point (`/vsdd-factory:<work-skill>`). Step files are the decomposed version referenced by the phase sub-workflow.

---

## Step Naming

### Step IDs

Steps use **lowercase alphabetic IDs** only: `step-a-`, `step-b-`, `step-c-`, etc.

**Rules:**
- Always alphabetic: `a`, `b`, `c`, ... `z`
- Never numeric: no `step-0-`, `step-1-`, `step-2-`
- Never sub-stepped: no `step-b5-`, `step-b6-`. If a step needs sub-steps, promote them to their own letter.
- Always descriptive: `step-a-broad-sweep`, not `step-a` or `step-1`

### Step File Structure

```
skills/<work-skill>/steps/
  _shared-context.md          # Cross-step constraints (Iron Law, Red Flags, etc.)
  step-a-<name>.md            # First step
  step-b-<name>.md            # Second step
  ...
```

Every step file:
1. Has frontmatter with `name` and `description`
2. Opens with a shared-context reference
3. Contains **full procedural content** — no "see parent" deferrals for load-bearing clauses
4. Ends with Artifacts, Commit message, and Success Criteria sections

### Lobster Step Names

Lobster step `name:` fields MUST match the step file ID (without `step-` prefix):

```yaml
- name: broad-sweep                    # matches step-a-broad-sweep.md
  skill: "skills/brownfield-ingest/steps/step-a-broad-sweep.md"

- name: backup-broad-sweep             # state-manager backup follows same pattern
  agent: state-manager
```

---

## Shared Context File

`_shared-context.md` contains constraints that apply to ALL steps in the phase:

- Iron Law (if the skill has one)
- Red Flags table
- Subagent delivery protocol
- Sandbox considerations
- File naming conventions
- Templates list
- Prerequisites
- Cross-step reference material

**Rule:** If content appears in 2+ steps, it belongs in `_shared-context.md`. If it's specific to one step, it belongs in that step file.

---

## Content Completeness Rule

> **No content loss on decomposition.** Every section, rule, constraint, protocol detail, verbatim clause, and procedural instruction in the parent SKILL.md must appear in exactly one of: `_shared-context.md`, a step file, or justified as intentionally excluded.

To verify: diff the parent against the union of all step files + shared context. Any content present in the parent but absent from steps requires a justification comment in `_shared-context.md` under a `## Intentional Exclusions` section.

---

## Lobster Sub-Workflow Structure

Every phase sub-workflow follows this pattern:

```yaml
workflow:
  name: phase-N-<name>
  version: "X.Y.Z"

  steps:
    # === Step A: <Name> ===
    - name: <step-name>
      type: skill
      skill: "skills/<work-skill>/steps/step-a-<name>.md"
      depends_on: []

    - name: backup-<step-name>
      type: agent
      agent: state-manager
      depends_on: [<step-name>]
      task: >
        Commit artifacts. Update STATE.md: phase: N, step: <step-name>, status: complete.

    # ... more steps ...

    # === Phase Gate ===
    - name: phase-N-gate
      type: gate
      depends_on: [last-backup]
      gate:
        criteria: [...]
        fail_action: block

    # === Input-Hash Drift Check ===
    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [phase-N-gate]

    # === Human Approval ===
    - name: human-approval
      type: human-approval
      depends_on: [input-hash-drift-check]
```

---

## What NOT to Do

- **Don't use fractional phase numbers.** No "Phase 3.5" — use 0-7.
- **Don't use numeric step IDs.** Always alphabetic (a, b, c).
- **Don't use sub-step numbering.** No `step-b5`. Promote to its own letter.
- **Don't gut the parent SKILL.md.** It remains the complete monolithic version.
- **Don't defer load-bearing content to "see parent."** Step files must be self-contained.
- **Don't create step files without a lobster workflow.** Steps without wiring are dead code.
- **Don't skip `_shared-context.md`.** Even if only one constraint is shared, use the file.

---

## Verification

After any structural change to phases or steps:

1. `lobster-parse` succeeds on all workflow files
2. Every `skill:` path in every lobster file resolves to an existing file
3. Content completeness: no section from parent SKILL.md missing from steps + shared context
4. BATS structural test passes (lobster path resolution)
5. Phase numbers are consistent across lobster files, skills, agents, and docs
6. No references to old phase numbers remain (grep verification)
