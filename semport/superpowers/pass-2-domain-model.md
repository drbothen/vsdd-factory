# Pass 2: Domain Model

_Phase B convergence round 1._

## Changes from Phase A

Phase A captured the skeleton (11 concepts). Round 1 expands: full `writing-skills` meta-skill vocabulary, `receiving-code-review` state machine, `finishing-a-development-branch` 4-option decision node, `dispatching-parallel-agents` boundary, multi-platform Skill Activation Mechanism abstraction, OpenCode plugin concepts, SDD status codes + model-selection tier, systematic-debugging Phase 4.5 architectural escape valve, legacy-migration warning, gate-function formalization, skill dependency graph, skill-priority rule.

## Core Concepts

### Skill (model-invoked)

A self-contained markdown file instructing the agent how to perform a specific kind of work. Defined by YAML frontmatter (`name`, `description`, max 1024 chars) and loaded via a platform-specific **Skill Activation Mechanism** (see below). Skills are **code that shapes agent behavior** (`CLAUDE.md:69`), not prose. Categorized as **Rigid** (TDD, debugging — follow exactly) or **Flexible** (patterns — adapt principles to context) (`skills/using-superpowers/SKILL.md:107-113`). Three **Skill Types** (`skills/writing-skills/SKILL.md:62-70`):

- **Technique** — concrete method with steps (condition-based-waiting, root-cause-tracing)
- **Pattern** — way of thinking about problems (flatten-with-flags)
- **Reference** — API docs, syntax guides, tool documentation

### Skill Activation Mechanism (abstract)

The platform-provided tool the agent uses to load a skill's full content on demand. Concrete instances (`skills/using-superpowers/SKILL.md:28-40`):

| Platform | Mechanism | Notes |
|---|---|---|
| Claude Code | `Skill` tool | Never use Read tool on skill files |
| Copilot CLI | `skill` tool | Auto-discovered from plugins |
| Gemini CLI | `activate_skill` tool | Metadata at session start, full content on demand |
| OpenCode | native `skill` tool | Skills path injected via config hook |

### Bootstrap Injection Protocol

The session-start mechanism that injects using-superpowers into the first turn. Platform-dependent JSON shape (`hooks/session-start:40-55`):

| Env detected | JSON shape |
|---|---|
| `CURSOR_PLUGIN_ROOT` set | `{"additional_context": "..."}` (snake_case) |
| `CLAUDE_PLUGIN_ROOT` set, `COPILOT_CLI` unset | `{"hookSpecificOutput": {"hookEventName": "SessionStart", "additionalContext": "..."}}` |
| Otherwise (Copilot CLI / SDK-standard) | `{"additionalContext": "..."}` (top-level) |

