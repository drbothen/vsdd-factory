# vsdd-factory Plugin Inventory

Generated 2026-04-08. Scans all files under `/Users/jmagady/Dev/vsdd-factory/` excluding `.git/`, `.factory/`, `.reference/`.

## Repo root

| File | LOC | Purpose |
|---|---|---|
| `README.md` | 43 | Marketplace overview, install snippet, pipeline phase list, local-dev invocation, MIT license note. |
| `CHANGELOG.md` | 126 | Wave-by-wave changelog from 0.1.0 → 0.8.0. Latest: Wave 7 validation infra (41 TAP tests). |
| `LICENSE` | 21 | MIT. |
| `.gitignore` | 8 | Standard ignores. |
| `.claude-plugin/marketplace.json` | 18 | Marketplace manifest (see below). |
| `.github/workflows/plugin-validation.yml` | 47 | CI: bats tests, JSON manifest validation, workflow parse checks. |
| `plugins/vsdd-factory/.claude-plugin/plugin.json` | 12 | Plugin manifest (see below). |

## Marketplace

`.claude-plugin/marketplace.json`:
- `name: vsdd-factory`, `owner.name: drbothen`
- Description: "VSDD dark factory marketplace — spec-driven SDLC pipeline plugins for Claude Code."
- Single plugin entry: `vsdd-factory` at `./plugins/vsdd-factory`, category `sdlc`, tags `[vsdd, spec-driven, tdd, factory]`.
- No explicit `version` field in marketplace.json (version only lives in plugin.json).
- No issues in structure; single-plugin marketplace.

## Plugin: plugins/vsdd-factory/

### plugin.json

```json
{
  "name": "vsdd-factory",
  "description": "Verified Spec-Driven Development (VSDD) dark factory for software — full SDLC pipeline: brownfield ingest, spec crystallization, story decomposition, TDD delivery, adversarial review, holdout evaluation, formal verification, and release gating.",
  "version": "0.8.0",
  "author": { "name": "drbothen" },
  "homepage": "https://github.com/drbothen/vsdd-factory",
  "repository": "https://github.com/drbothen/vsdd-factory",
  "license": "MIT",
  "keywords": ["vsdd", "spec-driven", "tdd", "factory", "sdlc", "agents", "pipeline"]
}
```

Version `0.8.0` matches CHANGELOG top entry.

### agents/ (count: 33 top-level .md files)

All have YAML frontmatter (`fm=1`). Many descriptions are stub placeholders reading literally "VSDD factory agent: <name>".

