# Pass 1: Architecture

_Phase B convergence round 2 — supersedes pass-1-architecture-deep-r1.md. Consolidates Phase A + round 1 delta + round 2 additions (skill-chain dependency graph)._

## The Injection Mechanism (Bootstrap Injection Protocol)

Superpowers has no traditional "runtime" — it is a bundle of markdown + one bash hook + one JS shim. Sequence:

1. Platform loads plugin → fires SessionStart event
2. `hooks/session-start` reads `skills/using-superpowers/SKILL.md` verbatim (`hooks/session-start:18-19`)
3. Script wraps in `<EXTREMELY_IMPORTANT>You have superpowers.\n...</EXTREMELY_IMPORTANT>` and JSON-escapes (`hooks/session-start:38`)
4. Emits JSON with platform-conditional shape (`hooks/session-start:49-67`):

```
session-start (bash)
  |
  +-- if CURSOR_PLUGIN_ROOT set
  |     -> { "additional_context": <text> }                                 [Cursor]
  +-- elif CLAUDE_PLUGIN_ROOT set
  |     -> { "hookSpecificOutput": { "additionalContext": <text> } }        [Claude Code]
  +-- else
        -> { "additionalContext": <text> }                                  [Copilot CLI / SDK default]
```

5. Platform concatenates the injected context into the session's system prompt
6. Agent is primed with the 1% rule and full skill catalog

## Skill Composition Model

Skills are model-invoked units, not code. Each is `skills/<name>/SKILL.md` + optional supporting files. Frontmatter `description` acts as trigger criterion for self-selection. Loaded via Skill tool / activate_skill depending on platform. Reading skill files via the Read tool is explicitly forbidden — all loads must go through the platform's skill mechanism so invocation is tracked.

**Size model (round 2):** The SKILL.md file is the contract; 62% of skill content lives in adjacent supporting files. Three skills are "library-class" (writing-skills 2249 LOC, brainstorming 996 LOC, systematic-debugging 959 LOC of supporting content). The others are small or leaf.

## Skill Activation Mechanism (cross-platform abstraction)

```
Skill Activation Mechanism (conceptual)
  |
  +-- Claude Code:  Skill tool (invoke by name)
  +-- Copilot CLI:  Skill tool (same pattern)
  +-- Gemini CLI:   activate_skill function
  +-- OpenCode:     plugin-loaded via .opencode/plugins/superpowers.js (112 LOC JS shim)
  +-- Codex:        reference doc at skills/using-superpowers/references/codex-tools.md
```

Abstraction is leaky: OpenCode requires the JS adapter because its plugin API is not markdown-native. This makes superpowers a **2-language package** (markdown+bash core + JS shim for a single platform). `package.json:5` declares `main: .opencode/plugins/superpowers.js`, confirming this is the only "code" entry point.

## Skill-Chain Dependency Graph (round 2, load-bearing)

Extracted from SKILL.md files by grepping `superpowers:<name>`, `REQUIRED SUB-SKILL`, and `REQUIRED BACKGROUND`. **21 directed edges, 3 edge types.**

### Edge types

- **REQ_SUB** (hard requirement, runtime): "REQUIRED SUB-SKILL: Use superpowers:X" — caller MUST invoke X as part of its own flow.
- **REQ_BG** (hard requirement, prerequisite): "REQUIRED BACKGROUND: You MUST understand superpowers:X" — reader must have loaded X before using this skill.
- **REF** (soft reference in Related Skills / prose): mentions X as relevant but not mandatory.

### Adjacency list

```
using-superpowers
  (no outbound — bootstrap root)

brainstorming
  -> writing-plans         (REF, via pipeline prose / README)

writing-plans
  -> subagent-driven-development  (REQ_SUB, recommended, writing-plans/SKILL.md:52,147)
  -> executing-plans              (REQ_SUB, alternative, writing-plans/SKILL.md:52,151)

executing-plans
  -> finishing-a-development-branch (REQ_SUB, executing-plans/SKILL.md:36)
  -> using-git-worktrees            (REF, REQUIRED in Related Skills, executing-plans/SKILL.md:68)
  -> writing-plans                  (REF, executing-plans/SKILL.md:69)
  -> subagent-driven-development    (REF, "use this instead if subagents available", executing-plans/SKILL.md:14)

subagent-driven-development
  -> finishing-a-development-branch (REQ_SUB, SDD/SKILL.md:64,83,271)
  -> using-git-worktrees            (REF, REQUIRED in Related Skills, SDD/SKILL.md:268)
  -> writing-plans                  (REF, SDD/SKILL.md:269)
  -> requesting-code-review         (REF, SDD/SKILL.md:270)
  -> test-driven-development        (REF, SDD/SKILL.md:274)
  -> executing-plans                (REF, alternate path, SDD/SKILL.md:277)

test-driven-development
  (no outbound)

systematic-debugging
  -> test-driven-development        (REF, systematic-debugging/SKILL.md:179,287)
  -> verification-before-completion (REF, systematic-debugging/SKILL.md:288)

writing-skills
  -> test-driven-development        (REQ_BG, writing-skills/SKILL.md:18,393)
  -> systematic-debugging           (REQ_BG, writing-skills/SKILL.md:284 — as example in pedagogy)

requesting-code-review
  -> code-reviewer (Agent, not a skill; requesting-code-review/SKILL.md:8,34,59)

receiving-code-review
  (no outbound superpowers:X refs found)

verification-before-completion
  (no outbound)

using-git-worktrees
  (no outbound)

finishing-a-development-branch
  (no outbound — terminal sink)

dispatching-parallel-agents
  (no outbound)
```

