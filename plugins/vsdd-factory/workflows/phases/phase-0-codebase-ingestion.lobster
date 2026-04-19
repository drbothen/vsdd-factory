workflow:
  name: phase-0-codebase-ingestion
  description: >
    Brownfield Mode Phase 0: Analyze existing codebase using the broad-then-converge
    protocol. Steps follow brownfield-ingest skill's internal phases: source acquisition,
    broad sweep (7 passes), convergence deepening, coverage audit, extraction validation,
    and final synthesis. State-manager commits after each step for crash recovery.
  version: "3.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "4h"

  steps:
    # === Step 0: Source Acquisition ===

    - name: source-acquisition
      type: skill
      skill: "skills/brownfield-ingest/steps/step-0-source-acquisition.md"
      depends_on: []

    - name: backup-source-acquisition
      type: agent
      agent: state-manager
      depends_on: [source-acquisition]
      task: >
        Commit source acquisition artifacts to factory-artifacts.
        Update STATE.md: phase: 0, step: source-acquisition, status: complete.

    # === Step A: Broad Sweep (Passes 0-6) ===

    - name: broad-sweep
      type: skill
      skill: "skills/brownfield-ingest/steps/step-a-broad-sweep.md"
      depends_on: [backup-source-acquisition]
      timeout: "2h"

    - name: backup-broad-sweep
      type: agent
      agent: state-manager
      depends_on: [broad-sweep]
      task: >
        Commit broad sweep artifacts (7 pass files) to factory-artifacts.
        Update STATE.md: phase: 0, step: broad-sweep, status: complete.

    # === Step B: Convergence Deepening ===

    - name: convergence-deepening
      type: skill
      skill: "skills/brownfield-ingest/steps/step-b-convergence-deepening.md"
      depends_on: [backup-broad-sweep]
      timeout: "8h"

    - name: backup-convergence-deepening
      type: agent
      agent: state-manager
      depends_on: [convergence-deepening]
      task: >
        Commit all deepening round artifacts to factory-artifacts.
        Update STATE.md: phase: 0, step: convergence-deepening, status: complete.

    # === Step B.5: Coverage Audit ===

    - name: coverage-audit
      type: skill
      skill: "skills/brownfield-ingest/steps/step-b5-coverage-audit.md"
      depends_on: [backup-convergence-deepening]
      timeout: "2h"

    - name: backup-coverage-audit
      type: agent
      agent: state-manager
      depends_on: [coverage-audit]
      task: >
        Commit coverage audit artifacts to factory-artifacts.
        Update STATE.md: phase: 0, step: coverage-audit, status: complete.

    # === Step B.6: Extraction Validation ===

    - name: extraction-validation
      type: skill
      skill: "skills/brownfield-ingest/steps/step-b6-extraction-validation.md"
      depends_on: [backup-coverage-audit]
      timeout: "1h"

    - name: backup-extraction-validation
      type: agent
      agent: state-manager
      depends_on: [extraction-validation]
      task: >
        Commit extraction validation artifacts to factory-artifacts.
        Update STATE.md: phase: 0, step: extraction-validation, status: complete.

    # === Step C: Final Synthesis ===

    - name: final-synthesis
      type: skill
      skill: "skills/brownfield-ingest/steps/step-c-final-synthesis.md"
      depends_on: [backup-extraction-validation]
      timeout: "1h"

    - name: backup-final-synthesis
      type: agent
      agent: state-manager
      depends_on: [final-synthesis]
      task: >
        Commit final synthesis to factory-artifacts.
        Update STATE.md: phase: 0, step: final-synthesis, status: complete.

    # === Phase 0 Gate ===

    - name: phase-0-gate
      type: gate
      depends_on: [backup-final-synthesis]
      gate:
        criteria:
          - "Project context document exists at .factory/phase-0-ingestion/project-context.md"
          - "Module criticality classification exists at .factory/module-criticality.md"
          - "At least one behavioral contract exists in .factory/phase-0-ingestion/behavioral-contracts/"
          - "All behavioral contracts use BC-S.SS.NNN format with origin: recovered"
          - "Security audit complete at .factory/phase-0-ingestion/security-audit.md"
          - "Consistency validation passed at .factory/phase-0-ingestion/validation-report.md"
          - "Holdout scenarios seeded in .factory/holdout-scenarios/"
        fail_action: block

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [phase-0-gate]
      description: >
        Scan Phase 0 artifacts for stale input-hashes before human approval.

    - name: human-approval
      type: human-approval
      depends_on: [input-hash-drift-check]
      approval:
        prompt: "Review the project context document. Is the recovered architecture accurate? Are conventions correct? Are restricted areas correctly identified?"
        artifacts:
          - ".factory/phase-0-ingestion/project-context.md"
          - ".factory/phase-0-ingestion/recovered-architecture.md"
          - ".factory/phase-0-ingestion/conventions.md"
          - ".factory/phase-0-ingestion/security-audit.md"
          - ".factory/phase-0-ingestion/validation-report.md"
          - ".factory/module-criticality.md"
        timeout: "24h"
