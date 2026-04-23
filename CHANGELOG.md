# Changelog

## 0.56.0 — Observability Phase 1: emit-event safety scaffold

First increment of the local-first observability plan. Ships the foundation
without instrumenting any hooks yet — the guarantee being proven in this
release is that **calling the emitter cannot break vsdd-factory** under any
failure mode.

### Added

- **`bin/emit-event`** — failure-tolerant structured event emitter. Writes
  one JSON event per invocation to `.factory/logs/events-YYYY-MM-DD.jsonl`.
  Hard guarantees:
  - Exits 0 on every path (missing `jq`, missing disk, read-only log dir,
    malformed args, disk full — all silent drops).
  - No stdout/stderr on success.
  - POSIX-portable date format (macOS, Linux, WSL, git-bash).
  - Atomic append (relies on POSIX PIPE_BUF guarantee, no `flock`).
  - `VSDD_TELEMETRY=off` kill switch (line-1 short-circuit).
  - `VSDD_LOG_DIR=<path>` override (default `.factory/logs`).
  - Args are `key=value` pairs; values may contain any characters (spaces,
    quotes, newlines, `=`); jq handles escaping.
  - Auto-adds `ts` (ISO-8601 w/ tz) and `schema_version` (integer) fields.

### Added (tests)

- `tests/emit-event.bats` — 35 tests covering structural checks, exit-code
  guarantees (garbage args, binary data, 10KB values, 50 field pairs), kill
  switch, graceful degradation (missing jq, readonly log dir, unwritable
  parent, auto-creation of deep dirs), emission correctness (valid JSON,
  ISO-8601 timestamp, schema version, field preservation, quote/backslash
  escaping, `=` in values, dotted keys stay flat), and append semantics.
- 827 tests across 22 suites, 0 failures.

### Notes

- Nothing currently calls `emit-event`. Phase 2 will instrument the 21
  existing hooks to emit events at decision points.
- Logs land under `.factory/logs/` which is already gitignored on `main`.
- Target shells: bash on macOS, Linux, WSL, and git-bash. Native PowerShell
  on Windows is out of scope for Phase 1.

## 0.55.0 — Harden destructive-command-guard + add protect-secrets hook

### Added

- **`destructive-command-guard.sh` — Tier 1 guards** (new):
  - Catastrophic root targets: `rm -rf /`, `/*`, `~`, `~/`, `$HOME`, `*`, `.*` (and flag-order variants)
  - Clobbering redirects to source-of-truth files: `> STATE.md`, `: > STATE.md`, `truncate -s 0`, `cp /dev/null`. `>>` (append) and `sed -i` remain allowed.
  - `find … -delete` and `find … -exec rm` on `.factory`, `src/`, `tests/`
  - `git stash drop` / `git stash clear`
  - `git branch -D main|master|develop`
  - `git filter-branch` / `git filter-repo`
- **`destructive-command-guard.sh` — Tier 2 guards** (new):
  - `--no-verify` on `git commit|merge|rebase|cherry-pick|am`, plus `--no-gpg-sign` on commit
  - `gh repo delete`, `gh release delete`, `gh pr close`, `gh issue delete`
  - `git reflog expire --expire=now` and `git gc --prune=now`
  - `git worktree remove --force` outside `.worktrees/`
  - Pipe-to-interpreter RCE pattern: `curl|wget|fetch ... | bash|sh|zsh|python|perl|ruby`
- **`destructive-command-guard.sh` — Tier 3 guards** (new):
  - Recursive `chmod -R` / `chown -R` / `--recursive` on `.factory`, `src/`, `tests/`, `.git/`
- **`protect-secrets.sh`** (new PreToolUse hook, matches Bash and Read):
  - Blocks `Read` of `.env`, `.env.*`, `.envrc` (allows `.env.example` / `.sample` / `.template`)
  - Blocks `cat|less|more|head|tail|bat|xxd|od|strings|grep|awk|sed` on real .env files
  - Blocks `cp|mv|rsync|scp` when the **source** is a real .env (allows template-bootstrap like `cp .env.example .env`)
  - Blocks `tar|zip` that archive real .env files
  - Blocks `echo|printf` of secret-shaped env vars (`$*_TOKEN`, `$*_SECRET`, `$*_PASSWORD`, `$*_API_KEY`, `$*_PRIVATE_KEY`, `$*_ACCESS_KEY`, `$*_CREDENTIAL`, `$*_AUTH`)
  - Blocks `env|printenv|set | grep` for secret-shaped names
  - Allows existence checks (`ls .env*`, `test -f .env`, `[ -f .env ]`) and sourcing (`source .env`, `. .env`)

### Fixed

- **`destructive-command-guard.sh` — bare `.factory` match** — `rm -rf .factory` (no trailing slash) previously slipped through the substring check; now caught via regex that handles end-of-command, slash, or separator boundary.
- **`destructive-command-guard.sh` — long-form recursive flag** — `rm --recursive` is now recognized alongside `-r`, `-R`, `-rf`, `-fr`, `-Rf`, `-fR`.

### Added (tests)

- 41 new tests in `tests/destructive-guard.bats` (catastrophic roots, SoT redirection, find-delete, git-stash-drop, git-branch-D, history rewriters, --no-verify, gh destructive, curl|bash, recursive chmod/chown) — 97 tests total in suite
- `tests/protect-secrets.bats` (new) — 55 tests covering Read + Bash paths, cp source-vs-destination semantics, secret env echo/grep
- `run-all.sh` wires the new protect-secrets suite
- 792 tests across 21 suites, 0 failures

## 0.54.0 — Unify red-gate-log path + factory path root guard

### Fixed

- **Red-gate-log path unified** — three conflicting paths (`.factory/stories/`, `.factory/phase-f4-implementation/`, `.factory/cycles/**/implementation/`) consolidated to `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md` across deliver-story skill, step files, phase-3 guide, per-story-delivery orchestrator, and phase-f4 skill.

### Added

- **`validate-factory-path-root.sh`** (PostToolUse on Write/Edit) — blocks `.factory/` writes that resolve inside `.worktrees/STORY-NNN/.factory/` instead of the project root. Catches agents using relative paths from inside story worktrees. Diagnostic shows the expected absolute path.
- 8 new BATS tests for factory path root validation
- 691 tests across 21 suites, 0 failures

## 0.53.0 — Wave gate completeness enforcement + GATE_CHECK telemetry

### Added

- **`validate-wave-gate-completeness.sh`** (PostToolUse on Write to wave-state.yaml) — blocks marking gate_status: passed unless the gate report contains evidence of all 6 gates (test suite, DTU validation, adversarial review, demo evidence, holdout evaluation, state update). Accepts both human-readable "Gate N" headings and structured GATE_CHECK telemetry lines.
- **GATE_CHECK telemetry** added to wave-gate skill — structured `GATE_CHECK: gate=N name=<name> status=<pass|fail|skip> note=<reason>` lines emitted after each gate, enabling mechanical validation by the completeness hook.
- 10 new BATS tests for gate completeness validation
- 682 tests across 21 suites, 0 failures

## 0.52.0 — Wave gate enforcement + PR lifecycle hooks + AI attribution blocker

### Added

- **Wave gate enforcement** (3 hooks):
  - `validate-wave-gate-prerequisite.sh` (PreToolUse on Agent) — blocks Wave N+1 worker dispatch if Wave N gate not passed/deferred
  - `update-wave-state-on-merge.sh` (SubagentStop on pr-manager) — auto-updates wave-state.yaml when stories merge, flips gate_status to pending when all wave stories merged
  - `warn-pending-wave-gate.sh` (Stop) — session-end reminder for pending gates
  - `wave-state-template.yaml` — structured wave lifecycle tracker template

- **PR lifecycle enforcement** (3 hooks):
  - `validate-pr-description-completeness.sh` (PostToolUse on Write) — blocks incomplete PR descriptions missing required sections or containing unresolved template placeholders
  - `validate-pr-review-posted.sh` (SubagentStop on pr-reviewer) — blocks if pr-reviewer didn't write pr-review.md or used `gh pr comment` instead of `gh pr review`
  - `validate-pr-merge-prerequisites.sh` (PreToolUse on Agent) — blocks merge dispatch if evidence trail (pr-description.md, pr-review.md, security-review.md) is missing

- **`block-ai-attribution.sh`** (PreToolUse on Bash) — blocks git commits containing Co-Authored-By: Claude/GPT/Gemini, "Generated with Claude Code", or noreply@anthropic.com patterns

- 80 new BATS tests across 2 test files (37 wave-gate + 43 PR lifecycle)
- 672 tests across 21 suites, 0 failures

## 0.51.0 — PR manager completion guard hook (FM4 detection)

### Added

- **`pr-manager-completion-guard.sh`** — SubagentStop hook that detects FM4 (pr-manager exiting before completing all 9 lifecycle steps). Blocks the stop via exit 2 and injects a continuation prompt with the next step hint. Accepts 8+ STEP_COMPLETE emissions as complete (allows one N/A skip). Passes through legitimate BLOCKED exits.
- 22 new BATS tests covering: scope filtering, full completion, partial exits at steps 0/1/3/5/7/8, BLOCKED passthrough, agent name matching variants, edge cases
- 592 tests across 19 suites, 0 failures

## 0.50.0 — Replace sessions_spawn with Agent tool + pr-manager hardening

### Fixed

- **pr-manager agent** — replaced all 13 `sessions_spawn` calls with `Agent(subagent_type=...)` syntax. Added COORDINATOR RULE meta-instruction, explicit continuation clauses after every sub-agent spawn (prevents premature exit on reviewer APPROVE), merge pre-authorization (`AUTHORIZE_MERGE=yes`), and STEP_COMPLETE instrumentation for all 9 steps.
- **sessions_spawn removed project-wide** — all references across 22 files (orchestrator, FACTORY.md, pr-reviewer, security-reviewer, code-delivery, 12 skill boilerplate lines, templates) replaced with Agent tool syntax. Zero `sessions_spawn` references remain.

### Changed

- FACTORY.md Sub-Agent Delegation Rule rewritten for Agent tool pattern
- Orchestrator delegation section updated with Agent tool examples
- agent-file-review audit rule updated to reflect pr-manager as legitimate Agent tool user

## 0.49.0 — Per-story demo evidence scoping (POL-010)

### Added

- **POL-010: `demo_evidence_story_scoped`** — all demo-recorder output must live under `docs/demo-evidence/<STORY-ID>/`, preventing evidence-report.md collisions across stories
- **`validate-demo-evidence-story-scoped.sh`** — PostToolUse lint hook that blocks flat-level file creation at `docs/demo-evidence/*.md`
- Policy entry added to `policies-template.yaml` (id: 10)

