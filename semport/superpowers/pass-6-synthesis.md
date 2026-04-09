# Pass 6: Final Synthesis — superpowers

_Phase C — Final Synthesis. Supersedes the Phase A `pass-6-synthesis.md`. Consolidates the converged Phase B outputs of Passes 0-5 into the authoritative unified knowledge document for obra/superpowers @ 917e5f53b16b115b70a3a355ed5f4993b9f8b73d._

---

## 1. Executive Summary

Superpowers is a **behavior-shaping plugin, not a pipeline**. Its entire value is injecting the right markdown into the right agent context at the right time and making it rhetorically impossible for the agent to rationalize its way out of the discipline. The "runtime" is one bash hook (`hooks/session-start`) that reads one markdown file (`skills/using-superpowers/SKILL.md`), wraps it in `<EXTREMELY_IMPORTANT>`, JSON-escapes it, and emits a platform-conditional JSON shape (`hooks/session-start:40-67`). Everything else — 14 skills, 1 agent, ~7k LOC of markdown + bash + one 112-LOC JS shim for OpenCode — is content the agent reads on demand via a platform-provided Skill Activation Mechanism. There is no orchestrator. There is no scheduler. There is no phase gate. **The agent itself is the orchestrator**, and the skills are priority-ordered behavioral constraints it applies to itself.

The central thesis has four load-bearing claims: **(a)** skills are code that shapes agent behavior, not prose (`CLAUDE.md:69`); **(b)** Iron Laws + Red Flags tables are the sole enforcement mechanism, substituting rhetorical saturation for type-system or runtime checks; **(c)** skill content is **empirically tuned** via adversarial pressure testing against fresh subagents, not author-designed — carefully-tuned content is protected by a governance NFR requiring before/after eval evidence for any edit (`CLAUDE.md:67-74`); **(d)** the agent is a **parahuman** whose compliance can be moved by the same Cialdini persuasion principles that move humans, empirically validated in Meincke et al. 2025 (N=28,000 AI conversations, compliance rate 33% → 72%, p < .001, cited at `skills/writing-skills/persuasion-principles.md:7`).

**What changed in Phase B convergence.** Three round-1 hallucinations were caught and corrected by the strict-binary novelty protocol: **(1)** the Pressure Taxonomy was partly fabricated (invented "urgency/flattery/confusion"; missed Economic/Exhaustion/Pragmatic entirely); **(2)** the Persuasion Matrix was documented as 6 principles when it is actually 7 (Reciprocity was missing); **(3)** the Plan No-Placeholders forbidden-token list was over-extrapolated — actual verbatim list is only `"TBD", "TODO", "implement later", "fill in details"`, and the broader rule is an invariant ("every step must contain the actual content an engineer needs") not a token blacklist. Passes 2 and 3 converged at round 3 with zero substantive findings; Passes 0, 1, 4, 5 converged at round 2. No pass needed the 5-round maximum.

---

## 2. System Architecture

### 2.1 Deployment Topology

Zero-dependency, zero-runtime markdown+bash package with a single 112-LOC JS shim for OpenCode (`.opencode/plugins/superpowers.js`). Runs wherever the host CLI supports plugin hooks. No services, no state, no build step, **no CI workflows** — zero `.github/workflows/` files. All testing is invoked locally via `tests/*/run-*.sh`. Validation is aspirational; no automated gate prevents broken edges from landing.

### 2.2 Bootstrap Injection Protocol

Superpowers has no traditional runtime. Session priming happens through one hook:

```
Platform loads plugin
  |
  v
SessionStart event (matcher: startup|clear|compact)   hooks/hooks.json:4-13
  |
  v
hooks/session-start (bash)                            hooks/session-start:18-19
  reads skills/using-superpowers/SKILL.md verbatim
  wraps in <EXTREMELY_IMPORTANT>You have superpowers. ...</EXTREMELY_IMPORTANT>
  JSON-escapes                                         hooks/session-start:38
  |
  +-- if CURSOR_PLUGIN_ROOT set
  |     -> { "additional_context": <text> }            [Cursor, snake_case]
  +-- elif CLAUDE_PLUGIN_ROOT set && COPILOT_CLI unset
  |     -> { "hookSpecificOutput":
  |            { "hookEventName": "SessionStart",
  |              "additionalContext": <text> } }       [Claude Code, camelCase]
  +-- else
        -> { "additionalContext": <text> }             [Copilot CLI / SDK default]
  |
  v
Platform concatenates injected context into session system prompt
  |
  v
Agent is primed with the 1% rule and full skill catalog
```

