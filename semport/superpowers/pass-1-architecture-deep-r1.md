# Pass 1 Deepening Round 1: Architecture Expansion

Builds on `pass-1-architecture.md` (Phase A). Phase A content preserved verbatim; this round adds concepts passes 2+3 convergence surfaced.

## Update 1: Platform-Conditional JSON Shape Switching (Injection Mechanism)

Phase A step 4 enumerated the 3 JSON shapes. Formalized as decision tree in `hooks/session-start`:

```
session-start (bash)
  |
  +-- read skills/using-superpowers/SKILL.md verbatim
  +-- JSON-escape into <EXTREMELY_IMPORTANT>...</EXTREMELY_IMPORTANT>
  |
  +-- if CURSOR_PLUGIN_ROOT set
  |     -> emit { "additional_context": <text> }           [Cursor shape]
  +-- elif CLAUDE_PLUGIN_ROOT set
  |     -> emit { "hookSpecificOutput": { "additionalContext": <text> } }  [Claude Code shape]
  +-- else
        -> emit { "additionalContext": <text> }            [Copilot CLI / SDK-standard shape]
```

This is the **Bootstrap Injection Protocol with 3 JSON shapes** pass 2 round 3 named. Single hook, three output contracts, env-var detected.

## Update 2: Skill Activation Mechanism (new architectural abstraction)

Pass 2 round 2 formalized this as an abstraction layer across platforms:

```
Skill Activation Mechanism (conceptual)
  |
  +-- Claude Code: Skill tool (invoke by skill name)
  +-- Copilot CLI: Skill tool (same pattern)
  +-- Gemini CLI:  activate_skill function
  +-- OpenCode:    plugin-loaded via .opencode/plugins/superpowers.js
  +-- Codex:       reference doc in skills/using-superpowers/references/codex-tools.md
```

The architecture assumes every target platform provides *some* mechanism to load a named markdown skill into context on demand. `skills/using-superpowers/references/{codex,copilot,gemini}-tools.md` (3 files) are the per-platform binding documents. The abstraction is leaky: OpenCode requires a JS adapter (112 LOC) because its plugin API is not markdown-native.

## Update 3: OpenCode JS Adapter Node

`.opencode/plugins/superpowers.js` (112 LOC) is a separate architectural node, not part of the markdown+bash core:

```
OpenCode runtime
  |
  +-- loads .opencode/plugins/superpowers.js
        +-- first-user-message hook -> injects using-superpowers SKILL.md content
        +-- registers hook path for SessionStart equivalent
        +-- bridges OpenCode plugin events -> superpowers' markdown skill pattern
```

This makes superpowers a **2-language package** (markdown+bash for most platforms, + JS shim for OpenCode), though the JS is only invoked on that single platform.

## Update 4: Updated Brainstorm → Plan → Execute Pipeline

Pass 2 rounds 2+3 expanded the pipeline with the Execution Handoff decision node and SDD terminal final-reviewer. Redrawn:

```
brainstorming (HARD-GATE: no impl until design approved)
  | output: docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md
  | inline Self-Review via spec-document-reviewer-prompt.md
  v
using-git-worktrees  (creates isolated worktree, verifies clean baseline)
  v
writing-plans
  | output: docs/superpowers/plans/YYYY-MM-DD-<feature>.md
  | INLINE Self-Review (NOT a subagent dispatch) via plan-document-reviewer-prompt.md
  v
[Execution Handoff Decision Node]
  |
  +-- branch A: subagent-driven-development (fresh context per task)
  |     | per-task loop:
  |     |   implementer subagent -> spec-reviewer subagent -> code-quality-reviewer subagent
  |     |   decision: APPROVED | DONE_WITH_CONCERNS | BLOCKED
  |     |     BLOCKED -> remediation ladder (retry / reframe / escalate)
  |     | TERMINAL final-reviewer checkpoint (after all tasks)
  |     v
  +-- branch B: executing-plans (single session, no subagents)
        v
  [merge]
  v
test-driven-development  (injected throughout both branches; RED-GREEN-REFACTOR)
  v
requesting-code-review -> dispatches agents/code-reviewer.md (the Agent, fresh context)
  v
receiving-code-review  (verify before implementing feedback)
  v
verification-before-completion  (gate before completion claim)
  v
finishing-a-development-branch  (merge/PR/keep/discard; clean worktree)
```

Key corrections vs Phase A:

