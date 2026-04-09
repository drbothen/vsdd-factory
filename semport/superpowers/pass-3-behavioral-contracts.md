# Pass 3: Behavioral Contracts

_Phase B convergence round 1._

## Changes from Phase A

Phase A captured 9 BC-DRAFTs. Round 1 adds 24 new contracts (BC-DRAFT-010 through BC-DRAFT-033), a tension-audit section, and expands Phase A's BC-DRAFT-005 (debugging) and BC-DRAFT-006 (verification) into sub-contracts per phase/step. Source scan now includes subagent prompt files, session-start bash, OpenCode plugin, and writing-skills meta-contracts.

Superpowers does not have formal behavioral contract files (no BC-S.SS.NNN structure). Its contracts are encoded in:

1. **Skill markdown** — Iron Laws, HARD-GATEs, checklists, red-flag tables
2. **Subagent prompt templates** — implementer, spec-reviewer, code-quality-reviewer
3. **Hook script** — bootstrap injection + legacy warning
4. **OpenCode plugin** — config-hook + first-user-message transform
5. **Tests** — prompts + assertion scripts validating skill trigger + follow-through

## Contracts from Skills (HIGH confidence)

### BC-DRAFT-001: Bootstrap skill is injected every session
**Pre:** Plugin installed; platform fires SessionStart event with matcher `startup|clear|compact`
**Post:** Session context contains `<EXTREMELY_IMPORTANT>You have superpowers. ...using-superpowers SKILL.md content...</EXTREMELY_IMPORTANT>` via platform-specific JSON field
**Evidence:** `hooks/hooks.json:4-13`; `hooks/session-start:40-67`
**Confidence:** HIGH

### BC-DRAFT-002: Agent invokes Skill tool before any response when skill could apply
**Pre:** User message received
**Post:** If any skill description could plausibly match (1% threshold), Skill tool is invoked before any assistant text output
**Error case:** Agent rationalizes via Red-Flag pattern → skill content instructs it to stop and invoke
**Evidence:** `skills/using-superpowers/SKILL.md:10-16, 46, 80-95`
**Confidence:** HIGH (normative) / MEDIUM (compliance depends on the agent)

### BC-DRAFT-003: No implementation before design approval (brainstorming HARD-GATE)
**Pre:** Any creative work request
**Post:** A design has been presented and explicitly approved by the human partner before any implementation skill is invoked or code written
**Evidence:** `skills/brainstorming/SKILL.md:12-14`
**Confidence:** HIGH

### BC-DRAFT-004: No production code without a failing test (TDD Iron Law)
**Pre:** Implementing feature/bugfix/refactor
**Post:** A test existed and was observed to fail before corresponding production code was written
**Error case:** Code written before test → delete it, start over; no "adapt while writing tests"
**Evidence:** `skills/test-driven-development/SKILL.md:33-45`
**Confidence:** HIGH

### BC-DRAFT-005: Systematic debugging phase-ordering with architectural escape valve
**Pre:** Bug / test failure / unexpected behavior observed
**Post:** Phases completed sequentially: 1 (root cause) → 2 (pattern analysis) → 3 (single hypothesis, written down) → 4 (failing test, single fix, verify). No fixes proposed before Phase 1 complete. Phase 3 requires exactly ONE hypothesis at a time. Phase 4 requires failing test BEFORE fix.
**Phase 4.5 sub-contract:** If fix#3 fails, MUST STOP and question architecture with human partner. MUST NOT attempt fix#4.
**Evidence:** `skills/systematic-debugging/SKILL.md:18-21, 48-213`
**Confidence:** HIGH

### BC-DRAFT-006: 5-step verification gate before any completion claim
**Pre:** Agent about to claim "done", "passing", "fixed", or any synonym/paraphrase/implication of completion
**Post:** In the current message, the agent has executed: (1) IDENTIFY command, (2) RUN full command fresh, (3) READ full output + exit code + failure count, (4) VERIFY output confirms claim, (5) CLAIM with evidence. Each step is a sub-contract; skipping any = "lying, not verifying".
**Evidence:** `skills/verification-before-completion/SKILL.md:18-38`
**Confidence:** HIGH