Claude Code reads BOTH `additional_context` and `hookSpecificOutput` without dedup — the hook MUST emit only the field the current platform consumes. OpenCode diverges further: injects via `experimental.chat.messages.transform` into the first USER message (not system), to avoid token bloat (`superpowers.js:97-110`, issues #750, #894). Idempotency guard: skip injection if `EXTREMELY_IMPORTANT` already present.

### Legacy Skills Directory Warning

If `~/.config/superpowers/skills` exists at session start, the hook injects an `<important-reminder>` the agent MUST surface in its first reply telling the user to migrate to `~/.claude/skills` (`hooks/session-start:12-15`).

### Command (user-invoked)

Slash-command entry point. In superpowers, all three commands are deprecation shims; functionality has moved to skills. Commands are for users who know exactly what they want; skills are for the agent to discover itself.

### Hook

Platform lifecycle callback. Superpowers uses only SessionStart (`hooks/hooks.json:4`), and only to inject the bootstrap skill.

### Subagent

A fresh, context-isolated Claude invocation dispatched by the primary agent. Never inherits parent context. Used for implementation, spec compliance review, code quality review, parallel investigation (`skills/dispatching-parallel-agents/SKILL.md:10-11`; `skills/subagent-driven-development/SKILL.md:6-13`).

### Implementer Status Codes

Discrete status returned by an implementer subagent (`skills/subagent-driven-development/implementer-prompt.md:100-113`):

| Status | Meaning | Controller action |
|---|---|---|
| DONE | Task complete, self-reviewed | Proceed to spec compliance review |
| DONE_WITH_CONCERNS | Complete but flagged doubts | Read concerns; address correctness issues before review |
| NEEDS_CONTEXT | Missing information | Provide context, re-dispatch |
| BLOCKED | Cannot complete | Diagnose: more context, more capable model, smaller tasks, or escalate to human |

### Model Selection Tier (SDD)

Match model power to task class to conserve cost/speed (`skills/subagent-driven-development/SKILL.md:87-100`):

- Mechanical implementation (1-2 files, complete spec) → cheap/fast model
- Integration/judgment (multi-file, debugging) → standard model
- Architecture/design/review → most capable model

### Subagent-Driven Development (SDD)

Workflow where every plan task is delegated to a fresh implementer subagent, then verified by TWO review subagents in strict order: **spec compliance reviewer** first, **code quality reviewer** second. Never start quality review until spec review returns ✅ (`skills/subagent-driven-development/SKILL.md:247`). Never dispatch implementer subagents in parallel — conflicts (`SKILL.md:240`). The controller MUST provide full task text inline; implementer MUST NOT read the plan file (`SKILL.md:241`).

### Two-Stage Review

The ordered pair (spec reviewer → quality reviewer). Spec reviewer's mandate: verify by reading code, NOT by trusting the implementer report; implementer "finished suspiciously quickly" is the default framing (`spec-reviewer-prompt.md:21-37`). Quality reviewer's scope: one-responsibility-per-file, decomposition, plan-structure adherence, newly-created-large files (`code-quality-reviewer-prompt.md:20-24`).

### Implementer Self-Review Rubric

Mandatory introspection before reporting DONE, four categories (`implementer-prompt.md:74-98`):

- **Completeness** — did I implement everything? edge cases?
- **Quality** — best work? clear names? clean code?
- **Discipline** — YAGNI? only what was requested? existing patterns?
- **Testing** — tests verify behavior (not just mocks)? TDD followed? comprehensive?

### Dispatching Parallel Agents (distinct from SDD)

Used for **independent investigations** only (bug triage across unrelated test files), never for implementation. Boundary rules (`skills/dispatching-parallel-agents/SKILL.md:36-46, 127-132`):

- 3+ distinct failures
- Each in independent problem domain (no shared state)
- Don't use when failures might be related
- Don't use for exploratory debugging

Contrast with SDD: SDD is sequential implementation with review; dispatching-parallel-agents is concurrent investigation without review cycles.

### Systematic Debugging (Four Phases + Architectural Escape Valve)

Iron Law: `NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST` (`skills/systematic-debugging/SKILL.md:18-19`). Four phases with strict sequencing (`SKILL.md:46-213`):

1. **Root Cause Investigation** — read errors, reproduce, check recent changes, gather multi-layer evidence, trace data flow backward to source
2. **Pattern Analysis** — find working examples, compare references, identify every difference, understand dependencies
3. **Hypothesis and Testing** — single hypothesis written down, minimal test, one variable
4. **Implementation** — failing test FIRST, single fix, verify

**Phase 4.5 Architectural Escape Valve:** after 3 failed fixes, STOP and question the architecture instead of attempting fix #4. Pattern indicators: each fix reveals new shared state/coupling, fixes require "massive refactoring", each fix creates new symptoms elsewhere. Discuss with human partner before continuing (`SKILL.md:192-213`).

**Partner-Signal Interrupts** (`SKILL.md:234-243`) — phrases that mean STOP and return to Phase 1: "Is that not happening?", "Will it show us...?", "Stop guessing", "Ultrathink this", "We're stuck?"

### Verification-Before-Completion Gate Function

Iron Law: `NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE` (`skills/verification-before-completion/SKILL.md:18-19`). A 5-step sequential function — each step is a sub-contract. Skipping any step = "lying, not verifying" (`SKILL.md:26-38`):

1. IDENTIFY: what command proves this claim?
2. RUN: execute the FULL command (fresh, complete)
3. READ: full output, exit code, count failures
4. VERIFY: output confirms claim?
5. CLAIM: state claim WITH evidence

Applies to ALL variations (exact phrases, paraphrases, implications, ANY communication suggesting completion).

### Brainstorming Workflow (9-step checklist, HARD-GATE, Visual Companion)

Nine-item checklist (`skills/brainstorming/SKILL.md:22-32`):

1. Explore project context
2. Offer Visual Companion (own message, no combined content)
3. Ask clarifying questions (one at a time)
4. Propose 2-3 approaches with recommendation
5. Present design in scaled sections, approve per section
6. Write design doc to `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`, commit
7. Spec self-review (placeholder / contradiction / scope / ambiguity)
8. User reviews written spec
9. Transition to writing-plans skill

**Terminal state:** invoking writing-plans, and ONLY writing-plans (`SKILL.md:66`). The HARD-GATE (`SKILL.md:12-14`) forbids any implementation skill or code before user approval of the design. **Visual Companion** is a tool, not a mode — per-question decision: "would the user understand this better by seeing it than reading it?" (`SKILL.md:156-162`).

### Receiving Code Review (State Machine)

6-state pipeline (`skills/receiving-code-review/SKILL.md:14-25`):

READ (complete feedback without reacting) → UNDERSTAND (restate or ask) → VERIFY (check against codebase reality) → EVALUATE (technically sound for THIS codebase?) → RESPOND (technical acknowledgment or reasoned pushback) → IMPLEMENT (one item at a time, test each).

**Source-specific sub-handling** (`SKILL.md:59-86`):

- From human partner: trusted, implement after understanding, no performative agreement
- From external reviewer: 5-check gate — correct for THIS codebase? breaks existing? reason for current impl? works all platforms? reviewer has full context?

**Forbidden Gratitude Class** (`SKILL.md:28-33, 132-148`) — "You're absolutely right!", "Great point!", "Thanks for X", ANY gratitude expression. Catch-and-delete rule: if about to type "Thanks", DELETE IT.

**Unclear-item rule** (`SKILL.md:42-48`): if ANY item is unclear, STOP entire implementation (items may be related; partial understanding = wrong implementation).

**YAGNI Check** (`SKILL.md:88-98`): if reviewer demands "proper" feature, grep codebase first; if unused, propose removal.

**Discomfort signal codephrase** (`SKILL.md:129`): "Strange things are afoot at the Circle K" — tells partner the agent is uncomfortable pushing back.

### Finishing a Development Branch (4-Option Decision Node)

Sequential process (`skills/finishing-a-development-branch/SKILL.md`):

1. Verify tests pass (STOP if failing)
2. Determine base branch
3. Present EXACTLY 4 options with no explanation:
   - Merge to base locally
   - Push and create PR
   - Keep branch as-is
   - Discard
4. Execute choice
5. Cleanup worktree (Options 1 & 4 only; Options 2 & 3 keep worktree)

**Discard gate**: requires typed "discard" string confirmation (`SKILL.md:116-124`). **Terminal skill**: invoked by both SDD (final step after all tasks) and executing-plans (Step 3).

### Writing-Skills (Meta-Skill)

TDD applied to process documentation (`skills/writing-skills/SKILL.md`). 655 LOC — the single largest skill.

**TDD Mapping** (`SKILL.md:32-45`):

| TDD | Skill creation |
|---|---|
| Test case | Pressure scenario with subagent |
| Production code | SKILL.md |
| Test fails (RED) | Agent violates rule WITHOUT skill (baseline) |
| Test passes (GREEN) | Agent complies WITH skill |
| Refactor | Close loopholes while maintaining compliance |

**Iron Law**: `NO SKILL WITHOUT A FAILING TEST FIRST`. Applies to NEW skills AND EDITS (`SKILL.md:374-393`). No exceptions for "simple additions" or "documentation updates".

**Skill-Type Test Methodology** (`SKILL.md:396-442`), four kinds:

- Discipline-enforcing (TDD, verification): academic + pressure scenarios combining time + sunk cost + exhaustion pressures
- Technique: application + variation + missing-info scenarios
- Pattern: recognition + application + counter-example scenarios
- Reference: retrieval + application + gap scenarios

**Claude Search Optimization (CSO)** (`SKILL.md:140-277`): the discipline of making skills findable by future Claude. Core rule: **description = WHEN to use, NOT WHAT the skill does**. Empirically validated — a description summarizing workflow causes Claude to follow the description and SKIP the skill body. "code review between tasks" caused ONE review; "Use when executing implementation plans with independent tasks" caused correct TWO reviews.

**Token budgets**: getting-started skills <150 words, frequently-loaded <200 total, others <500. Techniques: move details to `--help`, cross-reference other skills by name (no `@` force-loading).

**Pressure Testing Protocol**: RED (run scenario without skill, capture verbatim rationalizations) → GREEN (write minimal skill addressing those specific rationalizations) → REFACTOR (find new rationalizations, plug, re-test until bulletproof).

**Rationalization Table + Red Flags List**: every skill enforcing discipline builds both from baseline testing.

**Persuasion Principles** (`persuasion-principles.md`): supporting file grounding discipline-skill authoring in Cialdini (2021) and Meincke et al. (2025) — authority, commitment, scarcity, social proof, unity.

### Red-Flag Detection (generalized)

Every critical skill embeds a table of **rationalizations** the agent might use to skip the skill, paired with rebuttals. Behavioral inoculation against convergent failure modes. Examples: using-superpowers (12 rationalizations), writing-skills (8), verification (8), TDD (multiple), systematic-debugging (8).

### Skill Priority Rule

When multiple skills could apply (`skills/using-superpowers/SKILL.md:98-104`):

1. **Process skills first** (brainstorming, debugging) — determine HOW to approach the task
2. **Implementation skills second** (frontend-design, mcp-builder) — guide execution

"Let's build X" → brainstorming first. "Fix this bug" → debugging first.

### User Instructions Priority

Three-tier precedence (`skills/using-superpowers/SKILL.md:19-26`):

1. **User instructions** (CLAUDE.md, GEMINI.md, AGENTS.md, direct requests) — highest
2. **Superpowers skills** — override default system behavior
3. **Default system prompt** — lowest

Resolves the apparent tension between "skills are mandatory" and user control. **User instructions say WHAT, not HOW** (`SKILL.md:116-117`) — "add X" does not mean skip workflows.

### TDD Enforcement ("Iron Law")

`NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST` (`skills/test-driven-development/SKILL.md:33-34`). Violation remedy: delete the code, start over. Forbids "reference" retention or "adapting" pre-test code. Exceptions (throwaway prototypes, generated code, config) require asking the human partner first.

### Iron Law (generalized pattern)

A non-negotiable rule embedded in a skill. Inventory:

| Skill | Iron Law |
|---|---|
| test-driven-development | NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST |
| systematic-debugging | NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST |
| verification-before-completion | NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE |
| writing-skills | NO SKILL WITHOUT A FAILING TEST FIRST |
| brainstorming | (HARD-GATE) NO IMPLEMENTATION BEFORE DESIGN APPROVAL |

All Iron Laws share the same structure: absolute prohibition + violation remedy (start over / delete / STOP) + explicit "violating the letter = violating the spirit" clause + rationalization table.

### HARD-GATE

Markdown tag marking non-negotiable blocking conditions. Used in brainstorming (`skills/brainstorming/SKILL.md:12-14`). Not machine-enforced — relies on the agent parsing the tag and self-gating.

### EXTREMELY-IMPORTANT / SUBAGENT-STOP

Tag-based directives: `<EXTREMELY-IMPORTANT>` for absolute rules (`skills/using-superpowers/SKILL.md:10-16`); `<SUBAGENT-STOP>` to exempt dispatched subagents from the bootstrap (`skills/using-superpowers/SKILL.md:6-8`) — prevents recursive bootstrap loading.

## Skill Dependency Graph

```
using-superpowers (session bootstrap, root)
  |
  +-- brainstorming (HARD-GATE) --> writing-plans --> (SDD | executing-plans)
  |                                                       |
  |                                                       +--> using-git-worktrees (REQUIRED prerequisite)
  |                                                       |
  |                                                       +--> test-driven-development (per task)
  |                                                       |
  |                                                       +--> finishing-a-development-branch (terminal)
  |
  +-- systematic-debugging
  |     +-- verification-before-completion (for fix verification)
  |     +-- test-driven-development (Phase 4 test creation)
  |
  +-- receiving-code-review (on review feedback)
  +-- requesting-code-review (on asking for review)
  +-- dispatching-parallel-agents (independent investigations only)
  +-- writing-skills (meta — when authoring/editing skills)
```

## Ubiquitous Language Glossary

- **Your human partner** — deliberate term for the user; collaborative/protective framing (`CLAUDE.md:9`). NOT interchangeable with "the user".
- **Skill** — behavior-shaping reference document.
- **Iron Law** — non-negotiable rule within a skill.
- **HARD-GATE** — tagged blocking condition.
- **Red Flag** — a rationalization the agent must catch and reject.
- **Pressure testing** — adversarial scenarios against a skill to verify it holds.
- **Rationalization Table** — captured excuses + rebuttals from baseline testing.
- **Baseline** — agent behavior WITHOUT the skill (the RED state for writing-skills).
- **RED/GREEN/REFACTOR** — TDD cycle states; also skill-authoring cycle states.
- **Fresh subagent** — context-isolated delegate.
- **Spec reviewer / code quality reviewer** — the two ordered stages of post-implementation review.
- **DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT** — implementer status codes.
- **CSO (Claude Search Optimization)** — discipline of making skills findable via description/keyword/naming.
- **Gate Function** — 5-step verification sequence.
- **Phase 4.5 Architectural Escape Valve** — 3+ failed fixes → stop, question architecture.
- **Circle K signal** — "Strange things are afoot at the Circle K", codephrase for pushback discomfort.
- **Visual Companion** — optional browser tool for visual brainstorming questions.
- **Skill Activation Mechanism** — abstract: `Skill` / `skill` / `activate_skill` across platforms.
- **Bootstrap Injection Protocol** — SessionStart hook JSON shape per platform.

## State Machines

- **Brainstorming**: explore → visual-companion-offer? → clarify → approaches → design-sections → approve-per-section → write-spec → self-review → user-reviews → transition-to-writing-plans (`skills/brainstorming/SKILL.md:22-32`)
- **TDD cycle**: red → verify_red → green → verify_green → refactor → verify_green → next → red (`skills/test-driven-development/SKILL.md:49-68`)
- **SDD per-task loop**: dispatch-implementer → {ask-questions? → answer → re-dispatch} → implementer-done(status) → {spec-reviewer → {fix-spec-gaps → re-review}*} → spec-✅ → {quality-reviewer → {fix-quality → re-review}*} → quality-✅ → mark-complete (`skills/subagent-driven-development/SKILL.md:42-85`)
- **Debugging**: Phase1 → Phase2 → Phase3 → Phase4 → (fix#<3? → Phase1 : Phase4.5 architectural-stop) (`skills/systematic-debugging/SKILL.md:48-213`)
- **Verification Gate**: IDENTIFY → RUN → READ → VERIFY → CLAIM (sequential, no skipping) (`skills/verification-before-completion/SKILL.md:26-38`)
- **Receiving code review**: READ → UNDERSTAND → VERIFY → EVALUATE → RESPOND → IMPLEMENT (one-at-a-time, test each) (`skills/receiving-code-review/SKILL.md:14-25`)
- **Finishing branch**: verify-tests → determine-base → present-4-options → execute{merge|pr|keep|discard} → cleanup-worktree? (`skills/finishing-a-development-branch/SKILL.md`)
- **Skill authoring (writing-skills RED-GREEN-REFACTOR)**: baseline-pressure-test (RED) → write-minimal-skill (GREEN) → close-loopholes-iterate (REFACTOR) (`skills/writing-skills/SKILL.md:533-560`)

## No Persistent Domain Entities

Superpowers has no runtime data model, no state files, no database. The "domain" is conversational behavior. Artifacts it produces:

- `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` (brainstorming output)
- `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` (writing-plans output)

These are outputs of the workflow, not managed domain entities.