### Graph summary

- **Nodes**: 14 skills + 1 agent (code-reviewer) = 15
- **Edges**: 21 (6 REQ_SUB, 2 REQ_BG, 13 REF)
- **Hub nodes (high out-degree)**: subagent-driven-development (6 outbound), executing-plans (4 outbound)
- **finishing-a-development-branch is the universal terminal sink** — both executing-plans and SDD hard-require it. Every implementation path converges here.
- **test-driven-development is the universal prerequisite sink** — writing-skills requires it as background, SDD and systematic-debugging reference it.

### Architectural implications

1. **Two hard pipelines terminate at finishing-a-development-branch** (REQ_SUB from both SDD and executing-plans). This is the only skill mandated by two independent parents. **It is the architectural commit-point.**
2. **writing-plans is a dispatcher**, not a leaf — it hard-requires the caller to continue into SDD or executing-plans. Planning without execution handoff is a protocol violation.
3. **systematic-debugging is a parallel sub-pipeline**, not part of the main brainstorm→plan→execute chain. It has its own entry (triggered by bug symptoms) and only soft-references TDD / verification. Architecturally orthogonal.
4. **writing-skills sits outside the runtime pipeline** — it is a meta-skill for authoring new skills. Its REQ_BG edges (to TDD and systematic-debugging) are pedagogical, not operational.
5. **Enforcement is prose.** Every edge in this graph is a markdown directive. No code checks that SDD actually dispatches code-reviewer or that executing-plans actually invokes finishing-a-development-branch. The 1% rule, Red Flags tables, and Iron Laws are the *only* compliance mechanism. **This is the architectural weak point**: graph integrity depends on the model obeying instructions it reads.

## The Brainstorm → Plan → Execute Pipeline (corrected, round 1)

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
requesting-code-review -> dispatches agents/code-reviewer.md (fresh context)
  v
receiving-code-review  (verify before implementing feedback)
  v
verification-before-completion  (gate before completion claim)
  v
finishing-a-development-branch  (merge/PR/keep/discard; clean worktree)
```

## Subagent Architecture

```
subagent-driven-development (host session)
  |
  +-- construct fresh-context prompt from templates:
  |     +-- skills/subagent-driven-development/implementer-prompt.md       (113 LOC)
  |     +-- skills/subagent-driven-development/spec-reviewer-prompt.md     (61 LOC)
  |     +-- skills/subagent-driven-development/code-quality-reviewer-prompt.md (26 LOC)
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

Invariant: every Task() call uses fresh context constructed from prompt template + minimal task data. No session-history inheritance. Per-task (not only per-review) fresh-context — the differentiating architectural choice vs single-session execution.

## Multi-Platform Manifest Pattern

```
Runtime entry contract per platform:
  Claude Code  -> CLAUDE.md  (contributor guidelines; runtime uses hook injection)
  Codex/Agents -> AGENTS.md  (symlink -> CLAUDE.md)
  Gemini       -> GEMINI.md  (2 lines; imports using-superpowers SKILL.md via `@./` syntax)
  OpenCode     -> .opencode/plugins/superpowers.js (JS plugin API shim)
```

Symlink + `@./` import = deliberate manifest deduplication: one source of truth, N platform entry points.

## Empirical Test Architecture

```
Test architecture (no CI — all local invocation)
  |
  +-- Unit-style (skill behavior on fixed prompts)
  |     tests/skill-triggering/run-all.sh
  |     tests/explicit-skill-requests/run-all.sh  (incl. haiku + multiturn variants)
  |
  +-- Integration (platform adapters)
  |     tests/claude-code/run-skill-tests.sh
  |     tests/opencode/run-tests.sh + setup.sh + 3 test-*.sh
  |     tests/brainstorm-server/  (5 files, JS + bash)
  |
  +-- Empirical / end-to-end (fixture triple pattern)
        tests/subagent-driven-dev/run-test.sh
          +-- go-fractals/ {design.md, plan.md, scaffold.sh}
          +-- svelte-todo/ {design.md, plan.md, scaffold.sh}
```

Fixture triple (design + plan + scaffold) is a first-class architectural contract for SDD validation. **No `.github/workflows/` file exists** — this suite is invoked by hand or by downstream dogfooding, not by automated CI. Architectural implication: validation is aspirational; no gate prevents broken edges from landing.

## Deployment Topology

Zero-dependency, zero-runtime markdown+bash package with a single 112-LOC JS shim for OpenCode. Runs wherever the host CLI supports plugin hooks. No services, no state, no build step, no CI.

## Component Inventory

| Component | Kind | LOC | Role |
|---|---|---|---|
| 14 SKILL.md | markdown (model-invoked) | 3159 | Behavioral contracts (entry points) |
| 32 skill supporting files | mixed | 5279 | Knowledge payload (62% of skill LOC) |
| 1 agent (code-reviewer) | markdown (subagent) | 48 | Dispatched code review |
| 3 commands | markdown shims | 15 | Deprecated slash entries |
| 4 hooks | bash + JSON | 129 | SessionStart injection |
| OpenCode adapter | JS | 112 | Platform shim (package.json main) |
| 6 subagent prompt templates | markdown | (in skills) | Fresh-context payloads |
| SDD fixtures (2 triples) | mixed | 636 | Empirical test scenarios |
| Platform manifests | JSON+symlink+md | small | Per-platform entry |
| CI workflows | — | 0 | **None** |

## Convergence Declaration

Pass 1 has converged. Further rounds would be edge-labeling nitpicks. The directed graph, pipeline control flow, subagent topology, multi-platform manifest pattern, empirical test architecture, and deployment topology are all captured.
