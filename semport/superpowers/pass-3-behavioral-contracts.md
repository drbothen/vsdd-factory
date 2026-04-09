# Pass 3: Behavioral Contracts

_Phase B convergence round 3 — **CONVERGED**. 2 contract amendments (no new contracts), 5 nitpicks. Protocol minimum satisfied; round 3 is terminal for passes 2 and 3._

## Changes from round 2

Round 3 read the full bodies of `skills/executing-plans/SKILL.md` (70 LOC), `skills/subagent-driven-development/SKILL.md` (278 LOC past cited ranges), both SDD fixtures (`tests/subagent-driven-dev/{go-fractals,svelte-todo}/`), `tests/subagent-driven-dev/run-test.sh`, `tests/claude-code/test-subagent-driven-development.sh`, and `tests/claude-code/test-subagent-driven-development-integration.sh`. Round 3 adds two contract amendments (to BC-DRAFT-007 and BC-DRAFT-011) and declares both passes converged. No new domain entities discovered.

### BC-DRAFT-007 Amendment: Terminal final-code-reviewer pass

**Additional post-condition:** After ALL per-task two-stage reviews complete and before `finishing-a-development-branch`, SDD dispatches ONE MORE code-reviewer subagent over the entire implementation ("Dispatch final code reviewer subagent for entire implementation"). This is a third review checkpoint distinct from the per-task pair. Only after the final reviewer returns does the controller invoke `superpowers:finishing-a-development-branch`.
**Evidence:** `skills/subagent-driven-development/SKILL.md:63, 82-83, 195-197`
**Confidence:** HIGH

### BC-DRAFT-011 Amendment: Ordered BLOCKED remediation ladder

**Additional post-condition for BLOCKED status:** Controller MUST apply remediation in this order before escalating: (1) if context problem → provide more context, re-dispatch same model; (2) if needs more reasoning → re-dispatch with more capable model; (3) if task too large → break into smaller pieces; (4) if plan itself is wrong → escalate to human. Absolute rule: "Never ignore an escalation or force the same model to retry without changes" (`SKILL.md:118`). DONE_WITH_CONCERNS has a parallel decision split: correctness/scope concerns → fix before review; observational concerns (e.g., "this file is getting large") → note and proceed.
**Evidence:** `skills/subagent-driven-development/SKILL.md:102-118`
**Confidence:** HIGH

### Round 3 nitpicks (recorded, not promoted to contracts)

- **SDD fixture triple format:** Each SDD test under `tests/subagent-driven-dev/<name>/` consists of exactly three files: `design.md`, `plan.md`, `scaffold.sh`. Runner scaffolds via `scaffold.sh <outdir>/project`, then dispatches the literal prompt `"Execute this plan using superpowers:subagent-driven-development. The plan is at: <plan-path>"` via `claude -p --output-format stream-json --verbose --dangerously-skip-permissions` (`tests/subagent-driven-dev/run-test.sh:36-81`). Language-specific verification: go-fractals → `go test ./...`; svelte-todo → `npm test && npx playwright test` (`run-test.sh:102-105`).
- **executing-plans subagent-availability advisory:** `executing-plans/SKILL.md:14` instructs the agent to TELL the human partner that superpowers "works much better with access to subagents" and to prefer `subagent-driven-development` when available. Advisory nudge, not hard contract.
- **executing-plans terminal handoff:** `executing-plans/SKILL.md:32-38` mandates `finishing-a-development-branch` as the REQUIRED SUB-SKILL after all tasks — already covered by BC-DRAFT-027.
- **Integration test assertions** (`tests/claude-code/test-subagent-driven-development-integration.sh:189-267`): asserts session transcript contains `"name":"Skill".*"skill":"superpowers:subagent-driven-development"`, Task tool count ≥ 2, TodoWrite count ≥ 1, git commit count > 2, and NO extra math functions beyond spec (`divide`/`power`/`subtract`). Extra-feature check is graded WARN, not FAIL — tests spec-reviewer effectiveness without gating the build.
- **Unit test assertions** (`tests/claude-code/test-subagent-driven-development.sh:36, 87, 131`): nine assertions via `run_claude` including ordering (`spec.*compliance` BEFORE `code.*quality`), skeptical reviewer attitude (`not trust|skeptical|suspiciously`), negative assertion that subagents do NOT read files. These are compliance tests against already-captured normative contracts.

