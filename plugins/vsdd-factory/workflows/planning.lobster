workflow:
  name: adaptive-planning
  description: >
    Adaptive front-end for the VSDD pipeline. Detects existing artifacts,
    validates quality, identifies gaps, and routes to the correct entry point.
    Supports both Collaborative Discovery (from ideas) and Spec Intake
    (from existing artifacts).

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    Key additions from DF-029: environment setup as first step for
    research agent MCP access, 4-level hierarchy detection (BC-S.SS.NNN),
    sharded architecture handling, state-manager commits, cost monitoring.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  cost_monitoring:
    enabled: true
    metadata:
      mode: planning

  steps:
    # =========================================================================
    # Environment Setup (cross-cutting, DF-027)
    # =========================================================================

    - name: environment-setup
      type: agent
      agent: dx-engineer
      depends_on: []
      task: >
        Run environment setup:
        1. Install tools (direnv, just, lefthook, mcporter)
        2. LLM health check (Claude, GPT-5.4, Gemini — all REQUIRED)
        3. MCP preflight (mcporter call → verify Perplexity, Context7, Tally)
        4. Supply chain audit via security-reviewer (before installs)
        Block on any security finding or missing required model.
      timeout: "15m"

    - name: environment-gate
      type: gate
      depends_on: [environment-setup]
      gate:
        criteria:
          - "All required tools installed"
          - "All 3 model families reachable"
          - "MCP servers accessible via mcporter"
        fail_action: block

    # =========================================================================
    # Step 1: Artifact Detection & Routing
    # Updated for 4-level hierarchy (BC-S.SS.NNN) and sharded architecture
    # =========================================================================

    - name: artifact-detection
      type: skill
      skill: "skills/artifact-detection/SKILL.md"
      depends_on: [environment-gate]

    - name: routing-gate
      type: gate
      depends_on: [artifact-detection]
      gate:
        criteria:
          - "Artifact inventory written to .factory/planning/artifact-inventory.md"
          - "Gap analysis written to .factory/planning/gap-analysis.md"
          - "Routing decision recorded in .factory/planning/routing-decision.md"
          - "Format detection: BC-S.SS.NNN (current) or FR-NNN (legacy) identified"
          - "Structure detection: ARCH-INDEX.md or architecture.md identified"
        fail_action: block

    - name: state-backup-routing
      type: agent
      agent: state-manager
      depends_on: [routing-gate]
      task: >
        Commit planning artifacts to factory-artifacts.
        Update STATE.md: phase: planning, step: artifact-detection-complete.

    # =========================================================================
    # Market Intelligence Assessment (runs before any spec work)
    # Depth scales by level: L0 full landscape -> L4 freshness check
    # =========================================================================

    - name: market-intelligence
      type: skill
      skill: "skills/market-intelligence-assessment/SKILL.md"
      depends_on: [routing-gate]
      config:
        depth_by_level:
          L0: "full-landscape"
          L1: "validation"
          L2: "competitive-deep-dive"
          L3: "risk-check"
          L4: "freshness-check"

    - name: market-intel-review
      type: human-approval
      depends_on: [market-intelligence]
      approval:
        prompt: >
          Market intelligence assessment complete. Review the competitive
          landscape, customer pain validation, and risk signals. The
          recommendation is [GO/CAUTION/STOP]. Approve to proceed,
          or provide direction (pivot, research more, abort).
        artifacts:
          - ".factory/planning/market-intel.md"
        timeout: "48h"

    - name: state-backup-market-intel
      type: agent
      agent: state-manager
      depends_on: [market-intel-review]
      task: >
        Commit market intel artifacts to factory-artifacts.
        Update STATE.md: step: market-intel-complete.

    # =========================================================================
    # Collaborative Discovery Track (L0 — nothing exists)
    # research-agent uses  for all MCP access
    # =========================================================================

    - name: brainstorming
      type: skill
      skill: "skills/brainstorming/SKILL.md"
      depends_on: [market-intel-review]
      condition: "routing.level == 'L0' and routing.choice == 'brainstorm'"

    - name: planning-research
      type: skill
      skill: "skills/planning-research/SKILL.md"
      depends_on: [market-intel-review]
      condition: "routing.level == 'L0' and routing.choice == 'research'"
      # research-agent accesses Perplexity/Context7 (DF-027)

    - name: guided-brief-creation
      type: skill
      skill: "skills/guided-brief-creation/SKILL.md"
      depends_on: [market-intel-review, brainstorming, planning-research]
      condition: "routing.level == 'L0'"

    - name: brief-validation
      type: skill
      skill: "skills/validate-brief/SKILL.md"
      depends_on: [guided-brief-creation]
      condition: "routing.level == 'L0'"
      # Cross-checks brief claims against market-intel.md findings (DF-033 D4)

    - name: brief-approval
      type: human-approval
      depends_on: [brief-validation]
      condition: "routing.level == 'L0'"
      approval:
        prompt: >
          Product brief created through collaborative discovery. Review the
          brief and approve to proceed to Phase 1 (Spec Crystallization).
        artifacts:
          - ".factory/planning/product-brief.md"
          - ".factory/planning/brief-validation.md"
          - ".factory/planning/brainstorming-report.md"
          - ".factory/planning/research-report.md"
          - ".factory/planning/market-intel.md"
        timeout: "24h"

    - name: state-backup-l0
      type: agent
      agent: state-manager
      depends_on: [brief-approval]
      condition: "routing.level == 'L0'"
      task: >
        Commit L0 planning artifacts to factory-artifacts.
        Update STATE.md: step: brief-approved.

    # =========================================================================
    # Spec Intake Track (L1-L4 — artifacts exist)
    # Validates against 4-level hierarchy (BC-S.SS.NNN)
    # Handles both old (architecture.md) and new (ARCH-INDEX.md) formats
    # =========================================================================

    - name: validate-existing-brief
      type: skill
      skill: "skills/validate-brief/SKILL.md"
      depends_on: [market-intel-review]
      condition: "routing.level == 'L1'"

    - name: validate-existing-prd
      type: agent
      depends_on: [market-intel-review]
      condition: "routing.level in ['L2', 'L3', 'L4']"
      agent: consistency-validator
      task: >
        Validate the existing PRD against quality criteria: unique IDs
        (BC-S.SS.NNN format preferred, FR-NNN flagged for migration),
        measurable acceptance criteria, edge case catalog, SMART requirements.
        If PRD uses old FR-NNN format, flag for migration before proceeding.
        Write validation report to .factory/planning/prd-validation.md.

    - name: validate-existing-architecture
      type: agent
      depends_on: [routing-gate]
      condition: "routing.level in ['L3', 'L4']"
      agent: architect
      task: >
        Validate architecture document. Handle both formats:
        - New format: ARCH-INDEX.md + section files (verify all sections present)
        - Old format: single architecture.md (flag for sharding migration)
        Check: component inventory, technology stack, purity boundary map,
        verification properties, consistency with PRD.
        Write report to .factory/planning/architecture-validation.md.

    - name: implementation-readiness
      type: skill
      skill: "skills/implementation-readiness/SKILL.md"
      depends_on: [routing-gate]
      condition: "routing.level == 'L4'"

    - name: intake-approval
      type: human-approval
      depends_on:
        - validate-existing-brief
        - validate-existing-prd
        - validate-existing-architecture
        - implementation-readiness
      condition: "routing.level in ['L1', 'L2', 'L3', 'L4']"
      approval:
        prompt: >
          Existing artifacts validated. Review the gap analysis and validation
          reports. Approve to proceed to the first missing pipeline step.
        artifacts:
          - ".factory/planning/gap-analysis.md"
          - ".factory/planning/brief-validation.md"
          - ".factory/planning/prd-validation.md"
          - ".factory/planning/architecture-validation.md"
          - ".factory/planning/readiness-report.md"
          - ".factory/planning/market-intel.md"
        timeout: "24h"

    - name: state-backup-intake
      type: agent
      agent: state-manager
      depends_on: [intake-approval]
      condition: "routing.level in ['L1', 'L2', 'L3', 'L4']"
      task: >
        Commit intake planning artifacts to factory-artifacts.
        Update STATE.md: step: intake-approved, routing_level: L[N].

    # =========================================================================
    # Route to Pipeline (both tracks converge here)
    # =========================================================================

    - name: start-pipeline
      type: sub-workflow
      depends_on: [brief-approval, intake-approval]
      sub_workflow: "greenfield.lobster"
      # The greenfield workflow is entered at the appropriate step based on
      # routing.level. L0/L1 start at Phase 1 Step 1. L2 skips to architecture.
      # L3 skips to Phase 2. L4 skips to Phase 3.

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # Reviews the planning run itself — research quality, market intel accuracy,
    # brainstorming effectiveness, scoring calibration, planning cost.
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [start-pipeline]
      # Runs after routing to pipeline completes

    - name: session-review-approval
      type: human-approval
      depends_on: [session-review]
      approval:
        prompt: >
          Session review complete. Review the analysis and improvement proposals
          for the planning run. For each proposal: Approve (create task),
          Defer (add to backlog), or Reject (log reason).
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
