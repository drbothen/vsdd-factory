# Pass 3: Behavioral Contracts

Superpowers does not have formal behavioral contract files (no BC-S.SS.NNN structure). Its contracts are encoded in three places:

1. **Skill markdown** — Iron Laws, HARD-GATEs, checklists
2. **Hook script** — what gets injected into sessions
3. **Tests** — prompts + assertion scripts validating skills trigger and are followed

## Contracts from Skills (HIGH confidence; directly stated)

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
**Post:** A design has been presented and explicitly approved by the human partner before any impl skill is invoked or code written
**Evidence:** `skills/brainstorming/SKILL.md:12-14`
**Confidence:** HIGH

### BC-DRAFT-004: No production code without a failing test (TDD Iron Law)
**Pre:** Implementing feature/bugfix/refactor
**Post:** A test existed and was observed to fail before corresponding production code was written
**Error case:** Code written before test → delete it, start over; no "adapt while writing tests"
**Evidence:** `skills/test-driven-development/SKILL.md:33-45`
**Confidence:** HIGH

### BC-DRAFT-005: No fixes without root cause (debugging Iron Law)
**Pre:** Bug/test-failure/unexpected-behavior observed
**Post:** Phase 1 root cause investigation complete before any fix proposed
**Evidence:** `skills/systematic-debugging/SKILL.md:18-21`
**Confidence:** HIGH

### BC-DRAFT-006: No completion claims without fresh verification
**Pre:** Agent about to claim "done", "passing", "fixed"
**Post:** Agent ran the verification command in the current message and read full output and exit code
**Error case:** Skipping any gate step = "lying, not verifying"
**Evidence:** `skills/verification-before-completion/SKILL.md:18-38`
**Confidence:** HIGH

### BC-DRAFT-007: Per-task fresh subagent + two-stage review
**Pre:** Executing a plan with independent tasks, subagent-driven-development active
**Post:** Each task dispatched to fresh subagent; spec reviewer and code quality reviewer subagents dispatched after; all three subagents receive precisely crafted context, never parent history
**Evidence:** `skills/subagent-driven-development/SKILL.md:6-13, 42-59`
**Confidence:** HIGH

### BC-DRAFT-008: User instructions override skills
**Pre:** Conflict between a skill's rule and CLAUDE.md/GEMINI.md/AGENTS.md/direct request
**Post:** User instructions win
**Evidence:** `skills/using-superpowers/SKILL.md:19-26`
**Confidence:** HIGH

### BC-DRAFT-009: Subagents skip the bootstrap skill
**Pre:** Agent invoked as a subagent for a specific task
**Post:** using-superpowers is skipped via `<SUBAGENT-STOP>` tag
**Evidence:** `skills/using-superpowers/SKILL.md:6-8`
**Confidence:** HIGH

## Contracts from Tests (HIGH confidence; behavioral assertions)

- `tests/skill-triggering/prompts/` — implicit phrasing (e.g. "help me debug this issue") should auto-trigger the matching skill. Assertions in `tests/skill-triggering/run-test.sh`.
- `tests/explicit-skill-requests/prompts/` — explicit phrasing ("please use brainstorming") must trigger the skill even under abbreviation ("SDD means", `i-know-what-sdd-means.txt`).
- `tests/claude-code/test-subagent-driven-development.sh` + `-integration.sh` — SDD skill dispatches correct subagent sequence.
- `tests/subagent-driven-dev/{go-fractals,svelte-todo}/` — end-to-end: scaffolded project + pre-written plan.md/design.md → agent should execute plan using SDD.
- `tests/opencode/test-priority.sh` — validates skill priority ordering (process skills first).
- `tests/brainstorm-server/*.test.js` — unit tests for the brainstorm visual-companion websocket server.

## Contracts from Contributor Guidelines (`CLAUDE.md`)

`CLAUDE.md` is for PR contributors, not runtime behavior. It encodes meta-contracts about skill modification:

- Skill content is carefully tuned; changes require eval evidence (`CLAUDE.md:68-75`)
- "human partner" terminology is deliberate and not interchangeable (`CLAUDE.md:77-78`)
- Third-party dependencies forbidden (`CLAUDE.md:31-33`)
- Anthropic skill-authoring "compliance" PRs explicitly rejected (`CLAUDE.md:35-37`)

## Gaps

- No machine-readable contract index. Contracts are prose inside skills.
- Compliance is ultimately probabilistic (model adherence to instructions); the mitigation is adversarial pressure testing, not type system enforcement.
