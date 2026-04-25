# Pass 3 Deepening — Skills Batch 3 (skills 81-119, alphabetical)

**Date:** 2026-04-25
**Pass:** 3 (Behavioral Contracts) | **Round:** Deep / Skills batch 3 / 3
**Project:** vsdd-factory (self-referential ingest — engine and product in same repo)
**Numbering:** BC-AUDIT-600..799 reserved for this batch

## 1. Round metadata

**Scope:** 39 skills under `plugins/vsdd-factory/skills/` from `pr-create` through `writing-skills` (alphabetical positions 81-119 of the 119 skills enumerated in Pass 0).

**Skills covered (39):**
pr-create, pr-review-triage, quick-dev-routing, record-demo, recover-state, register-artifact, release, repo-initialization, research, research-cache-ops, responsive-validation, run-phase, scaffold-claude-md, sdk-generation, semport-analyze, session-review, setup-env, spec-drift, spec-versioning, state-burst, state-update, storybook-mcp-integration, systematic-debugging, toolchain-provisioning, traceability-extension, track-debt, ui-completeness-check, ui-quality-gate, ux-heuristic-evaluation, validate-brief, validate-consistency, validate-template-compliance, validate-workflow, visual-companion, wave-gate, wave-scheduling, wave-status, worktree-manage, writing-skills.

**Inputs read:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md`
- All 39 SKILL.md files listed above (full read).

**Source-line citations:** Cite `<file-path>:<line-num>` with `<file-path>` = `plugins/vsdd-factory/skills/<skill>/SKILL.md`.

**BC pattern per skill:** 1 identity + 1+ behavioral + 1+ quality-gate + 1 output. Reference-only/thin skills compressed to 1 minimal BC. Target 3-8 per substantive skill.

---

## 2. BC catalog — BC-AUDIT-600..799

### A. pr-create (skill at plugins/vsdd-factory/skills/pr-create/SKILL.md)

### BC-AUDIT-600 — pr-create: identity & invocation contract

**Skill:** `plugins/vsdd-factory/skills/pr-create/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Command `/vsdd-factory:pr-create [STORY-NNN]` invoked.
**Behavior:** Generates structured PR for completed story; `disable-model-invocation: true`, `allowed-tools: Read, Bash, Glob, Grep`; argument-hint `[STORY-NNN]`.
**Acceptance:** Frontmatter declares the four allowed-tools and disables model invocation.

### BC-AUDIT-601 — pr-create: gathers story context before generating body

**Skill:** `plugins/vsdd-factory/skills/pr-create/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-30
**Trigger:** Skill running step 1 (Gather Context).
**Behavior:** Reads `.factory/stories/STORY-NNN.md`, the story's behavioral contracts, captures `git diff develop...HEAD` (in story worktree), counts tests added/modified.
**Acceptance:** PR body cannot be generated without first sourcing the four context items listed.

### BC-AUDIT-602 — pr-create: PR body must follow templated structure with mermaid + traceability table

**Skill:** `plugins/vsdd-factory/skills/pr-create/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-16, 32-74
**Trigger:** Step 2 (Generate PR Body).
**Behavior:** PR body follows `templates/pr-description-template.md` and includes Summary, Behavioral Contracts table (Contract / Status), Changes, Dependency Diagram (mermaid), Test Plan, TDD Evidence sections.
**Acceptance:** Generated PR body contains all six sections and references the BC-table format.

### BC-AUDIT-603 — pr-create: PR creation targets develop with feat-prefixed title

**Skill:** `plugins/vsdd-factory/skills/pr-create/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 78-83
**Trigger:** Step 3 (Create PR).
**Behavior:** Runs `gh pr create --title "feat(STORY-NNN): <title>" --body <generated> --base develop`.
**Acceptance:** `--base develop` is non-overridable; title carries `feat(STORY-NNN):` prefix.

### BC-AUDIT-604 — pr-create: post-creation report includes URL + next steps

**Skill:** `plugins/vsdd-factory/skills/pr-create/SKILL.md`
**Confidence:** MEDIUM
**Source line(s):** 85-87
**Trigger:** Step 4 (Report).
**Behavior:** Reports PR URL plus next steps (review, holdout evaluation if wave complete).
**Acceptance:** Output references PR URL string; mentions reviewer/holdout next-step.

---

### B. pr-review-triage

### BC-AUDIT-605 — pr-review-triage: identity & dispatch role

**Skill:** `plugins/vsdd-factory/skills/pr-review-triage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** pr-manager invokes after pr-reviewer posts REQUEST_CHANGES.
**Behavior:** Classifies each finding and routes to fix agent.
**Acceptance:** Skill is referenced by pr-manager only; not user-invoked.

### BC-AUDIT-606 — pr-review-triage: classification table is complete and exhaustive

**Skill:** `plugins/vsdd-factory/skills/pr-review-triage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 36-49
**Trigger:** Step 2 (Classify and Route) for each finding.
**Behavior:** Maps (category × severity) to a fix route per the explicit 12-row table covering coherence/coverage/description/size/missing/dependency.
**Acceptance:** Every finding maps to exactly one row of the routing table.

### BC-AUDIT-607 — pr-review-triage: size-blocking finding STOPS pr-manager

**Skill:** `plugins/vsdd-factory/skills/pr-review-triage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 43, 67-68
**Trigger:** Finding with category=size and severity=blocking.
**Behavior:** pr-manager STOPS the review loop and returns to orchestrator with story split recommendation.
**Acceptance:** No further routing/dispatch occurs in that cycle when size-blocking is present.

### BC-AUDIT-608 — pr-review-triage: ten-cycle escalation cap

**Skill:** `plugins/vsdd-factory/skills/pr-review-triage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 67
**Trigger:** Reaching cycle 10 with blocking findings still open.
**Behavior:** Escalate to human.
**Acceptance:** Convergence table shows escalation row at cycle 10 (or earlier per table semantics).

### BC-AUDIT-609 — pr-review-triage: writes review-findings.md with cycle row + triage table

**Skill:** `plugins/vsdd-factory/skills/pr-review-triage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-64, 73-83
**Trigger:** Step 4 (Update Convergence Tracking) and final output.
**Behavior:** Updates `.factory/code-delivery/STORY-NNN/review-findings.md` with new cycle row and triage routing table.
**Acceptance:** Quality-gate criteria require: every finding classified, every finding routed, no findings unclassified, convergence table updated.

---

### C. quick-dev-routing

### BC-AUDIT-610 — quick-dev-routing: identity & qualification gate

**Skill:** `plugins/vsdd-factory/skills/quick-dev-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-31
**Trigger:** Phase F1 delta analysis flags trivial-scope change.
**Behavior:** All seven qualification criteria (files changed ≤3, new ≤2, modified ≤1 internal, dependents 0, no architecture impact, no security impact, zero blast radius) must hold; otherwise route to full F1-F7.
**Acceptance:** A change with any criterion failing routes to standard Feature Mode (F1-F7).

### BC-AUDIT-611 — quick-dev-routing: multi-goal detection precedes routing

**Skill:** `plugins/vsdd-factory/skills/quick-dev-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-52
**Trigger:** Before committing to a route.
**Behavior:** Detects multi-goal signals; if found, presents Split (`S`)/Keep (`K`) prompt to human; on split, creates separate feature requests.
**Acceptance:** Routing decision records the multi-goal detection result and chosen path.

### BC-AUDIT-612 — quick-dev-routing: compressed pipeline preserves regression + adversary + human merge

**Skill:** `plugins/vsdd-factory/skills/quick-dev-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 54-72
**Trigger:** Quick Dev or One-Shot route selected.
**Behavior:** Skips F2/F3/F6 only. Always preserved: full regression suite (F4), at least one adversarial review pass (F5), human merge authorization (F7).
**Acceptance:** No skip is permitted on regression/adversary/human-merge axes regardless of size.

### BC-AUDIT-613 — quick-dev-routing: writes routing-decision.md and falls back on regression failure

**Skill:** `plugins/vsdd-factory/skills/quick-dev-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 86-97
**Trigger:** End of routing or regression failure mid-flight.
**Behavior:** Writes `.factory/phase-f1-delta-analysis/routing-decision.md` with qualification assessment, multi-goal result, route choice, compressed plan; if regression fails post-Quick-Dev, escalate to full F4.
**Acceptance:** routing-decision.md exists with all four sections; regression failure routes back to full F4 with regression-log.md.

---

### D. record-demo

### BC-AUDIT-614 — record-demo: identity & template usage

**Skill:** `plugins/vsdd-factory/skills/record-demo/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-19
**Trigger:** `/vsdd-factory:record-demo STORY-NNN`.
**Behavior:** disable-model-invocation true, allowed-tools Read/Write/Bash/Glob; reads three templates (demo-evidence-report, demo-playwright-template.spec.ts, demo-tape-template.tape).
**Acceptance:** Frontmatter and template references match the three named templates.

### BC-AUDIT-615 — record-demo: per-AC evidence capture (CLI vs web)

**Skill:** `plugins/vsdd-factory/skills/record-demo/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-56
**Trigger:** For each acceptance criterion in the story.
**Behavior:** CLI: `script` or `asciinema`; web: Playwright MCP — navigate, act, screenshot to `.factory/demo-evidence/STORY-NNN/AC-<N>.png`.
**Acceptance:** Each AC has at least one evidence artifact (cast, png, or text fallback).

### BC-AUDIT-616 — record-demo: writes demo-report.md with per-AC table

**Skill:** `plugins/vsdd-factory/skills/record-demo/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 58-77
**Trigger:** Step 3 after all ACs captured.
**Behavior:** Writes `.factory/demo-evidence/STORY-NNN/demo-report.md` with per-AC method/evidence/result/notes, plus summary (criteria demonstrated count, missing list).
**Acceptance:** Report file exists at the named path with mandatory sections.

### BC-AUDIT-617 — record-demo: tool-unavailable fallback never skips evidence

**Skill:** `plugins/vsdd-factory/skills/record-demo/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 88-96
**Trigger:** Recording tools (asciinema, Playwright, termshot) not available.
**Behavior:** Creates text-based evidence report describing steps and observed behavior; never skips demo evidence.
**Acceptance:** Even with no tools, demo-report.md is produced with text evidence per AC.

### BC-AUDIT-618 — record-demo: commits evidence to factory-artifacts

**Skill:** `plugins/vsdd-factory/skills/record-demo/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 79-85
**Trigger:** Step 4 after report written.
**Behavior:** Runs `git add demo-evidence/STORY-NNN/` + commit `factory(phase-3): demo evidence for STORY-NNN` on `.factory` worktree.
**Acceptance:** Single commit per story tagged with phase-3 prefix.

---

### E. recover-state

### BC-AUDIT-619 — recover-state: identity, dry-run, and backup

**Skill:** `plugins/vsdd-factory/skills/recover-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-33
**Trigger:** STATE.md missing/corrupted/out-of-sync; or `--dry-run` flag.
**Behavior:** Backs up existing STATE.md to `STATE.md.backup-YYYY-MM-DD-HHMMSS` if present; `--dry-run` skips writes and only reports.
**Acceptance:** No write occurs in dry-run; backup created in non-dry-run when prior file exists.

### BC-AUDIT-620 — recover-state: artifact directory probe table is exhaustive

**Skill:** `plugins/vsdd-factory/skills/recover-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 34-52
**Trigger:** Step 2 (Scan artifact directories).
**Behavior:** Probes each row of the 12-row scan table (product-brief, domain-spec, prd, behavioral-contracts, architecture, verification-properties, dtu-assessment, stories, sprint-state, cycles, holdout-scenarios, current-cycle).
**Acceptance:** Each present artifact directory contributes a determination signal as enumerated.

### BC-AUDIT-621 — recover-state: phase decision tree is total and ordered

**Skill:** `plugins/vsdd-factory/skills/recover-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-71
**Trigger:** Step 3 (Determine current phase).
**Behavior:** Applies the explicit decision tree (no brief → pre-pipeline; brief only → 1a; …; convergence-report → 6); story status branches inside Phase 3.
**Acceptance:** Reconstructed phase matches a single line of the published tree.

### BC-AUDIT-622 — recover-state: requires user approval before write

**Skill:** `plugins/vsdd-factory/skills/recover-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 113-148
**Trigger:** Step 8 (Validate reconstruction) before Step 9 (Write).
**Behavior:** Presents reconstructed snapshot to user with phase progress, artifact counts, DTU; prompts approve/adjust; only writes on approval.
**Acceptance:** STATE.md is never written without an explicit approve.

### BC-AUDIT-623 — recover-state: documented limitations are honored (no fabrication)

**Skill:** `plugins/vsdd-factory/skills/recover-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 149-164
**Trigger:** Reconstruction time.
**Behavior:** Decisions Log, Skip Log, Blocking Issues, Current Phase Steps, cost data left empty (cannot be reconstructed from artifacts). Read-only on artifacts; does NOT commit STATE.md.
**Acceptance:** Reconstructed STATE.md leaves the five enumerated sections empty; commit is delegated to state-manager.

---

### F. register-artifact

