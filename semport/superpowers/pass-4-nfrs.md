# Pass 4: Non-Functional Requirements

_Phase B convergence round 2 — **CONVERGED**. Round 2 was primarily corrective (fixed round 1 hallucinations in Pressure Taxonomy and eval-evidence line citation) plus additive (Meincke 2025 empirical anchor, writing-skills Iron Law verbatim, 4 new Governance NFR items)._

## Quality Guarantees

### TDD Enforcement (Rigidity)

`skills/test-driven-development/SKILL.md` frames TDD as a Rigid skill (`skills/using-superpowers/SKILL.md:110`). The Iron Law (line 33-34) and the "delete means delete" rule (lines 39-45) remove the agent's ability to partially comply.

### Verification Before Completion

`skills/verification-before-completion/SKILL.md:18-38` defines a 5-step gate function (IDENTIFY → RUN → READ → VERIFY → CLAIM) that must fire before every completion claim.

### Root Cause Discipline

`skills/systematic-debugging/SKILL.md:46-50` requires completing Phase 1 before proposing fixes. Iron Law at line 18-19.

### Iron Law Inventory

Passes 2+3 rounds 2/3 surfaced five Iron Laws across the Rigid-skill class. Reserved for skills where partial compliance is worse than non-compliance.

| Iron Law | Skill | File:line |
|---|---|---|
| NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST | test-driven-development | `test-driven-development/SKILL.md:33` |
| NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST | systematic-debugging | `systematic-debugging/SKILL.md:18` |
| NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE | verification-before-completion | `verification-before-completion/SKILL.md` |
| **NO SKILL WITHOUT A FAILING TEST FIRST** | writing-skills | **`writing-skills/SKILL.md:377`** (round 2 exact quote; header at :374 is `## The Iron Law (Same as TDD)`) |
| HARD-GATE on implementation until brainstorming approved | brainstorming | `brainstorming/SKILL.md` |

**Round 2 note**: the writing-skills Iron Law is explicitly captioned `## The Iron Law (Same as TDD)` at line 374 — the authoring discipline inherits TDD's verb structure verbatim, substituting "SKILL" for "PRODUCTION CODE". Confirms the canonical form `NO <verb> <scope> WITHOUT <prerequisite> FIRST` is deliberately reused, not coincidental.

The Iron Law pattern is the repo's mechanism for making the wrong thing impossible at the prompt-engineering layer. Flexible skills deliberately do NOT get Iron Laws; they get "Core principle" instead.

### TDD Enforcement Extensions

- **Verify-RED gate**: after writing a failing test, the agent MUST run it and observe a red result before proceeding.
- **Verify-GREEN gate**: after implementation, the agent MUST re-run and observe green. Skipping the re-run is a Red Flag.
- **Exception protocol**: explicit short-list of when TDD does not apply (pure exploration, throwaway spikes); "I'm just exploring" becomes a rationalization if the code is later kept.
- **Debugging integration**: "never fix bugs without a test." The debugging skill requires a failing reproduction test before any fix is proposed.

### Eval-Evidence Requirement (round 2 citation corrected)

BC-DRAFT-046 formalizes **`CLAUDE.md:67-74`** (round 1 cited `:47-52` — wrong). Key operative rules verbatim:

- "Skills are not prose - they are code that shapes agent behavior" (`CLAUDE.md:69`)
- "Run adversarial pressure testing across multiple sessions" (`CLAUDE.md:72`)
- "Show before/after eval results in your PR" (`CLAUDE.md:73`)
- "Do not modify carefully-tuned content (Red Flags tables, rationalization lists, 'human partner' language) without evidence the change is an improvement" (`CLAUDE.md:74`)

Behavior-shaping prose is treated as code; changes are rejected without evidence they shift agent outputs favorably. Pressure testing via `skills/writing-skills` is the sanctioned eval harness.

## Safety & Guardrails

### Anti-rationalization (Red Flags)

Every behavior-shaping skill ships with a Red Flags table listing the rationalizations the agent will try to use to skip the skill, paired with rebuttals. Primary guardrail against AI hallucination-to-convenience. Examples: `using-superpowers/SKILL.md:80-95` (12 rationalizations); similar tables in TDD, debugging, verification skills.

### HARD-GATE markup

`<HARD-GATE>` blocks (`skills/brainstorming/SKILL.md:12-14`) explicitly forbid implementation actions until approval. Tag-based, trust-based rail.

### Subagent context isolation

SDD and dispatching-parallel-agents both require fresh context per subagent.

### Subagent-Stop

`<SUBAGENT-STOP>` prevents recursive skill loading when a subagent is dispatched.

### Instruction Priority