### BC-DRAFT-007: Per-task fresh subagent + strict-order two-stage review
**Pre:** Executing a plan with independent tasks under SDD
**Post:** Each task dispatched to fresh implementer subagent; on implementer DONE, spec compliance reviewer dispatched; ONLY after spec reviewer returns ✅, code quality reviewer dispatched. Review loops until both return ✅. All three subagents receive precisely crafted context, never parent history.
**Error case:** Starting code quality review before spec compliance ✅ = forbidden (`SKILL.md:247`)
**Evidence:** `skills/subagent-driven-development/SKILL.md:6-13, 42-85, 247`
**Confidence:** HIGH

### BC-DRAFT-008: User instructions override skills
**Pre:** Conflict between a skill's rule and CLAUDE.md/GEMINI.md/AGENTS.md/direct request
**Post:** User instructions win. Priority order: user > skills > default system prompt.
**Evidence:** `skills/using-superpowers/SKILL.md:19-26`
**Confidence:** HIGH

### BC-DRAFT-009: Subagents skip the bootstrap skill
**Pre:** Agent invoked as a subagent for a specific task
**Post:** using-superpowers is skipped via `<SUBAGENT-STOP>` tag
**Evidence:** `skills/using-superpowers/SKILL.md:6-8`
**Confidence:** HIGH

## Contracts from Subagent Prompt Templates (HIGH confidence)

### BC-DRAFT-010: Implementer self-review before reporting DONE
**Pre:** Implementer subagent about to report status
**Post:** Agent completed 4-category self-review (Completeness, Quality, Discipline, Testing) and fixed any issues found BEFORE reporting
**Evidence:** `skills/subagent-driven-development/implementer-prompt.md:74-98`
**Confidence:** HIGH

### BC-DRAFT-011: Implementer MUST return one of four status codes; no silent uncertain work
**Pre:** Implementer finishing a task
**Post:** Reports exactly one of: DONE, DONE_WITH_CONCERNS, BLOCKED, NEEDS_CONTEXT. "Never silently produce work you're unsure about."
**Evidence:** `skills/subagent-driven-development/implementer-prompt.md:100-113`; `SKILL.md:103-118`
**Confidence:** HIGH

### BC-DRAFT-012: Spec reviewer MUST verify by reading code, NOT by trusting implementer report (adversarial)
**Pre:** Spec reviewer subagent dispatched after implementer DONE
**Post:** Reviewer reads actual code files, compares to requirements line-by-line, ignores implementer's claims. Default framing: "The implementer finished suspiciously quickly. Their report may be incomplete, inaccurate, or optimistic."
**Evidence:** `skills/subagent-driven-development/spec-reviewer-prompt.md:21-37, 56`
**Confidence:** HIGH

### BC-DRAFT-013: Quality review MUST NOT start before spec review returns ✅
**Pre:** SDD task has been reviewed by spec-reviewer
**Post:** Code quality reviewer is dispatched ONLY if spec reviewer returned ✅. If ❌, implementer fixes + spec re-review first.
**Evidence:** `skills/subagent-driven-development/SKILL.md:247`; `code-quality-reviewer-prompt.md:7`
**Confidence:** HIGH

## Meta-Contracts from writing-skills (HIGH confidence)

### BC-DRAFT-014: No skill (new or edited) without a failing pressure test
**Pre:** Creating or editing any SKILL.md
**Post:** A pressure-test scenario was run with a subagent WITHOUT the skill, baseline rationalizations were captured verbatim, then the skill was written to address those specific rationalizations
**Error case:** Skill written before testing → delete, start over. Applies to additions, edits, "documentation updates".
**Evidence:** `skills/writing-skills/SKILL.md:374-393`
**Confidence:** HIGH

