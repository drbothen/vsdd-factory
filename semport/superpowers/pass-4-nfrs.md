# Pass 4: Non-Functional Requirements

_Phase B deepening round 1._

## Quality Guarantees

### TDD Enforcement (Rigidity)

`skills/test-driven-development/SKILL.md` frames TDD as a Rigid skill (`skills/using-superpowers/SKILL.md:110`: "Rigid (TDD, debugging): Follow exactly. Don't adapt away discipline"). The Iron Law (line 33-34) and the "delete means delete" rule (lines 39-45) remove the agent's ability to partially comply.

### Verification Before Completion

`skills/verification-before-completion/SKILL.md:18-38` defines a 5-step gate function (IDENTIFY → RUN → READ → VERIFY → CLAIM) that must fire before every completion claim. Same philosophy as the Corverax convergence protocol but at message granularity.

### Root Cause Discipline

`skills/systematic-debugging/SKILL.md:46-50` requires completing Phase 1 before proposing fixes. Iron Law at line 18-19.

### Iron Law Inventory (round 1)

Passes 2+3 rounds 2/3 surfaced a formal inventory of five Iron Laws across the Rigid-skill class. Iron Laws are the highest-severity quality guarantee in the repo: a single all-caps blockquote the skill author commits to non-negotiability on. Reserved for skills where partial compliance is worse than non-compliance.

| Iron Law | Skill | Class |
|---|---|---|
| NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST | test-driven-development | TDD discipline |
| NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST | systematic-debugging | Debugging discipline |
| NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE | verification-before-completion | Completion discipline |
| Writing-skills Iron Law (pressure-test-before-ship) | writing-skills | Authoring discipline |
| HARD-GATE on implementation until brainstorming approved | brainstorming | Planning discipline |

The Iron Law pattern is the repo's mechanism for making the wrong thing impossible at the prompt-engineering layer. Flexible skills deliberately do NOT get Iron Laws; they get "Core principle" instead.

### TDD Enforcement Extensions (round 1)

Pass 3 round 2 added contracts Phase A did not capture:

- **Verify-RED gate**: after writing a failing test, the agent MUST run it and observe a red result before proceeding (not merely write the test).
- **Verify-GREEN gate**: after implementation, the agent MUST run the previously-red test and observe green. Skipping the re-run is a Red Flag.
- **Exception protocol**: explicit short-list of when TDD does not apply (pure exploration, throwaway spikes) with a companion rule that "I'm just exploring" becomes a rationalization if the code is later kept.
- **Debugging integration**: "never fix bugs without a test." The debugging skill requires a failing reproduction test before any fix is proposed, chaining TDD discipline into the debugging flow.

### Eval-Evidence Requirement (NEW NFR, round 1)

BC-DRAFT-046 formalizes `CLAUDE.md:47-52` ("Skill Changes Require Evaluation"): any modification to skill content — especially Red Flags tables, rationalization lists, and "your human partner" language — requires before/after eval evidence demonstrating the change improves agent behavior. Behavior-shaping prose is treated as code; changes are rejected without evidence they shift agent outputs favorably. Pressure testing via `skills/writing-skills` is the sanctioned eval harness.

## Safety & Guardrails

### Anti-rationalization (Red Flags)

Every behavior-shaping skill ships with a Red Flags table listing the rationalizations the agent will try to use to skip the skill, paired with rebuttals. Primary guardrail against AI hallucination-to-convenience. Examples: `using-superpowers/SKILL.md:80-95` (12 rationalizations); similar tables in TDD, debugging, verification skills.

### HARD-GATE markup

`<HARD-GATE>` blocks (`skills/brainstorming/SKILL.md:12-14`) explicitly forbid implementation actions until approval. Tag-based, not enforced by hooks — relies on the agent parsing and honoring the tag. Round 1: this is explicitly acknowledged as a trust-based rail, not machine-enforced.

### Subagent context isolation

SDD and dispatching-parallel-agents both require fresh context per subagent to prevent context pollution and exhaustion. Mirrors Corverax's information-asymmetry principle for adversarial review.

### Subagent-Stop

`<SUBAGENT-STOP>` prevents recursive skill loading when a subagent is dispatched (`skills/using-superpowers/SKILL.md:6-8`).

