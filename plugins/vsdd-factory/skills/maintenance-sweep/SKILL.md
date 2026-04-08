---
name: maintenance-sweep
description: >
  Periodic quality sweep that scans for dependency vulnerabilities, documentation
  drift, pattern inconsistencies, stale holdout scenarios, and performance
  regressions. Opens cleanup PRs through the standard quality gate pipeline.
---

# Maintenance Sweep

## Trigger

Maintenance sweeps can be triggered:
1. **Scheduled** -- cron or GitHub Actions schedule (recommended: weekly)
2. **Manual** -- human requests "Run maintenance sweep"
3. **Post-deploy** -- after a Feature Mode merge completes

## Sweep Types

### Sweep 1: Dependency Audit (Split: DF-029)

**Agent:** `dx-engineer` (T3) runs scan commands, `security-reviewer` (T2) analyzes

Same split pattern as brownfield Phase 0e-sec: the T3 agent runs the commands,
the T2 agent analyzes the results.

**dx-engineer** runs:
```bash
# Rust
cargo audit 2>&1 | tee .factory/maintenance/dependency-audit-raw.log
cargo deny check 2>&1 | tee -a .factory/maintenance/dependency-audit-raw.log

# TypeScript
npm audit --json 2>&1 | tee .factory/maintenance/dependency-audit-raw.log

# Python
pip-audit 2>&1 | tee .factory/maintenance/dependency-audit-raw.log
```

**security-reviewer** analyzes results, classifies by severity, checks for
compromised or unmaintained dependencies.

**Action on findings:**
- CRITICAL/HIGH CVE: Open immediate fix PR via code-delivery.lobster
- MEDIUM: Log for next feature sprint
- LOW: Log only

### Sweep 2: Documentation Drift

**Agent:** `technical-writer` + `consistency-validator`

Compare documentation against current code:

1. Read README.md -- verify installation, usage examples, and API docs match current code
2. Read architecture docs -- verify component list matches actual module structure
3. Read API docs -- verify endpoint list matches actual routes/handlers
4. Read FACTORY.md / AGENTS.md -- verify referenced files still exist at stated paths
5. Check for TODO/FIXME/HACK comments older than 30 days

**Action on findings:**
- Stale docs: Open PR with updated documentation
- Broken file references: Open PR fixing references
- Ancient TODOs: Log for human triage

### Sweep 3: Pattern Consistency

**Agent:** `consistency-validator` + `code-reviewer`

Scan for code pattern drift:

1. Compare error handling patterns across all modules (detect mixed styles)
2. Compare naming conventions (detect inconsistencies between old and new code)
3. Check for deprecated pattern usage flagged by linters
4. Verify import ordering follows conventions
5. Check that new modules follow the architecture's layer rules

**Action on findings:**
- Automated fix available (import ordering, formatting): Open PR with fix
- Manual fix needed (error handling migration): Log with estimated effort
- Architecture violations: Flag for architect review

### Sweep 4: Holdout Scenario Freshness

**Agent:** `holdout-evaluator` + `product-owner`

Verify holdout scenarios still match the current system:

1. Run all holdout scenarios against the current deployed/built artifact
2. Identify scenarios that fail due to intentional behavior changes (not bugs)
3. Identify scenarios that no longer cover meaningful behavior (feature was replaced)
4. Check scenario coverage: are there features with zero holdout scenarios?

**Action on findings:**
- Stale scenarios (fail due to intentional changes): Flag for product-owner to update
- Missing coverage: Flag for product-owner to write new scenarios
- All scenarios pass: No action needed

### Sweep 5: Performance Regression Detection

**Agent:** `performance-engineer`

Run performance baselines against current code:

1. Execute existing benchmarks (criterion, pytest-benchmark, etc.)
2. Compare against last recorded baseline
3. Flag any metric that degraded more than 10% since last sweep
4. Track long-term performance trends

**Action on findings:**
- >25% degradation: Open immediate PR (or flag if cause unclear)
- 10-25% degradation: Log with trend data
- <10%: No action

### Sweep 6: DTU Fidelity Drift (DF-026)

**Agent:** `dtu-validator` (T3)

Check DTU clones against real API behavior:
- Run contract tests against DTU clones via mcporter
- Spot-check against real APIs
- Compute fidelity score drift since last check
- Flag degradation for clone update

**Condition:** Product has DTU clones

### Sweep 7: Spec Coherence (DF-030)

**Agent:** `consistency-validator` (T2)

Run 25 spec coherence checks from DF-030:
- L1->L4 chain integrity
- BC coverage completeness
- VP alignment with BCs
- Story-to-BC mapping
- Architecture section consistency

Flag spec drift for spec-steward update.

### Sweep 8: Tech Debt Register (DF-030)

**Agent:** `orchestrator` (T1)

