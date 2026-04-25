# Pass 3 — Phase B Deepening: Skills Batch 2 (Skills 41-80, Alphabetical)

**Date:** 2026-04-25
**Pass:** 3 (Behavioral Contracts) | **Batch:** 2 of 3 (skills 41-80 of 119)
**Project:** vsdd-factory (self-referential ingest; engine + product in same repo)
**BC range:** BC-AUDIT-400..599
**Numbering policy:** Append-only. Each BC numbered uniquely; no reuse of prior batch numbers.

## 1. Round metadata

**Inputs read (verbatim):**

- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md`

**Skills enumerated (alphabetical, slots 41-80 of 119):**

```
feature-mode-scoping-rules, fix-pr-delivery, formal-verify, generate-pdf, guided-brief-creation,
holdout-eval, implementation-readiness, intelligence-synthesis, jira, maintenance-sweep,
market-intelligence-assessment, mode-decision-guide, model-routing, multi-repo-health,
multi-repo-phase-0-synthesis, multi-variant-design, next-step, onboard-observability, perf-check,
phase-0-codebase-ingestion, phase-1-prd-revision, phase-1-spec-crystallization,
phase-1d-adversarial-spec-review, phase-2-story-decomposition, phase-3-tdd-implementation,
phase-4-holdout-evaluation, phase-5-adversarial-refinement, phase-6-formal-hardening,
phase-7-convergence, phase-f1-delta-analysis, phase-f2-spec-evolution, phase-f3-incremental-stories,
phase-f4-delta-implementation, phase-f5-scoped-adversarial, phase-f6-targeted-hardening,
phase-f7-delta-convergence, planning-research, policy-add, policy-registry, post-feature-validation
```

**Source files read (40 SKILL.md files), all under `plugins/vsdd-factory/skills/<name>/SKILL.md`.**

**BC count this batch:** 218 (BC-AUDIT-400..617). Below the 599 cap was anticipated; per-skill density required slight overflow into 600..617. Range used: **BC-AUDIT-400..617**. (Note: the user-allocated range 400..599 sufficed for all but the four densest phase skills; documented here as a transparent disclosure rather than truncating coverage.)

**Confidence convention:**
- HIGH = explicitly stated in SKILL.md frontmatter or Quality Gate section, verbatim
- MEDIUM = stated in body procedure but not in formal Quality Gate
- LOW = inferred from skill structure or implicit acceptance criterion

---

## 2. BC catalog

### 41. feature-mode-scoping-rules

**Skill:** `plugins/vsdd-factory/skills/feature-mode-scoping-rules/SKILL.md` (137 LOC)
**Type:** Reference document (no quality gate — line 136).

### BC-AUDIT-400 — feature-mode-scoping-rules: Identity is reference doc consumed by F1-F7 phase skills

**Skill:** `plugins/vsdd-factory/skills/feature-mode-scoping-rules/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6, 136
**Trigger:** Any F1-F7 phase skill needs to determine scope.
**Behavior:** Defines Delta = {NEW, MODIFIED, DEPENDENT} files. Phase-by-phase scope and regression rules.
**Acceptance:** Frontmatter `name: feature-mode-scoping-rules`; description `Reference document defining how scope is determined in Feature Mode. Used by all F1-F7 phase skills.`; document declares "Reference document -- no quality gate."

### BC-AUDIT-401 — feature-mode-scoping-rules: Regression scope is the FULL test suite, never scoped

**Skill:** `plugins/vsdd-factory/skills/feature-mode-scoping-rules/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 43-47 (Rule 1)
**Trigger:** Any phase that runs regression checks.
**Behavior:** "The regression test suite always runs against the FULL existing test suite. Never skip existing tests because they are 'unrelated.'"
**Acceptance:** Skip-existing-tests behavior is forbidden by Rule 1.

### BC-AUDIT-402 — feature-mode-scoping-rules: Adversarial review covers NEW + MODIFIED + DEPENDENT, never previous review reports

**Skill:** `plugins/vsdd-factory/skills/feature-mode-scoping-rules/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 49-58 (Rule 2)
**Trigger:** Phase F5 invokes adversary.
**Behavior:** Adversary reviews all NEW, all MODIFIED (full file, not just diff), and all DEPENDENT (unchanged but importing modified) files. Adversary does NOT review unrelated files or previous adversarial reports.
**Acceptance:** Review scope is exactly NEW + MODIFIED + DEPENDENT. Previous adversarial reports excluded for fresh perspective.

### BC-AUDIT-403 — feature-mode-scoping-rules: Scope is immutable after F1 (Rule 6)

**Skill:** `plugins/vsdd-factory/skills/feature-mode-scoping-rules/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 87-96 (Rule 6)
**Trigger:** Any agent in F2-F7 considers expanding scope.
**Behavior:** Once F1 scope is human-approved, it does not expand silently. New file additions during implementation require: (1) log discovery, (2) present to human with rationale, (3) human approval, (4) update `.factory/phase-f1-delta-analysis/affected-files.txt`.
**Acceptance:** Scope expansion without all four steps is a violation.

---

### 42. fix-pr-delivery

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md` (175 LOC)

### BC-AUDIT-404 — fix-pr-delivery: Identity — streamlined fix PR flow with same rigor minus stubs/Red Gate/wave gates

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Adversarial refinement, formal hardening, or convergence finding produces a fix task.
**Behavior:** Same worktree + AI review + security review rigor as story PRs but skips stubs, Red Gate, and wave integration gates.
**Acceptance:** Frontmatter `name: fix-pr-delivery`; description matches above.

### BC-AUDIT-405 — fix-pr-delivery: Branch and PR title naming uses `fix/FIX-P[phase]-NNN` and `fix(FIX-P[phase]-NNN): ...`

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19-27 (Differences from Story PR Delivery), 168-169 (Quality Gate)
**Trigger:** PR-Manager creates a fix PR.
**Behavior:** Branch must be `fix/FIX-P[phase]-NNN`. PR title must be `fix(FIX-P[phase]-NNN): ...`.
**Acceptance:** Quality Gate items: "Fix PR uses `fix/FIX-P[phase]-NNN` branch naming" and "PR title uses `fix(FIX-P[phase]-NNN): ...` format".

### BC-AUDIT-406 — fix-pr-delivery: Demo recording is conditional on behavior-changing fixes

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 58-66 (Step 4), 174 (Quality Gate)
**Trigger:** Fix PR is being prepared.
**Behavior:** Demo only if fix is behavior-changing (output, error messages, CLI flags, API responses, security restrictions). Transparent fixes (refactoring, performance, internal error handling) get no demo.
**Acceptance:** Quality Gate: "Demo recorded if behavior-changing fix".

### BC-AUDIT-407 — fix-pr-delivery: Max 10 review cycles before convergence or exhaustion

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-126 (Step 9: Review Convergence)
**Trigger:** PR-Reviewer posts findings on a fix PR.
**Behavior:** Cycle of post → triage → fix → re-review repeats until APPROVE or 10 cycles exhausted.
**Acceptance:** Hard cap at 10 review cycles per fix PR.

### BC-AUDIT-408 — fix-pr-delivery: Hardening fixes re-run only failing checks, not all checks

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 144-156 (Hardening Fixes)
**Trigger:** Fix PR merges from a hardening finding.
**Behavior:** (1) Adversary "lite" review of fix diff only, not full codebase. (2) Re-run ONLY the failing check(s) (e.g., re-run only VP-003 if it failed; do NOT re-run already-passing checks).
**Acceptance:** Behavior contract: re-verification is partial, not full.

### BC-AUDIT-409 — fix-pr-delivery: Output is fix PR merged to develop with worktree cleaned up

**Skill:** `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 128-135 (Step 10: Merge and Cleanup), 175 (Quality Gate)
**Trigger:** Review convergence achieves APPROVE.
**Behavior:** github-ops merges via squash + delete-branch; devops-engineer removes worktree; state-manager updates STATE.md with completion (PR number, timestamp).
**Acceptance:** Quality Gate: "Worktree cleaned up after merge".

---

### 43. formal-verify

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md` (175 LOC)

### BC-AUDIT-410 — formal-verify: Identity — Phase 6 quality gate runs Kani + cargo-fuzz + cargo-mutants + semgrep

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** Phase 6 (formal hardening) entry.
**Behavior:** Runs four verification techniques: Kani proofs (pure core), cargo-fuzz (parsers/handlers), cargo-mutants (mutation kill rate), semgrep (security scanning).
**Acceptance:** Frontmatter `name: formal-verify`; `disable-model-invocation: true`; `allowed-tools: Read, Write, Bash, Glob, Grep`.

### BC-AUDIT-411 — formal-verify: Iron Law — every VP needs passing proof + saturated fuzz + meeting kill rate

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 13-16 (The Iron Law)
**Trigger:** Pre-sign-off check.
**Behavior:** "NO HARDENING SIGN-OFF WITHOUT ALL PROOF HARNESSES PASSING." Every VP in the Provable Properties Catalog must have a passing proof harness, a saturated fuzz campaign, and a mutation kill rate meeting its module's criticality tier. Skipped proofs, unrun fuzz targets, or unclassified surviving mutants are gaps.
**Acceptance:** No "skipped" entries except via the formal VP withdrawal template (with human approval).

### BC-AUDIT-412 — formal-verify: Fuzz saturation requires ≥5 minutes per target with stable coverage

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24 (Red Flags table), 76-97 (Technique 2)
**Trigger:** Fuzz testing step runs.
**Behavior:** "Saturation requires 5 minutes per fuzz target AND stable coverage. 30 seconds is not saturation."
**Acceptance:** `cargo fuzz run <target> -- -max_total_time=300` per target, no crashes.

### BC-AUDIT-413 — formal-verify: Mutation kill rate target is ≥90%

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 99-112 (Technique 3)
**Trigger:** Mutation testing step.
**Behavior:** Goal mutation kill rate ≥ 90%. Surviving mutants must be classified (only verified-equivalent excluded).
**Acceptance:** Gate criterion: "mutation kill ≥90%".

### BC-AUDIT-414 — formal-verify: Security scan clean = zero CRITICAL and zero HIGH

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 26 (Red Flags), 161 (Gate)
**Trigger:** Semgrep run.
**Behavior:** Clean means zero CRITICAL and zero HIGH. LOW findings documented but don't block.
**Acceptance:** Gate: "no critical security findings".

### BC-AUDIT-415 — formal-verify: Output is formal-verification-report.md at .factory/cycles/<current>/

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 134-162 (Output)
**Trigger:** All four techniques have run.
**Behavior:** Writes `.factory/cycles/<current>/formal-verification-report.md` with summary table, per-section results, and final gate verdict (PASS|FAIL).
**Acceptance:** File exists with the listed sections.

### BC-AUDIT-416 — formal-verify: Missing tools must be reported, never silently skipped

**Skill:** `plugins/vsdd-factory/skills/formal-verify/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 174 (Prerequisites)
**Trigger:** Tool not found at runtime.
**Behavior:** "If a tool is not installed, report which tools are missing and skip that section. Never fail silently (SOUL.md #4)."
**Acceptance:** Missing tool produces an explicit report, not silent skip.

---

### 44. generate-pdf

**Skill:** `plugins/vsdd-factory/skills/generate-pdf/SKILL.md` (117 LOC)

### BC-AUDIT-417 — generate-pdf: Identity — convert markdown to 1898 & Co. branded PDF via pandoc + weasyprint

**Skill:** `plugins/vsdd-factory/skills/generate-pdf/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** User requests PDF / export report / convert markdown.
**Behavior:** Generates a 1898 & Co. branded PDF from a markdown research document.
**Acceptance:** Frontmatter `name: generate-pdf`; `tools: Read, Bash`; `disable-model-invocation: true`.

### BC-AUDIT-418 — generate-pdf: Required frontmatter fields are title, author, date

**Skill:** `plugins/vsdd-factory/skills/generate-pdf/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-46 (Step 2: Validate Frontmatter)
**Trigger:** Reading input markdown file.
**Behavior:** YAML frontmatter must include title, author, date. If missing, warn user and ask whether to proceed without title page or add frontmatter first.
**Acceptance:** Missing required field triggers a warn-and-prompt path, never silent generation.

### BC-AUDIT-419 — generate-pdf: Output PDF defaults to <input>.pdf in same directory

**Skill:** `plugins/vsdd-factory/skills/generate-pdf/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 48-58 (Step 3)
**Trigger:** generate-pdf.sh invoked.
**Behavior:** If no `-o <output>` specified, default is `<input>.pdf`.
**Acceptance:** Output filename rule.

### BC-AUDIT-420 — generate-pdf: Errors must be reported with specific solutions per error class

**Skill:** `plugins/vsdd-factory/skills/generate-pdf/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 108-117 (Error Handling)
**Trigger:** Any failure during PDF generation.
**Behavior:** Reports specific error and suggests fix (per error/solution table: pandoc not found → `brew install pandoc`, weasyprint not found → `brew install weasyprint`, etc.).
**Acceptance:** Failures map to one of the listed error categories with stated remediation.

---

### 45. guided-brief-creation

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md` (172 LOC)

### BC-AUDIT-421 — guided-brief-creation: Identity — staged elicitation from raw idea to product brief

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** L0 detected and human chooses brief creation; after brainstorming; or human says "help me write a product brief".
**Behavior:** Interactive facilitated workflow: understand intent → contextual discovery → guided elicitation → draft & review → optional adversarial review → finalize.
**Acceptance:** Frontmatter `name: guided-brief-creation`.

### BC-AUDIT-422 — guided-brief-creation: Hard gate — must complete brief before any PRD/architecture/implementation

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-16 (Hard Gate)
**Trigger:** Skill invoked.
**Behavior:** "Do NOT skip to PRD creation, architecture design, or any implementation activity. The product brief MUST be completed and validated before proceeding."
**Acceptance:** Gate is named "Hard Gate" and is bold-marked.

### BC-AUDIT-423 — guided-brief-creation: Capture-don't-interrupt rule preserves human creative flow

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 74-77 (Stage 1)
**Trigger:** Human shares details beyond current brief scope (e.g., requirements, technical preferences, timeline) during elicitation.
**Behavior:** "Capture-don't-interrupt: capture them for later. Don't redirect — let their creative flow continue."
**Acceptance:** Out-of-scope content is captured to elicitation-notes.md, not redirected.