## Changes from round 1

Round 1 added 24 contracts (BC-DRAFT-010 through BC-DRAFT-033). Round 2 adds 13 new contracts (BC-DRAFT-034 through BC-DRAFT-047) covering: Plan Document required header/schema, No-Placeholders ban, inline plan self-review, writing-plans execution handoff, worktree directory-ignore + baseline-test + precedence, implementer file-growth escalation, implementer in-over-your-head STOP, code-review severity triage, skill-triggering test pass semantics, Anthropic-compliance PR rejection, deprecation-shim response contract. Expands round 1 BC-DRAFT-004 with TDD sub-gates (Verify RED / Verify GREEN).

## Changes from Phase A

Phase A captured 9 BC-DRAFTs. Round 1 added 24 new contracts, a tension-audit section, and expanded BC-DRAFT-005 / BC-DRAFT-006 into sub-contracts.

Superpowers does not have formal behavioral contract files. Its contracts are encoded in:

1. **Skill markdown** — Iron Laws, HARD-GATEs, checklists, red-flag tables
2. **Subagent prompt templates** — implementer, spec-reviewer, code-quality-reviewer
3. **Hook script** — bootstrap injection + legacy warning
4. **OpenCode plugin** — config-hook + first-user-message transform
5. **Tests** — prompts + assertion scripts validating skill trigger + follow-through
6. **(Round 2)** **Agent file** — `agents/code-reviewer.md` with six-part mandate and Critical/Important/Minor taxonomy
7. **(Round 2)** **Plan document schema** — `writing-plans/SKILL.md` encodes a machine-readable header directive naming the required execution skill
8. **(Round 2)** **Contributor guidelines (`CLAUDE.md`)** — governance contracts (no third-party deps, no Anthropic-compliance PRs, etc.)

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
**Error case:** Code written before test → delete it, start over; no "adapt while writing tests", no "keep as reference"
**Sub-contracts (round 2):**
- **Verify-RED gate** (`test-driven-development/SKILL.md:114-128`): after writing the test, agent MUST execute it and observe failure for the correct reason (feature missing, not typo, not pre-existing behavior). If the test passes immediately, it was testing existing behavior; fix the test. If it errors, fix the error until it fails correctly.
- **Verify-GREEN gate** (`SKILL.md:168-184`): after implementing, agent MUST re-run and observe (a) target test passes, (b) all other tests still pass, (c) output pristine — no warnings/errors. Any regression: fix immediately, not "later".
- **Exception protocol** (`SKILL.md:24-29`): only throwaway prototypes, generated code, and configuration files may skip TDD, and ONLY after asking the human partner.
- **Debugging integration** (`SKILL.md:351-355`): bugs MUST be reproduced via a failing test before any fix attempt. "Never fix bugs without a test."
**Evidence:** `skills/test-driven-development/SKILL.md:33-45, 114-128, 168-184, 24-29, 328-340, 351-355`
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
**Post:** Each task dispatched to fresh implementer subagent; on implementer DONE, spec compliance reviewer dispatched; ONLY after spec reviewer returns correct, code quality reviewer dispatched. Review loops until both return correct. All three subagents receive precisely crafted context, never parent history.
**Error case:** Starting code quality review before spec compliance correct = forbidden (`SKILL.md:247`)
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
**Post:** Reviewer reads actual code files, compares to requirements line-by-line, ignores implementer's claims. Default framing: "The implementer finished suspiciously quickly. Their report may be incomplete, inaccurate, or optimistic." Reviewer returns either a spec-compliant signal, OR a list of specific missing/extra items with file:line references.
**Evidence:** `skills/subagent-driven-development/spec-reviewer-prompt.md:21-37, 56-61`
**Confidence:** HIGH