### Instruction Priority

Explicit 3-level priority: user instructions > superpowers skills > default system prompt (`skills/using-superpowers/SKILL.md:19-26`).

### Forbidden Gratitude Class (round 1)

`skills/receiving-code-review/SKILL.md:27-38` formalizes a class of banned phrases the agent is never allowed to emit: "You're absolutely right!", "Great point!", "Excellent feedback!", "Let me implement that now" (pre-verification). Linguistic guardrail: the repo treats performative agreement as a correctness bug because it correlates with skipping verification. Enforced by prose, escalated to explicit CLAUDE.md violation status.

### Circle K Codephrase (round 1)

`receiving-code-review` introduces "Circle K" as a user-invokable codephrase the human partner can drop to signal the agent is sliding into performative/compliant behavior and should stop and recenter. First explicit out-of-band user-to-agent safety signal in the repo.

### Unclear-Item Halt Rule (round 1)

`skills/receiving-code-review/SKILL.md:40-57`: if ANY review item is unclear, STOP and ask before implementing any item — even the clear ones. Rationale: items are often related, and partial understanding produces wrong implementation. Halt-on-ambiguity rule at the feedback-ingestion boundary.

### YAGNI Check (round 1)

Pass 3 round 2 surfaced a YAGNI guardrail embedded in the brainstorming/planning flow: before adding scope, the agent must justify present need — not speculative future need. The antibody to the "speculative or theoretical fixes" PR rejection class.

### Plan Document No-Placeholders Class (round 1)

Plan documents have a forbidden-token set — `TODO`, `TBD`, `XXX`, `<fill in>`, `???`, ellipsis-as-content are disallowed. A plan containing any of them is considered incomplete and must not be executed. Plan-layer analog of the PR template no-placeholders rule.

### Worktree Safety Invariants (round 1)

`skills/using-git-worktrees` and `skills/finishing-a-development-branch` encode:

- Worktrees created under a gitignored parent directory so they cannot be accidentally committed.
- Finishing a branch requires explicit confirmation before deletion; branch removal is never implicit.
- Worktree creation must succeed (verification gate) before any work is attempted inside it.

### Implementer STOP Triggers (round 1)

Implementation skills define STOP conditions: when the agent recognizes it is in over its head (scope surprise, cascading unknowns, repeated test failures without hypothesis) it MUST halt and surface the situation to the human partner rather than continuing to flail. Reliability control encoded as behavioral rule.

### File-Growth Escalation Rule (round 1)

If a file being edited grows substantially beyond the planned delta, the agent must stop and escalate — unbounded file growth is a signal that the mental model is wrong and the plan needs revisiting.

## Observability

- No metrics, no tracing, no structured logging infrastructure.
- Observability is indirect, through tests:
  - `tests/claude-code/analyze-token-usage.py` — token usage analysis
  - `tests/skill-triggering/run-test.sh` — empirical skill-trigger validation (round 1: BC-DRAFT-045 — grep-based assertion over Claude Code stream-json output; test passes iff the skill name appears in the tool-use stream for the given trigger prompt). Canonical behavioral observability harness.
  - Pressure testing (`skills/writing-skills/SKILL.md:14-17`) — subagent-based behavioral verification. Round 1: passes 2+3 formalized a **7-type pressure taxonomy** (time pressure, authority pressure, sunk cost, social proof, urgency, flattery, confusion) that the author must run a new skill against before shipping.

## Reliability

