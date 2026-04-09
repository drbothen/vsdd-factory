workflow:
  name: multi-repo-vsdd
  description: >
    Orchestrates the VSDD pipeline across multiple repositories. Reads the project
    manifest (project.yaml) to determine repo-level dependencies and wave ordering.
    Each repo runs its own greenfield/brownfield/feature pipeline, coordinated by
    cross-repo gates.

    Cross-cutting infrastructure (DF-020-028) applies per-repo via the appropriate
    sub-workflow. See docs/factory-paths.md Cross-Cutting Infrastructure section
    for shared baseline.
    Key additions from DF-029: per-repo environment setup, per-repo CI/CD,
    repo-level wave computation (Kahn's algorithm), per-repo sub-workflows
    with cwd, cross-repo worktrees and PR coordination, cross-repo integration
    gate (Docker Compose + e2e + holdout + adversary + security), cross-repo
    information asymmetry walls, cross-repo DTU, contract change detection,
    SDK generation for generated repos, coordinated release, mixed mode support,
    market intel at project level, .factory-project/ as git worktree,
    state-manager at project + per-repo levels, cost monitoring aggregated.
    Key additions from DF-032: multi-repo detection activation, per-repo mode
    classification (greenfield/brownfield/feature/generated), multi-repo brownfield
    Phase 0 with project-level synthesis, workspace configuration from project.yaml,
    cross-repo information asymmetry wall enforcement, cross-repo cost aggregation,
    multi-repo feature/bug-fix/maintenance paths.
  version: "3.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "4h"

  cost_monitoring:
    enabled: true
    metadata:
      mode: multi-repo
      level: project

  # ===========================================================================
  # Cross-Repo Information Asymmetry Walls (DF-025 + DF-032 D7)
  #
  # These walls extend single-repo walls to multi-repo context:
  #
  # | Agent                      | Multi-Repo Wall Extension                          |
  # |----------------------------|---------------------------------------------------|
  # | holdout-evaluator          | Cannot see ANY repo's source, specs, impl notes    |
  # | pr-reviewer                | Cannot see .factory-project/ or any .factory/       |
  # | adversary (cross-repo)     | Cannot see any repo's TDD logs or adversary history |
  # | security-reviewer (cross)  | Sees cross-repo API contracts + integration only    |
  #
  # Enforced via context.exclude blocks on cross-repo gate steps below.
  # ===========================================================================

  steps:
    # =========================================================================
    # Phase 0: Project-Level Planning
    # =========================================================================

    - name: environment-setup
      type: agent
      agent: dx-engineer
      depends_on: []
      task: >
        Per-repo environment setup: install tools for ALL repo languages
        (e.g., cargo + node + python if project spans all).
        LLM health, MCP preflight, supply chain audit.
        Block on any security finding or missing required model.
      timeout: "30m"

    - name: read-project-manifest
      type: agent
      agent: orchestrator
      depends_on: [environment-setup]
      task: >
        Read project.yaml and determine: which repos exist, their languages/frameworks,
        cross-repo dependencies, and wave ordering. Classify each repo mode
        (greenfield/brownfield/feature/generated). Write project plan to
        .factory-project/project-plan.md.

    - name: compute-repo-waves
      type: agent
      agent: orchestrator
      depends_on: [read-project-manifest]
      task: >
        Compute repo-level waves from the dependency_graph in project.yaml
        using Kahn's algorithm at repo level.
        Write wave plan to .factory-project/repo-waves.md.
        Detect circular repo dependencies and report as errors.
        Distinguish contract dependencies (consumer starts after spec approval)
        from generation dependencies (blocks until contract is stable).

    - name: per-repo-mode-detection
      type: agent
      agent: orchestrator
      depends_on: [read-project-manifest]
      task: >
        For each repo in project.yaml, classify mode independently:
        - Repo has source code + no project-context.md -> brownfield (Phase 0)
        - Repo has source code + has project-context.md -> feature (skip Phase 0)
        - Repo path empty or nonexistent -> greenfield (skip Phase 0)
        - Repo role is "generated" -> generated (sdk-generation skill)
        Record per-repo modes in .factory-project/STATE.md.

    - name: per-repo-setup
      type: agent
      agent: devops-engineer
      depends_on: [compute-repo-waves, per-repo-mode-detection]
      task: >
        Per-repo setup: verify/create remotes, CI/CD workflows, .factory/
        worktrees on factory-artifacts branches, branch protection.
        Project-level: .factory-project/ as worktree on factory-project-artifacts.
        Per-repo merge-config.yaml, git rerere.

    - name: state-init
      type: agent
      agent: state-manager
      depends_on: [per-repo-setup]
      task: >
        Initialize .factory-project/STATE.md (project level) and
        per-repo STATE.md files. Record project_type: multi-repo,
        per-repo modes, and repo-wave assignments.
        Commit to factory-project-artifacts.

    - name: configure-workspaces
      type: agent
      agent: orchestrator
      depends_on: [state-init]
      task: >
        Generate openclaw.json workspace configuration for multi-repo:
        - T1 agents (read-only): array workspace spanning all repos
        - T2 agents (spec-writers): workspace scoped to assigned repo
        - T3 agents (coding): workspace scoped to assigned repo
        - Cross-repo agents (architect, consistency-validator, holdout-evaluator):
          array workspace spanning all repos
        - Apply tools.deny lists per DF-023 tier model
        Agent workspace scoping:
        | orchestrator (T1)         | All repos                |
        | codebase-analyzer (T1)    | All repos (read-only)    |
        | architect (T2)            | All repos                |
        | consistency-validator (T2)| All repos                |
        | implementer (T3)          | Single repo (assigned)   |
        | test-writer (T3)          | Single repo (assigned)   |
        | formal-verifier (T3)      | Single repo (assigned)   |
        | holdout-evaluator (T3)    | All repos (read-only)    |
        | e2e-tester (T3)           | All repos                |
        | devops-engineer (T3)      | All repos                |
        | adversary (T2)            | Per-repo during phases,  |
        |                           | all repos at cross-repo  |

    # =========================================================================
    # Multi-Repo Brownfield Phase 0 (DF-032 D4)
    # Runs per-repo Phase 0 for brownfield repos, then project-level synthesis.
    # Greenfield repos skip this section entirely.
    # =========================================================================

    - name: per-repo-phase-0
      type: parallel-foreach
      collection: "repos.where(mode == 'brownfield')"
      depends_on: [configure-workspaces]
      condition: "any_repo_mode == 'brownfield'"
      iterator:
        - name: "phase-0-${repo.name}"
          type: sub-workflow
          sub_workflow: "brownfield.lobster"
          cwd: "./${repo.name}"
          task: >
            Run DF-031 upgraded Phase 0 (all 12 steps) for ${repo.name}:
            0a -> 0b -> 0c -> 0d (+ spec-steward review) -> 0e ->
            0e-sec (block) -> 0e.5 -> 0f-pre -> 0f -> 0f-adv -> 0f-post -> gate
            Produces: project-discovery.md, recovered-architecture.md,
            conventions.md, behavioral-contracts/*.md, verification-gap-analysis.md,
            security-audit.md, module-criticality.md, holdout-scenarios/*.md,
            project-context.md, validation-report.md.
            state-manager commits per-repo artifacts to factory-artifacts.

    - name: project-level-synthesis
      type: skill
      skill: "skills/multi-repo-phase-0-synthesis/SKILL.md"
      depends_on: [per-repo-phase-0]
      condition: "any_repo_mode == 'brownfield'"
      task: >
        Synthesize per-repo Phase 0 outputs into unified project-level context.
        Produces:
        - .factory-project/phase-0-synthesis/cross-repo-dependencies.md
        - .factory-project/phase-0-synthesis/unified-architecture.md
        - .factory-project/phase-0-synthesis/convention-reconciliation.md
        - .factory-project/phase-0-synthesis/unified-security-posture.md
        - .factory-project/cross-repo-holdout-scenarios/*.md
        - .factory-project/phase-0-synthesis/project-context.md
        - .factory-project/phase-0-synthesis/synthesis-validation-report.md

    - name: project-phase-0-gate
      type: human-approval
      depends_on: [project-level-synthesis]
      condition: "any_repo_mode == 'brownfield'"
      approval:
        prompt: >
          Multi-repo Phase 0 complete. Review the unified project context:
          - Unified architecture spanning all repos
          - Cross-repo dependencies and API contracts
          - Convention conflicts between repos
          - Unified security posture
          - Per-repo ingestion accuracy
          Approve to proceed to Phase 1.
        artifacts:
          - ".factory-project/phase-0-synthesis/project-context.md"
          - ".factory-project/phase-0-synthesis/unified-architecture.md"
          - ".factory-project/phase-0-synthesis/cross-repo-dependencies.md"
          - ".factory-project/phase-0-synthesis/convention-reconciliation.md"
          - ".factory-project/phase-0-synthesis/unified-security-posture.md"
        timeout: "72h"

    - name: post-phase-0-routing
      type: agent
      agent: orchestrator
      depends_on: [project-phase-0-gate]
      condition: "any_repo_mode == 'brownfield'"
      task: >
        Post-Phase 0 routing (per DF-031 gap 4). Ask human:
        - Feature in mind -> proceed to Phase 1 (multi-repo greenfield, wave 0+)
        - Explore -> route to Path 5 (Adaptive Planning)
        - Neither -> park (STATE.md: phase: 0-complete, awaiting: feature-request)

    # =========================================================================
    # Market Intelligence (DF-033, project level -- not per-repo)
    # =========================================================================

    - name: market-intelligence
      type: skill
      skill: "skills/market-intelligence-assessment/SKILL.md"
      depends_on: [configure-workspaces]
      condition: "no_brownfield_repos OR post_phase_0_routing == 'feature'"
      config:
        scope: "project"
        note: "Single assessment for entire product, not per-repo"

    - name: market-intel-review
      type: human-approval
      depends_on: [market-intelligence]
      approval:
        prompt: >
          Market intelligence for the multi-repo product is complete.
          Review the assessment. Approve to proceed.
        artifacts:
          - ".factory-project/planning/market-intel.md"
        timeout: "48h"

    # =========================================================================
    # Wave 0: Primary Services (no cross-repo dependencies)
    # Per-repo sub-workflows with cwd
    # =========================================================================

    - name: wave-0-spec
      type: skill
      skill: "skills/phase-1-spec-crystallization/SKILL.md"
      depends_on: [market-intel-review]
      cwd: "./api-server"
      condition: "repo_waves[0].includes('api-server')"

    - name: wave-0-spec-approval
      type: human-approval
      depends_on: [wave-0-spec]
      approval:
        prompt: >
          Wave 0 (api-server) spec crystallization complete. Review the spec package
          including API contracts (OpenAPI). These contracts will be used by consumer
          repos in Wave 1. Approve to proceed.
        artifacts:
          - "./api-server/.factory/specs/prd.md"
          - "./api-server/.factory/specs/architecture/ARCH-INDEX.md"
          - "./api-server/openapi.yaml"

    - name: wave-0-impl
      type: sub-workflow
      sub_workflow: "greenfield.lobster"
      depends_on: [wave-0-spec-approval]
      cwd: "./api-server"
      cost_monitoring:
        repo: "api-server"
      # All cross-cutting infrastructure applies per-repo via greenfield.lobster

    - name: wave-0-state-commit
      type: agent
      agent: state-manager
      depends_on: [wave-0-impl]
      task: >
        Commit wave 0 artifacts to factory-project-artifacts.
        Update .factory-project/STATE.md: wave-0 complete.
        Update per-repo STATE.md for api-server.

    # =========================================================================
    # Contract Change Detection (after wave 0 implementation)
    # =========================================================================

    - name: contract-change-detection
      type: agent
      agent: consistency-validator
      depends_on: [wave-0-impl]
      task: >
        Detect if any API contract has changed since last SDK generation:
        - Compare current OpenAPI/protobuf/GraphQL schemas against cached versions
        - Identify backward-incompatible changes (removed fields, changed types)
        - List all consumer repos affected by the changes
        Write analysis to .factory-project/contract-changes.md.
        Write contract registry to .factory-project/contract-registry.md.

    # =========================================================================
    # Wave 1+: Consumer Services (depend on Wave 0 contracts)
    # Per-repo sub-workflows with cwd and mode classification
    # =========================================================================

    - name: wave-1-frontend
      type: sub-workflow
      sub_workflow: "greenfield.lobster"
      depends_on: [wave-0-spec-approval]
      cwd: "./frontend"
      cost_monitoring:
        repo: "frontend"
      # Mode determined by per-repo classification (mixed mode support)
      # Consumer repos start after wave 0 spec approval (contract dependency)

    - name: wave-1-sdk-gen
      type: skill
      skill: "skills/sdk-generation/SKILL.md"
      depends_on: [wave-0-spec-approval]
      # SDK generation runs once contracts are approved, doesn't wait for full impl
      # SDK repos (role: generated) use sdk-generation skill, not full greenfield

    - name: sdk-regeneration
      type: skill
      skill: "skills/sdk-generation/SKILL.md"
      depends_on: [contract-change-detection]
      condition: "contract_changes.detected == true"

    - name: sdk-validation
      type: agent
      agent: consistency-validator
      depends_on: [sdk-regeneration]
      condition: "contract_changes.detected == true"
      task: >
        Run consumer repos' test suites against regenerated SDKs.
        Flag breaking changes that require MAJOR version bump.
        Write validation results to .factory-project/sdk-validation.md.

    - name: wave-1-state-commit
      type: agent
      agent: state-manager
      depends_on: [wave-1-frontend, wave-1-sdk-gen]
      task: >
        Commit wave 1 artifacts to factory-project-artifacts.
        Update .factory-project/STATE.md: wave-1 complete.
        Update per-repo STATE.md for frontend and SDK repos.

    # =========================================================================
    # Cross-Repo Integration Gate
    # Docker Compose + e2e + holdout + adversary + security
    # Information asymmetry walls enforced (DF-025 + DF-032 D7)
    # =========================================================================

    - name: cross-repo-docker-env
      type: agent
      agent: devops-engineer
      depends_on: [wave-0-impl, wave-1-frontend, wave-1-sdk-gen]
      task: >
        Start Docker Compose integration environment with all repos.
        Verify all services start and communicate.

    - name: cross-repo-e2e
      type: agent
      agent: e2e-tester
      depends_on: [cross-repo-docker-env]
      task: >
        Run cross-repo integration tests (e2e).
        Test user flows that span multiple repos.
        Write results to .factory-project/integration-results/.

    - name: cross-repo-holdout
      type: skill
      skill: "skills/phase-3.5-holdout-evaluation/SKILL.md"
      depends_on: [cross-repo-docker-env]
      context:
        exclude:
          # Information asymmetry wall (DF-032 D7):
          # holdout-evaluator cannot see ANY repo's source code, specs, or impl notes
          - "./**/src/**"
          - "./**/lib/**"
          - "./**/app/**"
          - "**/.factory/specs/**"
          - "**/.factory/cycles/**/implementation/**"
          - "**/.factory/phase-*/**"
          - ".factory-project/phase-0-synthesis/**"

    - name: cross-repo-adversary
      type: agent
      agent: adversary
      model_tier: adversary
      depends_on: [cross-repo-docker-env]
      task: >
        Review cross-repo integration points. Fresh context.
        Focus on contract compliance, authentication flow, error propagation.
      context:
        exclude:
          # Information asymmetry wall (DF-032 D7):
          # Cannot see any repo's Phase 3 TDD logs or per-repo adversary history
          - "**/.factory/cycles/**/implementation/**"
          - "**/.factory/semport/**"
          - "**/.factory/phase-4-adversarial/**"

    - name: cross-repo-security
      type: agent
      agent: security-reviewer
      depends_on: [cross-repo-docker-env]
      task: >
        Cross-repo security scan. Focus on: authentication flow across repos,
        API key handling, data flow between services, network boundaries.
      context:
        exclude:
          # Information asymmetry wall (DF-032 D7):
          # Sees cross-repo API contracts + integration config only
          - "**/.factory/cycles/**/implementation/**/reasoning/**"
          - "**/.factory/phase-1-specs/**/rationale/**"

    - name: cross-repo-a11y
      type: agent
      agent: accessibility-auditor
      depends_on: [cross-repo-docker-env]
      condition: "project.has_ui_repos == true"
      task: >
        Cross-repo accessibility audit on integrated UI.
      context:
        exclude:
          - "**/.factory/specs/architecture/**"

    - name: cross-repo-pr-review
      type: agent
      agent: pr-reviewer
      depends_on: [cross-repo-docker-env]
      task: >
        Cross-repo PR review of integrated changes.
      context:
        exclude:
          # Information asymmetry wall (DF-032 D7):
          # pr-reviewer cannot see .factory-project/ OR any repo's .factory/
          - "**/.factory/**"
          - ".factory-project/**"

    - name: integration-gate
      type: gate
      depends_on: [cross-repo-e2e, cross-repo-holdout, cross-repo-adversary, cross-repo-security, cross-repo-a11y, cross-repo-pr-review]
      gate:
        criteria:
          - "All repos have passing test suites"
          - "API contracts are consistent across all consumers"
          - "Docker Compose integration environment starts successfully"
          - "Cross-repo e2e tests pass"
          - "Cross-repo holdout scenarios pass"
          - "Cross-repo adversary review converged"
          - "Cross-repo security scan clean"
        fail_action: block

    - name: integration-gate-state-commit
      type: agent
      agent: state-manager
      depends_on: [integration-gate]
      task: >
        Commit integration gate results to factory-project-artifacts.
        Update .factory-project/STATE.md: integration-gate passed.
        Write integration results to .factory-project/integration-results/.

    # =========================================================================
    # Cross-Repo Convergence
    # 5-dimensional convergence across integrated system
    # =========================================================================

    - name: cross-repo-convergence
      type: skill
      skill: "skills/phase-6-convergence/SKILL.md"
      depends_on: [integration-gate]

    - name: convergence-human-approval
      type: human-approval
      depends_on: [cross-repo-convergence]
      approval:
        prompt: >
          Cross-repo convergence achieved. Review the integrated convergence
          report. Approve to authorize coordinated release.
        artifacts:
          - ".factory-project/convergence-report.md"
        timeout: "48h"

    # =========================================================================
    # Coordinated Release
    # Per-repo in dependency order: semver -> CHANGELOG -> tag
    # Project-level release notes spanning all repos
    # =========================================================================

    - name: coordinated-release
      type: agent
      agent: devops-engineer
      depends_on: [convergence-human-approval]
      task: >
        Coordinated multi-repo release:
        1. Per-repo (in dependency order): semver version, CHANGELOG, tag
        2. Publish packages in dependency order (API before consumers)
        3. Project-level release notes spanning all repos
        4. GitHub Releases for each repo
        5. Write release plan to .factory-project/release/release-plan.md
        6. Write per-repo changelogs to .factory-project/release/per-repo-changelogs/
        7. Notify human: multi-repo release published.

    - name: state-final
      type: agent
      agent: state-manager
      depends_on: [coordinated-release]
      task: >
        Commit final release artifacts to factory-project-artifacts.
        Update .factory-project/STATE.md: pipeline: COMPLETE.
        Update per-repo STATE.md files.
        Cost monitoring final aggregation at project level.
        Write project-level cost summary to .factory-project/cost-summary.md.

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [coordinated-release]
      # Runs after coordinated release (final multi-repo step)

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

  # ===========================================================================
  # Multi-Repo Feature Mode (Path 3 + Path 11) -- DF-032 D10
  #
  # When a feature request affects multiple repos in a multi-repo project,
  # feature.lobster runs per-repo with cross-repo coordination.
  # This is invoked by the orchestrator when mode=feature + project_type=multi-repo.
  # ===========================================================================

  feature_mode:
    steps:
      - name: cross-repo-delta-analysis
        type: agent
        agent: orchestrator
        task: >
          F1: Cross-repo delta analysis.
          Parse project.yaml, identify affected repos.
          Spawn architect for per-repo impact boundary + cross-repo impact
          (e.g., API change in api-server affects frontend + SDKs).
          If API contracts change: primary service first in wave ordering.

      - name: per-repo-spec-evolution
        type: parallel-foreach
        collection: "affected_repos"
        depends_on: [cross-repo-delta-analysis]
        iterator:
          - name: "f2-f3-${repo.name}"
            type: sub-workflow
            sub_workflow: "feature.lobster"
            cwd: "./${repo.name}"
            phases: ["F2", "F3"]
            task: >
              Per-repo F2-F3: spec evolution + story creation.

      - name: cross-repo-consistency-check
        type: agent
        agent: consistency-validator
        depends_on: [per-repo-spec-evolution]
        task: >
          Cross-repo consistency check after F2-F3:
          Are contract changes propagated to all consumers?
          Do inter-repo interfaces remain compatible?

      - name: per-repo-wave-loops
        type: parallel-foreach
        collection: "affected_repos_by_wave"
        depends_on: [cross-repo-consistency-check]
        iterator:
          - name: "f4-${repo.name}"
            type: sub-workflow
            sub_workflow: "feature.lobster"
            cwd: "./${repo.name}"
            phases: ["F4"]
            task: >
              Per-repo F4 wave loop. Primary service first if contracts change.

      - name: feature-cross-repo-integration
        type: gate
        depends_on: [per-repo-wave-loops]
        gate:
          criteria:
            - "Cross-repo integration gate (Docker Compose + e2e + holdout + adversary + security)"

      - name: per-repo-hardening
        type: parallel-foreach
        collection: "affected_repos"
        depends_on: [feature-cross-repo-integration]
        iterator:
          - name: "f5-f7-${repo.name}"
            type: sub-workflow
            sub_workflow: "feature.lobster"
            cwd: "./${repo.name}"
            phases: ["F5", "F6", "F7"]

      - name: feature-coordinated-release
        type: agent
        agent: devops-engineer
        depends_on: [per-repo-hardening]
        task: >
          Coordinated MINOR/PATCH release across affected repos (dependency order).

  # ===========================================================================
  # Multi-Repo Bug Fix (Path 4 + Path 11) -- DF-032 D11
  #
  # Single-repo bug fix with cross-repo regression check when the fix
  # touches API contracts consumed by other repos.
  # ===========================================================================

  bugfix_mode:
    steps:
      - name: bug-analysis
        type: agent
        agent: orchestrator
        task: >
          F1: Bug analysis (single repo).
          If fix touches API contract consumed by other repos:
          set cross_repo_regression_flag = true.

      - name: single-repo-fix
        type: sub-workflow
        sub_workflow: "feature.lobster"
        depends_on: [bug-analysis]
        cwd: "./${affected_repo}"
        phases: ["F4"]
        task: >
          Fix story delivery in the affected repo, per Path 4.

      - name: cross-repo-regression
        type: agent
        agent: e2e-tester
        depends_on: [single-repo-fix]
        condition: "cross_repo_regression_flag == true"
        task: >
          Run consumer repo test suites against fixed API.
          Cross-repo integration check.

      - name: bugfix-release
        type: agent
        agent: devops-engineer
        depends_on: [cross-repo-regression, single-repo-fix]
        task: >
          PATCH release: single-repo if no contract change,
          coordinated PATCH if consumers need update.

  # ===========================================================================
  # Multi-Repo Maintenance (Path 10 + Path 11) -- DF-032 D12
  #
  # Per-repo sweeps run in parallel, followed by cross-repo integration check.
  # ===========================================================================

  maintenance_mode:
    steps:
      - name: per-repo-sweeps
        type: parallel-foreach
        collection: "all_repos"
        task: >
          Per-repo: 9 maintenance sweeps each (parallel across repos).
        iterator:
          - name: "maintenance-${repo.name}"
            type: sub-workflow
            sub_workflow: "maintenance.lobster"
            cwd: "./${repo.name}"

      - name: cross-repo-maintenance-checks
        type: agent
        agent: consistency-validator
        depends_on: [per-repo-sweeps]
        task: >
          Cross-repo checks after per-repo sweeps:
          - Contract drift between repos (API changed in one, not consumed)
          - Cross-repo integration test suite
          - Dependency version consistency across repos

      - name: cross-repo-integration-test
        type: agent
        agent: e2e-tester
        depends_on: [cross-repo-maintenance-checks]
        task: >
          Cross-repo integration test after maintenance fixes.

      - name: maintenance-fix-prs
        type: agent
        agent: devops-engineer
        depends_on: [cross-repo-integration-test]
        task: >
          Fix PRs per-repo. Cross-repo regression check after fixes.