| File | has-fm | description (first line) | model | color | LOC | issues |
|---|---|---|---|---|---|---|
| accessibility-auditor.md | yes | WCAG compliance auditing at multiple pipeline points. Validates accessibility | — | — | 286 | |
| adversary.md | yes | Fresh-context adversarial reviewer for specs and implementation. Finds gaps, contradictions, missing edge cases, and unstated assumptions. Uses different model for genuine perspective diversity. Cannot see prior review passes. | sonnet | — | 105 | |
| architect.md | yes | VSDD factory agent: architect | — | — | 440 | stub description |
| business-analyst.md | yes | VSDD factory agent: business-analyst | — | — | 172 | stub description |
| code-reviewer.md | yes | VSDD factory agent: code-reviewer | — | — | 155 | stub description |
| codebase-analyzer.md | yes | Deep codebase scanner for semantic analysis, language detection, pattern extraction, and reference implementation assessment. Uses a structured 6-pass analysis protocol with file prioritization, test-as-spec extraction, and state checkpointing. Used by brownfield-ingest and semport-analyze. | opus | — | 419 | |
| consistency-validator.md | yes | Cross-document validation and consistency checking. Validates spec chains, | — | — | 302 | |
| data-engineer.md | yes | VSDD factory agent: data-engineer | — | — | 122 | stub description |
| demo-recorder.md | yes | VSDD factory agent: demo-recorder | — | — | 161 | stub description |
| devops-engineer.md | yes | VSDD factory agent: devops-engineer | — | — | 383 | stub description |
| dtu-validator.md | yes | VSDD factory agent: dtu-validator | — | — | 156 | stub description |
| dx-engineer.md | yes | VSDD factory agent: dx-engineer | — | — | 234 | stub description |
| e2e-tester.md | yes | End-to-end testing of user journeys, visual validation, and browser-based | — | — | 238 | |
| formal-verifier.md | yes | VSDD factory agent: formal-verifier | — | — | 250 | stub description |
| github-ops.md | yes | VSDD factory agent: github-ops | — | — | 125 | stub description |
| holdout-evaluator.md | yes | Evaluate implementation against hidden acceptance scenarios with strict information asymmetry. Cannot see source code internals, specs, implementation notes, or prior review passes. Only sees public API surface and holdout scenarios. | sonnet | — | 87 | |
| implementer.md | yes | Strict TDD implementation agent. Picks next failing test, writes minimum code | — | — | 305 | |
| performance-engineer.md | yes | Performance optimization, benchmarking, and Core Web Vitals enforcement. | — | — | 177 | |
| pr-manager.md | yes | VSDD factory agent: pr-manager | — | — | 212 | stub description |
| pr-reviewer.md | yes | VSDD factory agent: pr-reviewer | — | — | 213 | stub description |
| product-owner.md | yes | VSDD factory agent: product-owner | — | — | 361 | stub description |
| research-agent.md | yes | Conduct external research — technology evaluations, library comparisons, security advisory lookups, architecture pattern research, and domain research. Always cites sources, verifies library versions against registries, and flags inconclusive findings. | opus | — | 140 | |
| security-reviewer.md | yes | VSDD factory agent: security-reviewer | — | — | 272 | stub description |
| session-review.md | yes | Reviews completed sessions — captures lessons, decisions, and follow-ups into sidecar learning artifacts. | — | — | 208 | dup name with `skills/session-review/` |
| spec-reviewer.md | yes | VSDD factory agent: spec-reviewer | — | — | 198 | stub description |
| spec-steward.md | yes | VSDD factory agent: spec-steward | — | — | 221 | stub description |
| state-manager.md | yes | VSDD factory agent: state-manager | — | — | 278 | stub description |
| story-writer.md | yes | VSDD factory agent: story-writer | — | — | 417 | stub description |
| technical-writer.md | yes | VSDD factory agent: technical-writer | — | — | 71 | stub description; smallest agent |
| test-writer.md | yes | VSDD factory agent: test-writer | — | — | 316 | stub description |
| ux-designer.md | yes | Creates UX specifications, wireframes, and interaction designs for UI products. | — | — | 220 | |
| validate-extraction.md | yes | AgenticAKM-style validator for codebase analysis output. Verifies extracted behavioral contracts, domain models, and architecture docs against actual code. Catches hallucinated dependencies, phantom modules, and inaccurate contracts. Max 3 refinement iterations. | sonnet | — | 113 | |
| visual-reviewer.md | yes | Visual verification of demos, screenshots, and UI fidelity. Uses multimodal | — | — | 205 | |

Observations:
- No agent file sets a `color:` field.
- Only 5 agents set `model:` (adversary/holdout-evaluator/validate-extraction = sonnet; codebase-analyzer/research-agent = opus). The remaining 28 inherit default.
- 22 of 33 agents still carry the generic `VSDD factory agent: <name>` stub description — the README claims bespoke agents; in practice only the 5 Corverax-origin ones plus UI/UX-flavoured ones have real descriptions.
- No truncations or empty bodies detected; all have frontmatter.

### agents/orchestrator/ (count: 10 files)