Check tech debt register for overdue items:
- Items past due date: surface for human triage
- Items approaching due: WARNING notification

### Sweep 9: Accessibility Regression (UI products only)

**Agent:** `accessibility-auditor` (T2)

Run automated a11y scan on current build:
- Compare against last scan baseline
- Flag regressions
- Information asymmetry wall: cannot see architecture

**Condition:** Product has UI

## Sweep Execution Flow

```
Trigger (schedule / manual / post-deploy)
    |
    v
[Orchestrator] Reads maintenance-config.yaml
[state-manager] STATE.md -> maintenance_run: STARTED
    |
    +-- Sweep 1: Dependency Audit (dx-engineer + security-reviewer)
    +-- Sweep 2: Documentation Drift
    +-- Sweep 3: Pattern Consistency
    +-- Sweep 4: Holdout Freshness         (9 sweeps in parallel)
    +-- Sweep 5: Performance Baseline
    +-- Sweep 6: DTU Fidelity Drift
    +-- Sweep 7: Spec Coherence
    +-- Sweep 8: Tech Debt Register
    +-- Sweep 9: Accessibility Regression
    |
    +-- [state-manager commits after EACH sweep]
    |
    v
[Orchestrator] Collects all findings
    |
    +-- Automated fixes available?
    |   YES --> Fix PR via code-delivery.lobster (worktree -> PR ->
    |           pr-reviewer wall -> security if CRIT -> merge)
    |           --> Gate passes? --> auto-merge at Level 3.5+
    |           --> Gate fails? --> Log for human review
    |
    +-- Manual fixes needed?
    |   YES --> Create issue/ticket with findings and estimated effort
    |
    +-- No findings?
        YES --> Log clean sweep report
    |
    v
Notifications:
  CRITICAL findings -> BLOCKING notification
  Overdue tech debt -> WARNING notification
  Sweep complete -> INFO notification
    |
    v
[state-manager] Final commit, STATE.md -> maintenance_run: COMPLETE
Write sweep report to `.factory/maintenance/sweep-report-YYYY-MM-DD.md`
```

## Output Artifacts

- `.factory/maintenance/sweep-report-YYYY-MM-DD.md`
- `.factory/maintenance/dependency-audit.log`
- `.factory/maintenance/doc-drift-findings.md`
- `.factory/maintenance/pattern-findings.md`
- `.factory/maintenance/holdout-freshness.md`
- `.factory/maintenance/performance-baseline.md`

## Quality Gate for Auto-Generated PRs

Maintenance PRs go through the SAME quality gates as Feature Mode code:
1. All existing tests pass (regression)
2. Holdout scenarios pass (>= 90%)
3. Adversarial review (scoped to changed files) -- uses adversary model primary; review-tier model secondary pass for security-related fixes
4. Lint/format clean

This prevents maintenance from introducing new problems.

**Note:** For dependency audit fixes (CVE upgrades), the review-tier model review tier is recommended for the adversarial review step, as its 91.9% GPQA Diamond score gives it strong domain knowledge for evaluating whether dependency upgrades introduce behavioral changes or security regressions.

## Maintenance Configuration

**File:** `.factory/maintenance-config.yaml` (created at first maintenance run)

```yaml
# Maintenance Mode Configuration
# Controls which sweeps run, on what schedule, and what actions to take.

schedule:
  frequency: weekly          # daily | weekly | manual-only
  day_of_week: sunday        # For weekly runs
  time_utc: "02:00"          # Run during low-activity hours

sweeps:
  dependency_audit:
    enabled: true
    auto_pr_severity: high   # Auto-open PR for this severity and above

  documentation_drift:
    enabled: true
    auto_pr: true            # Auto-open PR for fixable drift

  pattern_consistency:
    enabled: true
    auto_pr: false           # Log only -- patterns need human judgment

  holdout_freshness:
    enabled: true
    coverage_threshold: 0.8  # Flag features with <80% scenario coverage

  performance_baseline:
    enabled: true
    regression_threshold: 0.10  # 10% degradation triggers warning
    critical_threshold: 0.25   # 25% degradation triggers PR/alert

# Quality gate for auto-generated PRs
quality_gate:
  require_holdout_pass: true
  require_adversarial_review: true
  require_regression_pass: true
  auto_merge: false          # Set to true at autonomy Level 3.5+
```

## GitHub Actions Integration

The recommended way to schedule maintenance sweeps is via GitHub Actions:

```yaml
# .github/workflows/maintenance-sweep.yml
name: Maintenance Sweep
on:
  schedule:
    - cron: '0 2 * * 0'  # Weekly, Sunday 2am UTC
  workflow_dispatch:       # Manual trigger

jobs:
  sweep:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run maintenance sweep
        run: |
          claude --agent orchestrator --skill maintenance-sweep
```
