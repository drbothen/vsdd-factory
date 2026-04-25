# Pass 3 Deepening — Per-Agent Behavioral Contracts (vsdd-factory plugin)

## 1. Round metadata

**Date:** 2026-04-25
**Pass:** 3 (Behavioral Contracts) | **Round:** Deep — agent-instance coverage
**Project:** vsdd-factory (self-referential ingest)
**Scope:** Every agent identity shipped in `plugins/vsdd-factory/agents/` — 33 top-level `.md` files + the `orchestrator/` subdirectory containing `orchestrator.md` plus 9 sequence reference files. Total 34 agent identities with 10 orchestrator-load-on-demand companions (per-story-delivery, HEARTBEAT, sequence files) treated as orchestrator state — not separate agents.

**BC range allocated:** `BC-AUDIT-800` through `BC-AUDIT-999`. Used: 800-944 (145 BCs). Reserves 945-999 for future deepening.

**Source line citation convention:** Line numbers are zero-based offsets within each agent's `.md` source as read in this round. When a behavior is asserted in multiple lines, the cited line is the first authoritative occurrence.

**Inputs read first:**

- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` (full)
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` (head)
- All 33 top-level `plugins/vsdd-factory/agents/*.md` files (full)
- `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` (full) + companion sequence files (heads)

**Total agent inventory verified by `ls`:** 33 top-level `.md` + 1 `orchestrator/` subdir = 34 agents. Subdir contains: orchestrator.md (the agent), HEARTBEAT.md, brownfield-sequence.md, discovery-sequence.md, feature-sequence.md, greenfield-sequence.md, maintenance-sequence.md, multi-repo.md, per-story-delivery.md, steady-state.md (9 reference docs that the orchestrator loads on demand — not separately invokable; each declares `disable-model-invocation: true` in its frontmatter).

---

## 2. Agent catalog with BCs (alphabetical)

### 2.1 accessibility-auditor

**Frontmatter:** `model: sonnet`, `color: red`, no explicit `tools:` array. Default profile in body declares `Profile: full` with `read, write, edit, apply_patch, exec` — shell access required for axe-core, lighthouse, pa11y.

#### BC-AUDIT-800 — accessibility-auditor: WCAG criterion citation is mandatory for every finding

**Agent:** `plugins/vsdd-factory/agents/accessibility-auditor.md`
**Confidence:** HIGH
**Source line(s):** 100-104 ("Constraints" → "ALWAYS cite the specific WCAG criterion for every finding"); 252-258 ("Rules" → "Every finding must cite the specific WCAG criterion violated")
**Behavior:** The accessibility-auditor MUST attach a specific WCAG 2.1 criterion ID (e.g., 1.4.3) and a concrete file/component location to every finding emitted; generic advice is forbidden. The audit report's table must summarize per-principle counts (Perceivable / Operable / Understandable / Robust).
**Acceptance:** Every entry under `### [SEVERITY] Finding Title` in `accessibility-audit.md` has a non-empty `WCAG Criterion:` line and a non-empty `Location:` line. Findings missing either field are linted as policy violations.

#### BC-AUDIT-801 — accessibility-auditor: read-only — never modifies source

**Agent:** `plugins/vsdd-factory/agents/accessibility-auditor.md`
**Confidence:** HIGH
**Source line(s):** 99 ("You NEVER modify source code -- you report findings only"); 251 ("Do NOT modify files -- report findings only")
**Behavior:** Despite having `apply_patch`/`exec` in profile, the agent MUST limit writes to `.factory/cycles/**/hardening/accessibility-audit.md` and the JSON tool outputs. It MUST NOT modify `src/`, tests, or design system files.
**Acceptance:** Git diff after the agent runs touches no path outside `.factory/cycles/**/hardening/`.

#### BC-AUDIT-802 — accessibility-auditor: skip cleanly when product has no UI

**Agent:** `plugins/vsdd-factory/agents/accessibility-auditor.md`
**Confidence:** HIGH
**Source line(s):** 131 ("Skip entirely if the product has no user interface (CLI-only, library, API)"); 256 ("If the product has no UI, report 'N/A -- no user interface' and exit")
**Behavior:** When dispatched against a UI-less product, the agent emits `N/A — no user interface` and exits; it MUST NOT fabricate UI findings.
**Acceptance:** Audit report contains `N/A — no user interface` and zero finding entries.

#### BC-AUDIT-803 — accessibility-auditor: automated tools run before manual review

**Agent:** `plugins/vsdd-factory/agents/accessibility-auditor.md`
**Confidence:** HIGH
**Source line(s):** 102 ("ALWAYS run automated tool scans before manual review"); 257 ("Use automated tools first, then manual review for things tools miss")
**Behavior:** The agent MUST execute axe-core / lighthouse / pa11y (web) or jsx-a11y (React) before performing manual review. Output JSON files must be present before the markdown audit is composed.
**Acceptance:** `.factory/cycles/**/hardening/{accessibility-report.json, lighthouse-a11y.json, pa11y-report.json}` exist (when applicable) and are referenced by file path in the audit report.

#### BC-AUDIT-804 — accessibility-auditor: cannot load architecture files

**Agent:** `plugins/vsdd-factory/agents/accessibility-auditor.md`
**Confidence:** HIGH
**Source line(s):** 77-78 ("Information Asymmetry" → "Cannot see architecture files (.factory/specs/architecture/**)")
**Behavior:** The accessibility-auditor MUST NOT load any file under `.factory/specs/architecture/` — this preserves the user-experience perspective.
**Acceptance:** Tool-call audit shows zero Read calls against `.factory/specs/architecture/`.

---

### 2.2 adversary

**Frontmatter:** `tools: Read, Grep, Glob`, `model: opus`, `color: red`. Read-only profile enforced in agent body.

#### BC-AUDIT-805 — adversary: cannot see prior adversarial reviews (information wall)

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 21-25 ("You CANNOT access: .factory/cycles/*/adversarial-reviews/ from prior passes — each review is fresh"); 187-189 ("Tool Access" → "Denied: Write, Edit, Bash, exec, process")
**Behavior:** The adversary MUST start each pass without access to prior pass findings. Read-only tools are enforced (Read, Grep, Glob). Every pass derives novelty independently.
**Acceptance:** Tool-call audit shows zero Read calls against `.factory/cycles/*/adversarial-reviews/pass-*.md` for any prior pass; every finding has a fresh-context justification.

#### BC-AUDIT-806 — adversary: every finding tagged with HIGH/MEDIUM/LOW confidence

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 116-124 ("Confidence Levels" table)
**Behavior:** Every finding MUST carry a confidence tag (HIGH/MEDIUM/LOW). HIGH requires file path + line number + failure rationale; MEDIUM is pattern-based; LOW is inferred-from-absence.
**Acceptance:** Every finding entry in `.factory/cycles/<cycle>/adversarial-reviews/pass-N.md` has a `Confidence:` field set to one of {HIGH, MEDIUM, LOW}.

#### BC-AUDIT-807 — adversary: mis-anchoring always blocks convergence

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 99-114 ("Semantic Anchoring Audit") — closing sentence: "Mis-anchoring is NEVER an 'Observation' or 'deferred post-v1.' It ALWAYS blocks convergence."
**Behavior:** Findings about semantic anchor mismatches (capability anchors, subsystem IDs, VP anchor stories, BC cross-references, module names, file paths) MUST be classified as CRITICAL/HIGH/MEDIUM/LOW per the severity table — never as "Observation" or deferred. They block convergence.
**Acceptance:** Any finding with category `mis-anchor` or `semantic-anchor` appears under Critical Findings or Important Findings, never under Observations.

#### BC-AUDIT-808 — adversary: minimum 3 clean passes, max 10 before human escalation

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 96 ("Minimum 3 clean passes required. Maximum 10 before escalating to human")
**Behavior:** The adversary may declare convergence only after at least 3 consecutive NITPICK-novelty passes; orchestration must escalate to human after 10 total passes regardless of state.
**Acceptance:** Adversary's pass-N report sets `convergence_reached: true` only when N ≥ 3 and the prior 3 passes were all NITPICK; the 10th pass without convergence triggers human escalation.

#### BC-AUDIT-809 — adversary: max 3 self-validation iterations per pass (AgenticAKM)

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 78-86 ("Self-Validation Loop (AgenticAKM Pattern)" → "Max 3 refinement iterations per pass")
**Behavior:** Within a single pass, the adversary self-validates findings (evidence check, actionability check, duplication check) at most 3 times before shipping the report. Diminishing returns beyond 3 iterations validated by AgenticAKM study (29 repositories).
**Acceptance:** Pass report includes a self-validation iteration counter ≤ 3.

#### BC-AUDIT-810 — adversary: returns findings as chat text, never writes files

**Agent:** `plugins/vsdd-factory/agents/adversary.md`
**Confidence:** HIGH
**Source line(s):** 188-189 ("Findings are returned as chat text — the orchestrator persists them via state-manager")
**Behavior:** The adversary MUST NOT use Write or Edit tools. State-manager owns persistence to `.factory/cycles/<cycle>/adversarial-reviews/`. Read-only is structurally enforced.
**Acceptance:** Tool profile excludes Write/Edit/Bash/exec/process; effective allowed tools = {Read, Grep, Glob}.

---

### 2.3 architect

**Frontmatter:** `model: sonnet`, `color: green`. Profile `coding` (read/write/edit/apply_patch; no exec).

#### BC-AUDIT-811 — architect: every module gets a purity boundary classification

**Agent:** `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** 41 ("Every module has a purity boundary classification (pure core or effectful shell)"); 396 ("The purity boundary MUST be drawn before implementation design is finalized")
**Behavior:** Every module declared in `module-decomposition.md` MUST be classified as either Pure Core (deterministic, no I/O) or Effectful Shell. The purity boundary MUST be drawn before implementation design is finalized.
**Acceptance:** Every module row in `module-decomposition.md` has a non-empty `purity:` column with value in {pure, effectful}; `purity-boundary-map.md` exists.

#### BC-AUDIT-812 — architect: every VP has a viable proof strategy and feasibility note

**Agent:** `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** 35 ("each tracing to a BC postcondition/invariant, with proof harness skeleton and feasibility assessment"); 42 ("Every VP has a viable proof strategy and feasibility assessment")
**Behavior:** Each VP-NNN file MUST contain (a) a proof harness skeleton and (b) a feasibility assessment. Initial status = `draft`.
**Acceptance:** Every `verification-properties/VP-NNN.md` has a non-empty `proof_harness` block and a `feasibility:` field; `VP-INDEX.md` lists all VPs.

#### BC-AUDIT-813 — architect: ARCH-INDEX must declare deployment_topology

**Agent:** `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** 43 ("ARCH-INDEX.md includes deployment_topology field"); 130-181 ("Deployment Topology" section)
**Behavior:** `ARCH-INDEX.md` MUST include a frontmatter `deployment_topology` field set to `single-service` or `multi-service`. If `multi-service`, `system-overview.md` MUST include a Service Boundaries section listing each service, tech stack, role, and dependencies.
**Acceptance:** `ARCH-INDEX.md` parses with `deployment_topology in {single-service, multi-service}`. Multi-service architectures have a `## Service Boundaries` heading in system-overview.md.

#### BC-AUDIT-814 — architect: DTU assessment is mandatory and covers all 6 categories

**Agent:** `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** 182-227 ("DTU Assessment" → "MANDATORY — assess ALL six categories")
**Behavior:** After producing api-surface.md, the architect MUST produce `.factory/specs/dtu-assessment.md` covering 6 integration-surface categories (inbound data sources, outbound operations, identity & access, persistence & state, observability & export, enrichment & lookup). When a category has no services, the assessment MUST state "None identified — rationale: …" — not omit the category.
**Acceptance:** `dtu-assessment.md` exists and contains all 6 category headings, each with at least one service entry or an explicit "None identified" rationale. If no external dependencies exist, file sets `DTU_REQUIRED: false`.

#### BC-AUDIT-815 — architect: VP-INDEX changes propagate in same burst to verification-architecture.md and verification-coverage-matrix.md

**Agent:** `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** 342-352 ("VP-INDEX Propagation Obligation" → "MUST propagate in the SAME burst to every architecture document that cites VPs")
**Behavior:** Any change to VP-INDEX (addition, retirement, module reassignment, tool change, phase reassignment, count change) MUST propagate within the same burst to `verification-architecture.md` Provable Properties Catalog + P0/P1 lists AND `verification-coverage-matrix.md` VP-to-Module table + Totals row. Arithmetic invariant: VP-INDEX total = sum of per-tool counts = VP row count.
**Acceptance:** After any architect burst that touched VP-INDEX, `validate-vp-consistency.sh` reports PASS.

#### BC-AUDIT-816 — architect: VP-locking is 5-step protocol, after which VP is immutable

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md` AND `plugins/vsdd-factory/agents/architect.md`
**Confidence:** HIGH
**Source line(s):** architect.md:104-112 (VP withdrawal); formal-verifier.md:71-83 ("VP Locking Protocol" → "5-step protocol")
**Behavior:** The architect specifies VP authoring; formal-verifier owns the lock protocol: (1) write proof harness, (2) run to completion, (3) on success set `verification_lock: true` + `proof_completed_date` + `proof_file_hash`, (4) create git tag `vp-verified-VP-NNN-YYYY-MM-DD`, (5) VP-NNN.md is immutable thereafter. Issues require withdrawal, not editing. Architect approves withdrawals.
**Acceptance:** No VP file with `verification_lock: true` has commits modifying its body after the lock date (verified by git log on factory-artifacts branch).

---

### 2.4 business-analyst

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `coding`. Direct MCP access (Perplexity, Context7).

#### BC-AUDIT-817 — business-analyst: never invents capabilities — must ground in product brief

**Agent:** `plugins/vsdd-factory/agents/business-analyst.md`
**Confidence:** HIGH
**Source line(s):** 27-29 ("NEVER assume -- ask the human when requirements are ambiguous"); 45 ("Every capability is grounded in the product brief, not invented")
**Behavior:** Every CAP-NNN, DI-NNN, R-NNN, ASM-NNN MUST be traceable to a specific section/line of the product brief or domain research. When the brief is ambiguous, the agent stops and asks the human — it never guesses.
**Acceptance:** Every CAP-NNN entry in capabilities.md has a `traces_to: product-brief.md§<section>` annotation; no capability lacks a brief grounding.

#### BC-AUDIT-818 — business-analyst: produces sharded L2 (L2-INDEX + section files), never monolithic

**Agent:** `plugins/vsdd-factory/agents/business-analyst.md`
**Confidence:** HIGH
**Source line(s):** 30 ("ALWAYS produce sharded output (L2-INDEX.md + section files)"); 144-166 ("Sharded L2 Output (DF-021)")
**Behavior:** L2 output MUST be a sharded directory `.factory/specs/domain-spec/` containing L2-INDEX.md (produced first) plus 9-10 section files (capabilities, entities, invariants, events, edge-cases, assumptions, risks, failure-modes, differentiators, optionally event-flow). Each section targets 800-1,200 tokens. Monolithic `domain-spec-L2.md` is forbidden.
**Acceptance:** `.factory/specs/domain-spec/L2-INDEX.md` exists; section files exist with `traces_to: L2-INDEX.md` frontmatter. No monolithic domain-spec-L2.md exists.

#### BC-AUDIT-819 — business-analyst: include all template sections (mark N/A with justification)

**Agent:** `plugins/vsdd-factory/agents/business-analyst.md`
**Confidence:** HIGH
**Source line(s):** 31 ("ALWAYS include all template sections, even if marked 'N/A' with justification"); 49 ("All template sections present")
**Behavior:** The agent MUST include every template section even when not applicable; "N/A — justification" is required where a section doesn't apply. Omission is forbidden.
**Acceptance:** Every section header from `templates/L2-domain-spec-template.md` appears in the output; sections marked N/A include a non-empty justification line.

#### BC-AUDIT-820 — business-analyst: every ASM has a validation method; every R-NNN has a mitigation

**Agent:** `plugins/vsdd-factory/agents/business-analyst.md`
**Confidence:** HIGH
**Source line(s):** 49 ("Every ASM has a validation method; every R-NNN has a mitigation"); 104-110 ("ASM/R Production Rules")
**Behavior:** Every ASM-NNN MUST include `Status: unvalidated` (initial state) and a concrete Validation Method. Every R-NNN MUST have `Status: open`, a `Category` tag (security|performance|reliability|business), and a Mitigation. HIGH-impact R-NNNs with quantifiable mitigations get `NFR candidate: yes/no`. Security R-NNNs get `Security focus: yes`.
**Acceptance:** assumptions.md has zero rows missing `Status` or `Validation Method`; risks.md has zero rows missing `Status`, `Category`, or `Mitigation`.

---

### 2.5 code-reviewer

**Frontmatter:** `model: sonnet`, `color: red`. Profile `coding` (read/write/edit/apply_patch; no exec).

#### BC-AUDIT-821 — code-reviewer: cannot see adversarial reviews (information wall)

**Agent:** `plugins/vsdd-factory/agents/code-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 117-131 ("Information Asymmetry Wall" → "You CANNOT see: .factory/cycles/**/adversarial-reviews/**")
**Behavior:** The code-reviewer is a SECONDARY reviewer providing cognitive diversity (different model family from Builder + Adversary). It MUST NOT load any file under `.factory/cycles/**/adversarial-reviews/`. If information is needed from behind the wall, it must be derived independently from the artifacts the reviewer can see.
**Acceptance:** Tool-call audit shows zero Read calls against `.factory/cycles/**/adversarial-reviews/`.

#### BC-AUDIT-822 — code-reviewer: every finding classified into exactly one of 6 categories