| File | has-fm | description | LOC | notes |
|---|---|---|---|---|
| orchestrator.md | yes | VSDD pipeline driver — reads workflow data and spawns sub-agents in dependency order across phases and modes. | 322 | The actual agent definition. |
| brownfield-sequence.md | no | — | 125 | Included data, not a sub-agent. |
| discovery-sequence.md | no | — | 101 | Included data. |
| feature-sequence.md | no | — | 160 | Included data. |
| greenfield-sequence.md | no | — | 203 | Included data. |
| HEARTBEAT.md | no | — | 44 | Runtime heartbeat doc (uppercase filename outlier). |
| maintenance-sequence.md | no | — | 103 | Included data. |
| multi-repo.md | no | — | 83 | Included data. |
| per-story-delivery.md | no | — | 131 | Included data. |
| steady-state.md | no | — | 137 | Included data. |

Note: Only `orchestrator.md` is an actual agent. The other 9 files look like mode-sequence documentation the orchestrator references. They would be reported as agents by a naive scan because they live under `agents/` — may cause a validator to flag missing frontmatter.

### skills/ (count: 91 skills)

All skills have a `SKILL.md`. 89 have YAML frontmatter; **2 lack frontmatter** entirely:
- `skills/excalidraw-export/SKILL.md` — opens with `# Excalidraw PNG Export`
- `skills/jira/SKILL.md` — opens with `# Jira CLI Reference`

Both are clearly "reference docs" rather than invokable skills, but they still sit in the skills tree.

`disable-model-invocation: true` set on 20 skills (explicit-invoke gates): adversarial-review, convergence-check, create-architecture, create-brief, create-domain-spec, create-prd, create-story, decompose-stories, deliver-story, dtu-validate, factory-health, formal-verify, generate-pdf, holdout-eval, perf-check, pr-create, record-demo, research, setup-env, spec-drift, track-debt, validate-consistency, wave-gate, worktree-manage.

Many descriptions use YAML folded style (`description: >`) — these render as "FOLDED" in simple scans but are valid.

Skill table (dir | LOC | has-fm | disable-model-invocation | allowed-tools | description summary / issues):

