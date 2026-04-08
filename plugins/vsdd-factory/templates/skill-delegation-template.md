<!--
  Dark Factory SKILL.md Template — Delegation Skill

  Usage: Copy this template when creating a skill that the ORCHESTRATOR reads
  to coordinate multi-agent work. The orchestrator spawns specialist agents
  for each step — it does NOT execute the steps directly.

  Target: 1,500-3,000 words. Use reference files in the skill directory for
  deeper material (step details, templates, examples).

  See: docs/dark-factory-skill-design-principles.md
-->

---
name: [skill-name]
description: >
  [One-line description of what this skill produces and when to invoke it.
  This appears in the system prompt skill list — keep it concise.]
---

> **Delegation Reference:** This skill describes work the orchestrator
> delegates to specialist agents via sessions_spawn. Each step names the
> target agent role. The orchestrator does NOT execute these steps directly
> — it spawns the named agent for each step and reviews the output.

# [Phase/Capability Name]

## Prerequisites

[What must exist before this skill runs. Be specific — artifact paths,
gate passes, human approvals.]

- `.factory/[artifact]` exists and is validated
- [Previous phase] gate passed
- Human approved [previous output]

## Steps

[Each step names an agent role and describes the task to delegate.
Do NOT include detailed procedures — that's the specialist agent's job.
Describe WHAT to produce, not HOW to produce it.]

### Step 1: [Description]

Spawn [agent role]: "[Task description including input paths, output paths,
and template references. Self-contained — the agent should be able to
execute from this description alone.]"

### Step 2: [Description]

Spawn [agent role]: "[Task description]"

### Step 3: [Decision Point]

If [condition]:
  Spawn [agent role A]: "[task]"
If [other condition]:
  Spawn [agent role B]: "[task]"

### Step N: [Validation]

Spawn [validator role]: "[Validation task — what to check, criteria to apply]"

## Quality Gate

[Measurable criteria for this skill to be considered complete.]

- [ ] [Artifact exists at expected path]
- [ ] [Validator passed with specific criteria]
- [ ] [No CRITICAL/HIGH findings remain]
- [ ] Human approval (if required at this point)

## Failure & Escalation

[What to do when things go wrong at the orchestration level.]

- If prerequisite missing: STOP, report to human with recovery steps
- If agent output fails quality gate: re-spawn agent with specific feedback (max 3 retries)
- If agent times out: spawn NEW agent with narrower scope
- If 3 retries exhausted: escalate to human with summary of attempts

## Output Artifacts

[Complete list of artifacts this skill produces, for traceability.]

| Artifact | Path | Producer |
|----------|------|----------|
| [name] | `.factory/[path]` | [agent role] |

## Remediation Sequencing

(OPTIONAL — for skills where multiple artifacts must be updated in order)

[If artifacts are coupled (e.g., PRD and architecture), specify the
update order to prevent drift.]