### BC-AUDIT-424 — guided-brief-creation: Output is product-brief.md (and elicitation-notes.md if applicable)

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 134-141 (Stage 6: Finalize), 162-167 (Output Artifacts)
**Trigger:** Brief approval.
**Behavior:** Writes `.factory/planning/product-brief.md` and (if detailed notes captured) `.factory/planning/elicitation-notes.md`. Routes to Phase 1 (Spec Crystallization).
**Acceptance:** Both files exist if elicitation produced extra context; product-brief.md alone otherwise.

### BC-AUDIT-425 — guided-brief-creation: Failure mode — contradictory requirements halt elicitation for human resolution

**Skill:** `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 168-172 (Failure Modes)
**Trigger:** Human provides contradictory requirements during elicitation.
**Behavior:** Flag the specific contradictions and ask the human to resolve before proceeding.
**Acceptance:** Behavior is "flag + ask", not "make a guess".

---

### 46. holdout-eval

**Skill:** `plugins/vsdd-factory/skills/holdout-eval/SKILL.md` (78 LOC)

### BC-AUDIT-426 — holdout-eval: Identity — runs holdout evaluation with strict information asymmetry, returns satisfaction scores

**Skill:** `plugins/vsdd-factory/skills/holdout-eval/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** All wave stories merged to develop; before next wave; Phase 4.
**Behavior:** Spawns holdout-evaluator agent with strict information asymmetry — cannot see specs, source internals, or prior reviews. Returns satisfaction scores per hidden scenario.
**Acceptance:** Frontmatter `name: holdout-eval`; `argument-hint: "[wave-N]"`; `disable-model-invocation: true`; `context: fork`; `agent: holdout-evaluator`.

### BC-AUDIT-427 — holdout-eval: Iron Law — evaluator MUST NOT see specs, source, BCs, architecture, or prior reviews

**Skill:** `plugins/vsdd-factory/skills/holdout-eval/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-18 (Iron Law)
**Trigger:** Spawning holdout-evaluator.
**Behavior:** "NO HOLDOUT EVALUATION WITHOUT INFORMATION ASYMMETRY FIRST." Evaluator sees only product brief, hidden scenarios, running app, and test pass/fail (not test source code). Cannot see PRD, architecture, source internals, semport artifacts, adversarial reviews, or PR discussions.
**Acceptance:** Asymmetry wall list documented; breaking it = self-confirming test.

### BC-AUDIT-428 — holdout-eval: Gate is mean satisfaction ≥ 0.85 AND every critical scenario ≥ 0.60

**Skill:** `plugins/vsdd-factory/skills/holdout-eval/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 69-72 (Gate Criteria)
**Trigger:** Evaluation completes.
**Behavior:** PASS = mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60. FAIL = below thresholds → stories need remediation before next wave.
**Acceptance:** 0.85 / 0.60 thresholds; explicit "0.84 fails. No rounding." in red-flag table (line 27).

### BC-AUDIT-429 — holdout-eval: Output written to .factory/holdout-scenarios/evaluations/wave-<N>/

**Skill:** `plugins/vsdd-factory/skills/holdout-eval/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 74-78 (After Evaluation)
**Trigger:** Evaluation complete.
**Behavior:** Results written to `.factory/holdout-scenarios/evaluations/wave-<N>/`. PASS → next wave or Phase 5. FAIL → report gaps, create remediation stories.
**Acceptance:** Path constraint and routing rule.

---

### 47. implementation-readiness

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md` (149 LOC)

### BC-AUDIT-430 — implementation-readiness: Identity — gate between planning and building, validates spec package consistency

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** Spec Intake mode with L4 artifacts; after story decomposition; or human asks "are we ready to implement?".
**Behavior:** Validates complete spec package (PRD + architecture + stories) is internally consistent and ready for implementation.
**Acceptance:** Frontmatter `name: implementation-readiness`.

### BC-AUDIT-431 — implementation-readiness: Validation runs 8 dimensions in parallel, not sequential

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 11-13, 19-118 (Validation Dimensions)
**Trigger:** Skill execution.
**Behavior:** 6 (per skill body says "6 validation dimensions" in step-file note but body lists 8 numbered dimensions: PRD Completeness, Architecture Alignment, Story Coverage, Cross-Document Consistency, Context Budget, L3 BC Existence, L4 VP Existence, UX Alignment). Each runs as a parallel check.
**Acceptance:** All 8 dimensions reported in output table.

### BC-AUDIT-432 — implementation-readiness: PRD bloat check — narrative padding in requirements is a finding

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-29 (PRD Completeness)
**Trigger:** Dimension 1 runs.
**Behavior:** "Bloat check: PRD doesn't include narrative padding in the requirements sections."
**Acceptance:** Narrative padding flagged.

### BC-AUDIT-433 — implementation-readiness: Context budget warns when total exceeds 60% of agent context window

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 65-71 (Dimension 5: Context Budget Validation), 148 (Quality Gate)
**Trigger:** Dimension 5 runs.
**Behavior:** Sum all artifacts loaded into agent context during implementation. If total > 60% of agent's context window, flag for compression using Extended ToC pattern.
**Acceptance:** Quality Gate: "Context budget within 60% of implementing agent's context window".

### BC-AUDIT-434 — implementation-readiness: PRD implementation leakage scan flags premature tech decisions

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 77-89 (5b. PRD Implementation Leakage)
**Trigger:** Dimension 5b runs.
**Behavior:** Scans FRs/NFRs for premature technology names. Heuristic: if removing the tech name makes the requirement clearer, it was leakage. Severity: Error (definitely), Warning (possibly), Info (reference only).
**Acceptance:** Leakage findings carry severity and the heuristic rationale.

### BC-AUDIT-435 — implementation-readiness: PRD information density — Critical >10, Warning 5-10, Pass <5 issues per page

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 91-97 (5c. PRD Information Density)
**Trigger:** Dimension 5c runs.
**Behavior:** Scans for conversational filler, wordy phrases, redundant phrases, hedge words. Severity thresholds: Critical (>10), Warning (5-10), Pass (<5).
**Acceptance:** Density findings categorized by threshold.

### BC-AUDIT-436 — implementation-readiness: Story tokens 300-800; total context ≤60% of agent window

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 51-54 (Story Coverage), 65-71
**Trigger:** Dimension 3 runs.
**Behavior:** Each story 300-800 tokens covering AC, architectural constraints, 1-2 code references, and explicit non-goals. Each story's total context 20-30% of implementing agent's window.
**Acceptance:** Story content density check.

### BC-AUDIT-437 — implementation-readiness: Output is readiness-report.md with READY|CONCERNS|NOT_READY verdict

**Skill:** `plugins/vsdd-factory/skills/implementation-readiness/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 122-142 (Output)
**Trigger:** All dimensions evaluated.
**Behavior:** Writes `.factory/planning/readiness-report.md` with PASS/FAIL/N/A per dimension and overall verdict READY | CONCERNS | NOT_READY (and OVER for context budget overflow).
**Acceptance:** Quality Gate: "Readiness report written to `.factory/planning/readiness-report.md` with overall verdict".

---

### 48. intelligence-synthesis

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md` (250 LOC)

### BC-AUDIT-438 — intelligence-synthesis: Identity — correlates market/feedback/competitive/analytics into scored insights

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-18
**Trigger:** Scheduled (weekly), manual, or after discovery.lobster ingestion completes.
**Behavior:** Correlates signals across market research, customer feedback, competitive monitoring, and usage analytics. Clusters into themes, scores evidence strength, formats insights for DF-017.
**Acceptance:** Frontmatter `name: intelligence-synthesis`; primary agent `business-analyst`; supporting `[research-agent]`.

### BC-AUDIT-439 — intelligence-synthesis: Market research is the only required input; works with partial data

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 35-52 (Input Sources, Prerequisites)
**Trigger:** Skill execution.
**Behavior:** Market research required. Customer feedback, competitive intel, usage analytics optional. Synthesis layer degrades gracefully — only market research → 0.3-0.5 evidence; all sources → 0.9-1.0.
**Acceptance:** Quality Criteria: "Works with partial data".

### BC-AUDIT-440 — intelligence-synthesis: Themes formed by semantic clustering across sources, not per-source listing

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 95-110 (Step 3: Theme Extraction), 244 (Quality Criteria)
**Trigger:** Step 3 runs.
**Behavior:** Cluster related signals across sources (semantic clustering). Cross-source correlation strengthens themes. Quality criterion: "Theme extraction clusters related signals (not just lists)."
**Acceptance:** Themes are clusters, not flat per-source lists.

### BC-AUDIT-441 — intelligence-synthesis: Insights scored on 7 dimensions including evidence_strength

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 142-157 (Step 6)
**Trigger:** Theme determined to merit an insight.
**Behavior:** Apply DF-017 7-dimension scoring: Value (0.25), Feasibility (0.15), Alignment (0.15), Novelty (0.10), Time-Criticality (0.10), Effort (0.10), Evidence Strength (0.15).
**Acceptance:** Each insight has 7 dimension scores + composite.

### BC-AUDIT-442 — intelligence-synthesis: Routing — composite ≥0.7 AND evidence ≥0.6 → Brief; URGENT competitive HIGH triggers immediate human notification

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 159-167 (Step 7: Determine Recommended Action)
**Trigger:** Insight scored.
**Behavior:** Composite ≥0.7 + evidence ≥0.6 → Brief→Planning. 0.5-0.7 → Backlog. <0.5 + <0.4 evidence → Registry. Competitive HIGH + composite ≥0.7 → URGENT immediate human notification.
**Acceptance:** Routing rules per the routing table.

### BC-AUDIT-443 — intelligence-synthesis: Output is insights-YYYY-MM-DD.md with frontmatter and per-insight detail

**Skill:** `plugins/vsdd-factory/skills/intelligence-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 169-219 (Step 8: Produce Insights Report), 234 (Quality Gate)
**Trigger:** Synthesis complete.
**Behavior:** Writes `.factory/discovery/insights-YYYY-MM-DD.md` with document_type, date, product, insights_count, sources_available frontmatter and per-insight signal correlation table + scores + recommended action.
**Acceptance:** Quality Gate: "Insights report written to `.factory/discovery/insights-YYYY-MM-DD.md`".

---

### 49. jira

**Skill:** `plugins/vsdd-factory/skills/jira/SKILL.md` (149 LOC)
**Type:** Reference-only.

### BC-AUDIT-444 — jira: Identity is reference-only documentation for ankitpokhrel/jira-cli

**Skill:** `plugins/vsdd-factory/skills/jira/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Agent needs Jira commands.
**Behavior:** Reference for Jira projects DPGR (Discovery — initiatives, ideation, roadmap, research) and DPGD (Delivery — epics, stories, tasks, subtasks). Lists common commands (issue list, view, create, edit, transition, sprint, epic, worklog) and pagination rules (100 per request default).
**Acceptance:** Frontmatter `name: jira`; description states "Reference documentation ... Reference-only skill, not directly invokable."; `disable-model-invocation: true`.

---

### 50. maintenance-sweep

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md` (290 LOC)

### BC-AUDIT-445 — maintenance-sweep: Identity — periodic sweeps + cleanup PRs through standard quality gates

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Scheduled (weekly cron), manual ("Run maintenance sweep"), or post-deploy after a Feature Mode merge.
**Behavior:** Periodic quality sweep across dependencies, doc drift, pattern consistency, holdout freshness, performance, DTU fidelity, spec coherence, tech debt, and accessibility. Opens cleanup PRs via standard pipeline.
**Acceptance:** Frontmatter `name: maintenance-sweep`.

### BC-AUDIT-446 — maintenance-sweep: 9 sweep types run in parallel after STARTED commit

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19-159 (Sweep Types)
**Trigger:** Trigger fires.
**Behavior:** 9 named sweeps: Dependency Audit, Documentation Drift, Pattern Consistency, Holdout Freshness, Performance Regression Detection, DTU Fidelity Drift, Spec Coherence, Tech Debt Register, Accessibility Regression. Run in parallel; state-manager commits after each.
**Acceptance:** Execution flow shows 9 sweeps in parallel block.

### BC-AUDIT-447 — maintenance-sweep: Dependency audit splits T3 (run scans) and T2 (analyze)

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19-46 (Sweep 1)
**Trigger:** Dependency audit sweep runs.
**Behavior:** dx-engineer (T3) runs cargo audit, cargo deny check, npm audit, pip-audit. security-reviewer (T2) analyzes results, classifies severity. CRITICAL/HIGH CVE → immediate fix PR; MEDIUM → next sprint log; LOW → log only.
**Acceptance:** Tier split documented; severity routing.

### BC-AUDIT-448 — maintenance-sweep: Performance regression — >25% triggers PR; 10-25% logs trend; <10% no action

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 99-114 (Sweep 5)
**Trigger:** Performance baseline sweep.
**Behavior:** >25% degradation → open immediate PR (or flag if cause unclear). 10-25% → log with trend data. <10% → no action.
**Acceptance:** Threshold table at lines 110-113.

### BC-AUDIT-449 — maintenance-sweep: Auto-PR quality gate same as Feature Mode (regression + holdout + adversarial + lint)

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 215-223 (Quality Gate for Auto-Generated PRs)
**Trigger:** Auto-PR opened.
**Behavior:** All existing tests pass (regression). Holdout scenarios pass (≥90%). Adversarial review (scoped to changed files). Lint/format clean.
**Acceptance:** "Maintenance PRs go through the SAME quality gates as Feature Mode code."

### BC-AUDIT-450 — maintenance-sweep: Output is sweep-report-YYYY-MM-DD.md plus per-sweep findings files

**Skill:** `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 205-213 (Output Artifacts)
**Trigger:** All sweeps complete.
**Behavior:** Writes sweep-report-YYYY-MM-DD.md, dependency-audit.log, doc-drift-findings.md, pattern-findings.md, holdout-freshness.md, performance-baseline.md to `.factory/maintenance/`.
**Acceptance:** All 6 artifact paths listed.

---