### BC-DRAFT-013: Quality review MUST NOT start before spec review returns correct
**Pre:** SDD task has been reviewed by spec-reviewer
**Post:** Code quality reviewer is dispatched ONLY if spec reviewer returned a correct signal. If issues found, implementer fixes + spec re-review first. Quality reviewer additionally checks: one-responsibility-per-file, unit decomposability, plan-structure adherence, newly-created-large-files (not pre-existing file sizes).
**Evidence:** `skills/subagent-driven-development/SKILL.md:247`; `code-quality-reviewer-prompt.md:7, 20-24`
**Confidence:** HIGH

## Meta-Contracts from writing-skills (HIGH confidence)

### BC-DRAFT-014: No skill (new or edited) without a failing pressure test
**Pre:** Creating or editing any SKILL.md
**Post:** A pressure-test scenario was run with a subagent WITHOUT the skill, baseline rationalizations were captured verbatim, then the skill was written to address those specific rationalizations. **Pressure scenario MUST combine 3+ pressures drawn from {Time, Sunk cost, Authority, Economic, Exhaustion, Social, Pragmatic}** (`testing-skills-with-subagents.md:130-141`). Agent behavior must be documented word-for-word, not summarized.
**Evidence:** `skills/writing-skills/SKILL.md:374-393`; `skills/writing-skills/testing-skills-with-subagents.md:1-116, 130-141`
**Confidence:** HIGH

### BC-DRAFT-015: Skill descriptions MUST NOT summarize workflow
**Pre:** Authoring a skill's YAML description field
**Post:** Description contains ONLY triggering conditions ("Use when..."). Does NOT summarize what the skill does or its workflow.
**Evidence:** `skills/writing-skills/SKILL.md:150-172`
**Confidence:** HIGH

### BC-DRAFT-016: Discipline skills MUST include rationalization table + red flags list
**Pre:** Authoring a skill that enforces discipline (Iron Law-class)
**Post:** Skill contains (a) rationalization table, (b) red flags list, (c) explicit loophole closures, (d) "violating the letter = violating the spirit" clause. **Round 2 addendum:** discipline skills MUST use Authority + Commitment + Social Proof language and MUST NOT use Liking (creates sycophancy) or heavy Reciprocity (feels manipulative) (`persuasion-principles.md:126-133`).
**Evidence:** `skills/writing-skills/SKILL.md:459-531`; `persuasion-principles.md:126-165`
**Confidence:** HIGH

### BC-DRAFT-017: Skills MUST NOT use `@` force-loading references to other skills
**Pre:** Cross-referencing another skill from within a skill
**Post:** Reference by name with explicit requirement marker; no `@skills/...` which force-loads 200k+ context
**Evidence:** `skills/writing-skills/SKILL.md:278-288`
**Confidence:** HIGH

## Brainstorming Sub-Contracts (HIGH confidence)

### BC-DRAFT-018: Brainstorming terminal state is writing-plans exclusively
**Pre:** Brainstorming checklist complete + user approved spec
**Post:** writing-plans is invoked. NO other skill is invoked as the next step.
**Evidence:** `skills/brainstorming/SKILL.md:66`

### BC-DRAFT-019: Visual Companion offer MUST be its own message
**Evidence:** `skills/brainstorming/SKILL.md:152-154`

### BC-DRAFT-020: Per-question visual-vs-terminal decision
**Post:** Agent decides per-question: browser if "user would understand better by SEEING than reading"; terminal otherwise. Round 2: content written to visual-companion is a **fragment by default**, full HTML documents only when frame-template override is needed.
**Evidence:** `skills/brainstorming/SKILL.md:156-162`; `brainstorming/visual-companion.md:27-31`

## Receiving-Code-Review Contracts (HIGH confidence)

### BC-DRAFT-021: Unclear review items halt ALL implementation
**Evidence:** `skills/receiving-code-review/SKILL.md:42-48`

### BC-DRAFT-022: Forbidden gratitude class in review responses
**Evidence:** `skills/receiving-code-review/SKILL.md:28-33, 132-148`

### BC-DRAFT-023: YAGNI check before implementing "proper" features
**Evidence:** `skills/receiving-code-review/SKILL.md:88-98`

## Finishing-a-Development-Branch Contracts (HIGH confidence)

