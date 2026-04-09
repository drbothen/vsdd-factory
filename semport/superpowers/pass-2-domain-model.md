# Pass 2: Domain Model

_Phase B convergence round 3 — **CONVERGED**. Round 3 read full bodies of `executing-plans`, `subagent-driven-development`, SDD fixtures, and test runners; 0 substantive findings, 4 nitpicks. No new domain entities. Round 2 content is final._

_Phase B convergence round 2._

## Changes from round 1

Round 1 captured the skill catalog, state machines, and dependency graph. Round 2 deepens: Plan Document as a first-class domain entity with schema, Worktree Directory-Selection + safety-verification domain, Code-Reviewer AGENT (distinct from the subagent prompt template), Review Severity Taxonomy, Persuasion-Principles authoring matrix, Execution Handoff decision node, writing-plans inline Self-Review (contrasted with SDD two-stage review), and the file-growth escalation concept for implementers.

## Changes from Phase A

Phase A captured the skeleton (11 concepts). Round 1 expanded to full `writing-skills` meta-skill vocabulary, `receiving-code-review` state machine, `finishing-a-development-branch` 4-option decision node, `dispatching-parallel-agents` boundary, multi-platform Skill Activation Mechanism abstraction, OpenCode plugin concepts, SDD status codes + model-selection tier, systematic-debugging Phase 4.5 architectural escape valve, legacy-migration warning, gate-function formalization, skill dependency graph, skill-priority rule.

## Core Concepts

### Skill (model-invoked)

A self-contained markdown file instructing the agent how to perform a specific kind of work. Defined by YAML frontmatter (`name`, `description`, max 1024 chars) and loaded via a platform-specific **Skill Activation Mechanism**. Skills are **code that shapes agent behavior** (`CLAUDE.md:69`), not prose. Categorized as **Rigid** (TDD, debugging — follow exactly) or **Flexible** (patterns — adapt principles to context) (`skills/using-superpowers/SKILL.md:107-113`). Three **Skill Types** (`skills/writing-skills/SKILL.md:62-70`):

- **Technique** — concrete method with steps (condition-based-waiting, root-cause-tracing)
- **Pattern** — way of thinking about problems (flatten-with-flags)
- **Reference** — API docs, syntax guides, tool documentation

### Skill Activation Mechanism (abstract)

The platform-provided tool the agent uses to load a skill's full content on demand (`skills/using-superpowers/SKILL.md:28-40`):

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

Slash-command entry point. In superpowers, all three commands are deprecation shims; functionality has moved to skills. **Round 2 detail:** the deprecation shims (`commands/brainstorm.md`, `commands/write-plan.md`, `commands/execute-plan.md`) contain only a single imperative sentence telling the agent to inform the human partner the command is deprecated and name the replacement skill — they perform no work themselves.

### Hook

Platform lifecycle callback. Superpowers uses only SessionStart (`hooks/hooks.json:4`), and only to inject the bootstrap skill.

### Subagent

A fresh, context-isolated Claude invocation dispatched by the primary agent. Never inherits parent context. Used for implementation, spec compliance review, code quality review, parallel investigation (`skills/dispatching-parallel-agents/SKILL.md:10-11`; `skills/subagent-driven-development/SKILL.md:6-13`).

### Agent (round 2 — distinct from subagent template)

`agents/code-reviewer.md` is the SOLE agent-file artifact in the repo (distinct from the subagent prompt TEMPLATES under `skills/subagent-driven-development/`). It declares a `model: inherit` frontmatter and a six-part mandate: (1) Plan Alignment Analysis, (2) Code Quality Assessment, (3) Architecture and Design Review, (4) Documentation and Standards, (5) Issue Identification and Recommendations, (6) Communication Protocol (`agents/code-reviewer.md:10-47`). The same file defines the canonical **Review Issue Severity Taxonomy** used across the project: **Critical** (must fix), **Important** (should fix), **Suggestions** (nice to have) (`agents/code-reviewer.md:38`). Round 1 conflated this agent with the prompt template — they are distinct: the TEMPLATE lives under `skills/requesting-code-review/code-reviewer.md`, the AGENT lives under `agents/code-reviewer.md`.

### Review Issue Severity Taxonomy (round 2)

Three universal tiers across requesting-code-review, quality review, and the code-reviewer agent:

| Tier | Treatment | Source |
|---|---|---|
| Critical | Fix immediately; do not proceed | `skills/requesting-code-review/SKILL.md:44, 96`; `agents/code-reviewer.md:38` |
| Important | Fix before next task; do not compound | `skills/requesting-code-review/SKILL.md:45, 97` |
| Minor / Suggestions | Defer; note for later | `skills/requesting-code-review/SKILL.md:46`; `agents/code-reviewer.md:38` |

### Plan Document (round 2 — first-class entity)

A markdown artifact saved to `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` (`skills/writing-plans/SKILL.md:18`) that is the handoff between brainstorming and any execution skill (SDD or executing-plans). Has a strict schema:

**Required header** (`writing-plans/SKILL.md:47-61`): includes a frontmatter-style block naming **REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans** — i.e., the plan file itself carries a machine-readable directive telling any future agent which skill must execute it. Must also contain `**Goal:**`, `**Architecture:**`, `**Tech Stack:**`.

**Task structure** (`writing-plans/SKILL.md:64-104`): Each Task carries a **Files** block (Create/Modify/Test with exact paths; Modify includes `path:line-line` ranges) and checkbox steps using `- [ ]` syntax.

**Step granularity** (`writing-plans/SKILL.md:36-44`): each step is exactly one 2-5 minute action. Canonical sequence per feature unit: write failing test → run to verify it fails → write minimal code → run to verify it passes → commit. A plan step that describes WHAT to do without SHOWING HOW (code block) is malformed.

**No Placeholders class** (`writing-plans/SKILL.md:106-114`): a closed set of forbidden plan tokens, each counting as a plan failure:

- "TBD", "TODO", "implement later", "fill in details"
- "Add appropriate error handling" / "add validation" / "handle edge cases"
- "Write tests for the above" without the actual test code
- "Similar to Task N" (must be inlined — engineer may read tasks out of order)
- Steps describing actions without code blocks
- References to types/functions/methods not defined in any task

**Plan Self-Review** (`writing-plans/SKILL.md:122-132`, ROUND 2): an **inline 3-check pass** by the plan author — NOT a subagent dispatch — consisting of (1) spec coverage, (2) placeholder scan, (3) type/name consistency across tasks. Explicitly: "This is a checklist you run yourself — not a subagent dispatch." This distinguishes plan self-review from the SDD **two-stage subagent review** (spec-reviewer → quality-reviewer). They are different artifacts in different phases of the workflow.

**Execution Handoff** (`writing-plans/SKILL.md:134-152`, ROUND 2): after save, writing-plans MUST present exactly two execution options to the human partner:

- Option 1 Subagent-Driven (recommended) — fresh subagent per task + two-stage review — proceeds via superpowers:subagent-driven-development
- Option 2 Inline Execution — batch execution with checkpoints — proceeds via superpowers:executing-plans

The handoff is a decision NODE between writing-plans and {SDD | executing-plans}; the plan document terminates writing-plans but does not auto-select an executor.

### Worktree Domain (round 2 — first-class)

`skills/using-git-worktrees/SKILL.md` models a decision and creation pipeline:

**Directory-selection precedence ladder** (`:18-49`, deterministic, round 2):

1. Existing directories, priority `.worktrees/` > `worktrees/`. If both, `.worktrees/` wins.
2. If none, grep `CLAUDE.md` for a `worktree.*director` preference; use without asking.
3. Else ask human partner between `.worktrees/` (project-local, hidden) and `~/.config/superpowers/worktrees/<project>/` (global).

**Safety verification invariant** (`:52-70`): for project-local directories, the directory MUST be gitignore-covered before creation. Verified via `git check-ignore`. If not ignored: add line to `.gitignore`, commit, then proceed. Codifies Jesse's "Fix broken things immediately" rule. Global directory is exempt (outside project).

**Setup auto-detection** (`:102-118`): pipeline branches on project type — `package.json` → npm install; `Cargo.toml` → cargo build; `requirements.txt` → pip install -r; `pyproject.toml` → poetry install; `go.mod` → go mod download.

**Clean baseline gate** (`:120-134`): tests must run clean after worktree creation. On failure, agent reports and asks permission before proceeding. "Can't distinguish new bugs from pre-existing issues" is the stated rationale.

**Integration points** (`:210-218`): called by brainstorming (Phase 4), subagent-driven-development, executing-plans as a REQUIRED prerequisite; paired with finishing-a-development-branch for cleanup.

