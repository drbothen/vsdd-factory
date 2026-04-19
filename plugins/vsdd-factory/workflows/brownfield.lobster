workflow:
  name: brownfield-vsdd
  description: >
    VSDD pipeline for existing codebases. Adds Phase 0 (Codebase Ingestion)
    before the standard greenfield pipeline. Phase 0 analyzes the existing code,
    recovers architecture, extracts conventions, and builds a project context
    document that scopes all subsequent phases to NEW features only.

    Cross-cutting infrastructure (DF-020-028) applies via greenfield.lobster
    sub-workflow after Phase 0. See docs/factory-paths.md Cross-Cutting
    Infrastructure section for shared baseline.

    DF-031 upgrades: T1 write routing, security audit split, BC identifier
    migration, per-step commits, consistency validation, adversarial review,
    spec-steward review, research-agent on-demand, post-Phase 0 routing,
    repo verification (not creation), cost monitoring, artifact sharding.
  version: "3.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    # =========================================================================
    # Environment Setup (cross-cutting, DF-027)
    # =========================================================================

    - name: environment-setup
      type: agent
      agent: dx-engineer
      depends_on: []
      task: >
        Run full environment setup:
        1. Install tools (direnv, just, lefthook, mcporter)
        2. Create .env.example + .envrc (product keys only)
        3. LLM health check (Claude, GPT-5.4, Gemini — all REQUIRED)
        4. MCP preflight (mcporter call → verify Perplexity, Context7, Tally)
        5. Supply chain audit via security-reviewer (before installs)
        Block on any security finding or missing required model.
      timeout: "30m"

    - name: environment-gate
      type: gate
      depends_on: [environment-setup]
      gate:
        criteria:
          - "All required tools installed and version-checked"
          - "Supply chain security audit passed"
          - "All 3 model families reachable (Claude, GPT-5.4, Gemini)"
          - "MCP servers accessible via mcporter"
        fail_action: block

    # =========================================================================
    # Repo Verification (not creation — repo already exists) (DF-031 D8)
    # =========================================================================

    - name: repo-verification
      type: agent
      agent: devops-engineer
      depends_on: [environment-gate]
      task: >
        Verify existing repo infrastructure (NOT create — repo already exists):
        1. Verify git remote exists and is accessible
        2. Verify default branch is set
        3. Check for CI/CD workflows (.github/workflows/):
           - If missing: add ci.yml, release.yml, security.yml
           - If present: verify they are functional
        4. Check branch protection rules:
           - If missing: add protection for default branch
           - If present: verify configuration
        5. Create .factory/ worktree on factory-artifacts branch
        6. Enable git rerere
        7. Add merge-config.yaml if missing
        IMPORTANT: Preserve all existing repo configuration. No destructive changes.
      timeout: "15m"

    # =========================================================================
    # Factory Worktree Health Check (BLOCKING)
    # Validates .factory/ is a valid git worktree on factory-artifacts branch.
    # Auto-creates orphan branch + worktree if missing. Checks sync state.
    # =========================================================================

    - name: factory-worktree-health
      type: skill
      skill: "skills/factory-worktree-health/SKILL.md"
      agent: devops-engineer
      depends_on: [repo-verification]

    - name: factory-worktree-gate
      type: gate
      depends_on: [factory-worktree-health]
      gate:
        criteria:
          - "FACTORY_WORKTREE_HEALTH: PASS"
          - ".factory/ is a valid git worktree on factory-artifacts branch"

    # =========================================================================
    # Project Instructions (CLAUDE.md)
    # Optional: generates project-specific CLAUDE.md if not present.
    # =========================================================================

    - name: scaffold-claude-md
      type: skill
      skill: "skills/scaffold-claude-md/SKILL.md"
      depends_on: [factory-worktree-gate]
      condition: "!file_exists('CLAUDE.md')"
      optional: true

    - name: state-initialization
      type: agent
      agent: state-manager
      depends_on: [factory-worktree-gate, scaffold-claude-md]
      task: >
        Initialize STATE.md: pipeline: STARTED, phase: pre-0,
        product: [name], mode: brownfield, timestamp.
        Commit to factory-artifacts branch.

    # =========================================================================
    # Phase 0: Codebase Ingestion (Brownfield-only)
    #
    # Step order (DF-031):
    #   0a (project discovery) + backup
    #   0b (architecture recovery) + backup
    #   0c (convention extraction) + backup
    #   0d (spec reverse-engineering + spec-steward review) + backup
    #   0e (verification gap analysis) + backup
    #   0e-sec (security & dependency audit) + backup + BLOCK
    #   0e.5 (module criticality) + backup
    #   0f-pre (holdout seeding) + backup
    #   0f (project context synthesis) + backup
    #   0f-adv (adversarial review, GPT-5.4, WALL) + backup
    #   0f-post (consistency validation) + fix loop
    #   phase-0 gate (human approval)
    # =========================================================================

    - name: phase-0-codebase-ingestion
      type: skill
      skill: "skills/phase-0-codebase-ingestion/SKILL.md"
      depends_on: [state-initialization]
      timeout: "4h"

    - name: phase-0-artifact-backup
      type: agent
      agent: state-manager
      depends_on: [phase-0-codebase-ingestion]
      task: >
        Final commit of all Phase 0 artifacts to factory-artifacts branch.
        Update STATE.md: phase: 0-complete, awaiting: human-approval.

    - name: phase-0-gate
      type: gate
      depends_on: [phase-0-artifact-backup]
      gate:
        criteria:
          - "Project context document generated at .factory/phase-0-ingestion/project-context.md"
          - "Architecture recovery complete at .factory/phase-0-ingestion/recovered-architecture.md"
          - "Conventions extracted at .factory/phase-0-ingestion/conventions.md"
          - "Behavioral contracts documented for all public API surfaces (BC-S.SS.NNN format)"
          - "All BCs have origin: recovered in frontmatter"
          - "Verification gap analysis complete"
          - "Module criticality classification complete"
          - "Security audit complete at .factory/phase-0-ingestion/security-audit.md"
          - "Adversarial review complete at .factory/phase-0-ingestion/adversarial-review-0.md"
          - "Consistency validation passed at .factory/phase-0-ingestion/validation-report.md"
          - "Holdout scenarios seeded from recovered BCs"
        fail_action: block

    - name: phase-0-human-approval
      type: human-approval
      depends_on: [phase-0-gate]
      approval:
        prompt: >
          Phase 0 Codebase Ingestion is complete. Please review the project context
          document, recovered architecture, convention extraction, security audit,
          adversarial review findings, and consistency validation report. Approve to
          proceed to next step (feature request, exploration, or park).
        artifacts:
          - ".factory/phase-0-ingestion/project-context.md"
          - ".factory/phase-0-ingestion/recovered-architecture.md"
          - ".factory/phase-0-ingestion/conventions.md"
          - ".factory/phase-0-ingestion/behavioral-contracts/"
          - ".factory/phase-0-ingestion/verification-gap-analysis.md"
          - ".factory/phase-0-ingestion/security-audit.md"
          - ".factory/phase-0-ingestion/adversarial-review-0.md"
          - ".factory/phase-0-ingestion/validation-report.md"
          - ".factory/specs/module-criticality.md"
        timeout: "24h"

    # =========================================================================
    # Post-Phase 0 Routing (DF-031 D14)
    # =========================================================================

    - name: post-phase-0-routing
      type: agent
      agent: orchestrator
      depends_on: [phase-0-human-approval]
      task: >
        Ask the human: Phase 0 complete. What would you like to do?
        1. "I have a feature in mind" → continue to market intelligence + Phase 1
        2. "Help me figure out what to build" → route to planning.lobster (Path 5)
        3. "Nothing right now" → park:
           - state-manager sets STATE.md to phase: 0-complete, awaiting: feature-request
           - Notify human: "Codebase onboarded. Return anytime with a feature request."

        When human returns later with a feature request:
        - Orchestrator detects STATE.md has phase: 0-complete
        - Routes to Phase 1 with Phase 0 context (skips Phase 0)
        - This is effectively Feature Mode (Path 3) since Phase 0 is done

    # =========================================================================
    # Market Intelligence Assessment (brownfield feature validation)
    # Only runs if human chose option 1 ("I have a feature in mind")
    # =========================================================================

    - name: brownfield-market-intel
      type: skill
      skill: "skills/market-intelligence-assessment/SKILL.md"
      depends_on: [post-phase-0-routing]
      condition: "routing.choice == 'feature'"
      config:
        depth: "L1"
        scope: "feature"
        context: ".factory/phase-0-ingestion/project-context.md"

    - name: brownfield-market-review
      type: human-approval
      depends_on: [brownfield-market-intel]
      condition: "routing.choice == 'feature'"
      approval:
        prompt: >
          Market intelligence for the proposed feature is complete.
          Review the assessment before proceeding to Phase 1.
        artifacts:
          - ".factory/planning/market-intel.md"
        timeout: "48h"

    # =========================================================================
    # Cross-Language Porting Detection (DF-014 Semport)
    # =========================================================================

    - name: detect-cross-language-porting
      type: agent
      agent: architect
      depends_on: [brownfield-market-review]
      condition: "routing.choice == 'feature'"
      task: >
        Read Phase 0 behavioral contracts and project discovery.
        Determine if the human has requested porting to a different language
        (e.g., Python to Rust migration). Check for explicit language migration
        signals in the feature request or human instructions.
        Write decision to .factory/brownfield/porting-decision.md with:
        - source_language, target_language (if porting requested)
        - porting_scope (full codebase or specific modules)
        - needs_semport: true/false

    - name: semport-translation
      type: skill
      skill: "skills/semport-analyze/SKILL.md"
      depends_on: [detect-cross-language-porting]
      condition: "brownfield.needs_semport == true"
      timeout: "4h"

    - name: semport-validation-gate
      type: gate
      depends_on: [semport-translation]
      condition: "brownfield.needs_semport == true"
      gate:
        criteria:
          - "Semantic equivalence validated via execution trace comparison"
          - "Target language code compiles without errors"
          - "All behavioral contracts tested and passed"
        fail_action: block

    # =========================================================================
    # Transition: Feed Phase 0 output + market intel into Phase 1 context
    # =========================================================================

    # --- DF-037: Brownfield Design System Extraction (D2) ---
    - name: brownfield-design-system-extract
      type: skill
      skill: "skills/design-system-bootstrap/SKILL.md"
      depends_on: [brownfield-market-review, semport-validation-gate]
      condition: "routing.choice == 'feature' and feature_type in ['ui', 'full-stack']"
      # D2 brownfield flow: Extract design system from existing codebase.
      # codebase-analyzer extracts styles -> ux-designer normalizes to tokens ->
      # accessibility-auditor validates -> human approves.

    - name: brownfield-design-system-approval
      type: human-approval
      depends_on: [brownfield-design-system-extract]
      condition: "routing.choice == 'feature' and feature_type in ['ui', 'full-stack']"
      approval:
        prompt: >
          Design system extracted from existing codebase. Review tokens,
          contracts, and constraints. Override any values.
        artifacts:
          - ".factory/design-system/"
        timeout: "24h"

    - name: brownfield-to-greenfield-transition
      type: agent
      agent: state-manager
      depends_on: [brownfield-market-review, semport-validation-gate, brownfield-design-system-approval]
      condition: "routing.choice == 'feature'"
      task: >
        Prepare Phase 1 context from Phase 0 output:
        1. Ensure all Phase 0 artifacts are committed to factory-artifacts
        2. Update STATE.md: phase: transitioning, next: phase-1
        3. Verify Phase 0 artifacts are accessible as context for Phase 1 agents
        4. Include market intelligence if available
        5. Include design system if bootstrapped (DF-037 D2)

    # =========================================================================
    # Phases 1-6: Standard VSDD (imported from greenfield)
    # All cross-cutting infrastructure applies via greenfield.lobster:
    # - Agent tiers (T1/T2/T3)
    # - Per-story delivery (worktree -> PR -> merge)
    # - Information asymmetry walls (8 agents)
    # - Security review touchpoints (4 places)
    # - Cost monitoring (continuous)
    # - Notifications (configurable channel)
    # - Artifact backup (state-manager at every phase gate)
    # - Crash recovery (phase-level resume)
    # - Artifact sharding (index + detail files)
    # - 4-level spec hierarchy (L1->L4, BC-S.SS.NNN)
    # - Fix PR delivery (FIX-P[N]-NNN)
    # - CI/CD (verified/created during repo-verification step)
    # - Release (semver -> CHANGELOG -> tag -> publish)
    # - DTU (assessment + clones + validation)
    # - Gene transfusion (assessment + translation)
    # =========================================================================

    - name: greenfield-pipeline
      type: sub-workflow
      sub_workflow: "greenfield.lobster"
      depends_on: [brownfield-to-greenfield-transition]
      condition: "routing.choice == 'feature'"

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # Note: greenfield.lobster has its own session review, but brownfield adds
    # a wrapper review covering Phase 0 and brownfield-specific steps.
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [greenfield-pipeline]
      # Runs after all pipeline work, including release via greenfield

    - name: session-review-approval
      type: human-approval
      depends_on: [session-review]
      approval:
        prompt: >
          Session review complete. Review the analysis and improvement proposals.
          For each proposal: Approve (create task), Defer (add to backlog),
          or Reject (log reason).
        artifacts:
          - ".factory/session-reviews/review-YYYY-MM-DD-[run-id].md"
          - ".factory/session-reviews/improvement-proposals-[run-id].md"
        timeout: "72h"

    - name: process-review-decisions
      type: agent
      agent: state-manager
      depends_on: [session-review-approval]
      task: >
        Process human decisions on improvement proposals:
        - APPROVE: add to product backlog or create story
        - DEFER: append to .factory/session-reviews/improvement-backlog.md
        - REJECT: log to pattern-database.yaml with rejection reason
        Update benchmarks.yaml with this run's metrics.
        Set STATE.md pipeline status to COMPLETE.
