---
name: phase-f6-targeted-hardening
description: >
  Feature Mode Phase F6: Formal verification, fuzz testing, and mutation testing
  scoped to the delta. Full regression and security scans on full tree.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F6: Targeted Hardening

## Prerequisites

- Phase F5 Scoped Adversarial Review complete (converged)
- `.factory/phase-f5-adversarial/convergence-summary.md` shows CONVERGED
- `.factory/phase-f1-delta-analysis/affected-files.txt` exists

## Workflow

### Step 1: Determine Hardening Scope

Read the delta analysis and build the hardening scope:

| Tool | Scope | Rationale |
|------|-------|-----------|
| Kani proofs | New/modified modules only | Prove properties of new code |
| Fuzz testing | New code paths only | Find crashes in new input handling |
| Mutation testing | Changed files only | Verify test quality for new code |
| Semgrep | Changed files + new files | Security patterns on delta |
| Regression tests | Full existing test suite | Protect existing behavior |
| cargo audit / npm audit | Full dependency tree | New deps may introduce vulns |

### Step 2: Formal Verification (Kani Proofs)

Spawn `formal-verifier` agent to:
- Read verification properties from `.factory/phase-f2-spec-evolution/verification-delta.md`
- Write Kani proof harnesses for each new verification property
- Run proofs: `cargo kani --harness <name>`
- For each property:
  - PASS: record in verification log
  - FAIL: identify the violation, fix implementation, re-run
  - UNREACHABLE: verify the property is correctly specified

Write results to `.factory/phase-f6-hardening/kani-results.md`

If the project is not Rust or does not use Kani, substitute the appropriate
formal verification tool (e.g., Hypothesis for Python, fast-check for TypeScript)
or skip with documented justification.

### Step 3: Fuzz Testing

Spawn `formal-verifier` agent to:
- Write fuzz targets for new public interfaces and input handling code
- Run fuzzing for at least 5 minutes per target: `cargo fuzz run <target> -- -max_total_time=300`
- Report any crashes found
- For each crash: create a regression test, fix the code, re-fuzz

Write results to `.factory/phase-f6-hardening/fuzz-results.md`

If the project does not support fuzzing, skip with documented justification.

### Step 4: Mutation Testing

Run mutation testing scoped to changed files:

```bash
# Rust:
cargo mutants --file <changed-file-1> --file <changed-file-2> ...
# The file list comes from .factory/phase-f1-delta-analysis/affected-files.txt
```

Target: mutation kill rate >= 90% for changed files (>= 95% for security-critical modules).

If kill rate is below threshold:
- Identify surviving mutants
- Write additional tests to kill them
- Re-run mutation testing

Write results to `.factory/phase-f6-hardening/mutation-results.md`

### Step 5: Security Scanning

Run security scans on the full tree (not just delta):

```bash
# Semgrep on changed files:
semgrep --config=auto <changed-files>

# Dependency audit on full tree:
cargo audit  # or npm audit, pip-audit, etc.
```

Write results to `.factory/phase-f6-hardening/security-scan-results.md`

Any CRITICAL or HIGH severity findings must be fixed before proceeding.

### Step 6: Full Regression Suite

Run the complete test suite one final time:
- All existing tests must pass
- All new tests must pass
- Compare against the regression baseline from Phase F4

### Step 7: Hardening Summary

Write summary to `.factory/phase-f6-hardening/summary.md`:
- Kani: N proofs attempted, N passed, N failed (with fix status)
- Fuzz: N targets, M minutes total, crashes found/fixed
- Mutation: kill rate per file, overall kill rate
- Security: findings by severity, all CRITICAL/HIGH resolved
- Regression: full suite pass/fail

Phase F6 is COMPLETE when all hardening checks pass and regression suite is clean.
No human gate -- this is an automated quality gate.

### Step 7b: DTU Adversarial Testing (if external service interaction changed)

If the feature changed how the product interacts with external services:
- dtu-validator configures adversarial DTU clones (L4)
- Failure injection, latency injection, partial corruption, rate limit simulation
- Run SUT integration tests and fuzz targets against adversarial clones
- Report results to `.factory/phase-f6-hardening/dtu-adversarial-results.md`

### Step 7c: Fix PR Delivery (DF-025)

When F6 finds issues on merged develop, fixes go through code-delivery.lobster:
- FIX-F6-NNN -> worktree -> fix -> PR -> AI review -> merge
- Then: F5 "lite" review of fix diffs (not full F5 re-run)
- Then: re-verify ONLY failing Phase F6 checks (partial re-verification)

### Step 7d: Accessibility Re-Check (if UI feature)

Condition: `feature_type in ['ui', 'full-stack']`

Spawn `accessibility-auditor` (T2) to:
- Re-check accessibility after hardening fixes
- Verify F6 fix PRs didn't break accessibility compliance
- Final a11y pass before convergence
- Write `.factory/phase-f6-hardening/accessibility-recheck.md`

### Information Asymmetry Wall

The formal-verifier CANNOT see adversarial review findings from F5 (DF-025).
This ensures independent verification — the formal-verifier should verify
properties from the spec, not guided by what the adversary looked for.

## Output Artifacts

- `.factory/phase-f6-hardening/kani-results.md`
- `.factory/phase-f6-hardening/fuzz-results.md`
- `.factory/phase-f6-hardening/mutation-results.md`
- `.factory/phase-f6-hardening/security-scan-results.md`
- `.factory/phase-f6-hardening/dtu-adversarial-results.md` (if external service interaction changed)
- `.factory/phase-f6-hardening/accessibility-recheck.md` (if UI feature)
- `.factory/phase-f6-hardening/summary.md`

## Quality Gate Criteria

- [ ] Kani proofs pass for all new verification properties (or justified skip)
- [ ] Fuzz testing clean after 5 min/target (or justified skip)
- [ ] Mutation kill rate >= 90% on changed files (>= 95% for critical modules)
- [ ] No CRITICAL or HIGH security findings unresolved (CRIT/HIGH -> BLOCK for human)
- [ ] Full regression suite passes
- [ ] DTU adversarial testing passed (if external service interaction changed)
- [ ] Accessibility re-check passed (if UI feature)
- [ ] Fix PRs via code-delivery.lobster (FIX-F6-NNN)
- [ ] F6 re-verifies only failing checks after fix (partial re-verification)
- [ ] Hardening summary written with all metrics