### BC-AUDIT-624 — register-artifact: identity & justification

**Skill:** `plugins/vsdd-factory/skills/register-artifact/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** New artifact file (BC, VP, story, holdout) just created.
**Behavior:** Appends INDEX row; exists to prevent orphan artifacts that fail consistency-validator criterion 23.
**Acceptance:** Skill is referenced from agents (story-writer/product-owner/architect) and the consistency check uses INDEX presence.

### BC-AUDIT-625 — register-artifact: type identification by path pattern (4-row table)

**Skill:** `plugins/vsdd-factory/skills/register-artifact/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 17-28
**Trigger:** Step 1 path parsing.
**Behavior:** Maps `behavioral-contracts/BC-*.md` → BC; `verification-properties/VP-*.md` → VP; `stories/STORY-*.md` → Story; `holdout-scenarios/HS-*.md` → HS. Unrecognized → reports unrecognized-type error.
**Acceptance:** Exactly one of the four artifact types is selected, or type-error is reported.

### BC-AUDIT-626 — register-artifact: idempotent (refuses duplicate ID)

**Skill:** `plugins/vsdd-factory/skills/register-artifact/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 71-77
**Trigger:** Step 4 duplicate check.
**Behavior:** Reads INDEX, checks if artifact ID already has a row; if yes, reports "<ID> already registered" and skips.
**Acceptance:** No duplicate row is appended.

### BC-AUDIT-627 — register-artifact: refuses to create INDEX file (separation of concerns)

**Skill:** `plugins/vsdd-factory/skills/register-artifact/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 65-69, 130-135
**Trigger:** Step 3 INDEX existence check or out-of-scope guard.
**Behavior:** If INDEX missing, report and stop; do NOT create INDEX (creator agents own that), do NOT create the artifact file, do NOT update story frontmatter, do NOT validate content.
**Acceptance:** Skill returns error rather than creating an INDEX from scratch.

### BC-AUDIT-628 — register-artifact: batch mode aggregates results

