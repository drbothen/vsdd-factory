---
name: orchestrator-greenfield-sequence
description: Orchestrator workflow reference for the full greenfield VSDD pipeline from brief through release. Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

# Greenfield Delegation Sequence

Reference file for the orchestrator. Load when running a greenfield pipeline.

## Step 1: Gather Human Input (Interactive)

Have a conversation with the human to understand the product idea.
Capture: what they want to build, target audience, key differentiators,
constraints, tech stack preferences.

## Step 2: Repository Initialization

When no existing repo (no repo path, or path has no `.git/`):

S2-01. Ask: GitHub org, repo name, visibility (default: private), default branch (default: develop)
S2-02. Spawn devops-engineer: "Create GitHub repo, develop branch, branch protection,
   git rerere, merge-config.yaml, AND factory-artifacts orphan branch with .factory/
   mounted as git worktree. Use the repo-initialization skill. Verify worktree."
S2-03. Spawn devops-engineer with factory-worktree-health skill (BLOCKING).
   Do NOT proceed until worktree health passes.

## Step 3: State & Brief Initialization

S3-01. Spawn state-manager: "Initialize STATE.md and .factory/ directory structure."
S3-02. Spawn product-owner: "Create product brief from the following human input:
   [include the human's product description, answers, and context from Step 1].
   Write to .factory/planning/product-brief.md using templates/product-brief-template.md."
S3-03. HUMAN reviews and approves the brief.

## Step 4: Pre-Pipeline

S4-01. Spawn dx-engineer: "Environment setup, toolchain preflight, LLM health check, MCP preflight"

## Step 5: Market Intelligence

S5-01. Spawn business-analyst: "Extract market context from the product brief"
S5-02. Spawn research-agent: "Market landscape research"
S5-03. Spawn business-analyst: "Synthesize into market-intel.md with GO/CAUTION/STOP"
S5-04. HUMAN reviews market intel -> proceed / pivot / abort

## Phase 1: Spec Crystallization

P1-01. Spawn research-agent: "Domain research"
P1-02. Spawn business-analyst: "SHARDED L2 Domain Spec -> .factory/specs/domain-spec/ directory.
    Produce L2-INDEX.md FIRST. Then section files: capabilities.md (CAP-NNN), entities.md,
    invariants.md (DI-NNN), events.md, edge-cases.md (DEC-NNN), assumptions.md (ASM-NNN),
    risks.md (R-NNN), failure-modes.md (FM-NNN), differentiators.md. Each 800-1,200 tokens.
    Every section: traces_to: L2-INDEX.md. Use L2-domain-spec-*-template.md.
    Do NOT produce a monolithic domain-spec-L2.md."
P1-03. Spawn product-owner: "L3 PRD index (.factory/specs/prd.md) + per-file behavioral
    contracts in behavioral-contracts/BC-S.SS.NNN.md. Use behavioral-contract-template.md.
    Also produce ALL 4 PRD supplements in prd-supplements/. Produce holdout scenarios
    in holdout-scenarios/HS-NNN.md with HS-INDEX.md."
P1-04. Architecture feasibility loop (max 3 iterations):
    a. Spawn architect: "Architecture feasibility review"
    b. If PASS → proceed to P1-05
    c. If issues found → spawn product-owner: "Revise PRD based on architect feedback: [findings]"
    d. Go back to (a)
P1-05. Spawn architect: "SHARDED architecture -> .factory/specs/architecture/ directory.
    Produce ARCH-INDEX.md FIRST. Then 8 section files. Each 600-1,000 tokens.
    Use architecture-*-template.md. Also produce L4 VP-NNN files in
    verification-properties/VP-NNN.md with VP-INDEX.md. Also produce module-criticality.md.
    IMPORTANT: architecture output MUST include a deployment_topology field in
    ARCH-INDEX.md frontmatter — either 'single-service' or 'multi-service'.
    If multi-service, system-overview.md MUST include a Service Boundaries section
    listing each service, its tech stack, and inter-service contracts."
P1-06. Spawn architect: "DTU assessment" (MANDATORY — always produces dtu-assessment.md)
    - Scan dependency-graph.md External Dependency Summary for HTTP clients, SDKs, auth providers
    - If external dependencies found: produce dtu-assessment.md with fidelity classifications
    - If NO external dependencies found: produce dtu-assessment.md with "DTU_REQUIRED: false"
      and rationale (e.g., "pure library, no external I/O")
    - Gate: dtu-assessment.md MUST exist before Phase 2 gate approval
P1-07. Spawn architect: "Gene transfusion assessment" (if reference implementations exist)

### Multi-Repo Transition (if architect identifies multi-service topology)

After P1-05, check the architect's `deployment_topology` in ARCH-INDEX.md:

- **single-service:** Continue with single-repo greenfield (P1-08+)
- **multi-service:** Transition to multi-repo:
  a. Present architect's service boundaries to human for confirmation
  b. Human confirms or overrides (may force single-repo mono-repo)
  c. If confirmed multi-repo:
     - Spawn devops-engineer: "Create per-service repos from architect's
       service map. Set up .factory/ worktree in each. Create .factory-project/
       worktree in primary repo. Generate project.yaml."
     - Spawn state-manager: "Move unified specs from .factory/specs/ to
       .factory-project/specs/. Initialize .factory-project/STATE.md."
     - Switch to multi-repo.lobster for per-repo Phase 1 refinement + Phase 2+
     - STOP processing this greenfield sequence — multi-repo.lobster takes over

### Continue single-repo (or before multi-repo transition)

P1-08. Spawn ux-designer: "SHARDED UX spec -> .factory/specs/ux-spec/ directory.
    Produce UX-INDEX.md FIRST. Then per-screen and per-flow files.
    Use ux-spec-*-template.md. Also run design-system-bootstrap skill."
    (if UI/full-stack product)
P1-09. Spawn accessibility-auditor: "Review UX spec for WCAG 2.1 AA compliance.
     Check screen definitions for contrast, keyboard nav, focus order, alt text
     coverage, timing constraints." (if UI/full-stack product)
P1-10. Adversarial spec review loop (max 10 passes):
    a. Spawn adversary: "Adversarial spec review (Part A: verify prior fixes, Part B: find new issues)"
    b. If convergence reached (see VSDD.md Phase 6 Spec dimension) → exit loop
    c. Spawn spec-reviewer: "Constructive review post-adversary (SR-NNN)"
    d. Route findings per VSDD.md Feedback Integration Loop
    e. Go back to (a) with fresh adversary context
    After 10 passes: escalate to human.
P1-11. Spawn state-manager: "Commit Phase 1 artifacts to factory-artifacts"
P1-12. HUMAN APPROVAL

## Phase 2: Story Decomposition

P2-01. Spawn story-writer: "SHARDED stories -> per-file STORY-NNN.md in .factory/stories/
    with STORY-INDEX.md, epics.md, dependency-graph.md. Use story-template.md.
    AC->BC tracing, wave schedule, wave holdout scenarios."
P2-02. Spawn consistency-validator: "Full validation including AC completeness AND sharding
    integrity: verify all INDEX files exist, all detail files have traces_to pointing at
    their index, no orphaned detail files."
P2-03. Adversarial story review loop (max 10 passes):
    a. Spawn adversary: "Story review (coverage gaps, sizing, dependency issues)"
    b. If convergence reached (see VSDD.md Phase 6 Spec dimension) → exit loop
    c. Spawn spec-reviewer: "Constructive story review post-adversary (SR-NNN)"
    d. Route findings per VSDD.md Feedback Integration Loop
    e. Spawn consistency-validator: "Re-validate after story fixes"
    f. Go back to (a) with fresh adversary context
P2-04. Spawn state-manager: "Commit Phase 2 artifacts"
P2-05. HUMAN APPROVAL

## Phase 3: Test-First Implementation

See `per-story-delivery.md` for wave/story delivery cycle.

P3-01. Spawn dx-engineer: "Full toolchain preflight for Phase 3"
P3-02. For each wave: run per-story delivery cycle from per-story-delivery.md

## Phase 3.5: Holdout Evaluation

P3H-01. Spawn dtu-validator: "Start DTU clones" (if dtu-assessment.md has DTU_REQUIRED: true)
    Pre-check: verify dtu-creation has been run, docker-compose.dtu.yml exists,
    at least one clone validation-report.md shows fidelity >= threshold.
    If DTU_REQUIRED: false in dtu-assessment.md, skip with logged reason.
P3H-02. Spawn holdout-evaluator: "Evaluate all holdout scenarios"

## Phase 4: Adversarial Refinement

P4-01. Adversarial code review loop (max 10 passes per VSDD.md Phase 4):
    a. Spawn adversary: "Fresh context code review (ADV-PN-NNN, Part A: verify fixes, Part B: find new)"
    b. Classify fixes: transparent vs behavior-changing
    c. Spawn code-reviewer: "Constructive code review (CR-NNN)"
    d. Spawn security-reviewer: "Triage adversary security findings"
    e. If convergence reached (see VSDD.md Phase 6) → exit loop
    f. Route findings per VSDD.md Feedback Integration Loop
    g. Fix PR delivery (FIX-P4-NNN via code-delivery skill)
    h. If behavior-changing fixes → spawn holdout-evaluator: "Regression check"
    i. Go back to (a) with fresh adversary context

## Phase 5: Formal Hardening

P5-01. Formal hardening loop (per VSDD.md Phase 5 — all results feed back into Phase 4):
    a. Spawn formal-verifier: "Per-file VP-NNN proofs (Kani, proptest, fuzz)"
       VP locking on proof pass (git tag vp-verified-VP-NNN)
    b. Spawn formal-verifier: "Security scan" → security-reviewer triages HIGH/CRIT
    c. Spawn dtu-validator: "L4 adversarial testing" (if DTU clones exist)
    d. If all pass → exit loop
    e. If failures → Fix PR delivery (FIX-P5-NNN), re-run ONLY failing checks
    f. Go back to (d)

## Phase 6: Convergence

P6-01. Convergence loop (max 10 cycles):
    a. Spawn consistency-validator: "7-dimensional convergence report"
       Dark Factory's 7 dimensions (operational expansion of VSDD.md Phase 6):
       1. Spec→Code Traceability (L1→L4)
       2. Implementation Completeness
       3. Adversarial Review Convergence (finding decay to zero)
       4. Formal Verification (VP pass rate)
       5. Holdout Evaluation (mean satisfaction ≥ 0.85, std dev < 0.15)
       6. L3 BC Convergence
       7. L4 VP Convergence
    b. If ALL 7 dimensions converge → exit loop (Maximum Viable Refinement reached)
    c. Route failing dimensions per VSDD.md Feedback Integration Loop
    d. Fix PR delivery for each fix
    e. Go back to (a)
    After 10 cycles: escalate to human with cost-benefit analysis.
P6-02. Spawn state-manager: "Final artifact commit"
P6-03. HUMAN APPROVAL

## Post-Pipeline

PP-01. Spawn devops-engineer: "Release (semver -> CHANGELOG -> tag -> gh release -> publish)"
PP-02. Steady-state handoff (see steady-state.md)
PP-03. Spawn session-review: "8-dimension post-run analysis"
PP-04. HUMAN reviews improvement proposals (72h, approve/defer/reject)
PP-05. Spawn state-manager: "Update pattern database, benchmarks, improvement backlog"
PP-06. Post-feature validation (7/30/90 day monitoring, if configured)

## Demo Recording (MANDATORY)

After all Phase 3 implementation is complete and tests pass:
1. Spawn demo-recorder: "Record per-AC demos for all implemented stories"
2. Wait for demo-evidence/report.md
3. Only THEN proceed to Phase 3.5 holdout evaluation

After Phase 6 convergence:
1. Spawn devops-engineer: "Create worktree for final demo:
   `git worktree add .worktrees/final-demo -b demo/final-journey develop`"
2. Spawn demo-recorder: "Record final user journey demo in .worktrees/final-demo/"
3. Wait for updated evidence report
4. Spawn pr-manager: "Create PR for final demo evidence (demo/final-journey → develop)"
5. After PR merged, spawn devops-engineer: "Remove worktree .worktrees/final-demo"
6. Only THEN proceed to release