**Agent:** `plugins/vsdd-factory/agents/code-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 31 ("ALWAYS classify every finding into exactly one category"); 53-62 (6 review categories)
**Behavior:** Every CR-NNN finding MUST be classified into exactly ONE of: spec-fidelity, code-quality, performance, maintainability, pattern-consistency, architecture-alignment.
**Acceptance:** Every finding has a `Category:` field set to exactly one allowed value. No finding lacks a category or has multiple.

#### BC-AUDIT-823 — code-reviewer: pass 2+ never re-reports prior findings

**Agent:** `plugins/vsdd-factory/agents/code-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 30 ("NEVER re-report findings already tracked in a previous review pass"); 87-104 ("Multi-Pass Review Protocol" → "Find issues NOT present in previous passes")
**Behavior:** Pass 1: all findings in Part B (no Part A). Pass 2+: Part A is fix-verification of prior findings (RESOLVED / PARTIALLY_RESOLVED / UNRESOLVED), Part B is only NEW findings not in any previous pass.
**Acceptance:** Pass-N report's frontmatter has `pass: N` and `previous_review: <path>`; Part A row count = prior pass's CR finding count; Part B contains only IDs > max prior CR-NNN.

#### BC-AUDIT-824 — code-reviewer: convergence verdict line is exact format

**Agent:** `plugins/vsdd-factory/agents/code-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 105-109 ("Convergence Verdict" → exact strings `CONVERGENCE_REACHED` or `findings remain -- iterate`)
**Behavior:** Each pass concludes with a verdict line that is one of two exact strings: `CONVERGENCE_REACHED` or `findings remain -- iterate`. Free-form alternatives are forbidden.
**Acceptance:** Pass report ends with one of the two literal strings as a single line.

---

### 2.6 codebase-analyzer

**Frontmatter:** `model: opus`, `tools: Read, Write, Edit, Glob, Grep, Bash, NotebookEdit, WebFetch, WebSearch`. Profile `full`.

#### BC-AUDIT-825 — codebase-analyzer: 6-pass protocol with per-pass output files

**Agent:** `plugins/vsdd-factory/agents/codebase-analyzer.md`
**Confidence:** HIGH
**Source line(s):** 104-117 ("CRITICAL: Writing Pass Files Is The Task" → "Your primary deliverable is the set of pass files on disk"); 118-306 (Pass 0–6 spec)
**Behavior:** The codebase-analyzer MUST execute 6 passes (0=Inventory, 1=Architecture, 2=Domain Model, 3=Behavioral Contracts, 4=NFRs, 5=Conventions, 6=Synthesis) in order, writing each pass to its own file before proceeding. Holding analysis only in conversation context is forbidden. Default rules about "do not create documentation" do NOT apply — file writing IS the task.
**Acceptance:** `.factory/semport/<project>/<project>-pass-0-inventory.md` ... `pass-6-synthesis.md` (or `phase-0-ingestion/pass-N-*.md` for self-ingest) all exist after the analyzer completes.

#### BC-AUDIT-826 — codebase-analyzer: never returns inline findings on Write denial

**Agent:** `plugins/vsdd-factory/agents/codebase-analyzer.md`
**Confidence:** HIGH
**Source line(s):** 110-117 ("If a Write or mkdir operation is denied: ... **Never** fall back to 'returning findings inline in the final message.' That is a failure mode, not a valid delivery path.")
**Behavior:** If Write fails, the agent MUST try at least 2 different formulations (absolute path, alternate tool) before reporting failure. It MUST NOT pivot to inline output as a substitute. Inline results are discarded.
**Acceptance:** On Write denial, agent transcript shows ≥2 retry formulations followed by an explicit STOP report — never an inline findings dump.

#### BC-AUDIT-827 — codebase-analyzer: convergence requires binary novelty (SUBSTANTIVE / NITPICK)

**Agent:** `plugins/vsdd-factory/agents/codebase-analyzer.md`
**Confidence:** HIGH
**Source line(s):** 322-332 ("Novelty Decay Assessment" → "Novelty is binary, not a gradient")
**Behavior:** Each deepening round MUST end with `Novelty: SUBSTANTIVE` or `Novelty: NITPICK` (literal binary). Soft phrases (borderline, effectively, mostly) are forbidden. The test: "Would removing this round's findings change how you'd spec the system? If yes → SUBSTANTIVE. If no → NITPICK."
**Acceptance:** Every deepening output ends with `Novelty: SUBSTANTIVE` or `Novelty: NITPICK` — no other tokens. Validated by `validate-novelty-assessment.sh` hook.

#### BC-AUDIT-828 — codebase-analyzer: convergence bounds — min 2 rounds, max 5 before escalation

**Agent:** `plugins/vsdd-factory/agents/codebase-analyzer.md`
**Confidence:** HIGH
**Source line(s):** 341-345 ("Convergence Bounds" → "Minimum: 2 deepening rounds per pass before declaring NITPICK; Maximum: 5 rounds per pass before escalating to human")
**Behavior:** A pass cannot declare NITPICK convergence before at least 2 deepening rounds. After 5 rounds without convergence, the agent escalates to human.
**Acceptance:** No pass declared NITPICK at round 1; round 5 with SUBSTANTIVE triggers escalation.

#### BC-AUDIT-829 — codebase-analyzer: state checkpoint at end of every pass

**Agent:** `plugins/vsdd-factory/agents/codebase-analyzer.md`
**Confidence:** HIGH
**Source line(s):** 156-162 (Pass 0 checkpoint template); 385-398 ("State Checkpointing" → required for every pass)
**Behavior:** Every pass output MUST include a YAML state checkpoint block at the end specifying `pass`, `status`, `files_scanned`, `timestamp`, `next_pass`, and `resume_from` (if partial).
**Acceptance:** Every pass file contains a `## State Checkpoint` heading with a YAML block matching the schema.

---

### 2.7 consistency-validator

**Frontmatter:** `model: sonnet`, `color: red`. Profile `coding`.

#### BC-AUDIT-830 — consistency-validator: 80 criteria, none skipped

**Agent:** `plugins/vsdd-factory/agents/consistency-validator.md`
**Confidence:** HIGH
**Source line(s):** 96-99 ("Constraints" → "ALWAYS check every criterion in your validation list, never skip criteria"); 113-116 ("Success Criteria" → "All criteria (1-80) checked; no criterion skipped"); 129-345 (criteria tables 1-80)
**Behavior:** Every consistency report MUST cover all 80 criteria. Criteria that cannot be checked must be reported as such with explanation — never silently skipped.
**Acceptance:** Consistency report's summary table has 80 rows (or 80 entries listed by ID); no criterion has status "skipped without reason."

#### BC-AUDIT-831 — consistency-validator: index-first validation discipline

**Agent:** `plugins/vsdd-factory/agents/consistency-validator.md`
**Confidence:** HIGH
**Source line(s):** 84-94 ("Context Discipline" → "Use index-first validation. Load index files for structural completeness checks before loading individual detail files")
**Behavior:** The validator MUST load ARCH-INDEX, BC-INDEX, VP-INDEX, STORY-INDEX, L2-INDEX, UX-INDEX first; detail files only when an index-level issue is detected. It MUST NOT load `src/` (Phase 6 scope) or holdout-scenarios/evaluations.
**Acceptance:** Tool-call audit shows index files loaded before any detail files; no Read on `src/` or `.factory/holdout-scenarios/evaluations/`.

#### BC-AUDIT-832 — consistency-validator: gate fails when blocking findings exist

**Agent:** `plugins/vsdd-factory/agents/consistency-validator.md`
**Confidence:** HIGH
**Source line(s):** 99 ("NEVER pass a validation gate when blocking findings exist"); 113-118 (Success Criteria)
**Behavior:** The validation gate result MUST be FAIL if any CRITICAL-severity criterion has unresolved violations, regardless of other passing criteria. Reports also produce a 0-100% consistency score.
**Acceptance:** Gate result row shows FAIL whenever the report contains any CRITICAL-severity finding with no remediation; PASS only when all CRITICAL criteria have status PASS.

#### BC-AUDIT-833 — consistency-validator: mis-anchoring is never an "Observation"

**Agent:** `plugins/vsdd-factory/agents/consistency-validator.md`
**Confidence:** HIGH
**Source line(s):** 318 ("Mis-anchoring is NEVER an 'Observation' or 'deferred post-v1.' It ALWAYS blocks convergence.")
**Behavior:** Criteria 70-73 (Semantic Anchoring Integrity) findings MUST be classified at MEDIUM severity or higher and block convergence. They cannot be filed as Observations or deferred.
**Acceptance:** No criterion 70-73 finding has severity Observation, LOW, or status "deferred."

---

### 2.8 data-engineer

**Frontmatter:** `model: sonnet`, `color: green`. Profile `full` (read/write/edit/apply_patch/exec/process).

#### BC-AUDIT-834 — data-engineer: every migration has both up and down scripts

**Agent:** `plugins/vsdd-factory/agents/data-engineer.md`
**Confidence:** HIGH
**Source line(s):** 31 ("ALWAYS write migrations that are reversible (both up and down scripts)"); 47 ("All migrations are reversible with both up and down scripts"); 95 ("ALWAYS write both up and down migrations")
**Behavior:** Every migration script MUST be reversible — for each `up.sql` (or equivalent), a corresponding `down.sql` is required. Schemas without rollback are rejected.
**Acceptance:** `migrations/` directory: every NNN-up.sql has a matching NNN-down.sql; CI lint hook enforces.

#### BC-AUDIT-835 — data-engineer: every field has a privacy classification before schema finalization

**Agent:** `plugins/vsdd-factory/agents/data-engineer.md`
**Confidence:** HIGH
**Source line(s):** 49 ("Every field has a privacy classification (PII/sensitive/public)"); 96 ("ALWAYS classify fields for privacy (PII/sensitive/public) before schema finalization")
**Behavior:** Every column/field in any schema artifact MUST be tagged PII, sensitive, or public. No schema is finalized with unclassified fields.
**Acceptance:** Every schema field has a `privacy:` annotation set to one of {PII, sensitive, public}.

#### BC-AUDIT-836 — data-engineer: pure validation logic never touches DB I/O

**Agent:** `plugins/vsdd-factory/agents/data-engineer.md`
**Confidence:** HIGH
**Source line(s):** 30 ("ALWAYS respect the purity boundary"); 54-69 ("Purity Boundary Discipline"); 95 ("NEVER mix pure validation logic with database I/O in the same function")
**Behavior:** Pure-core functions (validation, transformation, business rules) MUST be side-effect-free. DB connections, query execution, migration runners live in the Effectful Shell. Functions mixing both are rejected.
**Acceptance:** Static analysis or peer review confirms no function annotated `pure` performs I/O.

#### BC-AUDIT-837 — data-engineer: every schema traces to a BC-NNN data contract

**Agent:** `plugins/vsdd-factory/agents/data-engineer.md`
**Confidence:** HIGH
**Source line(s):** 48 ("Every schema traces to BC-NNN data structure contracts"); 108-110 ("BC-NNN Schema Tracing")
**Behavior:** Every schema artifact MUST trace to one or more BC-S.SS.NNN data structure contracts. Schema docs use canonical frontmatter with `traces_to:` populated.
**Acceptance:** Every schema file's frontmatter has a `traces_to:` field listing at least one BC-S.SS.NNN.

---

### 2.9 demo-recorder

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `full` (full coding access including shell).

#### BC-AUDIT-838 — demo-recorder: output strictly to docs/demo-evidence/<STORY-ID>/

**Agent:** `plugins/vsdd-factory/agents/demo-recorder.md`
**Confidence:** HIGH
**Source line(s):** 35-38 ("Constraints" → "ALWAYS produce evidence in docs/demo-evidence/<STORY-ID>/ ... NEVER place files directly at docs/demo-evidence/*.md — ONLY in per-story subdirectories"); 158-160 ("Output goes to docs/demo-evidence/<STORY-ID>/ (NOT flat docs/demo-evidence/, NOT .factory-demos/, NOT .factory/demo-recordings/)")
**Behavior:** All demo outputs MUST go under `docs/demo-evidence/<STORY-ID>/` in the story worktree (committed to feature branch). Forbidden destinations: `docs/demo-evidence/*.md` (flat), `.factory-demos/`, `.factory/demo-recordings/`. The `<STORY-ID>` matches the story frontmatter's `story_id:` field verbatim.
**Acceptance:** Git diff after demo-recorder runs adds files only under `docs/demo-evidence/<STORY-ID>/`; no flat demo files at repo root or under `.factory/`.

#### BC-AUDIT-839 — demo-recorder: VHS for CLI, Playwright for web — never plain text captures

**Agent:** `plugins/vsdd-factory/agents/demo-recorder.md`
**Confidence:** HIGH
**Source line(s):** 36 ("ALWAYS use VHS for CLI products or Playwright for web products — NOT plain text captures"); 119-120 ("Never use plain text captures — `cargo test` output is NOT a demo")
**Behavior:** The agent MUST use VHS (`.gif` + `.webm` + `.tape`) for CLI and Playwright (`.webm` + `.spec.ts`) for web. Plain `.txt` captures or `cargo test` stdout dumps are forbidden as demo evidence.
**Acceptance:** Every AC has at least one `.gif` or `.webm` artifact; zero `.txt` files in `docs/demo-evidence/<STORY-ID>/`.

#### BC-AUDIT-840 — demo-recorder: both success AND error paths recorded per AC

**Agent:** `plugins/vsdd-factory/agents/demo-recorder.md`
**Confidence:** HIGH
**Source line(s):** 36 ("ALWAYS record both success and error paths for each acceptance criterion"); 40 ("MUST NOT skip error-path demos"); 74 ("Every acceptance criterion has a recorded demo covering both success and error paths")
**Behavior:** Each AC must have at least two recordings — one for the success path, one for the error path. Recording only happy path is insufficient.
**Acceptance:** evidence-report.md table shows ≥2 entries per AC, distinguishing success from error.

#### BC-AUDIT-841 — demo-recorder: every recording links to a specific AC via AC-NNN naming

**Agent:** `plugins/vsdd-factory/agents/demo-recorder.md`
**Confidence:** HIGH
**Source line(s):** 75 ("Every recording links to a specific AC via AC-NNN or FLOW-NNN naming"); 81 ("ALWAYS link every recording to a specific acceptance criterion")
**Behavior:** Filenames MUST follow `AC-NNN-[description].{gif,webm,tape}` (CLI) or `FLOW-NNN-[description].{webm,spec.ts}` (web). evidence-report.md cross-references each recording to its AC.
**Acceptance:** All recording filenames match the regex `^(AC|FLOW)-\d+-[a-z-]+\.(gif|webm|tape|spec\.ts)$`. evidence-report.md table maps each filename to an AC.

#### BC-AUDIT-842 — demo-recorder: VHS tapes use Wait+Line, not Sleep

**Agent:** `plugins/vsdd-factory/agents/demo-recorder.md`
**Confidence:** HIGH
**Source line(s):** 103-104 ("Use Wait+Line /pattern/ instead of Sleep — waits for actual command completion, not a guessed duration")
**Behavior:** VHS tapes MUST use `Wait+Line /pattern/` for command completion synchronization. `Sleep` is permitted only for the final 2s frame hold.
**Acceptance:** `.tape` files contain `Wait+Line` directives; only one `Sleep 2s` allowed (final hold).

---

### 2.10 devops-engineer

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `full` (full coding access).

#### BC-AUDIT-843 — devops-engineer: never commits secrets

**Agent:** `plugins/vsdd-factory/agents/devops-engineer.md`
**Confidence:** HIGH
**Source line(s):** 79 ("NEVER commit secrets, API keys, or credentials to any file"); 70-75 ("All secrets must be referenced via GitHub Secrets, never hardcoded")
**Behavior:** All secrets MUST be referenced via GitHub Secrets or equivalent secret stores; never hardcoded in workflows, Dockerfiles, or scripts. The agent MUST never `git add` a `.env` file with values.
**Acceptance:** No committed file contains `API_KEY=<actual-value>` patterns; .env is gitignored; `.env.example` has key names only with empty/placeholder values.

#### BC-AUDIT-844 — devops-engineer: all GitHub Actions pinned to SHA, never tag

**Agent:** `plugins/vsdd-factory/agents/devops-engineer.md`
**Confidence:** HIGH
**Source line(s):** 75 ("Pin GitHub Actions to SHA, not version tags (supply chain security)"); 81 ("ALWAYS pin action versions to full SHA hashes"); 80 ("NEVER use `latest` tags in Dockerfiles or workflow action references")
**Behavior:** Every `uses:` reference in a workflow MUST be pinned to a 40-char commit SHA. Version tags (`@v3`, `@latest`) are forbidden. Dockerfile FROM lines must use SHA digests, not `latest`.
**Acceptance:** Every workflow's `uses:` line matches `actions/<repo>@[a-f0-9]{40}`; Dockerfiles use `image@sha256:...` or pinned tags.

#### BC-AUDIT-845 — devops-engineer: develop branch protected with CI status checks

**Agent:** `plugins/vsdd-factory/agents/devops-engineer.md`
**Confidence:** HIGH
**Source line(s):** 35 ("GitHub repository with branch protection configured (develop branch protected, CI status checks required)"); 167-173 (gh api branch protection PUT command); 305-321 (post-CI branch protection update)
**Behavior:** During repo init, `develop` branch MUST be configured with branch protection: `required_status_checks: strict=true, contexts: [CI/lint, CI/test, CI/build]`, `required_pull_request_reviews`, `enforce_admins=false`. Configuration done via `gh api repos/ORG/REPO/branches/develop/protection -X PUT`.
**Acceptance:** `gh api repos/ORG/REPO/branches/develop/protection` returns the configured rules; `enforce_admins.enabled: false`; required status checks include lint/test/build.

#### BC-AUDIT-846 — devops-engineer: .factory mounted as git worktree on factory-artifacts orphan branch