### BC-DRAFT-015: Skill descriptions MUST NOT summarize workflow
**Pre:** Authoring a skill's YAML description field
**Post:** Description contains ONLY triggering conditions ("Use when..."). Does NOT summarize what the skill does or its workflow.
**Rationale:** Empirically validated — workflow-summarizing descriptions cause Claude to follow the description shortcut and SKIP the skill body. "code review between tasks" caused ONE review; "Use when executing implementation plans with independent tasks" caused correct TWO reviews.
**Evidence:** `skills/writing-skills/SKILL.md:150-172`
**Confidence:** HIGH

### BC-DRAFT-016: Discipline skills MUST include rationalization table + red flags list
**Pre:** Authoring a skill that enforces discipline (Iron Law-class)
**Post:** Skill contains (a) a rationalization table with excuses and rebuttals captured from baseline testing, (b) a red flags list enabling agent self-check, (c) explicit loophole closures ("don't keep as reference", "delete means delete"), (d) "violating the letter = violating the spirit" clause
**Evidence:** `skills/writing-skills/SKILL.md:459-531`
**Confidence:** HIGH

### BC-DRAFT-017: Skills MUST NOT use `@` force-loading references to other skills
**Pre:** Cross-referencing another skill from within a skill
**Post:** Reference by name with explicit requirement marker ("**REQUIRED SUB-SKILL:** Use superpowers:test-driven-development"); no `@skills/...` which force-loads 200k+ context
**Evidence:** `skills/writing-skills/SKILL.md:278-288`
**Confidence:** HIGH

## Brainstorming Sub-Contracts (HIGH confidence)

### BC-DRAFT-018: Brainstorming terminal state is writing-plans exclusively
**Pre:** Brainstorming checklist complete + user approved spec
**Post:** writing-plans is invoked. NO other skill (not frontend-design, not mcp-builder, not any implementation skill) is invoked as the next step.
**Evidence:** `skills/brainstorming/SKILL.md:66`
**Confidence:** HIGH

### BC-DRAFT-019: Visual Companion offer MUST be its own message
**Pre:** Agent anticipates upcoming visual questions in brainstorming
**Post:** Offer message contains ONLY the offer text — no clarifying questions, no context summaries, no combined content. Agent waits for response before continuing.
**Evidence:** `skills/brainstorming/SKILL.md:152-154`
**Confidence:** HIGH

### BC-DRAFT-020: Per-question visual-vs-terminal decision
**Pre:** User has accepted visual companion; agent about to ask next brainstorming question
**Post:** Agent decides per-question: browser if "user would understand better by SEEING than reading" (mockups, layouts, diagrams); terminal otherwise (requirements, concepts, tradeoffs). UI-topic questions are not automatically visual.
**Evidence:** `skills/brainstorming/SKILL.md:156-162`
**Confidence:** HIGH

## Receiving-Code-Review Contracts (HIGH confidence)

### BC-DRAFT-021: Unclear review items halt ALL implementation
**Pre:** Multi-item code review received; any item unclear
**Post:** STOP — no items implemented. Ask for clarification on unclear items FIRST. "Items may be related. Partial understanding = wrong implementation."
**Evidence:** `skills/receiving-code-review/SKILL.md:42-48`
**Confidence:** HIGH

### BC-DRAFT-022: Forbidden gratitude class in review responses
**Pre:** Responding to code review feedback
**Post:** Agent does NOT emit "You're absolutely right", "Great point", "Thanks [anything]", or any gratitude expression. Catch-and-delete rule: if about to type "Thanks", DELETE IT.
**Evidence:** `skills/receiving-code-review/SKILL.md:28-33, 132-148`; CLAUDE.md violation reference
**Confidence:** HIGH

### BC-DRAFT-023: YAGNI check before implementing "proper" features
**Pre:** External reviewer suggests "implementing X properly" with features
**Post:** Agent greps codebase for actual usage. If unused, proposes removal (YAGNI). If used, then implements properly.
**Evidence:** `skills/receiving-code-review/SKILL.md:88-98`
**Confidence:** HIGH