OpenCode diverges further: it injects via `experimental.chat.messages.transform` into the **first user message** (not the system prompt) to avoid token bloat (`.opencode/plugins/superpowers.js:84-110`, issues #750, #894). Idempotency guard: skip if `EXTREMELY_IMPORTANT` already present. Claude Code reads BOTH `additional_context` and `hookSpecificOutput` without dedup — the hook MUST emit exactly one of the three shapes (contract BC-DRAFT-031).

**Legacy skills-directory warning:** if `~/.config/superpowers/skills` exists at session start, the hook injects an `<important-reminder>` the agent must surface in its first reply, telling the user to migrate to `~/.claude/skills` (`hooks/session-start:12-15`, BC-DRAFT-032).

### 2.3 Skill Activation Mechanism (cross-platform abstraction)

Skills are model-invoked units, not code. The YAML `description` field is the trigger criterion for agent self-selection. **Reading skill files via the Read tool is explicitly forbidden** — all loads go through the platform's skill mechanism so invocation is tracked.

| Platform | Mechanism | Notes |
|---|---|---|
| Claude Code | `Skill` tool | Never use Read tool on skill files |
| Copilot CLI | `skill` tool | Auto-discovered from plugins |
| Gemini CLI | `activate_skill` tool | Metadata at session start, full content on demand |
| OpenCode | native `skill` tool | Skills path injected via config hook |
| Codex | reference doc at skills/using-superpowers/references/codex-tools.md | — |

Abstraction is **leaky**: OpenCode requires the JS adapter because its plugin API is not markdown-native. Makes superpowers a **2-language package** — markdown+bash core plus JS shim. `package.json:5` declares `main: .opencode/plugins/superpowers.js`.

### 2.4 Multi-Platform Manifest Pattern

```
Runtime entry contract per platform:
  Claude Code  -> CLAUDE.md  (contributor guidelines; runtime uses hook injection)
  Codex/Agents -> AGENTS.md  (symlink -> CLAUDE.md)
  Gemini       -> GEMINI.md  (2 lines; imports using-superpowers SKILL.md via `@./` syntax)
  OpenCode     -> .opencode/plugins/superpowers.js  (JS plugin API shim)
  Cursor       -> .cursor-plugin/plugin.json + hooks/hooks-cursor.json
  Copilot CLI  -> skills/using-superpowers/references/copilot-tools.md
```

Symlink + `@./` import = deliberate **manifest deduplication**: one source of truth, N platform entry points. **CLAUDE.md is contributor guidelines, not runtime instructions.**

### 2.5 Skill Composition Model and Size Distribution

14 `SKILL.md` files (3159 LOC) + 23 supporting files (3859 LOC). **55% of skill content lives in adjacent supporting files**; SKILL.md is entry-point contract, real knowledge payload is in sibling markdown. Three "library-class" skills:

| Skill | Supporting LOC | % of supporting total |
|---|---|---|
| writing-skills | 1910 | 50% |
| systematic-debugging | 734 | 19% |
| brainstorming | 336 | 9% |

These three account for 2980 of the 3859 supporting LOC (77%). Six skills have zero supporting files. `writing-skills/anthropic-best-practices.md` is 1150 lines — single largest content file in the repo, larger than any SKILL.md.

_(Round 2 of Phase B pass-0 inventory over-counted supporting files at 32 / 5279 LOC; validate-extraction corrected this to 23 / 3859 LOC. Behavioral and architectural claims elsewhere in the synthesis are unaffected.)_

### 2.6 Skill-Chain Dependency Graph (21 edges, load-bearing)

Three edge types:

- **REQ_SUB** — hard runtime requirement: "REQUIRED SUB-SKILL: Use superpowers:X"
- **REQ_BG** — hard prerequisite: "REQUIRED BACKGROUND: You MUST understand superpowers:X"
- **REF** — soft reference in Related Skills / prose

```
using-superpowers
  (no outbound — bootstrap root)

brainstorming
  -> writing-plans                                      REF

writing-plans
  -> subagent-driven-development                        REQ_SUB  writing-plans/SKILL.md:52,147
  -> executing-plans                                    REQ_SUB  writing-plans/SKILL.md:52,151

executing-plans
  -> finishing-a-development-branch                     REQ_SUB  executing-plans/SKILL.md:36
  -> using-git-worktrees                                REF      executing-plans/SKILL.md:68
  -> writing-plans                                      REF      executing-plans/SKILL.md:69
  -> subagent-driven-development                        REF      executing-plans/SKILL.md:14

subagent-driven-development
  -> finishing-a-development-branch                     REQ_SUB  SDD/SKILL.md:64,83,271
  -> using-git-worktrees                                REF      SDD/SKILL.md:268
  -> writing-plans                                      REF      SDD/SKILL.md:269
  -> requesting-code-review                             REF      SDD/SKILL.md:270
  -> test-driven-development                            REF      SDD/SKILL.md:274
  -> executing-plans                                    REF      SDD/SKILL.md:277

test-driven-development
  (no outbound)

systematic-debugging
  -> test-driven-development                            REF      systematic-debugging/SKILL.md:179,287
  -> verification-before-completion                     REF      systematic-debugging/SKILL.md:288

writing-skills
  -> test-driven-development                            REQ_BG   writing-skills/SKILL.md:18,393
  -> systematic-debugging                               REQ_BG   writing-skills/SKILL.md:284

requesting-code-review
  -> code-reviewer (agent, not a skill)                          requesting-code-review/SKILL.md:8,34,59

receiving-code-review                                   (no outbound)
verification-before-completion                          (no outbound)
using-git-worktrees                                     (no outbound)
finishing-a-development-branch                          (no outbound — universal terminal sink)
dispatching-parallel-agents                             (no outbound)
```

**Graph summary:** 14 skills + 1 agent = 15 nodes; 21 edges (6 REQ_SUB, 2 REQ_BG, 13 REF).

**Architectural implications:**

1. **Hub nodes**: subagent-driven-development (6 outbound), executing-plans (4 outbound).
2. **finishing-a-development-branch is the universal terminal sink** — both executing-plans and SDD hard-require it. The only skill mandated by two independent parents. **The architectural commit-point.**
3. **test-driven-development is the universal prerequisite sink** — writing-skills REQ_BG; SDD + systematic-debugging reference it.
4. **writing-plans is a dispatcher, not a leaf** — hard-requires caller to continue into SDD or executing-plans. Planning without execution handoff is a protocol violation.
5. **systematic-debugging is a parallel sub-pipeline** — orthogonal to brainstorm→plan→execute.
6. **writing-skills sits outside the runtime pipeline** — meta-skill for authoring new skills; REQ_BG edges are pedagogical.
7. **Enforcement is prose.** Every edge is a markdown directive. **Architectural weak point**: graph integrity depends on the model obeying instructions it reads.

### 2.7 The Brainstorm → Plan → Execute Pipeline

```
brainstorming (HARD-GATE: no implementation until design approved)
  | output: docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md
  | inline Self-Review via spec-document-reviewer-prompt.md
  v
using-git-worktrees  (creates isolated worktree, verifies clean baseline)
  v
writing-plans
  | output: docs/superpowers/plans/YYYY-MM-DD-<feature>.md
  | INLINE Self-Review (NOT a subagent dispatch)
  v
[Execution Handoff Decision Node — exactly 2 options presented]
  |
  +-- branch A: subagent-driven-development (fresh context per task)
  |     | per-task loop:
  |     |   implementer subagent -> spec-reviewer subagent -> code-quality-reviewer subagent
  |     |   decision: APPROVED | DONE_WITH_CONCERNS | BLOCKED | NEEDS_CONTEXT
  |     |     BLOCKED -> ordered remediation ladder (context / model / split / escalate)
  |     | TERMINAL final-code-reviewer checkpoint (after all tasks complete)
  |     v
  +-- branch B: executing-plans (single session, no subagents)
        v
  [merge]
  v
test-driven-development  (RED-GREEN-REFACTOR)
  v
requesting-code-review -> dispatches agents/code-reviewer.md (fresh context)
  v
receiving-code-review  (verify before implementing feedback)
  v
verification-before-completion  (gate before completion claim)
  v
finishing-a-development-branch  (merge/PR/keep/discard; clean worktree)
```

### 2.8 Subagent Architecture (SDD detail)

```
subagent-driven-development (host session)
  |
  +-- construct fresh-context prompt from templates:
  |     +-- implementer-prompt.md          (113 LOC)
  |     +-- spec-reviewer-prompt.md        (61 LOC)
  |     +-- code-quality-reviewer-prompt.md (26 LOC)
  |
  +-- per task (bite-sized, 2-5 min):
  |     Task(implementer_prompt)      -> implementation + tests
  |     Task(spec_reviewer_prompt)    -> checks against plan/spec
  |     Task(code_quality_prompt)     -> checks against coding standards
  |     combine verdicts -> {APPROVED, DONE_WITH_CONCERNS, BLOCKED, NEEDS_CONTEXT}
  |     BLOCKED -> ordered remediation ladder
  |
  +-- after all tasks: Task(final-reviewer)  -- terminal third checkpoint
```

**Invariant:** every `Task()` call uses fresh context. **Per-task (not only per-review) fresh context** is the differentiating architectural choice vs single-session execution. Round-3 amendment to BC-DRAFT-007: **terminal final-code-reviewer** is dispatched over the entire implementation before `finishing-a-development-branch`.

### 2.9 Empirical Test Architecture

```
Test architecture (no CI — all local invocation)
  |
  +-- Unit-style (skill behavior on fixed prompts)
  |     tests/skill-triggering/run-all.sh
  |     tests/explicit-skill-requests/run-all.sh
  |
  +-- Integration (platform adapters)
  |     tests/claude-code/run-skill-tests.sh
  |     tests/opencode/run-tests.sh
  |     tests/brainstorm-server/  (5 files, JS + bash)
  |
  +-- Empirical / end-to-end (fixture triple pattern)
        tests/subagent-driven-dev/run-test.sh
          +-- go-fractals/  {design.md, plan.md, scaffold.sh}
          +-- svelte-todo/  {design.md, plan.md, scaffold.sh}
```

**Fixture triple (design + plan + scaffold)** is a first-class architectural contract for SDD validation. Runner scaffolds via `scaffold.sh <outdir>/project`, dispatches the literal prompt `"Execute this plan using superpowers:subagent-driven-development. The plan is at: <plan-path>"` via `claude -p --output-format stream-json`. Pass assertion is `grep -qE` over stream-json for `"name":"Skill"` AND `"skill":"(namespace:)?<skill-name>"` — tool-invocation match, not output match (BC-DRAFT-045).

### 2.10 Component Inventory

| Component | Kind | LOC | Role |
|---|---|---|---|
| 14 SKILL.md | markdown (model-invoked) | 3159 | Behavioral contracts (entry points) |
| 32 skill supporting files | mixed | 5279 | Knowledge payload (62% of skill LOC) |
| 1 agent (code-reviewer) | markdown subagent | 48 | Dispatched code review |
| 3 commands | markdown deprecation shims | 15 | Legacy slash entries |
| 4 hooks | bash + JSON | 129 | SessionStart injection |
| OpenCode adapter | JS | 112 | Platform shim |
| 6 subagent prompt templates | markdown | (in skills) | Fresh-context payloads |
| SDD fixtures (2 triples) | mixed | 636 | Empirical test scenarios |
| Platform manifests | JSON + symlink + md | small | Per-platform entry |
| CI workflows | — | 0 | **None** |

---

## 3. Domain Model (Concepts)

### 3.1 Ubiquitous Language Glossary

- **Skill** — self-contained markdown file. Categorized along two axes: **Rigidity** (Rigid=Iron Law / Flexible=Core principle) and **Function** (Discipline / Technique / Pattern / Reference).
- **Iron Law** — non-negotiable rule in a Rigid skill. Canonical form: `NO <verb> <scope> WITHOUT <prerequisite> FIRST`. Five instances.
- **HARD-GATE** — tagged blocking condition (e.g., brainstorming's "no implementation until design approved").
- **Red Flag** — a rationalization the agent might use to skip a skill, paired with a rebuttal. Protected by the governance NFR.
- **Pressure testing** — adversarial scenarios against a skill. Requires **3+ combined Pressure Types**. The skill-authoring RED phase.
- **Rationalization Table** — verbatim excuses from baseline testing + rebuttals.
- **Baseline** — agent behavior WITHOUT the skill, captured during the RED phase.
- **RED / GREEN / REFACTOR** — TDD cycle + skill-authoring cycle. RED=baseline, GREEN=write minimal skill, REFACTOR=close loopholes.
- **Fresh subagent** — context-isolated Claude invocation. Never inherits parent context.
- **Spec reviewer / code quality reviewer** — the two ordered stages of post-implementation review in SDD.
- **DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT** — the four implementer status codes.
- **CSO (Claude Search Optimization)** — description field is WHEN-not-WHAT.
- **Gate Function** — the 5-step verification sequence: IDENTIFY → RUN → READ → VERIFY → CLAIM.
- **Phase 4.5 Architectural Escape Valve** — after 3 failed fixes, STOP and question the architecture.
- **Circle K signal** — codephrase the human partner uses to signal the agent is sliding into performative behavior.
- **Visual Companion** — optional browser-based tool for visual brainstorming questions.
- **Skill Activation Mechanism** — abstract concept: `Skill` tool (Claude Code, Copilot CLI), `activate_skill` (Gemini), native `skill` tool (OpenCode), reference doc (Codex).
- **Bootstrap Injection Protocol** — SessionStart hook + platform-conditional JSON shape.
- **Plan Document** — first-class entity at `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` with strict schema.
- **No Placeholders class** — closed set of forbidden plan content; violations = **"plan failures"** (verbatim taxonomic term at `writing-plans/SKILL.md:108`).
- **Execution Handoff** — decision node between writing-plans and {SDD | executing-plans}.
- **Directory-Selection Precedence** — worktree directory selection ladder.
- **Clean Baseline** — pre-work green-test invariant for new worktrees.
- **Code-Reviewer Agent** — `agents/code-reviewer.md`, the SOLE agent-file in the repo. `model: inherit`, six-part mandate. Distinct from subagent prompt templates.
- **Review Severity Tiers** — Critical / Important / Minor (Suggestions).
- **Pressure Types** — seven empirically-tested adversarial pressures.
- **Persuasion Matrix** — seven Cialdini principles mapped to skill-type permissibility.
- **In-Over-Your-Head Contract** — implementer's named STOP triggers.
- **Deprecation Shim** — slash command that does no work; tells human partner the command is deprecated.

### 3.2 State Machines

- **Brainstorming:** explore → visual-companion-offer? → clarify → approaches → design-sections → approve-per-section → write-spec → self-review → user-reviews → transition-to-writing-plans
- **Writing-plans:** scope-check → file-structure → task-decomposition → write-header → write-tasks → inline-self-review → save → execution-handoff{SDD | inline}
- **Worktree creation:** directory-select → gitignore-verify → create-branch → auto-detect-setup → baseline-test → report-ready | ask-on-failure
- **TDD cycle:** red → verify_red → green → verify_green → refactor → verify_green → next → red
- **SDD per-task loop:** dispatch-implementer → implementer-done(status) → {spec-reviewer loop} → spec-correct → {quality-reviewer loop} → quality-correct → mark-complete → (after all tasks) → final-code-reviewer → finishing-a-development-branch
- **Debugging:** Phase1(root cause) → Phase2(pattern analysis) → Phase3(single hypothesis) → Phase4(failing test + fix + verify) → (fix#<3? → Phase1 : Phase4.5 architectural-stop)
- **Verification Gate:** IDENTIFY → RUN → READ → VERIFY → CLAIM
- **Receiving code review:** READ → UNDERSTAND → VERIFY → EVALUATE → RESPOND → IMPLEMENT
- **Finishing branch:** verify-tests → determine-base → present-4-options → execute → cleanup-worktree?
- **Skill authoring RED-GREEN-REFACTOR:** baseline-pressure-test → write-minimal-skill → close-loopholes-iterate → meta-test → bulletproof

### 3.3 Persistent Artifacts

Superpowers has no runtime data model. Workflow-produced artifacts:

- `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` (brainstorming output)
- `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` (writing-plans output — first-class schema)
- `.superpowers/brainstorm/<session>/{content,state}/` (visual-companion)

---

## 4. Behavioral Contracts (Catalog)

**47 BC-DRAFTs** extracted across the converged passes.

### 4.1 Contracts from Skills (9 — HIGH confidence)

**BC-DRAFT-001: Bootstrap skill is injected every session.** `hooks/hooks.json:4-13`; `hooks/session-start:40-67`.

**BC-DRAFT-002: Agent invokes Skill tool before any response when skill could apply** (1% rule). `using-superpowers/SKILL.md:10-16`. HIGH (normative) / MEDIUM (compliance).

**BC-DRAFT-003: No implementation before design approval (brainstorming HARD-GATE).** `brainstorming/SKILL.md:12-14`.

**BC-DRAFT-004: No production code without a failing test (TDD Iron Law).** `test-driven-development/SKILL.md:33-45, 114-128, 168-184`. **Sub-contracts:**
- **Verify-RED gate**: observe failure for the correct reason before proceeding.
- **Verify-GREEN gate**: target test passes + other tests still pass + output pristine.
- **Exception protocol**: only throwaway prototypes, generated code, config files may skip — AFTER asking human partner.
- **Debugging integration**: "never fix bugs without a test."

**BC-DRAFT-005: Systematic debugging phase-ordering with architectural escape valve.** `systematic-debugging/SKILL.md:18-21, 48-213`. Phases sequential; **Phase 4.5**: if fix#3 fails, STOP and question architecture.

**BC-DRAFT-006: 5-step verification gate before any completion claim.** `verification-before-completion/SKILL.md:18-38`. IDENTIFY → RUN → READ → VERIFY → CLAIM. Skipping any = "lying, not verifying".

**BC-DRAFT-007: Per-task fresh subagent + strict-order two-stage review (round-3 amended).** `SKILL.md:6-13, 42-85, 247, 63, 82-83, 195-197`. **Round-3 amendment — terminal final-reviewer:** after ALL per-task reviews, SDD dispatches ONE MORE code-reviewer subagent over the entire implementation before `finishing-a-development-branch`.

**BC-DRAFT-008: User instructions override skills.** `using-superpowers/SKILL.md:19-26`. Priority: user > skills > default.

**BC-DRAFT-009: Subagents skip the bootstrap skill.** `using-superpowers/SKILL.md:6-8`. Via `<SUBAGENT-STOP>`.

### 4.2 Contracts from Subagent Prompt Templates (4 — HIGH confidence)

**BC-DRAFT-010: Implementer self-review before reporting DONE.** `implementer-prompt.md:74-98`. 4-category self-review (Completeness, Quality, Discipline, Testing).

**BC-DRAFT-011: Implementer MUST return one of four status codes (round-3 amended).** `implementer-prompt.md:100-113`. **Round-3 amendment — ordered BLOCKED remediation ladder:** (1) more context, (2) more capable model, (3) split into smaller pieces, (4) escalate to human. **Absolute rule**: "Never ignore an escalation or force the same model to retry without changes" (`SKILL.md:118`). DONE_WITH_CONCERNS decision split: correctness → fix first; observational → proceed.

**BC-DRAFT-012: Spec reviewer MUST verify by reading code, NOT by trusting implementer report (adversarial).** `spec-reviewer-prompt.md:21-37`. Default framing: "The implementer finished suspiciously quickly. Their report may be incomplete, inaccurate, or optimistic."

**BC-DRAFT-013: Quality review MUST NOT start before spec review returns correct.** `SKILL.md:247`.

### 4.3 Meta-Contracts from writing-skills (4 — HIGH confidence)

**BC-DRAFT-014: No skill (new or edited) without a failing pressure test.** `writing-skills/SKILL.md:374-393`. Pressure scenario MUST combine **3+ pressures** from {Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic}. Agent behavior documented word-for-word.

**BC-DRAFT-015: Skill descriptions MUST NOT summarize workflow.** `writing-skills/SKILL.md:150-172`. Only triggering conditions ("Use when...").

**BC-DRAFT-016: Discipline skills MUST include rationalization table + red flags list.** `writing-skills/SKILL.md:459-531`. Discipline skills MUST use Authority + Commitment + Social Proof and MUST NOT use Liking or heavy Reciprocity.

**BC-DRAFT-017: Skills MUST NOT use `@` force-loading references.** `writing-skills/SKILL.md:278-288`. Reference by name with explicit marker; no `@skills/...` which force-loads 200k+ context.

### 4.4 Brainstorming Contracts (3)

**BC-DRAFT-018:** Brainstorming terminal state is writing-plans exclusively. `brainstorming/SKILL.md:66`.
**BC-DRAFT-019:** Visual Companion offer MUST be its own message. `:152-154`.
**BC-DRAFT-020:** Per-question visual-vs-terminal decision; content fragments default. `:156-162`.

### 4.5 Receiving-Code-Review Contracts (3)

**BC-DRAFT-021:** Unclear review items halt ALL implementation. `receiving-code-review/SKILL.md:42-48`.
**BC-DRAFT-022:** Forbidden gratitude class. `:28-33, 132-148`.
**BC-DRAFT-023:** YAGNI check before implementing "proper" features. `:88-98`.

### 4.6 Finishing-a-Development-Branch Contracts (4)

**BC-DRAFT-024:** Tests MUST pass before presenting options.
**BC-DRAFT-025:** Exactly 4 options, no explanation added.
**BC-DRAFT-026:** Discard requires typed "discard" confirmation.
**BC-DRAFT-027:** executing-plans MUST invoke finishing-a-development-branch as terminal step.

### 4.7 SDD Safety Contracts (3)

**BC-DRAFT-028:** SDD MUST NOT start on main/master without explicit user consent.
**BC-DRAFT-029:** SDD MUST NOT dispatch multiple implementer subagents in parallel.
**BC-DRAFT-030:** Controller provides full task text inline; subagent MUST NOT read plan file.

### 4.8 Bootstrap & Platform Contracts (3)

**BC-DRAFT-031:** Bootstrap injection JSON shape is platform-conditional (3 shapes).
**BC-DRAFT-032:** Legacy skills dir triggers first-reply warning.
**BC-DRAFT-033:** OpenCode bootstrap injects into first user message, idempotently.

### 4.9 Plan Document Contracts (4)

**BC-DRAFT-034:** Plan header MANDATORY and declares required execution sub-skill. `writing-plans/SKILL.md:47-61`.
**BC-DRAFT-035:** No-placeholders contract (see §5.5 for verbatim list).
**BC-DRAFT-036:** Plan self-review is inline 3-check, NOT a subagent dispatch. `writing-plans/SKILL.md:122-132`.
**BC-DRAFT-037:** Writing-plans MUST end with execution handoff offering exactly two options.

### 4.10 Worktree Contracts (3)

**BC-DRAFT-038: Project-local worktree MUST be gitignore-covered before creation.** `git check-ignore` verification.
**BC-DRAFT-039: Clean-baseline test gate on new worktree.**
**BC-DRAFT-040: Deterministic worktree directory selection precedence.**

### 4.11 Implementer Escalation Contracts (2)

**BC-DRAFT-041:** Implementer MUST NOT unilaterally split files beyond plan intent; report DONE_WITH_CONCERNS.
**BC-DRAFT-042:** Implementer MUST stop and escalate when in over head. 5 named STOP triggers.

### 4.12 Code Review Dispatch Contracts (2)

**BC-DRAFT-043:** 3-tier severity taxonomy (Critical/Important/Minor).
**BC-DRAFT-044:** Requesting-code-review passes precisely crafted context with BASE_SHA/HEAD_SHA.

### 4.13 Test Assertion Contracts (1)

**BC-DRAFT-045:** Skill-triggering test pass = stream-json tool-invocation match. `run-test.sh:57-68`. `grep -qE '"skill":"([^"]*:)?<SKILL_NAME>"'`.

### 4.14 Governance / Contributor Contracts (2)

**BC-DRAFT-046:** Anthropic-compliance skill PRs rejected without eval evidence. `CLAUDE.md:35-37, 67-74`.
**BC-DRAFT-047:** Deprecation-shim commands MUST respond by naming replacement skill. All 3 instances in `commands/`.

### 4.15 Tension Audit (all 8 resolved)

| ID | Tension | Resolution |
|---|---|---|
| T1 | Skills mandatory vs user overrides | Priority: user > skills > default. No conflict. |
| T2 | SDD sequential vs parallel-agents | Scope-distinguished (implementation vs investigation). |
| T3 | Verify-before-completion vs social pressure | Pressure IS the adversary the skill addresses. |
| T4 | Push back vs politeness | Technical correctness > social comfort. The tension IS the contract. |
| T5 | Implementer self-review vs spec-reviewer skepticism | Defense in depth. Intentional adversarial layer. |
| T6 | Inline plan self-review vs SDD two-stage review | Different phases: authorial (pre-impl) vs post-impl. |
| T7 | anthropic-best-practices.md vs BC-046 rejection | Reference material, not authority. Deliberate tension. |
| T8 | Persuasion "use Authority" vs Anthropic "concise" | Different axes: tone vs token budget. |

**Zero genuine contradictions.**

---

## 5. Non-Functional Requirements

### 5.1 Iron Law Inventory (5 laws)

| # | Iron Law (verbatim) | Skill | Location |
|---|---|---|---|
| 1 | **NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST** | test-driven-development | `test-driven-development/SKILL.md:33` |
| 2 | **NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST** | systematic-debugging | `systematic-debugging/SKILL.md:18` |
| 3 | **NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE** | verification-before-completion | `verification-before-completion/SKILL.md:18-19` |
| 4 | **NO SKILL WITHOUT A FAILING TEST FIRST** | writing-skills | `writing-skills/SKILL.md:377` (header at :374: `## The Iron Law (Same as TDD)`) |
| 5 | **HARD-GATE on implementation until brainstorming approved** | brainstorming | `brainstorming/SKILL.md:12-14` |

Flexible skills deliberately do NOT get Iron Laws; they get `**Core principle:** <one-liner>` instead.

### 5.2 TDD Enforcement Extensions

- **Verify-RED gate** / **Verify-GREEN gate** / **Exception protocol** / **Debugging integration** / **"Delete means delete" loophole closures** / **When-stuck table**

### 5.3 Eval-Evidence Requirement (Skills as Code)

`CLAUDE.md:67-74` formalizes the eval-evidence NFR. Key operative rules verbatim:

- "Skills are not prose - they are code that shapes agent behavior" (`:69`)
- "Run adversarial pressure testing across multiple sessions" (`:72`)
- "Show before/after eval results in your PR" (`:73`)
- "Do not modify carefully-tuned content (Red Flags tables, rationalization lists, 'human partner' language) without evidence the change is an improvement" (`:74`)

### 5.4 Safety Guardrails

Red Flags tables, HARD-GATE markup, Forbidden Gratitude Class, Circle K codephrase, Unclear-Item Halt, YAGNI Check, Plan No-Placeholders, Worktree gitignore invariant, Clean-baseline gate, Implementer STOP triggers, File-Growth Escalation, SUBAGENT-STOP tag, Instruction Priority.

### 5.5 Plan Document No-Placeholders Class (CORRECTED)

**Round 1 claimed the list included `XXX`, `???`, and ellipsis-as-content — WRONG.** The actual exact list at `writing-plans/SKILL.md:109` reads verbatim:

> "TBD", "TODO", "implement later", "fill in details"

The full section at lines 108-114 is broader than a token list:

1. The four literal placeholder strings above (line 109).
2. "Add appropriate error handling" / "add validation" / "handle edge cases" (line 110) — vague-verb placeholders.
3. "Write tests for the above" without actual test code (line 111).
4. "Similar to Task N" without repeating the code (line 112).
5. Steps that describe what to do without showing how (line 113).
6. References to types/functions/methods not defined in any task (line 114).

**The rule is an invariant** ("every step must contain the actual content an engineer needs", line 107), not a token blacklist. Violations are classified as **"plan failures"** (line 108, verbatim taxonomic term).

### 5.6 Observability

- No metrics, no tracing, no structured logging.
- `tests/claude-code/analyze-token-usage.py`
- `tests/skill-triggering/run-test.sh` — grep-based stream-json assertion (BC-DRAFT-045). **Canonical behavioral observability harness.**
- Pressure testing as behavioral verification.

### 5.7 Pressure Taxonomy (CORRECTED)

**Round 1 claimed "time, authority, sunk cost, social proof, urgency, flattery, confusion" — partly fabricated.** Actual canonical seven at `testing-skills-with-subagents.md:128-138`:

| Pressure | Example (verbatim from source) |
|---|---|
| **Time** | Emergency, deadline, deploy window closing |
| **Sunk cost** | Hours of work, "waste" to delete |
| **Authority** | Senior says skip it, manager overrides |
| **Economic** | Job, promotion, company survival at stake |
| **Exhaustion** | End of day, already tired, want to go home |
| **Social** | Looking dogmatic, seeming inflexible |
| **Pragmatic** | "Being pragmatic vs dogmatic" |

**Corrections:** "urgency" doesn't exist (subsumed by Time); "flattery" doesn't exist (forbidden as authoring principle, not a tested pressure); "confusion" doesn't exist. **Economic, Exhaustion, Pragmatic missed entirely.**

**"Pragmatic" is load-bearing**: the repo treats **the anti-dogma stance itself as an attack vector** against discipline skills. This maps directly onto SOUL.md #1 ("Pragmatism Over Purity") as a tension point — superpowers treats pragmatism-as-excuse as the enemy of discipline; vsdd-factory treats pragmatism as the governing meta-principle.

**Compounding rule** (line 140, verbatim): "Best tests combine 3+ pressures."

**Meincke 2025 empirical anchor**: `skills/writing-skills/persuasion-principles.md:7` — **N=28,000 AI conversations, persuasion techniques more than doubled compliance (33% → 72%, p < .001)**.

### 5.8 Reliability

- Zero-dependency policy (`CLAUDE.md:31-33`).
- Polyglot hook dispatcher (`hooks/run-hook.cmd`) for Windows.
- Bash 5.3+ heredoc hang workaround (issue #571).
- OpenCode idempotency invariant.
- Legacy skills-directory warning.

### 5.9 Performance / Context Budget

- Only `using-superpowers` injected eagerly; all other skills lazy-loaded.
- SDD fresh-subagent-per-task preserves parent context.
- **Token budgets**: Flexible < 500 words; Discipline < 200 words in core body; description field ≤ 1024 chars.

### 5.10 Portability (6 platforms)

Claude Code, Cursor, Codex, OpenCode, Gemini CLI, Copilot CLI. Portability encoded as contracts (BC-031, BC-033, BC-045).

### 5.11 Governance / PR-Hygiene (first-class NFR)

**The most distinctive NFR category in the repo.** 14 rules from `CLAUDE.md:1-86`:

1. **94% PR rejection rate** as empirical justification (`:7`).
2. **PR template must be fully filled** (`:13, :23`).
3. **Existing-PR search** open AND closed mandatory (`:14, :25`).
4. **No speculative fixes** (`:15, :47-49`).
5. **One problem per PR** (`:45, :83`).
6. **Domain-specific skills → standalone plugins** (`:16, :41`).
7. **Third-party dependencies forbidden** except new-harness support (`:31-33`).
8. **"Compliance" PRs rejected** without eval evidence (`:35-37`).
9. **Fabricated / fork-specific / bundled-unrelated = immediate close** (`:55-65`).
10. **Human-involvement evidence required** (`:17, :27`).
11. **"Understand the project before contributing"** (`:76-78`) — rejection class for PRs that "rewrite the project's voice or restructure its approach without understanding why it exists".
12. **"Test on at least one harness and report results in the environment table"** (`:84`).
13. **"Describe the problem you solved, not just what you changed"** (`:85`). Aligns with SOUL.md #6.
14. **Framing rule** (`:9`, verbatim): "Your job is to protect your human partner from that outcome. Submitting a low-quality PR doesn't help them [...] That is being a tool of embarrassment." **Reframes the agent's objective function** from "maximize PR throughput" to "maximize human-partner reputation preservation".

The agent is framed as an **accomplice to the human partner, not an autonomous contributor**. Entirely prose-and-reviewer enforced.

### 5.12 Missing / Unexpected

- No formal verification. No fuzzing. No proofs.
- No versioning of individual skills.
- No CI workflows.

---

## 6. Conventions

### 6.1 SKILL.md Frontmatter Schema

```
---
name: <kebab-case-skill-name>
description: <trigger criterion string>
---
```

### 6.2 CSO Rule (WHEN-not-WHAT) — Canonical Case Study

- **Hard ceiling:** 1024 characters.
- **Semantic rule (`writing-skills/SKILL.md:99-100`):**
  > `description`: Third-person, describes ONLY when to use (NOT what it does)
  >   - Start with "Use when..." to focus on triggering conditions
- **Empirical rationale (`:154-156`, verbatim):**
  > "Testing revealed that when a description summarizes the skill's workflow, Claude may follow the description instead of reading the full skill content. A description saying 'code review between tasks' caused Claude to do ONE review, even though the skill's flowchart clearly showed TWO reviews [...]. When the description was changed to just 'Use when executing implementation plans with independent tasks' (no workflow summary), Claude correctly read the flowchart and followed the two-stage review process."

**CSO is not a style preference; it is a behavioral correctness requirement.**

### 6.3 Naming and Skill Document Structure

**Naming:** kebab-case, action-gerund ("brainstorming", "writing-plans", "using-git-worktrees").

**Skill Document Structure (10 sections):**

1. Frontmatter (name + description)
2. Optional `<SUBAGENT-STOP>` / `<EXTREMELY-IMPORTANT>` / `<HARD-GATE>` tags
3. `# Title`
4. `## Overview` — 1-3 sentences
5. `**Core principle:** <one-liner>` (Flexible) or `## The Iron Law` (Rigid)
6. `**Violating the letter of the rules is violating the spirit of the rules.**` line
7. `**Announce at start:** "I'm using the <skill> skill to <purpose>."` — **not optional** for behavior-shaping skills.
8. `## When to Use` / `## The Process` — flow as inline Graphviz `dot`. **Never Mermaid.**
9. `## Red Flags` — rationalization table AFTER main process. Load-bearing ordering.
10. Examples / anti-patterns / references

### 6.4 Tag Convention

| Tag | Semantics | Enforcement |
|---|---|---|
| `<EXTREMELY-IMPORTANT>` | Non-optional, read-first. Iron Laws and safety rails. | Trust-based |
| `<HARD-GATE>` | Forbidden from proceeding past gate until approval. | Trust-based |
| `<SUBAGENT-STOP>` | Terminates recursive skill loading in dispatched subagents. | Trust-based |

### 6.5 Iron Law Canonical Form

```
> NO <verb> <scope> WITHOUT <prerequisite> FIRST
```

Reserved for Rigid skills. Flexible skills use `**Core principle:** <one-liner>`.

### 6.6 Skill-Type Two-Axis Categorization

- **Axis 1 — Rigidity:** Rigid (Iron Law) vs Flexible (Core principle).
- **Axis 2 — Function:** Discipline / Technique / Pattern / Reference.

Compositions: TDD = Rigid-Discipline, writing-plans = Flexible-Technique, dispatching-parallel-agents = Flexible-Pattern, `copilot-tools.md` = Flexible-Reference.

### 6.7 Persuasion Principles Matrix (CORRECTED — 7 principles)

**Round 1 documented 6. Actual is SEVEN.** Grounded in Cialdini 2021 and Meincke 2025.

| # | Principle | Permitted? | Source |
|---|---|---|---|
| 1 | **Authority** | Yes — primary for discipline skills | `persuasion-principles.md:11-28` |
| 2 | **Commitment** | Yes — announce-at-start, TodoWrite, forced choices | `:30-47` |
| 3 | **Scarcity** | **Yes — first-class permitted** (round 2 correction) | `:49-66` |
| 4 | **Social Proof** | Yes — "Every time", "X without Y = failure" | `:68-85` |
| 5 | **Unity** | Yes — "we're colleagues", "your human partner" | `:87-103` |
| 6 | **Reciprocity** | **Avoid almost always** (round 2: missed entirely in round 1) | `:105-113` |
| 7 | **Liking** | **FORBIDDEN for compliance enforcement** — creates sycophancy | `:115-124` |

**Combination matrix** (`:128-133`, verbatim):

| Skill Type | Use | Avoid |
|---|---|---|
| Discipline-enforcing | Authority + Commitment + Social Proof | Liking, Reciprocity |
| Guidance/technique | Moderate Authority + Unity | Heavy authority |
| Collaborative | Unity + Commitment | Authority, Liking |
| Reference | Clarity only | All persuasion |

**Ethical test** (`:165`): "Would this technique serve the user's genuine interests if they fully understood it?"

**Parahuman-model claim** (`:147-151`): "LLMs are parahuman. Trained on human text containing these patterns." **Theoretical basis for the entire repo's behavior-shaping philosophy.**

A skill relying on Liking is a **correctness bug**; a skill relying on Reciprocity is an **authoring smell**.

### 6.8 Voice Conventions

- **"Your human partner"** — not "the user" (`CLAUDE.md:78`).
- Imperative mood, ALL-CAPS for absolute rules.
- Confrontational tone toward agent's own tendencies.
- **Forbidden Gratitude class** (`receiving-code-review/SKILL.md:29-32`): banned phrases.
- **Circle K codephrase**: out-of-band reset token.

### 6.9 Plan Document Conventions

Plan files at `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`.

**No-Placeholders forbidden set (verbatim from `writing-plans/SKILL.md:106-114`):**

> Every step must contain the actual content an engineer needs. These are **plan failures** - never write them:
> - "TBD", "TODO", "implement later", "fill in details"
> - "Add appropriate error handling" / "add validation" / "handle edge cases"
> - "Write tests for the above" (without actual test code)
> - "Similar to Task N" (repeat the code)
> - Steps that describe what to do without showing how
> - References to types, functions, or methods not defined in any task

**Canonical taxonomic term**: **"plan failures"** (verbatim, named class).

### 6.10 Graphviz DOT, Not Mermaid

Every process flow in `dot` code. New skill using Mermaid = convention violation.

### 6.11 Red Flags Table Pattern

12+ entries typical. Positioned after the main process. **Behavioral TDD**: enumerate failure modes from pressure testing, plug each. Protected by eval-evidence rule (`CLAUDE.md:74`).

### 6.12 Skill-Priority Rules

`using-superpowers/SKILL.md:98-105`:
1. Process skills first (brainstorming, debugging) — determine HOW
2. Implementation skills second — guide execution

### 6.13 Cross-Platform Tool-Mapping

Every non-Claude platform gets a tool-name translation table in `skills/using-superpowers/references/`. A missing mapping is a portability bug.

### 6.14 Deprecation-Shim Template (Located and Enumerated)

**All three instances live in `commands/`, not `skills/`:**

| File | Legacy entry | Replacement | Verbatim redirect text |
|---|---|---|---|
| `commands/brainstorm.md:5` | `/brainstorm` | `superpowers brainstorming` | "Tell your human partner that this command is deprecated and will be removed in the next major release. They should ask you to use the 'superpowers brainstorming' skill instead." |
| `commands/write-plan.md:5` | `/write-plan` | `superpowers writing-plans` | (same structure) |
| `commands/execute-plan.md:5` | `/execute-plan` | `superpowers executing-plans` | (same structure) |

**Template rule:** a deprecation shim (a) tells human partner the command is deprecated, (b) names exactly one replacement skill, (c) announces removal in "the next major release", (d) contains no process content. **Command-layer mechanism, not skill-layer.**

### 6.15 Output Conventions

- Design docs: `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`
- Plans: `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`

### 6.16 Anti-Patterns List

- `testing-anti-patterns.md` — reference doc (299 LOC).
- "This is just a simple question" (Red Flag).
- "This Is Too Simple To Need A Design".
- **Compliance-with-Anthropic-skills-docs restructuring** (per `CLAUDE.md:35-37`).
- **Liking-as-persuasion** — sycophancy risk.
- **Reciprocity-as-persuasion** — manipulative.
- **Mermaid flow diagrams** where DOT is expected.
- **"the user"** instead of "your human partner".
- **Description-summarizes-workflow** — empirically causes skill body to be skipped.
- **First-person descriptions** — "I can help you..." forbidden.

---

## 7. Tensions, Gaps, and Convergence Corrections

### 7.1 Tension Audit Status (all 8 resolved)

See §4.15. Zero genuine contradictions.

### 7.2 Unresolved Gaps (by design)

- **(a) No machine-readable contract index.** Will never exist by design — contracts live in the same prose the agent reads.
- **(b) Compliance is probabilistic.** Not a bug; it is the **governing constraint** of the entire project. Mitigation is adversarial pressure testing. The Meincke 2025 anchor is the evidence this mitigation works.

### 7.3 Convergence Corrections (Protocol Validation)

Phase B caught and corrected **three round-1 hallucinations** — core evidence that the strict-binary novelty protocol actually finds fabrications.

**Correction 1 — Pressure Taxonomy (Pass 4).** Round 1 claimed "time, authority, sunk cost, social proof, urgency, flattery, confusion". Actual canonical seven at `testing-skills-with-subagents.md:128-138`: **Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic**. Round 1 invented "urgency/flattery/confusion" and missed Economic/Exhaustion/Pragmatic entirely.

**Correction 2 — Persuasion Matrix (Pass 5).** Round 1 documented 6 principles. Actual is **seven** — Reciprocity was missing. Scarcity was under-classified as "framing only" when source treats it as first-class permitted.

**Correction 3 — Plan No-Placeholders token list (Pass 4/5).** Round 1 claimed the list included `XXX`, `???`, ellipsis. None appear in actual source. Actual verbatim list at `writing-plans/SKILL.md:109` is only `"TBD", "TODO", "implement later", "fill in details"`. Broader rule is an invariant, not a token blacklist. "Plan failures" captured as named taxonomic term.

**Protocol validation:** all three corrections were caught by re-reading source files during round 2. The strict-binary novelty protocol (SUBSTANTIVE vs NITPICK, no middle ground) forced round 2 to justify findings as model-changing, which surfaced round 1's errors. **The protocol works.**

---

## 8. Philosophical Comparison: Superpowers vs vsdd-factory

### 8.1 TL;DR

| Axis | Superpowers | vsdd-factory (Corverax VSDD) |
|---|---|---|
| **Control model** | Auto-triggered skills; agent is the orchestrator | Orchestrator-driven; skills are phases invoked by slash commands |
| **Spec model** | Spec-emergent (brainstorm → design → plan) | Spec-first (brief → domain → PRD → arch → BCs → VPs) |
| **Gate model** | Skill priorities + Iron Laws (rhetorical) | Phase gates + wave gates + convergence checks (procedural) |
| **Enforcement** | Red Flags + HARD-GATE + Iron Laws (prose) | Hooks, lefthook, CI, types |
| **TDD** | Iron Law; "delete pre-test code" | /deliver-story RED gate + micro-commits + mutation kill ≥90% |
| **Adversarial review** | Per-task two-stage + terminal final reviewer via fresh subagents | Dedicated adversary agent with novelty-decay loops |
| **Persistence** | `docs/superpowers/` dated files | `.factory/` orphan branch with STATE.md, BC-INDEX |
| **Platforms** | 6 platforms first-class | Claude Code only |
| **Domain artifacts** | None (workflow IS the product) | L1-L4 spec hierarchy, BCs, VPs, ARCH, holdout scenarios |
| **Governance** | First-class NFR with 14 rules, 94% rejection rate | Conventional commits + lefthook + CI |
| **Tone** | Confrontational anti-rationalization | Pragmatic engineering (SOUL.md #1) |
| **Observability** | Stream-json grep harness | Tracing from inception |

### 8.2 Section 1 — Orchestrator-driven vs Auto-triggered Skills

**vsdd-factory:** Human drives the pipeline with explicit slash commands. The human is the scheduler.

**Superpowers:** Agent drives the pipeline via auto-invocation. The *agent* is the scheduler.

**Trade-off:** vsdd-factory gives predictability and auditability; superpowers gives adaptability and low friction. vsdd-factory handles complex multi-artifact dependencies (BC-to-VP-to-ARCH traceability); superpowers collapses under that complexity because there is no dependency graph.

**The crucial difference:** vsdd-factory's gates can fail-closed (automated check blocks progress), while superpowers' gates can only fail-open (rhetoric + model discretion).

### 8.3 Section 2 — Spec-first vs Spec-emergent

**vsdd-factory:** L1 product brief → L2 domain spec → L3 behavioral contracts → L4 verification properties → architecture → stories → waves. Specs ARE the product (SOUL.md #3).

**Superpowers:** Brainstorm → design doc → plan doc. Two artifacts. Plans are bite-sized task lists (2-5 min each). No ubiquitous-language glossary, no entity model, no BC library.

**When each wins:** For systems with meaningful invariants and multi-wave development, vsdd-factory's hierarchy prevents drift. For a solo engineer's next feature, superpowers' brainstorm→plan→execute is dramatically faster.

**The insight:** superpowers' spec artifacts are *ephemeral by design*. Design docs and plans are dated files, not living documents. vsdd-factory assumes specs are living and must be protected from drift. **These are incompatible philosophies**, encoding different assumptions about **how many artifacts need to stay consistent with each other**.

### 8.4 Section 3 — Phase Gates vs Skill Priorities

**vsdd-factory** gates are procedural and artifact-checked: `/wave-gate` runs 6 checks; `/convergence-check` verifies 7 dimensions. A gate either passes or fails; failing blocks progress.

**Superpowers** has no gates. Skill priorities + HARD-GATE tags. Enforcement is rhetorical.

**Risk asymmetry:** vsdd-factory's gates can block bad releases but can also falsely block on a tooling bug; superpowers' gates cannot block but also cannot be broken by a tooling bug.

### 8.5 Section 4 — Hook Enforcement

**vsdd-factory:** Hooks are **active enforcement** — spec-steward blocks, push-guard verifies, etc. These hooks **reject** actions.

**Superpowers:** The one hook is **passive injection**. It cannot reject anything.

**This is the clearest architectural difference:** vsdd-factory treats hooks as **guards**, superpowers treats hooks as **loaders**. vsdd-factory's hooks implement Principle #2 ("Make the wrong thing impossible") at the filesystem/git layer.

### 8.6 Section 5 — TDD Discipline

**Superpowers:** Iron Law + multiple Red Flags + loophole closures + Verify-RED/GREEN gates + "delete means delete" rule. Extensive rhetorical enforcement.

**vsdd-factory:** Multi-layered — workflow + CI + mutation kill rate ≥90% + RED gate verifying correct failure reason. Evidence-based enforcement.

**vsdd-factory is stronger on verification that TDD was actually followed** (mutation kill catches vacuous tests). **Superpowers is stronger on the front-end discipline of starting with the test.** Complementary, not substitutes.

### 8.7 Section 6 — What Each Does That the Other Doesn't

#### 8.7.1 Superpowers does, vsdd-factory doesn't

- **Multi-platform first-class support** (6 platforms).
- **Auto-triggered skills with the 1% rule.**
- **Per-task fresh-subagent discipline** (SDD pattern).
- **SDD terminal final-reviewer** (Phase B round 3 discovery).
- **BLOCKED remediation ladder** (Phase B round 3 discovery).
- **Plan Document schema with self-describing header** (Phase B round 2 discovery).
- **Worktree Safety domain** (Phase B round 2 discovery).
- **Persuasion Principles Matrix** (grounded in Cialdini + Meincke).
- **Governance NFR as first-class category** (14 rules, 94% rejection rate).
- **21-edge skill-chain dependency graph** with typed edges.
- **SDD fixture triple format** for empirical skill validation.
- **Universal terminal sink convention** (finishing-a-development-branch).
- **Adversarial skill pressure-testing methodology** (RED-GREEN-REFACTOR on agent behavior).
- **Red Flags table pattern** in every discipline skill.
- **Verification-before-completion as a dedicated skill** with 5-step gate.
- **Deprecation as a first-class move** (shim pattern).
- **Dispatching-parallel-agents skill** with boundary rules.
- **Circle K codephrase** for out-of-band agent reset.
- **In-Over-Your-Head contract** (5 explicit STOP triggers).
- **File-Growth Escalation rule** (DONE_WITH_CONCERNS over unilateral splitting).

#### 8.7.2 vsdd-factory does, superpowers doesn't

- **Hierarchical spec model** (L1-L4 with traceability).
- **Phase and wave gates** that actually block progress.
- **Persistent state** via `.factory/` orphan worktree.
- **Holdout scenario evaluation**.
- **Formal verification path** (Kani, fuzz, mutants, semgrep).
- **Mutation testing** as a quality gate (≥90%).
- **Cycle-scoped artifacts**.
- **Active guarding hooks** that reject.
- **Per-module criticality classification**.
- **Gene-transfusion / semport** alternative implementation strategy.
- **Brownfield ingest with convergence deepening** (this operation).
- **Type-system enforcement** of invariants.
- **Conventional Commits + lefthook + CI**.
- **Structured tracing and observability** from inception.
- **Git flow with main/develop/feature branches** and branch protection.
- **Single-language codebase** with MSRV and workspace deps.

### 8.8 Section 7 — What vsdd-factory Should Learn from Superpowers

1. **Adopt the Red Flags table pattern in every skill and slash command.** Free quality.
2. **Adopt the "announce at start" convention.** Creates human-visible audit trail.
3. **Adopt the per-task fresh-subagent pattern (SDD) for /deliver-story.** Arguably the biggest potential improvement.
4. **Adopt the SDD terminal final-reviewer pass.** Run one adversarial review over the entire wave diff before `/wave-gate`.
5. **Adopt skill pressure-testing methodology.** Empirically tune skills instead of hand-designing.
6. **Adopt the BLOCKED remediation ladder.** Ordered escalation: context → model → split → human.
7. **Adopt the Plan Document self-describing header pattern.** Stories carry `REQUIRED SKILL:` directive.
8. **Adopt the Plan Document No-Placeholders invariant.** Label violations as "story failures".
9. **Consider a lightweight SessionStart injection.** Primes every session with "check STATE.md first".
10. **Adopt `<EXTREMELY_IMPORTANT>` and `<HARD-GATE>` tag conventions** inside skills.
11. **Adopt the Persuasion Principles Matrix** for skill authoring audits.
12. **Adopt the Governance NFR as a first-class concept.** Elevate PR hygiene.
13. **Adopt the "tool of embarrassment" framing.** Reframe objective function.
14. **Adopt the verification-before-completion skill.** Dedicated 5-step gate at completion-claim boundaries.
15. **Adopt Circle K (or equivalent) codephrase.** Mid-session agent recovery signal.
16. **Adopt the Dispatching-Parallel-Agents skill** with boundary rules.
17. **Adopt the Worktree Safety domain model.**
18. **Adopt the fixture triple format (design + plan + scaffold)** for skill validation.
19. **Consider multi-platform portability as a future concern.**
20. **Adopt the CSO rule (WHEN not WHAT)** for slash-command descriptions.

### 8.9 Section 8 — What Superpowers Could Learn from vsdd-factory

1. **Hierarchical specs** for multi-wave systems.
2. **Active enforcement hooks** (not just injection). Push-guard that blocks pushes without verification.
3. **Mutation testing** as evidence that TDD was actually followed.
4. **Persistent state** via `.factory/`-style orphan worktree.
5. **Holdout scenarios** as a skill.
6. **Formal verification paths** for critical modules.
7. **Criticality-based review depth** (CRITICAL vs MEDIUM).
8. **Conventional Commits + lefthook** pre-commit layer.
9. **CI workflows.** Even light CI would add safety net.
10. **Single-language runtime** (eliminate the JS shim).

---

## 9. Lessons for vsdd-factory (Priority-Ordered)

### 9.1 P0 — Adopt Immediately

**P0-1: Per-task fresh-subagent SDD pattern in /deliver-story.**

- **Current:** `/deliver-story` runs RED/GREEN/REFACTOR in one agent context across an entire story.
- **Superpowers:** Every task delegated to fresh implementer subagent → spec reviewer → quality reviewer → (after all tasks) final reviewer.
- **Gap:** Context pollution across tasks, inherited rationalizations, no per-task adversarial check.
- **Actions:**
  1. Split `/deliver-story` into controller + implementer + spec-reviewer + quality-reviewer subagent pattern.
  2. Define implementer/spec-reviewer/quality-reviewer prompt templates.
  3. Add the **four status codes** (DONE, DONE_WITH_CONCERNS, BLOCKED, NEEDS_CONTEXT).
  4. Implement the **BLOCKED remediation ladder** with absolute rule "never force same model to retry without changes".
  5. Add **terminal final-reviewer** pass before `/wave-gate`.
  6. Add **File-Growth Escalation rule** (DONE_WITH_CONCERNS over unilateral splitting).
  7. Add **In-Over-Your-Head contract** (5 explicit STOP triggers).

*Evidence:* per-task pattern is the single biggest architectural differentiator; Phase B found it substantive enough to require round 3.

**P0-2: Red Flags tables in every vsdd-factory skill and slash command.**

- **Current:** No Red Flags tables; discipline relies on engineer attention + CI.
- **Superpowers:** Every discipline skill ships a Red Flags table with 12+ rationalization-rebuttal pairs, empirically derived.
- **Actions:**
  1. For each slash command, enumerate top 5-10 rationalizations.
  2. Add `## Red Flags` section with `| Thought | Reality |` table.
  3. Position AFTER the main process (load-bearing ordering).
  4. Treat tables as protected — changes require eval evidence.

*Evidence:* "free quality" — no architectural change, documented empirical effect on compliance.

**P0-3: "Announce at start" convention.**

- **Current:** Commands don't require announcement.
- **Superpowers:** Every behavior-shaping skill requires `**Announce at start:** "I'm using the <skill> skill to <purpose>."`
- **Actions:**
  1. Add requirement to every skill template.
  2. Consider stream-json grep test analogous to `tests/skill-triggering/run-test.sh`.

*Evidence:* leverages Commitment persuasion principle.

### 9.2 P1 — Adopt in Next Cycle

**P1-1: Adversarial skill pressure-testing methodology.**

- **Current:** Corverax skills hand-designed. No evidence they shift agent behavior.
- **Superpowers:** RED-GREEN-REFACTOR on agent behavior. Baseline subagent + 3+ combined pressures.
- **Actions:**
  1. Define baseline scenarios for each skill.
  2. Run without the skill; capture verbatim rationalizations.
  3. Revise skill to address them.
  4. Re-run. Iterate until bulletproof.
  5. Adopt **seven-type Pressure Taxonomy** (Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic).
  6. **Special attention to Pragmatic pressure**: "I'm just being pragmatic" is a named first-class attack vector. vsdd-factory's SOUL.md #1 ("Pragmatism Over Purity") creates a real tension — must distinguish principled pragmatism from pragmatism-as-rationalization.

**P1-2: Persuasion Principles Matrix audit.**

- **Actions:**
  1. Audit every skill against the Matrix.
  2. Discipline skills use Authority + Commitment + Social Proof; flag Liking or Reciprocity as correctness bugs.
  3. Add as `.claude/rules/` reference doc.
  4. Adopt ethical test: "Would this technique serve the user's genuine interests if they fully understood it?"

**P1-3: Plan Document self-describing header and No-Placeholders invariant for stories.**

- **Actions:**
  1. Add `REQUIRED SKILL:` frontmatter field to every story.
  2. Elevate story-completeness violations to a named class.
  3. Adopt invariant "every step must contain the actual content an engineer needs".

**P1-4: CSO rule (WHEN-not-WHAT) for slash-command descriptions.**

- **Actions:**
  1. Audit every skill's description field.
  2. Rewrite workflow-summarizing descriptions to describe triggering conditions only.
  3. Enforce third-person; forbid first-person.
  4. Enforce 1024-char ceiling.

### 9.3 P2 — Adopt When Convenient

**P2-1: Lightweight SessionStart injection.** Add a hook that primes every session with "check `.factory/STATE.md` first" + 1% rule. Passive loader complementing existing active guards.

**P2-2: `<EXTREMELY_IMPORTANT>` and `<HARD-GATE>` tag conventions.** Adopt tags for rules that MUST not be skipped.

**P2-3: Circle K codephrase (or equivalent).** Out-of-band reset signal. Document in CLAUDE.md + SOUL.md.

**P2-4: Dispatching-Parallel-Agents as a named pattern.** Create the skill with explicit boundary rules.

**P2-5: Worktree Safety domain model formalization.** Directory-Selection Precedence, gitignore invariant, clean-baseline gate, setup auto-detection. Add as BCs.

**P2-6: Governance NFR as first-class category.** Create `.claude/rules/governance-nfr.md`. Adopt "tool of embarrassment" framing.

### 9.4 P3 — Consider Later

**P3-1: Multi-platform portability.** Not a priority.
**P3-2: Fixture triple format.** Defer until holdout scenarios prove insufficient.
**P3-3: Skill-chain dependency graph with typed edges.** High effort for moderate payoff.
**P3-4: Graphviz DOT for flow diagrams.** Mermaid is fine; skip.
**P3-5: Deprecation shim pattern.** Adopt when first deprecation becomes necessary.
**P3-6: SDD fresh-subagent for Phase 3.5 holdout evaluation.** Already partially covered.

---

## 10. Convergence Metadata

### 10.1 Round Counts

| Pass | Rounds to converge | Final novelty |
|---|---|---|
| Pass 0 — Inventory | 2 | NITPICK |
| Pass 1 — Architecture | 2 | NITPICK |
| Pass 2 — Domain Model | 3 | NITPICK |
| Pass 3 — Behavioral Contracts | 3 | NITPICK |
| Pass 4 — NFRs | 2 | NITPICK |
| Pass 5 — Conventions | 2 | NITPICK |

**Total rounds:** 14. No pass needed the 5-round maximum.

### 10.2 Corrections Summary

Phase B caught and corrected **three round-1 hallucinations**, all in round 2:

1. **Pressure Taxonomy** (Pass 4): invented "urgency/flattery/confusion"; missed "Economic/Exhaustion/Pragmatic".
2. **Persuasion Matrix** (Pass 5): documented 6 when actual is 7; missed Reciprocity; under-classified Scarcity.
3. **Plan No-Placeholders token list** (Passes 4, 5): over-extrapolated; actual is 4 tokens, broader rule is an invariant.

### 10.3 Protocol Validation

The strict-binary novelty protocol is **validated**. Round 2 of Passes 4 and 5 was forced to justify findings as model-changing, which surfaced the three cases where round 1's model was factually wrong. **The protocol works.**

### 10.4 Source Artifact

- **Repo:** `obra/superpowers`
- **Commit:** `917e5f53b16b115b70a3a355ed5f4993b9f8b73d`
- **Total LOC analyzed:** ~8630 (skills + agents + commands + hooks + 112-LOC OpenCode JS)
- **Files analyzed:** 14 SKILL.md + 32 skill supporting files + 1 agent + 3 commands + 4 hooks + 1 OpenCode plugin + CLAUDE.md + test runners + SDD fixture triples

This document supersedes the Phase A `pass-6-synthesis.md` and becomes the authoritative knowledge document for obra/superpowers.
