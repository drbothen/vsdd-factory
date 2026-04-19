---
name: agent-file-review
description: >
  Reviews AGENTS.md files for compliance with Dark Factory agent design principles.
  Checks token budget, contradictions, negative examples, FACTORY.md duplication,
  tool profile mismatches, and structural compliance with the canonical template.
---

# Agent File Review

## Prerequisites

- `agents/*/AGENTS.md` files exist for the target agent(s)
- `openclaw.json` exists at repo root for tool profile verification
- `FACTORY.md` exists at repo root (needed for duplication checks)

## When This Skill Runs

- When creating or modifying an agent's AGENTS.md file
- When debugging agent misbehavior (self-spawning, wrong outputs, tool errors)
- As a periodic audit across all 34 agents
- Before committing AGENTS.md changes

## Inputs

- **Target:** A single agent AGENTS.md path, or "all" to review all agents
- **Config:** `openclaw.json` for tool profile verification

## Review Checklist

Run ALL checks below. Report each as PASS / WARN / FAIL with specific findings.

### 1. Token Budget (FAIL if >5,000 words, WARN if >3,500 words)

Count words in the AGENTS.md file. Research shows compliance degrades
significantly above ~4,000 tokens (~3,500 words).

```bash
wc -w <agents-md-path>
```

- **PASS:** <= 3,500 words
- **WARN:** 3,501-5,000 words — consider splitting into core + reference files
- **FAIL:** > 5,000 words — must split; "lost middle" compliance degradation is severe

### 2. Global Header Present (FAIL if missing)

First line must contain:
```
> **Global Operating Rules:** Read `../../FACTORY.md`
```

### 3. Hard Constraints in First 20% (WARN if missing)

Check that constraint keywords (NEVER, MUST NOT, ALWAYS, CANNOT) appear in
the first 20% of the file. Primacy effect: first 20% gets 15-25% higher compliance.

### 4. Recency Restatement (WARN if missing)

The last section should restate the agent's most critical constraint.
Look for a "Remember" section or equivalent at the end of the file.

### 5. No Negative Code Examples (FAIL if found)

Search for code blocks preceded by "WRONG", "incorrect", "bad", "don't do this".
LLMs attend to code structure, not labels — negative examples teach the wrong pattern.

```
grep -B2 '```' <file> | grep -i 'wrong\|incorrect\|bad example\|don.t do'
```

### 6. No Model Names (WARN if found)

Search for specific model names that should be engine config, not agent knowledge:
- adversary model, GPT-4, gpt-
- Claude, implementation-tier, judgment-tier, claude-haiku
- Gemini, gemini-
- DeepSeek
- Codestral, Qwen

Exception: if the model name is in the context of "you provide cognitive diversity"
without naming the specific model, that's acceptable.

### 7. No Pipeline Position References (WARN if found)

Search for phase-specific references like "Phase 1d", "Phase 4", "during Phase 3"
that indicate the agent knows its position in the pipeline. Station isolation
principle: agents should know their inputs/outputs, not their pipeline position.

Exception: the orchestrator AGENTS.md IS allowed pipeline position knowledge.
Exception: brief references like "the orchestrator spawns you" are acceptable.

### 8. Tool Profile Match (FAIL if mismatch)

Read the agent's tool profile from `openclaw.json` and compare to the
"Tool Restrictions" or "Tool Access" section in AGENTS.md.

Common mismatches:
- AGENTS.md says "messaging" but config says "full" with deny list
- AGENTS.md says "cannot write" but config allows write
- AGENTS.md says "can execute" but config denies exec

### 9. Internal Contradictions (FAIL if found)

Scan for sections that contradict each other:
- "You MAY write X" in one section vs "You cannot write" in another
- "You have tool X" vs "You do NOT have tool X"
- Different descriptions of the same capability in different sections

### 10. FACTORY.md Rule Duplication (WARN if found)

Check if the AGENTS.md repeats rules that are already in FACTORY.md.
Common duplications:
- Sub-Agent Delegation Rule (sessions_spawn syntax)
- VSDD constraints (Red Before Green, Spec Supremacy)
- Artifact path conventions

The agent should reference FACTORY.md, not repeat it.

### 11. Contract-Based Structure (WARN if missing)

Check for the presence of input/output/success-criteria sections:
- "Inputs" or "Input" or "Your Inputs" or "Primary Input"
- "Outputs" or "Output" or "Your Outputs"
- "Success Criteria" or "When You're Done" or "Approval Criteria"

### 12. Escalation Levels Defined (WARN if missing)

Check for escalation/failure handling:
- "Escalat" keyword present
- "HALT" or "halt" conditions defined
- "Level 1" / "Level 2" / "Level 3" or equivalent tiers

### 13. Context Discipline Section (WARN if missing for Tier 2+ agents)

Check for DF-021 context discipline:
- "Load:" and "Do NOT load:" patterns
- "Context Discipline" or "Architecture Context Discipline" section header

### 14. Information Wall Documented (WARN if applicable but missing)

For agents that should have information walls (adversary, holdout-evaluator,
code-reviewer, spec-reviewer), check that the wall is documented.

### 15. No sessions_spawn in Non-Orchestrator Agents (WARN if found)

Only the orchestrator should contain `sessions_spawn` references.
Other agents receive work from the orchestrator — they don't spawn sub-agents.

Exception: agents that legitimately delegate (e.g., research-agent delegating
to ) may have spawn references.

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/agent-file-review-template.md` for the agent file review report format.

## Output Format

```markdown
# Agent File Review: [agent-name]

**File:** [path]
**Words:** [count] ([PASS/WARN/FAIL])
**Overall:** [PASS / WARN / FAIL] ([N] issues found)

## Results

| # | Check | Result | Details |
|---|-------|--------|---------|
| 1 | Token budget | PASS | 594 words |
| 2 | Global header | PASS | Present |
| 3 | Constraints in first 20% | WARN | No NEVER/ALWAYS in first 20 lines |
| ... | ... | ... | ... |

## Recommendations

1. [Specific actionable recommendation]
2. [Specific actionable recommendation]
```

## Batch Mode

When reviewing all agents, produce a summary table:

```markdown
# Agent File Review — Batch Summary

| Agent | Words | FAIL | WARN | PASS | Top Issue |
|-------|-------|------|------|------|-----------|
| orchestrator | 1,057 | 0 | 1 | 14 | — |
| adversary | 2,731 | 1 | 2 | 12 | Model names |
| ... | ... | ... | ... | ... | ... |

**Agents needing attention (sorted by FAIL count):**
1. [agent] — [N] FAILs: [list]
2. [agent] — [N] FAILs: [list]
```