| Skill | LOC | fm | dmi | allowed-tools | Description / issues |
|---|---|---|---|---|---|
| activate | 36 | y | — | — | Opt in to VSDD persona; writes `.claude/settings.local.json`. |
| adversarial-review | 50 | y | true | — | Launch fresh-context adversarial review via adversary agent. References templates (3). |
| agent-file-review | 193 | y | — | — | Folded desc. |
| analytics-integration | 192 | y | — | — | Folded desc. |
| artifact-detection | 175 | y | — | — | Folded desc. Has 5 steps files. |
| brainstorming | 111 | y | — | — | Folded desc. 6 steps files. |
| brownfield-ingest | 373 | y | — | — | Broad-then-converge analysis protocol, 6 passes + deepening. |
| code-delivery | 231 | y | — | — | Folded desc. |
| competitive-monitoring | 175 | y | — | — | Folded desc. |
| consistency-validation | 288 | y | — | — | Folded desc. |
| convergence-check | 95 | y | true | Read, Write, Bash, Glob, Grep | 7-dimension convergence validation. |
| convergence-tracking | 164 | y | — | — | Folded desc. |
| create-architecture | 106 | y | true | Read, Write, Edit, Bash, AskUserQuestion | Sharded architecture docs from PRD+BCs. |
| create-brief | 108 | y | true | Read, Write, Edit, Bash, AskUserQuestion | Guided discovery product brief. |
| create-domain-spec | 110 | y | true | Read, Write, Edit, Bash, AskUserQuestion | Sharded L2 domain spec. |
| create-prd | 133 | y | true | Read, Write, Edit, Bash, AskUserQuestion | PRD with BCs from brief+domain. |
| create-story | 88 | y | true | Read, Write, Edit, Bash, AskUserQuestion | Single story spec with AC/tasks. |
| customer-feedback-ingestion | 238 | y | — | — | Folded desc. |
| deactivate | 22 | y | — | — | Reverse activate. |
| decompose-stories | 147 | y | true | Read, Write, Edit, Bash, AskUserQuestion | Epics, stories, wave schedule. |
| deliver-story | 130 | y | true | Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion | Per-story TDD delivery. |
| demo-recording | 297 | y | — | — | Folded desc. |
| design-drift-detection | 110 | y | — | — | Folded desc. |
| design-system-bootstrap | 148 | y | — | — | Folded desc. |
| discovery-engine | 315 | y | — | — | Folded desc. |
| disposition-pass | 170 | y | — | — | Pass 9 disposition rollup through corverax vision lens. |
| dtu-creation | 124 | y | — | — | Folded desc. |
| dtu-validate | 123 | y | true | Read, Write, Edit, Bash, Glob, Grep | DTU behavioral clones. |
| **excalidraw-export** | 46 | **NO** | — | — | **Missing YAML frontmatter — reference doc.** |
| factory-health | 125 | y | true | Bash, Read, Write | Validate/repair `.factory/` worktree. |
| factory-worktree-health | 205 | y | — | — | Folded desc. Duplicates concept of factory-health at 2x the LOC. |
| feature-mode-scoping-rules | 136 | y | — | — | Folded desc. |
| fix-pr-delivery | 175 | y | — | — | Folded desc. |
| formal-verify | 151 | y | true | Read, Write, Bash, Glob, Grep | Kani/fuzz/mutation/semgrep. |
| generate-pdf | 116 | y | true | (empty) | 1898 & Co branded PDF. allowed-tools declared empty. |
| guided-brief-creation | 151 | y | — | — | Folded desc. 6 steps files. |
| holdout-eval | 59 | y | true | (empty) | Holdout-evaluator agent spawn. |
| implementation-readiness | 145 | y | — | — | Folded desc. |
| intelligence-synthesis | 249 | y | — | — | Folded desc. |
| **jira** | 142 | **NO** | — | — | **Missing YAML frontmatter — reference doc.** |
| maintenance-sweep | 289 | y | — | — | Folded desc. |
| market-intelligence-assessment | 249 | y | — | — | Folded desc. |
| mode-decision-guide | 131 | y | — | — | Folded desc. |
| model-routing | 69 | y | — | — | Folded desc. |
| multi-repo-health | 22 | y | — | — | Scan `.worktrees/` for multi-repo. (smallest framed skill) |
| multi-repo-phase-0-synthesis | 164 | y | — | — | Folded desc. |
| multi-variant-design | 87 | y | — | — | Folded desc. |
| next-step | 39 | y | — | — | Reads STATE.md + active workflow. |
| perf-check | 120 | y | true | Read, Write, Bash, Glob, Grep | Benchmark/budget compliance. |
| phase-1-prd-revision | 59 | y | — | — | Folded desc. |
| phase-1d-adversarial-spec-review | 80 | y | — | — | Has 5 steps files. |
| phase-f1-delta-analysis | 198 | y | — | — | Folded desc. 7 steps files. |
| phase-f2-spec-evolution | 184 | y | — | — | Folded desc. 8 steps files. |
| phase-f3-incremental-stories | 147 | y | — | — | Folded desc. 8 steps files. |
| phase-f4-delta-implementation | 175 | y | — | — | Folded desc. 8 steps files. |
| phase-f5-scoped-adversarial | 186 | y | — | — | Folded desc. 8 steps files. |
| phase-f6-targeted-hardening | 171 | y | — | — | Folded desc. 7 steps files. |
| phase-f7-delta-convergence | 178 | y | — | — | Folded desc. 5 steps files. |
| planning-research | 66 | y | — | — | Folded desc. |
| post-feature-validation | 212 | y | — | — | Folded desc. |
| pr-create | 87 | y | true | Read, Bash, Glob, Grep | PR with BC traceability. |
| pr-review-triage | 89 | y | — | — | Folded desc. |
| quick-dev-routing | 98 | y | — | — | Folded desc. |
| record-demo | 96 | y | true | Read, Write, Bash, Glob | Playwright demo capture. |
| release | 183 | y | — | — | Folded desc. |
| repo-initialization | 476 | y | — | — | Folded desc. Largest skill file. |
| research | 64 | y | true | (empty) | Spawns research-agent with MCP tools. |
| research-cache-ops | 38 | y | — | — | Research cache wrapper. |
| responsive-validation | 127 | y | — | — | Folded desc. |
| run-phase | 60 | y | — | — | Execute phase from Lobster file. |
| sdk-generation | 383 | y | — | — | Folded desc. |
| semport-analyze | 124 | y | — | — | Semantic porting analysis. |
| session-review | 157 | y | — | — | Folded desc. dup of `agents/session-review.md`. |
| setup-env | 106 | y | true | Bash, Read, Write | Validate dev environment. |
| spec-drift | 76 | y | true | (empty) | Detect code vs spec drift. |
| spec-versioning | 160 | y | — | — | Folded desc. |
| state-update | 88 | y | — | Bash, Read, Edit | Update STATE.md; internal skill. Note: `disable-model-invocation` not set despite being "internal". |
| storybook-mcp-integration | 184 | y | — | — | Folded desc. |
| toolchain-provisioning | 404 | y | — | — | Folded desc. 3rd largest skill. |
| traceability-extension | 153 | y | — | — | Folded desc. |
| track-debt | 84 | y | true | Read, Write, Edit, Bash | Tech-debt register. |
| ui-completeness-check | 171 | y | — | — | Folded desc. |
| ui-quality-gate | 148 | y | — | — | Folded desc. |
| ux-heuristic-evaluation | 163 | y | — | — | Folded desc. |
| validate-brief | 133 | y | — | — | Folded desc. |
| validate-consistency | 84 | y | true | Read, Bash, Glob, Grep | Cross-file consistency. |
| validate-workflow | 46 | y | — | — | Lobster schema check. |
| wave-gate | 120 | y | true | Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion | Post-wave integration gate. |
| wave-scheduling | 68 | y | — | — | Folded desc. |
| wave-status | 22 | y | — | — | Read-only sprint-state query. |
| worktree-manage | 104 | y | true | Bash, Read | Story worktree lifecycle. |

