workflow:
  name: maintenance-sweep
  description: >
    Background quality sweeps that run on a schedule. Detects dependency
    vulnerabilities, documentation drift, pattern inconsistencies, stale holdout
    scenarios, performance regressions, DTU clone fidelity drift, spec coherence
    violations, overdue tech debt, and accessibility regressions. Opens fix PRs
    through the per-story delivery flow (code-delivery.lobster).

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    Key additions from DF-029: 9 sweep types (4 new), dependency audit split
    (dx-engineer runs + security-reviewer analyzes), state-manager commits,
    fix PRs via code-delivery.lobster, notification integration, crash recovery.
    DF-037 additions: Sweep 10 (design drift detection for UI products --
    token overrides, component misuse, pattern violations, emergent patterns).
  version: "2.0.0"

  defaults:
    on_failure: skip
    max_retries: 1
    timeout: "1h"

  cost_monitoring:
    enabled: true
    metadata:
      mode: maintenance

  steps:
    # =========================================================================
    # Load Config + State Init
    # =========================================================================

    - name: load-config
      type: agent
      agent: orchestrator
      depends_on: []
      task: >
        Read .factory/maintenance-config.yaml for sweep configuration.
        Read STATE.md to check for interrupted maintenance runs (crash recovery).
        If interrupted run found, resume from last committed sweep.

    - name: state-init
      type: agent
      agent: state-manager
      depends_on: [load-config]
      task: >
        Update STATE.md: maintenance_run: STARTED, run_id: [uuid],
        sweep_progress: {}, findings_count: 0.
        Commit to factory-artifacts.

    # =========================================================================
    # 10 Parallel Sweeps
    # =========================================================================

    # --- Sweep 1: Dependency Audit (split: dx-engineer runs, security-reviewer analyzes) ---
    - name: dependency-audit-scan
      type: agent
      depends_on: [state-init]
      agent: dx-engineer
      task: >
        Run dependency scan commands:
        - cargo audit, cargo deny (Rust)
        - npm audit (Node.js)
        - pip-audit (Python)
        - semgrep --config=auto (all)
        Write raw results to .factory/maintenance/dependency-audit-raw.log.
      timeout: "15m"

    - name: dependency-audit-analysis
      type: agent
      depends_on: [dependency-audit-scan]
      agent: security-reviewer
      task: >
        Analyze dependency scan results from dx-engineer.
        Classify findings by severity (CRITICAL/HIGH/MEDIUM/LOW).
        Check for compromised or unmaintained dependencies.
        Write analysis to .factory/maintenance/dependency-audit.md.

    - name: state-backup-sweep-1
      type: agent
      agent: state-manager
      depends_on: [dependency-audit-analysis]
      task: "Commit sweep 1 artifacts. Update sweep_progress."

    # --- Sweep 2: Documentation Drift ---
    - name: doc-drift-scan
      type: agent
      depends_on: [state-init]
      agent: consistency-validator
      task: >
        Compare documentation (README, API docs, architecture docs) against
        current implementation. Flag stale sections, missing features, and
        incorrect examples. Write to .factory/maintenance/doc-drift.md.

    - name: state-backup-sweep-2
      type: agent
      agent: state-manager
      depends_on: [doc-drift-scan]
      task: "Commit sweep 2 artifacts. Update sweep_progress."

    # --- Sweep 3: Pattern Consistency ---
    - name: pattern-consistency-scan
      type: agent
      depends_on: [state-init]
      agent: code-reviewer
      model_tier: review
      task: >
        Scan codebase for pattern inconsistencies: newer code following updated
        patterns while older code retains legacy patterns. Identify candidates
        for batch refactoring. Write to .factory/maintenance/pattern-consistency.md.

    - name: state-backup-sweep-3
      type: agent
      agent: state-manager
      depends_on: [pattern-consistency-scan]
      task: "Commit sweep 3 artifacts. Update sweep_progress."

    # --- Sweep 4: Holdout Scenario Freshness (DF-030 lifecycle-aware) ---
    - name: holdout-freshness-check
      type: agent
      depends_on: [state-init]
      agent: holdout-evaluator
      task: >
        Check all holdout scenarios for staleness (DF-030 lifecycle):
        1. Scenarios referencing deprecated/removed features -> mark stale
        2. Scenarios not evaluated in 3+ releases -> mark stale
        3. Scenarios whose expected behavior no longer matches product -> mark stale
        4. Update lifecycle_status in scenario frontmatter (active/stale/retired)
        5. Update HS-INDEX.md to reflect current active set
        Write to .factory/maintenance/holdout-freshness.md.
      condition: "state.has_holdout_scenarios == true"

    - name: state-backup-sweep-4
      type: agent
      agent: state-manager
      depends_on: [holdout-freshness-check]
      task: "Commit sweep 4 artifacts. Update sweep_progress."

    # --- Sweep 5: Performance Regression Detection ---
    - name: performance-regression-scan
      type: agent
      depends_on: [state-init]
      agent: performance-engineer
      task: >
        Run benchmark suite and compare against baseline. Flag regressions
        exceeding 10% threshold. Write to .factory/maintenance/performance.md.
      condition: "state.has_benchmarks == true"

    - name: state-backup-sweep-5
      type: agent
      agent: state-manager
      depends_on: [performance-regression-scan]
      task: "Commit sweep 5 artifacts. Update sweep_progress."

    # --- Sweep 6: DTU Fidelity Drift (DF-026) ---
    - name: dtu-fidelity-drift
      type: agent
      depends_on: [state-init]
      agent: dtu-validator
      condition: "state.has_dtu_clones == true"
      task: >
        Check DTU clones against real API behavior (via mcporter).
        Compute fidelity score drift since last check.
        Flag fidelity degradation for clone update.
        Write to .factory/maintenance/dtu-fidelity-drift.md.

    - name: state-backup-sweep-6
      type: agent
      agent: state-manager
      depends_on: [dtu-fidelity-drift]
      task: "Commit sweep 6 artifacts. Update sweep_progress."

    # --- Sweep 7: Spec Coherence (DF-030's 33 checks) ---
    - name: spec-coherence
      type: agent
      depends_on: [state-init]
      agent: consistency-validator
      task: >
        Run full spec coherence check (33 criteria, DF-030):
        Criteria 1-23 (existing): L1->L4 chain, BC coverage, VP alignment,
        story mapping, architecture consistency, sharding integrity.
        Criteria 24-33 (lifecycle, DF-030):
        - No deprecated BCs referenced by active stories
        - No withdrawn VPs in active VP-INDEX
        - No retired holdout scenarios in active evaluation
        - All active BCs have at least one active story
        - All active VPs have proofs or justification
        - module-criticality.md matches current architecture
        - DTU assessment matches current external deps
        - Story count matches STORY-INDEX
        - No cross-cycle BC numbering conflicts
        - Spec snapshot exists for every released version
        Flag spec drift for spec-steward update.
        Write to .factory/maintenance/spec-coherence.md.

    - name: state-backup-sweep-7
      type: agent
      agent: state-manager
      depends_on: [spec-coherence]
      task: "Commit sweep 7 artifacts. Update sweep_progress."

    # --- Sweep 8: Tech Debt Scan + Register Update (DF-030) ---
    - name: tech-debt-register
      type: agent
      depends_on: [state-init]
      agent: consistency-validator
      task: >
        Scan for tech debt and update register (DF-030):
        1. Deferred adversarial findings -> add to register
        2. Spec drift (BCs don't match implementation) -> add to register
        3. Outdated dependencies (major versions) -> add to register
        4. Stale DTU clones (fidelity degradation) -> add to register
        5. Anti-patterns and code smells -> add to register
        6. Check overdue items: past due -> surface for human triage
        7. Items approaching due -> WARNING notification
        Update .factory/tech-debt-register.md.
        Write sweep summary to .factory/maintenance/tech-debt-check.md.

    - name: state-backup-sweep-8
      type: agent
      agent: state-manager
      depends_on: [tech-debt-register]
      task: "Commit sweep 8 artifacts. Update sweep_progress."

    # --- Sweep 9: Accessibility Regression (UI products only) ---
    - name: accessibility-regression
      type: agent
      depends_on: [state-init]
      agent: accessibility-auditor
      condition: "state.has_ui == true"
      task: >
        Run automated a11y scan on current build.
        Compare against last scan baseline.
        Flag regressions.
        Write to .factory/maintenance/accessibility-regression.md.
      context:
        exclude:
          # Information asymmetry wall: a11y auditor cannot see architecture
          - ".factory/specs/architecture/**"

    - name: state-backup-sweep-9
      type: agent
      agent: state-manager
      depends_on: [accessibility-regression]
      task: "Commit sweep 9 artifacts. Update sweep_progress."

    # --- Sweep 10: Design Drift Detection (DF-037 D12, UI products only) ---
    - name: design-drift-scan
      type: skill
      skill: "skills/design-drift-detection/SKILL.md"
      depends_on: [state-init]
      condition: "state.has_ui == true"
      # Detects: token overrides, component misuse, pattern violations,
      # emergent patterns (D15), style inconsistencies.
      # Output: .factory/maintenance/design-drift.md

    - name: state-backup-sweep-10
      type: agent
      agent: state-manager
      depends_on: [design-drift-scan]
      task: "Commit sweep 10 artifacts. Update sweep_progress."

    # --- Sweep 11: Risk & Assumption Monitoring ---
    - name: risk-assumption-monitoring
      type: agent
      depends_on: [state-init]
      agent: consistency-validator
      task: >
        Monitor assumptions and risks from the L2 Domain Spec for staleness
        and validity:
        1. ASMs still unvalidated after 2+ releases -> flag for validation
           or escalation to risk register
        2. R-NNNs with open status and monitoring flag -> re-assess likelihood
           and impact against current product state
        3. Cross-reference invalidated ASMs with risk register: every invalidated
           ASM should have triggered a new or updated R-NNN
        4. R-NNN mitigations still valid against current architecture: check that
           architecture changes haven't invalidated existing mitigations
        Write to .factory/maintenance/risk-assumption-monitoring.md.

    - name: state-backup-sweep-11
      type: agent
      agent: state-manager
      depends_on: [risk-assumption-monitoring]
      task: "Commit sweep 11 artifacts. Update sweep_progress."

    # =========================================================================
    # Aggregate Findings
    # =========================================================================

    - name: maintenance-report
      type: agent
      depends_on:
        - dependency-audit-analysis
        - doc-drift-scan
        - pattern-consistency-scan
        - holdout-freshness-check
        - performance-regression-scan
        - dtu-fidelity-drift
        - spec-coherence
        - tech-debt-register
        - accessibility-regression
        - design-drift-scan
        - risk-assumption-monitoring
      agent: orchestrator
      task: >
        Aggregate all sweep findings into a single maintenance report.
        Classify findings: automated fix available vs manual fix needed.
        Write to .factory/maintenance/sweep-report-YYYY-MM-DD.md.

    # =========================================================================
    # Fix PR Delivery (per finding via code-delivery.lobster)
    # pr-reviewer sees only PR diff (information asymmetry wall)
    # security-reviewer reviews if CRIT/HIGH finding
    # =========================================================================

    - name: fix-pr-delivery
      type: loop
      depends_on: [maintenance-report]
      condition: "maintenance.has_auto_fixable_findings == true"
      loop:
        for_each: finding in auto_fixable_findings
        steps:
          - name: generate-fix
            type: agent
            agent: implementer
            task: >
              Generate fix for finding. Create worktree, implement fix,
              run quality gates (holdout + adversarial + regression).

          - name: deliver-fix
            type: sub-workflow
            sub_workflow: "code-delivery.lobster"
            depends_on: [generate-fix]
            inputs:
              story_id: "FIX-MAINT-finding.id"
              module_criticality: "finding.severity"

    # =========================================================================
    # On-Demand Demo Recording (DF-018)
    # =========================================================================

    - name: maintenance-demo-recording
      type: skill
      skill: "skills/demo-recording/SKILL.md"
      depends_on: [maintenance-report]
      condition: "maintenance.request_demo == true"
      timeout: "30m"

    # =========================================================================
    # Notifications
    # =========================================================================

    - name: notifications
      type: agent
      agent: orchestrator
      depends_on: [fix-pr-delivery, maintenance-report]
      task: >
        Send notifications based on findings:
        - CRITICAL findings (CVE, security, major regression):
          BLOCKING notification to human
        - Overdue tech debt: WARNING notification
        - Clean sweep or findings addressed: INFO notification
        Use configured notification channel (github-issues default).

    # =========================================================================
    # Final State Commit
    # =========================================================================

    - name: state-final
      type: agent
      agent: state-manager
      depends_on: [fix-pr-delivery, notifications]
      task: >
        Commit final maintenance artifacts. Update STATE.md:
        maintenance_run: COMPLETE, findings_count: [N],
        fixes_applied: [N], fixes_pending: [N].

    - name: maintenance-gate
      type: gate
      depends_on: [maintenance-report, state-final]
      gate:
        criteria:
          - "Zero critical CVEs in dependencies (or fix PRs created)"
          - "Auto-fix PRs pass full regression suite"
          - "All 11 sweeps completed (or skipped with documented reason)"
        fail_action: warn

    # =========================================================================
    # POST-PIPELINE: Session Review (DF-036)
    # Reviews the maintenance sweep — sweep effectiveness, false positive rate,
    # fix PR success rate, sweep cost vs value.
    # =========================================================================

    - name: session-review
      type: skill
      skill: "skills/session-review/SKILL.md"
      depends_on: [maintenance-gate]
      # Runs after maintenance gate (final maintenance-specific step)

    - name: session-review-approval
      type: human-approval
      depends_on: [session-review]
      approval:
        prompt: >
          Session review complete. Review the analysis and improvement proposals
          for the maintenance sweep. For each proposal: Approve (create task),
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
