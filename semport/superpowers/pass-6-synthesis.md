# Pass 6: Synthesis

## Executive Summary

Superpowers is a **behavior-shaping plugin**, not a pipeline. Its entire value is injecting the right markdown into the right agent context at the right time and making it socially and rhetorically impossible for the agent to rationalize its way out of the discipline. The "runtime" is one bash hook that reads one markdown file and injects it wrapped in `<EXTREMELY_IMPORTANT>`. Everything else — 14 skills, 1 subagent, ~8.6k LOC of markdown — is content that the agent reads on demand via the Skill tool. There is no orchestrator. There is no state machine. There is no phase gate. The *agent itself* is the orchestrator, and the skills are priority-ordered behavioral constraints it applies to itself.

## Key Findings

1. **Zero orchestration code.** Hooks inject using-superpowers, which installs the 1% rule, which the agent uses to dispatch skills. The agent is the scheduler.
2. **Commands are deprecated.** All three slash commands are shims pointing to skills. The project has chosen model-invoked over user-invoked throughout.
3. **Fresh subagent per task is the discipline.** Every SDD task = three fresh subagents (implementer + spec reviewer + code quality reviewer). This is the same information-asymmetry pattern Corverax uses for adversarial review, applied per-task instead of per-wave.
4. **Iron Laws + Red Flags are the enforcement mechanism.** No types, no hooks, no validators — just rhetorically overwhelming anti-rationalization content. The skills document what observed agents tried to do wrong and pre-empt each excuse.
5. **Skills are empirical artifacts.** `CLAUDE.md:35-37` explicitly rejects "compliance" PRs against Anthropic's skill-authoring guidance because the content is tuned via pressure testing (`skills/writing-skills`). Skill content is *measured*, not designed.
6. **Multi-platform portability is first-class.** Claude Code, Cursor, Codex, OpenCode, Gemini, Copilot CLI — all supported via platform-detection logic in one hook script.
7. **TDD and verification discipline are the spine.** Every other skill ultimately flows into or from TDD + verification-before-completion.

## Confidence

| Area | Confidence | Basis |
|---|---|---|
| Architecture | HIGH | tiny surface, fully read |
| Domain model | HIGH | explicit in skill markdown |
| Behavioral contracts | HIGH (normative) / MEDIUM (compliance) | contracts are prose; compliance depends on agent adherence |
| NFRs | HIGH | stated explicitly in skills |
| Conventions | HIGH | consistent across all 14 skills |

## Gaps & Risks

- Compliance is probabilistic. A sufficiently stubborn model can reason around any prose rule.
- No machine-readable contract index for automated traceability.
- No per-skill versioning means improvements can regress older flows with no audit trail.

---

# Superpowers vs vsdd-factory: Philosophical Comparison

## TL;DR