### Implementer Status Codes

Discrete status returned by an implementer subagent (`skills/subagent-driven-development/implementer-prompt.md:100-113`):

| Status | Meaning | Controller action |
|---|---|---|
| DONE | Task complete, self-reviewed | Proceed to spec compliance review |
| DONE_WITH_CONCERNS | Complete but flagged doubts | Read concerns; address correctness issues before review |
| NEEDS_CONTEXT | Missing information | Provide context, re-dispatch |
| BLOCKED | Cannot complete | Diagnose: more context, more capable model, smaller tasks, or escalate to human |

**Round 2 escalation triggers** (`implementer-prompt.md:58-67`): "in over your head" contract names explicit STOP conditions — (a) task requires architectural decisions with multiple valid approaches, (b) needs code understanding beyond what was provided with no clarity path, (c) uncertain the approach is correct, (d) task involves restructuring outside what the plan anticipated, (e) reading file after file without progress. Escalation format: status BLOCKED or NEEDS_CONTEXT with specifics of what's stuck + what was tried + what kind of help is needed.

**Round 2 file-growth escalation** (`implementer-prompt.md:45-56`): if a file the implementer is creating grows beyond the plan's intent, implementer MUST NOT unilaterally split — MUST report DONE_WITH_CONCERNS. Mirrored concern-raising clause for modifying pre-existing large/tangled files.

### Model Selection Tier (SDD)

Match model power to task class to conserve cost/speed (`skills/subagent-driven-development/SKILL.md:87-100`):

- Mechanical implementation (1-2 files, complete spec) → cheap/fast model
- Integration/judgment (multi-file, debugging) → standard model
- Architecture/design/review → most capable model

### Subagent-Driven Development (SDD)

Workflow where every plan task is delegated to a fresh implementer subagent, then verified by TWO review subagents in strict order: **spec compliance reviewer** first, **code quality reviewer** second. Never start quality review until spec review returns correct (`skills/subagent-driven-development/SKILL.md:247`). Never dispatch implementer subagents in parallel — conflicts (`SKILL.md:240`). The controller MUST provide full task text inline; implementer MUST NOT read the plan file (`SKILL.md:241`).

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

**Phase 4.5 Architectural Escape Valve:** after 3 failed fixes, STOP and question the architecture instead of attempting fix #4.

**Partner-Signal Interrupts** (`SKILL.md:234-243`) — phrases that mean STOP and return to Phase 1: "Is that not happening?", "Will it show us...?", "Stop guessing", "Ultrathink this", "We're stuck?"

**Supporting techniques (round 2):**

- **Root-Cause Tracing** (`root-cause-tracing.md:1-24`) — "trace backward through the call chain until you find the original trigger, then fix at the source; also add defense-in-depth at symptom point if feasible." Canonical example: empty-string `projectDir` cascading into `git init` in the source tree.
- **Condition-Based Waiting** (`condition-based-waiting.md:1-55`) — "wait for the actual condition you care about, not a guess about how long it takes"; bans arbitrary `setTimeout`/`sleep` in tests except when explicitly testing timing behavior, in which case rationale must be documented.

### Verification-Before-Completion Gate Function

Iron Law: `NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE` (`skills/verification-before-completion/SKILL.md:18-19`). A 5-step sequential function — each step is a sub-contract. Skipping any step = "lying, not verifying" (`SKILL.md:26-38`):

1. IDENTIFY: what command proves this claim?
2. RUN: execute the FULL command (fresh, complete)
3. READ: full output, exit code, count failures
4. VERIFY: output confirms claim?
5. CLAIM: state claim WITH evidence

### Brainstorming Workflow

Nine-item checklist (`skills/brainstorming/SKILL.md:22-32`) terminating in writing-plans exclusively (`SKILL.md:66`). HARD-GATE (`SKILL.md:12-14`) forbids any implementation skill or code before user approval of the design. **Visual Companion** is a tool, not a mode — per-question decision: "would the user understand this better by seeing it than reading it?" (`SKILL.md:156-162`; `brainstorming/visual-companion.md:7-26`). Round 2: visual-companion is a browser-based file-drop server writing content HTML files to a `screen_dir` and recording click selections to `state_dir/events`; **content fragments (not full documents) are the default** since the server auto-wraps them in the frame template (`visual-companion.md:27-31`).

