# Changelog

## 0.5.0 — Wave 4: Enforcement layer (hooks)

Ported dark-factory's OpenClaw runtime extensions to Claude Code hooks. This is the "make the wrong thing impossible" wave — recovering the enforcement discipline that was missing from the initial extract.

**New hooks** (in `plugins/vsdd-factory/hooks/`):

- `brownfield-discipline.sh` (PreToolUse) — blocks edits to `.reference/**`
- `protect-bc.sh` (PreToolUse) — blocks edits to green Behavioral Contracts
- `red-gate.sh` (PreToolUse) — enforces TDD red-before-green when `.factory/red-gate-state.json` declares strict mode; opt-in per project
- `purity-check.sh` (PostToolUse, warn) — flags side-effect patterns in pure-core paths (`*/pure/**`, `*/core/**`, `*_pure.rs`, `*.pure.ts`, `*/kernel/*`)
- `regression-gate.sh` (PostToolUse) — records cargo/pytest/npm/go test outcomes to `.factory/regression-state.json`, warns on pass→fail transitions
- `handoff-validator.sh` (SubagentStop) — warns on empty/truncated subagent output
- `session-learning.sh` (Stop) — appends session-end markers to `.factory/sidecar-learning.md`

**Wired existing hooks**:

- `protect-vp.sh` (PreToolUse, Edit|Write) — already shipped, now registered
- `verify-git-push.sh` (PreToolUse, Bash) — registered
- `check-factory-commit.sh` (PreToolUse, Bash) — registered

All hooks follow `.claude/rules/bash.md`: `set -euo pipefail`, jq-based JSON parsing with stderr guards, no `eval`, tool availability checks, STDERR-EXEMPT tags where stderr suppression is intentional. All 10 hooks pass `bash -n` syntax checks and basic smoke tests.

**Not portable** (needs API-level integration Claude Code doesn't expose):

- Cost tracker, attention heatmap, tiered-context enforcement, full sidecar-learning synthesis — will ship as doc stubs in Wave 6.

## 0.4.0 — Wave 3: Design system, UX, and market intelligence

- Ported 13 skills for UI-heavy projects and product-intelligence workflows
- **Design & UX:** `design-drift-detection`, `design-system-bootstrap`, `multi-variant-design`, `storybook-mcp-integration`, `responsive-validation`, `ui-completeness-check`, `ui-quality-gate`, `ux-heuristic-evaluation`
- **Market & customer:** `competitive-monitoring`, `customer-feedback-ingestion`, `intelligence-synthesis`, `market-intelligence-assessment`, `analytics-integration`
- `templates/design-system/` already present from initial extraction

Total skills: 83.

## 0.3.0 — Wave 2: Skill coverage (feature-mode + maintenance)

- Ported 39 skills from dark-factory workflow catalogue
- **Feature-mode (F1–F7):** `phase-f1-delta-analysis`, `phase-f2-spec-evolution`, `phase-f3-incremental-stories`, `phase-f4-delta-implementation`, `phase-f5-scoped-adversarial`, `phase-f6-targeted-hardening`, `phase-f7-delta-convergence`
- **Maintenance & discovery:** `maintenance-sweep`, `discovery-engine`, `planning-research`, `post-feature-validation`, `pr-review-triage`, `fix-pr-delivery`
- **Mode routing:** `mode-decision-guide`, `quick-dev-routing`, `feature-mode-scoping-rules`, `implementation-readiness`, `validate-brief`
- **Infrastructure:** `model-routing`, `repo-initialization`, `toolchain-provisioning`, `wave-scheduling`, `spec-versioning`, `traceability-extension`, `sdk-generation`
- **Session & consistency:** `consistency-validation`, `convergence-tracking`, `artifact-detection`, `phase-1-prd-revision`, `phase-1d-adversarial-spec-review`, `multi-repo-phase-0-synthesis`, `factory-worktree-health`, `dtu-creation`, `brainstorming`, `agent-file-review`, `code-delivery`, `demo-recording`, `session-review`, `guided-brief-creation`
- Replaced Corverax's `release` skill with dark-factory's authoritative version per merge rules

Total skills: 70 (was 31).

## 0.2.0 — Wave 1: Foundation

- Shipped `docs/VSDD.md`, `docs/FACTORY.md`, `docs/CONVERGENCE.md` — the methodology documents
- Shipped `docs/AGENT-SOUL.md` — shared engine-wide agent principles
- Ported 28 dark-factory agents into single-file triune format (`## Identity` + `## Operating Procedure`) with synthesized frontmatter:
  `accessibility-auditor, architect, business-analyst, code-reviewer, consistency-validator, data-engineer, demo-recorder, devops-engineer, dtu-validator, dx-engineer, e2e-tester, formal-verifier, github-ops, implementer, performance-engineer, pr-manager, pr-reviewer, product-owner, security-reviewer, session-review, spec-reviewer, spec-steward, state-manager, story-writer, technical-writer, test-writer, ux-designer, visual-reviewer`
- Shipped `agents/orchestrator/` as a directory containing `orchestrator.md` plus 9 mode-sequence sub-files (greenfield, brownfield, feature, maintenance, discovery, multi-repo, per-story-delivery, steady-state, heartbeat)
- Kept Corverax's enhanced versions of the 5 overlapping agents (adversary, codebase-analyzer, holdout-evaluator, research-agent, validate-extraction) unchanged

Total agents: 34 (33 dark-factory + 1 Corverax addition).

## 0.1.0 — Initial marketplace

- Extracted Corverax `.claude/` VSDD pipeline into a shareable plugin marketplace
- 5 agents, 31 skills, 3 hooks, rules, templates