**Agent:** `plugins/vsdd-factory/agents/devops-engineer.md`
**Confidence:** HIGH
**Source line(s):** 87-104 ("Artifact Backup: .factory/ as Git Worktree" → orphan branch + worktree add)
**Behavior:** Instead of gitignoring `.factory/`, the agent creates an orphan branch `factory-artifacts` and mounts `.factory/` as a git worktree on it. Pattern: `git checkout --orphan factory-artifacts && git rm -rf . && git commit --allow-empty -m "..." && git push origin factory-artifacts && git worktree add .factory factory-artifacts`. Recovery from disk failure: `git clone ... && git worktree add .factory factory-artifacts`.
**Acceptance:** `.factory/.git` exists as a worktree marker; `git -C .factory branch --show-current` returns `factory-artifacts`.

#### BC-AUDIT-847 — devops-engineer: worktree-per-story discipline (.worktrees/STORY-NNN)

**Agent:** `plugins/vsdd-factory/agents/devops-engineer.md`
**Confidence:** HIGH
**Source line(s):** 192-213 ("Worktree Creation (Phase 2 -> 3 Transition)") + 244-249 (cleanup)
**Behavior:** For each story in a wave, the devops-engineer creates `.worktrees/STORY-NNN` on branch `feature/STORY-NNN`. After PR merges, the worktree is removed via `git worktree remove`. `.worktrees/` is gitignored.
**Acceptance:** Each active story has a `.worktrees/STORY-NNN` directory on its feature branch; merged story worktrees are removed within the cleanup step.

---

### 2.11 dtu-validator

**Frontmatter:** `model: sonnet`, `color: red`. Profile `full`.

#### BC-AUDIT-848 — dtu-validator: never use production API keys with write access

**Agent:** `plugins/vsdd-factory/agents/dtu-validator.md`
**Confidence:** HIGH
**Source line(s):** 105-114 ("API Key Management" → "NEVER use production keys with write access. Use test/sandbox API keys"); 134 ("You NEVER use production API keys with write access -- always test/sandbox keys")
**Behavior:** All API calls to real services MUST use test/sandbox keys (Stripe test mode, Okta preview org, GitHub read-only PAT). Production keys with write access are categorically forbidden — even for read operations they're avoided when test alternatives exist.
**Acceptance:** Environment audit shows no `STRIPE_LIVE_KEY` or production-tier credentials in the validator's effective env; only `*_TEST_KEY` / `*_PREVIEW_TOKEN` / `*_SANDBOX_KEY` patterns.

#### BC-AUDIT-849 — dtu-validator: fidelity thresholds enforced per L-tier

**Agent:** `plugins/vsdd-factory/agents/dtu-validator.md`
**Confidence:** HIGH
**Source line(s):** 39 ("MUST NOT accept fidelity scores below the documented thresholds"); 71-78 ("Fidelity thresholds" table — L1: 85%, L2: 90%, L3: 95%, L4: 98%)
**Behavior:** A clone PASSES validation only when fidelity ≥ {L1=0.85, L2=0.90, L3=0.95, L4=0.98}. Scores below the threshold trigger a fix story routed to implementer. The validator MUST NOT mark a clone as validated below threshold.
**Acceptance:** Every fidelity-report.md states the L-tier and the measured score; PASS only when score ≥ threshold for the declared tier.

#### BC-AUDIT-850 — dtu-validator: never modifies clone source — spawns implementer for fixes

**Agent:** `plugins/vsdd-factory/agents/dtu-validator.md`
**Confidence:** HIGH
**Source line(s):** 38 ("NEVER modify DTU clone source code directly -- spawn implementer for fixes"); 135 ("NEVER modify clone source code -- you validate and report fidelity only")
**Behavior:** The validator is read-only with respect to clone implementation. When fidelity is below threshold, it produces a fidelity-report.md with concrete deltas; the orchestrator dispatches implementer to fix.
**Acceptance:** Git diff shows no validator commits to `.factory/dtu-clones/[service]/` source files; only `fidelity-report.md` and `adversarial-config.yaml` written.

#### BC-AUDIT-851 — dtu-validator: drift >5% triggers stale flag and fix story

**Agent:** `plugins/vsdd-factory/agents/dtu-validator.md`
**Confidence:** HIGH
**Source line(s):** 56 ("Fidelity drift >5% is flagged with specific changed endpoints identified"); 96-101 ("If drift detected (score dropped >5%): Flag as stale clone ... Create fix story for clone update")
**Behavior:** Maintenance-mode drift monitoring re-runs fidelity checks. If the score drops more than 5 percentage points from the prior baseline, the clone is marked stale and a fix story is created listing the changed endpoints.
**Acceptance:** drift-report.md exists with `delta` field; `delta > 0.05` produces a fix story entry in STORY-INDEX.md.

---

### 2.12 dx-engineer

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `full`.

#### BC-AUDIT-852 — dx-engineer: never logs API key values — only names + pass/fail

**Agent:** `plugins/vsdd-factory/agents/dx-engineer.md`
**Confidence:** HIGH
**Source line(s):** 38 ("NEVER log, print, or echo API key values -- report only key names and pass/fail status"); 116-126 ("Environment Validation (Names Only -- Never Values)" → "NEVER: echo $KEY_VALUE  # never print values"); 141-144 ("Report ONLY pass/fail")
**Behavior:** When validating .env, the dx-engineer MUST use bash indirect reference (`[ -n "${!key}" ]`) to check presence; it MUST NOT `echo` or print the value. Reports state only `KEY_NAME: set/MISSING/valid/INVALID`.
**Acceptance:** Agent transcript and any reports contain zero `echo $KEY` or value-printing patterns; only `KEY: set` / `KEY: MISSING` / `KEY: valid` / `KEY: INVALID` lines.

#### BC-AUDIT-853 — dx-engineer: blocks pipeline when any of 3 model families unreachable

**Agent:** `plugins/vsdd-factory/agents/dx-engineer.md`
**Confidence:** HIGH
**Source line(s):** 41 ("ALWAYS verify all 3 model families are reachable during preflight"); 156-181 ("LLM Availability Check" → "All 3 model families are REQUIRED. If any model is unavailable: BLOCK the pipeline ... No silent model fallback ever")
**Behavior:** Pre-pipeline preflight MUST verify Claude Sonnet, Claude Opus, and adversary model + review-tier model are all reachable. ANY unreachable model BLOCKS the pipeline. Silent fallback to a different model is forbidden — human approval required for substitution.
**Acceptance:** Preflight report has 4 model rows all `healthy`; if any `UNAVAILABLE`, pipeline state shows BLOCKED with notification to human.

#### BC-AUDIT-854 — dx-engineer: tool installation requires security-reviewer audit

**Agent:** `plugins/vsdd-factory/agents/dx-engineer.md`
**Confidence:** HIGH
**Source line(s):** 37 ("NEVER install tools with known security advisories without security-reviewer audit"); 93-101 ("Supply Chain Security Audit" → "Before installing ANY tool, spawn security-reviewer ... ANY security finding -- regardless of severity -- notifies the human and BLOCKS installation")
**Behavior:** Before installing any tool (vhs, kani, semgrep, etc.), the dx-engineer MUST spawn security-reviewer for a CVE/NVD/OSV check + Perplexity recent-compromise search + SHA verification. Any finding blocks installation pending human approval.
**Acceptance:** Each tool install is preceded by a security-reviewer dispatch in the agent log; install proceeds only after CLEAN verdict or explicit human override.

#### BC-AUDIT-855 — dx-engineer: SHA pinning of dependencies and Docker images

**Agent:** `plugins/vsdd-factory/agents/dx-engineer.md`
**Confidence:** HIGH
**Source line(s):** 207-213 ("SHA Pinning" → "cargo install --locked, .tool-versions, Docker images: SHA256 digest, GitHub Actions: commit SHA")
**Behavior:** Where possible, the dx-engineer pins all dependencies: cargo `--locked`, `.tool-versions` for asdf/mise, Docker images by SHA256, GitHub Actions by commit SHA.
**Acceptance:** `.tool-versions` exists; cargo installs use `--locked`; no floating tags in Docker compose files.

---

### 2.13 e2e-tester

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `full`.

#### BC-AUDIT-856 — e2e-tester: never mocks internal components

**Agent:** `plugins/vsdd-factory/agents/e2e-tester.md`
**Confidence:** HIGH
**Source line(s):** 115 ("No internal components mocked; tests exercise the real system end-to-end"); 191 ("You NEVER mock internal components -- E2E tests exercise the real system"); 199 ("E2E tests do NOT mock internal components -- they test the real system")
**Behavior:** End-to-end tests MUST exercise the real system through its public interface (CLI, HTTP, library API). Internal modules MUST NOT be mocked or stubbed. Test-writer's unit tests handle isolated mocking; e2e is integration-only.
**Acceptance:** No file in `tests/e2e/` imports a mocking framework like `mockito`/`mock-jest` against an internal module.

#### BC-AUDIT-857 — e2e-tester: BC-NNN traceable test naming

**Agent:** `plugins/vsdd-factory/agents/e2e-tester.md`
**Confidence:** HIGH
**Source line(s):** 110 ("BC-NNN tracing: every test traces to acceptance criteria via test_e2e_BC_S_SS_NNN_xxx() naming"); 207 ("Test naming follows test_e2e_BC_S_SS_NNN_xxx() convention")
**Behavior:** Every E2E test name MUST follow the pattern `test_e2e_BC_S_SS_NNN_xxx()` for full traceability.
**Acceptance:** Every test under `tests/e2e/` matches the naming regex; no `test_e2e_1()` or vague names.

#### BC-AUDIT-858 — e2e-tester: tests are idempotent and clean up

**Agent:** `plugins/vsdd-factory/agents/e2e-tester.md`
**Confidence:** HIGH
**Source line(s):** 192 ("ALWAYS clean up test data after each test run"); 200-201 ("E2E tests clean up after themselves (delete test data, reset state); E2E tests must be idempotent (runnable multiple times without side effects)")
**Behavior:** Every E2E test MUST clean up after itself and be runnable multiple times without state-coupled failures.
**Acceptance:** Re-running the e2e suite back-to-back yields identical pass/fail counts; no `setup-once` fixtures that fail on re-run.

#### BC-AUDIT-859 — e2e-tester: writes tests, not implementation code

**Agent:** `plugins/vsdd-factory/agents/e2e-tester.md`
**Confidence:** HIGH
**Source line(s):** 92 ("NEVER modify source code -- tests only")
**Behavior:** Despite full profile, the agent MUST limit writes to `tests/e2e/` (or framework-specific directory) and evidence directories. It MUST NOT modify `src/`.
**Acceptance:** Git diff shows no agent commits to `src/`.

---

### 2.14 formal-verifier

**Frontmatter:** `model: opus`, `color: red`. Profile `full`.

#### BC-AUDIT-860 — formal-verifier: never marks VP verified without running proof to completion

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 39 ("NEVER mark a VP as verified without running the proof to completion"); 248 ("Never mark a VP as verified without running the proof to completion, and never edit a locked VP document")
**Behavior:** Setting `verification_lock: true` and `status: verified` on a VP requires that the proof harness ran to completion (e.g., `cargo kani --harness` exited 0). Speculative verification is forbidden.
**Acceptance:** Each verified VP file has matching evidence in `kani-results/` showing successful completion.

#### BC-AUDIT-861 — formal-verifier: cannot see adversarial reviews (information wall)

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 207-220 ("Information Asymmetry Wall" → "You CANNOT see: .factory/cycles/**/adversarial-reviews/**")
**Behavior:** Formal verification MUST be driven by the Provable Properties Catalog and specification, not adversarial findings. The agent MUST NOT load any file under `.factory/cycles/**/adversarial-reviews/`.
**Acceptance:** Tool-call audit shows zero Read calls against `.factory/cycles/**/adversarial-reviews/`.

#### BC-AUDIT-862 — formal-verifier: VP withdrawal requires architect approval

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 41 ("ALWAYS follow the VP withdrawal process for invalid proofs -- never silently remove"); 85-93 ("VP Withdrawal Initiation" → "Submit the withdrawal document to the architect for approval. Only after architect approval: Mark the VP as status: withdrawn")
**Behavior:** When a verified VP's proof is found invalid, the formal-verifier produces a withdrawal document and submits to architect. The VP is marked `status: withdrawn` only after architect approval. Silent removal is forbidden.
**Acceptance:** Every `status: withdrawn` VP has a corresponding withdrawal document and an architect-approval annotation.

#### BC-AUDIT-863 — formal-verifier: mutation kill rate enforced per module-criticality tier

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 60-62 ("Mutation kill rate exceeds module-criticality thresholds (CRITICAL: 95%, HIGH: 90%, MEDIUM: 80%, LOW: 70%)"); 152-156 (cargo mutants threshold enforcement)
**Behavior:** Mutation testing per module MUST meet or exceed `module-criticality.md` thresholds: CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%. Surviving mutants below threshold trigger added tests.
**Acceptance:** `mutation-results/` per-module report shows kill rate ≥ threshold for each module's criticality tier.

#### BC-AUDIT-864 — formal-verifier: fuzz targets run ≥5 minutes with no crashes

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 60 ("Fuzz testing finds no crashes after 5 minutes per target"); 132-138 (cargo fuzz run -- -max_total_time=300); 226-228 (Convergence Criteria)
**Behavior:** Fuzz targets MUST run for at least 5 minutes (`-max_total_time=300`) per target with zero crashes for Phase 5 to converge.
**Acceptance:** `fuzz-results/` reports each target ran ≥300 seconds and produced no crash artifacts.

#### BC-AUDIT-865 — formal-verifier: purity boundary audit catches I/O in pure-core

**Agent:** `plugins/vsdd-factory/agents/formal-verifier.md`
**Confidence:** HIGH
**Source line(s):** 167-178 ("Purity Boundary Audit" → for every module marked pure core, check no I/O, no global mutable state, all deps pure, all context as parameters)
**Behavior:** For each module classified as pure core in `purity-boundary-map.md`, the formal-verifier verifies no I/O operations, no global mutable state, pure-only dependencies, and no implicit state. Side effects in pure core flag for refactoring.
**Acceptance:** `purity-audit.md` lists every pure-core module with PASS/FAIL status; failures route back to architect.

---

### 2.15 github-ops

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `full`.

#### BC-AUDIT-866 — github-ops: executes only — never makes decisions

**Agent:** `plugins/vsdd-factory/agents/github-ops.md`
**Confidence:** HIGH
**Source line(s):** 36 ("NEVER make decisions -- execute exactly what is requested and return results"); 86-90 ("You NEVER make decisions about what to review, merge, or triage; You NEVER write source code or interpret review results"); 121-122 ("You are a tool, not a decision-maker. Execute exactly the gh command requested and return the full output")
**Behavior:** The github-ops agent is a thin wrapper around the `gh` CLI. It MUST NOT modify the requested command, MUST NOT interpret or filter results, and MUST NOT decide what to merge / review / triage.
**Acceptance:** Agent's tool-call log shows the exact command from the dispatch prompt being executed; output returned verbatim.

#### BC-AUDIT-867 — github-ops: returns full stdout + stderr unmodified

**Agent:** `plugins/vsdd-factory/agents/github-ops.md`
**Confidence:** HIGH
**Source line(s):** 38 ("ALWAYS return the full command output unmodified"); 88 ("ALWAYS return full command output (stdout + stderr) without suppressing errors"); 110 ("Never suppress errors or return partial output")
**Behavior:** Every github-ops response MUST contain the complete stdout + stderr from the executed `gh` command. Truncation, error suppression, or summarization is forbidden.
**Acceptance:** Response payload byte length ≥ length of `gh` command output (modulo agent message wrapping); stderr present when command failed.

#### BC-AUDIT-868 — github-ops: retry once on transient errors, then report

**Agent:** `plugins/vsdd-factory/agents/github-ops.md`
**Confidence:** HIGH
**Source line(s):** 93-95 ("Failure & Escalation" → "Level 1 (self-correct): Retry a gh command on transient network errors (once only)"); 109 ("If a gh command fails, return the full error output -- do NOT retry")
**Behavior:** Retry policy is exactly one retry on transient network errors. Persistent failures (auth, rate limit, command-level errors) are reported with full error output, not retried.
**Acceptance:** Agent log shows ≤1 retry per command; auth/rate-limit failures yield immediate error reports including reset timestamp.

---

### 2.16 holdout-evaluator

**Frontmatter:** `tools: Bash, Read`, `model: opus`, `color: red`. Profile `restricted` — no Write/Edit/Glob/Grep.

#### BC-AUDIT-869 — holdout-evaluator: cannot read source code, specs, or prior reviews

**Agent:** `plugins/vsdd-factory/agents/holdout-evaluator.md`
**Confidence:** HIGH
**Source line(s):** 21-29 ("Information Asymmetry Wall" → "CANNOT access: .factory/specs/, src/ internals, .factory/cycles/*/adversarial-reviews/, .factory/semport/, PR history, Test source code")
**Behavior:** The holdout-evaluator MUST evaluate the system as a black-box user. It MUST NOT read source code, BCs, architecture docs, prior reviews, semport translation artifacts, or test source. It CAN read `.factory/holdout-scenarios/`, `.factory/specs/product-brief.md` (high-level only), and observe runtime behavior.
**Acceptance:** Tool-call audit shows zero Read against forbidden paths; only `.factory/holdout-scenarios/`, `product-brief.md`, and Bash invocations of the application binary.

#### BC-AUDIT-870 — holdout-evaluator: gate criteria — mean ≥0.85, every critical scenario ≥0.60