Skill subdirectories with `steps/` (declared stepwise skills):

| Skill | steps files | Notes |
|---|---|---|
| artifact-detection | 5 | Each step file is ~5 LOC — stubs. |
| brainstorming | 6 | Each ~5 LOC — stubs. |
| guided-brief-creation | 6 | Each ~5 LOC — stubs. |
| phase-1d-adversarial-spec-review | 5 | |
| phase-f1-delta-analysis | 7 | |
| phase-f2-spec-evolution | 8 | |
| phase-f3-incremental-stories | 8 | |
| phase-f4-delta-implementation | 8 | |
| phase-f5-scoped-adversarial | 8 | |
| phase-f6-targeted-hardening | 7 | |
| phase-f7-delta-convergence | 5 | |

The non-phase skills' steps files are all 5 LOC each — essentially title+placeholder stubs. The phase-f* and phase-1d step files are fuller (20-35 LOC range).

### commands/

**No `commands/` directory exists** in `plugins/vsdd-factory/`. All user-facing invocations come through skill names directly (or via the marketplace slash-command synthesis).

### hooks/

`hooks/hooks.json` (50 LOC) registers 10 hook commands across 4 events:

| Event | Matcher | Command | Type | Timeout |
|---|---|---|---|---|
| PreToolUse | `Edit\|Write` | `brownfield-discipline.sh` | command | 5 |
| PreToolUse | `Edit\|Write` | `protect-vp.sh` | command | 5 |
| PreToolUse | `Edit\|Write` | `protect-bc.sh` | command | 5 |
| PreToolUse | `Edit\|Write` | `red-gate.sh` | command | 5 |
| PreToolUse | `Bash` | `verify-git-push.sh` | command | 10 |
| PreToolUse | `Bash` | `check-factory-commit.sh` | command | 5 |
| PostToolUse | `Edit\|Write` | `purity-check.sh` | command | 5 |
| PostToolUse | `Bash` | `regression-gate.sh` | command | 5 |
| SubagentStop | — | `handoff-validator.sh` | command | 5 |
| Stop | — | `session-learning.sh` | command | 5 |