### Changed

- **demo-recorder agent** — output root changed from `docs/demo-evidence/` to `docs/demo-evidence/<STORY-ID>/`, constraint added forbidding flat-level files
- **per-story-delivery orchestrator** — task prompt updated with `<STORY-ID>` subfolder and collision-prevention explanation
- **deliver-story skill + steps** — all path references updated to per-story scoped
- **code-delivery skill** — paths, gates, and tree diagram updated
- **pr-manager, pr-reviewer agents** — evidence gate paths updated
- **record-demo skill** — output paths updated to per-story scoped
- **demo-recording skill** — internal factory paths unchanged (`.factory/demo-evidence/`)
- **Templates** — demo-tape-template.tape, demo-playwright-template.spec.ts output paths updated with `{STORY_ID}` placeholder
- **Workflows** — greenfield.lobster, code-delivery.lobster commit paths updated
- **Docs** — phase-3-tdd-delivery.md, FACTORY.md, configuration.md, factory-protocol.md updated
- Removed Tally MCP dependency — all 27 references across 18 files removed. Traceability chain simplified: `BC → VP → test → src → ADV → KANI` (TALLY-xxx link was redundant)

## 0.48.0 — Semantic drift lint hooks

### Added

- **`validate-state-index-status-coherence.sh`** — PostToolUse hook detecting drift between STATE.md `convergence_status:` frontmatter and cycle INDEX.md `**Status:**` lines. Normalizes underscores/hyphens and case for comparison, trims trailing descriptions after em-dash. Warning only (exit 1) — state transitions often land at different commits within the same burst.
- **`validate-anchor-capabilities-union.sh`** — PostToolUse hook enforcing the invariant that a story's `anchor_capabilities:` must equal the sorted union of `capability:` fields across all referenced BCs. Handles dual-anchor BCs with CSV capabilities, warns (but doesn't block) on missing BC files. Block on mismatch (exit 2) with BC→CAP mapping diagnostic.
- 22 new BATS tests covering both hooks (pass, fail, edge cases, wiring)

## 0.47.0 — Glob/directory expansion for input-hash + docs update

### Added

- **Glob/directory expansion** in `compute-input-hash` — artifacts can now use `inputs: [behavioral-contracts/**]` (recursive glob) or `inputs: [domain-spec/]` (directory) instead of listing every file. Expansion uses `find` + `LC_ALL=C sort` for deterministic ordering.
- 4 new glob tests (directory expansion, ** wildcard, deterministic hash, missing glob directory)

### Changed

- README updated: 116 skills, 108 commands, 26 hooks, 99 templates, 538 tests across 17 suites. All per-suite test counts updated. Contributing section: 9 Iron Laws, 80 Red Flags.
- hooks-reference.md: updated from 19 to 26 hooks with 8 new entries
- id-reference.md: ADV producer "Phase 1d/2/3/5", FIX producer "Phase 5/6/7", enforcement table expanded with 8 new hooks
- 538 tests across 17 suites

## 0.46.1 — Strip .factory/ prefix from input paths

### Fixed

- **compute-input-hash** — inputs with `.factory/` prefix (e.g., `inputs: [.factory/specs/prd.md]`) resolved to double-nested paths that don't exist. Now strips the prefix when FACTORY_ROOT is known, falls back to original if stripping fails. 534 tests.

## 0.46.0 — Input resolution mode + partial hash safety

### Added

- **`compute-input-hash --resolve`** — per-file and batch mode to check if all inputs are resolvable, naming specific missing files
- **`--scan <dir> --resolve`** — batch find all artifacts with unresolvable inputs (TOTAL/RESOLVABLE/UNRESOLVABLE)

### Fixed

- **Refuse partial hashing** — binary no longer silently hashes incomplete input sets. Missing inputs produce PARTIAL warning; hash/update/print all refuse to proceed. Prevents false MATCH from two partial hashes agreeing.
- **Named missing files** — all warnings now list the specific missing input file names (was just a count)
- **Warn in all modes** — missing input warnings fire in check, update, and print modes (was print-only)

### Changed

- `check-input-drift` skill: Step 2 (Resolve inputs) is now MANDATORY before interpreting drift results. Documents common causes (not yet produced, renamed, wrong path, deleted) with fix actions. Steps renumbered 1-6.
- 532 tests across 17 suites

## 0.45.1 — Fix absolute vs relative path bug in compute-input-hash

### Fixed

- **compute-input-hash** — absolute paths to `--scan` produced 62 false-positive STALE results vs 2 correct with relative paths. Root cause: `FACTORY_ROOT` pattern match behaved differently with relative vs absolute file paths. Fix: canonicalize `FILE` to absolute via `cd+pwd` before resolution logic runs. Both paths now produce identical results.
- 2 regression tests added (path identity + different-cwd scan)
- 527 tests across 17 suites

## 0.45.0 — Cluster drift triage for input-hash scanning

### Added

- **Step 5: Triage cluster drift** in `check-input-drift` skill — recognizes patterns where multiple related artifacts drift simultaneously (all domain-spec shards, all BCs for a subsystem, PRD + supplements) and routes to producing agents for content review before hash bumping
- Dispatch table mapping 7 artifact types to producing agents (business-analyst, product-owner, architect, story-writer)
- Task template for dispatched agents with content review instructions
- Common Cluster Drift Patterns reference table (7 patterns)
- Warning gate: Step 4 `--update` on >3 files requires Step 5 triage first
- 6 new BATS tests — 526 tests across 17 suites

## 0.44.0 — Batch input-hash drift scanning

### Added

- **`compute-input-hash --scan <dir>`** — batch drift detection that walks all `.factory/**/*.md` artifacts in one command. Avoids Claude Code's multi-line shell auto-backgrounding issue that killed inline bash loops before output flushed.
- **`compute-input-hash --scan <dir> --update`** — batch remediation of all stale hashes
- Summary line output: `TOTAL=N MATCH=N STALE=N UNCOMPUTED=N NOINPUT=N UPDATED=N UPDATE_FAILED=N`
- 14 new BATS tests (input-hash-scan.bats) — 520 tests across 17 suites

### Changed

- `check-input-drift` skill updated: uses `--scan` instead of manual iteration, documents both batch and per-file modes
- All existing per-file invocations (`<file>`, `<file> --update`, `<file> --check`) unchanged

## 0.43.1 — Shellcheck gate + post-release fixes

### Added

- **Shellcheck gate test** in hook-robustness.bats — runs shellcheck on all hooks and bin scripts, catches SC2001/SC2012 before release (506 tests)

### Fixed

- `validate-index-self-reference.sh` — sed → parameter expansion (shellcheck SC2001)
- `validate-table-cell-count.sh` — sed → parameter expansion (shellcheck SC2001)

## 0.43.0 — Corpus lint hooks + comprehensive glossary

### Added

- **4 new corpus lint hooks** targeting recurring adversarial finding defect classes:
  - `validate-table-cell-count.sh` — blocks markdown table rows with mismatched pipe count (caught 134 BCs in one Prism pass)
  - `validate-changelog-monotonicity.sh` — blocks duplicate versions, ascending order, date inversions, frontmatter mismatch
  - `validate-state-pin-freshness.sh` — blocks STATE.md version pins that don't match actual artifact versions
  - `validate-index-self-reference.sh` — warns when INDEX/burst-log edited without current burst row
- Extended `validate-input-hash.sh` — now blocks non-7-char hashes and non-lowercase-hex characters (caught 23 files in one pass)
- 45 new BATS tests across corpus-lint.bats (35) and edge cases (10) — 505 total across 16 suites
- **Glossary expanded to 110 entries** — added all spec levels (L1-L4), all 8 phases (0-7), architecture concepts (Purity Boundary, Pure Core, Effectful Shell), convergence metrics (Confidence Score, Hallucination Fingerprinting, MVR, Satisfaction Score), operational concepts (Burst, Per-Story Delivery Flow, Model Tier), and verification tools (Kani, Semgrep)

### Fixed

- `validate-input-hash.sh` null handling — `null` is not a valid placeholder (warns), only `[live-state]` and `[pending-recompute]` silently skip
- 26 hooks total, 505 tests across 16 suites

## 0.42.0 — Iron Laws and Red Flags for all critical pipeline skills

### Added

- **5 new Iron Laws** with Red Flags tables for critical skills:
  - `decompose-stories`: NO STORY WITHOUT BC TRACEABILITY FIRST (8 flags)
  - `holdout-eval`: NO HOLDOUT EVALUATION WITHOUT INFORMATION ASYMMETRY FIRST (8 flags)
  - `convergence-check`: NO RELEASE WITHOUT ALL SEVEN DIMENSIONS CONVERGED (8 flags)
  - `formal-verify`: NO HARDENING SIGN-OFF WITHOUT ALL PROOF HARNESSES PASSING (8 flags)
  - `create-architecture`: NO ARCHITECTURE WITHOUT VERIFICATION FEASIBILITY ASSESSMENT (8 flags)
- Total: 9 Iron Laws, 80 Red Flag entries across the pipeline

### Fixed

- **Minimum pass count inconsistency** — adversarial-review SKILL.md and adversary.md both said "Minimum 2 passes", now correctly say "Minimum 3 clean passes" matching CONVERGENCE.md and all convergence hooks
- **Maximum pass escalation** — "Maximum 5" → "Maximum 10" matching convergence loop `max_iterations` in all lobster files

## 0.41.3 — Restore adversary agent model tier

### Fixed

- **adversary.md** — `model: adversary` (invalid tier) restored to `model: opus`

## 0.41.2 — Cycle file templates for state management

### Added

- **5 new templates** for cycle-scoped state files: `burst-log-template.md`, `convergence-trajectory-template.md`, `session-checkpoints-template.md`, `lessons-template.md`, `blocking-issues-resolved-template.md`
- Template references added to `compact-state` skill and `state-manager` agent (99 templates total)

## 0.41.1 — Post-release bugfixes

### Fixed

- **Command files** — `check-state-health.md` and `compact-state.md` had empty body text after frontmatter, causing `cache_control cannot be set for empty text blocks` API error when invoked
- **convergence-tracker.sh** — replaced `ls` with `find` to satisfy shellcheck SC2012

## 0.41.0 — Adversarial review hardening + state management + convergence enforcement

### Added