**Skill:** `plugins/vsdd-factory/skills/register-artifact/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 110-122
**Trigger:** `$ARGUMENTS` contains multiple space-separated paths.
**Behavior:** Processes each path, reports summary (Registered N artifacts, Skipped K already registered) at end.
**Acceptance:** Output message includes both registered and skipped counts.

---

### G. release

### BC-AUDIT-629 — release: identity, modes, factory worktree pre-flight

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-34
**Trigger:** `/vsdd-factory:release [init|<version>|--dry-run]`.
**Behavior:** Three modes: Bootstrap (`init`), Dry Run (`--dry-run`), Release (else). Pre-flight ensures `.factory/` worktree exists; if `factory-artifacts` branch missing, triggers Bootstrap.
**Acceptance:** Mode selection follows verbatim parse rules; release-config.yaml read precedes any other action.

### BC-AUDIT-630 — release: announces protocol verbatim

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-22
**Trigger:** Skill start, before any other action.
**Behavior:** Says: "**Release Pipeline** — reading release config from `.factory/release-config.yaml`."
**Acceptance:** First user-visible line is the verbatim announce.

### BC-AUDIT-631 — release: bootstrap detects markers across five project types

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 49-66
**Trigger:** Bootstrap Mode Step 1.
**Behavior:** Scans for `Cargo.toml`, `package.json`, `pyproject.toml`, `plugins/*/.claude-plugin/plugin.json`, `go.mod` and matching publish/version conventions; also detects test scripts, CHANGELOG, CI workflows, README badge, marketplace.json.
**Acceptance:** Generated config covers any markers found; absent markers do not appear.

### BC-AUDIT-632 — release: version bump determined from story types when no explicit version

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 96-107
**Trigger:** Release Mode Step 2 with no explicit version arg.
**Behavior:** `type: feat` → MINOR, only `type: fix` → PATCH, any `breaking_change: true` → MAJOR; otherwise prompt user. Never bumps without confirmation.
**Acceptance:** Bump category derives from frontmatter rules in priority order; user confirmation required.

### BC-AUDIT-633 — release: quality-gate modes (standard/vsdd-partial/vsdd-full)

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 109-128
**Trigger:** Release Mode Step 3.
**Behavior:** `standard` skips to Step 4. `vsdd-partial`/`vsdd-full` checks each enabled gate (convergence, holdout, formal verification, adversarial passes, human approval) and aborts on failure with expected vs actual.
**Acceptance:** Failed gate halts before any version bump.

### BC-AUDIT-634 — release: per-format version-bump dispatch (json/toml/yaml/regex)

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 142-156
**Trigger:** Step 5 (Bump Versions).
**Behavior:** For each entry in `version_sources`/`global_version_sources`: `json`→`jq`, `toml`→`sed`/toml, `yaml`→`yq`, `regex`→`sed`. Reports each file updated.
**Acceptance:** No version source updated by an unsupported format handler.

### BC-AUDIT-635 — release: tag, push (with confirm), CI watch, gh-release verify

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 169-216
**Trigger:** Steps 7-11 of Release Mode.
**Behavior:** Commit `chore: release vX.Y.Z`; tag `vX.Y.Z`; ASK before push; push commit + tag; if `ci_workflow` configured, `gh run watch --exit-status`; verify GitHub Release with CHANGELOG content; create release if CI didn't.
**Acceptance:** No push happens without user yes; CI failure surfaces a link and does NOT re-tag.

### BC-AUDIT-636 — release: dry-run produces complete plan with no side effects

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 230-256
**Trigger:** `--dry-run` mode.
**Behavior:** Prints version bump, files to update, CHANGELOG plan, commit/tag/push intent, CI workflow ref, publish plan, gates mode, pre-release checks list — without changing any file.
**Acceptance:** Filesystem and git state unchanged after dry-run completion.

### BC-AUDIT-637 — release: error-handling catalog

**Skill:** `plugins/vsdd-factory/skills/release/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 259-266
**Trigger:** Each named failure (unknown schema, unknown gate mode, pre-release fail, version bump fail, push fail, CI fail).
**Behavior:** Aborts with specific message; never retries push or re-tags on CI failure.
**Acceptance:** Each enumerated failure returns the documented message and abort behavior.

---

### H. repo-initialization

### BC-AUDIT-638 — repo-initialization: identity & delegation reference

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** Orchestrator detects no repo for new project (or human requests creation).
**Behavior:** Skill describes work the orchestrator delegates to specialist agents; orchestrator does NOT execute these steps directly.
**Acceptance:** Each step names a target specialist agent (orchestrator/architect/devops-engineer/dx-engineer).

### BC-AUDIT-639 — repo-initialization: workspace-isolation guard refuses dark-factory cwd

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 81-91
**Trigger:** devops-engineer step before any repo/git command.
**Behavior:** If `pwd` contains `dark-factory`, exit FATAL with prescribed remedy.
**Acceptance:** Guard runs before `gh repo create`/`git clone`/`git config`.

### BC-AUDIT-640 — repo-initialization: develop branch is the protected default

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 45-48, 124-141
**Trigger:** Step 1 (gather requirements) and Step 3 (create + protect).
**Behavior:** Default branch name is `develop` (VSDD standard, NOT main); branch protection applied via `gh api repos/.../branches/develop/protection`.
**Acceptance:** New repos report `develop` as protected default.

### BC-AUDIT-641 — repo-initialization: factory-artifacts orphan worktree pre-check (NOT dark-factory)

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 146-172
**Trigger:** Step 3 item 8 worktree setup.
**Behavior:** Pre-check verifies `git remote get-url origin` does NOT contain `dark-factory`. After `git worktree add .factory factory-artifacts`, post-verify: `rev-parse --git-dir` succeeds, `branch --show-current` is `factory-artifacts`, `.factory/.git` gitdir does not point into engine repo.
**Acceptance:** All three post-checks must pass; otherwise FATAL.

### BC-AUDIT-642 — repo-initialization: architect signal table for multi-vs-single-repo

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 49-75
**Trigger:** Step 2 (Architect Recommends Repo Strategy).
**Behavior:** Weighted signal tables: deployment targets / tech stacks / release cycles / team boundaries / shared contracts / SOA favor multi-repo; single deployment target / single stack / tight coupling / single team / simple product favor single-repo.
**Acceptance:** Recommendation cites the strongest matching signal(s).

### BC-AUDIT-643 — repo-initialization: multi-repo creates .factory-project/ + project.yaml + per-service repos

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 189-274
**Trigger:** Architect's `deployment_topology: multi-service` + human confirms multi-repo.
**Behavior:** Creates `.factory-project/` orphan worktree on `factory-project-artifacts`; per-service repos with develop/branch protection/git rerere/.factory worktree each; emits `project.yaml` with primary_repo, repos[], dependency_graph; .factory-project/ has STATE.md, cost/, integration/, specs/, wave-plans/.
**Acceptance:** All four artifacts (worktree, project.yaml, per-service repos, .factory-project structure) are produced.

### BC-AUDIT-644 — repo-initialization: dx-engineer environment setup (DF-027)

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 275-347
**Trigger:** Step 5 after repo creation.
**Behavior:** Installs direnv; creates committed `.envrc` (`dotenv .env`) and `.env.example` (empty at init, populated during DTU); updates `.gitignore` for `.env`/`.env.local`; runs `direnv allow .`; installs `mcporter`; configures perplexity/context7/playwright MCP servers; LLM health check (3 model families); MCP preflight.
**Acceptance:** All eight numbered substeps completed; LLM health blocks if any model unavailable.

### BC-AUDIT-645 — repo-initialization: CI/CD setup is deferred to post-architecture

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 182-187
**Trigger:** Repo init time before architecture exists.
**Behavior:** CI/CD pipeline creation is deferred until after architecture is produced (Phase 1, P1-05). Devops-engineer creates workflows in separate `phase-1-cicd-setup` step.
**Acceptance:** No `.github/workflows/` is created at repo-initialization time.

### BC-AUDIT-646 — repo-initialization: outputs

**Skill:** `plugins/vsdd-factory/skills/repo-initialization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 348-368
**Trigger:** Skill completion.
**Behavior:** Reports presence of: GitHub repo with develop+branch protection, local clone, `.gitignore`, `.factory/merge-config.yaml`, `.envrc` (direnv allowed), `.env.example`, `.factory/` worktree, mcporter+MCP servers, LLM health pass, git rerere enabled. Excalidraw MCP for UI products.
**Acceptance:** Final report enumerates all bullet artifacts that were produced.

---

### I. research

### BC-AUDIT-647 — research: identity & sub-agent fork

**Skill:** `plugins/vsdd-factory/skills/research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** `/research [domain|general] <topic>`.
**Behavior:** Spawns research-agent in forked context (disable-model-invocation true, agent: research-agent).
**Acceptance:** Frontmatter declares `context: fork` and `agent: research-agent`.

### BC-AUDIT-648 — research: domain vs general routing by first arg

**Skill:** `plugins/vsdd-factory/skills/research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-43
**Trigger:** Argument parsing.
**Behavior:** First word `domain` → produces `.factory/specs/research/domain-<slug>-<YYYY-MM-DD>.md`. Anything else → general → `general-<slug>-<YYYY-MM-DD>.md`.
**Acceptance:** Output filename prefix matches (`domain-` vs `general-`).

### BC-AUDIT-649 — research: pre-run cache scan + post-run index update + factory commit

**Skill:** `plugins/vsdd-factory/skills/research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 46-64
**Trigger:** Before/after research-agent invocation.
**Behavior:** Before: ensure `.factory/specs/research/` exists; read RESEARCH-INDEX.md (if exists) and share prior research. After: verify `## Research Methods` section listing MCP tools; append to RESEARCH-INDEX.md; commit on factory-artifacts with `factory(research): <type> — <topic>`.
**Acceptance:** Methods section present; index appended; commit message prefix matches.

---

### J. research-cache-ops

### BC-AUDIT-650 — research-cache-ops: identity & wraps research-cache binary

**Skill:** `plugins/vsdd-factory/skills/research-cache-ops/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** Cache inspection or invalidation request.
**Behavior:** Wraps `${CLAUDE_PLUGIN_ROOT}/bin/research-cache` operations.
**Acceptance:** All operations invoke the named binary.

### BC-AUDIT-651 — research-cache-ops: six operations (stats/key/has/get/put/clear)

**Skill:** `plugins/vsdd-factory/skills/research-cache-ops/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 10-17
**Trigger:** Operation-name argument.
**Behavior:** stats, key, has, get, put, clear with documented exit-code semantics (`has` exit 0 if present).
**Acceptance:** Skill exposes exactly these six operations.

### BC-AUDIT-652 — research-cache-ops: SHA-256 deterministic key + research-agent integration pattern

**Skill:** `plugins/vsdd-factory/skills/research-cache-ops/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 12, 26-37
**Trigger:** Research-agent makes Perplexity/Context7 call.
**Behavior:** Computes deterministic SHA-256 key for query; pseudocode shows `key=$(... key "$query")`; `has` short-circuits to `get`; on miss, run real query and `put` result.
**Acceptance:** Same query yields same key; cached result reused on second invocation.

---

### K. responsive-validation

### BC-AUDIT-653 — responsive-validation: identity, agents, conditional UI gating

**Skill:** `plugins/vsdd-factory/skills/responsive-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** `feature_type in ['ui', 'full-stack']`.
**Behavior:** Primary e2e-tester, supporting visual-reviewer; inputs `.factory/ui-traceability.yaml` + `.factory/design-system/tokens/sizing.json`; outputs `.factory/ui-evidence/SCR-NNN/` + `.factory/ui-quality/responsive-report.md`.
**Acceptance:** Skill is no-op when feature_type is non-UI.

### BC-AUDIT-654 — responsive-validation: 4 mandatory breakpoints (375/768/1024/1440)

**Skill:** `plugins/vsdd-factory/skills/responsive-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-52, 117
**Trigger:** Implementation of breakpoint test suite.
**Behavior:** Mobile 375 (no horizontal scroll, touch ≥48px, text ≥16px, navigation usable); Tablet 768 (layout adapts, sidebar correct, tables scrollable/stacked); Desktop 1024 (full layout, keyboard nav, hover, multi-column); Wide 1440 (max-width, whitespace, line length <80ch).
**Acceptance:** Quality gate: all 4 breakpoints tested for every screen.

### BC-AUDIT-655 — responsive-validation: critical-failure list is blocking

**Skill:** `plugins/vsdd-factory/skills/responsive-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 84-90
**Trigger:** Detection of any blocking failure.
**Behavior:** Horizontal scroll, touch <48px, text <14px, unusable nav, content overflow/cropping at any breakpoint = blocking.
**Acceptance:** Any one of these failures blocks the gate.

### BC-AUDIT-656 — responsive-validation: screenshot evidence + per-screen pass/fail matrix

**Skill:** `plugins/vsdd-factory/skills/responsive-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 70-115
**Trigger:** Per-screen test cycle.
**Behavior:** Stores screenshots at `.factory/ui-evidence/SCR-NNN/{mobile-375,tablet-768,desktop-1024,wide-1440,CMP-NNN-default,CMP-NNN-loading,CMP-NNN-error}.png`; emits `responsive-report.md` with per-screen P/F matrix.
**Acceptance:** Quality gate: screenshots captured per screen; report shows pass-rate %.

### BC-AUDIT-657 — responsive-validation: failure modes (resize/screenshot/auth)

**Skill:** `plugins/vsdd-factory/skills/responsive-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 124-128
**Trigger:** Tool/page failure mid-test.
**Behavior:** Resize fail → log, skip, flag in report; screenshot fail → retry once, then mark missing evidence; unreachable screen (auth/broken route) → flag untestable, report to orchestrator.
**Acceptance:** Each failure mode has documented recovery; skill never silently drops a screen.

---

### L. run-phase

### BC-AUDIT-658 — run-phase: identity & resolution rules

**Skill:** `plugins/vsdd-factory/skills/run-phase/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-27
**Trigger:** `/vsdd-factory:run-phase <phase-id>`.
**Behavior:** Argument is one of the 8 phase-* IDs or one of the 8 mode workflows (greenfield, brownfield, feature, maintenance, discovery, planning, multi-repo, code-delivery). Phase IDs resolve to `workflows/phases/<id>.lobster`; modes to `workflows/<id>.lobster`.
**Acceptance:** Failure to resolve produces actionable error with file path.

### BC-AUDIT-659 — run-phase: validates workflow before execution

**Skill:** `plugins/vsdd-factory/skills/run-phase/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-39
**Trigger:** Step 2 (Validate the workflow).
**Behavior:** Calls `bin/lobster-parse` with `.workflow | {name, version, steps: (.steps | length)}`; confirms name + ≥1 step.
**Acceptance:** Workflow with no name or zero steps fails before execution.

### BC-AUDIT-660 — run-phase: topological execution honors depends_on

**Skill:** `plugins/vsdd-factory/skills/run-phase/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 35-46
**Trigger:** Step 3-4 (Enumerate / Execute).
**Behavior:** Extracts steps in dependency order; per type: `agent` → spawn via Task tool with step.task as prompt; `skill` → invoke Skill; `command` → run via Bash; on failure honors `on_failure` (escalate/retry/skip) and `max_retries`.
**Acceptance:** No step runs before its `depends_on` predecessors.

### BC-AUDIT-661 — run-phase: STATE.md update + final summary

**Skill:** `plugins/vsdd-factory/skills/run-phase/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-50, 52-56
**Trigger:** After each step / end of run.
**Behavior:** Appends timestamped line to `.factory/STATE.md` with phase, step name, outcome; if STATE.md missing, halt and tell user to run `/vsdd-factory:factory-health`. Final report: phase name, step count, passes, failures, elapsed wall time. Non-goals: do not edit workflow file, do not invent steps, do not skip depends_on.
**Acceptance:** STATE.md grows by N lines for an N-step phase; summary contains all four metrics.

---

### M. scaffold-claude-md

### BC-AUDIT-662 — scaffold-claude-md: identity & overwrite confirmation

**Skill:** `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** `/vsdd-factory:scaffold-claude-md`.
**Behavior:** If existing CLAUDE.md present, displays it and asks overwrite-or-cancel; on cancel, stops.
**Acceptance:** Existing CLAUDE.md never overwritten without explicit user confirmation.

### BC-AUDIT-663 — scaffold-claude-md: four detectors run in priority order

**Skill:** `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 17-95
**Trigger:** Detection phase.
**Behavior:** D1 Language/Toolchain (7-row marker table); D2 Build/Test/Lint priority — Justfile→Makefile→CI workflows→toolchain defaults→formatter/linter configs; D3 Git Workflow (default branch, git-flow, branch patterns, commit conventions, AI attribution); D4 Project References (8-row reference table).
**Acceptance:** Each detector outputs a section, with TODO placeholder if nothing found (never omit).

### BC-AUDIT-664 — scaffold-claude-md: CLAUDE.md does NOT duplicate plugin methodology

**Skill:** `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 7-9, 119-122
**Trigger:** Assembly phase.
**Behavior:** Output structure: Build & Test, Git Workflow, Project References. Do NOT add `@SOUL.md`, `@.claude/rules/`, or VSDD methodology — vsdd-factory plugin provides those.
**Acceptance:** Generated CLAUDE.md contains zero references to SOUL.md / rules/ / VSDD phases.

### BC-AUDIT-665 — scaffold-claude-md: present-confirm-write loop

**Skill:** `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 124-145
**Trigger:** Final assembly.
**Behavior:** Present in fenced markdown block; user can request changes (apply, re-present, ask again); only writes after approval. Reports file path written, TODO count, regeneration reminder.
**Acceptance:** Write occurs only post-approval; user-requested edits applied before write.

---

### N. sdk-generation

### BC-AUDIT-666 — sdk-generation: identity & trigger triad

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-25
**Trigger:** Contracts approved (story decomposition); contract change detected (Feature Mode); periodic check (Maintenance Mode).
**Behavior:** Generates SDKs (TS/Python/Go/proto) from API contracts; idiomatic per language.
**Acceptance:** Each of three triggers invokes the same workflow.

### BC-AUDIT-667 — sdk-generation: contract validation gates generation

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-38, 380-381
**Trigger:** Step 1 (Read Contract).
**Behavior:** Reads source from `project.yaml`; runs `npx @apidevtools/swagger-cli validate "$CONTRACT_PATH"`; if validation fails, halt and report invalid contract to orchestrator.
**Acceptance:** Generation never starts on an invalid contract.

### BC-AUDIT-668 — sdk-generation: language idiom enforcement (TS async, Py snake_case, Go errors)

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 5-7, 53-119, 372-377
**Trigger:** Per-language Step 3 generation.
**Behavior:** TypeScript: async/await + types + retry; Python: snake_case + Pydantic + httpx async; Go: idiomatic error returns + context. Quality gate requires "follows target language idioms".
**Acceptance:** Generated SDK passes per-language idiom checklist (axe of: async/await, snake_case, error returns).

### BC-AUDIT-669 — sdk-generation: tool-format dispatch (OpenAPI/protobuf/GraphQL)

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 40-49, 121-140
**Trigger:** Step 2 (Select Generation Tool).
**Behavior:** OpenAPI → Fern (default), Speakeasy, OpenAPI Generator; protobuf → Buf+protoc / grpc-tools; GraphQL → graphql-codegen / Apollo Codegen.
**Acceptance:** Tool selection follows the explicit mapping table.

### BC-AUDIT-670 — sdk-generation: contract-test integration (Pact / Specmatic / Schemathesis / openapi-diff)

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 175-260
**Trigger:** Validation step + Pipeline points table.
**Behavior:** Consumer-driven (Pact pattern); Specification-driven (Specmatic); Diff-based (openapi-diff/Optic); Generative (Schemathesis). Integrated at implementation, holdout, adversarial, F-modes.
**Acceptance:** Contract testing role is documented at each pipeline point per the table.

### BC-AUDIT-671 — sdk-generation: contract evolution (semver + breaking detection)

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 298-345
**Trigger:** Contract version change.
**Behavior:** MAJOR for breaking (remove endpoint/required field, type change, auth change); MINOR for additions (new endpoint, optional field, optional query param, new error status); PATCH for corrections. Detection: openapi-diff / `buf breaking` / graphql-inspector diff.
**Acceptance:** Breaking changes route to consumer notification + SDK regeneration + DF-012 integration gate.

### BC-AUDIT-672 — sdk-generation: outputs and quality gate

**Skill:** `plugins/vsdd-factory/skills/sdk-generation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 167-174, 372-383
**Trigger:** Workflow completion.
**Behavior:** Outputs: `[sdk-repo]/src/generated/`, package metadata, `tests/`, `README.md`, `.factory-project/sdk-generation-report.md`. Quality gate: compiles, contract tests pass, idioms followed, report produced.
**Acceptance:** All five output artifacts present; gate criteria PASS.

---

### O. semport-analyze

### BC-AUDIT-673 — semport-analyze: identity, two modes, reference resolution

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-28
**Trigger:** `/semport-analyze [source-path] [target-language] [--incremental <module>]`.
**Behavior:** Two modes: Full Ingestion (default) or Incremental (`--incremental`). If arg matches `.reference/` directory name, resolves to `.reference/<project>/`; canonical inventory in `.factory/reference-manifest.yaml`.
**Acceptance:** Mode selection, target-language arg, and `.reference/` resolution all correct.

### BC-AUDIT-674 — semport-analyze: incremental protocol is delta-only (5 steps)

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 39-56
**Trigger:** `--incremental <module>` mode.
**Behavior:** (1) read existing pass-6-synthesis.md, (2) scan only specified module + direct dependencies, (3) produce delta (changed/new/removed), (4) update existing pass files (append, don't replace), (5) write `.factory/semport/<project>/<project>-delta-<module>-<date>.md`.
**Acceptance:** No re-scan of full codebase; existing pass files only appended.

### BC-AUDIT-675 — semport-analyze: full mode runs codebase-analyzer 6-pass protocol

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 58-69
**Trigger:** Default (Full Ingestion).
**Behavior:** Six passes: (1) Inventory, (2) Architecture, (3) Domain Model, (4) Behavioral Contracts, (5) NFR Extraction, (6) Synthesis. All outputs write to `.factory/semport/`.
**Acceptance:** Six pass files exist (pass-0 through pass-6) per project.

### BC-AUDIT-676 — semport-analyze: language idiom mapping table (Python/TS/Rust pairs)

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 70-91
**Trigger:** Phase 2 (Target Language Design).
**Behavior:** Documents Python→Rust, TypeScript→Rust, Python→TypeScript idiom mappings (dataclass→struct, generator→Iterator, decorator→proc macro, etc.).
**Acceptance:** Per-module target design references the appropriate row of this table.

### BC-AUDIT-677 — semport-analyze: validate-extraction agent post-pass (max 3 iterations)

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 92-99
**Trigger:** After analysis.
**Behavior:** Spawns validate-extraction agent to verify: BCs match code, domain model aligns with tests, no hallucinated dependencies; max 3 refinement iterations.
**Acceptance:** Validation result is included; refinement halts at iteration 3.

### BC-AUDIT-678 — semport-analyze: outputs and post-skill report

**Skill:** `plugins/vsdd-factory/skills/semport-analyze/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 100-124
**Trigger:** End of skill.
**Behavior:** Full mode: pass-0..pass-8 files + `<module>-target-design.md` per module + `semport-assessment.md`. Incremental: delta file + updated target designs. Reports mode, passes complete, modules analyzed, BC counts (HIGH/MEDIUM/LOW), validation result.
**Acceptance:** Output report matches the named template fields.

---

### P. session-review

### BC-AUDIT-679 — session-review: identity, trigger, primary agent

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-38
**Trigger:** Final step in every top-level Lobster workflow, after all pipeline work (including release), before pipeline marked DONE.
**Behavior:** Primary agent session-review (T1, adversary model); supporting state-manager.
**Acceptance:** No top-level workflow can mark COMPLETE before this skill runs.

### BC-AUDIT-680 — session-review: 8 analysis dimensions

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 75-86
**Trigger:** Analysis phase.
**Behavior:** Cost, Timing, Convergence, Agent Behavior, Gate Outcome, Wall Integrity, Quality Signal, Pattern Detection.
**Acceptance:** Review report has eight named sections.

### BC-AUDIT-681 — session-review: 10 proposal categories with routing

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 88-127
**Trigger:** Proposal generation + routing on approval.
**Behavior:** Categories: cost, timing, convergence, agent, gate, wall, quality, pattern, workflow, template. Each routes to a specific destination (model config, gate timeouts, adversary prompt, AGENTS.md, test suite, lobster context.exclude, mutation kill rate target, research task, story in dark-factory repo, template file).
**Acceptance:** Proposal carries category + route per the explicit table.

### BC-AUDIT-682 — session-review: 72h non-blocking timeout

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 128-132
**Trigger:** No human response within 72h.
**Behavior:** All proposals auto-defer to backlog; pipeline marked COMPLETE regardless. Session review never blocks pipeline completion indefinitely.
**Acceptance:** Pipeline COMPLETE state achievable without manual approval.

### BC-AUDIT-683 — session-review: cross-run pattern database + benchmarks

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 134-146
**Trigger:** Run completion.
**Behavior:** Maintains `.factory/session-reviews/{review-YYYY-MM-DD-[run-id].md, improvement-proposals-[run-id].md, improvement-backlog.md, pattern-database.yaml, benchmarks.yaml}`. Tracks self-cost; flags if >5% of pipeline cost.
**Acceptance:** Five named files exist after first run; subsequent runs append.

### BC-AUDIT-684 — session-review: failure-mode safety (incomplete logs / missing cost / corrupt pattern db)

**Skill:** `plugins/vsdd-factory/skills/session-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 154-158
**Trigger:** Each named failure.
**Behavior:** Incomplete logs → analyze available, document gaps. No cost data → skip cost dimension, note. Corrupt pattern-database.yaml → start fresh, flag for recovery.
**Acceptance:** Each failure mode produces a defined fallback rather than crashing.

---

### Q. setup-env

### BC-AUDIT-685 — setup-env: identity & frontmatter

**Skill:** `plugins/vsdd-factory/skills/setup-env/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** `/vsdd-factory:setup-env`.
**Behavior:** disable-model-invocation true; allowed-tools Bash/Read/Write.
**Acceptance:** Frontmatter matches.

### BC-AUDIT-686 — setup-env: tool-check tables (8 required + 8 optional)

**Skill:** `plugins/vsdd-factory/skills/setup-env/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 13-41
**Trigger:** Validation phase.
**Behavior:** Required: rustc ≥1.85, cargo, nightly rustfmt, clippy, git, gh, just, jq. Optional: cargo-kani, cargo-fuzz, cargo-mutants, cargo-deny, semgrep, lefthook, asciinema, hyperfine.
**Acceptance:** Output report enumerates each row of both tables.

### BC-AUDIT-687 — setup-env: MCP env-var prefix check + git config (rerere on)

**Skill:** `plugins/vsdd-factory/skills/setup-env/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 43-66
**Trigger:** MCP + git phases.
**Behavior:** `echo $PERPLEXITY_API_KEY | head -c 4` (non-secret prefix); set `git config rerere.enabled true` if not enabled.
**Acceptance:** Report shows MCP env-var presence + git config rerere status.

### BC-AUDIT-688 — setup-env: factory-health invocation + final missing-tools list

**Skill:** `plugins/vsdd-factory/skills/setup-env/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 68-105
**Trigger:** End of setup.
**Behavior:** Runs `/factory-health`; output enumerates Required/Optional/MCP/Git/Factory status with checkmark/warning/cross + missing-required and missing-optional lists.
**Acceptance:** Report contains "Missing required tools:" and "Missing optional tools:" lines.

---

### R. spec-drift

### BC-AUDIT-689 — spec-drift: identity & forked Explore agent

**Skill:** `plugins/vsdd-factory/skills/spec-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** Drift detection request.
**Behavior:** disable-model-invocation true; context fork; agent Explore. Compares spec vs implementation in fresh context for objectivity.
**Acceptance:** Frontmatter `context: fork` + `agent: Explore`.

### BC-AUDIT-690 — spec-drift: scans 4 spec dirs + checks naming + finds orphans

**Skill:** `plugins/vsdd-factory/skills/spec-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 10-46
**Trigger:** Drift analysis.
**Behavior:** Reads `.factory/specs/{prd.md, behavioral-contracts/*, verification-properties/*, architecture/*, prd-supplements/*}`; for each BC verify behavior, error cases, naming; for each architecture decision verify chosen option + dependency direction; check naming consistency (entity, error, endpoint); find orphans (code without spec, spec without code).
**Acceptance:** Each section of the report covers BCs / architecture / naming / orphans.

### BC-AUDIT-691 — spec-drift: writes spec-drift-report.md to current cycle

**Skill:** `plugins/vsdd-factory/skills/spec-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 48-80
**Trigger:** Step 5 (output).
**Behavior:** Writes `.factory/cycles/<current>/spec-drift-report.md` with summary counts, drift details (severity critical/important/cosmetic), architecture violations, recommendations.
**Acceptance:** File exists at named path with all required sections.

---

### S. spec-versioning

### BC-AUDIT-692 — spec-versioning: identity & semver scheme

**Skill:** `plugins/vsdd-factory/skills/spec-versioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** Spec evolution events.
**Behavior:** Specs follow MAJOR.MINOR.PATCH semver.
**Acceptance:** Every bump categorized into one of three.

### BC-AUDIT-693 — spec-versioning: bump-type rules (MAJOR/MINOR/PATCH)

**Skill:** `plugins/vsdd-factory/skills/spec-versioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-54
**Trigger:** Determining bump.
**Behavior:** MAJOR = breaking (architectural rework, removed reqs, behavior change, renamed IDs). MINOR = backward-compat additions (new FRs, NFRs, VPs, new components). PATCH = clarifications/typos/example additions/correcting wrong constraint.
**Acceptance:** Bump rationale references one of these three categories.

### BC-AUDIT-694 — spec-versioning: per-story spec_version + drift detection

**Skill:** `plugins/vsdd-factory/skills/spec-versioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-105
**Trigger:** Story creation, then drift detection at Phase F1 / after Phase F2 / on demand.
**Behavior:** Story frontmatter records `spec_version`; commit message references "Spec version: X.Y.Z". Drift report compares story vs current; PATCH = informational, MINOR = check overlap, MAJOR = mandatory review.
**Acceptance:** Drift report rows include story ID, built-against, drift count, severity, action.

### BC-AUDIT-695 — spec-versioning: L4 immutability rules + locked-VP enforcement

**Skill:** `plugins/vsdd-factory/skills/spec-versioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 107-148
**Trigger:** VP gets `status: locked` after passing proof.
**Behavior:** (1) Locked VPs cannot be modified; (2) refinements via VP-NNN+1 with `amends: VP-NNN`; (3) withdrawal requires justification + impact + human approval, sets `status: withdrawn`; (4) hierarchy version propagation L4→L3→L2→L1. consistency-validator detects modifications to locked VPs and reports CRITICAL findings that block pipeline.
**Acceptance:** Modification of locked VP file is detected and treated as CRITICAL.

### BC-AUDIT-696 — spec-versioning: failure-mode safety (inconsistent versions / locked-VP modified / unparseable)

**Skill:** `plugins/vsdd-factory/skills/spec-versioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 156-160
**Trigger:** Each named failure.
**Behavior:** Inconsistent history → flag, request human resolution before proceeding. Locked VP modified → CRITICAL, block pipeline, no auto-fix. Unparseable spec_version → flag story for manual review.
**Acceptance:** No failure mode results in silent recovery; each gets explicit handling.

---

### T. state-burst

### BC-AUDIT-697 — state-burst: identity & defect-class context

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** Adversarial-pass remediation burst on factory-artifacts branch (STATE.md + SESSION-HANDOFF.md + wave-state.yaml in lockstep).
**Behavior:** Single Canonical SHA + Two-Commit Protocol; structural fix for defect class with 6 consecutive recurrences (case study: docs/lessons-learned/wave-gate-bookkeeping.md).
**Acceptance:** disable-model-invocation false; allowed-tools Read/Write/Edit/Bash; user-invocable.

### BC-AUDIT-698 — state-burst: announces protocol verbatim

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 16-22
**Trigger:** Skill start.
**Behavior:** Says verbatim: "I'm using the state-burst skill to execute the Single Canonical SHA + Two-Commit Protocol for this remediation burst. Stage 1 will write all fixes with the `15fa97e6` placeholder; Stage 2 will backfill the real SHA via global replace. No 3rd commit is permitted."
**Acceptance:** First user-visible line is the verbatim announce.

### BC-AUDIT-699 — state-burst: Stage 1 with `15fa97e6` placeholder + tense rule

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-100
**Trigger:** Stage 1 application.
**Behavior:** Updates STATE.md frontmatter (adversary_<wave>_pass_N_<gate> with remediation_sha=15fa97e6, convergence_status advanced, awaiting outcome-neutral, body table, session-resume snapshot, version bump). Updates SESSION-HANDOFF.md (factory-artifacts HEAD=15fa97e6, develop HEAD real). Updates wave-state.yaml (gate_pass_N + status + notes, next_gate_required). Tense rule: past tense always ("REMEDIATED — Awaiting Pass N+1") never "in progress". Commit message must NOT contain `backfill`.
**Acceptance:** Hook accepts placeholder hits; Stage-1 commit message lacks `backfill`.

### BC-AUDIT-700 — state-burst: Stage 2 global SHA replace + backfill commit

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-156
**Trigger:** After Stage 1 verification.
**Behavior:** `STAGE1_SHA=$(git rev-parse HEAD | cut -c1-8)`; portable sed replaces `15fa97e6` across STATE.md/SESSION-HANDOFF.md/wave-state.yaml; verifies single SHA value cited; commits with `chore(<wave>): backfill stage-1 SHA $STAGE1_SHA into pass-N records`. `verify-sha-currency.sh` must report PASS.
**Acceptance:** Exactly two commits on burst chain; second contains `backfill` token.

### BC-AUDIT-701 — state-burst: refuses 3rd commit + recovery via `git reset --soft HEAD~2`

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 156-205
**Trigger:** Stage 2 hook reports FAIL or any anti-pattern.
**Behavior:** DO NOT add a third commit; instead `git -C .factory reset --soft HEAD~2` and redo from Stage 1. Six anti-pattern detections: 3rd-commit chain, unbackfilled placeholder, in-progress voice, Stage-1 commit containing `backfill`, fabricated SHA (no git object), cross-record SHA drift.
**Acceptance:** Any failure path uses reset+redo, never appended commit.

### BC-AUDIT-702 — state-burst: documented bypass paths

**Skill:** `plugins/vsdd-factory/skills/state-burst/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 198-216
**Trigger:** First state-manager burst on brand-new project (no wave-state.yaml) OR manual recovery after force-push.
**Behavior:** Bypass acceptable; document bypass reason in `SESSION-HANDOFF.md → Recent Burst Episodes`.
**Acceptance:** Bypass record exists when skill is bypassed.

---

### U. state-update

### BC-AUDIT-703 — state-update: identity & internal-only contract

**Skill:** `plugins/vsdd-factory/skills/state-update/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Internal call from another skill (not direct user invocation).
**Behavior:** disable-model-invocation true; user-invocable false; allowed-tools Bash/Read/Edit. Updates `.factory/STATE.md` to reflect pipeline progress.
**Acceptance:** Skill not invoked directly by `/` slash command.

### BC-AUDIT-704 — state-update: 4-step procedure (read → frontmatter → history → commit)

**Skill:** `plugins/vsdd-factory/skills/state-update/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19-66
**Trigger:** Calling skill provides {phase, status, notes}.
**Behavior:** Read STATE.md; update frontmatter (pipeline status, phase, product, mode, timestamp, previous_phase); append row to Phase History table; commit with `factory(<phase>): <status> — <description>`.
**Acceptance:** Each call produces exactly one commit on factory-artifacts.

### BC-AUDIT-705 — state-update: enumerates 5 pipeline statuses + 7 phase IDs

**Skill:** `plugins/vsdd-factory/skills/state-update/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 67-90
**Trigger:** Status transition.
**Behavior:** Statuses: INITIALIZED, RUNNING, PAUSED, BLOCKED, COMPLETED. Phase IDs: pre-1, phase-1, phase-2, phase-3, phase-4 (Holdout AND Adversarial), phase-5 (Hardening), phase-6 (Convergence), release.
**Acceptance:** Frontmatter `pipeline:` value is one of the five; phase: is one of the listed.

---

### V. storybook-mcp-integration

### BC-AUDIT-706 — storybook-mcp-integration: identity & UI-conditional invocation

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-22
**Trigger:** `feature_type in ['ui', 'full-stack']`; UI framework present (React/Vue/Svelte/Angular).
**Behavior:** Primary dx-engineer; supporting devops-engineer/implementer/test-writer/e2e-tester/visual-reviewer. Outputs Storybook + addon-mcp + MCP server at http://localhost:6006/mcp.
**Acceptance:** Skill no-op for non-UI products.

### BC-AUDIT-707 — storybook-mcp-integration: install + config + register procedure

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-58
**Trigger:** dx-engineer invocation.
**Behavior:** `npx storybook@latest init`; `npm install -D @storybook/addon-mcp`; configure `.storybook/main.ts` with addons + `experimentalComponentsManifest: true` + `experimentalCodeExamples: true`; start dev server (`npm run storybook`); register MCP server JSON config at http://localhost:6006/mcp.
**Acceptance:** All three artifacts (addon installed, main.ts updated, MCP server reachable) present.

### BC-AUDIT-708 — storybook-mcp-integration: 6 MCP tools mapped to agent roles

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 59-69
**Trigger:** Agent invocation pattern.
**Behavior:** `list-all-documentation` (implementer reuse), `get-documentation` (implementer ref), `get-documentation-for-story` (test-writer), `get-storybook-story-instructions` (test-writer MUST call before writing stories), `preview-stories` (implementer/visual-reviewer), `run-story-tests` (test-writer/e2e-tester).
**Acceptance:** Quality gate verifies all 6 tools responding.

### BC-AUDIT-709 — storybook-mcp-integration: T1/T2/T3 access pattern via DF-023+DF-027

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 70-78
**Trigger:** Agent calling MCP.
**Behavior:** T3 agents (implementer/test-writer/e2e-tester) call directly via `mcporter call`; T2 agents (ux-designer/accessibility-auditor/visual-reviewer) delegate; T1 agents (orchestrator) call MCP tools directly.
**Acceptance:** T2 agents do NOT call mcporter directly.

### BC-AUDIT-710 — storybook-mcp-integration: self-healing loop with 10-iteration cap

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 80-111
**Trigger:** Component test cycle.
**Behavior:** implementer writes Component.tsx → test-writer calls get-storybook-story-instructions → test-writer writes Component.stories.tsx → implementer calls preview-stories (fix loop) → test-writer calls run-story-tests → on FAIL, fix + re-run; max 10 iterations then escalate to human.
**Acceptance:** Loop count is bounded at 10; escalation occurs at exhaustion.

### BC-AUDIT-711 — storybook-mcp-integration: reuse-first enforcement before new components

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 113-119
**Trigger:** implementer about to create a new component.
**Behavior:** (1) Call `list-all-documentation`; (2) search inventory for matching component; (3) if match, use existing with design-system tokens; (4) only create new with justification in commit message.
**Acceptance:** New components carry justification commit message; reuse precedes creation.

### BC-AUDIT-712 — storybook-mcp-integration: non-React fallback (manifest only)

**Skill:** `plugins/vsdd-factory/skills/storybook-mcp-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 152-158, 180-185
**Trigger:** Non-React framework detected (Vue/Svelte/Angular).
**Behavior:** list-all-documentation, preview-stories, run-story-tests still work; component manifest unavailable; agents read source files directly. Failure modes: not installed → skip + flag gap; unreachable → retry once + fallback to file-based.
**Acceptance:** Non-React projects do not block; manifest gap noted.

---

### W. systematic-debugging

### BC-AUDIT-713 — systematic-debugging: identity + hard gate

**Skill:** `plugins/vsdd-factory/skills/systematic-debugging/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** Any technical bug, test failure, unexpected behavior — before proposing fixes.
**Behavior:** Hard Gate: NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST. Phase 1 must complete before fixes. Violating letter = violating spirit.
**Acceptance:** Gate refuses Phase-4 work prior to Phase-1 sign-off.

### BC-AUDIT-714 — systematic-debugging: 4-phase sequence (root cause → pattern → hypothesis → implementation)

**Skill:** `plugins/vsdd-factory/skills/systematic-debugging/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-87
**Trigger:** Bug investigation start.
**Behavior:** Phase 1 (read errors, reproduce, check changes, trace data); Phase 2 (find working examples, compare, identify diffs); Phase 3 (single hypothesis, minimal test, one variable); Phase 4 (failing test FIRST per Red Gate, single fix, verify).
**Acceptance:** Each phase produces named artifacts before next phase begins.

### BC-AUDIT-715 — systematic-debugging: Phase 4.5 — 3+ failed fixes = STOP and escalate

**Skill:** `plugins/vsdd-factory/skills/systematic-debugging/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 86-104
**Trigger:** Three or more failed fix attempts.
**Behavior:** STOP and escalate. Report BLOCKED with: investigated findings, hypotheses tested, why architectural, recommended discussion topics. Do NOT attempt Fix #4 without human approval.
**Acceptance:** No 4th fix attempt without human approval.

### BC-AUDIT-716 — systematic-debugging: 8-row red-flag rationalization table

**Skill:** `plugins/vsdd-factory/skills/systematic-debugging/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 106-118
**Trigger:** Self-check during debugging.
**Behavior:** Catches "quick fix for now", "just try X", "multiple changes", "skip the test", "probably X let me fix", "don't fully understand", "one more attempt", "I see the problem".
**Acceptance:** Each red-flag row has the explicit counter-argument.

### BC-AUDIT-717 — systematic-debugging: BC-aware mode + status-protocol reporting

**Skill:** `plugins/vsdd-factory/skills/systematic-debugging/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-134
**Trigger:** Bug violates a behavioral contract; or end of work.
**Behavior:** Read BC preconditions/postconditions/invariants → identify violated clause → trace back to impl → fix must restore BC guarantees (not just make test pass). Reports DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
**Acceptance:** Report uses one of the four enumerated statuses.

---

### X. toolchain-provisioning

### BC-AUDIT-718 — toolchain-provisioning: identity & 4 trigger points

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-33
**Trigger:** After architecture (Greenfield); after codebase ingestion (Brownfield); after spec evolution (Feature Mode introduces new language); on-demand request.
**Behavior:** Provisions verification toolchain dynamically based on detected language(s).
**Acceptance:** Skill runs at each named pipeline point.

### BC-AUDIT-719 — toolchain-provisioning: precedence rule (architect > verification > manifest > human)

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 35-46
**Trigger:** Tool-decision time.
**Behavior:** Order: (1) ARCH-INDEX.md Technology Stack, (2) verification-architecture/ARCH-INDEX.md Tooling Selection, (3) `config/verification-toolchains.yaml` defaults. Architect's "skip mutation testing" overrides manifest "required". Human-specified overrides have highest precedence.
**Acceptance:** Architect's pinned version wins over manifest default.

### BC-AUDIT-720 — toolchain-provisioning: language detection cascade

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 58-89
**Trigger:** Step 1 (Detect Language(s)).
**Behavior:** Greenfield → ARCH-INDEX. Brownfield → project-discovery.md. Fallback → file scan (Cargo.toml→Rust, package.json+tsconfig.json→TS, pyproject.toml/setup.py→Python, go.mod→Go). Multiple languages → provision all. Writes `.factory/toolchain-state.yaml`.
**Acceptance:** Detected languages match the source signals; toolchain-state.yaml written.

### BC-AUDIT-721 — toolchain-provisioning: install-priority (lang-native → brew → system) + pkg-mgr per type

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 144-176
**Trigger:** Step 4 (Install Missing Tools).
**Behavior:** Priority: language-native pkg manager → Homebrew (macOS) → apt/dnf. Cargo deps (proptest) → project Cargo.toml [dev-dependencies] not global. npm devDeps (fast-check, Stryker) → `npm install --save-dev`. pip deps (Hypothesis, mutmut) → project venv if exists, else global. Verify after each install.
**Acceptance:** No global install of project-scoped dependency; verification follows install.

### BC-AUDIT-722 — toolchain-provisioning: writes detailed toolchain-state.yaml

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 184-274
**Trigger:** Step 6 (Write Provisioning Report).
**Behavior:** `.factory/toolchain-state.yaml` lists per-language per-category per-tool: status (installed / already_present / skipped / failed / added_to_cargo_toml / builtin / added_to_package_json), version, verify_output. Summary: total/installed/already/skipped/failed.
**Acceptance:** YAML schema matches the published example.

### BC-AUDIT-723 — toolchain-provisioning: integration with formal-hardening + multi-repo + new-language

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 275-313
**Trigger:** Formal hardening start; project.yaml multi-repo; adding new language.
**Behavior:** formal-verifier reads toolchain-state.yaml first; missing required tool triggers re-provisioning. Multi-repo: per-repo language detection. Adding new language: edit `config/verification-toolchains.yaml` only (no skill change).
**Acceptance:** formal-verifier never executes without toolchain-state.yaml; multi-repo writes per-repo state.

### BC-AUDIT-724 — toolchain-provisioning: Storybook + Excalidraw MCP for UI products

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 314-405
**Trigger:** UI/full-stack feature_type.
**Behavior:** Install Storybook + addon-mcp; configure .storybook/main.ts; register MCP at http://localhost:6006/mcp; write to toolchain-state.yaml ui_tooling.storybook. Also Excalidraw via mcporter (`mcporter config add excalidraw --url https://mcp.excalidraw.com`); install `@tommywalkie/excalidraw-cli`. Quality gate: Excalidraw responds to list-tools before UX spec.
**Acceptance:** UI products have both Storybook MCP and Excalidraw MCP entries in toolchain-state.

### BC-AUDIT-725 — toolchain-provisioning: quality-gate criteria

**Skill:** `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 369-377
**Trigger:** End of provisioning.
**Behavior:** Gate: all detected languages have manifest entries; all required tools installed or explicitly skipped; toolchain-state.yaml written; no required tool with status=failed; verify commands pass; Storybook+addon-mcp installed for UI; MCP server registered (DF-037 D18).
**Acceptance:** Each gate item resolves PASS/FAIL with explicit evidence.

---

### Y. traceability-extension

### BC-AUDIT-726 — traceability-extension: identity & chain semantics

**Skill:** `plugins/vsdd-factory/skills/traceability-extension/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-19
**Trigger:** Feature mode adding incremental traceability.
**Behavior:** Reference document. The chain: L1 Brief Section ↔ L2 CAP-NNN ↔ L3 BC-S.SS.NNN ↔ L4 VP-NNN ↔ Story ↔ AC-NNN ↔ Test Case ↔ Implementation ↔ Adversarial Review ↔ Formal Proof.
**Acceptance:** Forward (L1→Proof) and reverse (Proof→L1) navigability both maintained.

### BC-AUDIT-727 — traceability-extension: 7 extension rules (IDs new, links append-only, deprecated stays)

**Skill:** `plugins/vsdd-factory/skills/traceability-extension/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-134
**Trigger:** Feature mode chain extension.
**Behavior:** R1 New IDs continue sequence (FR-024 → FR-025), permanent (DEPRECATED, do not delete). R2 New stories link to new + existing (`implements:` + `extends:`). R3 New tests trace via header comments (Test/Story/Requirements/VP). R4 Cross-references (depends_on / extends / assumes). R5 Append-only chain file. R6 Verify completeness (L1→Proof, 7 links). R7 Deprecated requirements stay in chain.
**Acceptance:** Each rule cross-cuts to traceability-chain.md format.

### BC-AUDIT-728 — traceability-extension: architecture-section-level references (DF-021)

**Skill:** `plugins/vsdd-factory/skills/traceability-extension/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-39
**Trigger:** Trace references to architecture.
**Behavior:** Use specific section paths (`module-decomposition.md`, `api-surface.md`, `verification-architecture/ARCH-INDEX.md`, `purity-boundary-map.md`, `dependency-graph.md`) instead of monolithic `architecture/ARCH-INDEX.md`.
**Acceptance:** No traceability row points to monolithic ARCH-INDEX.md.

### BC-AUDIT-729 — traceability-extension: chain verification command

**Skill:** `plugins/vsdd-factory/skills/traceability-extension/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 136-149, 151-153
**Trigger:** Phase F7 convergence pre-check.
**Behavior:** Pseudocode counts requirements with link_count<7 and emits "INCOMPLETE CHAIN: $line".
**Acceptance:** Reference document only — consumed by story-writer + spec-steward.

---

### Z. track-debt

### BC-AUDIT-730 — track-debt: identity & three commands (add/list/resolve)

**Skill:** `plugins/vsdd-factory/skills/track-debt/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** `/vsdd-factory:track-debt [add|list|resolve] [description]`.
**Behavior:** disable-model-invocation true; allowed-tools Read/Write/Edit/Bash. Manages `.factory/tech-debt-register.md`.
**Acceptance:** Frontmatter matches; only three commands recognized.

### BC-AUDIT-731 — track-debt: add assigns next TD-NNN with full metadata

**Skill:** `plugins/vsdd-factory/skills/track-debt/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-35
**Trigger:** `add <description>`.
**Behavior:** Read register, assign next ID `TD-NNN`, capture severity (critical/high/medium/low), category (design/performance/security/testing/documentation/dependency), source (story/PR/review), impact, effort (S/M/L/XL); append + commit on factory-artifacts.
**Acceptance:** Each new entry has all six fields populated.

### BC-AUDIT-732 — track-debt: register format (Active vs Resolved sections)

**Skill:** `plugins/vsdd-factory/skills/track-debt/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-75
**Trigger:** Register write.
**Behavior:** Active table columns: ID, Severity, Category, Description, Effort, Source, Created. Resolved columns: ID, Description, Resolved By, Date.
**Acceptance:** Both sections present; resolve moves entry from Active to Resolved.

### BC-AUDIT-733 — track-debt: when-to-add catalogue (6 sources)

**Skill:** `plugins/vsdd-factory/skills/track-debt/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 77-85
**Trigger:** Source identification.
**Behavior:** Adversarial real issue (deferred); code-review shortcut for timeline; `// TODO`/`// HACK` comments; deferred performance issues; dependency version pinning needing update; accepted test coverage gaps for timeline.
**Acceptance:** Source-line of each TD entry maps to one of these six categories.

---

### AA. ui-completeness-check

### BC-AUDIT-734 — ui-completeness-check: identity, agents, UI gating, zero-gap rule

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-23
**Trigger:** `feature_type in ['ui', 'full-stack']`.
**Behavior:** Primary consistency-validator; supporting ux-designer/e2e-tester. Inputs design-system + UX-INDEX + stories + source code + tests + ui-evidence; outputs ui-traceability.yaml + completeness-report.md. Gate: ZERO gaps required before convergence.
**Acceptance:** Zero gaps is the convergence gate (not just a warning).

### BC-AUDIT-735 — ui-completeness-check: 3 pipeline points with strictness gradient

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 27-34
**Trigger:** After story creation / after each wave gate / before convergence.
**Behavior:** After story creation → screens + interactions covered. After wave gate → states + tests exist. Before convergence → full traceability matrix + zero gaps.
**Acceptance:** Strictness escalates per pipeline point.

### BC-AUDIT-736 — ui-completeness-check: ui-traceability.yaml schema (screens with components/interactions/responsive/a11y/perf)

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 36-81
**Trigger:** Skill writes traceability matrix.
**Behavior:** Schema requires per-screen: id, name, ux_spec_ref, story_ids, status (specified→story-created→implemented→tested→verified), components[] with states, interactions[] with e2e_test, responsive[] (375/768/1024/1440), accessibility (axe_clean, keyboard_navigable, screen_reader_tested, touch_targets_valid), performance (lcp_ms, cls, fid_ms, meets_targets).
**Acceptance:** ui-traceability.yaml validates against this schema.

### BC-AUDIT-737 — ui-completeness-check: 7-axis gap detection (screens/components/states/interactions/responsive/a11y/perf)

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 83-94
**Trigger:** Gap analysis phase.
**Behavior:** Detects: screens without stories, components without impl, required states (from contract) not implemented, UX interactions without e2e tests, breakpoints without coverage, axe-core/keyboard/screen-reader fails, perf metrics exceeding targets.
**Acceptance:** Each axis is a separate finding category in report.

### BC-AUDIT-738 — ui-completeness-check: state coverage (D4) — 4 async states + per-component-type required states

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 130-148
**Trigger:** State coverage validation.
**Behavior:** Every data-fetching component MUST have LOADING/SUCCESS/EMPTY/ERROR. Required states by component type table: Button (default/hover/active/focus/disabled/loading), Form Field, Modal, List, Card, Navigation, Toast/Alert, Data Table, Dropdown, Tabs.
**Acceptance:** Quality gate: data-fetching components have all 4 async states; type-specific states match table.

### BC-AUDIT-739 — ui-completeness-check: 100% fidelity target + fix story generation

**Skill:** `plugins/vsdd-factory/skills/ui-completeness-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-128, 150-156
**Trigger:** Per gap.
**Behavior:** Each gap → FIX-UI-NNN story → routes through code-delivery.lobster → re-runs check after fixes. Fidelity = (implemented elements / specified elements) × 100; target 100% before convergence.
**Acceptance:** No gap survives convergence.

---

### AB. ui-quality-gate

### BC-AUDIT-740 — ui-quality-gate: identity, agents, UI conditional

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** `feature_type in ['ui', 'full-stack']`.
**Behavior:** Primary consistency-validator; supporting accessibility-auditor/e2e-tester/performance-engineer/ux-designer/visual-reviewer.
**Acceptance:** Skill gated by feature_type.

### BC-AUDIT-741 — ui-quality-gate: comprehensive checklist across 5 dimensions

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-51
**Trigger:** Full checklist run.
**Behavior:** Design system (token/component/pattern); Completeness (screens/states/interactions/responsive); Quality (heuristic ≥0.7, task completion ≥0.8, axe zero, keyboard, perf LCP<2.5s/FID<100ms/CLS<0.1); Visual (regression/responsive/screenshots); States (loading/error/empty for each async view).
**Acceptance:** Every checklist item is a row in gate-report.md.

### BC-AUDIT-742 — ui-quality-gate: 4 strictness levels (per-story/wave/build/convergence)

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-96
**Trigger:** Pipeline point.
**Behavior:** Per-story → token+a11y+component test (block merge). Wave gate → +responsive+perf+states (block next wave). Build verification → all (block release). Convergence → 100% on every dimension; zero traceability gaps; 100% fidelity (block convergence).
**Acceptance:** Each gate level applies its named subset.

### BC-AUDIT-743 — ui-quality-gate: failure → FIX-UI-NNN routing

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 98-105
**Trigger:** Any check fails.
**Behavior:** Log failure in gate-report.md; create FIX-UI-NNN; route through code-delivery.lobster; re-run gate after fix.
**Acceptance:** Each failure produces a FIX-UI story with downstream routing.

### BC-AUDIT-744 — ui-quality-gate: performance targets (D8 LCP/FID/CLS/TTI/bundle/images)

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 107-122
**Trigger:** Performance dimension check.
**Behavior:** LCP <2.5s, FID <100ms, CLS <0.1, TTI <3.8s, bundle <200KB JS per route, images WebP+responsive+lazy. Perceived: skeleton screens, loading indicators, optimistic updates, progressive loading, image optimization.
**Acceptance:** Each metric measured by Lighthouse CI / build analysis / asset audit.

### BC-AUDIT-745 — ui-quality-gate: gate-report.md structure (Gate Level + Result + Checklist + Failures + Perf table)

**Skill:** `plugins/vsdd-factory/skills/ui-quality-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 124-148
**Trigger:** End of gate.
**Behavior:** Output structure: ## UI Quality Gate Report → Gate Level → Result PASS/FAIL → Checklist table → Failures list (mapping to FIX-UI-NNN) → Performance Metrics table.
**Acceptance:** Generated report contains all five sections.

---

### AC. ux-heuristic-evaluation

### BC-AUDIT-746 — ux-heuristic-evaluation: identity, conditional, four pipeline points

**Skill:** `plugins/vsdd-factory/skills/ux-heuristic-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-29
**Trigger:** `feature_type in ['ui', 'full-stack']`.
**Behavior:** Primary ux-designer; supporting accessibility-auditor/business-analyst. Runs at: after UX spec produced, after holdout evaluation, F2 (feature spec delta), wave gate.
**Acceptance:** Skill called at each pipeline point per the table.

### BC-AUDIT-747 — ux-heuristic-evaluation: Nielsen 10 heuristics with explicit subchecks

**Skill:** `plugins/vsdd-factory/skills/ux-heuristic-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 31-93
**Trigger:** Heuristic eval phase.
**Behavior:** Score each 0.0-1.0: H1 Visibility of System Status; H2 Match real-world; H3 User Control & Freedom (undo/cancel/back/exit); H4 Consistency & Standards; H5 Error Prevention; H6 Recognition over Recall; H7 Flexibility & Efficiency; H8 Aesthetic & Minimalist; H9 Help recover from Errors; H10 Help & Documentation.
**Acceptance:** Quality gate requires all 10 evaluated and scored.

### BC-AUDIT-748 — ux-heuristic-evaluation: cognitive walkthrough per key task

**Skill:** `plugins/vsdd-factory/skills/ux-heuristic-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 94-119
**Trigger:** Per key user task in UX spec.
**Behavior:** Define task → walk steps (find/understand/confirm) → measure (steps required, error_potential, backtrack_points) → score 0.0-1.0. YAML schema: name/expected_steps/actual_steps/error_potential/backtrack_points/score/issues.
**Acceptance:** Each key task has filled YAML object.

### BC-AUDIT-749 — ux-heuristic-evaluation: 0.7 threshold + remediation flagging

**Skill:** `plugins/vsdd-factory/skills/ux-heuristic-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 121-128, 152-157
**Trigger:** Scoring phase.
**Behavior:** Threshold 0.7; scores below flagged for remediation; findings feed into adversarial review context.
**Acceptance:** Quality gate: scores below 0.7 flagged; report has remediation items section.

### BC-AUDIT-750 — ux-heuristic-evaluation: report path + failure modes

**Skill:** `plugins/vsdd-factory/skills/ux-heuristic-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 130-150, 159-163
**Trigger:** End of skill.
**Behavior:** Output `.factory/ui-quality/heuristic-evaluation.md` with Heuristic Scores table, Task Completion Analysis table, Remediation Items, Overall Score. Fail modes: no UX spec → gap report; no recordings → spec-only with reduced confidence; no key tasks → derive from screen defs + flag.
**Acceptance:** Output file present with named sections; fail modes documented.

---

### AD. validate-brief

### BC-AUDIT-751 — validate-brief: identity & step-file note

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** Brief validation.
**Behavior:** Skill explicitly does NOT use step-file decomposition; checks run as parallel checks within a single context load.
**Acceptance:** No sequential step-file fan-out.

### BC-AUDIT-752 — validate-brief: structure check requires 6 sections each meeting minimums

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 16-28
**Trigger:** Structure phase.
**Behavior:** What Is This (≥2 sentences); Who Is It For (≥1 specific persona + pain + workaround); Scope In (3-7 capabilities); Scope Out (≥1 exclusion); Success Criteria (≥2 measurable + numeric); Constraints (≥1).
**Acceptance:** Sections failing minimums marked FAIL/WEAK.

### BC-AUDIT-753 — validate-brief: bloat check (<500/<800/<1500 token bands)

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-50
**Trigger:** Context-engineering phase.
**Behavior:** Core sections <500 words ideal; flag if >800 (becoming PRD); flag narrative padding/business justification/competitive analysis/market research; flag requirements leakage (FR-XXX, ACs, architecture decisions); estimate tokens; warn if >1,500.
**Acceptance:** Bloat score in report; OK/WARNING/OVER label.

### BC-AUDIT-754 — validate-brief: implementation-leakage tech-name scanner

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 52-74
**Trigger:** Leakage check.
**Behavior:** Scans for frameworks (React/Next.js/Vue/Angular/Django/Rails/Spring Boot/Express/FastAPI/Svelte), databases (PostgreSQL/MySQL/MongoDB/Redis/DynamoDB/Supabase/SQLite), infra (AWS/GCP/Azure/Docker/Kubernetes/Terraform/Vercel/Netlify), prescriptive language ("must use Rust"), libraries (Redux/Prisma/Drizzle/tRPC/GraphQL prescriptive). Severity: error in Scope/Success; warning in Constraints; info in Overflow Context.
**Acceptance:** Each detected token classified by section + severity.

### BC-AUDIT-755 — validate-brief: information density anti-patterns (4 categories + thresholds)

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-109
**Trigger:** Density phase.
**Behavior:** Conversational filler ("As you know"); wordy phrases ("in order to"→"to"); redundant phrases ("past history"→"history"); hedge words ("somewhat", "may potentially"). Thresholds: >10 = Critical (rewrite); 5-10 = Warning (specifics); <5 = Pass.
**Acceptance:** Counts per category reported with line refs and rewrites.

### BC-AUDIT-756 — validate-brief: market intel cross-check + report file + overall verdict

**Skill:** `plugins/vsdd-factory/skills/validate-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 116-138
**Trigger:** Cross-check + finalization.
**Behavior:** Validates audience/pain claims against market intel (unconfirmed claims flagged; missing risks flagged; differentiation gaps noted). Writes `.factory/planning/brief-validation.md` with per-section status table; bloat score; overall verdict VALID/NEEDS_WORK/INCOMPLETE/OVER_SPECIFIED.
**Acceptance:** Report has all three sections + verdict.

---

### AE. validate-consistency

### BC-AUDIT-757 — validate-consistency: identity & frontmatter

**Skill:** `plugins/vsdd-factory/skills/validate-consistency/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** `/vsdd-factory:validate-consistency`.
**Behavior:** disable-model-invocation true; allowed-tools Read/Bash/Glob/Grep.
**Acceptance:** Frontmatter matches.

### BC-AUDIT-758 — validate-consistency: 7 cross-file checks (BC/VP/Story/Architecture/Counts/Status/Naming)

**Skill:** `plugins/vsdd-factory/skills/validate-consistency/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19-60
**Trigger:** Validation run.
**Behavior:** (1) BC ID integrity (file⇄INDEX, prd refs exist, no dups, S.SS.NNN convention). (2) VP ID integrity. (3) Story traceability (each story refs ≥1 BC; each BC referenced by ≥1 story; deps match graph; wave assignments consistent). (4) Architecture cross-refs (ARCH-INDEX↔ARCH-NN files, BC arch refs valid). (5) Count consistency (STORY-INDEX↔files; BC-INDEX↔files; epic counts↔stories). (6) Status consistency (sprint-state↔STORY-INDEX↔BC files). (7) Naming consistency (entity names↔domain-spec; module names↔arch).
**Acceptance:** Each check produces PASS/FAIL/WARN row in report.

### BC-AUDIT-759 — validate-consistency: report format with Failures/Warnings/All Passed

**Skill:** `plugins/vsdd-factory/skills/validate-consistency/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 65-87
**Trigger:** End of validation.
**Behavior:** Template-driven output: Summary (checks/passed/failed/warnings), Failures table, Warnings table, All Passed list.
**Acceptance:** Report has all four sections, even if Failures or Warnings empty.

---

### AF. validate-template-compliance

### BC-AUDIT-760 — validate-template-compliance: identity & three scopes (file/dir/all)

**Skill:** `plugins/vsdd-factory/skills/validate-template-compliance/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-26
**Trigger:** `/vsdd-factory:validate-template-compliance <path>`.
**Behavior:** Single file (.factory/specs/prd.md), directory (recurse `.md`), or `.factory/` for full audit. Read-only — never modifies files.
**Acceptance:** No artifact modified during run.

### BC-AUDIT-761 — validate-template-compliance: template resolution by document_type then path

**Skill:** `plugins/vsdd-factory/skills/validate-template-compliance/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 27-58
**Trigger:** Step 2 per file.
**Behavior:** Primary lookup: `document_type` frontmatter → matching template. Fallback: 14-row path-pattern table mapping (BC files, VP files, story files, architecture sections, domain-spec, holdout-scenarios, prd, brief, STATE, dtu-assessment, module-criticality). If neither resolves, skip with explicit message.
**Acceptance:** Each file gets a template or "no template found" report.

### BC-AUDIT-762 — validate-template-compliance: 3-level compliance check (frontmatter/sections/tables)

**Skill:** `plugins/vsdd-factory/skills/validate-template-compliance/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-96
**Trigger:** Per file+template pair.
**Behavior:** L1 frontmatter (Present/Missing/Extra; missing required = FAIL, extra = INFO). L2 sections (`## ` H2 headings; in-order; missing = WARN, out-of-order = INFO). L3 table column compliance (header match by nearest preceding H2; missing column = WARN, reordered = INFO).
**Acceptance:** Per-file report has the three levels.

### BC-AUDIT-763 — validate-template-compliance: report format (per-file detail + summary table + aggregate counts)

**Skill:** `plugins/vsdd-factory/skills/validate-template-compliance/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 98-141
**Trigger:** End of audit.
**Behavior:** Per-file detail block; directory summary table (file/template/frontmatter/sections/tables/overall); aggregate counts (PASS/WARN/FAIL with %).
**Acceptance:** Output template matches `consistency-validation-report-template.md`.

### BC-AUDIT-764 — validate-template-compliance: documented limitations (no content quality, no value validation)

**Skill:** `plugins/vsdd-factory/skills/validate-template-compliance/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 142-149
**Trigger:** Skill scope.
**Behavior:** Does NOT check content quality, does NOT validate frontmatter values, does NOT check table data correctness. INDEX files generally have no template (auto-generated, skipped).
**Acceptance:** Skill never reports content errors — only structural.

---

### AG. validate-workflow

### BC-AUDIT-765 — validate-workflow: identity & static-only contract

**Skill:** `plugins/vsdd-factory/skills/validate-workflow/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** `/vsdd-factory:validate-workflow <file>`.
**Behavior:** Schema-checks .lobster file. Static — does not execute steps; does not modify file.
**Acceptance:** No file edits; no agent/skill spawned.

### BC-AUDIT-766 — validate-workflow: 6 checks (required fields/agent/skill/depends_on/dup names/top-level)

**Skill:** `plugins/vsdd-factory/skills/validate-workflow/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-23
**Trigger:** Per-step + top-level check.
**Behavior:** (1) name + type ∈ {agent,skill,command} + task. (2) Agent type → confirm `agents/<a>.md` or `agents/<a>/<a>.md`. (3) Skill type → confirm `skills/<s>/SKILL.md`. (4) Each `depends_on` names earlier step; no cycles, no forward refs, no dangling. (5) No duplicate step names. (6) Top-level `workflow.{name,version,steps}` exist.
**Acceptance:** Each check pass/fail in compact table; topological sort produces no cycles.

### BC-AUDIT-767 — validate-workflow: collects all errors (no early bail) + exit code

**Skill:** `plugins/vsdd-factory/skills/validate-workflow/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 26-41
**Trigger:** Walk + report.
**Behavior:** Parses with `bin/lobster-parse`; walks step list collecting all errors (does not bail on first); existence checks via `ls`; reports compact table + summary (`<n> checks, <p> passed, <f> failed`); exits 0 on all-pass, non-zero otherwise.
**Acceptance:** All errors surfaced in single run; exit code matches outcome.

---

### AH. visual-companion

### BC-AUDIT-768 — visual-companion: identity, prerequisites, optional setup

**Skill:** `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-28
**Trigger:** Brainstorming, brief creation, architecture design (visual content).
**Behavior:** Local Node.js server (ES2020+); user consents to URL. Optional excalidraw setup via `setup.sh`. Without setup, `.excalidraw` shows "run setup" message; HTML still works.
**Acceptance:** Skill is OPTIONAL; degrades gracefully.

### BC-AUDIT-769 — visual-companion: server lifecycle (start/url/state-dir/stop, 30-min auto-exit)

**Skill:** `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-65
**Trigger:** Session begin/end.
**Behavior:** `start-server.sh --project-dir <path>` returns JSON {type:server-started, port, url, screen_dir, state_dir}; user opens URL; stop with `stop-server.sh <session_dir>`; auto-exits after 30 min inactivity.
**Acceptance:** Session JSON contains all five fields; stop is explicit or auto.

### BC-AUDIT-770 — visual-companion: write-loop discipline (Write tool, semantic filenames, no reuse)

**Skill:** `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-56
**Trigger:** Each new screen.
**Behavior:** (1) Verify `$STATE_DIR/server-info` exists; (2) write HTML using Write tool (NOT cat/heredoc); semantic filenames (layout.html, architecture.html); never reuse filenames; content fragments only (no `<!DOCTYPE>`); (3) tell user URL, end turn; (4) next turn read `$STATE_DIR/events`; (5) iterate or advance; (6) unload by pushing waiting screen.
**Acceptance:** Each screen is in a fresh file; cat/heredoc forbidden.

### BC-AUDIT-771 — visual-companion: visual-vs-terminal decision rule

**Skill:** `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 66-72
**Trigger:** Question routing.
**Behavior:** Visual content (mockups, wireframes, layouts, diagrams, side-by-side) → browser. Text content (requirements, choices, tradeoffs, scope) → terminal. UI topic ≠ automatically visual.
**Acceptance:** Rule explicitly cited when choosing browser vs terminal.

### BC-AUDIT-772 — visual-companion: excalidraw mode + composed views (screen.json manifest)

**Skill:** `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 74-104
**Trigger:** `.excalidraw` file write or split view.
**Behavior:** Excalidraw: server auto-switches to interactive canvas; user edits sync via WebSocket to file; agent reads on next turn. Composed: `screen.json` with `{layout:split, panes:[{file,label}]}`.
**Acceptance:** Excalidraw edits round-trip via WebSocket; manifest with layout=split shows both panes.

---

### AI. wave-gate

### BC-AUDIT-773 — wave-gate: identity, allowed tools, pre-flight

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7, 41-51
**Trigger:** `/vsdd-factory:wave-gate wave-N`.
**Behavior:** disable-model-invocation true; allowed-tools Read/Write/Edit/Bash/Glob/Grep/AskUserQuestion. Pre-flight: all wave stories must be `merged` in sprint-state.yaml; otherwise abort with which stories blocking.
**Acceptance:** Skill aborts if any wave story is not merged.

### BC-AUDIT-774 — wave-gate: announces protocol verbatim + TodoWrite per gate

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-27
**Trigger:** Skill start.
**Behavior:** Iron Law: "NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST"; "close enough" not passing; skipped/mocked/partial = failed. Says verbatim: "I'm using the wave-gate skill to run the post-wave integration gate for wave-N." Creates TodoWrite entry per gate (six entries).
**Acceptance:** Six TodoWrite entries created; only completed when independently verified.

### BC-AUDIT-775 — wave-gate: gate sequence is load-bearing (1→2→3→4→5→6, stop on first failure)

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-117
**Trigger:** Gate execution.
**Behavior:** G1 Test Suite (cargo test --release + clippy + nightly fmt --check on develop). G2 DTU Validation (read module-criticality.md; if CRITICAL/HIGH modules touched, run DTU comparison; CRITICAL divergence is blocking). G3 Adversarial Review (`/adversarial-review implementation` on wave diff; pass = no CRITICAL, HIGH documented). G4 Demo Evidence (`.factory/demo-evidence/STORY-NNN/demo-report.md` for each story, all ACs covered). G5 Holdout Eval (`/holdout-eval wave-N` with information asymmetry; pass = mean ≥0.85, every critical ≥0.60). G6 State Update (sprint-state.yaml all wave stories→completed; STATE.md; commit). Stop on any failure — do NOT continue.
**Acceptance:** Order is fixed; first failure halts.

### BC-AUDIT-776 — wave-gate: 8-row red-flag rationalization table

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-39
**Trigger:** Self-check during gate.
**Behavior:** Catches: skip = SKIP (with reason); flaky = re-run gate-1; HIGH = tech debt entry; missing demo = dispatch demo-recorder; 0.84 ≠ 0.85 (no rounding); fixed gate order; advance with failed gate = forbidden; develop run is required.
**Acceptance:** Each rationalization has the explicit counter.

### BC-AUDIT-777 — wave-gate: GATE_CHECK telemetry lines (validated by hook)

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 118-133
**Trigger:** End of each gate.
**Behavior:** Emit `GATE_CHECK: gate=N name=<name> status=<pass|fail|skip> note=<...>`; `validate-wave-gate-completeness` hook validates these are present in gate report before allowing `gate_status: passed` in wave-state.yaml. skip requires reason in note.
**Acceptance:** Six GATE_CHECK lines (one per gate) present in report; hook accepts.

### BC-AUDIT-778 — wave-gate: outputs and post-pass guidance

**Skill:** `plugins/vsdd-factory/skills/wave-gate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 134-180
**Trigger:** Gate completion (pass or fail).
**Behavior:** Output: human-readable summary (Gate 1..6 with check/skip/cross + GATE_CHECK lines). PASS → tells user wave complete, next-wave stories unblocked, next: `/deliver-story` or `/adversarial-review implementation` if last wave. FAIL → "Wave Gate FAILED at Gate N", lists blocking findings, instructs to fix and retry.
**Acceptance:** Report follows verbatim format; user instruction matches pass/fail.

---

### AJ. wave-scheduling

### BC-AUDIT-779 — wave-scheduling: identity & topo-sort algorithm

**Skill:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-30
**Trigger:** Story decomposition complete.
**Behavior:** Reads STORY-INDEX.md dep graph; topo sort. Wave 1 = stories with `depends_on: []`. Wave N = stories whose deps are all in Waves 1..(N-1).
**Acceptance:** Each story belongs to exactly one wave.

### BC-AUDIT-780 — wave-scheduling: parallel groups (≤2 S/M or ≤1 L/XL per group)

**Skill:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-39
**Trigger:** Step 3 (Parallel Group Sub-Partitioning).
**Behavior:** Within each wave: max 2 S/M stories per group; max 1 L/XL per group. Each group has its own test-writer→implementer sequence.
**Acceptance:** No group exceeds the per-size limits.

### BC-AUDIT-781 — wave-scheduling: pipeline overlap (Wave N+1 stubs while Wave N implements)

**Skill:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-46
**Trigger:** Optimization step.
**Behavior:** Start Wave N+1 stubs while Wave N implementation runs (stubs depend only on types). Wave N+1 tests depend on Wave N types being available. `cargo check` between stub creation and test writing.
**Acceptance:** Stub creation precedes test-writing on same wave.

### BC-AUDIT-782 — wave-scheduling: wave-schedule.md output + quality gate

**Skill:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-66
**Trigger:** Output stage.
**Behavior:** Writes `wave-schedule.md` under `.factory/cycles/**/implementation/` per template. Quality gate: all stories assigned, deps respected, no cycles, group sizes respected.
**Acceptance:** Schedule rows {Wave, Group, Stories, Test-Writer Scope, Implementer Scope}.

### BC-AUDIT-783 — wave-scheduling: failure modes (cycle/missing dep/no roots)

**Skill:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 68-72
**Trigger:** Each failure type.
**Behavior:** Circular dep → report exact cycle (e.g., `STORY-004→STORY-007→STORY-004`) + STOP (no scheduling). Missing dep → flag + exclude story. No Wave-1 candidates (all deps everywhere) → "no root stories found" + STOP.
**Acceptance:** Each failure halts with explicit message; no partial schedule.

---

### AK. wave-status

### BC-AUDIT-784 — wave-status: identity & read-only contract

**Skill:** `plugins/vsdd-factory/skills/wave-status/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-23
**Trigger:** "What wave are we on, is it ready to ship?"
**Behavior:** Wraps `bin/wave-state` (summary, current, stories, ready). Reports current wave/total, story list, readiness breakdown (ready/in-progress/blocked/not-ready), recommendation (run `/vsdd-factory:wave-gate` if all ready; else list blockers). Does NOT mutate sprint-state.yaml.
**Acceptance:** Skill is read-only; advancement is orchestrator's job.