Hook script files:

| File | LOC | Purpose |
|---|---|---|
| brownfield-discipline.sh | 34 | Blocks edits under `.reference/**`. |
| check-factory-commit.sh | 23 | Pre-Bash guard on `.factory/` commits. |
| handoff-validator.sh | 42 | SubagentStop — warn on empty/truncated subagent output. |
| protect-bc.sh | 36 | Blocks edits to green Behavioral Contracts. |
| protect-vp.sh | 44 | Blocks edits to green Verification Properties. |
| purity-check.sh | 70 | Flags side-effect patterns in pure-core paths. |
| red-gate.sh | 74 | Enforces TDD red-before-green when strict mode enabled. |
| regression-gate.sh | 65 | Records test outcomes; warns on pass→fail transitions. |
| session-learning.sh | 33 | Stop event — appends session-end markers. |
| verify-git-push.sh | 21 | PreToolUse Bash guard on git push. |

All hook scripts are referenced from `hooks.json`; no orphan scripts.

### templates/ (count: 108 files)

Total LOC across templates: ~8,579.

Top-level templates (Markdown + a few YAML/tape/TS files):

adversarial-finding-template.md, adversarial-review-index-template.md, adversarial-review-template.md, agents-md-template.md, architecture-feasibility-report-template.md, architecture-index-template.md, architecture-section-template.md, architecture-template.md, autonomy-config-template.yaml, behavioral-contract-template.md, code-review-template.md, consistency-report-template.md, conventions-template.md, convergence-report-template.md, cycle-manifest-template.md, delta-analysis-report-template.md, demo-ci-workflow-template.yaml, demo-evidence-report-template.md, demo-playwright-template.spec.ts, demo-tape-template.tape, discovery-config-template.yaml, discovery-report-template.md, domain-research-template.md, dtu-assessment-template.md, dtu-clone-spec-template.md, dtu-fidelity-report-template.md, epic-template.md, evaluation-index-template.md, evaluation-per-scenario-template.md, evaluation-summary-template.md, factory-project-state-template.md, factory-project-structure-template.md, feature-request-template.md, findings-tracker-template.md, fuzz-report-template.md, gene-transfusion-assessment-template.md, holdout-evaluation-report-template.md, holdout-scenario-template.md, idea-brief-template.md, L2-domain-spec-index-template.md, L2-domain-spec-section-template.md, L2-domain-spec-template.md, L4-verification-property-template.md, merge-config-template.yaml, module-criticality-template.md, pr-description-template.md, prd-supplement-error-taxonomy-template.md, prd-supplement-interface-definitions-template.md, prd-supplement-nfr-catalog-template.md, prd-supplement-test-vectors-template.md, prd-template.md, product-brief-template.md, project-context-template.md, project-justfile-template (no extension), project-manifest-template.yaml, recovered-architecture-template.md, red-gate-log-template.md, release-notes-template.md, review-findings-template.md, security-review-template.md, security-scan-report-template.md, session-review-template.md, skill-delegation-template.md, skill-execution-template.md, spec-changelog-template.md, state-template.md, story-template.md, sweep-report-template.md, tech-debt-register-template.md, traceability-matrix-template.md, ui-traceability-template.yaml, verification-gap-analysis-template.md, vp-withdrawal-template.md, wave-schedule-template.md.

Subdirectories:

- `templates/adversary-prompt-templates/` — `phase-1d-spec-review.md`, `phase-2-story-review.md`, `phase-4-code-review.md` (3 files).
- `templates/design-system/components/` — `component-registry.yaml` + 11 component contract YAMLs (alert, button, card, data-table, dropdown, form-field, list, modal, navigation, tabs, toast).
- `templates/design-system/constraints.yaml`.
- `templates/design-system/patterns/` — 3 pattern YAMLs (form, layout, navigation).
- `templates/design-system/tokens/` — 7 token JSON files (accessibility, colors, elevation, motion, sizing, spacing, typography).
- `templates/ui-quality/` — 4 templates (completeness, gate, heuristic-evaluation, responsive reports).
- `templates/ux-spec-*.md` — flow, index, screen, main (4 files).