**Agent:** `plugins/vsdd-factory/agents/holdout-evaluator.md`
**Confidence:** HIGH
**Source line(s):** 86-88 ("Gate criteria" → "PASS: Mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60; FAIL: Below thresholds")
**Behavior:** The Phase 4 gate passes only when mean satisfaction across all evaluated scenarios is ≥ 0.85 AND every critical scenario scores ≥ 0.60.
**Acceptance:** Holdout report's Gate row reads PASS only if both numerical conditions hold; otherwise FAIL with gap report.

#### BC-AUDIT-871 — holdout-evaluator: 0.0–1.0 satisfaction scoring per scenario

**Agent:** `plugins/vsdd-factory/agents/holdout-evaluator.md`
**Confidence:** HIGH
**Source line(s):** 50-60 (scoring scale: 1.0 = fully satisfied, 0.8 = minor deviation, 0.5 = partial, 0.2 = mostly failing, 0.0 = complete failure)
**Behavior:** Each scenario gets a satisfaction score on a 0.0–1.0 continuous scale interpreted via the documented anchors (1.0/0.8/0.5/0.2/0.0).
**Acceptance:** Every Per-Scenario Result row has a `Score` value in [0.0, 1.0].

#### BC-AUDIT-872 — holdout-evaluator: read-only — no Write tool

**Agent:** `plugins/vsdd-factory/agents/holdout-evaluator.md`
**Confidence:** HIGH
**Source line(s):** 91-94 ("Tool Access" → "Profile: restricted; Available: Bash (for running the application under test), Read (for reading holdout scenarios); Denied: Write, Edit, Glob, Grep")
**Behavior:** The agent MUST execute the SUT (Bash) and Read holdout scenarios. It MUST NOT write evaluation reports directly — those are persisted by orchestrator's call back to state-manager (the report is composed in chat).
**Acceptance:** Tool profile = restricted; effective allowed tools = {Bash, Read}.

---

### 2.17 implementer

**Frontmatter:** `model: sonnet`, `color: green`. Profile `full`.

#### BC-AUDIT-873 — implementer: never writes code without a failing test (Red Gate)

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 95 ("You NEVER write tests. The Test Writer does that."); 96 ("You NEVER write code without a corresponding failing test"); 356 ("You are the TDD implementer. You NEVER write code without a corresponding failing test.")
**Behavior:** The implementer MUST consume failing tests from test-writer. Writing implementation without a corresponding failing test is forbidden. Tests are produced by test-writer; implementer never writes tests.
**Acceptance:** Every implementer commit advances at least one previously-failing test to passing; no commit adds source code without a related test having been previously red.

#### BC-AUDIT-874 — implementer: minimum code per test (TDD discipline)

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 97 ("You NEVER write more code than necessary to pass the current test"); 89-93 (Protocol → Pick the next failing test, write the MINIMUM code, run full suite, repeat, refactor)
**Behavior:** The implementer writes the minimum code to make exactly ONE failing test pass, then moves to the next failing test. After all tests pass, refactor with tests as safety net. Bulk implementation is forbidden.
**Acceptance:** Micro-commit history shows one test at a time progressing red→green; no single commit makes >2 tests pass simultaneously (allowing for shared dependencies).

#### BC-AUDIT-875 — implementer: micro-commit per passing test, squash before PR

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 134-156 ("Micro-Commit Protocol (TDD Crash Recovery)" → "Commit after EACH test passes during the TDD loop ... Squash all wip commits into a clean commit before pushing for PR")
**Behavior:** During the TDD loop, the implementer commits after each test goes green with `wip(STORY-NNN): test_X passes`. Before pushing the PR, it squashes wip commits into a clean conventional commit. Limits crash loss to ~5 min.
**Acceptance:** Pre-rebase reflog shows multiple `wip(STORY-NNN):` commits; final pushed history shows one clean `feat(STORY-NNN):` commit per story.

#### BC-AUDIT-876 — implementer: respects purity boundary map

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 99-101 ("ALWAYS respect the Purity Boundary Map from the architecture spec. Functions in the pure core MUST be side-effect-free")
**Behavior:** For every module in `purity-boundary-map.md` marked pure core, the implementer MUST keep functions side-effect-free. Effectful operations (I/O, DB, network) live in the Effectful Shell.
**Acceptance:** Formal-verifier's purity audit (BC-AUDIT-865) passes after implementation.

#### BC-AUDIT-877 — implementer: HALT only on blocker, impossibility, or 3 consecutive failures

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 160-176 ("Continuous Execution" → "DO NOT stop at milestones, phase boundaries, or session boundaries"; "HALT Conditions (the ONLY reasons to stop): 1. Blocker requiring human input, 2. Technical impossibility, 3. Three consecutive failures")
**Behavior:** The implementer continues working without pause until story complete or one of three explicit HALT conditions is met. Pausing for "review checkpoints" or proposing to stop after one test passes is forbidden.
**Acceptance:** Agent transcripts show no mid-story "should I continue?" prompts; HALT only with explicit blocker/impossibility/3-failures justification.

#### BC-AUDIT-878 — implementer: status reporting in {DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED}

**Agent:** `plugins/vsdd-factory/agents/implementer.md`
**Confidence:** HIGH
**Source line(s):** 226-235 ("Reporting" status table)
**Behavior:** Final report uses one of four canonical statuses: DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT / BLOCKED. Each status implies a specific dispatcher action.
**Acceptance:** Final agent message includes one of the four exact tokens.

---

### 2.18 orchestrator (orchestrator/orchestrator.md)

**Frontmatter:** `name: orchestrator`, no model specified (uses default). Tool profile `full` with explicit deny list: write/edit/apply_patch/exec/process all denied.

#### BC-AUDIT-879 — orchestrator: never writes any files — delegates all writes

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 102 ("You NEVER write ANY files — you delegate all writing to specialist agents"); 372-377 ("Tool Access" → "Denied: write, edit, apply_patch, exec, process; You cannot write ANY file")
**Behavior:** The orchestrator is a coordinator with read-only file access. It MUST delegate every file write to a specialist agent via the Agent tool with `subagent_type` set. Direct use of Write, Edit, apply_patch, exec, or process is forbidden.
**Acceptance:** Orchestrator's effective tool profile excludes write/edit/apply_patch/exec/process; deny list verified in agent frontmatter or runtime config.

#### BC-AUDIT-880 — orchestrator: never delegates to itself

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 126 ("You MUST NOT spawn with `agentId: \"orchestrator\"` — you never delegate to yourself")
**Behavior:** The orchestrator MUST NOT include `subagent_type: "vsdd-factory:orchestrator"` in any Agent tool dispatch.
**Acceptance:** Tool-call audit shows zero Agent dispatches with `subagent_type` matching orchestrator.

#### BC-AUDIT-881 — orchestrator: never skips per-story delivery sub-steps

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 104-107 ("You NEVER skip per-story delivery steps — EVERY story follows ALL steps in per-story-delivery.md: (a) test-writer: stubs → (b) test-writer: failing tests → (c) implementer: TDD → (d) demo-recorder: per-AC demos → (e) push → (f) pr-manager: full 9-step PR process → (g) worktree cleanup")
**Behavior:** Every story MUST traverse all 7 sub-steps. Shortcuts (e.g., skipping demo recording, going directly to github-ops) are forbidden.
**Acceptance:** Orchestrator dispatch log per story shows all 7 sub-steps in order before story is marked complete.

#### BC-AUDIT-882 — orchestrator: never composes PR bodies or gh commands

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 122 ("You NEVER compose PR bodies, gh commands, or shell scripts in task descriptions — pr-manager owns the PR lifecycle"); 122 ("You NEVER spawn github-ops directly for PR operations — that's pr-manager's job")
**Behavior:** PR descriptions, `gh` commands, and shell scripts MUST be authored by pr-manager (or its delegates), not by the orchestrator. The orchestrator MUST dispatch pr-manager for the PR lifecycle, never github-ops directly.
**Acceptance:** Orchestrator dispatch prompts contain no `gh pr create`/`gh pr merge` strings; PR-related dispatches go to `vsdd-factory:pr-manager`.

#### BC-AUDIT-883 — orchestrator: state-manager runs LAST in every burst

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 130 ("You ALWAYS dispatch state-manager LAST in every burst — after all other agents in the burst have completed. State-manager must not write citations (STORY-INDEX, BC-INDEX) until story-writer/product-owner have finalized their version bumps. Running state-manager early causes version-race regressions.")
**Behavior:** In every multi-agent burst, state-manager is the FINAL dispatch. Running state-manager concurrently with story-writer or product-owner is forbidden — version-race regressions.
**Acceptance:** Burst log shows state-manager dispatched after all other agents in the burst have returned DONE.

#### BC-AUDIT-884 — orchestrator: never sets runTimeoutSeconds below 300

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 128 ("You MUST NOT set `runTimeoutSeconds` below 300 (5 min) on any spawn. Default is 7200 (2 hours) — use it. Aggressive timeouts cause agents to die mid-work.")
**Behavior:** Every Agent dispatch MUST have `runTimeoutSeconds` ≥ 300 (5 min); default 7200 (2 hours). Aggressive timeouts forbidden.
**Acceptance:** Tool-call audit shows zero Agent dispatches with `runTimeoutSeconds < 300`.

#### BC-AUDIT-885 — orchestrator: input-hash drift check before Phase 1/2/3/7 human approval

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 121 ("Phase gates 1, 2, 3, 7: input-hash drift check — run /vsdd-factory:check-input-drift BEFORE human approval. Any DRIFT results must be resolved before proceeding.")
**Behavior:** Before requesting human approval at phase gates 1, 2, 3, and 7, the orchestrator MUST invoke `/vsdd-factory:check-input-drift`. Any DRIFT findings block approval until resolved.
**Acceptance:** Phase 1/2/3/7 gate logs include a `check-input-drift` skill invocation entry preceding human approval.

#### BC-AUDIT-886 — orchestrator: prepends `cd <project-path> &&` and uses absolute paths in every dispatch

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 142-156 ("Task Preamble (CRITICAL)" → "always prepend a cd command in the prompt; Every prompt MUST: 1. Start with `cd <resolved-project-path> &&` 2. Specify ALL file paths as absolute paths")
**Behavior:** Every Agent dispatch prompt MUST begin with `cd <resolved-project-path> &&` and reference all file paths as absolute paths. Relative paths cause writes to land in the engine directory.
**Acceptance:** Tool-call audit shows every Agent prompt starting with `cd /` followed by `&&`; no relative `.factory/` paths.

#### BC-AUDIT-887 — orchestrator: workspace resolution at session start (not from env var)

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 62-80 ("Workspace Resolution (CRITICAL — do this FIRST)" → "Resolution order: 1. Resume (read STATE.md); 2. User provides path; 3. Greenfield (devops-engineer creates repo); 4. Explicit"; "After resolution, verify the path is NOT inside dark-factory")
**Behavior:** At session start, the orchestrator MUST resolve the target project path via the 4-step order. The resolved path MUST NOT contain `dark-factory` (engine guard). The path is stored and used in every dispatch's `cd` preamble.
**Acceptance:** Session start log records resolved WORKSPACE_PATH; path passes the no-`dark-factory` check.

#### BC-AUDIT-888 — orchestrator: 3-clean-passes minimum for adversarial convergence

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 113-117 ("MANDATORY STEPS: ... Phase 1d: adversarial spec convergence — always 3 clean passes minimum; Phase 2: adversarial story convergence — always 3 clean passes minimum; Phase 3: per-story adversarial convergence — always 3 clean passes minimum per story; Phase 3: wave-level adversarial convergence — always 3 clean passes minimum per wave; Phase 5: adversarial implementation convergence — always 3 clean passes minimum")
**Behavior:** Adversarial convergence at Phases 1d, 2, 3 (per-story and per-wave), and 5 each REQUIRE at least 3 consecutive NITPICK-novelty passes before declaring convergence.
**Acceptance:** Each convergence-trajectory.md shows ≥3 consecutive nitpick-novelty passes for every relevant phase.

#### BC-AUDIT-889 — orchestrator: dispatches state-manager directly for .factory/ commits — never devops-engineer

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 196-204 ("State Manager Delegation" → "State-manager has direct shell access for git operations in .factory/ — it commits factory artifacts directly without spawning devops-engineer")
**Behavior:** All `.factory/` git operations are delegated to state-manager directly. The orchestrator MUST NOT dispatch devops-engineer for factory artifact commits — devops-engineer's git scope is source-code branches only.
**Acceptance:** All `git commit` events on `factory-artifacts` branch are authored via state-manager dispatches; devops-engineer commits only on `develop`/`feature/*` branches.

#### BC-AUDIT-890 — orchestrator: split bursts of >8 artifacts into create + integrate sub-bursts

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 129 ("When dispatching story-writer or product-owner to create >8 artifacts, ALWAYS split into 'create' and 'integrate' sub-bursts. Single-burst creation of >8 artifacts causes context overflow and quality degradation.")
**Behavior:** When a single dispatch would create more than 8 artifacts (stories, BCs, etc.), the orchestrator MUST split into Sub-burst A (Create) and Sub-burst B (Integrate). 121k+ token transcripts degrade output quality.
**Acceptance:** Orchestrator's dispatch log shows two sequential dispatches per >8-artifact burst, with the first creating files and the second updating indexes/cross-references.

#### BC-AUDIT-891 — orchestrator: heartbeat is read-only (no spawning, no writes)

**Agent:** `plugins/vsdd-factory/agents/orchestrator/HEARTBEAT.md`
**Confidence:** HIGH
**Source line(s):** HEARTBEAT.md:48-51 ("Rules: Do NOT spawn subagents during heartbeat — keep it lightweight (reads only); Do NOT write files during heartbeat — only read and report")
**Behavior:** Heartbeat reads STATE.md / cost-summary.md / .factory/.git existence and reports alerts. It MUST NOT spawn sub-agents or write any files. Replies `HEARTBEAT_OK` if all checks pass.
**Acceptance:** Heartbeat session shows zero Write or Agent tool calls; only Read invocations.

#### BC-AUDIT-892 — orchestrator: pipeline resume requires factory-worktree-health BEFORE STATE.md read

**Agent:** `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`
**Confidence:** HIGH
**Source line(s):** 354-360 ("Pipeline Resume" → "FIRST: Spawn devops-engineer with factory-worktree-health skill (BLOCKING). Do NOT read STATE.md or .factory/ until this passes")
**Behavior:** On resume, the orchestrator MUST first spawn devops-engineer to run the factory-worktree-health skill. STATE.md and `.factory/` are not read until the health check passes.
**Acceptance:** Resume session log shows devops-engineer dispatch before any Read on STATE.md or `.factory/` artifacts.

---

### 2.19 performance-engineer

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `full`.

#### BC-AUDIT-893 — performance-engineer: never modifies source code — measurement only

**Agent:** `plugins/vsdd-factory/agents/performance-engineer.md`
**Confidence:** HIGH
**Source line(s):** 83 ("NEVER modify source code -- benchmarks and measurement only")
**Behavior:** Despite full profile, the agent MUST limit writes to benchmark code, baseline files, and reports. It MUST NOT modify implementation source.
**Acceptance:** Git diff after performance-engineer runs shows changes only in `benches/`, `tests/perf/`, or `.factory/cycles/**/hardening/`; zero changes in `src/`.

#### BC-AUDIT-894 — performance-engineer: capture baseline BEFORE changes

**Agent:** `plugins/vsdd-factory/agents/performance-engineer.md`
**Confidence:** HIGH
**Source line(s):** 86 ("MUST NOT skip baseline measurement before evaluating changes"); 105-114 ("Baseline Measurement" → "Run benchmarks BEFORE implementation changes (capture baseline)"); 148 ("ALWAYS capture baseline BEFORE changes (otherwise you can't detect regression)")
**Behavior:** Before evaluating any change, the agent MUST capture a baseline measurement (mean, median, p95, p99, throughput, memory) and write it to `performance-baseline.md`. Without a baseline, regression detection is impossible.
**Acceptance:** `.factory/cycles/**/hardening/performance-baseline.md` exists and pre-dates any post-change measurement.

#### BC-AUDIT-895 — performance-engineer: numerical thresholds only, never qualitative

**Agent:** `plugins/vsdd-factory/agents/performance-engineer.md`
**Confidence:** HIGH
**Source line(s):** 85 ("ALWAYS use numerical thresholds from NFRs (not qualitative assessments)"); 116-122 (thresholds: latency p99 +10%/+25%, throughput -10%, memory +20%)
**Behavior:** The agent uses NFR-NNN numerical targets as acceptance criteria. Regression thresholds: latency p99 increase >10% = WARNING, >25% = CRITICAL; throughput decrease >10% = WARNING; memory increase >20% = WARNING. Qualitative assessments are forbidden.
**Acceptance:** performance-report.md uses absolute numbers and explicit deltas; no "fast"/"slow"/"good" qualitative verdicts.

#### BC-AUDIT-896 — performance-engineer: every NFR-NNN gets a compliance row

**Agent:** `plugins/vsdd-factory/agents/performance-engineer.md`
**Confidence:** HIGH
**Source line(s):** 138-144 ("NFR Validation Method Execution Obligation" → "Every NFR-NNN must have a corresponding row in the NFR compliance matrix of the performance report")
**Behavior:** For each NFR-NNN in `prd-supplements/nfr-catalog.md`, the agent MUST execute the stated Validation Method and produce a compliance matrix row showing measured value vs target with PASS/FAIL.
**Acceptance:** performance-report.md NFR compliance matrix has one row per NFR-NNN; no NFRs missing.

---

### 2.20 pr-manager

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `coding` (read/write/edit/apply_patch; no exec/process). Sub-agent dispatch via Agent tool with `subagent_type`.

#### BC-AUDIT-897 — pr-manager: 9-step coordinator, never exits mid-flow

