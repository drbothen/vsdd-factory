# Takeaways: claude-code + superpowers → vsdd-factory

_Synthesis of `.factory/semport/claude-code/pass-6-synthesis.md` (~900 lines, anthropics/claude-code reference plugins) and `.factory/semport/superpowers/pass-6-synthesis.md` (~1050 lines, obra/superpowers @ 917e5f5) against the current state of `plugins/vsdd-factory/`._

---

## Executive Summary

1. **`plugins/vsdd-factory/plugin.json` does not exist.** The marketplace points at `./plugins/vsdd-factory` but the loadable manifest is missing. Per `claude-code` BC-DRAFT-M01: absent `plugin.json` = plugin not loadable. **This is the single highest-priority bug.**
2. **~25 of 34 agents have placeholder one-line descriptions** (`description: VSDD factory agent: <name>`) and no `model` / `color` / `tools` fields. Per claude-code BC-DRAFT-A05/A06, `color` is **required** and `description` should be 10–5000 chars with `<example>` blocks. These agents are silently broken on Claude Code's strict validators and useless for description-match dispatch.
3. **`/deliver-story` runs RED→GREEN→REFACTOR in a single context.** Superpowers' subagent-driven-development pattern (per-task fresh implementer + spec-reviewer + quality-reviewer + terminal final-reviewer, with the four-status-code protocol and BLOCKED remediation ladder) is the highest-leverage architectural adoption — call it the biggest single behavior win available.
4. **No skill ships a Red Flags table, an Iron Law, or an "Announce at start" line.** Superpowers' empirical thesis (Meincke 2025, N=28k, compliance 33%→72%) is that this rhetorical scaffolding is what makes skills actually shape behavior. Cheap to add, large effect.
5. **Hooks are exit-code blockers only.** Should be upgraded to `permissionDecision` + `updatedInput` envelopes (BC-DRAFT-H11), and the LLM-evaluated `type:"prompt"` form (BC-DRAFT-H08) should be used for the 3–5 hooks that currently shell out to "ask Claude to check X."

---

## What vsdd-factory already does well

- **Hooks use `${CLAUDE_PLUGIN_ROOT}`** uniformly (`hooks/hooks.json:7-45`). NFR-V06 compliance — no portability bugs.
- **Active enforcement hooks** (protect-vp, protect-bc, red-gate, brownfield-discipline, regression-gate, verify-git-push) — superpowers has zero blocking hooks. This is a real differentiator and aligns with SOUL.md #2 ("Make the wrong thing impossible").
- **Information-asymmetry agents** (`adversary.md`, `holdout-evaluator.md`) are well-formed: explicit `tools:` whitelist, `model: sonnet` for diversity, and stated "cannot see X" sections. These are the template the rest of the agent fleet should follow.
- **Specs-as-product hierarchy** (L1–L4, BCs, VPs, ARCH) is something superpowers explicitly lacks and would benefit from. Don't regress this.
- **Brownfield-ingest convergence protocol works**: the strict-binary novelty discipline caught three round-1 hallucinations during this very ingest (Pressure Taxonomy, Persuasion Matrix, No-Placeholders list). Documented in `.factory/semport/superpowers/pass-6-synthesis.md` §7.3 and §10.2.
- **Per-project semport subfolders** (`.factory/semport/<project>/`) — clean, scales.

---

## P0 — Correctness gaps (must fix)

### P0-1. Missing `plugin.json` on the loadable plugin