One file lacks extension: `templates/project-justfile-template`.

### bin/ (4 helpers)

| File | LOC | Purpose |
|---|---|---|
| lobster-parse | 51 | yq+jq wrapper to emit `.lobster` workflows as JSON. |
| multi-repo-scan | 56 | Detects multi-repo layouts under `.worktrees/`. |
| research-cache | 81 | SHA-keyed disk cache for Perplexity/Context7 results. |
| wave-state | 63 | Read-only query of `.factory/stories/sprint-state.yaml`. |

All shell scripts; no binary compiled tools.

### lib/

No `lib/` directory.

### tests/ (3 files)

| File | LOC | Purpose |
|---|---|---|
| bin.bats | 135 | 13 TAP tests for bin helpers. |
| hooks.bats | 217 | 28 TAP tests for hooks allow/block paths. |
| run-all.sh | 37 | Runner: syntax checks + hook tests + bin tests. |

Total TAP tests: 41 (matches CHANGELOG 0.8.0).

### fixtures/

`fixtures/smoke-project/` — minimal Rust crate:
- `Cargo.toml` (8 LOC)
- `src/lib.rs` (13 LOC)

### docs/ (5 files)

| File | LOC | Purpose |
|---|---|---|
| AGENT-SOUL.md | 183 | Shared agent principles. |
| CONVERGENCE.md | 506 | Convergence methodology. |
| FACTORY.md | 920 | Factory overview — largest doc. |
| not-portable.md | 56 | Extensions that can't port to Claude Code. |
| VSDD.md | 284 | VSDD methodology. |

### rules/ (8 files)

`_index.md` + 7 rule files: bash.md (120), factory-protocol.md (81), git-commits.md (70), rust.md (57), spec-format.md (184), story-completeness.md (127), worktree-protocol.md (77). Mirror of Corverax's `.claude/rules/`.

### workflows/ (15 `.lobster` files)

Mode workflows (8): brownfield (363), code-delivery (386), discovery (435), feature (1470 — largest file in repo), greenfield (1223), maintenance (425), multi-repo (720), planning (298).

Phase sub-workflows (7) under `workflows/phases/`: phase-0-codebase-ingestion (272), phase-1-spec-crystallization (72), phase-3-test-first-implementation (29), phase-3.5-holdout-evaluation (38), phase-4-adversarial-refinement (54), phase-5-formal-hardening (30), phase-6-convergence (62).

CHANGELOG says "all 15 .lobster files shipped" — 8 mode + 7 phase = 15. Matches.

## Cross-cutting observations