### Receiving Code Review (State Machine)

6-state pipeline: READ → UNDERSTAND → VERIFY → EVALUATE → RESPOND → IMPLEMENT (`skills/receiving-code-review/SKILL.md:14-25`).

**Source-specific sub-handling**, **Forbidden Gratitude Class**, **Unclear-item rule**, **YAGNI Check**, **Circle K discomfort signal** — see round 1 details.

### Requesting Code Review (round 2)

Dispatch skill for the code-reviewer subagent/agent (`skills/requesting-code-review/SKILL.md`). Mandatory invocation points: (a) after each task in SDD, (b) after completing a major feature, (c) before merge to main (`:14-17`). Placeholder-templated prompt passed to the Task tool: `{WHAT_WAS_IMPLEMENTED}`, `{PLAN_OR_REQUIREMENTS}`, `{BASE_SHA}`, `{HEAD_SHA}`, `{DESCRIPTION}` (`:36-42`). BASE_SHA / HEAD_SHA are derived from git so the reviewer sees exactly the diff in scope, not the session history. **Feedback triage contract** (`:43-47`): Critical → fix immediately, Important → fix before proceeding, Minor → note for later.

### Finishing a Development Branch (4-Option Decision Node)

Sequential process (`skills/finishing-a-development-branch/SKILL.md`): verify tests pass → determine base branch → present exactly 4 options (merge locally / push+PR / keep / discard) → execute → cleanup worktree (merge and discard only). Discard gate requires typed "discard" confirmation.

### Writing-Skills (Meta-Skill)

TDD applied to process documentation. 655 LOC. Iron Law: `NO SKILL WITHOUT A FAILING TEST FIRST`.

**Pressure Testing Protocol** (round 2 detail, `testing-skills-with-subagents.md:1-116`): RED (run scenario without skill, capture verbatim rationalizations) → Verify RED → GREEN (write minimal skill addressing only those specific rationalizations — no hypothetical extras) → Verify GREEN → REFACTOR → Stay GREEN. Pressure scenarios REQUIRE 3+ combined pressures drawn from: Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic (`testing-skills-with-subagents.md:130-141`). **Meta-testing protocol** (`:240-266`): if agent chooses wrong option despite the skill, ask "how could the skill have been written to make option A the only answer?" **Bulletproof signals** (`:268-280`): correct choice under max pressure + cites skill sections + acknowledges temptation + meta-test confirms clarity.

**Persuasion Principles** (round 2, `persuasion-principles.md` in full): grounded in Cialdini (2021) and Meincke et al. (2025) — N=28,000 AI conversations, compliance rates more than doubled (33% → 72%, p < .001) with persuasion techniques. Seven principles with prescriptive use-matrix:

| Principle | Use for skill type | Mechanism in skill text |
|---|---|---|
| Authority | Discipline-enforcing | "YOU MUST", "Never", "No exceptions" — eliminates decision fatigue |
| Commitment | Discipline, multi-step | Required announcements ("I'm using the X skill"), forced A/B/C choices, TodoWrite |
| Scarcity | Immediate-verification, sequential deps | "Before proceeding", "Immediately after X" |
| Social proof | Universal practices | "Every time", "X without Y = failure" |
| Unity | Collaborative workflows | "our codebase", "we're colleagues" |
| Reciprocity | Use sparingly, rarely needed | — |
| Liking | **NEVER use for compliance** — conflicts with honest feedback, creates sycophancy | — |

Skill-type combinations: discipline → Authority + Commitment + Social Proof (avoid Liking/Reciprocity). Guidance → moderate Authority + Unity. Collaborative → Unity + Commitment. Reference → clarity only, no persuasion. Ethical test: "Would this technique serve the user's genuine interests if they fully understood it?" (`persuasion-principles.md:154-165`).

**Anthropic best-practices tension** (round 2, `anthropic-best-practices.md:1-200`; `CLAUDE.md:35-37`): the repo includes `skills/writing-skills/anthropic-best-practices.md` as a reference copy of Anthropic's own guidance. Simultaneously, `CLAUDE.md:35-37` states: "Our internal skill philosophy DIFFERS from Anthropic's published guidance... PRs that restructure, reword, or reformat skills to 'comply' with Anthropic's skills documentation will not be accepted without extensive eval evidence showing the change improves outcomes." The file is kept for reference and as a contrast point, not as a rulebook. This is a deliberate governance tension: the project treats Anthropic's guidance as ONE input, not authority, and requires eval evidence for any compliance PR.