---

### AL. worktree-manage

### BC-AUDIT-785 — worktree-manage: identity & 3 commands (create/list/cleanup)

**Skill:** `plugins/vsdd-factory/skills/worktree-manage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** `/worktree-manage [create|list|cleanup] [STORY-NNN]`.
**Behavior:** disable-model-invocation true; allowed-tools Bash/Read.
**Acceptance:** Frontmatter matches.

### BC-AUDIT-786 — worktree-manage: create produces `.worktrees/STORY-NNN/` on `feature/STORY-NNN-<desc>`

**Skill:** `plugins/vsdd-factory/skills/worktree-manage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 17-47
**Trigger:** `create STORY-NNN [description]`.
**Behavior:** Validate story exists (warn if not); refuse if `.worktrees/STORY-NNN` exists; ensure `develop` exists (create from main if needed); `mkdir -p .worktrees`; `git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN-<desc> develop`; update sprint-state.yaml with worktree path; report path/branch/base.
**Acceptance:** New worktree on feature branch off develop.

### BC-AUDIT-787 — worktree-manage: cleanup refuses dirty + warns unmerged

**Skill:** `plugins/vsdd-factory/skills/worktree-manage/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 68-104
**Trigger:** `cleanup STORY-NNN`.
**Behavior:** Check uncommitted (`git status --porcelain`); abort+warn if dirty (user must commit/stash). Check merged to develop; if not merged, warn but allow with confirmation. Remove worktree (`git worktree remove`); delete branch if merged.
**Acceptance:** Never force-removes dirty worktree; never deletes unmerged branch without confirmation.

---

### AM. writing-skills

### BC-AUDIT-788 — writing-skills: identity & TDD-for-skills mapping

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15, 38-45
**Trigger:** Creating/editing/verifying a skill.
**Behavior:** Treats skill creation as TDD: pressure scenario = test case; SKILL.md = production code; agent fails without skill = RED; agent complies = GREEN; close loopholes = REFACTOR.
**Acceptance:** Mapping table is authoritative for the methodology.

### BC-AUDIT-789 — writing-skills: hard gate (NO SKILL WITHOUT FAILING TEST FIRST)

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-22
**Trigger:** Skill creation begins.
**Behavior:** Hard Gate: NO SKILL WITHOUT FAILING TEST FIRST. "Write skill before testing? Delete it. Start over." Editing skill without testing = same violation. No exceptions.
**Acceptance:** Skill creation MUST start with RED-phase pressure scenario.

### BC-AUDIT-790 — writing-skills: when-to-create vs when-not catalogues

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-36
**Trigger:** Decision phase.
**Behavior:** Create when: not intuitively obvious / cross-project ref / broadly applicable / others benefit. Don't for: one-off / standard practice / project-specific (CLAUDE.md/rules) / mechanical constraints (hooks).
**Acceptance:** Decision documented per these catalogues.

### BC-AUDIT-791 — writing-skills: SKILL.md structure with 6 mandatory sections

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 56-74
**Trigger:** Authoring SKILL.md.
**Behavior:** Frontmatter (name with hyphens; description). Sections: 1 Hard Gate (if discipline); 2 When to Use (symptoms/triggers); 3 Core Process; 4 Red Flags (rationalization table; if discipline); 5 Reporting (status protocol); 6 Quick Reference (table for scanning).
**Acceptance:** Each new skill carries appropriate sections per its type.

### BC-AUDIT-792 — writing-skills: CSO description rules (Use when… , no workflow summary, <500 chars)

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-92
**Trigger:** Description authoring.
**Behavior:** Start with "Use when..." (triggering conditions only); do NOT summarize workflow; specific symptoms + keywords; <500 chars. BAD: "Use for TDD - write test first…". GOOD: "Use when implementing any feature or bugfix, before writing implementation code".
**Acceptance:** All new skill descriptions follow CSO rules.

### BC-AUDIT-793 — writing-skills: red-green-refactor cycle for skills + bulletproofing table

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 93-127
**Trigger:** Skill iteration.
**Behavior:** RED: pressure scenario WITHOUT skill, document choices/rationalizations/triggers. GREEN: minimal skill addressing those rationalizations, run scenarios WITH skill, agent should comply. REFACTOR: close loopholes, address spirit-vs-letter, rationalization table, Red Flags list. Bulletproofing table catches "skill is obviously clear", "just a reference", "testing is overkill", "test if problems emerge", "too simple to test".
**Acceptance:** Each new discipline-skill has a rationalization table and red-flags list.

### BC-AUDIT-794 — writing-skills: vsdd-factory conventions + checklist

**Skill:** `plugins/vsdd-factory/skills/writing-skills/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 128-172
**Trigger:** vsdd-factory plugin skill authorship.
**Behavior:** File org: skills/<name>/SKILL.md required, supporting files only if reference exceeds 100 lines. Conventions: skill-delegation-template / skill-execution-template / agents-md-template; standard status protocol (DONE/DONE_WITH_CONCERNS/NEEDS_CONTEXT/BLOCKED); Context Discipline section if loading `.factory/`; target 1,500-3,000 words. Checklist (RED/GREEN/REFACTOR/Quality phases) covers 14 items including no narrative storytelling and git commit.
**Acceptance:** Each new skill review uses this checklist.