- **Stub agent descriptions**: 22 of 33 top-level agents carry the boilerplate `VSDD factory agent: <name>` description. Wave 2 CHANGELOG says they were "ported … with synthesized frontmatter" — the synthesis was mechanical and did not produce meaningful descriptions. Bodies are present and sized 71–440 LOC, so these are not empty, just badly described in frontmatter.
- **Missing skill frontmatter**: `skills/excalidraw-export/SKILL.md` and `skills/jira/SKILL.md` lack YAML frontmatter. They are reference docs rather than invokable skills but still reside in the skills tree. A marketplace loader expecting `name:`/`description:` will skip or error.
- **Orchestrator sub-files without frontmatter**: 9 files in `agents/orchestrator/` (brownfield-sequence, discovery-sequence, feature-sequence, greenfield-sequence, HEARTBEAT, maintenance-sequence, multi-repo, per-story-delivery, steady-state) lack frontmatter. They are treated as "included data" by orchestrator.md but a scanner will flag them as malformed agents. `HEARTBEAT.md` uppercase naming is inconsistent with sibling files.
- **Name collisions (not file collisions)**: `agents/session-review.md` vs `skills/session-review/` — separate artifact kinds with the same name; may confuse cross-references.
- **No `commands/` directory**: README and CHANGELOG never mention a commands tree and none exists. Skills function as commands.
- **Step-file stubs**: Several skills declare `steps/` subdirectories whose files are 5 LOC placeholder stubs (artifact-detection, brainstorming, guided-brief-creation). The phase-f* series has fuller step files. The stub steps look like drafts never completed.
- **`state-update` skill** describes itself as "Internal skill called by other skills — not invoked directly by users" yet does not set `disable-model-invocation: true`. By contrast comparable internal-ish skills (`holdout-eval`, `research`) do set it. Minor inconsistency.
- **`generate-pdf` / `holdout-eval` / `research` / `spec-drift`** declare `allowed-tools:` with no value — parsed as empty tool list. May or may not be intended (may cause tool access denial in strict loaders).
- **Version consistency**: `plugin.json` says `0.8.0`, CHANGELOG top entry is `0.8.0 — Wave 7`. Marketplace.json does not carry a version; no drift to report.
- **Agent count consistency**: CHANGELOG 0.2.0 says "Total agents: 34 (33 dark-factory + 1 Corverax addition)" — the current `agents/` dir has 33 top-level files plus the orchestrator directory. Counting orchestrator.md as the 34th matches.
- **Skill count consistency**: CHANGELOG 0.8.0 footer implies 91 skills (Wave 6 says 88, Wave 7 adds nothing; actual count is 91). CHANGELOG does not explicitly restate the total at 0.8.0, but Wave 7 added 3 bin-wrapper skills in 0.7.0 (research-cache-ops, wave-status, multi-repo-health) which brings 88 + 3 = 91. Matches directory count.
- **No duplicated file content detected** across locations (not verified byte-for-byte, but no name collisions).
- **Rules are a copy** of the Corverax rules set (bash.md, factory-protocol.md, etc. — same files this conversation loaded from the parent project). Intentional per 0.1.0 extraction.
- **Templates referenced but not verified**: Several skill `## Templates` sections cite template paths; no stale-reference audit performed here beyond existence at the `templates/` root. The design-system JSON tokens and YAML contracts are well organized.
- **CI workflow**: `.github/workflows/plugin-validation.yml` runs bats + JSON validation on push/PR to `main`. The repo's current branch is `develop` per git status — PRs will target `main` via marketplace conventions, not the `develop`-based flow described in CLAUDE.md.

## Size summary

- **Total tracked files** (excluding `.git/`, `.factory/`, `.reference/`): **364**
- **Total LOC**: **42,390**
- **Largest 10 files by LOC**:
  1. `plugins/vsdd-factory/workflows/feature.lobster` — 1470
  2. `plugins/vsdd-factory/workflows/greenfield.lobster` — 1223
  3. `plugins/vsdd-factory/docs/FACTORY.md` — 920
  4. `plugins/vsdd-factory/workflows/multi-repo.lobster` — 720
  5. `plugins/vsdd-factory/docs/CONVERGENCE.md` — 506
  6. `plugins/vsdd-factory/skills/repo-initialization/SKILL.md` — 476
  7. `plugins/vsdd-factory/agents/architect.md` — 440
  8. `plugins/vsdd-factory/workflows/discovery.lobster` — 435
  9. `plugins/vsdd-factory/workflows/maintenance.lobster` — 425
  10. `plugins/vsdd-factory/agents/codebase-analyzer.md` — 419
- **Smallest / near-empty files**: ~15 files at 5 LOC each — all stepwise step files under `skills/artifact-detection/steps/`, `skills/brainstorming/steps/`, `skills/guided-brief-creation/steps/`. No zero-byte files detected.
- **Smallest JSON/manifest**: `plugins/vsdd-factory/.claude-plugin/plugin.json` (12 LOC), `.claude-plugin/marketplace.json` (18 LOC).
