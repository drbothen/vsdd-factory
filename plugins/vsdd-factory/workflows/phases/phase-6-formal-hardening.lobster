workflow:
  name: phase-6-formal-hardening
  description: >
    Phase 5: Formal Hardening. Four verification techniques (Kani proofs,
    fuzz testing, mutation testing, security scanning) with state-manager
    backups for crash recovery.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "4h"

  steps:
    # === Step A: Kani Proofs ===

    - name: kani-proofs
      type: skill
      skill: "skills/formal-verify/steps/step-a-kani-proofs.md"
      depends_on: []
      timeout: "2h"

    - name: backup-kani-proofs
      type: agent
      agent: state-manager
      depends_on: [kani-proofs]
      task: >
        Commit Kani proof harnesses and results to factory-artifacts.
        Update STATE.md: phase: 6, step: kani-proofs, status: complete.

    # === Step B: Fuzz Testing ===

    - name: fuzz-testing
      type: skill
      skill: "skills/formal-verify/steps/step-b-fuzz-testing.md"
      depends_on: [backup-kani-proofs]
      timeout: "2h"

    - name: backup-fuzz-testing
      type: agent
      agent: state-manager
      depends_on: [fuzz-testing]
      task: >
        Commit fuzz targets and results to factory-artifacts.
        Update STATE.md: phase: 6, step: fuzz-testing, status: complete.

    # === Step C: Mutation Testing ===

    - name: mutation-testing
      type: skill
      skill: "skills/formal-verify/steps/step-c-mutation-testing.md"
      depends_on: [backup-fuzz-testing]
      timeout: "1h"

    - name: backup-mutation-testing
      type: agent
      agent: state-manager
      depends_on: [mutation-testing]
      task: >
        Commit mutation testing results to factory-artifacts.
        Update STATE.md: phase: 6, step: mutation-testing, status: complete.

    # === Step D: Security Scan ===

    - name: security-scan
      type: skill
      skill: "skills/formal-verify/steps/step-d-security-scan.md"
      depends_on: [backup-mutation-testing]

    - name: backup-security-scan
      type: agent
      agent: state-manager
      depends_on: [security-scan]
      task: >
        Commit security scan results to factory-artifacts.
        Update STATE.md: phase: 6, step: security-scan, status: complete.

    # === Hardening Gate ===

    - name: hardening-gate
      type: gate
      depends_on: [backup-security-scan]
      gate:
        criteria:
          - "All Kani proofs pass"
          - "Fuzz testing: 5 minutes per target, zero crashes"
          - "Mutation kill rate > 90% (adjusted by module criticality)"
          - "Zero critical/high Semgrep findings"
          - "Cargo audit: no known vulnerabilities"
          - "Purity boundaries intact -- no side effects in pure core"
        fail_action: block