## Finishing-a-Development-Branch Contracts (HIGH confidence)

### BC-DRAFT-024: Tests MUST pass before presenting completion options
**Pre:** Implementation claimed complete; about to present options
**Post:** Full test suite run fresh; tests pass. If failing, STOP — do not present options, report failures and require fix.
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:18-38`
**Confidence:** HIGH

### BC-DRAFT-025: Exactly 4 options, no explanation added
**Pre:** Tests verified passing
**Post:** Agent presents exactly these options: (1) merge locally, (2) push+PR, (3) keep as-is, (4) discard. No added explanation.
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:50-64`
**Confidence:** HIGH

### BC-DRAFT-026: Discard requires typed "discard" confirmation
**Pre:** User selects Option 4
**Post:** Agent requires user to type the exact string "discard" before any destructive action. If anything else typed, abort.
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:116-124`
**Confidence:** HIGH

### BC-DRAFT-027: executing-plans MUST invoke finishing-a-development-branch as terminal step
**Pre:** All tasks in a plan executed under executing-plans
**Post:** Agent invokes finishing-a-development-branch as the completion step (not manual merge/PR).
**Evidence:** `skills/executing-plans/SKILL.md:34-38`
**Confidence:** HIGH

## SDD Safety Contracts (HIGH confidence)

### BC-DRAFT-028: SDD MUST NOT start on main/master without explicit user consent
**Pre:** About to begin SDD execution
**Post:** Agent verifies current branch; if main or master, obtains explicit user consent before proceeding. Also: using-git-worktrees invoked BEFORE any implementer dispatch.
**Evidence:** `skills/subagent-driven-development/SKILL.md:237, 268`
**Confidence:** HIGH

### BC-DRAFT-029: SDD MUST NOT dispatch multiple implementer subagents in parallel
**Pre:** Executing SDD with multiple tasks
**Post:** Implementer subagents dispatched strictly sequentially. Parallel dispatch forbidden — conflicts. (Contrast: dispatching-parallel-agents is for independent INVESTIGATIONS only.)
**Evidence:** `skills/subagent-driven-development/SKILL.md:240`
**Confidence:** HIGH

### BC-DRAFT-030: Controller provides full task text inline; subagent MUST NOT read plan file
**Pre:** Dispatching implementer subagent for a task
**Post:** The controller extracted and included full task text in the prompt. Implementer does NOT read the plan file — avoids file-reading overhead and context pollution.
**Evidence:** `skills/subagent-driven-development/SKILL.md:241`; `implementer-prompt.md:9-18`
**Confidence:** HIGH

## Bootstrap & Platform Contracts (HIGH confidence)

### BC-DRAFT-031: Bootstrap injection JSON shape is platform-conditional
**Pre:** SessionStart hook fires
**Post:** Hook emits exactly ONE of three JSON shapes based on env-var detection:
- `CURSOR_PLUGIN_ROOT` set → `{"additional_context": "..."}`
- `CLAUDE_PLUGIN_ROOT` set && `COPILOT_CLI` unset → `{"hookSpecificOutput": {"hookEventName": "SessionStart", "additionalContext": "..."}}`
- otherwise → `{"additionalContext": "..."}` (SDK standard, Copilot CLI)
Emitting both `additional_context` and `hookSpecificOutput` would cause Claude Code to double-inject (no dedup).
**Evidence:** `hooks/session-start:40-55`
**Confidence:** HIGH

### BC-DRAFT-032: Legacy skills dir triggers first-reply warning
**Pre:** SessionStart detects `~/.config/superpowers/skills` exists
**Post:** Hook injects an `<important-reminder>` the agent MUST surface in its FIRST reply telling the user to migrate custom skills to `~/.claude/skills`
**Evidence:** `hooks/session-start:12-15`
**Confidence:** HIGH

### BC-DRAFT-033: OpenCode bootstrap injects into first user message, idempotently
**Pre:** OpenCode session starts; `experimental.chat.messages.transform` fires
**Post:** Bootstrap injected into first USER message (not system — avoids token bloat #750 and multi-system-message breakage on Qwen #894). Guarded by substring check on `EXTREMELY_IMPORTANT`; no double-injection. Also: skills path auto-registered via `config` hook into `config.skills.paths` (no symlinks required).
**Evidence:** `.opencode/plugins/superpowers.js:84-110`
**Confidence:** HIGH

## Contracts from Tests (HIGH confidence; behavioral assertions)

- `tests/skill-triggering/prompts/` — 6 prompt files (dispatching-parallel-agents, executing-plans, requesting-code-review, systematic-debugging, test-driven-development, writing-plans) encode implicit phrasing that must auto-trigger the matching skill. Assertions in `tests/skill-triggering/run-test.sh`.
- `tests/explicit-skill-requests/prompts/` — explicit phrasing must trigger skill even under abbreviation ("SDD means", `i-know-what-sdd-means.txt`).
- `tests/claude-code/test-subagent-driven-development.sh` + `-integration.sh` — SDD dispatches correct subagent sequence.
- `tests/subagent-driven-dev/{go-fractals,svelte-todo}/` — end-to-end: scaffolded project + pre-written plan.md/design.md → agent should execute plan using SDD.
- `tests/opencode/test-priority.sh` — validates skill priority ordering (process skills first).
- `tests/brainstorm-server/*.test.js` — unit tests for the brainstorm visual-companion websocket server.

## Contracts from Contributor Guidelines (CLAUDE.md)

`CLAUDE.md` encodes meta-contracts about skill modification:

- Skill content is carefully tuned; changes require eval evidence (`CLAUDE.md:68-75`)
- "human partner" terminology is deliberate and not interchangeable (`CLAUDE.md:77-78`)
- Third-party dependencies forbidden (`CLAUDE.md:31-33`)
- Anthropic skill-authoring "compliance" PRs explicitly rejected (`CLAUDE.md:35-37`)

## Tension Audit

**T1: "Skills are mandatory" (BC-002, BC-014) vs "User instructions override skills" (BC-008).**
Resolution: explicit priority order in `using-superpowers.md:19-26`. User > skills > default. Skills are mandatory relative to the default system prompt, NOT relative to user instructions. No true conflict.

**T2: "SDD sequential implementer dispatch" (BC-029) vs "dispatching-parallel-agents parallel dispatch" (implicit from that skill).**
Resolution: scope-distinguished. Parallel dispatch is for independent INVESTIGATIONS (read-only bug triage across unrelated domains). Sequential dispatch is for IMPLEMENTATION under SDD. The skills are for different phases.

**T3: "Verify before completion" (BC-006) vs agent social pressure to respond quickly.**
Resolution: verification-before-completion codifies this as an Iron Law with rationalization table explicitly countering "I'm tired", "just this once", "confidence enough". No conflict — social pressure is the adversary the skill addresses.

**T4: "Receiving-code-review push back when wrong" (BC-022, forbidden gratitude) vs typical politeness.**
Resolution: the skill treats politeness as performative agreement and forbids it by name. Technical correctness > social comfort is the explicit principle (`SKILL.md:10-12`). No conflict — the "tension" IS the contract.

**T5: Implementer self-review (BC-010) vs spec reviewer "don't trust the report" (BC-012).**
Resolution: defense in depth. Self-review catches what the implementer knows. Spec reviewer catches what the implementer doesn't know or misrepresents. Adversarial layer is intentional. No conflict.

## Gaps

- No machine-readable contract index. Contracts are prose inside skills, prompt files, hook scripts, and a JS plugin.
- Compliance is ultimately probabilistic (model adherence to instructions); mitigation is adversarial pressure testing (per writing-skills), not type-system enforcement.
- `tests/skill-triggering/run-test.sh` scoring and assertion format not yet extracted into formal contracts.
- Third-party dependency ban (CLAUDE.md:31-33) is a repo-level policy contract but not attached to a skill.