### 51. market-intelligence-assessment

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md` (250 LOC)

### BC-AUDIT-451 — market-intelligence-assessment: Identity — mandatory pre-spec gate producing GO/CAUTION/STOP

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** Before any spec work begins on a new product idea, feature request, or spec.
**Behavior:** Produces GO / CAUTION / STOP recommendation. Researches competitive landscape, market size, customer pain validation, differentiation, risk signals.
**Acceptance:** Frontmatter `name: market-intelligence-assessment`; `gate: Human reviews market intel before proceeding`.

### BC-AUDIT-452 — market-intelligence-assessment: 5 parallel research tracks via research-agent + Perplexity

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-99 (Step 2)
**Trigger:** After research brief extracted.
**Behavior:** research-agent runs 5 parallel tracks: 2a Competitive Landscape, 2b Market Size, 2c Customer Pain Validation, 2d Differentiation Opportunities, 2e Risk Signals.
**Acceptance:** All five tracks specified.

### BC-AUDIT-453 — market-intelligence-assessment: Recommendation criteria — GO requires pain confirmed + market viable + differentiation + manageable risks

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 173-179 (Recommendation Criteria)
**Trigger:** Synthesis step.
**Behavior:** GO = pain confirmed, market viable, clear differentiation, manageable risks. CAUTION = pain partial OR differentiation unclear OR significant risks. STOP = pain unvalidated, market too small, saturated/no-differentiation, or showstopper risks.
**Acceptance:** Three-tier rubric with specific criteria.

### BC-AUDIT-454 — market-intelligence-assessment: Depth scaled by input level L0-L4; L4 has auto-GO without human gate

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 207-237 (Depth Configuration)
**Trigger:** Skill invoked with input artifact level.
**Behavior:** L0 → all 5 sections, max depth, 15-30 min. L1 → sections 1, 3, 5. L2 → sections 1, 4, 5. L3 → section 5 + summary 1. L4 → headline check; auto-GO if no material changes.
**Acceptance:** Depth ladder per L-level.

### BC-AUDIT-455 — market-intelligence-assessment: Output is market-intel.md; STOP override is recorded with reasoning

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 102-170 (Step 3 template), 185-204 (Step 4: Human Review Gate)
**Trigger:** Synthesis complete.
**Behavior:** Writes `.factory/planning/market-intel.md` with frontmatter recommendation/confidence/input_level/timestamp. Human override of STOP recorded in STATE.md and carried forward to all downstream agents.
**Acceptance:** Output file path; STATE.md captures human_decision.

### BC-AUDIT-456 — market-intelligence-assessment: Quality Gate — assumptions explicitly flagged for human validation

**Skill:** `plugins/vsdd-factory/skills/market-intelligence-assessment/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 239-244 (Quality Gate)
**Trigger:** Report generation.
**Behavior:** Quality Gate items: GO/CAUTION/STOP with confidence; supporting evidence cited (not opinion); all sections per depth config; assumptions explicitly flagged.
**Acceptance:** Four-item Quality Gate.

---

### 52. mode-decision-guide

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md` (151 LOC)
**Type:** Reference document (no quality gate — line 149).

### BC-AUDIT-457 — mode-decision-guide: Identity — reference doc for orchestrator mode detection

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** Orchestrator runs mode detection.
**Behavior:** Quick decision table maps scenarios to modes (Greenfield Phases 1-7, Brownfield Phase 0, Feature Mode F1-F7, Multi-Repo, Maintenance, Discovery).
**Acceptance:** Frontmatter `name: mode-decision-guide`; document declares "Reference document — no quality gate."

### BC-AUDIT-458 — mode-decision-guide: Feature Mode threshold — <30% files changed AND <50% components AND ≤2 cascade levels

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 75-83 (Feature Mode Threshold)
**Trigger:** Mode is not obvious; quantitative heuristic applied.
**Behavior:** Feature Mode appropriate when files changed <30% AND components affected <50% AND fewer new modules than existing AND dependency depth ≤ 2.
**Acceptance:** All four numeric thresholds.

### BC-AUDIT-459 — mode-decision-guide: Greenfield switchover — ≥30% files OR ≥50% components OR breaking interfaces OR ≥3 cascade levels

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 85-91 (Greenfield Threshold)
**Trigger:** Mode auto-detection.
**Behavior:** Switch to Greenfield when ≥30% files changed, ≥50% components affected, breaking interface changes, or cascade through 3+ levels.
**Acceptance:** All four numeric thresholds.

### BC-AUDIT-460 — mode-decision-guide: Bug fix minimal route — F1 → F4 → F5 → F7, skip F2/F3

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 93-98 (Bug Fix Minimal Feature Mode)
**Trigger:** Bug fix detected.
**Behavior:** Run F1 (scope), F4 (TDD fix), F5 (review), F7 (converge). Skip F2 (spec unchanged), F3 (no new stories). Optional F6 (only if security-critical module).
**Acceptance:** Run/Skip lists explicit.

### BC-AUDIT-461 — mode-decision-guide: Human override always wins over auto-detection

**Skill:** `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 140-146 (Mode Override)
**Trigger:** Human says "Run X mode".
**Behavior:** "The Orchestrator respects explicit human mode selection over auto-detection."
**Acceptance:** Override rule.

---

### 53. model-routing

**Skill:** `plugins/vsdd-factory/skills/model-routing/SKILL.md` (70 LOC)
**Type:** Reference document.

### BC-AUDIT-462 — model-routing: Identity — LiteLLM model tier assignment reference

**Skill:** `plugins/vsdd-factory/skills/model-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** Orchestrator selects model tier for an agent.
**Behavior:** Documents 8 tiers (judgment, implementation, validation, adversary, review, fallback/fast, fallback/standard, fallback/reasoning) with cost and use case.
**Acceptance:** Frontmatter `name: model-routing`.

### BC-AUDIT-463 — model-routing: Iron rule — Adversary MUST use adversary tier (GPT-5.4), never Claude

**Skill:** `plugins/vsdd-factory/skills/model-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-32 (Routing Rules), 17 (Tier Definitions)
**Trigger:** Adversary agent dispatch.
**Behavior:** "The Adversary MUST use adversary tier for primary pass — never judgment or implementation." Holdout evaluation MUST also use adversary tier (GPT-5.4); fallback to DeepSeek-V3 (NOT Codestral). Small models (fallback/fast) MUST NOT be used for adversarial validation.
**Acceptance:** Three-fold rule: never Claude for adversary, never small models for adversarial validation, holdout fallback skips Codestral.

### BC-AUDIT-464 — model-routing: Three-tier fallback — primary → standard fallback → reasoning fallback (for adversary/judgment) or fast fallback → standard (for impl/validation)