- **Per-story adversarial review** — full convergence loop in `code-delivery.lobster` after implementation, before demo recording. Scoped to story diff with information asymmetry walls
- **Phase 2 adversarial convergence loop** — added to `phase-2-story-decomposition.lobster` (was missing entirely)
- **Wave-level adversarial convergence** — upgraded from single-pass to full convergence loop in both `greenfield.lobster` and `feature.lobster`
- **`validate-novelty-assessment.sh` hook** (21st) — blocks adversarial review files missing the structured Novelty Assessment section (pass, novelty score, trajectory, verdict)
- **`convergence-tracker.sh` hook** (22nd) — lightweight convergence rule enforcement: trajectory monotonicity, minimum 3 clean passes, novelty ≤ 0.15 for CONVERGENCE_REACHED, zero-findings first pass warning
- **`validate-state-size.sh` hook** (20th) — warns at 200 lines, blocks at 500 lines (allows compaction writes that reduce size)
- **`check-state-health` skill + command** — 7-check diagnostic (existence, frontmatter, size, phase numbering, structure, content routing, convergence counter)
- **`compact-state` skill + command** — extracts historical content from bloated STATE.md to cycle-scoped files (burst-log, convergence-trajectory, session-checkpoints, lessons, blocking-issues-resolved)
- **Content routing rules** in state-manager agent — explicit routing tables and anti-patterns preventing STATE.md bloat
- **STATE.md health check** (check 8) wired into `factory-health` — catches bloat at every session start
- **CONVERGENCE.md Dimensions 6-7** — Visual Convergence (demo evidence) and Documentation Convergence (CHANGELOG, README, API docs)
- **Adversarial review template** updated with structured Novelty Assessment section (7 required fields)
- **State template v2.0** — Phase 0 included, size budget comment, Historical Content section, session checkpoint limit
- 68 new BATS tests across 3 new suites (state-health: 33, novelty-assessment: 18, convergence-tracker: 17)

### Fixed

- **Phase 1d step files** — rewrote 5 empty stubs into full alphabetic step files (a-e) with `_shared-context.md` containing Iron Law, Red Flags, convergence protocol, persistence protocol
- **Phase F5 step files** — enriched 8 thin files into full alphabetic step files (a-h) with `_shared-context.md` containing DF-025 walls, DF-013 multi-repo, holdout regression, security touchpoint
- **Orchestrator mandatory checks** — corrected all phase references (1d, 2, 3, 4, 5), added per-story and wave-level adversarial as mandatory steps
- **Stale phase references** swept across ~30 files: agents (7), docs (VSDD.md, FACTORY.md), templates (14), skills (4), lobster files (5), orchestrator sequences
- **CONVERGENCE.md autonomy config** — fixed phase numbering (3.5→4, 4→5, 5→6, 6→7)
- **adversary.md** — `model: opus` → `model: adversary`, "Phase 4" → "Phase 5"
- **Prompt template** renamed `phase-4-code-review.md` → `phase-5-code-review.md`
- **Template frontmatter** — 4 adversarial templates updated from `phase: 1d|2|4` to `1d|2|5`

### Changed

- 460 tests across 15 suites (was 392 across 12)
- 22 hooks (was 19)
- 116 skills (was 112), 108 commands (was 104)

## 0.40.0 — Full pipeline path verification + workflow mode documentation

### Added

- **docs/guide/pipeline-paths.md** — definitive routing reference documenting all 14 paths through the factory with step traces, gate criteria, Mermaid routing diagram, and transition logic
- **docs/guide/workflow-modes.md** — all 8 workflow modes with routing diagram, per-mode documentation, mode detection logic, greenfield→multi-repo and brownfield→multi-repo transition explanations
- **Greenfield→multi-repo transition** formalized as 4 lobster steps: topology-check, human-confirmation, repo-transition, state-migration
- **Brownfield→multi-repo auto hand-off** — `multi-repo-handoff-check` step detects if greenfield sub-workflow triggered multi-repo transition

### Fixed

- `wave-integration-gate` schema: `type: skill` → `type: compound` in greenfield + feature lobster (had no `skill:` path)
- Multi-repo `wave-0`/`wave-1` steps parameterized with `${repo.name}` and `${repo.mode}` (was hardcoded `api-server`/`frontend`)
- Brownfield `depends_on` on conditional steps: uses `wait_for_optional` for `semport-validation-gate` and `brownfield-design-system-approval`
- Maintenance description: updated from "9 sweep types" to "11 sweep types" (sweeps 10-11 were added but description not updated)
- Planning + multi-repo added to orchestrator Mode Detection and Reference Files table
- Mode-decision-guide: added multi-repo, maintenance, discovery to decision table + flowchart
- All "Phases 1-6" references updated to "Phases 1-7"

### Path Trace Results

14 paths traced. Zero broken file references. All `skill:` paths resolve, all `agent:` names resolve, all `sub_workflow:` references resolve.

## 0.39.0 — Workflow standardization + phase renumbering

### Changed

- **Phase renumbering**: Eliminated fractional Phase 3.5. Pipeline is now 8 phases (0-7): Codebase Ingestion, Spec Crystallization, Story Decomposition, TDD Implementation, Holdout Evaluation, Adversarial Refinement, Formal Hardening, Convergence.
- **Step decomposition**: All phases decomposed into step files with `_shared-context.md` and state-manager crash recovery. 30 step files across 5 work skills.
- **Three-layer architecture**: Top-level lobster → phase entry-point skill → phase sub-workflow lobster → step files.
- **Pure alphabetic step naming**: All step IDs use `step-a-`, `step-b-`, etc. No numeric or sub-step IDs.

### Added

- **8 phase entry-point skills** (`phase-0-codebase-ingestion` through `phase-7-convergence`) bridging top-level lobsters to phase sub-workflows
- **`phase-2-story-decomposition.lobster`** — previously missing phase sub-workflow
- **`rules/step-decomposition.md`** — comprehensive standard for phase numbering, step naming, workflow structure
- **8 structural BATS tests** for lobster path resolution, phase entry-point skills, no old numbering, shared-context, alphabetic naming
- Renamed `docs/guide/phase-4-adversarial-refinement.md` → `phase-5-adversarial-refinement.md`
- Renamed `docs/guide/phase-6-convergence-release.md` → `phase-7-convergence-release.md`

### Fixed

- **20 broken skill paths** in lobster workflow files (referenced non-existent phase skill directories)
- **~150 old phase number references** across 47 files
- `semport/SKILL.md` → `semport-analyze/SKILL.md` in brownfield.lobster
- README Mermaid diagram now shows all 8 phases
- Skills: 104 → 112, Workflows: 15 → 16, Rules: 8 → 9, Tests: 384 → 392

## 0.38.1 — Fix compute-input-hash path resolution

### Fixed

- **compute-input-hash** now searches `.factory/phase-0-ingestion/`, `.factory/stories/`, and `.factory/holdout-scenarios/` when resolving input paths. Previously only searched artifact dir, parent, specs/, and .factory/ root — causing silent partial hashes for artifacts referencing brownfield ingestion outputs, stories, or holdout scenarios.

### Added

- 3 new BATS tests for path resolution: phase-0-ingestion, stories, holdout-scenarios (Tests: 381 → 384)

## 0.38.0 — Orchestrator audit + workflow drift checks

### Added

- **Input-hash drift checks in lobster workflows** — phase-1, phase-3, phase-6 now include `input-hash-drift-check` step before gates/human approval
- **Orchestrator mandatory step** — input-hash drift check required at phase gates 1, 2, 3, 6
- **5 cross-cutting skills** added to orchestrator reference: `validate-template-compliance`, `conform-to-template`, `register-artifact`, `recover-state`, `factory-cycles-bootstrap`
- **validate-finding-format.sh** and **validate-input-hash.sh** detail sections in hooks-reference.md

### Changed

- README counts updated: 104 skills, 104 commands, 19 hooks, 94 templates, 5 bin helpers, 381 tests across 12 suites
- hooks-reference.md updated: 17 → 19 hooks with full detail sections
- configuration.md updated: 17 → 19 hooks in PostToolUse table

## 0.37.0 — Input-hash drift detection tooling

### Added

- **bin/compute-input-hash** — 7-char MD5 hash from `inputs:` files. `--update` writes to frontmatter, `--check` compares (exit 2 on drift).
- **validate-input-hash.sh** — PostToolUse hook warning on missing/stale input-hash. Advisory.
- **check-input-drift skill** — batch scan + report. Optional `--fix`.
- **22 BATS tests** for all three components.

### Changed

- Producer agents updated to compute input-hash after writing
- Hooks: 18 → 19, Tests: 359 → 380, Suites: 11 → 12, Skills: 103 → 105

## 0.36.0 — Strict ID enforcement + comprehensive ID reference

### Added

- **validate-finding-format.sh** — PostToolUse hook blocking legacy ADV-NNN, ADV-P[N]-NNN, and STORY-NNN-FIX formats. Only current formats accepted: `ADV-<CYCLE>-P[N]-[SEV]-NNN` and `FIX-P[N]-NNN`. 15 tests.
- **docs/guide/id-reference.md** — comprehensive reference documenting all 30 ID formats with scope rules, producer, registry, hook validation, and detailed descriptions for key IDs

### Changed

- **All migration graces removed.** Only current formats accepted:
  - `behavioral_contracts` (not `bcs`), `target_module` (not `crate`)
  - `document_type: domain-spec-index` (not `domain-spec-section`)
  - `SS-NN` IDs (not subsystem names)
  - Current table headers only (no legacy acceptance)
  - Legacy ADV/FIX ID formats removed from FACTORY.md
- **validate-story-bc-sync.sh** reads both `behavioral_contracts:` and `bcs:` functionally (finds data regardless of field name — template-compliance hook flags the wrong name separately)
- Hooks: 17 → 18, Tests: 342 → 359, Suites: 10 → 11

## 0.35.0 — Complete ID system formalization + Subsystem Registry

### Added

- **Subsystem Registry** in ARCH-INDEX template with formal SS-NN ID format, lifecycle documentation, and naming rules
- **31-entry ID Format Reference** in FACTORY.md (was 16) — every ID system in the plugin now formally documented
- **ID Scope Definitions** — Lifecycle (append-only), Cycle (resets), Local (scoped to parent)
- **Legacy ID Formats** migration table — ADV-NNN, STORY-NNN-FIX mapped to current formats
- **3 new templates:** holdout-scenario-index (HS-INDEX with WHS section), epic-index, fix
- **5 glossary terms:** Epic, Fix PR, Gap Register, ID Scope, Wave Holdout Scenario

### Changed

- **Subsystem references now use SS-NN IDs** (was human-readable names). BC `subsystem:` and story `subsystems:` fields hold SS-IDs, not names.
- **validate-subsystem-names.sh** hook matches SS-NN IDs against ARCH-INDEX registry. Error messages show `SS-01 (Core Engine)` pairs.
- **Language-agnostic terminology:** "Crate" → "module/package" across agents and templates. Story field `crate:` → `target_module:` (migration alias accepted).
- Templates: 124 → 127
- Policy 6 enforcement: validates SS-IDs, not names