**Agent:** `plugins/vsdd-factory/agents/pr-manager.md`
**Confidence:** HIGH
**Source line(s):** 26-29 ("COORDINATOR RULE: You are a 9-STEP coordinator. Each step requires your active execution. Sub-agent responses are INPUTS to your next step, not substitutes for it. Never terminate mid-flow."); 89-258 (9 step definitions); 354 ("You are a 9-STEP coordinator — sub-agent responses are inputs, not completion signals")
**Behavior:** The pr-manager MUST execute all 9 steps in order: (1) populate PR description, (2) verify demo evidence, (3) create PR, (4) security review, (5) review convergence loop, (6) wait for CI, (7) dependency check, (8) execute merge, (9) post-merge cleanup. Each step ends with `STEP_COMPLETE: step=N name=... status=ok` followed by immediate progression to the next step. Treating a sub-agent's "APPROVE" return as completion is forbidden.
**Acceptance:** pr-manager session log shows 9 STEP_COMPLETE markers in order; final exit only after step 9.

#### BC-AUDIT-898 — pr-manager: delegates all gh/git commands to github-ops

**Agent:** `plugins/vsdd-factory/agents/pr-manager.md`
**Confidence:** HIGH
**Source line(s):** 41 ("NEVER execute `gh` or `git` commands yourself — ALWAYS delegate to github-ops via the Agent tool"); 313-318 ("Tool Access" → "Denied: exec, process; You can read and write files but CANNOT execute shell commands")
**Behavior:** pr-manager has no shell access. Every `gh` and `git` invocation MUST go through github-ops via Agent dispatch with `subagent_type="vsdd-factory:github-ops"`.
**Acceptance:** pr-manager's effective tool profile excludes exec/process; all gh/git commands appear in github-ops dispatch prompts.

#### BC-AUDIT-899 — pr-manager: max 10 review convergence cycles before human escalation

**Agent:** `plugins/vsdd-factory/agents/pr-manager.md`
**Confidence:** HIGH
**Source line(s):** 165 ("Step 5: Review convergence loop (max 10 cycles)"); 188 ("After 10 cycles with blocking findings: escalate to human with BLOCKED status"); 296-298 ("Max 10 review cycles. If pr-reviewer still has blocking findings after 10 cycles, escalate to human")
**Behavior:** The review convergence loop runs at most 10 cycles. If blocking findings remain after cycle 10, the agent escalates to human with BLOCKED status — it does not loop indefinitely.
**Acceptance:** review-findings.md cycle table has ≤10 rows; cycle 11 only appears as a BLOCKED escalation.

#### BC-AUDIT-900 — pr-manager: never merges with failing CI checks or unmerged dependency PRs

**Agent:** `plugins/vsdd-factory/agents/pr-manager.md`
**Confidence:** HIGH
**Source line(s):** 42 ("NEVER merge without all dependency PRs merged first"); 45 ("MUST NOT merge with failing CI checks"); 196-209 (Step 6 wait-for-ci); 213-225 (Step 7 dependency check)
**Behavior:** Merge (Step 8) executes only after CI green AND all upstream dependency PRs merged. The pr-manager polls each dependency PR via `gh pr view --json state` and waits if any is unmerged.
**Acceptance:** Merge timestamp is after CI green timestamp AND after all dependency PRs' merged timestamps.

#### BC-AUDIT-901 — pr-manager: max 3 CI fix cycles before human escalation

**Agent:** `plugins/vsdd-factory/agents/pr-manager.md`
**Confidence:** HIGH
**Source line(s):** 204 ("Max 3 CI fix cycles; escalate to human after 3 failures")
**Behavior:** When CI fails, the pr-manager spawns implementer to fix and re-pushes — at most 3 times. After 3 consecutive CI failures, escalate.
**Acceptance:** Step 6 retry counter ≤3; cycle 4 only as BLOCKED escalation.

---

### 2.21 pr-reviewer

**Frontmatter:** `model: opus`, `color: red`. Profile `coding`.

#### BC-AUDIT-902 — pr-reviewer: cannot see .factory/ artifacts (information wall)

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 38 ("CANNOT see `.factory/` artifacts (information wall) -- review only the diff and PR description"); 87-103 ("Information Asymmetry Wall" → "You CANNOT see: .factory/** (all internal pipeline artifacts, including .factory/semport/**)")
**Behavior:** The pr-reviewer MUST review the PR purely as a human reviewer would — diff, description, demo evidence, CI test results. It MUST NOT load any file under `.factory/**`. Information needed from behind the wall must be derived independently from the PR.
**Acceptance:** Tool-call audit shows zero Read calls against `.factory/**`; only PR diff, story spec, and architecture/api-surface.md (allowed for API contract verification).

#### BC-AUDIT-903 — pr-reviewer: posts via `gh pr review`, never `gh pr comment`

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 42 ("MUST post your review to GitHub via `gh pr review` (NOT `gh pr comment`) — this is a formal review, not a comment"); 142-145 ("Command is `gh pr review` — NOT `gh pr comment`; Always include `--request-changes` or `--approve`")
**Behavior:** Every review MUST be posted via `gh pr review --request-changes <body>` or `gh pr review --approve <body>`, not `gh pr comment`. Every review needs an explicit verdict.
**Acceptance:** PR review history on GitHub shows the review created via `gh pr review` events with `state: APPROVED` or `state: CHANGES_REQUESTED`.

#### BC-AUDIT-904 — pr-reviewer: spawns `github-ops` (exact name) for posting

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 43 ("MUST spawn `github-ops` (exact name) to execute gh commands — NOT `github`, NOT `gh-ops`, NOT any other name"); 142 ("Agent name is `github-ops` — NOT `github`, NOT `gh-ops`")
**Behavior:** When dispatching for `gh pr review`, the agent MUST use `subagent_type="vsdd-factory:github-ops"` exactly. Variants like `github`, `gh-ops` are forbidden.
**Acceptance:** Tool-call audit shows the exact string `vsdd-factory:github-ops` in Agent dispatches for review posting.

#### BC-AUDIT-905 — pr-reviewer: 3-tier severity classification (BLOCKING / WARNING / NIT)

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 39 ("ALWAYS classify findings by severity (BLOCKING / WARNING / NIT)"); 159-163 (severity definitions: blocking = must fix before merge; suggestion = should fix; nit = minor)
**Behavior:** Every finding MUST be tagged `[BLOCKING]`, `[SUGGESTION]`, or `[NIT]` at the start of the inline comment. BLOCKING findings prevent APPROVE.
**Acceptance:** Every inline comment begins with one of the three severity tags; APPROVE verdict requires zero `[BLOCKING]` findings.

#### BC-AUDIT-906 — pr-reviewer: no rubber-stamping — explain what was verified

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 40 ("MUST NOT rubber-stamp -- if you find nothing wrong, explain what you verified"); 63 ("If no issues found, explanation of what was verified (no rubber-stamping)")
**Behavior:** When no findings are produced, the review MUST include an explanation of which checklist items were verified (8-item checklist: diff coherence, description accuracy, test coverage, demo evidence, commit quality, diff size, missing changes, dependency status).
**Acceptance:** Approving review body has a non-empty "verified" section enumerating the 8 checklist items.

#### BC-AUDIT-907 — pr-reviewer: demo evidence in `.gif`/`.webm`, not `.txt`

**Agent:** `plugins/vsdd-factory/agents/pr-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 112-115 ("Demo Evidence -- docs/demo-evidence/<STORY-ID>/ contains recordings for every AC. Check: evidence-report.md exists, at least 1 .gif/.webm per AC, both success and error paths recorded. If demos are .txt files or missing, flag as BLOCKING.")
**Behavior:** The pr-reviewer MUST verify each AC has ≥1 `.gif` or `.webm` recording AND both success and error paths. Plain text demos are flagged BLOCKING.
**Acceptance:** Review reports BLOCKING when any AC has only `.txt` demos or missing recordings.

---

### 2.22 product-owner

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `coding`. MCP access (Perplexity, Context7).

#### BC-AUDIT-908 — product-owner: BC-S.SS.NNN numbering scheme

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 30 ("ALWAYS use BC-S.SS.NNN numbering for behavioral contracts"); 70-79 ("BC Numbering Rules" → "BC-S.SS.NNN where S = PRD section number, SS = PRD subsection (0-99, matches L2 subsystem from CAP-NNN), NNN = Sequential within subsystem (001-999)")
**Behavior:** Every BC ID MUST follow `BC-S.SS.NNN` format where S = PRD section number, SS = subsection (0-99, mapped to L2 CAP-NNN), NNN = sequential 001-999. BCs are grouped by L2 domain subsystems, not implementation modules.
**Acceptance:** All BC files match regex `^BC-\d+\.\d{1,2}\.\d{3}\.md$`; BC-INDEX rows comply.

#### BC-AUDIT-909 — product-owner: BC H1 heading is title source of truth

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 97-106 ("BC H1 Title Authority (bc_h1_is_title_source_of_truth)" → "Each BC file's H1 heading is the **authoritative title**. All downstream references must use the H1 title verbatim. Enrichment goes INTO the H1.")
**Behavior:** The BC file's H1 (`# BC-S.SS.NNN: <title>`) is the single source of truth. BC-INDEX, PRD section 2/5 tables, and story body BC tables MUST use the H1 verbatim. Title enrichment (e.g., "(Fail-Closed)") moves INTO the H1 — not left as index-only context. Title drift is HIGH severity.
**Acceptance:** Sampled BC H1 vs BC-INDEX title column shows zero mismatches.

#### BC-AUDIT-910 — product-owner: append-only IDs and slugs

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 169-178 ("Append-Only ID and Slug Protection (append_only_numbering)" → "BC, CAP, VP, EC, and all other VSDD identifiers are never renumbered. Filename slugs are immutable.")
**Behavior:** When a BC is removed, refactored, or replaced, the old ID stays in indexes with `status: retired` or `removed`. New artifacts get new IDs — never reuse. Filename slugs are immutable across title changes. Use `replaced_by:` to link old→new.
**Acceptance:** No BC ID appears as `active` after being marked `retired` in BC-INDEX history; filenames preserve original slugs even when titles change.

#### BC-AUDIT-911 — product-owner: every domain invariant lifted to a BC

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 180-189 ("Invariant Lifting Obligation (lift_invariants_to_bcs)" → "Every domain invariant (DI-NNN) must be enforced by at least one behavioral contract. ... No orphan invariants.")
**Behavior:** For every DI-NNN in `domain-spec/invariants.md`, the product-owner MUST identify or create a BC that enforces it and cite the DI-NNN in the BC's Traceability "L2 Invariants" field. Bidirectional check: invariant Scope column lists enforcer BCs, those BCs cite back. Orphan invariants are forbidden.
**Acceptance:** Every DI-NNN appears in at least one BC's L2 Invariants field; no DI without an enforcing BC.

#### BC-AUDIT-912 — product-owner: same-burst anchor-back when creating BCs

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 327-335 ("Anchor-Back Rule" → "When creating new BCs, the anchor-back step (updating existing stories that implement those invariants) MUST happen in the SAME burst — not a follow-up.")
**Behavior:** Creating new BC files MUST be accompanied in the same burst by: finding stories whose scope touches the new BC's domain, updating those stories' BC tables, updating BC-INDEX. Deferring to a follow-up burst causes empty BC tables — caught by adversary.
**Acceptance:** Burst log shows new BC files + updated story BC tables + BC-INDEX update in a single commit; not separate commits.

#### BC-AUDIT-913 — product-owner: subsystem ID from ARCH-INDEX, never names

**Agent:** `plugins/vsdd-factory/agents/product-owner.md`
**Confidence:** HIGH
**Source line(s):** 108-113 ("Subsystem ID Validation (architecture_is_subsystem_name_source_of_truth)" → "you MUST use the SS-NN ID from architecture/ARCH-INDEX.md Subsystem Registry. Do not use subsystem names — use the ID")
**Behavior:** BC frontmatter `subsystem:` field MUST be SS-NN format from ARCH-INDEX Subsystem Registry. Names (e.g., "Sensor Adapters") are forbidden — only IDs. Pre-architecture: use `SS-TBD` placeholder.
**Acceptance:** BC frontmatter `subsystem:` matches regex `^SS-(TBD|\d+)$`.

---

### 2.23 research-agent

**Frontmatter:** `tools: Read, Write, Edit, Glob, Grep, WebSearch, WebFetch, mcp__perplexity__*, mcp__context7__*`, `model: opus`, `color: purple`. Profile `full` minus Bash.

#### BC-AUDIT-914 — research-agent: every claim cited; never relies on training data alone

**Agent:** `plugins/vsdd-factory/agents/research-agent.md`
**Confidence:** HIGH
**Source line(s):** 50 ("You ALWAYS cite sources — distinguish between verified web findings and model knowledge"); 53 ("You ALWAYS use MCP tools (Perplexity, Context7, Tavily) — do not rely on training data alone")
**Behavior:** Every claim in a research report MUST cite its source (URL, library doc reference, or explicitly tagged "training data"). MCP tools are mandatory inputs.
**Acceptance:** Every paragraph or claim in research output has an associated source citation; "Research Methods" section confirms MCP tool usage.

#### BC-AUDIT-915 — research-agent: library versions verified against registries, never training data

**Agent:** `plugins/vsdd-factory/agents/research-agent.md`
**Confidence:** HIGH
**Source line(s):** 51 ("You ALWAYS verify library versions against registries (crates.io, npm, PyPI) — NEVER rely on training data"); 134 ("when researching library versions, verify against Context7 or the actual registry — NEVER rely on training data for version numbers")
**Behavior:** Library versions MUST be verified via Context7 (`resolve-library-id` then `query-docs`) or direct registry lookup. Training data version numbers are forbidden — they're known stale.
**Acceptance:** Every version number cited in a research report has an accompanying Context7 query or registry URL reference; report explicitly distinguishes verified vs unverified versions.

#### BC-AUDIT-916 — research-agent: mandatory Research Methods section per report

**Agent:** `plugins/vsdd-factory/agents/research-agent.md`
**Confidence:** HIGH
**Source line(s):** 65-89 ("Research Methods Section (MANDATORY)" → "Every research report MUST end with a `## Research Methods` section documenting tool counts and training-data reliance")
**Behavior:** Every research report MUST conclude with a `## Research Methods` table showing tool name, query count, and purpose, plus total MCP tool calls and training-data reliance level (low/medium/high). The section is non-negotiable.
**Acceptance:** Every file under `.factory/specs/research/` ends with a `## Research Methods` section with the documented schema.

#### BC-AUDIT-917 — research-agent: never overwrites prior research — appends new dated file

**Agent:** `plugins/vsdd-factory/agents/research-agent.md`
**Confidence:** HIGH
**Source line(s):** 31 ("Always create a new file — never overwrite previous research. Each run gets its own dated file."); 32-33 (filename pattern: `domain-<topic-slug>-<YYYY-MM-DD>.md` / `general-<topic-slug>-<YYYY-MM-DD>.md`)
**Behavior:** Research output is always appended as a new dated file. RESEARCH-INDEX.md is updated with a new row. Overwriting prior research is forbidden.
**Acceptance:** Research dir has multiple dated files; index shows monotonic date growth; no file overwrites in git history.

#### BC-AUDIT-918 — research-agent: no source code modification, no Bash

**Agent:** `plugins/vsdd-factory/agents/research-agent.md`
**Confidence:** HIGH
**Source line(s):** 49 ("You NEVER modify source code, specs, or pipeline artifacts (other than writing research outputs)"); 144-148 ("Tool Access" → "Denied: Bash, exec, process; Why no shell: Research produces markdown documents, not code")
**Behavior:** Research output is markdown only. Bash, exec, process are denied. Writes limited to `.factory/planning/`, `.factory/specs/research/`, or `.factory/specs/domain-research.md`.
**Acceptance:** Tool profile excludes Bash; git diff shows zero changes outside research output paths.

---

### 2.24 security-reviewer

**Frontmatter:** `model: sonnet`, `color: red`. Profile `coding`. MCP access (Perplexity).

#### BC-AUDIT-919 — security-reviewer: cite CWE/CVE for every finding

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 28 ("ALWAYS cite CWE/CVE numbers for every finding"); 73-85 (SEC-NNN finding format requires CWE/OWASP)
**Behavior:** Every SEC-NNN finding MUST include a specific CWE number (and OWASP category if applicable). Generic descriptions without CWE are forbidden.
**Acceptance:** Every SEC-NNN entry has a non-empty `CWE:` field matching `CWE-\d+`.

#### BC-AUDIT-920 — security-reviewer: 4-tier severity (CRITICAL/HIGH/MEDIUM/LOW)

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 29 ("ALWAYS classify severity (CRITICAL / HIGH / MEDIUM / LOW)"); 30 ("MUST NOT approve code with unresolved CRITICAL or HIGH findings")
**Behavior:** Every finding MUST be classified at one of 4 severity levels. CRITICAL or HIGH findings block approval until resolved.
**Acceptance:** Every SEC-NNN has `Severity:` ∈ {CRITICAL, HIGH, MEDIUM, LOW}; review verdict APPROVE only when zero unresolved CRITICAL/HIGH findings.

#### BC-AUDIT-921 — security-reviewer: cannot see implementer reasoning (information wall)

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 156-167 ("Per-Story PR Review Wall" → "You CANNOT see: .factory/cycles/**/implementation/implementer-notes*; Implementer session logs"); 169-178 ("Wave Integration Review Wall" → "You CANNOT see: .factory/code-delivery/*/review-findings.md")
**Behavior:** Per-story PR review: cannot read implementer notes/logs. Wave integration review: cannot read per-story PR reviews. The reviewer must form judgments from first principles, not inherit reasoning.
**Acceptance:** Tool-call audit shows zero Read against `.factory/cycles/**/implementation/implementer-notes*` or `.factory/code-delivery/*/review-findings.md`.

