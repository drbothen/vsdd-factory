---
name: writing-skills
description: >
  Use when creating new skills, editing existing skills, or verifying skills work
  before deployment. Applies TDD methodology to process documentation — write
  pressure scenarios first, verify agent fails without skill, then write the skill.
---

# Writing Skills

Writing skills IS Test-Driven Development applied to process documentation.

You write test cases (pressure scenarios with subagents), watch them fail (baseline behavior), write the skill (documentation), watch tests pass (agents comply), and refactor (close loopholes).

**Core principle:** If you didn't watch an agent fail without the skill, you don't know if the skill teaches the right thing.

## Hard Gate

NO SKILL WITHOUT A FAILING TEST FIRST.

Write skill before testing? Delete it. Start over. Edit skill without testing? Same violation. No exceptions.

## When to Create a Skill

**Create when:**
- Technique wasn't intuitively obvious
- You'd reference this again across projects
- Pattern applies broadly (not project-specific)
- Others would benefit

**Don't create for:**
- One-off solutions
- Standard practices well-documented elsewhere
- Project-specific conventions (put in CLAUDE.md or rules/)
- Mechanical constraints enforceable with hooks or validation

## TDD Mapping for Skills

| TDD Concept | Skill Creation |
|-------------|----------------|
| Test case | Pressure scenario with subagent |
| Production code | Skill document (SKILL.md) |
| Test fails (RED) | Agent violates rule without skill |
| Test passes (GREEN) | Agent complies with skill present |
| Refactor | Close loopholes while maintaining compliance |

## Skill Types

| Type | Examples | Test approach |
|------|----------|---------------|
| **Discipline** (rules) | TDD, verification-before-completion | Pressure scenarios (time, sunk cost, exhaustion) |
| **Technique** (how-to) | systematic-debugging, root-cause-tracing | Application and variation scenarios |
| **Pattern** (mental model) | information-hiding, reduce-complexity | Recognition and counter-examples |
| **Reference** (docs) | API docs, tool guides | Retrieval and application scenarios |

## SKILL.md Structure

```yaml
---
name: skill-name-with-hyphens
description: >
  Use when [specific triggering conditions and symptoms].
  Do NOT summarize the skill's workflow here — just triggers.
---
```

Sections:
1. **Hard Gate** — the non-negotiable rule (if discipline skill)
2. **When to Use** — symptoms, situations, triggers
3. **Core Process** — the technique or pattern
4. **Red Flags** — rationalization table (if discipline skill)
5. **Reporting** — standard status protocol
6. **Quick Reference** — table for scanning

## CSO (Claude Search Optimization)

The `description` field determines whether Claude finds your skill. Critical rules:

- Start with "Use when..." describing ONLY triggering conditions
- Do NOT summarize the skill's process (agents shortcut to description, skip the body)
- Include specific symptoms and keywords agents would search for
- Keep under 500 characters

```yaml
# BAD: Summarizes workflow — agent follows this instead of reading skill
description: Use for TDD - write test first, watch it fail, write minimal code

# GOOD: Just triggers — agent must read skill for process
description: Use when implementing any feature or bugfix, before writing implementation code
```

## RED-GREEN-REFACTOR for Skills

### RED: Write Failing Test (Baseline)

Run pressure scenario with subagent WITHOUT the skill. Document:
- What choices did they make?
- What rationalizations did they use (verbatim)?
- Which pressures triggered violations?

### GREEN: Write Minimal Skill

Write skill addressing those specific rationalizations. Don't add content for hypothetical cases. Run same scenarios WITH skill — agent should now comply.

### REFACTOR: Close Loopholes

Agent found new rationalization? Add explicit counter. Re-test until bulletproof.

Key techniques:
- **Close every loophole explicitly** — don't just state the rule, forbid specific workarounds
- **Address "spirit vs letter"** — add: "Violating the letter of the rules is violating the spirit"
- **Build rationalization table** — every excuse agents make goes in the table
- **Create Red Flags list** — make it easy for agents to self-check

## Bulletproofing Against Rationalization

For discipline-enforcing skills, rationalization resistance is critical:

| Excuse | Counter |
|--------|---------|
| "Skill is obviously clear" | Clear to you does not equal clear to agents. Test it. |
| "It's just a reference" | References can have gaps. Test retrieval. |
| "Testing is overkill" | Untested skills have issues. Always. |
| "I'll test if problems emerge" | Problems = agents failing. Test BEFORE deploying. |
| "Too simple to need a test" | Simple skills hide simple gaps. Test anyway. |

## File Organization

```
skills/
  skill-name/
    SKILL.md              # Main reference (required)
    supporting-file.*     # Only if needed (heavy reference, reusable tools)
```

Keep content inline unless reference material exceeds 100 lines.

## vsdd-factory Conventions

When writing skills for this plugin:
- Follow `templates/skill-delegation-template.md` for dispatcher skills
- Follow `templates/skill-execution-template.md` for specialist skills
- Follow `templates/agents-md-template.md` for agent definitions
- Include standard status protocol (DONE/DONE_WITH_CONCERNS/NEEDS_CONTEXT/BLOCKED)
- Include Context Discipline section if the skill loads `.factory/` artifacts
- Target 1,500-3,000 words per skill

## Skill Creation Checklist

**RED Phase:**
- [ ] Create pressure scenarios (3+ combined pressures for discipline skills)
- [ ] Run scenarios WITHOUT skill — document baseline behavior
- [ ] Identify patterns in rationalizations/failures

**GREEN Phase:**
- [ ] Name uses letters, numbers, hyphens only
- [ ] Frontmatter with `name` and `description` (description starts with "Use when...")
- [ ] Hard Gate (if discipline skill)
- [ ] Address specific baseline failures from RED phase
- [ ] Run scenarios WITH skill — verify compliance

**REFACTOR Phase:**
- [ ] Identify new rationalizations from testing
- [ ] Add explicit counters and Red Flags table
- [ ] Re-test until bulletproof

**Quality:**
- [ ] Quick reference table for scanning
- [ ] Common mistakes section
- [ ] No narrative storytelling
- [ ] Commit to git
