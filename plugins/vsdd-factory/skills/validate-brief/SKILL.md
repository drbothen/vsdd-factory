---
name: validate-brief
description: >
  Validates a product brief against the required structure, quality, and
  context engineering criteria. Checks for both gaps (missing substance)
  and bloat (over-detail that wastes agent context budget). Produces a
  validation report with specific remediation guidance.
---

# Brief Validation

> **Step-file note:** This skill does NOT use step-file decomposition.
> Its validation checks (structure, quality, bloat, leakage, density, completeness)
> run as parallel checks within a single context load, not as sequential steps.

## Validation Checks

### Structure Check (Required Sections)
For each required section, verify it exists and has substantive content:

| Section | Minimum Criteria |
|---------|-----------------|
| What Is This? | At least 2 sentences describing the product |
| Who Is It For? | At least 1 specific persona with pain point and current workaround |
| Scope -- In Scope | 3-7 capabilities listed (not features, not user stories) |
| Scope -- Out of Scope | At least 1 explicit exclusion |
| Success Criteria | At least 2 measurable outcomes with numerical targets |
| Constraints & Integration Points | At least 1 constraint or integration point |

### Quality Check
- **Specificity:** No section relies on vague language ("various users," "improve things,"
  "should be performant")
- **Measurability:** Success criteria have numbers, not qualitative descriptions.
  "Reduce alert volume by 40%" not "improve alert management"
- **Scope bounds:** In-scope and out-of-scope don't contradict each other
- **Audience clarity:** Personas are specific enough to generate requirements for.
  "DevOps engineers at mid-size SaaS companies" not "developers"
- **Constraint actionability:** Every constraint would cause implementation failure
  if missed (justifies its inclusion by cost-of-not-knowing)

### Bloat Check (Context Engineering)
Research shows over-detailed briefs degrade agent performance:
- **Word count:** Core sections (excluding Overflow Context) should be under 500 words.
  Flag if over 800 words -- the brief is becoming a PRD.
- **Narrative padding:** Flag sections with business justification, competitive analysis,
  or market research in the core sections -- these belong in Overflow Context
- **Requirements leakage:** Flag numbered requirements (FR-XXX), acceptance criteria,
  or architecture decisions -- these belong in the PRD/architecture, not the brief
- **Token estimate:** Estimate total tokens. If brief exceeds 1,500 tokens, warn that
  it will consume significant agent context budget during spec crystallization

### Implementation Leakage Check
Product briefs should describe WHAT, not HOW. Scan for technology names and
implementation choices that do not belong at the brief stage:

**Technology name scan -- flag occurrences of:**
- Frameworks: React, Next.js, Vue, Angular, Django, Rails, Spring Boot, Express, FastAPI, Svelte
- Databases: PostgreSQL, MySQL, MongoDB, Redis, DynamoDB, Supabase, SQLite
- Infrastructure: AWS, GCP, Azure, Docker, Kubernetes, Terraform, Vercel, Netlify
- Languages (when prescriptive): "must use Rust", "written in Python", "built with Go"
- Libraries: Redux, Prisma, Drizzle, tRPC, GraphQL (when prescriptive, not descriptive)

**Distinguish capability-relevant from leakage:**
- ACCEPTABLE: "Must work offline" (capability constraint)
- LEAKAGE: "Must use IndexedDB for offline storage" (implementation)
- ACCEPTABLE: "Must integrate with Slack" (integration constraint)
- LEAKAGE: "Must use Slack's Bolt SDK v3.12" (implementation)
- ACCEPTABLE: "Real-time collaborative editing" (capability)
- LEAKAGE: "WebSocket server using Socket.io" (implementation)

**Severity by section:**
- **Error:** Technology name in Scope/In-Scope or Success Criteria
- **Warning:** Technology name in Constraints section (may be legitimate)
- **Info:** Technology name in Overflow Context (acceptable)

### Information Density Check
Scan for low-density prose patterns that waste agent context budget:

**Anti-pattern categories:**

1. **Conversational filler:** "As you know," "It goes without saying," "Needless
   to say," "In today's world," "It is important to note that"

2. **Wordy phrases replaceable by single words:**
   - "in order to" -> "to"
   - "due to the fact that" -> "because"
   - "at this point in time" -> "now"
   - "in the event that" -> "if"
   - "a large number of" -> "many"
   - "has the ability to" -> "can"

3. **Redundant phrases:**
   - "past history" -> "history"
   - "future plans" -> "plans"
   - "end result" -> "result"
   - "each and every" -> "each"
   - "absolutely essential" -> "essential"

4. **Hedge words that weaken precision:**
   - "somewhat," "fairly," "rather," "quite"
   - "may potentially," "could possibly"
   - "in some cases" (without specifying which cases)

**Severity thresholds:**
- **Critical (>10 instances):** Brief needs rewriting for density
- **Warning (5-10 instances):** Flag specific instances for tightening
- **Pass (<5 instances):** Acceptable density

Count instances and report with specific line references and suggested rewrites.

### Completeness Check
- Brief is not just a title or one-liner
- Total content is at least 150 words (a real brief, not a placeholder)
- No sections contain only "TBD" or "TODO"

## Market Intelligence Cross-Check

Validate the brief's audience and pain claims against market intel findings:
- If the brief claims a pain that market intel marked as "unconfirmed," flag it
- If market intel identified risks not addressed in the brief, flag them
- If differentiation opportunities exist that the brief doesn't leverage, note them

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/brief-validation-template.md` for the brief validation report format.

## Output

Write validation report to `.factory/planning/brief-validation.md`:

| Section | Status | Finding |
|---------|--------|---------|
| [section] | PASS / FAIL / WEAK / BLOATED | [specific feedback] |

**Bloat Score:** [estimated tokens] / 1,500 recommended max -- [OK / WARNING / OVER]

Overall: VALID / NEEDS_WORK / INCOMPLETE / OVER_SPECIFIED