### BC-DRAFT-024: Tests MUST pass before presenting completion options
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:18-38`

### BC-DRAFT-025: Exactly 4 options, no explanation added
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:50-64`

### BC-DRAFT-026: Discard requires typed "discard" confirmation
**Evidence:** `skills/finishing-a-development-branch/SKILL.md:116-124`

### BC-DRAFT-027: executing-plans MUST invoke finishing-a-development-branch as terminal step
**Evidence:** `skills/executing-plans/SKILL.md:34-38`

## SDD Safety Contracts (HIGH confidence)

### BC-DRAFT-028: SDD MUST NOT start on main/master without explicit user consent
**Evidence:** `skills/subagent-driven-development/SKILL.md:237, 268`

### BC-DRAFT-029: SDD MUST NOT dispatch multiple implementer subagents in parallel
**Evidence:** `skills/subagent-driven-development/SKILL.md:240`

### BC-DRAFT-030: Controller provides full task text inline; subagent MUST NOT read plan file
**Evidence:** `skills/subagent-driven-development/SKILL.md:241`; `implementer-prompt.md:9-18`

## Bootstrap & Platform Contracts (HIGH confidence)

### BC-DRAFT-031: Bootstrap injection JSON shape is platform-conditional
**Post:** Hook emits exactly ONE of three JSON shapes based on env-var detection:
- `CURSOR_PLUGIN_ROOT` set → `{"additional_context": "..."}`
- `CLAUDE_PLUGIN_ROOT` set && `COPILOT_CLI` unset → `{"hookSpecificOutput": {"hookEventName": "SessionStart", "additionalContext": "..."}}`
- otherwise → `{"additionalContext": "..."}` (SDK standard, Copilot CLI)
**Evidence:** `hooks/session-start:40-55`

### BC-DRAFT-032: Legacy skills dir triggers first-reply warning
**Evidence:** `hooks/session-start:12-15`

### BC-DRAFT-033: OpenCode bootstrap injects into first user message, idempotently
**Evidence:** `.opencode/plugins/superpowers.js:84-110`

## Round 2 — Plan Document Contracts (HIGH confidence)

### BC-DRAFT-034: Plan header is MANDATORY and declares the required execution sub-skill
**Pre:** Writing a plan file via writing-plans
**Post:** The plan file begins with a header block containing (a) title, (b) a `> **For agentic workers:**` directive naming `REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans`, (c) `**Goal:**`, (d) `**Architecture:**`, (e) `**Tech Stack:**`. Any later execution agent reads this header to determine which skill should run the plan.
**Evidence:** `skills/writing-plans/SKILL.md:47-61`
**Confidence:** HIGH

### BC-DRAFT-035: No-placeholders contract in plan documents
**Pre:** Plan being written
**Post:** Plan contains NO tokens from the forbidden set: "TBD", "TODO", "implement later", "fill in details", "Add appropriate error handling", "add validation", "handle edge cases", "Write tests for the above" without code, "Similar to Task N" without inlined code, steps describing WHAT without HOW (no code block), references to types/functions/methods not defined in any task.
**Evidence:** `skills/writing-plans/SKILL.md:106-120`
**Confidence:** HIGH

### BC-DRAFT-036: Plan self-review is an inline 3-check, NOT a subagent dispatch
**Pre:** Plan draft complete
**Post:** Author runs the 3-check pass themselves: (1) spec-coverage, (2) placeholder-scan, (3) type-consistency.
**Distinguishing rule:** Explicitly NOT the SDD two-stage review. "This is a checklist you run yourself — not a subagent dispatch." (`writing-plans/SKILL.md:124`)
**Evidence:** `skills/writing-plans/SKILL.md:122-132`
**Confidence:** HIGH

### BC-DRAFT-037: Writing-plans MUST end with explicit execution handoff offering exactly two options
**Pre:** Plan saved
**Post:** Agent presents: (1) Subagent-Driven (recommended) — maps to `REQUIRED SUB-SKILL: superpowers:subagent-driven-development`; (2) Inline Execution — maps to `REQUIRED SUB-SKILL: superpowers:executing-plans`. No other options. No default. Agent waits for choice.
**Evidence:** `skills/writing-plans/SKILL.md:134-152`
**Confidence:** HIGH