Explicit 3-level priority: user instructions > superpowers skills > default system prompt (`skills/using-superpowers/SKILL.md:19-26`).

### Forbidden Gratitude Class

`skills/receiving-code-review/SKILL.md:27-38` formalizes banned phrases: "You're absolutely right!", "Great point!", "Excellent feedback!", "Let me implement that now" (pre-verification).

### Circle K Codephrase

`receiving-code-review` introduces "Circle K" as a user-invokable codephrase the human partner can drop to signal the agent is sliding into performative/compliant behavior.

### Unclear-Item Halt Rule

`skills/receiving-code-review/SKILL.md:40-57`: if ANY review item is unclear, STOP and ask before implementing any item.

### YAGNI Check

Pass 3 round 2: before adding scope, justify present need — not speculative future need.

### Plan Document No-Placeholders Class (round 2 CORRECTED)

Plan documents have a forbidden-token set. **Round 1 claimed the list included `XXX`, `???`, and ellipsis-as-content — WRONG.** The actual exact list at `skills/writing-plans/SKILL.md:109` reads verbatim:

> "TBD", "TODO", "implement later", "fill in details"

`XXX`, `???`, and ellipsis were round 1 over-extrapolation from the PR-template rule. The actual section header is `## No Placeholders` (line 106) and the full forbidden classification at lines 108-114 is broader than a token list:

- The four literal placeholder strings above (line 109).
- "Add appropriate error handling" / "add validation" / "handle edge cases" (line 110) — vague-verb placeholders also forbidden.
- "Write tests for the above" without actual test code (line 111).
- "Similar to Task N" without repeating the code (line 112) — justified by "the engineer may be reading tasks out of order".
- Steps that describe what to do without showing how (line 113).
- References to types/functions/methods not defined in any task (line 114).

**The rule is broader than a token list: it is an "every step must contain the actual content an engineer needs" invariant** (line 107). Violations are classified as **"plan failures"** (line 108, verbatim — newly-captured taxonomic term).

### Worktree Safety Invariants

`skills/using-git-worktrees` and `skills/finishing-a-development-branch`:

- Worktrees created under a gitignored parent directory.
- Finishing a branch requires explicit confirmation before deletion.
- Worktree creation must succeed (verification gate) before any work inside.

### Implementer STOP Triggers

When the agent recognizes it is in over its head (scope surprise, cascading unknowns, repeated test failures without hypothesis) it MUST halt and surface the situation to the human partner.

### File-Growth Escalation Rule

If a file being edited grows substantially beyond the planned delta, the agent must stop and escalate.

## Observability

- No metrics, no tracing, no structured logging infrastructure.
- Indirect observability via tests:
  - `tests/claude-code/analyze-token-usage.py` — token usage analysis
  - `tests/skill-triggering/run-test.sh` — grep-based assertion over Claude Code stream-json output (BC-DRAFT-045). Canonical behavioral observability harness.
  - Pressure testing via `skills/writing-skills` — subagent-based behavioral verification.

### Pressure Taxonomy (round 2 HALLUCINATION CORRECTED)

**Round 1 claimed the taxonomy was "time, authority, sunk cost, social proof, urgency, flattery, confusion" — this was partially fabricated.** The actual taxonomy at `skills/writing-skills/testing-skills-with-subagents.md:128-138` (named table `### Pressure Types`):

| Pressure | Example (verbatim from source) |
|---|---|
| **Time** | Emergency, deadline, deploy window closing |
| **Sunk cost** | Hours of work, "waste" to delete |
| **Authority** | Senior says skip it, manager overrides |
| **Economic** | Job, promotion, company survival at stake |
| **Exhaustion** | End of day, already tired, want to go home |
| **Social** | Looking dogmatic, seeming inflexible |
| **Pragmatic** | "Being pragmatic vs dogmatic" |

**Corrections**:
- "urgency" — does NOT exist (subsumed by Time)
- "flattery" — does NOT exist (forbidden as authoring principle under Liking ban, not a tested pressure)
- "confusion" — does NOT exist
- **Economic**, **Exhaustion**, and **Pragmatic** — round 1 missed entirely

"Pragmatic" is particularly distinctive: the repo treats **the anti-dogma stance itself as an attack vector** against discipline skills. The rationalization "I'm just being pragmatic" is a named first-class pressure.

**Compounding rule** (line 140, verbatim): "Best tests combine 3+ pressures." A single-pressure scenario is flagged as weak (line 342-344).

**Research foundation (round 2 NEW)**: pressure testing is grounded in **Meincke et al. (2025)**, cited in `skills/writing-skills/persuasion-principles.md:7` — **N=28,000 AI conversations, persuasion techniques more than doubled compliance (33% → 72%, p < .001)**. The compliance-under-pressure NFR is empirically anchored, not author-opinion.