- **Today:** `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/plugin.json` does not exist (verified). Marketplace at `.claude-plugin/marketplace.json:9-13` points at `./plugins/vsdd-factory` but the runtime will refuse to load it.
- **Reference:** claude-code `plugin-structure/SKILL.md:22-37`, `manifest-reference.md:5-9`. BC-DRAFT-M01: "Runtime loads plugin only if `./.claude-plugin/plugin.json` exists exactly." `manifest-reference.md:365-371`: custom paths *supplement* defaults.
- **Gap:** Plugin is currently unloadable in the strictest read of the spec. If it loads today, that's harness leniency, not contract.
- **Action:** Create `plugins/vsdd-factory/.claude-plugin/plugin.json` with at minimum:
  ```json
  {"name":"vsdd-factory","version":"0.1.0","description":"...","author":{"name":"drbothen"}}
  ```
  Use defaults for components paths (don't override `commands`/`agents`/`hooks` unless necessary). Note: there's no `commands/` directory either — see P0-3.

### P0-2. Bulk agent frontmatter is invalid against claude-code's strict schema

- **Today:** Of 34 agent files, ~25 have placeholder descriptions like `description: VSDD factory agent: business-analyst` and **no `model:` or `color:` fields**. Sampled offenders include `architect.md`, `business-analyst.md`, `code-reviewer.md`, `data-engineer.md`, `devops-engineer.md`, `dtu-validator.md`, `dx-engineer.md`, `formal-verifier.md`, `github-ops.md`, `pr-manager.md`, `pr-reviewer.md`, `product-owner.md`, `security-reviewer.md`, `spec-reviewer.md`, `spec-steward.md`, `state-manager.md`, `story-writer.md`, `technical-writer.md`, `test-writer.md`. Also `implementer.md` cuts off mid-sentence after `description:`.
- **Reference:** claude-code BC-DRAFT-A05/A06 (`agent-development/SKILL.md:64-80,128-141,264-273,351-357`):
  - `name`: 3–50 chars, `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`
  - `description`: **10–5000 chars**, should embed 2–3 `<example>` dialogue blocks
  - `model`: required enum `{inherit,sonnet,opus,haiku}`
  - `color`: **REQUIRED** enum `{blue,cyan,green,yellow,magenta,red}`
  - System prompt body: 20–10000 chars
- **Gap:** Description-match dispatch can't work on a 30-character one-liner. Strict validators reject. Some files (`implementer.md`) have a *truncated* description line, which is a parse hazard.
- **Action:**
  1. Run an audit script over `agents/*.md` flagging files where description < 100 chars OR `color` missing OR `model` missing.
  2. For each placeholder, write a real third-person `description:` enumerating trigger phrases plus 2 `<example>` blocks. Use `adversary.md` and `holdout-evaluator.md` as templates.
  3. Add `model:` and `color:` to every agent. Default to `model: sonnet`, varied `color:`.
  4. Fix the truncated `implementer.md` description line.
  5. Add a `scripts/validate-agent-frontmatter.sh` lint and wire it into a check hook.
- **Files to edit:** ~25 agent files in `plugins/vsdd-factory/agents/`. See appendix.

### P0-3. No `commands/` directory

- **Today:** `plugins/vsdd-factory/` has `agents/`, `hooks/`, `skills/`, but **no `commands/`**. The CLAUDE.md project doc references slash commands extensively (`/factory-health`, `/brownfield-ingest`, `/deliver-story`, `/wave-gate`, etc.), but they appear to be defined as skills, not commands.
- **Reference:** claude-code BC-DRAFT-C01/C07 — slash commands auto-discover from `commands/*.md`. Skills auto-activate by description match; commands fire on `/slash-command` invocation. They are **disjoint dispatch paths** (claude-code §3.3 "Critical disjointness").
- **Gap:** Either (a) the CLAUDE.md user-facing slash commands don't exist as commands and the docs are aspirational, or (b) the harness is treating skills as commands which is convention drift.
- **Action:** Decide one model and apply consistently:
  - **Option A** — Create `plugins/vsdd-factory/commands/` and add a thin command file per public entry point that invokes the matching skill. Recommended; this matches claude-code Pattern A.
  - **Option B** — Document explicitly that vsdd-factory uses skills-only dispatch and the `/foo` references in CLAUDE.md are skill activations, not commands. Update CLAUDE.md to say "skill" not slash.

### P0-4. `/deliver-story` is single-context — adopt SDD per-task fresh subagent pattern

- **Today:** `skills/deliver-story/SKILL.md` runs the entire RED/GREEN/REFACTOR loop in one agent context across an entire story.
- **Reference:** superpowers `subagent-driven-development/SKILL.md:6-13,42-85,247` and BC-DRAFT-007/010/011/012/013, with terminal final-reviewer (round-3 amendment, BC-007) and **BLOCKED remediation ladder** (round-3 amendment, BC-011): context → more capable model → split → escalate. Absolute rule (`SDD/SKILL.md:118`): "Never ignore an escalation or force the same model to retry without changes."
- **Gap:** Context pollution across tasks; rationalizations inherited; no per-task adversarial pass; no terminal review gate.
- **Action:**
  1. Restructure `deliver-story` as controller that dispatches per-task: `Task(implementer)` → `Task(spec-reviewer)` → `Task(quality-reviewer)`.
  2. Define three prompt templates in `templates/` (model after superpowers `implementer-prompt.md`, `spec-reviewer-prompt.md`, `code-quality-reviewer-prompt.md`).
  3. Implement four status codes: `DONE | DONE_WITH_CONCERNS | BLOCKED | NEEDS_CONTEXT`.
  4. Implement BLOCKED remediation ladder, verbatim absolute rule.
  5. Add **terminal final-reviewer** pass over the whole story before PR creation.
  6. Add **File-Growth Escalation rule**: implementer must report `DONE_WITH_CONCERNS` rather than unilaterally split files.
  7. Add **In-Over-Your-Head contract**: 5 named STOP triggers.
- **Files to edit:** `skills/deliver-story/SKILL.md`, `agents/implementer.md` (also needs P0-2 fix), new `agents/spec-reviewer.md` content (currently a stub), `templates/`.

### P0-5. `implementer.md` description is truncated mid-sentence

- **Today:** Frontmatter `description: Strict TDD implementation agent. Picks next failing test, writes minimum code` (no closing). Body restates content. This is a parse risk and was caught in the agent sample.
- **Action:** Fix in same pass as P0-2.

---

## P1 — High-ROI adoptions

### P1-1. Iron Law + Red Flags + "Announce at start" in every discipline skill

- **Today:** No skill in `plugins/vsdd-factory/skills/` carries an Iron Law line, a Red Flags rationalization table, or an "Announce at start" directive.
- **Reference:** Superpowers `test-driven-development/SKILL.md:33` (Iron Law canonical form: `> NO <verb> <scope> WITHOUT <prerequisite> FIRST`), `writing-skills/SKILL.md:374-393` (Red Flags pattern, must come *after* main process — load-bearing ordering), §6.3 of pass-6 (Skill Document 10-section structure). Empirical anchor: Meincke et al. 2025, N=28,000, compliance 33% → 72% (`persuasion-principles.md:7`).
- **Gap:** vsdd-factory skills are descriptive prose. They don't apply rhetorical pressure. Subagents have nothing to hold them to the line.
- **Action:**
  1. For every Rigid skill (deliver-story, brownfield-ingest, adversarial-review, wave-gate, convergence-check, red-gate, formal-verify), add an Iron Law line in canonical form.
  2. For every skill, add `**Announce at start:** "I'm using the <skill> skill to <purpose>."`
  3. Add `## Red Flags` table after the main process, 8–12 rows. Format: `| Thought | Reality |`.
  4. Mark Red Flags tables as protected — changes require eval evidence (mirror superpowers `CLAUDE.md:67-74` governance NFR).

### P1-2. Persuasion Principles Matrix audit (verbatim, 7 principles)

- **Today:** Skills written without persuasion-principle awareness; no audit.
- **Reference:** Superpowers `persuasion-principles.md:11-133`. The corrected 7 principles + combination matrix (verbatim from `:128-133`):

  | Skill Type | Use | Avoid |
  |---|---|---|
  | Discipline-enforcing | Authority + Commitment + Social Proof | Liking, Reciprocity |
  | Guidance/technique | Moderate Authority + Unity | Heavy authority |
  | Collaborative | Unity + Commitment | Authority, Liking |
  | Reference | Clarity only | All persuasion |

  Liking is **forbidden** for compliance enforcement (sycophancy risk). Reciprocity is "avoid almost always". Scarcity is first-class permitted (round-2 correction).

- **Action:** Add `.claude/rules/persuasion-principles.md` with the matrix verbatim. Audit every skill for Liking/Reciprocity smells. Adopt ethical test (`:165`): _"Would this technique serve the user's genuine interests if they fully understood it?"_

### P1-3. Pressure Taxonomy + skill pressure-testing methodology

- **Today:** No pressure testing; skills are hand-designed.
- **Reference:** Superpowers `testing-skills-with-subagents.md:128-138` — canonical seven (verbatim):

  | Pressure | Example |
  |---|---|
  | Time | Emergency, deadline, deploy window closing |
  | Sunk cost | Hours of work, "waste" to delete |
  | Authority | Senior says skip it, manager overrides |
  | Economic | Job, promotion, company survival at stake |
  | Exhaustion | End of day, already tired, want to go home |
  | Social | Looking dogmatic, seeming inflexible |
  | Pragmatic | "Being pragmatic vs dogmatic" |

  Compounding rule (`:140`, verbatim): _"Best tests combine 3+ pressures."_ The strongest combo is 3+ pressures from this list.

- **Special tension with SOUL.md #1 ("Pragmatism Over Purity"):** Superpowers treats "pragmatic" as an attack vector against discipline; vsdd-factory enshrines pragmatism as the governing meta-principle. **You must distinguish principled pragmatism (ROI thinking) from pragmatism-as-rationalization (skipping discipline because it's hard).** Add this to SOUL.md as an explicit footnote on Principle #1.
- **Action:** Add `skills/pressure-test/SKILL.md` (or extend `writing-skills`-equivalent). For each new or edited Rigid skill, run baseline + 3-pressure adversarial test against a fresh subagent before merging.

### P1-4. Adopt CSO rule (WHEN-not-WHAT) for skill descriptions

- **Today:** Most skills describe what they do, not when to use them. Examples: deliver-story description starts "Execute per-story TDD delivery — create worktree…" — workflow summary, not trigger.
- **Reference:** Superpowers `writing-skills/SKILL.md:99-100,154-156` (verbatim case study): _"Testing revealed that when a description summarizes the skill's workflow, Claude may follow the description instead of reading the full skill content. A description saying 'code review between tasks' caused Claude to do ONE review, even though the skill's flowchart clearly showed TWO reviews […]. When the description was changed to just 'Use when executing implementation plans with independent tasks' (no workflow summary), Claude correctly read the flowchart and followed the two-stage review process."_
- **Empirical:** This is a behavioral correctness bug, not a style preference.
- **Action:** Audit every skill description. Rewrite as `Use when <triggering condition>...`. Third person. ≤1024 chars. Forbid first-person ("I can help...").

### P1-5. Plugin Settings overlay for runtime toggles

- **Today:** No runtime-adjustable surface. Every config change requires session restart.
- **Reference:** claude-code BC-DRAFT-PS01/PS02 — `.claude/<plugin-name>.local.md` (YAML frontmatter + markdown body), gitignored, `chmod 600`. **Only** runtime-mutable plugin surface. Two adopters in the corpus.
- **Action:**
  1. Create `.claude/vsdd-factory.local.md` template with `activation_enabled`, `current_phase`, `current_wave`, `paused_hooks[]`, `dry_run`, `log_level`.
  2. Update `.gitignore` to include `.claude/*.local.md` AND `.claude/*.local.json`.
  3. Teach `factory-health`, `state-update`, hooks to read it at runtime.
  4. **Do not conflate config and state in one file** (P3-3 below).

### P1-6. Upgrade hooks to `permissionDecision` envelopes + `type:"prompt"` for LLM checks

- **Today:** `hooks/hooks.json` uses `type:"command"` everywhere with exit-code-2 blocking. `protect-vp.sh`, `protect-bc.sh`, `red-gate.sh` are deny-only.
- **Reference:** claude-code BC-DRAFT-H11 (`hook-development/SKILL.md:144-153`): PreToolUse can emit `{hookSpecificOutput:{permissionDecision:"allow"|"deny"|"ask",updatedInput:{...}},systemMessage}` — can **rewrite tool input**. BC-DRAFT-H08 (`:22-34`): `type:"prompt"` is LLM-evaluated, supported on `Stop|SubagentStop|UserPromptSubmit|PreToolUse`.
- **Gap:** vsdd-factory's hooks miss the rewrite path entirely. A green-VP edit could be auto-rerouted to "create supersession VP" instead of just denied.
- **Action:**
  1. Upgrade `protect-vp.sh` and `protect-bc.sh` to emit JSON envelopes with `permissionDecision`. Add `"ask"` for borderline cases.
  2. Add a "supersede, don't mutate" rewrite for green-VP edits: instead of denying, rewrite the call to create VP-NNN+1 superseding.
  3. Identify hooks that are essentially "ask Claude to check X" and convert to `type:"prompt"`: candidates are `session-learning.sh` (Stop), `handoff-validator.sh` (SubagentStop). Both are well-suited.

### P1-7. Validator-inspired hook lint (NFR-V01..V10)

- **Today:** No hook-schema lint.
- **Reference:** claude-code `validate-hook-schema.sh` enforces event enum, type enum, prompt-event scoping, `${CLAUDE_PLUGIN_ROOT}` portability, timeout bounds [5,600].
- **Action:** Port the script to `scripts/validate-hook-schema.sh`. Wire into pre-commit/CI. **Decide TENSION-03**: vsdd-factory should require `matcher` (it does today — keep that explicit).

### P1-8. Verification-before-completion as a dedicated skill / 5-step gate

- **Today:** No explicit verification gate.
- **Reference:** Superpowers `verification-before-completion/SKILL.md:18-38` — IDENTIFY → RUN → READ → VERIFY → CLAIM. _"Skipping any = lying, not verifying."_
- **Action:** Add `skills/verification-before-completion/SKILL.md` modeled on superpowers. Reference from `deliver-story`, `wave-gate`, `convergence-check` as a sub-skill prior to any "done/passed" claim.

---

## P2 — Worth considering

- **P2-1. Story `REQUIRED SKILL:` self-describing header** (superpowers `writing-plans/SKILL.md:47-61`). Every STORY-NNN.md declares which delivery skill it requires. Lets `deliver-story` cross-check before dispatch.
- **P2-2. Story No-Placeholders invariant** with named "story failures" class. Verbatim forbidden tokens from `writing-plans/SKILL.md:106-114`: `"TBD", "TODO", "implement later", "fill in details"`, plus the broader invariant: _"every step must contain the actual content an engineer needs"_. Vague-verb placeholders ("add appropriate error handling", "handle edge cases") are also failures. Wire into `story-completeness.md` checklist.
- **P2-3. SessionStart injection hook** that primes every session with "read `.factory/STATE.md` first" and the 1% rule (mirrors superpowers `hooks/session-start`). Passive loader complements existing active guards.
- **P2-4. `<EXTREMELY_IMPORTANT>` / `<HARD-GATE>` / `<SUBAGENT-STOP>` tag conventions** in skill bodies for non-skippable rules.
- **P2-5. Title Case skill `name:` frontmatter.** Round-2 reversal in claude-code corpus: 8/10 skills use Title Case, kebab is the outlier. Cosmetic but consistency matters.
- **P2-6. SKILL.md size budget 1500-2000 words with `references/` offload** (claude-code `skill-development/SKILL.md:190`, BC-DRAFT-S04). Audit current SKILL.md word counts.
- **P2-7. `headersHelper` MCP auth pattern** for any future MCP server with rotating tokens (claude-code `authentication.md:233-258`).
- **P2-8. Tighten `allowed-tools` with subcommand globs** (claude-code `commit.md:2` is the canonical example). Blast-radius control.
- **P2-9. Governance NFR as first-class category in SOUL.md or rules/.** Adopt the 14 rules and the "tool of embarrassment" framing (superpowers `CLAUDE.md:9` verbatim): _"Your job is to protect your human partner from that outcome. Submitting a low-quality PR doesn't help them […] That is being a tool of embarrassment."_
- **P2-10. Circle K codephrase** (or equivalent) — out-of-band reset signal for performative-mode recovery.
- **P2-11. Dispatching-Parallel-Agents skill** with explicit boundary rules (independence, no shared state).

---

## P3 — Known divergences to document

- **P3-1. We require tests; superpowers does not.** Superpowers has `tests/` invocation harnesses but no CI. Document our divergence in SOUL.md #4 (silent failures): we do require tests, full stop.
- **P3-2. We are Claude Code-only.** Multi-platform support (Cursor/Codex/Gemini/OpenCode/Copilot) is explicitly out of scope. Document in plugin.json description.
- **P3-3. Plugin Settings: one semantic per file.** Superpowers' adopters conflate user-config and machine-state in one `.local.md`, which is a latent corruption hazard. Vsdd-factory should split: `.claude/vsdd-factory.local.md` = user config, `.claude/vsdd-factory-state.local.json` = machine state with `# DO NOT EDIT` header. Document in `factory-protocol.md`.
- **P3-4. We use Mermaid, not Graphviz DOT.** Superpowers' DOT convention is dogmatic and mainly cosmetic; Mermaid is fine here.
- **P3-5. We use a `commands/` directory plus skills.** Decide-and-document Pattern A from claude-code (commands invoke skills). Superpowers uses skills only.
- **P3-6. Spec-emergent vs spec-first.** Superpowers' brainstorm→plan→execute is incompatible with vsdd-factory's L1–L4 hierarchy by design. Document the trade-off in CLAUDE.md.
- **P3-7. Pragmatism principle tension** (see P1-3): SOUL.md #1 must explicitly distinguish principled pragmatism from pragmatism-as-rationalization, because superpowers' Pressure Taxonomy treats the latter as the #1 attack vector.

---

## Meta-lessons from running the ingest protocol itself

These inform the `brownfield-ingest` skill specifically.

1. **Strict-binary novelty (SUBSTANTIVE vs NITPICK, no middle ground) catches hallucinations.** The superpowers ingest caught **three round-1 fabrications** in round 2: invented Pressure types ("urgency/flattery/confusion"), miscounted Persuasion principles (6 vs 7), and over-extrapolated the No-Placeholders token list. The strict binary forced round 2 to justify findings as model-changing, which surfaced the errors. **Keep this protocol verbatim.**
2. **Honest convergence works.** All passes converged in 2-3 rounds; no pass needed the 5-round maximum. The "deepen until novelty decays" rule did not run away.
3. **Inline delivery (semport docs in `.factory/semport/<project>/`) is the right output shape.** Per-project subfolders + pass-N files + a single pass-6 final synthesis is readable and diff-able. Keep.
4. **Metric vs behavioral split:** the highest-value content from superpowers is **behavioral** (Iron Laws, Red Flags, persuasion matrix); the highest-value from claude-code is **metric/schema** (frontmatter regexes, validator rules, lifecycle events). Two reference repos surfaced two different artifact classes — the ingest protocol should explicitly tag findings as behavioral vs schema, since they feed different vsdd-factory subsystems.
5. **Round-1 hallucination classes** observed:
   - **Fabricated enumerations** (invented Pressure names) — caused by completing patterns from training data.
   - **Miscounts** (6 vs 7 principles) — caused by skimming.
   - **Over-extrapolation of token lists** (XXX/???/ellipsis claimed but absent) — caused by treating "common forbidden tokens" as canonical.
   - **Direction reversal** (kebab-vs-Title-Case majority claim flipped) — caused by stopping after the first 2 examples.
   All four were caught only when round 2 was forced to re-read source files. Lesson: **the first re-read pass is non-negotiable.**
6. **Validate-extraction agent corrections matter.** Phase A pass 0 over-counted superpowers supporting files at 32/5279 LOC; validate-extraction corrected to 23/3859 LOC. The downstream behavioral claims were unaffected, but the metric was wrong. Always run validation before final synthesis.
7. **Ingest cap vs true unknown** distinction (claude-code §7.2) is useful — separate "we didn't have time" from "the source genuinely doesn't say". Add this distinction to brownfield-ingest's gap reporting.
8. **The repo, not the synthesis, is authority.** When the two diverged in this run (e.g., the Plugin Settings adopter count), the repo won. Bake this rule into the brownfield-ingest skill explicitly.

---

## Appendix: File-by-file change list

| Priority | File | Change |
|---|---|---|
| P0-1 | `plugins/vsdd-factory/.claude-plugin/plugin.json` (NEW) | Create minimal manifest with name/version/description/author |
| P0-2 | `plugins/vsdd-factory/agents/architect.md` | Replace stub description, add `model:`, `color:` |
| P0-2 | `plugins/vsdd-factory/agents/business-analyst.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/code-reviewer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/data-engineer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/demo-recorder.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/devops-engineer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/dtu-validator.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/dx-engineer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/formal-verifier.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/github-ops.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/pr-manager.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/pr-reviewer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/product-owner.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/security-reviewer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/spec-reviewer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/spec-steward.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/state-manager.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/story-writer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/technical-writer.md` | Same |
| P0-2 | `plugins/vsdd-factory/agents/test-writer.md` | Same |
| P0-2 / P0-5 | `plugins/vsdd-factory/agents/implementer.md` | Fix truncated description; add `model:`, `color:`, `tools:` |
| P0-2 | `scripts/validate-agent-frontmatter.sh` (NEW) | Lint script enforcing BC-DRAFT-A05/A06 |
| P0-3 | `plugins/vsdd-factory/commands/` (NEW dir) OR `CLAUDE.md` | Either create thin commands or rewrite slash refs as skill activations |
| P0-4 | `plugins/vsdd-factory/skills/deliver-story/SKILL.md` | Restructure as SDD controller; add 4-status protocol, BLOCKED ladder, terminal final reviewer |
| P0-4 | `plugins/vsdd-factory/templates/implementer-prompt.md` (NEW) | Fresh-context implementer template |
| P0-4 | `plugins/vsdd-factory/templates/spec-reviewer-prompt.md` (NEW) | Fresh-context spec reviewer template |
| P0-4 | `plugins/vsdd-factory/templates/quality-reviewer-prompt.md` (NEW) | Fresh-context code-quality reviewer template |
| P1-1 | All `skills/*/SKILL.md` (Rigid skills first) | Add Iron Law, Announce-at-start, Red Flags table |
| P1-2 | `.claude/rules/persuasion-principles.md` (NEW) | Verbatim 7-principle matrix + ethical test |
| P1-3 | `SOUL.md` | Footnote on Principle #1 distinguishing principled pragmatism from rationalization |
| P1-3 | `plugins/vsdd-factory/skills/pressure-test/SKILL.md` (NEW) | Pressure-testing methodology with 7-type taxonomy |
| P1-4 | All `skills/*/SKILL.md` | Rewrite descriptions WHEN-not-WHAT, third person, ≤1024 chars |
| P1-5 | `.claude/vsdd-factory.local.md.template` (NEW) | Plugin Settings template |
| P1-5 | `.gitignore` | Add `.claude/*.local.md` and `.claude/*.local.json` |
| P1-6 | `plugins/vsdd-factory/hooks/protect-vp.sh` | Emit `permissionDecision` envelope; add supersede-rewrite path |
| P1-6 | `plugins/vsdd-factory/hooks/protect-bc.sh` | Same |
| P1-6 | `plugins/vsdd-factory/hooks/hooks.json` | Convert `session-learning.sh` and `handoff-validator.sh` entries to `type:"prompt"` |
| P1-7 | `scripts/validate-hook-schema.sh` (NEW) | Port from claude-code reference |
| P1-8 | `plugins/vsdd-factory/skills/verification-before-completion/SKILL.md` (NEW) | 5-step gate skill; reference from deliver-story, wave-gate, convergence-check |
| P2-1 | `.claude/rules/story-completeness.md` | Add `REQUIRED SKILL:` header field requirement |
| P2-1 | `plugins/vsdd-factory/skills/create-story/SKILL.md` | Emit `REQUIRED SKILL:` in story frontmatter |
| P2-2 | `.claude/rules/story-completeness.md` | Add No-Placeholders invariant + verbatim forbidden tokens |
| P2-3 | `plugins/vsdd-factory/hooks/session-start.sh` (NEW) | Inject STATE.md priming text |
| P2-3 | `plugins/vsdd-factory/hooks/hooks.json` | Add SessionStart entry |
| P2-9 | `.claude/rules/governance-nfr.md` (NEW) | Adopt 14 governance rules + tool-of-embarrassment framing |
| P3-3 | `.claude/rules/factory-protocol.md` | Document Plugin Settings dual-purpose split |
| P3-7 | `SOUL.md` | Pragmatism tension footnote (also P1-3) |
| Meta | `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` | Tag findings as behavioral vs schema; document repo-wins-over-synthesis rule; document ingest-cap vs true-unknown distinction |

_End of report._