- Zero-dependency by policy (`CLAUDE.md:31-33`).
- Polyglot hook dispatcher (`hooks/run-hook.cmd`, docs at `docs/windows/polyglot-hooks.md`) for Windows compatibility.
- Bash 5.3+ heredoc hang workaround documented inline (`hooks/session-start:44`, issue #571).
- (round 1) **OpenCode idempotency invariant**: `experimental.chat.messages.transform` hook must be safe to invoke repeatedly without producing duplicated transformations. Encoded as behavioral invariant of the transform function.
- (round 1) **Legacy skills-directory warning**: `hooks/session-start:12-15` detects a pre-plugin install location and warns the user to migrate. Reliability surface for upgrade-in-place users.

## Performance / Context Budget

- Only `using-superpowers` is injected eagerly; all other skills are lazy-loaded on Skill-tool invocation.
- SDD's fresh-subagent-per-task pattern explicitly described as context preservation.
- **Token budgets per skill class** (round 1, pass 2 round 2):
  - Flexible/reference skills: soft budget < 500 words of behavior-shaping prose
  - Discipline skills: < 200 words in the core body before examples
  - Trigger-description (`description` frontmatter field): hard ceiling 1024 chars, with CSO guidance to stay far below

Skills that exceed their class budget are candidates for sharding into references.

## Portability (Platform NFR)

First-class across: Claude Code, Cursor, Codex, OpenCode, Gemini CLI, Copilot CLI. Platform detection in `hooks/session-start:49-67` emits different JSON shapes per detected env. Tool-name mappings in `skills/using-superpowers/references/` (`copilot-tools.md`, `codex-tools.md`, `gemini-tools.md`).

Round 1: portability is encoded as **contracts**, not just docs. BC-031 (hook emits platform-correct JSON), BC-033 (tool-name translation exists for every non-Claude platform), and BC-045 (stream-json test harness only asserts on Claude Code) bound the portability surface. The repo treats "works on all six platforms" as a testable property, not aspiration.

## Governance / PR-Hygiene (NEW NFR, round 1)

`CLAUDE.md:1-90` establishes a governance NFR layer that has no analog in most codebases. It is a set of repo-level behavioral contracts the agent must satisfy before interacting with the project's PR surface. The NFR framing is appropriate because these rules are not functional requirements of the plugin — they are quality-of-contribution requirements enforced at the maintainer boundary.

Key elements:

- **94% PR rejection rate** is cited as empirical evidence that the guardrails are necessary (`CLAUDE.md:7`).
- **PR template must be fully filled**; no placeholders, no summaries-in-place-of-answers (`CLAUDE.md:13,25`).
- **Existing-PR search** (open AND closed) mandatory before opening a new one (`CLAUDE.md:14,27`).
- **No speculative fixes**: every PR must describe a real session/error/user-experience that motivated it (`CLAUDE.md:15,53`).
- **One problem per PR**, no bulk/spray-and-pray (`CLAUDE.md:49,86`).
- **Domain-specific skills belong in standalone plugins**, not core (`CLAUDE.md:16,55,61`).
- **Third-party dependencies forbidden** except for new harness support (`CLAUDE.md:33`).
- **"Compliance" PRs** that reformat skills to match Anthropic's published skills docs are rejected without eval evidence (`CLAUDE.md:37`).
- **Fabricated content, fork-specific changes, bundled unrelated changes** are immediate-close (`CLAUDE.md:63,67,71`).
- **Human-involvement evidence** required: a human must review the complete diff before submission (`CLAUDE.md:29`).

This layer encodes a governance contract: the plugin's quality bar is maintained by rejecting low-quality contributions rather than by machine verification. The agent is explicitly framed as an accomplice to the human partner ("your job is to protect your human partner from that outcome", `CLAUDE.md:9`) rather than as an autonomous contributor.

## Missing / Unexpected

- No formal verification. No fuzzing. No proofs. Quality is entirely empirical-behavioral.
- No versioning of individual skills — single plugin version (`package.json:3`).
- No dependency on specific model — designed to work with the frontier model of each platform.
- (round 1) No machine enforcement of the Governance NFR — entirely prose-and-reviewer enforced, with the agent itself as first-line enforcement via CLAUDE.md.

## Delta Summary

- New items added: Iron Law Inventory (5), TDD Enforcement Extensions (4 sub-rules), Eval-Evidence Requirement NFR, Forbidden Gratitude class, Circle K codephrase, Unclear-Item Halt rule, YAGNI Check, Plan No-Placeholders class, Worktree Safety Invariants, Implementer STOP Triggers, File-Growth Escalation, stream-json observability harness, Pressure Taxonomy (7 types), OpenCode idempotency invariant, legacy-dir warning, Token Budget NFR, portability-as-contracts reframe, Governance/PR-Hygiene NFR (9 sub-rules).
- Refined: Observability (3 bullets → named harness with citation), Portability (list → contract-bounded property), HARD-GATE (trust-based acknowledgment).