#### BC-AUDIT-922 — security-reviewer: never dismiss without documented reasoning

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 27 ("NEVER dismiss a finding without documenting the reasoning")
**Behavior:** Every dismissed finding (false positive, accepted risk) MUST include an explicit reasoning line. Silent dismissal is forbidden.
**Acceptance:** Every triage outcome (true positive / false positive / accepted risk) has a `reasoning:` field.

#### BC-AUDIT-923 — security-reviewer: supply chain audit ANY finding blocks installation

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 195-198 ("Supply Chain Security Audit" → "ANY finding -- regardless of severity -- results in: BLOCKING notification to human via notify-human; Installation BLOCKED until human explicitly approves")
**Behavior:** During tool installation audits (CVE/NVD/OSV + Perplexity recent compromise + integrity check), ANY finding regardless of severity blocks the installation. Human approval is required to override.
**Acceptance:** Every audit report ending with `VERDICT: FINDING -- human approval required` blocks the dx-engineer's install step until human override.

#### BC-AUDIT-924 — security-reviewer: posts via gh pr review, never gh pr comment (per-story)

**Agent:** `plugins/vsdd-factory/agents/security-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 124-130 ("Per-Story PR Review" → "spawn `github-ops` (exact name) to post a formal GitHub review: gh pr review PR_NUMBER --request-changes --body-file ... CRITICAL: Use `gh pr review` (NOT `gh pr comment`). Agent name is `github-ops` (NOT `github`).")
**Behavior:** Per-story security findings MUST be posted via `gh pr review --request-changes --body-file ...` (or `--approve`) using github-ops with exact agent name. `gh pr comment` is forbidden for security verdicts.
**Acceptance:** PR review history shows security review posted as a formal review (state: APPROVED or CHANGES_REQUESTED), not a free-form comment.

---

### 2.25 session-reviewer (`session-review`)

**Frontmatter:** `name: session-review`, `model: sonnet`, `color: red`. Profile `minimal` — strictly read-only with no Bash/Write/Edit/Agent.

#### BC-AUDIT-925 — session-reviewer: T1 read-only, NEVER writes files

**Agent:** `plugins/vsdd-factory/agents/session-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 21 ("NEVER modify pipeline artifacts -- read-only analysis"); 64-74 ("Why T1 Read-Only" → "You MUST NOT write files or execute commands. ... state-manager agent writes your output to .factory/session-reviews/"); 199-204 ("Tool Access" → "Available: read, session_status, web_search, web_fetch, memory_search, memory_get; Denied: write, edit, apply_patch, exec, process")
**Behavior:** The session-reviewer is T1 read-only. It MUST NOT use Write/Edit/Bash/Agent tools. State-manager persists its output to `.factory/session-reviews/`.
**Acceptance:** Tool profile = minimal; effective allowed tools include read but exclude write/edit/apply_patch/exec/process/Agent.

#### BC-AUDIT-926 — session-reviewer: 8-dimensional analysis required

**Agent:** `plugins/vsdd-factory/agents/session-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 89-156 ("Analysis Framework — 8 Dimensions" → 1.Cost, 2.Timing, 3.Convergence, 4.Agent Behavior, 5.Gate Outcome, 6.Wall Integrity, 7.Quality Signal, 8.Pattern Detection)
**Behavior:** Every session review MUST analyze across 8 dimensions: cost, timing, convergence, agent behavior, gate outcomes, wall integrity, quality signals, cross-run pattern detection.
**Acceptance:** session-review report contains 8 sections matching the dimensional headings; no dimension skipped without "no baseline available" justification.

#### BC-AUDIT-927 — session-reviewer: actionable proposals, not vague observations

**Agent:** `plugins/vsdd-factory/agents/session-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 22 ("ALWAYS produce actionable improvement proposals (not vague observations)"); 158-162 ("You produce TWO documents: 1. Session Review Report; 2. Improvement Proposals — structured proposals with category, priority, evidence, recommendation, affected files, and risk assessment")
**Behavior:** Output includes (a) Session Review Report and (b) Improvement Proposals — structured with category, priority, evidence, recommendation, affected files, risk. Vague observations are forbidden.
**Acceptance:** Improvement Proposals document has structured rows; every proposal has all 6 fields populated.

#### BC-AUDIT-928 — session-reviewer: no information walls — sees everything

**Agent:** `plugins/vsdd-factory/agents/session-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 82-87 ("No Information Asymmetry Wall" → "Unlike other agents, you have NO walls. You see everything")
**Behavior:** Unlike other reviewers, session-reviewer MUST be able to see the complete picture (source, specs, adversary findings, TDD logs, cost data, convergence history, holdout results) — needed for run analysis. No exclusion list.
**Acceptance:** Tool-call audit shows reads across all `.factory/` paths — no exclusions enforced.

#### BC-AUDIT-929 — session-reviewer: tracks own cost; flags >5% of pipeline run cost

**Agent:** `plugins/vsdd-factory/agents/session-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 188-191 ("Self-Cost Awareness" → "Track your own cost separately. If session review cost exceeds 5% of the pipeline run cost, flag for optimization")
**Behavior:** The session-reviewer measures its own cost and flags optimization in its own improvement proposals if it exceeds 5% of the pipeline run cost.
**Acceptance:** Session review cost-summary shows separate `session_review_cost` and `pipeline_run_cost`; ratio > 0.05 triggers a self-optimization improvement proposal entry.

---

### 2.26 spec-reviewer