## Round 2 — Worktree Contracts (HIGH confidence)

### BC-DRAFT-038: Project-local worktree directory MUST be gitignore-covered before creation
**Pre:** About to create a worktree under `.worktrees/` or `worktrees/`
**Post:** `git check-ignore` confirms the directory is ignored. If NOT, agent adds the line to `.gitignore`, commits, then proceeds. Global directory `~/.config/superpowers/worktrees/<project>/` is exempt.
**Rationale:** Prevents accidentally committing worktree contents. Codifies Jesse's "Fix broken things immediately" rule.
**Evidence:** `skills/using-git-worktrees/SKILL.md:52-70`
**Confidence:** HIGH

### BC-DRAFT-039: Clean-baseline test gate on new worktree
**Pre:** Worktree created and project setup run
**Post:** Full test suite runs in the fresh worktree; if all pass, agent reports "ready". If any fail, agent STOPS and asks human partner whether to proceed or investigate.
**Evidence:** `skills/using-git-worktrees/SKILL.md:120-134, 168-172`
**Confidence:** HIGH

### BC-DRAFT-040: Worktree directory selection follows deterministic precedence
**Post:** (1) existing `.worktrees/` or `worktrees/` (tie → `.worktrees/`); (2) else grep `CLAUDE.md` for a `worktree.*director` preference; (3) else ask human partner.
**Evidence:** `skills/using-git-worktrees/SKILL.md:17-49, 144-156`
**Confidence:** HIGH

## Round 2 — Implementer Escalation Contracts (HIGH confidence)

### BC-DRAFT-041: Implementer MUST NOT unilaterally split files beyond plan intent
**Pre:** Implementer working on a task; a file being created grows beyond the plan's intended scope
**Post:** Implementer does NOT split the file on its own. It reports status DONE_WITH_CONCERNS and flags the file-size concern.
**Evidence:** `skills/subagent-driven-development/implementer-prompt.md:45-56`
**Confidence:** HIGH

### BC-DRAFT-042: Implementer MUST stop and escalate when in over its head
**Pre:** Implementer encounters ANY of: architectural decisions with multiple valid approaches, code understanding needs beyond provided context, uncertainty about correctness, restructuring not anticipated by the plan, repeated file reads without progress
**Post:** Implementer STOPS. Reports status BLOCKED or NEEDS_CONTEXT with (a) specifically what it is stuck on, (b) what it tried, (c) what kind of help it needs.
**Evidence:** `skills/subagent-driven-development/implementer-prompt.md:58-72`
**Confidence:** HIGH

## Round 2 — Code Review Dispatch Contracts (HIGH confidence)

### BC-DRAFT-043: Code-review feedback triage follows 3-tier severity taxonomy
**Pre:** Code-reviewer subagent/agent returns findings
**Post:** Agent categorizes each finding as Critical / Important / Minor (Suggestions) and acts accordingly: Critical → fix immediately; Important → fix before next task; Minor → note for later. Push back with technical reasoning if reviewer is wrong.
**Evidence:** `skills/requesting-code-review/SKILL.md:43-48, 92-104`; `agents/code-reviewer.md:37-40`
**Confidence:** HIGH

### BC-DRAFT-044: Requesting-code-review passes a precisely crafted context, never session history
**Pre:** About to dispatch code-reviewer
**Post:** Agent computes `BASE_SHA` and `HEAD_SHA` from git so the reviewer sees exactly the diff in scope. Dispatches via Task tool using the `code-reviewer.md` template filled with `{WHAT_WAS_IMPLEMENTED}`, `{PLAN_OR_REQUIREMENTS}`, `{BASE_SHA}`, `{HEAD_SHA}`, `{DESCRIPTION}`. Mandatory invocation points: after each SDD task, after completing a major feature, before merge to main.
**Evidence:** `skills/requesting-code-review/SKILL.md:8, 14-17, 26-42`
**Confidence:** HIGH

## Round 2 — Test Assertion Contracts (HIGH confidence)