---

## 3. Observations

1. **Coverage profile is bimodal.** The batch contains heavy procedural skills (release, repo-initialization, sdk-generation, toolchain-provisioning, state-burst, wave-gate) at 200-400 LOC each, and thin reference/dispatcher skills (wave-status, validate-workflow, research-cache-ops, scaffold-claude-md) at <100 LOC. Heavy skills get 7-9 BCs (one per major section); thin skills get 1-3 BCs. State-burst is uniquely dense (6 BCs) because every anti-pattern carries explicit detection + recovery.

2. **Hard Gates are an explicit pattern.** Three skills declare a verbatim "Hard Gate" section: systematic-debugging ("NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST"), writing-skills ("NO SKILL WITHOUT A FAILING TEST FIRST"), wave-gate ("NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST"). The Iron Law / Hard Gate pattern combines: (1) verbatim phrasing, (2) Red Flags rationalization table, (3) "violating letter = violating spirit" clause. Worth lifting into a cross-cutting BC during synthesis (not added here as it would duplicate per-skill identity BCs).

3. **Verbatim announce strings are convention.** state-burst, release, and wave-gate all require a literal announce string before any other action. This pattern surfaces in batch 1 (activate skill, brownfield-ingest) too. Likely a universal "announce-at-start" cross-cutting BC opportunity.