## 0.34.0 — Template v1.1 schema update (Prism-validated improvements)

### Changed — Template Schema Updates

All changes are additive or header renames with migration acceptance. Existing artifacts remain valid.

- **behavioral-contract-template.md (v1.1):** Added `## Description` (required), `## Related BCs`, `## Architecture Anchors`, `## Story Anchor`, `## VP Anchors` (Recommended)
- **story-template.md (v1.1):** Added optional frontmatter: wave, crate, subsystems, estimated_days. UX Screens conditional. Table header simplifications with migration acceptance.
- **module-criticality-template.md (v1.1):** Added Module Inventory, Per-Module Risk Assessment, Dependency Graph, Implementation Priority, Cross-Cutting Concerns (Recommended), Anti-Patterns (Conditional)
- **verification-architecture-template.md (v1.1):** Sections allow inline OR reference-to-shard
- **L2-domain-spec-index-template.md (v1.1):** Clarified canonical document_type, strengthened sections: as REQUIRED

### Updated — Companion Changes

- product-owner and story-writer agents updated for v1.1 template guidance
- validate-template-compliance hook filters Recommended/Conditional sections

## 0.33.0 — Template compliance enforcement hook

### Added

- **validate-template-compliance.sh** — PostToolUse hook that automatically validates every Write to `.factory/**/*.md` against its corresponding template. Checks required frontmatter fields and H2 section headings. Resolves templates via `document_type` frontmatter or path-pattern fallback. Warning messages suggest `/vsdd-factory:conform-to-template` for fixes.
- **14 BATS tests** with 4 fixtures (compliant BC, non-compliant BC, non-compliant story, holdout with no frontmatter)

### Changed

- Hooks: 16 → 17
- Tests: 328 → 342 across 10 suites

### Context

Built in response to Prism template compliance audit showing 0% strict compliance across stories (76 FAIL) and holdout scenarios (8 FAIL). The hook catches these drift patterns at write time — agents see the warning immediately and can self-correct.

## 0.32.0 — Template compliance skills + hook integration audit

### Added

- **validate-template-compliance skill** — read-only audit checking artifact files against their templates at three levels: frontmatter fields, section headings, table column headers. Resolves templates via `document_type` frontmatter or file path patterns. Reports PASS/WARN/FAIL per file with aggregate summary.
- **conform-to-template skill** — remediation skill that fixes structural gaps by adding missing frontmatter fields, section headings, and `[TODO]` placeholders. Safety guarantees: never deletes content, always shows diff before applying, creates backup. Reports table/order mismatches for manual fix.
- **8 BATS tests** for both skills (structure, three-level check, mapping, safety guarantees, commands)

### Fixed

- **handoff-validator.sh:** Was reading `.result` / `.output` / `.tool_response` but Claude Code SubagentStop sends `last_assistant_message`. Hook was always seeing empty content. Fixed with correct field + legacy fallback.

### Changed

- Skills: 101 → 103, Commands: 101 → 103
- Tests: 319 → 328

### Hook Integration Audit Results