| Axis | Superpowers | vsdd-factory (Corverax VSDD) |
|---|---|---|
| **Control model** | Auto-triggered skills; agent is the orchestrator | Orchestrator-driven; skills are phases invoked by slash commands |
| **Spec model** | Spec-emergent (brainstorm → design → plan) | Spec-first (brief → domain → PRD → arch → BCs → VPs) |
| **Gate model** | Skill priorities + Iron Laws (rhetorical) | Phase gates + wave gates + convergence checks (procedural) |
| **Enforcement** | Red Flags tables + HARD-GATE tags + Iron Laws | Hooks (spec-steward, push-guard), lefthook, CI, types |
| **TDD** | Iron Law in a rigid skill; "delete pre-test code" | Part of /deliver-story flow with RED gate + micro-commits |
| **Adversarial review** | Two-stage per-task review (spec + quality) via fresh subagents | Dedicated adversary agent across phases with novelty-decay loops |
| **Persistence** | Design doc + plan doc in `docs/superpowers/` | `.factory/` worktree on orphan branch with STATE.md, BC-INDEX, etc. |
| **Platforms** | 6+ platforms first-class | Claude Code only |
| **Domain artifacts** | None (workflow is the product) | L1-L4 spec hierarchy, BCs, VPs, ARCH, holdout scenarios |
| **Tone** | Confrontational anti-rationalization | Pragmatic engineering ("Pragmatism over Purity", SOUL.md #1) |

## 1. Orchestrator-driven vs Auto-triggered Skills

**vsdd-factory**: The human drives the pipeline with explicit slash commands (`/brainstorm`, `/create-brief`, `/create-prd`, `/decompose-stories`, `/deliver-story`, `/wave-gate`, etc.). Each command is a scripted workflow the skill executes in sequence. The human is the scheduler.

**Superpowers**: The agent drives the pipeline by auto-invoking skills based on "if there's a 1% chance it applies." Slash commands exist but are deprecated. The *agent* is the scheduler, and the hook merely primes it with a catalog + a rule about how aggressively to use it.

**Trade-off**: vsdd-factory gives predictability and auditability (STATE.md, sprint-state.yaml, phase-gate commits); superpowers gives adaptability and low friction (no need to remember which command to run). vsdd-factory's orchestrator model handles complex multi-artifact dependencies (BC-to-VP-to-ARCH traceability); superpowers' auto-trigger model collapses under that complexity.

## 2. Spec-first vs Spec-emergent

**vsdd-factory**: L1 product brief → L2 domain spec (sharded) → L3 behavioral contracts (BC-S.SS.NNN) → L4 verification properties (immutable once green) → architecture (sharded) → stories → waves. Specs are the product (SOUL.md #3: "The code is disposable. The spec is the product."). Six phases, each a gate.

**Superpowers**: Brainstorm → design doc → plan doc. Two artifacts. Plans are bite-sized task lists (2-5 min each), not hierarchies of requirements. There's no ubiquitous-language glossary, no entity model, no behavioral contract library. Spec emerges from a conversation, not from a methodology.

**When each wins**: For a system with meaningful invariants and multi-wave development, vsdd-factory's hierarchy prevents drift. For a solo engineer's next feature, superpowers' lightweight brainstorm→plan→execute is dramatically faster and loses nothing because there's no cross-wave consistency to protect.

## 3. Phase Gates vs Skill Priorities

**vsdd-factory** phase gates are **procedural and artifact-checked**: `/wave-gate` runs 6 checks (tests, DTU, adversarial review, demo evidence, holdout eval, state update); `/convergence-check` verifies 7 dimensions. A gate either passes or fails; failing blocks progress.

**Superpowers** has no gates. It has **skill priorities** (process skills first, impl second) and HARD-GATE tags inside individual skills (e.g. brainstorming forbids impl until design approved). Enforcement is rhetorical — the agent reading "Do NOT invoke any implementation skill..." is what stops it.

**Risk asymmetry**: vsdd-factory's gates can genuinely block a bad release; superpowers' gates cannot — a sufficiently motivated agent can reason past them. Conversely, vsdd-factory's gates can also *falsely* block progress when an automated check has a bug, while superpowers can never be blocked by a tooling failure.

## 4. Hook Enforcement

**vsdd-factory**: Hooks are *active enforcement* — spec-steward blocks edits to green VPs; push-guard verifies tests/clippy/no-todos before push; worktree-naming validates branch patterns; state-nudge reminds to update STATE.md on `.factory/` commits. These hooks *reject* actions.

**Superpowers**: The one hook is *passive injection* — SessionStart reads a file and returns JSON. It cannot reject anything. Its only job is to make sure the agent starts every session with the right primer.

This is the clearest architectural difference: vsdd-factory treats hooks as guards, superpowers treats hooks as loaders.

## 5. TDD Discipline

Both encode TDD as non-negotiable, but differently:

**Superpowers**: One Iron Law in a Rigid skill. "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST. Write code before the test? Delete it. Start over." (`skills/test-driven-development/SKILL.md:33-45`). The anti-rationalization content is extensive; the enforcement is rhetoric.

**vsdd-factory**: TDD is embedded in `/deliver-story`'s RED-GREEN-REFACTOR phases with micro-commits per pass, then enforced by the RED gate (verify tests fail for the right reason, not build errors) and the downstream mutation-kill-rate check (≥90%). The enforcement is multi-layered (workflow + CI + mutation score). The `bash.md` rules further add "no silent failures in test assertions" at the test-content level.

**vsdd-factory is stronger on verification that TDD was actually followed** (mutation kill rate catches vacuous tests). **Superpowers is stronger on the front-end discipline of starting with the test** (Iron Law + delete-pre-test-code rule).

## 6. What Each Does That the Other Doesn't

**Superpowers does, vsdd-factory doesn't:**

- **Multi-platform first-class support** (Cursor, Codex, Gemini, Copilot CLI, OpenCode). vsdd-factory is Claude Code only.
- **Auto-triggered skills with the 1% rule.** vsdd-factory requires the user to type the right slash command.
- **Per-task fresh-subagent discipline** (SDD pattern). vsdd-factory uses fresh-context subagents only for adversarial review and holdout eval, not for every implementation task.
- **Adversarial skill pressure-testing methodology.** Skills are developed by RED-GREEN-REFACTOR on agent behavior (`skills/writing-skills/SKILL.md:30-44`). vsdd-factory has no equivalent discipline for its skill/command authoring.
- **Red Flags table pattern.** Explicit rationalization inoculation per skill. vsdd-factory has no equivalent; it relies on engineer discipline and CI.
- **Verification-before-completion as a dedicated skill.** vsdd-factory has the same value (SOUL.md #4) but not a skill that enforces it at message boundaries.
- **Deprecation as a first-class move.** Superpowers shipped shim commands to migrate users away from them. vsdd-factory has no deprecation path.
- **Dispatching-parallel-agents skill.** Explicit recognition that independent failures should be investigated in parallel by independent subagents. vsdd-factory does this implicitly in some commands but doesn't name it as a generalizable pattern.

**vsdd-factory does, superpowers doesn't:**

- **Hierarchical spec model** (L1-L4 with traceability from BCs to VPs to ARCH sections). Superpowers has design doc + plan doc, nothing more.
- **Phase gates and wave gates** that actually block progress based on automated checks.
- **Persistent state** via `.factory/` worktree on orphan branch with STATE.md, sprint-state.yaml, cycle manifests.
- **Holdout scenario evaluation** (information-asymmetry test of the full system).
- **Formal verification path** (Kani, cargo-fuzz, cargo-mutants, semgrep).
- **Mutation testing** as a quality gate (90% kill rate requirement).
- **Cycle-scoped artifacts** (`.factory/cycles/vX.Y.Z/`) vs. superpowers' date-prefixed files with no cycle concept.
- **Active guarding hooks** (spec-steward, push-guard, etc.) that reject rather than inject.
- **Per-module criticality classification** (CRITICAL/HIGH/MEDIUM/LOW) feeding review depth decisions.
- **Gene-transfusion / semport** as an alternative implementation strategy.
- **Brownfield ingest with convergence deepening** (this very operation).
- **Type-system enforcement** of invariants (SOUL.md #2 "Make the wrong thing impossible"). Superpowers has no type system to lean on — it is markdown.
- **Conventional Commits + lefthook + CI** as a compliance layer for all commits.

## 7. What vsdd-factory Should Learn from Superpowers

1. **Adopt the Red Flags table pattern in every skill.** For each vsdd-factory slash command, enumerate the rationalizations an agent will use to shortcut the workflow, and pre-empt each with a rebuttal. This is free quality.
2. **Adopt the "announce at start" convention.** Every skill/command should emit "I'm using the X skill to Y" at start. Creates a human-visible audit trail and forces the agent to commit publicly to using the right workflow.
3. **Adopt the per-task fresh-subagent pattern (SDD) for /deliver-story.** Currently /deliver-story runs the full RED/GREEN/REFACTOR in one agent context. Dispatching fresh subagents per task + spec review + quality review would preserve the parent context for coordination and improve adherence. This is arguably the biggest potential improvement.
4. **Adopt skill pressure-testing methodology for vsdd-factory's own skills.** Write adversarial scenarios, run without the skill, observe failure modes, write the skill to close them. The Corverax skills are currently hand-designed; they should be empirically tuned.
5. **Consider a lightweight SessionStart injection** that primes every Claude Code session with a "check .factory/STATE.md first" nudge and the 1% rule for vsdd-factory skills. Currently this relies on the human remembering `/factory-health`.
6. **Adopt `<EXTREMELY_IMPORTANT>` and `<HARD-GATE>` tag conventions** inside skills for the rules that MUST not be skipped (e.g. "never commit to main", "never skip RED gate"). The tags are redundant with the prose but empirically increase compliance.
7. **Multi-platform portability as a future concern.** vsdd-factory is Claude Code only; superpowers shows it's feasible to support 6+ platforms with a single hook dispatcher. Not a priority now, but cheap to design for.

## 8. What Superpowers Could Learn from vsdd-factory

1. **Hierarchical specs** when building multi-wave systems. A brainstorm+plan is insufficient for systems with cross-cutting invariants.
2. **Active enforcement hooks** (not just injection). A push-guard that blocks pushes without verification would materialize the verification-before-completion skill at git level.
3. **Mutation testing** as evidence that the TDD Iron Law was actually followed (not just rhetorically enforced).
4. **Persistent state** via `.factory/`-style orphan worktree so workflow history survives across sessions.
5. **Holdout scenarios** (information-asymmetry end-to-end evaluation) as a skill.
6. **Formal verification paths** for critical modules.
7. **Criticality-based review depth** (CRITICAL gets security review, MEDIUM does not).