4. **Quality-gate sections are nearly universal.** 27 of 39 skills carry an explicit `## Quality Gate` checklist. The 12 without typically are reference docs (traceability-extension, writing-skills) or pure dispatchers (run-phase, validate-workflow, wave-status). This is consistent with skill-execution-template guidance.

5. **`disable-model-invocation: true` correlates with user-facing slash commands.** pr-create, record-demo, recover-state (no, not declared but inferred), setup-env, spec-drift, state-update, track-debt, validate-consistency, worktree-manage all set this. Skills that are agent-spawned (responsive-validation, ui-completeness-check, ui-quality-gate) leave it default.

6. **UI-conditional gating is consistent.** Eight skills (responsive-validation, storybook-mcp-integration, ui-completeness-check, ui-quality-gate, ux-heuristic-evaluation, plus toolchain-provisioning's UI section) gate on `feature_type in ['ui', 'full-stack']`. Cross-cutting: a non-UI project never sees UI quality artifacts.

7. **Failure-mode sections are an established convention.** 22 of 39 skills carry a `## Failure Modes` section with named failure → recovery mapping. This is significantly more pervasive than I assumed at the start of the batch and confirms Pass 5 conventions findings.

8. **Two skills have an embedded antipattern table.** state-burst has six anti-pattern→detection→recovery rows; systematic-debugging has eight red-flag→counter rows; wave-gate has eight; writing-skills has five. These are all bulletproofing tables — empirical structural pattern within "discipline" skills.

9. **Composite reference: traceability-extension uses 7-step rule numbering.** Other reference-style skills (validate-template-compliance, spec-versioning) use H3-section numbering. Different style for similar content type (reference vs procedural reference).

10. **Skills that delegate explicitly mark "delegation reference".** repo-initialization opens with: "**Delegation Reference:** This skill describes work the orchestrator delegates to specialist agents via the Agent tool. Each step names the target agent. The orchestrator does NOT execute these steps directly." This pattern is unique to repo-initialization in this batch (was also in dispatcher skills earlier batches). Important when authoring new skills that delegate.

---

## 4. Delta Summary

- New BCs added this batch: **195** (BC-AUDIT-600..794).
- Existing BCs refined: **0** (this is a parallel-coverage extension, not a refinement of prior round).
- Skills with full coverage: **39 / 39**.
- Skills with single minimal BC: **0** (every skill in this batch had at least 1 procedural detail justifying multi-BC coverage; thinnest is wave-status with 1 BC at 22-line file).
- Heaviest coverage: release (9 BCs), toolchain-provisioning (8), wave-gate (6), repo-initialization (9).
- Remaining gaps for downstream synthesis to address:
  - Cross-cutting "verbatim announce" pattern (3+ skills declare it)
  - Cross-cutting "Hard Gate / Iron Law" pattern (3+ skills)
  - Cross-cutting "Failure Modes" section (22+ skills)
  - Cross-cutting UI-feature_type gating (8 skills)

---

## Novelty Assessment

- **Pass** | 3 (deepening — skills batch 3 of 3, alphabetical 81-119)
- **Novelty score** | SUBSTANTIVE (binary, per skill instructions; no nitpicks)
- **Trajectory** | First parallel-coverage pass for skills 81-119; prior pass-3 files contained zero per-skill BCs for any of these 39 skills, so trajectory is initial-coverage rather than refinement-of-prior. Findings monotone-additive vs round 1.
- **Verdict** | FINDINGS_REMAIN

Justification: Of the 195 BCs in this batch, every one is a first-time per-skill behavioral contract — none of skills 81-119 had per-skill BCs in `pass-3-behavioral-contracts.md` or `pass-3-behavioral-contracts-deep-r1.md`. The earlier passes catalogued cross-cutting policy contracts (BC-AUDIT-001..099) and a handful of skill-level BCs for activate/brownfield-ingest/state-burst/wave-gate (≤6 total spanning the whole skills catalogue). This batch closes the rebuild-coverage gap for 39 of 119 skills — roughly one-third of the per-skill spec surface area. Removing these 195 BCs would leave the spec unable to rebuild (e.g.) the release pipeline modes, the wave-gate iron law, the state-burst two-commit protocol, or the toolchain-provisioning precedence rule. The user explicitly requested "spec must be sufficient to rebuild what currently ships — full per-instance BC coverage required"; this batch is on the critical path to that requirement.

Cross-cutting observations 2/3/7/10 in §3 also surface previously-uncatalogued structural patterns (Hard Gate convention, verbatim announce convention, delegation-reference marker) that downstream synthesis can lift into universal BCs. Those are NEW findings not present in either prior pass-3 file.

---

## 6. Convergence Declaration

Verdict: **FINDINGS_REMAIN** — substantive gaps remain for 80 of 119 skills (those alphabetically before pr-create). The skills batches are cumulative parallel coverage, not iterative refinement of the same skills. Per-skill BC coverage for the remaining 80 skills (activate through post-feature-validation) is the next required substantive deepening. This batch should run alongside batch-1 (skills 1-40) and batch-2 (skills 41-80) before final synthesis can claim convergence on the per-skill axis.

Within batch-3's own scope, however, every skill is fully covered with the documented BC mix (1 identity + 1+ behavioral + 1+ quality-gate + 1 output, compressed for thin skills). No additional round is needed for skills 81-119 specifically.

---

## 7. State Checkpoint

```yaml
pass: 3
round: deep-skills-batch-3
status: complete
batch_scope: alphabetical_81_to_119
skills_covered: 39
bcs_added: 195
bc_id_range: BC-AUDIT-600..BC-AUDIT-794
timestamp: 2026-04-25T00:00:00Z
novelty: SUBSTANTIVE
next_action: parallel batch-1 (skills 1-40) and batch-2 (skills 41-80) for full per-skill coverage; then final synthesis
```
