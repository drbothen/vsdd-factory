workflow:
  name: phase-5-formal-hardening
  description: >
    Phase 5 decomposed into reusable sub-workflow. Kani proofs, fuzzing,
    mutation testing, and security scanning based on toolchain-state.yaml.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "4h"

  steps:
    - name: formal-hardening
      type: skill
      skill: "skills/phase-5-formal-hardening/SKILL.md"
      depends_on: []

    - name: hardening-gate
      type: gate
      depends_on: [formal-hardening]
      gate:
        criteria:
          - "All Kani proofs pass"
          - "Fuzz testing: 5 minutes per target, zero crashes"
          - "Mutation kill rate > 90% (adjusted by module criticality)"
          - "Zero critical/high Semgrep findings"
          - "Cargo audit: no known vulnerabilities"
          - "Purity boundaries intact -- no side effects in pure core"
        fail_action: block
