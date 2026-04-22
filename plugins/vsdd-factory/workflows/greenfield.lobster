workflow:
  name: greenfield-vsdd
  description: >
    Full VSDD pipeline for greenfield development. Transforms a product brief
    into verified, adversarially-reviewed, formally-proven code through six phases.

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    This is the reference path — all other primary paths are subsets or variations.
    Key features: adaptive planning + market intel before Phase 1, state-manager
    commits at every phase gate, per-story delivery via code-delivery.lobster,
    all 8 information asymmetry walls, all 4 security review touchpoints,
    cost monitoring with phase metadata, fix PR delivery, release pipeline,
    post-feature validation hook.
    DF-037 additions: design system bootstrap (Phase 1), multi-variant design,
    heuristic evaluation, wave-level UI quality gates (responsive, performance,
    completeness), convergence UI quality gate (100% fidelity), Storybook MCP
    self-healing component test loop in code-delivery.
  version: "2.1.0"

  cost_monitoring:
    enabled: true
    metadata:
      mode: greenfield

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    # =========================================================================
    # Repo Initialization (FIRST — .factory/ must exist before any artifacts)
    # Factory-artifacts orphan branch, worktree. CI/CD deferred to post-arch.
    # =========================================================================

    - name: repo-initialization
      type: skill
      skill: "skills/repo-initialization/SKILL.md"
      depends_on: []
      # Includes: repo creation, branch protection,
      # .factory/ worktree on factory-artifacts branch
      # NOTE: CI/CD setup deferred to phase-1-cicd-setup (after architecture)

    # =========================================================================
    # Factory Worktree Health Check (BLOCKING)
    # Validates .factory/ is a valid git worktree on factory-artifacts branch.
    # Auto-creates orphan branch + worktree if missing. Checks sync state.
    # =========================================================================

    - name: factory-worktree-health
      type: skill
      skill: "skills/factory-worktree-health/SKILL.md"
      agent: devops-engineer
      depends_on: [repo-initialization]

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

    # =========================================================================
    # State Initialization
    # =========================================================================

    - name: state-initialization
      type: agent
      agent: state-manager
      depends_on: [factory-worktree-gate, scaffold-claude-md]
      task: >
        Initialize STATE.md: pipeline: STARTED, phase: pre-1,
        product: [name], mode: greenfield, timestamp.
        Commit to factory-artifacts branch.

    # =========================================================================
    # Adaptive Planning (DF-016) — artifact detection + routing
    # Runs AFTER repo init so .factory/ exists for writing planning artifacts.
    # Market intel (DF-033) runs at depth scaled by level (L0-L4).
    # Product brief created by product-owner (orchestrator does not write).
    # =========================================================================

    - name: adaptive-planning
      type: sub-workflow
      sub_workflow: "planning.lobster"
      depends_on: [state-initialization]
      condition: "config.skip_planning != true"
      # planning.lobster handles: environment setup, artifact detection,
      # market intelligence, brief creation/validation, and routes back here

    # =========================================================================
    # Phase 1: Spec Crystallization
    # =========================================================================

    - name: phase-1-spec-crystallization
      type: skill
      skill: "skills/phase-1-spec-crystallization/SKILL.md"
      depends_on: [adaptive-planning]

    # Architect Feasibility Review (DF-020g: PO↔Architect collaborative iteration)
    - name: architect-feasibility-review
      type: agent
      agent: architect
      depends_on: [phase-1-spec-crystallization]
      task: "Review L3 PRD subsystem grouping for NFR coherence, integration
             feasibility, purity boundary alignment, verifiability.
             Produce architecture-feasibility-report.md."

    # PRD Revision (conditional on architect feedback)
    - name: prd-revision
      type: skill
      skill: "skills/phase-1-prd-revision/SKILL.md"
      depends_on: [architect-feasibility-review]
      condition: "architect.verdict == 'request-changes'"

    # DTU Assessment (after architecture design in Phase 1)
    - name: phase-1-dtu-assessment
      type: agent
      agent: architect
      depends_on: [phase-1-spec-crystallization]
      task: "Assess external service dependencies. Produce dtu-assessment.md.
             Spawn research-agent for API specs of DTU candidates."
      # MANDATORY: always produces dtu-assessment.md (DTU_REQUIRED: true/false with rationale)

    # Gene Transfusion Assessment (after architecture design in Phase 1)
    - name: phase-1-gene-transfusion-assessment
      type: agent
      agent: architect
      depends_on: [phase-1-spec-crystallization]
      task: "Assess modules for gene transfusion candidates.
             Produce gene-transfusion-assessment.md using
             templates/gene-transfusion-assessment-template.md.
             Spawn research-agent for reference docs, license, and
             test coverage of each candidate."

    # =========================================================================
    # CI/CD Pipeline Setup (MANDATORY — after architecture determines tech stack)
    # Creates .github/workflows/ based on architecture language, stack, and
    # service topology. Cannot run during repo-init because tech stack is unknown.
    # =========================================================================

    - name: phase-1-cicd-setup
      type: agent
      agent: devops-engineer
      depends_on: [phase-1-spec-crystallization]
      task: >
        Create CI/CD pipelines based on the architecture:
        1. Read architecture/ARCH-INDEX.md for language, framework, deployment topology
        2. Create .github/workflows/ci.yml (lint, test, build on PR)
        3. Create .github/workflows/release.yml (build + publish on tag)
        4. Create .github/workflows/security.yml (weekly audit + PR check)
        5. Update branch protection to require CI status checks
        6. Produce .factory/specs/cicd-setup.md documenting what was created
        Gate: .github/workflows/ci.yml MUST exist. cicd-setup.md MUST exist.
      # MANDATORY: always produces CI/CD workflows and cicd-setup.md

    # --- DF-037: Design System Bootstrap (UI products, Phase 1) ---
    - name: phase-1-design-system-bootstrap
      type: skill
      skill: "skills/design-system-bootstrap/SKILL.md"
      depends_on: [phase-1-spec-crystallization]
      condition: "feature_type in ['ui', 'full-stack']"
      # D2: Bootstrap design system from brief + brand guidelines.
      # Creates .factory/design-system/ (tokens, contracts, patterns, constraints).

    - name: phase-1-design-system-approval
      type: human-approval
      depends_on: [phase-1-design-system-bootstrap]
      condition: "feature_type in ['ui', 'full-stack']"
      approval:
        prompt: >
          Design system bootstrapped from product brief. Review tokens,
          component contracts, and constraints. Override any values.
        artifacts:
          - ".factory/design-system/"
        timeout: "24h"

    # --- DF-037: Multi-Variant Design (UI products, complex screens) ---
    - name: phase-1-multi-variant-design
      type: skill
      skill: "skills/multi-variant-design/SKILL.md"
      depends_on: [phase-1-spec-crystallization, phase-1-design-system-approval]
      condition: "feature_type in ['ui', 'full-stack'] and ux_spec.has_complex_screens == true"
      # D5: Generate 2-3 variants for complex screens, score, present to human.

    - name: phase-1-multi-variant-approval
      type: human-approval
      depends_on: [phase-1-multi-variant-design]
      condition: "feature_type in ['ui', 'full-stack'] and ux_spec.has_complex_screens == true"
      approval:
        prompt: >
          Design variants generated for complex screens. Select preferred
          variant or request synthesis. See .factory/ui-quality/variants/.
        artifacts:
          - ".factory/ui-quality/variants/"
        timeout: "24h"

    # --- DF-037: Heuristic Evaluation on UX Spec (Phase 1) ---
    - name: phase-1-heuristic-evaluation
      type: skill
      skill: "skills/ux-heuristic-evaluation/SKILL.md"
      depends_on: [phase-1-spec-crystallization]
      condition: "feature_type in ['ui', 'full-stack']"
      # D6: Nielsen's 10 heuristics on UX spec. Scores < 0.7 flag for remediation.

    - name: phase-1-consistency-audit
      type: agent
      agent: consistency-validator
      depends_on: [phase-1-spec-crystallization, phase-1-dtu-assessment, phase-1-cicd-setup]
      task: >
        Fresh-context consistency audit of ALL Phase 1 artifacts.
        Check: BC↔capability anchors, story↔subsystem anchors, VP↔story anchors,
        traceability table descriptions, crate/file path references.
        Verify semantic correctness, not just structural validity.
      # MANDATORY: runs before every phase gate, not just adversarial convergence

    - name: phase-1-gate
      type: gate
      depends_on: [phase-1-spec-crystallization, architect-feasibility-review, prd-revision, phase-1-dtu-assessment, phase-1-gene-transfusion-assessment, phase-1-cicd-setup, phase-1-design-system-approval, phase-1-heuristic-evaluation, phase-1-consistency-audit]
      gate:
        criteria:
          - "All requirements have unique IDs and numerical targets"
          - "L2 Domain Spec exists and covers all L1 capabilities"
          - "Every L2 capability maps to at least one BC-S.SS.NNN"
          - "Architecture Feasibility Report produced (approved or no issues)"
          - "L4 Provable Properties Catalog drafted with feasibility assessments"
          - "Provable Properties Catalog covers all security boundaries"
          - "Purity Boundary Map is complete -- every module classified"
          - "Verification tooling selected and constraints documented"
          - "Module criticality classification written to .factory/specs/module-criticality.md"
          - "DTU assessment completed (dtu-assessment.md exists)"
          - "If DTU candidates: API specs fetched for each"
          - "Gene transfusion assessment completed (gene-transfusion-assessment.md exists)"
          - "Design system bootstrapped and approved (if UI product, DF-037 D2)"
          - "Heuristic evaluation score >= 0.7 on UX spec (if UI product, DF-037 D6)"
          - "CI/CD pipelines created (.github/workflows/ci.yml exists)"
          - "cicd-setup.md exists in .factory/specs/ documenting CI/CD configuration"
          - "Branch protection requires CI status checks"
          - "Every ASM-NNN has Status, Validation Method, and Impact populated"
          - "Every R-NNN has Status, Category, and Mitigation populated"
          - "Every R-NNN NFR candidate has corresponding NFR-NNN"
          - "Every HIGH-impact R-NNN addressed in architecture"
        fail_action: block

    - name: phase-1d-adversarial-spec-review
      type: loop
      depends_on: [phase-1-gate]
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: spawn-adversary-spec-review
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review all Phase 1 spec artifacts with fresh context. Check for
              ambiguity, missing edge cases, contradictions, verification gaps.
              Write findings to .factory/specs/adversarial-reviews/ as
              per-finding files with ADV-P[N]-INDEX.md.
              Report CONVERGENCE_REACHED if findings are cosmetic only.
            context:
              include:
                - ".factory/specs/domain-spec-L2.md"
                - ".factory/specs/architecture/ARCH-INDEX.md"
                - ".factory/specs/architecture/*.md"
                - ".factory/specs/behavioral-contracts/**"
                - ".factory/specs/verification-properties/**"
                - ".factory/specs/architecture-feasibility-report.md"
                - ".factory/specs/prd.md"
                - ".factory/specs/prd-supplements/**"
                - ".factory/specs/adversarial-reviews/**"
                - ".factory/specs/module-criticality.md"
              exclude:
                - ".factory/holdout-scenarios/**"

          - name: fix-spec-findings
            type: skill
            skill: "skills/phase-1d-adversarial-spec-review/SKILL.md"
            depends_on: [spawn-adversary-spec-review]

    # Gemini spec review — constructive 3rd perspective after adversary remediation
    - name: phase-1d-spec-review-gemini
      type: loop
      depends_on: [phase-1d-adversarial-spec-review]
      loop:
        max_iterations: 10
        exit_condition: "spec_reviewer.verdict == 'APPROVED' OR spec_reviewer.findings.critical == 0"
        steps:
          - name: gemini-spec-review
            type: agent
            agent: spec-reviewer
            task: >
              Review ALL Phase 1 spec artifacts (L2 Domain Spec, L3 PRD + BCs,
              Architecture, L4 VPs) AFTER adversary remediation. Focus on what
              the adversary missed. Use SR-NNN finding IDs.
              Write to .factory/specs/adversarial-reviews/spec-review-pass[N].md
            context:
              include:
                - ".factory/specs/**"
                - ".factory/design-system/**"
              exclude:
                - ".factory/cycles/**/implementation/**"
          - name: gemini-spec-remediation
            type: agent
            agent: orchestrator
            depends_on: [gemini-spec-review]
            condition: "spec_reviewer.findings.critical > 0 OR spec_reviewer.findings.high > 0"
            task: >
              Triage spec-reviewer SR-NNN findings. Route fixes to originating
              agents (upstream-first: L2 before L3 before architecture).

    - name: phase-1-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-1d-spec-review-gemini]
      task: >
        Commit Phase 1 artifacts to factory-artifacts branch.
        Update STATE.md: phase: 1-complete.

    - name: phase-1-human-approval
      type: human-approval
      depends_on: [phase-1-state-backup]
      approval:
        prompt: >
          Phase 1 Spec Crystallization is complete. Please review the spec package
          and approve to proceed to Story Decomposition.
        artifacts:
          - ".factory/specs/prd.md"
          - ".factory/specs/prd-supplements/"
          - ".factory/specs/architecture/ARCH-INDEX.md"
          - ".factory/specs/architecture/"
          - ".factory/specs/ux-spec/UX-INDEX.md"
          - ".factory/specs/ux-spec/screens/"
          - ".factory/specs/ux-spec/flows/"
          - ".factory/specs/adversarial-reviews/"
          - ".factory/specs/module-criticality.md"
          - ".factory/design-system/"
          - ".factory/ui-quality/heuristic-evaluation.md"
        timeout: "24h"

    # =========================================================================
    # Multi-Repo Topology Check (after Phase 1 architecture)
    # If architect set deployment_topology: multi-service in ARCH-INDEX.md,
    # transition to multi-repo mode. Human confirms before repos are created.
    # =========================================================================

    - name: multi-repo-topology-check
      type: agent
      agent: orchestrator
      depends_on: [phase-1-human-approval]
      task: >
        Read .factory/specs/architecture/ARCH-INDEX.md frontmatter.
        Check deployment_topology field:
        - If 'single-service' or field absent: set MULTI_REPO=false, continue.
        - If 'multi-service': present architect's Service Boundaries section
          to human for confirmation. Include the service table from
          system-overview.md showing each service's tech stack, role,
          and dependencies.

    - name: multi-repo-human-confirmation
      type: human-approval
      depends_on: [multi-repo-topology-check]
      condition: "deployment_topology == 'multi-service'"
      approval:
        prompt: >
          The architect identified a multi-service topology. Review the
          service boundaries and confirm whether to split into multiple
          repositories or continue as a mono-repo.
        artifacts:
          - ".factory/specs/architecture/ARCH-INDEX.md"
          - ".factory/specs/architecture/system-overview.md"
        timeout: "24h"

    - name: multi-repo-transition
      type: agent
      agent: devops-engineer
      depends_on: [multi-repo-human-confirmation]
      condition: "human_approved_multi_repo == true"
      task: >
        Create per-service repos from architect's service map in
        .factory/specs/architecture/system-overview.md Service Boundaries.
        Set up .factory/ worktree in each repo.
        Create .factory-project/ worktree in primary repo.
        Generate project.yaml with repo entries, dependencies, and modes.

    - name: multi-repo-state-migration
      type: agent
      agent: state-manager
      depends_on: [multi-repo-transition]
      condition: "human_approved_multi_repo == true"
      task: >
        Move unified specs from .factory/specs/ to .factory-project/specs/.
        Initialize .factory-project/STATE.md.
        Record project_type: multi-repo in STATE.md.
        STOP: switch to multi-repo.lobster for per-repo Phase 1
        refinement and Phase 2+.

    # =========================================================================
    # Phase 2: Story Decomposition (single-repo continues here)
    # =========================================================================

    - name: phase-2-story-decomposition
      type: skill
      skill: "skills/phase-2-story-decomposition/SKILL.md"
      depends_on: [phase-1-human-approval, multi-repo-topology-check]
      condition: "deployment_topology != 'multi-service' OR human_approved_multi_repo == false"

    - name: phase-2-consistency-check
      type: agent
      depends_on: [phase-2-story-decomposition]
      agent: consistency-validator
      task: >
        Run full consistency validation across PRD, architecture, stories, and
        dependency graph. Write report to .factory/stories/consistency-report.md.

    - name: phase-2-adversarial-review
      type: agent
      depends_on: [phase-2-consistency-check]
      agent: adversary
      model_tier: adversary
      task: >
        Review story breakdown for missing stories, oversized stories, insufficient
        verification properties, and dependency ordering issues. Fresh context.
        Write findings to .factory/stories/adversarial-reviews/ as
        per-finding files with ADV-P[N]-INDEX.md.
      context:
        include:
          - ".factory/specs/architecture/ARCH-INDEX.md"
          - ".factory/specs/architecture/*.md"
          - ".factory/specs/behavioral-contracts/**"
          - ".factory/specs/prd.md"
          - ".factory/specs/prd-supplements/**"
          - ".factory/stories/**"
        exclude:
          - ".factory/holdout-scenarios/**"

    - name: phase-2-consistency-audit
      type: agent
      agent: consistency-validator
      depends_on: [phase-2-adversarial-review]
      task: >
        Fresh-context consistency audit of ALL Phase 2 artifacts.
        Check: story↔BC coverage, AC↔BC precondition/postcondition anchors,
        VP↔story anchors, dependency graph acyclicity, wave assignments,
        traceability matrix completeness. Verify semantic correctness,
        not just structural validity.
      # MANDATORY: runs before every phase gate, not just adversarial convergence

    - name: phase-2-gate
      type: gate
      depends_on: [phase-2-adversarial-review, phase-2-consistency-audit]
      gate:
        criteria:
          - "Every PRD requirement covered by at least one story"
          - "Every BC-S.SS.NNN covered by at least one story"
          - "Every AC-NNN traces to a BC precondition/postcondition"
          - "Every VP-NNN in L4 registry has proof harness skeleton"
          - "L1→L2→L3→L4 chain completeness validated"
          - "No story exceeds 13 story points"
          - "No story exceeds 60% of implementing agent context window"
          - "Dependency graph is acyclic (topological sort succeeds)"
          - "Every story has verification properties"
          - "Consistency validation score >= 90/100"
          - "Every BC precondition, postcondition, and invariant clause has at least one AC or justified gap"
          - "Every BC edge case (EC-NNN) covered by story AC or edge case"
          - "Every error taxonomy entry (E-xxx-NNN) covered by story AC or edge case"
          - "Every P0/P1 NFR-NNN referenced by at least one story"
          - "UI stories cover all required component states from contracts"
          - "AC Completeness score >= 90% (weighted L1/L2/L3)"
          - "Every HIGH-impact ASM has holdout scenario"
          - "Every HIGH/HIGH R-NNN has holdout scenario"
          - "Every testable ASM has story with assumption_validations"
          - "Every HIGH R-NNN mitigation covered by story with risk_mitigations"
          - "Every FM-NNN has at least one holdout scenario"
          - "ASM/R Traceability section passes"
          - "PRD Out of Scope not violated by any story AC"
          - "dtu-assessment.md exists in .factory/specs/"
          - "If DTU_REQUIRED: true, at least one service classified with fidelity level"
          - "If DTU_REQUIRED: false, rationale documented in dtu-assessment.md"
        fail_action: block

    # Gemini story review — constructive 3rd perspective after adversary remediation
    - name: phase-2-spec-review-gemini
      type: loop
      depends_on: [phase-2-gate]
      loop:
        max_iterations: 10
        exit_condition: "spec_reviewer.verdict == 'APPROVED' OR spec_reviewer.findings.critical == 0"
        steps:
          - name: gemini-story-review
            type: agent
            agent: spec-reviewer
            task: >
              Review ALL Phase 2 artifacts (stories, dependency graph, holdout
              scenarios, BC coverage, traceability) AFTER adversary remediation.
              Focus on decomposition quality, AC completeness, and what the
              adversary missed. Use SR-NNN finding IDs.
              Write to .factory/stories/spec-review-pass[N].md
            context:
              include:
                - ".factory/specs/**"
                - ".factory/stories/**"
                - ".factory/holdout-scenarios/**"
              exclude:
                - ".factory/cycles/**/implementation/**"
          - name: gemini-story-remediation
            type: agent
            agent: orchestrator
            depends_on: [gemini-story-review]
            condition: "spec_reviewer.findings.critical > 0 OR spec_reviewer.findings.high > 0"
            task: >
              Triage spec-reviewer SR-NNN findings for stories. Route fixes to
              story-writer or product-owner as appropriate.

    - name: phase-2-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-2-spec-review-gemini]
      task: >
        Commit Phase 2 artifacts to factory-artifacts branch.
        Update STATE.md: phase: 2-complete.

    - name: phase-2-human-approval
      type: human-approval
      depends_on: [phase-2-state-backup]
      approval:
        prompt: >
          Phase 2 Story Decomposition is complete. Review stories, dependency graph,
          wave schedule, and holdout scenarios.
          
          Questions for review:
          - Story coverage: are ALL BCs covered? Any intentional gaps?
          - DTU scope: does dtu-assessment.md cover all integration surfaces (sensors, actions, infusions, log forwarding)?
          - Wave ordering: does the dependency graph make sense for your team's workflow?
          - Naming: are subsystem IDs, error codes, and crate names consistent with Phase 1 decisions?
          
          Approve to proceed to Phase 3 implementation.
        artifacts:
          - ".factory/stories/epics.md"
          - ".factory/stories/dependency-graph.md"
          - ".factory/stories/consistency-report.md"
          - ".factory/stories/stories/"
        timeout: "24h"

    # =========================================================================
    # Pre-Phase-3: DX Engineer Preflight (DF-027)
    # =========================================================================
    #
    # After Phase 2 human approval and before Phase 3, dx-engineer validates
    # the full toolchain, environment keys, LLM availability, and MCP access.
    # Any missing tool triggers security-reviewer audit before installation.

    - name: dx-engineer-preflight
      type: agent
      depends_on: [phase-2-human-approval]
      agent: dx-engineer
      task: >
        Run full toolchain preflight for Phase 3:
        1. Check all required tools (cargo, kani, fuzz, semgrep, vhs, etc.)
        2. For missing tools: spawn security-reviewer for supply chain audit,
           then ask human for install approval
        3. Validate .env product keys (names only, never values)
        4. Run key health checks (pass/fail only)
        5. Verify LLM availability (Claude, GPT-5.4, Gemini -- all REQUIRED)
        6. MCP preflight (mcporter -> Perplexity, Context7, Playwright, Excalidraw)
        7. If UI product: run skills/toolchain-provisioning/SKILL.md for Storybook MCP + Excalidraw MCP
        8. If UI product: run skills/storybook-mcp-integration/SKILL.md for Storybook setup
        9. SHA pinning check (--locked, .tool-versions, Docker SHA digests)
        Block pipeline on any security finding or missing required model.
      timeout: "30m"

    - name: dx-engineer-preflight-gate
      type: gate
      depends_on: [dx-engineer-preflight]
      gate:
        criteria:
          - "All required tools installed and version-checked"
          - "Supply chain security audit passed for all installed tools"
          - "Environment keys validated (names only)"
          - "All 3 model families reachable (Claude, GPT-5.4, Gemini)"
          - "MCP servers accessible via mcporter"
        fail_action: block

    # =========================================================================
    # Pre-Phase 4: DTU Clone Existence Check
    # If dtu-assessment.md says DTU_REQUIRED: true, verify clones are built
    # and validated before implementation begins.
    # =========================================================================

    - name: pre-phase-4-dtu-gate
      type: gate
      depends_on: [dx-engineer-preflight-gate]
      condition: "dtu_assessment.DTU_REQUIRED == true"
      gate:
        criteria:
          - "dtu-creation skill has been run"
          - "docker-compose.dtu.yml exists"
          - "At least one clone validation-report.md shows fidelity >= threshold"
        fail_action: block

    - name: pre-phase-4-cicd-gate
      type: gate
      depends_on: [dx-engineer-preflight-gate]
      gate:
        criteria:
          - ".github/workflows/ci.yml exists and is valid"
          - "Branch protection requires CI status checks on develop"
          - "CI pipeline runs successfully on current develop branch"
        fail_action: block

    # =========================================================================
    # Phase 3: Per-Story Code Delivery (Wave-Based with Worktrees)
    # =========================================================================
    #
    # Each wave creates worktrees, runs stub -> test -> implement -> demo ->
    # push -> PR -> AI review -> merge per story, then verifies wave integration.
    # This replaces the monolithic phase-3 + code-delivery approach (DF-024).

    - name: phase-3-per-story-delivery
      type: loop
      depends_on: [dx-engineer-preflight-gate, pre-phase-4-dtu-gate, pre-phase-4-cicd-gate]
      timeout: "12h"
      loop:
        for_each: wave in wave-schedule.md
        steps:
          # --- Create worktrees for this wave ---
          - name: create-worktrees
            type: agent
            agent: devops-engineer
            task: >
              Create git worktrees for all stories in this wave:
              git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN develop
              One worktree per story. .worktrees/ is gitignored.

          # --- Per-story: stub -> test -> implement -> demo -> push -> PR -> review -> merge ---
          - name: per-story-delivery
            type: parallel
            depends_on: [create-worktrees]
            for_each: story_group in wave.groups
            steps:
              - name: stub-architect
                type: agent
                agent: test-writer
                task: >
                  Create compilable stubs for STORY-NNN in .worktrees/STORY-NNN/.
                  cargo check must pass. Do NOT write tests or implementation.

              - name: test-writer-step
                type: agent
                agent: test-writer
                depends_on: [stub-architect]
                task: >
                  Write failing tests for STORY-NNN in .worktrees/STORY-NNN/.
                  Tests must compile but all fail (Red Gate). Do NOT implement.

              - name: implementer-step
                type: agent
                agent: implementer
                depends_on: [test-writer-step]
                task: >
                  Implement STORY-NNN via TDD in .worktrees/STORY-NNN/.
                  Pick next failing test, write minimum code, repeat.
                  All tests must pass when done.

              - name: story-demo
                type: agent
                agent: demo-recorder
                depends_on: [implementer-step]
                task: >
                  Record per-AC demos for STORY-NNN in .worktrees/STORY-NNN/.
                  Commit evidence to docs/demo-evidence/<STORY-ID>/ on the feature branch.
                timeout: "30m"

              - name: push-branch
                type: agent
                agent: implementer
                depends_on: [story-demo]
                task: >
                  Push feature/STORY-NNN to remote using --force-with-lease.
                  git push --force-with-lease origin feature/STORY-NNN

              - name: create-pr
                type: agent
                agent: pr-manager
                depends_on: [push-branch]
                task: >
                  Create PR for STORY-NNN with structured description from
                  templates/pr-description-template.md. Include Mermaid diagrams
                  (architecture, dependency, traceability), test evidence, and
                  demo evidence. Delegate gh pr create to github-ops.

              - name: ai-pr-review
                type: agent
                agent: pr-reviewer
                depends_on: [create-pr]
                task: >
                  Review PR diff for STORY-NNN. Evaluate 8-item checklist:
                  diff coherence, description accuracy, test coverage, demo
                  evidence, commit quality, diff size, missing changes,
                  dependency status. Post findings on GitHub via github-ops.
                context:
                  exclude:
                    # ▓ WALL: PR reviewer sees only the PR diff, not .factory/
                    - ".factory/**"

              # --- Security review for stories touching CRIT/HIGH modules ---
              - name: security-review-pr
                type: agent
                agent: security-reviewer
                depends_on: [create-pr]
                condition: "story.touches_critical_module == true"
                task: >
                  Security review PR diff for STORY-NNN. Post findings
                  as inline review comments via github-ops. CWE/OWASP
                  analysis of changed lines. Max 3 security review cycles.
                context:
                  exclude:
                    # ▓ WALL: no implementer reasoning
                    - ".factory/cycles/**/implementation/implementer-notes*"

              - name: pr-review-convergence
                type: loop
                depends_on: [ai-pr-review]
                loop:
                  max_iterations: 10
                  exit_condition: "pr_reviewer.verdict == 'APPROVE'"
                  steps:
                    - name: triage-findings
                      type: agent
                      agent: pr-manager
                      task: >
                        Triage pr-reviewer findings using pr-review-triage skill.
                        Route fixes to implementer/test-writer/demo-recorder.
                    - name: fix-findings
                      type: agent
                      agent: implementer
                      depends_on: [triage-findings]
                      task: >
                        Fix findings in .worktrees/STORY-NNN/, push to
                        feature branch with --force-with-lease.
                    - name: re-review
                      type: agent
                      agent: pr-reviewer
                      depends_on: [fix-findings]
                      task: "Re-review PR diff for STORY-NNN"

              - name: wait-ci
                type: agent
                agent: pr-manager
                depends_on: [pr-review-convergence]
                task: >
                  Wait for CI to pass on PR. Poll gh pr checks via github-ops.
                  Max 30 min wait, 3 fix cycles for CI failures.

              - name: dependency-merge-check
                type: agent
                agent: pr-manager
                depends_on: [wait-ci]
                task: >
                  Verify all dependency PRs for STORY-NNN are merged.
                  If any dep PR is NOT merged, WAIT. After all deps merged,
                  rebase onto develop and re-verify tests.

              - name: merge-pr
                type: agent
                agent: pr-manager
                depends_on: [dependency-merge-check]
                task: >
                  Merge PR via github-ops: gh pr merge --squash --delete-branch.
                  Read .factory/merge-config.yaml for autonomy level.

              - name: cleanup-worktree
                type: agent
                agent: devops-engineer
                depends_on: [merge-pr]
                task: >
                  Remove worktree: git worktree remove .worktrees/STORY-NNN.
                  Update .factory/STATE.md with merge status.

          # --- Wave Integration Gate (after all stories in wave merged) ---
          - name: wave-integration-gate
            type: compound
            depends_on: [per-story-delivery]
            steps:
              # 1. Full test suite on merged develop
              - name: wave-full-test
                type: agent
                agent: implementer
                task: >
                  Checkout develop (all wave N stories now merged).
                  Run the FULL test suite -- not just wave N tests, ALL tests
                  including prior waves. Report results.
                  Any failure = wave integration issue.

              # 2. Wave-level adversarial review (convergence loop on combined diff)
              - name: wave-adversarial-review
                type: loop
                depends_on: [wave-full-test]
                loop:
                  max_iterations: 10
                  exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
                  steps:
                    - name: spawn-wave-adversary
                      type: agent
                      agent: adversary
                      model_tier: adversary
                      task: >
                        Review the combined diff of all wave N stories merged to
                        develop. Fresh context. Focus on cross-story integration
                        issues, inconsistencies, shared state mutations, and API
                        contract mismatches.
                        Write findings to .factory/cycles/**/implementation/
                          wave-N-integration-review.md
                        Report CONVERGENCE_REACHED if findings are cosmetic only.
                      context:
                        include:
                          - ".factory/specs/architecture/ARCH-INDEX.md"
                          - ".factory/specs/architecture/*.md"
                          - ".factory/specs/behavioral-contracts/**"
                          - ".factory/specs/prd.md"
                          - ".factory/specs/prd-supplements/**"
                          - "src/**"
                          - "tests/**"
                        exclude:
                          # ▓ WALL: wave adversary cannot see per-story PR reviews
                          - ".factory/code-delivery/*/review-findings.md"
                          # ▓ WALL: no prior wave adversarial history
                          - ".factory/cycles/**/implementation/wave-*-integration-review.md"

                    - name: fix-wave-adversary-findings
                      type: agent
                      agent: implementer
                      depends_on: [spawn-wave-adversary]
                      condition: "adversary.has_blocking_findings == true"
                      task: >
                        Fix adversary findings on wave integration. Re-run
                        tests to verify fixes don't break anything.

              # 2b. Wave-level security review (if wave has CRIT/HIGH stories)
              - name: wave-security-review
                type: agent
                agent: security-reviewer
                depends_on: [wave-full-test]
                condition: "wave.has_critical_stories == true"
                task: >
                  Security review combined wave diff for cross-story attack
                  surfaces. Focus on: do combined changes create new attack
                  surfaces that individual story reviews could not detect?
                  Write findings to wave-security-review.md.
                context:
                  exclude:
                    # ▓ WALL: no per-story PR review comments
                    - ".factory/code-delivery/*/review-findings.md"

              # 3. Holdout regression (affected scenarios)
              - name: wave-holdout-regression
                type: agent
                agent: holdout-evaluator
                depends_on: [wave-full-test]
                task: >
                  Re-evaluate holdout scenarios that touch any module modified
                  by wave N stories. Compare to Phase 4 baseline.
                  Report regressions.

              # 4. Wave-level demo (integration user flows)
              - name: wave-demo
                type: agent
                agent: demo-recorder
                depends_on: [wave-full-test]
                task: >
                  Record wave-level integration demos on develop branch.
                  Focus on user flows that span multiple stories in this wave.
                condition: "wave.stories.length > 1"

              # 5. Integration fix loop (if issues found)
              - name: wave-integration-fix
                type: loop
                depends_on: [wave-adversarial-review, wave-holdout-regression, wave-security-review]
                condition: "wave_review.has_findings OR wave_holdout.has_regressions OR wave_security.has_findings"
                loop:
                  max_iterations: 10
                  exit_condition: "wave_integration.all_clear"
                  steps:
                    - name: triage-wave-findings
                      type: agent
                      agent: pr-manager
                      task: >
                        Triage wave integration findings. Create fix stories
                        (STORY-NNN-FIX-001) for each issue. Route through
                        the same per-story delivery flow.
                    - name: fix-and-deliver
                      type: sub-workflow
                      depends_on: [triage-wave-findings]
                      sub_workflow: "code-delivery.lobster"
                    - name: re-verify
                      type: agent
                      agent: implementer
                      depends_on: [fix-and-deliver]
                      task: "Re-run full test suite on develop after fixes merged"

          # --- DF-037 Wave-Level UI Quality Checks ---
          - name: wave-ui-responsive
            type: skill
            skill: "skills/responsive-validation/SKILL.md"
            depends_on: [wave-integration-gate]
            condition: "feature_type in ['ui', 'full-stack']"

          - name: wave-ui-performance
            type: agent
            agent: performance-engineer
            depends_on: [wave-integration-gate]
            condition: "feature_type in ['ui', 'full-stack']"
            task: >
              DF-037 D8: Lighthouse CI on wave pages. CWV targets,
              bundle size, skeleton screen check.

          - name: wave-ui-completeness
            type: skill
            skill: "skills/ui-completeness-check/SKILL.md"
            depends_on: [wave-integration-gate]
            condition: "feature_type in ['ui', 'full-stack']"

          - name: wave-ui-quality-gate
            type: skill
            skill: "skills/ui-quality-gate/SKILL.md"
            depends_on: [wave-ui-responsive, wave-ui-performance, wave-ui-completeness]
            condition: "feature_type in ['ui', 'full-stack']"

          # Wave gate -- must pass before next wave starts
          - name: wave-gate
            type: gate
            depends_on: [wave-integration-gate, wave-ui-quality-gate]
            gate:
              criteria:
                - "Full test suite passes on merged develop"
                - "Wave adversarial review converged (cosmetic only)"
                - "Holdout regression check passed (no regressions)"
                - "Wave demo recorded (if multi-story wave)"
                - "All integration fix PRs merged (if any)"
                - "UI quality gate passed (if UI product, DF-037 D16)"
              fail_action: block

    # DTU clone validation after Wave 1 merges
    - name: phase-3-dtu-validation
      type: agent
      agent: devops-engineer
      depends_on: [phase-3-per-story-delivery]
      condition: "dtu_assessment.has_candidates == true"
      task: "Start DTU clones. Run contract tests. Validate fidelity.
             Spawn dtu-validator for fidelity scoring against real APIs.
             Produce fidelity-report.md per clone."

    - name: phase-3-consistency-audit
      type: agent
      agent: consistency-validator
      depends_on: [phase-3-per-story-delivery, phase-3-dtu-validation]
      task: >
        Fresh-context consistency audit of ALL Phase 3 artifacts.
        Check: implementation↔spec alignment, test coverage↔story AC,
        purity boundary compliance, PR merge status, wave integration results.
        Verify semantic correctness, not just structural validity.
      # MANDATORY: runs before every phase gate, not just adversarial convergence

    - name: phase-3-gate
      type: gate
      depends_on: [phase-3-per-story-delivery, phase-3-dtu-validation, phase-3-consistency-audit]
      gate:
        criteria:
          - "Red Gate passed -- all tests failed before implementation"
          - "All tests now pass on merged develop"
          - "Implementation follows purity boundary map"
          - "No unwrap() in production code"
          - "Clippy/lint passes with zero warnings"
          - "All story PRs merged to develop"
          - "All wave integration gates passed"
        fail_action: block

    # =========================================================================
    # Phase 4: Holdout Evaluation (with scenario rotation)
    # Runs on merged develop after all waves complete
    # =========================================================================

    - name: phase-4-scenario-rotation
      type: agent
      depends_on: [phase-3-gate]
      agent: orchestrator
      task: >
        Perform scenario rotation for this pipeline run. Randomly select 80% of
        holdout scenarios from .factory/holdout-scenarios/ for evaluation. Write
        the selected scenario IDs to .factory/holdout-evaluation/scenario-selection.json.
        The remaining 20% are held back as "holdout of the holdout" -- scenarios
        the evaluator has never seen for this particular run. Seed the random
        selection from the pipeline run ID for deterministic per-run, different
        across-run behavior.

    # DTU clones must be running before holdout evaluation
    - name: phase-4-dtu-startup
      type: agent
      agent: devops-engineer
      depends_on: [phase-4-scenario-rotation]
      condition: "dtu_assessment.has_candidates == true"
      task: "Ensure DTU clones running. Source dtu-env.sh.
             Configure SUT to use clones instead of real services."

    - name: phase-4-holdout-evaluation
      type: skill
      skill: "skills/phase-4-holdout-evaluation/SKILL.md"
      depends_on: [phase-4-scenario-rotation, phase-4-dtu-startup]

    - name: phase-4-gate
      type: gate
      depends_on: [phase-4-holdout-evaluation]
      gate:
        criteria:
          - "Holdout evaluator used different model family (GPT-5.4, not Claude)"
          - "Holdout evaluator had NO access to source code or spec documents"
          - "Each scenario executed 3 times with satisfaction scoring (geometric mean)"
          - "Mean satisfaction score >= 0.85"
          - "No must-pass scenario below 0.6 satisfaction"
          - "Satisfaction standard deviation < 0.15"
          - "Scenario rotation applied (80% subset selected for this run)"
          - "Full scenario text was NEVER revealed to builder agents"
        fail_action: block

    # =========================================================================
    # Phase 4 Demo Recording (DF-018)
    # =========================================================================

    - name: phase-4-demo-recording
      type: skill
      skill: "skills/demo-recording/SKILL.md"
      depends_on: [phase-4-gate]
      condition: "config.demo_recording.enabled != false"
      timeout: "30m"

    # =========================================================================
    # Phase 5: Adversarial Refinement
    # =========================================================================

    # =========================================================================
    # Phases 4-6 run on fully-merged develop after all waves pass gates.
    # Any findings create fix PRs through the same per-story delivery flow.
    # =========================================================================

    - name: phase-5-adversarial-refinement
      type: loop
      depends_on: [phase-4-gate]
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: adversary-code-review
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review complete codebase with fresh context. Check spec fidelity,
              test quality, code quality, security surface, and spec gaps.
              Write findings to .factory/cycles/**/adversarial-reviews/adversarial-reviews/
              as per-finding files with ADV-P[N]-INDEX.md.
              Report CONVERGENCE_REACHED if findings are cosmetic only.
            context:
              include:
                - "src/**"
                - "tests/**"
                - ".factory/specs/architecture/ARCH-INDEX.md"
                - ".factory/specs/architecture/*.md"
                - ".factory/specs/behavioral-contracts/**"
                - ".factory/specs/verification-properties/**"
                - ".factory/specs/prd.md"
                - ".factory/specs/prd-supplements/**"
              exclude:
                # ▓ WALL: no implementation reasoning
                - ".factory/cycles/**/implementation/red-gate-log*"
                - ".factory/cycles/**/implementation/wave-schedule*"
                - ".factory/cycles/**/implementation/implementer-notes*"
                # ▓ WALL: no prior adversarial history (fresh eyes)
                - ".factory/cycles/**/adversarial-reviews/**"
                # ▓ WALL: no semport history (DF-028)
                - ".factory/semport/**"

          - name: triage-and-fix
            type: skill
            skill: "skills/phase-5-adversarial-refinement/SKILL.md"
            depends_on: [adversary-code-review]

          # Fix PRs for Phase 4 findings
          - name: phase-4-fix-delivery
            type: skill
            skill: "skills/fix-pr-delivery/SKILL.md"
            depends_on: [triage-and-fix]
            condition: "phase4.has_findings == true"

    - name: phase-5-gemini-review
      type: agent
      depends_on: [phase-5-adversarial-refinement]
      agent: code-reviewer
      model_tier: review
      task: >
        Secondary adversarial review using Gemini 3.1 Pro. Fresh context.
        Focus on patterns the GPT-5.4 adversary may have missed due to training
        bias. Write findings to .factory/cycles/**/adversarial-reviews/gemini-review.md.
      condition: "config.enable_secondary_adversary == true"
      context:
        include:
          - ".factory/specs/architecture/ARCH-INDEX.md"
          - ".factory/specs/architecture/*.md"
          - ".factory/specs/behavioral-contracts/**"
          - ".factory/specs/prd.md"
          - ".factory/specs/prd-supplements/**"
          - "src/**"
          - "tests/**"
        exclude:
          # ▓ WALL: code-reviewer cannot see adversary findings
          - ".factory/cycles/**/adversarial-reviews/**"

    # =========================================================================
    # Phase 4b: Holdout Regression Gate (conditional on behavior-changing fixes)
    # =========================================================================

    - name: phase-4b-holdout-regression
      type: skill
      skill: "skills/phase-4-holdout-evaluation/SKILL.md"
      depends_on: [phase-5-adversarial-refinement]
      condition: "phase4.has_behavior_changing_fixes == true"
      context:
        note: "This is a REGRESSION check, not a full evaluation.
               Only re-evaluate scenarios flagged as affected by
               behavior-changing fixes. Carry forward passing scores
               for unaffected scenarios."
      gate:
        - "All previously-passing holdout scenarios still pass"
        - "Any newly-failing scenarios have updated test expectations
           or documented justification"

    # =========================================================================
    # Phase 6: Formal Hardening
    # =========================================================================

    - name: phase-6-formal-hardening
      type: skill
      skill: "skills/phase-6-formal-hardening/SKILL.md"
      depends_on: [phase-5-adversarial-refinement, phase-5-gemini-review]
      timeout: "4h"
      context:
        include:
          - "src/**"
          - "tests/**"
          - ".factory/specs/verification-properties/**"
          - ".factory/specs/architecture/verification-architecture.md"
          - ".factory/specs/architecture/purity-boundary-map.md"
          - ".factory/specs/architecture/tooling-selection.md"
        exclude:
          # ▓ WALL: formal-verifier cannot see adversary history
          - ".factory/cycles/**/adversarial-reviews/**"

    # Fix PRs for Phase 5 findings (partial re-verification)
    - name: phase-5-fix-delivery
      type: skill
      skill: "skills/fix-pr-delivery/SKILL.md"
      depends_on: [phase-6-formal-hardening]
      condition: "phase5.has_findings == true"

    # DTU adversarial testing with L4 clones
    - name: phase-6-dtu-adversarial
      type: agent
      agent: formal-verifier
      depends_on: [phase-6-formal-hardening]
      condition: "dtu_clones.has_l4 == true"
      task: "Run adversarial testing against L4 DTU clones.
             dtu-validator configures failure injection, latency injection,
             partial corruption, rate limit simulation. Run SUT integration
             tests and fuzz targets against adversarial clones."

    - name: phase-5-gate
      type: gate
      depends_on: [phase-6-formal-hardening, phase-6-dtu-adversarial]
      gate:
        criteria:
          - "All Kani proofs pass"
          - "All VP-NNN in L4 registry have proofs running or documented justification"
          - "Fuzz testing: 5 minutes per target, zero crashes"
          - "Mutation kill rate > 90% (adjusted by module criticality)"
          - "Zero critical/high Semgrep findings"
          - "Cargo audit: no known vulnerabilities"
          - "Purity boundaries intact -- no side effects in pure core"
        fail_action: block

    # =========================================================================
    # Pre-Phase 6: UI Quality Loop Final Checks (DF-037)
    # =========================================================================

    - name: phase-6-heuristic-evaluation
      type: skill
      skill: "skills/ux-heuristic-evaluation/SKILL.md"
      depends_on: [phase-5-gate]
      condition: "feature_type in ['ui', 'full-stack']"
      # D6: Final heuristic evaluation on running application.

    - name: phase-6-ui-completeness-final
      type: skill
      skill: "skills/ui-completeness-check/SKILL.md"
      depends_on: [phase-5-gate]
      condition: "feature_type in ['ui', 'full-stack']"
      # D3/D10: Final completeness check. Zero gaps required.

    - name: phase-6-responsive-final
      type: skill
      skill: "skills/responsive-validation/SKILL.md"
      depends_on: [phase-5-gate]
      condition: "feature_type in ['ui', 'full-stack']"
      # D7: Full responsive suite.

    - name: phase-6-ui-quality-gate
      type: skill
      skill: "skills/ui-quality-gate/SKILL.md"
      depends_on: [phase-6-heuristic-evaluation, phase-6-ui-completeness-final, phase-6-responsive-final]
      condition: "feature_type in ['ui', 'full-stack']"
      # D16: Complete gate -- 100% on every dimension.

    - name: phase-6-ui-fix-delivery
      type: sub-workflow
      sub_workflow: "code-delivery.lobster"
      depends_on: [phase-6-ui-quality-gate]
      condition: "feature_type in ['ui', 'full-stack'] and ui_quality_gate.has_failures == true"

    # =========================================================================
    # Phase 7: Convergence
    # =========================================================================

    - name: phase-7-convergence
      type: skill
      skill: "skills/phase-7-convergence/SKILL.md"
      depends_on: [phase-5-gate, phase-6-ui-fix-delivery]

    - name: phase-6-gate
      type: gate
      depends_on: [phase-7-convergence]
      gate:
        criteria:
          - "Spec dimension: CONVERGED"
          - "Test dimension: CONVERGED"
          - "Implementation dimension: CONVERGED"
          - "Verification dimension: CONVERGED"
          - "Holdout dimension: CONVERGED (satisfaction >= 0.85)"
          - "L3 Behavioral Contract dimension: CONVERGED"
          - "L4 Verification Property dimension: CONVERGED"
          - "Adversarial convergence metrics show decay to zero"
          - "Traceability matrix generated with full L1→L4 chain"
          - "Convergence report generated"
          - "UI quality gate: PASS (100% fidelity, zero gaps, if UI product, DF-037 D16)"
          - "Heuristic evaluation >= 0.7 on all heuristics (if UI product, DF-037 D6)"
          - "Responsive validation: all breakpoints pass (if UI product, DF-037 D7)"
        fail_action: block

    # =========================================================================
    # Phase 6 Demo Recording & Visual Review (MANDATORY — DF-022)
    # =========================================================================

    - name: phase-6-final-demo
      type: agent
      agent: demo-recorder
      task: "Record final user journey demo showing the converged product.
             This is the 'money shot' for PR evidence and README."
      depends_on: [phase-6-gate]
      timeout: "30m"
      gate:
        - "demo-evidence/report.md updated with final journey recording"

    - name: phase-6-visual-review
      type: agent
      agent: visual-reviewer
      model_tier: review
      depends_on: [phase-6-final-demo]
      task: >
        Analyze all demo recordings from this pipeline run using native video
        input. Verify each recording visually demonstrates its acceptance
        criterion. Score satisfaction on functional correctness, visual quality,
        timing, and completeness. Write findings to
        .factory/demo-evidence/visual-review.md.
      context:
        include:
          - ".factory/demo-recordings/**"
          - ".factory/demo-evidence/**"
          - ".factory/stories/**"

    - name: phase-6-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-6-gate]
      task: >
        Commit final Phase 6 artifacts to factory-artifacts branch.
        Update STATE.md: phase: 6-complete, convergence: ACHIEVED.

    - name: phase-6-human-approval
      type: human-approval
      depends_on: [phase-6-state-backup, phase-6-visual-review]
      approval:
        prompt: >
          ZERO-SLOP ACHIEVED. All five convergence dimensions report CONVERGED.
          Review the convergence report and traceability matrix. Approve to
          authorize deployment.
        artifacts:
          - ".factory/cycles/**/convergence/convergence-report.md"
          - ".factory/cycles/**/convergence/traceability-matrix.md"
        timeout: "48h"

    # =========================================================================
    # Release Pipeline (DF-025)
    # =========================================================================

    - name: release
      type: agent
      agent: devops-engineer
      depends_on: [phase-6-human-approval]
      task: >
        Execute release pipeline: determine semver version from story types,
        generate CHANGELOG.md, create annotated git tag, push to trigger
        release.yml CI, create GitHub Release with binaries and demo evidence,
        update README.md version badge and installation instructions.
        See skills/release/SKILL.md for full protocol.

    # =========================================================================
    # Steady-State Handoff (DF-030)
    # Transition from "building a product" to "maintaining and evolving"
    # =========================================================================

    - name: steady-state-handoff
      type: agent
      agent: orchestrator
      depends_on: [release]
      task: >
        Transition to steady-state. Delegate to state-manager:
        1. Archive greenfield cycle artifacts to cycles/vX.Y.Z-greenfield/
           (adversarial reviews, convergence report, wave schedule, cost).
        2. Create cycle manifest using templates/cycle-manifest-template.md.
        3. Snapshot factory-artifacts branch (git tag vX.Y.Z).
        4. Initialize tech-debt-register.md from template.
        5. Update STATE.md: pipeline: STEADY-STATE, last_release: vX.Y.Z.
        6. Enable maintenance sweep schedule (Path 10).
        7. Enable discovery schedule (Path 8, if configured).
        8. Notify human: "vX.Y.Z released. Factory is now in steady-state mode.
           Maintenance sweeps running weekly. Ready for feature requests."

    # =========================================================================
    # Post-Feature Validation (DF-034, optional)
    # =========================================================================

    - name: post-feature-validation
      type: agent
      agent: orchestrator
      depends_on: [steady-state-handoff]
      condition: "config.post_feature_validation.enabled == true"
      task: >
        Schedule post-feature validation checks at 7, 30, and 90 days:
        feature adoption rate, customer feedback, error rates, issues reported.
        Verdict: SUCCESS / PARTIAL / MISS.
        Feeds back into discovery engine for next iteration.

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [post-feature-validation, release]
      # Runs after all pipeline work, including release

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