### Red-Flag Detection (generalized)

Every critical skill embeds a table of **rationalizations** the agent might use to skip the skill, paired with rebuttals. Examples: using-superpowers (12), writing-skills (8), verification (8), TDD (multiple), systematic-debugging (8).

### Skill Priority Rule

When multiple skills could apply (`skills/using-superpowers/SKILL.md:98-104`): process skills first (brainstorming, debugging) → implementation skills second.

### User Instructions Priority

Three-tier precedence (`skills/using-superpowers/SKILL.md:19-26`): user instructions > superpowers skills > default system prompt.

### TDD Enforcement (Iron Law + round 2 sub-gates)

`NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST` (`skills/test-driven-development/SKILL.md:33-34`). Violation remedy: delete the code, start over. Round 2 sub-rules:

- **RED verification gate** (`SKILL.md:114-128`): MANDATORY, never skip. Confirm test fails (not errors), failure message expected, fails because feature missing (not typo). If test passes, it was testing existing behavior — fix the test. If test errors, fix error until it fails correctly.
- **GREEN verification gate** (`SKILL.md:168-184`): MANDATORY. Confirm test passes AND other tests still pass AND output pristine (no warnings/errors).
- **"Delete means delete" loophole closures** (`SKILL.md:37-45`): no keeping as reference, no "adapting" while writing tests, no looking at it.
- **Verification checklist** (`SKILL.md:328-340`): 8-item final check; inability to check all = skipped TDD = start over.
- **When-stuck table** (`SKILL.md:343-349`): "don't know how to test" → write wished-for API; "test too complicated" → design too complicated, simplify; "must mock everything" → code too coupled, use dependency injection; "test setup huge" → extract helpers.
- **Debugging integration** (`SKILL.md:351-355`): bugs MUST be reproduced via failing test before fix attempted; "never fix bugs without a test."

### Iron Law (generalized pattern)

| Skill | Iron Law |
|---|---|
| test-driven-development | NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST |
| systematic-debugging | NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST |
| verification-before-completion | NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE |
| writing-skills | NO SKILL WITHOUT A FAILING TEST FIRST |
| brainstorming | (HARD-GATE) NO IMPLEMENTATION BEFORE DESIGN APPROVAL |

All Iron Laws share the same structure: absolute prohibition + violation remedy + explicit "violating the letter = violating the spirit" clause + rationalization table.

### HARD-GATE / EXTREMELY-IMPORTANT / SUBAGENT-STOP

Tag-based directives for non-negotiable blocking, absolute rules, and subagent bootstrap exemption respectively.

## Skill Dependency Graph

```
using-superpowers (session bootstrap, root)
  |
  +-- brainstorming (HARD-GATE) --> writing-plans --> {Execution Handoff: SDD | executing-plans}
  |                                                       |
  |                                                       +--> using-git-worktrees (REQUIRED prerequisite)
  |                                                       +--> test-driven-development (per task)
  |                                                       +--> finishing-a-development-branch (terminal)
  |
  +-- systematic-debugging
  |     +-- verification-before-completion (for fix verification)
  |     +-- test-driven-development (Phase 4 test creation)
  |     +-- root-cause-tracing (technique)
  |     +-- condition-based-waiting (technique, flaky tests)
  |
  +-- receiving-code-review (on review feedback)
  +-- requesting-code-review --> code-reviewer (agent | template) [Critical/Important/Minor taxonomy]
  +-- dispatching-parallel-agents (independent investigations only)
  +-- writing-skills (meta — when authoring/editing skills)
        +-- testing-skills-with-subagents (protocol)
        +-- persuasion-principles (authoring vocabulary)
        +-- anthropic-best-practices (reference only; NOT authority — see CLAUDE.md:35-37)
```

## Ubiquitous Language Glossary