## Reliability

- Zero-dependency by policy (`CLAUDE.md:31-33`).
- Polyglot hook dispatcher (`hooks/run-hook.cmd`) for Windows compatibility.
- Bash 5.3+ heredoc hang workaround documented inline (`hooks/session-start:44`, issue #571).
- **OpenCode idempotency invariant**: `experimental.chat.messages.transform` hook must be safe to invoke repeatedly.
- **Legacy skills-directory warning**: `hooks/session-start:12-15` detects pre-plugin install location.

## Performance / Context Budget

- Only `using-superpowers` is injected eagerly; all other skills are lazy-loaded.
- SDD's fresh-subagent-per-task pattern preserves parent context.
- **Token budgets per skill class**:
  - Flexible/reference skills: soft budget < 500 words
  - Discipline skills: < 200 words in core body before examples
  - Trigger-description field: hard ceiling 1024 chars

## Portability (Platform NFR)

First-class across Claude Code, Cursor, Codex, OpenCode, Gemini CLI, Copilot CLI. Platform detection in `hooks/session-start:49-67`. Portability encoded as **contracts**, not just docs: BC-031, BC-033, BC-045.

## Governance / PR-Hygiene (round 2 expanded)

`CLAUDE.md:1-86` establishes a governance NFR layer with no analog in most codebases. Round 2 full re-read corrected line citations (file is 86 lines, not 90) and added 4 items round 1 missed:

**Round 1 rules (verified)**:
- **94% PR rejection rate** as empirical justification (`CLAUDE.md:7, :61`).
- **PR template must be fully filled** (`:13, :23`).
- **Existing-PR search** open AND closed mandatory (`:14, :25`).
- **No speculative fixes** (`:15, :47-49`).
- **One problem per PR** (`:45, :83`).
- **Domain-specific skills → standalone plugins** (`:16, :41, :51-53`).
- **Third-party dependencies forbidden** except new-harness support (`:31-33`).
- **"Compliance" PRs rejected** without eval evidence (`:35-37`).
- **Fabricated / fork-specific / bundled-unrelated = immediate close** (`:55-57, :59-61, :63-65`).
- **Human-involvement evidence required** (`:17, :27`).

**Round 2 additions**:

- **"Understand the project before contributing"** (`CLAUDE.md:76-78`) — rejection class for PRs that "rewrite the project's voice or restructure its approach without understanding why it exists". Protects voice conventions at the governance layer.
- **"Test on at least one harness and report results in the environment table"** (`CLAUDE.md:84`) — concrete reproducibility NFR. Portability testing is not optional; every PR must exercise at least one of the six platforms and record the result.
- **"Describe the problem you solved, not just what you changed"** (`CLAUDE.md:85`) — PR-narrative convention elevated to requirement. Aligns with SOUL.md "Explain Why, Not Just What".
- **Framing rule** (`CLAUDE.md:9`, verbatim): "Your job is to protect your human partner from that outcome. Submitting a low-quality PR doesn't help them [...] That is being a tool of embarrassment." **Reframes the agent's objective function** from "maximize PR throughput" to "maximize human-partner reputation preservation". First-principles statement of the governance NFR's purpose.

This layer encodes a governance contract: the plugin's quality bar is maintained by rejecting low-quality contributions rather than by machine verification. The agent is framed as an accomplice to the human partner, not an autonomous contributor.

## Missing / Unexpected

- No formal verification. No fuzzing. No proofs. Quality is entirely empirical-behavioral.
- No versioning of individual skills — single plugin version.
- No dependency on specific model.
- No machine enforcement of the Governance NFR — entirely prose-and-reviewer enforced.

## Changes from round 1 (convergence corrections)

1. **Pressure Taxonomy hallucination corrected**: actual 7 types are Time / Sunk cost / Authority / Economic / Exhaustion / Social / Pragmatic. Round 1 fabricated "urgency/flattery/confusion" and missed Economic/Exhaustion/Pragmatic.
2. **Meincke 2025 empirical anchor added** (N=28000, 33%→72% compliance).
3. **Writing-skills Iron Law exact form captured**: `NO SKILL WITHOUT A FAILING TEST FIRST` at `writing-skills/SKILL.md:377`, explicitly labeled "Same as TDD".
4. **Eval-evidence citation corrected** from `CLAUDE.md:47-52` to `:67-74`.
5. **Governance NFR expanded** with 4 new items: understand-project rule, harness-test-table rule, problem-not-change narrative rule, "tool of embarrassment" framing statement.
