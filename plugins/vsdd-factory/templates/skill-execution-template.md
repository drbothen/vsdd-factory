<!--
  Dark Factory SKILL.md Template — Execution Skill

  Usage: Copy this template when creating a skill that a SPECIALIST AGENT
  reads to perform work directly. The agent executes the steps itself —
  it does NOT spawn other agents.

  Target: 1,500-3,000 words. Use reference files in the skill directory
  for deeper material.

  See: docs/dark-factory-skill-design-principles.md
-->

---
name: [skill-name]
description: >
  [One-line description of what this skill does and when to invoke it.
  This appears in the system prompt skill list — keep it concise.]
---

# [Capability Name]

[1-2 sentences: what this skill does. Do not reference pipeline position
("Phase 3") or specific models. The skill should be context-agnostic.]

## Contract

### Inputs

[What the agent receives. Be specific about file paths and formats.]

- `[artifact-path]` — [description]
- `[artifact-path]` — [description]

### Outputs

[What the agent produces. Include paths and template references.]

| Artifact | Path | Template |
|----------|------|----------|
| [name] | `.factory/[path]` | `templates/[template].md` |

### Success Criteria

[Measurable definition of "done".]

- [ ] [criterion 1]
- [ ] [criterion 2]

## Procedure

[Step-by-step instructions for the specialist agent. This is the HOW.
Include enough detail for the agent to execute correctly, but assume
the agent has domain knowledge (don't over-explain fundamentals).]

1. Read [input artifact] to understand [context]
2. [Action step with specific guidance]
3. [Action step]
4. Write output to [path] using [template]
5. Verify [quality check]

## Quality Checks

[Self-verification the agent performs before returning output.]

- [ ] [Check 1 — format compliance]
- [ ] [Check 2 — content completeness]
- [ ] [Check 3 — template adherence]

## Failure Modes

[What can go wrong and what to do about it.]

- If [input missing]: Report to orchestrator — cannot proceed without [artifact]
- If [output doesn't meet criteria]: Retry with adjusted approach (max 3 attempts)
- If [ambiguity in input]: Flag the specific ambiguity and request clarification
- If [three consecutive failures]: Stop and report to orchestrator with summary

## Reference Files

(OPTIONAL — additional docs in this skill directory for deeper material)

- `{baseDir}/[reference-file].md` — [what it contains]
