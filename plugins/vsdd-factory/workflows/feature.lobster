workflow:
  name: feature-vsdd
  description: >
    Incremental development pipeline for adding features to an existing VSDD-managed
    codebase. Scopes all work to the delta (changed/new code) while regression testing
    protects existing functionality. This is the steady-state operating mode.

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    Key additions from DF-029: per-story delivery via code-delivery.lobster,
    4-level hierarchy (BC-S.SS.NNN), wave gates, information asymmetry walls,
    security review touchpoints, fix PR delivery, cost monitoring, artifact
    backup, release step.

    DF-035 upgrades:
    - 4-level spec hierarchy (BC-S.SS.NNN) in F1/F2
    - DTU re-assessment + gene transfusion in F2
    - Per-file stories + wave schedule in F3
    - Per-story delivery via code-delivery.lobster in F4
    - Information asymmetry walls in F4/F5/F6
    - 4 security review touchpoints (per-story PR, wave gate, F5, F6)
    - Fix PR delivery for F5/F6/F7
    - state-manager at every gate with crash recovery state
    - Cost monitoring with tiered response
    - Market intel before F1 + post-feature validation after release
    - Release step after F7 (MINOR/PATCH)
    - UI/UX routing (ux-designer, accessibility-auditor, e2e-tester)
    - DF-037 UI quality loop: design system bootstrap (F2), multi-variant design (F2),
      per-story UI quality gate (code-delivery), wave-level responsive/performance/completeness
      checks, heuristic evaluation, convergence UI quality gate (100% fidelity)
    - Quick dev routing for trivial changes
    - Bug fix routing (Path 4) with mandatory regression test
    - Max 5 convergence cycles in F7 with cost-benefit escalation

    Routing:
    - feature/enhancement (standard): F1 -> F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> release
    - feature/enhancement (trivial): F1 -> F4 single story -> regression -> F7 lite -> PATCH
    - bug-fix: F1 -> single fix story -> holdout -> F5 -> F6 -> F7 -> PATCH
    - bug-fix (CRITICAL): expedited flow with minimal gates

    Crash Recovery (DF-035 D8):
    If Feature Mode crashes mid-pipeline, orchestrator reconstructs from:
    1. STATE.md -- current phase, wave, feature type
    2. Git worktrees -- in-progress stories (uncommitted work = max ~5 min loss)
    3. Open PRs -- stories submitted but not merged
    4. factory-artifacts branch -- committed phase artifacts
    5. Regression baseline SHA -- to verify no regression was introduced pre-crash

    Resume logic:
    - Crash in F1-F3: restart the phase (spec work is lightweight)
    - Crash in F4 wave loop: resume at current wave, skip merged stories
    - Crash in F5-F7: resume at current convergence round

  version: "3.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "1h"

  # DF-027 Cost Monitoring: Every agent call logged with metadata
  cost_tracking:
    enabled: true
    metadata:
      mode: feature
      feature: "${feature.name}"
      phase: "${current_phase}"
      wave: "${current_wave}"
    thresholds:
      warn: 0.70        # 70% of budget -> warn human
      pause: 0.95       # 95% of budget -> pause for human approval
    protected_agents:    # Never downgraded, even under cost pressure
      - adversary
      - security-reviewer
      - formal-verifier
    summary_artifact: ".factory/feature/cost-summary.md"

  steps:
    # =========================================================================
    # Factory Worktree Health Check (BLOCKING)
    # Validates .factory/ is a valid git worktree on factory-artifacts branch.
    # Auto-creates orphan branch + worktree if missing. Checks sync state.
    # Runs first on every pipeline start — before any .factory/ reads/writes.
    # =========================================================================

    - name: factory-worktree-health
      type: skill
      skill: "skills/factory-worktree-health/SKILL.md"
      agent: devops-engineer
      depends_on: []

    - name: factory-worktree-gate
      type: gate
      depends_on: [factory-worktree-health]
      gate:
        criteria:
          - "FACTORY_WORKTREE_HEALTH: PASS"
          - ".factory/ is a valid git worktree on factory-artifacts branch"

    # =========================================================================
    # Feature Cycle Initialization (DF-030)
    # Create cycle directory, initialize manifest, pause maintenance
    # =========================================================================

    - name: feature-cycle-init
      type: agent
      agent: state-manager
      depends_on: [factory-worktree-gate]
      task: >
        Create cycle directory: cycles/vX.Y.Z-feature-NAME/.
        Initialize cycle-manifest.md from templates/cycle-manifest-template.md.
        Update STATE.md with active cycle: pipeline: FEATURE-CYCLE,
        active_cycle: vX.Y.Z-feature-NAME. Pause maintenance sweeps.

    # =========================================================================
    # Environment Setup Check (cross-cutting, DF-027)
    # May skip if recently validated
    # =========================================================================

    - name: environment-check
      type: agent
      agent: dx-engineer
      depends_on: [feature-cycle-init]
      task: >
        Verify environment still current:
        1. Check tool versions, LLM health, MCP preflight
        2. If product has been idle: full toolchain preflight
        3. If product was active recently: lightweight check only
        Block on any security finding or missing required model.
      timeout: "15m"

    # =========================================================================
    # Market Intelligence Assessment (feature market validation)
    # =========================================================================

    - name: feature-market-intel
      type: skill
      skill: "skills/market-intelligence-assessment/SKILL.md"
      depends_on: [environment-check]
      condition: "request.intent != 'bug-fix'"
      config:
        depth: "L1"
        scope: "feature"
      # Bug fixes skip market intel (bugs don't need market validation)

    - name: feature-market-review
      type: human-approval
      depends_on: [feature-market-intel]
      condition: "request.intent != 'bug-fix'"
      approval:
        prompt: >
          Market intelligence for the feature request is complete.
          Does this feature make market sense? Approve to proceed to F1.
        artifacts:
          - ".factory/planning/market-intel.md"
        timeout: "48h"

    # =========================================================================
    # Pre-F1: Establish Demo Baseline (DF-018 visual regression)
    # =========================================================================

    - name: establish-demo-baseline
      type: agent
      agent: orchestrator
      depends_on: []
      condition: "config.demo_recording.enabled != false"
      task: >
        Record baseline demos of existing behavior BEFORE any changes are made.
        Load previous demo recordings from .factory/demo-evidence/ or record
        fresh baseline demos of features in the delta's impact zone.
        Store baseline at .factory/feature/demo-baseline/.
        This enables visual regression comparison after implementation.

    # =========================================================================
    # Phase F1: Delta Analysis
    # Uses 4-level hierarchy (L1->L4, BC-S.SS.NNN format)
    # =========================================================================

    - name: phase-f1-delta-analysis
      type: skill
      skill: "skills/phase-f1-delta-analysis/SKILL.md"
      depends_on: [feature-market-review, environment-check]
      # For features/enhancements: waits for market review
      # For bug fixes: market intel is skipped, proceeds after environment check

    - name: phase-f1-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f1-delta-analysis]
      task: >
        Commit F1 artifacts to factory-artifacts branch.
        Update STATE.md with:
        - phase: F1-complete
        - feature_name: (from delta analysis)
        - feature_type: ui | backend | full-stack | infrastructure (from F1)
        - intent: feature | enhancement | bug-fix (from F1)
        - scope: trivial | standard (from F1)
        - severity: (if bug-fix, from F1)
        - regression_baseline_ref: (commit SHA before changes)
        - demo_baseline_ref: .factory/feature/demo-baseline/
        - market_intel_recommendation: GO | CAUTION | STOP + human decision

    - name: phase-f1-gate
      type: gate
      depends_on: [phase-f1-delta-analysis]
      gate:
        criteria:
          - "Impact boundary identified -- affected modules listed"
          - "Affected spec/architecture/story mapping complete (BC-S.SS.NNN references)"
          - "Regression risk assessment complete"
          - "Feature type classified (ui | backend | full-stack | infrastructure)"
          - "Intent classified (feature | enhancement | bug-fix)"
          - "Scope classified (trivial | standard)"
          - "Severity classified if bug-fix (CRITICAL | HIGH | MEDIUM | LOW)"
          - "Every new/modified ASM-NNN has Status, Validation Method, and Impact populated (delta scope)"
          - "Every new/modified R-NNN has Status, Category, and Mitigation populated (delta scope)"
          - "Every new R-NNN NFR candidate has corresponding NFR-NNN (delta scope)"
          - "Every new HIGH-impact R-NNN addressed in architecture (delta scope)"
        fail_action: block

    - name: phase-f1-human-approval
      type: human-approval
      depends_on: [phase-f1-gate]
      approval:
        prompt: >
          Delta Analysis is complete. Review the impact boundary and regression
          risk assessment. Approve the scope before proceeding to spec evolution.
        artifacts:
          - ".factory/feature/delta-analysis.md"
        timeout: "24h"

    # =========================================================================
    # Quick Dev Routing (D13) — trivial changes skip F2/F3/F5/F6
    # F1 (quick scope) -> F4 single story -> regression -> F7 lite -> PATCH
    # =========================================================================

    - name: quick-dev-single-story
      type: sub-workflow
      sub_workflow: "code-delivery.lobster"
      depends_on: [phase-f1-human-approval]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      inputs:
        story_id: "STORY-QUICK-${feature.name}"
        feature_type: "f1.feature_type"
        module_criticality: "f1.module_criticality"

    - name: quick-dev-regression
      type: agent
      agent: implementer
      depends_on: [quick-dev-single-story]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      task: "Run full regression suite after quick dev story merges."

    - name: quick-dev-f7-lite
      type: skill
      skill: "skills/phase-f7-delta-convergence/SKILL.md"
      depends_on: [quick-dev-regression]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      config:
        mode: "quick-convergence"

    - name: quick-dev-f7-human-approval
      type: human-approval
      depends_on: [quick-dev-f7-lite]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      approval:
        prompt: >
          Quick dev convergence check complete. Approve for PATCH release.
        timeout: "24h"

    - name: quick-dev-release
      type: agent
      agent: devops-engineer
      depends_on: [quick-dev-f7-human-approval]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      task: >
        Execute PATCH release: semver bump, CHANGELOG, git tag, gh release,
        publish. Notify human: trivial change shipped.

    - name: quick-dev-state-backup
      type: agent
      agent: state-manager
      depends_on: [quick-dev-release]
      condition: "delta.scope == 'trivial' and delta.intent != 'bug-fix'"
      task: "Commit quick dev artifacts. Update STATE.md: pipeline complete."

    # =========================================================================
    # Bug Fix Routing (D14, Path 4) — skip F2/F3, compressed pipeline
    # Mandatory regression test: MUST fail before fix, pass after
    # =========================================================================

    - name: bugfix-demo-baseline
      type: agent
      agent: demo-recorder
      depends_on: [phase-f1-human-approval]
      condition: "delta.intent == 'bug-fix' and delta.severity != 'CRITICAL'"
      task: >
        Record buggy behavior as demo baseline. Proof the bug exists +
        visual regression reference. Store at .factory/feature/demo-baseline/.

    - name: bugfix-single-story-delivery
      type: sub-workflow
      sub_workflow: "code-delivery.lobster"
      depends_on: [phase-f1-human-approval, bugfix-demo-baseline]
      condition: "delta.intent == 'bug-fix'"
      inputs:
        story_id: "STORY-FIX-${feature.name}"
        feature_type: "f1.feature_type"
        module_criticality: "f1.module_criticality"
        bug_fix_mode: true
        # bug_fix_mode instructs test-writer:
        #   1. Write regression test that REPRODUCES the bug
        #   2. Test MUST FAIL before fix (proves bug exists)
        #   3. Test MUST PASS after fix (proves fix works)
        #   4. If test cannot reproduce bug -> escalate to human (root cause may be wrong)
        # bug_fix_mode instructs e2e-tester (if UI bug):
        #   Write browser regression test for the fixed behavior

    - name: bugfix-build-verification
      type: agent
      agent: implementer
      depends_on: [bugfix-single-story-delivery]
      condition: "delta.intent == 'bug-fix'"
      task: "Full release build + complete test suite on develop."

    - name: bugfix-holdout-scoped
      type: agent
      agent: holdout-evaluator
      depends_on: [bugfix-build-verification]
      condition: "delta.intent == 'bug-fix'"
      task: >
        Re-evaluate holdout scenarios related to fixed module only (not full product).
        Compare to baseline. Report regressions.
      context:
        exclude:
          # Information asymmetry wall: holdout cannot see source code
          - "src/**"
          - ".factory/specs/**"

    - name: bugfix-f5-scoped
      type: loop
      depends_on: [bugfix-holdout-scoped]
      condition: "delta.intent == 'bug-fix'"
      loop:
        max_iterations: 1
        # Bug fixes are small -> typically converges in 1 round
        # CRITICAL bugs: 1 round max enforced
        steps:
          - name: bugfix-adversary
            type: agent
            agent: adversary
            model_tier: adversary
            task: "Review fix diff only with fresh context."
            context:
              exclude:
                - ".factory/cycles/**/implementation/red-gate-log*"
                - ".factory/cycles/**/implementation/implementer-notes*"
          - name: bugfix-code-reviewer
            type: agent
            agent: code-reviewer
            model_tier: review
            depends_on: [bugfix-adversary]
            task: "Secondary pass on fix diff (Gemini). Fresh context."
            context:
              exclude:
                - ".factory/phase-f5-adversarial/**"

    - name: bugfix-f6-scoped
      type: skill
      skill: "skills/phase-f6-targeted-hardening/SKILL.md"
      depends_on: [bugfix-f5-scoped]
      condition: "delta.intent == 'bug-fix'"
      config:
        scope: "affected-module-only"
        # CRITICAL bugs: security scan only (skip proofs/fuzz/mutation)
        critical_expedited: "${delta.severity == 'CRITICAL'}"
      context:
        exclude:
          - ".factory/phase-f5-adversarial/**"

    - name: bugfix-f6-a11y
      type: agent
      agent: accessibility-auditor
      depends_on: [bugfix-f6-scoped]
      condition: "delta.intent == 'bug-fix' and f1.feature_type in ['ui', 'full-stack']"
      task: "Accessibility re-check on fixed UI code."
      context:
        exclude:
          - ".factory/specs/architecture/**"

    - name: bugfix-demo-comparison
      type: agent
      agent: visual-reviewer
      model_tier: review
      depends_on: [bugfix-f6-scoped]
      condition: "delta.intent == 'bug-fix' and delta.severity != 'CRITICAL'"
      task: >
        Compare fixed behavior demo against buggy baseline demo.
        Verify bug is fixed + no visual regressions.

    - name: bugfix-f7-verification
      type: skill
      skill: "skills/phase-f7-delta-convergence/SKILL.md"
      depends_on: [bugfix-f6-scoped, bugfix-demo-comparison]
      condition: "delta.intent == 'bug-fix'"
      config:
        mode: "fix-verification"
        # Verify: regression test passes, all holdout scenarios pass,
        # adversary converged, hardening clean, no regressions

    - name: bugfix-f7-human-approval
      type: human-approval
      depends_on: [bugfix-f7-verification]
      condition: "delta.intent == 'bug-fix'"
      approval:
        prompt: >
          Bug fix verification complete. Regression test passes, holdout
          scenarios pass, adversary converged, hardening clean.
          Approve for PATCH release.
        timeout: "24h"

    - name: bugfix-release
      type: agent
      agent: devops-engineer
      depends_on: [bugfix-f7-human-approval]
      condition: "delta.intent == 'bug-fix'"
      task: >
        Execute PATCH release (always PATCH for bug fixes):
        semver bump, CHANGELOG, git tag, gh release, publish.
        Notify human: bug fix shipped.

    - name: bugfix-state-backup
      type: agent
      agent: state-manager
      depends_on: [bugfix-release]
      condition: "delta.intent == 'bug-fix'"
      task: >
        Commit bug fix artifacts. Update STATE.md: pipeline complete.
        If expedited: log expedited: true with justification.
        Queue skipped steps for post-release follow-up.

    - name: bugfix-post-monitoring
      type: agent
      agent: orchestrator
      depends_on: [bugfix-release]
      condition: "delta.intent == 'bug-fix' and config.post_feature_validation.enabled == true"
      task: >
        Schedule post-fix monitoring check at 7 days:
        Bug still fixed? No new regressions reported?

    # =========================================================================
    # STANDARD PIPELINE (F2-F7): Feature and Enhancement changes
    # Condition: delta.scope != 'trivial' AND delta.intent != 'bug-fix'
    # Trivial changes route through quick-dev above.
    # Bug fixes route through bugfix-* steps above.
    # All steps below are skipped when quick-dev or bugfix routes are active.
    # =========================================================================

    # =========================================================================
    # Phase F2: Spec Evolution
    # Uses 4-level hierarchy (L1->L4), BC-S.SS.NNN format
    # Includes DTU re-assessment + gene transfusion assessment
    # Includes UX design + a11y review for UI features
    # =========================================================================

    - name: phase-f2-spec-evolution
      type: skill
      skill: "skills/phase-f2-spec-evolution/SKILL.md"
      depends_on: [phase-f1-human-approval]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix'"

    - name: phase-f2-dtu-reassessment
      type: agent
      agent: architect
      depends_on: [phase-f2-spec-evolution]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and feature.adds_external_deps == true"
      task: >
        DTU re-assessment: feature adds new external service dependencies.
        Assess each new dependency for DTU candidacy (fidelity levels L1-L4).
        Update dtu-assessment.md. Spawn research-agent for API specs.

    - name: phase-f2-gene-transfusion-assessment
      type: agent
      agent: architect
      depends_on: [phase-f2-spec-evolution]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and feature.has_reference_impl == true"
      task: >
        Gene transfusion assessment: feature has reference implementation in
        another language. Evaluate FOR/AGAINST signals. Update
        gene-transfusion-assessment.md.

    # --- F2 UX/Accessibility (conditional: UI features only) ---

    - name: phase-f2-ux-design
      type: agent
      agent: ux-designer
      model_tier: builder
      depends_on: [phase-f2-spec-evolution]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and f1.feature_type in ['ui', 'full-stack']"
      task: >
        Update UX spec for the feature delta. New screens, modified flows,
        updated components. Reference design system tokens and component
        contracts by name (DF-037 D1: "Button/primary" not "a blue button").
        If no design system exists: run design-system-bootstrap skill first.
        Create/update wireframes via Excalidraw MCP for all new/changed screens.
        Create/update flow diagrams for new/changed interaction flows.
        Store wireframe PNGs in ux-spec/screens/wireframes/ and flow SVGs
        in ux-spec/flows/diagrams/.
        Write .factory/feature/ux-delta.md.

    # --- F2 Design System Bootstrap (DF-037 D2, if none exists) ---

    - name: phase-f2-design-system-bootstrap
      type: skill
      skill: "skills/design-system-bootstrap/SKILL.md"
      depends_on: [phase-f2-spec-evolution]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and f1.feature_type in ['ui', 'full-stack'] and design_system.exists == false"
      # Bootstrap only if no .factory/design-system/ exists yet.
      # Creates tokens, contracts, patterns, constraints from brief.

    # --- F2 Multi-Variant Design (DF-037 D5, complex screens) ---

    - name: phase-f2-multi-variant
      type: skill
      skill: "skills/multi-variant-design/SKILL.md"
      depends_on: [phase-f2-ux-design]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and f1.feature_type in ['ui', 'full-stack'] and ux_delta.has_complex_screens == true"
      # Generate 2-3 design variants for screens with >5 interactive elements.
      # Human selects or requests synthesis.

    - name: phase-f2-multi-variant-approval
      type: human-approval
      depends_on: [phase-f2-multi-variant]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and f1.feature_type in ['ui', 'full-stack'] and ux_delta.has_complex_screens == true"
      approval:
        prompt: >
          Design variants generated for complex screens. Select preferred
          variant or request synthesis of best aspects.
        artifacts:
          - ".factory/ui-quality/variants/"
        timeout: "24h"

    - name: phase-f2-a11y-review
      type: agent
      agent: accessibility-auditor
      model_tier: review
      depends_on: [phase-f2-ux-design, phase-f2-design-system-bootstrap]
      condition: "delta.scope != 'trivial' and delta.intent != 'bug-fix' and f1.feature_type in ['ui', 'full-stack']"
      task: >
        Spec-level a11y review (DF-037 D9 7-point validation, point 1):
        Review UX delta for WCAG compliance. Focus on keyboard navigation,
        screen reader support, color contrast, touch targets.
        Validate design system tokens for a11y compliance (contrast ratios,
        focus styles, touch target minimums).
        Verify component contracts have correct a11y requirements.
        Write .factory/feature/accessibility-review-f2.md.
      context:
        exclude:
          # Information asymmetry wall: a11y auditor cannot see architecture or impl plans
          - ".factory/specs/architecture/**"
          - ".factory/phase-f4-implementation/**"

    - name: phase-f2-adversarial-review
      type: agent
      depends_on: [phase-f2-spec-evolution, phase-f2-dtu-reassessment, phase-f2-gene-transfusion-assessment, phase-f2-ux-design]
      agent: adversary
      model_tier: adversary
      task: >
        Review spec CHANGES only (not full spec) with fresh context.
        Focus on delta consistency, backward compatibility, and regression risk.
        Verify BC-S.SS.NNN numbering is correct and L1->L4 chain is complete.
        If UI feature: also review UX spec delta (.factory/feature/ux-delta.md).
      context:
        include:
          - ".factory/specs/**"
          - ".factory/feature/spec-delta.md"
          - ".factory/feature/ux-delta.md"
        exclude:
          # Information asymmetry wall: adversary cannot see prior adversary findings
          - ".factory/feature/adversarial-*"

    # Gemini spec review — constructive 3rd perspective on F2 spec evolution
    - name: phase-f2-spec-review-gemini
      type: loop
      depends_on: [phase-f2-adversarial-review]
      loop:
        max_iterations: 10
        exit_condition: "spec_reviewer.verdict == 'APPROVED' OR spec_reviewer.findings.critical == 0"
        steps:
          - name: gemini-f2-spec-review
            type: agent
            agent: spec-reviewer
            task: >
              Review F2 spec evolution artifacts AFTER adversary remediation.
              Focus on delta quality: are new/modified BCs coherent with existing
              specs? Are architecture changes justified? Use SR-NNN finding IDs.
              Write to .factory/specs/adversarial-reviews/f2-spec-review-pass[N].md
            context:
              include:
                - ".factory/specs/**"
                - ".factory/design-system/**"
              exclude:
                - ".factory/cycles/**/implementation/**"
          - name: gemini-f2-remediation
            type: agent
            agent: orchestrator
            depends_on: [gemini-f2-spec-review]
            condition: "spec_reviewer.findings.critical > 0 OR spec_reviewer.findings.high > 0"
            task: >
              Triage spec-reviewer SR-NNN findings for F2. Route fixes to
              product-owner or architect as appropriate.

    - name: phase-f2-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f2-spec-review-gemini]
      task: >
        Commit F2 artifacts to factory-artifacts branch.
        Update STATE.md: phase: F2-complete.

    - name: phase-f2-gate
      type: gate
      depends_on: [phase-f2-spec-review-gemini, phase-f2-a11y-review]
      gate:
        criteria:
          - "PRD updated with new requirements (delta only, BC-S.SS.NNN format)"
          - "Architecture updated if structural changes needed (sharded format)"
          - "Verification properties updated for new proofs (VP-NNN format)"
          - "Adversary approved spec changes"
          - "DTU re-assessment complete (if new external deps)"
          - "Gene transfusion assessment complete (if reference impl exists)"
          - "L1->L4 chain validated by consistency-validator"
          - "UX delta reviewed by ux-designer (if UI feature)"
          - "UX spec references design tokens and component contracts (DF-037 D1)"
          - "Accessibility review of UX delta complete (if UI feature)"
          - "Design system exists in .factory/design-system/ (if UI feature, DF-037 D2)"
          - "Multi-variant design reviewed for complex screens (if applicable, DF-037 D5)"
        fail_action: block

    - name: phase-f2-human-approval
      type: human-approval
      depends_on: [phase-f2-gate]
      approval:
        prompt: >
          Spec evolution is complete. Review the spec delta and adversarial
          review findings. Approve to proceed to incremental story creation.
        artifacts:
          - ".factory/feature/spec-delta.md"
          - ".factory/feature/adversarial-spec-review.md"
        timeout: "24h"

    # =========================================================================
    # Phase F3: Incremental Story Creation
    # Per-file stories (STORY-NNN.md), wave schedule, wave holdout scenarios
    # =========================================================================

    - name: phase-f3-incremental-stories
      type: skill
      skill: "skills/phase-f3-incremental-stories/SKILL.md"
      depends_on: [phase-f2-human-approval]

    # Gemini story review — constructive review of F3 incremental stories
    - name: phase-f3-spec-review-gemini
      type: loop
      depends_on: [phase-f3-incremental-stories]
      loop:
        max_iterations: 10
        exit_condition: "spec_reviewer.verdict == 'APPROVED' OR spec_reviewer.findings.critical == 0"
        steps:
          - name: gemini-f3-story-review
            type: agent
            agent: spec-reviewer
            task: >
              Review F3 incremental stories: decomposition quality, AC
              completeness, BC coverage, dependency graph, wave schedule.
              Focus on delta-specific concerns — do new stories integrate
              cleanly with existing stories? Use SR-NNN finding IDs.
              Write to .factory/stories/f3-spec-review-pass[N].md
            context:
              include:
                - ".factory/specs/**"
                - ".factory/stories/**"
                - ".factory/holdout-scenarios/**"
              exclude:
                - ".factory/cycles/**/implementation/**"
          - name: gemini-f3-remediation
            type: agent
            agent: orchestrator
            depends_on: [gemini-f3-story-review]
            condition: "spec_reviewer.findings.critical > 0 OR spec_reviewer.findings.high > 0"
            task: >
              Triage spec-reviewer SR-NNN findings for F3 stories.
              Route fixes to story-writer.

    - name: phase-f3-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f3-spec-review-gemini]
      task: >
        Commit F3 artifacts to factory-artifacts branch.
        Update STATE.md: phase: F3-complete.

    - name: phase-f3-gate
      type: gate
      depends_on: [phase-f3-spec-review-gemini]
      gate:
        criteria:
          - "New per-file stories (STORY-NNN.md) added to existing dependency graph"
          - "No dependency cycles introduced"
          - "No conflicts with in-progress work"
          - "Every new story has verification properties (VP-NNN)"
          - "Every new story references BCs (BC-S.SS.NNN)"
          - "Wave schedule generated for new stories"
          - "Wave holdout scenarios generated"
          - "Every BC precondition, postcondition, and invariant clause has at least one AC or justified gap"
          - "Every BC edge case (EC-NNN) covered by story AC or edge case"
          - "Every error taxonomy entry (E-xxx-NNN) covered by story AC or edge case"
          - "Every P0/P1 NFR-NNN referenced by at least one story"
          - "UI stories cover all required component states from contracts"
          - "AC Completeness score >= 90% (weighted L1/L2/L3)"
          - "Every HIGH-impact ASM has holdout scenario (delta scope)"
          - "Every HIGH/HIGH R-NNN has holdout scenario (delta scope)"
          - "Every testable ASM has story with assumption_validations (delta scope)"
          - "Every HIGH R-NNN mitigation covered by story with risk_mitigations (delta scope)"
          - "Every FM-NNN has at least one holdout scenario (delta scope)"
          - "ASM/R Traceability section passes (delta scope)"
          - "PRD Out of Scope not violated by any new story AC"
        fail_action: block

    - name: phase-f3-human-approval
      type: human-approval
      depends_on: [phase-f3-gate]
      approval:
        prompt: >
          Incremental stories created. Review new stories, wave schedule, and
          updated dependency graph. Approve to proceed to delta implementation.
        artifacts:
          - ".factory/feature/new-stories/"
          - ".factory/stories/dependency-graph.md"
          - ".factory/feature/wave-schedule.md"
        timeout: "24h"

    # =========================================================================
    # Toolchain Preflight (if not recently run)
    # =========================================================================

    - name: toolchain-preflight
      type: agent
      agent: dx-engineer
      depends_on: [phase-f3-human-approval]
      condition: "toolchain.recently_checked == false"
      task: >
        Check tool versions, SHA pins, new dependency security audit.
        Block on any security finding.
      timeout: "15m"

    # =========================================================================
    # Phase F4: Delta Implementation (Wave Loop)
    # Per-story delivery via code-delivery.lobster sub-workflow
    # Two-step Red Gate (stub architect + test writer)
    # Wave integration gates
    # =========================================================================

    - name: phase-f4-delta-implementation
      type: loop
      depends_on: [phase-f3-human-approval, toolchain-preflight]
      timeout: "8h"
      loop:
        for_each: wave in feature-wave-schedule.md
        steps:
          # --- Create worktrees for this wave ---
          - name: create-worktrees
            type: agent
            agent: devops-engineer
            task: >
              Create git worktrees for all stories in this wave:
              git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN develop

          # --- Per-story delivery via code-delivery.lobster ---
          - name: per-story-delivery
            type: parallel
            depends_on: [create-worktrees]
            for_each: story in wave.stories
            steps:
              - name: story-delivery
                type: sub-workflow
                sub_workflow: "code-delivery.lobster"
                inputs:
                  story_id: "story.id"
                  worktree_path: ".worktrees/story.id"
                  feature_type: "f1.feature_type"
                  module_criticality: "story.module_criticality"
                  implementation_strategy: "story.implementation_strategy"

          # --- Wave Integration Gate ---
          - name: wave-integration-gate
            type: skill
            depends_on: [per-story-delivery]
            steps:
              - name: wave-full-test
                type: agent
                agent: implementer
                task: >
                  Run FULL test suite on merged develop (all tests, including
                  existing + all prior waves). Any failure = wave integration issue.

              - name: wave-adversarial-review
                type: agent
                agent: adversary
                depends_on: [wave-full-test]
                model_tier: adversary
                task: >
                  Review combined wave diff on develop. Fresh context.
                  Focus on cross-story integration issues.
                context:
                  exclude:
                    # Information asymmetry wall: wave adversary cannot see per-story PR reviews
                    - ".factory/code-delivery/*/review-findings.md"

              - name: wave-security-review
                type: agent
                agent: security-reviewer
                depends_on: [wave-full-test]
                condition: "wave.has_critical_stories == true"
                task: >
                  Security review combined wave diff for cross-story attack surfaces.
                context:
                  exclude:
                    # Information asymmetry wall
                    - ".factory/code-delivery/*/review-findings.md"

              - name: wave-holdout-regression
                type: agent
                agent: holdout-evaluator
                depends_on: [wave-full-test]
                task: >
                  Re-evaluate holdout scenarios that touch modules modified by
                  this wave. Compare to baseline. Report regressions.
                context:
                  exclude:
                    # Information asymmetry wall: holdout cannot see source code
                    - "src/**"
                    - ".factory/specs/**"

              - name: wave-e2e-regression
                type: agent
                agent: e2e-tester
                depends_on: [wave-full-test]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                task: >
                  Run e2e test suite (new + regression browser tests) on
                  merged develop. Playwright/Cypress visual + functional tests.

              - name: wave-a11y-audit
                type: agent
                agent: accessibility-auditor
                depends_on: [wave-full-test]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                task: >
                  Automated a11y checks on rendered UI from wave changes.
                  WCAG compliance on new/modified components.
                context:
                  exclude:
                    # Information asymmetry wall: a11y auditor cannot see architecture
                    - ".factory/specs/architecture/**"

              # --- DF-037 Wave-Level UI Quality Checks ---

              - name: wave-responsive-validation
                type: skill
                skill: "skills/responsive-validation/SKILL.md"
                depends_on: [wave-full-test]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                # D7: Test all screens at 4 breakpoints (375, 768, 1024, 1440).
                # Capture screenshots to .factory/ui-evidence/.

              - name: wave-performance-check
                type: agent
                agent: performance-engineer
                depends_on: [wave-full-test]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                task: >
                  DF-037 D8: Wave-level performance validation.
                  Run Lighthouse CI on all pages modified in this wave.
                  Check CWV targets: LCP < 2.5s, FID < 100ms, CLS < 0.1.
                  Bundle size per route < 200KB JS.
                  Skeleton screen presence check for async views.
                  Write .factory/ui-quality/wave-performance-[wave_id].md.

              - name: wave-ui-completeness
                type: skill
                skill: "skills/ui-completeness-check/SKILL.md"
                depends_on: [wave-full-test]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                # D3/D10: Update ui-traceability.yaml with wave progress.
                # Partial completeness acceptable during implementation.
                # Report gaps for next wave.

              - name: wave-ui-quality-gate
                type: skill
                skill: "skills/ui-quality-gate/SKILL.md"
                depends_on: [wave-a11y-audit, wave-responsive-validation, wave-performance-check, wave-ui-completeness]
                condition: "f1.feature_type in ['ui', 'full-stack']"
                # D16: Standard gate -- token + a11y + responsive + performance + states.

              - name: wave-demo
                type: agent
                agent: demo-recorder
                depends_on: [wave-full-test]
                task: "Record wave-level integration demos on develop."
                condition: "wave.stories.length > 1"

              - name: wave-integration-fix
                type: loop
                depends_on: [wave-adversarial-review, wave-holdout-regression, wave-security-review, wave-e2e-regression, wave-a11y-audit, wave-ui-quality-gate]
                condition: "wave_review.has_findings OR wave_holdout.has_regressions OR wave_security.has_findings"
                loop:
                  max_iterations: 10
                  exit_condition: "wave_integration.all_clear"
                  steps:
                    - name: triage-wave-findings
                      type: agent
                      agent: pr-manager
                      task: >
                        Triage wave integration findings. Create fix stories.
                        Route through code-delivery.lobster sub-workflow.
                    - name: fix-and-deliver
                      type: sub-workflow
                      sub_workflow: "code-delivery.lobster"
                      depends_on: [triage-wave-findings]
                    - name: re-verify
                      type: agent
                      agent: implementer
                      depends_on: [fix-and-deliver]
                      task: "Re-run full test suite on develop after fixes merged."

          # --- DTU validation (if new clones built in this wave) ---
          - name: dtu-validation
            type: agent
            agent: dtu-validator
            depends_on: [wave-integration-gate]
            condition: "wave.has_dtu_clones == true"
            task: "Validate DTU clone fidelity against real APIs via mcporter."

          # --- Wave state backup ---
          - name: wave-state-backup
            type: agent
            agent: state-manager
            depends_on: [wave-integration-gate]
            task: "Commit wave artifacts to factory-artifacts. Update STATE.md."

          # Wave gate
          - name: wave-gate
            type: gate
            depends_on: [wave-integration-gate]
            gate:
              criteria:
                - "Full test suite passes on merged develop"
                - "Wave adversarial review converged"
                - "Security review passed (if wave has CRIT/HIGH stories)"
                - "Holdout regression check passed"
                - "E2E regression passed (if UI/full-stack)"
                - "Accessibility audit passed (if UI/full-stack)"
                - "All integration fix PRs merged (if any)"
              fail_action: block

    - name: phase-f4-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f4-delta-implementation]
      task: >
        Commit F4 artifacts to factory-artifacts branch.
        Update STATE.md:
        - phase: F4-complete
        - wave_progress: which waves complete, which stories merged
        - regression_baseline_ref: (commit SHA before F4 started)

    - name: phase-f4-gate
      type: gate
      depends_on: [phase-f4-delta-implementation]
      gate:
        criteria:
          - "Regression baseline recorded before F4 started"
          - "Two-step Red Gate passed (stubs then tests)"
          - "All new tests pass"
          - "Full regression suite passes (existing tests unbroken)"
          - "Per-story delivery: worktree -> PR -> review -> merge for each story"
          - "pr-reviewer (Gemini) reviewed each story PR"
          - "security-reviewer engaged for CRIT/HIGH module stories"
          - "PR review converged in max 10 rounds per story"
          - "All wave integration gates passed"
          - "Wave gates: adversary, security, holdout, a11y (if UI)"
          - "Implementation follows existing conventions"
          - "All story PRs merged to develop"
        fail_action: block

    # =========================================================================
    # Build Verification (full product including new feature)
    # =========================================================================

    - name: build-verification
      type: agent
      agent: implementer
      depends_on: [phase-f4-gate]
      task: >
        Full release build + complete test suite on develop.
        Catches any inter-wave issues the last wave gate might have missed.
      timeout: "30m"

    - name: build-gate
      type: gate
      depends_on: [build-verification]
      gate:
        criteria:
          - "Release build succeeds"
          - "Full test suite passes"
        fail_action: block

    # =========================================================================
    # Holdout Evaluation (full product — not just delta)
    # =========================================================================

    - name: holdout-evaluation
      type: skill
      skill: "skills/phase-4-holdout-evaluation/SKILL.md"
      depends_on: [build-gate]
      context:
        exclude:
          # Information asymmetry wall: holdout cannot see source, specs
          - "src/**"
          - ".factory/specs/**"
          - ".factory/cycles/**/implementation/**"

    - name: holdout-gate
      type: gate
      depends_on: [holdout-evaluation]
      gate:
        criteria:
          - "All existing holdout scenarios still pass (>= 0.6)"
          - "New feature scenarios: mean satisfaction >= 0.85"
          - "Holdout evaluator had NO access to source code or specs"
        fail_action: block

    # =========================================================================
    # Phase F5: Scoped Adversarial Review
    # Information asymmetry walls enforced
    # Fix PRs through code-delivery.lobster
    # =========================================================================

    - name: phase-f5-scoped-adversarial
      type: loop
      depends_on: [holdout-gate]
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: adversary-delta-review
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review CHANGED code only with fresh context. Check for regressions,
              convention violations, and spec drift. Report CONVERGENCE_REACHED
              if findings are cosmetic only.
            context:
              include:
                - ".factory/feature/**"
                - "src/**"
                - "tests/**"
              exclude:
                # Information asymmetry wall: fresh context, no Phase F4 TDD logs
                - ".factory/cycles/**/implementation/red-gate-log*"
                - ".factory/cycles/**/implementation/implementer-notes*"
                # Information asymmetry wall: no semport history
                - ".factory/semport/**"

          - name: code-reviewer-secondary
            type: agent
            agent: code-reviewer
            model_tier: review
            depends_on: [adversary-delta-review]
            task: >
              Secondary adversarial review using Gemini. Fresh context.
              Focus on patterns the GPT-5.4 adversary may have missed.
            context:
              exclude:
                # Information asymmetry wall: code-reviewer cannot see adversary findings
                - ".factory/phase-f5-adversarial/**"

          - name: security-review-f5
            type: agent
            agent: security-reviewer
            depends_on: [adversary-delta-review]
            condition: "adversary.has_security_findings == true"
            task: >
              Security review on adversary-identified security issues.
              CWE/OWASP analysis. Post findings.
            context:
              exclude:
                # Information asymmetry wall: no impl reasoning
                - ".factory/cycles/**/implementation/implementer-notes*"

          - name: fix-delta-findings
            type: skill
            skill: "skills/phase-f5-scoped-adversarial/SKILL.md"
            depends_on: [adversary-delta-review, code-reviewer-secondary]

          - name: fix-pr-delivery-f5
            type: sub-workflow
            sub_workflow: "code-delivery.lobster"
            depends_on: [fix-delta-findings]
            condition: "f5.has_findings == true"

          - name: holdout-regression-f5
            type: skill
            skill: "skills/phase-4-holdout-evaluation/SKILL.md"
            depends_on: [fix-pr-delivery-f5]
            condition: "f5.has_behavior_changing_fixes == true"

    - name: phase-f5-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f5-scoped-adversarial]
      task: >
        Commit F5 artifacts to factory-artifacts branch.
        Update STATE.md:
        - phase: F5-complete
        - convergence_status: round number, findings remaining

    # =========================================================================
    # Phase F6: Targeted Hardening
    # Security Review Touchpoint #4: scan changed code, CRIT/HIGH -> BLOCK
    # Information asymmetry walls enforced
    # DTU adversarial testing if external service interaction changed
    # Fix PRs -> F5 lite -> re-verify only failing checks
    # =========================================================================

    - name: phase-f6-targeted-hardening
      type: skill
      skill: "skills/phase-f6-targeted-hardening/SKILL.md"
      depends_on: [phase-f5-scoped-adversarial]
      timeout: "2h"
      context:
        include:
          - "src/**"
          - "tests/**"
          - ".factory/specs/verification-properties/**"
        exclude:
          # Information asymmetry wall: formal-verifier cannot see adversary findings
          - ".factory/phase-f5-adversarial/**"

    - name: phase-f6-security-scan
      type: agent
      agent: security-reviewer
      depends_on: [phase-f6-targeted-hardening]
      task: >
        Security Review Touchpoint #4 (DF-025): Security scan on all changed
        code from the feature delta. CRIT/HIGH findings -> BLOCK for human
        review. Write findings to .factory/phase-f6-hardening/security-scan-results.md.
      context:
        exclude:
          - ".factory/phase-f5-adversarial/**"

    - name: phase-f6-dtu-adversarial
      type: agent
      agent: formal-verifier
      depends_on: [phase-f6-targeted-hardening]
      condition: "feature.changed_external_service_interaction == true"
      task: >
        Run adversarial testing against DTU clones for changed external
        service interactions. Failure injection, latency injection,
        partial corruption, rate limit simulation.

    - name: phase-f6-a11y-recheck
      type: agent
      agent: accessibility-auditor
      depends_on: [phase-f6-targeted-hardening]
      condition: "f1.feature_type in ['ui', 'full-stack']"
      task: >
        Re-check accessibility after hardening fixes.
        Verify F6 fixes didn't break a11y compliance.
      context:
        exclude:
          # Information asymmetry wall
          - ".factory/specs/architecture/**"

    - name: phase-f6-fix-delivery
      type: sub-workflow
      sub_workflow: "code-delivery.lobster"
      depends_on: [phase-f6-targeted-hardening]
      condition: "f6.has_findings == true"

    - name: phase-f6-gate
      type: gate
      depends_on: [phase-f6-targeted-hardening, phase-f6-security-scan, phase-f6-dtu-adversarial, phase-f6-a11y-recheck, phase-f6-fix-delivery]
      gate:
        criteria:
          - "Kani proofs pass for new/modified modules"
          - "Fuzz testing on new code paths: zero crashes"
          - "Mutation testing scoped to changed files: kill rate > 90%"
          - "Security scan: no CRIT/HIGH findings (or BLOCKED for human)"
          - "Full regression suite passes"
          - "DTU adversarial testing passed (if applicable)"
          - "Accessibility re-check passed (if UI)"
          - "All fix PRs merged (FIX-F6-NNN)"
        fail_action: block

    - name: phase-f6-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f6-gate]
      task: >
        Commit F6 artifacts to factory-artifacts branch.
        Update STATE.md:
        - phase: F6-complete
        - convergence_status: round number, findings remaining

    # =========================================================================
    # Phase F6 Demo Recording (DF-018)
    # =========================================================================

    - name: phase-f6-demo-recording
      type: skill
      skill: "skills/demo-recording/SKILL.md"
      depends_on: [phase-f6-gate]
      condition: "config.demo_recording.enabled != false"
      timeout: "30m"

    - name: phase-f6-visual-regression
      type: agent
      agent: visual-reviewer
      model_tier: review
      depends_on: [phase-f6-demo-recording]
      condition: "config.demo_recording.enabled != false"
      task: >
        VISUAL REGRESSION DETECTION: Compare current demo recordings against
        baseline demos from .factory/feature/demo-baseline/.
        1. Watch baseline demo (previous version)
        2. Watch current demo (after changes)
        3. Identify intentional changes vs unintentional regressions
        4. Report regressions with timestamps and descriptions
        Write findings to .factory/demo-evidence/visual-review.md.
      context:
        include:
          - ".factory/demo-recordings/**"
          - ".factory/feature/demo-baseline/**"
          - ".factory/demo-evidence/**"
          - ".factory/feature/new-stories/**"

    # =========================================================================
    # Pre-F7: UI Quality Loop Final Checks (DF-037)
    # Heuristic evaluation, full UI completeness, convergence quality gate
    # =========================================================================

    - name: phase-f7-heuristic-evaluation
      type: skill
      skill: "skills/ux-heuristic-evaluation/SKILL.md"
      depends_on: [phase-f6-gate]
      condition: "f1.feature_type in ['ui', 'full-stack']"
      # D6: Nielsen's 10 heuristics + cognitive walkthrough.
      # Scores < 0.7 flag for remediation before convergence.

    - name: phase-f7-ui-completeness-final
      type: skill
      skill: "skills/ui-completeness-check/SKILL.md"
      depends_on: [phase-f6-gate]
      condition: "f1.feature_type in ['ui', 'full-stack']"
      # D3/D10: Final completeness check. Zero gaps required.
      # Fidelity: 100% (all specified elements implemented).

    - name: phase-f7-responsive-final
      type: skill
      skill: "skills/responsive-validation/SKILL.md"
      depends_on: [phase-f6-gate]
      condition: "f1.feature_type in ['ui', 'full-stack']"
      # D7: Full responsive suite at all breakpoints for all screens.

    - name: phase-f7-ui-quality-gate
      type: skill
      skill: "skills/ui-quality-gate/SKILL.md"
      depends_on: [phase-f7-heuristic-evaluation, phase-f7-ui-completeness-final, phase-f7-responsive-final]
      condition: "f1.feature_type in ['ui', 'full-stack']"
      # D16: Convergence gate -- 100% on every dimension.
      # All checks pass or create FIX-UI-NNN stories.

    - name: phase-f7-ui-fix-delivery
      type: sub-workflow
      sub_workflow: "code-delivery.lobster"
      depends_on: [phase-f7-ui-quality-gate]
      condition: "f1.feature_type in ['ui', 'full-stack'] and ui_quality_gate.has_failures == true"
      # Deliver fixes for any UI quality gate failures before F7 convergence.

    # =========================================================================
    # Phase F7: Delta Convergence
    # 7-dimensional convergence on delta + regression
    # Cost-benefit analysis from cost-tracker data
    # Max 10 cycles with targeted failure routing
    # =========================================================================

    - name: phase-f7-delta-convergence
      type: loop
      depends_on: [phase-f6-gate, phase-f7-ui-fix-delivery]
      loop:
        max_iterations: 10
        exit_condition: "convergence.all_dimensions_pass == true"
        steps:
          - name: f7-convergence-check
            type: skill
            skill: "skills/phase-f7-delta-convergence/SKILL.md"

          # Convergence failure routing: targeted re-run of failing dimensions
          - name: f7-failure-routing
            type: agent
            agent: orchestrator
            depends_on: [f7-convergence-check]
            condition: "convergence.all_dimensions_pass == false"
            task: >
              Route convergence failures to responsible phases:
              - Spec/Impl not converged -> F5 (adversarial) with fix PRs
              - Tests not converged -> F6 (mutation + tests) with fix PRs
              - Holdout not converged -> holdout re-evaluation with fix PRs
              Fix PRs through code-delivery.lobster sub-workflow.
              Cost-benefit: compare P(finding in next iteration) * Value_avg
              vs Cost_iteration * 1.5. Flag MAXIMUM_VIABLE_REFINEMENT_REACHED
              if cost exceeds expected value.

          - name: f7-fix-pr-delivery
            type: sub-workflow
            sub_workflow: "code-delivery.lobster"
            depends_on: [f7-failure-routing]
            condition: "convergence.has_fix_prs == true"

    - name: phase-f7-gate
      type: gate
      depends_on: [phase-f7-delta-convergence]
      gate:
        criteria:
          - "Delta spec dimension: CONVERGED"
          - "Delta test dimension: CONVERGED"
          - "Delta implementation dimension: CONVERGED"
          - "Delta verification dimension: CONVERGED"
          - "Delta holdout dimension: CONVERGED (satisfaction >= 0.85)"
          - "Regression suite: ALL PASS"
          - "Traceability chain extended (not replaced)"
          - "Cost-benefit analysis included"
          - "Max 10 convergence cycles (or MAXIMUM_VIABLE_REFINEMENT_REACHED)"
          - "All fix PRs merged (FIX-F7-NNN)"
          - "UI quality gate: PASS (100% fidelity, zero gaps, if UI feature, DF-037 D16)"
          - "Heuristic evaluation >= 0.7 on all 10 heuristics (if UI feature, DF-037 D6)"
          - "Responsive validation: all breakpoints pass (if UI feature, DF-037 D7)"
        fail_action: block

    - name: phase-f7-state-backup
      type: agent
      agent: state-manager
      depends_on: [phase-f7-gate]
      task: >
        Commit F7 artifacts to factory-artifacts branch.
        Update STATE.md:
        - phase: F7-complete
        - convergence_status: all dimensions converged
        - convergence_rounds: N (out of max 10)

    - name: phase-f7-human-approval
      type: human-approval
      depends_on: [phase-f7-gate]
      approval:
        prompt: >
          DELTA CONVERGENCE ACHIEVED. All convergence dimensions report CONVERGED.
          Review the delta convergence report, traceability matrix, and cost-benefit
          analysis. Approve to authorize release.
        artifacts:
          - ".factory/phase-f7-convergence/delta-convergence-report.md"
          - ".factory/phase-f7-convergence/traceability-chain-delta.md"
          - ".factory/ui-quality/gate-report.md"
          - ".factory/ui-quality/completeness-report.md"
          - ".factory/ui-quality/heuristic-evaluation.md"
        timeout: "48h"

    # =========================================================================
    # Release (DF-025)
    # MINOR bump (new feature) or PATCH (enhancement)
    # =========================================================================

    - name: release
      type: skill
      skill: "skills/release/SKILL.md"
      depends_on: [phase-f7-human-approval]
      config:
        # MINOR bump for new feature, PATCH for enhancement
        bump_type: "${delta.intent == 'feature' ? 'MINOR' : 'PATCH'}"
      # Release flow:
      # 1. Determine version bump (MINOR for feature, PATCH for enhancement)
      # 2. .env product keys (if feature needs new keys -- notify human)
      # 3. Update CHANGELOG.md
      # 4. Create git tag
      # 5. gh release with notes
      # 6. Publish to package registries
      # 7. Notify human: feature shipped

    # =========================================================================
    # Feature Cycle Handoff (DF-030)
    # Archive cycle, snapshot, resume maintenance, present backlog
    # =========================================================================

    - name: feature-cycle-handoff
      type: agent
      agent: orchestrator
      depends_on: [release]
      task: >
        Archive feature cycle and return to steady-state. Delegate to state-manager:
        1. Archive cycle operational artifacts to cycles/vX.Y.Z-feature-NAME/.
        2. Update cycle manifest with delivered stories, BCs, VPs, cost.
        3. Snapshot factory-artifacts branch (git tag vX.Y.Z).
        4. Resume maintenance sweeps.
        5. Run spec coherence check (33 criteria via consistency-validator).
        6. Update tech-debt-register.md with any deferred findings.
        7. Update STATE.md: pipeline: STEADY-STATE, last_release: vX.Y.Z.
        8. Present product backlog to human:
           "Feature cycle complete. vX.Y.Z released.
            Next in backlog: [cycle name]. Start? [yes / reorder / defer]"

    # =========================================================================
    # Post-Feature Validation (DF-034, optional)
    # =========================================================================

    - name: post-feature-validation
      type: skill
      skill: "skills/post-feature-validation/SKILL.md"
      depends_on: [feature-cycle-handoff]
      condition: "config.post_feature_validation.enabled == true"
      # DF-034: Scheduled post-feature validation at 7, 30, and 90 days.
      # Checks feedback channels and analytics for feature reception signals.
      # Produces feature impact report: SUCCESS / PARTIAL / MISS verdict.
      # Recommendations feed back into discovery engine for next iteration.

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
