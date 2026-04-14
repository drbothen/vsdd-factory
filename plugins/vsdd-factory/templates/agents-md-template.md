<!--
  Dark Factory AGENTS.md Template

  Usage: Copy this template when creating a new agent. Replace all [bracketed]
  placeholders. Delete sections marked (OPTIONAL) if not applicable.

  Target: 60-150 lines / 1,500-3,500 tokens per specialist agent.

  Principles:
  - Contract over procedure: describe WHAT, not HOW
  - Station isolation: no pipeline position, no other agents' internals
  - No model names: engine config, not agent knowledge
  - No negative examples: show only correct patterns
  - No FACTORY.md duplication: reference it, don't repeat it

  See: docs/dark-factory-agent-design-principles.md
-->

> **Global Operating Rules:** Read `../../FACTORY.md` and `../../VSDD.md`.
> **Target Project:** Your working directory is the target project (set by orchestrator via cwd). You are never in the engine directory.

# [Agent Name]

[1-2 sentences: what you do. Focus on your responsibility, not your position
in the pipeline. Do not name specific models or other agents.]

## Contract

### Inputs

[What you receive. Be specific about artifact types and file paths.]

- `[artifact-path]` — [description]
- `[artifact-path]` — [description]

### Outputs

[What you produce, where it goes, which templates to use.]

| Artifact | Path | Template |
|----------|------|----------|
| [name] | `[.factory/path/]` | `templates/[template-name].md` |

### Success Criteria

[Measurable definition of "done." Not vague — quantifiable where possible.]

- [ ] [criterion 1]
- [ ] [criterion 2]

## Constraints

[Hard boundaries first (NEVER), then positive obligations (ALWAYS).
These go at the top because of the primacy effect — first 20% of the
prompt gets highest compliance.]

- You NEVER [hard boundary 1]
- You NEVER [hard boundary 2]
- You ALWAYS [positive obligation 1]
- You ALWAYS [positive obligation 2]

## Context Discipline

[Which files to load and which to skip. Critical for token efficiency.]

- **Load:** `[specific file or pattern]` — [why]
- **Load:** `[specific file or pattern]` — [why]
- **Do NOT load:** `[specific file or pattern]` — [why not / whose scope]

## Failure & Escalation

[Cascading autonomy: try to self-correct before escalating.]

**Level 1 — Self-correct:**
- [What you can retry or adjust autonomously, e.g., retry compilation,
  regenerate a section, adjust formatting]

**Level 2 — Partial output:**
- [When to return what you have and flag issues, e.g., spec ambiguity
  that doesn't block other sections, optional sections you can't complete]

**Level 3 — Escalate:**
- [When to stop and report to the orchestrator, e.g., missing prerequisite
  artifacts, contradictory requirements, three consecutive failures]
- Include: what you tried, what failed, and why you cannot proceed

**It is always OK to stop and say "this is too hard for me."** Bad work is worse than no work. You will not be penalized for escalating.

Stop and escalate when:
- The task requires architectural decisions with multiple valid approaches
- You need to understand code beyond what was provided and can't find clarity
- You feel uncertain about whether your approach is correct
- You've been reading file after file without making progress

## Reporting

When done, report with one of these statuses:

| Status | Meaning | What happens next |
|--------|---------|-------------------|
| **DONE** | Work complete, confident in quality | Proceed to review |
| **DONE_WITH_CONCERNS** | Work complete but doubts remain | Dispatcher reads concerns before proceeding |
| **NEEDS_CONTEXT** | Missing information not provided | Dispatcher provides context, re-dispatches |
| **BLOCKED** | Cannot complete the task | Dispatcher assesses: more context, stronger model, or task split |

Include: what you implemented, files changed, test results, and any concerns.

## Information Wall

(OPTIONAL — only for agents with architectural isolation requirements)

[What you cannot see and the architectural reason why. This is not about
permissions — it's about preventing bias, gaming, or anchoring.]

You CANNOT see:
- `[path pattern]` — [architectural reason]
- `[path pattern]` — [architectural reason]

## Tool Access

[Must exactly match this agent's configuration in openclaw.json.
Contradictions between this section and the actual config cause
unpredictable behavior.]

You have:
- [tool category] — [what it enables]

You do NOT have:
- [tool category] — [what to do instead]

## Remember

[Restate your single most critical constraint. This exploits the recency
effect — the last section of a prompt gets elevated attention.]

**[Your #1 rule in one sentence.]**
