# Pass 4: Non-Functional Requirements

## Quality Guarantees

### TDD Enforcement (Rigidity)

`skills/test-driven-development/SKILL.md` frames TDD as a Rigid skill (`skills/using-superpowers/SKILL.md:110`: "Rigid (TDD, debugging): Follow exactly. Don't adapt away discipline"). The Iron Law (line 33-34) and the "delete means delete" rule (lines 39-45) remove the agent's ability to partially comply.

### Verification Before Completion

`skills/verification-before-completion/SKILL.md:18-38` defines a 5-step gate function (IDENTIFY → RUN → READ → VERIFY → CLAIM) that must fire before every completion claim. Same philosophy as the Corverax convergence protocol but at message granularity.

### Root Cause Discipline

`skills/systematic-debugging/SKILL.md:46-50` requires completing Phase 1 before proposing fixes. Iron Law at line 18-19.

## Safety & Guardrails

### Anti-rationalization (Red Flags)

Every behavior-shaping skill ships with a Red Flags table listing the rationalizations the agent will try to use to skip the skill, paired with rebuttals. This is the primary guardrail against AI hallucination-to-convenience. Examples:

- `skills/using-superpowers/SKILL.md:80-95` (12 rationalizations)
- Similar tables in TDD, debugging, verification skills

### HARD-GATE markup

`<HARD-GATE>` blocks (`skills/brainstorming/SKILL.md:12-14`) explicitly forbid implementation actions until approval. Tag-based, not enforced by hooks.

### Subagent context isolation

SDD and dispatching-parallel-agents both require fresh context per subagent to prevent context pollution and context exhaustion (`skills/subagent-driven-development/SKILL.md:10-11`; `skills/dispatching-parallel-agents/SKILL.md:10-11`). This mirrors Corverax's information-asymmetry principle for adversarial review.

### Subagent-Stop

`<SUBAGENT-STOP>` prevents recursive skill loading when a subagent is dispatched to execute a task (`skills/using-superpowers/SKILL.md:6-8`).

### Instruction Priority (safety rail on behavior-override)

Explicit 3-level priority: user instructions > superpowers skills > default system prompt (`skills/using-superpowers/SKILL.md:19-26`). Prevents the plugin from overriding legitimate user overrides.

## Observability

- No metrics, no tracing, no structured logging infrastructure
- Observability is indirect, through tests:
  - `tests/claude-code/analyze-token-usage.py` — token usage analysis
  - `tests/skill-triggering/run-test.sh` — empirical skill-trigger validation
  - Pressure testing (`skills/writing-skills/SKILL.md:14-17`) — subagent-based behavioral verification

## Reliability

- Zero-dependency by policy (`CLAUDE.md:31-33`)
- Polyglot hook dispatcher (`hooks/run-hook.cmd`, docs at `docs/windows/polyglot-hooks.md`) for Windows compatibility
- Bash 5.3+ heredoc hang workaround documented inline (`hooks/session-start:44`, issue #571)

## Performance / Context Budget

- Only `using-superpowers` is injected eagerly; all other skills are lazy-loaded on Skill tool invocation — preserves context window
- SDD's fresh-subagent-per-task pattern explicitly described as context preservation ("preserves your own context for coordination work", `skills/subagent-driven-development/SKILL.md:10-11`)

## Portability (Platform NFR)

First-class across: Claude Code, Cursor, Codex, OpenCode, Gemini CLI, Copilot CLI. Platform detection logic in `hooks/session-start:49-67` emits different JSON shapes per detected env. Tool name mappings for non-Claude platforms in `skills/using-superpowers/references/` (copilot-tools.md, codex-tools.md).

## Missing / Unexpected

- No formal verification. No fuzzing. No proofs. Quality is entirely empirical-behavioral.
- No versioning of individual skills — single plugin version (5.0.7 in `package.json:3`).
- No dependency on specific model — designed to work with the frontier model of each platform.