Verified all 16 hooks parse correct JSON fields per Claude Code documentation:
- Edit|Write hooks: `tool_input.file_path` — all 10 correct
- Bash hooks: `tool_input.command` — all 4 correct
- SubagentStop: `last_assistant_message` — fixed (was wrong)
- PostToolUse hooks correctly use exit 2 for prominent warnings (edits can't be undone)

## 0.31.0 — Template extraction + hook trigger fixes

### Added

- **12 new templates** extracted from inline format definitions:
  - HIGH: story-index, traceability-matrices, extraction-validation
  - MEDIUM: spec-drift-report, formal-verification, performance-report, implementation-readiness, design-drift, brief-validation, research-index
  - LOW: agent-file-review, consistency-validation-report
- **14 new BATS tests** for hook trigger edge cases:
  - check-factory-commit: 4 tests (was zero coverage)
  - red-gate: 3 absolute path tests
  - destructive-command-guard: 5 complex bash construct tests
  - verify-git-push: 2 edge case tests (--force-with-lease, -f at end)

### Fixed

- **verify-git-push:** `--force-with-lease` (safe force push) was incorrectly blocked because it substring-matched `--force`. Now allowed.
- **red-gate:** Absolute file paths from Claude Code tool_input never matched relative paths in `.red[]` array. Added PWD and git-root prefix stripping.

### Changed

- 13 agent/skill files updated to reference new templates instead of inline format definitions
- Templates: 112 → 124
- Tests: 305 → 319

## 0.30.2 — Generalize Policy 9 hook for multi-project portability

### Changed

- **validate-vp-consistency.sh Check (d):** Replaced hardcoded Kani/Proptest/Fuzz column detection with generic header-based discovery. Auto-detects ALL verification method columns from the Coverage by Module header row. Works for any tool names (Kani, CBMC, Hypothesis, fast-check, Stryker, etc.).
- **verification-coverage-matrix-template.md:** Method columns are now documented as project-specific with language-specific examples. Hook auto-detects — no configuration needed.
- Removed dependency on `.declared` temp file for Check (d) — uses `get_summary_total` directly with partial label matching.

## 0.30.1 — Fix Policy 9 hook defects + verification templates

### Fixed

- **validate-vp-consistency.sh Defect 1:** Multi-word Summary labels (e.g., "Integration test") triggered `set -u` abort on arithmetic comparison. All Policy 9 arithmetic enforcement (Checks c/d/e) was silently disabled. Fixed by normalizing labels to snake_case + defensive non-integer guard.
- **validate-vp-consistency.sh Defect 2:** EXIT trap clobbered non-zero exit codes to 0. Now preserves original rc through trap.
- **validate-vp-consistency.sh Defect 3:** Check (d) summed wrong columns (Criticality+Kani+Proptest, missing Fuzz). Replaced with header-detected column positions that find Kani/Proptest/Fuzz by name in the header row, handling any column layout.
- **Test fixtures** rebuilt from template format (was ad-hoc 5-column format that no project produces)

### Added

- **verification-coverage-matrix-template.md** — 7-column template with column definitions, arithmetic invariant rules, coverage gaps table, and domain invariant verification map
- **verification-architecture-template.md** — Provable Properties Catalog, P0/P1 lists, purity boundary, verification tooling tables
- **2 real-world regression tests** in policy9.bats (multi-word labels, fuzz column drift)
- **2 new test fixture sets** (policy-9-realworld, policy-9-fuzz-drift) matching template format

### Changed

- Architect agent references new templates for verification-coverage-matrix and verification-architecture sections
- Templates: 110 → 112

## 0.30.0 — Quality of life + comprehensive guardrails audit complete (Tier 5)

### Added

- **Policy reference matrix + violation playbook** (`docs/guide/policy-reference.md`) — single-page quick reference for all 9 policies with enforcement matrix (hook, criteria, agents) and per-policy step-by-step fix procedures
- **register-artifact skill** — automates INDEX file registration after creating BCs, VPs, stories, or holdout scenarios. Supports batch registration. 4 BATS tests.
- **recover-state skill** — reconstructs `.factory/STATE.md` from artifacts on disk when corrupted or missing. 9-step procedure with backup, artifact scanning, phase determination, user validation, and `--dry-run` option. 7 BATS tests.

### Changed

- Skills: 96 → 101, Commands: 96 → 101, Templates: 109 → 110
- Test count: 292 → 303 across 9 suites

### Audit Summary

This release completes the 5-tier comprehensive guardrails audit:
- **Tier 1 (v0.27.0):** 3 data safety hooks — destructive command guard, branch protection, factory branch guard
- **Tier 2 (v0.27.1):** 3 policy enforcement hooks — subsystem names, BC titles, story-BC sync
- **Tier 3 (v0.28.0):** All 33 agents standardized with Tool Access, Failure & Escalation, Remember, AGENT-SOUL.md
- **Tier 4 (v0.29.0):** 31 hook robustness + error contract tests, orchestrator FACTORY.md refs
- **Tier 5 (v0.30.0):** Policy reference docs, register-artifact skill, recover-state skill

Total impact: 16 hooks (was 11), 303 tests (was 133), 101 skills (was 96), all agents standardized.

## 0.29.0 — Testing gaps + orchestrator standardization (Tier 3-4)

### Added

- **FACTORY.md references** for all 9 orchestrator sequence files (brownfield, discovery, feature, greenfield, heartbeat, maintenance, multi-repo, per-story-delivery, steady-state)
- **hook-robustness.bats** — 31 tests verifying all 7 enforcement hooks handle: empty JSON, missing fields, nonexistent files, empty files, malformed frontmatter, and produce correct error contract (BLOCKED / POLICY N VIOLATION keywords)
- **BATS test** verifying all orchestrator files reference FACTORY.md

### Changed

- Test count: 260 → 292 across 9 suites
- README: updated test counts, suite counts, suite listings

## 0.28.0 — Agent standardization (Tier 3)

### Added

- **Tool Access sections** for 5 agents missing them: adversary (`read-only`), codebase-analyzer (`full`), holdout-evaluator (`restricted`), research-agent (`full`), validate-extraction (`full`)
- **Failure & Escalation sections** for 6 agents missing them (adversary, codebase-analyzer, holdout-evaluator, validate-extraction, implementer, plus research-agent already had one)
- **Remember + AGENT-SOUL.md closing** for all 5 newly standardized agents
- **4 structural completeness tests** — verify ALL 33 agents have: Tool Access, Failure & Escalation, Remember, AGENT-SOUL.md reference
- **5 profile tests** for newly documented agent profiles

### Changed

- Test count: 251 → 260 across 8 suites
- All 33 agents now have consistent structure: Tool Access, Failure & Escalation, Remember, AGENT-SOUL.md

## 0.27.1 — Policy enforcement hooks (Tier 2)

### Added

- **validate-subsystem-names.sh** (Policy 6) — PostToolUse hook verifying BC `subsystem:` and story `subsystems:` fields match ARCH-INDEX Subsystem Registry canonical names. Error messages list all valid names.
- **validate-bc-title.sh** (Policy 7) — PostToolUse hook verifying BC file H1 heading matches BC-INDEX title column. Shows both titles on mismatch.
- **validate-story-bc-sync.sh** (Policy 8) — PostToolUse hook verifying bidirectional BC completeness between story frontmatter `bcs:` array, body BC table, and AC trace annotations. Identifies specific missing BCs.
- **28 BATS tests** for all three hooks with shared fixtures (ARCH-INDEX, BC-INDEX, good/bad BC and story files)

### Changed

- Hook count: 13 → 16
- Test count: 223 → 251 across 8 suites
- hooks-reference.md, configuration.md: updated with all new hooks

### Analysis (Finding #6 conclusions)

Policies 1-5 remain gate-only (consistency-validator criteria). Research confirmed:
- Policy 1 (append_only_numbering): requires git history scan — too expensive for edit-time
- Policy 2 (lift_invariants_to_bcs): requires cross-file scan of all BCs — blocks intermediate states
- Policy 3 (state_manager_runs_last): temporal ordering — not a file-content check
- Policy 4 (semantic_anchoring_integrity): requires semantic judgment — only adversary can assess
- Policy 5 (creators_justify_anchors): can check presence but not correctness — adversary review is appropriate

## 0.27.0 — Data safety hooks (Tier 1)

### Added

- **destructive-command-guard.sh** — PreToolUse Bash hook blocking `rm -rf` on protected paths (.factory/, src/, tests/), `rm` on source-of-truth files (STATE.md, *-INDEX.md, prd.md), `git reset --hard`, `git clean -f`, `git checkout -- .`, `git restore .`, `git rm` on spec/story paths. Each block message includes the safe alternative. 46 BATS tests.
- **factory-branch-guard.sh** — PreToolUse Edit|Write hook blocking writes to `.factory/` when not mounted as git worktree on `factory-artifacts` branch. Also guards `.factory-project/` for multi-repo projects. Block messages include the exact recovery command. 6 BATS tests.
- **verify-git-push.sh** enhanced — now blocks direct push to protected branches (main, master, develop) in addition to force push. Block messages suggest the PR workflow. 10 BATS tests.

### Changed

- Hook count: 10 → 13
- Test count: 161 → 223 across 7 suites
- hooks-reference.md: complete rewrite covering all 13 hooks with detail sections
- configuration.md: updated hook tables with all new hooks
- README: updated hook count, test count, suite count, suite listings

## 0.26.0 — Policy registry, cycle management, scoped reviews

### Added

- **Policy registry** (`/vsdd-factory:policy-registry`) — declarative `.factory/policies.yaml` with init, list, validate, show commands. 9 baseline policies with `verification_steps` field providing step-by-step check procedures the adversary executes per-policy.
- **Policy add** (`/vsdd-factory:policy-add`) — register new governance policies mid-cycle with schema validation, sequential ID assignment, and verification steps
- **policies-template.yaml** — complete template with 9 baseline policies, verification steps, lint hook references, and scope declarations
- **Adversarial-review policy auto-loading** — orchestrator reads `policies.yaml` and injects full policy rubric (with verification steps) into every adversary dispatch, replacing manual copy-pasting
- **Cycle layout bootstrap** (`/vsdd-factory:factory-cycles-bootstrap`) — migrate from flat `specs/adversarial-review-pass-*.md` layout to cycle-keyed directories with `git mv` for history preservation
- **Scoped adversarial review** (`--scope` parameter) — `full` (default), `diff-from:<commit>` (focus on changed files), `paths:<pattern>` (target specific subsystems)
- **Cycle-prefixed finding IDs** — `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` format prevents ID collisions across multi-cycle projects

### Changed

- **adversarial-review** `disable-model-invocation` set to `false` — enables orchestrator-initiated invocation for autonomous convergence loops
- **Orchestrator** updated with policy rubric loading directive and new cross-cutting skills
- Finding ID format in 4 templates updated from `ADV-P[N]-NNN` to cycle-prefixed format
- README: test count 152 → 161, skills.bats 24 → 33

## 0.25.0 — Policy 9 lint hook + adversarial-review skill hardening

### Added

- **Policy 9 lint hook** (`hooks/validate-vp-consistency.sh`) — PostToolUse hook that validates VP-INDEX.md ↔ verification-architecture.md ↔ verification-coverage-matrix.md consistency on every edit. Checks: (a) every VP-INDEX VP appears in both arch docs, (b) per-tool summary totals match row counts, (c) coverage matrix Totals row matches data row sums, (d) no orphaned VP references in coverage matrix. Wired into hooks.json PostToolUse.
- **Architect source-of-truth invariants table** — consolidates ARCH-INDEX (subsystem names), BC H1 (titles), VP-INDEX (VP catalog) as the three authoritative sources with their downstream documents. Cross-links to validate-vp-consistency.sh as automated enforcer.
- **Adversarial-review post-adversary persistence** — explicit orchestrator step to capture adversary findings and dispatch state-manager to persist them to `.factory/cycles/<cycle>/adversarial-reviews/pass-<N>.md`. Prevents findings loss when adversary (read-only agent) returns chat text that disappears on session boundary.
- **Adversarial-review filename collision guard** — pre-flight check refuses to overwrite existing review files with different content. Points to cycle bootstrap skill for resolution. Warns about legacy flat-file layout.
- **11 BATS tests** for Policy 9 hook (3 fixture sets: green, canary with fuzz column drift, missing-VP)
- **3 BATS tests** for adversarial-review persistence + collision guard presence

### Changed

- README: test count 138 → 152, suite count 5 → 6
- run-all.sh: includes policy9.bats suite

## 0.24.1 — Policy 9: VP-INDEX source of truth

### Added

- **Policy 9: `vp_index_is_vp_catalog_source_of_truth`** — VP-INDEX.md is the authoritative VP enumeration; changes must propagate to verification-architecture.md and verification-coverage-matrix.md in the same burst
- **Architect agent:** VP-INDEX propagation obligation — must touch VP-INDEX AND both architecture anchor docs in the same output
- **Adversary agent:** VP-INDEX ↔ architecture document coherence review axis (4-point verification: self-consistency, catalog match, coverage matrix match, reverse check)
- **Product-owner agent:** VP citation change handoff — notes arch-doc propagation obligation when VP citations change in BCs
- **Consistency-validator criteria 78-80:** VP-INDEX self-consistency (arithmetic), VP-INDEX → verification-architecture.md completeness, VP-INDEX → verification-coverage-matrix.md completeness
- **5 BATS tests** for policy 9 presence in architect, adversary, product-owner, consistency-validator, FACTORY.md
- **Glossary term:** VP-INDEX Source of Truth

### Changed

- Consistency-validator criteria count: 77 → 80
- Session-review template: added policy 9 to governance policy audit checklist
- README: test count 133 → 138

## 0.24.0 — 8 governance policies formalized + permission model regression tests

### Added

- **Governance policy registry** in FACTORY.md — 8 top-level integrity policies with enforcing agents, validation criteria, and severity floors
- **8 governance policies** formally integrated into agent prompts:
  - `append_only_numbering` — IDs never renumbered, filename slugs immutable (product-owner, spec-steward, criterion 77)
  - `lift_invariants_to_bcs` — every DI-NNN must be cited by at least one BC (product-owner, adversary, criterion 74)
  - `state_manager_runs_last` — already present, verified across all agents
  - `semantic_anchoring_integrity` — formalized as criteria 70-73 (was unnumbered)
  - `creators_justify_anchors` — added to business-analyst for CAP-NNN creation
  - `architecture_is_subsystem_name_source_of_truth` — added product-owner BC subsystem validation, criterion 76
  - `bc_h1_is_title_source_of_truth` — added product-owner enrichment rule, adversary title sync axis, criterion 75
  - `bc_array_changes_propagate_to_body_and_acs` — story-writer, product-owner, adversary, criteria 67-69
- **permissions.bats** — new test suite (53 tests) covering agent permission model and governance policy presence
  - Profile enforcement: spec producers `coding`, code producers `full`, coordinators restricted
  - Tool-profile coherence: coding-profile agents have no shell commands in code blocks or inline backticks
  - Policy presence: each of 8 policies verified in correct agent prompts
- **Consistency-validator criteria 67-77** — 11 new validation criteria for governance policy enforcement
- **3 new adversary review axes** — BC title/subsystem sync, invariant-to-BC orphan detection, story frontmatter-body coherence
- **Glossary terms** — Append-Only Numbering, Governance Policy, Invariant Lifting

### Changed

- **accessibility-auditor** profile: `coding` → `full` (needs shell for axe-core, lighthouse, pa11y, eslint jsx-a11y)
- **FACTORY.md** permission model table: added "Tool-based reviewers" row for accessibility-auditor
- **Consistency-validator** criteria count: 66 → 77
- **Session-review template**: governance policy audit section expanded to cover all 8 policies
- **Cycle-manifest template**: added "Governance Policies Adopted" section
- **README.md**: test count 62 → 133, suite count 3 → 5

### Fixed

- **story-writer** pre-commit verification: changed "grep the story file" to "read the story file" (story-writer has `coding` profile, no shell access)
- **accessibility-auditor** tool-profile mismatch: was told to run `npx` commands but had `coding` profile (no exec)

## 0.23.0 — Comprehensive documentation update + Prism lessons + DTU taxonomy + agent permission model

### Added
- **12 Prism Phase 3 lessons** codified across agents and skills:
  - State-manager-last ordering, path-prefix verification, burst splitting (>8 artifacts), BC anchor-back in same burst
  - Fresh-context consistency audit at every gate, universal DTU integration surface taxonomy, BC retirement checklist, trajectory monotonicity, single source of truth rule
  - Structured human review questions at every gate, minimum 3 clean passes for convergence
- **Universal DTU integration surface taxonomy** — 6 mandatory categories (inbound data, outbound operations, identity & access, persistence & state, observability & export, enrichment & lookup) replacing project-specific categories
- **Agent permission model** documented in FACTORY.md, agents-reference, and configuration guide — spec producers (coding), code producers (full), coordinators (restricted), infrastructure (full)
- **Semantic Anchoring Integrity** — adversary, consistency-validator, product-owner, story-writer, architect enforce semantic correctness of all anchors
- **DTU assessment gate** mandatory in Phase 1 (P1-06) with pre-Phase 4 clone existence check
- **CI/CD deferred to post-architecture** (P1-06b) with pre-Phase 4 verification gate
- **Complete command coverage** — all 96 skills now have slash commands (was 47)
- **Activate agent ID fix** — 3-segment format (`vsdd-factory:orchestrator:orchestrator`)
- **Glossary entries** for semantic anchoring, convergence trajectory, integration surface taxonomy, single source of truth, anchor justification, trajectory monotonicity

### Changed
- **Getting-started guide** now includes `/activate` as Step 2, scaffold-claude-md as Step 5
- **Cross-cutting skills guide** expanded from ~25 to all 96 skills organized into 20 categories
- **Commands reference** expanded from ~47 to all 96 commands organized by category
- **Phase 1 guide** documents mandatory DTU assessment, CI/CD setup, anchor justification, consistency audit
- **Pipeline overview** documents DTU gate, CI/CD gate, consistency audit, pre-Phase 4 gates
- **README counts** corrected: commands 47→96, agents 34→33, templates 108→109

### Fixed
- **Command files** use colon syntax (`vsdd-factory:skill-name`) — was space syntax causing "Unknown skill" errors
- **Delegation commands** route through orchestrator instead of bouncing
- **Agent permissions** — product-owner, story-writer, architect reverted to `coding` profile (state-manager owns `.factory/` commits)
- **All project-specific references** removed from agent and skill files — generic examples only
- **Session-reviewer** agent name corrected throughout (was `session-review`)

## 0.22.0 — Semantic Anchoring Integrity

### Added
- **Adversary: Semantic Anchoring Audit** — new scan category verifying anchors are semantically correct, not just syntactically valid. 4-level severity matrix (CRITICAL/HIGH/MEDIUM/LOW). Mis-anchoring ALWAYS blocks convergence — never deferred as "Observation."
- **Consistency-validator: Anchor Semantic Audit** — verifies BC↔capability, story↔subsystem, VP↔anchor_story, and traceability table semantic correctness beyond structural ID matching
- **Product-owner: Anchor Justification Requirement** — must explicitly justify capability anchor choice citing source-of-truth when creating/modifying BCs
- **Story-writer: Anchor Justification Requirement** — must justify subsystem, dependency, and VP anchor choices with specific technical reasons
- **Architect: Anchor Justification Requirement** — must justify ADR references, subsystem assignments, and crate ownership claims. Planned-but-not-created crates must be marked `[PLANNED]`

## 0.21.0 — Orchestrator sync

### Fixed
- **Agent routing table** — added missing `codebase-analyzer` and `validate-extraction` agents, fixed `session-review` → `session-reviewer` to match agent filename, removed duplicate `product-owner` entry
- **Agents reference doc** — corrected `session-review` → `session-reviewer`

### Added
- **Cross-cutting skills reference** in orchestrator — table of 9 skills available at any pipeline point (scaffold-claude-md, visual-companion, create-excalidraw, systematic-debugging, writing-skills, validate-consistency, spec-drift, research, track-debt)

### Changed
- **State-manager delegation description** updated — orchestrator now documents that state-manager owns `.factory/` commits directly (no devops-engineer roundtrip)

## 0.20.1 — State-manager direct git commits

### Changed
- **State-manager now has shell access** for direct `.factory/` git commits. No longer spawns devops-engineer for every STATE.md update. Shell access is scoped: git commands inside `.factory/` only, no non-git commands, no source code branches.

## 0.20.0 — CI/CD deferred to post-architecture

### Changed
- **CI/CD setup moved out of repo-initialization** — repo-init no longer creates CI/CD workflows because the tech stack is unknown at init time. CI/CD is now a separate mandatory step (`phase-1-cicd-setup`) that runs after architecture determines the language, framework, and deployment topology.

### Added
- **Mandatory CI/CD setup step** (P1-06b) — devops-engineer creates `.github/workflows/` (ci.yml, release.yml, security.yml) based on architecture output, updates branch protection with CI status checks, produces `cicd-setup.md`
- **CI/CD criteria in Phase 1 gate** — ci.yml must exist, cicd-setup.md must exist, branch protection must require CI checks
- **Pre-Phase 4 CI/CD gate** — verifies CI pipeline exists and runs successfully before implementation begins
- CI/CD added to orchestrator's mandatory steps list (never skip, never conditional)

## 0.19.0 — Complete command coverage + activate agent ID fix

### Added
- **49 missing command files** — every skill now has a corresponding slash command for full autocomplete coverage
- Delegation reference commands (12) route through the orchestrator instead of bouncing
- Execution commands (37) invoke skills directly via the Skill tool

### Fixed
- **Activate skill writes correct 3-segment agent ID** (`vsdd-factory:orchestrator:orchestrator`). The 2-segment form (`vsdd-factory:orchestrator`) silently fell back to plain Claude because the orchestrator lives in a subdirectory.
- **Delegation command files** (dtu-creation, guided-brief-creation) now route through the orchestrator instead of trying to execute delegation-reference skills directly

## 0.18.0 — DTU assessment gate enforcement + command syntax fix

### Added
- **DTU assessment is now mandatory** (P1-06) — always produces `dtu-assessment.md`, even if the answer is "DTU_REQUIRED: false" with rationale. Prevents silent skip that occurred in Prism.
- **DTU checks in Phase 2 gate** — `dtu-assessment.md` must exist, fidelity classifications required if DTU_REQUIRED, rationale required if not
- **Pre-Phase 4 DTU clone existence gate** — if DTU_REQUIRED: true, verifies clones are built and validated before implementation begins
- **Mandatory steps list** in orchestrator — explicit "never skip, never conditional" list covering DTU assessment, adversarial convergence, holdout evaluation
- **DTU status in STATE.md** — state-manager writes `dtu_required`, `dtu_assessment`, `dtu_clones_built`, `dtu_services` fields for visibility across sessions

### Fixed
- **All 47 command files** now use colon syntax (`vsdd-factory:skill-name`) instead of space syntax (`vsdd-factory skill-name`). The space syntax caused "Unknown skill" errors when commands delegated to skills via the Skill tool.

## 0.17.0 — Prism Phase 3 lessons learned

### Added
- **8 lessons from Prism Phase 3 adversarial convergence** (29 passes, 46 stories, 167 BCs, 38 VPs) codified across 6 agent/skill files:
  - **story-writer:** must read source BC files (not summaries), use centralized version pins from dependency-graph.md, include forbidden dependencies section, use only existing error codes from taxonomy, pre-validate new stories against invariant list
  - **adversary:** accumulate confirmed invariants across passes (monotonically growing list)
  - **adversarial-review:** fix root causes not symptoms (rewrite from BC, don't patch lines), accumulate invariants, pre-validate new scope additions
  - **implementer:** fix root causes from BC source, read before editing and verify after editing
  - **deliver-story:** verify every fix landed correctly (read file, grep for pattern, check for side effects)
  - **create-story:** centralized version pins, forbidden dependencies section, error taxonomy compliance

## 0.16.1 — Reference manifest template + documentation fixes

### Added
- **reference-manifest-template.yaml** — standardized template for `.factory/reference-manifest.yaml` combining corverax and vsdd-factory formats (url, commit SHA, ingested date, depth, focus, status)
- End-user guide for visual companion (`docs/guide/visual-companion.md`) with Mermaid workflow diagrams

### Fixed
- Brownfield-ingest skill now references the template for manifest format
- Removed stale `/vsdd-factory:excalidraw-export` reference from visual companion See Also
- Added visual companion to README documentation table

## 0.16.0 — Excalidraw integration + visual companion testing

### Added
- **Excalidraw integration** in visual companion — `.excalidraw` files render as interactive canvases in the browser with user editing and WebSocket save-back
- **create-excalidraw skill** (`/vsdd-factory:create-excalidraw`) — generate `.excalidraw` JSON files for architecture diagrams, entity relationships, and flow charts
- **History sidebar** — collapsible panel showing all past screens (HTML and excalidraw), click to navigate
- **Composed views** — `screen.json` manifest for multi-pane layouts (split, side-by-side)
- **setup.sh** — one-time setup script installs React + excalidraw dependencies and builds the viewer
- **18 visual companion tests** — server routes, file-type detection, API endpoints, file serving
- React app scaffold (Vite + React 18 + @excalidraw/excalidraw v0.18)

### Fixed
- Server `__ACTIVE_FILE__` and `__MANIFEST__` injection now uses script tag insertion instead of string replacement
- Tiered visual tooling tables corrected across 5 files — replaced incorrect excalidraw-export reference with proper tiers (visual-companion excalidraw mode, create-excalidraw standalone, Mermaid, ASCII)

### Changed
- Visual companion server now supports `.html`, `.excalidraw`, and `screen.json` file types (was HTML-only)
- `/api/files` endpoint returns all screen files with metadata
- `/api/drawing/<name>` endpoint serves raw excalidraw JSON
- `/html/<name>` endpoint serves individual HTML files (for iframe embedding)
- Test suite now 80 tests across 4 suites (was 62 across 3)

## 0.15.0 — Systematic debugging, verification discipline, and writing-skills

### Added
- **systematic-debugging skill** — 4-phase root cause investigation process adapted from superpowers, with BC-aware debugging and "3+ fixes = architectural problem" escalation rule
- **writing-skills skill** — TDD methodology for creating and maintaining skills (RED-GREEN-REFACTOR applied to process documentation), with CSO guidance and rationalization resistance patterns
- **Verification discipline** — deliver-story and per-story-delivery now enforce independent verification of agent claims before proceeding (agent reports are claims, test output is evidence)
- **Review feedback guidance** — implementer and test-writer agents now have explicit guidance for receiving code review (verify before implementing, push back when wrong, BC is source of truth)

### Documentation
- Getting started guide now includes scaffold-claude-md as Step 4
- Cross-cutting skills guide documents visual-companion, systematic-debugging, and writing-skills
- Phase 1 guide documents visual tooling and self-review checklists
- Phase 2 guide documents scope check, plan failures, and self-review
- Phase 3 guide documents verification discipline, agent status protocol, model selection, review feedback handling, and debugging reference
- Agents reference documents the standard status protocol and self-review
- README skill count updated to 95

## 0.14.0 — Agent dispatch quality gaps

### Added
- **Standard agent status protocol** (DONE/DONE_WITH_CONCERNS/NEEDS_CONTEXT/BLOCKED) — agents-md-template, implementer, test-writer, pr-manager all report structured status codes
- **"Over your head" escalation language** — agents-md-template, implementer, test-writer explicitly encourage early escalation over bad work
- **Pre-handoff self-review checklists** — implementer (completeness, TDD, YAGNI), test-writer (coverage, behavior vs implementation, naming), pr-manager (description accuracy, traceability, demo evidence)
- **Model selection guidance** — deliver-story and per-story-delivery.md include tier mapping (fast/standard/capable) per dispatch task type
- **Extended Red Flags** — deliver-story adds 4 new dispatch anti-patterns (parallel dispatch, shared agents, skipped reviews, same-model retry)

## 0.13.0 — Story decomposition quality gaps

### Added
- **Hard gate language** — decompose-stories and create-story block premature implementation
- **Scope decomposition check** — decompose-stories verifies PRD describes a single product before breaking it down
- **"Plan Failures" anti-pattern list** — both skills explicitly ban "TBD", vague error handling, untestable ACs, and 4 other story-invalidating patterns
- **Self-review checklists** — decompose-stories checks spec coverage, consistency, and sizing; create-story checks completeness, testability, and context budget
- **Execution reference** in story template — points to `/vsdd-factory:deliver-story STORY-NNN`

## 0.12.0 — Early-phase quality gaps + visual companion

### Added
- **visual-companion skill** (`/vsdd-factory:visual-companion`) — browser-based mockups, diagrams, and interactive A/B choices during brainstorming, brief creation, and architecture design. Ported from superpowers. Optional, requires Node.js.
- **Tiered visual tooling strategy** — early-phase skills auto-detect available tools (visual-companion → excalidraw-export → Mermaid → ASCII) with no hard dependencies
- **Pre-adversarial self-review checklist** — added to create-brief, create-prd, create-architecture, and create-domain-spec to catch obvious gaps before the expensive adversary loop
- **Hard gate language** — explicit "do NOT skip to next phase" guards in brainstorming, guided-brief-creation, create-brief, create-prd, and create-architecture
- **Anti-pattern + Red Flags table** — brainstorming skill now calls out the "too simple to brainstorm" rationalization with a 7-row cognitive trap table

### Changed
- FACTORY.md documents visual companion in project tooling section
- VSDD.md references visual companion in Tooling section

## 0.11.0 — CLAUDE.md scaffolding skill

### Added
- **scaffold-claude-md skill** (`/vsdd-factory:scaffold-claude-md`) — auto-detects project language, build/test/lint commands, git workflow, and project references to generate a project-specific `CLAUDE.md`
- Activate skill now suggests `scaffold-claude-md` when no `CLAUDE.md` exists
- Optional `scaffold-claude-md` step in greenfield and brownfield workflows

### Changed
- FACTORY.md documents CLAUDE.md scaffolding in project setup section
- VSDD.md references the new skill in Tooling section

## 0.10.3 — Release infrastructure and CI/CD

### Added
- **Release workflow** (`.github/workflows/release.yml`) — tag-triggered validation + GitHub Release with CHANGELOG excerpt
- **Release config** (`.factory/release-config.yaml`) — declarative release manifest on factory-artifacts branch
- **Release skill rewrite** — config-driven, 3 modes (init/release/dry-run), quality gate spectrum
- Retroactive git tags and GitHub Releases for all 12 prior versions (v0.1.0 through v0.10.2)
- Version field in marketplace.json for release validation
- Factory-artifacts mount step in CI and release workflows

### Changed
- CI workflow renamed from `plugin-validation.yml` to `ci.yml` for cross-repo consistency
- Bump `actions/checkout` from v4 to v6 (Node.js 20 deprecation)

## 0.10.2 — Template path portability fix

Closes a portability hole that would have broken clean installs.

### The bug

Skills and agents referenced templates as `.claude/templates/<name>.md` — a path that only exists inside corverax, where the plugin was originally developed and `.claude/templates/` is pre-populated. A clean install of vsdd-factory into any other project would ship the templates at `plugins/vsdd-factory/templates/` (where they actually live) but every skill referencing `.claude/templates/...` would fail the lookup.

59 references across 24 files were affected:

- 20 skills: `research`, `semport-analyze`, `brownfield-ingest`, `create-brief`, `create-story`, `create-domain-spec`, `create-architecture`, `create-prd`, `adversarial-review`, `holdout-eval`, `state-update`, `record-demo`, `pr-create`, `decompose-stories`, `track-debt`, `convergence-check`, `validate-consistency`, `deliver-story`, `dtu-validate`, `formal-verify`
- 4 agents: `validate-extraction`, `research-agent`, `adversary`, `holdout-evaluator`

### The fix

All 59 references rewritten from `.claude/templates/<name>` to `${CLAUDE_PLUGIN_ROOT}/templates/<name>` — the Claude Code canonical environment variable for the plugin root directory. Agents shell-expand the variable when reading via bash, and the path resolves to the real template location that ships with the plugin regardless of install target.

### Regression guards (3 new tests)

`tests/skills.bats` grew a "Template path portability" section with three tests:

- `no skill references the non-portable .claude/templates/ path` — grep-based regression guard
- `no agent references the non-portable .claude/templates/ path` — same
- `every referenced template actually exists in plugin templates/` — extracts every `${CLAUDE_PLUGIN_ROOT}/templates/<file>` reference from skills and agents, strips the prefix, and asserts the file exists at `plugins/vsdd-factory/templates/<file>`. Caught zero dangling references on first run.

Test suite now **62 tests**, all pass.

### Note for future skill authors

When citing a template in a new skill or agent, use:

```
- `${CLAUDE_PLUGIN_ROOT}/templates/<name>.md` — <description>
```

The `.claude/templates/` prefix is never portable and is now a test failure.

## 0.10.1 — Step-file content fill

Closes the last deferred item from 0.9.0: empty `steps/` placeholder stubs in three skills now carry real per-step playbooks.

### 17 step files expanded (1566 LOC)

The three facilitation / inspection skills (`brainstorming`, `artifact-detection`, `guided-brief-creation`) had `## Step-File Decomposition` tables referencing per-step files that were 3-6 line placeholders. The parent SKILL.md carried the high-level flow; the step files existed only as stubs.

Each step file is now a 58-130 line self-contained playbook the orchestrator can load on demand when executing that specific step. Structure per file:

- **Inputs** — what previous steps produced, files to read, expected state
- **Outputs** — exact artifact paths and formats
- **Procedure** — specific moves, exact elicitation questions (for facilitation skills), exact commands and glob patterns (for inspection skills)
- **Decision points** — branches with criteria, where applicable
- **Failure modes** — step-level failures (distinct from whole-skill failures in parent SKILL.md)
- **Quality gate** — short checklist before advancing
- **Hand-off** — what to pass to the next step

**brainstorming (6 files, 487 LOC):** session setup, technique selection, facilitated ideation, synthesis, direction selection, report writing. Includes exact opening questions, transition phrases, SCAMPER/reverse-brainstorming/mind-mapping/constraint-removal scripts, and the verbatim markdown template for `brainstorming-report.md`.

**artifact-detection (5 files, 510 LOC):** scan, classify, validate, gap analysis, route decision. Includes exact glob patterns per artifact type, explicit validation checklists (rewritten from the SKILL.md prose as iterable rules), DF-020/DF-021 format-migration handling, and verbatim templates for `artifact-inventory.md`, `gap-analysis.md`, and `routing-decision.md`.

**guided-brief-creation (6 files, 569 LOC):** understand intent, contextual discovery, guided elicitation, draft review, adversarial review, finalize. Includes exact section-by-section elicitation questions, research-agent / adversary dispatch criteria, market-intel integration points, and verbatim structures for `product-brief.md` and `elicitation-notes.md`.

### Cross-step dependencies surfaced

Step files make explicit several dependencies that were implicit in the prose:

- **artifact-detection format flags propagate** — format detection in step 1b (FR-NNN vs BC-S.SS.NNN, old vs DF-021-sharded architecture) flows into step 3 validation rules and step 5 routing decisions
- **guided-brief-creation market-intel reference** — `market-intel.md` is read in step 3 and again in step 5 adversarial review for differentiation and risk signals
- **guided-brief-creation adversarial loopback** — step 5 feedback can send the agent back to step 3 (re-elicitation) or step 4 (redraft)

### Meta

- No SKILL.md files modified. Step-file decomposition tables unchanged.
- All 59 tests still pass. No new tests added for step-file content (content is prose, not behavior).
- Full analysis report at `.factory/semport/STEPS-REPORT.md`.

## 0.10.0 — Deferred remediation: commands, hook envelopes, structural tests

Closes out the remaining P1/P2 items deferred from 0.9.0.

### Commands directory (47 files)

Prior versions exposed skills only — many with `disable-model-invocation: true`, which meant users had no slash-command entry point for phase transitions, health checks, or delivery. This release ships `plugins/vsdd-factory/commands/` with **47 thin slash-command wrappers**, one per user-facing skill.

Each command is 15-30 lines: frontmatter (description + optional `argument-hint` mirrored from the skill) and a body that delegates via `Use the <skill-name> skill via the Skill tool`. Commands are entry points; skills remain the source of truth.

Coverage: all Phase 0-6 lifecycle skills (brownfield-ingest, semport-analyze, create-brief through release), cross-cutting ops (factory-health, track-debt, worktree-manage), and UI/design skills (design-system-bootstrap, ui-quality-gate, etc.).

### Hook upgrade: permissionDecision envelopes (POC on spec-steward)

`hooks/protect-vp.sh` and `hooks/protect-bc.sh` now emit `PreToolUse` JSON envelopes with `permissionDecision` + `permissionDecisionReason` instead of bare exit codes. The denial reasons are richer and instruct the agent to create a superseding artifact rather than just blocking the edit.

This is a POC on the two spec-steward hooks. The other hooks (`brownfield-discipline`, `red-gate`, `purity-check`, etc.) still use exit codes. Upgrading them requires per-hook design — deferred until a specific need motivates each one.

Tests updated: the two "blocks edit to green X" tests now assert `status -eq 0` with `permissionDecision:deny` in stdout, replacing the old `status -eq 2` stderr check.

### Structural tests for Iron Laws and Red Flags (18 new tests)

New `tests/skills.bats` enforces that the four discipline skills carry their behavior-shaping scaffolding. A discipline skill missing its Iron Law, "Announce at start" line, or Red Flags table is now a test failure — empirically load-bearing content cannot silently rot.

Test coverage per skill:
- `deliver-story`, `brownfield-ingest`, `adversarial-review`, `wave-gate`: Iron Law token + `## The Iron Law` section + `## Announce at Start` section + `## Red Flags` table with ≥8 rows
- `brownfield-ingest` specifically: Honest Convergence clause, Known Round-1 Hallucination Classes, Subagent Delivery Protocol (`=== FILE:` delimiter), Behavioral vs Metric split, Priority-ordered Lessons mandate
- `validate-extraction` agent: Behavioral vs Metric operating mode with Phase 1 / Phase 2 sections

Total suite: **59 tests** (41 pre-existing + 18 new). All pass.

### Name collision fix

`agents/session-review.md` renamed to `agents/session-reviewer.md` to disambiguate from the `skills/session-review/` directory. Non-breaking — no referring files use the old basename (verified via grep).

### Deferred

Placeholder `steps/` stubs in `brainstorming`, `artifact-detection`, and `guided-brief-creation` skills are still empty. These need real content (not a mechanical fix); tracked for a scoped content PR.

Non-spec-steward hooks remain on exit-code semantics until a per-hook motivation exists for the envelope upgrade.

## 0.9.0 — Self-ingest remediation: apply lessons from claude-code + superpowers

Applies the P0/P1 lessons from running the plugin's own `brownfield-ingest` protocol against `anthropics/claude-code` and `obra/superpowers` in the `.factory/semport/` analysis. The ingest caught 3 round-1 hallucinations via strict-binary novelty, which validated both the protocol and specific gaps in the plugin itself.

### Agent frontmatter remediation (Group A — 46 files)

- **26 agent descriptions rewritten** from the boilerplate stub `VSDD factory agent: <name>` to one-sentence "Use when..." triggers drawn from each agent's body, following superpowers' CSO rule (third-person, when-not-what, <1024 chars).
- **`model:` field added to 28 agents.** Defaults to `sonnet`. Exceptions on `opus`: `adversary`, `holdout-evaluator`, `formal-verifier`, `pr-reviewer`, `spec-reviewer` — terminal reviewers where reasoning quality dominates call volume.
- **`color:` field added to all 33 root agents**, grouped by function: reviewers=red, builders=green, planners=blue, ops=yellow, research=purple.
- **`implementer.md` description** fixed (was truncated mid-sentence).
- **9 `agents/orchestrator/` include files** gained YAML frontmatter with `disable-model-invocation: true` so strict loaders no longer trip on them.
- **`excalidraw-export` and `jira` SKILLs** gained frontmatter (reference-only, `disable-model-invocation: true`).
- **`state-update` skill** marked `disable-model-invocation: true` (internal).

### deliver-story dispatch rewrite (Group B)

`skills/deliver-story/SKILL.md` was a single-context script that quietly drifted from the `agents/orchestrator/per-story-delivery.md` workflow it was supposed to use. Rewritten as a thin dispatcher:

- Declares itself a dispatcher, not an implementer, via `EXTREMELY-IMPORTANT` block.
- Iron Law: `NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST`.
- Prerequisites check that STOPs on failure (no silent bypass).
- 9-step dispatch sequence: devops-engineer → test-writer (stubs) → test-writer (tests) → **independent Red Gate verification** → implementer → demo-recorder → implementer (push) → pr-manager → devops-engineer (cleanup) → state update.
- Context discipline table naming which files each specialist receives (prevents topic drift from passing whole-story context to every agent).
- Story split recovery flow for oversized PRs.
- 10-row Red Flags table targeting the rationalizations that lead back to single-context execution.
- `agents/orchestrator/per-story-delivery.md` header marked as canonical source.

### brownfield-ingest self-improvements (Group C)

Codifies the 5 lessons the ingest protocol taught itself when applied to real reference repos:

- **Honest Convergence clause** — mandatory verbatim text in every round prompt: "<3 substantive → declare converged, emit no file." Stops agents from fabricating findings under pressure to produce SUBSTANTIVE output.
- **Known Round-1 Hallucination Classes** — 5 named failure modes (over-extrapolated token lists, miscounted enumerations, named pattern conflation, same-basename artifact conflation, inflated/deflated metrics) with verbatim examples from superpowers round 1 (persuasion matrix, Pressure Taxonomy, writing-plans forbidden tokens). Round 2+ prompts must audit round 1 against these classes.
- **Subagent Delivery Protocol (inline-by-default)** — `=== FILE: <name> ===` delimiter pattern that works around sandbox Write denials. Explicit override of subagent default system prompts that forbid "inline fallback."
- **Behavioral vs Metric split** in Phase B.6 — mandatory two-phase validation: Phase 1 samples contracts/entities for CONFIRMED/INACCURATE/HALLUCINATED (judgment); Phase 2 independently recounts every numeric claim via `find` + `wc -l` (arithmetic, not judgment). Empirical anchor: superpowers Pass 0 round 1 claimed 32 files / 5279 LOC; recount showed 23 files / 3859 LOC.
- **Priority-ordered Lessons mandate** in Phase C — synthesis MUST include a `## Lessons for <target-project>` section with P0/P1/P2/P3 buckets, each lesson naming (a) what target does today, (b) what reference does, (c) gap, (d) specific action items with file paths. Makes the synthesis a directly actionable backlog.
- **`agents/validate-extraction.md`** updated with matching operating-mode split and two-table output format.

### Iron Laws and Red Flags rollout (Group D)

Applies superpowers' empirically-anchored behavior-shaping scaffolding to the 4 highest-stakes discipline skills. Iron Laws follow the canonical form `NO <verb> <scope> WITHOUT <prerequisite> FIRST`. Each skill gained an "Announce at Start" verbatim line and a Red Flags table enumerating the rationalizations observed during pressure testing.

- **`deliver-story`** — `NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST` (+ 10 Red Flags, included in Group B rewrite)
- **`brownfield-ingest`** — `NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST` (+ 10 Red Flags)
- **`adversarial-review`** — `NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST` (+ 8 Red Flags targeting information-asymmetry violations)
- **`wave-gate`** — `NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST` (+ 8 Red Flags targeting threshold rounding, gate skipping, flake handling)

### AGENT-SOUL pragmatism footnote (Group E)

`docs/AGENT-SOUL.md` §8 "Pragmatism Over Ceremony" gained a footnote distinguishing **principled pragmatism** (design-time, human-in-loop, ROI-reasoning, documented) from **rationalization** (execution-time, bypass-a-rule). References superpowers' Pressure Taxonomy and the Meincke 2025 empirical anchor (N=28000, compliance 33%→72% under persuasion pressure) — which names "I'm just being pragmatic" as a first-class attack vector on discipline skills. This is the principle most easily weaponized to justify skipping Iron Laws; the footnote exists to stop that.

### Meta

- Reference analysis artifacts live in `.factory/semport/claude-code/` and `.factory/semport/superpowers/` (Phase A + B + C complete, validated).
- `TAKEAWAYS.md` and `PLUGIN-INVENTORY.md` in `.factory/semport/` document the analysis → remediation trace.
- No behavior changes to hooks, workflows, or bin helpers.
- No new tests yet — Group F bookkeeping only. Test coverage for the new Iron Law / Red Flags content is deferred.

## 0.8.0 — Wave 7: Validation infrastructure

Ships the test harness that validates the enforcement layer actually works. Previously, Wave 4's hooks and Wave 5/6's bin helpers had only smoke tests ("does it run without crashing"). Wave 7 adds allow/block path coverage.

- **41 TAP tests** across two bats files:
  - `tests/hooks.bats` (28 tests) — allow and block paths for every hook: brownfield-discipline (4), protect-vp (4), protect-bc (3), red-gate (6), purity-check (3), handoff-validator (3), regression-gate (3), session-learning (2)
  - `tests/bin.bats` (13 tests) — lobster-parse (3, including all 15 workflow files parse), research-cache (4, round-trip + determinism + normalization), multi-repo-scan (3), wave-state (3)
- **Smoke fixture** `fixtures/smoke-project/` — minimal Rust crate with one passing test, for future hook integration tests
- **Test runner** `tests/run-all.sh` — syntax checks + hook tests + bin tests; tool-guarded per `bash.md`
- **GitHub Actions CI** `.github/workflows/plugin-validation.yml` — runs on push/PR to main: installs bats/jq/yq, syntax-checks shell scripts, runs both test suites, validates all JSON manifests, parses every workflow file

All 41 tests pass on first run locally.

## 0.7.0 — Wave 6: Runtime helpers and not-portable documentation

Finishes the runtime-extension port. Ships bin helpers for the extensions that map to bash+jq+yq, wraps them in skills, and documents the four that cannot be ported.

**New bin helpers** (`plugins/vsdd-factory/bin/`):

- `research-cache` — SHA-keyed disk cache for Perplexity/Context7 query results at `.factory/research-cache/`. Subcommands: `get`, `put`, `has`, `key`, `clear`, `stats`. Ports `research-cache.ts`.
- `wave-state` — read-only query of `.factory/stories/sprint-state.yaml`. Subcommands: `current`, `stories`, `ready`, `summary`. Read-only slice of `wave-orchestrator.ts`.
- `multi-repo-scan` — detects multi-repo layouts from `.worktrees/`, reports repos with manifest types. Read-only slice of `multi-repo-orchestrator.ts`.

**New skill wrappers**:

- `research-cache-ops` — operates the research cache from within a session
- `wave-status` — reports wave readiness with recommendations
- `multi-repo-health` — detects multi-repo layouts and cross-checks against `.factory/stories/`

**Not-portable documentation** (`docs/not-portable.md`):

Documents why four dark-factory extensions cannot port to Claude Code's plugin primitives:

- `cost-tracker.ts` — no `PreModelCall` hook
- `attention-heatmap.ts` — no read-event hooks
- `tiered-context.ts` — Claude Code manages context natively
- `sidecar-learning.ts` (full synthesis) — `Stop` hook has no transcript access; partial marker-only port shipped in Wave 4

All bin helpers follow `bash.md`: `set -euo pipefail`, stderr guards, STDERR-EXEMPT tags, tool availability checks. Pass `bash -n` syntax checks and basic smoke tests.

Total skills: 91. Total bin helpers: 4.

## 0.6.0 — Wave 5: Orchestrator + workflow data (Lobster replacement)

Replaces dark-factory's Lobster workflow DSL with "Lobster-as-data" driven by the orchestrator agent and a bash helper.

- **Workflow corpus** — shipped all 15 `.lobster` files as data under `plugins/vsdd-factory/workflows/`:
  - Mode workflows: greenfield, brownfield, feature, maintenance, discovery, planning, multi-repo, code-delivery
  - Phase sub-workflows: phase-0-codebase-ingestion, phase-1-spec-crystallization, phase-3-tdd-implementation, phase-3.5-holdout-evaluation, phase-4-adversarial-refinement, phase-5-formal-hardening, phase-6-convergence
- **`bin/lobster-parse`** — bash helper wrapping `yq` + `jq` that emits workflow files as JSON with optional jq expressions. Lobster files parse cleanly as YAML.
- **Orchestrator agent updated** — added a Workflow Data section that points at the `workflows/` corpus and documents the lobster-parse helper with worked examples.
- **Five new skills** in `skills/`:
  - `run-phase` — execute a phase by reading its Lobster file and spawning declared sub-agents in dependency order
  - `next-step` — read `.factory/STATE.md` + active workflow, propose next action (does not execute)
  - `validate-workflow` — static schema check: required fields, agent/skill existence, depends_on graph, cycles, duplicate step names
  - `activate` — per-project opt-in that writes `{"agent": "vsdd-factory:orchestrator"}` to `.claude/settings.local.json`
  - `deactivate` — removes the agent override; leaves plugin enabled

Opt-in design (vs hijacking default persona on plugin enable) chosen per earlier decision — activation is always an explicit user action, per-project.

Total skills: 88.

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
