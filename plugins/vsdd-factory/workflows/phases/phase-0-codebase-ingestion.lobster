workflow:
  name: phase-0-codebase-ingestion
  description: >
    Brownfield Mode Phase 0: Analyze existing codebase to produce project context
    document. Runs before the greenfield pipeline (Phases 1-6) for brownfield projects.

    Includes per-step state-manager commits for crash recovery (DF-031 D4),
    security audit step (DF-031 D2), adversarial review (DF-031 D12),
    consistency validation (DF-031 D5), and cost monitoring (DF-031 D9).
  version: "2.0.0"

  defaults:
    on_failure: escalate
    timeout: "1h"

  plugins:
    - cost-tracker:
        phase: 0
        tier_response:
          warn: 70
          pause: 95

  steps:
    # === Step 0a: Project Discovery ===

    - name: step-0a-project-discovery
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0a-project-discovery.md"
      depends_on: []
      cost_metadata:
        phase: 0
        step: "0a"

    - name: backup-0a
      type: agent
      agent: state-manager
      depends_on: [step-0a-project-discovery]
      task: >
        Commit artifacts for step 0a to factory-artifacts.
        Update STATE.md: phase: 0, step: 0a, status: complete.

    # === Step 0b: Architecture Recovery ===

    - name: step-0b-architecture-recovery
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0b-architecture-recovery.md"
      depends_on: [backup-0a]
      cost_metadata:
        phase: 0
        step: "0b"

    - name: backup-0b
      type: agent
      agent: state-manager
      depends_on: [step-0b-architecture-recovery]
      task: >
        Commit artifacts for step 0b to factory-artifacts.
        Update STATE.md: phase: 0, step: 0b, status: complete.

    # === Step 0c: Convention Extraction ===

    - name: step-0c-convention-extraction
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0c-convention-extraction.md"
      depends_on: [backup-0a, backup-0b]
      cost_metadata:
        phase: 0
        step: "0c"

    - name: backup-0c
      type: agent
      agent: state-manager
      depends_on: [step-0c-convention-extraction]
      task: >
        Commit artifacts for step 0c to factory-artifacts.
        Update STATE.md: phase: 0, step: 0c, status: complete.

    # === Step 0d: Spec Reverse-Engineering + spec-steward Review ===

    - name: step-0d-spec-reverse-engineering
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0d-spec-reverse-engineering.md"
      depends_on: [backup-0b, backup-0c]
      cost_metadata:
        phase: 0
        step: "0d"

    - name: backup-0d
      type: agent
      agent: state-manager
      depends_on: [step-0d-spec-reverse-engineering]
      task: >
        Commit artifacts for step 0d to factory-artifacts.
        Update STATE.md: phase: 0, step: 0d, status: complete.

    # === Step 0e: Verification Gap Analysis ===

    - name: step-0e-verification-gap-analysis
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0e-verification-gap-analysis.md"
      depends_on: [backup-0d]
      cost_metadata:
        phase: 0
        step: "0e"

    - name: backup-0e
      type: agent
      agent: state-manager
      depends_on: [step-0e-verification-gap-analysis]
      task: >
        Commit artifacts for step 0e to factory-artifacts.
        Update STATE.md: phase: 0, step: 0e, status: complete.

    # === Step 0e-sec: Security & Dependency Audit ===

    - name: step-0e-sec-security-audit
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0e-sec-security-audit.md"
      depends_on: [backup-0e]
      cost_metadata:
        phase: 0
        step: "0e-sec"

    - name: backup-0e-sec
      type: agent
      agent: state-manager
      depends_on: [step-0e-sec-security-audit]
      task: >
        Commit artifacts for step 0e-sec to factory-artifacts.
        Update STATE.md: phase: 0, step: 0e-sec, status: complete.

    - name: security-audit-gate
      type: human-approval
      depends_on: [backup-0e-sec]
      approval:
        prompt: >
          Security & dependency audit complete. Any security finding requires
          human decision: fix now (Wave 1 story), accept risk, or abort.
          Review .factory/phase-0-ingestion/security-audit.md.
        artifacts:
          - ".factory/phase-0-ingestion/security-audit.md"
        timeout: "48h"

    # === Step 0e.5: Module Criticality Classification ===

    - name: step-0e5-module-criticality
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0e5-module-criticality.md"
      depends_on: [security-audit-gate]
      cost_metadata:
        phase: 0
        step: "0e.5"

    - name: backup-0e5
      type: agent
      agent: state-manager
      depends_on: [step-0e5-module-criticality]
      task: >
        Commit artifacts for step 0e.5 to factory-artifacts.
        Update STATE.md: phase: 0, step: 0e.5, status: complete.

    # === Step 0f-pre: Holdout Scenario Seeding ===

    - name: step-0f-pre-holdout-seeding
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0f-pre-holdout-seeding.md"
      depends_on: [backup-0d, backup-0e5]
      cost_metadata:
        phase: 0
        step: "0f-pre"

    - name: backup-0f-pre
      type: agent
      agent: state-manager
      depends_on: [step-0f-pre-holdout-seeding]
      task: >
        Commit artifacts for step 0f-pre to factory-artifacts.
        Update STATE.md: phase: 0, step: 0f-pre, status: complete.

    # === Step 0f: Project Context Synthesis ===

    - name: step-0f-project-context
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0f-project-context.md"
      depends_on: [backup-0e, backup-0e-sec, backup-0e5, backup-0f-pre]
      cost_metadata:
        phase: 0
        step: "0f"

    - name: backup-0f
      type: agent
      agent: state-manager
      depends_on: [step-0f-project-context]
      task: >
        Commit artifacts for step 0f to factory-artifacts.
        Update STATE.md: phase: 0, step: 0f, status: complete.

    # === Step 0f-adv: Adversarial Review (GPT-5.4, Information Asymmetry Wall) ===

    - name: step-0f-adv-adversarial-review
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0f-adv-adversarial-review.md"
      depends_on: [backup-0f]
      cost_metadata:
        phase: 0
        step: "0f-adv"
      context:
        exclude:
          - "target_codebase/**"

    - name: backup-0f-adv
      type: agent
      agent: state-manager
      depends_on: [step-0f-adv-adversarial-review]
      task: >
        Commit artifacts for step 0f-adv to factory-artifacts.
        Update STATE.md: phase: 0, step: 0f-adv, status: complete.

    # === Step 0f-post: Consistency Validation (fix loop) ===

    - name: step-0f-post-consistency-validation
      type: skill
      skill: "skills/phase-0-codebase-ingestion/steps/step-0f-post-consistency-validation.md"
      depends_on: [backup-0f-adv]
      cost_metadata:
        phase: 0
        step: "0f-post"
      retry:
        max_attempts: 10
        on_failure: >
          Route validation failures to originating agents for correction,
          then re-run consistency validation.

    - name: backup-0f-post
      type: agent
      agent: state-manager
      depends_on: [step-0f-post-consistency-validation]
      task: >
        Commit artifacts for step 0f-post to factory-artifacts.
        Update STATE.md: phase: 0, step: 0f-post, status: complete.

    # === Phase 0 Gate ===

    - name: phase-0-gate
      type: gate
      depends_on: [backup-0f-post]
      gate:
        criteria:
          - "Project context document exists at .factory/phase-0-ingestion/project-context.md"
          - "Module criticality classification exists at .factory/module-criticality.md"
          - "At least one behavioral contract exists in .factory/phase-0-ingestion/behavioral-contracts/"
          - "All behavioral contracts use BC-S.SS.NNN format with origin: recovered"
          - "Security audit complete at .factory/phase-0-ingestion/security-audit.md"
          - "Adversarial review complete at .factory/phase-0-ingestion/adversarial-review-0.md"
          - "Consistency validation passed at .factory/phase-0-ingestion/validation-report.md"
          - "Holdout scenarios seeded in .factory/holdout-scenarios/"
        fail_action: block

    - name: phase-0-human-approval
      type: human-approval
      depends_on: [phase-0-gate]
      approval:
        prompt: "Review the project context document. Is the recovered architecture accurate? Are conventions correct? Are restricted areas correctly identified?"
        artifacts:
          - ".factory/phase-0-ingestion/project-context.md"
          - ".factory/phase-0-ingestion/recovered-architecture.md"
          - ".factory/phase-0-ingestion/conventions.md"
          - ".factory/phase-0-ingestion/security-audit.md"
          - ".factory/phase-0-ingestion/adversarial-review-0.md"
          - ".factory/phase-0-ingestion/validation-report.md"
          - ".factory/module-criticality.md"
        timeout: "24h"