- **Your human partner** — deliberate term for the user (`CLAUDE.md:9`). NOT interchangeable with "the user".
- **Skill** — behavior-shaping reference document.
- **Iron Law** — non-negotiable rule within a skill.
- **HARD-GATE** — tagged blocking condition.
- **Red Flag** — a rationalization the agent must catch and reject.
- **Pressure testing** — adversarial scenarios against a skill to verify it holds.
- **Rationalization Table** — captured excuses + rebuttals from baseline testing.
- **Baseline** — agent behavior WITHOUT the skill.
- **RED/GREEN/REFACTOR** — TDD cycle states; also skill-authoring cycle states.
- **Fresh subagent** — context-isolated delegate.
- **Spec reviewer / code quality reviewer** — the two ordered stages of post-implementation review.
- **DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT** — implementer status codes.
- **CSO (Claude Search Optimization)** — making skills findable via description/keyword/naming.
- **Gate Function** — 5-step verification sequence.
- **Phase 4.5 Architectural Escape Valve** — 3+ failed fixes → stop, question architecture.
- **Circle K signal** — "Strange things are afoot at the Circle K", pushback-discomfort codephrase.
- **Visual Companion** — optional browser tool for visual brainstorming questions.
- **Skill Activation Mechanism** — abstract: `Skill` / `skill` / `activate_skill` across platforms.
- **Bootstrap Injection Protocol** — SessionStart hook JSON shape per platform.
- **(Round 2) Plan Document** — `docs/superpowers/plans/YYYY-MM-DD-*.md`, strict schema, REQUIRED header, placeholder-free, inline self-review, terminal execution handoff.
- **(Round 2) No Placeholders class** — closed set of forbidden plan tokens.
- **(Round 2) Execution Handoff** — decision node between writing-plans and {SDD | executing-plans}.
- **(Round 2) Directory-Selection Precedence** — worktree: existing > CLAUDE.md > ask.
- **(Round 2) Clean Baseline** — pre-work green-test invariant for new worktrees.
- **(Round 2) Code-Reviewer Agent** — `agents/code-reviewer.md`, distinct from the prompt template under `skills/requesting-code-review/`.
- **(Round 2) Review Severity Tiers** — Critical / Important / Minor (Suggestions).
- **(Round 2) Pressure Types** — Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic (3+ required for a good pressure test).
- **(Round 2) Persuasion Matrix** — Authority, Commitment, Scarcity, Social Proof, Unity (Reciprocity rare; Liking forbidden for compliance).
- **(Round 2) In-Over-Your-Head Contract** — implementer's named triggers for BLOCKED/NEEDS_CONTEXT escalation.
- **(Round 2) Deprecation Shim** — slash commands that do no work; they tell the human partner to use the named skill instead.

## State Machines

- **Brainstorming**: explore → visual-companion-offer? → clarify → approaches → design-sections → approve-per-section → write-spec → self-review → user-reviews → transition-to-writing-plans
- **Writing-plans (round 2)**: scope-check → file-structure → task-decomposition → write-header → write-tasks → inline-self-review(coverage, placeholder, type-consistency) → save → execution-handoff{SDD | inline}
- **Worktree creation (round 2)**: directory-select(existing|CLAUDE.md|ask) → gitignore-verify → create-branch → auto-detect-setup → baseline-test → report-ready|ask-on-failure
- **TDD cycle**: red → verify_red → green → verify_green → refactor → verify_green → next → red
- **SDD per-task loop**: dispatch-implementer → {ask-questions? → answer → re-dispatch} → implementer-done(status) → {spec-reviewer → {fix-spec-gaps → re-review}*} → spec-correct → {quality-reviewer → {fix-quality → re-review}*} → quality-correct → mark-complete
- **Debugging**: Phase1 → Phase2 → Phase3 → Phase4 → (fix#<3? → Phase1 : Phase4.5 architectural-stop)
- **Verification Gate**: IDENTIFY → RUN → READ → VERIFY → CLAIM (sequential)
- **Receiving code review**: READ → UNDERSTAND → VERIFY → EVALUATE → RESPOND → IMPLEMENT
- **Finishing branch**: verify-tests → determine-base → present-4-options → execute → cleanup-worktree?
- **Skill authoring RED-GREEN-REFACTOR**: baseline-pressure-test (RED) → write-minimal-skill (GREEN) → close-loopholes-iterate (REFACTOR) → meta-test → bulletproof

## Persistent Artifacts

Superpowers has no runtime data model, no state files, no database. Workflow-produced artifacts:

- `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` (brainstorming output)
- `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` (writing-plans output — first-class entity with schema, round 2)
- `.superpowers/brainstorm/<session>/{content,state}/` (visual-companion screen_dir + state_dir; persisted under project when `--project-dir` passed)
