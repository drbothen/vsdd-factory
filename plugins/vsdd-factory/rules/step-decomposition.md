---
name: step-decomposition
description: Pattern for decomposing monolithic phase skills into step files with shared context, lobster workflow wiring, and state-manager crash recovery.
---

# Step Decomposition Pattern

When a VSDD phase skill is decomposed into steps, follow this structure exactly.

## Directory Layout

```
skills/<phase-skill>/
  SKILL.md                    # Monolithic entry point (command target, unchanged)
  steps/
    _shared-context.md        # Cross-step constraints loaded by every step
    step-<id>-<name>.md       # One file per step
```

## File Roles

### Parent SKILL.md

The monolithic skill remains as the **command entry point** — it's what `/vsdd-factory:<skill-name>` invokes for direct (non-orchestrated) execution. It is NOT modified when steps are added. Both execution paths (direct command and orchestrated lobster) are valid.

### _shared-context.md

Contains constraints that apply to **all steps** in the phase:

- Iron Law (if the skill has one)
- Red Flags table (if the skill has one)
- Subagent delivery protocol (if steps dispatch subagents)
- Sandbox considerations (if steps run in restricted environments)
- File naming conventions
- Cross-step reference material (comparison tables, templates list, etc.)

**Rule:** If content appears in 2+ steps, it belongs in `_shared-context.md`. If it's specific to one step, it belongs in that step file.

### Step Files

Each step file is **self-contained for its scope** — an orchestrator can dispatch it without reading the parent SKILL.md. Every step file:

1. Has frontmatter with `name` and `description`
2. Opens with a shared-context reference:
   ```markdown
   > **Shared context:** Read `./_shared-context.md` before executing this step — it contains [list relevant sections].
   ```
3. Contains the **full procedural content** for that step — no "see parent" deferrals for load-bearing clauses. If a verbatim clause must be included in subagent prompts (e.g., honest convergence clause), it must appear in full in the step file, not by reference.
4. Ends with Artifacts, Commit message, and Success Criteria sections

## Content Completeness Rule

> **No content loss on decomposition.** Every section, rule, constraint, protocol detail, verbatim clause, and procedural instruction in the parent SKILL.md must appear in exactly one of: `_shared-context.md`, a step file, or justified as intentionally excluded.

To verify: diff the parent against the union of all step files + shared context. Any content present in the parent but absent from steps requires a justification comment in `_shared-context.md` under a `## Intentional Exclusions` section explaining why.

## Step Naming

Step IDs follow the phase's natural internal structure:

- Sequential steps: `step-a-`, `step-b-`, `step-c-` (alphabetic)
- Sub-steps: `step-b5-`, `step-b6-` (numeric suffix)
- Source acquisition: `step-0-` (always first if needed)

The ID is descriptive: `step-a-broad-sweep`, not `step-a` or `step-1`.

## Lobster Workflow Wiring

The corresponding `workflows/phases/phase-N-<name>.lobster` file references step files with state-manager backup steps between each:

```yaml
steps:
  - name: <step-name>
    type: skill
    skill: "skills/<phase-skill>/steps/step-<id>-<name>.md"
    depends_on: [<prior-backup>]

  - name: backup-<step-name>
    type: agent
    agent: state-manager
    depends_on: [<step-name>]
    task: >
      Commit artifacts for <step-name> to factory-artifacts.
      Update STATE.md: phase: N, step: <step-name>, status: complete.
```

Every step is followed by a state-manager backup. This enables crash recovery — on resume, the orchestrator reads STATE.md and skips completed steps.

The lobster file ends with:
1. Phase gate (quality criteria)
2. Input-hash drift check (`skills/check-input-drift/SKILL.md`)
3. Human approval

## What NOT to Do

- **Don't gut the parent SKILL.md.** It remains the complete monolithic version. Steps are an alternative execution path, not a replacement.
- **Don't defer load-bearing content to "see parent."** If a subagent needs a verbatim clause, the step file must contain it. Pointers to the parent fail when steps are dispatched in isolation.
- **Don't create step files without a lobster workflow.** Steps without wiring are dead code.
- **Don't skip `_shared-context.md`.** Even if only one constraint is shared, use the file. Future steps will need it.
- **Don't split assessment/aggregation phases into steps.** If a phase reads data and produces a single report (e.g., convergence-check), it's naturally monolithic. Force-splitting assessment phases adds complexity without crash-recovery benefit.

## Verification

After decomposing a phase, verify:

1. `lobster-parse` succeeds on the workflow file
2. Every `skill:` path in the lobster resolves to an existing file
3. Content completeness: no section from parent SKILL.md is missing from steps + shared context
4. BATS test passes for skill path resolution (see `tests/skills.bats`)