### BC-DRAFT-045: Skill-triggering test pass condition is a stream-json tool-invocation match
**Pre:** Running `tests/skill-triggering/run-test.sh <skill-name> <prompt-file>` against a prompt that does NOT mention the skill by name
**Post:** Test PASSES if and only if the recorded stream-json log contains BOTH `"name":"Skill"` AND a `"skill":"(namespace:)?<skill-name>"` tool invocation. The assertion uses `grep -qE` against a regex `'"skill":"([^"]*:)?<SKILL_NAME>"'`. Timeout 300s, max-turns default 3.
**Evidence:** `tests/skill-triggering/run-test.sh:57-68`
**Confidence:** HIGH

## Round 2 — Governance / Contributor Contracts (HIGH confidence)

### BC-DRAFT-046: Anthropic-compliance skill PRs are rejected without eval evidence
**Pre:** PR modifies skill content to conform to Anthropic's published skill-authoring guidance
**Post:** PR is closed/rejected unless it includes extensive eval evidence showing the change improves outcomes. This applies even if `anthropic-best-practices.md` appears to support the change — that file is reference material, not authority.
**Evidence:** `CLAUDE.md:35-37, 67-74`
**Confidence:** HIGH

### BC-DRAFT-047: Deprecation-shim commands MUST respond by naming the replacement skill
**Pre:** User invokes `/brainstorm`, `/write-plan`, or `/execute-plan`
**Post:** Agent responds by telling the human partner the command is deprecated and should ask for the corresponding skill instead. Agent does NOT execute any work under the shim.
**Evidence:** `commands/brainstorm.md:1-6`; `commands/write-plan.md:1-6`; `commands/execute-plan.md:1-6`
**Confidence:** HIGH

## Tension Audit

**T1: "Skills are mandatory" (BC-002) vs "User instructions override" (BC-008).** Resolution: priority order user > skills > default. No conflict.

**T2: SDD sequential dispatch (BC-029) vs parallel-agents skill.** Resolution: scope-distinguished (implementation vs investigation). No conflict.

**T3: Verify before completion vs social pressure.** Resolution: pressure is the adversary the skill addresses. No conflict.

**T4: Push back when wrong (BC-022) vs politeness.** Resolution: skill forbids performative politeness. Technical correctness > social comfort. The tension IS the contract.

**T5: Implementer self-review (BC-010) vs spec reviewer "don't trust the report" (BC-012).** Resolution: defense in depth. Intentional adversarial layer. No conflict.

**T6 (round 2): writing-plans inline Self-Review (BC-036) vs SDD two-stage subagent review (BC-007, BC-012, BC-013).** Resolution: different phases. Plan self-review is authorial (BEFORE implementation). SDD two-stage review is post-implementation. The plan explicitly says "This is a checklist you run yourself — not a subagent dispatch." No conflict.

**T7 (round 2): `anthropic-best-practices.md` reference file vs BC-DRAFT-046 rejection of compliance PRs.** Resolution: the Anthropic guidance file is reference material retained for contrast, not authority. CLAUDE.md:35-37 explicitly states the project's philosophy "differs" and that compliance-restructuring PRs require eval evidence. Deliberate tension, not contradiction.

**T8 (round 2): Persuasion-principles "use Authority + Commitment" vs Anthropic best-practices "concise, avoid over-explaining".** Resolution: different axes. Persuasion governs imperative tone; Anthropic guidance governs token budget. Skills can be both concise AND authoritative. Writing-skills wins ties per CLAUDE.md:35-37.

## Gaps

- No machine-readable contract index. Contracts are prose inside skills, prompt files, hook scripts, a JS plugin, and a plan-document schema.
- Compliance is ultimately probabilistic; mitigation is adversarial pressure testing, not type-system enforcement.
- `executing-plans/SKILL.md` full body not yet extracted into formal contracts (batch size, checkpoint semantics).
- `skills/subagent-driven-development/SKILL.md` body past cited line ranges not yet fully extracted.
- `tests/subagent-driven-dev/{go-fractals,svelte-todo}/` scaffolding-format contract not yet extracted.