- writing-plans Self-Review is **inline**, not a subagent dispatch.
- **Execution Handoff is a decision node**, not an implicit sequential step.
- SDD has a **terminal final-reviewer** after all per-task reviews complete.
- SDD per-task review outcomes are **tri-valued**: APPROVED / DONE_WITH_CONCERNS / BLOCKED (not binary).
- BLOCKED triggers a **remediation ladder** (retry → reframe → escalate), not immediate failure.

## Update 5: Subagent Architecture (formalized)

```
subagent-driven-development (host session)
  |
  +-- construct fresh-context prompt from templates:
  |     +-- skills/subagent-driven-development/implementer-prompt.md
  |     +-- skills/subagent-driven-development/spec-reviewer-prompt.md
  |     +-- skills/subagent-driven-development/code-quality-reviewer-prompt.md
  |
  +-- per task (bite-sized, 2-5 min):
  |     Task(implementer_prompt)      -> implementation + tests
  |     Task(spec_reviewer_prompt)    -> checks against plan/spec
  |     Task(code_quality_prompt)     -> checks against coding standards
  |     combine verdicts -> {APPROVED, DONE_WITH_CONCERNS, BLOCKED}
  |     BLOCKED -> remediation ladder
  |
  +-- after all tasks: Task(final-reviewer)  -- terminal checkpoint
```

Invariant: every subagent Task() call uses **fresh context** constructed from prompt template + minimal task data. No session history inheritance. Mirrors the Corverax adversarial-review fresh-context principle but applied per-task.

## Update 6: Multi-Platform Manifest Pattern

```
Runtime entry contract per platform:
  Claude Code  -> CLAUDE.md   (but: CLAUDE.md is contributor guidelines; runtime uses hook injection)
  Codex/Agents -> AGENTS.md   (symlink -> CLAUDE.md)
  Gemini       -> GEMINI.md   (2 lines; imports using-superpowers SKILL.md via `@./` syntax)
```

`AGENTS.md → CLAUDE.md` symlink and `GEMINI.md`'s `@./` import are a **deliberate manifest deduplication pattern**: one source of truth, N platform entry points.

## Update 7: Empirical Test Architecture

```
Test architecture
  |
  +-- Unit-style (skill behavior on fixed prompts)
  |     tests/skill-triggering/run-all.sh
  |     tests/explicit-skill-requests/run-all.sh (incl. haiku + multiturn variants)
  |
  +-- Integration (platform adapters)
  |     tests/claude-code/run-skill-tests.sh
  |     tests/opencode/run-tests.sh + setup.sh + 3 test-*.sh
  |     tests/brainstorm-server/ (5 files, JS+bash)
  |
  +-- Empirical/end-to-end (fixture triple pattern)
        tests/subagent-driven-dev/run-test.sh
          +-- go-fractals/    {design.md, plan.md, scaffold.sh}
          +-- svelte-todo/    {design.md, plan.md, scaffold.sh}
```

The **fixture triple format** (design.md + plan.md + scaffold.sh) is a first-class architectural contract for SDD validation: design = problem spec, plan = writing-plans output, scaffold = repeatable bootstrap. Runner script drives subagents against the triples and asserts integration outcomes. **Empirical testing architecture** — measuring model behavior on realistic scenarios, not unit-mocking. Closely parallels Corverax's holdout evaluation pattern.

## Update 8: Component Inventory (summary)

| Component | Kind | LOC | Role |
|---|---|---|---|
| 14 skills | markdown (model-invoked) | 8438 | Behavioral directives |
| 1 agent (code-reviewer) | markdown (subagent) | 48 | Dispatched code review |
| 3 commands | markdown shims | 15 | Deprecated slash entries |
| 4 hooks | bash + JSON | 129 | SessionStart injection |
| OpenCode adapter | JS | 112 | Platform shim |
| 6 subagent prompt templates | markdown | (incl in skills) | Fresh-context payloads |
| SDD fixtures | mixed | 636 | Empirical test scenarios |
| Platform manifests | JSON+symlink+md | small | Per-platform entry |

## Delta Summary

- New concepts: Skill Activation Mechanism abstraction, OpenCode JS adapter node, Multi-Platform Manifest Pattern, Empirical Test Architecture with fixture triple format, Execution Handoff decision node, SDD tri-valued outcomes + BLOCKED remediation ladder, SDD terminal final-reviewer checkpoint, Bootstrap Injection Protocol decision tree.
- Refined: Pipeline redrawn with corrected control flow (inline plan review, explicit decision node, terminal checkpoint).
- Remaining gaps: Quantitative analysis of skill-chain dependency graph (which skills mandate which other skills) — round 2 candidate.