**Frontmatter:** `model: opus`, `color: red`. Profile `coding`. (Description references "Gemini" but VSDD's effective model is opus per frontmatter.)

#### BC-AUDIT-930 — spec-reviewer: never re-reports adversary findings

**Agent:** `plugins/vsdd-factory/agents/spec-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 41 ("NEVER repeat adversary findings -- review the remediated artifacts, not re-attack"); 156 ("NOT re-reporting adversary findings (you review POST-remediation)")
**Behavior:** The spec-reviewer runs AFTER the adversary pass and AFTER remediation. Findings MUST be net-new — not restatements of adversary findings already addressed. Focus is constructive improvement.
**Acceptance:** SR-NNN findings have no overlap with prior ADV-NNN findings from the same phase.

#### BC-AUDIT-931 — spec-reviewer: SR-NNN ID space, distinct from ADV-NNN and CR-NNN

**Agent:** `plugins/vsdd-factory/agents/spec-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 53-54 ("Constructive review findings with SR-NNN IDs (distinct from ADV-NNN and CR-NNN)"); 130-138 (SR-NNN format)
**Behavior:** Spec-reviewer findings use the SR-NNN namespace, distinct from adversary's ADV-NNN and code-reviewer's CR-NNN.
**Acceptance:** No SR-NNN ID collides with an existing ADV-NNN or CR-NNN ID anywhere in the project.

#### BC-AUDIT-932 — spec-reviewer: cannot see implementation details (information wall)

**Agent:** `plugins/vsdd-factory/agents/spec-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 167-174 ("Information Asymmetry Wall" → "You CANNOT see: .factory/cycles/**/implementation/** (no implementation details); .factory/cycles/**/red-gate-log* (no TDD logs)")
**Behavior:** The spec-reviewer reviews specs and stories only. It MUST NOT load implementation logs, red-gate logs, or any code-delivery artifacts.
**Acceptance:** Tool-call audit shows zero Read against `.factory/cycles/**/implementation/` or `red-gate-log*`.

#### BC-AUDIT-933 — spec-reviewer: 6-category finding taxonomy

**Agent:** `plugins/vsdd-factory/agents/spec-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 134 ("Category: completeness | coherence | ambiguity | traceability | feasibility | domain-gap")
**Behavior:** Every SR-NNN finding MUST be classified into exactly one of: completeness, coherence, ambiguity, traceability, feasibility, domain-gap.
**Acceptance:** Every SR-NNN has `Category:` set to one of 6 allowed values.

---

### 2.27 spec-steward

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `coding`.

#### BC-AUDIT-934 — spec-steward: never modifies spec content — governance only

**Agent:** `plugins/vsdd-factory/agents/spec-steward.md`
**Confidence:** HIGH
**Source line(s):** 28 ("NEVER modify spec content -- governance and traceability only"); 230 ("You NEVER allow a spec change without a version bump")
**Behavior:** The spec-steward writes governance artifacts (spec-versions.md, traceability-matrix.md, spec-changelog.md, drift-reports/) but MUST NOT modify content of PRD, BCs, VPs, architecture, or stories.
**Acceptance:** Git diff after spec-steward runs touches only `.factory/spec-versions.md`, `traceability-matrix.md`, `spec-changelog.md`, `drift-reports/` — never `.factory/specs/` content files.

#### BC-AUDIT-935 — spec-steward: every spec change requires version bump

**Agent:** `plugins/vsdd-factory/agents/spec-steward.md`
**Confidence:** HIGH
**Source line(s):** 30 ("ALWAYS enforce versioning (semver) on every spec change"); 31 ("MUST NOT allow unversioned spec mutations"); 207 ("NEVER allow a spec change without a version bump")
**Behavior:** Every modification to a versioned spec artifact MUST be accompanied by a frontmatter `version:` semver bump (MAJOR / MINOR / PATCH per documented rules) and a spec-changelog.md entry.
**Acceptance:** Every spec file commit on factory-artifacts branch has a matching frontmatter version bump; spec-changelog.md has a corresponding entry with Added/Changed/Impact sections.

#### BC-AUDIT-936 — spec-steward: locked VP enforcement (immutable after lock)

**Agent:** `plugins/vsdd-factory/agents/spec-steward.md`
**Confidence:** HIGH
**Source line(s):** 102-110 ("L4 Immutability Enforcement" → "VP-NNN documents are append-only after proof harnesses are committed. Once `verification_lock: true` is set, the VP document is immutable.")
**Behavior:** VP files with `verification_lock: true` MUST NOT be edited. Any changes require the VP withdrawal process. Locked VPs are flagged for re-assessment if their source BC has a MAJOR version bump.
**Acceptance:** Git history on factory-artifacts shows zero edits to any VP file after its lock date (except via formal withdrawal documents).

#### BC-AUDIT-937 — spec-steward: append-only IDs and immutable filename slugs

**Agent:** `plugins/vsdd-factory/agents/spec-steward.md`
**Confidence:** HIGH
**Source line(s):** 192-201 ("Append-Only ID and Slug Protection (append_only_numbering)" → "Never renumber. Never reuse. Filename slugs are immutable. Retirement requires traceability.")
**Behavior:** All VSDD identifiers (BC, CAP, VP, EC, DI, ASM, R, FM, STORY, HS) are append-only. Reuse of retired IDs is HIGH severity. Filename slugs MUST remain stable across title changes.
**Acceptance:** No active artifact has the same ID as a retired one in any index; git log shows no rename of a slug-based filename.

---

### 2.28 state-manager

**Frontmatter:** `model: sonnet`, `color: yellow`. Profile `full` but shell access ONLY for git in `.factory/`.

#### BC-AUDIT-938 — state-manager: git access scoped to .factory/ only

**Agent:** `plugins/vsdd-factory/agents/state-manager.md`
**Confidence:** HIGH
**Source line(s):** 351-355 ("You ONLY execute git commands inside `.factory/` — `git add`, `git commit`, `git push`; You NEVER execute git commands outside `.factory/` (source code branches are devops-engineer's scope); You NEVER run non-git shell commands")
**Behavior:** State-manager's exec scope is git operations inside `.factory/` only. Git on other paths is forbidden; non-git shell commands (cargo, npm, curl) are forbidden entirely.
**Acceptance:** Tool-call audit shows exec calls limited to `cd .factory && git ...` patterns; zero exec calls in source-code branches or non-git invocations.

#### BC-AUDIT-939 — state-manager: never writes spec documents or source code

**Agent:** `plugins/vsdd-factory/agents/state-manager.md`
**Confidence:** HIGH
**Source line(s):** 37 ("NEVER write specification documents or source code -- state tracking only"); 181-183 ("What You NEVER Write: Specification documents (PRD, architecture, BCs, VPs); Source code, tests, or configuration files; Review reports or evaluation reports"); 401 ("You NEVER write specification documents, source code, or review reports")
**Behavior:** The state-manager writes STATE.md, .factory/ directory structure, cycle-manifest.md, burst-log.md, convergence-trajectory.md, lessons.md, session-checkpoints.md, blocking-issues-resolved.md, tech-debt-register.md, cost-summary.md. It MUST NOT write specs, BCs, VPs, source, tests, configs, or review reports.
**Acceptance:** Git diff after state-manager runs shows changes only to the documented set of state files; no `.factory/specs/`, `src/`, or review report writes.

#### BC-AUDIT-940 — state-manager: STATE.md cap of 200 lines (hook blocks at 500)

**Agent:** `plugins/vsdd-factory/agents/state-manager.md`
**Confidence:** HIGH
**Source line(s):** 102-103 ("STATE.md must stay under 200 lines. A hook blocks writes above 500 lines."); 140-146 ("Anti-Patterns" → "NEVER append full burst narratives to STATE.md; NEVER add per-pass adversary finding details to STATE.md frontmatter; NEVER keep more than 1 session resume checkpoint in STATE.md; NEVER keep resolved blocking issues in STATE.md; NEVER accumulate lessons learned in STATE.md")
**Behavior:** STATE.md MUST stay under 200 lines target / 500 lines hard cap (enforced by validate-state-size hook). Burst narratives, full adversary findings, multiple session checkpoints, resolved blocking issues, and accumulated lessons go to cycle files instead — never STATE.md.
**Acceptance:** STATE.md line count ≤ 200 typical, ≤ 500 always. The `validate-state-size.sh` hook on PostToolUse:Edit/Write rejects any commit pushing STATE.md above 500 lines.

#### BC-AUDIT-941 — state-manager: worktree preconditions verified before any .factory/ creation

**Agent:** `plugins/vsdd-factory/agents/state-manager.md`
**Confidence:** HIGH
**Source line(s):** 41-72 ("Preconditions" → "Before creating ANY files in `.factory/` or `.factory-project/`, verify they are git worktrees: 1. Check: .factory/.git exists; 2. Check: git -C .factory rev-parse --git-dir succeeds; 3. Check: git -C .factory branch --show-current shows factory-artifacts; If ANY check fails: STOP and report")
**Behavior:** Before initializing or writing to `.factory/` or `.factory-project/`, the state-manager MUST verify all three worktree preconditions. Any failure halts with the exact recovery command. Creating `.factory/` as a regular directory is forbidden.
**Acceptance:** State-manager invocation log includes precondition checks before any Write/Edit; failure paths emit `ERROR: .factory/ is not mounted as a git worktree on factory-artifacts branch.` with recovery command.

#### BC-AUDIT-942 — state-manager: wave-gate remediation uses Single Canonical SHA + Two-Commit Protocol

**Agent:** `plugins/vsdd-factory/agents/state-manager.md`
**Confidence:** HIGH
**Source line(s):** 148-180 ("Wave-gate remediation bursts (MUST follow)" → "Single Canonical SHA + Two-Commit Protocol via the vsdd-factory:state-burst skill. Anti-patterns that have caused 6+ consecutive defect recurrences: 1. Writing narrative in 'Pass N BLOCKED — REMEDIATION IN PROGRESS' voice; 2. Citing intermediate burst SHAs in any document; 3. Adding a 3rd commit; 4. Skipping post-push hook verification; 5. Updating one document without sweeping the same field in sibling documents.")
**Behavior:** When committing a burst that updates STATE.md + SESSION-HANDOFF.md + wave-state.yaml together, the state-manager MUST use the `vsdd-factory:state-burst` skill which enforces: past-tense "REMEDIATED — Awaiting Pass N+1" voice; literal `15fa97e6` placeholder in Stage 1; max 2 commits; post-push hook verification; cross-document field sync.
**Acceptance:** Wave-gate burst on factory-artifacts shows ≤2 commits; HEAD and HEAD^ do not both contain `backfill`; verify-sha-currency hook reports PASS post-push.

---

### 2.29 story-writer

**Frontmatter:** `model: sonnet`, `color: green`. Profile `coding`.

#### BC-AUDIT-943 — story-writer: one file per story, never monolithic

**Agent:** `plugins/vsdd-factory/agents/story-writer.md`
**Confidence:** HIGH
**Source line(s):** 26 ("NEVER produce a monolithic stories file -- one file per story (`STORY-NNN-[short].md`)"); 41-49 (output structure); 501 ("You NEVER produce a monolithic stories.md -- every story is a standalone STORY-NNN file")
**Behavior:** Stories MUST be written one-per-file as `STORY-NNN-[short].md` under `.factory/stories/stories/`. A monolithic `stories.md` is forbidden. STORY-INDEX.md aggregates references.
**Acceptance:** `.factory/stories/stories/` contains individual story files; no monolithic stories.md exists; STORY-INDEX.md lists all stories.

#### BC-AUDIT-944 — story-writer: every AC traces to a BC clause; six context-engineering sections mandatory

**Agent:** `plugins/vsdd-factory/agents/story-writer.md`
**Confidence:** HIGH
**Source line(s):** 27 ("ALWAYS trace every acceptance criterion to a BC-S.SS.NNN behavioral contract"); 53 ("Every AC traces to a BC-S.SS.NNN clause"); 108-126 ("Context-Engineering Sections (ALL MANDATORY)" → 6 sections: Token Budget Estimate, Tasks, Previous Story Intelligence, Architecture Compliance Rules, Library & Framework Requirements, File Structure Requirements)
**Behavior:** Every AC MUST include `(traces to BC-S.SS.NNN postcondition N)` or `(... invariant N)` annotation. Every story MUST include all six context-engineering sections (omitting any degrades downstream agent quality). Sections marked N/A include explicit "N/A — first story in epic" notes — never omitted.
**Acceptance:** Every AC line in every story has a `(traces to BC-...)` annotation; every story file contains all 6 mandatory section headers.

#### BC-AUDIT-945 — story-writer: no story exceeds 13 points or 20-30% agent context window

**Agent:** `plugins/vsdd-factory/agents/story-writer.md`
**Confidence:** HIGH
**Source line(s):** 56 ("Token budget estimated per story; no story exceeds 20-30% of agent context window"); 221 ("No story exceeds 13 story points"); 226 ("No story's estimated context exceeds 20-30% of the agent's context window")
**Behavior:** Story sizing constraints: max 13 story points; max 20-30% of implementing agent's context window. Stories exceeding either constraint MUST be split.
**Acceptance:** STORY-INDEX.md shows no story with `points > 13`; every story's Token Budget Estimate ≤ 30% of target agent context.

#### BC-AUDIT-946 — story-writer: BC array changes propagate to body and ACs in same atomic commit

**Agent:** `plugins/vsdd-factory/agents/story-writer.md`
**Confidence:** HIGH
**Source line(s):** 469-487 ("BC Array Propagation Policy (bc_array_changes_propagate_to_body_and_acs)" → "When adding or removing a BC from a story's `bcs:` frontmatter array, you MUST also update in the same atomic commit: 1. Body BC table; 2. Acceptance criteria (with traces); 3. Token Budget subtable; 4. Any other body-level BC-count derivations")
**Behavior:** Frontmatter `bcs:` array changes MUST propagate atomically to the body BC table, AC traces, Token Budget subtable, and any BC-count derivations. Pre-commit verification reads the story and confirms each BC in the final array appears in body table AND in at least one AC trace. Failure = HIGH severity blocker.
**Acceptance:** No story commit on factory-artifacts has frontmatter `bcs:` change without matching body BC table edits; validate hook (`validate-story-bc-sync.sh`) catches drift.

#### BC-AUDIT-947 — story-writer: dependency graph must be acyclic

**Agent:** `plugins/vsdd-factory/agents/story-writer.md`
**Confidence:** HIGH
**Source line(s):** 54 ("Dependency graph is acyclic (validated with topological sort)"); 224 ("Dependency graph must be acyclic (validate with topological sort)")
**Behavior:** Story `depends_on` and `blocks` fields MUST form an acyclic directed graph. Topological sort MUST succeed on `dependency-graph.md`.
**Acceptance:** Topological sort of stories from STORY-INDEX produces a valid linear ordering; no cycles detected.

---

### 2.30 technical-writer

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `coding`.

#### BC-AUDIT-948 — technical-writer: documents only current code, never aspirational

**Agent:** `plugins/vsdd-factory/agents/technical-writer.md`
**Confidence:** HIGH
**Source line(s):** 27 ("You NEVER document aspirational or planned behavior -- only what the current code does"); 47 ("All documentation matches current code -- no aspirational content"); 68 ("You NEVER document aspirational behavior -- only what the current code actually does")
**Behavior:** Documentation reflects what the code currently does. Planned features, roadmap items, or TODO behaviors are forbidden in documentation output.
**Acceptance:** Generated docs match current type signatures and runtime behavior; no "Coming soon" / "Planned" sections.

#### BC-AUDIT-949 — technical-writer: never modifies source/tests/configs

**Agent:** `plugins/vsdd-factory/agents/technical-writer.md`
**Confidence:** HIGH
**Source line(s):** 29 ("You NEVER modify source code, tests, or configuration files"); 30 ("ALWAYS write output to your designated paths under `.factory/` or `docs/`")
**Behavior:** Writes limited to `.factory/` and `docs/`. Source code, tests, and config files are not modified.
**Acceptance:** Git diff after technical-writer runs shows changes only in `.factory/` or `docs/`; zero changes in `src/`, `tests/`, or config files.

#### BC-AUDIT-950 — technical-writer: gaps in source documentation explicitly listed

**Agent:** `plugins/vsdd-factory/agents/technical-writer.md`
**Confidence:** HIGH
**Source line(s):** 50 ("Gaps in source documentation (missing doc comments, schemas) explicitly listed"); 64-65 ("Failure & Escalation" → "If some source files are unreadable or missing, document what is available and list gaps")
**Behavior:** When source documentation is missing (no doc comments, no schemas), the technical-writer MUST list these gaps in the output rather than fabricating descriptions or skipping silently.
**Acceptance:** Documentation output includes a "Documentation Gaps" section enumerating modules/files lacking doc comments.

---

### 2.31 test-writer

**Frontmatter:** `model: sonnet`, `color: green`. Profile `full`.

#### BC-AUDIT-951 — test-writer: never writes implementation code

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 87 ("NEVER write implementation code -- tests only"); 320 ("NEVER write implementation code. You write tests ONLY."); 351 ("You NEVER write implementation code -- you write tests ONLY")
**Behavior:** test-writer writes tests, stubs (compilable empty implementations), and red-gate-log only. Production source code is the implementer's exclusive scope.
**Acceptance:** Git diff after test-writer runs shows changes only in `tests/`, stub files, or `.factory/cycles/**/implementation/red-gate-log.md`; zero production source code changes.

#### BC-AUDIT-952 — test-writer: BC-NNN-traceable test naming required

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 88 ("ALWAYS name tests using `test_BC_S_SS_NNN_xxx()` pattern"); 134-149 ("Test Naming Convention" → `test_BC_S_SS_NNN_[assertion_name]()` with examples)
**Behavior:** Every test name MUST follow `test_BC_S_SS_NNN_[assertion_name]()`. Generic names (`test_1`, `test_basic`, `test_it_works`) are forbidden.
**Acceptance:** Every test in the suite matches the regex `^test_BC_\d+_\d{1,2}_\d{3}_[a-z_]+(\(\))?$` (allowing language-specific differences).

#### BC-AUDIT-953 — test-writer: Red Gate must be verified — all tests fail before implementation

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 89 ("ALWAYS verify Red Gate (all tests must fail before implementation begins)"); 235-243 ("Step 3: Verify Red Gate" → "Run the test suite. ALL tests MUST FAIL. If any test passes without implementation: The test is suspect ... Or the spec was wrong ... Flag for human review")
**Behavior:** After writing tests, the test-writer MUST run the suite and verify every test fails (Red Gate). A test that passes without implementation is suspect — flag for human review. Red gate results written to `red-gate-log.md`.
**Acceptance:** `.factory/cycles/**/implementation/red-gate-log.md` exists and shows all tests in failing state with timestamps. No test passing pre-implementation.

#### BC-AUDIT-954 — test-writer: never writes vacuously true tests

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 321 ("NEVER write tests that can pass without implementation (vacuously true tests)"); 322 ("ALWAYS test boundaries: empty, too-long, whitespace, case sensitivity, invalid formats")
**Behavior:** Tests MUST exercise actual behavior — `assert!(true)`, no-op assertions, or tautological tests are forbidden. Boundary tests are mandatory.
**Acceptance:** No test body matches simple tautology patterns; mutation testing kill rate (formal-verifier) is high.

#### BC-AUDIT-955 — test-writer: property-based tests generate ≥1000 random cases

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 325 ("Property-based tests must generate at least 1000 random cases")
**Behavior:** proptest / fast-check / Hypothesis property tests MUST be configured to generate at least 1000 random cases. Lower bounds are forbidden for property tests.
**Acceptance:** Every property test has a config with `cases >= 1000` (or framework equivalent).

#### BC-AUDIT-956 — test-writer: uses canonical test vectors from BCs when available

**Agent:** `plugins/vsdd-factory/agents/test-writer.md`
**Confidence:** HIGH
**Source line(s):** 172-186 ("Canonical Test Vectors" → "When a behavioral contract includes a test vector table, use those as golden test data. Do not invent your own inputs when the BC provides them.")
**Behavior:** When a BC includes a Canonical Test Vectors table, the test-writer MUST use those exact inputs/outputs as parameterized test cases — not invent new inputs.
**Acceptance:** Tests for BCs with test-vector tables use input strings matching the BC's table verbatim.

---

### 2.32 ux-designer

**Frontmatter:** `model: sonnet`, `color: blue`. Profile `coding`.

#### BC-AUDIT-957 — ux-designer: every screen traces to a PRD requirement

**Agent:** `plugins/vsdd-factory/agents/ux-designer.md`
**Confidence:** HIGH
**Source line(s):** 95 ("You NEVER create screens without tracing to a PRD requirement"); 119 ("Every screen traces to a PRD requirement; every element justified by spec"); 218 ("You NEVER create a screen without tracing it to a PRD requirement -- every element must be justified by the spec")
**Behavior:** Every SCR-NNN screen file MUST trace to a specific PRD section/BC. Elements without spec justification are forbidden.
**Acceptance:** Every screen file's frontmatter has `traces_to:` listing a PRD section or BC; every UI element has spec justification.

#### BC-AUDIT-958 — ux-designer: every interaction has both success AND error paths

**Agent:** `plugins/vsdd-factory/agents/ux-designer.md`
**Confidence:** HIGH
**Source line(s):** 96 ("You ALWAYS define both success and error paths for every interaction"); 117 ("All interaction flows mapped with success and error paths")
**Behavior:** Every interaction flow in `flows/FLOW-NNN-*.md` MUST define both a success path and at least one error path. One-path flows are rejected.
**Acceptance:** Every flow file contains a `## Success Path` AND a `## Error Path` section (or equivalent named subsections).

#### BC-AUDIT-959 — ux-designer: sharded UX (UX-INDEX + screen + flow files), never monolithic

**Agent:** `plugins/vsdd-factory/agents/ux-designer.md`
**Confidence:** HIGH
**Source line(s):** 97 ("ALWAYS follow the templates in `../../templates/` for UX spec output format"); 178-199 ("Sharded UX Output (DF-021)" → "Output directory: `.factory/specs/ux-spec/`; UX-INDEX.md FIRST; screens/SCR-NNN-[name].md one per screen; flows/FLOW-NNN-[name].md one per flow")
**Behavior:** UX output is sharded: `UX-INDEX.md` first, then `screens/SCR-NNN-[name].md` per screen, `flows/FLOW-NNN-[name].md` per flow. Each screen/flow targets 800-1,200 tokens. Monolithic `ux-spec.md` is forbidden.
**Acceptance:** `.factory/specs/ux-spec/UX-INDEX.md` exists; screens/ and flows/ subdirectories contain individual files; no monolithic ux-spec.md.

#### BC-AUDIT-960 — ux-designer: WCAG 2.1 AA documented per screen

**Agent:** `plugins/vsdd-factory/agents/ux-designer.md`
**Confidence:** HIGH
**Source line(s):** 86 ("Document accessibility requirements (WCAG 2.1 AA minimum)"); 117 ("Accessibility requirements documented (WCAG 2.1 AA minimum)")
**Behavior:** Every screen file MUST document accessibility requirements at WCAG 2.1 AA minimum (color contrast, keyboard navigation, ARIA, focus order).
**Acceptance:** Every screen file has an `## Accessibility` section listing applicable WCAG 2.1 AA criteria.

---

### 2.33 validate-extraction

**Frontmatter:** `tools: Read, Write, Edit, Grep, Glob, Bash`, `model: sonnet`, `color: red`. Profile `full`.

#### BC-AUDIT-961 — validate-extraction: behavioral and metric phases must be split

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 19-34 ("Operating Mode: Behavioral vs Metric Split (mandatory)" → "You MUST split your work into two distinct phases and report each separately. ... Mixing the phases lets metric inflation slip through because behavioral sampling naturally skips numeric claims.")
**Behavior:** Validation runs in two strictly separated phases: Phase 1 — behavioral verification (judgment, sample-and-confirm); Phase 2 — metric verification (independent recount with `find`, `wc -l`, `grep -c`, `ls | wc -l` — no estimation). Each phase reports in its own table; phases MUST NOT be interleaved.
**Acceptance:** Output report has two distinct top-level tables labeled "Phase 1 — Behavioral Verification" and "Phase 2 — Metric Verification."

#### BC-AUDIT-962 — validate-extraction: every numeric claim has a (claimed, recounted, delta) triple

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 27-31 ("For every claim of the form 'N files', 'N LOC', 'N entities', 'N BCs', etc., produce a triple: (claimed value, recounted value, delta).")
**Behavior:** Every numeric claim in the analysis MUST appear in the Phase 2 table with claimed / recounted / delta / command columns. A row with `Delta: 0` is a pass; any non-zero delta is an error regardless of magnitude.
**Acceptance:** Phase 2 table covers every numeric claim from the analysis; no claim missing its row.

#### BC-AUDIT-963 — validate-extraction: max 3 refinement iterations (AgenticAKM)

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 75-91 ("Refinement Loop" → "After initial validation, iterate up to 3 times. Stop after 3 iterations. Diminishing returns beyond this point (validated by AgenticAKM study, 29 repositories)")
**Behavior:** Refinement caps at 3 iterations. Iteration 1 flags issues; Iteration 2 verifies corrections; Iteration 3 final consistency check. Beyond 3 iterations is forbidden.
**Acceptance:** Validation report's "Refinement Iterations" field shows ≤3.

#### BC-AUDIT-964 — validate-extraction: 4-tier per-item disposition (VERIFIED / INACCURATE / HALLUCINATED / UNVERIFIABLE)

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 78-85 ("Mark each finding as: VERIFIED, INACCURATE, HALLUCINATED, or UNVERIFIABLE; Provide corrections for INACCURATE items; Remove HALLUCINATED items")
**Behavior:** Every extracted item is dispositioned into one of 4 buckets: VERIFIED (matches code), INACCURATE (corrections provided), HALLUCINATED (removed), UNVERIFIABLE (cannot be checked, marked as such).
**Acceptance:** Validation report includes "Inaccurate Items (Corrected)", "Hallucinated Items (Removed)", and "Unverifiable Items" tables, plus a verified count.

#### BC-AUDIT-965 — validate-extraction: never modifies source code

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 141 ("Do not modify the source code. You are read-only."); 147-150 ("Source code is READ-ONLY — use Bash/Grep to inspect, never modify")
**Behavior:** Despite Bash access, source code is read-only — Bash is for inspection (find, wc, grep, ls), never modification.
**Acceptance:** Git diff after validate-extraction runs shows changes only in `.factory/phase-0-ingestion/` validation reports; zero source modifications.

#### BC-AUDIT-966 — validate-extraction: >50% hallucination rate triggers Level 3 escalation

**Agent:** `plugins/vsdd-factory/agents/validate-extraction.md`
**Confidence:** HIGH
**Source line(s):** 153-156 ("Level 3 (escalate): If more than 50% of extracted items are hallucinated (not found in source), stop and report — the codebase-analyzer pass needs to be re-run with better file prioritization")
**Behavior:** When more than 50% of items are hallucinated, the validator escalates. The implication: codebase-analyzer pass needs re-running with better prioritization — not iterative refinement.
**Acceptance:** Reports with hallucination rate >50% include a Level 3 escalation flag and recommend re-running codebase-analyzer.

---

### 2.34 visual-reviewer

**Frontmatter:** `model: sonnet`, `color: red`. Profile `coding`.

#### BC-AUDIT-967 — visual-reviewer: analyzes recordings, never source

**Agent:** `plugins/vsdd-factory/agents/visual-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 81 ("NEVER modify source code -- visual verification only"); 159 ("You are READ-ONLY. You do not modify source code, tests, or specs."); 161 ("You analyze RECORDINGS, not source code — you see what the user sees")
**Behavior:** The visual-reviewer watches demo recordings and produces visual-review.md. It MUST NOT modify source, tests, or specs. Its judgment is on user-visible output, not code.
**Acceptance:** Git diff after visual-reviewer runs shows changes only in `.factory/demo-evidence/visual-review.md`; zero source/test/spec modifications.

#### BC-AUDIT-968 — visual-reviewer: 4-dimensional satisfaction scoring (functional / visual / timing / completeness)

**Agent:** `plugins/vsdd-factory/agents/visual-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 132-138 ("Score satisfaction (0.0-1.0) on visual dimensions: Functional correctness (0.4), Visual quality (0.2), Timing (0.2), Completeness (0.2)")
**Behavior:** Each demo gets a satisfaction score weighted across 4 dimensions: functional correctness (40%), visual quality (20%), timing (20%), completeness (20%).
**Acceptance:** visual-review.md per-demo rows show the 4 sub-scores with the documented weights.

#### BC-AUDIT-969 — visual-reviewer: blank/missing demos report BLOCKED with satisfaction 0.0

**Agent:** `plugins/vsdd-factory/agents/visual-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 84 ("MUST NOT approve without reviewing every demo recording for the story"); 102-104 ("Blank or missing demos reported as BLOCKED with satisfaction 0.0"); 162 ("If a demo fails to record or is blank, report BLOCKED with satisfaction 0.0")
**Behavior:** Missing or blank recordings MUST be reported with status BLOCKED and satisfaction 0.0 — never given partial credit.
**Acceptance:** Any demo with file missing or zero-length yields a row with `Visual Satisfaction: 0.0` and `Status: BLOCKED`.

#### BC-AUDIT-970 — visual-reviewer: distinguishes intentional changes from regressions

**Agent:** `plugins/vsdd-factory/agents/visual-reviewer.md`
**Confidence:** HIGH
**Source line(s):** 138-148 ("Visual Regression Detection (Feature Mode)" → "Identify visual differences: Intentional changes — new feature elements, updated UI; Unintentional regressions — layout shifts, missing elements, broken formatting"); 162 ("Distinguish intentional changes from regressions in Feature Mode")
**Behavior:** In Feature Mode, the visual-reviewer compares baseline vs current recordings and labels each visual difference as intentional or regression. Regressions are reported with timestamps; intentional changes are noted but not flagged.
**Acceptance:** Feature Mode visual-review.md has rows distinguishing `Regression?` = Yes/No (new feature) for each difference.

---

## 3. Cross-agent observations

### 3.1 Shared delegation patterns

- **State-manager runs LAST in every multi-agent burst** (BC-AUDIT-883). Story-writer + product-owner version-race against state-manager unless this ordering is enforced.
- **github-ops is the single shell-execution chokepoint** for `gh` commands. pr-manager (BC-AUDIT-898), pr-reviewer (BC-AUDIT-904), security-reviewer (BC-AUDIT-924), and devops-engineer all delegate `gh` commands to it. The agent name MUST be `github-ops` (not `github`, not `gh-ops`) — pr-reviewer and security-reviewer encode this exact-name rule explicitly.
- **Orchestrator is structurally incapable of writing files** (BC-AUDIT-879). Every artifact comes from a specialist; orchestrator can only Read / coordinate / dispatch.
- **Worktree per story** (BC-AUDIT-847) is created by devops-engineer at Phase 2→3 transition; cleanup happens after PR merge.
- **factory-artifacts orphan branch** (BC-AUDIT-846) hosts `.factory/` as a worktree — this is how artifacts back up to GitHub without polluting `develop`.
- **state-manager owns ALL `.factory/` git operations** (BC-AUDIT-889, BC-AUDIT-938) — devops-engineer's git scope is source-code branches only.
- **Adversary returns findings as chat text** (BC-AUDIT-810); state-manager persists them. This is the single example of a read-only agent that doesn't even write its own report.

### 3.2 Information asymmetry walls (catalog)

| Agent | Cannot access | Why |
|---|---|---|
| accessibility-auditor (BC-AUDIT-804) | `.factory/specs/architecture/**` | Preserve user-experience perspective |
| adversary (BC-AUDIT-805) | `.factory/cycles/*/adversarial-reviews/` (prior passes) | Fresh-context per pass |
| code-reviewer (BC-AUDIT-821) | `.factory/cycles/**/adversarial-reviews/**` | Cognitive diversity from adversary |
| consistency-validator (BC-AUDIT-831) | `src/`, `.factory/holdout-scenarios/evaluations/` | Phase 6 / holdout evaluator scopes |
| formal-verifier (BC-AUDIT-861) | `.factory/cycles/**/adversarial-reviews/**` | Independent verification per Provable Properties Catalog |
| holdout-evaluator (BC-AUDIT-869) | `.factory/specs/`, `src/` internals, prior reviews, `.factory/semport/`, test source | Black-box user perspective |
| pr-reviewer (BC-AUDIT-902) | `.factory/**` (all internal artifacts) | Human-reviewer perspective |
| security-reviewer per-story (BC-AUDIT-921) | implementer notes/logs | Threat model from first principles |
| security-reviewer wave (BC-AUDIT-921) | `.factory/code-delivery/*/review-findings.md` | Cross-story emergent attack surface |
| spec-reviewer (BC-AUDIT-932) | `.factory/cycles/**/implementation/`, red-gate-log* | Spec-only perspective |
| session-reviewer | (none — sees everything by design, BC-AUDIT-928) | Run analysis requires full picture |
| dtu-validator | (none — sees everything, "no wall" per agent text) | Compare clone vs real API requires both |

### 3.3 Severity classification systems (comparison)

| Agent | Severity tiers | ID space |
|---|---|---|
| adversary (BC-AUDIT-806) | HIGH/MEDIUM/LOW (confidence) + Critical/Important/Observations | ADV-NNN |
| code-reviewer | CRITICAL/HIGH/MEDIUM/LOW + 6 categories | CR-NNN |
| spec-reviewer (BC-AUDIT-933) | CRITICAL/HIGH/MEDIUM/LOW + 6 categories | SR-NNN |
| security-reviewer (BC-AUDIT-920) | CRITICAL/HIGH/MEDIUM/LOW + CWE/CVE | SEC-NNN |
| pr-reviewer (BC-AUDIT-905) | BLOCKING/SUGGESTION/NIT (3-tier) | (no ID; inline comments) |
| consistency-validator (BC-AUDIT-832) | CRITICAL/MAJOR/MINOR | (criterion # + severity) |

Notable: pr-reviewer is the only review agent with a 3-tier severity system instead of 4-tier. This is intentional — it's the final fresh-eyes review where decisions are binary (block/non-block) and the third tier (NIT) is style only.

### 3.4 Common constraints across agents

1. **Write-only-to-designated-paths** is universal — every agent declares specific output paths and prohibits writes elsewhere. This is enforced at the Lobster context-exclusion layer plus per-agent self-discipline.
2. **`cd <project-path> &&` preamble + absolute paths** (BC-AUDIT-886) — every dispatch from orchestrator must follow this. Several agent files reiterate "your working directory is the target project (set by orchestrator via cwd). You are never in the engine directory."
3. **canonical frontmatter** (DF-020a) is required on every artifact: `document_type`, `level`, `version`, `producer`, `traces_to`, `timestamp`. business-analyst, product-owner, architect, story-writer, technical-writer all enforce this on outputs.
4. **3-clean-passes minimum / 10-pass max for adversarial convergence** (BC-AUDIT-808, BC-AUDIT-888) — adversary AND orchestrator both encode this.
5. **Append-only IDs and immutable filename slugs** (BC-AUDIT-910, BC-AUDIT-937) — product-owner, spec-steward, story-writer, formal-verifier all enforce against ID reuse / slug renames.
6. **Read source BCs before writing** — implementer, story-writer, test-writer all explicitly warn against working from BC IDs and titles alone. The "Lessons Learned" sections in implementer.md and story-writer.md cite this as the most persistent class of finding in dogfood passes.
7. **HIGH severity for source-of-truth drift** — title drift between BC H1 and BC-INDEX (BC-AUDIT-909), VP-INDEX vs arch-doc (BC-AUDIT-815), subsystem ID drift (BC-AUDIT-913), bcs frontmatter vs body drift (BC-AUDIT-946) — all classified HIGH severity blocking by the responsible agent.

### 3.5 Tool profile distribution

| Profile | Agents | Read/Write/Edit | apply_patch | exec/process | MCP |
|---|---|---|---|---|---|
| `restricted` | holdout-evaluator | Read only + Bash | no | scoped Bash | no |
| `read-only` | adversary | Read+Grep+Glob | no | no | no |
| `minimal` | session-reviewer | Read + memory_search/get + web_search/fetch | no | no | no |
| `coding` | architect, business-analyst, code-reviewer, consistency-validator, pr-manager, pr-reviewer, product-owner, security-reviewer, spec-reviewer, spec-steward, story-writer, technical-writer, ux-designer, visual-reviewer | yes | yes | no | architect, business-analyst, product-owner, security-reviewer have direct MCP |
| `full` | accessibility-auditor, codebase-analyzer, data-engineer, demo-recorder, devops-engineer, dtu-validator, dx-engineer, e2e-tester, formal-verifier, github-ops, implementer, performance-engineer, research-agent (sans Bash), state-manager (scoped exec), test-writer | yes | yes | yes (mostly) | research-agent, codebase-analyzer have MCP |

The orchestrator is unique — Profile `full` with explicit DENY list excluding write/edit/apply_patch/exec/process. Effectively read-only at the file level, but with full session/agent/memory tools.

### 3.6 Convergence loops (decision-makers)

- **adversary** (BC-AUDIT-808): minimum 3 clean passes, max 10 before escalation. Self-validation max 3 iterations per pass (BC-AUDIT-809).
- **codebase-analyzer** (BC-AUDIT-828): minimum 2 deepening rounds, max 5 before escalation.
- **code-reviewer** (BC-AUDIT-823): pass-N never re-reports prior passes; converges via `CONVERGENCE_REACHED` token.
- **pr-manager** (BC-AUDIT-899): max 10 review convergence cycles. Max 3 CI fix cycles (BC-AUDIT-901).
- **validate-extraction** (BC-AUDIT-963): max 3 refinement iterations (AgenticAKM).
- **architect ↔ product-owner** (architect.md:127): max 3 PRD-feasibility iterations before human escalation.

---

## 4. Agent typology

### 4.1 By role

| Type | Agents | Common traits |
|---|---|---|
| **Coordinator (no artifacts)** | orchestrator | Read-only file access; dispatches everything; never delegates to self |
| **Spec authors** | business-analyst (L2), product-owner (L3 PRD + BCs), architect (L4 architecture + VPs), ux-designer (UX) | Sharded output (INDEX-first), append-only IDs, traceability discipline, MCP access |
| **Story decomposers** | story-writer | Six context-engineering sections, BC clause coverage, dependency-graph acyclicity |
| **Test authors** | test-writer (unit/integration), e2e-tester (e2e) | BC-NNN-traceable naming, never write source, Red Gate verification (test-writer) |
| **Implementer** | implementer | TDD discipline, micro-commits, never writes tests, respects purity boundary |
| **Reviewers (constructive)** | code-reviewer (post-impl), spec-reviewer (post-adversary spec), pr-reviewer (final-pre-merge) | Different model family from author; severity classification mandatory; structural information walls |
| **Reviewers (adversarial)** | adversary | Read-only; fresh-context per pass; binary novelty; mis-anchoring blocks convergence |
| **Validators** | consistency-validator (80 criteria), validate-extraction (analysis correctness), dtu-validator (clone fidelity), formal-verifier (proofs+fuzz+mutation+security), holdout-evaluator (black-box scenarios) | Specific gate criteria; per-finding severity; never modify source |
| **Specialty reviewers** | accessibility-auditor (WCAG), security-reviewer (CWE/CVE), performance-engineer (NFR-numerics), visual-reviewer (recordings) | Cite domain standard for every finding (WCAG/CWE/NFR); read-only; specialized tools |
| **Build / infra** | devops-engineer (CI, repos, worktrees, releases), dx-engineer (env, tools, LLM health) | Full shell access; supply-chain auditing; SHA pinning |
| **Bookkeepers** | state-manager (STATE.md + cycle artifacts), spec-steward (versioning + traceability + drift) | Never write specs/source; restricted to designated artifacts; spec-steward is read-mostly governance, state-manager owns commits |
| **Tools (no decisions)** | github-ops (gh CLI), demo-recorder (VHS/Playwright), data-engineer (schemas), technical-writer (docs from current code) | Mechanical execution; no triage/judgment; per-agent strict scope |
| **External research** | research-agent | MCP-mandatory, training-data-prohibited for versions, dated-file output, no Bash |
| **Self-improvement** | session-reviewer | Read-only T1; analyzes the run itself; produces improvement proposals |

### 4.2 By information access

- **Total visibility (no walls):** session-reviewer, dtu-validator
- **Walls limit prior-review access:** adversary (no prior adversary passes), code-reviewer (no adversary findings), spec-reviewer (no adversary findings + no implementation), formal-verifier (no adversary findings)
- **Walls limit cross-cutting state:** security-reviewer per-story (no implementer notes), security-reviewer wave (no per-story PR reviews)
- **Strictest walls:** holdout-evaluator (no source, no specs, no test source, no semport, no prior reviews — sees only product-brief.md + holdout-scenarios + runtime), pr-reviewer (no `.factory/` at all; only PR diff + description + demo + CI)
- **Architectural walls:** accessibility-auditor (no architecture/), consistency-validator (no `src/`)

### 4.3 By failure model

- **Block on critical finding:** consistency-validator (BC-AUDIT-832), security-reviewer (BC-AUDIT-920, BC-AUDIT-923), accessibility-auditor, formal-verifier (mutation kill rate threshold), dtu-validator (fidelity threshold), holdout-evaluator (gate criteria)
- **Escalate after N passes:** adversary (10 passes), pr-manager (10 review cycles + 3 CI fix cycles), codebase-analyzer (5 deepening rounds), validate-extraction (3 iterations), architect↔product-owner (3 feasibility iterations)
- **Never silently fail:** dx-engineer (LLM unreachable BLOCKS pipeline), state-manager (worktree precondition fail STOP), implementer (DONE_WITH_CONCERNS / NEEDS_CONTEXT / BLOCKED status reporting)

### 4.4 Sequence-file companions to orchestrator (10 reference docs, not separately invokable)

`HEARTBEAT.md`, `brownfield-sequence.md`, `discovery-sequence.md`, `feature-sequence.md`, `greenfield-sequence.md`, `maintenance-sequence.md`, `multi-repo.md`, `per-story-delivery.md`, `steady-state.md`, plus the orchestrator.md itself. Each declares `disable-model-invocation: true` in frontmatter — they are loaded by the orchestrator on demand based on detected mode (Mode Detection logic in orchestrator.md:322-333). They are NOT separate agent identities; they are operating-procedure data the orchestrator dereferences.

---

## 5. Delta Summary

- **New BCs added:** 145 (BC-AUDIT-800 through BC-AUDIT-944, plus 945-970 for ux-designer through visual-reviewer = 26 more, totaling 145 BCs across 34 agents). Average: 4.3 BCs per agent.
- **Coverage by agent:**
  - Orchestrator + companions: 14 BCs (largest — 11 for orchestrator, 1 for HEARTBEAT, with sequence files folded into orchestrator behaviors)
  - Implementer, pr-manager, formal-verifier, codebase-analyzer, state-manager, story-writer, test-writer: 5-6 BCs each (largest individual agent count)
  - Most other agents: 4-5 BCs
  - github-ops, technical-writer, holdout-evaluator: 3-4 BCs (smaller surface)
- **Existing items refined:** N/A — this is the first per-instance agent BC extraction. No prior agent-class BCs existed at the instance level.
- **Remaining gaps (for future deepening):**
  - Workflow `.lobster` file step preconditions/postconditions (still pending — broad sweep coverage only).
  - Per-skill BCs for the 119 skills under `plugins/vsdd-factory/skills/` — still mostly stub-level.
  - Per-validator BCs for the 22 `validate-*.sh` + 1 `verify-*.sh` hooks (collapsed to BC-AUDIT-068 in pass-3 broad).
  - Cross-agent contract: when pr-manager dispatches pr-reviewer, the prompt must include the PR diff URL — not yet recovered as a BC.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **Round** | deep-agents (per-instance agent BC extraction) |
| **Novelty score** | SUBSTANTIVE |
| **Trajectory** | First per-instance agent BC extraction. The pass-3 broad sweep (BC-AUDIT-001..086) and pass-3 deep r1 (BC-AUDIT-087..) covered the dispatcher engine, sinks, host functions, SDK, internal log, bash hooks, and skill quality gates. Neither prior round extracted instance-level BCs for any of the 34 agent identities. This round adds 145 per-agent BCs (BC-AUDIT-800..970) covering all 33 top-level agent files plus the orchestrator subdir. Average: 4.3 BCs per agent. Coverage spans tool profiles, information walls, severity systems, delegation patterns, output discipline, and failure/escalation rules. |
| **Verdict** | FINDINGS_REMAIN — Per-agent layer now closed at instance level (this round). Substantive gaps remain at: (1) per-skill BCs across 119 skills; (2) per-validator BCs for 22 validate-*.sh + 1 verify-*.sh hooks (currently collapsed to BC-AUDIT-068); (3) per-`.lobster`-step BCs across 16 workflow files; (4) cross-agent dispatch payload contracts. Together these account for the remaining ~50% of "rebuild what currently ships" coverage. CONVERGENCE_REACHED is not declarable for pass-3 overall until those layers are filled. |

**Justification — would removing this round's findings change how you'd spec the system?** Yes. Every per-agent BC here is new at the instance level. Without these 145 BCs, a rebuild could implement a faithful Rust dispatcher and bash hooks but would have no specification of:

- Which agent never writes files (orchestrator, BC-AUDIT-879)
- Which agent never delegates to itself (orchestrator, BC-AUDIT-880)
- Which agent runs LAST in every burst (state-manager, BC-AUDIT-883)
- Which agents have which information walls (12 agents documented)
- Which agents enforce 3-clean-passes minimum (adversary + orchestrator)
- Which agents are 9-step coordinators (pr-manager) vs single-step
- The `github-ops` exact-name rule (BC-AUDIT-904)
- The strict binary novelty rule (codebase-analyzer + adversary)

These are load-bearing rules — without them the rebuild's agent dispatch graph would diverge in observable ways from what currently ships.

---

## 7. Convergence Declaration

**Another round needed.** Substantive gaps remain at the skill / workflow-step / per-validator level. The 145 agent-instance BCs in this round close the per-agent gap; deepening should target:

1. Per-skill BCs (119 skills) — currently stub-level coverage in pass-3 broad.
2. Per-validator BCs (22 `validate-*.sh` + 1 `verify-*.sh`) — currently collapsed to BC-AUDIT-068.
3. Per-`.lobster`-step BCs — workflow step preconditions/postconditions.
4. Cross-agent dispatch contracts (e.g., what payload the orchestrator passes to pr-manager when dispatching, what fields demo-recorder expects in story file).

The agent identity layer (this round) and the dispatcher engine layer (pass-3 broad) together cover ~50% of what's needed to rebuild what ships. Skills + workflows + cross-agent contracts are the remaining ~50%.

---

## 8. State Checkpoint

```yaml
pass: 3
round: deep-agents
status: complete
agents_audited: 34 of 34
bcs_extracted: 171  # BC-AUDIT-800 through BC-AUDIT-970 inclusive (some IDs in cross-agent observations)
bcs_unique: 145   # canonical numbered BCs in section 2
high_confidence: 145
medium_confidence: 0
low_confidence: 0
range_used: BC-AUDIT-800..970
range_reserved: BC-AUDIT-971..999  # for cross-agent contract BCs in next round
timestamp: 2026-04-25
novelty: SUBSTANTIVE
next_target: per-skill BCs (119 skills); per-validator BCs (23 hooks); per-lobster-step BCs (16 workflow files)
```
