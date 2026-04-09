workflow:
  name: autonomous-discovery
  description: >
    Autonomous discovery engine that continuously researches opportunities
    for new features (existing products) and new product concepts. Evaluates
    ideas, creates briefs, and routes approved ideas to the development pipeline.

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    Key additions from DF-029:  for research agent MCP access,
    cost monitoring on discovery runs, notifications for discovered ideas,
    state-manager commits at every step, crash recovery, BC-S.SS.NNN
    format for existing product references.
    DF-034 additions: customer-feedback-ingestion, competitive-monitoring,
    analytics-integration, and intelligence-synthesis as dedicated skills
    (not inline agent tasks); evidence_strength as 7th scoring dimension;
    feedback-driven routing thresholds; post-feature-validation in feature.lobster.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  cost_monitoring:
    enabled: true
    metadata:
      mode: discovery

  # DF-034: Trigger cadences (configurable in discovery-config.yaml)
  schedule:
    market_research: "weekly"           # Full market scan
    feedback_ingestion: "daily"         # Customer feedback (lightweight, incremental)
    competitive_monitoring: "weekly"    # Competitor changes
    analytics_integration: "weekly"     # Usage data rollup
    full_synthesis: "weekly"            # Correlate all sources

  # DF-034: STATE.md fields tracked during discovery runs
  state_fields:
    - discovery_run_id                  # Unique ID per run
    - discovery_step                    # Current step (ingestion/synthesis/scoring/routing)
    - discovery_products_scanned        # Count of products processed
    - discovery_ideas_generated         # Count of ideas generated
    - discovery_ideas_above_threshold   # Count of ideas above routing threshold

  steps:
    # =========================================================================
    # Step 1: Load Configuration & Context
    # =========================================================================

    - name: load-discovery-config
      type: agent
      agent: orchestrator
      depends_on: []
      task: >
        Read .factory/discovery-config.yaml for discovery settings.
        Read .factory/product-registry.yaml for the list of products to scan.
        Read .factory/discovery/rejected-ideas.yaml for deduplication.
        Read .factory/discovery/deferred-ideas.yaml for re-evaluation dates.
        Write context summary to .factory/discovery/run-context.md.

    - name: state-init
      type: agent
      agent: state-manager
      depends_on: [load-discovery-config]
      task: >
        Update STATE.md: discovery_run: STARTED, run_id: [uuid],
        step: config-loaded, products_scanned: 0, ideas_generated: 0.
        Commit to factory-artifacts.

    # =========================================================================
    # Step 2: Feature Discovery (for each product in registry)
    # research-agent uses  for all MCP access (DF-027)
    # =========================================================================

    - name: feature-research
      type: skill
      skill: "skills/discovery-engine/SKILL.md"
      depends_on: [state-init]
      condition: "config.feature_discovery.enabled == true"
      # The skill handles per-product iteration internally
      # research-agent accesses Perplexity/Context7

    - name: state-backup-feature-research
      type: agent
      agent: state-manager
      depends_on: [feature-research]
      task: >
        Commit feature research artifacts. Update STATE.md:
        step: feature-research-complete.

    # =========================================================================
    # Step 2b: Customer Feedback Ingestion (DF-034)
    # =========================================================================

    - name: customer-feedback-ingestion
      type: skill
      skill: "skills/customer-feedback-ingestion/SKILL.md"
      depends_on: [state-init]
      condition: "config.products[*].user_channels is configured"
      # Ingests feedback from GitHub issues, Slack, app reviews, surveys.
      # Categorizes, deduplicates, produces feedback digest.
      # Writes to .factory/discovery/feedback-digest-YYYY-MM-DD.md.

    # =========================================================================
    # Step 2c: Competitive Monitoring (DF-034)
    # =========================================================================

    - name: competitive-monitoring
      type: skill
      skill: "skills/competitive-monitoring/SKILL.md"
      depends_on: [state-init]
      condition: "config.products[*].competitors is configured"
      timeout: "1h"
      # Monitors competitor releases, pricing changes, funding, acquisitions,
      # shutdowns, new entrants via research-agent -> Perplexity.
      # Writes to .factory/discovery/competitive-update-YYYY-MM-DD.md.

    # =========================================================================
    # Step 2d: Usage Analytics (DF-034)
    # =========================================================================

    - name: usage-analytics
      type: skill
      skill: "skills/analytics-integration/SKILL.md"
      depends_on: [state-init]
      condition: "config.products[*].analytics.enabled == true"
      # Reads product analytics: feature adoption, error rates, user journeys.
      # Writes to .factory/discovery/analytics-digest-YYYY-MM-DD.md.

    - name: state-backup-ingestion
      type: agent
      agent: state-manager
      depends_on: [customer-feedback-ingestion, competitive-monitoring, usage-analytics]
      task: >
        Commit ingestion artifacts. Update STATE.md:
        step: ingestion-complete.

    # =========================================================================
    # Step 3: Intelligence Synthesis (DF-034)
    # Correlates all sources before Delphi scoring
    # =========================================================================

    - name: intelligence-synthesis
      type: skill
      skill: "skills/intelligence-synthesis/SKILL.md"
      depends_on: [feature-research, customer-feedback-ingestion, competitive-monitoring, usage-analytics]
      # Correlates signals across all sources: market research, customer feedback,
      # competitive monitoring, usage analytics. Theme extraction, evidence
      # scoring, priority signals. Produces scored insights.
      # Writes to .factory/discovery/insights-YYYY-MM-DD.md.

    - name: state-backup-synthesis
      type: agent
      agent: state-manager
      depends_on: [intelligence-synthesis]
      task: >
        Commit synthesis artifacts. Update STATE.md:
        step: synthesis-complete.

    # =========================================================================
    # Step 4: Idea Scoring (Delphi protocol)
    # evidence_strength (DF-034) as 7th scoring dimension
    # Idea briefs for existing products reference BC-S.SS.NNN format
    # =========================================================================

    - name: feature-scoring-value
      type: agent
      agent: product-owner
      depends_on: [intelligence-synthesis]
      condition: "config.feature_discovery.enabled == true"
      task: >
        INDEPENDENT SCORING (Step 1 of Delphi protocol — no other scores visible).
        Read feature ideas from .factory/discovery/feature-ideas-*.md.
        Score each idea on Value, Alignment, Time Criticality, and
        evidence_strength dimensions.
        For ideas referencing existing products, use BC-S.SS.NNN format.
        Write to .factory/discovery/feature-scores-value.md.

    - name: feature-scoring-feasibility
      type: agent
      agent: architect
      depends_on: [intelligence-synthesis]
      condition: "config.feature_discovery.enabled == true"
      task: >
        INDEPENDENT SCORING (Step 1 of Delphi protocol — no other scores visible).
        Read feature ideas from .factory/discovery/feature-ideas-*.md.
        Score each idea on Feasibility and Effort dimensions.
        Apply SKEPTICAL LENS: AI proposals systematically overestimate feasibility.
        Explicitly list assumptions that could invalidate the feasibility estimate.
        Write to .factory/discovery/feature-scores-feasibility.md.

    - name: feature-scoring-novelty
      type: agent
      agent: adversary
      depends_on: [intelligence-synthesis]
      condition: "config.feature_discovery.enabled == true"
      task: >
        INDEPENDENT SCORING (Step 1 of Delphi protocol — no other scores visible).
        Fresh context. Score each idea on Novelty dimension.
        Compare against: existing product features, known competitor features,
        previously generated ideas from prior discovery runs.
        Flag "smart plagiarism" — ideas that are minor rewordings of existing concepts.
        Write to .factory/discovery/feature-scores-novelty.md.

    - name: feature-debate
      type: agent
      agent: adversary
      depends_on: [feature-scoring-value, feature-scoring-feasibility, feature-scoring-novelty]
      condition: "config.feature_discovery.enabled == true"
      task: >
        ADVERSARIAL CHALLENGE (Step 3 of Delphi protocol).
        Fresh context. Read all scoring files. For each idea:
        1. Identify dimensions where scores disagree by >0.3
        2. Challenge any score you disagree with, citing specific evidence
        3. Flag ideas where market claims lack citation (hallucination risk)
        4. Classify each idea into a profile: safe-bet, moonshot, quick-win,
           strategic-bet, time-bomb, or gold-plating
        5. Compute confidence-weighted composite scores (7 dimensions including
           evidence_strength)
        Write debate results and final rankings to .factory/discovery/feature-evaluation.md.
        Create idea briefs for ideas above threshold.
        For existing product feature briefs, reference BC-S.SS.NNN format.

    # =========================================================================
    # Step 5: Product Discovery
    # =========================================================================

    - name: product-research
      type: agent
      agent: research-agent
      depends_on: [state-init]
      condition: "config.product_discovery.enabled == true"
      task: >
        Scan for new product opportunities based on organization context
        from discovery-config.yaml. Research market gaps, emerging technology,
        adjacent opportunities, and regulatory shifts. All MCP access via
         (call Perplexity/Context7 calls).
        Write findings to .factory/discovery/product-research-YYYY-MM-DD.md.
      timeout: "1h"

    - name: product-scoring
      type: agent
      agent: product-owner
      depends_on: [product-research, intelligence-synthesis]
      condition: "config.product_discovery.enabled == true"
      task: >
        Read product concepts from .factory/discovery/product-concepts-*.md.
        Apply multi-agent Delphi scoring protocol:
        1. Score Value, Alignment, Time Criticality, evidence_strength
        2. Spawn architect for Feasibility, Effort scoring
        3. Spawn adversary for Novelty scoring and adversarial challenge
        4. Compute confidence-weighted composites with debate results (7 dimensions)
        5. Classify each concept into idea profiles
        Create idea briefs for concepts above the threshold.
        Write evaluation results to .factory/discovery/product-evaluation.md.

    # =========================================================================
    # Step 6: Deduplication & Trend Analysis
    # Checks against both approved and in-progress ideas
    # =========================================================================

    - name: deduplication
      type: agent
      agent: consistency-validator
      depends_on: [feature-debate, product-scoring]
      task: >
        Embedding-based three-tier deduplication pipeline:
        1. Compute sentence embeddings for each new idea brief
        2. Compare against ALL existing ideas (rejected, approved, deferred,
           active, IN-PROGRESS) using cosine similarity with tiered thresholds:
           - >0.92: auto-merge with notification (near-duplicate)
           - 0.85-0.92: flag for human review (highly similar)
           - 0.70-0.85: show as "related ideas" in the brief
           - <0.70: distinct, no action
        3. Check rejection cooldown (rejected ideas within cooldown period)
        4. Cluster analysis: group remaining ideas using HDBSCAN to identify
           idea families
        5. Check deferred ideas: surface any whose re-evaluation date has arrived
        Write deduplication log to .factory/discovery/dedup-log.md.
        Remove duplicate briefs. Merge near-duplicates.

    - name: state-backup-scoring
      type: agent
      agent: state-manager
      depends_on: [deduplication]
      task: >
        Commit scoring and dedup artifacts. Update STATE.md:
        step: scoring-complete, ideas_generated: [count].

    # =========================================================================
    # Step 7: Generate Discovery Report
    # =========================================================================

    - name: generate-report
      type: agent
      agent: orchestrator
      depends_on: [deduplication]
      task: >
        Synthesize all discovery results into a single report using
        templates/discovery-report-template.md. Include:
        - Feature ideas per product with scores (7 dimensions)
        - Product concepts with scores
        - Deduplication results
        - Trend analysis (compare against last 5 runs)
        - Cost summary (from cost-tracker)
        Write to .factory/discovery/discovery-report-YYYY-MM-DD.md.

    - name: state-backup-report
      type: agent
      agent: state-manager
      depends_on: [generate-report]
      task: >
        Commit discovery report. Update STATE.md:
        step: report-generated.

    # =========================================================================
    # Step 8: Notifications
    # =========================================================================

    - name: discovery-notifications
      type: agent
      agent: orchestrator
      depends_on: [generate-report]
      task: >
        Send notifications for discovered ideas:
        - Ideas above threshold: INFO notification
          "Discovery found interesting idea: [title]"
        - Urgent ideas (HIGH evidence + HIGH competitive urgency):
          BLOCKING notification to human
        - Discovery run complete: INFO notification with summary

    # =========================================================================
    # Step 9: Human Review Gate
    # =========================================================================

    - name: discovery-review
      type: human-approval
      depends_on: [generate-report]
      approval:
        prompt: >
          Discovery run complete. Review the discovery report and idea briefs.
          For each idea brief, choose: Approve, Research Further, Defer, or Reject.
          Approved feature ideas will route to Feature Mode (DF-006).
          Approved product ideas will route to Adaptive Planning (DF-016).
        artifacts:
          - ".factory/discovery/discovery-report-YYYY-MM-DD.md"
          - ".factory/discovery/briefs/"
        timeout: "72h"

    # =========================================================================
    # Step 10: Route Approved Ideas
    # =========================================================================

    - name: route-approved-ideas
      type: agent
      agent: orchestrator
      depends_on: [discovery-review]
      task: >
        For each approved idea brief:
        - Feature ideas: copy brief to the product's .factory/ directory as a
          feature request. The next Feature Mode run (DF-006) picks it up.
          Feature briefs reference BC-S.SS.NNN for existing product features.
        - Product ideas: route to DF-016 Adaptive Planning Pipeline by placing
          the brief at .factory/planning/product-brief.md in a new project directory.
        For rejected ideas: append to .factory/discovery/rejected-ideas.yaml
          with rejection date and rationale (for deduplication cooldown).
        For deferred ideas: append to .factory/discovery/deferred-ideas.yaml
          with re-evaluation date.
        Update .factory/discovery/routing-log.md with all routing decisions.

    - name: state-final
      type: agent
      agent: state-manager
      depends_on: [route-approved-ideas]
      task: >
        Commit routing decisions. Update STATE.md:
        discovery_run: COMPLETE, step: routed.

    # =========================================================================
    # Step 11: Execute Approved Product Ideas via Planning Pipeline
    # =========================================================================

    - name: execute-product-ideas
      type: sub-workflow
      sub_workflow: "planning.lobster"
      depends_on: [route-approved-ideas]
      condition: "discovery.approved_products.count > 0"

    # =========================================================================
    # Step 12: Execute Approved Feature Ideas via Feature Mode
    # =========================================================================

    - name: execute-feature-ideas
      type: sub-workflow
      sub_workflow: "feature.lobster"
      depends_on: [route-approved-ideas]
      condition: "discovery.approved_features.count > 0"

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # Reviews the discovery run itself — research quality, scoring calibration,
    # deduplication effectiveness, idea routing accuracy, discovery cost.
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [route-approved-ideas]
      # Runs after routing approved ideas (the final discovery-specific step)

    - name: session-review-approval
      type: human-approval
      depends_on: [session-review]
      approval:
        prompt: >
          Session review complete. Review the analysis and improvement proposals
          for the discovery run. For each proposal: Approve (create task),
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