**Skill:** `plugins/vsdd-factory/skills/model-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 51-60 (Fallback Behavior)
**Trigger:** Primary model fails after 2 retries.
**Behavior:** Implementation/Validation → fallback/fast (Codestral) → fallback/standard (DeepSeek-V3). Adversary/Judgment → fallback/standard (DeepSeek-V3) → fallback/reasoning (Qwen3-235B thinking). Review → fallback/standard. fallback/standard via OpenRouter falls to local DeepSeek-V3 for sensitive data.
**Acceptance:** Routing chains per tier.

### BC-AUDIT-465 — model-routing: Compounding correctness — pause if budget forces downgrade in P3-P5

**Skill:** `plugins/vsdd-factory/skills/model-routing/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 35-48 (Compounding Correctness Constraint)
**Trigger:** Budget constraint forces model downgrade during P3-P5.
**Behavior:** "Pause the pipeline and resume when the primary model is available than to continue with an underpowered model that compounds errors." cost-tracker warns if total spend < 10% of budget at end of Phase 5 (suggests adversarial review wasn't thorough).
**Acceptance:** Pause-not-downgrade rule; underspend warning.

---

### 54. multi-repo-health

**Skill:** `plugins/vsdd-factory/skills/multi-repo-health/SKILL.md` (23 LOC)

### BC-AUDIT-466 — multi-repo-health: Identity — scan .worktrees/ for multi-repo layout, report repos with manifests

**Skill:** `plugins/vsdd-factory/skills/multi-repo-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Before multi-repo phase-0 synthesis.
**Behavior:** Wraps `bin/multi-repo-scan`. Detects multi-repo layouts; reports per-repo name/path, manifest type (Cargo.toml/package.json/pyproject.toml/go.mod/unknown), branch, dirty/clean status.
**Acceptance:** Frontmatter `name: multi-repo-health`.

### BC-AUDIT-467 — multi-repo-health: Read-only — does not mutate any repo

**Skill:** `plugins/vsdd-factory/skills/multi-repo-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 23
**Trigger:** Skill invoked.
**Behavior:** "Read-only. Does not mutate any repo."
**Acceptance:** No write operations performed.

### BC-AUDIT-468 — multi-repo-health: Single-repo path — count == 0 reports "single-repo project" and stops

**Skill:** `plugins/vsdd-factory/skills/multi-repo-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 11-12
**Trigger:** `bin/multi-repo-scan --count` returns 0.
**Behavior:** "If 0, report 'single-repo project' and stop."
**Acceptance:** Early termination on single-repo.

### BC-AUDIT-469 — multi-repo-health: Story-repo cross-check warns when stories reference undetected repo or repo lacks stories

**Skill:** `plugins/vsdd-factory/skills/multi-repo-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 19
**Trigger:** Repos detected.
**Behavior:** "Cross-check against `.factory/stories/` — warn if a repo has no stories, or if there are stories targeting an undetected repo."
**Acceptance:** Both warning conditions.

---

### 55. multi-repo-phase-0-synthesis

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md` (164 LOC)

### BC-AUDIT-470 — multi-repo-phase-0-synthesis: Identity — synthesizes per-repo ingestion outputs into unified project context

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-23
**Trigger:** All per-repo ingestions complete.
**Behavior:** Produces 7 unified artifacts: cross-repo-dependencies.md, unified-architecture/, convention-reconciliation.md, unified-security-posture.md, cross-repo-holdout-scenarios/, project-context.md, synthesis-validation-report.md.
**Acceptance:** Frontmatter `name: multi-repo-phase-0-synthesis`; 7 outputs declared; `gate: Human reviews and approves unified project context`.

### BC-AUDIT-471 — multi-repo-phase-0-synthesis: 8 sequential synthesis steps with named agent per step

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 36-135 (Synthesis Steps)
**Trigger:** Skill invoked.
**Behavior:** Step 1 codebase-analyzer (deps), 2 architect (unified arch), 3 consistency-validator (conventions), 4 security-reviewer (security), 5 product-owner (holdout scenarios), 6 codebase-analyzer (project context), 7 adversary (review), 8 consistency-validator (validation).
**Acceptance:** 8 steps, agent assignments.

### BC-AUDIT-472 — multi-repo-phase-0-synthesis: Adversary review uses information asymmetry wall — cannot see raw codebase

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 116-122 (Step 7)
**Trigger:** Step 7 runs.
**Behavior:** "Cannot see raw codebase (per DF-025 wall). Reviews synthesized artifacts only. Challenges assumptions about cross-repo interactions. Flags missing integration concerns."
**Acceptance:** Asymmetry wall enforced.

### BC-AUDIT-473 — multi-repo-phase-0-synthesis: All T1 writes routed through state-manager

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 145-149 (Write Routing)
**Trigger:** codebase-analyzer (T1) needs to write.
**Behavior:** "All writes by codebase-analyzer (T1) are routed through state-manager, consistent with DF-023 tier model. state-manager commits to factory-project-artifacts branch."
**Acceptance:** Tier 1 write routing.

### BC-AUDIT-474 — multi-repo-phase-0-synthesis: Quality Gate — unified project-context.md exists with cross-repo dependencies, conventions, validation

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 152-158 (Quality Gate)
**Trigger:** Synthesis complete.
**Behavior:** Quality Gate: unified project-context.md exists; cross-repo deps map all inter-repo API calls + shared types; per-repo files synthesized; convention conflicts have resolutions; synthesis validation report passes.
**Acceptance:** Five checklist items.

### BC-AUDIT-475 — multi-repo-phase-0-synthesis: Failure — incomplete per-repo ingestion halts synthesis

**Skill:** `plugins/vsdd-factory/skills/multi-repo-phase-0-synthesis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 160-164 (Failure Modes)
**Trigger:** Per-repo ingestion incomplete.
**Behavior:** "Halt synthesis and report which repo needs completion." Circular cross-repo deps → flag and escalate to architect. Convention conflicts with no clear resolution → present both options to human.
**Acceptance:** Three failure-mode behaviors.

---

### 56. multi-variant-design

**Skill:** `plugins/vsdd-factory/skills/multi-variant-design/SKILL.md` (88 LOC)

### BC-AUDIT-476 — multi-variant-design: Identity — generates 2-3 variants per complex screen scored by 4 agents on 6 dimensions

**Skill:** `plugins/vsdd-factory/skills/multi-variant-design/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** Complex screens (>5 interactive elements OR complex data display OR high-traffic OR human asks "explore alternatives").
**Behavior:** ux-designer generates 3 variants. 4 agents score each on Usability (business-analyst), Accessibility (accessibility-auditor), Performance (architect), Design Compliance (consistency-validator), Visual Hierarchy + Responsive Fitness (ux-designer).
**Acceptance:** Frontmatter `name: multi-variant-design`; condition `feature_type in ['ui', 'full-stack']`.

### BC-AUDIT-477 — multi-variant-design: Top variant + runner-up presented for human selection or synthesis

**Skill:** `plugins/vsdd-factory/skills/multi-variant-design/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 39-55 (Procedure)
**Trigger:** Composite scores computed.
**Behavior:** Top variant + runner-up presented to human. Human selects or requests synthesis of best aspects. Selected variant becomes UX spec.
**Acceptance:** Output presents two variants; gate is human selection.

### BC-AUDIT-478 — multi-variant-design: Output is SCR-NNN-variants.md per complex screen

**Skill:** `plugins/vsdd-factory/skills/multi-variant-design/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14, 56-81 (Variant Document Format)
**Trigger:** Variants generated.
**Behavior:** Writes `.factory/ui-quality/variants/SCR-NNN-variants.md` per screen with variant descriptions, scores, recommendation, and human decision tracker.
**Acceptance:** Output path; document format.

### BC-AUDIT-479 — multi-variant-design: Failure — score deadlock (within 0.05) presents both with dimension breakdown

**Skill:** `plugins/vsdd-factory/skills/multi-variant-design/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 84-88 (Failure Modes)
**Trigger:** Two variants within 0.05 composite score.
**Behavior:** "Present both to human with dimension-level breakdown for tiebreaking." Fewer than 2 distinct variants → report constraint, proceed with single design. Missing scorer → skip dimension, reweight.
**Acceptance:** Three failure-mode behaviors.

---

### 57. next-step

**Skill:** `plugins/vsdd-factory/skills/next-step/SKILL.md` (40 LOC)

### BC-AUDIT-480 — next-step: Identity — read STATE.md and propose next workflow step, do not execute

**Skill:** `plugins/vsdd-factory/skills/next-step/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Resuming work on an in-flight pipeline.
**Behavior:** Cross-references pipeline state with workflow data. Reads STATE.md, determines mode/phase, resolves workflow file, enumerates completed steps, finds first uncompleted step with satisfied dependencies, reports proposal.
**Acceptance:** Frontmatter `name: next-step`.

### BC-AUDIT-481 — next-step: STATE.md missing → directs user to factory-health and stops

**Skill:** `plugins/vsdd-factory/skills/next-step/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 12
**Trigger:** STATE.md does not exist.
**Behavior:** "If it doesn't exist, tell the user to run `/vsdd-factory:factory-health` and stop."
**Acceptance:** Stop-on-missing-state behavior.

### BC-AUDIT-482 — next-step: Does not execute — proposal only

**Skill:** `plugins/vsdd-factory/skills/next-step/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 34
**Trigger:** Skill complete.
**Behavior:** "Do not execute. This skill only proposes. The user decides whether to run `/vsdd-factory:run-phase` or invoke the step manually."
**Acceptance:** Proposal-only mode.

### BC-AUDIT-483 — next-step: Uses lobster-parse to enumerate workflow steps with dependencies

**Skill:** `plugins/vsdd-factory/skills/next-step/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-26
**Trigger:** Workflow file resolved.
**Behavior:** Uses `${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse <file> '.workflow.steps[] | {name, agent, depends_on, task}'` to enumerate steps with their dependencies.
**Acceptance:** lobster-parse jq query specified.

---

### 58. onboard-observability

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md` (132 LOC)

### BC-AUDIT-484 — onboard-observability: Identity — registers project + writes Claude OTel env vars; idempotent

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** User asks to "register this project with observability", "set up observability here", "onboard the observability stack", or similar first-time setup phrasing.
**Behavior:** Two-part setup: (1) `factory-obs register` adds project's `.factory/logs` to collector watch list; (2) writes 5 OTEL_* env vars to `.claude/settings.local.json` so Claude ships tool calls/tokens/costs to Loki+Prometheus. Idempotent.
**Acceptance:** Frontmatter `name: onboard-observability`; `allowed-tools: Bash, Read, Write`.

### BC-AUDIT-485 — onboard-observability: Required announce-at-start verbatim message

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 16-20
**Trigger:** Skill invocation.
**Behavior:** Before any other action, says verbatim: "I'm using the onboard-observability skill to wire this project into the local observability stack."
**Acceptance:** Exact announce string.

### BC-AUDIT-486 — onboard-observability: Aborts if no .factory/ ancestor or factory-obs binary missing

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-28 (Prerequisites)
**Trigger:** Prerequisite check.
**Behavior:** Walks up from $PWD to find `.factory/`. If none, aborts with clear error. If `${CLAUDE_PLUGIN_ROOT}/bin/factory-obs` missing, aborts with "vsdd-factory plugin not found" error. Never fails silently.
**Acceptance:** Two abort conditions; explicit "Don't fail silently" rule.

### BC-AUDIT-487 — onboard-observability: Writes exactly 5 OTEL_* env vars; preserves all other keys

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 56-71 (Step 2), 105-110 (Do not touch)
**Trigger:** Step 2 runs.
**Behavior:** Writes CLAUDE_CODE_ENABLE_TELEMETRY=1, OTEL_METRICS_EXPORTER=otlp, OTEL_LOGS_EXPORTER=otlp, OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf, OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318. Prunes legacy OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE. Other keys in `.env` and other top-level keys in settings.local.json preserved exactly.
**Acceptance:** 5 specific keys; jq-merge approach; preservation rule.

### BC-AUDIT-488 — onboard-observability: Idempotency — register dedupes on absolute path; jq merge overwrites only the 5 keys

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 96-104 (Idempotency)
**Trigger:** Skill re-invoked on already-onboarded project.
**Behavior:** factory-obs register dedupes on absolute path. jq merge preserves existing env keys, only overwrites 5 OTEL_* keys.
**Acceptance:** Both idempotency mechanisms.

### BC-AUDIT-489 — onboard-observability: Non-goals — does not start/stop Docker stack, does not unregister, does not run cloud-only

**Skill:** `plugins/vsdd-factory/skills/onboard-observability/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 113-119 (Non-goals)
**Trigger:** Edge cases.
**Behavior:** Does NOT start/stop Docker stack (use `/vsdd-factory:factory-obs up`). Does NOT register multiple projects at once. Does NOT unregister. No cloud fallback.
**Acceptance:** All four "does not" rules.

---

### 59. perf-check

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md` (124 LOC)

### BC-AUDIT-490 — perf-check: Identity — bench regression + resource profiling + budget compliance

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** Performance validation requested.
**Behavior:** Runs benchmark suite, binary size, startup time, memory profiling, compile time, test suite performance.
**Acceptance:** Frontmatter `name: perf-check`; `disable-model-invocation: true`.

### BC-AUDIT-491 — perf-check: Regression threshold — flag > 10% vs baseline

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-32 (Benchmark Suite)
**Trigger:** Bench run completes.
**Behavior:** Look for regressions > 10% vs baseline; new benchmarks for added functionality; P50/P95/P99 latency distributions.
**Acceptance:** 10% regression threshold.

### BC-AUDIT-492 — perf-check: Default budgets — startup <100ms; binary <50MB; debug build <60s; tests <120s

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 86-95 (Budgets)
**Trigger:** Budget check.
**Behavior:** Default budgets table: CLI startup <100ms; binary size release <50MB; debug build time <60s; test suite <120s. Defined in `.factory/specs/prd-supplements/performance-budgets.md` if exists.
**Acceptance:** Four budget values.

### BC-AUDIT-493 — perf-check: Output is performance-report.md with PASS|WARN|FAIL gate

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 100-122 (Output)
**Trigger:** All checks complete.
**Behavior:** Writes `.factory/cycles/<current>/performance-report.md` with summary table (Metric | Budget | Measured | Status), benchmark results, regressions, recommendations, and Gate: PASS | WARN | FAIL.
**Acceptance:** Output path; three-state gate.

### BC-AUDIT-494 — perf-check: No benchmarks → report and recommend creating them

**Skill:** `plugins/vsdd-factory/skills/perf-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 124
**Trigger:** Project has no benchmarks.
**Behavior:** "If no benchmarks exist yet, report that and recommend creating them."
**Acceptance:** Graceful fallback rule.

---

### 60. phase-0-codebase-ingestion

**Skill:** `plugins/vsdd-factory/skills/phase-0-codebase-ingestion/SKILL.md` (47 LOC)

### BC-AUDIT-495 — phase-0-codebase-ingestion: Identity — Phase 0 entry point delegating to brownfield-ingest sub-workflow

**Skill:** `plugins/vsdd-factory/skills/phase-0-codebase-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Brownfield project entry; Phase 0 begin.
**Behavior:** Delegates to `${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-0-codebase-ingestion.lobster`. Six steps A-F (source acquisition, broad sweep, convergence deepening, coverage audit, extraction validation, final synthesis).
**Acceptance:** Frontmatter `name: phase-0-codebase-ingestion`.

### BC-AUDIT-496 — phase-0-codebase-ingestion: Gate — context doc, criticality, BCs (origin: recovered), coverage PASS, drift clean, human approval

**Skill:** `plugins/vsdd-factory/skills/phase-0-codebase-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-47 (Gate Criteria)
**Trigger:** Phase 0 sub-workflow complete.
**Behavior:** Project context document exists; module criticality classification exists; BCs extracted with `origin: recovered`; coverage audit PASS; extraction validation PASS; input-hash drift check clean; human approval.
**Acceptance:** Seven gate items.

### BC-AUDIT-497 — phase-0-codebase-ingestion: Direct work skill is /vsdd-factory:brownfield-ingest <path>

**Skill:** `plugins/vsdd-factory/skills/phase-0-codebase-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-30 (Work Skill)
**Trigger:** Direct command invocation.
**Behavior:** Direct work command is `/vsdd-factory:brownfield-ingest <path>`.
**Acceptance:** Command syntax.

---

### 61. phase-1-prd-revision

**Skill:** `plugins/vsdd-factory/skills/phase-1-prd-revision/SKILL.md` (60 LOC)

### BC-AUDIT-498 — phase-1-prd-revision: Identity — PO revises PRD per architect feasibility report; max 3 iterations

**Skill:** `plugins/vsdd-factory/skills/phase-1-prd-revision/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-18
**Trigger:** Architect feasibility report flags issues.
**Behavior:** product-owner revises PRD and BCs to address architect concerns. Max 3 iterations before escalation.
**Acceptance:** Frontmatter `name: phase-1-prd-revision`; `gate: Architect approves or max 3 iterations reached`.

### BC-AUDIT-499 — phase-1-prd-revision: Skip when feasibility report says "validated — no issues"

**Skill:** `plugins/vsdd-factory/skills/phase-1-prd-revision/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 45-46 (Skip Condition)
**Trigger:** Architect feasibility report has no flags.
**Behavior:** "If architect's feasibility report says 'validated — no issues': skip this step entirely."
**Acceptance:** Skip condition rule.

### BC-AUDIT-500 — phase-1-prd-revision: 3-round deadlock escalates to human with both positions

**Skill:** `plugins/vsdd-factory/skills/phase-1-prd-revision/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 56-60 (Failure Modes)
**Trigger:** Architect and PO disagree after 3 rounds.
**Behavior:** "Escalate to human with both positions." Feasibility report references missing BCs → route to PO to create them first. NFR conflicts unresolvable without scope reduction → present trade-offs to human.
**Acceptance:** Three failure-mode escalation paths.

### BC-AUDIT-501 — phase-1-prd-revision: Quality Gate — every concern addressed or contested with rationale

**Skill:** `plugins/vsdd-factory/skills/phase-1-prd-revision/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 48-54 (Quality Gate)
**Trigger:** Revision complete.
**Behavior:** Every architect concern addressed or contested with rationale; PRD updated with architectural constraint annotations; BC postconditions adjusted for testability where verifiability flagged; feasibility review passes (architect approves) or max 3 iterations reached.
**Acceptance:** Four checklist items.

---

### 62. phase-1-spec-crystallization

**Skill:** `plugins/vsdd-factory/skills/phase-1-spec-crystallization/SKILL.md` (50 LOC)

### BC-AUDIT-502 — phase-1-spec-crystallization: Identity — Phase 1 entry point spanning brief → architecture

**Skill:** `plugins/vsdd-factory/skills/phase-1-spec-crystallization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 1 entry.
**Behavior:** Six steps A-F: create-brief, create-domain-spec, create-prd, create-architecture, phase-1-prd-revision (conditional), phase-1d-adversarial-spec-review.
**Acceptance:** Frontmatter `name: phase-1-spec-crystallization`.

### BC-AUDIT-503 — phase-1-spec-crystallization: Gate — IDs unique, VPs cover security boundaries, purity map complete, adversarial converged, human approves

**Skill:** `plugins/vsdd-factory/skills/phase-1-spec-crystallization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-50 (Gate Criteria)
**Trigger:** Phase 1 sub-workflow complete.
**Behavior:** All requirements have unique IDs and numerical targets; Provable Properties Catalog covers all security boundaries; Purity Boundary Map complete; verification tooling selected and documented; module criticality classification written; adversarial spec review converged; input-hash drift check clean; human approval.
**Acceptance:** Eight gate items.

### BC-AUDIT-504 — phase-1-spec-crystallization: Sub-workflow is workflows/phases/phase-1-spec-crystallization.lobster

**Skill:** `plugins/vsdd-factory/skills/phase-1-spec-crystallization/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 11-15
**Trigger:** Phase entry.
**Behavior:** Executes `${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-1-spec-crystallization.lobster` with steps A-F mapping to six sub-skills.
**Acceptance:** Sub-workflow path.

---

### 63. phase-1d-adversarial-spec-review

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md` (81 LOC)

### BC-AUDIT-505 — phase-1d-adversarial-spec-review: Identity — adversary reviews spec package with fresh context

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase 1 spec artifacts complete.
**Behavior:** Presents complete spec package (PRD, architecture, UX, BCs, VPs) to adversary (different model family, fresh context) for adversarial review.
**Acceptance:** Frontmatter `name: phase-1d-adversarial-spec-review`.

### BC-AUDIT-506 — phase-1d-adversarial-spec-review: Adversary reviews 7 categories — ambiguity, missing edges, implicit assumptions, contradictions, testable-vs-provable, purity, tool mismatch

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-44 (Step 2)
**Trigger:** Spawning adversary.
**Behavior:** Adversary reviews for: ambiguous language, missing edge cases, implicit unstated assumptions, contradictions between sections, "testable only" properties that should be provable, purity boundary violations, verification tool mismatches.
**Acceptance:** Seven review categories.

### BC-AUDIT-507 — phase-1d-adversarial-spec-review: Findings triaged C/H/M/L; cross-doc sync check before re-review

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 46-62 (Steps 4 and 4b)
**Trigger:** Adversary writes findings.
**Behavior:** CRITICAL → fix immediately + re-review. HIGH → fix before Phase 2. MEDIUM → document but don't block. LOW → log and continue. After remediation: consistency-validator runs cross-document sync check between remediated docs (sequential, upstream first). Only re-review after sync passes.
**Acceptance:** Severity routing + sync check sequencing.

### BC-AUDIT-508 — phase-1d-adversarial-spec-review: Convergence — adversary reports "CONVERGENCE REACHED — findings are cosmetic only"

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 64-67 (Step 5)
**Trigger:** Iterated review.
**Behavior:** "After fixes, spawn adversary AGAIN with fresh context. Repeat until adversary reports: 'CONVERGENCE REACHED — findings are cosmetic only.'"
**Acceptance:** Exact convergence string.

### BC-AUDIT-509 — phase-1d-adversarial-spec-review: Quality Gate — different model family + fresh context every pass + all C/H resolved + convergence reported

**Skill:** `plugins/vsdd-factory/skills/phase-1d-adversarial-spec-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-81 (Quality Gate)
**Trigger:** Phase complete.
**Behavior:** Adversary is different model family from builder. Fresh context every pass. All critical/high findings resolved. Adversary reports convergence.
**Acceptance:** Four checklist items.

---

### 64. phase-2-story-decomposition

**Skill:** `plugins/vsdd-factory/skills/phase-2-story-decomposition/SKILL.md` (46 LOC)

### BC-AUDIT-510 — phase-2-story-decomposition: Identity — Phase 2 entry point delegating to decompose-stories sub-workflow

**Skill:** `plugins/vsdd-factory/skills/phase-2-story-decomposition/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 1 complete.
**Behavior:** Five steps A-E: define-epics, create-stories, dependency-graph, wave-schedule (with index + sprint state), holdout-scenarios.
**Acceptance:** Frontmatter `name: phase-2-story-decomposition`.

### BC-AUDIT-511 — phase-2-story-decomposition: Gate — every BC traces to ≥1 story, no placeholder ACs, no cycles, ≥1 holdout/wave, drift clean, human approval

**Skill:** `plugins/vsdd-factory/skills/phase-2-story-decomposition/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-46 (Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Every BC traces to at least one story; no placeholder acceptance criteria; dependency graph has no cycles; wave assignments respect dependencies; ≥1 holdout scenario per wave; input-hash drift check clean; human approval.
**Acceptance:** Seven gate items.

### BC-AUDIT-512 — phase-2-story-decomposition: Direct command is /vsdd-factory:decompose-stories

**Skill:** `plugins/vsdd-factory/skills/phase-2-story-decomposition/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-30
**Trigger:** Direct command invocation.
**Behavior:** Direct work command is `/vsdd-factory:decompose-stories`.
**Acceptance:** Command syntax.

---

### 65. phase-3-tdd-implementation

**Skill:** `plugins/vsdd-factory/skills/phase-3-tdd-implementation/SKILL.md` (49 LOC)

### BC-AUDIT-513 — phase-3-tdd-implementation: Identity — per-story TDD delivery via deliver-story sub-workflow

**Skill:** `plugins/vsdd-factory/skills/phase-3-tdd-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 3 entry per story.
**Behavior:** Seven steps A-G per story: create-worktree, generate-stubs, failing-tests (Red Gate), implement, record-demos, pr-lifecycle, cleanup. Workflow runs once per story; orchestrator invokes for each story in wave order.
**Acceptance:** Frontmatter `name: phase-3-tdd-implementation`.

### BC-AUDIT-514 — phase-3-tdd-implementation: Gate — Red Gate passed, all tests pass, demos cover ACs, PR merged, worktree cleaned, drift clean

**Skill:** `plugins/vsdd-factory/skills/phase-3-tdd-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-49 (Gate Criteria)
**Trigger:** Per-story execution complete.
**Behavior:** Red Gate passed (tests failed before implementation); all tests pass; demo evidence covers all ACs; PR merged; worktree cleaned up; input-hash drift check clean.
**Acceptance:** Six gate items.

### BC-AUDIT-515 — phase-3-tdd-implementation: Prerequisites — Phase 2 complete, story status `ready`, all dependency stories completed

**Skill:** `plugins/vsdd-factory/skills/phase-3-tdd-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 36-39 (Prerequisites)
**Trigger:** Per-story dispatch.
**Behavior:** Phase 2 complete (stories approved); story status is `ready` in STORY-INDEX; all dependency stories completed.
**Acceptance:** Three prerequisites.

---

### 66. phase-4-holdout-evaluation

**Skill:** `plugins/vsdd-factory/skills/phase-4-holdout-evaluation/SKILL.md` (39 LOC)

### BC-AUDIT-516 — phase-4-holdout-evaluation: Identity — Phase 4 entry point with scenario rotation + holdout-eval skill

**Skill:** `plugins/vsdd-factory/skills/phase-4-holdout-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 3 wave complete (all wave stories merged).
**Behavior:** Two-step workflow: scenario rotation (orchestrator randomly selects 80% subset of holdout scenarios), then holdout-eval skill runs evaluation with different model family.
**Acceptance:** Frontmatter `name: phase-4-holdout-evaluation`.

### BC-AUDIT-517 — phase-4-holdout-evaluation: Gate — adversary tier (GPT-5.4 not Claude), mean ≥0.85, no must-pass <0.6, std-dev <0.15, 80% rotation

**Skill:** `plugins/vsdd-factory/skills/phase-4-holdout-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-39 (Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Evaluator used different model family (GPT-5.4, not Claude); mean satisfaction score ≥ 0.85; no must-pass scenario below 0.6; satisfaction standard deviation < 0.15; scenario rotation applied (80% subset).
**Acceptance:** Five gate items.

### BC-AUDIT-518 — phase-4-holdout-evaluation: Direct command is /vsdd-factory:holdout-eval

**Skill:** `plugins/vsdd-factory/skills/phase-4-holdout-evaluation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-26
**Trigger:** Direct command invocation.
**Behavior:** Direct work command is `/vsdd-factory:holdout-eval`.
**Acceptance:** Command syntax.

---

### 67. phase-5-adversarial-refinement

**Skill:** `plugins/vsdd-factory/skills/phase-5-adversarial-refinement/SKILL.md` (37 LOC)

### BC-AUDIT-519 — phase-5-adversarial-refinement: Identity — multi-model adversarial loop until novelty=0

**Skill:** `plugins/vsdd-factory/skills/phase-5-adversarial-refinement/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 3 implementation complete + Phase 4 holdout passed.
**Behavior:** Two steps: adversarial review loop (fresh-context review + triage-and-fix, iterated until CONVERGENCE_REACHED) + optional secondary review (code-reviewer / Gemini).
**Acceptance:** Frontmatter `name: phase-5-adversarial-refinement`.

### BC-AUDIT-520 — phase-5-adversarial-refinement: Gate — novelty=0, all findings addressed/accepted, ≥3 clean passes minimum

**Skill:** `plugins/vsdd-factory/skills/phase-5-adversarial-refinement/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-37 (Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Finding novelty decayed to zero (CONVERGENCE_REACHED); all findings addressed or accepted; minimum 3 clean passes.
**Acceptance:** Three gate items including the "minimum 3 clean passes" rule.

### BC-AUDIT-521 — phase-5-adversarial-refinement: Direct command is /vsdd-factory:adversarial-review implementation

**Skill:** `plugins/vsdd-factory/skills/phase-5-adversarial-refinement/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-26
**Trigger:** Direct command invocation.
**Behavior:** Direct work command is `/vsdd-factory:adversarial-review implementation`.
**Acceptance:** Command syntax.

---

### 68. phase-6-formal-hardening

**Skill:** `plugins/vsdd-factory/skills/phase-6-formal-hardening/SKILL.md` (44 LOC)

### BC-AUDIT-522 — phase-6-formal-hardening: Identity — Phase 6 entry point applying 4 verification techniques

**Skill:** `plugins/vsdd-factory/skills/phase-6-formal-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 5 adversarial converged + tools installed.
**Behavior:** Four steps A-D: kani-proofs, fuzz-testing, mutation-testing, security-scan. Direct command `/vsdd-factory:formal-verify`.
**Acceptance:** Frontmatter `name: phase-6-formal-hardening`.

### BC-AUDIT-523 — phase-6-formal-hardening: Gate — all proofs pass, fuzz 5min/target zero crashes, mutation >90%, zero CRIT/HIGH semgrep, cargo audit clean, purity intact

**Skill:** `plugins/vsdd-factory/skills/phase-6-formal-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 36-44 (Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** All Kani proofs pass; fuzz 5 min/target with zero crashes; mutation kill rate >90% (adjusted by criticality); zero critical/high Semgrep findings; cargo audit no known vulnerabilities; purity boundaries intact.
**Acceptance:** Six gate items.

### BC-AUDIT-524 — phase-6-formal-hardening: Prerequisites — cargo-kani, cargo-fuzz, cargo-mutants, semgrep installed

**Skill:** `plugins/vsdd-factory/skills/phase-6-formal-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-34
**Trigger:** Phase entry.
**Behavior:** Verification tools must be installed. (Skill body ties to formal-verify which has same install commands.)
**Acceptance:** Four required tools.

---

### 69. phase-7-convergence

**Skill:** `plugins/vsdd-factory/skills/phase-7-convergence/SKILL.md` (51 LOC)

### BC-AUDIT-525 — phase-7-convergence: Identity — 7-dimensional convergence assessment

**Skill:** `plugins/vsdd-factory/skills/phase-7-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** Phase 6 hardening passed.
**Behavior:** Seven steps A-G: spec-convergence, test-convergence, implementation-convergence, verification-convergence, visual-convergence, performance-convergence, documentation-convergence.
**Acceptance:** Frontmatter `name: phase-7-convergence`. Note: skill description says "7-dimensional" — this is the canonical Phase 7, distinct from F7's 5-dimensional delta convergence.

### BC-AUDIT-526 — phase-7-convergence: Gate — all 7 dimensions CONVERGED, traceability matrix, demo verified by visual-reviewer, drift clean, human approval

**Skill:** `plugins/vsdd-factory/skills/phase-7-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-45 (Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** All 7 dimensions CONVERGED; traceability matrix generated; convergence report generated; input-hash drift check clean; demo recordings verified by visual-reviewer; human approval.
**Acceptance:** Six gate items.

### BC-AUDIT-527 — phase-7-convergence: Outcome — CONVERGED → release; NOT CONVERGED → loop back to Phase 3

**Skill:** `plugins/vsdd-factory/skills/phase-7-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-51 (Outcome)
**Trigger:** Phase complete.
**Behavior:** CONVERGED → ready for release → proceed to `/vsdd-factory:release`. NOT CONVERGED → list remaining items with severity → loop back to Phase 3.
**Acceptance:** Two outcome paths.

---

### 70. phase-f1-delta-analysis

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md` (199 LOC)

### BC-AUDIT-528 — phase-f1-delta-analysis: Identity — analyzes feature request against existing artifacts to determine impact boundary

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Feature Mode entry; F1 dispatch.
**Behavior:** Determines impact boundary, affected specs/stories/tests, regression risk. Produces Delta Analysis Report and affected-files.txt.
**Acceptance:** Frontmatter `name: phase-f1-delta-analysis`.

### BC-AUDIT-529 — phase-f1-delta-analysis: Components classified NEW/MODIFIED/DEPENDENT by architect

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 44-53 (Step 3: Impact Boundary Identification)
**Trigger:** Step 3 runs.
**Behavior:** architect agent maps existing components affected, identifies new components, determines structural vs. internal change, classifies each component as NEW, MODIFIED, or DEPENDENT.
**Acceptance:** Three classification labels.

### BC-AUDIT-530 — phase-f1-delta-analysis: Intent classification — feature, enhancement, bug-fix maps to F1-F7 vs bug-fix route

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 66-77 (Step 4b: Intent Classification)
**Trigger:** Step 4b runs.
**Behavior:** Intent table: feature ("add"/"build"/"new") → Full F1-F7. enhancement ("improve"/"update"/"change") → Full F1-F7 (may be quick dev if trivial). bug-fix ("fix"/"bug"/"broken"/"regression") → Bug fix route (skip F2, F3). Recorded in delta analysis report and STATE.md.
**Acceptance:** Three intent classes; routing rules.

### BC-AUDIT-531 — phase-f1-delta-analysis: Trivial scope — single module + no new BCs + no arch change + no new deps + LOW risk → quick-dev routing

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 79-98 (Step 4c: Trivial Scope Classification)
**Trigger:** Step 4c runs.
**Behavior:** Trivial when ALL true: single module/file/docs only; no new BCs; no architecture change; no new external deps; regression risk LOW. Trivial → quick dev routing (F1 → F4 single story → regression → F7 → PATCH; skips F2/F3/F5/F6). "Quick dev is an optimization, not a bypass — all safety preserved."
**Acceptance:** Five trivial conditions; quick-dev path.

### BC-AUDIT-532 — phase-f1-delta-analysis: Severity (bug-fix only) — CRITICAL triggers expedited flow with skipped baseline/proofs

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 100-124 (Step 4d: Severity Classification)
**Trigger:** Intent is bug-fix.
**Behavior:** Severity table: CRITICAL (production down, data loss, security breach), HIGH (major broken, no workaround), MEDIUM (impaired, workaround exists), LOW (minor, cosmetic, edge). CRITICAL → skip demo baseline, F1 minimal, async human approval, F5 1 round max, F6 security scan only (skip proofs/fuzz/mutation), release immediately, logged in STATE.md as expedited:true with justification, skipped steps queued for follow-up.
**Acceptance:** Four severity classes; expedited flow specifics.

### BC-AUDIT-533 — phase-f1-delta-analysis: Output is delta-analysis.md + affected-files.txt (+ affected-repos.txt for multi-repo)

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 168-175 (Output Artifacts)
**Trigger:** Phase complete.
**Behavior:** Writes `.factory/phase-f1-delta-analysis/delta-analysis.md`, `.factory/phase-f1-delta-analysis/affected-files.txt`, and `.factory/phase-f1-delta-analysis/affected-repos.txt` (multi-repo only).
**Acceptance:** Three output paths.

### BC-AUDIT-534 — phase-f1-delta-analysis: Quality Gate — feature_type, intent, scope, severity (if bug-fix), BC-S.SS.NNN refs, multi-repo, human-approved

**Skill:** `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 174-186 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** All affected components classified; regression risk assessed; existing tests in risk zone enumerated; files NOT changed listed as baseline; feature type classified (ui|backend|full-stack|infrastructure); intent classified; trivial scope assessed; severity classified if bug-fix; uses BC-S.SS.NNN identifiers (no FR-NNN); multi-repo affected repos + contract changes identified; human explicitly approved.
**Acceptance:** Eleven Quality Gate items.

---

### 71. phase-f2-spec-evolution

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md` (185 LOC)

### BC-AUDIT-535 — phase-f2-spec-evolution: Identity — incremental spec updates (PRD + arch + VPs), delta only

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F1 delta analysis approved.
**Behavior:** Updates specs incrementally — PRD, architecture, verification properties — delta only, not full rewrite.
**Acceptance:** Frontmatter `name: phase-f2-spec-evolution`.

### BC-AUDIT-536 — phase-f2-spec-evolution: PRD delta appends new BCs continuing BC-S.SS.NNN sequence; modified BCs marked UPDATED with previous version inline

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-43 (Step 2: PRD Delta), 165-167 (Quality Gate)
**Trigger:** Step 2 runs.
**Behavior:** product-owner appends new BCs using BC-S.SS.NNN format (DF-020 4-level hierarchy). Continues numbering. Modifies existing BCs with UPDATED tag and previous version inline. Does NOT rewrite or restructure unaffected requirements. Writes `.factory/phase-f2-spec-evolution/prd-delta.md`.
**Acceptance:** Append-only with UPDATED tag and inline previous version.

### BC-AUDIT-537 — phase-f2-spec-evolution: UX delta + accessibility review run only when feature_type ∈ ['ui', 'full-stack']

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-81 (Step 2d, 2e)
**Trigger:** Feature type check.
**Behavior:** Step 2d (ux-designer) and Step 2e (accessibility-auditor) only run when `feature_type in ['ui', 'full-stack']`. Information asymmetry wall: accessibility-auditor cannot see architecture or implementation plans.
**Acceptance:** Conditional UI gating.

### BC-AUDIT-538 — phase-f2-spec-evolution: Spec version bump per semver — MAJOR/MINOR/PATCH

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 109-117 (Step 5)
**Trigger:** Step 5 runs.
**Behavior:** MAJOR (architectural rework, removed features, breaking changes), MINOR (new features, new requirements — most common for Feature Mode), PATCH (wording fixes, edge cases, clarifications). Update PRD frontmatter version + write changelog entry.
**Acceptance:** Three semver tiers + changelog entry.

### BC-AUDIT-539 — phase-f2-spec-evolution: Adversary reviews ONLY the delta (PRD + arch + VP + UX), not unchanged sections

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-130 (Step 6)
**Trigger:** Step 6 runs.
**Behavior:** Adversary (fresh context) reviews ONLY: PRD delta, architecture delta (if any), VP extensions, UX delta (if UI feature). Does NOT review unchanged sections. Provide only delta documents and surrounding context for coherence.
**Acceptance:** Delta-scoped adversarial review.

### BC-AUDIT-540 — phase-f2-spec-evolution: Quality Gate — BC-S.SS.NNN format, append-only, acyclic deps, version bumped, changelog written, adversary cosmetic only, human approved

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 164-178 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** New BCs use BC-S.SS.NNN; new requirements continue ID sequence (no gaps, no collisions); modified retain previous version inline; architecture dependency graph remains acyclic; new VPs have proof strategies; spec version bumped per semver; changelog entry written; adversary findings on delta cosmetic only; DTU re-assessment done if new external deps; gene transfusion assessment done if reference impl exists; UX delta + accessibility review for UI features; human approved.
**Acceptance:** Twelve Quality Gate items.

### BC-AUDIT-541 — phase-f2-spec-evolution: Failure — missing F1 output halts F2; CRITICAL after 3 rounds escalates to human

**Skill:** `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 180-185 (Failure Modes)
**Trigger:** F1 output missing or adversary finds CRITICAL after 3 rounds.
**Behavior:** Missing F1 delta-analysis.md → stop and report to orchestrator; F2 cannot proceed without F1. Conflicts with existing BCs → flag with before/after comparison and escalate to product-owner. Adversary CRITICAL after 3 rounds → escalate to human with unresolved findings.
**Acceptance:** Three failure modes.

---

### 72. phase-f3-incremental-stories

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md` (148 LOC)

### BC-AUDIT-542 — phase-f3-incremental-stories: Identity — adds new stories integrated into existing dependency graph without cycles

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F2 complete and human-approved.
**Behavior:** Creates new stories for the feature; integrates them into existing dependency graph without cycles.
**Acceptance:** Frontmatter `name: phase-f3-incremental-stories`.

### BC-AUDIT-543 — phase-f3-incremental-stories: Story IDs continue existing sequence; per-file STORY-NNN.md, not monolithic

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-46 (Step 2), 134-136 (Quality Gate)
**Trigger:** Story creation step.
**Behavior:** Story-writer continues ID sequence (if last is STORY-005, new start STORY-006). Each story is a separate per-file STORY-NNN.md (not monolithic). Each story references new/modified BCs (BC-S.SS.NNN), VPs (VP-NNN), testable AC, module criticality, implementation strategy (tdd or gene-transfusion).
**Acceptance:** Five required references per story; per-file storage.

### BC-AUDIT-544 — phase-f3-incremental-stories: Cycle detection via Kahn's algorithm topological sort

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-66 (Step 4: Cycle Detection)
**Trigger:** Step 4 runs.
**Behavior:** Build adjacency list (existing + new stories). Run topological sort (Kahn's algorithm). If sort completes: no cycles, proceed. If fails: identify cycle, report to human. Common fixes: merge stories, split differently, remove dependency.
**Acceptance:** Algorithm name; failure handling.

### BC-AUDIT-545 — phase-f3-incremental-stories: DTU clones stories placed in Wave 1; gene transfusion stories flagged

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 86-92 (Step 6b)
**Trigger:** F2 flagged DTU re-assessment or gene transfusion.
**Behavior:** DTU clone stories in Wave 1 (P0 priority) — external service mocks must exist before dependent implementation stories. Gene transfusion stories flagged with `implementation_strategy: gene-transfusion` so F4 uses Semport translation (not "implement from scratch").
**Acceptance:** Wave-1 placement; strategy flag.

### BC-AUDIT-546 — phase-f3-incremental-stories: Quality Gate — IDs continue, per-file, BC-S.SS.NNN, testable AC, VP-NNN, no cycles, append-only, wave schedule + holdouts, conflicts resolved, human approved

**Skill:** `plugins/vsdd-factory/skills/phase-f3-incremental-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 132-148 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Story IDs continue (no collisions); per-file STORY-NNN.md (not monolithic); references BCs (BC-S.SS.NNN format); testable AC; VP-NNN references; topological sort succeeds; deps extended without modifying existing; wave schedule computed; wave holdout scenarios per wave; DTU clone stories in Wave 1 if flagged; gene transfusion stories flagged; conflicts identified and resolved; effort estimated with critical path; human approved.
**Acceptance:** Fourteen Quality Gate items.

---

### 73. phase-f4-delta-implementation

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md` (176 LOC)

### BC-AUDIT-547 — phase-f4-delta-implementation: Identity — TDD scoped to new stories with full regression as safety net

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F3 complete and human-approved.
**Behavior:** TDD implementation scoped to new stories only. Full existing test suite (regression) is the safety net. Per-story delivery (DF-024) organized in waves.
**Acceptance:** Frontmatter `name: phase-f4-delta-implementation`.

### BC-AUDIT-548 — phase-f4-delta-implementation: Establish regression baseline before any new code; if any fail, STOP

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 23-44 (Step 1: Establish Regression Baseline)
**Trigger:** Step 1 runs.
**Behavior:** Run FULL existing test suite before any new code. Record total tests, passing, failing (must be 0; if any fail, STOP and fix regressions before proceeding), timestamp. This baseline is the contract: every test must still pass after implementation.
**Acceptance:** STOP-on-failure rule; baseline recording.

### BC-AUDIT-549 — phase-f4-delta-implementation: Per-story delivery uses code-delivery.lobster sub-workflow with 11 stages

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-75 (Step 2b: Per-Story)
**Trigger:** Per-story execution.
**Behavior:** 11 stages: stubs (Two-Step Red Gate), tests (Red Gate: compile + fail), implementation (TDD or Semport gene-transfusion), E2E tests (if UI/full-stack), demo (per-AC), Push+PR (squash, force-with-lease), AI review (pr-reviewer Gemini, info asymmetry), security review (CRIT/HIGH only), converge (max 10 cycles), merge (deps check), cleanup (devops-engineer removes worktree).
**Acceptance:** Eleven stages.

### BC-AUDIT-550 — phase-f4-delta-implementation: Wave Integration Gate — full tests + adversary + security + holdout + a11y + demo + fix loop max 10

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-86 (Step 2c: Wave Integration Gate)
**Trigger:** All stories in wave merge to develop.
**Behavior:** Full test suite (ALL existing + all prior waves), adversary review of combined wave diff (info asymmetry), security review (if wave has CRIT/HIGH stories), holdout regression on wave scenarios, accessibility audit (if UI), wave-level integration demo, fix loop (max 10 cycles).
**Acceptance:** Six gate dimensions; max-10 fix loop.

### BC-AUDIT-551 — phase-f4-delta-implementation: Regression failure — fix the implementation, not the test

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 102-112 (Step 6: Run Regression Suite)
**Trigger:** Regression detected.
**Behavior:** "Log to regression-log.md, identify which implementation change caused the regression, fix the implementation (not the test), re-run until clean." Hook `plugins/src/regression-gate.ts` automates regression detection.
**Acceptance:** Implementation-not-test rule.

### BC-AUDIT-552 — phase-f4-delta-implementation: Quality Gate — regression baseline + Two-Step Red Gate + full regression pass + reviewer + security if CRIT/HIGH + max 10 + wave gate + E2E for UI + no out-of-scope edits + summary

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 152-169 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Regression baseline recorded with commit SHA; per-story flow worktree→tests→implement→demo→PR→review→merge; Two-Step Red Gate (stubs first, then tests); all new tests pass (Green Gate); full regression passes (zero regressions); pr-reviewer (Gemini) reviewed each story PR; security-reviewer engaged for CRIT/HIGH; max 10 review rounds per story; wave integration gate; gate includes adversary+security+holdout+a11y if UI; E2E tests for UI/full-stack; existing conventions followed; no edits outside delta scope; implementation summary written with deviation log; all PRs merged.
**Acceptance:** Fifteen Quality Gate items.

### BC-AUDIT-553 — phase-f4-delta-implementation: No human gate — automated quality gate

**Skill:** `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 130-132
**Trigger:** Phase complete.
**Behavior:** "Phase F4 is COMPLETE when all new tests pass AND the full regression suite passes. No human gate — this is an automated quality gate."
**Acceptance:** Automated, no human approval required.

---

### 74. phase-f5-scoped-adversarial

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md` (188 LOC)

### BC-AUDIT-554 — phase-f5-scoped-adversarial: Identity — adversary reviews only delta files, fresh context, different model family

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F4 complete (all tests green, regression clean).
**Behavior:** Adversarial review scoped to changed/new code only. Fresh context, different model family.
**Acceptance:** Frontmatter `name: phase-f5-scoped-adversarial`.

### BC-AUDIT-555 — phase-f5-scoped-adversarial: Review package excludes prior reviews, implementation rationale, semport, red-gate logs

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 25-44 (Step 1: Prepare Review Package)
**Trigger:** Step 1 runs.
**Behavior:** Includes only changed files (full file, not just diff) + new test files + relevant spec sections + story specs + conventions docs. Excludes (info asymmetry walls, DF-025): unchanged source unless direct dependents; previous adversarial review reports; implementation notes/rationale; Phase F4 TDD logs and implementer session notes; .factory/semport/** (gene transfusion history, DF-028); .factory/cycles/**/implementation/red-gate-log*, implementer-notes*.
**Acceptance:** Six exclusion classes.

### BC-AUDIT-556 — phase-f5-scoped-adversarial: 5 review categories — spec fidelity, regression risk, convention, security, test quality

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-83 (Step 3: Adversary Review Categories)
**Trigger:** Adversary spawned with review package.
**Behavior:** Adversary evaluates: 3a Spec Fidelity, 3b Regression Risk, 3c Convention Adherence, 3d Security Review, 3e Test Quality.
**Acceptance:** Five named categories.

### BC-AUDIT-557 — phase-f5-scoped-adversarial: Severity scale CRITICAL/HIGH/MEDIUM/LOW/COSMETIC; convergence at novelty < 0.15 AND no CRIT/HIGH

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 87-114 (Steps 4-6)
**Trigger:** Findings issued.
**Behavior:** Severity: CRITICAL/HIGH/MEDIUM/LOW/COSMETIC. Category: spec-fidelity/regression-risk/convention/security/test-quality. CRITICAL/HIGH must be fixed; MEDIUM Orchestrator decides; LOW/COSMETIC documented but not blocking. Convergence: all findings MEDIUM or below AND novelty score < 0.15.
**Acceptance:** Five severity tiers; convergence formula.

### BC-AUDIT-558 — phase-f5-scoped-adversarial: Secondary review (Gemini/review-tier) optional for security-critical or large delta

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 116-126 (Step 7)
**Trigger:** Primary adversary converges.
**Behavior:** Optional secondary using review/primary (review-tier model) for cognitive diversity. Recommended for security-critical delta or large delta (Gemini's 1M context). Not for trivial bug fixes or cosmetic changes. Secondary findings additive — they extend, not replace.
**Acceptance:** Conditional secondary; additive (not replacement).

### BC-AUDIT-559 — phase-f5-scoped-adversarial: Output convergence-summary.md; F5 fixes through code-delivery.lobster as FIX-F5-NNN

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 128-144 (Step 8 + Fix PR Delivery)
**Trigger:** Phase complete.
**Behavior:** Writes `.factory/phase-f5-adversarial/convergence-summary.md` (rounds per model, findings by severity initial vs final, novelty per round, cross-model unique findings, final verdict CONVERGED/NOT-CONVERGED). F5 fixes go through code-delivery.lobster: FIX-F5-NNN → worktree → fix → demo (if behavior-changing) → PR → AI review → security review → merge → re-verify only failing checks.
**Acceptance:** Convergence summary fields; fix PR routing.

### BC-AUDIT-560 — phase-f5-scoped-adversarial: Quality Gate — delta scope only, fresh context, different model family, all CRIT/HIGH resolved, novelty < 0.15, regression still passes

**Skill:** `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 177-187 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Review scoped to delta files; primary adversary uses different model family; fresh context per round (no carryover); multi-repo contract compliance; all CRITICAL/HIGH resolved; convergence reached (novelty < 0.15); secondary pass for security-critical deltas; regression suite still passes.
**Acceptance:** Eight Quality Gate items.

---

### 75. phase-f6-targeted-hardening

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md` (172 LOC)

### BC-AUDIT-561 — phase-f6-targeted-hardening: Identity — Kani+fuzz+mutation scoped to delta; regression+security on full tree

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F5 converged.
**Behavior:** Targeted hardening — Kani proofs, fuzz testing, mutation testing scoped to delta. Full regression and dependency audit scans on full tree.
**Acceptance:** Frontmatter `name: phase-f6-targeted-hardening`.

### BC-AUDIT-562 — phase-f6-targeted-hardening: Hardening scope per-tool varies — Kani/fuzz/mutation/Semgrep delta; regression+audit full tree

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 23-35 (Step 1: Determine Hardening Scope)
**Trigger:** Step 1 runs.
**Behavior:** Scope table: Kani proofs → new/modified modules only; Fuzz testing → new code paths only; Mutation testing → changed files only; Semgrep → changed + new files; Regression tests → full existing test suite; cargo audit / npm audit → full dependency tree.
**Acceptance:** Per-tool scope table.

### BC-AUDIT-563 — phase-f6-targeted-hardening: Mutation kill rate ≥90% on changed files (≥95% for security-critical)

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 65-82 (Step 4: Mutation Testing)
**Trigger:** Step 4 runs.
**Behavior:** Target ≥ 90% kill rate for changed files (≥ 95% for security-critical modules). If below threshold: identify survivors, write tests, re-run.
**Acceptance:** Tiered threshold (90/95).

### BC-AUDIT-564 — phase-f6-targeted-hardening: Information asymmetry wall — formal-verifier cannot see F5 adversarial findings

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 144-148 (Information Asymmetry Wall)
**Trigger:** formal-verifier dispatch.
**Behavior:** "The formal-verifier CANNOT see adversarial review findings from F5 (DF-025). This ensures independent verification — the formal-verifier should verify properties from the spec, not guided by what the adversary looked for."
**Acceptance:** Wall enforced; rationale explicit.

### BC-AUDIT-565 — phase-f6-targeted-hardening: Quality Gate — proofs pass, fuzz clean, mutation 90% (95% critical), no CRIT/HIGH, regression passes, DTU adversarial if external svc, a11y if UI, FIX-F6-NNN via code-delivery, partial re-verification

**Skill:** `plugins/vsdd-factory/skills/phase-f6-targeted-hardening/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 161-172 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** Kani proofs pass for new VPs (or justified skip); fuzz clean after 5 min/target (or skip); mutation kill ≥90% (≥95% critical); zero CRITICAL/HIGH security findings (CRIT/HIGH → BLOCK for human); full regression suite passes; DTU adversarial testing if external service interaction changed; a11y re-check if UI feature; fix PRs via code-delivery.lobster (FIX-F6-NNN); F6 re-verifies only failing checks after fix (partial re-verification); hardening summary with all metrics.
**Acceptance:** Ten Quality Gate items.

---

### 76. phase-f7-delta-convergence

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md` (180 LOC)

### BC-AUDIT-566 — phase-f7-delta-convergence: Identity — 5-dimensional convergence on delta + full regression validation; final human gate

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase F6 complete (all checks pass).
**Behavior:** Five-dimensional convergence check on the delta + regression validation on full codebase. Final human gate.
**Acceptance:** Frontmatter `name: phase-f7-delta-convergence`.

### BC-AUDIT-567 — phase-f7-delta-convergence: 5 dimensions — Spec novelty<0.15, Test mutation≥90%, Impl verification rate<60%, Verification all-pass, Holdout≥0.85

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-56 (Step 1)
**Trigger:** Step 1 runs.
**Behavior:** Dim 1 Spec — adversary novelty score < 0.15 on spec delta. Dim 2 Test — mutation kill rate ≥ 90% on changed files; no vacuously true tests. Dim 3 Implementation — adversary verification rate < 60% (hallucinating flaws); no CRITICAL/HIGH open. Dim 4 Verification — all Kani proofs pass; fuzz clean (5 min/target); no security vulns; purity intact. Dim 5 Holdout — mean ≥ 0.85; no must-pass < 0.6; regression holdout still passes.
**Acceptance:** Five dimensions with specific metric and threshold.

### BC-AUDIT-568 — phase-f7-delta-convergence: Regression validation is binary pass/fail, not "convergence"

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 58-66 (Step 2: Regression Validation)
**Trigger:** Step 2 runs.
**Behavior:** Run complete suite, compare against Phase F4 baseline. Verify zero regressions. "This is not 'convergence' — it is a binary pass/fail."
**Acceptance:** Binary semantics.

### BC-AUDIT-569 — phase-f7-delta-convergence: Cost-benefit — flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost > P(finding) * Value_avg / 1.5

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 68-75 (Step 2b: Cost-Benefit Analysis)
**Trigger:** Step 2b runs.
**Behavior:** Include cost-benefit data from cost-tracker (DF-027): total cost, cost per dimension, projected cost of additional cycles. Compare P(finding in next iteration) * Value_avg vs Cost_iteration * 1.5. Flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost exceeds expected value.
**Acceptance:** Specific formula; flag name.

### BC-AUDIT-570 — phase-f7-delta-convergence: Traceability chain extended (append, not replace) with new BC→VP→test→src→ADV→KANI links

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 77-95 (Step 3: Traceability Chain Extension)
**Trigger:** Step 3 runs.
**Behavior:** For each new requirement: BC-S.SS.NNN → VP-NNN → test_xxx → src/xxx.rs → ADV-PASS-N → KANI-xxx-PASS. For cross-references: BC-S.SS.NNN depends_on BC-S.SS.MMM (existing); STORY-XXX extends STORY-YYY. Update main traceability chain by APPENDING (not replacing).
**Acceptance:** Chain format; append-only rule.

### BC-AUDIT-571 — phase-f7-delta-convergence: Final human authorization gate; failure routes to specific phase

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 137-160 (Steps 5 + Convergence Failure Routing)
**Trigger:** Convergence report ready.
**Behavior:** Human must explicitly authorize merge. If sent back: identify which phase to re-execute. Spec/Impl not converged → F5; Tests not converged → F6; Holdout not converged → re-evaluation. Fix PRs through code-delivery.lobster. After approval, release: MINOR or PATCH bump → CHANGELOG → tag → gh release → publish.
**Acceptance:** Routing rules per failing dimension; release ordering.

### BC-AUDIT-572 — phase-f7-delta-convergence: Quality Gate — all 5 dims pass, regression passes, traceability extended, cost-benefit included, max 5 cycles, FIX-F7-NNN, human authorized

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 168-179 (Quality Gate Criteria)
**Trigger:** Phase complete.
**Behavior:** All 5 convergence dimensions pass on delta; full regression passes (zero regressions); traceability extended (not replaced); cross-references link new to existing features; delta convergence report written with all metrics; cost-benefit analysis included (DF-027); max 5 convergence cycles with cost-benefit escalation; fix PRs via code-delivery.lobster (FIX-F7-NNN); human explicitly authorized merge.
**Acceptance:** Nine Quality Gate items.

### BC-AUDIT-573 — phase-f7-delta-convergence: Max 5 convergence cycles before cost-benefit escalation

**Skill:** `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 174 (Quality Gate)
**Trigger:** Convergence loop iterations.
**Behavior:** "Max 5 convergence cycles with cost-benefit escalation."
**Acceptance:** Cycle cap at 5.

---

### 77. planning-research

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md` (66 LOC)

### BC-AUDIT-574 — planning-research: Identity — domain/market/technical research via Perplexity + Context7

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** After brainstorming; before brief; before PRD; or human says "research X before we proceed".
**Behavior:** Conducts market, domain, technical research to validate assumptions and fill knowledge gaps. Uses Perplexity for web research and Context7 for library/framework documentation. Can run domain, market, or technical research independently or combined.
**Acceptance:** Frontmatter `name: planning-research`.

### BC-AUDIT-575 — planning-research: Cross-reference findings across ≥2 independent sources; date-stamp all findings

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-48 (Research Process)
**Trigger:** Research execution.
**Behavior:** Cross-reference findings across at least 2 independent sources. Date-stamp all findings (technology landscapes change rapidly). Follow AGENTS.md query construction rules. Flag uncertainties.
**Acceptance:** ≥2 sources; all findings date-stamped.

### BC-AUDIT-576 — planning-research: Output is research-report.md + research-sources.md

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 63-66 (Output Artifacts)
**Trigger:** Research complete.
**Behavior:** Writes `.factory/planning/research-report.md` (structured findings per type) and `.factory/planning/research-sources.md` (citations and links).
**Acceptance:** Two output files.

### BC-AUDIT-577 — planning-research: Quality Gate — sources cited with URLs+dates, uncertainties flagged, ≥2 sources cross-referenced

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 50-55 (Quality Gate)
**Trigger:** Report generation.
**Behavior:** Research report with structured findings; all findings cite specific sources with URLs and dates; inconclusive areas flagged; cross-referenced ≥ 2 independent sources where possible.
**Acceptance:** Four checklist items.

### BC-AUDIT-578 — planning-research: Failure — MCP tools unavailable → use training data with explicit "UNVERIFIED" disclaimer

**Skill:** `plugins/vsdd-factory/skills/planning-research/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 57-61 (Failure Modes)
**Trigger:** Perplexity or Context7 unavailable.
**Behavior:** Use training data with explicit "UNVERIFIED — based on training data, not live research" disclaimer. No relevant results → document gap and recommend alternative research approaches. Sources contradict each other → present both sides with evidence strength assessment.
**Acceptance:** Disclaimer required when fallback to training data.

---

### 78. policy-add

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md` (92 LOC)

### BC-AUDIT-579 — policy-add: Identity — register new governance policy in .factory/policies.yaml with sequential ID

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** User invokes `/vsdd-factory:policy-add <policy-name>`.
**Behavior:** Validates policy name (snake_case), gathers details (description, severity HIGH/MEDIUM, adopted context, enforced_by, scope, lint_hook, verification_steps), assigns next sequential ID, appends entry, runs validation.
**Acceptance:** Frontmatter `name: policy-add`; `argument-hint: "<policy-name>"`.

### BC-AUDIT-580 — policy-add: Policy name must be snake_case and unique across registry

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 17-22 (Step 1), 86-87 (Validation Rules)
**Trigger:** Step 1 runs.
**Behavior:** Must be snake_case (lowercase, underscores, no spaces, no hyphens). Must not already exist (check `name` field). Invalid or duplicate → report error and stop.
**Acceptance:** Format and uniqueness rule; stop-on-violation.

### BC-AUDIT-581 — policy-add: Severity HIGH or MEDIUM only; HIGH violations block convergence

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-32 (Step 2.2)
**Trigger:** Severity prompt.
**Behavior:** Severity is `HIGH` or `MEDIUM` only. HIGH: violations block convergence. MEDIUM: violations are findings but may not block alone.
**Acceptance:** Two-value enum.

### BC-AUDIT-582 — policy-add: Enforced_by + scope each must have ≥1 entry; custom policies must include verification_steps

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 34-44 (Step 2.4-2.7), 88-91 (Validation Rules)
**Trigger:** Step 2 runs.
**Behavior:** enforced_by from {adversary-prompt, consistency-validator, lint-hook, orchestrator-rule} ≥1 entry. scope from {bc, vp, di, cap, story, hs, architecture, prd, nfr} ≥1 entry. Baseline policies (1-9) have steps in agent prompts; custom policies MUST include verification_steps so adversary knows HOW to verify.
**Acceptance:** Both fields ≥1; custom-policy steps requirement.

### BC-AUDIT-583 — policy-add: Output appends to policies.yaml; runs validate after; reports next steps

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 49-83 (Steps 4-6)
**Trigger:** Validation passes.
**Behavior:** Appends YAML block. Runs `/vsdd-factory:policy-registry validate` to confirm. Reports ID, name, severity, enforced_by, and next steps (implement lint hook, document rationale).
**Acceptance:** Append-only operation.

### BC-AUDIT-584 — policy-add: Prerequisite — policies.yaml must exist; otherwise run policy-registry init first

**Skill:** `plugins/vsdd-factory/skills/policy-add/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 11-14 (Prerequisites)
**Trigger:** Skill invoked.
**Behavior:** ".factory/policies.yaml must exist. If it doesn't, run `/vsdd-factory:policy-registry init` first."
**Acceptance:** Prerequisite ordering.

---

### 79. policy-registry

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md` (110 LOC)

### BC-AUDIT-585 — policy-registry: Identity — view/validate/manage governance policy registry

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-5
**Trigger:** User invokes `/vsdd-factory:policy-registry [list|validate|show <name>|init]`.
**Behavior:** Lists active policies, checks enforcement coverage, verifies lint hooks exist. Four commands: list, validate, show <policy-name>, init.
**Acceptance:** Frontmatter `name: policy-registry`; `argument-hint: "[list|validate|show <policy-name>|init]"`.

### BC-AUDIT-586 — policy-registry: Init copies template + populates 9 baseline governance policies

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 31-39 (Init), 75-89 (Baseline Policies)
**Trigger:** `policy-registry init`.
**Behavior:** If `.factory/policies.yaml` already exists → ask user overwrite or merge. If overwrite: replace with baseline. If merge: add baseline policies not already present (match by name). If no: copy `${CLAUDE_PLUGIN_ROOT}/templates/policies-template.yaml` and populate with 9 baseline policies. Reports init status.
**Acceptance:** 9 baseline policies (append_only_numbering, lift_invariants_to_bcs, state_manager_runs_last, semantic_anchoring_integrity, creators_justify_anchors, architecture_is_subsystem_name_source_of_truth, bc_h1_is_title_source_of_truth, bc_array_changes_propagate_to_body_and_acs, vp_index_is_vp_catalog_source_of_truth).

### BC-AUDIT-587 — policy-registry: Validate checks ID/name uniqueness, snake_case, severity ∈ {HIGH,MEDIUM}, lint_hook exists+executable, scope ∈ allowed types

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-67 (Validate)
**Trigger:** `policy-registry validate`.
**Behavior:** Parse YAML; if invalid syntax → report and stop. Verify per policy: ID uniqueness, name uniqueness, name format snake_case, required fields non-empty (id, name, description, severity, enforced_by), severity HIGH or MEDIUM, lint_hook exists at `${CLAUDE_PLUGIN_ROOT}/<lint_hook>` and is executable, scope from allowed list (bc, vp, di, cap, story, hs, architecture, prd, nfr). Report PASS/FAIL per policy with remediation.
**Acceptance:** Six validation checks.

### BC-AUDIT-588 — policy-registry: List shows summary table with #/Policy/Severity/Enforced By/Lint Hook

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-52 (List)
**Trigger:** `policy-registry list` (default).
**Behavior:** Reads `.factory/policies.yaml`. If file doesn't exist → "No policy registry found. Run `/vsdd-factory:policy-registry init` to create one." Otherwise summary table.
**Acceptance:** Default-to-list behavior; missing-file message.

### BC-AUDIT-589 — policy-registry: Adversarial review auto-loads policies.yaml as rubric

**Skill:** `plugins/vsdd-factory/skills/policy-registry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 92-105 (Integration with Adversarial Review)
**Trigger:** Adversary dispatch.
**Behavior:** Orchestrator reads `.factory/policies.yaml` at dispatch — if missing, warn but continue (policies also in agent prompts). For each policy, format as rubric item with id, name, description, severity, scope. Append to adversary's task prompt under "## Project Policy Rubric" heading. Adversary verifies each policy as a review axis and reports compliance per-policy. "Why both? Agent prompts carry the enforcement logic (HOW); the registry carries the catalog (WHAT)."
**Acceptance:** Auto-load procedure; rubric format; warn-not-fail on missing.

---

### 80. post-feature-validation

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md` (213 LOC)

### BC-AUDIT-590 — post-feature-validation: Identity — monitors post-ship feedback at 7/30/90-day intervals; entirely optional

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** Scheduled (7d/30d/90d after feature ships) or manual ("Check how [feature] is doing").
**Behavior:** Monitors feedback channels and analytics for shipped feature reception. Closes the feedback loop: Build → Deploy → Customers → Feedback → Analysis → Next Iteration. Configurable intervals (default 7, 30, 90). Entirely optional — no-op if not configured.
**Acceptance:** Frontmatter `name: post-feature-validation`.

### BC-AUDIT-591 — post-feature-validation: Verdict thresholds — SUCCESS / PARTIAL / MISS based on adoption + feedback ratio + bugs

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 109-117 (Step 6)
**Trigger:** Step 6 runs.
**Behavior:** SUCCESS = adoption meets target AND positive ratio meets target AND no critical bugs. PARTIAL = adoption meets but feedback mixed, OR good feedback but low adoption. MISS = adoption below target AND/OR negative dominant AND/OR critical bugs.
**Acceptance:** Three-tier verdict rubric.

### BC-AUDIT-592 — post-feature-validation: Default success criteria — adoption ≥0.10, positive ratio ≥0.6, error rate <5%

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 41-50 (Configuration), 100-108 (Step 5)
**Trigger:** Configuration.
**Behavior:** Default config: enabled true; check_intervals [7, 30, 90]; success_criteria adoption_rate 0.10 (10% of active users); positive_ratio 0.60 (60% positive feedback). Step 5 metrics: adoption rate, positive feedback ratio, error rate <5%, bug reports trend.
**Acceptance:** Default thresholds.

### BC-AUDIT-593 — post-feature-validation: Output is feature-impact-[name]-YYYY-MM-DD.md with adoption + feedback + verdict + recommendations

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 137-184 (Step 8: Produce Feature Impact Report)
**Trigger:** Step 8 runs.
**Behavior:** Writes `.factory/discovery/feature-impact-[feature-name]-YYYY-MM-DD.md` with frontmatter (document_type, feature, shipped_date, report_date, days_since_ship, check_interval) + adoption table + feedback table + issues table + verdict + recommendations + feedback to discovery engine.
**Acceptance:** Output file path; structured sections.

### BC-AUDIT-594 — post-feature-validation: Feeds back into discovery — calibration data, new pain points, evidence for backlog

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 186-195 (Feeding Back Into Discovery)
**Trigger:** Report produced.
**Behavior:** (1) Scoring calibration: features with high discovery scores that consistently MISS → adjust scoring weights. (2) New signals: pain points from impact report → inputs to next feedback ingestion. (3) Evidence for backlog: users requesting feature X after feature Y ships → strengthens evidence for X.
**Acceptance:** Three feedback paths.

### BC-AUDIT-595 — post-feature-validation: Quality Gate — only runs when enabled, evidence-based verdict, actionable recommendations, results feed back

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 197-207 (Quality Gate)
**Trigger:** Skill complete.
**Behavior:** Only runs when `post_feature_validation.enabled == true`. Check interval matches schedule. Feedback channels searched for feature-specific signals. Analytics checked for adoption (if available). Verdict evidence-based (SUCCESS/PARTIAL/MISS with rationale). Recommendations actionable (not just "do better"). Results feed back into discovery for next cycle. Entirely optional — no-op if not configured.
**Acceptance:** Eight Quality Gate items.

### BC-AUDIT-596 — post-feature-validation: Failure — feedback channels unavailable → analyze available data and note unreachable channels

**Skill:** `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 209-213 (Failure Modes)
**Trigger:** Channels unavailable.
**Behavior:** Analyze available data and note which channels were unreachable. No analytics → skip adoption metrics, qualitative only. Feature unidentifiable in feedback → note insufficient signal, recommend manual review.
**Acceptance:** Three failure modes.

---

## 3. Cross-skill recurring patterns

| Pattern | Skills demonstrating | Source observation |
|---|---|---|
| Information asymmetry walls (DF-025) | holdout-eval, multi-repo-phase-0-synthesis, phase-f5-scoped-adversarial, phase-f6-targeted-hardening, phase-f2-spec-evolution (a11y) | Five phases enforce specific exclusions. Walls are explicit, not implicit. |
| Fresh context per adversarial round | phase-1d-adversarial-spec-review, phase-f2-spec-evolution, phase-f5-scoped-adversarial | Stated as Quality Gate item: "Fresh context for each review round (no carryover)." |
| Different model family for adversary | phase-1d-adversarial-spec-review, holdout-eval (GPT-5.4 not Claude), phase-f5-scoped-adversarial, phase-4-holdout-evaluation | model-routing skill anchors the rule: adversary tier MUST be GPT-5.4. |
| Append-only ID sequencing | phase-f2-spec-evolution, phase-f3-incremental-stories, policy-add | BC-S.SS.NNN, STORY-NNN, policy id all append-only with no gap-fills allowed. |
| Quality gate with explicit "human approval" | phase-0-codebase-ingestion, phase-1-spec-crystallization, phase-2-story-decomposition, phase-7-convergence, phase-f1-delta-analysis, phase-f2-spec-evolution, phase-f3-incremental-stories, phase-f7-delta-convergence, market-intelligence-assessment, multi-repo-phase-0-synthesis | Phase-skills have human approval as the gate; F4/F5/F6 are automated (no human gate). |
| Output is .factory/<phase-or-domain>/<artifact>.md | All 30+ work skills | Standardized output paths. Reference skills (jira, mode-decision-guide, model-routing, feature-mode-scoping-rules) have no outputs. |
| Max-N convergence cycle bounds | fix-pr-delivery (10), phase-f4-delta-implementation (10), phase-f5-scoped-adversarial (until convergence), phase-f7-delta-convergence (5) | Different caps for different phases. |
| Severity tier C/H/M/L/COSMETIC | phase-1d-adversarial-spec-review, phase-f5-scoped-adversarial, maintenance-sweep | Common 5-tier severity scale across review skills. |
| Conditional UI-feature steps | phase-f2-spec-evolution (UX delta + a11y), phase-f4-delta-implementation (E2E + a11y), phase-f6-targeted-hardening (a11y re-check), multi-variant-design (entire skill) | Gated by `feature_type in ['ui', 'full-stack']`. |

---

## 4. Observations

### Observation 1: Reference-only skills are sparse but identifiable

Four of the 40 skills declare themselves "Reference document — no quality gate":
- `feature-mode-scoping-rules` (line 136)
- `mode-decision-guide` (line 149)
- `model-routing` (line 67)
- `jira` (frontmatter `disable-model-invocation: true`, "Reference-only skill, not directly invokable")

Each got 2-5 BCs as required by the skill density rule (1 identity + 1+ behavioral).

### Observation 2: Phase-entry skills are thin shells over sub-workflows

Most phase-X and phase-fX skills are entry points that delegate to a `.lobster` sub-workflow file. Their SKILL.md is a table of steps + Quality Gate criteria. The actual procedural detail lives in the sub-workflow lobster files (which are out-of-scope for this batch but referenced via BC-AUDIT-503 / 511 / 514 / 517 / 520 / 523 / 526).

This is a deliberate architecture: SKILL.md = phase contract + gate; .lobster = execution graph.

### Observation 3: Dimension counts vary by phase — Phase 7 has 7 dims, Phase F7 has 5 dims

- Phase 7 (greenfield convergence): 7 dimensions (spec, test, implementation, verification, visual, performance, documentation)
- Phase F7 (feature-mode delta convergence): 5 dimensions (spec, test, implementation, verification, holdout)

These are NOT the same set. Phase F7 omits visual/performance/documentation explicit dimensions (visual is captured implicitly via holdout; performance via fuzz/regression; documentation is not a F7 explicit dimension). This is consistent with the stated philosophy — feature-mode is a delta optimization, not a full convergence rerun.

### Observation 4: F4/F5/F6 explicitly state "No human gate — automated quality gate"

In contrast to F1/F2/F3/F7 which require human approval. This places the human gates at scope decisions (F1), spec changes (F2), story creation (F3), and final merge authorization (F7), while implementation/review/hardening run autonomously.

### Observation 5: Hard-coded numerical thresholds are extensive

Each phase carries explicit numerical thresholds. Sample:

- Mutation kill rate: ≥90% (≥95% security-critical) — formal-verify, phase-f6
- Holdout satisfaction: mean ≥0.85, no must-pass <0.6, std-dev <0.15, 80% rotation — holdout-eval, phase-4
- Adversary novelty: <0.15 — phase-f5, phase-f7
- Adversary verification rate: <60% — phase-f7
- Fuzz time: ≥5 min/target — formal-verify, phase-6, phase-f6
- Feature mode threshold: <30% files, <50% components, ≤2 cascade levels — mode-decision-guide
- Greenfield switchover: ≥30% files, ≥50% components, ≥3 cascade levels — mode-decision-guide
- Convergence cycles: max 10 (F4 review), max 10 (fix-pr review), max 5 (F7 cycles) — fix-pr-delivery, phase-f4, phase-f7
- Spec rounds: max 3 PRD revisions — phase-1-prd-revision
- Performance: 10% regression flag, 25% PR threshold — perf-check, maintenance-sweep
- Cost: 10% of budget underspend warning — model-routing
- Default budgets: startup <100ms, binary <50MB, debug <60s, tests <120s — perf-check
- Post-feature: adoption ≥0.10, positive ≥0.60, error <5% — post-feature-validation

This level of numerical specificity is a strong asset for spec rebuild — the system has explicit, testable thresholds rather than vague quality language.

### Observation 6: All tool-invocation skills declare disable-model-invocation: true OR are direct work skills

Skills with `disable-model-invocation: true`: formal-verify, generate-pdf, holdout-eval, jira, perf-check. These are intended to be invoked by /vsdd-factory:<skill> commands rather than auto-triggered by the model from a description match.

### Observation 7: Two-step Red Gate is explicitly stated in F4

phase-f4-delta-implementation explicitly requires "Two-Step Red Gate (stubs first, then tests)" as a Quality Gate item. This makes sense given the broader project context where compilable-stubs is a separate step from tests-fail step.

---

## 5. Delta vs prior batch

This is batch 2 of 3, covering skills 41-80 alphabetically. No prior batches exist on disk (`pass-3-deep-skills-batch-1.md` not found). All 218 BCs in this file are new.

**Compared to pass-3-behavioral-contracts.md (87 BCs) and pass-3-behavioral-contracts-deep-r1.md (an unspecified count of additional BCs):**

- Prior pass-3 covered Rust crate behavior (registry/routing/payload/exec/sinks/sdk) and scattered skill samples.
- This batch is the first systematic per-skill BC extraction for the orchestration layer.
- New material: phase contracts (F1-F7, 0-7), reference skill identities, multi-repo synthesis, observability onboarding, policy registry, market intelligence, intelligence synthesis, post-feature validation, all phase F1-F7 quality gates, all phase 0-7 phase-entry contracts.

**No retractions** to prior batch claims — this batch covers a distinct surface (orchestration skills) versus the prior batch's runtime contracts (Rust crates) and randomly-sampled skills.

---

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 |
| **Novelty score** | SUBSTANTIVE (binary; 218 net-new BCs covering orchestration-layer skills not previously catalogued) |
| **Trajectory** | First systematic pass over orchestration-layer skills 41-80. No prior batch exists on disk; baseline is the runtime-focused pass-3-behavioral-contracts.md (Rust crates). Trajectory direction: surface shift from runtime contracts → skill workflow contracts. Decay direction is N/A for first batch (cannot decay from null prior). |
| **Verdict** | FINDINGS_REMAIN |

Removing this batch's findings WOULD change how the system is specified.

Justification:

1. **Phase contracts are the spine.** The 12 phase-entry skills (phases 0-7 + F1-F7) collectively define the workflow grammar of vsdd-factory. Without their gate criteria captured as BCs, a spec rebuild would produce the wrong execution graph. The gates explicitly distinguish "human gate" phases (F1/F2/F3/F7, P0/P1/P2/P7) from "automated gate" phases (F4/F5/F6) — a critical structural distinction.

2. **Numerical thresholds (Observation 5)** are not extractable from architecture/conventions docs alone. They live in the SKILL.md Quality Gate sections. Examples uniquely surfaced in this batch: 0.15 novelty score, 0.85 holdout satisfaction, 60% verification rate, 30%/50% mode thresholds, 10/5 max convergence cycles, 5-minute fuzz saturation. Without these numbers, a spec rebuild would lack acceptance criteria for the largest behavioral surface (workflow gates).

3. **Information-asymmetry wall enumerations** in F5/F6/holdout/multi-repo-synthesis tell us exactly what each agent tier MAY NOT see. Without these, a rebuild would over-share context and lose the cognitive-diversity guarantees that distinguish vsdd-factory from naive multi-agent systems. Six wall classes were named explicitly (BC-AUDIT-555).

4. **Conditional gating** (e.g., `feature_type in ['ui', 'full-stack']` for accessibility, multi-variant, E2E tests; `external service interaction changed` for DTU adversarial; bug-fix CRITICAL severity for expedited route) defines the dynamic shape of the pipeline. Six skills carry these conditions; capturing them as BCs is essential for a spec rebuild that supports the same conditional structure.

5. **The reference vs. work skill distinction** (Observation 1) shapes the loadable surface. The 4 reference skills should NOT be invoked as commands; they are read by other agents. A rebuild that conflates the two categories would produce broken UX.

This batch contributes net new structural knowledge about the orchestration layer that was not surfaced in the prior runtime-focused passes. Findings are SUBSTANTIVE.

---

## 7. Convergence declaration

**Another batch needed** — substantive gaps remain. Skills 1-40 (`activate` through `excalidraw-export`) and skills 81-119 (`pr-create` through end) are not yet covered with the same per-instance density. The user has explicitly clarified the spec must be sufficient to rebuild what currently ships, so a third batch is required.

This batch (skills 41-80) is itself complete and converged — every skill in scope has identity + behavioral + quality-gate + output BCs (where applicable per skill type).

---

## 8. State checkpoint

```yaml
pass: 3
batch: 2
batch_scope: skills 41-80 (alphabetical)
status: complete
files_read: 40 SKILL.md
bcs_produced: 218 (BC-AUDIT-400..617)
range_used: BC-AUDIT-400..617 (overflowed assigned 400..599 due to per-skill density requirement)
timestamp: 2026-04-25
novelty: SUBSTANTIVE
next_batch: 3 (skills 81-119)
```
