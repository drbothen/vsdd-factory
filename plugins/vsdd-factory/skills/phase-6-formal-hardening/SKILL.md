---
name: phase-6-formal-hardening
description: Phase 6 entry point — Kani proofs, fuzz testing, mutation testing, and security scanning. Delegates to phase sub-workflow.
---

# Phase 6: Formal Hardening

Phase entry point for formal verification and hardening. Four verification techniques applied to the codebase.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-6-formal-hardening.lobster
```

## Steps

| Step | File | What It Does |
|------|------|-------------|
| A | `formal-verify/steps/step-a-kani-proofs.md` | Kani proof harnesses for pure core |
| B | `formal-verify/steps/step-b-fuzz-testing.md` | Fuzz targets for parsers/handlers |
| C | `formal-verify/steps/step-c-mutation-testing.md` | Mutation kill rate verification |
| D | `formal-verify/steps/step-d-security-scan.md` | Semgrep security scanning |

## Work Skill

Direct command: `/vsdd-factory:formal-verify`

## Prerequisites

- Phase 5 adversarial review converged
- Verification tools installed (cargo-kani, cargo-fuzz, cargo-mutants, semgrep)

## Gate Criteria

- All Kani proofs pass
- Fuzz testing: 5 minutes per target, zero crashes
- Mutation kill rate > 90% (adjusted by module criticality)
- Zero critical/high Semgrep findings
- Cargo audit: no known vulnerabilities
- Purity boundaries intact
